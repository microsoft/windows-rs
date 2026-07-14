#[repr(C)]
#[derive(Clone, Copy)]
pub struct FILTER_AGGREGATE_BASIC_INFORMATION {
    pub NextEntryOffset: u32,
    pub Flags: u32,
    pub Type: FILTER_AGGREGATE_BASIC_INFORMATION_0,
}
impl Default for FILTER_AGGREGATE_BASIC_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union FILTER_AGGREGATE_BASIC_INFORMATION_0 {
    pub MiniFilter: FILTER_AGGREGATE_BASIC_INFORMATION_0_0,
    pub LegacyFilter: FILTER_AGGREGATE_BASIC_INFORMATION_0_1,
}
impl Default for FILTER_AGGREGATE_BASIC_INFORMATION_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FILTER_AGGREGATE_BASIC_INFORMATION_0_0 {
    pub FrameID: u32,
    pub NumberOfInstances: u32,
    pub FilterNameLength: u16,
    pub FilterNameBufferOffset: u16,
    pub FilterAltitudeLength: u16,
    pub FilterAltitudeBufferOffset: u16,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FILTER_AGGREGATE_BASIC_INFORMATION_0_1 {
    pub FilterNameLength: u16,
    pub FilterNameBufferOffset: u16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct FILTER_AGGREGATE_STANDARD_INFORMATION {
    pub NextEntryOffset: u32,
    pub Flags: u32,
    pub Type: FILTER_AGGREGATE_STANDARD_INFORMATION_0,
}
impl Default for FILTER_AGGREGATE_STANDARD_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union FILTER_AGGREGATE_STANDARD_INFORMATION_0 {
    pub MiniFilter: FILTER_AGGREGATE_STANDARD_INFORMATION_0_0,
    pub LegacyFilter: FILTER_AGGREGATE_STANDARD_INFORMATION_0_1,
}
impl Default for FILTER_AGGREGATE_STANDARD_INFORMATION_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FILTER_AGGREGATE_STANDARD_INFORMATION_0_0 {
    pub Flags: u32,
    pub FrameID: u32,
    pub NumberOfInstances: u32,
    pub FilterNameLength: u16,
    pub FilterNameBufferOffset: u16,
    pub FilterAltitudeLength: u16,
    pub FilterAltitudeBufferOffset: u16,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FILTER_AGGREGATE_STANDARD_INFORMATION_0_1 {
    pub Flags: u32,
    pub FilterNameLength: u16,
    pub FilterNameBufferOffset: u16,
    pub FilterAltitudeLength: u16,
    pub FilterAltitudeBufferOffset: u16,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FILTER_FULL_INFORMATION {
    pub NextEntryOffset: u32,
    pub FrameID: u32,
    pub NumberOfInstances: u32,
    pub FilterNameLength: u16,
    pub FilterNameBuffer: [u16; 1],
}
impl Default for FILTER_FULL_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type FILTER_INFORMATION_CLASS = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FILTER_MESSAGE_HEADER {
    pub ReplyLength: u32,
    pub MessageId: u64,
}
pub const FILTER_NAME_MAX_CHARS: u32 = 255;
#[repr(C)]
#[cfg(feature = "bcrypt")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FILTER_REPLY_HEADER {
    pub Status: super::bcrypt::NTSTATUS,
    pub MessageId: u64,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FILTER_VOLUME_BASIC_INFORMATION {
    pub FilterVolumeNameLength: u16,
    pub FilterVolumeName: [u16; 1],
}
impl Default for FILTER_VOLUME_BASIC_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type FILTER_VOLUME_INFORMATION_CLASS = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FILTER_VOLUME_STANDARD_INFORMATION {
    pub NextEntryOffset: u32,
    pub Flags: u32,
    pub FrameID: u32,
    pub FileSystemType: FLT_FILESYSTEM_TYPE,
    pub FilterVolumeNameLength: u16,
    pub FilterVolumeName: [u16; 1],
}
impl Default for FILTER_VOLUME_STANDARD_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const FLTFL_AGGREGATE_INFO_IS_LEGACYFILTER: u32 = 2;
pub const FLTFL_AGGREGATE_INFO_IS_MINIFILTER: u32 = 1;
pub const FLTFL_ASI_IS_LEGACYFILTER: u32 = 2;
pub const FLTFL_ASI_IS_MINIFILTER: u32 = 1;
pub const FLTFL_IASIL_DETACHED_VOLUME: u32 = 1;
pub const FLTFL_IASIM_DETACHED_VOLUME: u32 = 1;
pub const FLTFL_IASI_IS_LEGACYFILTER: u32 = 2;
pub const FLTFL_IASI_IS_MINIFILTER: u32 = 1;
pub const FLTFL_VSI_DETACHED_VOLUME: u32 = 1;
pub type FLT_FILESYSTEM_TYPE = i32;
pub const FLT_FSTYPE_BSUDF: FLT_FILESYSTEM_TYPE = 12;
pub const FLT_FSTYPE_CDFS: FLT_FILESYSTEM_TYPE = 4;
pub const FLT_FSTYPE_CIMFS: FLT_FILESYSTEM_TYPE = 30;
pub const FLT_FSTYPE_CSVFS: FLT_FILESYSTEM_TYPE = 27;
pub const FLT_FSTYPE_EXFAT: FLT_FILESYSTEM_TYPE = 22;
pub const FLT_FSTYPE_FAT: FLT_FILESYSTEM_TYPE = 3;
pub const FLT_FSTYPE_FS_REC: FLT_FILESYSTEM_TYPE = 19;
pub const FLT_FSTYPE_GPFS: FLT_FILESYSTEM_TYPE = 24;
pub const FLT_FSTYPE_INCD: FLT_FILESYSTEM_TYPE = 20;
pub const FLT_FSTYPE_INCD_FAT: FLT_FILESYSTEM_TYPE = 21;
pub const FLT_FSTYPE_LANMAN: FLT_FILESYSTEM_TYPE = 6;
pub const FLT_FSTYPE_MSFS: FLT_FILESYSTEM_TYPE = 26;
pub const FLT_FSTYPE_MS_NETWARE: FLT_FILESYSTEM_TYPE = 10;
pub const FLT_FSTYPE_MUP: FLT_FILESYSTEM_TYPE = 13;
pub const FLT_FSTYPE_NETWARE: FLT_FILESYSTEM_TYPE = 11;
pub const FLT_FSTYPE_NFS: FLT_FILESYSTEM_TYPE = 9;
pub const FLT_FSTYPE_NPFS: FLT_FILESYSTEM_TYPE = 25;
pub const FLT_FSTYPE_NTFS: FLT_FILESYSTEM_TYPE = 2;
pub const FLT_FSTYPE_OPENAFS: FLT_FILESYSTEM_TYPE = 29;
pub const FLT_FSTYPE_PSFS: FLT_FILESYSTEM_TYPE = 23;
pub const FLT_FSTYPE_RAW: FLT_FILESYSTEM_TYPE = 1;
pub const FLT_FSTYPE_RDPDR: FLT_FILESYSTEM_TYPE = 8;
pub const FLT_FSTYPE_REFS: FLT_FILESYSTEM_TYPE = 28;
pub const FLT_FSTYPE_ROXIO_UDF1: FLT_FILESYSTEM_TYPE = 15;
pub const FLT_FSTYPE_ROXIO_UDF2: FLT_FILESYSTEM_TYPE = 16;
pub const FLT_FSTYPE_ROXIO_UDF3: FLT_FILESYSTEM_TYPE = 17;
pub const FLT_FSTYPE_RSFX: FLT_FILESYSTEM_TYPE = 14;
pub const FLT_FSTYPE_TACIT: FLT_FILESYSTEM_TYPE = 18;
pub const FLT_FSTYPE_UDFS: FLT_FILESYSTEM_TYPE = 5;
pub const FLT_FSTYPE_UNKNOWN: FLT_FILESYSTEM_TYPE = 0;
pub const FLT_FSTYPE_WEBDAV: FLT_FILESYSTEM_TYPE = 7;
pub const FilterAggregateBasicInformation: FILTER_INFORMATION_CLASS = 1;
pub const FilterAggregateStandardInformation: FILTER_INFORMATION_CLASS = 2;
pub const FilterFullInformation: FILTER_INFORMATION_CLASS = 0;
pub const FilterVolumeBasicInformation: FILTER_VOLUME_INFORMATION_CLASS = 0;
pub const FilterVolumeStandardInformation: FILTER_VOLUME_INFORMATION_CLASS = 1;
#[cfg(feature = "winnt")]
pub type HFILTER = super::winnt::HANDLE;
#[cfg(feature = "winnt")]
pub type HFILTER_INSTANCE = super::winnt::HANDLE;
#[cfg(feature = "winnt")]
pub type HFILTER_VOLUME = super::winnt::HANDLE;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct INSTANCE_AGGREGATE_STANDARD_INFORMATION {
    pub NextEntryOffset: u32,
    pub Flags: u32,
    pub Type: INSTANCE_AGGREGATE_STANDARD_INFORMATION_0,
}
impl Default for INSTANCE_AGGREGATE_STANDARD_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union INSTANCE_AGGREGATE_STANDARD_INFORMATION_0 {
    pub MiniFilter: INSTANCE_AGGREGATE_STANDARD_INFORMATION_0_0,
    pub LegacyFilter: INSTANCE_AGGREGATE_STANDARD_INFORMATION_0_1,
}
impl Default for INSTANCE_AGGREGATE_STANDARD_INFORMATION_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct INSTANCE_AGGREGATE_STANDARD_INFORMATION_0_0 {
    pub Flags: u32,
    pub FrameID: u32,
    pub VolumeFileSystemType: FLT_FILESYSTEM_TYPE,
    pub InstanceNameLength: u16,
    pub InstanceNameBufferOffset: u16,
    pub AltitudeLength: u16,
    pub AltitudeBufferOffset: u16,
    pub VolumeNameLength: u16,
    pub VolumeNameBufferOffset: u16,
    pub FilterNameLength: u16,
    pub FilterNameBufferOffset: u16,
    pub SupportedFeatures: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct INSTANCE_AGGREGATE_STANDARD_INFORMATION_0_1 {
    pub Flags: u32,
    pub AltitudeLength: u16,
    pub AltitudeBufferOffset: u16,
    pub VolumeNameLength: u16,
    pub VolumeNameBufferOffset: u16,
    pub FilterNameLength: u16,
    pub FilterNameBufferOffset: u16,
    pub SupportedFeatures: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct INSTANCE_BASIC_INFORMATION {
    pub NextEntryOffset: u32,
    pub InstanceNameLength: u16,
    pub InstanceNameBufferOffset: u16,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct INSTANCE_FULL_INFORMATION {
    pub NextEntryOffset: u32,
    pub InstanceNameLength: u16,
    pub InstanceNameBufferOffset: u16,
    pub AltitudeLength: u16,
    pub AltitudeBufferOffset: u16,
    pub VolumeNameLength: u16,
    pub VolumeNameBufferOffset: u16,
    pub FilterNameLength: u16,
    pub FilterNameBufferOffset: u16,
}
pub type INSTANCE_INFORMATION_CLASS = i32;
pub const INSTANCE_NAME_MAX_CHARS: u32 = 255;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct INSTANCE_PARTIAL_INFORMATION {
    pub NextEntryOffset: u32,
    pub InstanceNameLength: u16,
    pub InstanceNameBufferOffset: u16,
    pub AltitudeLength: u16,
    pub AltitudeBufferOffset: u16,
}
pub const InstanceAggregateStandardInformation: INSTANCE_INFORMATION_CLASS = 3;
pub const InstanceBasicInformation: INSTANCE_INFORMATION_CLASS = 0;
pub const InstanceFullInformation: INSTANCE_INFORMATION_CLASS = 2;
pub const InstancePartialInformation: INSTANCE_INFORMATION_CLASS = 1;
pub type PFILTER_AGGREGATE_BASIC_INFORMATION = *mut FILTER_AGGREGATE_BASIC_INFORMATION;
pub type PFILTER_AGGREGATE_STANDARD_INFORMATION = *mut FILTER_AGGREGATE_STANDARD_INFORMATION;
pub type PFILTER_FULL_INFORMATION = *mut FILTER_FULL_INFORMATION;
pub type PFILTER_INFORMATION_CLASS = *mut FILTER_INFORMATION_CLASS;
pub type PFILTER_MESSAGE_HEADER = *mut FILTER_MESSAGE_HEADER;
#[cfg(feature = "bcrypt")]
pub type PFILTER_REPLY_HEADER = *mut FILTER_REPLY_HEADER;
pub type PFILTER_VOLUME_BASIC_INFORMATION = *mut FILTER_VOLUME_BASIC_INFORMATION;
pub type PFILTER_VOLUME_INFORMATION_CLASS = *mut FILTER_VOLUME_INFORMATION_CLASS;
pub type PFILTER_VOLUME_STANDARD_INFORMATION = *mut FILTER_VOLUME_STANDARD_INFORMATION;
pub type PFLT_FILESYSTEM_TYPE = *mut FLT_FILESYSTEM_TYPE;
pub type PINSTANCE_AGGREGATE_STANDARD_INFORMATION = *mut INSTANCE_AGGREGATE_STANDARD_INFORMATION;
pub type PINSTANCE_BASIC_INFORMATION = *mut INSTANCE_BASIC_INFORMATION;
pub type PINSTANCE_FULL_INFORMATION = *mut INSTANCE_FULL_INFORMATION;
pub type PINSTANCE_INFORMATION_CLASS = *mut INSTANCE_INFORMATION_CLASS;
pub type PINSTANCE_PARTIAL_INFORMATION = *mut INSTANCE_PARTIAL_INFORMATION;
pub const VOLUME_NAME_MAX_CHARS: u32 = 1024;
