pub const DVD_CGMS_COPY_ONCE: u32 = 16u32;
pub const DVD_CGMS_COPY_PERMITTED: u32 = 0u32;
pub const DVD_CGMS_COPY_PROTECT_MASK: u32 = 24u32;
pub const DVD_CGMS_NO_COPY: u32 = 24u32;
pub const DVD_CGMS_RESERVED_MASK: u32 = 120u32;
pub const DVD_COPYRIGHTED: u32 = 64u32;
pub const DVD_COPYRIGHT_MASK: u32 = 64u32;
pub const DVD_NOT_COPYRIGHTED: u32 = 0u32;
pub const DVD_SECTOR_NOT_PROTECTED: u32 = 0u32;
pub const DVD_SECTOR_PROTECTED: u32 = 32u32;
pub const DVD_SECTOR_PROTECT_MASK: u32 = 32u32;
pub const DiscControlBlockList: DISC_CONTROL_BLOCK_TYPE = DISC_CONTROL_BLOCK_TYPE(-1i32);
pub const DvdAsf: DVD_KEY_TYPE = DVD_KEY_TYPE(5i32);
pub const DvdBCADescriptor: DVD_STRUCTURE_FORMAT = DVD_STRUCTURE_FORMAT(3i32);
pub const DvdBusKey1: DVD_KEY_TYPE = DVD_KEY_TYPE(2i32);
pub const DvdBusKey2: DVD_KEY_TYPE = DVD_KEY_TYPE(3i32);
pub const DvdChallengeKey: DVD_KEY_TYPE = DVD_KEY_TYPE(1i32);
pub const DvdCopyrightDescriptor: DVD_STRUCTURE_FORMAT = DVD_STRUCTURE_FORMAT(1i32);
pub const DvdDiskKey: DVD_KEY_TYPE = DVD_KEY_TYPE(128i32);
pub const DvdDiskKeyDescriptor: DVD_STRUCTURE_FORMAT = DVD_STRUCTURE_FORMAT(2i32);
pub const DvdGetRpcKey: DVD_KEY_TYPE = DVD_KEY_TYPE(8i32);
pub const DvdInvalidateAGID: DVD_KEY_TYPE = DVD_KEY_TYPE(63i32);
pub const DvdManufacturerDescriptor: DVD_STRUCTURE_FORMAT = DVD_STRUCTURE_FORMAT(4i32);
pub const DvdMaxDescriptor: DVD_STRUCTURE_FORMAT = DVD_STRUCTURE_FORMAT(5i32);
pub const DvdPhysicalDescriptor: DVD_STRUCTURE_FORMAT = DVD_STRUCTURE_FORMAT(0i32);
pub const DvdSetRpcKey: DVD_KEY_TYPE = DVD_KEY_TYPE(6i32);
pub const DvdTitleKey: DVD_KEY_TYPE = DVD_KEY_TYPE(4i32);
pub const FormattingDiscControlBlock: DISC_CONTROL_BLOCK_TYPE = DISC_CONTROL_BLOCK_TYPE(1178878720i32);
pub const IOCTL_AACS_END_SESSION: u32 = 3363020u32;
pub const IOCTL_AACS_GENERATE_BINDING_NONCE: u32 = 3395824u32;
pub const IOCTL_AACS_GET_CERTIFICATE: u32 = 3363028u32;
pub const IOCTL_AACS_GET_CHALLENGE_KEY: u32 = 3363032u32;
pub const IOCTL_AACS_READ_BINDING_NONCE: u32 = 3363052u32;
pub const IOCTL_AACS_READ_MEDIA_ID: u32 = 3363048u32;
pub const IOCTL_AACS_READ_MEDIA_KEY_BLOCK: u32 = 3363012u32;
pub const IOCTL_AACS_READ_MEDIA_KEY_BLOCK_SIZE: u32 = 3363008u32;
pub const IOCTL_AACS_READ_SERIAL_NUMBER: u32 = 3363044u32;
pub const IOCTL_AACS_READ_VOLUME_ID: u32 = 3363040u32;
pub const IOCTL_AACS_SEND_CERTIFICATE: u32 = 3363024u32;
pub const IOCTL_AACS_SEND_CHALLENGE_KEY: u32 = 3363036u32;
pub const IOCTL_AACS_START_SESSION: u32 = 3363016u32;
pub const IOCTL_DVD_BASE: i32 = 51i32;
pub const IOCTL_DVD_END_SESSION: u32 = 3362828u32;
pub const IOCTL_DVD_GET_REGION: u32 = 3362836u32;
pub const IOCTL_DVD_READ_KEY: u32 = 3362820u32;
pub const IOCTL_DVD_READ_STRUCTURE: u32 = 3363136u32;
pub const IOCTL_DVD_SEND_KEY: u32 = 3362824u32;
pub const IOCTL_DVD_SEND_KEY2: u32 = 3395608u32;
pub const IOCTL_DVD_SET_READ_AHEAD: u32 = 3362832u32;
pub const IOCTL_DVD_START_SESSION: u32 = 3362816u32;
pub const IOCTL_STORAGE_SET_READ_AHEAD: u32 = 2966528u32;
pub const SessionInfoDiscControlBlock: DISC_CONTROL_BLOCK_TYPE = DISC_CONTROL_BLOCK_TYPE(1396982528i32);
pub const WriteInhibitDiscControlBlock: DISC_CONTROL_BLOCK_TYPE = DISC_CONTROL_BLOCK_TYPE(1464091392i32);
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DISC_CONTROL_BLOCK_TYPE(pub i32);
impl windows_core::TypeKind for DISC_CONTROL_BLOCK_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DISC_CONTROL_BLOCK_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DISC_CONTROL_BLOCK_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DVD_KEY_TYPE(pub i32);
impl windows_core::TypeKind for DVD_KEY_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DVD_KEY_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DVD_KEY_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DVD_STRUCTURE_FORMAT(pub i32);
impl windows_core::TypeKind for DVD_STRUCTURE_FORMAT {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DVD_STRUCTURE_FORMAT {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DVD_STRUCTURE_FORMAT").field(&self.0).finish()
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AACS_BINDING_NONCE {
    pub BindingNonce: [u8; 16],
    pub MAC: [u8; 16],
}
impl windows_core::TypeKind for AACS_BINDING_NONCE {
    type TypeKind = windows_core::CopyType;
}
impl Default for AACS_BINDING_NONCE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AACS_CERTIFICATE {
    pub Nonce: [u8; 20],
    pub Certificate: [u8; 92],
}
impl windows_core::TypeKind for AACS_CERTIFICATE {
    type TypeKind = windows_core::CopyType;
}
impl Default for AACS_CERTIFICATE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AACS_CHALLENGE_KEY {
    pub EllipticCurvePoint: [u8; 40],
    pub Signature: [u8; 40],
}
impl windows_core::TypeKind for AACS_CHALLENGE_KEY {
    type TypeKind = windows_core::CopyType;
}
impl Default for AACS_CHALLENGE_KEY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AACS_MEDIA_ID {
    pub MediaID: [u8; 16],
    pub MAC: [u8; 16],
}
impl windows_core::TypeKind for AACS_MEDIA_ID {
    type TypeKind = windows_core::CopyType;
}
impl Default for AACS_MEDIA_ID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct AACS_READ_BINDING_NONCE {
    pub SessionId: u32,
    pub NumberOfSectors: u32,
    pub StartLba: u64,
    pub Anonymous: AACS_READ_BINDING_NONCE_0,
}
impl windows_core::TypeKind for AACS_READ_BINDING_NONCE {
    type TypeKind = windows_core::CopyType;
}
impl Default for AACS_READ_BINDING_NONCE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union AACS_READ_BINDING_NONCE_0 {
    pub Handle: super::super::Foundation::HANDLE,
    pub ForceStructureLengthToMatch64bit: u64,
}
impl windows_core::TypeKind for AACS_READ_BINDING_NONCE_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for AACS_READ_BINDING_NONCE_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AACS_SEND_CERTIFICATE {
    pub SessionId: u32,
    pub Certificate: AACS_CERTIFICATE,
}
impl windows_core::TypeKind for AACS_SEND_CERTIFICATE {
    type TypeKind = windows_core::CopyType;
}
impl Default for AACS_SEND_CERTIFICATE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AACS_SEND_CHALLENGE_KEY {
    pub SessionId: u32,
    pub ChallengeKey: AACS_CHALLENGE_KEY,
}
impl windows_core::TypeKind for AACS_SEND_CHALLENGE_KEY {
    type TypeKind = windows_core::CopyType;
}
impl Default for AACS_SEND_CHALLENGE_KEY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AACS_SERIAL_NUMBER {
    pub PrerecordedSerialNumber: [u8; 16],
    pub MAC: [u8; 16],
}
impl windows_core::TypeKind for AACS_SERIAL_NUMBER {
    type TypeKind = windows_core::CopyType;
}
impl Default for AACS_SERIAL_NUMBER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AACS_VOLUME_ID {
    pub VolumeID: [u8; 16],
    pub MAC: [u8; 16],
}
impl windows_core::TypeKind for AACS_VOLUME_ID {
    type TypeKind = windows_core::CopyType;
}
impl Default for AACS_VOLUME_ID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BD_DISC_WRITE_PROTECT_PAC {
    pub Header: BD_PAC_HEADER,
    pub KnownPACEntireDiscFlags: u8,
    pub Reserved1: [u8; 3],
    pub WriteProtectControlByte: u8,
    pub Reserved2: [u8; 7],
    pub WriteProtectPassword: [u8; 32],
}
impl windows_core::TypeKind for BD_DISC_WRITE_PROTECT_PAC {
    type TypeKind = windows_core::CopyType;
}
impl Default for BD_DISC_WRITE_PROTECT_PAC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BD_PAC_HEADER {
    pub PACId: [u8; 3],
    pub PACFormatNumber: u8,
    pub PACUpdateCount: [u8; 4],
    pub UnknownPACRules: [u8; 4],
    pub UnkownPACEntireDiscFlag: u8,
    pub Reserved1: [u8; 2],
    pub NumberOfSegments: u8,
    pub Segments: [u8; 256],
    pub Reserved2: [u8; 112],
}
impl windows_core::TypeKind for BD_PAC_HEADER {
    type TypeKind = windows_core::CopyType;
}
impl Default for BD_PAC_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DVD_ASF {
    pub Reserved0: [u8; 3],
    pub _bitfield: u8,
}
impl windows_core::TypeKind for DVD_ASF {
    type TypeKind = windows_core::CopyType;
}
impl Default for DVD_ASF {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DVD_BCA_DESCRIPTOR {
    pub BCAInformation: [u8; 1],
}
impl windows_core::TypeKind for DVD_BCA_DESCRIPTOR {
    type TypeKind = windows_core::CopyType;
}
impl Default for DVD_BCA_DESCRIPTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DVD_BD_SPARE_AREA_INFORMATION {
    pub Reserved1: [u8; 4],
    pub NumberOfFreeSpareBlocks: [u8; 4],
    pub NumberOfAllocatedSpareBlocks: [u8; 4],
}
impl windows_core::TypeKind for DVD_BD_SPARE_AREA_INFORMATION {
    type TypeKind = windows_core::CopyType;
}
impl Default for DVD_BD_SPARE_AREA_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct DVD_COPYRIGHT_DESCRIPTOR {
    pub CopyrightProtectionType: u8,
    pub RegionManagementInformation: u8,
    pub Reserved: u16,
}
impl windows_core::TypeKind for DVD_COPYRIGHT_DESCRIPTOR {
    type TypeKind = windows_core::CopyType;
}
impl Default for DVD_COPYRIGHT_DESCRIPTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DVD_COPYRIGHT_MANAGEMENT_DESCRIPTOR {
    pub Anonymous: DVD_COPYRIGHT_MANAGEMENT_DESCRIPTOR_0,
    pub Reserved0: [u8; 3],
}
impl windows_core::TypeKind for DVD_COPYRIGHT_MANAGEMENT_DESCRIPTOR {
    type TypeKind = windows_core::CopyType;
}
impl Default for DVD_COPYRIGHT_MANAGEMENT_DESCRIPTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union DVD_COPYRIGHT_MANAGEMENT_DESCRIPTOR_0 {
    pub Dvdrom: DVD_COPYRIGHT_MANAGEMENT_DESCRIPTOR_0_0,
    pub DvdRecordable_Version1: DVD_COPYRIGHT_MANAGEMENT_DESCRIPTOR_0_1,
    pub Dvdram: DVD_COPYRIGHT_MANAGEMENT_DESCRIPTOR_0_2,
    pub DvdRecordable: DVD_COPYRIGHT_MANAGEMENT_DESCRIPTOR_0_3,
    pub CPR_MAI: u8,
}
impl windows_core::TypeKind for DVD_COPYRIGHT_MANAGEMENT_DESCRIPTOR_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for DVD_COPYRIGHT_MANAGEMENT_DESCRIPTOR_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DVD_COPYRIGHT_MANAGEMENT_DESCRIPTOR_0_1 {
    pub _bitfield: u8,
}
impl windows_core::TypeKind for DVD_COPYRIGHT_MANAGEMENT_DESCRIPTOR_0_1 {
    type TypeKind = windows_core::CopyType;
}
impl Default for DVD_COPYRIGHT_MANAGEMENT_DESCRIPTOR_0_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DVD_COPYRIGHT_MANAGEMENT_DESCRIPTOR_0_3 {
    pub _bitfield: u8,
}
impl windows_core::TypeKind for DVD_COPYRIGHT_MANAGEMENT_DESCRIPTOR_0_3 {
    type TypeKind = windows_core::CopyType;
}
impl Default for DVD_COPYRIGHT_MANAGEMENT_DESCRIPTOR_0_3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DVD_COPYRIGHT_MANAGEMENT_DESCRIPTOR_0_2 {
    pub Reserved0003: u8,
}
impl windows_core::TypeKind for DVD_COPYRIGHT_MANAGEMENT_DESCRIPTOR_0_2 {
    type TypeKind = windows_core::CopyType;
}
impl Default for DVD_COPYRIGHT_MANAGEMENT_DESCRIPTOR_0_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DVD_COPYRIGHT_MANAGEMENT_DESCRIPTOR_0_0 {
    pub _bitfield: u8,
}
impl windows_core::TypeKind for DVD_COPYRIGHT_MANAGEMENT_DESCRIPTOR_0_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for DVD_COPYRIGHT_MANAGEMENT_DESCRIPTOR_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct DVD_COPY_PROTECT_KEY {
    pub KeyLength: u32,
    pub SessionId: u32,
    pub KeyType: DVD_KEY_TYPE,
    pub KeyFlags: u32,
    pub Parameters: DVD_COPY_PROTECT_KEY_0,
    pub KeyData: [u8; 1],
}
impl windows_core::TypeKind for DVD_COPY_PROTECT_KEY {
    type TypeKind = windows_core::CopyType;
}
impl Default for DVD_COPY_PROTECT_KEY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union DVD_COPY_PROTECT_KEY_0 {
    pub FileHandle: super::super::Foundation::HANDLE,
    pub TitleOffset: i64,
}
impl windows_core::TypeKind for DVD_COPY_PROTECT_KEY_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for DVD_COPY_PROTECT_KEY_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct DVD_DESCRIPTOR_HEADER {
    pub Length: u16,
    pub Reserved: [u8; 2],
    pub Data: [u8; 1],
}
impl windows_core::TypeKind for DVD_DESCRIPTOR_HEADER {
    type TypeKind = windows_core::CopyType;
}
impl Default for DVD_DESCRIPTOR_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DVD_DISC_CONTROL_BLOCK_HEADER {
    pub ContentDescriptor: [u8; 4],
    pub ProhibitedActions: DVD_DISC_CONTROL_BLOCK_HEADER_0,
    pub VendorId: [u8; 32],
}
impl windows_core::TypeKind for DVD_DISC_CONTROL_BLOCK_HEADER {
    type TypeKind = windows_core::CopyType;
}
impl Default for DVD_DISC_CONTROL_BLOCK_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union DVD_DISC_CONTROL_BLOCK_HEADER_0 {
    pub Anonymous: DVD_DISC_CONTROL_BLOCK_HEADER_0_0,
    pub AsByte: [u8; 4],
}
impl windows_core::TypeKind for DVD_DISC_CONTROL_BLOCK_HEADER_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for DVD_DISC_CONTROL_BLOCK_HEADER_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DVD_DISC_CONTROL_BLOCK_HEADER_0_0 {
    pub ReservedDoNotUse_UseAsByteInstead_0: [u8; 3],
    pub _bitfield: u8,
}
impl windows_core::TypeKind for DVD_DISC_CONTROL_BLOCK_HEADER_0_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for DVD_DISC_CONTROL_BLOCK_HEADER_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DVD_DISC_CONTROL_BLOCK_LIST {
    pub header: DVD_DISC_CONTROL_BLOCK_HEADER,
    pub Reserved0: u8,
    pub ReadabldDCBs: u8,
    pub Reserved1: u8,
    pub WritableDCBs: u8,
    pub Dcbs: [DVD_DISC_CONTROL_BLOCK_LIST_DCB; 1],
}
impl windows_core::TypeKind for DVD_DISC_CONTROL_BLOCK_LIST {
    type TypeKind = windows_core::CopyType;
}
impl Default for DVD_DISC_CONTROL_BLOCK_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DVD_DISC_CONTROL_BLOCK_LIST_DCB {
    pub DcbIdentifier: [u8; 4],
}
impl windows_core::TypeKind for DVD_DISC_CONTROL_BLOCK_LIST_DCB {
    type TypeKind = windows_core::CopyType;
}
impl Default for DVD_DISC_CONTROL_BLOCK_LIST_DCB {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DVD_DISC_CONTROL_BLOCK_SESSION {
    pub header: DVD_DISC_CONTROL_BLOCK_HEADER,
    pub SessionNumber: [u8; 2],
    pub Reserved0: [u8; 22],
    pub DiscID: [u8; 32],
    pub Reserved1: [u8; 32],
    pub SessionItem: [DVD_DISC_CONTROL_BLOCK_SESSION_ITEM; 504],
    pub Reserved2: [u8; 24576],
}
impl windows_core::TypeKind for DVD_DISC_CONTROL_BLOCK_SESSION {
    type TypeKind = windows_core::CopyType;
}
impl Default for DVD_DISC_CONTROL_BLOCK_SESSION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DVD_DISC_CONTROL_BLOCK_SESSION_ITEM {
    pub AsByte: [u8; 16],
}
impl windows_core::TypeKind for DVD_DISC_CONTROL_BLOCK_SESSION_ITEM {
    type TypeKind = windows_core::CopyType;
}
impl Default for DVD_DISC_CONTROL_BLOCK_SESSION_ITEM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DVD_DISC_CONTROL_BLOCK_WRITE_INHIBIT {
    pub header: DVD_DISC_CONTROL_BLOCK_HEADER,
    pub UpdateCount: [u8; 4],
    pub WriteProtectActions: DVD_DISC_CONTROL_BLOCK_WRITE_INHIBIT_0,
    pub Reserved0: [u8; 16],
    pub UpdatePassword: [u8; 32],
    pub Reserved1: [u8; 32672],
}
impl windows_core::TypeKind for DVD_DISC_CONTROL_BLOCK_WRITE_INHIBIT {
    type TypeKind = windows_core::CopyType;
}
impl Default for DVD_DISC_CONTROL_BLOCK_WRITE_INHIBIT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union DVD_DISC_CONTROL_BLOCK_WRITE_INHIBIT_0 {
    pub Anonymous: DVD_DISC_CONTROL_BLOCK_WRITE_INHIBIT_0_0,
    pub AsByte: [u8; 4],
}
impl windows_core::TypeKind for DVD_DISC_CONTROL_BLOCK_WRITE_INHIBIT_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for DVD_DISC_CONTROL_BLOCK_WRITE_INHIBIT_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DVD_DISC_CONTROL_BLOCK_WRITE_INHIBIT_0_0 {
    pub ReservedDoNotUse_UseAsByteInstead_0: [u8; 3],
    pub _bitfield: u8,
}
impl windows_core::TypeKind for DVD_DISC_CONTROL_BLOCK_WRITE_INHIBIT_0_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for DVD_DISC_CONTROL_BLOCK_WRITE_INHIBIT_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DVD_DISK_KEY_DESCRIPTOR {
    pub DiskKeyData: [u8; 2048],
}
impl windows_core::TypeKind for DVD_DISK_KEY_DESCRIPTOR {
    type TypeKind = windows_core::CopyType;
}
impl Default for DVD_DISK_KEY_DESCRIPTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DVD_DUAL_LAYER_JUMP_INTERVAL_SIZE {
    pub Reserved1: [u8; 4],
    pub JumpIntervalSize: [u8; 4],
}
impl windows_core::TypeKind for DVD_DUAL_LAYER_JUMP_INTERVAL_SIZE {
    type TypeKind = windows_core::CopyType;
}
impl Default for DVD_DUAL_LAYER_JUMP_INTERVAL_SIZE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DVD_DUAL_LAYER_MANUAL_LAYER_JUMP {
    pub Reserved1: [u8; 4],
    pub ManualJumpLayerAddress: [u8; 4],
}
impl windows_core::TypeKind for DVD_DUAL_LAYER_MANUAL_LAYER_JUMP {
    type TypeKind = windows_core::CopyType;
}
impl Default for DVD_DUAL_LAYER_MANUAL_LAYER_JUMP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DVD_DUAL_LAYER_MIDDLE_ZONE_START_ADDRESS {
    pub _bitfield: u8,
    pub Reserved1: [u8; 3],
    pub ShiftedMiddleAreaStartAddress: [u8; 4],
}
impl windows_core::TypeKind for DVD_DUAL_LAYER_MIDDLE_ZONE_START_ADDRESS {
    type TypeKind = windows_core::CopyType;
}
impl Default for DVD_DUAL_LAYER_MIDDLE_ZONE_START_ADDRESS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DVD_DUAL_LAYER_RECORDING_INFORMATION {
    pub _bitfield: u8,
    pub Reserved1: [u8; 3],
    pub Layer0Sectors: [u8; 4],
}
impl windows_core::TypeKind for DVD_DUAL_LAYER_RECORDING_INFORMATION {
    type TypeKind = windows_core::CopyType;
}
impl Default for DVD_DUAL_LAYER_RECORDING_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DVD_DUAL_LAYER_REMAPPING_INFORMATION {
    pub Reserved1: [u8; 4],
    pub RemappingAddress: [u8; 4],
}
impl windows_core::TypeKind for DVD_DUAL_LAYER_REMAPPING_INFORMATION {
    type TypeKind = windows_core::CopyType;
}
impl Default for DVD_DUAL_LAYER_REMAPPING_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DVD_FULL_LAYER_DESCRIPTOR {
    pub commonHeader: DVD_LAYER_DESCRIPTOR,
    pub MediaSpecific: [u8; 2031],
}
impl windows_core::TypeKind for DVD_FULL_LAYER_DESCRIPTOR {
    type TypeKind = windows_core::CopyType;
}
impl Default for DVD_FULL_LAYER_DESCRIPTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct DVD_LAYER_DESCRIPTOR {
    pub _bitfield1: u8,
    pub _bitfield2: u8,
    pub _bitfield3: u8,
    pub _bitfield4: u8,
    pub StartingDataSector: u32,
    pub EndDataSector: u32,
    pub EndLayerZeroSector: u32,
    pub _bitfield5: u8,
}
impl windows_core::TypeKind for DVD_LAYER_DESCRIPTOR {
    type TypeKind = windows_core::CopyType;
}
impl Default for DVD_LAYER_DESCRIPTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DVD_LIST_OF_RECOGNIZED_FORMAT_LAYERS {
    pub TypeCodeOfFormatLayer: [u8; 2],
}
impl windows_core::TypeKind for DVD_LIST_OF_RECOGNIZED_FORMAT_LAYERS {
    type TypeKind = windows_core::CopyType;
}
impl Default for DVD_LIST_OF_RECOGNIZED_FORMAT_LAYERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DVD_LIST_OF_RECOGNIZED_FORMAT_LAYERS_TYPE_CODE {
    pub NumberOfRecognizedFormatLayers: u8,
    pub _bitfield: u8,
}
impl windows_core::TypeKind for DVD_LIST_OF_RECOGNIZED_FORMAT_LAYERS_TYPE_CODE {
    type TypeKind = windows_core::CopyType;
}
impl Default for DVD_LIST_OF_RECOGNIZED_FORMAT_LAYERS_TYPE_CODE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DVD_MANUFACTURER_DESCRIPTOR {
    pub ManufacturingInformation: [u8; 2048],
}
impl windows_core::TypeKind for DVD_MANUFACTURER_DESCRIPTOR {
    type TypeKind = windows_core::CopyType;
}
impl Default for DVD_MANUFACTURER_DESCRIPTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DVD_PRERECORDED_INFORMATION {
    pub FieldID_1: u8,
    pub DiscApplicationCode: u8,
    pub DiscPhysicalCode: u8,
    pub LastAddressOfDataRecordableArea: [u8; 3],
    pub _bitfield: u8,
    pub Reserved0: u8,
    pub FieldID_2: u8,
    pub OpcSuggestedCode: u8,
    pub WavelengthCode: u8,
    pub WriteStrategyCode: [u8; 4],
    pub Reserved2: u8,
    pub FieldID_3: u8,
    pub ManufacturerId_3: [u8; 6],
    pub Reserved3: u8,
    pub FieldID_4: u8,
    pub ManufacturerId_4: [u8; 6],
    pub Reserved4: u8,
    pub FieldID_5: u8,
    pub ManufacturerId_5: [u8; 6],
    pub Reserved5: u8,
    pub Reserved99: [u8; 24],
}
impl windows_core::TypeKind for DVD_PRERECORDED_INFORMATION {
    type TypeKind = windows_core::CopyType;
}
impl Default for DVD_PRERECORDED_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DVD_RAM_MEDIUM_STATUS {
    pub _bitfield: u8,
    pub DiscTypeIdentification: u8,
    pub Reserved2: u8,
    pub MediaSpecificWriteInhibitInformation: u8,
}
impl windows_core::TypeKind for DVD_RAM_MEDIUM_STATUS {
    type TypeKind = windows_core::CopyType;
}
impl Default for DVD_RAM_MEDIUM_STATUS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DVD_RAM_RECORDING_TYPE {
    pub _bitfield: u8,
    pub Reserved2: [u8; 3],
}
impl windows_core::TypeKind for DVD_RAM_RECORDING_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl Default for DVD_RAM_RECORDING_TYPE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DVD_RAM_SPARE_AREA_INFORMATION {
    pub FreePrimarySpareSectors: [u8; 4],
    pub FreeSupplementalSpareSectors: [u8; 4],
    pub AllocatedSupplementalSpareSectors: [u8; 4],
}
impl windows_core::TypeKind for DVD_RAM_SPARE_AREA_INFORMATION {
    type TypeKind = windows_core::CopyType;
}
impl Default for DVD_RAM_SPARE_AREA_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct DVD_READ_STRUCTURE {
    pub BlockByteOffset: i64,
    pub Format: DVD_STRUCTURE_FORMAT,
    pub SessionId: u32,
    pub LayerNumber: u8,
}
impl windows_core::TypeKind for DVD_READ_STRUCTURE {
    type TypeKind = windows_core::CopyType;
}
impl Default for DVD_READ_STRUCTURE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DVD_RECORDING_MANAGEMENT_AREA_DATA {
    pub LastRecordedRMASectorNumber: [u8; 4],
    pub RMDBytes: [u8; 1],
}
impl windows_core::TypeKind for DVD_RECORDING_MANAGEMENT_AREA_DATA {
    type TypeKind = windows_core::CopyType;
}
impl Default for DVD_RECORDING_MANAGEMENT_AREA_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DVD_REGION {
    pub CopySystem: u8,
    pub RegionData: u8,
    pub SystemRegion: u8,
    pub ResetCount: u8,
}
impl windows_core::TypeKind for DVD_REGION {
    type TypeKind = windows_core::CopyType;
}
impl Default for DVD_REGION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DVD_RPC_KEY {
    pub _bitfield: u8,
    pub RegionMask: u8,
    pub RpcScheme: u8,
    pub Reserved02: u8,
}
impl windows_core::TypeKind for DVD_RPC_KEY {
    type TypeKind = windows_core::CopyType;
}
impl Default for DVD_RPC_KEY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DVD_SET_RPC_KEY {
    pub PreferredDriveRegionCode: u8,
    pub Reserved: [u8; 3],
}
impl windows_core::TypeKind for DVD_SET_RPC_KEY {
    type TypeKind = windows_core::CopyType;
}
impl Default for DVD_SET_RPC_KEY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DVD_STRUCTURE_LIST_ENTRY {
    pub FormatCode: u8,
    pub _bitfield: u8,
    pub FormatLength: [u8; 2],
}
impl windows_core::TypeKind for DVD_STRUCTURE_LIST_ENTRY {
    type TypeKind = windows_core::CopyType;
}
impl Default for DVD_STRUCTURE_LIST_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DVD_UNIQUE_DISC_IDENTIFIER {
    pub Reserved0: [u8; 2],
    pub RandomNumber: [u8; 2],
    pub Year: [u8; 4],
    pub Month: [u8; 2],
    pub Day: [u8; 2],
    pub Hour: [u8; 2],
    pub Minute: [u8; 2],
    pub Second: [u8; 2],
}
impl windows_core::TypeKind for DVD_UNIQUE_DISC_IDENTIFIER {
    type TypeKind = windows_core::CopyType;
}
impl Default for DVD_UNIQUE_DISC_IDENTIFIER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DVD_WRITE_PROTECTION_STATUS {
    pub _bitfield: u8,
    pub Reserved1: [u8; 3],
}
impl windows_core::TypeKind for DVD_WRITE_PROTECTION_STATUS {
    type TypeKind = windows_core::CopyType;
}
impl Default for DVD_WRITE_PROTECTION_STATUS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HD_DVD_R_MEDIUM_STATUS {
    pub _bitfield: u8,
    pub NumberOfRemainingRMDsInRDZ: u8,
    pub NumberOfRemainingRMDsInCurrentRMZ: [u8; 2],
}
impl windows_core::TypeKind for HD_DVD_R_MEDIUM_STATUS {
    type TypeKind = windows_core::CopyType;
}
impl Default for HD_DVD_R_MEDIUM_STATUS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct STORAGE_SET_READ_AHEAD {
    pub TriggerAddress: i64,
    pub TargetAddress: i64,
}
impl windows_core::TypeKind for STORAGE_SET_READ_AHEAD {
    type TypeKind = windows_core::CopyType;
}
impl Default for STORAGE_SET_READ_AHEAD {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}