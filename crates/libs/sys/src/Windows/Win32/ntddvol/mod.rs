#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct FILE_EXTENT {
    pub VolumeOffset: super::ULONGLONG,
    pub ExtentLength: super::ULONGLONG,
}
pub const IOCTL_VOLUME_ALLOCATE_BC_STREAM: i32 = 5685312;
pub const IOCTL_VOLUME_BC_VERSION: i32 = 1;
pub const IOCTL_VOLUME_FREE_BC_STREAM: i32 = 5685316;
pub const IOCTL_VOLUME_GET_BC_PROPERTIES: i32 = 5652540;
pub const IOCTL_VOLUME_GET_CSVBLOCKCACHE_CALLBACK: i32 = 5685352;
pub const IOCTL_VOLUME_IS_CSV: i32 = 5636192;
pub const IOCTL_VOLUME_IS_DYNAMIC: i32 = 5636168;
pub const IOCTL_VOLUME_IS_IO_CAPABLE: i32 = 5636116;
pub const IOCTL_VOLUME_IS_OFFLINE: i32 = 5636112;
pub const IOCTL_VOLUME_IS_PARTITION: i32 = 5636136;
pub const IOCTL_VOLUME_LOGICAL_TO_PHYSICAL: i32 = 5636128;
pub const IOCTL_VOLUME_PHYSICAL_TO_LOGICAL: i32 = 5636132;
pub const IOCTL_VOLUME_POST_ONLINE: i32 = 5685348;
pub const IOCTL_VOLUME_PREPARE_FOR_CRITICAL_IO: i32 = 5685324;
pub const IOCTL_VOLUME_PREPARE_FOR_SHRINK: i32 = 5685340;
pub const IOCTL_VOLUME_QUERY_ALLOCATION_HINT: i32 = 5652562;
pub const IOCTL_VOLUME_QUERY_FAILOVER_SET: i32 = 5636120;
pub const IOCTL_VOLUME_QUERY_MINIMUM_SHRINK_SIZE: i32 = 5652568;
pub const IOCTL_VOLUME_QUERY_VOLUME_NUMBER: i32 = 5636124;
pub const IOCTL_VOLUME_READ_PLEX: i32 = 5652526;
pub const IOCTL_VOLUME_SET_GPT_ATTRIBUTES: i32 = 5636148;
pub const IOCTL_VOLUME_SUPPORTS_ONLINE_OFFLINE: i32 = 5636100;
pub const IOCTL_VOLUME_UPDATE_PROPERTIES: i32 = 5636180;
#[cfg(feature = "shared")]
pub type PFILE_EXTENT = *mut FILE_EXTENT;
#[cfg(feature = "shared")]
pub type PVOLUME_ALLOCATE_BC_STREAM_INPUT = *mut VOLUME_ALLOCATE_BC_STREAM_INPUT;
#[cfg(feature = "shared")]
pub type PVOLUME_ALLOCATE_BC_STREAM_OUTPUT = *mut VOLUME_ALLOCATE_BC_STREAM_OUTPUT;
#[cfg(feature = "shared")]
pub type PVOLUME_ALLOCATION_HINT_INPUT = *mut VOLUME_ALLOCATION_HINT_INPUT;
#[cfg(feature = "shared")]
pub type PVOLUME_ALLOCATION_HINT_OUTPUT = *mut VOLUME_ALLOCATION_HINT_OUTPUT;
#[cfg(feature = "shared")]
pub type PVOLUME_CRITICAL_IO = *mut VOLUME_CRITICAL_IO;
#[cfg(feature = "shared")]
pub type PVOLUME_FAILOVER_SET = *mut VOLUME_FAILOVER_SET;
#[cfg(feature = "shared")]
pub type PVOLUME_GET_BC_PROPERTIES_INPUT = *mut VOLUME_GET_BC_PROPERTIES_INPUT;
#[cfg(feature = "shared")]
pub type PVOLUME_GET_BC_PROPERTIES_OUTPUT = *mut VOLUME_GET_BC_PROPERTIES_OUTPUT;
#[cfg(feature = "shared")]
pub type PVOLUME_LOGICAL_OFFSET = *mut VOLUME_LOGICAL_OFFSET;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PVOLUME_NUMBER = *mut VOLUME_NUMBER;
#[cfg(feature = "shared")]
pub type PVOLUME_PHYSICAL_OFFSET = *mut VOLUME_PHYSICAL_OFFSET;
#[cfg(feature = "shared")]
pub type PVOLUME_PHYSICAL_OFFSETS = *mut VOLUME_PHYSICAL_OFFSETS;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PVOLUME_READ_PLEX_INPUT = *mut VOLUME_READ_PLEX_INPUT;
#[cfg(feature = "shared")]
pub type PVOLUME_SET_GPT_ATTRIBUTES_INFORMATION = *mut VOLUME_SET_GPT_ATTRIBUTES_INFORMATION;
#[cfg(feature = "shared")]
pub type PVOLUME_SHRINK_INFO = *mut VOLUME_SHRINK_INFO;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct VOLUME_ALLOCATE_BC_STREAM_INPUT {
    pub Version: super::ULONG,
    pub RequestsPerPeriod: super::ULONG,
    pub Period: super::ULONG,
    pub RetryFailures: super::BOOLEAN,
    pub Discardable: super::BOOLEAN,
    pub Reserved1: [super::BOOLEAN; 2],
    pub LowestByteOffset: super::ULONGLONG,
    pub HighestByteOffset: super::ULONGLONG,
    pub AccessType: super::ULONG,
    pub AccessMode: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for VOLUME_ALLOCATE_BC_STREAM_INPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct VOLUME_ALLOCATE_BC_STREAM_OUTPUT {
    pub RequestSize: super::ULONGLONG,
    pub NumOutStandingRequests: super::ULONG,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct VOLUME_ALLOCATION_HINT_INPUT {
    pub ClusterSize: super::ULONG,
    pub NumberOfClusters: super::ULONG,
    pub StartingClusterNumber: super::LONGLONG,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct VOLUME_ALLOCATION_HINT_OUTPUT {
    pub Bitmap: [super::ULONG; 1],
}
#[cfg(feature = "shared")]
impl Default for VOLUME_ALLOCATION_HINT_OUTPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct VOLUME_CRITICAL_IO {
    pub AccessType: super::ULONG,
    pub ExtentsCount: super::ULONG,
    pub Extents: [FILE_EXTENT; 1],
}
#[cfg(feature = "shared")]
impl Default for VOLUME_CRITICAL_IO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct VOLUME_FAILOVER_SET {
    pub NumberOfDisks: super::ULONG,
    pub DiskNumbers: [super::ULONG; 1],
}
#[cfg(feature = "shared")]
impl Default for VOLUME_FAILOVER_SET {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct VOLUME_GET_BC_PROPERTIES_INPUT {
    pub Version: super::ULONG,
    pub Reserved1: super::ULONG,
    pub LowestByteOffset: super::ULONGLONG,
    pub HighestByteOffset: super::ULONGLONG,
    pub AccessType: super::ULONG,
    pub AccessMode: super::ULONG,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct VOLUME_GET_BC_PROPERTIES_OUTPUT {
    pub MaximumRequestsPerPeriod: super::ULONG,
    pub MinimumPeriod: super::ULONG,
    pub MaximumRequestSize: super::ULONGLONG,
    pub EstimatedTimePerRequest: super::ULONG,
    pub NumOutStandingRequests: super::ULONG,
    pub RequestSize: super::ULONGLONG,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct VOLUME_LOGICAL_OFFSET {
    pub LogicalOffset: super::LONGLONG,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct VOLUME_NUMBER {
    pub VolumeNumber: super::ULONG,
    pub VolumeManagerName: [super::WCHAR; 8],
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for VOLUME_NUMBER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct VOLUME_PHYSICAL_OFFSET {
    pub DiskNumber: super::ULONG,
    pub Offset: super::LONGLONG,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct VOLUME_PHYSICAL_OFFSETS {
    pub NumberOfPhysicalOffsets: super::ULONG,
    pub PhysicalOffset: [VOLUME_PHYSICAL_OFFSET; 1],
}
#[cfg(feature = "shared")]
impl Default for VOLUME_PHYSICAL_OFFSETS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct VOLUME_READ_PLEX_INPUT {
    pub ByteOffset: super::LARGE_INTEGER,
    pub Length: super::ULONG,
    pub PlexNumber: super::ULONG,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for VOLUME_READ_PLEX_INPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct VOLUME_SET_GPT_ATTRIBUTES_INFORMATION {
    pub GptAttributes: super::ULONGLONG,
    pub RevertOnClose: super::BOOLEAN,
    pub ApplyToAllConnectedVolumes: super::BOOLEAN,
    pub Reserved1: super::USHORT,
    pub Reserved2: super::ULONG,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct VOLUME_SHRINK_INFO {
    pub VolumeSize: super::ULONGLONG,
}
