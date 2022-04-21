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
impl ::core::fmt::Debug for ASYNC_DUPLICATE_EXTENTS_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ASYNC_DUPLICATE_EXTENTS_STATUS").field("Version", &self.Version).field("State", &self.State).field("SourceFileOffset", &self.SourceFileOffset).field("TargetFileOffset", &self.TargetFileOffset).field("ByteCount", &self.ByteCount).field("BytesDuplicated", &self.BytesDuplicated).finish()
    }
}
unsafe impl ::windows::core::Abi for ASYNC_DUPLICATE_EXTENTS_STATUS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for ASYNC_DUPLICATE_EXTENTS_STATUS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<ASYNC_DUPLICATE_EXTENTS_STATUS>()) == 0 }
    }
}
impl ::core::cmp::Eq for ASYNC_DUPLICATE_EXTENTS_STATUS {}
impl ::core::default::Default for ASYNC_DUPLICATE_EXTENTS_STATUS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for BIN_COUNT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("BIN_COUNT").field("BinRange", &self.BinRange).field("BinCount", &self.BinCount).finish()
    }
}
unsafe impl ::windows::core::Abi for BIN_COUNT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for BIN_COUNT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<BIN_COUNT>()) == 0 }
    }
}
impl ::core::cmp::Eq for BIN_COUNT {}
impl ::core::default::Default for BIN_COUNT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for BIN_RANGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("BIN_RANGE").field("StartValue", &self.StartValue).field("Length", &self.Length).finish()
    }
}
unsafe impl ::windows::core::Abi for BIN_RANGE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for BIN_RANGE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<BIN_RANGE>()) == 0 }
    }
}
impl ::core::cmp::Eq for BIN_RANGE {}
impl ::core::default::Default for BIN_RANGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for BIN_RESULTS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("BIN_RESULTS").field("NumberOfBins", &self.NumberOfBins).field("BinCounts", &self.BinCounts).finish()
    }
}
unsafe impl ::windows::core::Abi for BIN_RESULTS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for BIN_RESULTS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<BIN_RESULTS>()) == 0 }
    }
}
impl ::core::cmp::Eq for BIN_RESULTS {}
impl ::core::default::Default for BIN_RESULTS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct BIN_TYPES(pub i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const RequestSize: BIN_TYPES = BIN_TYPES(0i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const RequestLocation: BIN_TYPES = BIN_TYPES(1i32);
impl ::core::marker::Copy for BIN_TYPES {}
impl ::core::clone::Clone for BIN_TYPES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for BIN_TYPES {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for BIN_TYPES {
    type Abi = Self;
}
impl ::core::fmt::Debug for BIN_TYPES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BIN_TYPES").field(&self.0).finish()
    }
}
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
impl ::core::fmt::Debug for BOOT_AREA_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("BOOT_AREA_INFO").field("BootSectorCount", &self.BootSectorCount).field("BootSectors", &self.BootSectors).finish()
    }
}
unsafe impl ::windows::core::Abi for BOOT_AREA_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for BOOT_AREA_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<BOOT_AREA_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for BOOT_AREA_INFO {}
impl ::core::default::Default for BOOT_AREA_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for BOOT_AREA_INFO_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("BOOT_AREA_INFO_0").field("Offset", &self.Offset).finish()
    }
}
unsafe impl ::windows::core::Abi for BOOT_AREA_INFO_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for BOOT_AREA_INFO_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<BOOT_AREA_INFO_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for BOOT_AREA_INFO_0 {}
impl ::core::default::Default for BOOT_AREA_INFO_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for BULK_SECURITY_TEST_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("BULK_SECURITY_TEST_DATA").field("DesiredAccess", &self.DesiredAccess).field("SecurityIds", &self.SecurityIds).finish()
    }
}
unsafe impl ::windows::core::Abi for BULK_SECURITY_TEST_DATA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for BULK_SECURITY_TEST_DATA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<BULK_SECURITY_TEST_DATA>()) == 0 }
    }
}
impl ::core::cmp::Eq for BULK_SECURITY_TEST_DATA {}
impl ::core::default::Default for BULK_SECURITY_TEST_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct CHANGER_DEVICE_PROBLEM_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const DeviceProblemNone: CHANGER_DEVICE_PROBLEM_TYPE = CHANGER_DEVICE_PROBLEM_TYPE(0i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const DeviceProblemHardware: CHANGER_DEVICE_PROBLEM_TYPE = CHANGER_DEVICE_PROBLEM_TYPE(1i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const DeviceProblemCHMError: CHANGER_DEVICE_PROBLEM_TYPE = CHANGER_DEVICE_PROBLEM_TYPE(2i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const DeviceProblemDoorOpen: CHANGER_DEVICE_PROBLEM_TYPE = CHANGER_DEVICE_PROBLEM_TYPE(3i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const DeviceProblemCalibrationError: CHANGER_DEVICE_PROBLEM_TYPE = CHANGER_DEVICE_PROBLEM_TYPE(4i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const DeviceProblemTargetFailure: CHANGER_DEVICE_PROBLEM_TYPE = CHANGER_DEVICE_PROBLEM_TYPE(5i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const DeviceProblemCHMMoveError: CHANGER_DEVICE_PROBLEM_TYPE = CHANGER_DEVICE_PROBLEM_TYPE(6i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const DeviceProblemCHMZeroError: CHANGER_DEVICE_PROBLEM_TYPE = CHANGER_DEVICE_PROBLEM_TYPE(7i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const DeviceProblemCartridgeInsertError: CHANGER_DEVICE_PROBLEM_TYPE = CHANGER_DEVICE_PROBLEM_TYPE(8i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const DeviceProblemPositionError: CHANGER_DEVICE_PROBLEM_TYPE = CHANGER_DEVICE_PROBLEM_TYPE(9i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const DeviceProblemSensorError: CHANGER_DEVICE_PROBLEM_TYPE = CHANGER_DEVICE_PROBLEM_TYPE(10i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const DeviceProblemCartridgeEjectError: CHANGER_DEVICE_PROBLEM_TYPE = CHANGER_DEVICE_PROBLEM_TYPE(11i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const DeviceProblemGripperError: CHANGER_DEVICE_PROBLEM_TYPE = CHANGER_DEVICE_PROBLEM_TYPE(12i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const DeviceProblemDriveError: CHANGER_DEVICE_PROBLEM_TYPE = CHANGER_DEVICE_PROBLEM_TYPE(13i32);
impl ::core::marker::Copy for CHANGER_DEVICE_PROBLEM_TYPE {}
impl ::core::clone::Clone for CHANGER_DEVICE_PROBLEM_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CHANGER_DEVICE_PROBLEM_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CHANGER_DEVICE_PROBLEM_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for CHANGER_DEVICE_PROBLEM_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CHANGER_DEVICE_PROBLEM_TYPE").field(&self.0).finish()
    }
}
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
impl ::core::fmt::Debug for CHANGER_ELEMENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CHANGER_ELEMENT").field("ElementType", &self.ElementType).field("ElementAddress", &self.ElementAddress).finish()
    }
}
unsafe impl ::windows::core::Abi for CHANGER_ELEMENT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for CHANGER_ELEMENT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CHANGER_ELEMENT>()) == 0 }
    }
}
impl ::core::cmp::Eq for CHANGER_ELEMENT {}
impl ::core::default::Default for CHANGER_ELEMENT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for CHANGER_ELEMENT_LIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CHANGER_ELEMENT_LIST").field("Element", &self.Element).field("NumberOfElements", &self.NumberOfElements).finish()
    }
}
unsafe impl ::windows::core::Abi for CHANGER_ELEMENT_LIST {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for CHANGER_ELEMENT_LIST {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CHANGER_ELEMENT_LIST>()) == 0 }
    }
}
impl ::core::cmp::Eq for CHANGER_ELEMENT_LIST {}
impl ::core::default::Default for CHANGER_ELEMENT_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for CHANGER_ELEMENT_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CHANGER_ELEMENT_STATUS").field("Element", &self.Element).field("SrcElementAddress", &self.SrcElementAddress).field("Flags", &self.Flags).field("ExceptionCode", &self.ExceptionCode).field("TargetId", &self.TargetId).field("Lun", &self.Lun).field("Reserved", &self.Reserved).field("PrimaryVolumeID", &self.PrimaryVolumeID).field("AlternateVolumeID", &self.AlternateVolumeID).finish()
    }
}
unsafe impl ::windows::core::Abi for CHANGER_ELEMENT_STATUS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for CHANGER_ELEMENT_STATUS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CHANGER_ELEMENT_STATUS>()) == 0 }
    }
}
impl ::core::cmp::Eq for CHANGER_ELEMENT_STATUS {}
impl ::core::default::Default for CHANGER_ELEMENT_STATUS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for CHANGER_ELEMENT_STATUS_EX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CHANGER_ELEMENT_STATUS_EX")
            .field("Element", &self.Element)
            .field("SrcElementAddress", &self.SrcElementAddress)
            .field("Flags", &self.Flags)
            .field("ExceptionCode", &self.ExceptionCode)
            .field("TargetId", &self.TargetId)
            .field("Lun", &self.Lun)
            .field("Reserved", &self.Reserved)
            .field("PrimaryVolumeID", &self.PrimaryVolumeID)
            .field("AlternateVolumeID", &self.AlternateVolumeID)
            .field("VendorIdentification", &self.VendorIdentification)
            .field("ProductIdentification", &self.ProductIdentification)
            .field("SerialNumber", &self.SerialNumber)
            .finish()
    }
}
unsafe impl ::windows::core::Abi for CHANGER_ELEMENT_STATUS_EX {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for CHANGER_ELEMENT_STATUS_EX {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CHANGER_ELEMENT_STATUS_EX>()) == 0 }
    }
}
impl ::core::cmp::Eq for CHANGER_ELEMENT_STATUS_EX {}
impl ::core::default::Default for CHANGER_ELEMENT_STATUS_EX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct CHANGER_ELEMENT_STATUS_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ELEMENT_STATUS_ACCESS: CHANGER_ELEMENT_STATUS_FLAGS = CHANGER_ELEMENT_STATUS_FLAGS(8u32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ELEMENT_STATUS_AVOLTAG: CHANGER_ELEMENT_STATUS_FLAGS = CHANGER_ELEMENT_STATUS_FLAGS(536870912u32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ELEMENT_STATUS_EXCEPT: CHANGER_ELEMENT_STATUS_FLAGS = CHANGER_ELEMENT_STATUS_FLAGS(4u32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ELEMENT_STATUS_EXENAB: CHANGER_ELEMENT_STATUS_FLAGS = CHANGER_ELEMENT_STATUS_FLAGS(16u32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ELEMENT_STATUS_FULL: CHANGER_ELEMENT_STATUS_FLAGS = CHANGER_ELEMENT_STATUS_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ELEMENT_STATUS_ID_VALID: CHANGER_ELEMENT_STATUS_FLAGS = CHANGER_ELEMENT_STATUS_FLAGS(8192u32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ELEMENT_STATUS_IMPEXP: CHANGER_ELEMENT_STATUS_FLAGS = CHANGER_ELEMENT_STATUS_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ELEMENT_STATUS_INENAB: CHANGER_ELEMENT_STATUS_FLAGS = CHANGER_ELEMENT_STATUS_FLAGS(32u32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ELEMENT_STATUS_INVERT: CHANGER_ELEMENT_STATUS_FLAGS = CHANGER_ELEMENT_STATUS_FLAGS(4194304u32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ELEMENT_STATUS_LUN_VALID: CHANGER_ELEMENT_STATUS_FLAGS = CHANGER_ELEMENT_STATUS_FLAGS(4096u32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ELEMENT_STATUS_NOT_BUS: CHANGER_ELEMENT_STATUS_FLAGS = CHANGER_ELEMENT_STATUS_FLAGS(32768u32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ELEMENT_STATUS_PVOLTAG: CHANGER_ELEMENT_STATUS_FLAGS = CHANGER_ELEMENT_STATUS_FLAGS(268435456u32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ELEMENT_STATUS_SVALID: CHANGER_ELEMENT_STATUS_FLAGS = CHANGER_ELEMENT_STATUS_FLAGS(8388608u32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ELEMENT_STATUS_PRODUCT_DATA: CHANGER_ELEMENT_STATUS_FLAGS = CHANGER_ELEMENT_STATUS_FLAGS(64u32);
impl ::core::marker::Copy for CHANGER_ELEMENT_STATUS_FLAGS {}
impl ::core::clone::Clone for CHANGER_ELEMENT_STATUS_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CHANGER_ELEMENT_STATUS_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CHANGER_ELEMENT_STATUS_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for CHANGER_ELEMENT_STATUS_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CHANGER_ELEMENT_STATUS_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for CHANGER_ELEMENT_STATUS_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for CHANGER_ELEMENT_STATUS_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for CHANGER_ELEMENT_STATUS_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for CHANGER_ELEMENT_STATUS_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for CHANGER_ELEMENT_STATUS_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
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
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for CHANGER_EXCHANGE_MEDIUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CHANGER_EXCHANGE_MEDIUM").field("Transport", &self.Transport).field("Source", &self.Source).field("Destination1", &self.Destination1).field("Destination2", &self.Destination2).field("Flip1", &self.Flip1).field("Flip2", &self.Flip2).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for CHANGER_EXCHANGE_MEDIUM {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CHANGER_EXCHANGE_MEDIUM {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CHANGER_EXCHANGE_MEDIUM>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CHANGER_EXCHANGE_MEDIUM {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CHANGER_EXCHANGE_MEDIUM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct CHANGER_FEATURES(pub u32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const CHANGER_BAR_CODE_SCANNER_INSTALLED: CHANGER_FEATURES = CHANGER_FEATURES(1u32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const CHANGER_CARTRIDGE_MAGAZINE: CHANGER_FEATURES = CHANGER_FEATURES(256u32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const CHANGER_CLEANER_ACCESS_NOT_VALID: CHANGER_FEATURES = CHANGER_FEATURES(262144u32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const CHANGER_CLEANER_SLOT: CHANGER_FEATURES = CHANGER_FEATURES(64u32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const CHANGER_CLOSE_IEPORT: CHANGER_FEATURES = CHANGER_FEATURES(4u32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const CHANGER_DEVICE_REINITIALIZE_CAPABLE: CHANGER_FEATURES = CHANGER_FEATURES(134217728u32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const CHANGER_DRIVE_CLEANING_REQUIRED: CHANGER_FEATURES = CHANGER_FEATURES(65536u32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const CHANGER_DRIVE_EMPTY_ON_DOOR_ACCESS: CHANGER_FEATURES = CHANGER_FEATURES(536870912u32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const CHANGER_EXCHANGE_MEDIA: CHANGER_FEATURES = CHANGER_FEATURES(32u32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const CHANGER_INIT_ELEM_STAT_WITH_RANGE: CHANGER_FEATURES = CHANGER_FEATURES(2u32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const CHANGER_KEYPAD_ENABLE_DISABLE: CHANGER_FEATURES = CHANGER_FEATURES(268435456u32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const CHANGER_LOCK_UNLOCK: CHANGER_FEATURES = CHANGER_FEATURES(128u32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const CHANGER_MEDIUM_FLIP: CHANGER_FEATURES = CHANGER_FEATURES(512u32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const CHANGER_OPEN_IEPORT: CHANGER_FEATURES = CHANGER_FEATURES(8u32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const CHANGER_POSITION_TO_ELEMENT: CHANGER_FEATURES = CHANGER_FEATURES(1024u32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const CHANGER_PREDISMOUNT_EJECT_REQUIRED: CHANGER_FEATURES = CHANGER_FEATURES(131072u32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const CHANGER_PREMOUNT_EJECT_REQUIRED: CHANGER_FEATURES = CHANGER_FEATURES(524288u32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const CHANGER_REPORT_IEPORT_STATE: CHANGER_FEATURES = CHANGER_FEATURES(2048u32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const CHANGER_SERIAL_NUMBER_VALID: CHANGER_FEATURES = CHANGER_FEATURES(67108864u32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const CHANGER_STATUS_NON_VOLATILE: CHANGER_FEATURES = CHANGER_FEATURES(16u32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const CHANGER_STORAGE_DRIVE: CHANGER_FEATURES = CHANGER_FEATURES(4096u32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const CHANGER_STORAGE_IEPORT: CHANGER_FEATURES = CHANGER_FEATURES(8192u32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const CHANGER_STORAGE_SLOT: CHANGER_FEATURES = CHANGER_FEATURES(16384u32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const CHANGER_STORAGE_TRANSPORT: CHANGER_FEATURES = CHANGER_FEATURES(32768u32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const CHANGER_VOLUME_ASSERT: CHANGER_FEATURES = CHANGER_FEATURES(4194304u32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const CHANGER_VOLUME_IDENTIFICATION: CHANGER_FEATURES = CHANGER_FEATURES(1048576u32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const CHANGER_VOLUME_REPLACE: CHANGER_FEATURES = CHANGER_FEATURES(8388608u32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const CHANGER_VOLUME_SEARCH: CHANGER_FEATURES = CHANGER_FEATURES(2097152u32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const CHANGER_VOLUME_UNDEFINE: CHANGER_FEATURES = CHANGER_FEATURES(16777216u32);
impl ::core::marker::Copy for CHANGER_FEATURES {}
impl ::core::clone::Clone for CHANGER_FEATURES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CHANGER_FEATURES {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CHANGER_FEATURES {
    type Abi = Self;
}
impl ::core::fmt::Debug for CHANGER_FEATURES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CHANGER_FEATURES").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for CHANGER_FEATURES {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for CHANGER_FEATURES {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for CHANGER_FEATURES {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for CHANGER_FEATURES {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for CHANGER_FEATURES {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
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
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for CHANGER_INITIALIZE_ELEMENT_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CHANGER_INITIALIZE_ELEMENT_STATUS").field("ElementList", &self.ElementList).field("BarCodeScan", &self.BarCodeScan).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for CHANGER_INITIALIZE_ELEMENT_STATUS {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CHANGER_INITIALIZE_ELEMENT_STATUS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CHANGER_INITIALIZE_ELEMENT_STATUS>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CHANGER_INITIALIZE_ELEMENT_STATUS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CHANGER_INITIALIZE_ELEMENT_STATUS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for CHANGER_MOVE_MEDIUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CHANGER_MOVE_MEDIUM").field("Transport", &self.Transport).field("Source", &self.Source).field("Destination", &self.Destination).field("Flip", &self.Flip).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for CHANGER_MOVE_MEDIUM {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CHANGER_MOVE_MEDIUM {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CHANGER_MOVE_MEDIUM>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CHANGER_MOVE_MEDIUM {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CHANGER_MOVE_MEDIUM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for CHANGER_PRODUCT_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CHANGER_PRODUCT_DATA").field("VendorId", &self.VendorId).field("ProductId", &self.ProductId).field("Revision", &self.Revision).field("SerialNumber", &self.SerialNumber).field("DeviceType", &self.DeviceType).finish()
    }
}
unsafe impl ::windows::core::Abi for CHANGER_PRODUCT_DATA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for CHANGER_PRODUCT_DATA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CHANGER_PRODUCT_DATA>()) == 0 }
    }
}
impl ::core::cmp::Eq for CHANGER_PRODUCT_DATA {}
impl ::core::default::Default for CHANGER_PRODUCT_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for CHANGER_READ_ELEMENT_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CHANGER_READ_ELEMENT_STATUS").field("ElementList", &self.ElementList).field("VolumeTagInfo", &self.VolumeTagInfo).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for CHANGER_READ_ELEMENT_STATUS {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CHANGER_READ_ELEMENT_STATUS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CHANGER_READ_ELEMENT_STATUS>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CHANGER_READ_ELEMENT_STATUS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CHANGER_READ_ELEMENT_STATUS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for CHANGER_SEND_VOLUME_TAG_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CHANGER_SEND_VOLUME_TAG_INFORMATION").field("StartingElement", &self.StartingElement).field("ActionCode", &self.ActionCode).field("VolumeIDTemplate", &self.VolumeIDTemplate).finish()
    }
}
unsafe impl ::windows::core::Abi for CHANGER_SEND_VOLUME_TAG_INFORMATION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for CHANGER_SEND_VOLUME_TAG_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CHANGER_SEND_VOLUME_TAG_INFORMATION>()) == 0 }
    }
}
impl ::core::cmp::Eq for CHANGER_SEND_VOLUME_TAG_INFORMATION {}
impl ::core::default::Default for CHANGER_SEND_VOLUME_TAG_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for CHANGER_SET_ACCESS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CHANGER_SET_ACCESS").field("Element", &self.Element).field("Control", &self.Control).finish()
    }
}
unsafe impl ::windows::core::Abi for CHANGER_SET_ACCESS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for CHANGER_SET_ACCESS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CHANGER_SET_ACCESS>()) == 0 }
    }
}
impl ::core::cmp::Eq for CHANGER_SET_ACCESS {}
impl ::core::default::Default for CHANGER_SET_ACCESS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for CHANGER_SET_POSITION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CHANGER_SET_POSITION").field("Transport", &self.Transport).field("Destination", &self.Destination).field("Flip", &self.Flip).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for CHANGER_SET_POSITION {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CHANGER_SET_POSITION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CHANGER_SET_POSITION>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CHANGER_SET_POSITION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CHANGER_SET_POSITION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for CLASS_MEDIA_CHANGE_CONTEXT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CLASS_MEDIA_CHANGE_CONTEXT").field("MediaChangeCount", &self.MediaChangeCount).field("NewState", &self.NewState).finish()
    }
}
unsafe impl ::windows::core::Abi for CLASS_MEDIA_CHANGE_CONTEXT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for CLASS_MEDIA_CHANGE_CONTEXT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CLASS_MEDIA_CHANGE_CONTEXT>()) == 0 }
    }
}
impl ::core::cmp::Eq for CLASS_MEDIA_CHANGE_CONTEXT {}
impl ::core::default::Default for CLASS_MEDIA_CHANGE_CONTEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for CLUSTER_RANGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CLUSTER_RANGE").field("StartingCluster", &self.StartingCluster).field("ClusterCount", &self.ClusterCount).finish()
    }
}
unsafe impl ::windows::core::Abi for CLUSTER_RANGE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for CLUSTER_RANGE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CLUSTER_RANGE>()) == 0 }
    }
}
impl ::core::cmp::Eq for CLUSTER_RANGE {}
impl ::core::default::Default for CLUSTER_RANGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for CONTAINER_ROOT_INFO_INPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CONTAINER_ROOT_INFO_INPUT").field("Flags", &self.Flags).finish()
    }
}
unsafe impl ::windows::core::Abi for CONTAINER_ROOT_INFO_INPUT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for CONTAINER_ROOT_INFO_INPUT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CONTAINER_ROOT_INFO_INPUT>()) == 0 }
    }
}
impl ::core::cmp::Eq for CONTAINER_ROOT_INFO_INPUT {}
impl ::core::default::Default for CONTAINER_ROOT_INFO_INPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for CONTAINER_ROOT_INFO_OUTPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CONTAINER_ROOT_INFO_OUTPUT").field("ContainerRootIdLength", &self.ContainerRootIdLength).field("ContainerRootId", &self.ContainerRootId).finish()
    }
}
unsafe impl ::windows::core::Abi for CONTAINER_ROOT_INFO_OUTPUT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for CONTAINER_ROOT_INFO_OUTPUT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CONTAINER_ROOT_INFO_OUTPUT>()) == 0 }
    }
}
impl ::core::cmp::Eq for CONTAINER_ROOT_INFO_OUTPUT {}
impl ::core::default::Default for CONTAINER_ROOT_INFO_OUTPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for CONTAINER_VOLUME_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CONTAINER_VOLUME_STATE").field("Flags", &self.Flags).finish()
    }
}
unsafe impl ::windows::core::Abi for CONTAINER_VOLUME_STATE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for CONTAINER_VOLUME_STATE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CONTAINER_VOLUME_STATE>()) == 0 }
    }
}
impl ::core::cmp::Eq for CONTAINER_VOLUME_STATE {}
impl ::core::default::Default for CONTAINER_VOLUME_STATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
unsafe impl ::windows::core::Abi for CREATE_DISK {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for CREATE_DISK {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CREATE_DISK>()) == 0 }
    }
}
impl ::core::cmp::Eq for CREATE_DISK {}
impl ::core::default::Default for CREATE_DISK {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
unsafe impl ::windows::core::Abi for CREATE_DISK_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for CREATE_DISK_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CREATE_DISK_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for CREATE_DISK_0 {}
impl ::core::default::Default for CREATE_DISK_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct CREATE_DISK_GPT {
    pub DiskId: ::windows::core::GUID,
    pub MaxPartitionCount: u32,
}
impl ::core::marker::Copy for CREATE_DISK_GPT {}
impl ::core::clone::Clone for CREATE_DISK_GPT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CREATE_DISK_GPT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CREATE_DISK_GPT").field("DiskId", &self.DiskId).field("MaxPartitionCount", &self.MaxPartitionCount).finish()
    }
}
unsafe impl ::windows::core::Abi for CREATE_DISK_GPT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for CREATE_DISK_GPT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CREATE_DISK_GPT>()) == 0 }
    }
}
impl ::core::cmp::Eq for CREATE_DISK_GPT {}
impl ::core::default::Default for CREATE_DISK_GPT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for CREATE_DISK_MBR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CREATE_DISK_MBR").field("Signature", &self.Signature).finish()
    }
}
unsafe impl ::windows::core::Abi for CREATE_DISK_MBR {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for CREATE_DISK_MBR {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CREATE_DISK_MBR>()) == 0 }
    }
}
impl ::core::cmp::Eq for CREATE_DISK_MBR {}
impl ::core::default::Default for CREATE_DISK_MBR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for CREATE_USN_JOURNAL_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CREATE_USN_JOURNAL_DATA").field("MaximumSize", &self.MaximumSize).field("AllocationDelta", &self.AllocationDelta).finish()
    }
}
unsafe impl ::windows::core::Abi for CREATE_USN_JOURNAL_DATA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for CREATE_USN_JOURNAL_DATA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CREATE_USN_JOURNAL_DATA>()) == 0 }
    }
}
impl ::core::cmp::Eq for CREATE_USN_JOURNAL_DATA {}
impl ::core::default::Default for CREATE_USN_JOURNAL_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct CSVFS_DISK_CONNECTIVITY(pub i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const CsvFsDiskConnectivityNone: CSVFS_DISK_CONNECTIVITY = CSVFS_DISK_CONNECTIVITY(0i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const CsvFsDiskConnectivityMdsNodeOnly: CSVFS_DISK_CONNECTIVITY = CSVFS_DISK_CONNECTIVITY(1i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const CsvFsDiskConnectivitySubsetOfNodes: CSVFS_DISK_CONNECTIVITY = CSVFS_DISK_CONNECTIVITY(2i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const CsvFsDiskConnectivityAllNodes: CSVFS_DISK_CONNECTIVITY = CSVFS_DISK_CONNECTIVITY(3i32);
impl ::core::marker::Copy for CSVFS_DISK_CONNECTIVITY {}
impl ::core::clone::Clone for CSVFS_DISK_CONNECTIVITY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CSVFS_DISK_CONNECTIVITY {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CSVFS_DISK_CONNECTIVITY {
    type Abi = Self;
}
impl ::core::fmt::Debug for CSVFS_DISK_CONNECTIVITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CSVFS_DISK_CONNECTIVITY").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct CSV_CONTROL_OP(pub i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const CsvControlStartRedirectFile: CSV_CONTROL_OP = CSV_CONTROL_OP(2i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const CsvControlStopRedirectFile: CSV_CONTROL_OP = CSV_CONTROL_OP(3i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const CsvControlQueryRedirectState: CSV_CONTROL_OP = CSV_CONTROL_OP(4i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const CsvControlQueryFileRevision: CSV_CONTROL_OP = CSV_CONTROL_OP(6i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const CsvControlQueryMdsPath: CSV_CONTROL_OP = CSV_CONTROL_OP(8i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const CsvControlQueryFileRevisionFileId128: CSV_CONTROL_OP = CSV_CONTROL_OP(9i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const CsvControlQueryVolumeRedirectState: CSV_CONTROL_OP = CSV_CONTROL_OP(10i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const CsvControlEnableUSNRangeModificationTracking: CSV_CONTROL_OP = CSV_CONTROL_OP(13i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const CsvControlMarkHandleLocalVolumeMount: CSV_CONTROL_OP = CSV_CONTROL_OP(14i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const CsvControlUnmarkHandleLocalVolumeMount: CSV_CONTROL_OP = CSV_CONTROL_OP(15i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const CsvControlGetCsvFsMdsPathV2: CSV_CONTROL_OP = CSV_CONTROL_OP(18i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const CsvControlDisableCaching: CSV_CONTROL_OP = CSV_CONTROL_OP(19i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const CsvControlEnableCaching: CSV_CONTROL_OP = CSV_CONTROL_OP(20i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const CsvControlStartForceDFO: CSV_CONTROL_OP = CSV_CONTROL_OP(21i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const CsvControlStopForceDFO: CSV_CONTROL_OP = CSV_CONTROL_OP(22i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const CsvControlQueryMdsPathNoPause: CSV_CONTROL_OP = CSV_CONTROL_OP(23i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const CsvControlSetVolumeId: CSV_CONTROL_OP = CSV_CONTROL_OP(24i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const CsvControlQueryVolumeId: CSV_CONTROL_OP = CSV_CONTROL_OP(25i32);
impl ::core::marker::Copy for CSV_CONTROL_OP {}
impl ::core::clone::Clone for CSV_CONTROL_OP {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CSV_CONTROL_OP {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CSV_CONTROL_OP {
    type Abi = Self;
}
impl ::core::fmt::Debug for CSV_CONTROL_OP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CSV_CONTROL_OP").field(&self.0).finish()
    }
}
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
impl ::core::fmt::Debug for CSV_CONTROL_PARAM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CSV_CONTROL_PARAM").field("Operation", &self.Operation).field("Unused", &self.Unused).finish()
    }
}
unsafe impl ::windows::core::Abi for CSV_CONTROL_PARAM {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for CSV_CONTROL_PARAM {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CSV_CONTROL_PARAM>()) == 0 }
    }
}
impl ::core::cmp::Eq for CSV_CONTROL_PARAM {}
impl ::core::default::Default for CSV_CONTROL_PARAM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for CSV_IS_OWNED_BY_CSVFS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CSV_IS_OWNED_BY_CSVFS").field("OwnedByCSVFS", &self.OwnedByCSVFS).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for CSV_IS_OWNED_BY_CSVFS {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CSV_IS_OWNED_BY_CSVFS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CSV_IS_OWNED_BY_CSVFS>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CSV_IS_OWNED_BY_CSVFS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CSV_IS_OWNED_BY_CSVFS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for CSV_MGMT_LOCK {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CSV_MGMT_LOCK").field("Flags", &self.Flags).finish()
    }
}
unsafe impl ::windows::core::Abi for CSV_MGMT_LOCK {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for CSV_MGMT_LOCK {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CSV_MGMT_LOCK>()) == 0 }
    }
}
impl ::core::cmp::Eq for CSV_MGMT_LOCK {}
impl ::core::default::Default for CSV_MGMT_LOCK {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for CSV_NAMESPACE_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CSV_NAMESPACE_INFO").field("Version", &self.Version).field("DeviceNumber", &self.DeviceNumber).field("StartingOffset", &self.StartingOffset).field("SectorSize", &self.SectorSize).finish()
    }
}
unsafe impl ::windows::core::Abi for CSV_NAMESPACE_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for CSV_NAMESPACE_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CSV_NAMESPACE_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for CSV_NAMESPACE_INFO {}
impl ::core::default::Default for CSV_NAMESPACE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for CSV_QUERY_FILE_REVISION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CSV_QUERY_FILE_REVISION").field("FileId", &self.FileId).field("FileRevision", &self.FileRevision).finish()
    }
}
unsafe impl ::windows::core::Abi for CSV_QUERY_FILE_REVISION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for CSV_QUERY_FILE_REVISION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CSV_QUERY_FILE_REVISION>()) == 0 }
    }
}
impl ::core::cmp::Eq for CSV_QUERY_FILE_REVISION {}
impl ::core::default::Default for CSV_QUERY_FILE_REVISION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
#[cfg(feature = "Win32_Storage_FileSystem")]
impl ::core::fmt::Debug for CSV_QUERY_FILE_REVISION_FILE_ID_128 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CSV_QUERY_FILE_REVISION_FILE_ID_128").field("FileId", &self.FileId).field("FileRevision", &self.FileRevision).finish()
    }
}
#[cfg(feature = "Win32_Storage_FileSystem")]
unsafe impl ::windows::core::Abi for CSV_QUERY_FILE_REVISION_FILE_ID_128 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Storage_FileSystem")]
impl ::core::cmp::PartialEq for CSV_QUERY_FILE_REVISION_FILE_ID_128 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CSV_QUERY_FILE_REVISION_FILE_ID_128>()) == 0 }
    }
}
#[cfg(feature = "Win32_Storage_FileSystem")]
impl ::core::cmp::Eq for CSV_QUERY_FILE_REVISION_FILE_ID_128 {}
#[cfg(feature = "Win32_Storage_FileSystem")]
impl ::core::default::Default for CSV_QUERY_FILE_REVISION_FILE_ID_128 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for CSV_QUERY_MDS_PATH {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CSV_QUERY_MDS_PATH").field("MdsNodeId", &self.MdsNodeId).field("DsNodeId", &self.DsNodeId).field("PathLength", &self.PathLength).field("Path", &self.Path).finish()
    }
}
unsafe impl ::windows::core::Abi for CSV_QUERY_MDS_PATH {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for CSV_QUERY_MDS_PATH {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CSV_QUERY_MDS_PATH>()) == 0 }
    }
}
impl ::core::cmp::Eq for CSV_QUERY_MDS_PATH {}
impl ::core::default::Default for CSV_QUERY_MDS_PATH {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
    pub VolumeId: ::windows::core::GUID,
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
impl ::core::fmt::Debug for CSV_QUERY_MDS_PATH_V2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CSV_QUERY_MDS_PATH_V2").field("Version", &self.Version).field("RequiredSize", &self.RequiredSize).field("MdsNodeId", &self.MdsNodeId).field("DsNodeId", &self.DsNodeId).field("Flags", &self.Flags).field("DiskConnectivity", &self.DiskConnectivity).field("VolumeId", &self.VolumeId).field("IpAddressOffset", &self.IpAddressOffset).field("IpAddressLength", &self.IpAddressLength).field("PathOffset", &self.PathOffset).field("PathLength", &self.PathLength).finish()
    }
}
unsafe impl ::windows::core::Abi for CSV_QUERY_MDS_PATH_V2 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for CSV_QUERY_MDS_PATH_V2 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CSV_QUERY_MDS_PATH_V2>()) == 0 }
    }
}
impl ::core::cmp::Eq for CSV_QUERY_MDS_PATH_V2 {}
impl ::core::default::Default for CSV_QUERY_MDS_PATH_V2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for CSV_QUERY_REDIRECT_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CSV_QUERY_REDIRECT_STATE").field("MdsNodeId", &self.MdsNodeId).field("DsNodeId", &self.DsNodeId).field("FileRedirected", &self.FileRedirected).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for CSV_QUERY_REDIRECT_STATE {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CSV_QUERY_REDIRECT_STATE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CSV_QUERY_REDIRECT_STATE>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CSV_QUERY_REDIRECT_STATE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CSV_QUERY_REDIRECT_STATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for CSV_QUERY_VETO_FILE_DIRECT_IO_OUTPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CSV_QUERY_VETO_FILE_DIRECT_IO_OUTPUT").field("VetoedFromAltitudeIntegral", &self.VetoedFromAltitudeIntegral).field("VetoedFromAltitudeDecimal", &self.VetoedFromAltitudeDecimal).field("Reason", &self.Reason).finish()
    }
}
unsafe impl ::windows::core::Abi for CSV_QUERY_VETO_FILE_DIRECT_IO_OUTPUT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for CSV_QUERY_VETO_FILE_DIRECT_IO_OUTPUT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CSV_QUERY_VETO_FILE_DIRECT_IO_OUTPUT>()) == 0 }
    }
}
impl ::core::cmp::Eq for CSV_QUERY_VETO_FILE_DIRECT_IO_OUTPUT {}
impl ::core::default::Default for CSV_QUERY_VETO_FILE_DIRECT_IO_OUTPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct CSV_QUERY_VOLUME_ID {
    pub VolumeId: ::windows::core::GUID,
}
impl ::core::marker::Copy for CSV_QUERY_VOLUME_ID {}
impl ::core::clone::Clone for CSV_QUERY_VOLUME_ID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CSV_QUERY_VOLUME_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CSV_QUERY_VOLUME_ID").field("VolumeId", &self.VolumeId).finish()
    }
}
unsafe impl ::windows::core::Abi for CSV_QUERY_VOLUME_ID {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for CSV_QUERY_VOLUME_ID {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CSV_QUERY_VOLUME_ID>()) == 0 }
    }
}
impl ::core::cmp::Eq for CSV_QUERY_VOLUME_ID {}
impl ::core::default::Default for CSV_QUERY_VOLUME_ID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for CSV_QUERY_VOLUME_REDIRECT_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CSV_QUERY_VOLUME_REDIRECT_STATE").field("MdsNodeId", &self.MdsNodeId).field("DsNodeId", &self.DsNodeId).field("IsDiskConnected", &self.IsDiskConnected).field("ClusterEnableDirectIo", &self.ClusterEnableDirectIo).field("DiskConnectivity", &self.DiskConnectivity).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for CSV_QUERY_VOLUME_REDIRECT_STATE {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CSV_QUERY_VOLUME_REDIRECT_STATE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CSV_QUERY_VOLUME_REDIRECT_STATE>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CSV_QUERY_VOLUME_REDIRECT_STATE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CSV_QUERY_VOLUME_REDIRECT_STATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct CSV_SET_VOLUME_ID {
    pub VolumeId: ::windows::core::GUID,
}
impl ::core::marker::Copy for CSV_SET_VOLUME_ID {}
impl ::core::clone::Clone for CSV_SET_VOLUME_ID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CSV_SET_VOLUME_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CSV_SET_VOLUME_ID").field("VolumeId", &self.VolumeId).finish()
    }
}
unsafe impl ::windows::core::Abi for CSV_SET_VOLUME_ID {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for CSV_SET_VOLUME_ID {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CSV_SET_VOLUME_ID>()) == 0 }
    }
}
impl ::core::cmp::Eq for CSV_SET_VOLUME_ID {}
impl ::core::default::Default for CSV_SET_VOLUME_ID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DECRYPTION_STATUS_BUFFER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DECRYPTION_STATUS_BUFFER").field("NoEncryptedStreams", &self.NoEncryptedStreams).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DECRYPTION_STATUS_BUFFER {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DECRYPTION_STATUS_BUFFER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DECRYPTION_STATUS_BUFFER>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DECRYPTION_STATUS_BUFFER {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DECRYPTION_STATUS_BUFFER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for DELETE_USN_JOURNAL_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DELETE_USN_JOURNAL_DATA").field("UsnJournalID", &self.UsnJournalID).field("DeleteFlags", &self.DeleteFlags).finish()
    }
}
unsafe impl ::windows::core::Abi for DELETE_USN_JOURNAL_DATA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DELETE_USN_JOURNAL_DATA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DELETE_USN_JOURNAL_DATA>()) == 0 }
    }
}
impl ::core::cmp::Eq for DELETE_USN_JOURNAL_DATA {}
impl ::core::default::Default for DELETE_USN_JOURNAL_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DETECTION_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const DetectNone: DETECTION_TYPE = DETECTION_TYPE(0i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const DetectInt13: DETECTION_TYPE = DETECTION_TYPE(1i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const DetectExInt13: DETECTION_TYPE = DETECTION_TYPE(2i32);
impl ::core::marker::Copy for DETECTION_TYPE {}
impl ::core::clone::Clone for DETECTION_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DETECTION_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DETECTION_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for DETECTION_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DETECTION_TYPE").field(&self.0).finish()
    }
}
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
unsafe impl ::windows::core::Abi for DEVICEDUMP_PRIVATE_SUBSECTION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DEVICEDUMP_PRIVATE_SUBSECTION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DEVICEDUMP_PRIVATE_SUBSECTION>()) == 0 }
    }
}
impl ::core::cmp::Eq for DEVICEDUMP_PRIVATE_SUBSECTION {}
impl ::core::default::Default for DEVICEDUMP_PRIVATE_SUBSECTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DEVICEDUMP_PUBLIC_SUBSECTION {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DEVICEDUMP_PUBLIC_SUBSECTION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DEVICEDUMP_PUBLIC_SUBSECTION>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DEVICEDUMP_PUBLIC_SUBSECTION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DEVICEDUMP_PUBLIC_SUBSECTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for DEVICEDUMP_RESTRICTED_SUBSECTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DEVICEDUMP_RESTRICTED_SUBSECTION").field("bData", &self.bData).finish()
    }
}
unsafe impl ::windows::core::Abi for DEVICEDUMP_RESTRICTED_SUBSECTION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DEVICEDUMP_RESTRICTED_SUBSECTION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DEVICEDUMP_RESTRICTED_SUBSECTION>()) == 0 }
    }
}
impl ::core::cmp::Eq for DEVICEDUMP_RESTRICTED_SUBSECTION {}
impl ::core::default::Default for DEVICEDUMP_RESTRICTED_SUBSECTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct DEVICEDUMP_SECTION_HEADER {
    pub guidDeviceDataId: ::windows::core::GUID,
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
unsafe impl ::windows::core::Abi for DEVICEDUMP_SECTION_HEADER {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DEVICEDUMP_SECTION_HEADER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DEVICEDUMP_SECTION_HEADER>()) == 0 }
    }
}
impl ::core::cmp::Eq for DEVICEDUMP_SECTION_HEADER {}
impl ::core::default::Default for DEVICEDUMP_SECTION_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
unsafe impl ::windows::core::Abi for DEVICEDUMP_STORAGEDEVICE_DATA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DEVICEDUMP_STORAGEDEVICE_DATA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DEVICEDUMP_STORAGEDEVICE_DATA>()) == 0 }
    }
}
impl ::core::cmp::Eq for DEVICEDUMP_STORAGEDEVICE_DATA {}
impl ::core::default::Default for DEVICEDUMP_STORAGEDEVICE_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
unsafe impl ::windows::core::Abi for DEVICEDUMP_STORAGESTACK_PUBLIC_DUMP {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DEVICEDUMP_STORAGESTACK_PUBLIC_DUMP {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DEVICEDUMP_STORAGESTACK_PUBLIC_DUMP>()) == 0 }
    }
}
impl ::core::cmp::Eq for DEVICEDUMP_STORAGESTACK_PUBLIC_DUMP {}
impl ::core::default::Default for DEVICEDUMP_STORAGESTACK_PUBLIC_DUMP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
unsafe impl ::windows::core::Abi for DEVICEDUMP_STORAGESTACK_PUBLIC_STATE_RECORD {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DEVICEDUMP_STORAGESTACK_PUBLIC_STATE_RECORD {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DEVICEDUMP_STORAGESTACK_PUBLIC_STATE_RECORD>()) == 0 }
    }
}
impl ::core::cmp::Eq for DEVICEDUMP_STORAGESTACK_PUBLIC_STATE_RECORD {}
impl ::core::default::Default for DEVICEDUMP_STORAGESTACK_PUBLIC_STATE_RECORD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
unsafe impl ::windows::core::Abi for DEVICEDUMP_STORAGESTACK_PUBLIC_STATE_RECORD_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DEVICEDUMP_STORAGESTACK_PUBLIC_STATE_RECORD_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DEVICEDUMP_STORAGESTACK_PUBLIC_STATE_RECORD_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for DEVICEDUMP_STORAGESTACK_PUBLIC_STATE_RECORD_0 {}
impl ::core::default::Default for DEVICEDUMP_STORAGESTACK_PUBLIC_STATE_RECORD_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
unsafe impl ::windows::core::Abi for DEVICEDUMP_STORAGESTACK_PUBLIC_STATE_RECORD_0_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DEVICEDUMP_STORAGESTACK_PUBLIC_STATE_RECORD_0_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DEVICEDUMP_STORAGESTACK_PUBLIC_STATE_RECORD_0_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for DEVICEDUMP_STORAGESTACK_PUBLIC_STATE_RECORD_0_0 {}
impl ::core::default::Default for DEVICEDUMP_STORAGESTACK_PUBLIC_STATE_RECORD_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
unsafe impl ::windows::core::Abi for DEVICEDUMP_STORAGESTACK_PUBLIC_STATE_RECORD_0_1 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DEVICEDUMP_STORAGESTACK_PUBLIC_STATE_RECORD_0_1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DEVICEDUMP_STORAGESTACK_PUBLIC_STATE_RECORD_0_1>()) == 0 }
    }
}
impl ::core::cmp::Eq for DEVICEDUMP_STORAGESTACK_PUBLIC_STATE_RECORD_0_1 {}
impl ::core::default::Default for DEVICEDUMP_STORAGESTACK_PUBLIC_STATE_RECORD_0_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
unsafe impl ::windows::core::Abi for DEVICEDUMP_STORAGESTACK_PUBLIC_STATE_RECORD_0_2 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DEVICEDUMP_STORAGESTACK_PUBLIC_STATE_RECORD_0_2 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DEVICEDUMP_STORAGESTACK_PUBLIC_STATE_RECORD_0_2>()) == 0 }
    }
}
impl ::core::cmp::Eq for DEVICEDUMP_STORAGESTACK_PUBLIC_STATE_RECORD_0_2 {}
impl ::core::default::Default for DEVICEDUMP_STORAGESTACK_PUBLIC_STATE_RECORD_0_2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
unsafe impl ::windows::core::Abi for DEVICEDUMP_STRUCTURE_VERSION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DEVICEDUMP_STRUCTURE_VERSION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DEVICEDUMP_STRUCTURE_VERSION>()) == 0 }
    }
}
impl ::core::cmp::Eq for DEVICEDUMP_STRUCTURE_VERSION {}
impl ::core::default::Default for DEVICEDUMP_STRUCTURE_VERSION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
unsafe impl ::windows::core::Abi for DEVICEDUMP_SUBSECTION_POINTER {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DEVICEDUMP_SUBSECTION_POINTER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DEVICEDUMP_SUBSECTION_POINTER>()) == 0 }
    }
}
impl ::core::cmp::Eq for DEVICEDUMP_SUBSECTION_POINTER {}
impl ::core::default::Default for DEVICEDUMP_SUBSECTION_POINTER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for DEVICE_COPY_OFFLOAD_DESCRIPTOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DEVICE_COPY_OFFLOAD_DESCRIPTOR")
            .field("Version", &self.Version)
            .field("Size", &self.Size)
            .field("MaximumTokenLifetime", &self.MaximumTokenLifetime)
            .field("DefaultTokenLifetime", &self.DefaultTokenLifetime)
            .field("MaximumTransferSize", &self.MaximumTransferSize)
            .field("OptimalTransferCount", &self.OptimalTransferCount)
            .field("MaximumDataDescriptors", &self.MaximumDataDescriptors)
            .field("MaximumTransferLengthPerDescriptor", &self.MaximumTransferLengthPerDescriptor)
            .field("OptimalTransferLengthPerDescriptor", &self.OptimalTransferLengthPerDescriptor)
            .field("OptimalTransferLengthGranularity", &self.OptimalTransferLengthGranularity)
            .field("Reserved", &self.Reserved)
            .finish()
    }
}
unsafe impl ::windows::core::Abi for DEVICE_COPY_OFFLOAD_DESCRIPTOR {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DEVICE_COPY_OFFLOAD_DESCRIPTOR {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DEVICE_COPY_OFFLOAD_DESCRIPTOR>()) == 0 }
    }
}
impl ::core::cmp::Eq for DEVICE_COPY_OFFLOAD_DESCRIPTOR {}
impl ::core::default::Default for DEVICE_COPY_OFFLOAD_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for DEVICE_DATA_SET_LBP_STATE_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DEVICE_DATA_SET_LBP_STATE_PARAMETERS").field("Version", &self.Version).field("Size", &self.Size).field("Flags", &self.Flags).field("OutputVersion", &self.OutputVersion).finish()
    }
}
unsafe impl ::windows::core::Abi for DEVICE_DATA_SET_LBP_STATE_PARAMETERS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DEVICE_DATA_SET_LBP_STATE_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DEVICE_DATA_SET_LBP_STATE_PARAMETERS>()) == 0 }
    }
}
impl ::core::cmp::Eq for DEVICE_DATA_SET_LBP_STATE_PARAMETERS {}
impl ::core::default::Default for DEVICE_DATA_SET_LBP_STATE_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for DEVICE_DATA_SET_LB_PROVISIONING_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DEVICE_DATA_SET_LB_PROVISIONING_STATE").field("Size", &self.Size).field("Version", &self.Version).field("SlabSizeInBytes", &self.SlabSizeInBytes).field("SlabOffsetDeltaInBytes", &self.SlabOffsetDeltaInBytes).field("SlabAllocationBitMapBitCount", &self.SlabAllocationBitMapBitCount).field("SlabAllocationBitMapLength", &self.SlabAllocationBitMapLength).field("SlabAllocationBitMap", &self.SlabAllocationBitMap).finish()
    }
}
unsafe impl ::windows::core::Abi for DEVICE_DATA_SET_LB_PROVISIONING_STATE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DEVICE_DATA_SET_LB_PROVISIONING_STATE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DEVICE_DATA_SET_LB_PROVISIONING_STATE>()) == 0 }
    }
}
impl ::core::cmp::Eq for DEVICE_DATA_SET_LB_PROVISIONING_STATE {}
impl ::core::default::Default for DEVICE_DATA_SET_LB_PROVISIONING_STATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for DEVICE_DATA_SET_LB_PROVISIONING_STATE_V2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DEVICE_DATA_SET_LB_PROVISIONING_STATE_V2").field("Size", &self.Size).field("Version", &self.Version).field("SlabSizeInBytes", &self.SlabSizeInBytes).field("SlabOffsetDeltaInBytes", &self.SlabOffsetDeltaInBytes).field("SlabAllocationBitMapBitCount", &self.SlabAllocationBitMapBitCount).field("SlabAllocationBitMapLength", &self.SlabAllocationBitMapLength).field("SlabAllocationBitMap", &self.SlabAllocationBitMap).finish()
    }
}
unsafe impl ::windows::core::Abi for DEVICE_DATA_SET_LB_PROVISIONING_STATE_V2 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DEVICE_DATA_SET_LB_PROVISIONING_STATE_V2 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DEVICE_DATA_SET_LB_PROVISIONING_STATE_V2>()) == 0 }
    }
}
impl ::core::cmp::Eq for DEVICE_DATA_SET_LB_PROVISIONING_STATE_V2 {}
impl ::core::default::Default for DEVICE_DATA_SET_LB_PROVISIONING_STATE_V2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for DEVICE_DATA_SET_RANGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DEVICE_DATA_SET_RANGE").field("StartingOffset", &self.StartingOffset).field("LengthInBytes", &self.LengthInBytes).finish()
    }
}
unsafe impl ::windows::core::Abi for DEVICE_DATA_SET_RANGE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DEVICE_DATA_SET_RANGE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DEVICE_DATA_SET_RANGE>()) == 0 }
    }
}
impl ::core::cmp::Eq for DEVICE_DATA_SET_RANGE {}
impl ::core::default::Default for DEVICE_DATA_SET_RANGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for DEVICE_DATA_SET_REPAIR_OUTPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DEVICE_DATA_SET_REPAIR_OUTPUT").field("ParityExtent", &self.ParityExtent).finish()
    }
}
unsafe impl ::windows::core::Abi for DEVICE_DATA_SET_REPAIR_OUTPUT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DEVICE_DATA_SET_REPAIR_OUTPUT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DEVICE_DATA_SET_REPAIR_OUTPUT>()) == 0 }
    }
}
impl ::core::cmp::Eq for DEVICE_DATA_SET_REPAIR_OUTPUT {}
impl ::core::default::Default for DEVICE_DATA_SET_REPAIR_OUTPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for DEVICE_DATA_SET_REPAIR_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DEVICE_DATA_SET_REPAIR_PARAMETERS").field("NumberOfRepairCopies", &self.NumberOfRepairCopies).field("SourceCopy", &self.SourceCopy).field("RepairCopies", &self.RepairCopies).finish()
    }
}
unsafe impl ::windows::core::Abi for DEVICE_DATA_SET_REPAIR_PARAMETERS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DEVICE_DATA_SET_REPAIR_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DEVICE_DATA_SET_REPAIR_PARAMETERS>()) == 0 }
    }
}
impl ::core::cmp::Eq for DEVICE_DATA_SET_REPAIR_PARAMETERS {}
impl ::core::default::Default for DEVICE_DATA_SET_REPAIR_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for DEVICE_DATA_SET_SCRUB_EX_OUTPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DEVICE_DATA_SET_SCRUB_EX_OUTPUT").field("BytesProcessed", &self.BytesProcessed).field("BytesRepaired", &self.BytesRepaired).field("BytesFailed", &self.BytesFailed).field("ParityExtent", &self.ParityExtent).field("BytesScrubbed", &self.BytesScrubbed).finish()
    }
}
unsafe impl ::windows::core::Abi for DEVICE_DATA_SET_SCRUB_EX_OUTPUT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DEVICE_DATA_SET_SCRUB_EX_OUTPUT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DEVICE_DATA_SET_SCRUB_EX_OUTPUT>()) == 0 }
    }
}
impl ::core::cmp::Eq for DEVICE_DATA_SET_SCRUB_EX_OUTPUT {}
impl ::core::default::Default for DEVICE_DATA_SET_SCRUB_EX_OUTPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for DEVICE_DATA_SET_SCRUB_OUTPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DEVICE_DATA_SET_SCRUB_OUTPUT").field("BytesProcessed", &self.BytesProcessed).field("BytesRepaired", &self.BytesRepaired).field("BytesFailed", &self.BytesFailed).finish()
    }
}
unsafe impl ::windows::core::Abi for DEVICE_DATA_SET_SCRUB_OUTPUT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DEVICE_DATA_SET_SCRUB_OUTPUT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DEVICE_DATA_SET_SCRUB_OUTPUT>()) == 0 }
    }
}
impl ::core::cmp::Eq for DEVICE_DATA_SET_SCRUB_OUTPUT {}
impl ::core::default::Default for DEVICE_DATA_SET_SCRUB_OUTPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for DEVICE_DATA_SET_TOPOLOGY_ID_QUERY_OUTPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DEVICE_DATA_SET_TOPOLOGY_ID_QUERY_OUTPUT").field("TopologyRangeBytes", &self.TopologyRangeBytes).field("TopologyId", &self.TopologyId).finish()
    }
}
unsafe impl ::windows::core::Abi for DEVICE_DATA_SET_TOPOLOGY_ID_QUERY_OUTPUT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DEVICE_DATA_SET_TOPOLOGY_ID_QUERY_OUTPUT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DEVICE_DATA_SET_TOPOLOGY_ID_QUERY_OUTPUT>()) == 0 }
    }
}
impl ::core::cmp::Eq for DEVICE_DATA_SET_TOPOLOGY_ID_QUERY_OUTPUT {}
impl ::core::default::Default for DEVICE_DATA_SET_TOPOLOGY_ID_QUERY_OUTPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct DEVICE_DSM_CONVERSION_OUTPUT {
    pub Version: u32,
    pub Source: ::windows::core::GUID,
}
impl ::core::marker::Copy for DEVICE_DSM_CONVERSION_OUTPUT {}
impl ::core::clone::Clone for DEVICE_DSM_CONVERSION_OUTPUT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DEVICE_DSM_CONVERSION_OUTPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DEVICE_DSM_CONVERSION_OUTPUT").field("Version", &self.Version).field("Source", &self.Source).finish()
    }
}
unsafe impl ::windows::core::Abi for DEVICE_DSM_CONVERSION_OUTPUT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DEVICE_DSM_CONVERSION_OUTPUT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DEVICE_DSM_CONVERSION_OUTPUT>()) == 0 }
    }
}
impl ::core::cmp::Eq for DEVICE_DSM_CONVERSION_OUTPUT {}
impl ::core::default::Default for DEVICE_DSM_CONVERSION_OUTPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DEVICE_DSM_DEFINITION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DEVICE_DSM_DEFINITION").field("Action", &self.Action).field("SingleRange", &self.SingleRange).field("ParameterBlockAlignment", &self.ParameterBlockAlignment).field("ParameterBlockLength", &self.ParameterBlockLength).field("HasOutput", &self.HasOutput).field("OutputBlockAlignment", &self.OutputBlockAlignment).field("OutputBlockLength", &self.OutputBlockLength).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DEVICE_DSM_DEFINITION {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DEVICE_DSM_DEFINITION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DEVICE_DSM_DEFINITION>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DEVICE_DSM_DEFINITION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DEVICE_DSM_DEFINITION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for DEVICE_DSM_FREE_SPACE_OUTPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DEVICE_DSM_FREE_SPACE_OUTPUT").field("Version", &self.Version).field("FreeSpace", &self.FreeSpace).finish()
    }
}
unsafe impl ::windows::core::Abi for DEVICE_DSM_FREE_SPACE_OUTPUT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DEVICE_DSM_FREE_SPACE_OUTPUT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DEVICE_DSM_FREE_SPACE_OUTPUT>()) == 0 }
    }
}
impl ::core::cmp::Eq for DEVICE_DSM_FREE_SPACE_OUTPUT {}
impl ::core::default::Default for DEVICE_DSM_FREE_SPACE_OUTPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for DEVICE_DSM_LOST_QUERY_OUTPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DEVICE_DSM_LOST_QUERY_OUTPUT").field("Version", &self.Version).field("Size", &self.Size).field("Alignment", &self.Alignment).field("NumberOfBits", &self.NumberOfBits).field("BitMap", &self.BitMap).finish()
    }
}
unsafe impl ::windows::core::Abi for DEVICE_DSM_LOST_QUERY_OUTPUT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DEVICE_DSM_LOST_QUERY_OUTPUT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DEVICE_DSM_LOST_QUERY_OUTPUT>()) == 0 }
    }
}
impl ::core::cmp::Eq for DEVICE_DSM_LOST_QUERY_OUTPUT {}
impl ::core::default::Default for DEVICE_DSM_LOST_QUERY_OUTPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for DEVICE_DSM_LOST_QUERY_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DEVICE_DSM_LOST_QUERY_PARAMETERS").field("Version", &self.Version).field("Granularity", &self.Granularity).finish()
    }
}
unsafe impl ::windows::core::Abi for DEVICE_DSM_LOST_QUERY_PARAMETERS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DEVICE_DSM_LOST_QUERY_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DEVICE_DSM_LOST_QUERY_PARAMETERS>()) == 0 }
    }
}
impl ::core::cmp::Eq for DEVICE_DSM_LOST_QUERY_PARAMETERS {}
impl ::core::default::Default for DEVICE_DSM_LOST_QUERY_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct DEVICE_DSM_NOTIFICATION_PARAMETERS {
    pub Size: u32,
    pub Flags: u32,
    pub NumFileTypeIDs: u32,
    pub FileTypeID: [::windows::core::GUID; 1],
}
impl ::core::marker::Copy for DEVICE_DSM_NOTIFICATION_PARAMETERS {}
impl ::core::clone::Clone for DEVICE_DSM_NOTIFICATION_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DEVICE_DSM_NOTIFICATION_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DEVICE_DSM_NOTIFICATION_PARAMETERS").field("Size", &self.Size).field("Flags", &self.Flags).field("NumFileTypeIDs", &self.NumFileTypeIDs).field("FileTypeID", &self.FileTypeID).finish()
    }
}
unsafe impl ::windows::core::Abi for DEVICE_DSM_NOTIFICATION_PARAMETERS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DEVICE_DSM_NOTIFICATION_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DEVICE_DSM_NOTIFICATION_PARAMETERS>()) == 0 }
    }
}
impl ::core::cmp::Eq for DEVICE_DSM_NOTIFICATION_PARAMETERS {}
impl ::core::default::Default for DEVICE_DSM_NOTIFICATION_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for DEVICE_DSM_NVCACHE_CHANGE_PRIORITY_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DEVICE_DSM_NVCACHE_CHANGE_PRIORITY_PARAMETERS").field("Size", &self.Size).field("TargetPriority", &self.TargetPriority).field("Reserved", &self.Reserved).finish()
    }
}
unsafe impl ::windows::core::Abi for DEVICE_DSM_NVCACHE_CHANGE_PRIORITY_PARAMETERS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DEVICE_DSM_NVCACHE_CHANGE_PRIORITY_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DEVICE_DSM_NVCACHE_CHANGE_PRIORITY_PARAMETERS>()) == 0 }
    }
}
impl ::core::cmp::Eq for DEVICE_DSM_NVCACHE_CHANGE_PRIORITY_PARAMETERS {}
impl ::core::default::Default for DEVICE_DSM_NVCACHE_CHANGE_PRIORITY_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for DEVICE_DSM_OFFLOAD_READ_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DEVICE_DSM_OFFLOAD_READ_PARAMETERS").field("Flags", &self.Flags).field("TimeToLive", &self.TimeToLive).field("Reserved", &self.Reserved).finish()
    }
}
unsafe impl ::windows::core::Abi for DEVICE_DSM_OFFLOAD_READ_PARAMETERS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DEVICE_DSM_OFFLOAD_READ_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DEVICE_DSM_OFFLOAD_READ_PARAMETERS>()) == 0 }
    }
}
impl ::core::cmp::Eq for DEVICE_DSM_OFFLOAD_READ_PARAMETERS {}
impl ::core::default::Default for DEVICE_DSM_OFFLOAD_READ_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
unsafe impl ::windows::core::Abi for DEVICE_DSM_OFFLOAD_WRITE_PARAMETERS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DEVICE_DSM_OFFLOAD_WRITE_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DEVICE_DSM_OFFLOAD_WRITE_PARAMETERS>()) == 0 }
    }
}
impl ::core::cmp::Eq for DEVICE_DSM_OFFLOAD_WRITE_PARAMETERS {}
impl ::core::default::Default for DEVICE_DSM_OFFLOAD_WRITE_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for DEVICE_DSM_PHYSICAL_ADDRESSES_OUTPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DEVICE_DSM_PHYSICAL_ADDRESSES_OUTPUT").field("Version", &self.Version).field("Flags", &self.Flags).field("TotalNumberOfRanges", &self.TotalNumberOfRanges).field("NumberOfRangesReturned", &self.NumberOfRangesReturned).field("Ranges", &self.Ranges).finish()
    }
}
unsafe impl ::windows::core::Abi for DEVICE_DSM_PHYSICAL_ADDRESSES_OUTPUT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DEVICE_DSM_PHYSICAL_ADDRESSES_OUTPUT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DEVICE_DSM_PHYSICAL_ADDRESSES_OUTPUT>()) == 0 }
    }
}
impl ::core::cmp::Eq for DEVICE_DSM_PHYSICAL_ADDRESSES_OUTPUT {}
impl ::core::default::Default for DEVICE_DSM_PHYSICAL_ADDRESSES_OUTPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
unsafe impl ::windows::core::Abi for DEVICE_DSM_RANGE_ERROR_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DEVICE_DSM_RANGE_ERROR_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DEVICE_DSM_RANGE_ERROR_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for DEVICE_DSM_RANGE_ERROR_INFO {}
impl ::core::default::Default for DEVICE_DSM_RANGE_ERROR_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DEVICE_DSM_REPORT_ZONES_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DEVICE_DSM_REPORT_ZONES_DATA").field("Size", &self.Size).field("ZoneCount", &self.ZoneCount).field("Attributes", &self.Attributes).field("Reserved0", &self.Reserved0).field("ZoneDescriptors", &self.ZoneDescriptors).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DEVICE_DSM_REPORT_ZONES_DATA {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DEVICE_DSM_REPORT_ZONES_DATA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DEVICE_DSM_REPORT_ZONES_DATA>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DEVICE_DSM_REPORT_ZONES_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DEVICE_DSM_REPORT_ZONES_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for DEVICE_DSM_REPORT_ZONES_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DEVICE_DSM_REPORT_ZONES_PARAMETERS").field("Size", &self.Size).field("ReportOption", &self.ReportOption).field("Partial", &self.Partial).field("Reserved", &self.Reserved).finish()
    }
}
unsafe impl ::windows::core::Abi for DEVICE_DSM_REPORT_ZONES_PARAMETERS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DEVICE_DSM_REPORT_ZONES_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DEVICE_DSM_REPORT_ZONES_PARAMETERS>()) == 0 }
    }
}
impl ::core::cmp::Eq for DEVICE_DSM_REPORT_ZONES_PARAMETERS {}
impl ::core::default::Default for DEVICE_DSM_REPORT_ZONES_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct DEVICE_DSM_TIERING_QUERY_INPUT {
    pub Version: u32,
    pub Size: u32,
    pub Flags: u32,
    pub NumberOfTierIds: u32,
    pub TierIds: [::windows::core::GUID; 1],
}
impl ::core::marker::Copy for DEVICE_DSM_TIERING_QUERY_INPUT {}
impl ::core::clone::Clone for DEVICE_DSM_TIERING_QUERY_INPUT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DEVICE_DSM_TIERING_QUERY_INPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DEVICE_DSM_TIERING_QUERY_INPUT").field("Version", &self.Version).field("Size", &self.Size).field("Flags", &self.Flags).field("NumberOfTierIds", &self.NumberOfTierIds).field("TierIds", &self.TierIds).finish()
    }
}
unsafe impl ::windows::core::Abi for DEVICE_DSM_TIERING_QUERY_INPUT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DEVICE_DSM_TIERING_QUERY_INPUT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DEVICE_DSM_TIERING_QUERY_INPUT>()) == 0 }
    }
}
impl ::core::cmp::Eq for DEVICE_DSM_TIERING_QUERY_INPUT {}
impl ::core::default::Default for DEVICE_DSM_TIERING_QUERY_INPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for DEVICE_DSM_TIERING_QUERY_OUTPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DEVICE_DSM_TIERING_QUERY_OUTPUT").field("Version", &self.Version).field("Size", &self.Size).field("Flags", &self.Flags).field("Reserved", &self.Reserved).field("Alignment", &self.Alignment).field("TotalNumberOfRegions", &self.TotalNumberOfRegions).field("NumberOfRegionsReturned", &self.NumberOfRegionsReturned).field("Regions", &self.Regions).finish()
    }
}
unsafe impl ::windows::core::Abi for DEVICE_DSM_TIERING_QUERY_OUTPUT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DEVICE_DSM_TIERING_QUERY_OUTPUT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DEVICE_DSM_TIERING_QUERY_OUTPUT>()) == 0 }
    }
}
impl ::core::cmp::Eq for DEVICE_DSM_TIERING_QUERY_OUTPUT {}
impl ::core::default::Default for DEVICE_DSM_TIERING_QUERY_OUTPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for DEVICE_INTERNAL_STATUS_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DEVICE_INTERNAL_STATUS_DATA")
            .field("Version", &self.Version)
            .field("Size", &self.Size)
            .field("T10VendorId", &self.T10VendorId)
            .field("DataSet1Length", &self.DataSet1Length)
            .field("DataSet2Length", &self.DataSet2Length)
            .field("DataSet3Length", &self.DataSet3Length)
            .field("DataSet4Length", &self.DataSet4Length)
            .field("StatusDataVersion", &self.StatusDataVersion)
            .field("Reserved", &self.Reserved)
            .field("ReasonIdentifier", &self.ReasonIdentifier)
            .field("StatusDataLength", &self.StatusDataLength)
            .field("StatusData", &self.StatusData)
            .finish()
    }
}
unsafe impl ::windows::core::Abi for DEVICE_INTERNAL_STATUS_DATA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DEVICE_INTERNAL_STATUS_DATA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DEVICE_INTERNAL_STATUS_DATA>()) == 0 }
    }
}
impl ::core::cmp::Eq for DEVICE_INTERNAL_STATUS_DATA {}
impl ::core::default::Default for DEVICE_INTERNAL_STATUS_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DEVICE_INTERNAL_STATUS_DATA_REQUEST_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const DeviceInternalStatusDataRequestTypeUndefined: DEVICE_INTERNAL_STATUS_DATA_REQUEST_TYPE = DEVICE_INTERNAL_STATUS_DATA_REQUEST_TYPE(0i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const DeviceCurrentInternalStatusDataHeader: DEVICE_INTERNAL_STATUS_DATA_REQUEST_TYPE = DEVICE_INTERNAL_STATUS_DATA_REQUEST_TYPE(1i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const DeviceCurrentInternalStatusData: DEVICE_INTERNAL_STATUS_DATA_REQUEST_TYPE = DEVICE_INTERNAL_STATUS_DATA_REQUEST_TYPE(2i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const DeviceSavedInternalStatusDataHeader: DEVICE_INTERNAL_STATUS_DATA_REQUEST_TYPE = DEVICE_INTERNAL_STATUS_DATA_REQUEST_TYPE(3i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const DeviceSavedInternalStatusData: DEVICE_INTERNAL_STATUS_DATA_REQUEST_TYPE = DEVICE_INTERNAL_STATUS_DATA_REQUEST_TYPE(4i32);
impl ::core::marker::Copy for DEVICE_INTERNAL_STATUS_DATA_REQUEST_TYPE {}
impl ::core::clone::Clone for DEVICE_INTERNAL_STATUS_DATA_REQUEST_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DEVICE_INTERNAL_STATUS_DATA_REQUEST_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DEVICE_INTERNAL_STATUS_DATA_REQUEST_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for DEVICE_INTERNAL_STATUS_DATA_REQUEST_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DEVICE_INTERNAL_STATUS_DATA_REQUEST_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DEVICE_INTERNAL_STATUS_DATA_SET(pub i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const DeviceStatusDataSetUndefined: DEVICE_INTERNAL_STATUS_DATA_SET = DEVICE_INTERNAL_STATUS_DATA_SET(0i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const DeviceStatusDataSet1: DEVICE_INTERNAL_STATUS_DATA_SET = DEVICE_INTERNAL_STATUS_DATA_SET(1i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const DeviceStatusDataSet2: DEVICE_INTERNAL_STATUS_DATA_SET = DEVICE_INTERNAL_STATUS_DATA_SET(2i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const DeviceStatusDataSet3: DEVICE_INTERNAL_STATUS_DATA_SET = DEVICE_INTERNAL_STATUS_DATA_SET(3i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const DeviceStatusDataSet4: DEVICE_INTERNAL_STATUS_DATA_SET = DEVICE_INTERNAL_STATUS_DATA_SET(4i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const DeviceStatusDataSetMax: DEVICE_INTERNAL_STATUS_DATA_SET = DEVICE_INTERNAL_STATUS_DATA_SET(5i32);
impl ::core::marker::Copy for DEVICE_INTERNAL_STATUS_DATA_SET {}
impl ::core::clone::Clone for DEVICE_INTERNAL_STATUS_DATA_SET {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DEVICE_INTERNAL_STATUS_DATA_SET {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DEVICE_INTERNAL_STATUS_DATA_SET {
    type Abi = Self;
}
impl ::core::fmt::Debug for DEVICE_INTERNAL_STATUS_DATA_SET {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DEVICE_INTERNAL_STATUS_DATA_SET").field(&self.0).finish()
    }
}
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
impl ::core::fmt::Debug for DEVICE_LB_PROVISIONING_DESCRIPTOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DEVICE_LB_PROVISIONING_DESCRIPTOR").field("Version", &self.Version).field("Size", &self.Size).field("_bitfield", &self._bitfield).field("Reserved1", &self.Reserved1).field("OptimalUnmapGranularity", &self.OptimalUnmapGranularity).field("UnmapGranularityAlignment", &self.UnmapGranularityAlignment).field("MaxUnmapLbaCount", &self.MaxUnmapLbaCount).field("MaxUnmapBlockDescriptorCount", &self.MaxUnmapBlockDescriptorCount).finish()
    }
}
unsafe impl ::windows::core::Abi for DEVICE_LB_PROVISIONING_DESCRIPTOR {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DEVICE_LB_PROVISIONING_DESCRIPTOR {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DEVICE_LB_PROVISIONING_DESCRIPTOR>()) == 0 }
    }
}
impl ::core::cmp::Eq for DEVICE_LB_PROVISIONING_DESCRIPTOR {}
impl ::core::default::Default for DEVICE_LB_PROVISIONING_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
unsafe impl ::windows::core::Abi for DEVICE_LOCATION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DEVICE_LOCATION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DEVICE_LOCATION>()) == 0 }
    }
}
impl ::core::cmp::Eq for DEVICE_LOCATION {}
impl ::core::default::Default for DEVICE_LOCATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
unsafe impl ::windows::core::Abi for DEVICE_LOCATION_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DEVICE_LOCATION_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DEVICE_LOCATION_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for DEVICE_LOCATION_0 {}
impl ::core::default::Default for DEVICE_LOCATION_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for DEVICE_LOCATION_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DEVICE_LOCATION_0_0").field("Channel", &self.Channel).field("Device", &self.Device).finish()
    }
}
unsafe impl ::windows::core::Abi for DEVICE_LOCATION_0_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DEVICE_LOCATION_0_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DEVICE_LOCATION_0_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for DEVICE_LOCATION_0_0 {}
impl ::core::default::Default for DEVICE_LOCATION_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for DEVICE_LOCATION_0_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DEVICE_LOCATION_0_1").field("Target", &self.Target).field("Lun", &self.Lun).finish()
    }
}
unsafe impl ::windows::core::Abi for DEVICE_LOCATION_0_1 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DEVICE_LOCATION_0_1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DEVICE_LOCATION_0_1>()) == 0 }
    }
}
impl ::core::cmp::Eq for DEVICE_LOCATION_0_1 {}
impl ::core::default::Default for DEVICE_LOCATION_0_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for DEVICE_MANAGE_DATA_SET_ATTRIBUTES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DEVICE_MANAGE_DATA_SET_ATTRIBUTES").field("Size", &self.Size).field("Action", &self.Action).field("Flags", &self.Flags).field("ParameterBlockOffset", &self.ParameterBlockOffset).field("ParameterBlockLength", &self.ParameterBlockLength).field("DataSetRangesOffset", &self.DataSetRangesOffset).field("DataSetRangesLength", &self.DataSetRangesLength).finish()
    }
}
unsafe impl ::windows::core::Abi for DEVICE_MANAGE_DATA_SET_ATTRIBUTES {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DEVICE_MANAGE_DATA_SET_ATTRIBUTES {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DEVICE_MANAGE_DATA_SET_ATTRIBUTES>()) == 0 }
    }
}
impl ::core::cmp::Eq for DEVICE_MANAGE_DATA_SET_ATTRIBUTES {}
impl ::core::default::Default for DEVICE_MANAGE_DATA_SET_ATTRIBUTES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for DEVICE_MANAGE_DATA_SET_ATTRIBUTES_OUTPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DEVICE_MANAGE_DATA_SET_ATTRIBUTES_OUTPUT").field("Size", &self.Size).field("Action", &self.Action).field("Flags", &self.Flags).field("OperationStatus", &self.OperationStatus).field("ExtendedError", &self.ExtendedError).field("TargetDetailedError", &self.TargetDetailedError).field("ReservedStatus", &self.ReservedStatus).field("OutputBlockOffset", &self.OutputBlockOffset).field("OutputBlockLength", &self.OutputBlockLength).finish()
    }
}
unsafe impl ::windows::core::Abi for DEVICE_MANAGE_DATA_SET_ATTRIBUTES_OUTPUT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DEVICE_MANAGE_DATA_SET_ATTRIBUTES_OUTPUT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DEVICE_MANAGE_DATA_SET_ATTRIBUTES_OUTPUT>()) == 0 }
    }
}
impl ::core::cmp::Eq for DEVICE_MANAGE_DATA_SET_ATTRIBUTES_OUTPUT {}
impl ::core::default::Default for DEVICE_MANAGE_DATA_SET_ATTRIBUTES_OUTPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
#[cfg(feature = "Win32_Storage_FileSystem")]
unsafe impl ::windows::core::Abi for DEVICE_MEDIA_INFO {
    type Abi = Self;
}
#[cfg(feature = "Win32_Storage_FileSystem")]
impl ::core::cmp::PartialEq for DEVICE_MEDIA_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DEVICE_MEDIA_INFO>()) == 0 }
    }
}
#[cfg(feature = "Win32_Storage_FileSystem")]
impl ::core::cmp::Eq for DEVICE_MEDIA_INFO {}
#[cfg(feature = "Win32_Storage_FileSystem")]
impl ::core::default::Default for DEVICE_MEDIA_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
#[cfg(feature = "Win32_Storage_FileSystem")]
unsafe impl ::windows::core::Abi for DEVICE_MEDIA_INFO_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Storage_FileSystem")]
impl ::core::cmp::PartialEq for DEVICE_MEDIA_INFO_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DEVICE_MEDIA_INFO_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Storage_FileSystem")]
impl ::core::cmp::Eq for DEVICE_MEDIA_INFO_0 {}
#[cfg(feature = "Win32_Storage_FileSystem")]
impl ::core::default::Default for DEVICE_MEDIA_INFO_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
#[cfg(feature = "Win32_Storage_FileSystem")]
impl ::core::fmt::Debug for DEVICE_MEDIA_INFO_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DEVICE_MEDIA_INFO_0_0").field("Cylinders", &self.Cylinders).field("MediaType", &self.MediaType).field("TracksPerCylinder", &self.TracksPerCylinder).field("SectorsPerTrack", &self.SectorsPerTrack).field("BytesPerSector", &self.BytesPerSector).field("NumberMediaSides", &self.NumberMediaSides).field("MediaCharacteristics", &self.MediaCharacteristics).finish()
    }
}
#[cfg(feature = "Win32_Storage_FileSystem")]
unsafe impl ::windows::core::Abi for DEVICE_MEDIA_INFO_0_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Storage_FileSystem")]
impl ::core::cmp::PartialEq for DEVICE_MEDIA_INFO_0_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DEVICE_MEDIA_INFO_0_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Storage_FileSystem")]
impl ::core::cmp::Eq for DEVICE_MEDIA_INFO_0_0 {}
#[cfg(feature = "Win32_Storage_FileSystem")]
impl ::core::default::Default for DEVICE_MEDIA_INFO_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
#[cfg(feature = "Win32_Storage_FileSystem")]
impl ::core::fmt::Debug for DEVICE_MEDIA_INFO_0_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DEVICE_MEDIA_INFO_0_1").field("Cylinders", &self.Cylinders).field("MediaType", &self.MediaType).field("TracksPerCylinder", &self.TracksPerCylinder).field("SectorsPerTrack", &self.SectorsPerTrack).field("BytesPerSector", &self.BytesPerSector).field("NumberMediaSides", &self.NumberMediaSides).field("MediaCharacteristics", &self.MediaCharacteristics).finish()
    }
}
#[cfg(feature = "Win32_Storage_FileSystem")]
unsafe impl ::windows::core::Abi for DEVICE_MEDIA_INFO_0_1 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Storage_FileSystem")]
impl ::core::cmp::PartialEq for DEVICE_MEDIA_INFO_0_1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DEVICE_MEDIA_INFO_0_1>()) == 0 }
    }
}
#[cfg(feature = "Win32_Storage_FileSystem")]
impl ::core::cmp::Eq for DEVICE_MEDIA_INFO_0_1 {}
#[cfg(feature = "Win32_Storage_FileSystem")]
impl ::core::default::Default for DEVICE_MEDIA_INFO_0_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
#[cfg(feature = "Win32_Storage_FileSystem")]
unsafe impl ::windows::core::Abi for DEVICE_MEDIA_INFO_0_2 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Storage_FileSystem")]
impl ::core::cmp::PartialEq for DEVICE_MEDIA_INFO_0_2 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DEVICE_MEDIA_INFO_0_2>()) == 0 }
    }
}
#[cfg(feature = "Win32_Storage_FileSystem")]
impl ::core::cmp::Eq for DEVICE_MEDIA_INFO_0_2 {}
#[cfg(feature = "Win32_Storage_FileSystem")]
impl ::core::default::Default for DEVICE_MEDIA_INFO_0_2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
#[cfg(feature = "Win32_Storage_FileSystem")]
unsafe impl ::windows::core::Abi for DEVICE_MEDIA_INFO_0_2_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Storage_FileSystem")]
impl ::core::cmp::PartialEq for DEVICE_MEDIA_INFO_0_2_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DEVICE_MEDIA_INFO_0_2_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Storage_FileSystem")]
impl ::core::cmp::Eq for DEVICE_MEDIA_INFO_0_2_0 {}
#[cfg(feature = "Win32_Storage_FileSystem")]
impl ::core::default::Default for DEVICE_MEDIA_INFO_0_2_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
#[cfg(feature = "Win32_Storage_FileSystem")]
impl ::core::fmt::Debug for DEVICE_MEDIA_INFO_0_2_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DEVICE_MEDIA_INFO_0_2_0_0").field("MediumType", &self.MediumType).field("DensityCode", &self.DensityCode).finish()
    }
}
#[cfg(feature = "Win32_Storage_FileSystem")]
unsafe impl ::windows::core::Abi for DEVICE_MEDIA_INFO_0_2_0_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Storage_FileSystem")]
impl ::core::cmp::PartialEq for DEVICE_MEDIA_INFO_0_2_0_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DEVICE_MEDIA_INFO_0_2_0_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Storage_FileSystem")]
impl ::core::cmp::Eq for DEVICE_MEDIA_INFO_0_2_0_0 {}
#[cfg(feature = "Win32_Storage_FileSystem")]
impl ::core::default::Default for DEVICE_MEDIA_INFO_0_2_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DEVICE_POWER_DESCRIPTOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DEVICE_POWER_DESCRIPTOR")
            .field("Version", &self.Version)
            .field("Size", &self.Size)
            .field("DeviceAttentionSupported", &self.DeviceAttentionSupported)
            .field("AsynchronousNotificationSupported", &self.AsynchronousNotificationSupported)
            .field("IdlePowerManagementEnabled", &self.IdlePowerManagementEnabled)
            .field("D3ColdEnabled", &self.D3ColdEnabled)
            .field("D3ColdSupported", &self.D3ColdSupported)
            .field("NoVerifyDuringIdlePower", &self.NoVerifyDuringIdlePower)
            .field("Reserved", &self.Reserved)
            .field("IdleTimeoutInMS", &self.IdleTimeoutInMS)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DEVICE_POWER_DESCRIPTOR {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DEVICE_POWER_DESCRIPTOR {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DEVICE_POWER_DESCRIPTOR>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DEVICE_POWER_DESCRIPTOR {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DEVICE_POWER_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DEVICE_SEEK_PENALTY_DESCRIPTOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DEVICE_SEEK_PENALTY_DESCRIPTOR").field("Version", &self.Version).field("Size", &self.Size).field("IncursSeekPenalty", &self.IncursSeekPenalty).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DEVICE_SEEK_PENALTY_DESCRIPTOR {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DEVICE_SEEK_PENALTY_DESCRIPTOR {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DEVICE_SEEK_PENALTY_DESCRIPTOR>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DEVICE_SEEK_PENALTY_DESCRIPTOR {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DEVICE_SEEK_PENALTY_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for DEVICE_STORAGE_ADDRESS_RANGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DEVICE_STORAGE_ADDRESS_RANGE").field("StartAddress", &self.StartAddress).field("LengthInBytes", &self.LengthInBytes).finish()
    }
}
unsafe impl ::windows::core::Abi for DEVICE_STORAGE_ADDRESS_RANGE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DEVICE_STORAGE_ADDRESS_RANGE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DEVICE_STORAGE_ADDRESS_RANGE>()) == 0 }
    }
}
impl ::core::cmp::Eq for DEVICE_STORAGE_ADDRESS_RANGE {}
impl ::core::default::Default for DEVICE_STORAGE_ADDRESS_RANGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
unsafe impl ::windows::core::Abi for DEVICE_STORAGE_RANGE_ATTRIBUTES {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DEVICE_STORAGE_RANGE_ATTRIBUTES {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DEVICE_STORAGE_RANGE_ATTRIBUTES>()) == 0 }
    }
}
impl ::core::cmp::Eq for DEVICE_STORAGE_RANGE_ATTRIBUTES {}
impl ::core::default::Default for DEVICE_STORAGE_RANGE_ATTRIBUTES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
unsafe impl ::windows::core::Abi for DEVICE_STORAGE_RANGE_ATTRIBUTES_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DEVICE_STORAGE_RANGE_ATTRIBUTES_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DEVICE_STORAGE_RANGE_ATTRIBUTES_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for DEVICE_STORAGE_RANGE_ATTRIBUTES_0 {}
impl ::core::default::Default for DEVICE_STORAGE_RANGE_ATTRIBUTES_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for DEVICE_STORAGE_RANGE_ATTRIBUTES_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DEVICE_STORAGE_RANGE_ATTRIBUTES_0_0").field("_bitfield", &self._bitfield).finish()
    }
}
unsafe impl ::windows::core::Abi for DEVICE_STORAGE_RANGE_ATTRIBUTES_0_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DEVICE_STORAGE_RANGE_ATTRIBUTES_0_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DEVICE_STORAGE_RANGE_ATTRIBUTES_0_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for DEVICE_STORAGE_RANGE_ATTRIBUTES_0_0 {}
impl ::core::default::Default for DEVICE_STORAGE_RANGE_ATTRIBUTES_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DEVICE_TRIM_DESCRIPTOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DEVICE_TRIM_DESCRIPTOR").field("Version", &self.Version).field("Size", &self.Size).field("TrimEnabled", &self.TrimEnabled).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DEVICE_TRIM_DESCRIPTOR {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DEVICE_TRIM_DESCRIPTOR {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DEVICE_TRIM_DESCRIPTOR>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DEVICE_TRIM_DESCRIPTOR {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DEVICE_TRIM_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DEVICE_WRITE_AGGREGATION_DESCRIPTOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DEVICE_WRITE_AGGREGATION_DESCRIPTOR").field("Version", &self.Version).field("Size", &self.Size).field("BenefitsFromWriteAggregation", &self.BenefitsFromWriteAggregation).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DEVICE_WRITE_AGGREGATION_DESCRIPTOR {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DEVICE_WRITE_AGGREGATION_DESCRIPTOR {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DEVICE_WRITE_AGGREGATION_DESCRIPTOR>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DEVICE_WRITE_AGGREGATION_DESCRIPTOR {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DEVICE_WRITE_AGGREGATION_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`, `\"Win32_Devices_Properties\"`*"]
#[cfg(feature = "Win32_Devices_Properties")]
pub const DEVPKEY_Storage_Disk_Number: super::super::Devices::Properties::DEVPROPKEY = super::super::Devices::Properties::DEVPROPKEY { fmtid: ::windows::core::GUID::from_u128(0x4d1ebee8_0803_4774_9842_b77db50265e9), pid: 5u32 };
#[doc = "*Required features: `\"Win32_System_Ioctl\"`, `\"Win32_Devices_Properties\"`*"]
#[cfg(feature = "Win32_Devices_Properties")]
pub const DEVPKEY_Storage_Gpt_Name: super::super::Devices::Properties::DEVPROPKEY = super::super::Devices::Properties::DEVPROPKEY { fmtid: ::windows::core::GUID::from_u128(0x4d1ebee8_0803_4774_9842_b77db50265e9), pid: 9u32 };
#[doc = "*Required features: `\"Win32_System_Ioctl\"`, `\"Win32_Devices_Properties\"`*"]
#[cfg(feature = "Win32_Devices_Properties")]
pub const DEVPKEY_Storage_Gpt_Type: super::super::Devices::Properties::DEVPROPKEY = super::super::Devices::Properties::DEVPROPKEY { fmtid: ::windows::core::GUID::from_u128(0x4d1ebee8_0803_4774_9842_b77db50265e9), pid: 8u32 };
#[doc = "*Required features: `\"Win32_System_Ioctl\"`, `\"Win32_Devices_Properties\"`*"]
#[cfg(feature = "Win32_Devices_Properties")]
pub const DEVPKEY_Storage_Mbr_Type: super::super::Devices::Properties::DEVPROPKEY = super::super::Devices::Properties::DEVPROPKEY { fmtid: ::windows::core::GUID::from_u128(0x4d1ebee8_0803_4774_9842_b77db50265e9), pid: 7u32 };
#[doc = "*Required features: `\"Win32_System_Ioctl\"`, `\"Win32_Devices_Properties\"`*"]
#[cfg(feature = "Win32_Devices_Properties")]
pub const DEVPKEY_Storage_Partition_Number: super::super::Devices::Properties::DEVPROPKEY = super::super::Devices::Properties::DEVPROPKEY { fmtid: ::windows::core::GUID::from_u128(0x4d1ebee8_0803_4774_9842_b77db50265e9), pid: 6u32 };
#[doc = "*Required features: `\"Win32_System_Ioctl\"`, `\"Win32_Devices_Properties\"`*"]
#[cfg(feature = "Win32_Devices_Properties")]
pub const DEVPKEY_Storage_Portable: super::super::Devices::Properties::DEVPROPKEY = super::super::Devices::Properties::DEVPROPKEY { fmtid: ::windows::core::GUID::from_u128(0x4d1ebee8_0803_4774_9842_b77db50265e9), pid: 2u32 };
#[doc = "*Required features: `\"Win32_System_Ioctl\"`, `\"Win32_Devices_Properties\"`*"]
#[cfg(feature = "Win32_Devices_Properties")]
pub const DEVPKEY_Storage_Removable_Media: super::super::Devices::Properties::DEVPROPKEY = super::super::Devices::Properties::DEVPROPKEY { fmtid: ::windows::core::GUID::from_u128(0x4d1ebee8_0803_4774_9842_b77db50265e9), pid: 3u32 };
#[doc = "*Required features: `\"Win32_System_Ioctl\"`, `\"Win32_Devices_Properties\"`*"]
#[cfg(feature = "Win32_Devices_Properties")]
pub const DEVPKEY_Storage_System_Critical: super::super::Devices::Properties::DEVPROPKEY = super::super::Devices::Properties::DEVPROPKEY { fmtid: ::windows::core::GUID::from_u128(0x4d1ebee8_0803_4774_9842_b77db50265e9), pid: 4u32 };
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
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DISK_CACHE_INFORMATION {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DISK_CACHE_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DISK_CACHE_INFORMATION>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DISK_CACHE_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DISK_CACHE_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DISK_CACHE_INFORMATION_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DISK_CACHE_INFORMATION_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DISK_CACHE_INFORMATION_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DISK_CACHE_INFORMATION_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DISK_CACHE_INFORMATION_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DISK_CACHE_INFORMATION_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DISK_CACHE_INFORMATION_0_0").field("Minimum", &self.Minimum).field("Maximum", &self.Maximum).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DISK_CACHE_INFORMATION_0_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DISK_CACHE_INFORMATION_0_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DISK_CACHE_INFORMATION_0_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DISK_CACHE_INFORMATION_0_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DISK_CACHE_INFORMATION_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DISK_CACHE_INFORMATION_0_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DISK_CACHE_INFORMATION_0_1").field("Minimum", &self.Minimum).field("Maximum", &self.Maximum).field("MaximumBlocks", &self.MaximumBlocks).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DISK_CACHE_INFORMATION_0_1 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DISK_CACHE_INFORMATION_0_1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DISK_CACHE_INFORMATION_0_1>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DISK_CACHE_INFORMATION_0_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DISK_CACHE_INFORMATION_0_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DISK_CACHE_RETENTION_PRIORITY(pub i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const EqualPriority: DISK_CACHE_RETENTION_PRIORITY = DISK_CACHE_RETENTION_PRIORITY(0i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const KeepPrefetchedData: DISK_CACHE_RETENTION_PRIORITY = DISK_CACHE_RETENTION_PRIORITY(1i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const KeepReadData: DISK_CACHE_RETENTION_PRIORITY = DISK_CACHE_RETENTION_PRIORITY(2i32);
impl ::core::marker::Copy for DISK_CACHE_RETENTION_PRIORITY {}
impl ::core::clone::Clone for DISK_CACHE_RETENTION_PRIORITY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DISK_CACHE_RETENTION_PRIORITY {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DISK_CACHE_RETENTION_PRIORITY {
    type Abi = Self;
}
impl ::core::fmt::Debug for DISK_CACHE_RETENTION_PRIORITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DISK_CACHE_RETENTION_PRIORITY").field(&self.0).finish()
    }
}
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
impl ::core::fmt::Debug for DISK_CONTROLLER_NUMBER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DISK_CONTROLLER_NUMBER").field("ControllerNumber", &self.ControllerNumber).field("DiskNumber", &self.DiskNumber).finish()
    }
}
unsafe impl ::windows::core::Abi for DISK_CONTROLLER_NUMBER {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DISK_CONTROLLER_NUMBER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DISK_CONTROLLER_NUMBER>()) == 0 }
    }
}
impl ::core::cmp::Eq for DISK_CONTROLLER_NUMBER {}
impl ::core::default::Default for DISK_CONTROLLER_NUMBER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
unsafe impl ::windows::core::Abi for DISK_DETECTION_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DISK_DETECTION_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DISK_DETECTION_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for DISK_DETECTION_INFO {}
impl ::core::default::Default for DISK_DETECTION_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
unsafe impl ::windows::core::Abi for DISK_DETECTION_INFO_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DISK_DETECTION_INFO_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DISK_DETECTION_INFO_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for DISK_DETECTION_INFO_0 {}
impl ::core::default::Default for DISK_DETECTION_INFO_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for DISK_DETECTION_INFO_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DISK_DETECTION_INFO_0_0").field("Int13", &self.Int13).field("ExInt13", &self.ExInt13).finish()
    }
}
unsafe impl ::windows::core::Abi for DISK_DETECTION_INFO_0_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DISK_DETECTION_INFO_0_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DISK_DETECTION_INFO_0_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for DISK_DETECTION_INFO_0_0 {}
impl ::core::default::Default for DISK_DETECTION_INFO_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for DISK_EXTENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DISK_EXTENT").field("DiskNumber", &self.DiskNumber).field("StartingOffset", &self.StartingOffset).field("ExtentLength", &self.ExtentLength).finish()
    }
}
unsafe impl ::windows::core::Abi for DISK_EXTENT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DISK_EXTENT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DISK_EXTENT>()) == 0 }
    }
}
impl ::core::cmp::Eq for DISK_EXTENT {}
impl ::core::default::Default for DISK_EXTENT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for DISK_EX_INT13_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DISK_EX_INT13_INFO").field("ExBufferSize", &self.ExBufferSize).field("ExFlags", &self.ExFlags).field("ExCylinders", &self.ExCylinders).field("ExHeads", &self.ExHeads).field("ExSectorsPerTrack", &self.ExSectorsPerTrack).field("ExSectorsPerDrive", &self.ExSectorsPerDrive).field("ExSectorSize", &self.ExSectorSize).field("ExReserved", &self.ExReserved).finish()
    }
}
unsafe impl ::windows::core::Abi for DISK_EX_INT13_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DISK_EX_INT13_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DISK_EX_INT13_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for DISK_EX_INT13_INFO {}
impl ::core::default::Default for DISK_EX_INT13_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for DISK_GEOMETRY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DISK_GEOMETRY").field("Cylinders", &self.Cylinders).field("MediaType", &self.MediaType).field("TracksPerCylinder", &self.TracksPerCylinder).field("SectorsPerTrack", &self.SectorsPerTrack).field("BytesPerSector", &self.BytesPerSector).finish()
    }
}
unsafe impl ::windows::core::Abi for DISK_GEOMETRY {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DISK_GEOMETRY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DISK_GEOMETRY>()) == 0 }
    }
}
impl ::core::cmp::Eq for DISK_GEOMETRY {}
impl ::core::default::Default for DISK_GEOMETRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for DISK_GEOMETRY_EX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DISK_GEOMETRY_EX").field("Geometry", &self.Geometry).field("DiskSize", &self.DiskSize).field("Data", &self.Data).finish()
    }
}
unsafe impl ::windows::core::Abi for DISK_GEOMETRY_EX {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DISK_GEOMETRY_EX {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DISK_GEOMETRY_EX>()) == 0 }
    }
}
impl ::core::cmp::Eq for DISK_GEOMETRY_EX {}
impl ::core::default::Default for DISK_GEOMETRY_EX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for DISK_GROW_PARTITION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DISK_GROW_PARTITION").field("PartitionNumber", &self.PartitionNumber).field("BytesToGrow", &self.BytesToGrow).finish()
    }
}
unsafe impl ::windows::core::Abi for DISK_GROW_PARTITION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DISK_GROW_PARTITION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DISK_GROW_PARTITION>()) == 0 }
    }
}
impl ::core::cmp::Eq for DISK_GROW_PARTITION {}
impl ::core::default::Default for DISK_GROW_PARTITION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for DISK_HISTOGRAM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DISK_HISTOGRAM").field("DiskSize", &self.DiskSize).field("Start", &self.Start).field("End", &self.End).field("Average", &self.Average).field("AverageRead", &self.AverageRead).field("AverageWrite", &self.AverageWrite).field("Granularity", &self.Granularity).field("Size", &self.Size).field("ReadCount", &self.ReadCount).field("WriteCount", &self.WriteCount).field("Histogram", &self.Histogram).finish()
    }
}
unsafe impl ::windows::core::Abi for DISK_HISTOGRAM {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DISK_HISTOGRAM {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DISK_HISTOGRAM>()) == 0 }
    }
}
impl ::core::cmp::Eq for DISK_HISTOGRAM {}
impl ::core::default::Default for DISK_HISTOGRAM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for DISK_INT13_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DISK_INT13_INFO").field("DriveSelect", &self.DriveSelect).field("MaxCylinders", &self.MaxCylinders).field("SectorsPerTrack", &self.SectorsPerTrack).field("MaxHeads", &self.MaxHeads).field("NumberDrives", &self.NumberDrives).finish()
    }
}
unsafe impl ::windows::core::Abi for DISK_INT13_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DISK_INT13_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DISK_INT13_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for DISK_INT13_INFO {}
impl ::core::default::Default for DISK_INT13_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for DISK_LOGGING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DISK_LOGGING").field("Function", &self.Function).field("BufferAddress", &self.BufferAddress).field("BufferSize", &self.BufferSize).finish()
    }
}
unsafe impl ::windows::core::Abi for DISK_LOGGING {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DISK_LOGGING {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DISK_LOGGING>()) == 0 }
    }
}
impl ::core::cmp::Eq for DISK_LOGGING {}
impl ::core::default::Default for DISK_LOGGING {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
unsafe impl ::windows::core::Abi for DISK_PARTITION_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DISK_PARTITION_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DISK_PARTITION_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for DISK_PARTITION_INFO {}
impl ::core::default::Default for DISK_PARTITION_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
unsafe impl ::windows::core::Abi for DISK_PARTITION_INFO_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DISK_PARTITION_INFO_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DISK_PARTITION_INFO_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for DISK_PARTITION_INFO_0 {}
impl ::core::default::Default for DISK_PARTITION_INFO_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct DISK_PARTITION_INFO_0_0 {
    pub DiskId: ::windows::core::GUID,
}
impl ::core::marker::Copy for DISK_PARTITION_INFO_0_0 {}
impl ::core::clone::Clone for DISK_PARTITION_INFO_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DISK_PARTITION_INFO_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DISK_PARTITION_INFO_0_0").field("DiskId", &self.DiskId).finish()
    }
}
unsafe impl ::windows::core::Abi for DISK_PARTITION_INFO_0_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DISK_PARTITION_INFO_0_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DISK_PARTITION_INFO_0_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for DISK_PARTITION_INFO_0_0 {}
impl ::core::default::Default for DISK_PARTITION_INFO_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for DISK_PARTITION_INFO_0_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DISK_PARTITION_INFO_0_1").field("Signature", &self.Signature).field("CheckSum", &self.CheckSum).finish()
    }
}
unsafe impl ::windows::core::Abi for DISK_PARTITION_INFO_0_1 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DISK_PARTITION_INFO_0_1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DISK_PARTITION_INFO_0_1>()) == 0 }
    }
}
impl ::core::cmp::Eq for DISK_PARTITION_INFO_0_1 {}
impl ::core::default::Default for DISK_PARTITION_INFO_0_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for DISK_PERFORMANCE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DISK_PERFORMANCE")
            .field("BytesRead", &self.BytesRead)
            .field("BytesWritten", &self.BytesWritten)
            .field("ReadTime", &self.ReadTime)
            .field("WriteTime", &self.WriteTime)
            .field("IdleTime", &self.IdleTime)
            .field("ReadCount", &self.ReadCount)
            .field("WriteCount", &self.WriteCount)
            .field("QueueDepth", &self.QueueDepth)
            .field("SplitCount", &self.SplitCount)
            .field("QueryTime", &self.QueryTime)
            .field("StorageDeviceNumber", &self.StorageDeviceNumber)
            .field("StorageManagerName", &self.StorageManagerName)
            .finish()
    }
}
unsafe impl ::windows::core::Abi for DISK_PERFORMANCE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DISK_PERFORMANCE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DISK_PERFORMANCE>()) == 0 }
    }
}
impl ::core::cmp::Eq for DISK_PERFORMANCE {}
impl ::core::default::Default for DISK_PERFORMANCE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DISK_RECORD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DISK_RECORD").field("ByteOffset", &self.ByteOffset).field("StartTime", &self.StartTime).field("EndTime", &self.EndTime).field("VirtualAddress", &self.VirtualAddress).field("NumberOfBytes", &self.NumberOfBytes).field("DeviceNumber", &self.DeviceNumber).field("ReadRequest", &self.ReadRequest).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DISK_RECORD {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DISK_RECORD {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DISK_RECORD>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DISK_RECORD {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DISK_RECORD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
unsafe impl ::windows::core::Abi for DRIVERSTATUS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DRIVERSTATUS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DRIVERSTATUS>()) == 0 }
    }
}
impl ::core::cmp::Eq for DRIVERSTATUS {}
impl ::core::default::Default for DRIVERSTATUS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DRIVE_LAYOUT_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DRIVE_LAYOUT_INFORMATION").field("PartitionCount", &self.PartitionCount).field("Signature", &self.Signature).field("PartitionEntry", &self.PartitionEntry).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DRIVE_LAYOUT_INFORMATION {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DRIVE_LAYOUT_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DRIVE_LAYOUT_INFORMATION>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DRIVE_LAYOUT_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DRIVE_LAYOUT_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DRIVE_LAYOUT_INFORMATION_EX {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DRIVE_LAYOUT_INFORMATION_EX {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DRIVE_LAYOUT_INFORMATION_EX>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DRIVE_LAYOUT_INFORMATION_EX {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DRIVE_LAYOUT_INFORMATION_EX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DRIVE_LAYOUT_INFORMATION_EX_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DRIVE_LAYOUT_INFORMATION_EX_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DRIVE_LAYOUT_INFORMATION_EX_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DRIVE_LAYOUT_INFORMATION_EX_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DRIVE_LAYOUT_INFORMATION_EX_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct DRIVE_LAYOUT_INFORMATION_GPT {
    pub DiskId: ::windows::core::GUID,
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
impl ::core::fmt::Debug for DRIVE_LAYOUT_INFORMATION_GPT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DRIVE_LAYOUT_INFORMATION_GPT").field("DiskId", &self.DiskId).field("StartingUsableOffset", &self.StartingUsableOffset).field("UsableLength", &self.UsableLength).field("MaxPartitionCount", &self.MaxPartitionCount).finish()
    }
}
unsafe impl ::windows::core::Abi for DRIVE_LAYOUT_INFORMATION_GPT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DRIVE_LAYOUT_INFORMATION_GPT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DRIVE_LAYOUT_INFORMATION_GPT>()) == 0 }
    }
}
impl ::core::cmp::Eq for DRIVE_LAYOUT_INFORMATION_GPT {}
impl ::core::default::Default for DRIVE_LAYOUT_INFORMATION_GPT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for DRIVE_LAYOUT_INFORMATION_MBR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DRIVE_LAYOUT_INFORMATION_MBR").field("Signature", &self.Signature).field("CheckSum", &self.CheckSum).finish()
    }
}
unsafe impl ::windows::core::Abi for DRIVE_LAYOUT_INFORMATION_MBR {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DRIVE_LAYOUT_INFORMATION_MBR {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DRIVE_LAYOUT_INFORMATION_MBR>()) == 0 }
    }
}
impl ::core::cmp::Eq for DRIVE_LAYOUT_INFORMATION_MBR {}
impl ::core::default::Default for DRIVE_LAYOUT_INFORMATION_MBR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DUPLICATE_EXTENTS_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DUPLICATE_EXTENTS_DATA").field("FileHandle", &self.FileHandle).field("SourceFileOffset", &self.SourceFileOffset).field("TargetFileOffset", &self.TargetFileOffset).field("ByteCount", &self.ByteCount).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DUPLICATE_EXTENTS_DATA {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DUPLICATE_EXTENTS_DATA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DUPLICATE_EXTENTS_DATA>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DUPLICATE_EXTENTS_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DUPLICATE_EXTENTS_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::fmt::Debug for DUPLICATE_EXTENTS_DATA32 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DUPLICATE_EXTENTS_DATA32").field("FileHandle", &self.FileHandle).field("SourceFileOffset", &self.SourceFileOffset).field("TargetFileOffset", &self.TargetFileOffset).field("ByteCount", &self.ByteCount).finish()
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
unsafe impl ::windows::core::Abi for DUPLICATE_EXTENTS_DATA32 {
    type Abi = Self;
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::PartialEq for DUPLICATE_EXTENTS_DATA32 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DUPLICATE_EXTENTS_DATA32>()) == 0 }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::Eq for DUPLICATE_EXTENTS_DATA32 {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::default::Default for DUPLICATE_EXTENTS_DATA32 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DUPLICATE_EXTENTS_DATA_EX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DUPLICATE_EXTENTS_DATA_EX").field("Size", &self.Size).field("FileHandle", &self.FileHandle).field("SourceFileOffset", &self.SourceFileOffset).field("TargetFileOffset", &self.TargetFileOffset).field("ByteCount", &self.ByteCount).field("Flags", &self.Flags).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DUPLICATE_EXTENTS_DATA_EX {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DUPLICATE_EXTENTS_DATA_EX {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DUPLICATE_EXTENTS_DATA_EX>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DUPLICATE_EXTENTS_DATA_EX {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DUPLICATE_EXTENTS_DATA_EX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::fmt::Debug for DUPLICATE_EXTENTS_DATA_EX32 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DUPLICATE_EXTENTS_DATA_EX32").field("Size", &self.Size).field("FileHandle", &self.FileHandle).field("SourceFileOffset", &self.SourceFileOffset).field("TargetFileOffset", &self.TargetFileOffset).field("ByteCount", &self.ByteCount).field("Flags", &self.Flags).finish()
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
unsafe impl ::windows::core::Abi for DUPLICATE_EXTENTS_DATA_EX32 {
    type Abi = Self;
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::PartialEq for DUPLICATE_EXTENTS_DATA_EX32 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DUPLICATE_EXTENTS_DATA_EX32>()) == 0 }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::Eq for DUPLICATE_EXTENTS_DATA_EX32 {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::default::Default for DUPLICATE_EXTENTS_DATA_EX32 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const DUPLICATE_EXTENTS_DATA_EX_ASYNC: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const DUPLICATE_EXTENTS_DATA_EX_SOURCE_ATOMIC: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DUPLICATE_EXTENTS_STATE(pub i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FileSnapStateInactive: DUPLICATE_EXTENTS_STATE = DUPLICATE_EXTENTS_STATE(0i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FileSnapStateSource: DUPLICATE_EXTENTS_STATE = DUPLICATE_EXTENTS_STATE(1i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FileSnapStateTarget: DUPLICATE_EXTENTS_STATE = DUPLICATE_EXTENTS_STATE(2i32);
impl ::core::marker::Copy for DUPLICATE_EXTENTS_STATE {}
impl ::core::clone::Clone for DUPLICATE_EXTENTS_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DUPLICATE_EXTENTS_STATE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DUPLICATE_EXTENTS_STATE {
    type Abi = Self;
}
impl ::core::fmt::Debug for DUPLICATE_EXTENTS_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DUPLICATE_EXTENTS_STATE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const DeviceDsmActionFlag_NonDestructive: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const EFS_TRACKED_OFFSET_HEADER_FLAG: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct ELEMENT_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const AllElements: ELEMENT_TYPE = ELEMENT_TYPE(0i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ChangerTransport: ELEMENT_TYPE = ELEMENT_TYPE(1i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ChangerSlot: ELEMENT_TYPE = ELEMENT_TYPE(2i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ChangerIEPort: ELEMENT_TYPE = ELEMENT_TYPE(3i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ChangerDrive: ELEMENT_TYPE = ELEMENT_TYPE(4i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ChangerDoor: ELEMENT_TYPE = ELEMENT_TYPE(5i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ChangerKeypad: ELEMENT_TYPE = ELEMENT_TYPE(6i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ChangerMaxElement: ELEMENT_TYPE = ELEMENT_TYPE(7i32);
impl ::core::marker::Copy for ELEMENT_TYPE {}
impl ::core::clone::Clone for ELEMENT_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ELEMENT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ELEMENT_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for ELEMENT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ELEMENT_TYPE").field(&self.0).finish()
    }
}
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
impl ::core::fmt::Debug for ENCRYPTED_DATA_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ENCRYPTED_DATA_INFO")
            .field("StartingFileOffset", &self.StartingFileOffset)
            .field("OutputBufferOffset", &self.OutputBufferOffset)
            .field("BytesWithinFileSize", &self.BytesWithinFileSize)
            .field("BytesWithinValidDataLength", &self.BytesWithinValidDataLength)
            .field("CompressionFormat", &self.CompressionFormat)
            .field("DataUnitShift", &self.DataUnitShift)
            .field("ChunkShift", &self.ChunkShift)
            .field("ClusterShift", &self.ClusterShift)
            .field("EncryptionFormat", &self.EncryptionFormat)
            .field("NumberOfDataBlocks", &self.NumberOfDataBlocks)
            .field("DataBlockSize", &self.DataBlockSize)
            .finish()
    }
}
unsafe impl ::windows::core::Abi for ENCRYPTED_DATA_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for ENCRYPTED_DATA_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<ENCRYPTED_DATA_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for ENCRYPTED_DATA_INFO {}
impl ::core::default::Default for ENCRYPTED_DATA_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for ENCRYPTION_BUFFER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ENCRYPTION_BUFFER").field("EncryptionOperation", &self.EncryptionOperation).field("Private", &self.Private).finish()
    }
}
unsafe impl ::windows::core::Abi for ENCRYPTION_BUFFER {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for ENCRYPTION_BUFFER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<ENCRYPTION_BUFFER>()) == 0 }
    }
}
impl ::core::cmp::Eq for ENCRYPTION_BUFFER {}
impl ::core::default::Default for ENCRYPTION_BUFFER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for ENCRYPTION_KEY_CTRL_INPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ENCRYPTION_KEY_CTRL_INPUT").field("HeaderSize", &self.HeaderSize).field("StructureSize", &self.StructureSize).field("KeyOffset", &self.KeyOffset).field("KeySize", &self.KeySize).field("DplLock", &self.DplLock).field("DplUserId", &self.DplUserId).field("DplCredentialId", &self.DplCredentialId).finish()
    }
}
unsafe impl ::windows::core::Abi for ENCRYPTION_KEY_CTRL_INPUT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for ENCRYPTION_KEY_CTRL_INPUT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<ENCRYPTION_KEY_CTRL_INPUT>()) == 0 }
    }
}
impl ::core::cmp::Eq for ENCRYPTION_KEY_CTRL_INPUT {}
impl ::core::default::Default for ENCRYPTION_KEY_CTRL_INPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for EXFAT_STATISTICS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EXFAT_STATISTICS")
            .field("CreateHits", &self.CreateHits)
            .field("SuccessfulCreates", &self.SuccessfulCreates)
            .field("FailedCreates", &self.FailedCreates)
            .field("NonCachedReads", &self.NonCachedReads)
            .field("NonCachedReadBytes", &self.NonCachedReadBytes)
            .field("NonCachedWrites", &self.NonCachedWrites)
            .field("NonCachedWriteBytes", &self.NonCachedWriteBytes)
            .field("NonCachedDiskReads", &self.NonCachedDiskReads)
            .field("NonCachedDiskWrites", &self.NonCachedDiskWrites)
            .finish()
    }
}
unsafe impl ::windows::core::Abi for EXFAT_STATISTICS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for EXFAT_STATISTICS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<EXFAT_STATISTICS>()) == 0 }
    }
}
impl ::core::cmp::Eq for EXFAT_STATISTICS {}
impl ::core::default::Default for EXFAT_STATISTICS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for EXTENDED_ENCRYPTED_DATA_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EXTENDED_ENCRYPTED_DATA_INFO").field("ExtendedCode", &self.ExtendedCode).field("Length", &self.Length).field("Flags", &self.Flags).field("Reserved", &self.Reserved).finish()
    }
}
unsafe impl ::windows::core::Abi for EXTENDED_ENCRYPTED_DATA_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for EXTENDED_ENCRYPTED_DATA_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<EXTENDED_ENCRYPTED_DATA_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for EXTENDED_ENCRYPTED_DATA_INFO {}
impl ::core::default::Default for EXTENDED_ENCRYPTED_DATA_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for FAT_STATISTICS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FAT_STATISTICS")
            .field("CreateHits", &self.CreateHits)
            .field("SuccessfulCreates", &self.SuccessfulCreates)
            .field("FailedCreates", &self.FailedCreates)
            .field("NonCachedReads", &self.NonCachedReads)
            .field("NonCachedReadBytes", &self.NonCachedReadBytes)
            .field("NonCachedWrites", &self.NonCachedWrites)
            .field("NonCachedWriteBytes", &self.NonCachedWriteBytes)
            .field("NonCachedDiskReads", &self.NonCachedDiskReads)
            .field("NonCachedDiskWrites", &self.NonCachedDiskWrites)
            .finish()
    }
}
unsafe impl ::windows::core::Abi for FAT_STATISTICS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for FAT_STATISTICS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FAT_STATISTICS>()) == 0 }
    }
}
impl ::core::cmp::Eq for FAT_STATISTICS {}
impl ::core::default::Default for FAT_STATISTICS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for FILESYSTEM_STATISTICS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FILESYSTEM_STATISTICS")
            .field("FileSystemType", &self.FileSystemType)
            .field("Version", &self.Version)
            .field("SizeOfCompleteStructure", &self.SizeOfCompleteStructure)
            .field("UserFileReads", &self.UserFileReads)
            .field("UserFileReadBytes", &self.UserFileReadBytes)
            .field("UserDiskReads", &self.UserDiskReads)
            .field("UserFileWrites", &self.UserFileWrites)
            .field("UserFileWriteBytes", &self.UserFileWriteBytes)
            .field("UserDiskWrites", &self.UserDiskWrites)
            .field("MetaDataReads", &self.MetaDataReads)
            .field("MetaDataReadBytes", &self.MetaDataReadBytes)
            .field("MetaDataDiskReads", &self.MetaDataDiskReads)
            .field("MetaDataWrites", &self.MetaDataWrites)
            .field("MetaDataWriteBytes", &self.MetaDataWriteBytes)
            .field("MetaDataDiskWrites", &self.MetaDataDiskWrites)
            .finish()
    }
}
unsafe impl ::windows::core::Abi for FILESYSTEM_STATISTICS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for FILESYSTEM_STATISTICS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FILESYSTEM_STATISTICS>()) == 0 }
    }
}
impl ::core::cmp::Eq for FILESYSTEM_STATISTICS {}
impl ::core::default::Default for FILESYSTEM_STATISTICS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for FILESYSTEM_STATISTICS_EX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FILESYSTEM_STATISTICS_EX")
            .field("FileSystemType", &self.FileSystemType)
            .field("Version", &self.Version)
            .field("SizeOfCompleteStructure", &self.SizeOfCompleteStructure)
            .field("UserFileReads", &self.UserFileReads)
            .field("UserFileReadBytes", &self.UserFileReadBytes)
            .field("UserDiskReads", &self.UserDiskReads)
            .field("UserFileWrites", &self.UserFileWrites)
            .field("UserFileWriteBytes", &self.UserFileWriteBytes)
            .field("UserDiskWrites", &self.UserDiskWrites)
            .field("MetaDataReads", &self.MetaDataReads)
            .field("MetaDataReadBytes", &self.MetaDataReadBytes)
            .field("MetaDataDiskReads", &self.MetaDataDiskReads)
            .field("MetaDataWrites", &self.MetaDataWrites)
            .field("MetaDataWriteBytes", &self.MetaDataWriteBytes)
            .field("MetaDataDiskWrites", &self.MetaDataDiskWrites)
            .finish()
    }
}
unsafe impl ::windows::core::Abi for FILESYSTEM_STATISTICS_EX {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for FILESYSTEM_STATISTICS_EX {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FILESYSTEM_STATISTICS_EX>()) == 0 }
    }
}
impl ::core::cmp::Eq for FILESYSTEM_STATISTICS_EX {}
impl ::core::default::Default for FILESYSTEM_STATISTICS_EX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FILESYSTEM_STATISTICS_TYPE(pub u16);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FILESYSTEM_STATISTICS_TYPE_EXFAT: FILESYSTEM_STATISTICS_TYPE = FILESYSTEM_STATISTICS_TYPE(3u16);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FILESYSTEM_STATISTICS_TYPE_FAT: FILESYSTEM_STATISTICS_TYPE = FILESYSTEM_STATISTICS_TYPE(2u16);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FILESYSTEM_STATISTICS_TYPE_NTFS: FILESYSTEM_STATISTICS_TYPE = FILESYSTEM_STATISTICS_TYPE(1u16);
impl ::core::marker::Copy for FILESYSTEM_STATISTICS_TYPE {}
impl ::core::clone::Clone for FILESYSTEM_STATISTICS_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FILESYSTEM_STATISTICS_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for FILESYSTEM_STATISTICS_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for FILESYSTEM_STATISTICS_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FILESYSTEM_STATISTICS_TYPE").field(&self.0).finish()
    }
}
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
impl ::core::fmt::Debug for FILE_ALLOCATED_RANGE_BUFFER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FILE_ALLOCATED_RANGE_BUFFER").field("FileOffset", &self.FileOffset).field("Length", &self.Length).finish()
    }
}
unsafe impl ::windows::core::Abi for FILE_ALLOCATED_RANGE_BUFFER {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for FILE_ALLOCATED_RANGE_BUFFER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FILE_ALLOCATED_RANGE_BUFFER>()) == 0 }
    }
}
impl ::core::cmp::Eq for FILE_ALLOCATED_RANGE_BUFFER {}
impl ::core::default::Default for FILE_ALLOCATED_RANGE_BUFFER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for FILE_DESIRED_STORAGE_CLASS_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FILE_DESIRED_STORAGE_CLASS_INFORMATION").field("Class", &self.Class).field("Flags", &self.Flags).finish()
    }
}
unsafe impl ::windows::core::Abi for FILE_DESIRED_STORAGE_CLASS_INFORMATION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for FILE_DESIRED_STORAGE_CLASS_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FILE_DESIRED_STORAGE_CLASS_INFORMATION>()) == 0 }
    }
}
impl ::core::cmp::Eq for FILE_DESIRED_STORAGE_CLASS_INFORMATION {}
impl ::core::default::Default for FILE_DESIRED_STORAGE_CLASS_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for FILE_FS_PERSISTENT_VOLUME_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FILE_FS_PERSISTENT_VOLUME_INFORMATION").field("VolumeFlags", &self.VolumeFlags).field("FlagMask", &self.FlagMask).field("Version", &self.Version).field("Reserved", &self.Reserved).finish()
    }
}
unsafe impl ::windows::core::Abi for FILE_FS_PERSISTENT_VOLUME_INFORMATION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for FILE_FS_PERSISTENT_VOLUME_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FILE_FS_PERSISTENT_VOLUME_INFORMATION>()) == 0 }
    }
}
impl ::core::cmp::Eq for FILE_FS_PERSISTENT_VOLUME_INFORMATION {}
impl ::core::default::Default for FILE_FS_PERSISTENT_VOLUME_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for FILE_INITIATE_REPAIR_OUTPUT_BUFFER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FILE_INITIATE_REPAIR_OUTPUT_BUFFER").field("Hint1", &self.Hint1).field("Hint2", &self.Hint2).field("Clsn", &self.Clsn).field("Status", &self.Status).finish()
    }
}
unsafe impl ::windows::core::Abi for FILE_INITIATE_REPAIR_OUTPUT_BUFFER {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for FILE_INITIATE_REPAIR_OUTPUT_BUFFER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FILE_INITIATE_REPAIR_OUTPUT_BUFFER>()) == 0 }
    }
}
impl ::core::cmp::Eq for FILE_INITIATE_REPAIR_OUTPUT_BUFFER {}
impl ::core::default::Default for FILE_INITIATE_REPAIR_OUTPUT_BUFFER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for FILE_LAYOUT_ENTRY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FILE_LAYOUT_ENTRY").field("Version", &self.Version).field("NextFileOffset", &self.NextFileOffset).field("Flags", &self.Flags).field("FileAttributes", &self.FileAttributes).field("FileReferenceNumber", &self.FileReferenceNumber).field("FirstNameOffset", &self.FirstNameOffset).field("FirstStreamOffset", &self.FirstStreamOffset).field("ExtraInfoOffset", &self.ExtraInfoOffset).field("ExtraInfoLength", &self.ExtraInfoLength).finish()
    }
}
unsafe impl ::windows::core::Abi for FILE_LAYOUT_ENTRY {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for FILE_LAYOUT_ENTRY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FILE_LAYOUT_ENTRY>()) == 0 }
    }
}
impl ::core::cmp::Eq for FILE_LAYOUT_ENTRY {}
impl ::core::default::Default for FILE_LAYOUT_ENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for FILE_LAYOUT_INFO_ENTRY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FILE_LAYOUT_INFO_ENTRY").field("BasicInformation", &self.BasicInformation).field("OwnerId", &self.OwnerId).field("SecurityId", &self.SecurityId).field("Usn", &self.Usn).field("StorageReserveId", &self.StorageReserveId).finish()
    }
}
unsafe impl ::windows::core::Abi for FILE_LAYOUT_INFO_ENTRY {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for FILE_LAYOUT_INFO_ENTRY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FILE_LAYOUT_INFO_ENTRY>()) == 0 }
    }
}
impl ::core::cmp::Eq for FILE_LAYOUT_INFO_ENTRY {}
impl ::core::default::Default for FILE_LAYOUT_INFO_ENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for FILE_LAYOUT_INFO_ENTRY_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FILE_LAYOUT_INFO_ENTRY_0").field("CreationTime", &self.CreationTime).field("LastAccessTime", &self.LastAccessTime).field("LastWriteTime", &self.LastWriteTime).field("ChangeTime", &self.ChangeTime).field("FileAttributes", &self.FileAttributes).finish()
    }
}
unsafe impl ::windows::core::Abi for FILE_LAYOUT_INFO_ENTRY_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for FILE_LAYOUT_INFO_ENTRY_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FILE_LAYOUT_INFO_ENTRY_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for FILE_LAYOUT_INFO_ENTRY_0 {}
impl ::core::default::Default for FILE_LAYOUT_INFO_ENTRY_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for FILE_LAYOUT_NAME_ENTRY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FILE_LAYOUT_NAME_ENTRY").field("NextNameOffset", &self.NextNameOffset).field("Flags", &self.Flags).field("ParentFileReferenceNumber", &self.ParentFileReferenceNumber).field("FileNameLength", &self.FileNameLength).field("Reserved", &self.Reserved).field("FileName", &self.FileName).finish()
    }
}
unsafe impl ::windows::core::Abi for FILE_LAYOUT_NAME_ENTRY {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for FILE_LAYOUT_NAME_ENTRY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FILE_LAYOUT_NAME_ENTRY>()) == 0 }
    }
}
impl ::core::cmp::Eq for FILE_LAYOUT_NAME_ENTRY {}
impl ::core::default::Default for FILE_LAYOUT_NAME_ENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for FILE_LEVEL_TRIM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FILE_LEVEL_TRIM").field("Key", &self.Key).field("NumRanges", &self.NumRanges).field("Ranges", &self.Ranges).finish()
    }
}
unsafe impl ::windows::core::Abi for FILE_LEVEL_TRIM {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for FILE_LEVEL_TRIM {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FILE_LEVEL_TRIM>()) == 0 }
    }
}
impl ::core::cmp::Eq for FILE_LEVEL_TRIM {}
impl ::core::default::Default for FILE_LEVEL_TRIM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for FILE_LEVEL_TRIM_OUTPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FILE_LEVEL_TRIM_OUTPUT").field("NumRangesProcessed", &self.NumRangesProcessed).finish()
    }
}
unsafe impl ::windows::core::Abi for FILE_LEVEL_TRIM_OUTPUT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for FILE_LEVEL_TRIM_OUTPUT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FILE_LEVEL_TRIM_OUTPUT>()) == 0 }
    }
}
impl ::core::cmp::Eq for FILE_LEVEL_TRIM_OUTPUT {}
impl ::core::default::Default for FILE_LEVEL_TRIM_OUTPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for FILE_LEVEL_TRIM_RANGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FILE_LEVEL_TRIM_RANGE").field("Offset", &self.Offset).field("Length", &self.Length).finish()
    }
}
unsafe impl ::windows::core::Abi for FILE_LEVEL_TRIM_RANGE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for FILE_LEVEL_TRIM_RANGE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FILE_LEVEL_TRIM_RANGE>()) == 0 }
    }
}
impl ::core::cmp::Eq for FILE_LEVEL_TRIM_RANGE {}
impl ::core::default::Default for FILE_LEVEL_TRIM_RANGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for FILE_MAKE_COMPATIBLE_BUFFER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FILE_MAKE_COMPATIBLE_BUFFER").field("CloseDisc", &self.CloseDisc).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for FILE_MAKE_COMPATIBLE_BUFFER {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for FILE_MAKE_COMPATIBLE_BUFFER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FILE_MAKE_COMPATIBLE_BUFFER>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for FILE_MAKE_COMPATIBLE_BUFFER {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for FILE_MAKE_COMPATIBLE_BUFFER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
unsafe impl ::windows::core::Abi for FILE_OBJECTID_BUFFER {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for FILE_OBJECTID_BUFFER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FILE_OBJECTID_BUFFER>()) == 0 }
    }
}
impl ::core::cmp::Eq for FILE_OBJECTID_BUFFER {}
impl ::core::default::Default for FILE_OBJECTID_BUFFER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
unsafe impl ::windows::core::Abi for FILE_OBJECTID_BUFFER_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for FILE_OBJECTID_BUFFER_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FILE_OBJECTID_BUFFER_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for FILE_OBJECTID_BUFFER_0 {}
impl ::core::default::Default for FILE_OBJECTID_BUFFER_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for FILE_OBJECTID_BUFFER_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FILE_OBJECTID_BUFFER_0_0").field("BirthVolumeId", &self.BirthVolumeId).field("BirthObjectId", &self.BirthObjectId).field("DomainId", &self.DomainId).finish()
    }
}
unsafe impl ::windows::core::Abi for FILE_OBJECTID_BUFFER_0_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for FILE_OBJECTID_BUFFER_0_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FILE_OBJECTID_BUFFER_0_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for FILE_OBJECTID_BUFFER_0_0 {}
impl ::core::default::Default for FILE_OBJECTID_BUFFER_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for FILE_PREFETCH {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FILE_PREFETCH").field("Type", &self.Type).field("Count", &self.Count).field("Prefetch", &self.Prefetch).finish()
    }
}
unsafe impl ::windows::core::Abi for FILE_PREFETCH {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for FILE_PREFETCH {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FILE_PREFETCH>()) == 0 }
    }
}
impl ::core::cmp::Eq for FILE_PREFETCH {}
impl ::core::default::Default for FILE_PREFETCH {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for FILE_PREFETCH_EX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FILE_PREFETCH_EX").field("Type", &self.Type).field("Count", &self.Count).field("Context", &self.Context).field("Prefetch", &self.Prefetch).finish()
    }
}
unsafe impl ::windows::core::Abi for FILE_PREFETCH_EX {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for FILE_PREFETCH_EX {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FILE_PREFETCH_EX>()) == 0 }
    }
}
impl ::core::cmp::Eq for FILE_PREFETCH_EX {}
impl ::core::default::Default for FILE_PREFETCH_EX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for FILE_PROVIDER_EXTERNAL_INFO_V0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FILE_PROVIDER_EXTERNAL_INFO_V0").field("Version", &self.Version).field("Algorithm", &self.Algorithm).finish()
    }
}
unsafe impl ::windows::core::Abi for FILE_PROVIDER_EXTERNAL_INFO_V0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for FILE_PROVIDER_EXTERNAL_INFO_V0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FILE_PROVIDER_EXTERNAL_INFO_V0>()) == 0 }
    }
}
impl ::core::cmp::Eq for FILE_PROVIDER_EXTERNAL_INFO_V0 {}
impl ::core::default::Default for FILE_PROVIDER_EXTERNAL_INFO_V0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for FILE_PROVIDER_EXTERNAL_INFO_V1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FILE_PROVIDER_EXTERNAL_INFO_V1").field("Version", &self.Version).field("Algorithm", &self.Algorithm).field("Flags", &self.Flags).finish()
    }
}
unsafe impl ::windows::core::Abi for FILE_PROVIDER_EXTERNAL_INFO_V1 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for FILE_PROVIDER_EXTERNAL_INFO_V1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FILE_PROVIDER_EXTERNAL_INFO_V1>()) == 0 }
    }
}
impl ::core::cmp::Eq for FILE_PROVIDER_EXTERNAL_INFO_V1 {}
impl ::core::default::Default for FILE_PROVIDER_EXTERNAL_INFO_V1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for FILE_QUERY_ON_DISK_VOL_INFO_BUFFER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FILE_QUERY_ON_DISK_VOL_INFO_BUFFER")
            .field("DirectoryCount", &self.DirectoryCount)
            .field("FileCount", &self.FileCount)
            .field("FsFormatMajVersion", &self.FsFormatMajVersion)
            .field("FsFormatMinVersion", &self.FsFormatMinVersion)
            .field("FsFormatName", &self.FsFormatName)
            .field("FormatTime", &self.FormatTime)
            .field("LastUpdateTime", &self.LastUpdateTime)
            .field("CopyrightInfo", &self.CopyrightInfo)
            .field("AbstractInfo", &self.AbstractInfo)
            .field("FormattingImplementationInfo", &self.FormattingImplementationInfo)
            .field("LastModifyingImplementationInfo", &self.LastModifyingImplementationInfo)
            .finish()
    }
}
unsafe impl ::windows::core::Abi for FILE_QUERY_ON_DISK_VOL_INFO_BUFFER {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for FILE_QUERY_ON_DISK_VOL_INFO_BUFFER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FILE_QUERY_ON_DISK_VOL_INFO_BUFFER>()) == 0 }
    }
}
impl ::core::cmp::Eq for FILE_QUERY_ON_DISK_VOL_INFO_BUFFER {}
impl ::core::default::Default for FILE_QUERY_ON_DISK_VOL_INFO_BUFFER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for FILE_QUERY_SPARING_BUFFER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FILE_QUERY_SPARING_BUFFER").field("SparingUnitBytes", &self.SparingUnitBytes).field("SoftwareSparing", &self.SoftwareSparing).field("TotalSpareBlocks", &self.TotalSpareBlocks).field("FreeSpareBlocks", &self.FreeSpareBlocks).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for FILE_QUERY_SPARING_BUFFER {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for FILE_QUERY_SPARING_BUFFER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FILE_QUERY_SPARING_BUFFER>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for FILE_QUERY_SPARING_BUFFER {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for FILE_QUERY_SPARING_BUFFER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for FILE_REFERENCE_RANGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FILE_REFERENCE_RANGE").field("StartingFileReferenceNumber", &self.StartingFileReferenceNumber).field("EndingFileReferenceNumber", &self.EndingFileReferenceNumber).finish()
    }
}
unsafe impl ::windows::core::Abi for FILE_REFERENCE_RANGE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for FILE_REFERENCE_RANGE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FILE_REFERENCE_RANGE>()) == 0 }
    }
}
impl ::core::cmp::Eq for FILE_REFERENCE_RANGE {}
impl ::core::default::Default for FILE_REFERENCE_RANGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for FILE_REGION_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FILE_REGION_INFO").field("FileOffset", &self.FileOffset).field("Length", &self.Length).field("Usage", &self.Usage).field("Reserved", &self.Reserved).finish()
    }
}
unsafe impl ::windows::core::Abi for FILE_REGION_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for FILE_REGION_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FILE_REGION_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for FILE_REGION_INFO {}
impl ::core::default::Default for FILE_REGION_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for FILE_REGION_INPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FILE_REGION_INPUT").field("FileOffset", &self.FileOffset).field("Length", &self.Length).field("DesiredUsage", &self.DesiredUsage).finish()
    }
}
unsafe impl ::windows::core::Abi for FILE_REGION_INPUT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for FILE_REGION_INPUT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FILE_REGION_INPUT>()) == 0 }
    }
}
impl ::core::cmp::Eq for FILE_REGION_INPUT {}
impl ::core::default::Default for FILE_REGION_INPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for FILE_REGION_OUTPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FILE_REGION_OUTPUT").field("Flags", &self.Flags).field("TotalRegionEntryCount", &self.TotalRegionEntryCount).field("RegionEntryCount", &self.RegionEntryCount).field("Reserved", &self.Reserved).field("Region", &self.Region).finish()
    }
}
unsafe impl ::windows::core::Abi for FILE_REGION_OUTPUT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for FILE_REGION_OUTPUT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FILE_REGION_OUTPUT>()) == 0 }
    }
}
impl ::core::cmp::Eq for FILE_REGION_OUTPUT {}
impl ::core::default::Default for FILE_REGION_OUTPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for FILE_SET_DEFECT_MGMT_BUFFER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FILE_SET_DEFECT_MGMT_BUFFER").field("Disable", &self.Disable).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for FILE_SET_DEFECT_MGMT_BUFFER {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for FILE_SET_DEFECT_MGMT_BUFFER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FILE_SET_DEFECT_MGMT_BUFFER>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for FILE_SET_DEFECT_MGMT_BUFFER {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for FILE_SET_DEFECT_MGMT_BUFFER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for FILE_SET_SPARSE_BUFFER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FILE_SET_SPARSE_BUFFER").field("SetSparse", &self.SetSparse).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for FILE_SET_SPARSE_BUFFER {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for FILE_SET_SPARSE_BUFFER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FILE_SET_SPARSE_BUFFER>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for FILE_SET_SPARSE_BUFFER {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for FILE_SET_SPARSE_BUFFER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FILE_SPECIAL_ACCESS: u32 = 0u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct FILE_STORAGE_TIER {
    pub Id: ::windows::core::GUID,
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
impl ::core::fmt::Debug for FILE_STORAGE_TIER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FILE_STORAGE_TIER").field("Id", &self.Id).field("Name", &self.Name).field("Description", &self.Description).field("Flags", &self.Flags).field("ProvisionedCapacity", &self.ProvisionedCapacity).field("MediaType", &self.MediaType).field("Class", &self.Class).finish()
    }
}
unsafe impl ::windows::core::Abi for FILE_STORAGE_TIER {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for FILE_STORAGE_TIER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FILE_STORAGE_TIER>()) == 0 }
    }
}
impl ::core::cmp::Eq for FILE_STORAGE_TIER {}
impl ::core::default::Default for FILE_STORAGE_TIER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FILE_STORAGE_TIER_CLASS(pub i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FileStorageTierClassUnspecified: FILE_STORAGE_TIER_CLASS = FILE_STORAGE_TIER_CLASS(0i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FileStorageTierClassCapacity: FILE_STORAGE_TIER_CLASS = FILE_STORAGE_TIER_CLASS(1i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FileStorageTierClassPerformance: FILE_STORAGE_TIER_CLASS = FILE_STORAGE_TIER_CLASS(2i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FileStorageTierClassMax: FILE_STORAGE_TIER_CLASS = FILE_STORAGE_TIER_CLASS(3i32);
impl ::core::marker::Copy for FILE_STORAGE_TIER_CLASS {}
impl ::core::clone::Clone for FILE_STORAGE_TIER_CLASS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FILE_STORAGE_TIER_CLASS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for FILE_STORAGE_TIER_CLASS {
    type Abi = Self;
}
impl ::core::fmt::Debug for FILE_STORAGE_TIER_CLASS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FILE_STORAGE_TIER_CLASS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FILE_STORAGE_TIER_DESCRIPTION_LENGTH: u32 = 512u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FILE_STORAGE_TIER_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FILE_STORAGE_TIER_FLAG_NO_SEEK_PENALTY: FILE_STORAGE_TIER_FLAGS = FILE_STORAGE_TIER_FLAGS(131072u32);
impl ::core::marker::Copy for FILE_STORAGE_TIER_FLAGS {}
impl ::core::clone::Clone for FILE_STORAGE_TIER_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FILE_STORAGE_TIER_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for FILE_STORAGE_TIER_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for FILE_STORAGE_TIER_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FILE_STORAGE_TIER_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for FILE_STORAGE_TIER_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for FILE_STORAGE_TIER_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for FILE_STORAGE_TIER_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for FILE_STORAGE_TIER_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for FILE_STORAGE_TIER_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FILE_STORAGE_TIER_FLAG_PARITY: u32 = 8388608u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FILE_STORAGE_TIER_FLAG_READ_CACHE: u32 = 4194304u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FILE_STORAGE_TIER_FLAG_SMR: u32 = 16777216u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FILE_STORAGE_TIER_FLAG_WRITE_BACK_CACHE: u32 = 2097152u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FILE_STORAGE_TIER_MEDIA_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FileStorageTierMediaTypeUnspecified: FILE_STORAGE_TIER_MEDIA_TYPE = FILE_STORAGE_TIER_MEDIA_TYPE(0i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FileStorageTierMediaTypeDisk: FILE_STORAGE_TIER_MEDIA_TYPE = FILE_STORAGE_TIER_MEDIA_TYPE(1i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FileStorageTierMediaTypeSsd: FILE_STORAGE_TIER_MEDIA_TYPE = FILE_STORAGE_TIER_MEDIA_TYPE(2i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FileStorageTierMediaTypeScm: FILE_STORAGE_TIER_MEDIA_TYPE = FILE_STORAGE_TIER_MEDIA_TYPE(4i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FileStorageTierMediaTypeMax: FILE_STORAGE_TIER_MEDIA_TYPE = FILE_STORAGE_TIER_MEDIA_TYPE(5i32);
impl ::core::marker::Copy for FILE_STORAGE_TIER_MEDIA_TYPE {}
impl ::core::clone::Clone for FILE_STORAGE_TIER_MEDIA_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FILE_STORAGE_TIER_MEDIA_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for FILE_STORAGE_TIER_MEDIA_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for FILE_STORAGE_TIER_MEDIA_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FILE_STORAGE_TIER_MEDIA_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FILE_STORAGE_TIER_NAME_LENGTH: u32 = 256u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct FILE_STORAGE_TIER_REGION {
    pub TierId: ::windows::core::GUID,
    pub Offset: u64,
    pub Length: u64,
}
impl ::core::marker::Copy for FILE_STORAGE_TIER_REGION {}
impl ::core::clone::Clone for FILE_STORAGE_TIER_REGION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for FILE_STORAGE_TIER_REGION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FILE_STORAGE_TIER_REGION").field("TierId", &self.TierId).field("Offset", &self.Offset).field("Length", &self.Length).finish()
    }
}
unsafe impl ::windows::core::Abi for FILE_STORAGE_TIER_REGION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for FILE_STORAGE_TIER_REGION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FILE_STORAGE_TIER_REGION>()) == 0 }
    }
}
impl ::core::cmp::Eq for FILE_STORAGE_TIER_REGION {}
impl ::core::default::Default for FILE_STORAGE_TIER_REGION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for FILE_SYSTEM_RECOGNITION_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FILE_SYSTEM_RECOGNITION_INFORMATION").field("FileSystem", &self.FileSystem).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for FILE_SYSTEM_RECOGNITION_INFORMATION {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for FILE_SYSTEM_RECOGNITION_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FILE_SYSTEM_RECOGNITION_INFORMATION>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for FILE_SYSTEM_RECOGNITION_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for FILE_SYSTEM_RECOGNITION_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FILE_TYPE_NOTIFICATION_FLAG_USAGE_BEGIN: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FILE_TYPE_NOTIFICATION_FLAG_USAGE_END: u32 = 2u32;
pub const FILE_TYPE_NOTIFICATION_GUID_CRASHDUMP_FILE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9d453eb7_d2a6_4dbd_a2e3_fbd0ed9109a9);
pub const FILE_TYPE_NOTIFICATION_GUID_HIBERNATION_FILE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb7624d64_b9a3_4cf8_8011_5b86c940e7b7);
pub const FILE_TYPE_NOTIFICATION_GUID_PAGE_FILE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0d0a64a1_38fc_4db8_9fe7_3f4352cd7c5c);
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct FILE_TYPE_NOTIFICATION_INPUT {
    pub Flags: u32,
    pub NumFileTypeIDs: u32,
    pub FileTypeID: [::windows::core::GUID; 1],
}
impl ::core::marker::Copy for FILE_TYPE_NOTIFICATION_INPUT {}
impl ::core::clone::Clone for FILE_TYPE_NOTIFICATION_INPUT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for FILE_TYPE_NOTIFICATION_INPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FILE_TYPE_NOTIFICATION_INPUT").field("Flags", &self.Flags).field("NumFileTypeIDs", &self.NumFileTypeIDs).field("FileTypeID", &self.FileTypeID).finish()
    }
}
unsafe impl ::windows::core::Abi for FILE_TYPE_NOTIFICATION_INPUT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for FILE_TYPE_NOTIFICATION_INPUT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FILE_TYPE_NOTIFICATION_INPUT>()) == 0 }
    }
}
impl ::core::cmp::Eq for FILE_TYPE_NOTIFICATION_INPUT {}
impl ::core::default::Default for FILE_TYPE_NOTIFICATION_INPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for FILE_ZERO_DATA_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FILE_ZERO_DATA_INFORMATION").field("FileOffset", &self.FileOffset).field("BeyondFinalZero", &self.BeyondFinalZero).finish()
    }
}
unsafe impl ::windows::core::Abi for FILE_ZERO_DATA_INFORMATION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for FILE_ZERO_DATA_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FILE_ZERO_DATA_INFORMATION>()) == 0 }
    }
}
impl ::core::cmp::Eq for FILE_ZERO_DATA_INFORMATION {}
impl ::core::default::Default for FILE_ZERO_DATA_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for FILE_ZERO_DATA_INFORMATION_EX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FILE_ZERO_DATA_INFORMATION_EX").field("FileOffset", &self.FileOffset).field("BeyondFinalZero", &self.BeyondFinalZero).field("Flags", &self.Flags).finish()
    }
}
unsafe impl ::windows::core::Abi for FILE_ZERO_DATA_INFORMATION_EX {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for FILE_ZERO_DATA_INFORMATION_EX {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FILE_ZERO_DATA_INFORMATION_EX>()) == 0 }
    }
}
impl ::core::cmp::Eq for FILE_ZERO_DATA_INFORMATION_EX {}
impl ::core::default::Default for FILE_ZERO_DATA_INFORMATION_EX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
#[cfg(feature = "Win32_Security")]
impl ::core::fmt::Debug for FIND_BY_SID_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FIND_BY_SID_DATA").field("Restart", &self.Restart).field("Sid", &self.Sid).finish()
    }
}
#[cfg(feature = "Win32_Security")]
unsafe impl ::windows::core::Abi for FIND_BY_SID_DATA {
    type Abi = Self;
}
#[cfg(feature = "Win32_Security")]
impl ::core::cmp::PartialEq for FIND_BY_SID_DATA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FIND_BY_SID_DATA>()) == 0 }
    }
}
#[cfg(feature = "Win32_Security")]
impl ::core::cmp::Eq for FIND_BY_SID_DATA {}
#[cfg(feature = "Win32_Security")]
impl ::core::default::Default for FIND_BY_SID_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for FIND_BY_SID_OUTPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FIND_BY_SID_OUTPUT").field("NextEntryOffset", &self.NextEntryOffset).field("FileIndex", &self.FileIndex).field("FileNameLength", &self.FileNameLength).field("FileName", &self.FileName).finish()
    }
}
unsafe impl ::windows::core::Abi for FIND_BY_SID_OUTPUT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for FIND_BY_SID_OUTPUT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FIND_BY_SID_OUTPUT>()) == 0 }
    }
}
impl ::core::cmp::Eq for FIND_BY_SID_OUTPUT {}
impl ::core::default::Default for FIND_BY_SID_OUTPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for FORMAT_EX_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FORMAT_EX_PARAMETERS").field("MediaType", &self.MediaType).field("StartCylinderNumber", &self.StartCylinderNumber).field("EndCylinderNumber", &self.EndCylinderNumber).field("StartHeadNumber", &self.StartHeadNumber).field("EndHeadNumber", &self.EndHeadNumber).field("FormatGapLength", &self.FormatGapLength).field("SectorsPerTrack", &self.SectorsPerTrack).field("SectorNumber", &self.SectorNumber).finish()
    }
}
unsafe impl ::windows::core::Abi for FORMAT_EX_PARAMETERS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for FORMAT_EX_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FORMAT_EX_PARAMETERS>()) == 0 }
    }
}
impl ::core::cmp::Eq for FORMAT_EX_PARAMETERS {}
impl ::core::default::Default for FORMAT_EX_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for FORMAT_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FORMAT_PARAMETERS").field("MediaType", &self.MediaType).field("StartCylinderNumber", &self.StartCylinderNumber).field("EndCylinderNumber", &self.EndCylinderNumber).field("StartHeadNumber", &self.StartHeadNumber).field("EndHeadNumber", &self.EndHeadNumber).finish()
    }
}
unsafe impl ::windows::core::Abi for FORMAT_PARAMETERS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for FORMAT_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FORMAT_PARAMETERS>()) == 0 }
    }
}
impl ::core::cmp::Eq for FORMAT_PARAMETERS {}
impl ::core::default::Default for FORMAT_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for FSCTL_GET_INTEGRITY_INFORMATION_BUFFER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FSCTL_GET_INTEGRITY_INFORMATION_BUFFER").field("ChecksumAlgorithm", &self.ChecksumAlgorithm).field("Reserved", &self.Reserved).field("Flags", &self.Flags).field("ChecksumChunkSizeInBytes", &self.ChecksumChunkSizeInBytes).field("ClusterSizeInBytes", &self.ClusterSizeInBytes).finish()
    }
}
unsafe impl ::windows::core::Abi for FSCTL_GET_INTEGRITY_INFORMATION_BUFFER {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for FSCTL_GET_INTEGRITY_INFORMATION_BUFFER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FSCTL_GET_INTEGRITY_INFORMATION_BUFFER>()) == 0 }
    }
}
impl ::core::cmp::Eq for FSCTL_GET_INTEGRITY_INFORMATION_BUFFER {}
impl ::core::default::Default for FSCTL_GET_INTEGRITY_INFORMATION_BUFFER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for FSCTL_OFFLOAD_READ_INPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FSCTL_OFFLOAD_READ_INPUT").field("Size", &self.Size).field("Flags", &self.Flags).field("TokenTimeToLive", &self.TokenTimeToLive).field("Reserved", &self.Reserved).field("FileOffset", &self.FileOffset).field("CopyLength", &self.CopyLength).finish()
    }
}
unsafe impl ::windows::core::Abi for FSCTL_OFFLOAD_READ_INPUT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for FSCTL_OFFLOAD_READ_INPUT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FSCTL_OFFLOAD_READ_INPUT>()) == 0 }
    }
}
impl ::core::cmp::Eq for FSCTL_OFFLOAD_READ_INPUT {}
impl ::core::default::Default for FSCTL_OFFLOAD_READ_INPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for FSCTL_OFFLOAD_READ_OUTPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FSCTL_OFFLOAD_READ_OUTPUT").field("Size", &self.Size).field("Flags", &self.Flags).field("TransferLength", &self.TransferLength).field("Token", &self.Token).finish()
    }
}
unsafe impl ::windows::core::Abi for FSCTL_OFFLOAD_READ_OUTPUT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for FSCTL_OFFLOAD_READ_OUTPUT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FSCTL_OFFLOAD_READ_OUTPUT>()) == 0 }
    }
}
impl ::core::cmp::Eq for FSCTL_OFFLOAD_READ_OUTPUT {}
impl ::core::default::Default for FSCTL_OFFLOAD_READ_OUTPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for FSCTL_OFFLOAD_WRITE_INPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FSCTL_OFFLOAD_WRITE_INPUT").field("Size", &self.Size).field("Flags", &self.Flags).field("FileOffset", &self.FileOffset).field("CopyLength", &self.CopyLength).field("TransferOffset", &self.TransferOffset).field("Token", &self.Token).finish()
    }
}
unsafe impl ::windows::core::Abi for FSCTL_OFFLOAD_WRITE_INPUT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for FSCTL_OFFLOAD_WRITE_INPUT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FSCTL_OFFLOAD_WRITE_INPUT>()) == 0 }
    }
}
impl ::core::cmp::Eq for FSCTL_OFFLOAD_WRITE_INPUT {}
impl ::core::default::Default for FSCTL_OFFLOAD_WRITE_INPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for FSCTL_OFFLOAD_WRITE_OUTPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FSCTL_OFFLOAD_WRITE_OUTPUT").field("Size", &self.Size).field("Flags", &self.Flags).field("LengthWritten", &self.LengthWritten).finish()
    }
}
unsafe impl ::windows::core::Abi for FSCTL_OFFLOAD_WRITE_OUTPUT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for FSCTL_OFFLOAD_WRITE_OUTPUT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FSCTL_OFFLOAD_WRITE_OUTPUT>()) == 0 }
    }
}
impl ::core::cmp::Eq for FSCTL_OFFLOAD_WRITE_OUTPUT {}
impl ::core::default::Default for FSCTL_OFFLOAD_WRITE_OUTPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for FSCTL_QUERY_FAT_BPB_BUFFER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FSCTL_QUERY_FAT_BPB_BUFFER").field("First0x24BytesOfBootSector", &self.First0x24BytesOfBootSector).finish()
    }
}
unsafe impl ::windows::core::Abi for FSCTL_QUERY_FAT_BPB_BUFFER {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for FSCTL_QUERY_FAT_BPB_BUFFER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FSCTL_QUERY_FAT_BPB_BUFFER>()) == 0 }
    }
}
impl ::core::cmp::Eq for FSCTL_QUERY_FAT_BPB_BUFFER {}
impl ::core::default::Default for FSCTL_QUERY_FAT_BPB_BUFFER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
    pub TierIds: [::windows::core::GUID; 1],
}
impl ::core::marker::Copy for FSCTL_QUERY_REGION_INFO_INPUT {}
impl ::core::clone::Clone for FSCTL_QUERY_REGION_INFO_INPUT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for FSCTL_QUERY_REGION_INFO_INPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FSCTL_QUERY_REGION_INFO_INPUT").field("Version", &self.Version).field("Size", &self.Size).field("Flags", &self.Flags).field("NumberOfTierIds", &self.NumberOfTierIds).field("TierIds", &self.TierIds).finish()
    }
}
unsafe impl ::windows::core::Abi for FSCTL_QUERY_REGION_INFO_INPUT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for FSCTL_QUERY_REGION_INFO_INPUT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FSCTL_QUERY_REGION_INFO_INPUT>()) == 0 }
    }
}
impl ::core::cmp::Eq for FSCTL_QUERY_REGION_INFO_INPUT {}
impl ::core::default::Default for FSCTL_QUERY_REGION_INFO_INPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for FSCTL_QUERY_REGION_INFO_OUTPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FSCTL_QUERY_REGION_INFO_OUTPUT").field("Version", &self.Version).field("Size", &self.Size).field("Flags", &self.Flags).field("Reserved", &self.Reserved).field("Alignment", &self.Alignment).field("TotalNumberOfRegions", &self.TotalNumberOfRegions).field("NumberOfRegionsReturned", &self.NumberOfRegionsReturned).field("Regions", &self.Regions).finish()
    }
}
unsafe impl ::windows::core::Abi for FSCTL_QUERY_REGION_INFO_OUTPUT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for FSCTL_QUERY_REGION_INFO_OUTPUT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FSCTL_QUERY_REGION_INFO_OUTPUT>()) == 0 }
    }
}
impl ::core::cmp::Eq for FSCTL_QUERY_REGION_INFO_OUTPUT {}
impl ::core::default::Default for FSCTL_QUERY_REGION_INFO_OUTPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for FSCTL_QUERY_STORAGE_CLASSES_OUTPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FSCTL_QUERY_STORAGE_CLASSES_OUTPUT").field("Version", &self.Version).field("Size", &self.Size).field("Flags", &self.Flags).field("TotalNumberOfTiers", &self.TotalNumberOfTiers).field("NumberOfTiersReturned", &self.NumberOfTiersReturned).field("Tiers", &self.Tiers).finish()
    }
}
unsafe impl ::windows::core::Abi for FSCTL_QUERY_STORAGE_CLASSES_OUTPUT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for FSCTL_QUERY_STORAGE_CLASSES_OUTPUT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FSCTL_QUERY_STORAGE_CLASSES_OUTPUT>()) == 0 }
    }
}
impl ::core::cmp::Eq for FSCTL_QUERY_STORAGE_CLASSES_OUTPUT {}
impl ::core::default::Default for FSCTL_QUERY_STORAGE_CLASSES_OUTPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for FSCTL_SET_INTEGRITY_INFORMATION_BUFFER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FSCTL_SET_INTEGRITY_INFORMATION_BUFFER").field("ChecksumAlgorithm", &self.ChecksumAlgorithm).field("Reserved", &self.Reserved).field("Flags", &self.Flags).finish()
    }
}
unsafe impl ::windows::core::Abi for FSCTL_SET_INTEGRITY_INFORMATION_BUFFER {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for FSCTL_SET_INTEGRITY_INFORMATION_BUFFER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FSCTL_SET_INTEGRITY_INFORMATION_BUFFER>()) == 0 }
    }
}
impl ::core::cmp::Eq for FSCTL_SET_INTEGRITY_INFORMATION_BUFFER {}
impl ::core::default::Default for FSCTL_SET_INTEGRITY_INFORMATION_BUFFER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for FSCTL_SET_INTEGRITY_INFORMATION_BUFFER_EX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FSCTL_SET_INTEGRITY_INFORMATION_BUFFER_EX").field("EnableIntegrity", &self.EnableIntegrity).field("KeepIntegrityStateUnchanged", &self.KeepIntegrityStateUnchanged).field("Reserved", &self.Reserved).field("Flags", &self.Flags).field("Version", &self.Version).field("Reserved2", &self.Reserved2).finish()
    }
}
unsafe impl ::windows::core::Abi for FSCTL_SET_INTEGRITY_INFORMATION_BUFFER_EX {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for FSCTL_SET_INTEGRITY_INFORMATION_BUFFER_EX {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FSCTL_SET_INTEGRITY_INFORMATION_BUFFER_EX>()) == 0 }
    }
}
impl ::core::cmp::Eq for FSCTL_SET_INTEGRITY_INFORMATION_BUFFER_EX {}
impl ::core::default::Default for FSCTL_SET_INTEGRITY_INFORMATION_BUFFER_EX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FS_BPIO_INFLAGS(pub i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSBPIO_INFL_None: FS_BPIO_INFLAGS = FS_BPIO_INFLAGS(0i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSBPIO_INFL_SKIP_STORAGE_STACK_QUERY: FS_BPIO_INFLAGS = FS_BPIO_INFLAGS(1i32);
impl ::core::marker::Copy for FS_BPIO_INFLAGS {}
impl ::core::clone::Clone for FS_BPIO_INFLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FS_BPIO_INFLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for FS_BPIO_INFLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for FS_BPIO_INFLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FS_BPIO_INFLAGS").field(&self.0).finish()
    }
}
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
impl ::core::fmt::Debug for FS_BPIO_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FS_BPIO_INFO").field("ActiveBypassIoCount", &self.ActiveBypassIoCount).field("StorageDriverNameLen", &self.StorageDriverNameLen).field("StorageDriverName", &self.StorageDriverName).finish()
    }
}
unsafe impl ::windows::core::Abi for FS_BPIO_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for FS_BPIO_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FS_BPIO_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for FS_BPIO_INFO {}
impl ::core::default::Default for FS_BPIO_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for FS_BPIO_INPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FS_BPIO_INPUT").field("Operation", &self.Operation).field("InFlags", &self.InFlags).field("Reserved1", &self.Reserved1).field("Reserved2", &self.Reserved2).finish()
    }
}
unsafe impl ::windows::core::Abi for FS_BPIO_INPUT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for FS_BPIO_INPUT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FS_BPIO_INPUT>()) == 0 }
    }
}
impl ::core::cmp::Eq for FS_BPIO_INPUT {}
impl ::core::default::Default for FS_BPIO_INPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FS_BPIO_OPERATIONS(pub i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FS_BPIO_OP_ENABLE: FS_BPIO_OPERATIONS = FS_BPIO_OPERATIONS(1i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FS_BPIO_OP_DISABLE: FS_BPIO_OPERATIONS = FS_BPIO_OPERATIONS(2i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FS_BPIO_OP_QUERY: FS_BPIO_OPERATIONS = FS_BPIO_OPERATIONS(3i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FS_BPIO_OP_VOLUME_STACK_PAUSE: FS_BPIO_OPERATIONS = FS_BPIO_OPERATIONS(4i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FS_BPIO_OP_VOLUME_STACK_RESUME: FS_BPIO_OPERATIONS = FS_BPIO_OPERATIONS(5i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FS_BPIO_OP_STREAM_PAUSE: FS_BPIO_OPERATIONS = FS_BPIO_OPERATIONS(6i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FS_BPIO_OP_STREAM_RESUME: FS_BPIO_OPERATIONS = FS_BPIO_OPERATIONS(7i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FS_BPIO_OP_GET_INFO: FS_BPIO_OPERATIONS = FS_BPIO_OPERATIONS(8i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FS_BPIO_OP_MAX_OPERATION: FS_BPIO_OPERATIONS = FS_BPIO_OPERATIONS(9i32);
impl ::core::marker::Copy for FS_BPIO_OPERATIONS {}
impl ::core::clone::Clone for FS_BPIO_OPERATIONS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FS_BPIO_OPERATIONS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for FS_BPIO_OPERATIONS {
    type Abi = Self;
}
impl ::core::fmt::Debug for FS_BPIO_OPERATIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FS_BPIO_OPERATIONS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FS_BPIO_OUTFLAGS(pub i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSBPIO_OUTFL_None: FS_BPIO_OUTFLAGS = FS_BPIO_OUTFLAGS(0i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSBPIO_OUTFL_VOLUME_STACK_BYPASS_PAUSED: FS_BPIO_OUTFLAGS = FS_BPIO_OUTFLAGS(1i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSBPIO_OUTFL_STREAM_BYPASS_PAUSED: FS_BPIO_OUTFLAGS = FS_BPIO_OUTFLAGS(2i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSBPIO_OUTFL_FILTER_ATTACH_BLOCKED: FS_BPIO_OUTFLAGS = FS_BPIO_OUTFLAGS(4i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FSBPIO_OUTFL_COMPATIBLE_STORAGE_DRIVER: FS_BPIO_OUTFLAGS = FS_BPIO_OUTFLAGS(8i32);
impl ::core::marker::Copy for FS_BPIO_OUTFLAGS {}
impl ::core::clone::Clone for FS_BPIO_OUTFLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FS_BPIO_OUTFLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for FS_BPIO_OUTFLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for FS_BPIO_OUTFLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FS_BPIO_OUTFLAGS").field(&self.0).finish()
    }
}
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
unsafe impl ::windows::core::Abi for FS_BPIO_OUTPUT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for FS_BPIO_OUTPUT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FS_BPIO_OUTPUT>()) == 0 }
    }
}
impl ::core::cmp::Eq for FS_BPIO_OUTPUT {}
impl ::core::default::Default for FS_BPIO_OUTPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
unsafe impl ::windows::core::Abi for FS_BPIO_OUTPUT_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for FS_BPIO_OUTPUT_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FS_BPIO_OUTPUT_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for FS_BPIO_OUTPUT_0 {}
impl ::core::default::Default for FS_BPIO_OUTPUT_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for FS_BPIO_RESULTS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FS_BPIO_RESULTS").field("OpStatus", &self.OpStatus).field("FailingDriverNameLen", &self.FailingDriverNameLen).field("FailingDriverName", &self.FailingDriverName).field("FailureReasonLen", &self.FailureReasonLen).field("FailureReason", &self.FailureReason).finish()
    }
}
unsafe impl ::windows::core::Abi for FS_BPIO_RESULTS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for FS_BPIO_RESULTS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FS_BPIO_RESULTS>()) == 0 }
    }
}
impl ::core::cmp::Eq for FS_BPIO_RESULTS {}
impl ::core::default::Default for FS_BPIO_RESULTS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
unsafe impl ::windows::core::Abi for GETVERSIONINPARAMS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for GETVERSIONINPARAMS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<GETVERSIONINPARAMS>()) == 0 }
    }
}
impl ::core::cmp::Eq for GETVERSIONINPARAMS {}
impl ::core::default::Default for GETVERSIONINPARAMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for GET_CHANGER_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GET_CHANGER_PARAMETERS")
            .field("Size", &self.Size)
            .field("NumberTransportElements", &self.NumberTransportElements)
            .field("NumberStorageElements", &self.NumberStorageElements)
            .field("NumberCleanerSlots", &self.NumberCleanerSlots)
            .field("NumberIEElements", &self.NumberIEElements)
            .field("NumberDataTransferElements", &self.NumberDataTransferElements)
            .field("NumberOfDoors", &self.NumberOfDoors)
            .field("FirstSlotNumber", &self.FirstSlotNumber)
            .field("FirstDriveNumber", &self.FirstDriveNumber)
            .field("FirstTransportNumber", &self.FirstTransportNumber)
            .field("FirstIEPortNumber", &self.FirstIEPortNumber)
            .field("FirstCleanerSlotAddress", &self.FirstCleanerSlotAddress)
            .field("MagazineSize", &self.MagazineSize)
            .field("DriveCleanTimeout", &self.DriveCleanTimeout)
            .field("Features0", &self.Features0)
            .field("Features1", &self.Features1)
            .field("MoveFromTransport", &self.MoveFromTransport)
            .field("MoveFromSlot", &self.MoveFromSlot)
            .field("MoveFromIePort", &self.MoveFromIePort)
            .field("MoveFromDrive", &self.MoveFromDrive)
            .field("ExchangeFromTransport", &self.ExchangeFromTransport)
            .field("ExchangeFromSlot", &self.ExchangeFromSlot)
            .field("ExchangeFromIePort", &self.ExchangeFromIePort)
            .field("ExchangeFromDrive", &self.ExchangeFromDrive)
            .field("LockUnlockCapabilities", &self.LockUnlockCapabilities)
            .field("PositionCapabilities", &self.PositionCapabilities)
            .field("Reserved1", &self.Reserved1)
            .field("Reserved2", &self.Reserved2)
            .finish()
    }
}
unsafe impl ::windows::core::Abi for GET_CHANGER_PARAMETERS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for GET_CHANGER_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<GET_CHANGER_PARAMETERS>()) == 0 }
    }
}
impl ::core::cmp::Eq for GET_CHANGER_PARAMETERS {}
impl ::core::default::Default for GET_CHANGER_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct GET_CHANGER_PARAMETERS_FEATURES1(pub u32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const CHANGER_CLEANER_AUTODISMOUNT: GET_CHANGER_PARAMETERS_FEATURES1 = GET_CHANGER_PARAMETERS_FEATURES1(2147483652u32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const CHANGER_CLEANER_OPS_NOT_SUPPORTED: GET_CHANGER_PARAMETERS_FEATURES1 = GET_CHANGER_PARAMETERS_FEATURES1(2147483712u32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const CHANGER_IEPORT_USER_CONTROL_CLOSE: GET_CHANGER_PARAMETERS_FEATURES1 = GET_CHANGER_PARAMETERS_FEATURES1(2147483904u32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const CHANGER_IEPORT_USER_CONTROL_OPEN: GET_CHANGER_PARAMETERS_FEATURES1 = GET_CHANGER_PARAMETERS_FEATURES1(2147483776u32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const CHANGER_MOVE_EXTENDS_IEPORT: GET_CHANGER_PARAMETERS_FEATURES1 = GET_CHANGER_PARAMETERS_FEATURES1(2147484160u32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const CHANGER_MOVE_RETRACTS_IEPORT: GET_CHANGER_PARAMETERS_FEATURES1 = GET_CHANGER_PARAMETERS_FEATURES1(2147484672u32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const CHANGER_PREDISMOUNT_ALIGN_TO_DRIVE: GET_CHANGER_PARAMETERS_FEATURES1 = GET_CHANGER_PARAMETERS_FEATURES1(2147483650u32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const CHANGER_PREDISMOUNT_ALIGN_TO_SLOT: GET_CHANGER_PARAMETERS_FEATURES1 = GET_CHANGER_PARAMETERS_FEATURES1(2147483649u32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const CHANGER_RTN_MEDIA_TO_ORIGINAL_ADDR: GET_CHANGER_PARAMETERS_FEATURES1 = GET_CHANGER_PARAMETERS_FEATURES1(2147483680u32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const CHANGER_SLOTS_USE_TRAYS: GET_CHANGER_PARAMETERS_FEATURES1 = GET_CHANGER_PARAMETERS_FEATURES1(2147483664u32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const CHANGER_TRUE_EXCHANGE_CAPABLE: GET_CHANGER_PARAMETERS_FEATURES1 = GET_CHANGER_PARAMETERS_FEATURES1(2147483656u32);
impl ::core::marker::Copy for GET_CHANGER_PARAMETERS_FEATURES1 {}
impl ::core::clone::Clone for GET_CHANGER_PARAMETERS_FEATURES1 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GET_CHANGER_PARAMETERS_FEATURES1 {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for GET_CHANGER_PARAMETERS_FEATURES1 {
    type Abi = Self;
}
impl ::core::fmt::Debug for GET_CHANGER_PARAMETERS_FEATURES1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GET_CHANGER_PARAMETERS_FEATURES1").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for GET_CHANGER_PARAMETERS_FEATURES1 {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for GET_CHANGER_PARAMETERS_FEATURES1 {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for GET_CHANGER_PARAMETERS_FEATURES1 {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for GET_CHANGER_PARAMETERS_FEATURES1 {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for GET_CHANGER_PARAMETERS_FEATURES1 {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
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
impl ::core::fmt::Debug for GET_DEVICE_INTERNAL_STATUS_DATA_REQUEST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GET_DEVICE_INTERNAL_STATUS_DATA_REQUEST").field("Version", &self.Version).field("Size", &self.Size).field("RequestDataType", &self.RequestDataType).field("RequestDataSet", &self.RequestDataSet).finish()
    }
}
unsafe impl ::windows::core::Abi for GET_DEVICE_INTERNAL_STATUS_DATA_REQUEST {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for GET_DEVICE_INTERNAL_STATUS_DATA_REQUEST {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<GET_DEVICE_INTERNAL_STATUS_DATA_REQUEST>()) == 0 }
    }
}
impl ::core::cmp::Eq for GET_DEVICE_INTERNAL_STATUS_DATA_REQUEST {}
impl ::core::default::Default for GET_DEVICE_INTERNAL_STATUS_DATA_REQUEST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for GET_DISK_ATTRIBUTES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GET_DISK_ATTRIBUTES").field("Version", &self.Version).field("Reserved1", &self.Reserved1).field("Attributes", &self.Attributes).finish()
    }
}
unsafe impl ::windows::core::Abi for GET_DISK_ATTRIBUTES {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for GET_DISK_ATTRIBUTES {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<GET_DISK_ATTRIBUTES>()) == 0 }
    }
}
impl ::core::cmp::Eq for GET_DISK_ATTRIBUTES {}
impl ::core::default::Default for GET_DISK_ATTRIBUTES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for GET_FILTER_FILE_IDENTIFIER_INPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GET_FILTER_FILE_IDENTIFIER_INPUT").field("AltitudeLength", &self.AltitudeLength).field("Altitude", &self.Altitude).finish()
    }
}
unsafe impl ::windows::core::Abi for GET_FILTER_FILE_IDENTIFIER_INPUT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for GET_FILTER_FILE_IDENTIFIER_INPUT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<GET_FILTER_FILE_IDENTIFIER_INPUT>()) == 0 }
    }
}
impl ::core::cmp::Eq for GET_FILTER_FILE_IDENTIFIER_INPUT {}
impl ::core::default::Default for GET_FILTER_FILE_IDENTIFIER_INPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for GET_FILTER_FILE_IDENTIFIER_OUTPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GET_FILTER_FILE_IDENTIFIER_OUTPUT").field("FilterFileIdentifierLength", &self.FilterFileIdentifierLength).field("FilterFileIdentifier", &self.FilterFileIdentifier).finish()
    }
}
unsafe impl ::windows::core::Abi for GET_FILTER_FILE_IDENTIFIER_OUTPUT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for GET_FILTER_FILE_IDENTIFIER_OUTPUT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<GET_FILTER_FILE_IDENTIFIER_OUTPUT>()) == 0 }
    }
}
impl ::core::cmp::Eq for GET_FILTER_FILE_IDENTIFIER_OUTPUT {}
impl ::core::default::Default for GET_FILTER_FILE_IDENTIFIER_OUTPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for GET_LENGTH_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GET_LENGTH_INFORMATION").field("Length", &self.Length).finish()
    }
}
unsafe impl ::windows::core::Abi for GET_LENGTH_INFORMATION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for GET_LENGTH_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<GET_LENGTH_INFORMATION>()) == 0 }
    }
}
impl ::core::cmp::Eq for GET_LENGTH_INFORMATION {}
impl ::core::default::Default for GET_LENGTH_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
#[cfg(feature = "Win32_Storage_FileSystem")]
unsafe impl ::windows::core::Abi for GET_MEDIA_TYPES {
    type Abi = Self;
}
#[cfg(feature = "Win32_Storage_FileSystem")]
impl ::core::cmp::PartialEq for GET_MEDIA_TYPES {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<GET_MEDIA_TYPES>()) == 0 }
    }
}
#[cfg(feature = "Win32_Storage_FileSystem")]
impl ::core::cmp::Eq for GET_MEDIA_TYPES {}
#[cfg(feature = "Win32_Storage_FileSystem")]
impl ::core::default::Default for GET_MEDIA_TYPES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const GET_VOLUME_BITMAP_FLAG_MASK_METADATA: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct GPT_ATTRIBUTES(pub u64);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const GPT_ATTRIBUTE_PLATFORM_REQUIRED: GPT_ATTRIBUTES = GPT_ATTRIBUTES(1u64);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const GPT_BASIC_DATA_ATTRIBUTE_NO_DRIVE_LETTER: GPT_ATTRIBUTES = GPT_ATTRIBUTES(9223372036854775808u64);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const GPT_BASIC_DATA_ATTRIBUTE_HIDDEN: GPT_ATTRIBUTES = GPT_ATTRIBUTES(4611686018427387904u64);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const GPT_BASIC_DATA_ATTRIBUTE_SHADOW_COPY: GPT_ATTRIBUTES = GPT_ATTRIBUTES(2305843009213693952u64);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const GPT_BASIC_DATA_ATTRIBUTE_READ_ONLY: GPT_ATTRIBUTES = GPT_ATTRIBUTES(1152921504606846976u64);
impl ::core::marker::Copy for GPT_ATTRIBUTES {}
impl ::core::clone::Clone for GPT_ATTRIBUTES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GPT_ATTRIBUTES {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for GPT_ATTRIBUTES {
    type Abi = Self;
}
impl ::core::fmt::Debug for GPT_ATTRIBUTES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GPT_ATTRIBUTES").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for GPT_ATTRIBUTES {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for GPT_ATTRIBUTES {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for GPT_ATTRIBUTES {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for GPT_ATTRIBUTES {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for GPT_ATTRIBUTES {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
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
unsafe impl ::windows::core::Abi for GP_LOG_PAGE_DESCRIPTOR {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for GP_LOG_PAGE_DESCRIPTOR {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<GP_LOG_PAGE_DESCRIPTOR>()) == 0 }
    }
}
impl ::core::cmp::Eq for GP_LOG_PAGE_DESCRIPTOR {}
impl ::core::default::Default for GP_LOG_PAGE_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const GUID_DEVICEDUMP_DRIVER_STORAGE_PORT: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xda82441d_7142_4bc1_b844_0807c5a4b67f);
pub const GUID_DEVICEDUMP_STORAGE_DEVICE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd8e2592f_1aab_4d56_a746_1f7585df40f4);
pub const GUID_DEVINTERFACE_CDCHANGER: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x53f56312_b6bf_11d0_94f2_00a0c91efb8b);
pub const GUID_DEVINTERFACE_CDROM: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x53f56308_b6bf_11d0_94f2_00a0c91efb8b);
pub const GUID_DEVINTERFACE_COMPORT: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x86e0d1e0_8089_11d0_9ce4_08003e301f73);
pub const GUID_DEVINTERFACE_DISK: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x53f56307_b6bf_11d0_94f2_00a0c91efb8b);
pub const GUID_DEVINTERFACE_FLOPPY: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x53f56311_b6bf_11d0_94f2_00a0c91efb8b);
pub const GUID_DEVINTERFACE_HIDDEN_VOLUME: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7f108a28_9833_4b3b_b780_2c6b5fa5c062);
pub const GUID_DEVINTERFACE_MEDIUMCHANGER: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x53f56310_b6bf_11d0_94f2_00a0c91efb8b);
pub const GUID_DEVINTERFACE_PARTITION: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x53f5630a_b6bf_11d0_94f2_00a0c91efb8b);
pub const GUID_DEVINTERFACE_SCM_PHYSICAL_DEVICE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4283609d_4dc2_43be_bbb4_4f15dfce2c61);
pub const GUID_DEVINTERFACE_SERENUM_BUS_ENUMERATOR: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4d36e978_e325_11ce_bfc1_08002be10318);
pub const GUID_DEVINTERFACE_SERVICE_VOLUME: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6ead3d82_25ec_46bc_b7fd_c1f0df8f5037);
pub const GUID_DEVINTERFACE_SES: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1790c9ec_47d5_4df3_b5af_9adf3cf23e48);
pub const GUID_DEVINTERFACE_STORAGEPORT: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2accfe60_c130_11d2_b082_00a0c91efb8b);
pub const GUID_DEVINTERFACE_TAPE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x53f5630b_b6bf_11d0_94f2_00a0c91efb8b);
pub const GUID_DEVINTERFACE_UNIFIED_ACCESS_RPMB: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x27447c21_bcc3_4d07_a05b_a3395bb4eee7);
pub const GUID_DEVINTERFACE_VMLUN: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6f416619_9f29_42a5_b20b_37e219ca02b0);
pub const GUID_DEVINTERFACE_VOLUME: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x53f5630d_b6bf_11d0_94f2_00a0c91efb8b);
pub const GUID_DEVINTERFACE_WRITEONCEDISK: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x53f5630c_b6bf_11d0_94f2_00a0c91efb8b);
pub const GUID_DEVINTERFACE_ZNSDISK: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb87941c5_ffdb_43c7_b6b1_20b632f0b109);
pub const GUID_SCM_PD_HEALTH_NOTIFICATION: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9da2d386_72f5_4ee3_8155_eca0678e3b06);
pub const GUID_SCM_PD_PASSTHROUGH_INVDIMM: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4309ac30_0d11_11e4_9191_0800200c9a66);
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
impl ::core::fmt::Debug for HISTOGRAM_BUCKET {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HISTOGRAM_BUCKET").field("Reads", &self.Reads).field("Writes", &self.Writes).finish()
    }
}
unsafe impl ::windows::core::Abi for HISTOGRAM_BUCKET {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for HISTOGRAM_BUCKET {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<HISTOGRAM_BUCKET>()) == 0 }
    }
}
impl ::core::cmp::Eq for HISTOGRAM_BUCKET {}
impl ::core::default::Default for HISTOGRAM_BUCKET {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for IDEREGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IDEREGS").field("bFeaturesReg", &self.bFeaturesReg).field("bSectorCountReg", &self.bSectorCountReg).field("bSectorNumberReg", &self.bSectorNumberReg).field("bCylLowReg", &self.bCylLowReg).field("bCylHighReg", &self.bCylHighReg).field("bDriveHeadReg", &self.bDriveHeadReg).field("bCommandReg", &self.bCommandReg).field("bReserved", &self.bReserved).finish()
    }
}
unsafe impl ::windows::core::Abi for IDEREGS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IDEREGS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IDEREGS>()) == 0 }
    }
}
impl ::core::cmp::Eq for IDEREGS {}
impl ::core::default::Default for IDEREGS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for IO_IRP_EXT_TRACK_OFFSET_HEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IO_IRP_EXT_TRACK_OFFSET_HEADER").field("Validation", &self.Validation).field("Flags", &self.Flags).field("TrackedOffsetCallback", &self.TrackedOffsetCallback.map(|f| f as usize)).finish()
    }
}
unsafe impl ::windows::core::Abi for IO_IRP_EXT_TRACK_OFFSET_HEADER {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IO_IRP_EXT_TRACK_OFFSET_HEADER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IO_IRP_EXT_TRACK_OFFSET_HEADER>()) == 0 }
    }
}
impl ::core::cmp::Eq for IO_IRP_EXT_TRACK_OFFSET_HEADER {}
impl ::core::default::Default for IO_IRP_EXT_TRACK_OFFSET_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for LOOKUP_STREAM_FROM_CLUSTER_ENTRY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("LOOKUP_STREAM_FROM_CLUSTER_ENTRY").field("OffsetToNext", &self.OffsetToNext).field("Flags", &self.Flags).field("Reserved", &self.Reserved).field("Cluster", &self.Cluster).field("FileName", &self.FileName).finish()
    }
}
unsafe impl ::windows::core::Abi for LOOKUP_STREAM_FROM_CLUSTER_ENTRY {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for LOOKUP_STREAM_FROM_CLUSTER_ENTRY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<LOOKUP_STREAM_FROM_CLUSTER_ENTRY>()) == 0 }
    }
}
impl ::core::cmp::Eq for LOOKUP_STREAM_FROM_CLUSTER_ENTRY {}
impl ::core::default::Default for LOOKUP_STREAM_FROM_CLUSTER_ENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for LOOKUP_STREAM_FROM_CLUSTER_INPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("LOOKUP_STREAM_FROM_CLUSTER_INPUT").field("Flags", &self.Flags).field("NumberOfClusters", &self.NumberOfClusters).field("Cluster", &self.Cluster).finish()
    }
}
unsafe impl ::windows::core::Abi for LOOKUP_STREAM_FROM_CLUSTER_INPUT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for LOOKUP_STREAM_FROM_CLUSTER_INPUT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<LOOKUP_STREAM_FROM_CLUSTER_INPUT>()) == 0 }
    }
}
impl ::core::cmp::Eq for LOOKUP_STREAM_FROM_CLUSTER_INPUT {}
impl ::core::default::Default for LOOKUP_STREAM_FROM_CLUSTER_INPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for LOOKUP_STREAM_FROM_CLUSTER_OUTPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("LOOKUP_STREAM_FROM_CLUSTER_OUTPUT").field("Offset", &self.Offset).field("NumberOfMatches", &self.NumberOfMatches).field("BufferSizeRequired", &self.BufferSizeRequired).finish()
    }
}
unsafe impl ::windows::core::Abi for LOOKUP_STREAM_FROM_CLUSTER_OUTPUT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for LOOKUP_STREAM_FROM_CLUSTER_OUTPUT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<LOOKUP_STREAM_FROM_CLUSTER_OUTPUT>()) == 0 }
    }
}
impl ::core::cmp::Eq for LOOKUP_STREAM_FROM_CLUSTER_OUTPUT {}
impl ::core::default::Default for LOOKUP_STREAM_FROM_CLUSTER_OUTPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for MARK_HANDLE_INFO {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MARK_HANDLE_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MARK_HANDLE_INFO>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MARK_HANDLE_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MARK_HANDLE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for MARK_HANDLE_INFO_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MARK_HANDLE_INFO_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MARK_HANDLE_INFO_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MARK_HANDLE_INFO_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MARK_HANDLE_INFO_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
unsafe impl ::windows::core::Abi for MARK_HANDLE_INFO32 {
    type Abi = Self;
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::PartialEq for MARK_HANDLE_INFO32 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MARK_HANDLE_INFO32>()) == 0 }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::Eq for MARK_HANDLE_INFO32 {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::default::Default for MARK_HANDLE_INFO32 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
unsafe impl ::windows::core::Abi for MARK_HANDLE_INFO32_0 {
    type Abi = Self;
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::PartialEq for MARK_HANDLE_INFO32_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MARK_HANDLE_INFO32_0>()) == 0 }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::Eq for MARK_HANDLE_INFO32_0 {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::default::Default for MARK_HANDLE_INFO32_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MEDIA_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const Unknown: MEDIA_TYPE = MEDIA_TYPE(0i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const F5_1Pt2_512: MEDIA_TYPE = MEDIA_TYPE(1i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const F3_1Pt44_512: MEDIA_TYPE = MEDIA_TYPE(2i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const F3_2Pt88_512: MEDIA_TYPE = MEDIA_TYPE(3i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const F3_20Pt8_512: MEDIA_TYPE = MEDIA_TYPE(4i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const F3_720_512: MEDIA_TYPE = MEDIA_TYPE(5i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const F5_360_512: MEDIA_TYPE = MEDIA_TYPE(6i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const F5_320_512: MEDIA_TYPE = MEDIA_TYPE(7i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const F5_320_1024: MEDIA_TYPE = MEDIA_TYPE(8i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const F5_180_512: MEDIA_TYPE = MEDIA_TYPE(9i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const F5_160_512: MEDIA_TYPE = MEDIA_TYPE(10i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const RemovableMedia: MEDIA_TYPE = MEDIA_TYPE(11i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FixedMedia: MEDIA_TYPE = MEDIA_TYPE(12i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const F3_120M_512: MEDIA_TYPE = MEDIA_TYPE(13i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const F3_640_512: MEDIA_TYPE = MEDIA_TYPE(14i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const F5_640_512: MEDIA_TYPE = MEDIA_TYPE(15i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const F5_720_512: MEDIA_TYPE = MEDIA_TYPE(16i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const F3_1Pt2_512: MEDIA_TYPE = MEDIA_TYPE(17i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const F3_1Pt23_1024: MEDIA_TYPE = MEDIA_TYPE(18i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const F5_1Pt23_1024: MEDIA_TYPE = MEDIA_TYPE(19i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const F3_128Mb_512: MEDIA_TYPE = MEDIA_TYPE(20i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const F3_230Mb_512: MEDIA_TYPE = MEDIA_TYPE(21i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const F8_256_128: MEDIA_TYPE = MEDIA_TYPE(22i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const F3_200Mb_512: MEDIA_TYPE = MEDIA_TYPE(23i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const F3_240M_512: MEDIA_TYPE = MEDIA_TYPE(24i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const F3_32M_512: MEDIA_TYPE = MEDIA_TYPE(25i32);
impl ::core::marker::Copy for MEDIA_TYPE {}
impl ::core::clone::Clone for MEDIA_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MEDIA_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for MEDIA_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for MEDIA_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MEDIA_TYPE").field(&self.0).finish()
    }
}
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
impl ::core::fmt::Debug for MFT_ENUM_DATA_V0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MFT_ENUM_DATA_V0").field("StartFileReferenceNumber", &self.StartFileReferenceNumber).field("LowUsn", &self.LowUsn).field("HighUsn", &self.HighUsn).finish()
    }
}
unsafe impl ::windows::core::Abi for MFT_ENUM_DATA_V0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MFT_ENUM_DATA_V0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MFT_ENUM_DATA_V0>()) == 0 }
    }
}
impl ::core::cmp::Eq for MFT_ENUM_DATA_V0 {}
impl ::core::default::Default for MFT_ENUM_DATA_V0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for MFT_ENUM_DATA_V1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MFT_ENUM_DATA_V1").field("StartFileReferenceNumber", &self.StartFileReferenceNumber).field("LowUsn", &self.LowUsn).field("HighUsn", &self.HighUsn).field("MinMajorVersion", &self.MinMajorVersion).field("MaxMajorVersion", &self.MaxMajorVersion).finish()
    }
}
unsafe impl ::windows::core::Abi for MFT_ENUM_DATA_V1 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MFT_ENUM_DATA_V1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MFT_ENUM_DATA_V1>()) == 0 }
    }
}
impl ::core::cmp::Eq for MFT_ENUM_DATA_V1 {}
impl ::core::default::Default for MFT_ENUM_DATA_V1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for MOVE_FILE_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MOVE_FILE_DATA").field("FileHandle", &self.FileHandle).field("StartingVcn", &self.StartingVcn).field("StartingLcn", &self.StartingLcn).field("ClusterCount", &self.ClusterCount).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for MOVE_FILE_DATA {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MOVE_FILE_DATA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MOVE_FILE_DATA>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MOVE_FILE_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MOVE_FILE_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::fmt::Debug for MOVE_FILE_DATA32 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MOVE_FILE_DATA32").field("FileHandle", &self.FileHandle).field("StartingVcn", &self.StartingVcn).field("StartingLcn", &self.StartingLcn).field("ClusterCount", &self.ClusterCount).finish()
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
unsafe impl ::windows::core::Abi for MOVE_FILE_DATA32 {
    type Abi = Self;
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::PartialEq for MOVE_FILE_DATA32 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MOVE_FILE_DATA32>()) == 0 }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::Eq for MOVE_FILE_DATA32 {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::default::Default for MOVE_FILE_DATA32 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for MOVE_FILE_RECORD_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MOVE_FILE_RECORD_DATA").field("FileHandle", &self.FileHandle).field("SourceFileRecord", &self.SourceFileRecord).field("TargetFileRecord", &self.TargetFileRecord).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for MOVE_FILE_RECORD_DATA {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MOVE_FILE_RECORD_DATA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MOVE_FILE_RECORD_DATA>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MOVE_FILE_RECORD_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MOVE_FILE_RECORD_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for NTFS_EXTENDED_VOLUME_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NTFS_EXTENDED_VOLUME_DATA")
            .field("ByteCount", &self.ByteCount)
            .field("MajorVersion", &self.MajorVersion)
            .field("MinorVersion", &self.MinorVersion)
            .field("BytesPerPhysicalSector", &self.BytesPerPhysicalSector)
            .field("LfsMajorVersion", &self.LfsMajorVersion)
            .field("LfsMinorVersion", &self.LfsMinorVersion)
            .field("MaxDeviceTrimExtentCount", &self.MaxDeviceTrimExtentCount)
            .field("MaxDeviceTrimByteCount", &self.MaxDeviceTrimByteCount)
            .field("MaxVolumeTrimExtentCount", &self.MaxVolumeTrimExtentCount)
            .field("MaxVolumeTrimByteCount", &self.MaxVolumeTrimByteCount)
            .finish()
    }
}
unsafe impl ::windows::core::Abi for NTFS_EXTENDED_VOLUME_DATA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for NTFS_EXTENDED_VOLUME_DATA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NTFS_EXTENDED_VOLUME_DATA>()) == 0 }
    }
}
impl ::core::cmp::Eq for NTFS_EXTENDED_VOLUME_DATA {}
impl ::core::default::Default for NTFS_EXTENDED_VOLUME_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for NTFS_FILE_RECORD_INPUT_BUFFER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NTFS_FILE_RECORD_INPUT_BUFFER").field("FileReferenceNumber", &self.FileReferenceNumber).finish()
    }
}
unsafe impl ::windows::core::Abi for NTFS_FILE_RECORD_INPUT_BUFFER {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for NTFS_FILE_RECORD_INPUT_BUFFER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NTFS_FILE_RECORD_INPUT_BUFFER>()) == 0 }
    }
}
impl ::core::cmp::Eq for NTFS_FILE_RECORD_INPUT_BUFFER {}
impl ::core::default::Default for NTFS_FILE_RECORD_INPUT_BUFFER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for NTFS_FILE_RECORD_OUTPUT_BUFFER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NTFS_FILE_RECORD_OUTPUT_BUFFER").field("FileReferenceNumber", &self.FileReferenceNumber).field("FileRecordLength", &self.FileRecordLength).field("FileRecordBuffer", &self.FileRecordBuffer).finish()
    }
}
unsafe impl ::windows::core::Abi for NTFS_FILE_RECORD_OUTPUT_BUFFER {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for NTFS_FILE_RECORD_OUTPUT_BUFFER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NTFS_FILE_RECORD_OUTPUT_BUFFER>()) == 0 }
    }
}
impl ::core::cmp::Eq for NTFS_FILE_RECORD_OUTPUT_BUFFER {}
impl ::core::default::Default for NTFS_FILE_RECORD_OUTPUT_BUFFER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for NTFS_STATISTICS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NTFS_STATISTICS")
            .field("LogFileFullExceptions", &self.LogFileFullExceptions)
            .field("OtherExceptions", &self.OtherExceptions)
            .field("MftReads", &self.MftReads)
            .field("MftReadBytes", &self.MftReadBytes)
            .field("MftWrites", &self.MftWrites)
            .field("MftWriteBytes", &self.MftWriteBytes)
            .field("MftWritesUserLevel", &self.MftWritesUserLevel)
            .field("MftWritesFlushForLogFileFull", &self.MftWritesFlushForLogFileFull)
            .field("MftWritesLazyWriter", &self.MftWritesLazyWriter)
            .field("MftWritesUserRequest", &self.MftWritesUserRequest)
            .field("Mft2Writes", &self.Mft2Writes)
            .field("Mft2WriteBytes", &self.Mft2WriteBytes)
            .field("Mft2WritesUserLevel", &self.Mft2WritesUserLevel)
            .field("Mft2WritesFlushForLogFileFull", &self.Mft2WritesFlushForLogFileFull)
            .field("Mft2WritesLazyWriter", &self.Mft2WritesLazyWriter)
            .field("Mft2WritesUserRequest", &self.Mft2WritesUserRequest)
            .field("RootIndexReads", &self.RootIndexReads)
            .field("RootIndexReadBytes", &self.RootIndexReadBytes)
            .field("RootIndexWrites", &self.RootIndexWrites)
            .field("RootIndexWriteBytes", &self.RootIndexWriteBytes)
            .field("BitmapReads", &self.BitmapReads)
            .field("BitmapReadBytes", &self.BitmapReadBytes)
            .field("BitmapWrites", &self.BitmapWrites)
            .field("BitmapWriteBytes", &self.BitmapWriteBytes)
            .field("BitmapWritesFlushForLogFileFull", &self.BitmapWritesFlushForLogFileFull)
            .field("BitmapWritesLazyWriter", &self.BitmapWritesLazyWriter)
            .field("BitmapWritesUserRequest", &self.BitmapWritesUserRequest)
            .field("BitmapWritesUserLevel", &self.BitmapWritesUserLevel)
            .field("MftBitmapReads", &self.MftBitmapReads)
            .field("MftBitmapReadBytes", &self.MftBitmapReadBytes)
            .field("MftBitmapWrites", &self.MftBitmapWrites)
            .field("MftBitmapWriteBytes", &self.MftBitmapWriteBytes)
            .field("MftBitmapWritesFlushForLogFileFull", &self.MftBitmapWritesFlushForLogFileFull)
            .field("MftBitmapWritesLazyWriter", &self.MftBitmapWritesLazyWriter)
            .field("MftBitmapWritesUserRequest", &self.MftBitmapWritesUserRequest)
            .field("MftBitmapWritesUserLevel", &self.MftBitmapWritesUserLevel)
            .field("UserIndexReads", &self.UserIndexReads)
            .field("UserIndexReadBytes", &self.UserIndexReadBytes)
            .field("UserIndexWrites", &self.UserIndexWrites)
            .field("UserIndexWriteBytes", &self.UserIndexWriteBytes)
            .field("LogFileReads", &self.LogFileReads)
            .field("LogFileReadBytes", &self.LogFileReadBytes)
            .field("LogFileWrites", &self.LogFileWrites)
            .field("LogFileWriteBytes", &self.LogFileWriteBytes)
            .field("Allocate", &self.Allocate)
            .field("DiskResourcesExhausted", &self.DiskResourcesExhausted)
            .finish()
    }
}
unsafe impl ::windows::core::Abi for NTFS_STATISTICS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for NTFS_STATISTICS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NTFS_STATISTICS>()) == 0 }
    }
}
impl ::core::cmp::Eq for NTFS_STATISTICS {}
impl ::core::default::Default for NTFS_STATISTICS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for NTFS_STATISTICS_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NTFS_STATISTICS_0").field("Calls", &self.Calls).field("Clusters", &self.Clusters).field("Hints", &self.Hints).field("RunsReturned", &self.RunsReturned).field("HintsHonored", &self.HintsHonored).field("HintsClusters", &self.HintsClusters).field("Cache", &self.Cache).field("CacheClusters", &self.CacheClusters).field("CacheMiss", &self.CacheMiss).field("CacheMissClusters", &self.CacheMissClusters).finish()
    }
}
unsafe impl ::windows::core::Abi for NTFS_STATISTICS_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for NTFS_STATISTICS_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NTFS_STATISTICS_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for NTFS_STATISTICS_0 {}
impl ::core::default::Default for NTFS_STATISTICS_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for NTFS_STATISTICS_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NTFS_STATISTICS_1").field("Write", &self.Write).field("Create", &self.Create).field("SetInfo", &self.SetInfo).finish()
    }
}
unsafe impl ::windows::core::Abi for NTFS_STATISTICS_1 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for NTFS_STATISTICS_1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NTFS_STATISTICS_1>()) == 0 }
    }
}
impl ::core::cmp::Eq for NTFS_STATISTICS_1 {}
impl ::core::default::Default for NTFS_STATISTICS_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for NTFS_STATISTICS_2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NTFS_STATISTICS_2").field("Write", &self.Write).field("Create", &self.Create).field("SetInfo", &self.SetInfo).field("Flush", &self.Flush).finish()
    }
}
unsafe impl ::windows::core::Abi for NTFS_STATISTICS_2 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for NTFS_STATISTICS_2 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NTFS_STATISTICS_2>()) == 0 }
    }
}
impl ::core::cmp::Eq for NTFS_STATISTICS_2 {}
impl ::core::default::Default for NTFS_STATISTICS_2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for NTFS_STATISTICS_3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NTFS_STATISTICS_3").field("Write", &self.Write).field("Create", &self.Create).field("SetInfo", &self.SetInfo).field("Flush", &self.Flush).finish()
    }
}
unsafe impl ::windows::core::Abi for NTFS_STATISTICS_3 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for NTFS_STATISTICS_3 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NTFS_STATISTICS_3>()) == 0 }
    }
}
impl ::core::cmp::Eq for NTFS_STATISTICS_3 {}
impl ::core::default::Default for NTFS_STATISTICS_3 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for NTFS_STATISTICS_4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NTFS_STATISTICS_4").field("Write", &self.Write).field("Create", &self.Create).field("SetInfo", &self.SetInfo).field("Flush", &self.Flush).finish()
    }
}
unsafe impl ::windows::core::Abi for NTFS_STATISTICS_4 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for NTFS_STATISTICS_4 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NTFS_STATISTICS_4>()) == 0 }
    }
}
impl ::core::cmp::Eq for NTFS_STATISTICS_4 {}
impl ::core::default::Default for NTFS_STATISTICS_4 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for NTFS_STATISTICS_EX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NTFS_STATISTICS_EX")
            .field("LogFileFullExceptions", &self.LogFileFullExceptions)
            .field("OtherExceptions", &self.OtherExceptions)
            .field("MftReads", &self.MftReads)
            .field("MftReadBytes", &self.MftReadBytes)
            .field("MftWrites", &self.MftWrites)
            .field("MftWriteBytes", &self.MftWriteBytes)
            .field("MftWritesUserLevel", &self.MftWritesUserLevel)
            .field("MftWritesFlushForLogFileFull", &self.MftWritesFlushForLogFileFull)
            .field("MftWritesLazyWriter", &self.MftWritesLazyWriter)
            .field("MftWritesUserRequest", &self.MftWritesUserRequest)
            .field("Mft2Writes", &self.Mft2Writes)
            .field("Mft2WriteBytes", &self.Mft2WriteBytes)
            .field("Mft2WritesUserLevel", &self.Mft2WritesUserLevel)
            .field("Mft2WritesFlushForLogFileFull", &self.Mft2WritesFlushForLogFileFull)
            .field("Mft2WritesLazyWriter", &self.Mft2WritesLazyWriter)
            .field("Mft2WritesUserRequest", &self.Mft2WritesUserRequest)
            .field("RootIndexReads", &self.RootIndexReads)
            .field("RootIndexReadBytes", &self.RootIndexReadBytes)
            .field("RootIndexWrites", &self.RootIndexWrites)
            .field("RootIndexWriteBytes", &self.RootIndexWriteBytes)
            .field("BitmapReads", &self.BitmapReads)
            .field("BitmapReadBytes", &self.BitmapReadBytes)
            .field("BitmapWrites", &self.BitmapWrites)
            .field("BitmapWriteBytes", &self.BitmapWriteBytes)
            .field("BitmapWritesFlushForLogFileFull", &self.BitmapWritesFlushForLogFileFull)
            .field("BitmapWritesLazyWriter", &self.BitmapWritesLazyWriter)
            .field("BitmapWritesUserRequest", &self.BitmapWritesUserRequest)
            .field("BitmapWritesUserLevel", &self.BitmapWritesUserLevel)
            .field("MftBitmapReads", &self.MftBitmapReads)
            .field("MftBitmapReadBytes", &self.MftBitmapReadBytes)
            .field("MftBitmapWrites", &self.MftBitmapWrites)
            .field("MftBitmapWriteBytes", &self.MftBitmapWriteBytes)
            .field("MftBitmapWritesFlushForLogFileFull", &self.MftBitmapWritesFlushForLogFileFull)
            .field("MftBitmapWritesLazyWriter", &self.MftBitmapWritesLazyWriter)
            .field("MftBitmapWritesUserRequest", &self.MftBitmapWritesUserRequest)
            .field("MftBitmapWritesUserLevel", &self.MftBitmapWritesUserLevel)
            .field("UserIndexReads", &self.UserIndexReads)
            .field("UserIndexReadBytes", &self.UserIndexReadBytes)
            .field("UserIndexWrites", &self.UserIndexWrites)
            .field("UserIndexWriteBytes", &self.UserIndexWriteBytes)
            .field("LogFileReads", &self.LogFileReads)
            .field("LogFileReadBytes", &self.LogFileReadBytes)
            .field("LogFileWrites", &self.LogFileWrites)
            .field("LogFileWriteBytes", &self.LogFileWriteBytes)
            .field("Allocate", &self.Allocate)
            .field("DiskResourcesExhausted", &self.DiskResourcesExhausted)
            .field("VolumeTrimCount", &self.VolumeTrimCount)
            .field("VolumeTrimTime", &self.VolumeTrimTime)
            .field("VolumeTrimByteCount", &self.VolumeTrimByteCount)
            .field("FileLevelTrimCount", &self.FileLevelTrimCount)
            .field("FileLevelTrimTime", &self.FileLevelTrimTime)
            .field("FileLevelTrimByteCount", &self.FileLevelTrimByteCount)
            .field("VolumeTrimSkippedCount", &self.VolumeTrimSkippedCount)
            .field("VolumeTrimSkippedByteCount", &self.VolumeTrimSkippedByteCount)
            .field("NtfsFillStatInfoFromMftRecordCalledCount", &self.NtfsFillStatInfoFromMftRecordCalledCount)
            .field("NtfsFillStatInfoFromMftRecordBailedBecauseOfAttributeListCount", &self.NtfsFillStatInfoFromMftRecordBailedBecauseOfAttributeListCount)
            .field("NtfsFillStatInfoFromMftRecordBailedBecauseOfNonResReparsePointCount", &self.NtfsFillStatInfoFromMftRecordBailedBecauseOfNonResReparsePointCount)
            .finish()
    }
}
unsafe impl ::windows::core::Abi for NTFS_STATISTICS_EX {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for NTFS_STATISTICS_EX {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NTFS_STATISTICS_EX>()) == 0 }
    }
}
impl ::core::cmp::Eq for NTFS_STATISTICS_EX {}
impl ::core::default::Default for NTFS_STATISTICS_EX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for NTFS_STATISTICS_EX_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NTFS_STATISTICS_EX_0").field("Calls", &self.Calls).field("RunsReturned", &self.RunsReturned).field("Hints", &self.Hints).field("HintsHonored", &self.HintsHonored).field("Cache", &self.Cache).field("CacheMiss", &self.CacheMiss).field("Clusters", &self.Clusters).field("HintsClusters", &self.HintsClusters).field("CacheClusters", &self.CacheClusters).field("CacheMissClusters", &self.CacheMissClusters).finish()
    }
}
unsafe impl ::windows::core::Abi for NTFS_STATISTICS_EX_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for NTFS_STATISTICS_EX_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NTFS_STATISTICS_EX_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for NTFS_STATISTICS_EX_0 {}
impl ::core::default::Default for NTFS_STATISTICS_EX_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for NTFS_STATISTICS_EX_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NTFS_STATISTICS_EX_1").field("Write", &self.Write).field("Create", &self.Create).field("SetInfo", &self.SetInfo).field("Flush", &self.Flush).finish()
    }
}
unsafe impl ::windows::core::Abi for NTFS_STATISTICS_EX_1 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for NTFS_STATISTICS_EX_1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NTFS_STATISTICS_EX_1>()) == 0 }
    }
}
impl ::core::cmp::Eq for NTFS_STATISTICS_EX_1 {}
impl ::core::default::Default for NTFS_STATISTICS_EX_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for NTFS_STATISTICS_EX_2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NTFS_STATISTICS_EX_2").field("Write", &self.Write).field("Create", &self.Create).field("SetInfo", &self.SetInfo).field("Flush", &self.Flush).finish()
    }
}
unsafe impl ::windows::core::Abi for NTFS_STATISTICS_EX_2 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for NTFS_STATISTICS_EX_2 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NTFS_STATISTICS_EX_2>()) == 0 }
    }
}
impl ::core::cmp::Eq for NTFS_STATISTICS_EX_2 {}
impl ::core::default::Default for NTFS_STATISTICS_EX_2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for NTFS_STATISTICS_EX_3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NTFS_STATISTICS_EX_3").field("Write", &self.Write).field("Create", &self.Create).field("SetInfo", &self.SetInfo).field("Flush", &self.Flush).finish()
    }
}
unsafe impl ::windows::core::Abi for NTFS_STATISTICS_EX_3 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for NTFS_STATISTICS_EX_3 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NTFS_STATISTICS_EX_3>()) == 0 }
    }
}
impl ::core::cmp::Eq for NTFS_STATISTICS_EX_3 {}
impl ::core::default::Default for NTFS_STATISTICS_EX_3 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for NTFS_STATISTICS_EX_4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NTFS_STATISTICS_EX_4").field("Write", &self.Write).field("Create", &self.Create).field("SetInfo", &self.SetInfo).field("Flush", &self.Flush).finish()
    }
}
unsafe impl ::windows::core::Abi for NTFS_STATISTICS_EX_4 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for NTFS_STATISTICS_EX_4 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NTFS_STATISTICS_EX_4>()) == 0 }
    }
}
impl ::core::cmp::Eq for NTFS_STATISTICS_EX_4 {}
impl ::core::default::Default for NTFS_STATISTICS_EX_4 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for NTFS_VOLUME_DATA_BUFFER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NTFS_VOLUME_DATA_BUFFER")
            .field("VolumeSerialNumber", &self.VolumeSerialNumber)
            .field("NumberSectors", &self.NumberSectors)
            .field("TotalClusters", &self.TotalClusters)
            .field("FreeClusters", &self.FreeClusters)
            .field("TotalReserved", &self.TotalReserved)
            .field("BytesPerSector", &self.BytesPerSector)
            .field("BytesPerCluster", &self.BytesPerCluster)
            .field("BytesPerFileRecordSegment", &self.BytesPerFileRecordSegment)
            .field("ClustersPerFileRecordSegment", &self.ClustersPerFileRecordSegment)
            .field("MftValidDataLength", &self.MftValidDataLength)
            .field("MftStartLcn", &self.MftStartLcn)
            .field("Mft2StartLcn", &self.Mft2StartLcn)
            .field("MftZoneStart", &self.MftZoneStart)
            .field("MftZoneEnd", &self.MftZoneEnd)
            .finish()
    }
}
unsafe impl ::windows::core::Abi for NTFS_VOLUME_DATA_BUFFER {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for NTFS_VOLUME_DATA_BUFFER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NTFS_VOLUME_DATA_BUFFER>()) == 0 }
    }
}
impl ::core::cmp::Eq for NTFS_VOLUME_DATA_BUFFER {}
impl ::core::default::Default for NTFS_VOLUME_DATA_BUFFER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for PARTITION_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PARTITION_INFORMATION").field("StartingOffset", &self.StartingOffset).field("PartitionLength", &self.PartitionLength).field("HiddenSectors", &self.HiddenSectors).field("PartitionNumber", &self.PartitionNumber).field("PartitionType", &self.PartitionType).field("BootIndicator", &self.BootIndicator).field("RecognizedPartition", &self.RecognizedPartition).field("RewritePartition", &self.RewritePartition).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for PARTITION_INFORMATION {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PARTITION_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PARTITION_INFORMATION>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PARTITION_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PARTITION_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for PARTITION_INFORMATION_EX {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PARTITION_INFORMATION_EX {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PARTITION_INFORMATION_EX>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PARTITION_INFORMATION_EX {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PARTITION_INFORMATION_EX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for PARTITION_INFORMATION_EX_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PARTITION_INFORMATION_EX_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PARTITION_INFORMATION_EX_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PARTITION_INFORMATION_EX_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PARTITION_INFORMATION_EX_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct PARTITION_INFORMATION_GPT {
    pub PartitionType: ::windows::core::GUID,
    pub PartitionId: ::windows::core::GUID,
    pub Attributes: GPT_ATTRIBUTES,
    pub Name: [u16; 36],
}
impl ::core::marker::Copy for PARTITION_INFORMATION_GPT {}
impl ::core::clone::Clone for PARTITION_INFORMATION_GPT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PARTITION_INFORMATION_GPT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PARTITION_INFORMATION_GPT").field("PartitionType", &self.PartitionType).field("PartitionId", &self.PartitionId).field("Attributes", &self.Attributes).field("Name", &self.Name).finish()
    }
}
unsafe impl ::windows::core::Abi for PARTITION_INFORMATION_GPT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for PARTITION_INFORMATION_GPT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PARTITION_INFORMATION_GPT>()) == 0 }
    }
}
impl ::core::cmp::Eq for PARTITION_INFORMATION_GPT {}
impl ::core::default::Default for PARTITION_INFORMATION_GPT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
    pub PartitionId: ::windows::core::GUID,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PARTITION_INFORMATION_MBR {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PARTITION_INFORMATION_MBR {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for PARTITION_INFORMATION_MBR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PARTITION_INFORMATION_MBR").field("PartitionType", &self.PartitionType).field("BootIndicator", &self.BootIndicator).field("RecognizedPartition", &self.RecognizedPartition).field("HiddenSectors", &self.HiddenSectors).field("PartitionId", &self.PartitionId).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for PARTITION_INFORMATION_MBR {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PARTITION_INFORMATION_MBR {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PARTITION_INFORMATION_MBR>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PARTITION_INFORMATION_MBR {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PARTITION_INFORMATION_MBR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PARTITION_STYLE(pub i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const PARTITION_STYLE_MBR: PARTITION_STYLE = PARTITION_STYLE(0i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const PARTITION_STYLE_GPT: PARTITION_STYLE = PARTITION_STYLE(1i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const PARTITION_STYLE_RAW: PARTITION_STYLE = PARTITION_STYLE(2i32);
impl ::core::marker::Copy for PARTITION_STYLE {}
impl ::core::clone::Clone for PARTITION_STYLE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PARTITION_STYLE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PARTITION_STYLE {
    type Abi = Self;
}
impl ::core::fmt::Debug for PARTITION_STYLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PARTITION_STYLE").field(&self.0).finish()
    }
}
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
impl ::core::fmt::Debug for PATHNAME_BUFFER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PATHNAME_BUFFER").field("PathNameLength", &self.PathNameLength).field("Name", &self.Name).finish()
    }
}
unsafe impl ::windows::core::Abi for PATHNAME_BUFFER {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for PATHNAME_BUFFER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PATHNAME_BUFFER>()) == 0 }
    }
}
impl ::core::cmp::Eq for PATHNAME_BUFFER {}
impl ::core::default::Default for PATHNAME_BUFFER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for PERF_BIN {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PERF_BIN").field("NumberOfBins", &self.NumberOfBins).field("TypeOfBin", &self.TypeOfBin).field("BinsRanges", &self.BinsRanges).finish()
    }
}
unsafe impl ::windows::core::Abi for PERF_BIN {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for PERF_BIN {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PERF_BIN>()) == 0 }
    }
}
impl ::core::cmp::Eq for PERF_BIN {}
impl ::core::default::Default for PERF_BIN {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
unsafe impl ::windows::core::Abi for PERSISTENT_RESERVE_COMMAND {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for PERSISTENT_RESERVE_COMMAND {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PERSISTENT_RESERVE_COMMAND>()) == 0 }
    }
}
impl ::core::cmp::Eq for PERSISTENT_RESERVE_COMMAND {}
impl ::core::default::Default for PERSISTENT_RESERVE_COMMAND {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
unsafe impl ::windows::core::Abi for PERSISTENT_RESERVE_COMMAND_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for PERSISTENT_RESERVE_COMMAND_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PERSISTENT_RESERVE_COMMAND_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for PERSISTENT_RESERVE_COMMAND_0 {}
impl ::core::default::Default for PERSISTENT_RESERVE_COMMAND_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for PERSISTENT_RESERVE_COMMAND_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PERSISTENT_RESERVE_COMMAND_0_0").field("_bitfield", &self._bitfield).field("AllocationLength", &self.AllocationLength).finish()
    }
}
unsafe impl ::windows::core::Abi for PERSISTENT_RESERVE_COMMAND_0_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for PERSISTENT_RESERVE_COMMAND_0_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PERSISTENT_RESERVE_COMMAND_0_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for PERSISTENT_RESERVE_COMMAND_0_0 {}
impl ::core::default::Default for PERSISTENT_RESERVE_COMMAND_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for PERSISTENT_RESERVE_COMMAND_0_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PERSISTENT_RESERVE_COMMAND_0_1").field("_bitfield1", &self._bitfield1).field("_bitfield2", &self._bitfield2).field("ParameterList", &self.ParameterList).finish()
    }
}
unsafe impl ::windows::core::Abi for PERSISTENT_RESERVE_COMMAND_0_1 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for PERSISTENT_RESERVE_COMMAND_0_1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PERSISTENT_RESERVE_COMMAND_0_1>()) == 0 }
    }
}
impl ::core::cmp::Eq for PERSISTENT_RESERVE_COMMAND_0_1 {}
impl ::core::default::Default for PERSISTENT_RESERVE_COMMAND_0_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for PHYSICAL_ELEMENT_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PHYSICAL_ELEMENT_STATUS").field("Version", &self.Version).field("Size", &self.Size).field("DescriptorCount", &self.DescriptorCount).field("ReturnedDescriptorCount", &self.ReturnedDescriptorCount).field("ElementIdentifierBeingDepoped", &self.ElementIdentifierBeingDepoped).field("Reserved", &self.Reserved).field("Descriptors", &self.Descriptors).finish()
    }
}
unsafe impl ::windows::core::Abi for PHYSICAL_ELEMENT_STATUS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for PHYSICAL_ELEMENT_STATUS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PHYSICAL_ELEMENT_STATUS>()) == 0 }
    }
}
impl ::core::cmp::Eq for PHYSICAL_ELEMENT_STATUS {}
impl ::core::default::Default for PHYSICAL_ELEMENT_STATUS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for PHYSICAL_ELEMENT_STATUS_DESCRIPTOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PHYSICAL_ELEMENT_STATUS_DESCRIPTOR").field("Version", &self.Version).field("Size", &self.Size).field("ElementIdentifier", &self.ElementIdentifier).field("PhysicalElementType", &self.PhysicalElementType).field("PhysicalElementHealth", &self.PhysicalElementHealth).field("Reserved1", &self.Reserved1).field("AssociatedCapacity", &self.AssociatedCapacity).field("Reserved2", &self.Reserved2).finish()
    }
}
unsafe impl ::windows::core::Abi for PHYSICAL_ELEMENT_STATUS_DESCRIPTOR {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for PHYSICAL_ELEMENT_STATUS_DESCRIPTOR {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PHYSICAL_ELEMENT_STATUS_DESCRIPTOR>()) == 0 }
    }
}
impl ::core::cmp::Eq for PHYSICAL_ELEMENT_STATUS_DESCRIPTOR {}
impl ::core::default::Default for PHYSICAL_ELEMENT_STATUS_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for PHYSICAL_ELEMENT_STATUS_REQUEST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PHYSICAL_ELEMENT_STATUS_REQUEST").field("Version", &self.Version).field("Size", &self.Size).field("StartingElement", &self.StartingElement).field("Filter", &self.Filter).field("ReportType", &self.ReportType).field("Reserved", &self.Reserved).finish()
    }
}
unsafe impl ::windows::core::Abi for PHYSICAL_ELEMENT_STATUS_REQUEST {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for PHYSICAL_ELEMENT_STATUS_REQUEST {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PHYSICAL_ELEMENT_STATUS_REQUEST>()) == 0 }
    }
}
impl ::core::cmp::Eq for PHYSICAL_ELEMENT_STATUS_REQUEST {}
impl ::core::default::Default for PHYSICAL_ELEMENT_STATUS_REQUEST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for PLEX_READ_DATA_REQUEST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PLEX_READ_DATA_REQUEST").field("ByteOffset", &self.ByteOffset).field("ByteLength", &self.ByteLength).field("PlexNumber", &self.PlexNumber).finish()
    }
}
unsafe impl ::windows::core::Abi for PLEX_READ_DATA_REQUEST {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for PLEX_READ_DATA_REQUEST {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PLEX_READ_DATA_REQUEST>()) == 0 }
    }
}
impl ::core::cmp::Eq for PLEX_READ_DATA_REQUEST {}
impl ::core::default::Default for PLEX_READ_DATA_REQUEST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for PREVENT_MEDIA_REMOVAL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PREVENT_MEDIA_REMOVAL").field("PreventMediaRemoval", &self.PreventMediaRemoval).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for PREVENT_MEDIA_REMOVAL {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PREVENT_MEDIA_REMOVAL {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PREVENT_MEDIA_REMOVAL>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PREVENT_MEDIA_REMOVAL {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PREVENT_MEDIA_REMOVAL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for QUERY_BAD_RANGES_INPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("QUERY_BAD_RANGES_INPUT").field("Flags", &self.Flags).field("NumRanges", &self.NumRanges).field("Ranges", &self.Ranges).finish()
    }
}
unsafe impl ::windows::core::Abi for QUERY_BAD_RANGES_INPUT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for QUERY_BAD_RANGES_INPUT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<QUERY_BAD_RANGES_INPUT>()) == 0 }
    }
}
impl ::core::cmp::Eq for QUERY_BAD_RANGES_INPUT {}
impl ::core::default::Default for QUERY_BAD_RANGES_INPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for QUERY_BAD_RANGES_INPUT_RANGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("QUERY_BAD_RANGES_INPUT_RANGE").field("StartOffset", &self.StartOffset).field("LengthInBytes", &self.LengthInBytes).finish()
    }
}
unsafe impl ::windows::core::Abi for QUERY_BAD_RANGES_INPUT_RANGE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for QUERY_BAD_RANGES_INPUT_RANGE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<QUERY_BAD_RANGES_INPUT_RANGE>()) == 0 }
    }
}
impl ::core::cmp::Eq for QUERY_BAD_RANGES_INPUT_RANGE {}
impl ::core::default::Default for QUERY_BAD_RANGES_INPUT_RANGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for QUERY_BAD_RANGES_OUTPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("QUERY_BAD_RANGES_OUTPUT").field("Flags", &self.Flags).field("NumBadRanges", &self.NumBadRanges).field("NextOffsetToLookUp", &self.NextOffsetToLookUp).field("BadRanges", &self.BadRanges).finish()
    }
}
unsafe impl ::windows::core::Abi for QUERY_BAD_RANGES_OUTPUT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for QUERY_BAD_RANGES_OUTPUT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<QUERY_BAD_RANGES_OUTPUT>()) == 0 }
    }
}
impl ::core::cmp::Eq for QUERY_BAD_RANGES_OUTPUT {}
impl ::core::default::Default for QUERY_BAD_RANGES_OUTPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for QUERY_BAD_RANGES_OUTPUT_RANGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("QUERY_BAD_RANGES_OUTPUT_RANGE").field("Flags", &self.Flags).field("Reserved", &self.Reserved).field("StartOffset", &self.StartOffset).field("LengthInBytes", &self.LengthInBytes).finish()
    }
}
unsafe impl ::windows::core::Abi for QUERY_BAD_RANGES_OUTPUT_RANGE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for QUERY_BAD_RANGES_OUTPUT_RANGE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<QUERY_BAD_RANGES_OUTPUT_RANGE>()) == 0 }
    }
}
impl ::core::cmp::Eq for QUERY_BAD_RANGES_OUTPUT_RANGE {}
impl ::core::default::Default for QUERY_BAD_RANGES_OUTPUT_RANGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const QUERY_DEPENDENT_VOLUME_REQUEST_FLAG_GUEST_VOLUMES: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const QUERY_DEPENDENT_VOLUME_REQUEST_FLAG_HOST_VOLUMES: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct QUERY_FILE_LAYOUT_FILTER_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const QUERY_FILE_LAYOUT_FILTER_TYPE_NONE: QUERY_FILE_LAYOUT_FILTER_TYPE = QUERY_FILE_LAYOUT_FILTER_TYPE(0i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const QUERY_FILE_LAYOUT_FILTER_TYPE_CLUSTERS: QUERY_FILE_LAYOUT_FILTER_TYPE = QUERY_FILE_LAYOUT_FILTER_TYPE(1i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const QUERY_FILE_LAYOUT_FILTER_TYPE_FILEID: QUERY_FILE_LAYOUT_FILTER_TYPE = QUERY_FILE_LAYOUT_FILTER_TYPE(2i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const QUERY_FILE_LAYOUT_FILTER_TYPE_STORAGE_RESERVE_ID: QUERY_FILE_LAYOUT_FILTER_TYPE = QUERY_FILE_LAYOUT_FILTER_TYPE(3i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const QUERY_FILE_LAYOUT_NUM_FILTER_TYPES: QUERY_FILE_LAYOUT_FILTER_TYPE = QUERY_FILE_LAYOUT_FILTER_TYPE(4i32);
impl ::core::marker::Copy for QUERY_FILE_LAYOUT_FILTER_TYPE {}
impl ::core::clone::Clone for QUERY_FILE_LAYOUT_FILTER_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for QUERY_FILE_LAYOUT_FILTER_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for QUERY_FILE_LAYOUT_FILTER_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for QUERY_FILE_LAYOUT_FILTER_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("QUERY_FILE_LAYOUT_FILTER_TYPE").field(&self.0).finish()
    }
}
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
unsafe impl ::windows::core::Abi for QUERY_FILE_LAYOUT_INPUT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for QUERY_FILE_LAYOUT_INPUT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<QUERY_FILE_LAYOUT_INPUT>()) == 0 }
    }
}
impl ::core::cmp::Eq for QUERY_FILE_LAYOUT_INPUT {}
impl ::core::default::Default for QUERY_FILE_LAYOUT_INPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
unsafe impl ::windows::core::Abi for QUERY_FILE_LAYOUT_INPUT_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for QUERY_FILE_LAYOUT_INPUT_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<QUERY_FILE_LAYOUT_INPUT_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for QUERY_FILE_LAYOUT_INPUT_0 {}
impl ::core::default::Default for QUERY_FILE_LAYOUT_INPUT_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
unsafe impl ::windows::core::Abi for QUERY_FILE_LAYOUT_INPUT_1 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for QUERY_FILE_LAYOUT_INPUT_1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<QUERY_FILE_LAYOUT_INPUT_1>()) == 0 }
    }
}
impl ::core::cmp::Eq for QUERY_FILE_LAYOUT_INPUT_1 {}
impl ::core::default::Default for QUERY_FILE_LAYOUT_INPUT_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for QUERY_FILE_LAYOUT_OUTPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("QUERY_FILE_LAYOUT_OUTPUT").field("FileEntryCount", &self.FileEntryCount).field("FirstFileOffset", &self.FirstFileOffset).field("Flags", &self.Flags).field("Reserved", &self.Reserved).finish()
    }
}
unsafe impl ::windows::core::Abi for QUERY_FILE_LAYOUT_OUTPUT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for QUERY_FILE_LAYOUT_OUTPUT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<QUERY_FILE_LAYOUT_OUTPUT>()) == 0 }
    }
}
impl ::core::cmp::Eq for QUERY_FILE_LAYOUT_OUTPUT {}
impl ::core::default::Default for QUERY_FILE_LAYOUT_OUTPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for READ_ELEMENT_ADDRESS_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("READ_ELEMENT_ADDRESS_INFO").field("NumberOfElements", &self.NumberOfElements).field("ElementStatus", &self.ElementStatus).finish()
    }
}
unsafe impl ::windows::core::Abi for READ_ELEMENT_ADDRESS_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for READ_ELEMENT_ADDRESS_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<READ_ELEMENT_ADDRESS_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for READ_ELEMENT_ADDRESS_INFO {}
impl ::core::default::Default for READ_ELEMENT_ADDRESS_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for READ_FILE_USN_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("READ_FILE_USN_DATA").field("MinMajorVersion", &self.MinMajorVersion).field("MaxMajorVersion", &self.MaxMajorVersion).finish()
    }
}
unsafe impl ::windows::core::Abi for READ_FILE_USN_DATA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for READ_FILE_USN_DATA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<READ_FILE_USN_DATA>()) == 0 }
    }
}
impl ::core::cmp::Eq for READ_FILE_USN_DATA {}
impl ::core::default::Default for READ_FILE_USN_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for READ_USN_JOURNAL_DATA_V0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("READ_USN_JOURNAL_DATA_V0").field("StartUsn", &self.StartUsn).field("ReasonMask", &self.ReasonMask).field("ReturnOnlyOnClose", &self.ReturnOnlyOnClose).field("Timeout", &self.Timeout).field("BytesToWaitFor", &self.BytesToWaitFor).field("UsnJournalID", &self.UsnJournalID).finish()
    }
}
unsafe impl ::windows::core::Abi for READ_USN_JOURNAL_DATA_V0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for READ_USN_JOURNAL_DATA_V0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<READ_USN_JOURNAL_DATA_V0>()) == 0 }
    }
}
impl ::core::cmp::Eq for READ_USN_JOURNAL_DATA_V0 {}
impl ::core::default::Default for READ_USN_JOURNAL_DATA_V0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for READ_USN_JOURNAL_DATA_V1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("READ_USN_JOURNAL_DATA_V1").field("StartUsn", &self.StartUsn).field("ReasonMask", &self.ReasonMask).field("ReturnOnlyOnClose", &self.ReturnOnlyOnClose).field("Timeout", &self.Timeout).field("BytesToWaitFor", &self.BytesToWaitFor).field("UsnJournalID", &self.UsnJournalID).field("MinMajorVersion", &self.MinMajorVersion).field("MaxMajorVersion", &self.MaxMajorVersion).finish()
    }
}
unsafe impl ::windows::core::Abi for READ_USN_JOURNAL_DATA_V1 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for READ_USN_JOURNAL_DATA_V1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<READ_USN_JOURNAL_DATA_V1>()) == 0 }
    }
}
impl ::core::cmp::Eq for READ_USN_JOURNAL_DATA_V1 {}
impl ::core::default::Default for READ_USN_JOURNAL_DATA_V1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for REASSIGN_BLOCKS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("REASSIGN_BLOCKS").field("Reserved", &self.Reserved).field("Count", &self.Count).field("BlockNumber", &self.BlockNumber).finish()
    }
}
unsafe impl ::windows::core::Abi for REASSIGN_BLOCKS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for REASSIGN_BLOCKS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<REASSIGN_BLOCKS>()) == 0 }
    }
}
impl ::core::cmp::Eq for REASSIGN_BLOCKS {}
impl ::core::default::Default for REASSIGN_BLOCKS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
unsafe impl ::windows::core::Abi for REASSIGN_BLOCKS_EX {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for REASSIGN_BLOCKS_EX {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<REASSIGN_BLOCKS_EX>()) == 0 }
    }
}
impl ::core::cmp::Eq for REASSIGN_BLOCKS_EX {}
impl ::core::default::Default for REASSIGN_BLOCKS_EX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const RECOVERED_READS_VALID: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const RECOVERED_WRITES_VALID: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct REFS_SMR_VOLUME_GC_ACTION(pub i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const SmrGcActionStart: REFS_SMR_VOLUME_GC_ACTION = REFS_SMR_VOLUME_GC_ACTION(1i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const SmrGcActionStartFullSpeed: REFS_SMR_VOLUME_GC_ACTION = REFS_SMR_VOLUME_GC_ACTION(2i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const SmrGcActionPause: REFS_SMR_VOLUME_GC_ACTION = REFS_SMR_VOLUME_GC_ACTION(3i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const SmrGcActionStop: REFS_SMR_VOLUME_GC_ACTION = REFS_SMR_VOLUME_GC_ACTION(4i32);
impl ::core::marker::Copy for REFS_SMR_VOLUME_GC_ACTION {}
impl ::core::clone::Clone for REFS_SMR_VOLUME_GC_ACTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for REFS_SMR_VOLUME_GC_ACTION {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for REFS_SMR_VOLUME_GC_ACTION {
    type Abi = Self;
}
impl ::core::fmt::Debug for REFS_SMR_VOLUME_GC_ACTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("REFS_SMR_VOLUME_GC_ACTION").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct REFS_SMR_VOLUME_GC_METHOD(pub i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const SmrGcMethodCompaction: REFS_SMR_VOLUME_GC_METHOD = REFS_SMR_VOLUME_GC_METHOD(1i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const SmrGcMethodCompression: REFS_SMR_VOLUME_GC_METHOD = REFS_SMR_VOLUME_GC_METHOD(2i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const SmrGcMethodRotation: REFS_SMR_VOLUME_GC_METHOD = REFS_SMR_VOLUME_GC_METHOD(3i32);
impl ::core::marker::Copy for REFS_SMR_VOLUME_GC_METHOD {}
impl ::core::clone::Clone for REFS_SMR_VOLUME_GC_METHOD {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for REFS_SMR_VOLUME_GC_METHOD {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for REFS_SMR_VOLUME_GC_METHOD {
    type Abi = Self;
}
impl ::core::fmt::Debug for REFS_SMR_VOLUME_GC_METHOD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("REFS_SMR_VOLUME_GC_METHOD").field(&self.0).finish()
    }
}
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
impl ::core::fmt::Debug for REFS_SMR_VOLUME_GC_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("REFS_SMR_VOLUME_GC_PARAMETERS").field("Version", &self.Version).field("Flags", &self.Flags).field("Action", &self.Action).field("Method", &self.Method).field("IoGranularity", &self.IoGranularity).field("CompressionFormat", &self.CompressionFormat).field("Unused", &self.Unused).finish()
    }
}
unsafe impl ::windows::core::Abi for REFS_SMR_VOLUME_GC_PARAMETERS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for REFS_SMR_VOLUME_GC_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<REFS_SMR_VOLUME_GC_PARAMETERS>()) == 0 }
    }
}
impl ::core::cmp::Eq for REFS_SMR_VOLUME_GC_PARAMETERS {}
impl ::core::default::Default for REFS_SMR_VOLUME_GC_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const REFS_SMR_VOLUME_GC_PARAMETERS_VERSION_V1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct REFS_SMR_VOLUME_GC_STATE(pub i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const SmrGcStateInactive: REFS_SMR_VOLUME_GC_STATE = REFS_SMR_VOLUME_GC_STATE(0i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const SmrGcStatePaused: REFS_SMR_VOLUME_GC_STATE = REFS_SMR_VOLUME_GC_STATE(1i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const SmrGcStateActive: REFS_SMR_VOLUME_GC_STATE = REFS_SMR_VOLUME_GC_STATE(2i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const SmrGcStateActiveFullSpeed: REFS_SMR_VOLUME_GC_STATE = REFS_SMR_VOLUME_GC_STATE(3i32);
impl ::core::marker::Copy for REFS_SMR_VOLUME_GC_STATE {}
impl ::core::clone::Clone for REFS_SMR_VOLUME_GC_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for REFS_SMR_VOLUME_GC_STATE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for REFS_SMR_VOLUME_GC_STATE {
    type Abi = Self;
}
impl ::core::fmt::Debug for REFS_SMR_VOLUME_GC_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("REFS_SMR_VOLUME_GC_STATE").field(&self.0).finish()
    }
}
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
impl ::core::fmt::Debug for REFS_SMR_VOLUME_INFO_OUTPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("REFS_SMR_VOLUME_INFO_OUTPUT")
            .field("Version", &self.Version)
            .field("Flags", &self.Flags)
            .field("SizeOfRandomlyWritableTier", &self.SizeOfRandomlyWritableTier)
            .field("FreeSpaceInRandomlyWritableTier", &self.FreeSpaceInRandomlyWritableTier)
            .field("SizeofSMRTier", &self.SizeofSMRTier)
            .field("FreeSpaceInSMRTier", &self.FreeSpaceInSMRTier)
            .field("UsableFreeSpaceInSMRTier", &self.UsableFreeSpaceInSMRTier)
            .field("VolumeGcState", &self.VolumeGcState)
            .field("VolumeGcLastStatus", &self.VolumeGcLastStatus)
            .field("CurrentGcBandFillPercentage", &self.CurrentGcBandFillPercentage)
            .field("Unused", &self.Unused)
            .finish()
    }
}
unsafe impl ::windows::core::Abi for REFS_SMR_VOLUME_INFO_OUTPUT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for REFS_SMR_VOLUME_INFO_OUTPUT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<REFS_SMR_VOLUME_INFO_OUTPUT>()) == 0 }
    }
}
impl ::core::cmp::Eq for REFS_SMR_VOLUME_INFO_OUTPUT {}
impl ::core::default::Default for REFS_SMR_VOLUME_INFO_OUTPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for REFS_VOLUME_DATA_BUFFER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("REFS_VOLUME_DATA_BUFFER")
            .field("ByteCount", &self.ByteCount)
            .field("MajorVersion", &self.MajorVersion)
            .field("MinorVersion", &self.MinorVersion)
            .field("BytesPerPhysicalSector", &self.BytesPerPhysicalSector)
            .field("VolumeSerialNumber", &self.VolumeSerialNumber)
            .field("NumberSectors", &self.NumberSectors)
            .field("TotalClusters", &self.TotalClusters)
            .field("FreeClusters", &self.FreeClusters)
            .field("TotalReserved", &self.TotalReserved)
            .field("BytesPerSector", &self.BytesPerSector)
            .field("BytesPerCluster", &self.BytesPerCluster)
            .field("MaximumSizeOfResidentFile", &self.MaximumSizeOfResidentFile)
            .field("FastTierDataFillRatio", &self.FastTierDataFillRatio)
            .field("SlowTierDataFillRatio", &self.SlowTierDataFillRatio)
            .field("DestagesFastTierToSlowTierRate", &self.DestagesFastTierToSlowTierRate)
            .field("Reserved", &self.Reserved)
            .finish()
    }
}
unsafe impl ::windows::core::Abi for REFS_VOLUME_DATA_BUFFER {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for REFS_VOLUME_DATA_BUFFER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<REFS_VOLUME_DATA_BUFFER>()) == 0 }
    }
}
impl ::core::cmp::Eq for REFS_VOLUME_DATA_BUFFER {}
impl ::core::default::Default for REFS_VOLUME_DATA_BUFFER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for REMOVE_ELEMENT_AND_TRUNCATE_REQUEST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("REMOVE_ELEMENT_AND_TRUNCATE_REQUEST").field("Version", &self.Version).field("Size", &self.Size).field("RequestCapacity", &self.RequestCapacity).field("ElementIdentifier", &self.ElementIdentifier).field("Reserved", &self.Reserved).finish()
    }
}
unsafe impl ::windows::core::Abi for REMOVE_ELEMENT_AND_TRUNCATE_REQUEST {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for REMOVE_ELEMENT_AND_TRUNCATE_REQUEST {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<REMOVE_ELEMENT_AND_TRUNCATE_REQUEST>()) == 0 }
    }
}
impl ::core::cmp::Eq for REMOVE_ELEMENT_AND_TRUNCATE_REQUEST {}
impl ::core::default::Default for REMOVE_ELEMENT_AND_TRUNCATE_REQUEST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for REPAIR_COPIES_INPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("REPAIR_COPIES_INPUT").field("Size", &self.Size).field("Flags", &self.Flags).field("FileOffset", &self.FileOffset).field("Length", &self.Length).field("SourceCopy", &self.SourceCopy).field("NumberOfRepairCopies", &self.NumberOfRepairCopies).field("RepairCopies", &self.RepairCopies).finish()
    }
}
unsafe impl ::windows::core::Abi for REPAIR_COPIES_INPUT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for REPAIR_COPIES_INPUT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<REPAIR_COPIES_INPUT>()) == 0 }
    }
}
impl ::core::cmp::Eq for REPAIR_COPIES_INPUT {}
impl ::core::default::Default for REPAIR_COPIES_INPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for REPAIR_COPIES_OUTPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("REPAIR_COPIES_OUTPUT").field("Size", &self.Size).field("Status", &self.Status).field("ResumeFileOffset", &self.ResumeFileOffset).finish()
    }
}
unsafe impl ::windows::core::Abi for REPAIR_COPIES_OUTPUT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for REPAIR_COPIES_OUTPUT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<REPAIR_COPIES_OUTPUT>()) == 0 }
    }
}
impl ::core::cmp::Eq for REPAIR_COPIES_OUTPUT {}
impl ::core::default::Default for REPAIR_COPIES_OUTPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for REQUEST_OPLOCK_INPUT_BUFFER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("REQUEST_OPLOCK_INPUT_BUFFER").field("StructureVersion", &self.StructureVersion).field("StructureLength", &self.StructureLength).field("RequestedOplockLevel", &self.RequestedOplockLevel).field("Flags", &self.Flags).finish()
    }
}
unsafe impl ::windows::core::Abi for REQUEST_OPLOCK_INPUT_BUFFER {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for REQUEST_OPLOCK_INPUT_BUFFER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<REQUEST_OPLOCK_INPUT_BUFFER>()) == 0 }
    }
}
impl ::core::cmp::Eq for REQUEST_OPLOCK_INPUT_BUFFER {}
impl ::core::default::Default for REQUEST_OPLOCK_INPUT_BUFFER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for REQUEST_OPLOCK_OUTPUT_BUFFER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("REQUEST_OPLOCK_OUTPUT_BUFFER").field("StructureVersion", &self.StructureVersion).field("StructureLength", &self.StructureLength).field("OriginalOplockLevel", &self.OriginalOplockLevel).field("NewOplockLevel", &self.NewOplockLevel).field("Flags", &self.Flags).field("AccessMode", &self.AccessMode).field("ShareMode", &self.ShareMode).finish()
    }
}
unsafe impl ::windows::core::Abi for REQUEST_OPLOCK_OUTPUT_BUFFER {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for REQUEST_OPLOCK_OUTPUT_BUFFER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<REQUEST_OPLOCK_OUTPUT_BUFFER>()) == 0 }
    }
}
impl ::core::cmp::Eq for REQUEST_OPLOCK_OUTPUT_BUFFER {}
impl ::core::default::Default for REQUEST_OPLOCK_OUTPUT_BUFFER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for REQUEST_RAW_ENCRYPTED_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("REQUEST_RAW_ENCRYPTED_DATA").field("FileOffset", &self.FileOffset).field("Length", &self.Length).finish()
    }
}
unsafe impl ::windows::core::Abi for REQUEST_RAW_ENCRYPTED_DATA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for REQUEST_RAW_ENCRYPTED_DATA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<REQUEST_RAW_ENCRYPTED_DATA>()) == 0 }
    }
}
impl ::core::cmp::Eq for REQUEST_RAW_ENCRYPTED_DATA {}
impl ::core::default::Default for REQUEST_RAW_ENCRYPTED_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for RETRIEVAL_POINTERS_AND_REFCOUNT_BUFFER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RETRIEVAL_POINTERS_AND_REFCOUNT_BUFFER").field("ExtentCount", &self.ExtentCount).field("StartingVcn", &self.StartingVcn).field("Extents", &self.Extents).finish()
    }
}
unsafe impl ::windows::core::Abi for RETRIEVAL_POINTERS_AND_REFCOUNT_BUFFER {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for RETRIEVAL_POINTERS_AND_REFCOUNT_BUFFER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RETRIEVAL_POINTERS_AND_REFCOUNT_BUFFER>()) == 0 }
    }
}
impl ::core::cmp::Eq for RETRIEVAL_POINTERS_AND_REFCOUNT_BUFFER {}
impl ::core::default::Default for RETRIEVAL_POINTERS_AND_REFCOUNT_BUFFER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for RETRIEVAL_POINTERS_AND_REFCOUNT_BUFFER_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RETRIEVAL_POINTERS_AND_REFCOUNT_BUFFER_0").field("NextVcn", &self.NextVcn).field("Lcn", &self.Lcn).field("ReferenceCount", &self.ReferenceCount).finish()
    }
}
unsafe impl ::windows::core::Abi for RETRIEVAL_POINTERS_AND_REFCOUNT_BUFFER_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for RETRIEVAL_POINTERS_AND_REFCOUNT_BUFFER_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RETRIEVAL_POINTERS_AND_REFCOUNT_BUFFER_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for RETRIEVAL_POINTERS_AND_REFCOUNT_BUFFER_0 {}
impl ::core::default::Default for RETRIEVAL_POINTERS_AND_REFCOUNT_BUFFER_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for RETRIEVAL_POINTERS_BUFFER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RETRIEVAL_POINTERS_BUFFER").field("ExtentCount", &self.ExtentCount).field("StartingVcn", &self.StartingVcn).field("Extents", &self.Extents).finish()
    }
}
unsafe impl ::windows::core::Abi for RETRIEVAL_POINTERS_BUFFER {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for RETRIEVAL_POINTERS_BUFFER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RETRIEVAL_POINTERS_BUFFER>()) == 0 }
    }
}
impl ::core::cmp::Eq for RETRIEVAL_POINTERS_BUFFER {}
impl ::core::default::Default for RETRIEVAL_POINTERS_BUFFER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for RETRIEVAL_POINTERS_BUFFER_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RETRIEVAL_POINTERS_BUFFER_0").field("NextVcn", &self.NextVcn).field("Lcn", &self.Lcn).finish()
    }
}
unsafe impl ::windows::core::Abi for RETRIEVAL_POINTERS_BUFFER_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for RETRIEVAL_POINTERS_BUFFER_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RETRIEVAL_POINTERS_BUFFER_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for RETRIEVAL_POINTERS_BUFFER_0 {}
impl ::core::default::Default for RETRIEVAL_POINTERS_BUFFER_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for RETRIEVAL_POINTER_BASE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RETRIEVAL_POINTER_BASE").field("FileAreaOffset", &self.FileAreaOffset).finish()
    }
}
unsafe impl ::windows::core::Abi for RETRIEVAL_POINTER_BASE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for RETRIEVAL_POINTER_BASE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RETRIEVAL_POINTER_BASE>()) == 0 }
    }
}
impl ::core::cmp::Eq for RETRIEVAL_POINTER_BASE {}
impl ::core::default::Default for RETRIEVAL_POINTER_BASE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for RETRIEVAL_POINTER_COUNT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RETRIEVAL_POINTER_COUNT").field("ExtentCount", &self.ExtentCount).finish()
    }
}
unsafe impl ::windows::core::Abi for RETRIEVAL_POINTER_COUNT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for RETRIEVAL_POINTER_COUNT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RETRIEVAL_POINTER_COUNT>()) == 0 }
    }
}
impl ::core::cmp::Eq for RETRIEVAL_POINTER_COUNT {}
impl ::core::default::Default for RETRIEVAL_POINTER_COUNT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for SCM_BUS_DEDICATED_MEMORY_DEVICES_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCM_BUS_DEDICATED_MEMORY_DEVICES_INFO").field("Version", &self.Version).field("Size", &self.Size).field("DeviceCount", &self.DeviceCount).field("Devices", &self.Devices).finish()
    }
}
unsafe impl ::windows::core::Abi for SCM_BUS_DEDICATED_MEMORY_DEVICES_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SCM_BUS_DEDICATED_MEMORY_DEVICES_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SCM_BUS_DEDICATED_MEMORY_DEVICES_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for SCM_BUS_DEDICATED_MEMORY_DEVICES_INFO {}
impl ::core::default::Default for SCM_BUS_DEDICATED_MEMORY_DEVICES_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct SCM_BUS_DEDICATED_MEMORY_DEVICE_INFO {
    pub DeviceGuid: ::windows::core::GUID,
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
impl ::core::fmt::Debug for SCM_BUS_DEDICATED_MEMORY_DEVICE_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCM_BUS_DEDICATED_MEMORY_DEVICE_INFO").field("DeviceGuid", &self.DeviceGuid).field("DeviceNumber", &self.DeviceNumber).field("Flags", &self.Flags).field("DeviceSize", &self.DeviceSize).finish()
    }
}
unsafe impl ::windows::core::Abi for SCM_BUS_DEDICATED_MEMORY_DEVICE_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SCM_BUS_DEDICATED_MEMORY_DEVICE_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SCM_BUS_DEDICATED_MEMORY_DEVICE_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for SCM_BUS_DEDICATED_MEMORY_DEVICE_INFO {}
impl ::core::default::Default for SCM_BUS_DEDICATED_MEMORY_DEVICE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for SCM_BUS_DEDICATED_MEMORY_DEVICE_INFO_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCM_BUS_DEDICATED_MEMORY_DEVICE_INFO_0").field("_bitfield", &self._bitfield).finish()
    }
}
unsafe impl ::windows::core::Abi for SCM_BUS_DEDICATED_MEMORY_DEVICE_INFO_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SCM_BUS_DEDICATED_MEMORY_DEVICE_INFO_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SCM_BUS_DEDICATED_MEMORY_DEVICE_INFO_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for SCM_BUS_DEDICATED_MEMORY_DEVICE_INFO_0 {}
impl ::core::default::Default for SCM_BUS_DEDICATED_MEMORY_DEVICE_INFO_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for SCM_BUS_DEDICATED_MEMORY_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCM_BUS_DEDICATED_MEMORY_STATE").field("ActivateState", &self.ActivateState).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for SCM_BUS_DEDICATED_MEMORY_STATE {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SCM_BUS_DEDICATED_MEMORY_STATE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SCM_BUS_DEDICATED_MEMORY_STATE>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SCM_BUS_DEDICATED_MEMORY_STATE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SCM_BUS_DEDICATED_MEMORY_STATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SCM_BUS_FIRMWARE_ACTIVATION_STATE(pub i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ScmBusFirmwareActivationState_Idle: SCM_BUS_FIRMWARE_ACTIVATION_STATE = SCM_BUS_FIRMWARE_ACTIVATION_STATE(0i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ScmBusFirmwareActivationState_Armed: SCM_BUS_FIRMWARE_ACTIVATION_STATE = SCM_BUS_FIRMWARE_ACTIVATION_STATE(1i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ScmBusFirmwareActivationState_Busy: SCM_BUS_FIRMWARE_ACTIVATION_STATE = SCM_BUS_FIRMWARE_ACTIVATION_STATE(2i32);
impl ::core::marker::Copy for SCM_BUS_FIRMWARE_ACTIVATION_STATE {}
impl ::core::clone::Clone for SCM_BUS_FIRMWARE_ACTIVATION_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SCM_BUS_FIRMWARE_ACTIVATION_STATE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for SCM_BUS_FIRMWARE_ACTIVATION_STATE {
    type Abi = Self;
}
impl ::core::fmt::Debug for SCM_BUS_FIRMWARE_ACTIVATION_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SCM_BUS_FIRMWARE_ACTIVATION_STATE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SCM_BUS_PROPERTY_ID(pub i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ScmBusProperty_RuntimeFwActivationInfo: SCM_BUS_PROPERTY_ID = SCM_BUS_PROPERTY_ID(0i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ScmBusProperty_DedicatedMemoryInfo: SCM_BUS_PROPERTY_ID = SCM_BUS_PROPERTY_ID(1i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ScmBusProperty_DedicatedMemoryState: SCM_BUS_PROPERTY_ID = SCM_BUS_PROPERTY_ID(2i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ScmBusProperty_Max: SCM_BUS_PROPERTY_ID = SCM_BUS_PROPERTY_ID(3i32);
impl ::core::marker::Copy for SCM_BUS_PROPERTY_ID {}
impl ::core::clone::Clone for SCM_BUS_PROPERTY_ID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SCM_BUS_PROPERTY_ID {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for SCM_BUS_PROPERTY_ID {
    type Abi = Self;
}
impl ::core::fmt::Debug for SCM_BUS_PROPERTY_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SCM_BUS_PROPERTY_ID").field(&self.0).finish()
    }
}
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
impl ::core::fmt::Debug for SCM_BUS_PROPERTY_QUERY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCM_BUS_PROPERTY_QUERY").field("Version", &self.Version).field("Size", &self.Size).field("PropertyId", &self.PropertyId).field("QueryType", &self.QueryType).field("AdditionalParameters", &self.AdditionalParameters).finish()
    }
}
unsafe impl ::windows::core::Abi for SCM_BUS_PROPERTY_QUERY {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SCM_BUS_PROPERTY_QUERY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SCM_BUS_PROPERTY_QUERY>()) == 0 }
    }
}
impl ::core::cmp::Eq for SCM_BUS_PROPERTY_QUERY {}
impl ::core::default::Default for SCM_BUS_PROPERTY_QUERY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for SCM_BUS_PROPERTY_SET {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCM_BUS_PROPERTY_SET").field("Version", &self.Version).field("Size", &self.Size).field("PropertyId", &self.PropertyId).field("SetType", &self.SetType).field("AdditionalParameters", &self.AdditionalParameters).finish()
    }
}
unsafe impl ::windows::core::Abi for SCM_BUS_PROPERTY_SET {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SCM_BUS_PROPERTY_SET {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SCM_BUS_PROPERTY_SET>()) == 0 }
    }
}
impl ::core::cmp::Eq for SCM_BUS_PROPERTY_SET {}
impl ::core::default::Default for SCM_BUS_PROPERTY_SET {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SCM_BUS_QUERY_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ScmBusQuery_Descriptor: SCM_BUS_QUERY_TYPE = SCM_BUS_QUERY_TYPE(0i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ScmBusQuery_IsSupported: SCM_BUS_QUERY_TYPE = SCM_BUS_QUERY_TYPE(1i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ScmBusQuery_Max: SCM_BUS_QUERY_TYPE = SCM_BUS_QUERY_TYPE(2i32);
impl ::core::marker::Copy for SCM_BUS_QUERY_TYPE {}
impl ::core::clone::Clone for SCM_BUS_QUERY_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SCM_BUS_QUERY_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for SCM_BUS_QUERY_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for SCM_BUS_QUERY_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SCM_BUS_QUERY_TYPE").field(&self.0).finish()
    }
}
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
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for SCM_BUS_RUNTIME_FW_ACTIVATION_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCM_BUS_RUNTIME_FW_ACTIVATION_INFO")
            .field("Version", &self.Version)
            .field("Size", &self.Size)
            .field("RuntimeFwActivationSupported", &self.RuntimeFwActivationSupported)
            .field("FirmwareActivationState", &self.FirmwareActivationState)
            .field("FirmwareActivationCapability", &self.FirmwareActivationCapability)
            .field("EstimatedFirmwareActivationTimeInUSecs", &self.EstimatedFirmwareActivationTimeInUSecs)
            .field("EstimatedProcessorAccessQuiesceTimeInUSecs", &self.EstimatedProcessorAccessQuiesceTimeInUSecs)
            .field("EstimatedIOAccessQuiesceTimeInUSecs", &self.EstimatedIOAccessQuiesceTimeInUSecs)
            .field("PlatformSupportedMaxIOAccessQuiesceTimeInUSecs", &self.PlatformSupportedMaxIOAccessQuiesceTimeInUSecs)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for SCM_BUS_RUNTIME_FW_ACTIVATION_INFO {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SCM_BUS_RUNTIME_FW_ACTIVATION_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SCM_BUS_RUNTIME_FW_ACTIVATION_INFO>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SCM_BUS_RUNTIME_FW_ACTIVATION_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SCM_BUS_RUNTIME_FW_ACTIVATION_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for SCM_BUS_RUNTIME_FW_ACTIVATION_INFO_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCM_BUS_RUNTIME_FW_ACTIVATION_INFO_0").field("_bitfield", &self._bitfield).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for SCM_BUS_RUNTIME_FW_ACTIVATION_INFO_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SCM_BUS_RUNTIME_FW_ACTIVATION_INFO_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SCM_BUS_RUNTIME_FW_ACTIVATION_INFO_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SCM_BUS_RUNTIME_FW_ACTIVATION_INFO_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SCM_BUS_RUNTIME_FW_ACTIVATION_INFO_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SCM_BUS_SET_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ScmBusSet_Descriptor: SCM_BUS_SET_TYPE = SCM_BUS_SET_TYPE(0i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ScmBusSet_IsSupported: SCM_BUS_SET_TYPE = SCM_BUS_SET_TYPE(1i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ScmBusSet_Max: SCM_BUS_SET_TYPE = SCM_BUS_SET_TYPE(2i32);
impl ::core::marker::Copy for SCM_BUS_SET_TYPE {}
impl ::core::clone::Clone for SCM_BUS_SET_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SCM_BUS_SET_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for SCM_BUS_SET_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for SCM_BUS_SET_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SCM_BUS_SET_TYPE").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct SCM_INTERLEAVED_PD_INFO {
    pub DeviceHandle: u32,
    pub DeviceGuid: ::windows::core::GUID,
}
impl ::core::marker::Copy for SCM_INTERLEAVED_PD_INFO {}
impl ::core::clone::Clone for SCM_INTERLEAVED_PD_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SCM_INTERLEAVED_PD_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCM_INTERLEAVED_PD_INFO").field("DeviceHandle", &self.DeviceHandle).field("DeviceGuid", &self.DeviceGuid).finish()
    }
}
unsafe impl ::windows::core::Abi for SCM_INTERLEAVED_PD_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SCM_INTERLEAVED_PD_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SCM_INTERLEAVED_PD_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for SCM_INTERLEAVED_PD_INFO {}
impl ::core::default::Default for SCM_INTERLEAVED_PD_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for SCM_LD_INTERLEAVE_SET_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCM_LD_INTERLEAVE_SET_INFO").field("Version", &self.Version).field("Size", &self.Size).field("InterleaveSetSize", &self.InterleaveSetSize).field("InterleaveSet", &self.InterleaveSet).finish()
    }
}
unsafe impl ::windows::core::Abi for SCM_LD_INTERLEAVE_SET_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SCM_LD_INTERLEAVE_SET_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SCM_LD_INTERLEAVE_SET_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for SCM_LD_INTERLEAVE_SET_INFO {}
impl ::core::default::Default for SCM_LD_INTERLEAVE_SET_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for SCM_LOGICAL_DEVICES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCM_LOGICAL_DEVICES").field("Version", &self.Version).field("Size", &self.Size).field("DeviceCount", &self.DeviceCount).field("Devices", &self.Devices).finish()
    }
}
unsafe impl ::windows::core::Abi for SCM_LOGICAL_DEVICES {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SCM_LOGICAL_DEVICES {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SCM_LOGICAL_DEVICES>()) == 0 }
    }
}
impl ::core::cmp::Eq for SCM_LOGICAL_DEVICES {}
impl ::core::default::Default for SCM_LOGICAL_DEVICES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct SCM_LOGICAL_DEVICE_INSTANCE {
    pub Version: u32,
    pub Size: u32,
    pub DeviceGuid: ::windows::core::GUID,
    pub SymbolicLink: [u16; 256],
}
impl ::core::marker::Copy for SCM_LOGICAL_DEVICE_INSTANCE {}
impl ::core::clone::Clone for SCM_LOGICAL_DEVICE_INSTANCE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SCM_LOGICAL_DEVICE_INSTANCE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCM_LOGICAL_DEVICE_INSTANCE").field("Version", &self.Version).field("Size", &self.Size).field("DeviceGuid", &self.DeviceGuid).field("SymbolicLink", &self.SymbolicLink).finish()
    }
}
unsafe impl ::windows::core::Abi for SCM_LOGICAL_DEVICE_INSTANCE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SCM_LOGICAL_DEVICE_INSTANCE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SCM_LOGICAL_DEVICE_INSTANCE>()) == 0 }
    }
}
impl ::core::cmp::Eq for SCM_LOGICAL_DEVICE_INSTANCE {}
impl ::core::default::Default for SCM_LOGICAL_DEVICE_INSTANCE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for SCM_PD_DESCRIPTOR_HEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCM_PD_DESCRIPTOR_HEADER").field("Version", &self.Version).field("Size", &self.Size).finish()
    }
}
unsafe impl ::windows::core::Abi for SCM_PD_DESCRIPTOR_HEADER {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SCM_PD_DESCRIPTOR_HEADER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SCM_PD_DESCRIPTOR_HEADER>()) == 0 }
    }
}
impl ::core::cmp::Eq for SCM_PD_DESCRIPTOR_HEADER {}
impl ::core::default::Default for SCM_PD_DESCRIPTOR_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct SCM_PD_DEVICE_HANDLE {
    pub Version: u32,
    pub Size: u32,
    pub DeviceGuid: ::windows::core::GUID,
    pub DeviceHandle: u32,
}
impl ::core::marker::Copy for SCM_PD_DEVICE_HANDLE {}
impl ::core::clone::Clone for SCM_PD_DEVICE_HANDLE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SCM_PD_DEVICE_HANDLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCM_PD_DEVICE_HANDLE").field("Version", &self.Version).field("Size", &self.Size).field("DeviceGuid", &self.DeviceGuid).field("DeviceHandle", &self.DeviceHandle).finish()
    }
}
unsafe impl ::windows::core::Abi for SCM_PD_DEVICE_HANDLE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SCM_PD_DEVICE_HANDLE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SCM_PD_DEVICE_HANDLE>()) == 0 }
    }
}
impl ::core::cmp::Eq for SCM_PD_DEVICE_HANDLE {}
impl ::core::default::Default for SCM_PD_DEVICE_HANDLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SCM_PD_DEVICE_INFO {
    pub Version: u32,
    pub Size: u32,
    pub DeviceGuid: ::windows::core::GUID,
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
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for SCM_PD_DEVICE_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCM_PD_DEVICE_INFO")
            .field("Version", &self.Version)
            .field("Size", &self.Size)
            .field("DeviceGuid", &self.DeviceGuid)
            .field("UnsafeShutdownCount", &self.UnsafeShutdownCount)
            .field("PersistentMemorySizeInBytes", &self.PersistentMemorySizeInBytes)
            .field("VolatileMemorySizeInBytes", &self.VolatileMemorySizeInBytes)
            .field("TotalMemorySizeInBytes", &self.TotalMemorySizeInBytes)
            .field("SlotNumber", &self.SlotNumber)
            .field("DeviceHandle", &self.DeviceHandle)
            .field("PhysicalId", &self.PhysicalId)
            .field("NumberOfFormatInterfaceCodes", &self.NumberOfFormatInterfaceCodes)
            .field("FormatInterfaceCodes", &self.FormatInterfaceCodes)
            .field("VendorId", &self.VendorId)
            .field("ProductId", &self.ProductId)
            .field("SubsystemDeviceId", &self.SubsystemDeviceId)
            .field("SubsystemVendorId", &self.SubsystemVendorId)
            .field("ManufacturingLocation", &self.ManufacturingLocation)
            .field("ManufacturingWeek", &self.ManufacturingWeek)
            .field("ManufacturingYear", &self.ManufacturingYear)
            .field("SerialNumber4Byte", &self.SerialNumber4Byte)
            .field("SerialNumberLengthInChars", &self.SerialNumberLengthInChars)
            .field("SerialNumber", &self.SerialNumber)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for SCM_PD_DEVICE_INFO {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SCM_PD_DEVICE_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SCM_PD_DEVICE_INFO>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SCM_PD_DEVICE_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SCM_PD_DEVICE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for SCM_PD_DEVICE_SPECIFIC_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCM_PD_DEVICE_SPECIFIC_INFO").field("Version", &self.Version).field("Size", &self.Size).field("NumberOfProperties", &self.NumberOfProperties).field("DeviceSpecificProperties", &self.DeviceSpecificProperties).finish()
    }
}
unsafe impl ::windows::core::Abi for SCM_PD_DEVICE_SPECIFIC_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SCM_PD_DEVICE_SPECIFIC_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SCM_PD_DEVICE_SPECIFIC_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for SCM_PD_DEVICE_SPECIFIC_INFO {}
impl ::core::default::Default for SCM_PD_DEVICE_SPECIFIC_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for SCM_PD_DEVICE_SPECIFIC_PROPERTY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCM_PD_DEVICE_SPECIFIC_PROPERTY").field("Name", &self.Name).field("Value", &self.Value).finish()
    }
}
unsafe impl ::windows::core::Abi for SCM_PD_DEVICE_SPECIFIC_PROPERTY {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SCM_PD_DEVICE_SPECIFIC_PROPERTY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SCM_PD_DEVICE_SPECIFIC_PROPERTY>()) == 0 }
    }
}
impl ::core::cmp::Eq for SCM_PD_DEVICE_SPECIFIC_PROPERTY {}
impl ::core::default::Default for SCM_PD_DEVICE_SPECIFIC_PROPERTY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for SCM_PD_FIRMWARE_ACTIVATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCM_PD_FIRMWARE_ACTIVATE").field("Version", &self.Version).field("Size", &self.Size).field("Flags", &self.Flags).field("Slot", &self.Slot).finish()
    }
}
unsafe impl ::windows::core::Abi for SCM_PD_FIRMWARE_ACTIVATE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SCM_PD_FIRMWARE_ACTIVATE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SCM_PD_FIRMWARE_ACTIVATE>()) == 0 }
    }
}
impl ::core::cmp::Eq for SCM_PD_FIRMWARE_ACTIVATE {}
impl ::core::default::Default for SCM_PD_FIRMWARE_ACTIVATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SCM_PD_FIRMWARE_ACTIVATION_STATE(pub i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ScmPdFirmwareActivationState_Idle: SCM_PD_FIRMWARE_ACTIVATION_STATE = SCM_PD_FIRMWARE_ACTIVATION_STATE(0i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ScmPdFirmwareActivationState_Armed: SCM_PD_FIRMWARE_ACTIVATION_STATE = SCM_PD_FIRMWARE_ACTIVATION_STATE(1i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ScmPdFirmwareActivationState_Busy: SCM_PD_FIRMWARE_ACTIVATION_STATE = SCM_PD_FIRMWARE_ACTIVATION_STATE(2i32);
impl ::core::marker::Copy for SCM_PD_FIRMWARE_ACTIVATION_STATE {}
impl ::core::clone::Clone for SCM_PD_FIRMWARE_ACTIVATION_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SCM_PD_FIRMWARE_ACTIVATION_STATE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for SCM_PD_FIRMWARE_ACTIVATION_STATE {
    type Abi = Self;
}
impl ::core::fmt::Debug for SCM_PD_FIRMWARE_ACTIVATION_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SCM_PD_FIRMWARE_ACTIVATION_STATE").field(&self.0).finish()
    }
}
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
impl ::core::fmt::Debug for SCM_PD_FIRMWARE_DOWNLOAD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCM_PD_FIRMWARE_DOWNLOAD").field("Version", &self.Version).field("Size", &self.Size).field("Flags", &self.Flags).field("Slot", &self.Slot).field("Reserved", &self.Reserved).field("Offset", &self.Offset).field("FirmwareImageSizeInBytes", &self.FirmwareImageSizeInBytes).field("FirmwareImage", &self.FirmwareImage).finish()
    }
}
unsafe impl ::windows::core::Abi for SCM_PD_FIRMWARE_DOWNLOAD {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SCM_PD_FIRMWARE_DOWNLOAD {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SCM_PD_FIRMWARE_DOWNLOAD>()) == 0 }
    }
}
impl ::core::cmp::Eq for SCM_PD_FIRMWARE_DOWNLOAD {}
impl ::core::default::Default for SCM_PD_FIRMWARE_DOWNLOAD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for SCM_PD_FIRMWARE_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCM_PD_FIRMWARE_INFO").field("Version", &self.Version).field("Size", &self.Size).field("ActiveSlot", &self.ActiveSlot).field("NextActiveSlot", &self.NextActiveSlot).field("SlotCount", &self.SlotCount).field("Slots", &self.Slots).finish()
    }
}
unsafe impl ::windows::core::Abi for SCM_PD_FIRMWARE_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SCM_PD_FIRMWARE_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SCM_PD_FIRMWARE_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for SCM_PD_FIRMWARE_INFO {}
impl ::core::default::Default for SCM_PD_FIRMWARE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for SCM_PD_FIRMWARE_SLOT_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCM_PD_FIRMWARE_SLOT_INFO").field("Version", &self.Version).field("Size", &self.Size).field("SlotNumber", &self.SlotNumber).field("_bitfield", &self._bitfield).field("Reserved1", &self.Reserved1).field("Revision", &self.Revision).finish()
    }
}
unsafe impl ::windows::core::Abi for SCM_PD_FIRMWARE_SLOT_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SCM_PD_FIRMWARE_SLOT_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SCM_PD_FIRMWARE_SLOT_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for SCM_PD_FIRMWARE_SLOT_INFO {}
impl ::core::default::Default for SCM_PD_FIRMWARE_SLOT_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for SCM_PD_FRU_ID_STRING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCM_PD_FRU_ID_STRING").field("Version", &self.Version).field("Size", &self.Size).field("IdentifierSize", &self.IdentifierSize).field("Identifier", &self.Identifier).finish()
    }
}
unsafe impl ::windows::core::Abi for SCM_PD_FRU_ID_STRING {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SCM_PD_FRU_ID_STRING {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SCM_PD_FRU_ID_STRING>()) == 0 }
    }
}
impl ::core::cmp::Eq for SCM_PD_FRU_ID_STRING {}
impl ::core::default::Default for SCM_PD_FRU_ID_STRING {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct SCM_PD_HEALTH_NOTIFICATION_DATA {
    pub DeviceGuid: ::windows::core::GUID,
}
impl ::core::marker::Copy for SCM_PD_HEALTH_NOTIFICATION_DATA {}
impl ::core::clone::Clone for SCM_PD_HEALTH_NOTIFICATION_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SCM_PD_HEALTH_NOTIFICATION_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCM_PD_HEALTH_NOTIFICATION_DATA").field("DeviceGuid", &self.DeviceGuid).finish()
    }
}
unsafe impl ::windows::core::Abi for SCM_PD_HEALTH_NOTIFICATION_DATA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SCM_PD_HEALTH_NOTIFICATION_DATA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SCM_PD_HEALTH_NOTIFICATION_DATA>()) == 0 }
    }
}
impl ::core::cmp::Eq for SCM_PD_HEALTH_NOTIFICATION_DATA {}
impl ::core::default::Default for SCM_PD_HEALTH_NOTIFICATION_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SCM_PD_HEALTH_STATUS(pub i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ScmPhysicalDeviceHealth_Unknown: SCM_PD_HEALTH_STATUS = SCM_PD_HEALTH_STATUS(0i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ScmPhysicalDeviceHealth_Unhealthy: SCM_PD_HEALTH_STATUS = SCM_PD_HEALTH_STATUS(1i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ScmPhysicalDeviceHealth_Warning: SCM_PD_HEALTH_STATUS = SCM_PD_HEALTH_STATUS(2i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ScmPhysicalDeviceHealth_Healthy: SCM_PD_HEALTH_STATUS = SCM_PD_HEALTH_STATUS(3i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ScmPhysicalDeviceHealth_Max: SCM_PD_HEALTH_STATUS = SCM_PD_HEALTH_STATUS(4i32);
impl ::core::marker::Copy for SCM_PD_HEALTH_STATUS {}
impl ::core::clone::Clone for SCM_PD_HEALTH_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SCM_PD_HEALTH_STATUS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for SCM_PD_HEALTH_STATUS {
    type Abi = Self;
}
impl ::core::fmt::Debug for SCM_PD_HEALTH_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SCM_PD_HEALTH_STATUS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SCM_PD_LAST_FW_ACTIVATION_STATUS(pub i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ScmPdLastFwActivationStatus_None: SCM_PD_LAST_FW_ACTIVATION_STATUS = SCM_PD_LAST_FW_ACTIVATION_STATUS(0i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ScmPdLastFwActivationStatus_Success: SCM_PD_LAST_FW_ACTIVATION_STATUS = SCM_PD_LAST_FW_ACTIVATION_STATUS(1i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ScmPdLastFwActivationStatus_FwNotFound: SCM_PD_LAST_FW_ACTIVATION_STATUS = SCM_PD_LAST_FW_ACTIVATION_STATUS(2i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ScmPdLastFwActivationStatus_ColdRebootRequired: SCM_PD_LAST_FW_ACTIVATION_STATUS = SCM_PD_LAST_FW_ACTIVATION_STATUS(3i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ScmPdLastFwActivaitonStatus_ActivationInProgress: SCM_PD_LAST_FW_ACTIVATION_STATUS = SCM_PD_LAST_FW_ACTIVATION_STATUS(4i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ScmPdLastFwActivaitonStatus_Retry: SCM_PD_LAST_FW_ACTIVATION_STATUS = SCM_PD_LAST_FW_ACTIVATION_STATUS(5i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ScmPdLastFwActivaitonStatus_FwUnsupported: SCM_PD_LAST_FW_ACTIVATION_STATUS = SCM_PD_LAST_FW_ACTIVATION_STATUS(6i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ScmPdLastFwActivaitonStatus_UnknownError: SCM_PD_LAST_FW_ACTIVATION_STATUS = SCM_PD_LAST_FW_ACTIVATION_STATUS(7i32);
impl ::core::marker::Copy for SCM_PD_LAST_FW_ACTIVATION_STATUS {}
impl ::core::clone::Clone for SCM_PD_LAST_FW_ACTIVATION_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SCM_PD_LAST_FW_ACTIVATION_STATUS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for SCM_PD_LAST_FW_ACTIVATION_STATUS {
    type Abi = Self;
}
impl ::core::fmt::Debug for SCM_PD_LAST_FW_ACTIVATION_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SCM_PD_LAST_FW_ACTIVATION_STATUS").field(&self.0).finish()
    }
}
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
impl ::core::fmt::Debug for SCM_PD_LOCATION_STRING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCM_PD_LOCATION_STRING").field("Version", &self.Version).field("Size", &self.Size).field("Location", &self.Location).finish()
    }
}
unsafe impl ::windows::core::Abi for SCM_PD_LOCATION_STRING {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SCM_PD_LOCATION_STRING {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SCM_PD_LOCATION_STRING>()) == 0 }
    }
}
impl ::core::cmp::Eq for SCM_PD_LOCATION_STRING {}
impl ::core::default::Default for SCM_PD_LOCATION_STRING {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for SCM_PD_MANAGEMENT_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCM_PD_MANAGEMENT_STATUS").field("Version", &self.Version).field("Size", &self.Size).field("Health", &self.Health).field("NumberOfOperationalStatus", &self.NumberOfOperationalStatus).field("NumberOfAdditionalReasons", &self.NumberOfAdditionalReasons).field("OperationalStatus", &self.OperationalStatus).field("AdditionalReasons", &self.AdditionalReasons).finish()
    }
}
unsafe impl ::windows::core::Abi for SCM_PD_MANAGEMENT_STATUS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SCM_PD_MANAGEMENT_STATUS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SCM_PD_MANAGEMENT_STATUS>()) == 0 }
    }
}
impl ::core::cmp::Eq for SCM_PD_MANAGEMENT_STATUS {}
impl ::core::default::Default for SCM_PD_MANAGEMENT_STATUS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const SCM_PD_MAX_OPERATIONAL_STATUS: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SCM_PD_MEDIA_REINITIALIZATION_STATUS(pub i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ScmPhysicalDeviceReinit_Success: SCM_PD_MEDIA_REINITIALIZATION_STATUS = SCM_PD_MEDIA_REINITIALIZATION_STATUS(0i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ScmPhysicalDeviceReinit_RebootNeeded: SCM_PD_MEDIA_REINITIALIZATION_STATUS = SCM_PD_MEDIA_REINITIALIZATION_STATUS(1i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ScmPhysicalDeviceReinit_ColdBootNeeded: SCM_PD_MEDIA_REINITIALIZATION_STATUS = SCM_PD_MEDIA_REINITIALIZATION_STATUS(2i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ScmPhysicalDeviceReinit_Max: SCM_PD_MEDIA_REINITIALIZATION_STATUS = SCM_PD_MEDIA_REINITIALIZATION_STATUS(3i32);
impl ::core::marker::Copy for SCM_PD_MEDIA_REINITIALIZATION_STATUS {}
impl ::core::clone::Clone for SCM_PD_MEDIA_REINITIALIZATION_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SCM_PD_MEDIA_REINITIALIZATION_STATUS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for SCM_PD_MEDIA_REINITIALIZATION_STATUS {
    type Abi = Self;
}
impl ::core::fmt::Debug for SCM_PD_MEDIA_REINITIALIZATION_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SCM_PD_MEDIA_REINITIALIZATION_STATUS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SCM_PD_OPERATIONAL_STATUS(pub i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ScmPhysicalDeviceOpStatus_Unknown: SCM_PD_OPERATIONAL_STATUS = SCM_PD_OPERATIONAL_STATUS(0i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ScmPhysicalDeviceOpStatus_Ok: SCM_PD_OPERATIONAL_STATUS = SCM_PD_OPERATIONAL_STATUS(1i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ScmPhysicalDeviceOpStatus_PredictingFailure: SCM_PD_OPERATIONAL_STATUS = SCM_PD_OPERATIONAL_STATUS(2i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ScmPhysicalDeviceOpStatus_InService: SCM_PD_OPERATIONAL_STATUS = SCM_PD_OPERATIONAL_STATUS(3i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ScmPhysicalDeviceOpStatus_HardwareError: SCM_PD_OPERATIONAL_STATUS = SCM_PD_OPERATIONAL_STATUS(4i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ScmPhysicalDeviceOpStatus_NotUsable: SCM_PD_OPERATIONAL_STATUS = SCM_PD_OPERATIONAL_STATUS(5i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ScmPhysicalDeviceOpStatus_TransientError: SCM_PD_OPERATIONAL_STATUS = SCM_PD_OPERATIONAL_STATUS(6i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ScmPhysicalDeviceOpStatus_Missing: SCM_PD_OPERATIONAL_STATUS = SCM_PD_OPERATIONAL_STATUS(7i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ScmPhysicalDeviceOpStatus_Max: SCM_PD_OPERATIONAL_STATUS = SCM_PD_OPERATIONAL_STATUS(8i32);
impl ::core::marker::Copy for SCM_PD_OPERATIONAL_STATUS {}
impl ::core::clone::Clone for SCM_PD_OPERATIONAL_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SCM_PD_OPERATIONAL_STATUS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for SCM_PD_OPERATIONAL_STATUS {
    type Abi = Self;
}
impl ::core::fmt::Debug for SCM_PD_OPERATIONAL_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SCM_PD_OPERATIONAL_STATUS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SCM_PD_OPERATIONAL_STATUS_REASON(pub i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ScmPhysicalDeviceOpReason_Unknown: SCM_PD_OPERATIONAL_STATUS_REASON = SCM_PD_OPERATIONAL_STATUS_REASON(0i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ScmPhysicalDeviceOpReason_Media: SCM_PD_OPERATIONAL_STATUS_REASON = SCM_PD_OPERATIONAL_STATUS_REASON(1i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ScmPhysicalDeviceOpReason_ThresholdExceeded: SCM_PD_OPERATIONAL_STATUS_REASON = SCM_PD_OPERATIONAL_STATUS_REASON(2i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ScmPhysicalDeviceOpReason_LostData: SCM_PD_OPERATIONAL_STATUS_REASON = SCM_PD_OPERATIONAL_STATUS_REASON(3i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ScmPhysicalDeviceOpReason_EnergySource: SCM_PD_OPERATIONAL_STATUS_REASON = SCM_PD_OPERATIONAL_STATUS_REASON(4i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ScmPhysicalDeviceOpReason_Configuration: SCM_PD_OPERATIONAL_STATUS_REASON = SCM_PD_OPERATIONAL_STATUS_REASON(5i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ScmPhysicalDeviceOpReason_DeviceController: SCM_PD_OPERATIONAL_STATUS_REASON = SCM_PD_OPERATIONAL_STATUS_REASON(6i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ScmPhysicalDeviceOpReason_MediaController: SCM_PD_OPERATIONAL_STATUS_REASON = SCM_PD_OPERATIONAL_STATUS_REASON(7i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ScmPhysicalDeviceOpReason_Component: SCM_PD_OPERATIONAL_STATUS_REASON = SCM_PD_OPERATIONAL_STATUS_REASON(8i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ScmPhysicalDeviceOpReason_BackgroundOperation: SCM_PD_OPERATIONAL_STATUS_REASON = SCM_PD_OPERATIONAL_STATUS_REASON(9i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ScmPhysicalDeviceOpReason_InvalidFirmware: SCM_PD_OPERATIONAL_STATUS_REASON = SCM_PD_OPERATIONAL_STATUS_REASON(10i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ScmPhysicalDeviceOpReason_HealthCheck: SCM_PD_OPERATIONAL_STATUS_REASON = SCM_PD_OPERATIONAL_STATUS_REASON(11i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ScmPhysicalDeviceOpReason_LostDataPersistence: SCM_PD_OPERATIONAL_STATUS_REASON = SCM_PD_OPERATIONAL_STATUS_REASON(12i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ScmPhysicalDeviceOpReason_DisabledByPlatform: SCM_PD_OPERATIONAL_STATUS_REASON = SCM_PD_OPERATIONAL_STATUS_REASON(13i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ScmPhysicalDeviceOpReason_PermanentError: SCM_PD_OPERATIONAL_STATUS_REASON = SCM_PD_OPERATIONAL_STATUS_REASON(14i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ScmPhysicalDeviceOpReason_LostWritePersistence: SCM_PD_OPERATIONAL_STATUS_REASON = SCM_PD_OPERATIONAL_STATUS_REASON(15i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ScmPhysicalDeviceOpReason_FatalError: SCM_PD_OPERATIONAL_STATUS_REASON = SCM_PD_OPERATIONAL_STATUS_REASON(16i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ScmPhysicalDeviceOpReason_DataPersistenceLossImminent: SCM_PD_OPERATIONAL_STATUS_REASON = SCM_PD_OPERATIONAL_STATUS_REASON(17i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ScmPhysicalDeviceOpReason_WritePersistenceLossImminent: SCM_PD_OPERATIONAL_STATUS_REASON = SCM_PD_OPERATIONAL_STATUS_REASON(18i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ScmPhysicalDeviceOpReason_MediaRemainingSpareBlock: SCM_PD_OPERATIONAL_STATUS_REASON = SCM_PD_OPERATIONAL_STATUS_REASON(19i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ScmPhysicalDeviceOpReason_PerformanceDegradation: SCM_PD_OPERATIONAL_STATUS_REASON = SCM_PD_OPERATIONAL_STATUS_REASON(20i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ScmPhysicalDeviceOpReason_ExcessiveTemperature: SCM_PD_OPERATIONAL_STATUS_REASON = SCM_PD_OPERATIONAL_STATUS_REASON(21i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ScmPhysicalDeviceOpReason_InternalFailure: SCM_PD_OPERATIONAL_STATUS_REASON = SCM_PD_OPERATIONAL_STATUS_REASON(22i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ScmPhysicalDeviceOpReason_Max: SCM_PD_OPERATIONAL_STATUS_REASON = SCM_PD_OPERATIONAL_STATUS_REASON(23i32);
impl ::core::marker::Copy for SCM_PD_OPERATIONAL_STATUS_REASON {}
impl ::core::clone::Clone for SCM_PD_OPERATIONAL_STATUS_REASON {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SCM_PD_OPERATIONAL_STATUS_REASON {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for SCM_PD_OPERATIONAL_STATUS_REASON {
    type Abi = Self;
}
impl ::core::fmt::Debug for SCM_PD_OPERATIONAL_STATUS_REASON {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SCM_PD_OPERATIONAL_STATUS_REASON").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct SCM_PD_PASSTHROUGH_INPUT {
    pub Version: u32,
    pub Size: u32,
    pub ProtocolGuid: ::windows::core::GUID,
    pub DataSize: u32,
    pub Data: [u8; 1],
}
impl ::core::marker::Copy for SCM_PD_PASSTHROUGH_INPUT {}
impl ::core::clone::Clone for SCM_PD_PASSTHROUGH_INPUT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SCM_PD_PASSTHROUGH_INPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCM_PD_PASSTHROUGH_INPUT").field("Version", &self.Version).field("Size", &self.Size).field("ProtocolGuid", &self.ProtocolGuid).field("DataSize", &self.DataSize).field("Data", &self.Data).finish()
    }
}
unsafe impl ::windows::core::Abi for SCM_PD_PASSTHROUGH_INPUT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SCM_PD_PASSTHROUGH_INPUT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SCM_PD_PASSTHROUGH_INPUT>()) == 0 }
    }
}
impl ::core::cmp::Eq for SCM_PD_PASSTHROUGH_INPUT {}
impl ::core::default::Default for SCM_PD_PASSTHROUGH_INPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for SCM_PD_PASSTHROUGH_INVDIMM_INPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCM_PD_PASSTHROUGH_INVDIMM_INPUT").field("Opcode", &self.Opcode).field("OpcodeParametersLength", &self.OpcodeParametersLength).field("OpcodeParameters", &self.OpcodeParameters).finish()
    }
}
unsafe impl ::windows::core::Abi for SCM_PD_PASSTHROUGH_INVDIMM_INPUT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SCM_PD_PASSTHROUGH_INVDIMM_INPUT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SCM_PD_PASSTHROUGH_INVDIMM_INPUT>()) == 0 }
    }
}
impl ::core::cmp::Eq for SCM_PD_PASSTHROUGH_INVDIMM_INPUT {}
impl ::core::default::Default for SCM_PD_PASSTHROUGH_INVDIMM_INPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for SCM_PD_PASSTHROUGH_INVDIMM_OUTPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCM_PD_PASSTHROUGH_INVDIMM_OUTPUT").field("GeneralStatus", &self.GeneralStatus).field("ExtendedStatus", &self.ExtendedStatus).field("OutputDataLength", &self.OutputDataLength).field("OutputData", &self.OutputData).finish()
    }
}
unsafe impl ::windows::core::Abi for SCM_PD_PASSTHROUGH_INVDIMM_OUTPUT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SCM_PD_PASSTHROUGH_INVDIMM_OUTPUT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SCM_PD_PASSTHROUGH_INVDIMM_OUTPUT>()) == 0 }
    }
}
impl ::core::cmp::Eq for SCM_PD_PASSTHROUGH_INVDIMM_OUTPUT {}
impl ::core::default::Default for SCM_PD_PASSTHROUGH_INVDIMM_OUTPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct SCM_PD_PASSTHROUGH_OUTPUT {
    pub Version: u32,
    pub Size: u32,
    pub ProtocolGuid: ::windows::core::GUID,
    pub DataSize: u32,
    pub Data: [u8; 1],
}
impl ::core::marker::Copy for SCM_PD_PASSTHROUGH_OUTPUT {}
impl ::core::clone::Clone for SCM_PD_PASSTHROUGH_OUTPUT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SCM_PD_PASSTHROUGH_OUTPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCM_PD_PASSTHROUGH_OUTPUT").field("Version", &self.Version).field("Size", &self.Size).field("ProtocolGuid", &self.ProtocolGuid).field("DataSize", &self.DataSize).field("Data", &self.Data).finish()
    }
}
unsafe impl ::windows::core::Abi for SCM_PD_PASSTHROUGH_OUTPUT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SCM_PD_PASSTHROUGH_OUTPUT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SCM_PD_PASSTHROUGH_OUTPUT>()) == 0 }
    }
}
impl ::core::cmp::Eq for SCM_PD_PASSTHROUGH_OUTPUT {}
impl ::core::default::Default for SCM_PD_PASSTHROUGH_OUTPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SCM_PD_PROPERTY_ID(pub i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ScmPhysicalDeviceProperty_DeviceInfo: SCM_PD_PROPERTY_ID = SCM_PD_PROPERTY_ID(0i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ScmPhysicalDeviceProperty_ManagementStatus: SCM_PD_PROPERTY_ID = SCM_PD_PROPERTY_ID(1i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ScmPhysicalDeviceProperty_FirmwareInfo: SCM_PD_PROPERTY_ID = SCM_PD_PROPERTY_ID(2i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ScmPhysicalDeviceProperty_LocationString: SCM_PD_PROPERTY_ID = SCM_PD_PROPERTY_ID(3i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ScmPhysicalDeviceProperty_DeviceSpecificInfo: SCM_PD_PROPERTY_ID = SCM_PD_PROPERTY_ID(4i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ScmPhysicalDeviceProperty_DeviceHandle: SCM_PD_PROPERTY_ID = SCM_PD_PROPERTY_ID(5i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ScmPhysicalDeviceProperty_FruIdString: SCM_PD_PROPERTY_ID = SCM_PD_PROPERTY_ID(6i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ScmPhysicalDeviceProperty_RuntimeFwActivationInfo: SCM_PD_PROPERTY_ID = SCM_PD_PROPERTY_ID(7i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ScmPhysicalDeviceProperty_RuntimeFwActivationArmState: SCM_PD_PROPERTY_ID = SCM_PD_PROPERTY_ID(8i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ScmPhysicalDeviceProperty_Max: SCM_PD_PROPERTY_ID = SCM_PD_PROPERTY_ID(9i32);
impl ::core::marker::Copy for SCM_PD_PROPERTY_ID {}
impl ::core::clone::Clone for SCM_PD_PROPERTY_ID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SCM_PD_PROPERTY_ID {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for SCM_PD_PROPERTY_ID {
    type Abi = Self;
}
impl ::core::fmt::Debug for SCM_PD_PROPERTY_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SCM_PD_PROPERTY_ID").field(&self.0).finish()
    }
}
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
impl ::core::fmt::Debug for SCM_PD_PROPERTY_QUERY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCM_PD_PROPERTY_QUERY").field("Version", &self.Version).field("Size", &self.Size).field("PropertyId", &self.PropertyId).field("QueryType", &self.QueryType).field("AdditionalParameters", &self.AdditionalParameters).finish()
    }
}
unsafe impl ::windows::core::Abi for SCM_PD_PROPERTY_QUERY {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SCM_PD_PROPERTY_QUERY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SCM_PD_PROPERTY_QUERY>()) == 0 }
    }
}
impl ::core::cmp::Eq for SCM_PD_PROPERTY_QUERY {}
impl ::core::default::Default for SCM_PD_PROPERTY_QUERY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for SCM_PD_PROPERTY_SET {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCM_PD_PROPERTY_SET").field("Version", &self.Version).field("Size", &self.Size).field("PropertyId", &self.PropertyId).field("SetType", &self.SetType).field("AdditionalParameters", &self.AdditionalParameters).finish()
    }
}
unsafe impl ::windows::core::Abi for SCM_PD_PROPERTY_SET {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SCM_PD_PROPERTY_SET {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SCM_PD_PROPERTY_SET>()) == 0 }
    }
}
impl ::core::cmp::Eq for SCM_PD_PROPERTY_SET {}
impl ::core::default::Default for SCM_PD_PROPERTY_SET {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SCM_PD_QUERY_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ScmPhysicalDeviceQuery_Descriptor: SCM_PD_QUERY_TYPE = SCM_PD_QUERY_TYPE(0i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ScmPhysicalDeviceQuery_IsSupported: SCM_PD_QUERY_TYPE = SCM_PD_QUERY_TYPE(1i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ScmPhysicalDeviceQuery_Max: SCM_PD_QUERY_TYPE = SCM_PD_QUERY_TYPE(2i32);
impl ::core::marker::Copy for SCM_PD_QUERY_TYPE {}
impl ::core::clone::Clone for SCM_PD_QUERY_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SCM_PD_QUERY_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for SCM_PD_QUERY_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for SCM_PD_QUERY_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SCM_PD_QUERY_TYPE").field(&self.0).finish()
    }
}
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
impl ::core::fmt::Debug for SCM_PD_REINITIALIZE_MEDIA_INPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCM_PD_REINITIALIZE_MEDIA_INPUT").field("Version", &self.Version).field("Size", &self.Size).field("Options", &self.Options).finish()
    }
}
unsafe impl ::windows::core::Abi for SCM_PD_REINITIALIZE_MEDIA_INPUT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SCM_PD_REINITIALIZE_MEDIA_INPUT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SCM_PD_REINITIALIZE_MEDIA_INPUT>()) == 0 }
    }
}
impl ::core::cmp::Eq for SCM_PD_REINITIALIZE_MEDIA_INPUT {}
impl ::core::default::Default for SCM_PD_REINITIALIZE_MEDIA_INPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for SCM_PD_REINITIALIZE_MEDIA_INPUT_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCM_PD_REINITIALIZE_MEDIA_INPUT_0").field("_bitfield", &self._bitfield).finish()
    }
}
unsafe impl ::windows::core::Abi for SCM_PD_REINITIALIZE_MEDIA_INPUT_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SCM_PD_REINITIALIZE_MEDIA_INPUT_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SCM_PD_REINITIALIZE_MEDIA_INPUT_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for SCM_PD_REINITIALIZE_MEDIA_INPUT_0 {}
impl ::core::default::Default for SCM_PD_REINITIALIZE_MEDIA_INPUT_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for SCM_PD_REINITIALIZE_MEDIA_OUTPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCM_PD_REINITIALIZE_MEDIA_OUTPUT").field("Version", &self.Version).field("Size", &self.Size).field("Status", &self.Status).finish()
    }
}
unsafe impl ::windows::core::Abi for SCM_PD_REINITIALIZE_MEDIA_OUTPUT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SCM_PD_REINITIALIZE_MEDIA_OUTPUT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SCM_PD_REINITIALIZE_MEDIA_OUTPUT>()) == 0 }
    }
}
impl ::core::cmp::Eq for SCM_PD_REINITIALIZE_MEDIA_OUTPUT {}
impl ::core::default::Default for SCM_PD_REINITIALIZE_MEDIA_OUTPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for SCM_PD_RUNTIME_FW_ACTIVATION_ARM_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCM_PD_RUNTIME_FW_ACTIVATION_ARM_STATE").field("ArmState", &self.ArmState).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for SCM_PD_RUNTIME_FW_ACTIVATION_ARM_STATE {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SCM_PD_RUNTIME_FW_ACTIVATION_ARM_STATE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SCM_PD_RUNTIME_FW_ACTIVATION_ARM_STATE>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SCM_PD_RUNTIME_FW_ACTIVATION_ARM_STATE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SCM_PD_RUNTIME_FW_ACTIVATION_ARM_STATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for SCM_PD_RUNTIME_FW_ACTIVATION_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCM_PD_RUNTIME_FW_ACTIVATION_INFO").field("Version", &self.Version).field("Size", &self.Size).field("LastFirmwareActivationStatus", &self.LastFirmwareActivationStatus).field("FirmwareActivationState", &self.FirmwareActivationState).finish()
    }
}
unsafe impl ::windows::core::Abi for SCM_PD_RUNTIME_FW_ACTIVATION_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SCM_PD_RUNTIME_FW_ACTIVATION_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SCM_PD_RUNTIME_FW_ACTIVATION_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for SCM_PD_RUNTIME_FW_ACTIVATION_INFO {}
impl ::core::default::Default for SCM_PD_RUNTIME_FW_ACTIVATION_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SCM_PD_SET_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ScmPhysicalDeviceSet_Descriptor: SCM_PD_SET_TYPE = SCM_PD_SET_TYPE(0i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ScmPhysicalDeviceSet_IsSupported: SCM_PD_SET_TYPE = SCM_PD_SET_TYPE(1i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ScmPhysicalDeviceSet_Max: SCM_PD_SET_TYPE = SCM_PD_SET_TYPE(2i32);
impl ::core::marker::Copy for SCM_PD_SET_TYPE {}
impl ::core::clone::Clone for SCM_PD_SET_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SCM_PD_SET_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for SCM_PD_SET_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for SCM_PD_SET_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SCM_PD_SET_TYPE").field(&self.0).finish()
    }
}
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
impl ::core::fmt::Debug for SCM_PHYSICAL_DEVICES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCM_PHYSICAL_DEVICES").field("Version", &self.Version).field("Size", &self.Size).field("DeviceCount", &self.DeviceCount).field("Devices", &self.Devices).finish()
    }
}
unsafe impl ::windows::core::Abi for SCM_PHYSICAL_DEVICES {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SCM_PHYSICAL_DEVICES {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SCM_PHYSICAL_DEVICES>()) == 0 }
    }
}
impl ::core::cmp::Eq for SCM_PHYSICAL_DEVICES {}
impl ::core::default::Default for SCM_PHYSICAL_DEVICES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for SCM_PHYSICAL_DEVICE_INSTANCE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCM_PHYSICAL_DEVICE_INSTANCE").field("Version", &self.Version).field("Size", &self.Size).field("NfitHandle", &self.NfitHandle).field("SymbolicLink", &self.SymbolicLink).finish()
    }
}
unsafe impl ::windows::core::Abi for SCM_PHYSICAL_DEVICE_INSTANCE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SCM_PHYSICAL_DEVICE_INSTANCE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SCM_PHYSICAL_DEVICE_INSTANCE>()) == 0 }
    }
}
impl ::core::cmp::Eq for SCM_PHYSICAL_DEVICE_INSTANCE {}
impl ::core::default::Default for SCM_PHYSICAL_DEVICE_INSTANCE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct SCM_REGION {
    pub Version: u32,
    pub Size: u32,
    pub Flags: u32,
    pub NfitHandle: u32,
    pub LogicalDeviceGuid: ::windows::core::GUID,
    pub AddressRangeType: ::windows::core::GUID,
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
impl ::core::fmt::Debug for SCM_REGION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCM_REGION")
            .field("Version", &self.Version)
            .field("Size", &self.Size)
            .field("Flags", &self.Flags)
            .field("NfitHandle", &self.NfitHandle)
            .field("LogicalDeviceGuid", &self.LogicalDeviceGuid)
            .field("AddressRangeType", &self.AddressRangeType)
            .field("AssociatedId", &self.AssociatedId)
            .field("Length", &self.Length)
            .field("StartingDPA", &self.StartingDPA)
            .field("BaseSPA", &self.BaseSPA)
            .field("SPAOffset", &self.SPAOffset)
            .field("RegionOffset", &self.RegionOffset)
            .finish()
    }
}
unsafe impl ::windows::core::Abi for SCM_REGION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SCM_REGION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SCM_REGION>()) == 0 }
    }
}
impl ::core::cmp::Eq for SCM_REGION {}
impl ::core::default::Default for SCM_REGION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for SCM_REGIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCM_REGIONS").field("Version", &self.Version).field("Size", &self.Size).field("RegionCount", &self.RegionCount).field("Regions", &self.Regions).finish()
    }
}
unsafe impl ::windows::core::Abi for SCM_REGIONS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SCM_REGIONS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SCM_REGIONS>()) == 0 }
    }
}
impl ::core::cmp::Eq for SCM_REGIONS {}
impl ::core::default::Default for SCM_REGIONS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SCM_REGION_FLAG(pub i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ScmRegionFlagNone: SCM_REGION_FLAG = SCM_REGION_FLAG(0i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ScmRegionFlagLabel: SCM_REGION_FLAG = SCM_REGION_FLAG(1i32);
impl ::core::marker::Copy for SCM_REGION_FLAG {}
impl ::core::clone::Clone for SCM_REGION_FLAG {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SCM_REGION_FLAG {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for SCM_REGION_FLAG {
    type Abi = Self;
}
impl ::core::fmt::Debug for SCM_REGION_FLAG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SCM_REGION_FLAG").field(&self.0).finish()
    }
}
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
impl ::core::fmt::Debug for SD_CHANGE_MACHINE_SID_INPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SD_CHANGE_MACHINE_SID_INPUT").field("CurrentMachineSIDOffset", &self.CurrentMachineSIDOffset).field("CurrentMachineSIDLength", &self.CurrentMachineSIDLength).field("NewMachineSIDOffset", &self.NewMachineSIDOffset).field("NewMachineSIDLength", &self.NewMachineSIDLength).finish()
    }
}
unsafe impl ::windows::core::Abi for SD_CHANGE_MACHINE_SID_INPUT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SD_CHANGE_MACHINE_SID_INPUT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SD_CHANGE_MACHINE_SID_INPUT>()) == 0 }
    }
}
impl ::core::cmp::Eq for SD_CHANGE_MACHINE_SID_INPUT {}
impl ::core::default::Default for SD_CHANGE_MACHINE_SID_INPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for SD_CHANGE_MACHINE_SID_OUTPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SD_CHANGE_MACHINE_SID_OUTPUT").field("NumSDChangedSuccess", &self.NumSDChangedSuccess).field("NumSDChangedFail", &self.NumSDChangedFail).field("NumSDUnused", &self.NumSDUnused).field("NumSDTotal", &self.NumSDTotal).field("NumMftSDChangedSuccess", &self.NumMftSDChangedSuccess).field("NumMftSDChangedFail", &self.NumMftSDChangedFail).field("NumMftSDTotal", &self.NumMftSDTotal).finish()
    }
}
unsafe impl ::windows::core::Abi for SD_CHANGE_MACHINE_SID_OUTPUT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SD_CHANGE_MACHINE_SID_OUTPUT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SD_CHANGE_MACHINE_SID_OUTPUT>()) == 0 }
    }
}
impl ::core::cmp::Eq for SD_CHANGE_MACHINE_SID_OUTPUT {}
impl ::core::default::Default for SD_CHANGE_MACHINE_SID_OUTPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for SD_ENUM_SDS_ENTRY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SD_ENUM_SDS_ENTRY").field("Hash", &self.Hash).field("SecurityId", &self.SecurityId).field("Offset", &self.Offset).field("Length", &self.Length).field("Descriptor", &self.Descriptor).finish()
    }
}
unsafe impl ::windows::core::Abi for SD_ENUM_SDS_ENTRY {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SD_ENUM_SDS_ENTRY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SD_ENUM_SDS_ENTRY>()) == 0 }
    }
}
impl ::core::cmp::Eq for SD_ENUM_SDS_ENTRY {}
impl ::core::default::Default for SD_ENUM_SDS_ENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for SD_ENUM_SDS_INPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SD_ENUM_SDS_INPUT").field("StartingOffset", &self.StartingOffset).field("MaxSDEntriesToReturn", &self.MaxSDEntriesToReturn).finish()
    }
}
unsafe impl ::windows::core::Abi for SD_ENUM_SDS_INPUT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SD_ENUM_SDS_INPUT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SD_ENUM_SDS_INPUT>()) == 0 }
    }
}
impl ::core::cmp::Eq for SD_ENUM_SDS_INPUT {}
impl ::core::default::Default for SD_ENUM_SDS_INPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for SD_ENUM_SDS_OUTPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SD_ENUM_SDS_OUTPUT").field("NextOffset", &self.NextOffset).field("NumSDEntriesReturned", &self.NumSDEntriesReturned).field("NumSDBytesReturned", &self.NumSDBytesReturned).field("SDEntry", &self.SDEntry).finish()
    }
}
unsafe impl ::windows::core::Abi for SD_ENUM_SDS_OUTPUT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SD_ENUM_SDS_OUTPUT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SD_ENUM_SDS_OUTPUT>()) == 0 }
    }
}
impl ::core::cmp::Eq for SD_ENUM_SDS_OUTPUT {}
impl ::core::default::Default for SD_ENUM_SDS_OUTPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
unsafe impl ::windows::core::Abi for SD_GLOBAL_CHANGE_INPUT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SD_GLOBAL_CHANGE_INPUT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SD_GLOBAL_CHANGE_INPUT>()) == 0 }
    }
}
impl ::core::cmp::Eq for SD_GLOBAL_CHANGE_INPUT {}
impl ::core::default::Default for SD_GLOBAL_CHANGE_INPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
unsafe impl ::windows::core::Abi for SD_GLOBAL_CHANGE_INPUT_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SD_GLOBAL_CHANGE_INPUT_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SD_GLOBAL_CHANGE_INPUT_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for SD_GLOBAL_CHANGE_INPUT_0 {}
impl ::core::default::Default for SD_GLOBAL_CHANGE_INPUT_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
unsafe impl ::windows::core::Abi for SD_GLOBAL_CHANGE_OUTPUT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SD_GLOBAL_CHANGE_OUTPUT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SD_GLOBAL_CHANGE_OUTPUT>()) == 0 }
    }
}
impl ::core::cmp::Eq for SD_GLOBAL_CHANGE_OUTPUT {}
impl ::core::default::Default for SD_GLOBAL_CHANGE_OUTPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
unsafe impl ::windows::core::Abi for SD_GLOBAL_CHANGE_OUTPUT_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SD_GLOBAL_CHANGE_OUTPUT_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SD_GLOBAL_CHANGE_OUTPUT_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for SD_GLOBAL_CHANGE_OUTPUT_0 {}
impl ::core::default::Default for SD_GLOBAL_CHANGE_OUTPUT_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for SD_QUERY_STATS_INPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SD_QUERY_STATS_INPUT").field("Reserved", &self.Reserved).finish()
    }
}
unsafe impl ::windows::core::Abi for SD_QUERY_STATS_INPUT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SD_QUERY_STATS_INPUT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SD_QUERY_STATS_INPUT>()) == 0 }
    }
}
impl ::core::cmp::Eq for SD_QUERY_STATS_INPUT {}
impl ::core::default::Default for SD_QUERY_STATS_INPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for SD_QUERY_STATS_OUTPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SD_QUERY_STATS_OUTPUT").field("SdsStreamSize", &self.SdsStreamSize).field("SdsAllocationSize", &self.SdsAllocationSize).field("SiiStreamSize", &self.SiiStreamSize).field("SiiAllocationSize", &self.SiiAllocationSize).field("SdhStreamSize", &self.SdhStreamSize).field("SdhAllocationSize", &self.SdhAllocationSize).field("NumSDTotal", &self.NumSDTotal).field("NumSDUnused", &self.NumSDUnused).finish()
    }
}
unsafe impl ::windows::core::Abi for SD_QUERY_STATS_OUTPUT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SD_QUERY_STATS_OUTPUT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SD_QUERY_STATS_OUTPUT>()) == 0 }
    }
}
impl ::core::cmp::Eq for SD_QUERY_STATS_OUTPUT {}
impl ::core::default::Default for SD_QUERY_STATS_OUTPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
unsafe impl ::windows::core::Abi for SENDCMDINPARAMS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SENDCMDINPARAMS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SENDCMDINPARAMS>()) == 0 }
    }
}
impl ::core::cmp::Eq for SENDCMDINPARAMS {}
impl ::core::default::Default for SENDCMDINPARAMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
unsafe impl ::windows::core::Abi for SENDCMDOUTPARAMS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SENDCMDOUTPARAMS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SENDCMDOUTPARAMS>()) == 0 }
    }
}
impl ::core::cmp::Eq for SENDCMDOUTPARAMS {}
impl ::core::default::Default for SENDCMDOUTPARAMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for SET_DAX_ALLOC_ALIGNMENT_HINT_INPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SET_DAX_ALLOC_ALIGNMENT_HINT_INPUT").field("Flags", &self.Flags).field("AlignmentShift", &self.AlignmentShift).field("FileOffsetToAlign", &self.FileOffsetToAlign).field("FallbackAlignmentShift", &self.FallbackAlignmentShift).finish()
    }
}
unsafe impl ::windows::core::Abi for SET_DAX_ALLOC_ALIGNMENT_HINT_INPUT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SET_DAX_ALLOC_ALIGNMENT_HINT_INPUT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SET_DAX_ALLOC_ALIGNMENT_HINT_INPUT>()) == 0 }
    }
}
impl ::core::cmp::Eq for SET_DAX_ALLOC_ALIGNMENT_HINT_INPUT {}
impl ::core::default::Default for SET_DAX_ALLOC_ALIGNMENT_HINT_INPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for SET_DISK_ATTRIBUTES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SET_DISK_ATTRIBUTES").field("Version", &self.Version).field("Persist", &self.Persist).field("Reserved1", &self.Reserved1).field("Attributes", &self.Attributes).field("AttributesMask", &self.AttributesMask).field("Reserved2", &self.Reserved2).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for SET_DISK_ATTRIBUTES {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SET_DISK_ATTRIBUTES {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SET_DISK_ATTRIBUTES>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SET_DISK_ATTRIBUTES {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SET_DISK_ATTRIBUTES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for SET_PARTITION_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SET_PARTITION_INFORMATION").field("PartitionType", &self.PartitionType).finish()
    }
}
unsafe impl ::windows::core::Abi for SET_PARTITION_INFORMATION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SET_PARTITION_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SET_PARTITION_INFORMATION>()) == 0 }
    }
}
impl ::core::cmp::Eq for SET_PARTITION_INFORMATION {}
impl ::core::default::Default for SET_PARTITION_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
unsafe impl ::windows::core::Abi for SET_PARTITION_INFORMATION_EX {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SET_PARTITION_INFORMATION_EX {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SET_PARTITION_INFORMATION_EX>()) == 0 }
    }
}
impl ::core::cmp::Eq for SET_PARTITION_INFORMATION_EX {}
impl ::core::default::Default for SET_PARTITION_INFORMATION_EX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
unsafe impl ::windows::core::Abi for SET_PARTITION_INFORMATION_EX_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SET_PARTITION_INFORMATION_EX_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SET_PARTITION_INFORMATION_EX_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for SET_PARTITION_INFORMATION_EX_0 {}
impl ::core::default::Default for SET_PARTITION_INFORMATION_EX_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for SET_PURGE_FAILURE_MODE_INPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SET_PURGE_FAILURE_MODE_INPUT").field("Flags", &self.Flags).finish()
    }
}
unsafe impl ::windows::core::Abi for SET_PURGE_FAILURE_MODE_INPUT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SET_PURGE_FAILURE_MODE_INPUT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SET_PURGE_FAILURE_MODE_INPUT>()) == 0 }
    }
}
impl ::core::cmp::Eq for SET_PURGE_FAILURE_MODE_INPUT {}
impl ::core::default::Default for SET_PURGE_FAILURE_MODE_INPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for SHRINK_VOLUME_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SHRINK_VOLUME_INFORMATION").field("ShrinkRequestType", &self.ShrinkRequestType).field("Flags", &self.Flags).field("NewNumberOfSectors", &self.NewNumberOfSectors).finish()
    }
}
unsafe impl ::windows::core::Abi for SHRINK_VOLUME_INFORMATION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SHRINK_VOLUME_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SHRINK_VOLUME_INFORMATION>()) == 0 }
    }
}
impl ::core::cmp::Eq for SHRINK_VOLUME_INFORMATION {}
impl ::core::default::Default for SHRINK_VOLUME_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SHRINK_VOLUME_REQUEST_TYPES(pub i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ShrinkPrepare: SHRINK_VOLUME_REQUEST_TYPES = SHRINK_VOLUME_REQUEST_TYPES(1i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ShrinkCommit: SHRINK_VOLUME_REQUEST_TYPES = SHRINK_VOLUME_REQUEST_TYPES(2i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ShrinkAbort: SHRINK_VOLUME_REQUEST_TYPES = SHRINK_VOLUME_REQUEST_TYPES(3i32);
impl ::core::marker::Copy for SHRINK_VOLUME_REQUEST_TYPES {}
impl ::core::clone::Clone for SHRINK_VOLUME_REQUEST_TYPES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SHRINK_VOLUME_REQUEST_TYPES {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for SHRINK_VOLUME_REQUEST_TYPES {
    type Abi = Self;
}
impl ::core::fmt::Debug for SHRINK_VOLUME_REQUEST_TYPES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SHRINK_VOLUME_REQUEST_TYPES").field(&self.0).finish()
    }
}
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
impl ::core::fmt::Debug for SI_COPYFILE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SI_COPYFILE").field("SourceFileNameLength", &self.SourceFileNameLength).field("DestinationFileNameLength", &self.DestinationFileNameLength).field("Flags", &self.Flags).field("FileNameBuffer", &self.FileNameBuffer).finish()
    }
}
unsafe impl ::windows::core::Abi for SI_COPYFILE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SI_COPYFILE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SI_COPYFILE>()) == 0 }
    }
}
impl ::core::cmp::Eq for SI_COPYFILE {}
impl ::core::default::Default for SI_COPYFILE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for SMB_SHARE_FLUSH_AND_PURGE_INPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SMB_SHARE_FLUSH_AND_PURGE_INPUT").field("Version", &self.Version).finish()
    }
}
unsafe impl ::windows::core::Abi for SMB_SHARE_FLUSH_AND_PURGE_INPUT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SMB_SHARE_FLUSH_AND_PURGE_INPUT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SMB_SHARE_FLUSH_AND_PURGE_INPUT>()) == 0 }
    }
}
impl ::core::cmp::Eq for SMB_SHARE_FLUSH_AND_PURGE_INPUT {}
impl ::core::default::Default for SMB_SHARE_FLUSH_AND_PURGE_INPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for SMB_SHARE_FLUSH_AND_PURGE_OUTPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SMB_SHARE_FLUSH_AND_PURGE_OUTPUT").field("cEntriesPurged", &self.cEntriesPurged).finish()
    }
}
unsafe impl ::windows::core::Abi for SMB_SHARE_FLUSH_AND_PURGE_OUTPUT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SMB_SHARE_FLUSH_AND_PURGE_OUTPUT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SMB_SHARE_FLUSH_AND_PURGE_OUTPUT>()) == 0 }
    }
}
impl ::core::cmp::Eq for SMB_SHARE_FLUSH_AND_PURGE_OUTPUT {}
impl ::core::default::Default for SMB_SHARE_FLUSH_AND_PURGE_OUTPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for STARTING_LCN_INPUT_BUFFER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STARTING_LCN_INPUT_BUFFER").field("StartingLcn", &self.StartingLcn).finish()
    }
}
unsafe impl ::windows::core::Abi for STARTING_LCN_INPUT_BUFFER {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for STARTING_LCN_INPUT_BUFFER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<STARTING_LCN_INPUT_BUFFER>()) == 0 }
    }
}
impl ::core::cmp::Eq for STARTING_LCN_INPUT_BUFFER {}
impl ::core::default::Default for STARTING_LCN_INPUT_BUFFER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for STARTING_LCN_INPUT_BUFFER_EX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STARTING_LCN_INPUT_BUFFER_EX").field("StartingLcn", &self.StartingLcn).field("Flags", &self.Flags).finish()
    }
}
unsafe impl ::windows::core::Abi for STARTING_LCN_INPUT_BUFFER_EX {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for STARTING_LCN_INPUT_BUFFER_EX {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<STARTING_LCN_INPUT_BUFFER_EX>()) == 0 }
    }
}
impl ::core::cmp::Eq for STARTING_LCN_INPUT_BUFFER_EX {}
impl ::core::default::Default for STARTING_LCN_INPUT_BUFFER_EX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for STARTING_VCN_INPUT_BUFFER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STARTING_VCN_INPUT_BUFFER").field("StartingVcn", &self.StartingVcn).finish()
    }
}
unsafe impl ::windows::core::Abi for STARTING_VCN_INPUT_BUFFER {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for STARTING_VCN_INPUT_BUFFER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<STARTING_VCN_INPUT_BUFFER>()) == 0 }
    }
}
impl ::core::cmp::Eq for STARTING_VCN_INPUT_BUFFER {}
impl ::core::default::Default for STARTING_VCN_INPUT_BUFFER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for STORAGE_ACCESS_ALIGNMENT_DESCRIPTOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STORAGE_ACCESS_ALIGNMENT_DESCRIPTOR").field("Version", &self.Version).field("Size", &self.Size).field("BytesPerCacheLine", &self.BytesPerCacheLine).field("BytesOffsetForCacheAlignment", &self.BytesOffsetForCacheAlignment).field("BytesPerLogicalSector", &self.BytesPerLogicalSector).field("BytesPerPhysicalSector", &self.BytesPerPhysicalSector).field("BytesOffsetForSectorAlignment", &self.BytesOffsetForSectorAlignment).finish()
    }
}
unsafe impl ::windows::core::Abi for STORAGE_ACCESS_ALIGNMENT_DESCRIPTOR {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for STORAGE_ACCESS_ALIGNMENT_DESCRIPTOR {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<STORAGE_ACCESS_ALIGNMENT_DESCRIPTOR>()) == 0 }
    }
}
impl ::core::cmp::Eq for STORAGE_ACCESS_ALIGNMENT_DESCRIPTOR {}
impl ::core::default::Default for STORAGE_ACCESS_ALIGNMENT_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for STORAGE_ADAPTER_DESCRIPTOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STORAGE_ADAPTER_DESCRIPTOR")
            .field("Version", &self.Version)
            .field("Size", &self.Size)
            .field("MaximumTransferLength", &self.MaximumTransferLength)
            .field("MaximumPhysicalPages", &self.MaximumPhysicalPages)
            .field("AlignmentMask", &self.AlignmentMask)
            .field("AdapterUsesPio", &self.AdapterUsesPio)
            .field("AdapterScansDown", &self.AdapterScansDown)
            .field("CommandQueueing", &self.CommandQueueing)
            .field("AcceleratedTransfer", &self.AcceleratedTransfer)
            .field("BusType", &self.BusType)
            .field("BusMajorVersion", &self.BusMajorVersion)
            .field("BusMinorVersion", &self.BusMinorVersion)
            .field("SrbType", &self.SrbType)
            .field("AddressType", &self.AddressType)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for STORAGE_ADAPTER_DESCRIPTOR {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for STORAGE_ADAPTER_DESCRIPTOR {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<STORAGE_ADAPTER_DESCRIPTOR>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for STORAGE_ADAPTER_DESCRIPTOR {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for STORAGE_ADAPTER_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for STORAGE_ADAPTER_SERIAL_NUMBER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STORAGE_ADAPTER_SERIAL_NUMBER").field("Version", &self.Version).field("Size", &self.Size).field("SerialNumber", &self.SerialNumber).finish()
    }
}
unsafe impl ::windows::core::Abi for STORAGE_ADAPTER_SERIAL_NUMBER {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for STORAGE_ADAPTER_SERIAL_NUMBER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<STORAGE_ADAPTER_SERIAL_NUMBER>()) == 0 }
    }
}
impl ::core::cmp::Eq for STORAGE_ADAPTER_SERIAL_NUMBER {}
impl ::core::default::Default for STORAGE_ADAPTER_SERIAL_NUMBER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for STORAGE_ALLOCATE_BC_STREAM_INPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STORAGE_ALLOCATE_BC_STREAM_INPUT").field("Version", &self.Version).field("RequestsPerPeriod", &self.RequestsPerPeriod).field("Period", &self.Period).field("RetryFailures", &self.RetryFailures).field("Discardable", &self.Discardable).field("Reserved1", &self.Reserved1).field("AccessType", &self.AccessType).field("AccessMode", &self.AccessMode).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for STORAGE_ALLOCATE_BC_STREAM_INPUT {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for STORAGE_ALLOCATE_BC_STREAM_INPUT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<STORAGE_ALLOCATE_BC_STREAM_INPUT>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for STORAGE_ALLOCATE_BC_STREAM_INPUT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for STORAGE_ALLOCATE_BC_STREAM_INPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for STORAGE_ALLOCATE_BC_STREAM_OUTPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STORAGE_ALLOCATE_BC_STREAM_OUTPUT").field("RequestSize", &self.RequestSize).field("NumOutStandingRequests", &self.NumOutStandingRequests).finish()
    }
}
unsafe impl ::windows::core::Abi for STORAGE_ALLOCATE_BC_STREAM_OUTPUT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for STORAGE_ALLOCATE_BC_STREAM_OUTPUT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<STORAGE_ALLOCATE_BC_STREAM_OUTPUT>()) == 0 }
    }
}
impl ::core::cmp::Eq for STORAGE_ALLOCATE_BC_STREAM_OUTPUT {}
impl ::core::default::Default for STORAGE_ALLOCATE_BC_STREAM_OUTPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct STORAGE_ASSOCIATION_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageIdAssocDevice: STORAGE_ASSOCIATION_TYPE = STORAGE_ASSOCIATION_TYPE(0i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageIdAssocPort: STORAGE_ASSOCIATION_TYPE = STORAGE_ASSOCIATION_TYPE(1i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageIdAssocTarget: STORAGE_ASSOCIATION_TYPE = STORAGE_ASSOCIATION_TYPE(2i32);
impl ::core::marker::Copy for STORAGE_ASSOCIATION_TYPE {}
impl ::core::clone::Clone for STORAGE_ASSOCIATION_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for STORAGE_ASSOCIATION_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for STORAGE_ASSOCIATION_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for STORAGE_ASSOCIATION_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("STORAGE_ASSOCIATION_TYPE").field(&self.0).finish()
    }
}
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
impl ::core::fmt::Debug for STORAGE_ATTRIBUTE_MGMT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STORAGE_ATTRIBUTE_MGMT").field("Version", &self.Version).field("Size", &self.Size).field("Action", &self.Action).field("Attribute", &self.Attribute).finish()
    }
}
unsafe impl ::windows::core::Abi for STORAGE_ATTRIBUTE_MGMT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for STORAGE_ATTRIBUTE_MGMT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<STORAGE_ATTRIBUTE_MGMT>()) == 0 }
    }
}
impl ::core::cmp::Eq for STORAGE_ATTRIBUTE_MGMT {}
impl ::core::default::Default for STORAGE_ATTRIBUTE_MGMT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct STORAGE_ATTRIBUTE_MGMT_ACTION(pub i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorAttributeMgmt_ClearAttribute: STORAGE_ATTRIBUTE_MGMT_ACTION = STORAGE_ATTRIBUTE_MGMT_ACTION(0i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorAttributeMgmt_SetAttribute: STORAGE_ATTRIBUTE_MGMT_ACTION = STORAGE_ATTRIBUTE_MGMT_ACTION(1i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorAttributeMgmt_ResetAttribute: STORAGE_ATTRIBUTE_MGMT_ACTION = STORAGE_ATTRIBUTE_MGMT_ACTION(2i32);
impl ::core::marker::Copy for STORAGE_ATTRIBUTE_MGMT_ACTION {}
impl ::core::clone::Clone for STORAGE_ATTRIBUTE_MGMT_ACTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for STORAGE_ATTRIBUTE_MGMT_ACTION {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for STORAGE_ATTRIBUTE_MGMT_ACTION {
    type Abi = Self;
}
impl ::core::fmt::Debug for STORAGE_ATTRIBUTE_MGMT_ACTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("STORAGE_ATTRIBUTE_MGMT_ACTION").field(&self.0).finish()
    }
}
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
impl ::core::fmt::Debug for STORAGE_BREAK_RESERVATION_REQUEST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STORAGE_BREAK_RESERVATION_REQUEST").field("Length", &self.Length).field("_unused", &self._unused).field("PathId", &self.PathId).field("TargetId", &self.TargetId).field("Lun", &self.Lun).finish()
    }
}
unsafe impl ::windows::core::Abi for STORAGE_BREAK_RESERVATION_REQUEST {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for STORAGE_BREAK_RESERVATION_REQUEST {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<STORAGE_BREAK_RESERVATION_REQUEST>()) == 0 }
    }
}
impl ::core::cmp::Eq for STORAGE_BREAK_RESERVATION_REQUEST {}
impl ::core::default::Default for STORAGE_BREAK_RESERVATION_REQUEST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for STORAGE_BUS_RESET_REQUEST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STORAGE_BUS_RESET_REQUEST").field("PathId", &self.PathId).finish()
    }
}
unsafe impl ::windows::core::Abi for STORAGE_BUS_RESET_REQUEST {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for STORAGE_BUS_RESET_REQUEST {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<STORAGE_BUS_RESET_REQUEST>()) == 0 }
    }
}
impl ::core::cmp::Eq for STORAGE_BUS_RESET_REQUEST {}
impl ::core::default::Default for STORAGE_BUS_RESET_REQUEST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct STORAGE_COMPONENT_HEALTH_STATUS(pub i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const HealthStatusUnknown: STORAGE_COMPONENT_HEALTH_STATUS = STORAGE_COMPONENT_HEALTH_STATUS(0i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const HealthStatusNormal: STORAGE_COMPONENT_HEALTH_STATUS = STORAGE_COMPONENT_HEALTH_STATUS(1i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const HealthStatusThrottled: STORAGE_COMPONENT_HEALTH_STATUS = STORAGE_COMPONENT_HEALTH_STATUS(2i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const HealthStatusWarning: STORAGE_COMPONENT_HEALTH_STATUS = STORAGE_COMPONENT_HEALTH_STATUS(3i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const HealthStatusDisabled: STORAGE_COMPONENT_HEALTH_STATUS = STORAGE_COMPONENT_HEALTH_STATUS(4i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const HealthStatusFailed: STORAGE_COMPONENT_HEALTH_STATUS = STORAGE_COMPONENT_HEALTH_STATUS(5i32);
impl ::core::marker::Copy for STORAGE_COMPONENT_HEALTH_STATUS {}
impl ::core::clone::Clone for STORAGE_COMPONENT_HEALTH_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for STORAGE_COMPONENT_HEALTH_STATUS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for STORAGE_COMPONENT_HEALTH_STATUS {
    type Abi = Self;
}
impl ::core::fmt::Debug for STORAGE_COMPONENT_HEALTH_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("STORAGE_COMPONENT_HEALTH_STATUS").field(&self.0).finish()
    }
}
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
unsafe impl ::windows::core::Abi for STORAGE_COUNTER {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for STORAGE_COUNTER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<STORAGE_COUNTER>()) == 0 }
    }
}
impl ::core::cmp::Eq for STORAGE_COUNTER {}
impl ::core::default::Default for STORAGE_COUNTER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
unsafe impl ::windows::core::Abi for STORAGE_COUNTER_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for STORAGE_COUNTER_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<STORAGE_COUNTER_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for STORAGE_COUNTER_0 {}
impl ::core::default::Default for STORAGE_COUNTER_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for STORAGE_COUNTER_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STORAGE_COUNTER_0_0").field("Week", &self.Week).field("Year", &self.Year).finish()
    }
}
unsafe impl ::windows::core::Abi for STORAGE_COUNTER_0_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for STORAGE_COUNTER_0_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<STORAGE_COUNTER_0_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for STORAGE_COUNTER_0_0 {}
impl ::core::default::Default for STORAGE_COUNTER_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
unsafe impl ::windows::core::Abi for STORAGE_COUNTERS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for STORAGE_COUNTERS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<STORAGE_COUNTERS>()) == 0 }
    }
}
impl ::core::cmp::Eq for STORAGE_COUNTERS {}
impl ::core::default::Default for STORAGE_COUNTERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct STORAGE_COUNTER_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageCounterTypeUnknown: STORAGE_COUNTER_TYPE = STORAGE_COUNTER_TYPE(0i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageCounterTypeTemperatureCelsius: STORAGE_COUNTER_TYPE = STORAGE_COUNTER_TYPE(1i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageCounterTypeTemperatureCelsiusMax: STORAGE_COUNTER_TYPE = STORAGE_COUNTER_TYPE(2i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageCounterTypeReadErrorsTotal: STORAGE_COUNTER_TYPE = STORAGE_COUNTER_TYPE(3i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageCounterTypeReadErrorsCorrected: STORAGE_COUNTER_TYPE = STORAGE_COUNTER_TYPE(4i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageCounterTypeReadErrorsUncorrected: STORAGE_COUNTER_TYPE = STORAGE_COUNTER_TYPE(5i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageCounterTypeWriteErrorsTotal: STORAGE_COUNTER_TYPE = STORAGE_COUNTER_TYPE(6i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageCounterTypeWriteErrorsCorrected: STORAGE_COUNTER_TYPE = STORAGE_COUNTER_TYPE(7i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageCounterTypeWriteErrorsUncorrected: STORAGE_COUNTER_TYPE = STORAGE_COUNTER_TYPE(8i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageCounterTypeManufactureDate: STORAGE_COUNTER_TYPE = STORAGE_COUNTER_TYPE(9i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageCounterTypeStartStopCycleCount: STORAGE_COUNTER_TYPE = STORAGE_COUNTER_TYPE(10i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageCounterTypeStartStopCycleCountMax: STORAGE_COUNTER_TYPE = STORAGE_COUNTER_TYPE(11i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageCounterTypeLoadUnloadCycleCount: STORAGE_COUNTER_TYPE = STORAGE_COUNTER_TYPE(12i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageCounterTypeLoadUnloadCycleCountMax: STORAGE_COUNTER_TYPE = STORAGE_COUNTER_TYPE(13i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageCounterTypeWearPercentage: STORAGE_COUNTER_TYPE = STORAGE_COUNTER_TYPE(14i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageCounterTypeWearPercentageWarning: STORAGE_COUNTER_TYPE = STORAGE_COUNTER_TYPE(15i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageCounterTypeWearPercentageMax: STORAGE_COUNTER_TYPE = STORAGE_COUNTER_TYPE(16i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageCounterTypePowerOnHours: STORAGE_COUNTER_TYPE = STORAGE_COUNTER_TYPE(17i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageCounterTypeReadLatency100NSMax: STORAGE_COUNTER_TYPE = STORAGE_COUNTER_TYPE(18i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageCounterTypeWriteLatency100NSMax: STORAGE_COUNTER_TYPE = STORAGE_COUNTER_TYPE(19i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageCounterTypeFlushLatency100NSMax: STORAGE_COUNTER_TYPE = STORAGE_COUNTER_TYPE(20i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageCounterTypeMax: STORAGE_COUNTER_TYPE = STORAGE_COUNTER_TYPE(21i32);
impl ::core::marker::Copy for STORAGE_COUNTER_TYPE {}
impl ::core::clone::Clone for STORAGE_COUNTER_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for STORAGE_COUNTER_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for STORAGE_COUNTER_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for STORAGE_COUNTER_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("STORAGE_COUNTER_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const STORAGE_CRASH_TELEMETRY_REGKEY: &str = "\\Registry\\Machine\\System\\CurrentControlSet\\Control\\CrashControl\\StorageTelemetry";
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct STORAGE_CRYPTO_ALGORITHM_ID(pub i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageCryptoAlgorithmUnknown: STORAGE_CRYPTO_ALGORITHM_ID = STORAGE_CRYPTO_ALGORITHM_ID(0i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageCryptoAlgorithmXTSAES: STORAGE_CRYPTO_ALGORITHM_ID = STORAGE_CRYPTO_ALGORITHM_ID(1i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageCryptoAlgorithmBitlockerAESCBC: STORAGE_CRYPTO_ALGORITHM_ID = STORAGE_CRYPTO_ALGORITHM_ID(2i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageCryptoAlgorithmAESECB: STORAGE_CRYPTO_ALGORITHM_ID = STORAGE_CRYPTO_ALGORITHM_ID(3i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageCryptoAlgorithmESSIVAESCBC: STORAGE_CRYPTO_ALGORITHM_ID = STORAGE_CRYPTO_ALGORITHM_ID(4i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageCryptoAlgorithmMax: STORAGE_CRYPTO_ALGORITHM_ID = STORAGE_CRYPTO_ALGORITHM_ID(5i32);
impl ::core::marker::Copy for STORAGE_CRYPTO_ALGORITHM_ID {}
impl ::core::clone::Clone for STORAGE_CRYPTO_ALGORITHM_ID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for STORAGE_CRYPTO_ALGORITHM_ID {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for STORAGE_CRYPTO_ALGORITHM_ID {
    type Abi = Self;
}
impl ::core::fmt::Debug for STORAGE_CRYPTO_ALGORITHM_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("STORAGE_CRYPTO_ALGORITHM_ID").field(&self.0).finish()
    }
}
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
impl ::core::fmt::Debug for STORAGE_CRYPTO_CAPABILITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STORAGE_CRYPTO_CAPABILITY").field("Version", &self.Version).field("Size", &self.Size).field("CryptoCapabilityIndex", &self.CryptoCapabilityIndex).field("AlgorithmId", &self.AlgorithmId).field("KeySize", &self.KeySize).field("DataUnitSizeBitmask", &self.DataUnitSizeBitmask).finish()
    }
}
unsafe impl ::windows::core::Abi for STORAGE_CRYPTO_CAPABILITY {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for STORAGE_CRYPTO_CAPABILITY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<STORAGE_CRYPTO_CAPABILITY>()) == 0 }
    }
}
impl ::core::cmp::Eq for STORAGE_CRYPTO_CAPABILITY {}
impl ::core::default::Default for STORAGE_CRYPTO_CAPABILITY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for STORAGE_CRYPTO_DESCRIPTOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STORAGE_CRYPTO_DESCRIPTOR").field("Version", &self.Version).field("Size", &self.Size).field("NumKeysSupported", &self.NumKeysSupported).field("NumCryptoCapabilities", &self.NumCryptoCapabilities).field("CryptoCapabilities", &self.CryptoCapabilities).finish()
    }
}
unsafe impl ::windows::core::Abi for STORAGE_CRYPTO_DESCRIPTOR {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for STORAGE_CRYPTO_DESCRIPTOR {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<STORAGE_CRYPTO_DESCRIPTOR>()) == 0 }
    }
}
impl ::core::cmp::Eq for STORAGE_CRYPTO_DESCRIPTOR {}
impl ::core::default::Default for STORAGE_CRYPTO_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const STORAGE_CRYPTO_DESCRIPTOR_VERSION_1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct STORAGE_CRYPTO_KEY_SIZE(pub i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageCryptoKeySizeUnknown: STORAGE_CRYPTO_KEY_SIZE = STORAGE_CRYPTO_KEY_SIZE(0i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageCryptoKeySize128Bits: STORAGE_CRYPTO_KEY_SIZE = STORAGE_CRYPTO_KEY_SIZE(1i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageCryptoKeySize192Bits: STORAGE_CRYPTO_KEY_SIZE = STORAGE_CRYPTO_KEY_SIZE(2i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageCryptoKeySize256Bits: STORAGE_CRYPTO_KEY_SIZE = STORAGE_CRYPTO_KEY_SIZE(3i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageCryptoKeySize512Bits: STORAGE_CRYPTO_KEY_SIZE = STORAGE_CRYPTO_KEY_SIZE(4i32);
impl ::core::marker::Copy for STORAGE_CRYPTO_KEY_SIZE {}
impl ::core::clone::Clone for STORAGE_CRYPTO_KEY_SIZE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for STORAGE_CRYPTO_KEY_SIZE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for STORAGE_CRYPTO_KEY_SIZE {
    type Abi = Self;
}
impl ::core::fmt::Debug for STORAGE_CRYPTO_KEY_SIZE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("STORAGE_CRYPTO_KEY_SIZE").field(&self.0).finish()
    }
}
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
impl ::core::fmt::Debug for STORAGE_DESCRIPTOR_HEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STORAGE_DESCRIPTOR_HEADER").field("Version", &self.Version).field("Size", &self.Size).finish()
    }
}
unsafe impl ::windows::core::Abi for STORAGE_DESCRIPTOR_HEADER {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for STORAGE_DESCRIPTOR_HEADER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<STORAGE_DESCRIPTOR_HEADER>()) == 0 }
    }
}
impl ::core::cmp::Eq for STORAGE_DESCRIPTOR_HEADER {}
impl ::core::default::Default for STORAGE_DESCRIPTOR_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for STORAGE_DEVICE_ATTRIBUTES_DESCRIPTOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STORAGE_DEVICE_ATTRIBUTES_DESCRIPTOR").field("Version", &self.Version).field("Size", &self.Size).field("Attributes", &self.Attributes).finish()
    }
}
unsafe impl ::windows::core::Abi for STORAGE_DEVICE_ATTRIBUTES_DESCRIPTOR {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for STORAGE_DEVICE_ATTRIBUTES_DESCRIPTOR {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<STORAGE_DEVICE_ATTRIBUTES_DESCRIPTOR>()) == 0 }
    }
}
impl ::core::cmp::Eq for STORAGE_DEVICE_ATTRIBUTES_DESCRIPTOR {}
impl ::core::default::Default for STORAGE_DEVICE_ATTRIBUTES_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_FileSystem"))]
impl ::core::fmt::Debug for STORAGE_DEVICE_DESCRIPTOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STORAGE_DEVICE_DESCRIPTOR")
            .field("Version", &self.Version)
            .field("Size", &self.Size)
            .field("DeviceType", &self.DeviceType)
            .field("DeviceTypeModifier", &self.DeviceTypeModifier)
            .field("RemovableMedia", &self.RemovableMedia)
            .field("CommandQueueing", &self.CommandQueueing)
            .field("VendorIdOffset", &self.VendorIdOffset)
            .field("ProductIdOffset", &self.ProductIdOffset)
            .field("ProductRevisionOffset", &self.ProductRevisionOffset)
            .field("SerialNumberOffset", &self.SerialNumberOffset)
            .field("BusType", &self.BusType)
            .field("RawPropertiesLength", &self.RawPropertiesLength)
            .field("RawDeviceProperties", &self.RawDeviceProperties)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_FileSystem"))]
unsafe impl ::windows::core::Abi for STORAGE_DEVICE_DESCRIPTOR {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_FileSystem"))]
impl ::core::cmp::PartialEq for STORAGE_DEVICE_DESCRIPTOR {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<STORAGE_DEVICE_DESCRIPTOR>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_FileSystem"))]
impl ::core::cmp::Eq for STORAGE_DEVICE_DESCRIPTOR {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_FileSystem"))]
impl ::core::default::Default for STORAGE_DEVICE_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct STORAGE_DEVICE_FAULT_DOMAIN_DESCRIPTOR {
    pub Version: u32,
    pub Size: u32,
    pub NumberOfFaultDomains: u32,
    pub FaultDomainIds: [::windows::core::GUID; 1],
}
impl ::core::marker::Copy for STORAGE_DEVICE_FAULT_DOMAIN_DESCRIPTOR {}
impl ::core::clone::Clone for STORAGE_DEVICE_FAULT_DOMAIN_DESCRIPTOR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for STORAGE_DEVICE_FAULT_DOMAIN_DESCRIPTOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STORAGE_DEVICE_FAULT_DOMAIN_DESCRIPTOR").field("Version", &self.Version).field("Size", &self.Size).field("NumberOfFaultDomains", &self.NumberOfFaultDomains).field("FaultDomainIds", &self.FaultDomainIds).finish()
    }
}
unsafe impl ::windows::core::Abi for STORAGE_DEVICE_FAULT_DOMAIN_DESCRIPTOR {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for STORAGE_DEVICE_FAULT_DOMAIN_DESCRIPTOR {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<STORAGE_DEVICE_FAULT_DOMAIN_DESCRIPTOR>()) == 0 }
    }
}
impl ::core::cmp::Eq for STORAGE_DEVICE_FAULT_DOMAIN_DESCRIPTOR {}
impl ::core::default::Default for STORAGE_DEVICE_FAULT_DOMAIN_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const STORAGE_DEVICE_FLAGS_PAGE_83_DEVICEGUID: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const STORAGE_DEVICE_FLAGS_RANDOM_DEVICEGUID_REASON_CONFLICT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const STORAGE_DEVICE_FLAGS_RANDOM_DEVICEGUID_REASON_NOHWID: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct STORAGE_DEVICE_FORM_FACTOR(pub i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FormFactorUnknown: STORAGE_DEVICE_FORM_FACTOR = STORAGE_DEVICE_FORM_FACTOR(0i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FormFactor3_5: STORAGE_DEVICE_FORM_FACTOR = STORAGE_DEVICE_FORM_FACTOR(1i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FormFactor2_5: STORAGE_DEVICE_FORM_FACTOR = STORAGE_DEVICE_FORM_FACTOR(2i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FormFactor1_8: STORAGE_DEVICE_FORM_FACTOR = STORAGE_DEVICE_FORM_FACTOR(3i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FormFactor1_8Less: STORAGE_DEVICE_FORM_FACTOR = STORAGE_DEVICE_FORM_FACTOR(4i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FormFactorEmbedded: STORAGE_DEVICE_FORM_FACTOR = STORAGE_DEVICE_FORM_FACTOR(5i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FormFactorMemoryCard: STORAGE_DEVICE_FORM_FACTOR = STORAGE_DEVICE_FORM_FACTOR(6i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FormFactormSata: STORAGE_DEVICE_FORM_FACTOR = STORAGE_DEVICE_FORM_FACTOR(7i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FormFactorM_2: STORAGE_DEVICE_FORM_FACTOR = STORAGE_DEVICE_FORM_FACTOR(8i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FormFactorPCIeBoard: STORAGE_DEVICE_FORM_FACTOR = STORAGE_DEVICE_FORM_FACTOR(9i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const FormFactorDimm: STORAGE_DEVICE_FORM_FACTOR = STORAGE_DEVICE_FORM_FACTOR(10i32);
impl ::core::marker::Copy for STORAGE_DEVICE_FORM_FACTOR {}
impl ::core::clone::Clone for STORAGE_DEVICE_FORM_FACTOR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for STORAGE_DEVICE_FORM_FACTOR {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for STORAGE_DEVICE_FORM_FACTOR {
    type Abi = Self;
}
impl ::core::fmt::Debug for STORAGE_DEVICE_FORM_FACTOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("STORAGE_DEVICE_FORM_FACTOR").field(&self.0).finish()
    }
}
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
impl ::core::fmt::Debug for STORAGE_DEVICE_ID_DESCRIPTOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STORAGE_DEVICE_ID_DESCRIPTOR").field("Version", &self.Version).field("Size", &self.Size).field("NumberOfIdentifiers", &self.NumberOfIdentifiers).field("Identifiers", &self.Identifiers).finish()
    }
}
unsafe impl ::windows::core::Abi for STORAGE_DEVICE_ID_DESCRIPTOR {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for STORAGE_DEVICE_ID_DESCRIPTOR {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<STORAGE_DEVICE_ID_DESCRIPTOR>()) == 0 }
    }
}
impl ::core::cmp::Eq for STORAGE_DEVICE_ID_DESCRIPTOR {}
impl ::core::default::Default for STORAGE_DEVICE_ID_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for STORAGE_DEVICE_IO_CAPABILITY_DESCRIPTOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STORAGE_DEVICE_IO_CAPABILITY_DESCRIPTOR").field("Version", &self.Version).field("Size", &self.Size).field("LunMaxIoCount", &self.LunMaxIoCount).field("AdapterMaxIoCount", &self.AdapterMaxIoCount).finish()
    }
}
unsafe impl ::windows::core::Abi for STORAGE_DEVICE_IO_CAPABILITY_DESCRIPTOR {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for STORAGE_DEVICE_IO_CAPABILITY_DESCRIPTOR {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<STORAGE_DEVICE_IO_CAPABILITY_DESCRIPTOR>()) == 0 }
    }
}
impl ::core::cmp::Eq for STORAGE_DEVICE_IO_CAPABILITY_DESCRIPTOR {}
impl ::core::default::Default for STORAGE_DEVICE_IO_CAPABILITY_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for STORAGE_DEVICE_LED_STATE_DESCRIPTOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STORAGE_DEVICE_LED_STATE_DESCRIPTOR").field("Version", &self.Version).field("Size", &self.Size).field("State", &self.State).finish()
    }
}
unsafe impl ::windows::core::Abi for STORAGE_DEVICE_LED_STATE_DESCRIPTOR {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for STORAGE_DEVICE_LED_STATE_DESCRIPTOR {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<STORAGE_DEVICE_LED_STATE_DESCRIPTOR>()) == 0 }
    }
}
impl ::core::cmp::Eq for STORAGE_DEVICE_LED_STATE_DESCRIPTOR {}
impl ::core::default::Default for STORAGE_DEVICE_LED_STATE_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
unsafe impl ::windows::core::Abi for STORAGE_DEVICE_LOCATION_DESCRIPTOR {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for STORAGE_DEVICE_LOCATION_DESCRIPTOR {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<STORAGE_DEVICE_LOCATION_DESCRIPTOR>()) == 0 }
    }
}
impl ::core::cmp::Eq for STORAGE_DEVICE_LOCATION_DESCRIPTOR {}
impl ::core::default::Default for STORAGE_DEVICE_LOCATION_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
unsafe impl ::windows::core::Abi for STORAGE_DEVICE_MANAGEMENT_STATUS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for STORAGE_DEVICE_MANAGEMENT_STATUS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<STORAGE_DEVICE_MANAGEMENT_STATUS>()) == 0 }
    }
}
impl ::core::cmp::Eq for STORAGE_DEVICE_MANAGEMENT_STATUS {}
impl ::core::default::Default for STORAGE_DEVICE_MANAGEMENT_STATUS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for STORAGE_DEVICE_NUMA_PROPERTY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STORAGE_DEVICE_NUMA_PROPERTY").field("Version", &self.Version).field("Size", &self.Size).field("NumaNode", &self.NumaNode).finish()
    }
}
unsafe impl ::windows::core::Abi for STORAGE_DEVICE_NUMA_PROPERTY {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for STORAGE_DEVICE_NUMA_PROPERTY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<STORAGE_DEVICE_NUMA_PROPERTY>()) == 0 }
    }
}
impl ::core::cmp::Eq for STORAGE_DEVICE_NUMA_PROPERTY {}
impl ::core::default::Default for STORAGE_DEVICE_NUMA_PROPERTY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for STORAGE_DEVICE_NUMBER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STORAGE_DEVICE_NUMBER").field("DeviceType", &self.DeviceType).field("DeviceNumber", &self.DeviceNumber).field("PartitionNumber", &self.PartitionNumber).finish()
    }
}
unsafe impl ::windows::core::Abi for STORAGE_DEVICE_NUMBER {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for STORAGE_DEVICE_NUMBER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<STORAGE_DEVICE_NUMBER>()) == 0 }
    }
}
impl ::core::cmp::Eq for STORAGE_DEVICE_NUMBER {}
impl ::core::default::Default for STORAGE_DEVICE_NUMBER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for STORAGE_DEVICE_NUMBERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STORAGE_DEVICE_NUMBERS").field("Version", &self.Version).field("Size", &self.Size).field("NumberOfDevices", &self.NumberOfDevices).field("Devices", &self.Devices).finish()
    }
}
unsafe impl ::windows::core::Abi for STORAGE_DEVICE_NUMBERS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for STORAGE_DEVICE_NUMBERS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<STORAGE_DEVICE_NUMBERS>()) == 0 }
    }
}
impl ::core::cmp::Eq for STORAGE_DEVICE_NUMBERS {}
impl ::core::default::Default for STORAGE_DEVICE_NUMBERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
    pub DeviceGuid: ::windows::core::GUID,
    pub PartitionNumber: u32,
}
impl ::core::marker::Copy for STORAGE_DEVICE_NUMBER_EX {}
impl ::core::clone::Clone for STORAGE_DEVICE_NUMBER_EX {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for STORAGE_DEVICE_NUMBER_EX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STORAGE_DEVICE_NUMBER_EX").field("Version", &self.Version).field("Size", &self.Size).field("Flags", &self.Flags).field("DeviceType", &self.DeviceType).field("DeviceNumber", &self.DeviceNumber).field("DeviceGuid", &self.DeviceGuid).field("PartitionNumber", &self.PartitionNumber).finish()
    }
}
unsafe impl ::windows::core::Abi for STORAGE_DEVICE_NUMBER_EX {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for STORAGE_DEVICE_NUMBER_EX {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<STORAGE_DEVICE_NUMBER_EX>()) == 0 }
    }
}
impl ::core::cmp::Eq for STORAGE_DEVICE_NUMBER_EX {}
impl ::core::default::Default for STORAGE_DEVICE_NUMBER_EX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for STORAGE_DEVICE_POWER_CAP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STORAGE_DEVICE_POWER_CAP").field("Version", &self.Version).field("Size", &self.Size).field("Units", &self.Units).field("MaxPower", &self.MaxPower).finish()
    }
}
unsafe impl ::windows::core::Abi for STORAGE_DEVICE_POWER_CAP {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for STORAGE_DEVICE_POWER_CAP {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<STORAGE_DEVICE_POWER_CAP>()) == 0 }
    }
}
impl ::core::cmp::Eq for STORAGE_DEVICE_POWER_CAP {}
impl ::core::default::Default for STORAGE_DEVICE_POWER_CAP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct STORAGE_DEVICE_POWER_CAP_UNITS(pub i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageDevicePowerCapUnitsPercent: STORAGE_DEVICE_POWER_CAP_UNITS = STORAGE_DEVICE_POWER_CAP_UNITS(0i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageDevicePowerCapUnitsMilliwatts: STORAGE_DEVICE_POWER_CAP_UNITS = STORAGE_DEVICE_POWER_CAP_UNITS(1i32);
impl ::core::marker::Copy for STORAGE_DEVICE_POWER_CAP_UNITS {}
impl ::core::clone::Clone for STORAGE_DEVICE_POWER_CAP_UNITS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for STORAGE_DEVICE_POWER_CAP_UNITS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for STORAGE_DEVICE_POWER_CAP_UNITS {
    type Abi = Self;
}
impl ::core::fmt::Debug for STORAGE_DEVICE_POWER_CAP_UNITS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("STORAGE_DEVICE_POWER_CAP_UNITS").field(&self.0).finish()
    }
}
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
impl ::core::fmt::Debug for STORAGE_DEVICE_RESILIENCY_DESCRIPTOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STORAGE_DEVICE_RESILIENCY_DESCRIPTOR").field("Version", &self.Version).field("Size", &self.Size).field("NameOffset", &self.NameOffset).field("NumberOfLogicalCopies", &self.NumberOfLogicalCopies).field("NumberOfPhysicalCopies", &self.NumberOfPhysicalCopies).field("PhysicalDiskRedundancy", &self.PhysicalDiskRedundancy).field("NumberOfColumns", &self.NumberOfColumns).field("Interleave", &self.Interleave).finish()
    }
}
unsafe impl ::windows::core::Abi for STORAGE_DEVICE_RESILIENCY_DESCRIPTOR {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for STORAGE_DEVICE_RESILIENCY_DESCRIPTOR {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<STORAGE_DEVICE_RESILIENCY_DESCRIPTOR>()) == 0 }
    }
}
impl ::core::cmp::Eq for STORAGE_DEVICE_RESILIENCY_DESCRIPTOR {}
impl ::core::default::Default for STORAGE_DEVICE_RESILIENCY_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for STORAGE_DEVICE_SELF_ENCRYPTION_PROPERTY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STORAGE_DEVICE_SELF_ENCRYPTION_PROPERTY").field("Version", &self.Version).field("Size", &self.Size).field("SupportsSelfEncryption", &self.SupportsSelfEncryption).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for STORAGE_DEVICE_SELF_ENCRYPTION_PROPERTY {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for STORAGE_DEVICE_SELF_ENCRYPTION_PROPERTY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<STORAGE_DEVICE_SELF_ENCRYPTION_PROPERTY>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for STORAGE_DEVICE_SELF_ENCRYPTION_PROPERTY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for STORAGE_DEVICE_SELF_ENCRYPTION_PROPERTY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for STORAGE_DEVICE_TIERING_DESCRIPTOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STORAGE_DEVICE_TIERING_DESCRIPTOR").field("Version", &self.Version).field("Size", &self.Size).field("Flags", &self.Flags).field("TotalNumberOfTiers", &self.TotalNumberOfTiers).field("NumberOfTiersReturned", &self.NumberOfTiersReturned).field("Tiers", &self.Tiers).finish()
    }
}
unsafe impl ::windows::core::Abi for STORAGE_DEVICE_TIERING_DESCRIPTOR {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for STORAGE_DEVICE_TIERING_DESCRIPTOR {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<STORAGE_DEVICE_TIERING_DESCRIPTOR>()) == 0 }
    }
}
impl ::core::cmp::Eq for STORAGE_DEVICE_TIERING_DESCRIPTOR {}
impl ::core::default::Default for STORAGE_DEVICE_TIERING_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for STORAGE_DEVICE_UNSAFE_SHUTDOWN_COUNT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STORAGE_DEVICE_UNSAFE_SHUTDOWN_COUNT").field("Version", &self.Version).field("Size", &self.Size).field("UnsafeShutdownCount", &self.UnsafeShutdownCount).finish()
    }
}
unsafe impl ::windows::core::Abi for STORAGE_DEVICE_UNSAFE_SHUTDOWN_COUNT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for STORAGE_DEVICE_UNSAFE_SHUTDOWN_COUNT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<STORAGE_DEVICE_UNSAFE_SHUTDOWN_COUNT>()) == 0 }
    }
}
impl ::core::cmp::Eq for STORAGE_DEVICE_UNSAFE_SHUTDOWN_COUNT {}
impl ::core::default::Default for STORAGE_DEVICE_UNSAFE_SHUTDOWN_COUNT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct STORAGE_DIAGNOSTIC_DATA {
    pub Version: u32,
    pub Size: u32,
    pub ProviderId: ::windows::core::GUID,
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
impl ::core::fmt::Debug for STORAGE_DIAGNOSTIC_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STORAGE_DIAGNOSTIC_DATA").field("Version", &self.Version).field("Size", &self.Size).field("ProviderId", &self.ProviderId).field("BufferSize", &self.BufferSize).field("Reserved", &self.Reserved).field("DiagnosticDataBuffer", &self.DiagnosticDataBuffer).finish()
    }
}
unsafe impl ::windows::core::Abi for STORAGE_DIAGNOSTIC_DATA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for STORAGE_DIAGNOSTIC_DATA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<STORAGE_DIAGNOSTIC_DATA>()) == 0 }
    }
}
impl ::core::cmp::Eq for STORAGE_DIAGNOSTIC_DATA {}
impl ::core::default::Default for STORAGE_DIAGNOSTIC_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const STORAGE_DIAGNOSTIC_FLAG_ADAPTER_REQUEST: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct STORAGE_DIAGNOSTIC_LEVEL(pub i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageDiagnosticLevelDefault: STORAGE_DIAGNOSTIC_LEVEL = STORAGE_DIAGNOSTIC_LEVEL(0i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageDiagnosticLevelMax: STORAGE_DIAGNOSTIC_LEVEL = STORAGE_DIAGNOSTIC_LEVEL(1i32);
impl ::core::marker::Copy for STORAGE_DIAGNOSTIC_LEVEL {}
impl ::core::clone::Clone for STORAGE_DIAGNOSTIC_LEVEL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for STORAGE_DIAGNOSTIC_LEVEL {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for STORAGE_DIAGNOSTIC_LEVEL {
    type Abi = Self;
}
impl ::core::fmt::Debug for STORAGE_DIAGNOSTIC_LEVEL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("STORAGE_DIAGNOSTIC_LEVEL").field(&self.0).finish()
    }
}
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
impl ::core::fmt::Debug for STORAGE_DIAGNOSTIC_REQUEST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STORAGE_DIAGNOSTIC_REQUEST").field("Version", &self.Version).field("Size", &self.Size).field("Flags", &self.Flags).field("TargetType", &self.TargetType).field("Level", &self.Level).finish()
    }
}
unsafe impl ::windows::core::Abi for STORAGE_DIAGNOSTIC_REQUEST {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for STORAGE_DIAGNOSTIC_REQUEST {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<STORAGE_DIAGNOSTIC_REQUEST>()) == 0 }
    }
}
impl ::core::cmp::Eq for STORAGE_DIAGNOSTIC_REQUEST {}
impl ::core::default::Default for STORAGE_DIAGNOSTIC_REQUEST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct STORAGE_DIAGNOSTIC_TARGET_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageDiagnosticTargetTypeUndefined: STORAGE_DIAGNOSTIC_TARGET_TYPE = STORAGE_DIAGNOSTIC_TARGET_TYPE(0i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageDiagnosticTargetTypePort: STORAGE_DIAGNOSTIC_TARGET_TYPE = STORAGE_DIAGNOSTIC_TARGET_TYPE(1i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageDiagnosticTargetTypeMiniport: STORAGE_DIAGNOSTIC_TARGET_TYPE = STORAGE_DIAGNOSTIC_TARGET_TYPE(2i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageDiagnosticTargetTypeHbaFirmware: STORAGE_DIAGNOSTIC_TARGET_TYPE = STORAGE_DIAGNOSTIC_TARGET_TYPE(3i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageDiagnosticTargetTypeMax: STORAGE_DIAGNOSTIC_TARGET_TYPE = STORAGE_DIAGNOSTIC_TARGET_TYPE(4i32);
impl ::core::marker::Copy for STORAGE_DIAGNOSTIC_TARGET_TYPE {}
impl ::core::clone::Clone for STORAGE_DIAGNOSTIC_TARGET_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for STORAGE_DIAGNOSTIC_TARGET_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for STORAGE_DIAGNOSTIC_TARGET_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for STORAGE_DIAGNOSTIC_TARGET_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("STORAGE_DIAGNOSTIC_TARGET_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct STORAGE_DISK_HEALTH_STATUS(pub i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const DiskHealthUnknown: STORAGE_DISK_HEALTH_STATUS = STORAGE_DISK_HEALTH_STATUS(0i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const DiskHealthUnhealthy: STORAGE_DISK_HEALTH_STATUS = STORAGE_DISK_HEALTH_STATUS(1i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const DiskHealthWarning: STORAGE_DISK_HEALTH_STATUS = STORAGE_DISK_HEALTH_STATUS(2i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const DiskHealthHealthy: STORAGE_DISK_HEALTH_STATUS = STORAGE_DISK_HEALTH_STATUS(3i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const DiskHealthMax: STORAGE_DISK_HEALTH_STATUS = STORAGE_DISK_HEALTH_STATUS(4i32);
impl ::core::marker::Copy for STORAGE_DISK_HEALTH_STATUS {}
impl ::core::clone::Clone for STORAGE_DISK_HEALTH_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for STORAGE_DISK_HEALTH_STATUS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for STORAGE_DISK_HEALTH_STATUS {
    type Abi = Self;
}
impl ::core::fmt::Debug for STORAGE_DISK_HEALTH_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("STORAGE_DISK_HEALTH_STATUS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct STORAGE_DISK_OPERATIONAL_STATUS(pub i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const DiskOpStatusNone: STORAGE_DISK_OPERATIONAL_STATUS = STORAGE_DISK_OPERATIONAL_STATUS(0i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const DiskOpStatusUnknown: STORAGE_DISK_OPERATIONAL_STATUS = STORAGE_DISK_OPERATIONAL_STATUS(1i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const DiskOpStatusOk: STORAGE_DISK_OPERATIONAL_STATUS = STORAGE_DISK_OPERATIONAL_STATUS(2i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const DiskOpStatusPredictingFailure: STORAGE_DISK_OPERATIONAL_STATUS = STORAGE_DISK_OPERATIONAL_STATUS(3i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const DiskOpStatusInService: STORAGE_DISK_OPERATIONAL_STATUS = STORAGE_DISK_OPERATIONAL_STATUS(4i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const DiskOpStatusHardwareError: STORAGE_DISK_OPERATIONAL_STATUS = STORAGE_DISK_OPERATIONAL_STATUS(5i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const DiskOpStatusNotUsable: STORAGE_DISK_OPERATIONAL_STATUS = STORAGE_DISK_OPERATIONAL_STATUS(6i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const DiskOpStatusTransientError: STORAGE_DISK_OPERATIONAL_STATUS = STORAGE_DISK_OPERATIONAL_STATUS(7i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const DiskOpStatusMissing: STORAGE_DISK_OPERATIONAL_STATUS = STORAGE_DISK_OPERATIONAL_STATUS(8i32);
impl ::core::marker::Copy for STORAGE_DISK_OPERATIONAL_STATUS {}
impl ::core::clone::Clone for STORAGE_DISK_OPERATIONAL_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for STORAGE_DISK_OPERATIONAL_STATUS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for STORAGE_DISK_OPERATIONAL_STATUS {
    type Abi = Self;
}
impl ::core::fmt::Debug for STORAGE_DISK_OPERATIONAL_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("STORAGE_DISK_OPERATIONAL_STATUS").field(&self.0).finish()
    }
}
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
impl ::core::fmt::Debug for STORAGE_EVENT_NOTIFICATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STORAGE_EVENT_NOTIFICATION").field("Version", &self.Version).field("Size", &self.Size).field("Events", &self.Events).finish()
    }
}
unsafe impl ::windows::core::Abi for STORAGE_EVENT_NOTIFICATION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for STORAGE_EVENT_NOTIFICATION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<STORAGE_EVENT_NOTIFICATION>()) == 0 }
    }
}
impl ::core::cmp::Eq for STORAGE_EVENT_NOTIFICATION {}
impl ::core::default::Default for STORAGE_EVENT_NOTIFICATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for STORAGE_FAILURE_PREDICTION_CONFIG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STORAGE_FAILURE_PREDICTION_CONFIG").field("Version", &self.Version).field("Size", &self.Size).field("Set", &self.Set).field("Enabled", &self.Enabled).field("Reserved", &self.Reserved).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for STORAGE_FAILURE_PREDICTION_CONFIG {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for STORAGE_FAILURE_PREDICTION_CONFIG {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<STORAGE_FAILURE_PREDICTION_CONFIG>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for STORAGE_FAILURE_PREDICTION_CONFIG {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for STORAGE_FAILURE_PREDICTION_CONFIG {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for STORAGE_FRU_ID_DESCRIPTOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STORAGE_FRU_ID_DESCRIPTOR").field("Version", &self.Version).field("Size", &self.Size).field("IdentifierSize", &self.IdentifierSize).field("Identifier", &self.Identifier).finish()
    }
}
unsafe impl ::windows::core::Abi for STORAGE_FRU_ID_DESCRIPTOR {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for STORAGE_FRU_ID_DESCRIPTOR {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<STORAGE_FRU_ID_DESCRIPTOR>()) == 0 }
    }
}
impl ::core::cmp::Eq for STORAGE_FRU_ID_DESCRIPTOR {}
impl ::core::default::Default for STORAGE_FRU_ID_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for STORAGE_GET_BC_PROPERTIES_OUTPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STORAGE_GET_BC_PROPERTIES_OUTPUT").field("MaximumRequestsPerPeriod", &self.MaximumRequestsPerPeriod).field("MinimumPeriod", &self.MinimumPeriod).field("MaximumRequestSize", &self.MaximumRequestSize).field("EstimatedTimePerRequest", &self.EstimatedTimePerRequest).field("NumOutStandingRequests", &self.NumOutStandingRequests).field("RequestSize", &self.RequestSize).finish()
    }
}
unsafe impl ::windows::core::Abi for STORAGE_GET_BC_PROPERTIES_OUTPUT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for STORAGE_GET_BC_PROPERTIES_OUTPUT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<STORAGE_GET_BC_PROPERTIES_OUTPUT>()) == 0 }
    }
}
impl ::core::cmp::Eq for STORAGE_GET_BC_PROPERTIES_OUTPUT {}
impl ::core::default::Default for STORAGE_GET_BC_PROPERTIES_OUTPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for STORAGE_HOTPLUG_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STORAGE_HOTPLUG_INFO").field("Size", &self.Size).field("MediaRemovable", &self.MediaRemovable).field("MediaHotplug", &self.MediaHotplug).field("DeviceHotplug", &self.DeviceHotplug).field("WriteCacheEnableOverride", &self.WriteCacheEnableOverride).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for STORAGE_HOTPLUG_INFO {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for STORAGE_HOTPLUG_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<STORAGE_HOTPLUG_INFO>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for STORAGE_HOTPLUG_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for STORAGE_HOTPLUG_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for STORAGE_HW_ENDURANCE_DATA_DESCRIPTOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STORAGE_HW_ENDURANCE_DATA_DESCRIPTOR").field("Version", &self.Version).field("Size", &self.Size).field("EnduranceInfo", &self.EnduranceInfo).finish()
    }
}
unsafe impl ::windows::core::Abi for STORAGE_HW_ENDURANCE_DATA_DESCRIPTOR {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for STORAGE_HW_ENDURANCE_DATA_DESCRIPTOR {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<STORAGE_HW_ENDURANCE_DATA_DESCRIPTOR>()) == 0 }
    }
}
impl ::core::cmp::Eq for STORAGE_HW_ENDURANCE_DATA_DESCRIPTOR {}
impl ::core::default::Default for STORAGE_HW_ENDURANCE_DATA_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for STORAGE_HW_ENDURANCE_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STORAGE_HW_ENDURANCE_INFO").field("ValidFields", &self.ValidFields).field("GroupId", &self.GroupId).field("Flags", &self.Flags).field("LifePercentage", &self.LifePercentage).field("BytesReadCount", &self.BytesReadCount).field("ByteWriteCount", &self.ByteWriteCount).finish()
    }
}
unsafe impl ::windows::core::Abi for STORAGE_HW_ENDURANCE_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for STORAGE_HW_ENDURANCE_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<STORAGE_HW_ENDURANCE_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for STORAGE_HW_ENDURANCE_INFO {}
impl ::core::default::Default for STORAGE_HW_ENDURANCE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for STORAGE_HW_ENDURANCE_INFO_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STORAGE_HW_ENDURANCE_INFO_0").field("_bitfield", &self._bitfield).finish()
    }
}
unsafe impl ::windows::core::Abi for STORAGE_HW_ENDURANCE_INFO_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for STORAGE_HW_ENDURANCE_INFO_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<STORAGE_HW_ENDURANCE_INFO_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for STORAGE_HW_ENDURANCE_INFO_0 {}
impl ::core::default::Default for STORAGE_HW_ENDURANCE_INFO_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for STORAGE_HW_FIRMWARE_ACTIVATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STORAGE_HW_FIRMWARE_ACTIVATE").field("Version", &self.Version).field("Size", &self.Size).field("Flags", &self.Flags).field("Slot", &self.Slot).field("Reserved0", &self.Reserved0).finish()
    }
}
unsafe impl ::windows::core::Abi for STORAGE_HW_FIRMWARE_ACTIVATE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for STORAGE_HW_FIRMWARE_ACTIVATE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<STORAGE_HW_FIRMWARE_ACTIVATE>()) == 0 }
    }
}
impl ::core::cmp::Eq for STORAGE_HW_FIRMWARE_ACTIVATE {}
impl ::core::default::Default for STORAGE_HW_FIRMWARE_ACTIVATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for STORAGE_HW_FIRMWARE_DOWNLOAD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STORAGE_HW_FIRMWARE_DOWNLOAD").field("Version", &self.Version).field("Size", &self.Size).field("Flags", &self.Flags).field("Slot", &self.Slot).field("Reserved", &self.Reserved).field("Offset", &self.Offset).field("BufferSize", &self.BufferSize).field("ImageBuffer", &self.ImageBuffer).finish()
    }
}
unsafe impl ::windows::core::Abi for STORAGE_HW_FIRMWARE_DOWNLOAD {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for STORAGE_HW_FIRMWARE_DOWNLOAD {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<STORAGE_HW_FIRMWARE_DOWNLOAD>()) == 0 }
    }
}
impl ::core::cmp::Eq for STORAGE_HW_FIRMWARE_DOWNLOAD {}
impl ::core::default::Default for STORAGE_HW_FIRMWARE_DOWNLOAD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for STORAGE_HW_FIRMWARE_DOWNLOAD_V2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STORAGE_HW_FIRMWARE_DOWNLOAD_V2").field("Version", &self.Version).field("Size", &self.Size).field("Flags", &self.Flags).field("Slot", &self.Slot).field("Reserved", &self.Reserved).field("Offset", &self.Offset).field("BufferSize", &self.BufferSize).field("ImageSize", &self.ImageSize).field("Reserved2", &self.Reserved2).field("ImageBuffer", &self.ImageBuffer).finish()
    }
}
unsafe impl ::windows::core::Abi for STORAGE_HW_FIRMWARE_DOWNLOAD_V2 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for STORAGE_HW_FIRMWARE_DOWNLOAD_V2 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<STORAGE_HW_FIRMWARE_DOWNLOAD_V2>()) == 0 }
    }
}
impl ::core::cmp::Eq for STORAGE_HW_FIRMWARE_DOWNLOAD_V2 {}
impl ::core::default::Default for STORAGE_HW_FIRMWARE_DOWNLOAD_V2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for STORAGE_HW_FIRMWARE_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STORAGE_HW_FIRMWARE_INFO")
            .field("Version", &self.Version)
            .field("Size", &self.Size)
            .field("_bitfield", &self._bitfield)
            .field("SlotCount", &self.SlotCount)
            .field("ActiveSlot", &self.ActiveSlot)
            .field("PendingActivateSlot", &self.PendingActivateSlot)
            .field("FirmwareShared", &self.FirmwareShared)
            .field("Reserved", &self.Reserved)
            .field("ImagePayloadAlignment", &self.ImagePayloadAlignment)
            .field("ImagePayloadMaxSize", &self.ImagePayloadMaxSize)
            .field("Slot", &self.Slot)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for STORAGE_HW_FIRMWARE_INFO {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for STORAGE_HW_FIRMWARE_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<STORAGE_HW_FIRMWARE_INFO>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for STORAGE_HW_FIRMWARE_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for STORAGE_HW_FIRMWARE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for STORAGE_HW_FIRMWARE_INFO_QUERY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STORAGE_HW_FIRMWARE_INFO_QUERY").field("Version", &self.Version).field("Size", &self.Size).field("Flags", &self.Flags).field("Reserved", &self.Reserved).finish()
    }
}
unsafe impl ::windows::core::Abi for STORAGE_HW_FIRMWARE_INFO_QUERY {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for STORAGE_HW_FIRMWARE_INFO_QUERY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<STORAGE_HW_FIRMWARE_INFO_QUERY>()) == 0 }
    }
}
impl ::core::cmp::Eq for STORAGE_HW_FIRMWARE_INFO_QUERY {}
impl ::core::default::Default for STORAGE_HW_FIRMWARE_INFO_QUERY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for STORAGE_HW_FIRMWARE_SLOT_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STORAGE_HW_FIRMWARE_SLOT_INFO").field("Version", &self.Version).field("Size", &self.Size).field("SlotNumber", &self.SlotNumber).field("_bitfield", &self._bitfield).field("Reserved1", &self.Reserved1).field("Revision", &self.Revision).finish()
    }
}
unsafe impl ::windows::core::Abi for STORAGE_HW_FIRMWARE_SLOT_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for STORAGE_HW_FIRMWARE_SLOT_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<STORAGE_HW_FIRMWARE_SLOT_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for STORAGE_HW_FIRMWARE_SLOT_INFO {}
impl ::core::default::Default for STORAGE_HW_FIRMWARE_SLOT_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for STORAGE_IDENTIFIER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STORAGE_IDENTIFIER").field("CodeSet", &self.CodeSet).field("Type", &self.Type).field("IdentifierSize", &self.IdentifierSize).field("NextOffset", &self.NextOffset).field("Association", &self.Association).field("Identifier", &self.Identifier).finish()
    }
}
unsafe impl ::windows::core::Abi for STORAGE_IDENTIFIER {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for STORAGE_IDENTIFIER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<STORAGE_IDENTIFIER>()) == 0 }
    }
}
impl ::core::cmp::Eq for STORAGE_IDENTIFIER {}
impl ::core::default::Default for STORAGE_IDENTIFIER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct STORAGE_IDENTIFIER_CODE_SET(pub i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageIdCodeSetReserved: STORAGE_IDENTIFIER_CODE_SET = STORAGE_IDENTIFIER_CODE_SET(0i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageIdCodeSetBinary: STORAGE_IDENTIFIER_CODE_SET = STORAGE_IDENTIFIER_CODE_SET(1i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageIdCodeSetAscii: STORAGE_IDENTIFIER_CODE_SET = STORAGE_IDENTIFIER_CODE_SET(2i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageIdCodeSetUtf8: STORAGE_IDENTIFIER_CODE_SET = STORAGE_IDENTIFIER_CODE_SET(3i32);
impl ::core::marker::Copy for STORAGE_IDENTIFIER_CODE_SET {}
impl ::core::clone::Clone for STORAGE_IDENTIFIER_CODE_SET {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for STORAGE_IDENTIFIER_CODE_SET {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for STORAGE_IDENTIFIER_CODE_SET {
    type Abi = Self;
}
impl ::core::fmt::Debug for STORAGE_IDENTIFIER_CODE_SET {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("STORAGE_IDENTIFIER_CODE_SET").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct STORAGE_IDENTIFIER_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageIdTypeVendorSpecific: STORAGE_IDENTIFIER_TYPE = STORAGE_IDENTIFIER_TYPE(0i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageIdTypeVendorId: STORAGE_IDENTIFIER_TYPE = STORAGE_IDENTIFIER_TYPE(1i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageIdTypeEUI64: STORAGE_IDENTIFIER_TYPE = STORAGE_IDENTIFIER_TYPE(2i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageIdTypeFCPHName: STORAGE_IDENTIFIER_TYPE = STORAGE_IDENTIFIER_TYPE(3i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageIdTypePortRelative: STORAGE_IDENTIFIER_TYPE = STORAGE_IDENTIFIER_TYPE(4i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageIdTypeTargetPortGroup: STORAGE_IDENTIFIER_TYPE = STORAGE_IDENTIFIER_TYPE(5i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageIdTypeLogicalUnitGroup: STORAGE_IDENTIFIER_TYPE = STORAGE_IDENTIFIER_TYPE(6i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageIdTypeMD5LogicalUnitIdentifier: STORAGE_IDENTIFIER_TYPE = STORAGE_IDENTIFIER_TYPE(7i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageIdTypeScsiNameString: STORAGE_IDENTIFIER_TYPE = STORAGE_IDENTIFIER_TYPE(8i32);
impl ::core::marker::Copy for STORAGE_IDENTIFIER_TYPE {}
impl ::core::clone::Clone for STORAGE_IDENTIFIER_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for STORAGE_IDENTIFIER_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for STORAGE_IDENTIFIER_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for STORAGE_IDENTIFIER_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("STORAGE_IDENTIFIER_TYPE").field(&self.0).finish()
    }
}
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
impl ::core::fmt::Debug for STORAGE_IDLE_POWER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STORAGE_IDLE_POWER").field("Version", &self.Version).field("Size", &self.Size).field("_bitfield", &self._bitfield).field("D3IdleTimeout", &self.D3IdleTimeout).finish()
    }
}
unsafe impl ::windows::core::Abi for STORAGE_IDLE_POWER {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for STORAGE_IDLE_POWER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<STORAGE_IDLE_POWER>()) == 0 }
    }
}
impl ::core::cmp::Eq for STORAGE_IDLE_POWER {}
impl ::core::default::Default for STORAGE_IDLE_POWER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for STORAGE_IDLE_POWERUP_REASON {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STORAGE_IDLE_POWERUP_REASON").field("Version", &self.Version).field("Size", &self.Size).field("PowerupReason", &self.PowerupReason).finish()
    }
}
unsafe impl ::windows::core::Abi for STORAGE_IDLE_POWERUP_REASON {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for STORAGE_IDLE_POWERUP_REASON {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<STORAGE_IDLE_POWERUP_REASON>()) == 0 }
    }
}
impl ::core::cmp::Eq for STORAGE_IDLE_POWERUP_REASON {}
impl ::core::default::Default for STORAGE_IDLE_POWERUP_REASON {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const STORAGE_IDLE_POWERUP_REASON_VERSION_V1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct STORAGE_ID_NAA_FORMAT(pub i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageIdNAAFormatIEEEExtended: STORAGE_ID_NAA_FORMAT = STORAGE_ID_NAA_FORMAT(2i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageIdNAAFormatIEEERegistered: STORAGE_ID_NAA_FORMAT = STORAGE_ID_NAA_FORMAT(3i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageIdNAAFormatIEEEERegisteredExtended: STORAGE_ID_NAA_FORMAT = STORAGE_ID_NAA_FORMAT(5i32);
impl ::core::marker::Copy for STORAGE_ID_NAA_FORMAT {}
impl ::core::clone::Clone for STORAGE_ID_NAA_FORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for STORAGE_ID_NAA_FORMAT {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for STORAGE_ID_NAA_FORMAT {
    type Abi = Self;
}
impl ::core::fmt::Debug for STORAGE_ID_NAA_FORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("STORAGE_ID_NAA_FORMAT").field(&self.0).finish()
    }
}
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
impl ::core::fmt::Debug for STORAGE_LB_PROVISIONING_MAP_RESOURCES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STORAGE_LB_PROVISIONING_MAP_RESOURCES").field("Size", &self.Size).field("Version", &self.Version).field("_bitfield1", &self._bitfield1).field("Reserved1", &self.Reserved1).field("_bitfield2", &self._bitfield2).field("Reserved3", &self.Reserved3).field("AvailableMappingResources", &self.AvailableMappingResources).field("UsedMappingResources", &self.UsedMappingResources).finish()
    }
}
unsafe impl ::windows::core::Abi for STORAGE_LB_PROVISIONING_MAP_RESOURCES {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for STORAGE_LB_PROVISIONING_MAP_RESOURCES {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<STORAGE_LB_PROVISIONING_MAP_RESOURCES>()) == 0 }
    }
}
impl ::core::cmp::Eq for STORAGE_LB_PROVISIONING_MAP_RESOURCES {}
impl ::core::default::Default for STORAGE_LB_PROVISIONING_MAP_RESOURCES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for STORAGE_MEDIA_SERIAL_NUMBER_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STORAGE_MEDIA_SERIAL_NUMBER_DATA").field("Reserved", &self.Reserved).field("SerialNumberLength", &self.SerialNumberLength).field("SerialNumber", &self.SerialNumber).finish()
    }
}
unsafe impl ::windows::core::Abi for STORAGE_MEDIA_SERIAL_NUMBER_DATA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for STORAGE_MEDIA_SERIAL_NUMBER_DATA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<STORAGE_MEDIA_SERIAL_NUMBER_DATA>()) == 0 }
    }
}
impl ::core::cmp::Eq for STORAGE_MEDIA_SERIAL_NUMBER_DATA {}
impl ::core::default::Default for STORAGE_MEDIA_SERIAL_NUMBER_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct STORAGE_MEDIA_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const DDS_4mm: STORAGE_MEDIA_TYPE = STORAGE_MEDIA_TYPE(32i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const MiniQic: STORAGE_MEDIA_TYPE = STORAGE_MEDIA_TYPE(33i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const Travan: STORAGE_MEDIA_TYPE = STORAGE_MEDIA_TYPE(34i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const QIC: STORAGE_MEDIA_TYPE = STORAGE_MEDIA_TYPE(35i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const MP_8mm: STORAGE_MEDIA_TYPE = STORAGE_MEDIA_TYPE(36i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const AME_8mm: STORAGE_MEDIA_TYPE = STORAGE_MEDIA_TYPE(37i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const AIT1_8mm: STORAGE_MEDIA_TYPE = STORAGE_MEDIA_TYPE(38i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const DLT: STORAGE_MEDIA_TYPE = STORAGE_MEDIA_TYPE(39i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const NCTP: STORAGE_MEDIA_TYPE = STORAGE_MEDIA_TYPE(40i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const IBM_3480: STORAGE_MEDIA_TYPE = STORAGE_MEDIA_TYPE(41i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const IBM_3490E: STORAGE_MEDIA_TYPE = STORAGE_MEDIA_TYPE(42i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const IBM_Magstar_3590: STORAGE_MEDIA_TYPE = STORAGE_MEDIA_TYPE(43i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const IBM_Magstar_MP: STORAGE_MEDIA_TYPE = STORAGE_MEDIA_TYPE(44i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const STK_DATA_D3: STORAGE_MEDIA_TYPE = STORAGE_MEDIA_TYPE(45i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const SONY_DTF: STORAGE_MEDIA_TYPE = STORAGE_MEDIA_TYPE(46i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const DV_6mm: STORAGE_MEDIA_TYPE = STORAGE_MEDIA_TYPE(47i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const DMI: STORAGE_MEDIA_TYPE = STORAGE_MEDIA_TYPE(48i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const SONY_D2: STORAGE_MEDIA_TYPE = STORAGE_MEDIA_TYPE(49i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const CLEANER_CARTRIDGE: STORAGE_MEDIA_TYPE = STORAGE_MEDIA_TYPE(50i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const CD_ROM: STORAGE_MEDIA_TYPE = STORAGE_MEDIA_TYPE(51i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const CD_R: STORAGE_MEDIA_TYPE = STORAGE_MEDIA_TYPE(52i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const CD_RW: STORAGE_MEDIA_TYPE = STORAGE_MEDIA_TYPE(53i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const DVD_ROM: STORAGE_MEDIA_TYPE = STORAGE_MEDIA_TYPE(54i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const DVD_R: STORAGE_MEDIA_TYPE = STORAGE_MEDIA_TYPE(55i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const DVD_RW: STORAGE_MEDIA_TYPE = STORAGE_MEDIA_TYPE(56i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const MO_3_RW: STORAGE_MEDIA_TYPE = STORAGE_MEDIA_TYPE(57i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const MO_5_WO: STORAGE_MEDIA_TYPE = STORAGE_MEDIA_TYPE(58i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const MO_5_RW: STORAGE_MEDIA_TYPE = STORAGE_MEDIA_TYPE(59i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const MO_5_LIMDOW: STORAGE_MEDIA_TYPE = STORAGE_MEDIA_TYPE(60i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const PC_5_WO: STORAGE_MEDIA_TYPE = STORAGE_MEDIA_TYPE(61i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const PC_5_RW: STORAGE_MEDIA_TYPE = STORAGE_MEDIA_TYPE(62i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const PD_5_RW: STORAGE_MEDIA_TYPE = STORAGE_MEDIA_TYPE(63i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ABL_5_WO: STORAGE_MEDIA_TYPE = STORAGE_MEDIA_TYPE(64i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const PINNACLE_APEX_5_RW: STORAGE_MEDIA_TYPE = STORAGE_MEDIA_TYPE(65i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const SONY_12_WO: STORAGE_MEDIA_TYPE = STORAGE_MEDIA_TYPE(66i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const PHILIPS_12_WO: STORAGE_MEDIA_TYPE = STORAGE_MEDIA_TYPE(67i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const HITACHI_12_WO: STORAGE_MEDIA_TYPE = STORAGE_MEDIA_TYPE(68i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const CYGNET_12_WO: STORAGE_MEDIA_TYPE = STORAGE_MEDIA_TYPE(69i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const KODAK_14_WO: STORAGE_MEDIA_TYPE = STORAGE_MEDIA_TYPE(70i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const MO_NFR_525: STORAGE_MEDIA_TYPE = STORAGE_MEDIA_TYPE(71i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const NIKON_12_RW: STORAGE_MEDIA_TYPE = STORAGE_MEDIA_TYPE(72i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const IOMEGA_ZIP: STORAGE_MEDIA_TYPE = STORAGE_MEDIA_TYPE(73i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const IOMEGA_JAZ: STORAGE_MEDIA_TYPE = STORAGE_MEDIA_TYPE(74i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const SYQUEST_EZ135: STORAGE_MEDIA_TYPE = STORAGE_MEDIA_TYPE(75i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const SYQUEST_EZFLYER: STORAGE_MEDIA_TYPE = STORAGE_MEDIA_TYPE(76i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const SYQUEST_SYJET: STORAGE_MEDIA_TYPE = STORAGE_MEDIA_TYPE(77i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const AVATAR_F2: STORAGE_MEDIA_TYPE = STORAGE_MEDIA_TYPE(78i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const MP2_8mm: STORAGE_MEDIA_TYPE = STORAGE_MEDIA_TYPE(79i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const DST_S: STORAGE_MEDIA_TYPE = STORAGE_MEDIA_TYPE(80i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const DST_M: STORAGE_MEDIA_TYPE = STORAGE_MEDIA_TYPE(81i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const DST_L: STORAGE_MEDIA_TYPE = STORAGE_MEDIA_TYPE(82i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const VXATape_1: STORAGE_MEDIA_TYPE = STORAGE_MEDIA_TYPE(83i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const VXATape_2: STORAGE_MEDIA_TYPE = STORAGE_MEDIA_TYPE(84i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const STK_9840: STORAGE_MEDIA_TYPE = STORAGE_MEDIA_TYPE(85i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const LTO_Ultrium: STORAGE_MEDIA_TYPE = STORAGE_MEDIA_TYPE(86i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const LTO_Accelis: STORAGE_MEDIA_TYPE = STORAGE_MEDIA_TYPE(87i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const DVD_RAM: STORAGE_MEDIA_TYPE = STORAGE_MEDIA_TYPE(88i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const AIT_8mm: STORAGE_MEDIA_TYPE = STORAGE_MEDIA_TYPE(89i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ADR_1: STORAGE_MEDIA_TYPE = STORAGE_MEDIA_TYPE(90i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ADR_2: STORAGE_MEDIA_TYPE = STORAGE_MEDIA_TYPE(91i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const STK_9940: STORAGE_MEDIA_TYPE = STORAGE_MEDIA_TYPE(92i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const SAIT: STORAGE_MEDIA_TYPE = STORAGE_MEDIA_TYPE(93i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const VXATape: STORAGE_MEDIA_TYPE = STORAGE_MEDIA_TYPE(94i32);
impl ::core::marker::Copy for STORAGE_MEDIA_TYPE {}
impl ::core::clone::Clone for STORAGE_MEDIA_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for STORAGE_MEDIA_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for STORAGE_MEDIA_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for STORAGE_MEDIA_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("STORAGE_MEDIA_TYPE").field(&self.0).finish()
    }
}
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
impl ::core::fmt::Debug for STORAGE_MEDIUM_PRODUCT_TYPE_DESCRIPTOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STORAGE_MEDIUM_PRODUCT_TYPE_DESCRIPTOR").field("Version", &self.Version).field("Size", &self.Size).field("MediumProductType", &self.MediumProductType).finish()
    }
}
unsafe impl ::windows::core::Abi for STORAGE_MEDIUM_PRODUCT_TYPE_DESCRIPTOR {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for STORAGE_MEDIUM_PRODUCT_TYPE_DESCRIPTOR {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<STORAGE_MEDIUM_PRODUCT_TYPE_DESCRIPTOR>()) == 0 }
    }
}
impl ::core::cmp::Eq for STORAGE_MEDIUM_PRODUCT_TYPE_DESCRIPTOR {}
impl ::core::default::Default for STORAGE_MEDIUM_PRODUCT_TYPE_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for STORAGE_MINIPORT_DESCRIPTOR {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for STORAGE_MINIPORT_DESCRIPTOR {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<STORAGE_MINIPORT_DESCRIPTOR>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for STORAGE_MINIPORT_DESCRIPTOR {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for STORAGE_MINIPORT_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for STORAGE_MINIPORT_DESCRIPTOR_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for STORAGE_MINIPORT_DESCRIPTOR_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<STORAGE_MINIPORT_DESCRIPTOR_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for STORAGE_MINIPORT_DESCRIPTOR_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for STORAGE_MINIPORT_DESCRIPTOR_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for STORAGE_MINIPORT_DESCRIPTOR_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STORAGE_MINIPORT_DESCRIPTOR_0_0").field("_bitfield", &self._bitfield).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for STORAGE_MINIPORT_DESCRIPTOR_0_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for STORAGE_MINIPORT_DESCRIPTOR_0_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<STORAGE_MINIPORT_DESCRIPTOR_0_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for STORAGE_MINIPORT_DESCRIPTOR_0_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for STORAGE_MINIPORT_DESCRIPTOR_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
unsafe impl ::windows::core::Abi for STORAGE_OFFLOAD_READ_OUTPUT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for STORAGE_OFFLOAD_READ_OUTPUT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<STORAGE_OFFLOAD_READ_OUTPUT>()) == 0 }
    }
}
impl ::core::cmp::Eq for STORAGE_OFFLOAD_READ_OUTPUT {}
impl ::core::default::Default for STORAGE_OFFLOAD_READ_OUTPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
unsafe impl ::windows::core::Abi for STORAGE_OFFLOAD_TOKEN {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for STORAGE_OFFLOAD_TOKEN {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<STORAGE_OFFLOAD_TOKEN>()) == 0 }
    }
}
impl ::core::cmp::Eq for STORAGE_OFFLOAD_TOKEN {}
impl ::core::default::Default for STORAGE_OFFLOAD_TOKEN {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
unsafe impl ::windows::core::Abi for STORAGE_OFFLOAD_TOKEN_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for STORAGE_OFFLOAD_TOKEN_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<STORAGE_OFFLOAD_TOKEN_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for STORAGE_OFFLOAD_TOKEN_0 {}
impl ::core::default::Default for STORAGE_OFFLOAD_TOKEN_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for STORAGE_OFFLOAD_TOKEN_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STORAGE_OFFLOAD_TOKEN_0_0").field("Reserved2", &self.Reserved2).finish()
    }
}
unsafe impl ::windows::core::Abi for STORAGE_OFFLOAD_TOKEN_0_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for STORAGE_OFFLOAD_TOKEN_0_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<STORAGE_OFFLOAD_TOKEN_0_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for STORAGE_OFFLOAD_TOKEN_0_0 {}
impl ::core::default::Default for STORAGE_OFFLOAD_TOKEN_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for STORAGE_OFFLOAD_WRITE_OUTPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STORAGE_OFFLOAD_WRITE_OUTPUT").field("OffloadWriteFlags", &self.OffloadWriteFlags).field("Reserved", &self.Reserved).field("LengthCopied", &self.LengthCopied).finish()
    }
}
unsafe impl ::windows::core::Abi for STORAGE_OFFLOAD_WRITE_OUTPUT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for STORAGE_OFFLOAD_WRITE_OUTPUT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<STORAGE_OFFLOAD_WRITE_OUTPUT>()) == 0 }
    }
}
impl ::core::cmp::Eq for STORAGE_OFFLOAD_WRITE_OUTPUT {}
impl ::core::default::Default for STORAGE_OFFLOAD_WRITE_OUTPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
unsafe impl ::windows::core::Abi for STORAGE_OPERATIONAL_REASON {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for STORAGE_OPERATIONAL_REASON {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<STORAGE_OPERATIONAL_REASON>()) == 0 }
    }
}
impl ::core::cmp::Eq for STORAGE_OPERATIONAL_REASON {}
impl ::core::default::Default for STORAGE_OPERATIONAL_REASON {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
unsafe impl ::windows::core::Abi for STORAGE_OPERATIONAL_REASON_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for STORAGE_OPERATIONAL_REASON_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<STORAGE_OPERATIONAL_REASON_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for STORAGE_OPERATIONAL_REASON_0 {}
impl ::core::default::Default for STORAGE_OPERATIONAL_REASON_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for STORAGE_OPERATIONAL_REASON_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STORAGE_OPERATIONAL_REASON_0_0").field("CriticalHealth", &self.CriticalHealth).field("ModuleHealth", &self.ModuleHealth).field("ErrorThresholdStatus", &self.ErrorThresholdStatus).finish()
    }
}
unsafe impl ::windows::core::Abi for STORAGE_OPERATIONAL_REASON_0_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for STORAGE_OPERATIONAL_REASON_0_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<STORAGE_OPERATIONAL_REASON_0_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for STORAGE_OPERATIONAL_REASON_0_0 {}
impl ::core::default::Default for STORAGE_OPERATIONAL_REASON_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for STORAGE_OPERATIONAL_REASON_0_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STORAGE_OPERATIONAL_REASON_0_1").field("SenseKey", &self.SenseKey).field("ASC", &self.ASC).field("ASCQ", &self.ASCQ).field("Reserved", &self.Reserved).finish()
    }
}
unsafe impl ::windows::core::Abi for STORAGE_OPERATIONAL_REASON_0_1 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for STORAGE_OPERATIONAL_REASON_0_1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<STORAGE_OPERATIONAL_REASON_0_1>()) == 0 }
    }
}
impl ::core::cmp::Eq for STORAGE_OPERATIONAL_REASON_0_1 {}
impl ::core::default::Default for STORAGE_OPERATIONAL_REASON_0_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct STORAGE_OPERATIONAL_STATUS_REASON(pub i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const DiskOpReasonUnknown: STORAGE_OPERATIONAL_STATUS_REASON = STORAGE_OPERATIONAL_STATUS_REASON(0i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const DiskOpReasonScsiSenseCode: STORAGE_OPERATIONAL_STATUS_REASON = STORAGE_OPERATIONAL_STATUS_REASON(1i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const DiskOpReasonMedia: STORAGE_OPERATIONAL_STATUS_REASON = STORAGE_OPERATIONAL_STATUS_REASON(2i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const DiskOpReasonIo: STORAGE_OPERATIONAL_STATUS_REASON = STORAGE_OPERATIONAL_STATUS_REASON(3i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const DiskOpReasonThresholdExceeded: STORAGE_OPERATIONAL_STATUS_REASON = STORAGE_OPERATIONAL_STATUS_REASON(4i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const DiskOpReasonLostData: STORAGE_OPERATIONAL_STATUS_REASON = STORAGE_OPERATIONAL_STATUS_REASON(5i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const DiskOpReasonEnergySource: STORAGE_OPERATIONAL_STATUS_REASON = STORAGE_OPERATIONAL_STATUS_REASON(6i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const DiskOpReasonConfiguration: STORAGE_OPERATIONAL_STATUS_REASON = STORAGE_OPERATIONAL_STATUS_REASON(7i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const DiskOpReasonDeviceController: STORAGE_OPERATIONAL_STATUS_REASON = STORAGE_OPERATIONAL_STATUS_REASON(8i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const DiskOpReasonMediaController: STORAGE_OPERATIONAL_STATUS_REASON = STORAGE_OPERATIONAL_STATUS_REASON(9i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const DiskOpReasonComponent: STORAGE_OPERATIONAL_STATUS_REASON = STORAGE_OPERATIONAL_STATUS_REASON(10i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const DiskOpReasonNVDIMM_N: STORAGE_OPERATIONAL_STATUS_REASON = STORAGE_OPERATIONAL_STATUS_REASON(11i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const DiskOpReasonBackgroundOperation: STORAGE_OPERATIONAL_STATUS_REASON = STORAGE_OPERATIONAL_STATUS_REASON(12i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const DiskOpReasonInvalidFirmware: STORAGE_OPERATIONAL_STATUS_REASON = STORAGE_OPERATIONAL_STATUS_REASON(13i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const DiskOpReasonHealthCheck: STORAGE_OPERATIONAL_STATUS_REASON = STORAGE_OPERATIONAL_STATUS_REASON(14i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const DiskOpReasonLostDataPersistence: STORAGE_OPERATIONAL_STATUS_REASON = STORAGE_OPERATIONAL_STATUS_REASON(15i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const DiskOpReasonDisabledByPlatform: STORAGE_OPERATIONAL_STATUS_REASON = STORAGE_OPERATIONAL_STATUS_REASON(16i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const DiskOpReasonLostWritePersistence: STORAGE_OPERATIONAL_STATUS_REASON = STORAGE_OPERATIONAL_STATUS_REASON(17i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const DiskOpReasonDataPersistenceLossImminent: STORAGE_OPERATIONAL_STATUS_REASON = STORAGE_OPERATIONAL_STATUS_REASON(18i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const DiskOpReasonWritePersistenceLossImminent: STORAGE_OPERATIONAL_STATUS_REASON = STORAGE_OPERATIONAL_STATUS_REASON(19i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const DiskOpReasonMax: STORAGE_OPERATIONAL_STATUS_REASON = STORAGE_OPERATIONAL_STATUS_REASON(20i32);
impl ::core::marker::Copy for STORAGE_OPERATIONAL_STATUS_REASON {}
impl ::core::clone::Clone for STORAGE_OPERATIONAL_STATUS_REASON {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for STORAGE_OPERATIONAL_STATUS_REASON {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for STORAGE_OPERATIONAL_STATUS_REASON {
    type Abi = Self;
}
impl ::core::fmt::Debug for STORAGE_OPERATIONAL_STATUS_REASON {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("STORAGE_OPERATIONAL_STATUS_REASON").field(&self.0).finish()
    }
}
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
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for STORAGE_PHYSICAL_ADAPTER_DATA {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for STORAGE_PHYSICAL_ADAPTER_DATA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<STORAGE_PHYSICAL_ADAPTER_DATA>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for STORAGE_PHYSICAL_ADAPTER_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for STORAGE_PHYSICAL_ADAPTER_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
unsafe impl ::windows::core::Abi for STORAGE_PHYSICAL_DEVICE_DATA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for STORAGE_PHYSICAL_DEVICE_DATA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<STORAGE_PHYSICAL_DEVICE_DATA>()) == 0 }
    }
}
impl ::core::cmp::Eq for STORAGE_PHYSICAL_DEVICE_DATA {}
impl ::core::default::Default for STORAGE_PHYSICAL_DEVICE_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for STORAGE_PHYSICAL_NODE_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STORAGE_PHYSICAL_NODE_DATA").field("NodeId", &self.NodeId).field("AdapterCount", &self.AdapterCount).field("AdapterDataLength", &self.AdapterDataLength).field("AdapterDataOffset", &self.AdapterDataOffset).field("DeviceCount", &self.DeviceCount).field("DeviceDataLength", &self.DeviceDataLength).field("DeviceDataOffset", &self.DeviceDataOffset).field("Reserved", &self.Reserved).finish()
    }
}
unsafe impl ::windows::core::Abi for STORAGE_PHYSICAL_NODE_DATA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for STORAGE_PHYSICAL_NODE_DATA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<STORAGE_PHYSICAL_NODE_DATA>()) == 0 }
    }
}
impl ::core::cmp::Eq for STORAGE_PHYSICAL_NODE_DATA {}
impl ::core::default::Default for STORAGE_PHYSICAL_NODE_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for STORAGE_PHYSICAL_TOPOLOGY_DESCRIPTOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STORAGE_PHYSICAL_TOPOLOGY_DESCRIPTOR").field("Version", &self.Version).field("Size", &self.Size).field("NodeCount", &self.NodeCount).field("Reserved", &self.Reserved).field("Node", &self.Node).finish()
    }
}
unsafe impl ::windows::core::Abi for STORAGE_PHYSICAL_TOPOLOGY_DESCRIPTOR {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for STORAGE_PHYSICAL_TOPOLOGY_DESCRIPTOR {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<STORAGE_PHYSICAL_TOPOLOGY_DESCRIPTOR>()) == 0 }
    }
}
impl ::core::cmp::Eq for STORAGE_PHYSICAL_TOPOLOGY_DESCRIPTOR {}
impl ::core::default::Default for STORAGE_PHYSICAL_TOPOLOGY_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct STORAGE_PORT_CODE_SET(pub i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StoragePortCodeSetReserved: STORAGE_PORT_CODE_SET = STORAGE_PORT_CODE_SET(0i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StoragePortCodeSetStorport: STORAGE_PORT_CODE_SET = STORAGE_PORT_CODE_SET(1i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StoragePortCodeSetSCSIport: STORAGE_PORT_CODE_SET = STORAGE_PORT_CODE_SET(2i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StoragePortCodeSetSpaceport: STORAGE_PORT_CODE_SET = STORAGE_PORT_CODE_SET(3i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StoragePortCodeSetATAport: STORAGE_PORT_CODE_SET = STORAGE_PORT_CODE_SET(4i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StoragePortCodeSetUSBport: STORAGE_PORT_CODE_SET = STORAGE_PORT_CODE_SET(5i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StoragePortCodeSetSBP2port: STORAGE_PORT_CODE_SET = STORAGE_PORT_CODE_SET(6i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StoragePortCodeSetSDport: STORAGE_PORT_CODE_SET = STORAGE_PORT_CODE_SET(7i32);
impl ::core::marker::Copy for STORAGE_PORT_CODE_SET {}
impl ::core::clone::Clone for STORAGE_PORT_CODE_SET {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for STORAGE_PORT_CODE_SET {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for STORAGE_PORT_CODE_SET {
    type Abi = Self;
}
impl ::core::fmt::Debug for STORAGE_PORT_CODE_SET {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("STORAGE_PORT_CODE_SET").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct STORAGE_POWERUP_REASON_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StoragePowerupUnknown: STORAGE_POWERUP_REASON_TYPE = STORAGE_POWERUP_REASON_TYPE(0i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StoragePowerupIO: STORAGE_POWERUP_REASON_TYPE = STORAGE_POWERUP_REASON_TYPE(1i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StoragePowerupDeviceAttention: STORAGE_POWERUP_REASON_TYPE = STORAGE_POWERUP_REASON_TYPE(2i32);
impl ::core::marker::Copy for STORAGE_POWERUP_REASON_TYPE {}
impl ::core::clone::Clone for STORAGE_POWERUP_REASON_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for STORAGE_POWERUP_REASON_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for STORAGE_POWERUP_REASON_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for STORAGE_POWERUP_REASON_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("STORAGE_POWERUP_REASON_TYPE").field(&self.0).finish()
    }
}
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
impl ::core::fmt::Debug for STORAGE_PREDICT_FAILURE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STORAGE_PREDICT_FAILURE").field("PredictFailure", &self.PredictFailure).field("VendorSpecific", &self.VendorSpecific).finish()
    }
}
unsafe impl ::windows::core::Abi for STORAGE_PREDICT_FAILURE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for STORAGE_PREDICT_FAILURE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<STORAGE_PREDICT_FAILURE>()) == 0 }
    }
}
impl ::core::cmp::Eq for STORAGE_PREDICT_FAILURE {}
impl ::core::default::Default for STORAGE_PREDICT_FAILURE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for STORAGE_PRIORITY_HINT_SUPPORT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STORAGE_PRIORITY_HINT_SUPPORT").field("SupportFlags", &self.SupportFlags).finish()
    }
}
unsafe impl ::windows::core::Abi for STORAGE_PRIORITY_HINT_SUPPORT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for STORAGE_PRIORITY_HINT_SUPPORT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<STORAGE_PRIORITY_HINT_SUPPORT>()) == 0 }
    }
}
impl ::core::cmp::Eq for STORAGE_PRIORITY_HINT_SUPPORT {}
impl ::core::default::Default for STORAGE_PRIORITY_HINT_SUPPORT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const STORAGE_PRIORITY_HINT_SUPPORTED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct STORAGE_PROPERTY_ID(pub i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageDeviceProperty: STORAGE_PROPERTY_ID = STORAGE_PROPERTY_ID(0i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageAdapterProperty: STORAGE_PROPERTY_ID = STORAGE_PROPERTY_ID(1i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageDeviceIdProperty: STORAGE_PROPERTY_ID = STORAGE_PROPERTY_ID(2i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageDeviceUniqueIdProperty: STORAGE_PROPERTY_ID = STORAGE_PROPERTY_ID(3i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageDeviceWriteCacheProperty: STORAGE_PROPERTY_ID = STORAGE_PROPERTY_ID(4i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageMiniportProperty: STORAGE_PROPERTY_ID = STORAGE_PROPERTY_ID(5i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageAccessAlignmentProperty: STORAGE_PROPERTY_ID = STORAGE_PROPERTY_ID(6i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageDeviceSeekPenaltyProperty: STORAGE_PROPERTY_ID = STORAGE_PROPERTY_ID(7i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageDeviceTrimProperty: STORAGE_PROPERTY_ID = STORAGE_PROPERTY_ID(8i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageDeviceWriteAggregationProperty: STORAGE_PROPERTY_ID = STORAGE_PROPERTY_ID(9i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageDeviceDeviceTelemetryProperty: STORAGE_PROPERTY_ID = STORAGE_PROPERTY_ID(10i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageDeviceLBProvisioningProperty: STORAGE_PROPERTY_ID = STORAGE_PROPERTY_ID(11i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageDevicePowerProperty: STORAGE_PROPERTY_ID = STORAGE_PROPERTY_ID(12i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageDeviceCopyOffloadProperty: STORAGE_PROPERTY_ID = STORAGE_PROPERTY_ID(13i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageDeviceResiliencyProperty: STORAGE_PROPERTY_ID = STORAGE_PROPERTY_ID(14i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageDeviceMediumProductType: STORAGE_PROPERTY_ID = STORAGE_PROPERTY_ID(15i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageAdapterRpmbProperty: STORAGE_PROPERTY_ID = STORAGE_PROPERTY_ID(16i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageAdapterCryptoProperty: STORAGE_PROPERTY_ID = STORAGE_PROPERTY_ID(17i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageDeviceIoCapabilityProperty: STORAGE_PROPERTY_ID = STORAGE_PROPERTY_ID(48i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageAdapterProtocolSpecificProperty: STORAGE_PROPERTY_ID = STORAGE_PROPERTY_ID(49i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageDeviceProtocolSpecificProperty: STORAGE_PROPERTY_ID = STORAGE_PROPERTY_ID(50i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageAdapterTemperatureProperty: STORAGE_PROPERTY_ID = STORAGE_PROPERTY_ID(51i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageDeviceTemperatureProperty: STORAGE_PROPERTY_ID = STORAGE_PROPERTY_ID(52i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageAdapterPhysicalTopologyProperty: STORAGE_PROPERTY_ID = STORAGE_PROPERTY_ID(53i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageDevicePhysicalTopologyProperty: STORAGE_PROPERTY_ID = STORAGE_PROPERTY_ID(54i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageDeviceAttributesProperty: STORAGE_PROPERTY_ID = STORAGE_PROPERTY_ID(55i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageDeviceManagementStatus: STORAGE_PROPERTY_ID = STORAGE_PROPERTY_ID(56i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageAdapterSerialNumberProperty: STORAGE_PROPERTY_ID = STORAGE_PROPERTY_ID(57i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageDeviceLocationProperty: STORAGE_PROPERTY_ID = STORAGE_PROPERTY_ID(58i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageDeviceNumaProperty: STORAGE_PROPERTY_ID = STORAGE_PROPERTY_ID(59i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageDeviceZonedDeviceProperty: STORAGE_PROPERTY_ID = STORAGE_PROPERTY_ID(60i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageDeviceUnsafeShutdownCount: STORAGE_PROPERTY_ID = STORAGE_PROPERTY_ID(61i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageDeviceEnduranceProperty: STORAGE_PROPERTY_ID = STORAGE_PROPERTY_ID(62i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageDeviceLedStateProperty: STORAGE_PROPERTY_ID = STORAGE_PROPERTY_ID(63i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageDeviceSelfEncryptionProperty: STORAGE_PROPERTY_ID = STORAGE_PROPERTY_ID(64i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageFruIdProperty: STORAGE_PROPERTY_ID = STORAGE_PROPERTY_ID(65i32);
impl ::core::marker::Copy for STORAGE_PROPERTY_ID {}
impl ::core::clone::Clone for STORAGE_PROPERTY_ID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for STORAGE_PROPERTY_ID {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for STORAGE_PROPERTY_ID {
    type Abi = Self;
}
impl ::core::fmt::Debug for STORAGE_PROPERTY_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("STORAGE_PROPERTY_ID").field(&self.0).finish()
    }
}
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
impl ::core::fmt::Debug for STORAGE_PROPERTY_QUERY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STORAGE_PROPERTY_QUERY").field("PropertyId", &self.PropertyId).field("QueryType", &self.QueryType).field("AdditionalParameters", &self.AdditionalParameters).finish()
    }
}
unsafe impl ::windows::core::Abi for STORAGE_PROPERTY_QUERY {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for STORAGE_PROPERTY_QUERY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<STORAGE_PROPERTY_QUERY>()) == 0 }
    }
}
impl ::core::cmp::Eq for STORAGE_PROPERTY_QUERY {}
impl ::core::default::Default for STORAGE_PROPERTY_QUERY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for STORAGE_PROPERTY_SET {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STORAGE_PROPERTY_SET").field("PropertyId", &self.PropertyId).field("SetType", &self.SetType).field("AdditionalParameters", &self.AdditionalParameters).finish()
    }
}
unsafe impl ::windows::core::Abi for STORAGE_PROPERTY_SET {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for STORAGE_PROPERTY_SET {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<STORAGE_PROPERTY_SET>()) == 0 }
    }
}
impl ::core::cmp::Eq for STORAGE_PROPERTY_SET {}
impl ::core::default::Default for STORAGE_PROPERTY_SET {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct STORAGE_PROTOCOL_ATA_DATA_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const AtaDataTypeUnknown: STORAGE_PROTOCOL_ATA_DATA_TYPE = STORAGE_PROTOCOL_ATA_DATA_TYPE(0i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const AtaDataTypeIdentify: STORAGE_PROTOCOL_ATA_DATA_TYPE = STORAGE_PROTOCOL_ATA_DATA_TYPE(1i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const AtaDataTypeLogPage: STORAGE_PROTOCOL_ATA_DATA_TYPE = STORAGE_PROTOCOL_ATA_DATA_TYPE(2i32);
impl ::core::marker::Copy for STORAGE_PROTOCOL_ATA_DATA_TYPE {}
impl ::core::clone::Clone for STORAGE_PROTOCOL_ATA_DATA_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for STORAGE_PROTOCOL_ATA_DATA_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for STORAGE_PROTOCOL_ATA_DATA_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for STORAGE_PROTOCOL_ATA_DATA_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("STORAGE_PROTOCOL_ATA_DATA_TYPE").field(&self.0).finish()
    }
}
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
impl ::core::fmt::Debug for STORAGE_PROTOCOL_COMMAND {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STORAGE_PROTOCOL_COMMAND")
            .field("Version", &self.Version)
            .field("Length", &self.Length)
            .field("ProtocolType", &self.ProtocolType)
            .field("Flags", &self.Flags)
            .field("ReturnStatus", &self.ReturnStatus)
            .field("ErrorCode", &self.ErrorCode)
            .field("CommandLength", &self.CommandLength)
            .field("ErrorInfoLength", &self.ErrorInfoLength)
            .field("DataToDeviceTransferLength", &self.DataToDeviceTransferLength)
            .field("DataFromDeviceTransferLength", &self.DataFromDeviceTransferLength)
            .field("TimeOutValue", &self.TimeOutValue)
            .field("ErrorInfoOffset", &self.ErrorInfoOffset)
            .field("DataToDeviceBufferOffset", &self.DataToDeviceBufferOffset)
            .field("DataFromDeviceBufferOffset", &self.DataFromDeviceBufferOffset)
            .field("CommandSpecific", &self.CommandSpecific)
            .field("Reserved0", &self.Reserved0)
            .field("FixedProtocolReturnData", &self.FixedProtocolReturnData)
            .field("Reserved1", &self.Reserved1)
            .field("Command", &self.Command)
            .finish()
    }
}
unsafe impl ::windows::core::Abi for STORAGE_PROTOCOL_COMMAND {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for STORAGE_PROTOCOL_COMMAND {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<STORAGE_PROTOCOL_COMMAND>()) == 0 }
    }
}
impl ::core::cmp::Eq for STORAGE_PROTOCOL_COMMAND {}
impl ::core::default::Default for STORAGE_PROTOCOL_COMMAND {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for STORAGE_PROTOCOL_DATA_DESCRIPTOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STORAGE_PROTOCOL_DATA_DESCRIPTOR").field("Version", &self.Version).field("Size", &self.Size).field("ProtocolSpecificData", &self.ProtocolSpecificData).finish()
    }
}
unsafe impl ::windows::core::Abi for STORAGE_PROTOCOL_DATA_DESCRIPTOR {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for STORAGE_PROTOCOL_DATA_DESCRIPTOR {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<STORAGE_PROTOCOL_DATA_DESCRIPTOR>()) == 0 }
    }
}
impl ::core::cmp::Eq for STORAGE_PROTOCOL_DATA_DESCRIPTOR {}
impl ::core::default::Default for STORAGE_PROTOCOL_DATA_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for STORAGE_PROTOCOL_DATA_DESCRIPTOR_EXT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STORAGE_PROTOCOL_DATA_DESCRIPTOR_EXT").field("Version", &self.Version).field("Size", &self.Size).field("ProtocolSpecificData", &self.ProtocolSpecificData).finish()
    }
}
unsafe impl ::windows::core::Abi for STORAGE_PROTOCOL_DATA_DESCRIPTOR_EXT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for STORAGE_PROTOCOL_DATA_DESCRIPTOR_EXT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<STORAGE_PROTOCOL_DATA_DESCRIPTOR_EXT>()) == 0 }
    }
}
impl ::core::cmp::Eq for STORAGE_PROTOCOL_DATA_DESCRIPTOR_EXT {}
impl ::core::default::Default for STORAGE_PROTOCOL_DATA_DESCRIPTOR_EXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
unsafe impl ::windows::core::Abi for STORAGE_PROTOCOL_DATA_SUBVALUE_GET_LOG_PAGE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for STORAGE_PROTOCOL_DATA_SUBVALUE_GET_LOG_PAGE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<STORAGE_PROTOCOL_DATA_SUBVALUE_GET_LOG_PAGE>()) == 0 }
    }
}
impl ::core::cmp::Eq for STORAGE_PROTOCOL_DATA_SUBVALUE_GET_LOG_PAGE {}
impl ::core::default::Default for STORAGE_PROTOCOL_DATA_SUBVALUE_GET_LOG_PAGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for STORAGE_PROTOCOL_DATA_SUBVALUE_GET_LOG_PAGE_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STORAGE_PROTOCOL_DATA_SUBVALUE_GET_LOG_PAGE_0").field("_bitfield", &self._bitfield).finish()
    }
}
unsafe impl ::windows::core::Abi for STORAGE_PROTOCOL_DATA_SUBVALUE_GET_LOG_PAGE_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for STORAGE_PROTOCOL_DATA_SUBVALUE_GET_LOG_PAGE_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<STORAGE_PROTOCOL_DATA_SUBVALUE_GET_LOG_PAGE_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for STORAGE_PROTOCOL_DATA_SUBVALUE_GET_LOG_PAGE_0 {}
impl ::core::default::Default for STORAGE_PROTOCOL_DATA_SUBVALUE_GET_LOG_PAGE_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct STORAGE_PROTOCOL_NVME_DATA_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const NVMeDataTypeUnknown: STORAGE_PROTOCOL_NVME_DATA_TYPE = STORAGE_PROTOCOL_NVME_DATA_TYPE(0i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const NVMeDataTypeIdentify: STORAGE_PROTOCOL_NVME_DATA_TYPE = STORAGE_PROTOCOL_NVME_DATA_TYPE(1i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const NVMeDataTypeLogPage: STORAGE_PROTOCOL_NVME_DATA_TYPE = STORAGE_PROTOCOL_NVME_DATA_TYPE(2i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const NVMeDataTypeFeature: STORAGE_PROTOCOL_NVME_DATA_TYPE = STORAGE_PROTOCOL_NVME_DATA_TYPE(3i32);
impl ::core::marker::Copy for STORAGE_PROTOCOL_NVME_DATA_TYPE {}
impl ::core::clone::Clone for STORAGE_PROTOCOL_NVME_DATA_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for STORAGE_PROTOCOL_NVME_DATA_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for STORAGE_PROTOCOL_NVME_DATA_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for STORAGE_PROTOCOL_NVME_DATA_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("STORAGE_PROTOCOL_NVME_DATA_TYPE").field(&self.0).finish()
    }
}
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
impl ::core::fmt::Debug for STORAGE_PROTOCOL_SPECIFIC_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STORAGE_PROTOCOL_SPECIFIC_DATA")
            .field("ProtocolType", &self.ProtocolType)
            .field("DataType", &self.DataType)
            .field("ProtocolDataRequestValue", &self.ProtocolDataRequestValue)
            .field("ProtocolDataRequestSubValue", &self.ProtocolDataRequestSubValue)
            .field("ProtocolDataOffset", &self.ProtocolDataOffset)
            .field("ProtocolDataLength", &self.ProtocolDataLength)
            .field("FixedProtocolReturnData", &self.FixedProtocolReturnData)
            .field("ProtocolDataRequestSubValue2", &self.ProtocolDataRequestSubValue2)
            .field("ProtocolDataRequestSubValue3", &self.ProtocolDataRequestSubValue3)
            .field("ProtocolDataRequestSubValue4", &self.ProtocolDataRequestSubValue4)
            .finish()
    }
}
unsafe impl ::windows::core::Abi for STORAGE_PROTOCOL_SPECIFIC_DATA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for STORAGE_PROTOCOL_SPECIFIC_DATA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<STORAGE_PROTOCOL_SPECIFIC_DATA>()) == 0 }
    }
}
impl ::core::cmp::Eq for STORAGE_PROTOCOL_SPECIFIC_DATA {}
impl ::core::default::Default for STORAGE_PROTOCOL_SPECIFIC_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for STORAGE_PROTOCOL_SPECIFIC_DATA_EXT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STORAGE_PROTOCOL_SPECIFIC_DATA_EXT")
            .field("ProtocolType", &self.ProtocolType)
            .field("DataType", &self.DataType)
            .field("ProtocolDataValue", &self.ProtocolDataValue)
            .field("ProtocolDataSubValue", &self.ProtocolDataSubValue)
            .field("ProtocolDataOffset", &self.ProtocolDataOffset)
            .field("ProtocolDataLength", &self.ProtocolDataLength)
            .field("FixedProtocolReturnData", &self.FixedProtocolReturnData)
            .field("ProtocolDataSubValue2", &self.ProtocolDataSubValue2)
            .field("ProtocolDataSubValue3", &self.ProtocolDataSubValue3)
            .field("ProtocolDataSubValue4", &self.ProtocolDataSubValue4)
            .field("ProtocolDataSubValue5", &self.ProtocolDataSubValue5)
            .field("Reserved", &self.Reserved)
            .finish()
    }
}
unsafe impl ::windows::core::Abi for STORAGE_PROTOCOL_SPECIFIC_DATA_EXT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for STORAGE_PROTOCOL_SPECIFIC_DATA_EXT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<STORAGE_PROTOCOL_SPECIFIC_DATA_EXT>()) == 0 }
    }
}
impl ::core::cmp::Eq for STORAGE_PROTOCOL_SPECIFIC_DATA_EXT {}
impl ::core::default::Default for STORAGE_PROTOCOL_SPECIFIC_DATA_EXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct STORAGE_PROTOCOL_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ProtocolTypeUnknown: STORAGE_PROTOCOL_TYPE = STORAGE_PROTOCOL_TYPE(0i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ProtocolTypeScsi: STORAGE_PROTOCOL_TYPE = STORAGE_PROTOCOL_TYPE(1i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ProtocolTypeAta: STORAGE_PROTOCOL_TYPE = STORAGE_PROTOCOL_TYPE(2i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ProtocolTypeNvme: STORAGE_PROTOCOL_TYPE = STORAGE_PROTOCOL_TYPE(3i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ProtocolTypeSd: STORAGE_PROTOCOL_TYPE = STORAGE_PROTOCOL_TYPE(4i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ProtocolTypeUfs: STORAGE_PROTOCOL_TYPE = STORAGE_PROTOCOL_TYPE(5i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ProtocolTypeProprietary: STORAGE_PROTOCOL_TYPE = STORAGE_PROTOCOL_TYPE(126i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ProtocolTypeMaxReserved: STORAGE_PROTOCOL_TYPE = STORAGE_PROTOCOL_TYPE(127i32);
impl ::core::marker::Copy for STORAGE_PROTOCOL_TYPE {}
impl ::core::clone::Clone for STORAGE_PROTOCOL_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for STORAGE_PROTOCOL_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for STORAGE_PROTOCOL_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for STORAGE_PROTOCOL_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("STORAGE_PROTOCOL_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct STORAGE_PROTOCOL_UFS_DATA_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const UfsDataTypeUnknown: STORAGE_PROTOCOL_UFS_DATA_TYPE = STORAGE_PROTOCOL_UFS_DATA_TYPE(0i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const UfsDataTypeQueryDescriptor: STORAGE_PROTOCOL_UFS_DATA_TYPE = STORAGE_PROTOCOL_UFS_DATA_TYPE(1i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const UfsDataTypeQueryAttribute: STORAGE_PROTOCOL_UFS_DATA_TYPE = STORAGE_PROTOCOL_UFS_DATA_TYPE(2i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const UfsDataTypeQueryFlag: STORAGE_PROTOCOL_UFS_DATA_TYPE = STORAGE_PROTOCOL_UFS_DATA_TYPE(3i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const UfsDataTypeQueryDmeAttribute: STORAGE_PROTOCOL_UFS_DATA_TYPE = STORAGE_PROTOCOL_UFS_DATA_TYPE(4i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const UfsDataTypeQueryDmePeerAttribute: STORAGE_PROTOCOL_UFS_DATA_TYPE = STORAGE_PROTOCOL_UFS_DATA_TYPE(5i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const UfsDataTypeMax: STORAGE_PROTOCOL_UFS_DATA_TYPE = STORAGE_PROTOCOL_UFS_DATA_TYPE(6i32);
impl ::core::marker::Copy for STORAGE_PROTOCOL_UFS_DATA_TYPE {}
impl ::core::clone::Clone for STORAGE_PROTOCOL_UFS_DATA_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for STORAGE_PROTOCOL_UFS_DATA_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for STORAGE_PROTOCOL_UFS_DATA_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for STORAGE_PROTOCOL_UFS_DATA_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("STORAGE_PROTOCOL_UFS_DATA_TYPE").field(&self.0).finish()
    }
}
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
#[cfg(feature = "Win32_Storage_Vhd")]
impl ::core::fmt::Debug for STORAGE_QUERY_DEPENDENT_VOLUME_LEV1_ENTRY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STORAGE_QUERY_DEPENDENT_VOLUME_LEV1_ENTRY").field("EntryLength", &self.EntryLength).field("DependencyTypeFlags", &self.DependencyTypeFlags).field("ProviderSpecificFlags", &self.ProviderSpecificFlags).field("VirtualStorageType", &self.VirtualStorageType).finish()
    }
}
#[cfg(feature = "Win32_Storage_Vhd")]
unsafe impl ::windows::core::Abi for STORAGE_QUERY_DEPENDENT_VOLUME_LEV1_ENTRY {
    type Abi = Self;
}
#[cfg(feature = "Win32_Storage_Vhd")]
impl ::core::cmp::PartialEq for STORAGE_QUERY_DEPENDENT_VOLUME_LEV1_ENTRY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<STORAGE_QUERY_DEPENDENT_VOLUME_LEV1_ENTRY>()) == 0 }
    }
}
#[cfg(feature = "Win32_Storage_Vhd")]
impl ::core::cmp::Eq for STORAGE_QUERY_DEPENDENT_VOLUME_LEV1_ENTRY {}
#[cfg(feature = "Win32_Storage_Vhd")]
impl ::core::default::Default for STORAGE_QUERY_DEPENDENT_VOLUME_LEV1_ENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
#[cfg(feature = "Win32_Storage_Vhd")]
impl ::core::fmt::Debug for STORAGE_QUERY_DEPENDENT_VOLUME_LEV2_ENTRY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STORAGE_QUERY_DEPENDENT_VOLUME_LEV2_ENTRY")
            .field("EntryLength", &self.EntryLength)
            .field("DependencyTypeFlags", &self.DependencyTypeFlags)
            .field("ProviderSpecificFlags", &self.ProviderSpecificFlags)
            .field("VirtualStorageType", &self.VirtualStorageType)
            .field("AncestorLevel", &self.AncestorLevel)
            .field("HostVolumeNameOffset", &self.HostVolumeNameOffset)
            .field("HostVolumeNameSize", &self.HostVolumeNameSize)
            .field("DependentVolumeNameOffset", &self.DependentVolumeNameOffset)
            .field("DependentVolumeNameSize", &self.DependentVolumeNameSize)
            .field("RelativePathOffset", &self.RelativePathOffset)
            .field("RelativePathSize", &self.RelativePathSize)
            .field("DependentDeviceNameOffset", &self.DependentDeviceNameOffset)
            .field("DependentDeviceNameSize", &self.DependentDeviceNameSize)
            .finish()
    }
}
#[cfg(feature = "Win32_Storage_Vhd")]
unsafe impl ::windows::core::Abi for STORAGE_QUERY_DEPENDENT_VOLUME_LEV2_ENTRY {
    type Abi = Self;
}
#[cfg(feature = "Win32_Storage_Vhd")]
impl ::core::cmp::PartialEq for STORAGE_QUERY_DEPENDENT_VOLUME_LEV2_ENTRY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<STORAGE_QUERY_DEPENDENT_VOLUME_LEV2_ENTRY>()) == 0 }
    }
}
#[cfg(feature = "Win32_Storage_Vhd")]
impl ::core::cmp::Eq for STORAGE_QUERY_DEPENDENT_VOLUME_LEV2_ENTRY {}
#[cfg(feature = "Win32_Storage_Vhd")]
impl ::core::default::Default for STORAGE_QUERY_DEPENDENT_VOLUME_LEV2_ENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for STORAGE_QUERY_DEPENDENT_VOLUME_REQUEST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STORAGE_QUERY_DEPENDENT_VOLUME_REQUEST").field("RequestLevel", &self.RequestLevel).field("RequestFlags", &self.RequestFlags).finish()
    }
}
unsafe impl ::windows::core::Abi for STORAGE_QUERY_DEPENDENT_VOLUME_REQUEST {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for STORAGE_QUERY_DEPENDENT_VOLUME_REQUEST {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<STORAGE_QUERY_DEPENDENT_VOLUME_REQUEST>()) == 0 }
    }
}
impl ::core::cmp::Eq for STORAGE_QUERY_DEPENDENT_VOLUME_REQUEST {}
impl ::core::default::Default for STORAGE_QUERY_DEPENDENT_VOLUME_REQUEST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
#[cfg(feature = "Win32_Storage_Vhd")]
unsafe impl ::windows::core::Abi for STORAGE_QUERY_DEPENDENT_VOLUME_RESPONSE {
    type Abi = Self;
}
#[cfg(feature = "Win32_Storage_Vhd")]
impl ::core::cmp::PartialEq for STORAGE_QUERY_DEPENDENT_VOLUME_RESPONSE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<STORAGE_QUERY_DEPENDENT_VOLUME_RESPONSE>()) == 0 }
    }
}
#[cfg(feature = "Win32_Storage_Vhd")]
impl ::core::cmp::Eq for STORAGE_QUERY_DEPENDENT_VOLUME_RESPONSE {}
#[cfg(feature = "Win32_Storage_Vhd")]
impl ::core::default::Default for STORAGE_QUERY_DEPENDENT_VOLUME_RESPONSE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
#[cfg(feature = "Win32_Storage_Vhd")]
unsafe impl ::windows::core::Abi for STORAGE_QUERY_DEPENDENT_VOLUME_RESPONSE_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Storage_Vhd")]
impl ::core::cmp::PartialEq for STORAGE_QUERY_DEPENDENT_VOLUME_RESPONSE_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<STORAGE_QUERY_DEPENDENT_VOLUME_RESPONSE_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Storage_Vhd")]
impl ::core::cmp::Eq for STORAGE_QUERY_DEPENDENT_VOLUME_RESPONSE_0 {}
#[cfg(feature = "Win32_Storage_Vhd")]
impl ::core::default::Default for STORAGE_QUERY_DEPENDENT_VOLUME_RESPONSE_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct STORAGE_QUERY_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const PropertyStandardQuery: STORAGE_QUERY_TYPE = STORAGE_QUERY_TYPE(0i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const PropertyExistsQuery: STORAGE_QUERY_TYPE = STORAGE_QUERY_TYPE(1i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const PropertyMaskQuery: STORAGE_QUERY_TYPE = STORAGE_QUERY_TYPE(2i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const PropertyQueryMaxDefined: STORAGE_QUERY_TYPE = STORAGE_QUERY_TYPE(3i32);
impl ::core::marker::Copy for STORAGE_QUERY_TYPE {}
impl ::core::clone::Clone for STORAGE_QUERY_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for STORAGE_QUERY_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for STORAGE_QUERY_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for STORAGE_QUERY_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("STORAGE_QUERY_TYPE").field(&self.0).finish()
    }
}
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
impl ::core::fmt::Debug for STORAGE_READ_CAPACITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STORAGE_READ_CAPACITY").field("Version", &self.Version).field("Size", &self.Size).field("BlockLength", &self.BlockLength).field("NumberOfBlocks", &self.NumberOfBlocks).field("DiskLength", &self.DiskLength).finish()
    }
}
unsafe impl ::windows::core::Abi for STORAGE_READ_CAPACITY {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for STORAGE_READ_CAPACITY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<STORAGE_READ_CAPACITY>()) == 0 }
    }
}
impl ::core::cmp::Eq for STORAGE_READ_CAPACITY {}
impl ::core::default::Default for STORAGE_READ_CAPACITY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for STORAGE_REINITIALIZE_MEDIA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STORAGE_REINITIALIZE_MEDIA").field("Version", &self.Version).field("Size", &self.Size).field("TimeoutInSeconds", &self.TimeoutInSeconds).field("SanitizeOption", &self.SanitizeOption).finish()
    }
}
unsafe impl ::windows::core::Abi for STORAGE_REINITIALIZE_MEDIA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for STORAGE_REINITIALIZE_MEDIA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<STORAGE_REINITIALIZE_MEDIA>()) == 0 }
    }
}
impl ::core::cmp::Eq for STORAGE_REINITIALIZE_MEDIA {}
impl ::core::default::Default for STORAGE_REINITIALIZE_MEDIA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for STORAGE_REINITIALIZE_MEDIA_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STORAGE_REINITIALIZE_MEDIA_0").field("_bitfield", &self._bitfield).finish()
    }
}
unsafe impl ::windows::core::Abi for STORAGE_REINITIALIZE_MEDIA_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for STORAGE_REINITIALIZE_MEDIA_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<STORAGE_REINITIALIZE_MEDIA_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for STORAGE_REINITIALIZE_MEDIA_0 {}
impl ::core::default::Default for STORAGE_REINITIALIZE_MEDIA_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct STORAGE_RESERVE_ID(pub i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageReserveIdNone: STORAGE_RESERVE_ID = STORAGE_RESERVE_ID(0i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageReserveIdHard: STORAGE_RESERVE_ID = STORAGE_RESERVE_ID(1i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageReserveIdSoft: STORAGE_RESERVE_ID = STORAGE_RESERVE_ID(2i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageReserveIdUpdateScratch: STORAGE_RESERVE_ID = STORAGE_RESERVE_ID(3i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageReserveIdMax: STORAGE_RESERVE_ID = STORAGE_RESERVE_ID(4i32);
impl ::core::marker::Copy for STORAGE_RESERVE_ID {}
impl ::core::clone::Clone for STORAGE_RESERVE_ID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for STORAGE_RESERVE_ID {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for STORAGE_RESERVE_ID {
    type Abi = Self;
}
impl ::core::fmt::Debug for STORAGE_RESERVE_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("STORAGE_RESERVE_ID").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct STORAGE_RPMB_COMMAND_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorRpmbProgramAuthKey: STORAGE_RPMB_COMMAND_TYPE = STORAGE_RPMB_COMMAND_TYPE(1i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorRpmbQueryWriteCounter: STORAGE_RPMB_COMMAND_TYPE = STORAGE_RPMB_COMMAND_TYPE(2i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorRpmbAuthenticatedWrite: STORAGE_RPMB_COMMAND_TYPE = STORAGE_RPMB_COMMAND_TYPE(3i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorRpmbAuthenticatedRead: STORAGE_RPMB_COMMAND_TYPE = STORAGE_RPMB_COMMAND_TYPE(4i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorRpmbReadResultRequest: STORAGE_RPMB_COMMAND_TYPE = STORAGE_RPMB_COMMAND_TYPE(5i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorRpmbAuthenticatedDeviceConfigWrite: STORAGE_RPMB_COMMAND_TYPE = STORAGE_RPMB_COMMAND_TYPE(6i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorRpmbAuthenticatedDeviceConfigRead: STORAGE_RPMB_COMMAND_TYPE = STORAGE_RPMB_COMMAND_TYPE(7i32);
impl ::core::marker::Copy for STORAGE_RPMB_COMMAND_TYPE {}
impl ::core::clone::Clone for STORAGE_RPMB_COMMAND_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for STORAGE_RPMB_COMMAND_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for STORAGE_RPMB_COMMAND_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for STORAGE_RPMB_COMMAND_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("STORAGE_RPMB_COMMAND_TYPE").field(&self.0).finish()
    }
}
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
impl ::core::fmt::Debug for STORAGE_RPMB_DATA_FRAME {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STORAGE_RPMB_DATA_FRAME").field("Stuff", &self.Stuff).field("KeyOrMAC", &self.KeyOrMAC).field("Data", &self.Data).field("Nonce", &self.Nonce).field("WriteCounter", &self.WriteCounter).field("Address", &self.Address).field("BlockCount", &self.BlockCount).field("OperationResult", &self.OperationResult).field("RequestOrResponseType", &self.RequestOrResponseType).finish()
    }
}
unsafe impl ::windows::core::Abi for STORAGE_RPMB_DATA_FRAME {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for STORAGE_RPMB_DATA_FRAME {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<STORAGE_RPMB_DATA_FRAME>()) == 0 }
    }
}
impl ::core::cmp::Eq for STORAGE_RPMB_DATA_FRAME {}
impl ::core::default::Default for STORAGE_RPMB_DATA_FRAME {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for STORAGE_RPMB_DESCRIPTOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STORAGE_RPMB_DESCRIPTOR").field("Version", &self.Version).field("Size", &self.Size).field("SizeInBytes", &self.SizeInBytes).field("MaxReliableWriteSizeInBytes", &self.MaxReliableWriteSizeInBytes).field("FrameFormat", &self.FrameFormat).finish()
    }
}
unsafe impl ::windows::core::Abi for STORAGE_RPMB_DESCRIPTOR {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for STORAGE_RPMB_DESCRIPTOR {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<STORAGE_RPMB_DESCRIPTOR>()) == 0 }
    }
}
impl ::core::cmp::Eq for STORAGE_RPMB_DESCRIPTOR {}
impl ::core::default::Default for STORAGE_RPMB_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const STORAGE_RPMB_DESCRIPTOR_VERSION_1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct STORAGE_RPMB_FRAME_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageRpmbFrameTypeUnknown: STORAGE_RPMB_FRAME_TYPE = STORAGE_RPMB_FRAME_TYPE(0i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageRpmbFrameTypeStandard: STORAGE_RPMB_FRAME_TYPE = STORAGE_RPMB_FRAME_TYPE(1i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageRpmbFrameTypeMax: STORAGE_RPMB_FRAME_TYPE = STORAGE_RPMB_FRAME_TYPE(2i32);
impl ::core::marker::Copy for STORAGE_RPMB_FRAME_TYPE {}
impl ::core::clone::Clone for STORAGE_RPMB_FRAME_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for STORAGE_RPMB_FRAME_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for STORAGE_RPMB_FRAME_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for STORAGE_RPMB_FRAME_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("STORAGE_RPMB_FRAME_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const STORAGE_RPMB_MINIMUM_RELIABLE_WRITE_SIZE: u32 = 512u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct STORAGE_SANITIZE_METHOD(pub i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageSanitizeMethodDefault: STORAGE_SANITIZE_METHOD = STORAGE_SANITIZE_METHOD(0i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageSanitizeMethodBlockErase: STORAGE_SANITIZE_METHOD = STORAGE_SANITIZE_METHOD(1i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageSanitizeMethodCryptoErase: STORAGE_SANITIZE_METHOD = STORAGE_SANITIZE_METHOD(2i32);
impl ::core::marker::Copy for STORAGE_SANITIZE_METHOD {}
impl ::core::clone::Clone for STORAGE_SANITIZE_METHOD {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for STORAGE_SANITIZE_METHOD {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for STORAGE_SANITIZE_METHOD {
    type Abi = Self;
}
impl ::core::fmt::Debug for STORAGE_SANITIZE_METHOD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("STORAGE_SANITIZE_METHOD").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct STORAGE_SET_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const PropertyStandardSet: STORAGE_SET_TYPE = STORAGE_SET_TYPE(0i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const PropertyExistsSet: STORAGE_SET_TYPE = STORAGE_SET_TYPE(1i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const PropertySetMaxDefined: STORAGE_SET_TYPE = STORAGE_SET_TYPE(2i32);
impl ::core::marker::Copy for STORAGE_SET_TYPE {}
impl ::core::clone::Clone for STORAGE_SET_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for STORAGE_SET_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for STORAGE_SET_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for STORAGE_SET_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("STORAGE_SET_TYPE").field(&self.0).finish()
    }
}
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
unsafe impl ::windows::core::Abi for STORAGE_SPEC_VERSION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for STORAGE_SPEC_VERSION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<STORAGE_SPEC_VERSION>()) == 0 }
    }
}
impl ::core::cmp::Eq for STORAGE_SPEC_VERSION {}
impl ::core::default::Default for STORAGE_SPEC_VERSION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
unsafe impl ::windows::core::Abi for STORAGE_SPEC_VERSION_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for STORAGE_SPEC_VERSION_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<STORAGE_SPEC_VERSION_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for STORAGE_SPEC_VERSION_0 {}
impl ::core::default::Default for STORAGE_SPEC_VERSION_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
unsafe impl ::windows::core::Abi for STORAGE_SPEC_VERSION_0_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for STORAGE_SPEC_VERSION_0_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<STORAGE_SPEC_VERSION_0_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for STORAGE_SPEC_VERSION_0_0 {}
impl ::core::default::Default for STORAGE_SPEC_VERSION_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for STORAGE_SPEC_VERSION_0_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STORAGE_SPEC_VERSION_0_0_0").field("SubMinor", &self.SubMinor).field("Minor", &self.Minor).finish()
    }
}
unsafe impl ::windows::core::Abi for STORAGE_SPEC_VERSION_0_0_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for STORAGE_SPEC_VERSION_0_0_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<STORAGE_SPEC_VERSION_0_0_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for STORAGE_SPEC_VERSION_0_0_0 {}
impl ::core::default::Default for STORAGE_SPEC_VERSION_0_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for STORAGE_TEMPERATURE_DATA_DESCRIPTOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STORAGE_TEMPERATURE_DATA_DESCRIPTOR").field("Version", &self.Version).field("Size", &self.Size).field("CriticalTemperature", &self.CriticalTemperature).field("WarningTemperature", &self.WarningTemperature).field("InfoCount", &self.InfoCount).field("Reserved0", &self.Reserved0).field("Reserved1", &self.Reserved1).field("TemperatureInfo", &self.TemperatureInfo).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for STORAGE_TEMPERATURE_DATA_DESCRIPTOR {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for STORAGE_TEMPERATURE_DATA_DESCRIPTOR {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<STORAGE_TEMPERATURE_DATA_DESCRIPTOR>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for STORAGE_TEMPERATURE_DATA_DESCRIPTOR {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for STORAGE_TEMPERATURE_DATA_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for STORAGE_TEMPERATURE_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STORAGE_TEMPERATURE_INFO").field("Index", &self.Index).field("Temperature", &self.Temperature).field("OverThreshold", &self.OverThreshold).field("UnderThreshold", &self.UnderThreshold).field("OverThresholdChangable", &self.OverThresholdChangable).field("UnderThresholdChangable", &self.UnderThresholdChangable).field("EventGenerated", &self.EventGenerated).field("Reserved0", &self.Reserved0).field("Reserved1", &self.Reserved1).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for STORAGE_TEMPERATURE_INFO {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for STORAGE_TEMPERATURE_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<STORAGE_TEMPERATURE_INFO>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for STORAGE_TEMPERATURE_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for STORAGE_TEMPERATURE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for STORAGE_TEMPERATURE_THRESHOLD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STORAGE_TEMPERATURE_THRESHOLD").field("Version", &self.Version).field("Size", &self.Size).field("Flags", &self.Flags).field("Index", &self.Index).field("Threshold", &self.Threshold).field("OverThreshold", &self.OverThreshold).field("Reserved", &self.Reserved).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for STORAGE_TEMPERATURE_THRESHOLD {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for STORAGE_TEMPERATURE_THRESHOLD {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<STORAGE_TEMPERATURE_THRESHOLD>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for STORAGE_TEMPERATURE_THRESHOLD {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for STORAGE_TEMPERATURE_THRESHOLD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const STORAGE_TEMPERATURE_THRESHOLD_FLAG_ADAPTER_REQUEST: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const STORAGE_TEMPERATURE_VALUE_NOT_REPORTED: u32 = 32768u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct STORAGE_TIER {
    pub Id: ::windows::core::GUID,
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
impl ::core::fmt::Debug for STORAGE_TIER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STORAGE_TIER").field("Id", &self.Id).field("Name", &self.Name).field("Description", &self.Description).field("Flags", &self.Flags).field("ProvisionedCapacity", &self.ProvisionedCapacity).field("MediaType", &self.MediaType).field("Class", &self.Class).finish()
    }
}
unsafe impl ::windows::core::Abi for STORAGE_TIER {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for STORAGE_TIER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<STORAGE_TIER>()) == 0 }
    }
}
impl ::core::cmp::Eq for STORAGE_TIER {}
impl ::core::default::Default for STORAGE_TIER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct STORAGE_TIER_CLASS(pub i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageTierClassUnspecified: STORAGE_TIER_CLASS = STORAGE_TIER_CLASS(0i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageTierClassCapacity: STORAGE_TIER_CLASS = STORAGE_TIER_CLASS(1i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageTierClassPerformance: STORAGE_TIER_CLASS = STORAGE_TIER_CLASS(2i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageTierClassMax: STORAGE_TIER_CLASS = STORAGE_TIER_CLASS(3i32);
impl ::core::marker::Copy for STORAGE_TIER_CLASS {}
impl ::core::clone::Clone for STORAGE_TIER_CLASS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for STORAGE_TIER_CLASS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for STORAGE_TIER_CLASS {
    type Abi = Self;
}
impl ::core::fmt::Debug for STORAGE_TIER_CLASS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("STORAGE_TIER_CLASS").field(&self.0).finish()
    }
}
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
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct STORAGE_TIER_MEDIA_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageTierMediaTypeUnspecified: STORAGE_TIER_MEDIA_TYPE = STORAGE_TIER_MEDIA_TYPE(0i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageTierMediaTypeDisk: STORAGE_TIER_MEDIA_TYPE = STORAGE_TIER_MEDIA_TYPE(1i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageTierMediaTypeSsd: STORAGE_TIER_MEDIA_TYPE = STORAGE_TIER_MEDIA_TYPE(2i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageTierMediaTypeScm: STORAGE_TIER_MEDIA_TYPE = STORAGE_TIER_MEDIA_TYPE(4i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const StorageTierMediaTypeMax: STORAGE_TIER_MEDIA_TYPE = STORAGE_TIER_MEDIA_TYPE(5i32);
impl ::core::marker::Copy for STORAGE_TIER_MEDIA_TYPE {}
impl ::core::clone::Clone for STORAGE_TIER_MEDIA_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for STORAGE_TIER_MEDIA_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for STORAGE_TIER_MEDIA_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for STORAGE_TIER_MEDIA_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("STORAGE_TIER_MEDIA_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const STORAGE_TIER_NAME_LENGTH: u32 = 256u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct STORAGE_TIER_REGION {
    pub TierId: ::windows::core::GUID,
    pub Offset: u64,
    pub Length: u64,
}
impl ::core::marker::Copy for STORAGE_TIER_REGION {}
impl ::core::clone::Clone for STORAGE_TIER_REGION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for STORAGE_TIER_REGION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STORAGE_TIER_REGION").field("TierId", &self.TierId).field("Offset", &self.Offset).field("Length", &self.Length).finish()
    }
}
unsafe impl ::windows::core::Abi for STORAGE_TIER_REGION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for STORAGE_TIER_REGION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<STORAGE_TIER_REGION>()) == 0 }
    }
}
impl ::core::cmp::Eq for STORAGE_TIER_REGION {}
impl ::core::default::Default for STORAGE_TIER_REGION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for STORAGE_WRITE_CACHE_PROPERTY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STORAGE_WRITE_CACHE_PROPERTY")
            .field("Version", &self.Version)
            .field("Size", &self.Size)
            .field("WriteCacheType", &self.WriteCacheType)
            .field("WriteCacheEnabled", &self.WriteCacheEnabled)
            .field("WriteCacheChangeable", &self.WriteCacheChangeable)
            .field("WriteThroughSupported", &self.WriteThroughSupported)
            .field("FlushCacheSupported", &self.FlushCacheSupported)
            .field("UserDefinedPowerProtection", &self.UserDefinedPowerProtection)
            .field("NVCacheEnabled", &self.NVCacheEnabled)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for STORAGE_WRITE_CACHE_PROPERTY {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for STORAGE_WRITE_CACHE_PROPERTY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<STORAGE_WRITE_CACHE_PROPERTY>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for STORAGE_WRITE_CACHE_PROPERTY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for STORAGE_WRITE_CACHE_PROPERTY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for STORAGE_ZONED_DEVICE_DESCRIPTOR {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for STORAGE_ZONED_DEVICE_DESCRIPTOR {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<STORAGE_ZONED_DEVICE_DESCRIPTOR>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for STORAGE_ZONED_DEVICE_DESCRIPTOR {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for STORAGE_ZONED_DEVICE_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for STORAGE_ZONED_DEVICE_DESCRIPTOR_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for STORAGE_ZONED_DEVICE_DESCRIPTOR_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<STORAGE_ZONED_DEVICE_DESCRIPTOR_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for STORAGE_ZONED_DEVICE_DESCRIPTOR_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for STORAGE_ZONED_DEVICE_DESCRIPTOR_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for STORAGE_ZONED_DEVICE_DESCRIPTOR_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STORAGE_ZONED_DEVICE_DESCRIPTOR_0_0").field("OptimalOpenZoneCount", &self.OptimalOpenZoneCount).field("Reserved", &self.Reserved).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for STORAGE_ZONED_DEVICE_DESCRIPTOR_0_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for STORAGE_ZONED_DEVICE_DESCRIPTOR_0_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<STORAGE_ZONED_DEVICE_DESCRIPTOR_0_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for STORAGE_ZONED_DEVICE_DESCRIPTOR_0_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for STORAGE_ZONED_DEVICE_DESCRIPTOR_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for STORAGE_ZONED_DEVICE_DESCRIPTOR_0_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STORAGE_ZONED_DEVICE_DESCRIPTOR_0_1").field("MaxOpenZoneCount", &self.MaxOpenZoneCount).field("UnrestrictedRead", &self.UnrestrictedRead).field("Reserved", &self.Reserved).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for STORAGE_ZONED_DEVICE_DESCRIPTOR_0_1 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for STORAGE_ZONED_DEVICE_DESCRIPTOR_0_1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<STORAGE_ZONED_DEVICE_DESCRIPTOR_0_1>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for STORAGE_ZONED_DEVICE_DESCRIPTOR_0_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for STORAGE_ZONED_DEVICE_DESCRIPTOR_0_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct STORAGE_ZONED_DEVICE_TYPES(pub i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ZonedDeviceTypeUnknown: STORAGE_ZONED_DEVICE_TYPES = STORAGE_ZONED_DEVICE_TYPES(0i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ZonedDeviceTypeHostManaged: STORAGE_ZONED_DEVICE_TYPES = STORAGE_ZONED_DEVICE_TYPES(1i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ZonedDeviceTypeHostAware: STORAGE_ZONED_DEVICE_TYPES = STORAGE_ZONED_DEVICE_TYPES(2i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ZonedDeviceTypeDeviceManaged: STORAGE_ZONED_DEVICE_TYPES = STORAGE_ZONED_DEVICE_TYPES(3i32);
impl ::core::marker::Copy for STORAGE_ZONED_DEVICE_TYPES {}
impl ::core::clone::Clone for STORAGE_ZONED_DEVICE_TYPES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for STORAGE_ZONED_DEVICE_TYPES {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for STORAGE_ZONED_DEVICE_TYPES {
    type Abi = Self;
}
impl ::core::fmt::Debug for STORAGE_ZONED_DEVICE_TYPES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("STORAGE_ZONED_DEVICE_TYPES").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct STORAGE_ZONES_ATTRIBUTES(pub i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ZonesAttributeTypeAndLengthMayDifferent: STORAGE_ZONES_ATTRIBUTES = STORAGE_ZONES_ATTRIBUTES(0i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ZonesAttributeTypeSameLengthSame: STORAGE_ZONES_ATTRIBUTES = STORAGE_ZONES_ATTRIBUTES(1i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ZonesAttributeTypeSameLastZoneLengthDifferent: STORAGE_ZONES_ATTRIBUTES = STORAGE_ZONES_ATTRIBUTES(2i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ZonesAttributeTypeMayDifferentLengthSame: STORAGE_ZONES_ATTRIBUTES = STORAGE_ZONES_ATTRIBUTES(3i32);
impl ::core::marker::Copy for STORAGE_ZONES_ATTRIBUTES {}
impl ::core::clone::Clone for STORAGE_ZONES_ATTRIBUTES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for STORAGE_ZONES_ATTRIBUTES {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for STORAGE_ZONES_ATTRIBUTES {
    type Abi = Self;
}
impl ::core::fmt::Debug for STORAGE_ZONES_ATTRIBUTES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("STORAGE_ZONES_ATTRIBUTES").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct STORAGE_ZONE_CONDITION(pub i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ZoneConditionConventional: STORAGE_ZONE_CONDITION = STORAGE_ZONE_CONDITION(0i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ZoneConditionEmpty: STORAGE_ZONE_CONDITION = STORAGE_ZONE_CONDITION(1i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ZoneConditionImplicitlyOpened: STORAGE_ZONE_CONDITION = STORAGE_ZONE_CONDITION(2i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ZoneConditionExplicitlyOpened: STORAGE_ZONE_CONDITION = STORAGE_ZONE_CONDITION(3i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ZoneConditionClosed: STORAGE_ZONE_CONDITION = STORAGE_ZONE_CONDITION(4i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ZoneConditionReadOnly: STORAGE_ZONE_CONDITION = STORAGE_ZONE_CONDITION(13i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ZoneConditionFull: STORAGE_ZONE_CONDITION = STORAGE_ZONE_CONDITION(14i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ZoneConditionOffline: STORAGE_ZONE_CONDITION = STORAGE_ZONE_CONDITION(15i32);
impl ::core::marker::Copy for STORAGE_ZONE_CONDITION {}
impl ::core::clone::Clone for STORAGE_ZONE_CONDITION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for STORAGE_ZONE_CONDITION {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for STORAGE_ZONE_CONDITION {
    type Abi = Self;
}
impl ::core::fmt::Debug for STORAGE_ZONE_CONDITION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("STORAGE_ZONE_CONDITION").field(&self.0).finish()
    }
}
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
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for STORAGE_ZONE_DESCRIPTOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STORAGE_ZONE_DESCRIPTOR").field("Size", &self.Size).field("ZoneType", &self.ZoneType).field("ZoneCondition", &self.ZoneCondition).field("ResetWritePointerRecommend", &self.ResetWritePointerRecommend).field("Reserved0", &self.Reserved0).field("ZoneSize", &self.ZoneSize).field("WritePointerOffset", &self.WritePointerOffset).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for STORAGE_ZONE_DESCRIPTOR {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for STORAGE_ZONE_DESCRIPTOR {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<STORAGE_ZONE_DESCRIPTOR>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for STORAGE_ZONE_DESCRIPTOR {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for STORAGE_ZONE_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for STORAGE_ZONE_GROUP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STORAGE_ZONE_GROUP").field("ZoneCount", &self.ZoneCount).field("ZoneType", &self.ZoneType).field("ZoneSize", &self.ZoneSize).finish()
    }
}
unsafe impl ::windows::core::Abi for STORAGE_ZONE_GROUP {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for STORAGE_ZONE_GROUP {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<STORAGE_ZONE_GROUP>()) == 0 }
    }
}
impl ::core::cmp::Eq for STORAGE_ZONE_GROUP {}
impl ::core::default::Default for STORAGE_ZONE_GROUP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct STORAGE_ZONE_TYPES(pub i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ZoneTypeUnknown: STORAGE_ZONE_TYPES = STORAGE_ZONE_TYPES(0i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ZoneTypeConventional: STORAGE_ZONE_TYPES = STORAGE_ZONE_TYPES(1i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ZoneTypeSequentialWriteRequired: STORAGE_ZONE_TYPES = STORAGE_ZONE_TYPES(2i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ZoneTypeSequentialWritePreferred: STORAGE_ZONE_TYPES = STORAGE_ZONE_TYPES(3i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const ZoneTypeMax: STORAGE_ZONE_TYPES = STORAGE_ZONE_TYPES(4i32);
impl ::core::marker::Copy for STORAGE_ZONE_TYPES {}
impl ::core::clone::Clone for STORAGE_ZONE_TYPES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for STORAGE_ZONE_TYPES {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for STORAGE_ZONE_TYPES {
    type Abi = Self;
}
impl ::core::fmt::Debug for STORAGE_ZONE_TYPES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("STORAGE_ZONE_TYPES").field(&self.0).finish()
    }
}
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
impl ::core::fmt::Debug for STREAMS_ASSOCIATE_ID_INPUT_BUFFER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STREAMS_ASSOCIATE_ID_INPUT_BUFFER").field("Flags", &self.Flags).field("StreamId", &self.StreamId).finish()
    }
}
unsafe impl ::windows::core::Abi for STREAMS_ASSOCIATE_ID_INPUT_BUFFER {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for STREAMS_ASSOCIATE_ID_INPUT_BUFFER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<STREAMS_ASSOCIATE_ID_INPUT_BUFFER>()) == 0 }
    }
}
impl ::core::cmp::Eq for STREAMS_ASSOCIATE_ID_INPUT_BUFFER {}
impl ::core::default::Default for STREAMS_ASSOCIATE_ID_INPUT_BUFFER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for STREAMS_QUERY_ID_OUTPUT_BUFFER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STREAMS_QUERY_ID_OUTPUT_BUFFER").field("StreamId", &self.StreamId).finish()
    }
}
unsafe impl ::windows::core::Abi for STREAMS_QUERY_ID_OUTPUT_BUFFER {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for STREAMS_QUERY_ID_OUTPUT_BUFFER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<STREAMS_QUERY_ID_OUTPUT_BUFFER>()) == 0 }
    }
}
impl ::core::cmp::Eq for STREAMS_QUERY_ID_OUTPUT_BUFFER {}
impl ::core::default::Default for STREAMS_QUERY_ID_OUTPUT_BUFFER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for STREAMS_QUERY_PARAMETERS_OUTPUT_BUFFER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STREAMS_QUERY_PARAMETERS_OUTPUT_BUFFER").field("OptimalWriteSize", &self.OptimalWriteSize).field("StreamGranularitySize", &self.StreamGranularitySize).field("StreamIdMin", &self.StreamIdMin).field("StreamIdMax", &self.StreamIdMax).finish()
    }
}
unsafe impl ::windows::core::Abi for STREAMS_QUERY_PARAMETERS_OUTPUT_BUFFER {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for STREAMS_QUERY_PARAMETERS_OUTPUT_BUFFER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<STREAMS_QUERY_PARAMETERS_OUTPUT_BUFFER>()) == 0 }
    }
}
impl ::core::cmp::Eq for STREAMS_QUERY_PARAMETERS_OUTPUT_BUFFER {}
impl ::core::default::Default for STREAMS_QUERY_PARAMETERS_OUTPUT_BUFFER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
unsafe impl ::windows::core::Abi for STREAM_EXTENT_ENTRY {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for STREAM_EXTENT_ENTRY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<STREAM_EXTENT_ENTRY>()) == 0 }
    }
}
impl ::core::cmp::Eq for STREAM_EXTENT_ENTRY {}
impl ::core::default::Default for STREAM_EXTENT_ENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
unsafe impl ::windows::core::Abi for STREAM_EXTENT_ENTRY_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for STREAM_EXTENT_ENTRY_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<STREAM_EXTENT_ENTRY_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for STREAM_EXTENT_ENTRY_0 {}
impl ::core::default::Default for STREAM_EXTENT_ENTRY_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
unsafe impl ::windows::core::Abi for STREAM_INFORMATION_ENTRY {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for STREAM_INFORMATION_ENTRY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<STREAM_INFORMATION_ENTRY>()) == 0 }
    }
}
impl ::core::cmp::Eq for STREAM_INFORMATION_ENTRY {}
impl ::core::default::Default for STREAM_INFORMATION_ENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
unsafe impl ::windows::core::Abi for STREAM_INFORMATION_ENTRY_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for STREAM_INFORMATION_ENTRY_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<STREAM_INFORMATION_ENTRY_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for STREAM_INFORMATION_ENTRY_0 {}
impl ::core::default::Default for STREAM_INFORMATION_ENTRY_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for STREAM_INFORMATION_ENTRY_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STREAM_INFORMATION_ENTRY_0_0").field("Length", &self.Length).field("Flags", &self.Flags).field("Reserved", &self.Reserved).field("Vdl", &self.Vdl).finish()
    }
}
unsafe impl ::windows::core::Abi for STREAM_INFORMATION_ENTRY_0_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for STREAM_INFORMATION_ENTRY_0_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<STREAM_INFORMATION_ENTRY_0_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for STREAM_INFORMATION_ENTRY_0_0 {}
impl ::core::default::Default for STREAM_INFORMATION_ENTRY_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for STREAM_INFORMATION_ENTRY_0_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STREAM_INFORMATION_ENTRY_0_1").field("Class", &self.Class).field("Flags", &self.Flags).finish()
    }
}
unsafe impl ::windows::core::Abi for STREAM_INFORMATION_ENTRY_0_1 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for STREAM_INFORMATION_ENTRY_0_1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<STREAM_INFORMATION_ENTRY_0_1>()) == 0 }
    }
}
impl ::core::cmp::Eq for STREAM_INFORMATION_ENTRY_0_1 {}
impl ::core::default::Default for STREAM_INFORMATION_ENTRY_0_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for STREAM_INFORMATION_ENTRY_0_2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STREAM_INFORMATION_ENTRY_0_2").field("Length", &self.Length).field("Flags", &self.Flags).field("EaSize", &self.EaSize).field("EaInformationOffset", &self.EaInformationOffset).finish()
    }
}
unsafe impl ::windows::core::Abi for STREAM_INFORMATION_ENTRY_0_2 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for STREAM_INFORMATION_ENTRY_0_2 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<STREAM_INFORMATION_ENTRY_0_2>()) == 0 }
    }
}
impl ::core::cmp::Eq for STREAM_INFORMATION_ENTRY_0_2 {}
impl ::core::default::Default for STREAM_INFORMATION_ENTRY_0_2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for STREAM_INFORMATION_ENTRY_0_3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STREAM_INFORMATION_ENTRY_0_3").field("Length", &self.Length).field("Flags", &self.Flags).field("ReparseDataSize", &self.ReparseDataSize).field("ReparseDataOffset", &self.ReparseDataOffset).finish()
    }
}
unsafe impl ::windows::core::Abi for STREAM_INFORMATION_ENTRY_0_3 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for STREAM_INFORMATION_ENTRY_0_3 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<STREAM_INFORMATION_ENTRY_0_3>()) == 0 }
    }
}
impl ::core::cmp::Eq for STREAM_INFORMATION_ENTRY_0_3 {}
impl ::core::default::Default for STREAM_INFORMATION_ENTRY_0_3 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for STREAM_LAYOUT_ENTRY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STREAM_LAYOUT_ENTRY")
            .field("Version", &self.Version)
            .field("NextStreamOffset", &self.NextStreamOffset)
            .field("Flags", &self.Flags)
            .field("ExtentInformationOffset", &self.ExtentInformationOffset)
            .field("AllocationSize", &self.AllocationSize)
            .field("EndOfFile", &self.EndOfFile)
            .field("StreamInformationOffset", &self.StreamInformationOffset)
            .field("AttributeTypeCode", &self.AttributeTypeCode)
            .field("AttributeFlags", &self.AttributeFlags)
            .field("StreamIdentifierLength", &self.StreamIdentifierLength)
            .field("StreamIdentifier", &self.StreamIdentifier)
            .finish()
    }
}
unsafe impl ::windows::core::Abi for STREAM_LAYOUT_ENTRY {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for STREAM_LAYOUT_ENTRY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<STREAM_LAYOUT_ENTRY>()) == 0 }
    }
}
impl ::core::cmp::Eq for STREAM_LAYOUT_ENTRY {}
impl ::core::default::Default for STREAM_LAYOUT_ENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for TAPE_GET_STATISTICS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TAPE_GET_STATISTICS").field("Operation", &self.Operation).finish()
    }
}
unsafe impl ::windows::core::Abi for TAPE_GET_STATISTICS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for TAPE_GET_STATISTICS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TAPE_GET_STATISTICS>()) == 0 }
    }
}
impl ::core::cmp::Eq for TAPE_GET_STATISTICS {}
impl ::core::default::Default for TAPE_GET_STATISTICS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for TAPE_STATISTICS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TAPE_STATISTICS").field("Version", &self.Version).field("Flags", &self.Flags).field("RecoveredWrites", &self.RecoveredWrites).field("UnrecoveredWrites", &self.UnrecoveredWrites).field("RecoveredReads", &self.RecoveredReads).field("UnrecoveredReads", &self.UnrecoveredReads).field("CompressionRatioReads", &self.CompressionRatioReads).field("CompressionRatioWrites", &self.CompressionRatioWrites).finish()
    }
}
unsafe impl ::windows::core::Abi for TAPE_STATISTICS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for TAPE_STATISTICS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TAPE_STATISTICS>()) == 0 }
    }
}
impl ::core::cmp::Eq for TAPE_STATISTICS {}
impl ::core::default::Default for TAPE_STATISTICS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for TXFS_CREATE_MINIVERSION_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TXFS_CREATE_MINIVERSION_INFO").field("StructureVersion", &self.StructureVersion).field("StructureLength", &self.StructureLength).field("BaseVersion", &self.BaseVersion).field("MiniVersion", &self.MiniVersion).finish()
    }
}
unsafe impl ::windows::core::Abi for TXFS_CREATE_MINIVERSION_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for TXFS_CREATE_MINIVERSION_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TXFS_CREATE_MINIVERSION_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for TXFS_CREATE_MINIVERSION_INFO {}
impl ::core::default::Default for TXFS_CREATE_MINIVERSION_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct TXFS_GET_METADATA_INFO_OUT {
    pub TxfFileId: TXFS_GET_METADATA_INFO_OUT_0,
    pub LockingTransaction: ::windows::core::GUID,
    pub LastLsn: u64,
    pub TransactionState: u32,
}
impl ::core::marker::Copy for TXFS_GET_METADATA_INFO_OUT {}
impl ::core::clone::Clone for TXFS_GET_METADATA_INFO_OUT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TXFS_GET_METADATA_INFO_OUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TXFS_GET_METADATA_INFO_OUT").field("TxfFileId", &self.TxfFileId).field("LockingTransaction", &self.LockingTransaction).field("LastLsn", &self.LastLsn).field("TransactionState", &self.TransactionState).finish()
    }
}
unsafe impl ::windows::core::Abi for TXFS_GET_METADATA_INFO_OUT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for TXFS_GET_METADATA_INFO_OUT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TXFS_GET_METADATA_INFO_OUT>()) == 0 }
    }
}
impl ::core::cmp::Eq for TXFS_GET_METADATA_INFO_OUT {}
impl ::core::default::Default for TXFS_GET_METADATA_INFO_OUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for TXFS_GET_METADATA_INFO_OUT_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TXFS_GET_METADATA_INFO_OUT_0").field("LowPart", &self.LowPart).field("HighPart", &self.HighPart).finish()
    }
}
unsafe impl ::windows::core::Abi for TXFS_GET_METADATA_INFO_OUT_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for TXFS_GET_METADATA_INFO_OUT_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TXFS_GET_METADATA_INFO_OUT_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for TXFS_GET_METADATA_INFO_OUT_0 {}
impl ::core::default::Default for TXFS_GET_METADATA_INFO_OUT_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for TXFS_GET_TRANSACTED_VERSION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TXFS_GET_TRANSACTED_VERSION").field("ThisBaseVersion", &self.ThisBaseVersion).field("LatestVersion", &self.LatestVersion).field("ThisMiniVersion", &self.ThisMiniVersion).field("FirstMiniVersion", &self.FirstMiniVersion).field("LatestMiniVersion", &self.LatestMiniVersion).finish()
    }
}
unsafe impl ::windows::core::Abi for TXFS_GET_TRANSACTED_VERSION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for TXFS_GET_TRANSACTED_VERSION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TXFS_GET_TRANSACTED_VERSION>()) == 0 }
    }
}
impl ::core::cmp::Eq for TXFS_GET_TRANSACTED_VERSION {}
impl ::core::default::Default for TXFS_GET_TRANSACTED_VERSION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for TXFS_LIST_TRANSACTIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TXFS_LIST_TRANSACTIONS").field("NumberOfTransactions", &self.NumberOfTransactions).field("BufferSizeRequired", &self.BufferSizeRequired).finish()
    }
}
unsafe impl ::windows::core::Abi for TXFS_LIST_TRANSACTIONS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for TXFS_LIST_TRANSACTIONS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TXFS_LIST_TRANSACTIONS>()) == 0 }
    }
}
impl ::core::cmp::Eq for TXFS_LIST_TRANSACTIONS {}
impl ::core::default::Default for TXFS_LIST_TRANSACTIONS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct TXFS_LIST_TRANSACTIONS_ENTRY {
    pub TransactionId: ::windows::core::GUID,
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
impl ::core::fmt::Debug for TXFS_LIST_TRANSACTIONS_ENTRY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TXFS_LIST_TRANSACTIONS_ENTRY").field("TransactionId", &self.TransactionId).field("TransactionState", &self.TransactionState).field("Reserved1", &self.Reserved1).field("Reserved2", &self.Reserved2).field("Reserved3", &self.Reserved3).finish()
    }
}
unsafe impl ::windows::core::Abi for TXFS_LIST_TRANSACTIONS_ENTRY {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for TXFS_LIST_TRANSACTIONS_ENTRY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TXFS_LIST_TRANSACTIONS_ENTRY>()) == 0 }
    }
}
impl ::core::cmp::Eq for TXFS_LIST_TRANSACTIONS_ENTRY {}
impl ::core::default::Default for TXFS_LIST_TRANSACTIONS_ENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct TXFS_LIST_TRANSACTION_LOCKED_FILES {
    pub KtmTransaction: ::windows::core::GUID,
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
impl ::core::fmt::Debug for TXFS_LIST_TRANSACTION_LOCKED_FILES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TXFS_LIST_TRANSACTION_LOCKED_FILES").field("KtmTransaction", &self.KtmTransaction).field("NumberOfFiles", &self.NumberOfFiles).field("BufferSizeRequired", &self.BufferSizeRequired).field("Offset", &self.Offset).finish()
    }
}
unsafe impl ::windows::core::Abi for TXFS_LIST_TRANSACTION_LOCKED_FILES {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for TXFS_LIST_TRANSACTION_LOCKED_FILES {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TXFS_LIST_TRANSACTION_LOCKED_FILES>()) == 0 }
    }
}
impl ::core::cmp::Eq for TXFS_LIST_TRANSACTION_LOCKED_FILES {}
impl ::core::default::Default for TXFS_LIST_TRANSACTION_LOCKED_FILES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for TXFS_LIST_TRANSACTION_LOCKED_FILES_ENTRY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TXFS_LIST_TRANSACTION_LOCKED_FILES_ENTRY").field("Offset", &self.Offset).field("NameFlags", &self.NameFlags).field("FileId", &self.FileId).field("Reserved1", &self.Reserved1).field("Reserved2", &self.Reserved2).field("Reserved3", &self.Reserved3).field("FileName", &self.FileName).finish()
    }
}
unsafe impl ::windows::core::Abi for TXFS_LIST_TRANSACTION_LOCKED_FILES_ENTRY {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for TXFS_LIST_TRANSACTION_LOCKED_FILES_ENTRY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TXFS_LIST_TRANSACTION_LOCKED_FILES_ENTRY>()) == 0 }
    }
}
impl ::core::cmp::Eq for TXFS_LIST_TRANSACTION_LOCKED_FILES_ENTRY {}
impl ::core::default::Default for TXFS_LIST_TRANSACTION_LOCKED_FILES_ENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for TXFS_MODIFY_RM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TXFS_MODIFY_RM").field("Flags", &self.Flags).field("LogContainerCountMax", &self.LogContainerCountMax).field("LogContainerCountMin", &self.LogContainerCountMin).field("LogContainerCount", &self.LogContainerCount).field("LogGrowthIncrement", &self.LogGrowthIncrement).field("LogAutoShrinkPercentage", &self.LogAutoShrinkPercentage).field("Reserved", &self.Reserved).field("LoggingMode", &self.LoggingMode).finish()
    }
}
unsafe impl ::windows::core::Abi for TXFS_MODIFY_RM {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for TXFS_MODIFY_RM {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TXFS_MODIFY_RM>()) == 0 }
    }
}
impl ::core::cmp::Eq for TXFS_MODIFY_RM {}
impl ::core::default::Default for TXFS_MODIFY_RM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
    pub RMName: ::windows::core::GUID,
    pub TmLogPathOffset: u32,
}
impl ::core::marker::Copy for TXFS_QUERY_RM_INFORMATION {}
impl ::core::clone::Clone for TXFS_QUERY_RM_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TXFS_QUERY_RM_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TXFS_QUERY_RM_INFORMATION")
            .field("BytesRequired", &self.BytesRequired)
            .field("TailLsn", &self.TailLsn)
            .field("CurrentLsn", &self.CurrentLsn)
            .field("ArchiveTailLsn", &self.ArchiveTailLsn)
            .field("LogContainerSize", &self.LogContainerSize)
            .field("HighestVirtualClock", &self.HighestVirtualClock)
            .field("LogContainerCount", &self.LogContainerCount)
            .field("LogContainerCountMax", &self.LogContainerCountMax)
            .field("LogContainerCountMin", &self.LogContainerCountMin)
            .field("LogGrowthIncrement", &self.LogGrowthIncrement)
            .field("LogAutoShrinkPercentage", &self.LogAutoShrinkPercentage)
            .field("Flags", &self.Flags)
            .field("LoggingMode", &self.LoggingMode)
            .field("Reserved", &self.Reserved)
            .field("RmState", &self.RmState)
            .field("LogCapacity", &self.LogCapacity)
            .field("LogFree", &self.LogFree)
            .field("TopsSize", &self.TopsSize)
            .field("TopsUsed", &self.TopsUsed)
            .field("TransactionCount", &self.TransactionCount)
            .field("OnePCCount", &self.OnePCCount)
            .field("TwoPCCount", &self.TwoPCCount)
            .field("NumberLogFileFull", &self.NumberLogFileFull)
            .field("OldestTransactionAge", &self.OldestTransactionAge)
            .field("RMName", &self.RMName)
            .field("TmLogPathOffset", &self.TmLogPathOffset)
            .finish()
    }
}
unsafe impl ::windows::core::Abi for TXFS_QUERY_RM_INFORMATION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for TXFS_QUERY_RM_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TXFS_QUERY_RM_INFORMATION>()) == 0 }
    }
}
impl ::core::cmp::Eq for TXFS_QUERY_RM_INFORMATION {}
impl ::core::default::Default for TXFS_QUERY_RM_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
unsafe impl ::windows::core::Abi for TXFS_READ_BACKUP_INFORMATION_OUT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for TXFS_READ_BACKUP_INFORMATION_OUT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TXFS_READ_BACKUP_INFORMATION_OUT>()) == 0 }
    }
}
impl ::core::cmp::Eq for TXFS_READ_BACKUP_INFORMATION_OUT {}
impl ::core::default::Default for TXFS_READ_BACKUP_INFORMATION_OUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
unsafe impl ::windows::core::Abi for TXFS_READ_BACKUP_INFORMATION_OUT_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for TXFS_READ_BACKUP_INFORMATION_OUT_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TXFS_READ_BACKUP_INFORMATION_OUT_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for TXFS_READ_BACKUP_INFORMATION_OUT_0 {}
impl ::core::default::Default for TXFS_READ_BACKUP_INFORMATION_OUT_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct TXFS_RMF_LAGS(pub u32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const TXFS_RM_FLAG_LOGGING_MODE: TXFS_RMF_LAGS = TXFS_RMF_LAGS(1u32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const TXFS_RM_FLAG_RENAME_RM: TXFS_RMF_LAGS = TXFS_RMF_LAGS(2u32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const TXFS_RM_FLAG_LOG_CONTAINER_COUNT_MAX: TXFS_RMF_LAGS = TXFS_RMF_LAGS(4u32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const TXFS_RM_FLAG_LOG_CONTAINER_COUNT_MIN: TXFS_RMF_LAGS = TXFS_RMF_LAGS(8u32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const TXFS_RM_FLAG_LOG_GROWTH_INCREMENT_NUM_CONTAINERS: TXFS_RMF_LAGS = TXFS_RMF_LAGS(16u32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const TXFS_RM_FLAG_LOG_GROWTH_INCREMENT_PERCENT: TXFS_RMF_LAGS = TXFS_RMF_LAGS(32u32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const TXFS_RM_FLAG_LOG_AUTO_SHRINK_PERCENTAGE: TXFS_RMF_LAGS = TXFS_RMF_LAGS(64u32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const TXFS_RM_FLAG_LOG_NO_CONTAINER_COUNT_MAX: TXFS_RMF_LAGS = TXFS_RMF_LAGS(128u32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const TXFS_RM_FLAG_LOG_NO_CONTAINER_COUNT_MIN: TXFS_RMF_LAGS = TXFS_RMF_LAGS(256u32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const TXFS_RM_FLAG_GROW_LOG: TXFS_RMF_LAGS = TXFS_RMF_LAGS(1024u32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const TXFS_RM_FLAG_SHRINK_LOG: TXFS_RMF_LAGS = TXFS_RMF_LAGS(2048u32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const TXFS_RM_FLAG_ENFORCE_MINIMUM_SIZE: TXFS_RMF_LAGS = TXFS_RMF_LAGS(4096u32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const TXFS_RM_FLAG_PRESERVE_CHANGES: TXFS_RMF_LAGS = TXFS_RMF_LAGS(8192u32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const TXFS_RM_FLAG_RESET_RM_AT_NEXT_START: TXFS_RMF_LAGS = TXFS_RMF_LAGS(16384u32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const TXFS_RM_FLAG_DO_NOT_RESET_RM_AT_NEXT_START: TXFS_RMF_LAGS = TXFS_RMF_LAGS(32768u32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const TXFS_RM_FLAG_PREFER_CONSISTENCY: TXFS_RMF_LAGS = TXFS_RMF_LAGS(65536u32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const TXFS_RM_FLAG_PREFER_AVAILABILITY: TXFS_RMF_LAGS = TXFS_RMF_LAGS(131072u32);
impl ::core::marker::Copy for TXFS_RMF_LAGS {}
impl ::core::clone::Clone for TXFS_RMF_LAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TXFS_RMF_LAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for TXFS_RMF_LAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for TXFS_RMF_LAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TXFS_RMF_LAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for TXFS_RMF_LAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for TXFS_RMF_LAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for TXFS_RMF_LAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for TXFS_RMF_LAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for TXFS_RMF_LAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
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
impl ::core::fmt::Debug for TXFS_ROLLFORWARD_REDO_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TXFS_ROLLFORWARD_REDO_INFORMATION").field("LastVirtualClock", &self.LastVirtualClock).field("LastRedoLsn", &self.LastRedoLsn).field("HighestRecoveryLsn", &self.HighestRecoveryLsn).field("Flags", &self.Flags).finish()
    }
}
unsafe impl ::windows::core::Abi for TXFS_ROLLFORWARD_REDO_INFORMATION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for TXFS_ROLLFORWARD_REDO_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TXFS_ROLLFORWARD_REDO_INFORMATION>()) == 0 }
    }
}
impl ::core::cmp::Eq for TXFS_ROLLFORWARD_REDO_INFORMATION {}
impl ::core::default::Default for TXFS_ROLLFORWARD_REDO_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for TXFS_SAVEPOINT_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TXFS_SAVEPOINT_INFORMATION").field("KtmTransaction", &self.KtmTransaction).field("ActionCode", &self.ActionCode).field("SavepointId", &self.SavepointId).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for TXFS_SAVEPOINT_INFORMATION {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for TXFS_SAVEPOINT_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TXFS_SAVEPOINT_INFORMATION>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for TXFS_SAVEPOINT_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for TXFS_SAVEPOINT_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for TXFS_START_RM_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TXFS_START_RM_INFORMATION")
            .field("Flags", &self.Flags)
            .field("LogContainerSize", &self.LogContainerSize)
            .field("LogContainerCountMin", &self.LogContainerCountMin)
            .field("LogContainerCountMax", &self.LogContainerCountMax)
            .field("LogGrowthIncrement", &self.LogGrowthIncrement)
            .field("LogAutoShrinkPercentage", &self.LogAutoShrinkPercentage)
            .field("TmLogPathOffset", &self.TmLogPathOffset)
            .field("TmLogPathLength", &self.TmLogPathLength)
            .field("LoggingMode", &self.LoggingMode)
            .field("LogPathLength", &self.LogPathLength)
            .field("Reserved", &self.Reserved)
            .field("LogPath", &self.LogPath)
            .finish()
    }
}
unsafe impl ::windows::core::Abi for TXFS_START_RM_INFORMATION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for TXFS_START_RM_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TXFS_START_RM_INFORMATION>()) == 0 }
    }
}
impl ::core::cmp::Eq for TXFS_START_RM_INFORMATION {}
impl ::core::default::Default for TXFS_START_RM_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for TXFS_TRANSACTION_ACTIVE_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TXFS_TRANSACTION_ACTIVE_INFO").field("TransactionsActiveAtSnapshot", &self.TransactionsActiveAtSnapshot).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for TXFS_TRANSACTION_ACTIVE_INFO {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for TXFS_TRANSACTION_ACTIVE_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TXFS_TRANSACTION_ACTIVE_INFO>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for TXFS_TRANSACTION_ACTIVE_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for TXFS_TRANSACTION_ACTIVE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for TXFS_WRITE_BACKUP_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TXFS_WRITE_BACKUP_INFORMATION").field("Buffer", &self.Buffer).finish()
    }
}
unsafe impl ::windows::core::Abi for TXFS_WRITE_BACKUP_INFORMATION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for TXFS_WRITE_BACKUP_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TXFS_WRITE_BACKUP_INFORMATION>()) == 0 }
    }
}
impl ::core::cmp::Eq for TXFS_WRITE_BACKUP_INFORMATION {}
impl ::core::default::Default for TXFS_WRITE_BACKUP_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct USN_DELETE_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const USN_DELETE_FLAG_DELETE: USN_DELETE_FLAGS = USN_DELETE_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const USN_DELETE_FLAG_NOTIFY: USN_DELETE_FLAGS = USN_DELETE_FLAGS(2u32);
impl ::core::marker::Copy for USN_DELETE_FLAGS {}
impl ::core::clone::Clone for USN_DELETE_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for USN_DELETE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for USN_DELETE_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for USN_DELETE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("USN_DELETE_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for USN_DELETE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for USN_DELETE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for USN_DELETE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for USN_DELETE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for USN_DELETE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
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
impl ::core::fmt::Debug for USN_JOURNAL_DATA_V0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("USN_JOURNAL_DATA_V0").field("UsnJournalID", &self.UsnJournalID).field("FirstUsn", &self.FirstUsn).field("NextUsn", &self.NextUsn).field("LowestValidUsn", &self.LowestValidUsn).field("MaxUsn", &self.MaxUsn).field("MaximumSize", &self.MaximumSize).field("AllocationDelta", &self.AllocationDelta).finish()
    }
}
unsafe impl ::windows::core::Abi for USN_JOURNAL_DATA_V0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for USN_JOURNAL_DATA_V0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<USN_JOURNAL_DATA_V0>()) == 0 }
    }
}
impl ::core::cmp::Eq for USN_JOURNAL_DATA_V0 {}
impl ::core::default::Default for USN_JOURNAL_DATA_V0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for USN_JOURNAL_DATA_V1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("USN_JOURNAL_DATA_V1").field("UsnJournalID", &self.UsnJournalID).field("FirstUsn", &self.FirstUsn).field("NextUsn", &self.NextUsn).field("LowestValidUsn", &self.LowestValidUsn).field("MaxUsn", &self.MaxUsn).field("MaximumSize", &self.MaximumSize).field("AllocationDelta", &self.AllocationDelta).field("MinSupportedMajorVersion", &self.MinSupportedMajorVersion).field("MaxSupportedMajorVersion", &self.MaxSupportedMajorVersion).finish()
    }
}
unsafe impl ::windows::core::Abi for USN_JOURNAL_DATA_V1 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for USN_JOURNAL_DATA_V1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<USN_JOURNAL_DATA_V1>()) == 0 }
    }
}
impl ::core::cmp::Eq for USN_JOURNAL_DATA_V1 {}
impl ::core::default::Default for USN_JOURNAL_DATA_V1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for USN_JOURNAL_DATA_V2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("USN_JOURNAL_DATA_V2")
            .field("UsnJournalID", &self.UsnJournalID)
            .field("FirstUsn", &self.FirstUsn)
            .field("NextUsn", &self.NextUsn)
            .field("LowestValidUsn", &self.LowestValidUsn)
            .field("MaxUsn", &self.MaxUsn)
            .field("MaximumSize", &self.MaximumSize)
            .field("AllocationDelta", &self.AllocationDelta)
            .field("MinSupportedMajorVersion", &self.MinSupportedMajorVersion)
            .field("MaxSupportedMajorVersion", &self.MaxSupportedMajorVersion)
            .field("Flags", &self.Flags)
            .field("RangeTrackChunkSize", &self.RangeTrackChunkSize)
            .field("RangeTrackFileSizeThreshold", &self.RangeTrackFileSizeThreshold)
            .finish()
    }
}
unsafe impl ::windows::core::Abi for USN_JOURNAL_DATA_V2 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for USN_JOURNAL_DATA_V2 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<USN_JOURNAL_DATA_V2>()) == 0 }
    }
}
impl ::core::cmp::Eq for USN_JOURNAL_DATA_V2 {}
impl ::core::default::Default for USN_JOURNAL_DATA_V2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for USN_RANGE_TRACK_OUTPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("USN_RANGE_TRACK_OUTPUT").field("Usn", &self.Usn).finish()
    }
}
unsafe impl ::windows::core::Abi for USN_RANGE_TRACK_OUTPUT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for USN_RANGE_TRACK_OUTPUT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<USN_RANGE_TRACK_OUTPUT>()) == 0 }
    }
}
impl ::core::cmp::Eq for USN_RANGE_TRACK_OUTPUT {}
impl ::core::default::Default for USN_RANGE_TRACK_OUTPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for USN_RECORD_COMMON_HEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("USN_RECORD_COMMON_HEADER").field("RecordLength", &self.RecordLength).field("MajorVersion", &self.MajorVersion).field("MinorVersion", &self.MinorVersion).finish()
    }
}
unsafe impl ::windows::core::Abi for USN_RECORD_COMMON_HEADER {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for USN_RECORD_COMMON_HEADER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<USN_RECORD_COMMON_HEADER>()) == 0 }
    }
}
impl ::core::cmp::Eq for USN_RECORD_COMMON_HEADER {}
impl ::core::default::Default for USN_RECORD_COMMON_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for USN_RECORD_EXTENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("USN_RECORD_EXTENT").field("Offset", &self.Offset).field("Length", &self.Length).finish()
    }
}
unsafe impl ::windows::core::Abi for USN_RECORD_EXTENT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for USN_RECORD_EXTENT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<USN_RECORD_EXTENT>()) == 0 }
    }
}
impl ::core::cmp::Eq for USN_RECORD_EXTENT {}
impl ::core::default::Default for USN_RECORD_EXTENT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
#[cfg(feature = "Win32_Storage_FileSystem")]
unsafe impl ::windows::core::Abi for USN_RECORD_UNION {
    type Abi = Self;
}
#[cfg(feature = "Win32_Storage_FileSystem")]
impl ::core::cmp::PartialEq for USN_RECORD_UNION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<USN_RECORD_UNION>()) == 0 }
    }
}
#[cfg(feature = "Win32_Storage_FileSystem")]
impl ::core::cmp::Eq for USN_RECORD_UNION {}
#[cfg(feature = "Win32_Storage_FileSystem")]
impl ::core::default::Default for USN_RECORD_UNION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for USN_RECORD_V2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("USN_RECORD_V2")
            .field("RecordLength", &self.RecordLength)
            .field("MajorVersion", &self.MajorVersion)
            .field("MinorVersion", &self.MinorVersion)
            .field("FileReferenceNumber", &self.FileReferenceNumber)
            .field("ParentFileReferenceNumber", &self.ParentFileReferenceNumber)
            .field("Usn", &self.Usn)
            .field("TimeStamp", &self.TimeStamp)
            .field("Reason", &self.Reason)
            .field("SourceInfo", &self.SourceInfo)
            .field("SecurityId", &self.SecurityId)
            .field("FileAttributes", &self.FileAttributes)
            .field("FileNameLength", &self.FileNameLength)
            .field("FileNameOffset", &self.FileNameOffset)
            .field("FileName", &self.FileName)
            .finish()
    }
}
unsafe impl ::windows::core::Abi for USN_RECORD_V2 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for USN_RECORD_V2 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<USN_RECORD_V2>()) == 0 }
    }
}
impl ::core::cmp::Eq for USN_RECORD_V2 {}
impl ::core::default::Default for USN_RECORD_V2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
#[cfg(feature = "Win32_Storage_FileSystem")]
impl ::core::fmt::Debug for USN_RECORD_V3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("USN_RECORD_V3")
            .field("RecordLength", &self.RecordLength)
            .field("MajorVersion", &self.MajorVersion)
            .field("MinorVersion", &self.MinorVersion)
            .field("FileReferenceNumber", &self.FileReferenceNumber)
            .field("ParentFileReferenceNumber", &self.ParentFileReferenceNumber)
            .field("Usn", &self.Usn)
            .field("TimeStamp", &self.TimeStamp)
            .field("Reason", &self.Reason)
            .field("SourceInfo", &self.SourceInfo)
            .field("SecurityId", &self.SecurityId)
            .field("FileAttributes", &self.FileAttributes)
            .field("FileNameLength", &self.FileNameLength)
            .field("FileNameOffset", &self.FileNameOffset)
            .field("FileName", &self.FileName)
            .finish()
    }
}
#[cfg(feature = "Win32_Storage_FileSystem")]
unsafe impl ::windows::core::Abi for USN_RECORD_V3 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Storage_FileSystem")]
impl ::core::cmp::PartialEq for USN_RECORD_V3 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<USN_RECORD_V3>()) == 0 }
    }
}
#[cfg(feature = "Win32_Storage_FileSystem")]
impl ::core::cmp::Eq for USN_RECORD_V3 {}
#[cfg(feature = "Win32_Storage_FileSystem")]
impl ::core::default::Default for USN_RECORD_V3 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
#[cfg(feature = "Win32_Storage_FileSystem")]
impl ::core::fmt::Debug for USN_RECORD_V4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("USN_RECORD_V4").field("Header", &self.Header).field("FileReferenceNumber", &self.FileReferenceNumber).field("ParentFileReferenceNumber", &self.ParentFileReferenceNumber).field("Usn", &self.Usn).field("Reason", &self.Reason).field("SourceInfo", &self.SourceInfo).field("RemainingExtents", &self.RemainingExtents).field("NumberOfExtents", &self.NumberOfExtents).field("ExtentSize", &self.ExtentSize).field("Extents", &self.Extents).finish()
    }
}
#[cfg(feature = "Win32_Storage_FileSystem")]
unsafe impl ::windows::core::Abi for USN_RECORD_V4 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Storage_FileSystem")]
impl ::core::cmp::PartialEq for USN_RECORD_V4 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<USN_RECORD_V4>()) == 0 }
    }
}
#[cfg(feature = "Win32_Storage_FileSystem")]
impl ::core::cmp::Eq for USN_RECORD_V4 {}
#[cfg(feature = "Win32_Storage_FileSystem")]
impl ::core::default::Default for USN_RECORD_V4 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct USN_SOURCE_INFO_ID(pub u32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const USN_SOURCE_AUXILIARY_DATA: USN_SOURCE_INFO_ID = USN_SOURCE_INFO_ID(2u32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const USN_SOURCE_DATA_MANAGEMENT: USN_SOURCE_INFO_ID = USN_SOURCE_INFO_ID(1u32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const USN_SOURCE_REPLICATION_MANAGEMENT: USN_SOURCE_INFO_ID = USN_SOURCE_INFO_ID(4u32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const USN_SOURCE_CLIENT_REPLICATION_MANAGEMENT: USN_SOURCE_INFO_ID = USN_SOURCE_INFO_ID(8u32);
impl ::core::marker::Copy for USN_SOURCE_INFO_ID {}
impl ::core::clone::Clone for USN_SOURCE_INFO_ID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for USN_SOURCE_INFO_ID {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for USN_SOURCE_INFO_ID {
    type Abi = Self;
}
impl ::core::fmt::Debug for USN_SOURCE_INFO_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("USN_SOURCE_INFO_ID").field(&self.0).finish()
    }
}
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
impl ::core::fmt::Debug for USN_TRACK_MODIFIED_RANGES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("USN_TRACK_MODIFIED_RANGES").field("Flags", &self.Flags).field("Unused", &self.Unused).field("ChunkSize", &self.ChunkSize).field("FileSizeThreshold", &self.FileSizeThreshold).finish()
    }
}
unsafe impl ::windows::core::Abi for USN_TRACK_MODIFIED_RANGES {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for USN_TRACK_MODIFIED_RANGES {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<USN_TRACK_MODIFIED_RANGES>()) == 0 }
    }
}
impl ::core::cmp::Eq for USN_TRACK_MODIFIED_RANGES {}
impl ::core::default::Default for USN_TRACK_MODIFIED_RANGES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for VERIFY_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VERIFY_INFORMATION").field("StartingOffset", &self.StartingOffset).field("Length", &self.Length).finish()
    }
}
unsafe impl ::windows::core::Abi for VERIFY_INFORMATION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for VERIFY_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<VERIFY_INFORMATION>()) == 0 }
    }
}
impl ::core::cmp::Eq for VERIFY_INFORMATION {}
impl ::core::default::Default for VERIFY_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for VIRTUALIZATION_INSTANCE_INFO_INPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VIRTUALIZATION_INSTANCE_INFO_INPUT").field("NumberOfWorkerThreads", &self.NumberOfWorkerThreads).field("Flags", &self.Flags).finish()
    }
}
unsafe impl ::windows::core::Abi for VIRTUALIZATION_INSTANCE_INFO_INPUT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for VIRTUALIZATION_INSTANCE_INFO_INPUT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<VIRTUALIZATION_INSTANCE_INFO_INPUT>()) == 0 }
    }
}
impl ::core::cmp::Eq for VIRTUALIZATION_INSTANCE_INFO_INPUT {}
impl ::core::default::Default for VIRTUALIZATION_INSTANCE_INFO_INPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for VIRTUALIZATION_INSTANCE_INFO_INPUT_EX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VIRTUALIZATION_INSTANCE_INFO_INPUT_EX").field("HeaderSize", &self.HeaderSize).field("Flags", &self.Flags).field("NotificationInfoSize", &self.NotificationInfoSize).field("NotificationInfoOffset", &self.NotificationInfoOffset).field("ProviderMajorVersion", &self.ProviderMajorVersion).finish()
    }
}
unsafe impl ::windows::core::Abi for VIRTUALIZATION_INSTANCE_INFO_INPUT_EX {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for VIRTUALIZATION_INSTANCE_INFO_INPUT_EX {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<VIRTUALIZATION_INSTANCE_INFO_INPUT_EX>()) == 0 }
    }
}
impl ::core::cmp::Eq for VIRTUALIZATION_INSTANCE_INFO_INPUT_EX {}
impl ::core::default::Default for VIRTUALIZATION_INSTANCE_INFO_INPUT_EX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct VIRTUALIZATION_INSTANCE_INFO_OUTPUT {
    pub VirtualizationInstanceID: ::windows::core::GUID,
}
impl ::core::marker::Copy for VIRTUALIZATION_INSTANCE_INFO_OUTPUT {}
impl ::core::clone::Clone for VIRTUALIZATION_INSTANCE_INFO_OUTPUT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for VIRTUALIZATION_INSTANCE_INFO_OUTPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VIRTUALIZATION_INSTANCE_INFO_OUTPUT").field("VirtualizationInstanceID", &self.VirtualizationInstanceID).finish()
    }
}
unsafe impl ::windows::core::Abi for VIRTUALIZATION_INSTANCE_INFO_OUTPUT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for VIRTUALIZATION_INSTANCE_INFO_OUTPUT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<VIRTUALIZATION_INSTANCE_INFO_OUTPUT>()) == 0 }
    }
}
impl ::core::cmp::Eq for VIRTUALIZATION_INSTANCE_INFO_OUTPUT {}
impl ::core::default::Default for VIRTUALIZATION_INSTANCE_INFO_OUTPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct VIRTUAL_STORAGE_BEHAVIOR_CODE(pub i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const VirtualStorageBehaviorUndefined: VIRTUAL_STORAGE_BEHAVIOR_CODE = VIRTUAL_STORAGE_BEHAVIOR_CODE(0i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const VirtualStorageBehaviorCacheWriteThrough: VIRTUAL_STORAGE_BEHAVIOR_CODE = VIRTUAL_STORAGE_BEHAVIOR_CODE(1i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const VirtualStorageBehaviorCacheWriteBack: VIRTUAL_STORAGE_BEHAVIOR_CODE = VIRTUAL_STORAGE_BEHAVIOR_CODE(2i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const VirtualStorageBehaviorStopIoProcessing: VIRTUAL_STORAGE_BEHAVIOR_CODE = VIRTUAL_STORAGE_BEHAVIOR_CODE(3i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const VirtualStorageBehaviorRestartIoProcessing: VIRTUAL_STORAGE_BEHAVIOR_CODE = VIRTUAL_STORAGE_BEHAVIOR_CODE(4i32);
impl ::core::marker::Copy for VIRTUAL_STORAGE_BEHAVIOR_CODE {}
impl ::core::clone::Clone for VIRTUAL_STORAGE_BEHAVIOR_CODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for VIRTUAL_STORAGE_BEHAVIOR_CODE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for VIRTUAL_STORAGE_BEHAVIOR_CODE {
    type Abi = Self;
}
impl ::core::fmt::Debug for VIRTUAL_STORAGE_BEHAVIOR_CODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VIRTUAL_STORAGE_BEHAVIOR_CODE").field(&self.0).finish()
    }
}
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
impl ::core::fmt::Debug for VIRTUAL_STORAGE_SET_BEHAVIOR_INPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VIRTUAL_STORAGE_SET_BEHAVIOR_INPUT").field("Size", &self.Size).field("BehaviorCode", &self.BehaviorCode).finish()
    }
}
unsafe impl ::windows::core::Abi for VIRTUAL_STORAGE_SET_BEHAVIOR_INPUT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for VIRTUAL_STORAGE_SET_BEHAVIOR_INPUT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<VIRTUAL_STORAGE_SET_BEHAVIOR_INPUT>()) == 0 }
    }
}
impl ::core::cmp::Eq for VIRTUAL_STORAGE_SET_BEHAVIOR_INPUT {}
impl ::core::default::Default for VIRTUAL_STORAGE_SET_BEHAVIOR_INPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for VOLUME_BITMAP_BUFFER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VOLUME_BITMAP_BUFFER").field("StartingLcn", &self.StartingLcn).field("BitmapSize", &self.BitmapSize).field("Buffer", &self.Buffer).finish()
    }
}
unsafe impl ::windows::core::Abi for VOLUME_BITMAP_BUFFER {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for VOLUME_BITMAP_BUFFER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<VOLUME_BITMAP_BUFFER>()) == 0 }
    }
}
impl ::core::cmp::Eq for VOLUME_BITMAP_BUFFER {}
impl ::core::default::Default for VOLUME_BITMAP_BUFFER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for VOLUME_DISK_EXTENTS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VOLUME_DISK_EXTENTS").field("NumberOfDiskExtents", &self.NumberOfDiskExtents).field("Extents", &self.Extents).finish()
    }
}
unsafe impl ::windows::core::Abi for VOLUME_DISK_EXTENTS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for VOLUME_DISK_EXTENTS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<VOLUME_DISK_EXTENTS>()) == 0 }
    }
}
impl ::core::cmp::Eq for VOLUME_DISK_EXTENTS {}
impl ::core::default::Default for VOLUME_DISK_EXTENTS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for VOLUME_GET_GPT_ATTRIBUTES_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VOLUME_GET_GPT_ATTRIBUTES_INFORMATION").field("GptAttributes", &self.GptAttributes).finish()
    }
}
unsafe impl ::windows::core::Abi for VOLUME_GET_GPT_ATTRIBUTES_INFORMATION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for VOLUME_GET_GPT_ATTRIBUTES_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<VOLUME_GET_GPT_ATTRIBUTES_INFORMATION>()) == 0 }
    }
}
impl ::core::cmp::Eq for VOLUME_GET_GPT_ATTRIBUTES_INFORMATION {}
impl ::core::default::Default for VOLUME_GET_GPT_ATTRIBUTES_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for WIM_PROVIDER_ADD_OVERLAY_INPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WIM_PROVIDER_ADD_OVERLAY_INPUT").field("WimType", &self.WimType).field("WimIndex", &self.WimIndex).field("WimFileNameOffset", &self.WimFileNameOffset).field("WimFileNameLength", &self.WimFileNameLength).finish()
    }
}
unsafe impl ::windows::core::Abi for WIM_PROVIDER_ADD_OVERLAY_INPUT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WIM_PROVIDER_ADD_OVERLAY_INPUT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WIM_PROVIDER_ADD_OVERLAY_INPUT>()) == 0 }
    }
}
impl ::core::cmp::Eq for WIM_PROVIDER_ADD_OVERLAY_INPUT {}
impl ::core::default::Default for WIM_PROVIDER_ADD_OVERLAY_INPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for WIM_PROVIDER_EXTERNAL_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WIM_PROVIDER_EXTERNAL_INFO").field("Version", &self.Version).field("Flags", &self.Flags).field("DataSourceId", &self.DataSourceId).field("ResourceHash", &self.ResourceHash).finish()
    }
}
unsafe impl ::windows::core::Abi for WIM_PROVIDER_EXTERNAL_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WIM_PROVIDER_EXTERNAL_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WIM_PROVIDER_EXTERNAL_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for WIM_PROVIDER_EXTERNAL_INFO {}
impl ::core::default::Default for WIM_PROVIDER_EXTERNAL_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub struct WIM_PROVIDER_OVERLAY_ENTRY {
    pub NextEntryOffset: u32,
    pub DataSourceId: i64,
    pub WimGuid: ::windows::core::GUID,
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
impl ::core::fmt::Debug for WIM_PROVIDER_OVERLAY_ENTRY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WIM_PROVIDER_OVERLAY_ENTRY").field("NextEntryOffset", &self.NextEntryOffset).field("DataSourceId", &self.DataSourceId).field("WimGuid", &self.WimGuid).field("WimFileNameOffset", &self.WimFileNameOffset).field("WimType", &self.WimType).field("WimIndex", &self.WimIndex).field("Flags", &self.Flags).finish()
    }
}
unsafe impl ::windows::core::Abi for WIM_PROVIDER_OVERLAY_ENTRY {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WIM_PROVIDER_OVERLAY_ENTRY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WIM_PROVIDER_OVERLAY_ENTRY>()) == 0 }
    }
}
impl ::core::cmp::Eq for WIM_PROVIDER_OVERLAY_ENTRY {}
impl ::core::default::Default for WIM_PROVIDER_OVERLAY_ENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for WIM_PROVIDER_REMOVE_OVERLAY_INPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WIM_PROVIDER_REMOVE_OVERLAY_INPUT").field("DataSourceId", &self.DataSourceId).finish()
    }
}
unsafe impl ::windows::core::Abi for WIM_PROVIDER_REMOVE_OVERLAY_INPUT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WIM_PROVIDER_REMOVE_OVERLAY_INPUT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WIM_PROVIDER_REMOVE_OVERLAY_INPUT>()) == 0 }
    }
}
impl ::core::cmp::Eq for WIM_PROVIDER_REMOVE_OVERLAY_INPUT {}
impl ::core::default::Default for WIM_PROVIDER_REMOVE_OVERLAY_INPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for WIM_PROVIDER_SUSPEND_OVERLAY_INPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WIM_PROVIDER_SUSPEND_OVERLAY_INPUT").field("DataSourceId", &self.DataSourceId).finish()
    }
}
unsafe impl ::windows::core::Abi for WIM_PROVIDER_SUSPEND_OVERLAY_INPUT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WIM_PROVIDER_SUSPEND_OVERLAY_INPUT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WIM_PROVIDER_SUSPEND_OVERLAY_INPUT>()) == 0 }
    }
}
impl ::core::cmp::Eq for WIM_PROVIDER_SUSPEND_OVERLAY_INPUT {}
impl ::core::default::Default for WIM_PROVIDER_SUSPEND_OVERLAY_INPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for WIM_PROVIDER_UPDATE_OVERLAY_INPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WIM_PROVIDER_UPDATE_OVERLAY_INPUT").field("DataSourceId", &self.DataSourceId).field("WimFileNameOffset", &self.WimFileNameOffset).field("WimFileNameLength", &self.WimFileNameLength).finish()
    }
}
unsafe impl ::windows::core::Abi for WIM_PROVIDER_UPDATE_OVERLAY_INPUT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WIM_PROVIDER_UPDATE_OVERLAY_INPUT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WIM_PROVIDER_UPDATE_OVERLAY_INPUT>()) == 0 }
    }
}
impl ::core::cmp::Eq for WIM_PROVIDER_UPDATE_OVERLAY_INPUT {}
impl ::core::default::Default for WIM_PROVIDER_UPDATE_OVERLAY_INPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
#[cfg(feature = "Win32_Storage_FileSystem")]
impl ::core::fmt::Debug for WOF_EXTERNAL_FILE_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WOF_EXTERNAL_FILE_ID").field("FileId", &self.FileId).finish()
    }
}
#[cfg(feature = "Win32_Storage_FileSystem")]
unsafe impl ::windows::core::Abi for WOF_EXTERNAL_FILE_ID {
    type Abi = Self;
}
#[cfg(feature = "Win32_Storage_FileSystem")]
impl ::core::cmp::PartialEq for WOF_EXTERNAL_FILE_ID {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WOF_EXTERNAL_FILE_ID>()) == 0 }
    }
}
#[cfg(feature = "Win32_Storage_FileSystem")]
impl ::core::cmp::Eq for WOF_EXTERNAL_FILE_ID {}
#[cfg(feature = "Win32_Storage_FileSystem")]
impl ::core::default::Default for WOF_EXTERNAL_FILE_ID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for WOF_EXTERNAL_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WOF_EXTERNAL_INFO").field("Version", &self.Version).field("Provider", &self.Provider).finish()
    }
}
unsafe impl ::windows::core::Abi for WOF_EXTERNAL_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WOF_EXTERNAL_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WOF_EXTERNAL_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for WOF_EXTERNAL_INFO {}
impl ::core::default::Default for WOF_EXTERNAL_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for WOF_VERSION_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WOF_VERSION_INFO").field("WofVersion", &self.WofVersion).finish()
    }
}
unsafe impl ::windows::core::Abi for WOF_VERSION_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WOF_VERSION_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WOF_VERSION_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for WOF_VERSION_INFO {}
impl ::core::default::Default for WOF_VERSION_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WRITE_CACHE_CHANGE(pub i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const WriteCacheChangeUnknown: WRITE_CACHE_CHANGE = WRITE_CACHE_CHANGE(0i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const WriteCacheNotChangeable: WRITE_CACHE_CHANGE = WRITE_CACHE_CHANGE(1i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const WriteCacheChangeable: WRITE_CACHE_CHANGE = WRITE_CACHE_CHANGE(2i32);
impl ::core::marker::Copy for WRITE_CACHE_CHANGE {}
impl ::core::clone::Clone for WRITE_CACHE_CHANGE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WRITE_CACHE_CHANGE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WRITE_CACHE_CHANGE {
    type Abi = Self;
}
impl ::core::fmt::Debug for WRITE_CACHE_CHANGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WRITE_CACHE_CHANGE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WRITE_CACHE_ENABLE(pub i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const WriteCacheEnableUnknown: WRITE_CACHE_ENABLE = WRITE_CACHE_ENABLE(0i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const WriteCacheDisabled: WRITE_CACHE_ENABLE = WRITE_CACHE_ENABLE(1i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const WriteCacheEnabled: WRITE_CACHE_ENABLE = WRITE_CACHE_ENABLE(2i32);
impl ::core::marker::Copy for WRITE_CACHE_ENABLE {}
impl ::core::clone::Clone for WRITE_CACHE_ENABLE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WRITE_CACHE_ENABLE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WRITE_CACHE_ENABLE {
    type Abi = Self;
}
impl ::core::fmt::Debug for WRITE_CACHE_ENABLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WRITE_CACHE_ENABLE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WRITE_CACHE_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const WriteCacheTypeUnknown: WRITE_CACHE_TYPE = WRITE_CACHE_TYPE(0i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const WriteCacheTypeNone: WRITE_CACHE_TYPE = WRITE_CACHE_TYPE(1i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const WriteCacheTypeWriteBack: WRITE_CACHE_TYPE = WRITE_CACHE_TYPE(2i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const WriteCacheTypeWriteThrough: WRITE_CACHE_TYPE = WRITE_CACHE_TYPE(3i32);
impl ::core::marker::Copy for WRITE_CACHE_TYPE {}
impl ::core::clone::Clone for WRITE_CACHE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WRITE_CACHE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WRITE_CACHE_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for WRITE_CACHE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WRITE_CACHE_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const WRITE_COMPRESSION_INFO_VALID: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WRITE_THROUGH(pub i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const WriteThroughUnknown: WRITE_THROUGH = WRITE_THROUGH(0i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const WriteThroughNotSupported: WRITE_THROUGH = WRITE_THROUGH(1i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const WriteThroughSupported: WRITE_THROUGH = WRITE_THROUGH(2i32);
impl ::core::marker::Copy for WRITE_THROUGH {}
impl ::core::clone::Clone for WRITE_THROUGH {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WRITE_THROUGH {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WRITE_THROUGH {
    type Abi = Self;
}
impl ::core::fmt::Debug for WRITE_THROUGH {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WRITE_THROUGH").field(&self.0).finish()
    }
}
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
impl ::core::fmt::Debug for WRITE_USN_REASON_INPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WRITE_USN_REASON_INPUT").field("Flags", &self.Flags).field("UsnReasonToWrite", &self.UsnReasonToWrite).finish()
    }
}
unsafe impl ::windows::core::Abi for WRITE_USN_REASON_INPUT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WRITE_USN_REASON_INPUT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WRITE_USN_REASON_INPUT>()) == 0 }
    }
}
impl ::core::cmp::Eq for WRITE_USN_REASON_INPUT {}
impl ::core::default::Default for WRITE_USN_REASON_INPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct _DEVICEDUMP_COLLECTION_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const TCCollectionBugCheck: _DEVICEDUMP_COLLECTION_TYPE = _DEVICEDUMP_COLLECTION_TYPE(1i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const TCCollectionApplicationRequested: _DEVICEDUMP_COLLECTION_TYPE = _DEVICEDUMP_COLLECTION_TYPE(2i32);
#[doc = "*Required features: `\"Win32_System_Ioctl\"`*"]
pub const TCCollectionDeviceRequested: _DEVICEDUMP_COLLECTION_TYPE = _DEVICEDUMP_COLLECTION_TYPE(3i32);
impl ::core::marker::Copy for _DEVICEDUMP_COLLECTION_TYPE {}
impl ::core::clone::Clone for _DEVICEDUMP_COLLECTION_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for _DEVICEDUMP_COLLECTION_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for _DEVICEDUMP_COLLECTION_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for _DEVICEDUMP_COLLECTION_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("_DEVICEDUMP_COLLECTION_TYPE").field(&self.0).finish()
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
