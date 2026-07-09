#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct FILE_EXTENT {
    pub VolumeOffset: u64,
    pub ExtentLength: u64,
}
pub const IOCTL_VOLUME_ALLOCATE_BC_STREAM: u32 = 5685312;
pub const IOCTL_VOLUME_BC_VERSION: u32 = 1;
pub const IOCTL_VOLUME_FREE_BC_STREAM: u32 = 5685316;
pub const IOCTL_VOLUME_GET_BC_PROPERTIES: u32 = 5652540;
pub const IOCTL_VOLUME_GET_CSVBLOCKCACHE_CALLBACK: u32 = 5685352;
pub const IOCTL_VOLUME_IS_CSV: u32 = 5636192;
pub const IOCTL_VOLUME_IS_DYNAMIC: u32 = 5636168;
pub const IOCTL_VOLUME_IS_IO_CAPABLE: u32 = 5636116;
pub const IOCTL_VOLUME_IS_OFFLINE: u32 = 5636112;
pub const IOCTL_VOLUME_IS_PARTITION: u32 = 5636136;
pub const IOCTL_VOLUME_LOGICAL_TO_PHYSICAL: u32 = 5636128;
pub const IOCTL_VOLUME_PHYSICAL_TO_LOGICAL: u32 = 5636132;
pub const IOCTL_VOLUME_POST_ONLINE: u32 = 5685348;
pub const IOCTL_VOLUME_PREPARE_FOR_CRITICAL_IO: u32 = 5685324;
pub const IOCTL_VOLUME_PREPARE_FOR_SHRINK: u32 = 5685340;
pub const IOCTL_VOLUME_QUERY_ALLOCATION_HINT: u32 = 5652562;
pub const IOCTL_VOLUME_QUERY_FAILOVER_SET: u32 = 5636120;
pub const IOCTL_VOLUME_QUERY_MINIMUM_SHRINK_SIZE: u32 = 5652568;
pub const IOCTL_VOLUME_QUERY_VOLUME_NUMBER: u32 = 5636124;
pub const IOCTL_VOLUME_READ_PLEX: u32 = 5652526;
pub const IOCTL_VOLUME_SET_GPT_ATTRIBUTES: u32 = 5636148;
pub const IOCTL_VOLUME_SUPPORTS_ONLINE_OFFLINE: u32 = 5636100;
pub const IOCTL_VOLUME_UPDATE_PROPERTIES: u32 = 5636180;
pub type PFILE_EXTENT = *mut FILE_EXTENT;
pub type PVOLUME_ALLOCATE_BC_STREAM_INPUT = *mut VOLUME_ALLOCATE_BC_STREAM_INPUT;
pub type PVOLUME_ALLOCATE_BC_STREAM_OUTPUT = *mut VOLUME_ALLOCATE_BC_STREAM_OUTPUT;
pub type PVOLUME_ALLOCATION_HINT_INPUT = *mut VOLUME_ALLOCATION_HINT_INPUT;
pub type PVOLUME_ALLOCATION_HINT_OUTPUT = *mut VOLUME_ALLOCATION_HINT_OUTPUT;
pub type PVOLUME_CRITICAL_IO = *mut VOLUME_CRITICAL_IO;
pub type PVOLUME_FAILOVER_SET = *mut VOLUME_FAILOVER_SET;
pub type PVOLUME_GET_BC_PROPERTIES_INPUT = *mut VOLUME_GET_BC_PROPERTIES_INPUT;
pub type PVOLUME_GET_BC_PROPERTIES_OUTPUT = *mut VOLUME_GET_BC_PROPERTIES_OUTPUT;
pub type PVOLUME_LOGICAL_OFFSET = *mut VOLUME_LOGICAL_OFFSET;
pub type PVOLUME_NUMBER = *mut VOLUME_NUMBER;
pub type PVOLUME_PHYSICAL_OFFSET = *mut VOLUME_PHYSICAL_OFFSET;
pub type PVOLUME_PHYSICAL_OFFSETS = *mut VOLUME_PHYSICAL_OFFSETS;
pub type PVOLUME_READ_PLEX_INPUT = *mut VOLUME_READ_PLEX_INPUT;
pub type PVOLUME_SET_GPT_ATTRIBUTES_INFORMATION = *mut VOLUME_SET_GPT_ATTRIBUTES_INFORMATION;
pub type PVOLUME_SHRINK_INFO = *mut VOLUME_SHRINK_INFO;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct VOLUME_ALLOCATE_BC_STREAM_INPUT {
    pub Version: u32,
    pub RequestsPerPeriod: u32,
    pub Period: u32,
    pub RetryFailures: bool,
    pub Discardable: bool,
    pub Reserved1: [bool; 2],
    pub LowestByteOffset: u64,
    pub HighestByteOffset: u64,
    pub AccessType: u32,
    pub AccessMode: u32,
}
impl Default for VOLUME_ALLOCATE_BC_STREAM_INPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct VOLUME_ALLOCATE_BC_STREAM_OUTPUT {
    pub RequestSize: u64,
    pub NumOutStandingRequests: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct VOLUME_ALLOCATION_HINT_INPUT {
    pub ClusterSize: u32,
    pub NumberOfClusters: u32,
    pub StartingClusterNumber: i64,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct VOLUME_ALLOCATION_HINT_OUTPUT {
    pub Bitmap: [u32; 1],
}
impl Default for VOLUME_ALLOCATION_HINT_OUTPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct VOLUME_CRITICAL_IO {
    pub AccessType: u32,
    pub ExtentsCount: u32,
    pub Extents: [FILE_EXTENT; 1],
}
impl Default for VOLUME_CRITICAL_IO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct VOLUME_FAILOVER_SET {
    pub NumberOfDisks: u32,
    pub DiskNumbers: [u32; 1],
}
impl Default for VOLUME_FAILOVER_SET {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct VOLUME_GET_BC_PROPERTIES_INPUT {
    pub Version: u32,
    pub Reserved1: u32,
    pub LowestByteOffset: u64,
    pub HighestByteOffset: u64,
    pub AccessType: u32,
    pub AccessMode: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct VOLUME_GET_BC_PROPERTIES_OUTPUT {
    pub MaximumRequestsPerPeriod: u32,
    pub MinimumPeriod: u32,
    pub MaximumRequestSize: u64,
    pub EstimatedTimePerRequest: u32,
    pub NumOutStandingRequests: u32,
    pub RequestSize: u64,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct VOLUME_LOGICAL_OFFSET {
    pub LogicalOffset: i64,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct VOLUME_NUMBER {
    pub VolumeNumber: u32,
    pub VolumeManagerName: [u16; 8],
}
impl Default for VOLUME_NUMBER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct VOLUME_PHYSICAL_OFFSET {
    pub DiskNumber: u32,
    pub Offset: i64,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct VOLUME_PHYSICAL_OFFSETS {
    pub NumberOfPhysicalOffsets: u32,
    pub PhysicalOffset: [VOLUME_PHYSICAL_OFFSET; 1],
}
impl Default for VOLUME_PHYSICAL_OFFSETS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct VOLUME_READ_PLEX_INPUT {
    pub ByteOffset: i64,
    pub Length: u32,
    pub PlexNumber: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct VOLUME_SET_GPT_ATTRIBUTES_INFORMATION {
    pub GptAttributes: u64,
    pub RevertOnClose: bool,
    pub ApplyToAllConnectedVolumes: bool,
    pub Reserved1: u16,
    pub Reserved2: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct VOLUME_SHRINK_INFO {
    pub VolumeSize: u64,
}
