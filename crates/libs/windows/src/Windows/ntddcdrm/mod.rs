pub const ADR_ENCODES_CURRENT_POSITION: u32 = 1;
pub const ADR_ENCODES_ISRC: u32 = 3;
pub const ADR_ENCODES_MEDIA_CATALOG: u32 = 2;
pub const ADR_NO_MODE_INFORMATION: u32 = 0;
pub const AUDIO_DATA_TRACK: u32 = 4;
pub const AUDIO_STATUS_IN_PROGRESS: u32 = 17;
pub const AUDIO_STATUS_NOT_SUPPORTED: u32 = 0;
pub const AUDIO_STATUS_NO_STATUS: u32 = 21;
pub const AUDIO_STATUS_PAUSED: u32 = 18;
pub const AUDIO_STATUS_PLAY_COMPLETE: u32 = 19;
pub const AUDIO_STATUS_PLAY_ERROR: u32 = 20;
pub const AUDIO_WITH_PREEMPHASIS: u32 = 1;
pub const CDDA: TRACK_MODE_TYPE = 2;
pub const CDROM_CD_TEXT_PACK_ALBUM_NAME: u32 = 128;
pub const CDROM_CD_TEXT_PACK_ARRANGER: u32 = 132;
pub const CDROM_CD_TEXT_PACK_COMPOSER: u32 = 131;
pub const CDROM_CD_TEXT_PACK_DISC_ID: u32 = 134;
pub const CDROM_CD_TEXT_PACK_GENRE: u32 = 135;
pub const CDROM_CD_TEXT_PACK_MESSAGES: u32 = 133;
pub const CDROM_CD_TEXT_PACK_PERFORMER: u32 = 129;
pub const CDROM_CD_TEXT_PACK_SIZE_INFO: u32 = 143;
pub const CDROM_CD_TEXT_PACK_SONGWRITER: u32 = 130;
pub const CDROM_CD_TEXT_PACK_TOC_INFO: u32 = 136;
pub const CDROM_CD_TEXT_PACK_TOC_INFO2: u32 = 137;
pub const CDROM_CD_TEXT_PACK_UPC_EAN: u32 = 142;
pub const CDROM_DISK_AUDIO_TRACK: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CDROM_DISK_DATA {
    pub DiskData: u32,
}
pub const CDROM_DISK_DATA_TRACK: u32 = 2;
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CDROM_EXCEPTION_PERFORMANCE_DESCRIPTOR {
    pub Lba: [u8; 4],
    pub Time: [u8; 2],
}
impl Default for CDROM_EXCEPTION_PERFORMANCE_DESCRIPTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CDROM_EXCLUSIVE_ACCESS {
    pub RequestType: EXCLUSIVE_ACCESS_REQUEST_TYPE,
    pub Flags: u32,
}
pub const CDROM_EXCLUSIVE_CALLER_LENGTH: u32 = 64;
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CDROM_EXCLUSIVE_LOCK {
    pub Access: CDROM_EXCLUSIVE_ACCESS,
    pub CallerName: [u8; 64],
}
impl Default for CDROM_EXCLUSIVE_LOCK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CDROM_EXCLUSIVE_LOCK_STATE {
    pub LockState: bool,
    pub CallerName: [u8; 64],
}
impl Default for CDROM_EXCLUSIVE_LOCK_STATE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CDROM_IN_EXCLUSIVE_MODE: u32 = 1;
pub const CDROM_LOCK_IGNORE_VOLUME: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CDROM_NOMINAL_PERFORMANCE_DESCRIPTOR {
    pub StartLba: [u8; 4],
    pub StartPerformance: [u8; 4],
    pub EndLba: [u8; 4],
    pub EndPerformance: [u8; 4],
}
impl Default for CDROM_NOMINAL_PERFORMANCE_DESCRIPTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CDROM_NOT_IN_EXCLUSIVE_MODE: u32 = 0;
pub const CDROM_NO_MEDIA_NOTIFICATIONS: u32 = 2;
pub type CDROM_OPC_INFO_TYPE = i32;
pub type CDROM_PERFORMANCE_EXCEPTION_TYPE = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CDROM_PERFORMANCE_HEADER {
    pub DataLength: [u8; 4],
    pub _bitfield: u8,
    pub Reserved2: [u8; 3],
    pub Data: [u8; 0],
}
impl Default for CDROM_PERFORMANCE_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CDROM_PERFORMANCE_REQUEST {
    pub RequestType: CDROM_PERFORMANCE_REQUEST_TYPE,
    pub PerformanceType: CDROM_PERFORMANCE_TYPE,
    pub Exceptions: CDROM_PERFORMANCE_EXCEPTION_TYPE,
    pub Tolerance: CDROM_PERFORMANCE_TOLERANCE_TYPE,
    pub StaringLba: u32,
}
pub type CDROM_PERFORMANCE_REQUEST_TYPE = i32;
pub type CDROM_PERFORMANCE_TOLERANCE_TYPE = i32;
pub type CDROM_PERFORMANCE_TYPE = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CDROM_PLAY_AUDIO_MSF {
    pub StartingM: u8,
    pub StartingS: u8,
    pub StartingF: u8,
    pub EndingM: u8,
    pub EndingS: u8,
    pub EndingF: u8,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CDROM_READ_TOC_EX {
    pub _bitfield: u8,
    pub SessionTrack: u8,
    pub Reserved2: u8,
    pub Reserved3: u8,
}
pub const CDROM_READ_TOC_EX_FORMAT_ATIP: u32 = 4;
pub const CDROM_READ_TOC_EX_FORMAT_CDTEXT: u32 = 5;
pub const CDROM_READ_TOC_EX_FORMAT_FULL_TOC: u32 = 2;
pub const CDROM_READ_TOC_EX_FORMAT_PMA: u32 = 3;
pub const CDROM_READ_TOC_EX_FORMAT_SESSION: u32 = 1;
pub const CDROM_READ_TOC_EX_FORMAT_TOC: u32 = 0;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CDROM_SEEK_AUDIO_MSF {
    pub M: u8,
    pub S: u8,
    pub F: u8,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CDROM_SET_SPEED {
    pub RequestType: CDROM_SPEED_REQUEST,
    pub ReadSpeed: u16,
    pub WriteSpeed: u16,
    pub RotationControl: WRITE_ROTATION,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CDROM_SET_STREAMING {
    pub RequestType: CDROM_SPEED_REQUEST,
    pub ReadSize: u32,
    pub ReadTime: u32,
    pub WriteSize: u32,
    pub WriteTime: u32,
    pub StartLba: u32,
    pub EndLba: u32,
    pub RotationControl: WRITE_ROTATION,
    pub RestoreDefaults: bool,
    pub SetExact: bool,
    pub RandomAccess: bool,
    pub Persistent: bool,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CDROM_SIMPLE_OPC_INFO {
    pub RequestType: CDROM_OPC_INFO_TYPE,
    pub Exclude0: bool,
    pub Exclude1: bool,
}
pub type CDROM_SPEED_REQUEST = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CDROM_STREAMING_CONTROL {
    pub RequestType: STREAMING_CONTROL_REQUEST_TYPE,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CDROM_SUB_Q_DATA_FORMAT {
    pub Format: u8,
    pub Track: u8,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CDROM_TOC {
    pub Length: [u8; 2],
    pub FirstTrack: u8,
    pub LastTrack: u8,
    pub TrackData: [TRACK_DATA; 100],
}
impl Default for CDROM_TOC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CDROM_TOC_ATIP_DATA {
    pub Length: [u8; 2],
    pub Reserved1: u8,
    pub Reserved2: u8,
    pub Descriptors: [CDROM_TOC_ATIP_DATA_BLOCK; 0],
}
impl Default for CDROM_TOC_ATIP_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CDROM_TOC_ATIP_DATA_BLOCK {
    pub _bitfield1: u8,
    pub _bitfield2: u8,
    pub _bitfield3: u8,
    pub Reserved7: u8,
    pub LeadInMsf: [u8; 3],
    pub Reserved8: u8,
    pub LeadOutMsf: [u8; 3],
    pub Reserved9: u8,
    pub A1Values: [u8; 3],
    pub Reserved10: u8,
    pub A2Values: [u8; 3],
    pub Reserved11: u8,
    pub A3Values: [u8; 3],
    pub Reserved12: u8,
}
impl Default for CDROM_TOC_ATIP_DATA_BLOCK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CDROM_TOC_CD_TEXT_DATA {
    pub Length: [u8; 2],
    pub Reserved1: u8,
    pub Reserved2: u8,
    pub Descriptors: [CDROM_TOC_CD_TEXT_DATA_BLOCK; 0],
}
impl Default for CDROM_TOC_CD_TEXT_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CDROM_TOC_CD_TEXT_DATA_BLOCK {
    pub PackType: u8,
    pub _bitfield1: u8,
    pub SequenceNumber: u8,
    pub _bitfield2: u8,
    pub Anonymous: CDROM_TOC_CD_TEXT_DATA_BLOCK_0,
    pub CRC: [u8; 2],
}
impl Default for CDROM_TOC_CD_TEXT_DATA_BLOCK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union CDROM_TOC_CD_TEXT_DATA_BLOCK_0 {
    pub Text: [u8; 12],
    pub WText: [u16; 6],
}
impl Default for CDROM_TOC_CD_TEXT_DATA_BLOCK_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CDROM_TOC_FULL_TOC_DATA {
    pub Length: [u8; 2],
    pub FirstCompleteSession: u8,
    pub LastCompleteSession: u8,
    pub Descriptors: [CDROM_TOC_FULL_TOC_DATA_BLOCK; 0],
}
impl Default for CDROM_TOC_FULL_TOC_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CDROM_TOC_FULL_TOC_DATA_BLOCK {
    pub SessionNumber: u8,
    pub _bitfield: u8,
    pub Reserved1: u8,
    pub Point: u8,
    pub MsfExtra: [u8; 3],
    pub Zero: u8,
    pub Msf: [u8; 3],
}
impl Default for CDROM_TOC_FULL_TOC_DATA_BLOCK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CDROM_TOC_PMA_DATA {
    pub Length: [u8; 2],
    pub Reserved1: u8,
    pub Reserved2: u8,
    pub Descriptors: [CDROM_TOC_FULL_TOC_DATA_BLOCK; 0],
}
impl Default for CDROM_TOC_PMA_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CDROM_TOC_SESSION_DATA {
    pub Length: [u8; 2],
    pub FirstCompleteSession: u8,
    pub LastCompleteSession: u8,
    pub TrackData: [TRACK_DATA; 1],
}
impl Default for CDROM_TOC_SESSION_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CDROM_WRITE_SPEED_DESCRIPTOR {
    pub _bitfield: u8,
    pub Reserved3: [u8; 3],
    pub EndLba: [u8; 4],
    pub ReadSpeed: [u8; 4],
    pub WriteSpeed: [u8; 4],
}
impl Default for CDROM_WRITE_SPEED_DESCRIPTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CDROM_WRITE_SPEED_REQUEST {
    pub RequestType: CDROM_PERFORMANCE_REQUEST_TYPE,
}
pub const CD_RAW_READ_C2_SIZE: u32 = 296;
pub const CD_RAW_READ_SUBCODE_SIZE: u32 = 96;
pub const CD_RAW_SECTOR_WITH_C2_AND_SUBCODE_SIZE: u32 = 2744;
pub const CD_RAW_SECTOR_WITH_C2_SIZE: u32 = 2648;
pub const CD_RAW_SECTOR_WITH_SUBCODE_SIZE: u32 = 2448;
pub const Cdrom10Nominal20Exceptions: CDROM_PERFORMANCE_TOLERANCE_TYPE = 1;
pub const CdromCAVRotation: WRITE_ROTATION = 1;
pub const CdromDefaultRotation: WRITE_ROTATION = 0;
pub const CdromEntirePerformanceList: CDROM_PERFORMANCE_EXCEPTION_TYPE = 2;
pub const CdromNominalPerformance: CDROM_PERFORMANCE_EXCEPTION_TYPE = 1;
pub const CdromPerformanceExceptionsOnly: CDROM_PERFORMANCE_EXCEPTION_TYPE = 3;
pub const CdromPerformanceRequest: CDROM_PERFORMANCE_REQUEST_TYPE = 1;
pub const CdromReadPerformance: CDROM_PERFORMANCE_TYPE = 1;
pub const CdromSetSpeed: CDROM_SPEED_REQUEST = 0;
pub const CdromSetStreaming: CDROM_SPEED_REQUEST = 1;
pub const CdromStreamingDisable: STREAMING_CONTROL_REQUEST_TYPE = 1;
pub const CdromStreamingEnableForReadOnly: STREAMING_CONTROL_REQUEST_TYPE = 2;
pub const CdromStreamingEnableForReadWrite: STREAMING_CONTROL_REQUEST_TYPE = 4;
pub const CdromStreamingEnableForWriteOnly: STREAMING_CONTROL_REQUEST_TYPE = 3;
pub const CdromWritePerformance: CDROM_PERFORMANCE_TYPE = 2;
pub const CdromWriteSpeedRequest: CDROM_PERFORMANCE_REQUEST_TYPE = 2;
pub const DIGITAL_COPY_PERMITTED: u32 = 2;
pub type EXCLUSIVE_ACCESS_REQUEST_TYPE = i32;
pub const ExclusiveAccessLockDevice: EXCLUSIVE_ACCESS_REQUEST_TYPE = 1;
pub const ExclusiveAccessQueryState: EXCLUSIVE_ACCESS_REQUEST_TYPE = 0;
pub const ExclusiveAccessUnlockDevice: EXCLUSIVE_ACCESS_REQUEST_TYPE = 2;
pub const IOCTL_CDROM_BASE: u32 = 2;
pub const IOCTL_CDROM_CHECK_VERIFY: u32 = 149504;
pub const IOCTL_CDROM_CURRENT_POSITION: u32 = 1;
pub const IOCTL_CDROM_DISK_TYPE: u32 = 131136;
pub const IOCTL_CDROM_EJECT_MEDIA: u32 = 149512;
pub const IOCTL_CDROM_ENABLE_STREAMING: u32 = 147560;
pub const IOCTL_CDROM_EXCLUSIVE_ACCESS: u32 = 180316;
pub const IOCTL_CDROM_FIND_NEW_DEVICES: u32 = 149528;
pub const IOCTL_CDROM_GET_CONFIGURATION: u32 = 147544;
pub const IOCTL_CDROM_GET_DRIVE_GEOMETRY: u32 = 147532;
pub const IOCTL_CDROM_GET_DRIVE_GEOMETRY_EX: u32 = 147536;
pub const IOCTL_CDROM_GET_INQUIRY_DATA: u32 = 147556;
pub const IOCTL_CDROM_GET_LAST_SESSION: u32 = 147512;
pub const IOCTL_CDROM_GET_PERFORMANCE: u32 = 147568;
pub const IOCTL_CDROM_GET_VOLUME: u32 = 147476;
pub const IOCTL_CDROM_LOAD_MEDIA: u32 = 149516;
pub const IOCTL_CDROM_MEDIA_CATALOG: u32 = 2;
pub const IOCTL_CDROM_MEDIA_REMOVAL: u32 = 149508;
pub const IOCTL_CDROM_PAUSE_AUDIO: u32 = 147468;
pub const IOCTL_CDROM_PLAY_AUDIO_MSF: u32 = 147480;
pub const IOCTL_CDROM_RAW_READ: u32 = 147518;
pub const IOCTL_CDROM_READ_Q_CHANNEL: u32 = 147500;
pub const IOCTL_CDROM_READ_TOC: u32 = 147456;
pub const IOCTL_CDROM_READ_TOC_EX: u32 = 147540;
pub const IOCTL_CDROM_RELEASE: u32 = 149524;
pub const IOCTL_CDROM_RESERVE: u32 = 149520;
pub const IOCTL_CDROM_RESUME_AUDIO: u32 = 147472;
pub const IOCTL_CDROM_SEEK_AUDIO_MSF: u32 = 147460;
pub const IOCTL_CDROM_SEND_OPC_INFORMATION: u32 = 180332;
pub const IOCTL_CDROM_SET_SPEED: u32 = 147552;
pub const IOCTL_CDROM_SET_VOLUME: u32 = 147496;
pub const IOCTL_CDROM_SIMBAD: u32 = 147468;
pub const IOCTL_CDROM_STOP_AUDIO: u32 = 147464;
pub const IOCTL_CDROM_SUB_Q_CHANNEL: u32 = 0;
pub const IOCTL_CDROM_TRACK_ISRC: u32 = 3;
pub const IOCTL_CDROM_UNLOAD_DRIVER: u32 = 151560;
pub const MAXIMUM_CDROM_INQUIRY_SIZE: u32 = 260;
pub const MAXIMUM_CDROM_SIZE: u32 = 804;
pub const MAXIMUM_NUMBER_TRACKS: u32 = 100;
pub type MEDIA_BLANK_TYPE = i32;
pub const MINIMUM_CDROM_INQUIRY_SIZE: u32 = 36;
pub const MINIMUM_CDROM_READ_TOC_EX_SIZE: u32 = 2;
pub const MediaBlankTypeEraseLastSession: MEDIA_BLANK_TYPE = 6;
pub const MediaBlankTypeFull: MEDIA_BLANK_TYPE = 0;
pub const MediaBlankTypeIncompleteTrack: MEDIA_BLANK_TYPE = 2;
pub const MediaBlankTypeMinimal: MEDIA_BLANK_TYPE = 1;
pub const MediaBlankTypeTrackTail: MEDIA_BLANK_TYPE = 4;
pub const MediaBlankTypeUncloseLastSession: MEDIA_BLANK_TYPE = 5;
pub const MediaBlankTypeUnreserveLastTrack: MEDIA_BLANK_TYPE = 3;
pub const OBSOLETE_IOCTL_CDROM_GET_CONTROL: u32 = 147508;
pub type PCDROM_DISK_DATA = *mut CDROM_DISK_DATA;
pub type PCDROM_EXCEPTION_PERFORMANCE_DESCRIPTOR = *mut CDROM_EXCEPTION_PERFORMANCE_DESCRIPTOR;
pub type PCDROM_EXCLUSIVE_ACCESS = *mut CDROM_EXCLUSIVE_ACCESS;
pub type PCDROM_EXCLUSIVE_LOCK = *mut CDROM_EXCLUSIVE_LOCK;
pub type PCDROM_EXCLUSIVE_LOCK_STATE = *mut CDROM_EXCLUSIVE_LOCK_STATE;
pub type PCDROM_NOMINAL_PERFORMANCE_DESCRIPTOR = *mut CDROM_NOMINAL_PERFORMANCE_DESCRIPTOR;
pub type PCDROM_OPC_INFO_TYPE = *mut CDROM_OPC_INFO_TYPE;
pub type PCDROM_PERFORMANCE_EXCEPTION_TYPE = *mut CDROM_PERFORMANCE_EXCEPTION_TYPE;
pub type PCDROM_PERFORMANCE_HEADER = *mut CDROM_PERFORMANCE_HEADER;
pub type PCDROM_PERFORMANCE_REQUEST = *mut CDROM_PERFORMANCE_REQUEST;
pub type PCDROM_PERFORMANCE_REQUEST_TYPE = *mut CDROM_PERFORMANCE_REQUEST_TYPE;
pub type PCDROM_PERFORMANCE_TOLERANCE_TYPE = *mut CDROM_PERFORMANCE_TOLERANCE_TYPE;
pub type PCDROM_PERFORMANCE_TYPE = *mut CDROM_PERFORMANCE_TYPE;
pub type PCDROM_PLAY_AUDIO_MSF = *mut CDROM_PLAY_AUDIO_MSF;
pub type PCDROM_READ_TOC_EX = *mut CDROM_READ_TOC_EX;
pub type PCDROM_SEEK_AUDIO_MSF = *mut CDROM_SEEK_AUDIO_MSF;
pub type PCDROM_SET_SPEED = *mut CDROM_SET_SPEED;
pub type PCDROM_SET_STREAMING = *mut CDROM_SET_STREAMING;
pub type PCDROM_SIMPLE_OPC_INFO = *mut CDROM_SIMPLE_OPC_INFO;
pub type PCDROM_SPEED_REQUEST = *mut CDROM_SPEED_REQUEST;
pub type PCDROM_STREAMING_CONTROL = *mut CDROM_STREAMING_CONTROL;
pub type PCDROM_SUB_Q_DATA_FORMAT = *mut CDROM_SUB_Q_DATA_FORMAT;
pub type PCDROM_TOC = *mut CDROM_TOC;
pub type PCDROM_TOC_ATIP_DATA = *mut CDROM_TOC_ATIP_DATA;
pub type PCDROM_TOC_ATIP_DATA_BLOCK = *mut CDROM_TOC_ATIP_DATA_BLOCK;
pub type PCDROM_TOC_CD_TEXT_DATA = *mut CDROM_TOC_CD_TEXT_DATA;
pub type PCDROM_TOC_CD_TEXT_DATA_BLOCK = *mut CDROM_TOC_CD_TEXT_DATA_BLOCK;
pub type PCDROM_TOC_FULL_TOC_DATA = *mut CDROM_TOC_FULL_TOC_DATA;
pub type PCDROM_TOC_FULL_TOC_DATA_BLOCK = *mut CDROM_TOC_FULL_TOC_DATA_BLOCK;
pub type PCDROM_TOC_PMA_DATA = *mut CDROM_TOC_PMA_DATA;
pub type PCDROM_TOC_SESSION_DATA = *mut CDROM_TOC_SESSION_DATA;
pub type PCDROM_WRITE_SPEED_DESCRIPTOR = *mut CDROM_WRITE_SPEED_DESCRIPTOR;
pub type PCDROM_WRITE_SPEED_REQUEST = *mut CDROM_WRITE_SPEED_REQUEST;
pub type PEXCLUSIVE_ACCESS_REQUEST_TYPE = *mut EXCLUSIVE_ACCESS_REQUEST_TYPE;
pub type PMEDIA_BLANK_TYPE = *mut MEDIA_BLANK_TYPE;
pub type PRAW_READ_INFO = *mut RAW_READ_INFO;
pub type PSTREAMING_CONTROL_REQUEST_TYPE = *mut STREAMING_CONTROL_REQUEST_TYPE;
pub type PSUB_Q_CHANNEL_DATA = *mut SUB_Q_CHANNEL_DATA;
pub type PSUB_Q_CURRENT_POSITION = *mut SUB_Q_CURRENT_POSITION;
pub type PSUB_Q_HEADER = *mut SUB_Q_HEADER;
pub type PSUB_Q_MEDIA_CATALOG_NUMBER = *mut SUB_Q_MEDIA_CATALOG_NUMBER;
pub type PSUB_Q_TRACK_ISRC = *mut SUB_Q_TRACK_ISRC;
pub type PTRACK_DATA = *mut TRACK_DATA;
pub type PTRACK_MODE_TYPE = *mut TRACK_MODE_TYPE;
pub type PVOLUME_CONTROL = *mut VOLUME_CONTROL;
pub type PWRITE_ROTATION = *mut WRITE_ROTATION;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RAW_READ_INFO {
    pub DiskOffset: i64,
    pub SectorCount: u32,
    pub TrackMode: TRACK_MODE_TYPE,
}
pub const RawWithC2: TRACK_MODE_TYPE = 4;
pub const RawWithC2AndSubCode: TRACK_MODE_TYPE = 3;
pub const RawWithSubCode: TRACK_MODE_TYPE = 5;
pub type STREAMING_CONTROL_REQUEST_TYPE = i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub union SUB_Q_CHANNEL_DATA {
    pub CurrentPosition: SUB_Q_CURRENT_POSITION,
    pub MediaCatalog: SUB_Q_MEDIA_CATALOG_NUMBER,
    pub TrackIsrc: SUB_Q_TRACK_ISRC,
}
impl Default for SUB_Q_CHANNEL_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SUB_Q_CURRENT_POSITION {
    pub Header: SUB_Q_HEADER,
    pub FormatCode: u8,
    pub _bitfield: u8,
    pub TrackNumber: u8,
    pub IndexNumber: u8,
    pub AbsoluteAddress: [u8; 4],
    pub TrackRelativeAddress: [u8; 4],
}
impl Default for SUB_Q_CURRENT_POSITION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SUB_Q_HEADER {
    pub Reserved: u8,
    pub AudioStatus: u8,
    pub DataLength: [u8; 2],
}
impl Default for SUB_Q_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SUB_Q_MEDIA_CATALOG_NUMBER {
    pub Header: SUB_Q_HEADER,
    pub FormatCode: u8,
    pub Reserved: [u8; 3],
    pub _bitfield: u8,
    pub MediaCatalog: [u8; 15],
}
impl Default for SUB_Q_MEDIA_CATALOG_NUMBER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SUB_Q_TRACK_ISRC {
    pub Header: SUB_Q_HEADER,
    pub FormatCode: u8,
    pub Reserved0: u8,
    pub Track: u8,
    pub Reserved1: u8,
    pub _bitfield: u8,
    pub TrackIsrc: [u8; 15],
}
impl Default for SUB_Q_TRACK_ISRC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SimpleOpcInfo: CDROM_OPC_INFO_TYPE = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TRACK_DATA {
    pub Reserved: u8,
    pub _bitfield: u8,
    pub TrackNumber: u8,
    pub Reserved1: u8,
    pub Address: [u8; 4],
}
impl Default for TRACK_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type TRACK_MODE_TYPE = i32;
pub const TWO_FOUR_CHANNEL_AUDIO: u32 = 8;
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VOLUME_CONTROL {
    pub PortVolume: [u8; 4],
}
impl Default for VOLUME_CONTROL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type WRITE_ROTATION = i32;
pub const XAForm2: TRACK_MODE_TYPE = 1;
pub const YellowMode2: TRACK_MODE_TYPE = 0;
