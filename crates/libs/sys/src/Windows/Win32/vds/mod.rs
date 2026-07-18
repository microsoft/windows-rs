pub const BeepAlarm: VDS_MAINTENANCE_OPERATION = 2;
pub const BlinkLight: VDS_MAINTENANCE_OPERATION = 1;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CHANGE_ATTRIBUTES_PARAMETERS {
    pub style: VDS_PARTITION_STYLE,
    pub Anonymous: CHANGE_ATTRIBUTES_PARAMETERS_0,
}
impl Default for CHANGE_ATTRIBUTES_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union CHANGE_ATTRIBUTES_PARAMETERS_0 {
    pub MbrPartInfo: CHANGE_ATTRIBUTES_PARAMETERS_0_0,
    pub GptPartInfo: CHANGE_ATTRIBUTES_PARAMETERS_0_1,
}
impl Default for CHANGE_ATTRIBUTES_PARAMETERS_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct CHANGE_ATTRIBUTES_PARAMETERS_0_0 {
    pub bootIndicator: bool,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct CHANGE_ATTRIBUTES_PARAMETERS_0_1 {
    pub attributes: u64,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CHANGE_PARTITION_TYPE_PARAMETERS {
    pub style: VDS_PARTITION_STYLE,
    pub Anonymous: CHANGE_PARTITION_TYPE_PARAMETERS_0,
}
impl Default for CHANGE_PARTITION_TYPE_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union CHANGE_PARTITION_TYPE_PARAMETERS_0 {
    pub MbrPartInfo: CHANGE_PARTITION_TYPE_PARAMETERS_0_0,
    pub GptPartInfo: CHANGE_PARTITION_TYPE_PARAMETERS_0_1,
}
impl Default for CHANGE_PARTITION_TYPE_PARAMETERS_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct CHANGE_PARTITION_TYPE_PARAMETERS_0_0 {
    pub partitionType: u8,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct CHANGE_PARTITION_TYPE_PARAMETERS_0_1 {
    pub partitionType: windows_sys::core::GUID,
}
pub const CLSID_VdsLoader: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x9c38ed61_d565_4728_aeee_c80952f0ecde);
pub const CLSID_VdsService: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x7d1933cb_86f6_4a98_8628_01be94c9a575);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CREATE_PARTITION_PARAMETERS {
    pub style: VDS_PARTITION_STYLE,
    pub Anonymous: CREATE_PARTITION_PARAMETERS_0,
}
impl Default for CREATE_PARTITION_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union CREATE_PARTITION_PARAMETERS_0 {
    pub MbrPartInfo: CREATE_PARTITION_PARAMETERS_0_0,
    pub GptPartInfo: CREATE_PARTITION_PARAMETERS_0_1,
}
impl Default for CREATE_PARTITION_PARAMETERS_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct CREATE_PARTITION_PARAMETERS_0_0 {
    pub partitionType: u8,
    pub bootIndicator: bool,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CREATE_PARTITION_PARAMETERS_0_1 {
    pub partitionType: windows_sys::core::GUID,
    pub partitionId: windows_sys::core::GUID,
    pub attributes: u64,
    pub name: [u16; 36],
}
impl Default for CREATE_PARTITION_PARAMETERS_0_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const GPT_PARTITION_NAME_LENGTH: u32 = 36;
pub const MAX_FS_ALLOWED_CLUSTER_SIZES_SIZE: u32 = 32;
pub const MAX_FS_FORMAT_SUPPORT_NAME_SIZE: u32 = 32;
pub const MAX_FS_NAME_SIZE: u32 = 8;
#[cfg(feature = "vdslun")]
pub type PVDS_ADVANCEDDISK_PROP = *mut VDS_ADVANCEDDISK_PROP;
pub type PVDS_CONTROLLER_PROP = *mut VDS_CONTROLLER_PROP;
pub type PVDS_CONTROLLER_STATUS = *mut VDS_CONTROLLER_STATUS;
pub type PVDS_CREATE_VDISK_PARAMETERS = *mut VDS_CREATE_VDISK_PARAMETERS;
pub type PVDS_DISK_EXTENT = *mut VDS_DISK_EXTENT;
pub type PVDS_DISK_FREE_EXTENT = *mut VDS_DISK_FREE_EXTENT;
#[cfg(feature = "vdslun")]
pub type PVDS_DISK_PROP = *mut VDS_DISK_PROP;
#[cfg(feature = "vdslun")]
pub type PVDS_DISK_PROP2 = *mut VDS_DISK_PROP2;
pub type PVDS_DRIVE_EXTENT = *mut VDS_DRIVE_EXTENT;
pub type PVDS_DRIVE_FLAG = *mut VDS_DRIVE_FLAG;
pub type PVDS_DRIVE_LETTER_PROP = *mut VDS_DRIVE_LETTER_PROP;
pub type PVDS_DRIVE_PROP = *mut VDS_DRIVE_PROP;
#[cfg(feature = "vdslun")]
pub type PVDS_DRIVE_PROP2 = *mut VDS_DRIVE_PROP2;
pub type PVDS_DRIVE_STATUS = *mut VDS_DRIVE_STATUS;
pub type PVDS_FILE_SYSTEM_FORMAT_SUPPORT_PROP = *mut VDS_FILE_SYSTEM_FORMAT_SUPPORT_PROP;
pub type PVDS_FILE_SYSTEM_PROP = *mut VDS_FILE_SYSTEM_PROP;
pub type PVDS_FILE_SYSTEM_TYPE_PROP = *mut VDS_FILE_SYSTEM_TYPE_PROP;
pub type PVDS_HINTS = *mut VDS_HINTS;
#[cfg(feature = "vdslun")]
pub type PVDS_HINTS2 = *mut VDS_HINTS2;
pub type PVDS_INTERCONNECT_FLAG = *mut VDS_INTERCONNECT_FLAG;
pub type PVDS_ISCSI_PORTALGROUP_PROP = *mut VDS_ISCSI_PORTALGROUP_PROP;
pub type PVDS_ISCSI_PORTAL_PROP = *mut VDS_ISCSI_PORTAL_PROP;
pub type PVDS_ISCSI_TARGET_PROP = *mut VDS_ISCSI_TARGET_PROP;
pub type PVDS_LUN_FLAG = *mut VDS_LUN_FLAG;
pub type PVDS_LUN_PLEX_PROP = *mut VDS_LUN_PLEX_PROP;
pub type PVDS_LUN_PROP = *mut VDS_LUN_PROP;
pub type PVDS_LUN_STATUS = *mut VDS_LUN_STATUS;
pub type PVDS_LUN_TYPE = *mut VDS_LUN_TYPE;
pub type PVDS_PACK_PROP = *mut VDS_PACK_PROP;
#[cfg(feature = "vdslun")]
pub type PVDS_POOL_ATTRIBUTES = *mut VDS_POOL_ATTRIBUTES;
pub type PVDS_POOL_CUSTOM_ATTRIBUTES = *mut VDS_POOL_CUSTOM_ATTRIBUTES;
pub type PVDS_PORT_PROP = *mut VDS_PORT_PROP;
pub type PVDS_PORT_STATUS = *mut VDS_PORT_STATUS;
pub type PVDS_RAID_TYPE = *mut VDS_RAID_TYPE;
pub type PVDS_REPARSE_POINT_PROP = *mut VDS_REPARSE_POINT_PROP;
pub type PVDS_STORAGE_POOL_DRIVE_EXTENT = *mut VDS_STORAGE_POOL_DRIVE_EXTENT;
pub type PVDS_STORAGE_POOL_PROP = *mut VDS_STORAGE_POOL_PROP;
pub type PVDS_SUB_SYSTEM_FLAG = *mut VDS_SUB_SYSTEM_FLAG;
pub type PVDS_SUB_SYSTEM_PROP = *mut VDS_SUB_SYSTEM_PROP;
pub type PVDS_SUB_SYSTEM_PROP2 = *mut VDS_SUB_SYSTEM_PROP2;
pub type PVDS_SUB_SYSTEM_STATUS = *mut VDS_SUB_SYSTEM_STATUS;
pub type PVDS_SUB_SYSTEM_SUPPORTED_RAID_TYPE_FLAG = *mut VDS_SUB_SYSTEM_SUPPORTED_RAID_TYPE_FLAG;
#[cfg(all(feature = "virtdisk", feature = "winioctl"))]
pub type PVDS_VDISK_PROPERTIES = *mut VDS_VDISK_PROPERTIES;
pub type PVDS_VOLUME_PLEX_PROP = *mut VDS_VOLUME_PLEX_PROP;
pub type PVDS_VOLUME_PROP = *mut VDS_VOLUME_PROP;
pub type PVDS_VOLUME_PROP2 = *mut VDS_VOLUME_PROP2;
pub const Ping: VDS_MAINTENANCE_OPERATION = 5;
pub const SpinDown: VDS_MAINTENANCE_OPERATION = 3;
pub const SpinUp: VDS_MAINTENANCE_OPERATION = 4;
pub const VDSDiskOfflineReasonCollision: VDS_DISK_OFFLINE_REASON = 4;
pub const VDSDiskOfflineReasonDIScan: VDS_DISK_OFFLINE_REASON = 7;
pub const VDSDiskOfflineReasonLostDataPersistence: VDS_DISK_OFFLINE_REASON = 8;
pub const VDSDiskOfflineReasonNone: VDS_DISK_OFFLINE_REASON = 0;
pub const VDSDiskOfflineReasonPolicy: VDS_DISK_OFFLINE_REASON = 1;
pub const VDSDiskOfflineReasonRedundantPath: VDS_DISK_OFFLINE_REASON = 2;
pub const VDSDiskOfflineReasonResourceExhaustion: VDS_DISK_OFFLINE_REASON = 5;
pub const VDSDiskOfflineReasonSnapshot: VDS_DISK_OFFLINE_REASON = 3;
pub const VDSDiskOfflineReasonWriteFailure: VDS_DISK_OFFLINE_REASON = 6;
#[repr(C)]
#[cfg(feature = "vdslun")]
#[derive(Clone, Copy)]
pub struct VDS_ADVANCEDDISK_PROP {
    pub pwszId: windows_sys::core::PWSTR,
    pub pwszPathname: windows_sys::core::PWSTR,
    pub pwszLocation: windows_sys::core::PWSTR,
    pub pwszFriendlyName: windows_sys::core::PWSTR,
    pub pswzIdentifier: windows_sys::core::PWSTR,
    pub usIdentifierFormat: u16,
    pub ulNumber: u32,
    pub pwszSerialNumber: windows_sys::core::PWSTR,
    pub pwszFirmwareVersion: windows_sys::core::PWSTR,
    pub pwszManufacturer: windows_sys::core::PWSTR,
    pub pwszModel: windows_sys::core::PWSTR,
    pub ullTotalSize: u64,
    pub ullAllocatedSize: u64,
    pub ulLogicalSectorSize: u32,
    pub ulPhysicalSectorSize: u32,
    pub ulPartitionCount: u32,
    pub status: VDS_DISK_STATUS,
    pub health: VDS_HEALTH,
    pub BusType: super::VDS_STORAGE_BUS_TYPE,
    pub PartitionStyle: VDS_PARTITION_STYLE,
    pub Anonymous: VDS_ADVANCEDDISK_PROP_0,
    pub ulFlags: u32,
    pub dwDeviceType: u32,
}
#[cfg(feature = "vdslun")]
impl Default for VDS_ADVANCEDDISK_PROP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "vdslun")]
#[derive(Clone, Copy)]
pub union VDS_ADVANCEDDISK_PROP_0 {
    pub dwSignature: u32,
    pub DiskGuid: windows_sys::core::GUID,
}
#[cfg(feature = "vdslun")]
impl Default for VDS_ADVANCEDDISK_PROP_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const VDS_ASYNCOUT_ADDLUNPLEX: VDS_ASYNC_OUTPUT_TYPE = 52;
pub const VDS_ASYNCOUT_ADDPORTAL: VDS_ASYNC_OUTPUT_TYPE = 65;
pub const VDS_ASYNCOUT_ADDVOLUMEPLEX: VDS_ASYNC_OUTPUT_TYPE = 4;
pub const VDS_ASYNCOUT_ATTACH_VDISK: VDS_ASYNC_OUTPUT_TYPE = 201;
pub const VDS_ASYNCOUT_BREAKVOLUMEPLEX: VDS_ASYNC_OUTPUT_TYPE = 5;
pub const VDS_ASYNCOUT_CLEAN: VDS_ASYNC_OUTPUT_TYPE = 11;
pub const VDS_ASYNCOUT_COMPACT_VDISK: VDS_ASYNC_OUTPUT_TYPE = 202;
pub const VDS_ASYNCOUT_CREATELUN: VDS_ASYNC_OUTPUT_TYPE = 50;
pub const VDS_ASYNCOUT_CREATEPARTITION: VDS_ASYNC_OUTPUT_TYPE = 10;
pub const VDS_ASYNCOUT_CREATEPORTALGROUP: VDS_ASYNC_OUTPUT_TYPE = 63;
pub const VDS_ASYNCOUT_CREATETARGET: VDS_ASYNC_OUTPUT_TYPE = 62;
pub const VDS_ASYNCOUT_CREATEVOLUME: VDS_ASYNC_OUTPUT_TYPE = 1;
pub const VDS_ASYNCOUT_CREATE_VDISK: VDS_ASYNC_OUTPUT_TYPE = 200;
pub const VDS_ASYNCOUT_DELETEPORTALGROUP: VDS_ASYNC_OUTPUT_TYPE = 67;
pub const VDS_ASYNCOUT_DELETETARGET: VDS_ASYNC_OUTPUT_TYPE = 64;
pub const VDS_ASYNCOUT_EXPAND_VDISK: VDS_ASYNC_OUTPUT_TYPE = 204;
pub const VDS_ASYNCOUT_EXTENDLUN: VDS_ASYNC_OUTPUT_TYPE = 54;
pub const VDS_ASYNCOUT_EXTENDVOLUME: VDS_ASYNC_OUTPUT_TYPE = 2;
pub const VDS_ASYNCOUT_FORMAT: VDS_ASYNC_OUTPUT_TYPE = 101;
pub const VDS_ASYNCOUT_LOGINTOTARGET: VDS_ASYNC_OUTPUT_TYPE = 60;
pub const VDS_ASYNCOUT_LOGOUTFROMTARGET: VDS_ASYNC_OUTPUT_TYPE = 61;
pub const VDS_ASYNCOUT_MERGE_VDISK: VDS_ASYNC_OUTPUT_TYPE = 203;
pub const VDS_ASYNCOUT_RECOVERLUN: VDS_ASYNC_OUTPUT_TYPE = 56;
pub const VDS_ASYNCOUT_RECOVERPACK: VDS_ASYNC_OUTPUT_TYPE = 8;
pub const VDS_ASYNCOUT_REMOVELUNPLEX: VDS_ASYNC_OUTPUT_TYPE = 53;
pub const VDS_ASYNCOUT_REMOVEPORTAL: VDS_ASYNC_OUTPUT_TYPE = 66;
pub const VDS_ASYNCOUT_REMOVEVOLUMEPLEX: VDS_ASYNC_OUTPUT_TYPE = 6;
pub const VDS_ASYNCOUT_REPAIRVOLUMEPLEX: VDS_ASYNC_OUTPUT_TYPE = 7;
pub const VDS_ASYNCOUT_REPLACEDISK: VDS_ASYNC_OUTPUT_TYPE = 9;
pub const VDS_ASYNCOUT_SHRINKLUN: VDS_ASYNC_OUTPUT_TYPE = 55;
pub const VDS_ASYNCOUT_SHRINKVOLUME: VDS_ASYNC_OUTPUT_TYPE = 3;
pub const VDS_ASYNCOUT_UNKNOWN: VDS_ASYNC_OUTPUT_TYPE = 0;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct VDS_ASYNC_OUTPUT {
    pub r#type: VDS_ASYNC_OUTPUT_TYPE,
    pub Anonymous: VDS_ASYNC_OUTPUT_0,
}
impl Default for VDS_ASYNC_OUTPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union VDS_ASYNC_OUTPUT_0 {
    pub cp: VDS_ASYNC_OUTPUT_0_0,
    pub cv: VDS_ASYNC_OUTPUT_0_1,
    pub bvp: VDS_ASYNC_OUTPUT_0_2,
    pub sv: VDS_ASYNC_OUTPUT_0_3,
    pub cl: VDS_ASYNC_OUTPUT_0_4,
    pub ct: VDS_ASYNC_OUTPUT_0_5,
    pub cpg: VDS_ASYNC_OUTPUT_0_6,
    pub cvd: VDS_ASYNC_OUTPUT_0_7,
}
impl Default for VDS_ASYNC_OUTPUT_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct VDS_ASYNC_OUTPUT_0_0 {
    pub ullOffset: u64,
    pub volumeId: VDS_OBJECT_ID,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct VDS_ASYNC_OUTPUT_0_1 {
    pub pVolumeUnk: *mut core::ffi::c_void,
}
impl Default for VDS_ASYNC_OUTPUT_0_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct VDS_ASYNC_OUTPUT_0_2 {
    pub pVolumeUnk: *mut core::ffi::c_void,
}
impl Default for VDS_ASYNC_OUTPUT_0_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct VDS_ASYNC_OUTPUT_0_3 {
    pub ullReclaimedBytes: u64,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct VDS_ASYNC_OUTPUT_0_4 {
    pub pLunUnk: *mut core::ffi::c_void,
}
impl Default for VDS_ASYNC_OUTPUT_0_4 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct VDS_ASYNC_OUTPUT_0_5 {
    pub pTargetUnk: *mut core::ffi::c_void,
}
impl Default for VDS_ASYNC_OUTPUT_0_5 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct VDS_ASYNC_OUTPUT_0_6 {
    pub pPortalGroupUnk: *mut core::ffi::c_void,
}
impl Default for VDS_ASYNC_OUTPUT_0_6 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct VDS_ASYNC_OUTPUT_0_7 {
    pub pVDiskUnk: *mut core::ffi::c_void,
}
impl Default for VDS_ASYNC_OUTPUT_0_7 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type VDS_ASYNC_OUTPUT_TYPE = i32;
pub const VDS_ATTACH_VIRTUAL_DISK_FLAG_USE_FILE_ACL: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct VDS_CONTROLLER_NOTIFICATION {
    pub ulEvent: u32,
    pub controllerId: VDS_OBJECT_ID,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct VDS_CONTROLLER_PROP {
    pub id: VDS_OBJECT_ID,
    pub pwszFriendlyName: windows_sys::core::PWSTR,
    pub pwszIdentification: windows_sys::core::PWSTR,
    pub status: VDS_CONTROLLER_STATUS,
    pub health: VDS_HEALTH,
    pub sNumberOfPorts: i16,
}
impl Default for VDS_CONTROLLER_PROP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type VDS_CONTROLLER_STATUS = i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct VDS_CREATE_VDISK_PARAMETERS {
    pub UniqueId: windows_sys::core::GUID,
    pub MaximumSize: u64,
    pub BlockSizeInBytes: u32,
    pub SectorSizeInBytes: u32,
    pub pParentPath: windows_sys::core::PWSTR,
    pub pSourcePath: windows_sys::core::PWSTR,
}
impl Default for VDS_CREATE_VDISK_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const VDS_CS_FAILED: VDS_CONTROLLER_STATUS = 5;
pub const VDS_CS_NOT_READY: VDS_CONTROLLER_STATUS = 2;
pub const VDS_CS_OFFLINE: VDS_CONTROLLER_STATUS = 4;
pub const VDS_CS_ONLINE: VDS_CONTROLLER_STATUS = 1;
pub const VDS_CS_REMOVED: VDS_CONTROLLER_STATUS = 8;
pub const VDS_CS_UNKNOWN: VDS_CONTROLLER_STATUS = 0;
pub const VDS_DET_CLUSTER: VDS_DISK_EXTENT_TYPE = 7;
pub const VDS_DET_DATA: VDS_DISK_EXTENT_TYPE = 2;
pub const VDS_DET_ESP: VDS_DISK_EXTENT_TYPE = 4;
pub const VDS_DET_FREE: VDS_DISK_EXTENT_TYPE = 1;
pub const VDS_DET_LDM: VDS_DISK_EXTENT_TYPE = 6;
pub const VDS_DET_MSR: VDS_DISK_EXTENT_TYPE = 5;
pub const VDS_DET_OEM: VDS_DISK_EXTENT_TYPE = 3;
pub const VDS_DET_UNKNOWN: VDS_DISK_EXTENT_TYPE = 0;
pub const VDS_DET_UNUSABLE: VDS_DISK_EXTENT_TYPE = 32767;
pub const VDS_DF_AUDIO_CD: VDS_DISK_FLAG = 1;
pub const VDS_DF_BOOT_DISK: VDS_DISK_FLAG = 256;
pub const VDS_DF_BOOT_FROM_DISK: VDS_DISK_FLAG = 16384;
pub const VDS_DF_CLUSTERED: VDS_DISK_FLAG = 32;
pub const VDS_DF_CRASHDUMP_DISK: VDS_DISK_FLAG = 2048;
pub const VDS_DF_CURRENT_READ_ONLY: VDS_DISK_FLAG = 32768;
pub const VDS_DF_DYNAMIC: VDS_DISK_FLAG = 8192;
pub const VDS_DF_HAS_ARC_PATH: VDS_DISK_FLAG = 4096;
pub const VDS_DF_HIBERNATIONFILE_DISK: VDS_DISK_FLAG = 1024;
pub const VDS_DF_HOTSPARE: VDS_DISK_FLAG = 2;
pub const VDS_DF_MASKED: VDS_DISK_FLAG = 8;
pub const VDS_DF_PAGEFILE_DISK: VDS_DISK_FLAG = 512;
pub const VDS_DF_READ_ONLY: VDS_DISK_FLAG = 64;
pub const VDS_DF_REFS_NOT_SUPPORTED: VDS_DISK_FLAG = 65536;
pub const VDS_DF_RESERVE_CAPABLE: VDS_DISK_FLAG = 4;
pub const VDS_DF_STYLE_CONVERTIBLE: VDS_DISK_FLAG = 16;
pub const VDS_DF_SYSTEM_DISK: VDS_DISK_FLAG = 128;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct VDS_DISK_EXTENT {
    pub diskId: VDS_OBJECT_ID,
    pub r#type: VDS_DISK_EXTENT_TYPE,
    pub ullOffset: u64,
    pub ullSize: u64,
    pub volumeId: VDS_OBJECT_ID,
    pub plexId: VDS_OBJECT_ID,
    pub memberIdx: u32,
}
pub type VDS_DISK_EXTENT_TYPE = i32;
pub type VDS_DISK_FLAG = i32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct VDS_DISK_FREE_EXTENT {
    pub diskId: VDS_OBJECT_ID,
    pub ullOffset: u64,
    pub ullSize: u64,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct VDS_DISK_NOTIFICATION {
    pub ulEvent: u32,
    pub diskId: VDS_OBJECT_ID,
}
pub type VDS_DISK_OFFLINE_REASON = i32;
#[repr(C)]
#[cfg(feature = "vdslun")]
#[derive(Clone, Copy)]
pub struct VDS_DISK_PROP {
    pub id: VDS_OBJECT_ID,
    pub status: VDS_DISK_STATUS,
    pub ReserveMode: VDS_LUN_RESERVE_MODE,
    pub health: VDS_HEALTH,
    pub dwDeviceType: u32,
    pub dwMediaType: u32,
    pub ullSize: u64,
    pub ulBytesPerSector: u32,
    pub ulSectorsPerTrack: u32,
    pub ulTracksPerCylinder: u32,
    pub ulFlags: u32,
    pub BusType: super::VDS_STORAGE_BUS_TYPE,
    pub PartitionStyle: VDS_PARTITION_STYLE,
    pub Anonymous: VDS_DISK_PROP_0,
    pub pwszDiskAddress: windows_sys::core::PWSTR,
    pub pwszName: windows_sys::core::PWSTR,
    pub pwszFriendlyName: windows_sys::core::PWSTR,
    pub pwszAdaptorName: windows_sys::core::PWSTR,
    pub pwszDevicePath: windows_sys::core::PWSTR,
}
#[cfg(feature = "vdslun")]
impl Default for VDS_DISK_PROP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "vdslun")]
#[derive(Clone, Copy)]
pub union VDS_DISK_PROP_0 {
    pub dwSignature: u32,
    pub DiskGuid: windows_sys::core::GUID,
}
#[cfg(feature = "vdslun")]
impl Default for VDS_DISK_PROP_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "vdslun")]
#[derive(Clone, Copy)]
pub struct VDS_DISK_PROP2 {
    pub id: VDS_OBJECT_ID,
    pub status: VDS_DISK_STATUS,
    pub OfflineReason: VDS_DISK_OFFLINE_REASON,
    pub ReserveMode: VDS_LUN_RESERVE_MODE,
    pub health: VDS_HEALTH,
    pub dwDeviceType: u32,
    pub dwMediaType: u32,
    pub ullSize: u64,
    pub ulBytesPerSector: u32,
    pub ulSectorsPerTrack: u32,
    pub ulTracksPerCylinder: u32,
    pub ulFlags: u32,
    pub BusType: super::VDS_STORAGE_BUS_TYPE,
    pub PartitionStyle: VDS_PARTITION_STYLE,
    pub Anonymous: VDS_DISK_PROP2_0,
    pub pwszDiskAddress: windows_sys::core::PWSTR,
    pub pwszName: windows_sys::core::PWSTR,
    pub pwszFriendlyName: windows_sys::core::PWSTR,
    pub pwszAdaptorName: windows_sys::core::PWSTR,
    pub pwszDevicePath: windows_sys::core::PWSTR,
    pub pwszLocationPath: windows_sys::core::PWSTR,
}
#[cfg(feature = "vdslun")]
impl Default for VDS_DISK_PROP2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "vdslun")]
#[derive(Clone, Copy)]
pub union VDS_DISK_PROP2_0 {
    pub dwSignature: u32,
    pub DiskGuid: windows_sys::core::GUID,
}
#[cfg(feature = "vdslun")]
impl Default for VDS_DISK_PROP2_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type VDS_DISK_STATUS = i32;
pub const VDS_DLF_NON_PERSISTENT: VDS_DRIVE_LETTER_FLAG = 1;
pub const VDS_DRF_ASSIGNED: VDS_DRIVE_FLAG = 2;
pub const VDS_DRF_HOTSPARE: VDS_DRIVE_FLAG = 1;
pub const VDS_DRF_HOTSPARE_IN_USE: VDS_DRIVE_FLAG = 8;
pub const VDS_DRF_HOTSPARE_STANDBY: VDS_DRIVE_FLAG = 16;
pub const VDS_DRF_UNASSIGNED: VDS_DRIVE_FLAG = 4;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct VDS_DRIVE_EXTENT {
    pub id: VDS_OBJECT_ID,
    pub LunId: VDS_OBJECT_ID,
    pub ullSize: u64,
    pub bUsed: windows_sys::core::BOOL,
}
pub type VDS_DRIVE_FLAG = i32;
pub type VDS_DRIVE_LETTER_FLAG = i32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct VDS_DRIVE_LETTER_NOTIFICATION {
    pub ulEvent: u32,
    pub wcLetter: u16,
    pub volumeId: VDS_OBJECT_ID,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct VDS_DRIVE_LETTER_PROP {
    pub wcLetter: u16,
    pub volumeId: VDS_OBJECT_ID,
    pub ulFlags: u32,
    pub bUsed: windows_sys::core::BOOL,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct VDS_DRIVE_NOTIFICATION {
    pub ulEvent: u32,
    pub driveId: VDS_OBJECT_ID,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct VDS_DRIVE_PROP {
    pub id: VDS_OBJECT_ID,
    pub ullSize: u64,
    pub pwszFriendlyName: windows_sys::core::PWSTR,
    pub pwszIdentification: windows_sys::core::PWSTR,
    pub ulFlags: u32,
    pub status: VDS_DRIVE_STATUS,
    pub health: VDS_HEALTH,
    pub sInternalBusNumber: i16,
    pub sSlotNumber: i16,
}
impl Default for VDS_DRIVE_PROP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "vdslun")]
#[derive(Clone, Copy)]
pub struct VDS_DRIVE_PROP2 {
    pub id: VDS_OBJECT_ID,
    pub ullSize: u64,
    pub pwszFriendlyName: windows_sys::core::PWSTR,
    pub pwszIdentification: windows_sys::core::PWSTR,
    pub ulFlags: u32,
    pub status: VDS_DRIVE_STATUS,
    pub health: VDS_HEALTH,
    pub sInternalBusNumber: i16,
    pub sSlotNumber: i16,
    pub ulEnclosureNumber: u32,
    pub busType: super::VDS_STORAGE_BUS_TYPE,
    pub ulSpindleSpeed: u32,
}
#[cfg(feature = "vdslun")]
impl Default for VDS_DRIVE_PROP2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type VDS_DRIVE_STATUS = i32;
pub const VDS_DRS_FAILED: VDS_DRIVE_STATUS = 5;
pub const VDS_DRS_NOT_READY: VDS_DRIVE_STATUS = 2;
pub const VDS_DRS_OFFLINE: VDS_DRIVE_STATUS = 4;
pub const VDS_DRS_ONLINE: VDS_DRIVE_STATUS = 1;
pub const VDS_DRS_REMOVED: VDS_DRIVE_STATUS = 8;
pub const VDS_DRS_UNKNOWN: VDS_DRIVE_STATUS = 0;
pub const VDS_DS_FAILED: VDS_DISK_STATUS = 5;
pub const VDS_DS_MISSING: VDS_DISK_STATUS = 6;
pub const VDS_DS_NOT_READY: VDS_DISK_STATUS = 2;
pub const VDS_DS_NO_MEDIA: VDS_DISK_STATUS = 3;
pub const VDS_DS_OFFLINE: VDS_DISK_STATUS = 4;
pub const VDS_DS_ONLINE: VDS_DISK_STATUS = 1;
pub const VDS_DS_UNKNOWN: VDS_DISK_STATUS = 0;
pub type VDS_FILE_SYSTEM_FLAG = i32;
pub type VDS_FILE_SYSTEM_FORMAT_SUPPORT_FLAG = i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct VDS_FILE_SYSTEM_FORMAT_SUPPORT_PROP {
    pub ulFlags: u32,
    pub usRevision: u16,
    pub ulDefaultUnitAllocationSize: u32,
    pub rgulAllowedUnitAllocationSizes: [u32; 32],
    pub wszName: [u16; 32],
}
impl Default for VDS_FILE_SYSTEM_FORMAT_SUPPORT_PROP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct VDS_FILE_SYSTEM_NOTIFICATION {
    pub ulEvent: u32,
    pub volumeId: VDS_OBJECT_ID,
    pub dwPercentCompleted: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct VDS_FILE_SYSTEM_PROP {
    pub r#type: VDS_FILE_SYSTEM_TYPE,
    pub volumeId: VDS_OBJECT_ID,
    pub ulFlags: u32,
    pub ullTotalAllocationUnits: u64,
    pub ullAvailableAllocationUnits: u64,
    pub ulAllocationUnitSize: u32,
    pub pwszLabel: windows_sys::core::PWSTR,
}
impl Default for VDS_FILE_SYSTEM_PROP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type VDS_FILE_SYSTEM_PROP_FLAG = i32;
pub type VDS_FILE_SYSTEM_TYPE = i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct VDS_FILE_SYSTEM_TYPE_PROP {
    pub r#type: VDS_FILE_SYSTEM_TYPE,
    pub wszName: [u16; 8],
    pub ulFlags: u32,
    pub ulCompressionFlags: u32,
    pub ulMaxLableLength: u32,
    pub pwszIllegalLabelCharSet: windows_sys::core::PWSTR,
}
impl Default for VDS_FILE_SYSTEM_TYPE_PROP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type VDS_FORMAT_OPTION_FLAGS = i32;
pub const VDS_FPF_COMPRESSED: VDS_FILE_SYSTEM_PROP_FLAG = 1;
pub const VDS_FSF_ALLOCATION_UNIT_128K: VDS_FILE_SYSTEM_FLAG = 16777216;
pub const VDS_FSF_ALLOCATION_UNIT_16K: VDS_FILE_SYSTEM_FLAG = 2097152;
pub const VDS_FSF_ALLOCATION_UNIT_1K: VDS_FILE_SYSTEM_FLAG = 131072;
pub const VDS_FSF_ALLOCATION_UNIT_256K: VDS_FILE_SYSTEM_FLAG = 33554432;
pub const VDS_FSF_ALLOCATION_UNIT_2K: VDS_FILE_SYSTEM_FLAG = 262144;
pub const VDS_FSF_ALLOCATION_UNIT_32K: VDS_FILE_SYSTEM_FLAG = 4194304;
pub const VDS_FSF_ALLOCATION_UNIT_4K: VDS_FILE_SYSTEM_FLAG = 524288;
pub const VDS_FSF_ALLOCATION_UNIT_512: VDS_FILE_SYSTEM_FLAG = 65536;
pub const VDS_FSF_ALLOCATION_UNIT_64K: VDS_FILE_SYSTEM_FLAG = 8388608;
pub const VDS_FSF_ALLOCATION_UNIT_8K: VDS_FILE_SYSTEM_FLAG = 1048576;
pub const VDS_FSF_SUPPORT_COMPRESS: VDS_FILE_SYSTEM_FLAG = 4;
pub const VDS_FSF_SUPPORT_EXTEND: VDS_FILE_SYSTEM_FLAG = 64;
pub const VDS_FSF_SUPPORT_FORMAT: VDS_FILE_SYSTEM_FLAG = 1;
pub const VDS_FSF_SUPPORT_MOUNT_POINT: VDS_FILE_SYSTEM_FLAG = 16;
pub const VDS_FSF_SUPPORT_QUICK_FORMAT: VDS_FILE_SYSTEM_FLAG = 2;
pub const VDS_FSF_SUPPORT_REMOVABLE_MEDIA: VDS_FILE_SYSTEM_FLAG = 32;
pub const VDS_FSF_SUPPORT_SPECIFY_LABEL: VDS_FILE_SYSTEM_FLAG = 8;
pub const VDS_FSOF_COMPRESSION: VDS_FORMAT_OPTION_FLAGS = 4;
pub const VDS_FSOF_DUPLICATE_METADATA: VDS_FORMAT_OPTION_FLAGS = 8;
pub const VDS_FSOF_FORCE: VDS_FORMAT_OPTION_FLAGS = 1;
pub const VDS_FSOF_NONE: VDS_FORMAT_OPTION_FLAGS = 0;
pub const VDS_FSOF_QUICK: VDS_FORMAT_OPTION_FLAGS = 2;
pub const VDS_FSS_DEFAULT: VDS_FILE_SYSTEM_FORMAT_SUPPORT_FLAG = 1;
pub const VDS_FSS_PREVIOUS_REVISION: VDS_FILE_SYSTEM_FORMAT_SUPPORT_FLAG = 2;
pub const VDS_FSS_RECOMMENDED: VDS_FILE_SYSTEM_FORMAT_SUPPORT_FLAG = 4;
pub const VDS_FST_CDFS: VDS_FILE_SYSTEM_TYPE = 5;
pub const VDS_FST_CSVFS: VDS_FILE_SYSTEM_TYPE = 8;
pub const VDS_FST_EXFAT: VDS_FILE_SYSTEM_TYPE = 7;
pub const VDS_FST_FAT: VDS_FILE_SYSTEM_TYPE = 2;
pub const VDS_FST_FAT32: VDS_FILE_SYSTEM_TYPE = 3;
pub const VDS_FST_NTFS: VDS_FILE_SYSTEM_TYPE = 4;
pub const VDS_FST_RAW: VDS_FILE_SYSTEM_TYPE = 1;
pub const VDS_FST_REFS: VDS_FILE_SYSTEM_TYPE = 9;
pub const VDS_FST_UDF: VDS_FILE_SYSTEM_TYPE = 6;
pub const VDS_FST_UNKNOWN: VDS_FILE_SYSTEM_TYPE = 0;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct VDS_HBAPORT_PROP {
    pub id: VDS_OBJECT_ID,
    pub wwnNode: VDS_WWN,
    pub wwnPort: VDS_WWN,
    pub r#type: VDS_HBAPORT_TYPE,
    pub status: VDS_HBAPORT_STATUS,
    pub ulPortSpeed: u32,
    pub ulSupportedPortSpeed: u32,
}
pub type VDS_HBAPORT_SPEED_FLAG = i32;
pub type VDS_HBAPORT_STATUS = i32;
pub type VDS_HBAPORT_TYPE = i32;
pub type VDS_HEALTH = i32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct VDS_HINTS {
    pub ullHintMask: u64,
    pub ullExpectedMaximumSize: u64,
    pub ulOptimalReadSize: u32,
    pub ulOptimalReadAlignment: u32,
    pub ulOptimalWriteSize: u32,
    pub ulOptimalWriteAlignment: u32,
    pub ulMaximumDriveCount: u32,
    pub ulStripeSize: u32,
    pub bFastCrashRecoveryRequired: windows_sys::core::BOOL,
    pub bMostlyReads: windows_sys::core::BOOL,
    pub bOptimizeForSequentialReads: windows_sys::core::BOOL,
    pub bOptimizeForSequentialWrites: windows_sys::core::BOOL,
    pub bRemapEnabled: windows_sys::core::BOOL,
    pub bReadBackVerifyEnabled: windows_sys::core::BOOL,
    pub bWriteThroughCachingEnabled: windows_sys::core::BOOL,
    pub bHardwareChecksumEnabled: windows_sys::core::BOOL,
    pub bIsYankable: windows_sys::core::BOOL,
    pub sRebuildPriority: i16,
}
#[repr(C)]
#[cfg(feature = "vdslun")]
#[derive(Clone, Copy, Default)]
pub struct VDS_HINTS2 {
    pub ullHintMask: u64,
    pub ullExpectedMaximumSize: u64,
    pub ulOptimalReadSize: u32,
    pub ulOptimalReadAlignment: u32,
    pub ulOptimalWriteSize: u32,
    pub ulOptimalWriteAlignment: u32,
    pub ulMaximumDriveCount: u32,
    pub ulStripeSize: u32,
    pub ulReserved1: u32,
    pub ulReserved2: u32,
    pub ulReserved3: u32,
    pub bFastCrashRecoveryRequired: windows_sys::core::BOOL,
    pub bMostlyReads: windows_sys::core::BOOL,
    pub bOptimizeForSequentialReads: windows_sys::core::BOOL,
    pub bOptimizeForSequentialWrites: windows_sys::core::BOOL,
    pub bRemapEnabled: windows_sys::core::BOOL,
    pub bReadBackVerifyEnabled: windows_sys::core::BOOL,
    pub bWriteThroughCachingEnabled: windows_sys::core::BOOL,
    pub bHardwareChecksumEnabled: windows_sys::core::BOOL,
    pub bIsYankable: windows_sys::core::BOOL,
    pub bAllocateHotSpare: windows_sys::core::BOOL,
    pub bUseMirroredCache: windows_sys::core::BOOL,
    pub bReadCachingEnabled: windows_sys::core::BOOL,
    pub bWriteCachingEnabled: windows_sys::core::BOOL,
    pub bMediaScanEnabled: windows_sys::core::BOOL,
    pub bConsistencyCheckEnabled: windows_sys::core::BOOL,
    pub BusType: super::VDS_STORAGE_BUS_TYPE,
    pub bReserved1: windows_sys::core::BOOL,
    pub bReserved2: windows_sys::core::BOOL,
    pub bReserved3: windows_sys::core::BOOL,
    pub sRebuildPriority: i16,
}
pub const VDS_HINT_ALLOCATEHOTSPARE: u32 = 512;
pub const VDS_HINT_BUSTYPE: u32 = 1024;
pub const VDS_HINT_CONSISTENCYCHECKENABLED: u32 = 32768;
pub const VDS_HINT_FASTCRASHRECOVERYREQUIRED: u32 = 1;
pub const VDS_HINT_HARDWARECHECKSUMENABLED: u32 = 128;
pub const VDS_HINT_ISYANKABLE: u32 = 256;
pub const VDS_HINT_MEDIASCANENABLED: u32 = 16384;
pub const VDS_HINT_MOSTLYREADS: u32 = 2;
pub const VDS_HINT_OPTIMIZEFORSEQUENTIALREADS: u32 = 4;
pub const VDS_HINT_OPTIMIZEFORSEQUENTIALWRITES: u32 = 8;
pub const VDS_HINT_READBACKVERIFYENABLED: u32 = 16;
pub const VDS_HINT_READCACHINGENABLED: u32 = 4096;
pub const VDS_HINT_REMAPENABLED: u32 = 32;
pub const VDS_HINT_USEMIRROREDCACHE: u32 = 2048;
pub const VDS_HINT_WRITECACHINGENABLED: u32 = 8192;
pub const VDS_HINT_WRITETHROUGHCACHINGENABLED: u32 = 64;
pub const VDS_HPS_BYPASSED: VDS_HBAPORT_STATUS = 4;
pub const VDS_HPS_DIAGNOSTICS: VDS_HBAPORT_STATUS = 5;
pub const VDS_HPS_ERROR: VDS_HBAPORT_STATUS = 7;
pub const VDS_HPS_LINKDOWN: VDS_HBAPORT_STATUS = 6;
pub const VDS_HPS_LOOPBACK: VDS_HBAPORT_STATUS = 8;
pub const VDS_HPS_OFFLINE: VDS_HBAPORT_STATUS = 3;
pub const VDS_HPS_ONLINE: VDS_HBAPORT_STATUS = 2;
pub const VDS_HPS_UNKNOWN: VDS_HBAPORT_STATUS = 1;
pub const VDS_HPT_EPORT: VDS_HBAPORT_TYPE = 9;
pub const VDS_HPT_FLPORT: VDS_HBAPORT_TYPE = 7;
pub const VDS_HPT_FPORT: VDS_HBAPORT_TYPE = 8;
pub const VDS_HPT_GPORT: VDS_HBAPORT_TYPE = 10;
pub const VDS_HPT_LPORT: VDS_HBAPORT_TYPE = 20;
pub const VDS_HPT_NLPORT: VDS_HBAPORT_TYPE = 6;
pub const VDS_HPT_NOTPRESENT: VDS_HBAPORT_TYPE = 3;
pub const VDS_HPT_NPORT: VDS_HBAPORT_TYPE = 5;
pub const VDS_HPT_OTHER: VDS_HBAPORT_TYPE = 2;
pub const VDS_HPT_PTP: VDS_HBAPORT_TYPE = 21;
pub const VDS_HPT_UNKNOWN: VDS_HBAPORT_TYPE = 1;
pub const VDS_HSF_10GBIT: VDS_HBAPORT_SPEED_FLAG = 4;
pub const VDS_HSF_1GBIT: VDS_HBAPORT_SPEED_FLAG = 1;
pub const VDS_HSF_2GBIT: VDS_HBAPORT_SPEED_FLAG = 2;
pub const VDS_HSF_4GBIT: VDS_HBAPORT_SPEED_FLAG = 8;
pub const VDS_HSF_NOT_NEGOTIATED: VDS_HBAPORT_SPEED_FLAG = 32768;
pub const VDS_HSF_UNKNOWN: VDS_HBAPORT_SPEED_FLAG = 0;
pub type VDS_HWPROVIDER_TYPE = i32;
pub const VDS_HWT_FIBRE_CHANNEL: VDS_HWPROVIDER_TYPE = 2;
pub const VDS_HWT_HYBRID: VDS_HWPROVIDER_TYPE = 5;
pub const VDS_HWT_ISCSI: VDS_HWPROVIDER_TYPE = 3;
pub const VDS_HWT_PCI_RAID: VDS_HWPROVIDER_TYPE = 1;
pub const VDS_HWT_SAS: VDS_HWPROVIDER_TYPE = 4;
pub const VDS_HWT_UNKNOWN: VDS_HWPROVIDER_TYPE = 0;
pub const VDS_H_DEGRADED: VDS_HEALTH = 11;
pub const VDS_H_FAILED: VDS_HEALTH = 8;
pub const VDS_H_FAILED_REDUNDANCY: VDS_HEALTH = 6;
pub const VDS_H_FAILED_REDUNDANCY_FAILING: VDS_HEALTH = 7;
pub const VDS_H_FAILING: VDS_HEALTH = 4;
pub const VDS_H_FAILING_REDUNDANCY: VDS_HEALTH = 5;
pub const VDS_H_HEALTHY: VDS_HEALTH = 1;
pub const VDS_H_PENDING_FAILURE: VDS_HEALTH = 10;
pub const VDS_H_REBUILDING: VDS_HEALTH = 2;
pub const VDS_H_REPLACED: VDS_HEALTH = 9;
pub const VDS_H_STALE: VDS_HEALTH = 3;
pub const VDS_H_UNKNOWN: VDS_HEALTH = 0;
pub const VDS_IAT_CHAP: VDS_ISCSI_AUTH_TYPE = 1;
pub const VDS_IAT_MUTUAL_CHAP: VDS_ISCSI_AUTH_TYPE = 2;
pub const VDS_IAT_NONE: VDS_ISCSI_AUTH_TYPE = 0;
pub const VDS_IIF_AGGRESSIVE_MODE: VDS_ISCSI_IPSEC_FLAG = 8;
pub const VDS_IIF_IKE: VDS_ISCSI_IPSEC_FLAG = 2;
pub const VDS_IIF_MAIN_MODE: VDS_ISCSI_IPSEC_FLAG = 4;
pub const VDS_IIF_PFS_ENABLE: VDS_ISCSI_IPSEC_FLAG = 16;
pub const VDS_IIF_TRANSPORT_MODE_PREFERRED: VDS_ISCSI_IPSEC_FLAG = 32;
pub const VDS_IIF_TUNNEL_MODE_PREFERRED: VDS_ISCSI_IPSEC_FLAG = 64;
pub const VDS_IIF_VALID: VDS_ISCSI_IPSEC_FLAG = 1;
pub const VDS_ILF_MULTIPATH_ENABLED: VDS_ISCSI_LOGIN_FLAG = 2;
pub const VDS_ILF_REQUIRE_IPSEC: VDS_ISCSI_LOGIN_FLAG = 1;
pub const VDS_ILT_BOOT: VDS_ISCSI_LOGIN_TYPE = 2;
pub const VDS_ILT_MANUAL: VDS_ISCSI_LOGIN_TYPE = 0;
pub const VDS_ILT_PERSISTENT: VDS_ISCSI_LOGIN_TYPE = 1;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct VDS_INPUT_DISK {
    pub diskId: VDS_OBJECT_ID,
    pub ullSize: u64,
    pub plexId: VDS_OBJECT_ID,
    pub memberIdx: u32,
}
pub type VDS_INTERCONNECT_FLAG = i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct VDS_IPADDRESS {
    pub r#type: VDS_IPADDRESS_TYPE,
    pub ipv4Address: u32,
    pub ipv6Address: [u8; 16],
    pub ulIpv6FlowInfo: u32,
    pub ulIpv6ScopeId: u32,
    pub wszTextAddress: [u16; 257],
    pub ulPort: u32,
}
impl Default for VDS_IPADDRESS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type VDS_IPADDRESS_TYPE = i32;
pub const VDS_IPS_FAILED: VDS_ISCSI_PORTAL_STATUS = 5;
pub const VDS_IPS_NOT_READY: VDS_ISCSI_PORTAL_STATUS = 2;
pub const VDS_IPS_OFFLINE: VDS_ISCSI_PORTAL_STATUS = 4;
pub const VDS_IPS_ONLINE: VDS_ISCSI_PORTAL_STATUS = 1;
pub const VDS_IPS_UNKNOWN: VDS_ISCSI_PORTAL_STATUS = 0;
pub const VDS_IPT_EMPTY: VDS_IPADDRESS_TYPE = 3;
pub const VDS_IPT_IPV4: VDS_IPADDRESS_TYPE = 1;
pub const VDS_IPT_IPV6: VDS_IPADDRESS_TYPE = 2;
pub const VDS_IPT_TEXT: VDS_IPADDRESS_TYPE = 0;
pub type VDS_ISCSI_AUTH_TYPE = i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct VDS_ISCSI_INITIATOR_ADAPTER_PROP {
    pub id: VDS_OBJECT_ID,
    pub pwszName: windows_sys::core::PWSTR,
}
impl Default for VDS_ISCSI_INITIATOR_ADAPTER_PROP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct VDS_ISCSI_INITIATOR_PORTAL_PROP {
    pub id: VDS_OBJECT_ID,
    pub address: VDS_IPADDRESS,
    pub ulPortIndex: u32,
}
pub type VDS_ISCSI_IPSEC_FLAG = i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct VDS_ISCSI_IPSEC_KEY {
    pub pKey: *mut u8,
    pub ulKeySize: u32,
}
impl Default for VDS_ISCSI_IPSEC_KEY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type VDS_ISCSI_LOGIN_FLAG = i32;
pub type VDS_ISCSI_LOGIN_TYPE = i32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct VDS_ISCSI_PORTALGROUP_PROP {
    pub id: VDS_OBJECT_ID,
    pub tag: VDS_ISCSI_PORTALGROUP_TAG,
}
pub type VDS_ISCSI_PORTALGROUP_TAG = u16;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct VDS_ISCSI_PORTAL_PROP {
    pub id: VDS_OBJECT_ID,
    pub address: VDS_IPADDRESS,
    pub status: VDS_ISCSI_PORTAL_STATUS,
}
pub type VDS_ISCSI_PORTAL_STATUS = i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct VDS_ISCSI_SHARED_SECRET {
    pub pSharedSecret: *mut u8,
    pub ulSharedSecretSize: u32,
}
impl Default for VDS_ISCSI_SHARED_SECRET {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct VDS_ISCSI_TARGET_PROP {
    pub id: VDS_OBJECT_ID,
    pub pwszIscsiName: windows_sys::core::PWSTR,
    pub pwszFriendlyName: windows_sys::core::PWSTR,
    pub bChapEnabled: windows_sys::core::BOOL,
}
impl Default for VDS_ISCSI_TARGET_PROP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const VDS_ITF_FIBRE_CHANNEL: VDS_INTERCONNECT_FLAG = 2;
pub const VDS_ITF_ISCSI: VDS_INTERCONNECT_FLAG = 4;
pub const VDS_ITF_PCI_RAID: VDS_INTERCONNECT_FLAG = 1;
pub const VDS_ITF_SAS: VDS_INTERCONNECT_FLAG = 8;
pub const VDS_LBF_DYN_LEAST_QUEUE_DEPTH: VDS_PROVIDER_LBSUPPORT_FLAG = 8;
pub const VDS_LBF_FAILOVER: VDS_PROVIDER_LBSUPPORT_FLAG = 1;
pub const VDS_LBF_LEAST_BLOCKS: VDS_PROVIDER_LBSUPPORT_FLAG = 32;
pub const VDS_LBF_ROUND_ROBIN: VDS_PROVIDER_LBSUPPORT_FLAG = 2;
pub const VDS_LBF_ROUND_ROBIN_WITH_SUBSET: VDS_PROVIDER_LBSUPPORT_FLAG = 4;
pub const VDS_LBF_VENDOR_SPECIFIC: VDS_PROVIDER_LBSUPPORT_FLAG = 64;
pub const VDS_LBF_WEIGHTED_PATHS: VDS_PROVIDER_LBSUPPORT_FLAG = 16;
pub const VDS_LBP_DYN_LEAST_QUEUE_DEPTH: VDS_LOADBALANCE_POLICY_ENUM = 4;
pub const VDS_LBP_FAILOVER: VDS_LOADBALANCE_POLICY_ENUM = 1;
pub const VDS_LBP_LEAST_BLOCKS: VDS_LOADBALANCE_POLICY_ENUM = 6;
pub const VDS_LBP_ROUND_ROBIN: VDS_LOADBALANCE_POLICY_ENUM = 2;
pub const VDS_LBP_ROUND_ROBIN_WITH_SUBSET: VDS_LOADBALANCE_POLICY_ENUM = 3;
pub const VDS_LBP_UNKNOWN: VDS_LOADBALANCE_POLICY_ENUM = 0;
pub const VDS_LBP_VENDOR_SPECIFIC: VDS_LOADBALANCE_POLICY_ENUM = 7;
pub const VDS_LBP_WEIGHTED_PATHS: VDS_LOADBALANCE_POLICY_ENUM = 5;
pub const VDS_LF_CONSISTENCY_CHECK_ENABLED: VDS_LUN_FLAG = 128;
pub const VDS_LF_HARDWARE_CHECKSUM_ENABLED: VDS_LUN_FLAG = 8;
pub const VDS_LF_LBN_REMAP_ENABLED: VDS_LUN_FLAG = 1;
pub const VDS_LF_MEDIA_SCAN_ENABLED: VDS_LUN_FLAG = 64;
pub const VDS_LF_READ_BACK_VERIFY_ENABLED: VDS_LUN_FLAG = 2;
pub const VDS_LF_READ_CACHE_ENABLED: VDS_LUN_FLAG = 16;
pub const VDS_LF_SNAPSHOT: VDS_LUN_FLAG = 256;
pub const VDS_LF_WRITE_CACHE_ENABLED: VDS_LUN_FLAG = 32;
pub const VDS_LF_WRITE_THROUGH_CACHING_ENABLED: VDS_LUN_FLAG = 4;
pub type VDS_LOADBALANCE_POLICY_ENUM = i32;
pub const VDS_LPF_LBN_REMAP_ENABLED: VDS_LUN_PLEX_FLAG = 1;
pub const VDS_LPS_FAILED: VDS_LUN_PLEX_STATUS = 5;
pub const VDS_LPS_NOT_READY: VDS_LUN_PLEX_STATUS = 2;
pub const VDS_LPS_OFFLINE: VDS_LUN_PLEX_STATUS = 4;
pub const VDS_LPS_ONLINE: VDS_LUN_PLEX_STATUS = 1;
pub const VDS_LPS_UNKNOWN: VDS_LUN_PLEX_STATUS = 0;
pub const VDS_LPT_PARITY: VDS_LUN_PLEX_TYPE = 14;
pub const VDS_LPT_RAID03: VDS_LUN_PLEX_TYPE = 21;
pub const VDS_LPT_RAID05: VDS_LUN_PLEX_TYPE = 22;
pub const VDS_LPT_RAID10: VDS_LUN_PLEX_TYPE = 23;
pub const VDS_LPT_RAID15: VDS_LUN_PLEX_TYPE = 24;
pub const VDS_LPT_RAID2: VDS_LUN_PLEX_TYPE = 15;
pub const VDS_LPT_RAID3: VDS_LUN_PLEX_TYPE = 16;
pub const VDS_LPT_RAID30: VDS_LUN_PLEX_TYPE = 25;
pub const VDS_LPT_RAID4: VDS_LUN_PLEX_TYPE = 17;
pub const VDS_LPT_RAID5: VDS_LUN_PLEX_TYPE = 18;
pub const VDS_LPT_RAID50: VDS_LUN_PLEX_TYPE = 26;
pub const VDS_LPT_RAID53: VDS_LUN_PLEX_TYPE = 28;
pub const VDS_LPT_RAID6: VDS_LUN_PLEX_TYPE = 19;
pub const VDS_LPT_RAID60: VDS_LUN_PLEX_TYPE = 29;
pub const VDS_LPT_SIMPLE: VDS_LUN_PLEX_TYPE = 10;
pub const VDS_LPT_SPAN: VDS_LUN_PLEX_TYPE = 11;
pub const VDS_LPT_STRIPE: VDS_LUN_PLEX_TYPE = 12;
pub const VDS_LPT_UNKNOWN: VDS_LUN_PLEX_TYPE = 0;
pub const VDS_LRM_EXCLUSIVE_RO: VDS_LUN_RESERVE_MODE = 2;
pub const VDS_LRM_EXCLUSIVE_RW: VDS_LUN_RESERVE_MODE = 1;
pub const VDS_LRM_NONE: VDS_LUN_RESERVE_MODE = 0;
pub const VDS_LRM_SHARED_RO: VDS_LUN_RESERVE_MODE = 3;
pub const VDS_LRM_SHARED_RW: VDS_LUN_RESERVE_MODE = 4;
pub const VDS_LS_FAILED: VDS_LUN_STATUS = 5;
pub const VDS_LS_NOT_READY: VDS_LUN_STATUS = 2;
pub const VDS_LS_OFFLINE: VDS_LUN_STATUS = 4;
pub const VDS_LS_ONLINE: VDS_LUN_STATUS = 1;
pub const VDS_LS_UNKNOWN: VDS_LUN_STATUS = 0;
pub const VDS_LT_DEFAULT: VDS_LUN_TYPE = 1;
pub const VDS_LT_FAULT_TOLERANT: VDS_LUN_TYPE = 2;
pub const VDS_LT_MIRROR: VDS_LUN_TYPE = 13;
pub const VDS_LT_NON_FAULT_TOLERANT: VDS_LUN_TYPE = 3;
pub const VDS_LT_PARITY: VDS_LUN_TYPE = 14;
pub const VDS_LT_RAID01: VDS_LUN_TYPE = 20;
pub const VDS_LT_RAID03: VDS_LUN_TYPE = 21;
pub const VDS_LT_RAID05: VDS_LUN_TYPE = 22;
pub const VDS_LT_RAID10: VDS_LUN_TYPE = 23;
pub const VDS_LT_RAID15: VDS_LUN_TYPE = 24;
pub const VDS_LT_RAID2: VDS_LUN_TYPE = 15;
pub const VDS_LT_RAID3: VDS_LUN_TYPE = 16;
pub const VDS_LT_RAID30: VDS_LUN_TYPE = 25;
pub const VDS_LT_RAID4: VDS_LUN_TYPE = 17;
pub const VDS_LT_RAID5: VDS_LUN_TYPE = 18;
pub const VDS_LT_RAID50: VDS_LUN_TYPE = 26;
pub const VDS_LT_RAID51: VDS_LUN_TYPE = 27;
pub const VDS_LT_RAID53: VDS_LUN_TYPE = 28;
pub const VDS_LT_RAID6: VDS_LUN_TYPE = 19;
pub const VDS_LT_RAID60: VDS_LUN_TYPE = 29;
pub const VDS_LT_RAID61: VDS_LUN_TYPE = 30;
pub const VDS_LT_SIMPLE: VDS_LUN_TYPE = 10;
pub const VDS_LT_SPAN: VDS_LUN_TYPE = 11;
pub const VDS_LT_STRIPE: VDS_LUN_TYPE = 12;
pub const VDS_LT_UNKNOWN: VDS_LUN_TYPE = 0;
pub type VDS_LUN_FLAG = i32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct VDS_LUN_NOTIFICATION {
    pub ulEvent: u32,
    pub LunId: VDS_OBJECT_ID,
}
pub type VDS_LUN_PLEX_FLAG = i32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct VDS_LUN_PLEX_PROP {
    pub id: VDS_OBJECT_ID,
    pub ullSize: u64,
    pub r#type: VDS_LUN_PLEX_TYPE,
    pub status: VDS_LUN_PLEX_STATUS,
    pub health: VDS_HEALTH,
    pub TransitionState: VDS_TRANSITION_STATE,
    pub ulFlags: u32,
    pub ulStripeSize: u32,
    pub sRebuildPriority: i16,
}
pub type VDS_LUN_PLEX_STATUS = i32;
pub type VDS_LUN_PLEX_TYPE = i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct VDS_LUN_PROP {
    pub id: VDS_OBJECT_ID,
    pub ullSize: u64,
    pub pwszFriendlyName: windows_sys::core::PWSTR,
    pub pwszIdentification: windows_sys::core::PWSTR,
    pub pwszUnmaskingList: windows_sys::core::PWSTR,
    pub ulFlags: u32,
    pub r#type: VDS_LUN_TYPE,
    pub status: VDS_LUN_STATUS,
    pub health: VDS_HEALTH,
    pub TransitionState: VDS_TRANSITION_STATE,
    pub sRebuildPriority: i16,
}
impl Default for VDS_LUN_PROP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type VDS_LUN_RESERVE_MODE = i32;
pub type VDS_LUN_STATUS = i32;
pub type VDS_LUN_TYPE = i32;
pub type VDS_MAINTENANCE_OPERATION = i32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct VDS_MOUNT_POINT_NOTIFICATION {
    pub ulEvent: u32,
    pub volumeId: VDS_OBJECT_ID,
}
pub const VDS_MPS_FAILED: VDS_PATH_STATUS = 5;
pub const VDS_MPS_ONLINE: VDS_PATH_STATUS = 1;
pub const VDS_MPS_STANDBY: VDS_PATH_STATUS = 7;
pub const VDS_MPS_UNKNOWN: VDS_PATH_STATUS = 0;
pub const VDS_NF_CONTROLLER_ARRIVE: u32 = 103;
pub const VDS_NF_CONTROLLER_DEPART: u32 = 104;
pub const VDS_NF_CONTROLLER_MODIFY: u32 = 350;
pub const VDS_NF_CONTROLLER_REMOVED: u32 = 351;
pub const VDS_NF_DISK_ARRIVE: u32 = 8;
pub const VDS_NF_DISK_DEPART: u32 = 9;
pub const VDS_NF_DISK_MODIFY: u32 = 10;
pub const VDS_NF_DRIVE_ARRIVE: u32 = 105;
pub const VDS_NF_DRIVE_DEPART: u32 = 106;
pub const VDS_NF_DRIVE_LETTER_ASSIGN: u32 = 202;
pub const VDS_NF_DRIVE_LETTER_FREE: u32 = 201;
pub const VDS_NF_DRIVE_MODIFY: u32 = 107;
pub const VDS_NF_DRIVE_REMOVED: u32 = 354;
pub const VDS_NF_FILE_SYSTEM_FORMAT_PROGRESS: u32 = 204;
pub const VDS_NF_FILE_SYSTEM_MODIFY: u32 = 203;
pub const VDS_NF_FILE_SYSTEM_SHRINKING_PROGRESS: u32 = 206;
pub const VDS_NF_LUN_ARRIVE: u32 = 108;
pub const VDS_NF_LUN_DEPART: u32 = 109;
pub const VDS_NF_LUN_MODIFY: u32 = 110;
pub const VDS_NF_MOUNT_POINTS_CHANGE: u32 = 205;
pub const VDS_NF_PACK_ARRIVE: u32 = 1;
pub const VDS_NF_PACK_DEPART: u32 = 2;
pub const VDS_NF_PACK_MODIFY: u32 = 3;
pub const VDS_NF_PARTITION_ARRIVE: u32 = 11;
pub const VDS_NF_PARTITION_DEPART: u32 = 12;
pub const VDS_NF_PARTITION_MODIFY: u32 = 13;
pub const VDS_NF_PORTAL_ARRIVE: u32 = 123;
pub const VDS_NF_PORTAL_DEPART: u32 = 124;
pub const VDS_NF_PORTAL_GROUP_ARRIVE: u32 = 129;
pub const VDS_NF_PORTAL_GROUP_DEPART: u32 = 130;
pub const VDS_NF_PORTAL_GROUP_MODIFY: u32 = 131;
pub const VDS_NF_PORTAL_MODIFY: u32 = 125;
pub const VDS_NF_PORT_ARRIVE: u32 = 121;
pub const VDS_NF_PORT_DEPART: u32 = 122;
pub const VDS_NF_PORT_MODIFY: u32 = 352;
pub const VDS_NF_PORT_REMOVED: u32 = 353;
pub const VDS_NF_SERVICE_OUT_OF_SYNC: u32 = 301;
pub const VDS_NF_SUB_SYSTEM_ARRIVE: u32 = 101;
pub const VDS_NF_SUB_SYSTEM_DEPART: u32 = 102;
pub const VDS_NF_SUB_SYSTEM_MODIFY: u32 = 151;
pub const VDS_NF_TARGET_ARRIVE: u32 = 126;
pub const VDS_NF_TARGET_DEPART: u32 = 127;
pub const VDS_NF_TARGET_MODIFY: u32 = 128;
pub const VDS_NF_VOLUME_ARRIVE: u32 = 4;
pub const VDS_NF_VOLUME_DEPART: u32 = 5;
pub const VDS_NF_VOLUME_MODIFY: u32 = 6;
pub const VDS_NF_VOLUME_REBUILDING_PROGRESS: u32 = 7;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct VDS_NOTIFICATION {
    pub objectType: VDS_NOTIFICATION_TARGET_TYPE,
    pub Anonymous: VDS_NOTIFICATION_0,
}
impl Default for VDS_NOTIFICATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union VDS_NOTIFICATION_0 {
    pub Pack: VDS_PACK_NOTIFICATION,
    pub Disk: VDS_DISK_NOTIFICATION,
    pub Volume: VDS_VOLUME_NOTIFICATION,
    pub Partition: VDS_PARTITION_NOTIFICATION,
    pub Letter: VDS_DRIVE_LETTER_NOTIFICATION,
    pub FileSystem: VDS_FILE_SYSTEM_NOTIFICATION,
    pub MountPoint: VDS_MOUNT_POINT_NOTIFICATION,
    pub SubSystem: VDS_SUB_SYSTEM_NOTIFICATION,
    pub Controller: VDS_CONTROLLER_NOTIFICATION,
    pub Drive: VDS_DRIVE_NOTIFICATION,
    pub Lun: VDS_LUN_NOTIFICATION,
    pub Port: VDS_PORT_NOTIFICATION,
    pub Portal: VDS_PORTAL_NOTIFICATION,
    pub Target: VDS_TARGET_NOTIFICATION,
    pub PortalGroup: VDS_PORTAL_GROUP_NOTIFICATION,
    pub Service: VDS_SERVICE_NOTIFICATION,
}
impl Default for VDS_NOTIFICATION_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type VDS_NOTIFICATION_TARGET_TYPE = i32;
pub const VDS_NTT_CONTROLLER: VDS_NOTIFICATION_TARGET_TYPE = 31;
pub const VDS_NTT_DISK: VDS_NOTIFICATION_TARGET_TYPE = 13;
pub const VDS_NTT_DRIVE: VDS_NOTIFICATION_TARGET_TYPE = 32;
pub const VDS_NTT_DRIVE_LETTER: VDS_NOTIFICATION_TARGET_TYPE = 61;
pub const VDS_NTT_FILE_SYSTEM: VDS_NOTIFICATION_TARGET_TYPE = 62;
pub const VDS_NTT_LUN: VDS_NOTIFICATION_TARGET_TYPE = 33;
pub const VDS_NTT_MOUNT_POINT: VDS_NOTIFICATION_TARGET_TYPE = 63;
pub const VDS_NTT_PACK: VDS_NOTIFICATION_TARGET_TYPE = 10;
pub const VDS_NTT_PARTITION: VDS_NOTIFICATION_TARGET_TYPE = 60;
pub const VDS_NTT_PORT: VDS_NOTIFICATION_TARGET_TYPE = 35;
pub const VDS_NTT_PORTAL: VDS_NOTIFICATION_TARGET_TYPE = 36;
pub const VDS_NTT_PORTAL_GROUP: VDS_NOTIFICATION_TARGET_TYPE = 38;
pub const VDS_NTT_SERVICE: VDS_NOTIFICATION_TARGET_TYPE = 200;
pub const VDS_NTT_SUB_SYSTEM: VDS_NOTIFICATION_TARGET_TYPE = 30;
pub const VDS_NTT_TARGET: VDS_NOTIFICATION_TARGET_TYPE = 37;
pub const VDS_NTT_UNKNOWN: VDS_NOTIFICATION_TARGET_TYPE = 0;
pub const VDS_NTT_VOLUME: VDS_NOTIFICATION_TARGET_TYPE = 11;
pub type VDS_OBJECT_ID = windows_sys::core::GUID;
pub type VDS_OBJECT_TYPE = i32;
pub const VDS_OT_ASYNC: VDS_OBJECT_TYPE = 100;
pub const VDS_OT_CONTROLLER: VDS_OBJECT_TYPE = 31;
pub const VDS_OT_DISK: VDS_OBJECT_TYPE = 13;
pub const VDS_OT_DRIVE: VDS_OBJECT_TYPE = 32;
pub const VDS_OT_ENUM: VDS_OBJECT_TYPE = 101;
pub const VDS_OT_HBAPORT: VDS_OBJECT_TYPE = 90;
pub const VDS_OT_INIT_ADAPTER: VDS_OBJECT_TYPE = 91;
pub const VDS_OT_INIT_PORTAL: VDS_OBJECT_TYPE = 92;
pub const VDS_OT_LUN: VDS_OBJECT_TYPE = 33;
pub const VDS_OT_LUN_PLEX: VDS_OBJECT_TYPE = 34;
pub const VDS_OT_OPEN_VDISK: VDS_OBJECT_TYPE = 201;
pub const VDS_OT_PACK: VDS_OBJECT_TYPE = 10;
pub const VDS_OT_PORT: VDS_OBJECT_TYPE = 35;
pub const VDS_OT_PORTAL: VDS_OBJECT_TYPE = 36;
pub const VDS_OT_PORTAL_GROUP: VDS_OBJECT_TYPE = 38;
pub const VDS_OT_PROVIDER: VDS_OBJECT_TYPE = 1;
pub const VDS_OT_STORAGE_POOL: VDS_OBJECT_TYPE = 39;
pub const VDS_OT_SUB_SYSTEM: VDS_OBJECT_TYPE = 30;
pub const VDS_OT_TARGET: VDS_OBJECT_TYPE = 37;
pub const VDS_OT_UNKNOWN: VDS_OBJECT_TYPE = 0;
pub const VDS_OT_VDISK: VDS_OBJECT_TYPE = 200;
pub const VDS_OT_VOLUME: VDS_OBJECT_TYPE = 11;
pub const VDS_OT_VOLUME_PLEX: VDS_OBJECT_TYPE = 12;
pub type VDS_PACK_FLAG = i32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct VDS_PACK_NOTIFICATION {
    pub ulEvent: u32,
    pub packId: VDS_OBJECT_ID,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct VDS_PACK_PROP {
    pub id: VDS_OBJECT_ID,
    pub pwszName: windows_sys::core::PWSTR,
    pub status: VDS_PACK_STATUS,
    pub ulFlags: u32,
}
impl Default for VDS_PACK_PROP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type VDS_PACK_STATUS = i32;
pub type VDS_PARTITION_FLAG = i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct VDS_PARTITION_INFORMATION_EX {
    pub dwPartitionStyle: __VDS_PARTITION_STYLE,
    pub ullStartingOffset: u64,
    pub ullPartitionLength: u64,
    pub dwPartitionNumber: u32,
    pub bRewritePartition: bool,
    pub Anonymous: VDS_PARTITION_INFORMATION_EX_0,
}
impl Default for VDS_PARTITION_INFORMATION_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union VDS_PARTITION_INFORMATION_EX_0 {
    pub Mbr: VDS_PARTITION_INFO_MBR,
    pub Gpt: VDS_PARTITION_INFO_GPT,
}
impl Default for VDS_PARTITION_INFORMATION_EX_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct VDS_PARTITION_INFO_GPT {
    pub partitionType: windows_sys::core::GUID,
    pub partitionId: windows_sys::core::GUID,
    pub attributes: u64,
    pub name: [u16; 36],
}
impl Default for VDS_PARTITION_INFO_GPT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct VDS_PARTITION_INFO_MBR {
    pub partitionType: u8,
    pub bootIndicator: bool,
    pub recognizedPartition: bool,
    pub hiddenSectors: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct VDS_PARTITION_NOTIFICATION {
    pub ulEvent: u32,
    pub diskId: VDS_OBJECT_ID,
    pub ullOffset: u64,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct VDS_PARTITION_PROP {
    pub PartitionStyle: VDS_PARTITION_STYLE,
    pub ulFlags: u32,
    pub ulPartitionNumber: u32,
    pub ullOffset: u64,
    pub ullSize: u64,
    pub Anonymous: VDS_PARTITION_PROP_0,
}
impl Default for VDS_PARTITION_PROP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union VDS_PARTITION_PROP_0 {
    pub Mbr: VDS_PARTITION_INFO_MBR,
    pub Gpt: VDS_PARTITION_INFO_GPT,
}
impl Default for VDS_PARTITION_PROP_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type VDS_PARTITION_STYLE = i32;
pub const VDS_PARTITION_STYLE_GPT: __VDS_PARTITION_STYLE = 1;
pub const VDS_PARTITION_STYLE_MBR: __VDS_PARTITION_STYLE = 0;
pub const VDS_PARTITION_STYLE_RAW: __VDS_PARTITION_STYLE = 2;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct VDS_PATH_ID {
    pub ullSourceId: u64,
    pub ullPathId: u64,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct VDS_PATH_INFO {
    pub pathId: VDS_PATH_ID,
    pub r#type: VDS_HWPROVIDER_TYPE,
    pub status: VDS_PATH_STATUS,
    pub Anonymous: VDS_PATH_INFO_0,
    pub Anonymous2: VDS_PATH_INFO_1,
    pub Anonymous3: VDS_PATH_INFO_2,
}
impl Default for VDS_PATH_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union VDS_PATH_INFO_0 {
    pub controllerPortId: VDS_OBJECT_ID,
    pub targetPortalId: VDS_OBJECT_ID,
}
impl Default for VDS_PATH_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union VDS_PATH_INFO_1 {
    pub hbaPortId: VDS_OBJECT_ID,
    pub initiatorAdapterId: VDS_OBJECT_ID,
}
impl Default for VDS_PATH_INFO_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union VDS_PATH_INFO_2 {
    pub pHbaPortProp: *mut VDS_HBAPORT_PROP,
    pub pInitiatorPortalIpAddr: *mut VDS_IPADDRESS,
}
impl Default for VDS_PATH_INFO_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct VDS_PATH_POLICY {
    pub pathId: VDS_PATH_ID,
    pub bPrimaryPath: windows_sys::core::BOOL,
    pub ulWeight: u32,
}
pub type VDS_PATH_STATUS = i32;
pub const VDS_PF_DYNAMIC: VDS_PROVIDER_FLAG = 1;
pub const VDS_PF_INTERNAL_HARDWARE_PROVIDER: VDS_PROVIDER_FLAG = 2;
pub const VDS_PF_ONE_DISK_ONLY_PER_PACK: VDS_PROVIDER_FLAG = 4;
pub const VDS_PF_ONE_PACK_ONLINE_ONLY: VDS_PROVIDER_FLAG = 8;
pub const VDS_PF_SUPPORT_DYNAMIC: VDS_PROVIDER_FLAG = -2147483648;
pub const VDS_PF_SUPPORT_DYNAMIC_1394: VDS_PROVIDER_FLAG = 536870912;
pub const VDS_PF_SUPPORT_FAULT_TOLERANT: VDS_PROVIDER_FLAG = 1073741824;
pub const VDS_PF_SUPPORT_MIRROR: VDS_PROVIDER_FLAG = 32;
pub const VDS_PF_SUPPORT_RAID5: VDS_PROVIDER_FLAG = 64;
pub const VDS_PF_VOLUME_SPACE_MUST_BE_CONTIGUOUS: VDS_PROVIDER_FLAG = 16;
pub const VDS_PKF_CORRUPTED: VDS_PACK_FLAG = 8;
pub const VDS_PKF_FOREIGN: VDS_PACK_FLAG = 1;
pub const VDS_PKF_NOQUORUM: VDS_PACK_FLAG = 2;
pub const VDS_PKF_ONLINE_ERROR: VDS_PACK_FLAG = 16;
pub const VDS_PKF_POLICY: VDS_PACK_FLAG = 4;
#[repr(C)]
#[cfg(feature = "vdslun")]
#[derive(Clone, Copy)]
pub struct VDS_POOL_ATTRIBUTES {
    pub ullAttributeMask: u64,
    pub raidType: VDS_RAID_TYPE,
    pub busType: super::VDS_STORAGE_BUS_TYPE,
    pub pwszIntendedUsage: windows_sys::core::PWSTR,
    pub bSpinDown: windows_sys::core::BOOL,
    pub bIsThinProvisioned: windows_sys::core::BOOL,
    pub ullProvisionedSpace: u64,
    pub bNoSinglePointOfFailure: windows_sys::core::BOOL,
    pub ulDataRedundancyMax: u32,
    pub ulDataRedundancyMin: u32,
    pub ulDataRedundancyDefault: u32,
    pub ulPackageRedundancyMax: u32,
    pub ulPackageRedundancyMin: u32,
    pub ulPackageRedundancyDefault: u32,
    pub ulStripeSize: u32,
    pub ulStripeSizeMax: u32,
    pub ulStripeSizeMin: u32,
    pub ulDefaultStripeSize: u32,
    pub ulNumberOfColumns: u32,
    pub ulNumberOfColumnsMax: u32,
    pub ulNumberOfColumnsMin: u32,
    pub ulDefaultNumberofColumns: u32,
    pub ulDataAvailabilityHint: u32,
    pub ulAccessRandomnessHint: u32,
    pub ulAccessDirectionHint: u32,
    pub ulAccessSizeHint: u32,
    pub ulAccessLatencyHint: u32,
    pub ulAccessBandwidthWeightHint: u32,
    pub ulStorageCostHint: u32,
    pub ulStorageEfficiencyHint: u32,
    pub ulNumOfCustomAttributes: u32,
    pub pPoolCustomAttributes: *mut VDS_POOL_CUSTOM_ATTRIBUTES,
    pub bReserved1: windows_sys::core::BOOL,
    pub bReserved2: windows_sys::core::BOOL,
    pub ulReserved1: u32,
    pub ulReserved2: u32,
    pub ullReserved1: u64,
    pub ullReserved2: u64,
}
#[cfg(feature = "vdslun")]
impl Default for VDS_POOL_ATTRIBUTES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const VDS_POOL_ATTRIB_ACCS_BDW_WT_HINT: u32 = 16777216;
pub const VDS_POOL_ATTRIB_ACCS_DIR_HINT: u32 = 2097152;
pub const VDS_POOL_ATTRIB_ACCS_LTNCY_HINT: u32 = 8388608;
pub const VDS_POOL_ATTRIB_ACCS_RNDM_HINT: u32 = 1048576;
pub const VDS_POOL_ATTRIB_ACCS_SIZE_HINT: u32 = 4194304;
pub const VDS_POOL_ATTRIB_ALLOW_SPINDOWN: u32 = 4;
pub const VDS_POOL_ATTRIB_BUSTYPE: u32 = 2;
pub const VDS_POOL_ATTRIB_CUSTOM_ATTRIB: u32 = 134217728;
pub const VDS_POOL_ATTRIB_DATA_AVL_HINT: u32 = 524288;
pub const VDS_POOL_ATTRIB_DATA_RDNCY_DEF: u32 = 128;
pub const VDS_POOL_ATTRIB_DATA_RDNCY_MAX: u32 = 32;
pub const VDS_POOL_ATTRIB_DATA_RDNCY_MIN: u32 = 64;
pub const VDS_POOL_ATTRIB_NO_SINGLE_POF: u32 = 16;
pub const VDS_POOL_ATTRIB_NUM_CLMNS: u32 = 32768;
pub const VDS_POOL_ATTRIB_NUM_CLMNS_DEF: u32 = 262144;
pub const VDS_POOL_ATTRIB_NUM_CLMNS_MAX: u32 = 65536;
pub const VDS_POOL_ATTRIB_NUM_CLMNS_MIN: u32 = 131072;
pub const VDS_POOL_ATTRIB_PKG_RDNCY_DEF: u32 = 1024;
pub const VDS_POOL_ATTRIB_PKG_RDNCY_MAX: u32 = 256;
pub const VDS_POOL_ATTRIB_PKG_RDNCY_MIN: u32 = 512;
pub const VDS_POOL_ATTRIB_RAIDTYPE: u32 = 1;
pub const VDS_POOL_ATTRIB_STOR_COST_HINT: u32 = 33554432;
pub const VDS_POOL_ATTRIB_STOR_EFFCY_HINT: u32 = 67108864;
pub const VDS_POOL_ATTRIB_STRIPE_SIZE: u32 = 2048;
pub const VDS_POOL_ATTRIB_STRIPE_SIZE_DEF: u32 = 16384;
pub const VDS_POOL_ATTRIB_STRIPE_SIZE_MAX: u32 = 4096;
pub const VDS_POOL_ATTRIB_STRIPE_SIZE_MIN: u32 = 8192;
pub const VDS_POOL_ATTRIB_THIN_PROVISION: u32 = 8;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct VDS_POOL_CUSTOM_ATTRIBUTES {
    pub pwszName: windows_sys::core::PWSTR,
    pub pwszValue: windows_sys::core::PWSTR,
}
impl Default for VDS_POOL_CUSTOM_ATTRIBUTES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct VDS_PORTAL_GROUP_NOTIFICATION {
    pub ulEvent: u32,
    pub portalGroupId: VDS_OBJECT_ID,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct VDS_PORTAL_NOTIFICATION {
    pub ulEvent: u32,
    pub portalId: VDS_OBJECT_ID,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct VDS_PORT_NOTIFICATION {
    pub ulEvent: u32,
    pub portId: VDS_OBJECT_ID,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct VDS_PORT_PROP {
    pub id: VDS_OBJECT_ID,
    pub pwszFriendlyName: windows_sys::core::PWSTR,
    pub pwszIdentification: windows_sys::core::PWSTR,
    pub status: VDS_PORT_STATUS,
}
impl Default for VDS_PORT_PROP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type VDS_PORT_STATUS = i32;
pub type VDS_PROVIDER_FLAG = i32;
pub type VDS_PROVIDER_LBSUPPORT_FLAG = i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct VDS_PROVIDER_PROP {
    pub id: VDS_OBJECT_ID,
    pub pwszName: windows_sys::core::PWSTR,
    pub guidVersionId: windows_sys::core::GUID,
    pub pwszVersion: windows_sys::core::PWSTR,
    pub r#type: VDS_PROVIDER_TYPE,
    pub ulFlags: u32,
    pub ulStripeSizeFlags: u32,
    pub sRebuildPriority: i16,
}
impl Default for VDS_PROVIDER_PROP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type VDS_PROVIDER_TYPE = i32;
pub const VDS_PRS_FAILED: VDS_PORT_STATUS = 5;
pub const VDS_PRS_NOT_READY: VDS_PORT_STATUS = 2;
pub const VDS_PRS_OFFLINE: VDS_PORT_STATUS = 4;
pub const VDS_PRS_ONLINE: VDS_PORT_STATUS = 1;
pub const VDS_PRS_REMOVED: VDS_PORT_STATUS = 8;
pub const VDS_PRS_UNKNOWN: VDS_PORT_STATUS = 0;
pub const VDS_PST_GPT: VDS_PARTITION_STYLE = 2;
pub const VDS_PST_MBR: VDS_PARTITION_STYLE = 1;
pub const VDS_PST_UNKNOWN: VDS_PARTITION_STYLE = 0;
pub const VDS_PS_OFFLINE: VDS_PACK_STATUS = 4;
pub const VDS_PS_ONLINE: VDS_PACK_STATUS = 1;
pub const VDS_PS_UNKNOWN: VDS_PACK_STATUS = 0;
pub const VDS_PTF_SYSTEM: VDS_PARTITION_FLAG = 1;
pub const VDS_PT_HARDWARE: VDS_PROVIDER_TYPE = 2;
pub const VDS_PT_MAX: VDS_PROVIDER_TYPE = 4;
pub const VDS_PT_SOFTWARE: VDS_PROVIDER_TYPE = 1;
pub const VDS_PT_UNKNOWN: VDS_PROVIDER_TYPE = 0;
pub const VDS_PT_VIRTUALDISK: VDS_PROVIDER_TYPE = 3;
pub const VDS_QUERY_HARDWARE_PROVIDERS: VDS_QUERY_PROVIDER_FLAG = 2;
pub type VDS_QUERY_PROVIDER_FLAG = i32;
pub const VDS_QUERY_SOFTWARE_PROVIDERS: VDS_QUERY_PROVIDER_FLAG = 1;
pub const VDS_QUERY_VIRTUALDISK_PROVIDERS: VDS_QUERY_PROVIDER_FLAG = 4;
pub type VDS_RAID_TYPE = i32;
pub const VDS_RA_REFRESH: VDS_RECOVER_ACTION = 1;
pub const VDS_RA_RESTART: VDS_RECOVER_ACTION = 2;
pub const VDS_RA_UNKNOWN: VDS_RECOVER_ACTION = 0;
pub const VDS_REBUILD_PRIORITY_MAX: u32 = 16;
pub const VDS_REBUILD_PRIORITY_MIN: u32 = 0;
pub type VDS_RECOVER_ACTION = i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct VDS_REPARSE_POINT_PROP {
    pub SourceVolumeId: VDS_OBJECT_ID,
    pub pwszPath: windows_sys::core::PWSTR,
}
impl Default for VDS_REPARSE_POINT_PROP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const VDS_RT_RAID0: VDS_RAID_TYPE = 10;
pub const VDS_RT_RAID01: VDS_RAID_TYPE = 17;
pub const VDS_RT_RAID03: VDS_RAID_TYPE = 18;
pub const VDS_RT_RAID05: VDS_RAID_TYPE = 19;
pub const VDS_RT_RAID1: VDS_RAID_TYPE = 11;
pub const VDS_RT_RAID10: VDS_RAID_TYPE = 20;
pub const VDS_RT_RAID15: VDS_RAID_TYPE = 21;
pub const VDS_RT_RAID2: VDS_RAID_TYPE = 12;
pub const VDS_RT_RAID3: VDS_RAID_TYPE = 13;
pub const VDS_RT_RAID30: VDS_RAID_TYPE = 22;
pub const VDS_RT_RAID4: VDS_RAID_TYPE = 14;
pub const VDS_RT_RAID5: VDS_RAID_TYPE = 15;
pub const VDS_RT_RAID50: VDS_RAID_TYPE = 23;
pub const VDS_RT_RAID51: VDS_RAID_TYPE = 24;
pub const VDS_RT_RAID53: VDS_RAID_TYPE = 25;
pub const VDS_RT_RAID6: VDS_RAID_TYPE = 16;
pub const VDS_RT_RAID60: VDS_RAID_TYPE = 26;
pub const VDS_RT_RAID61: VDS_RAID_TYPE = 27;
pub const VDS_RT_UNKNOWN: VDS_RAID_TYPE = 0;
pub type VDS_SAN_POLICY = i32;
pub type VDS_SERVICE_FLAG = i32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct VDS_SERVICE_NOTIFICATION {
    pub ulEvent: u32,
    pub action: VDS_RECOVER_ACTION,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct VDS_SERVICE_PROP {
    pub pwszVersion: windows_sys::core::PWSTR,
    pub ulFlags: u32,
}
impl Default for VDS_SERVICE_PROP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const VDS_SF_CONSISTENCY_CHECK_CAPABLE: VDS_SUB_SYSTEM_FLAG = 16777216;
pub const VDS_SF_DRIVE_EXTENT_CAPABLE: VDS_SUB_SYSTEM_FLAG = 8;
pub const VDS_SF_HARDWARE_CHECKSUM_CAPABLE: VDS_SUB_SYSTEM_FLAG = 16;
pub const VDS_SF_LUN_MASKING_CAPABLE: VDS_SUB_SYSTEM_FLAG = 1;
pub const VDS_SF_LUN_PLEXING_CAPABLE: VDS_SUB_SYSTEM_FLAG = 2;
pub const VDS_SF_LUN_REMAPPING_CAPABLE: VDS_SUB_SYSTEM_FLAG = 4;
pub const VDS_SF_MEDIA_SCAN_CAPABLE: VDS_SUB_SYSTEM_FLAG = 8388608;
pub const VDS_SF_RADIUS_CAPABLE: VDS_SUB_SYSTEM_FLAG = 32;
pub const VDS_SF_READ_BACK_VERIFY_CAPABLE: VDS_SUB_SYSTEM_FLAG = 64;
pub const VDS_SF_READ_CACHING_CAPABLE: VDS_SUB_SYSTEM_FLAG = 2097152;
pub const VDS_SF_SUPPORTS_AUTH_CHAP: VDS_SUB_SYSTEM_FLAG = 65536;
pub const VDS_SF_SUPPORTS_AUTH_MUTUAL_CHAP: VDS_SUB_SYSTEM_FLAG = 131072;
pub const VDS_SF_SUPPORTS_FAULT_TOLERANT_LUNS: VDS_SUB_SYSTEM_FLAG = 512;
pub const VDS_SF_SUPPORTS_LUN_NUMBER: VDS_SUB_SYSTEM_FLAG = 524288;
pub const VDS_SF_SUPPORTS_MIRRORED_CACHE: VDS_SUB_SYSTEM_FLAG = 1048576;
pub const VDS_SF_SUPPORTS_MIRROR_LUNS: VDS_SUB_SYSTEM_FLAG = 16384;
pub const VDS_SF_SUPPORTS_NON_FAULT_TOLERANT_LUNS: VDS_SUB_SYSTEM_FLAG = 1024;
pub const VDS_SF_SUPPORTS_PARITY_LUNS: VDS_SUB_SYSTEM_FLAG = 32768;
pub const VDS_SF_SUPPORTS_RAID01_LUNS: VDS_SUB_SYSTEM_SUPPORTED_RAID_TYPE_FLAG = 32;
pub const VDS_SF_SUPPORTS_RAID03_LUNS: VDS_SUB_SYSTEM_SUPPORTED_RAID_TYPE_FLAG = 64;
pub const VDS_SF_SUPPORTS_RAID05_LUNS: VDS_SUB_SYSTEM_SUPPORTED_RAID_TYPE_FLAG = 128;
pub const VDS_SF_SUPPORTS_RAID10_LUNS: VDS_SUB_SYSTEM_SUPPORTED_RAID_TYPE_FLAG = 256;
pub const VDS_SF_SUPPORTS_RAID15_LUNS: VDS_SUB_SYSTEM_SUPPORTED_RAID_TYPE_FLAG = 512;
pub const VDS_SF_SUPPORTS_RAID2_LUNS: VDS_SUB_SYSTEM_SUPPORTED_RAID_TYPE_FLAG = 1;
pub const VDS_SF_SUPPORTS_RAID30_LUNS: VDS_SUB_SYSTEM_SUPPORTED_RAID_TYPE_FLAG = 1024;
pub const VDS_SF_SUPPORTS_RAID3_LUNS: VDS_SUB_SYSTEM_SUPPORTED_RAID_TYPE_FLAG = 2;
pub const VDS_SF_SUPPORTS_RAID4_LUNS: VDS_SUB_SYSTEM_SUPPORTED_RAID_TYPE_FLAG = 4;
pub const VDS_SF_SUPPORTS_RAID50_LUNS: VDS_SUB_SYSTEM_SUPPORTED_RAID_TYPE_FLAG = 2048;
pub const VDS_SF_SUPPORTS_RAID51_LUNS: VDS_SUB_SYSTEM_SUPPORTED_RAID_TYPE_FLAG = 4096;
pub const VDS_SF_SUPPORTS_RAID53_LUNS: VDS_SUB_SYSTEM_SUPPORTED_RAID_TYPE_FLAG = 8192;
pub const VDS_SF_SUPPORTS_RAID5_LUNS: VDS_SUB_SYSTEM_SUPPORTED_RAID_TYPE_FLAG = 8;
pub const VDS_SF_SUPPORTS_RAID60_LUNS: VDS_SUB_SYSTEM_SUPPORTED_RAID_TYPE_FLAG = 16384;
pub const VDS_SF_SUPPORTS_RAID61_LUNS: VDS_SUB_SYSTEM_SUPPORTED_RAID_TYPE_FLAG = 32768;
pub const VDS_SF_SUPPORTS_RAID6_LUNS: VDS_SUB_SYSTEM_SUPPORTED_RAID_TYPE_FLAG = 16;
pub const VDS_SF_SUPPORTS_SIMPLE_LUNS: VDS_SUB_SYSTEM_FLAG = 2048;
pub const VDS_SF_SUPPORTS_SIMPLE_TARGET_CONFIG: VDS_SUB_SYSTEM_FLAG = 262144;
pub const VDS_SF_SUPPORTS_SPAN_LUNS: VDS_SUB_SYSTEM_FLAG = 4096;
pub const VDS_SF_SUPPORTS_STRIPE_LUNS: VDS_SUB_SYSTEM_FLAG = 8192;
pub const VDS_SF_WRITE_CACHING_CAPABLE: VDS_SUB_SYSTEM_FLAG = 4194304;
pub const VDS_SF_WRITE_THROUGH_CACHING_CAPABLE: VDS_SUB_SYSTEM_FLAG = 128;
pub const VDS_SPS_NOT_READY: VDS_STORAGE_POOL_STATUS = 2;
pub const VDS_SPS_OFFLINE: VDS_STORAGE_POOL_STATUS = 4;
pub const VDS_SPS_ONLINE: VDS_STORAGE_POOL_STATUS = 1;
pub const VDS_SPS_UNKNOWN: VDS_STORAGE_POOL_STATUS = 0;
pub const VDS_SPT_CONCRETE: VDS_STORAGE_POOL_TYPE = 2;
pub const VDS_SPT_PRIMORDIAL: VDS_STORAGE_POOL_TYPE = 1;
pub const VDS_SPT_UNKNOWN: VDS_STORAGE_POOL_TYPE = 0;
pub const VDS_SP_MAX: VDS_SAN_POLICY = 5;
pub const VDS_SP_OFFLINE: VDS_SAN_POLICY = 3;
pub const VDS_SP_OFFLINE_INTERNAL: VDS_SAN_POLICY = 4;
pub const VDS_SP_OFFLINE_SHARED: VDS_SAN_POLICY = 2;
pub const VDS_SP_ONLINE: VDS_SAN_POLICY = 1;
pub const VDS_SP_UNKNOWN: VDS_SAN_POLICY = 0;
pub const VDS_SSS_FAILED: VDS_SUB_SYSTEM_STATUS = 5;
pub const VDS_SSS_NOT_READY: VDS_SUB_SYSTEM_STATUS = 2;
pub const VDS_SSS_OFFLINE: VDS_SUB_SYSTEM_STATUS = 4;
pub const VDS_SSS_ONLINE: VDS_SUB_SYSTEM_STATUS = 1;
pub const VDS_SSS_PARTIALLY_MANAGED: VDS_SUB_SYSTEM_STATUS = 9;
pub const VDS_SSS_UNKNOWN: VDS_SUB_SYSTEM_STATUS = 0;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct VDS_STORAGE_POOL_DRIVE_EXTENT {
    pub id: VDS_OBJECT_ID,
    pub ullSize: u64,
    pub bUsed: windows_sys::core::BOOL,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct VDS_STORAGE_POOL_PROP {
    pub id: VDS_OBJECT_ID,
    pub status: VDS_STORAGE_POOL_STATUS,
    pub health: VDS_HEALTH,
    pub r#type: VDS_STORAGE_POOL_TYPE,
    pub pwszName: windows_sys::core::PWSTR,
    pub pwszDescription: windows_sys::core::PWSTR,
    pub ullTotalConsumedSpace: u64,
    pub ullTotalManagedSpace: u64,
    pub ullRemainingFreeSpace: u64,
}
impl Default for VDS_STORAGE_POOL_PROP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type VDS_STORAGE_POOL_STATUS = i32;
pub type VDS_STORAGE_POOL_TYPE = i32;
pub type VDS_SUB_SYSTEM_FLAG = i32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct VDS_SUB_SYSTEM_NOTIFICATION {
    pub ulEvent: u32,
    pub subSystemId: VDS_OBJECT_ID,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct VDS_SUB_SYSTEM_PROP {
    pub id: VDS_OBJECT_ID,
    pub pwszFriendlyName: windows_sys::core::PWSTR,
    pub pwszIdentification: windows_sys::core::PWSTR,
    pub ulFlags: u32,
    pub ulStripeSizeFlags: u32,
    pub status: VDS_SUB_SYSTEM_STATUS,
    pub health: VDS_HEALTH,
    pub sNumberOfInternalBuses: i16,
    pub sMaxNumberOfSlotsEachBus: i16,
    pub sMaxNumberOfControllers: i16,
    pub sRebuildPriority: i16,
}
impl Default for VDS_SUB_SYSTEM_PROP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct VDS_SUB_SYSTEM_PROP2 {
    pub id: VDS_OBJECT_ID,
    pub pwszFriendlyName: windows_sys::core::PWSTR,
    pub pwszIdentification: windows_sys::core::PWSTR,
    pub ulFlags: u32,
    pub ulStripeSizeFlags: u32,
    pub ulSupportedRaidTypeFlags: u32,
    pub status: VDS_SUB_SYSTEM_STATUS,
    pub health: VDS_HEALTH,
    pub sNumberOfInternalBuses: i16,
    pub sMaxNumberOfSlotsEachBus: i16,
    pub sMaxNumberOfControllers: i16,
    pub sRebuildPriority: i16,
    pub ulNumberOfEnclosures: u32,
}
impl Default for VDS_SUB_SYSTEM_PROP2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type VDS_SUB_SYSTEM_STATUS = i32;
pub type VDS_SUB_SYSTEM_SUPPORTED_RAID_TYPE_FLAG = i32;
pub const VDS_SVF_AUTO_MOUNT_OFF: VDS_SERVICE_FLAG = 32;
pub const VDS_SVF_CLUSTER_SERVICE_CONFIGURED: VDS_SERVICE_FLAG = 16;
pub const VDS_SVF_EFI: VDS_SERVICE_FLAG = 128;
pub const VDS_SVF_OS_UNINSTALL_VALID: VDS_SERVICE_FLAG = 64;
pub const VDS_SVF_SUPPORT_DYNAMIC: VDS_SERVICE_FLAG = 1;
pub const VDS_SVF_SUPPORT_DYNAMIC_1394: VDS_SERVICE_FLAG = 8;
pub const VDS_SVF_SUPPORT_FAULT_TOLERANT: VDS_SERVICE_FLAG = 2;
pub const VDS_SVF_SUPPORT_GPT: VDS_SERVICE_FLAG = 4;
pub const VDS_SVF_SUPPORT_MIRROR: VDS_SERVICE_FLAG = 256;
pub const VDS_SVF_SUPPORT_RAID5: VDS_SERVICE_FLAG = 512;
pub const VDS_SVF_SUPPORT_REFS: VDS_SERVICE_FLAG = 1024;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct VDS_TARGET_NOTIFICATION {
    pub ulEvent: u32,
    pub targetId: VDS_OBJECT_ID,
}
pub type VDS_TRANSITION_STATE = i32;
pub const VDS_TS_EXTENDING: VDS_TRANSITION_STATE = 2;
pub const VDS_TS_RECONFIGING: VDS_TRANSITION_STATE = 4;
pub const VDS_TS_RESTRIPING: VDS_TRANSITION_STATE = 5;
pub const VDS_TS_SHRINKING: VDS_TRANSITION_STATE = 3;
pub const VDS_TS_STABLE: VDS_TRANSITION_STATE = 1;
pub const VDS_TS_UNKNOWN: VDS_TRANSITION_STATE = 0;
#[repr(C)]
#[cfg(all(feature = "virtdisk", feature = "winioctl"))]
#[derive(Clone, Copy)]
pub struct VDS_VDISK_PROPERTIES {
    pub Id: VDS_OBJECT_ID,
    pub State: VDS_VDISK_STATE,
    pub VirtualDeviceType: super::VIRTUAL_STORAGE_TYPE,
    pub VirtualSize: u64,
    pub PhysicalSize: u64,
    pub pPath: windows_sys::core::PWSTR,
    pub pDeviceName: windows_sys::core::PWSTR,
    pub DiskFlag: super::DEPENDENT_DISK_FLAG,
    pub bIsChild: windows_sys::core::BOOL,
    pub pParentPath: windows_sys::core::PWSTR,
}
#[cfg(all(feature = "virtdisk", feature = "winioctl"))]
impl Default for VDS_VDISK_PROPERTIES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type VDS_VDISK_STATE = i32;
pub type VDS_VERSION_SUPPORT_FLAG = i32;
pub const VDS_VF_ACTIVE: VDS_VOLUME_FLAG = 4;
pub const VDS_VF_BACKED_BY_WIM_IMAGE: VDS_VOLUME_FLAG = 33554432;
pub const VDS_VF_BACKS_BOOT_VOLUME: VDS_VOLUME_FLAG = 16777216;
pub const VDS_VF_BOOT_VOLUME: VDS_VOLUME_FLAG = 2;
pub const VDS_VF_CAN_EXTEND: VDS_VOLUME_FLAG = 32;
pub const VDS_VF_CAN_SHRINK: VDS_VOLUME_FLAG = 64;
pub const VDS_VF_CRASHDUMP: VDS_VOLUME_FLAG = 512;
pub const VDS_VF_DIRTY: VDS_VOLUME_FLAG = 4194304;
pub const VDS_VF_FAT32_NOT_SUPPORTED: VDS_VOLUME_FLAG = 32768;
pub const VDS_VF_FAT_NOT_SUPPORTED: VDS_VOLUME_FLAG = 65536;
pub const VDS_VF_FORMATTING: VDS_VOLUME_FLAG = 4096;
pub const VDS_VF_FVE_ENABLED: VDS_VOLUME_FLAG = 2097152;
pub const VDS_VF_HIBERNATION: VDS_VOLUME_FLAG = 256;
pub const VDS_VF_HIDDEN: VDS_VOLUME_FLAG = 16;
pub const VDS_VF_INSTALLABLE: VDS_VOLUME_FLAG = 1024;
pub const VDS_VF_LBN_REMAP_ENABLED: VDS_VOLUME_FLAG = 2048;
pub const VDS_VF_NOT_FORMATTABLE: VDS_VOLUME_FLAG = 8192;
pub const VDS_VF_NO_DEFAULT_DRIVE_LETTER: VDS_VOLUME_FLAG = 131072;
pub const VDS_VF_NTFS_NOT_SUPPORTED: VDS_VOLUME_FLAG = 16384;
pub const VDS_VF_PAGEFILE: VDS_VOLUME_FLAG = 128;
pub const VDS_VF_PERMANENTLY_DISMOUNTED: VDS_VOLUME_FLAG = 262144;
pub const VDS_VF_PERMANENT_DISMOUNT_SUPPORTED: VDS_VOLUME_FLAG = 524288;
pub const VDS_VF_READONLY: VDS_VOLUME_FLAG = 8;
pub const VDS_VF_REFS_NOT_SUPPORTED: VDS_VOLUME_FLAG = 8388608;
pub const VDS_VF_SHADOW_COPY: VDS_VOLUME_FLAG = 1048576;
pub const VDS_VF_SYSTEM_VOLUME: VDS_VOLUME_FLAG = 1;
pub type VDS_VOLUME_FLAG = i32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct VDS_VOLUME_NOTIFICATION {
    pub ulEvent: u32,
    pub volumeId: VDS_OBJECT_ID,
    pub plexId: VDS_OBJECT_ID,
    pub ulPercentCompleted: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct VDS_VOLUME_PLEX_PROP {
    pub id: VDS_OBJECT_ID,
    pub r#type: VDS_VOLUME_PLEX_TYPE,
    pub status: VDS_VOLUME_PLEX_STATUS,
    pub health: VDS_HEALTH,
    pub TransitionState: VDS_TRANSITION_STATE,
    pub ullSize: u64,
    pub ulStripeSize: u32,
    pub ulNumberOfMembers: u32,
}
pub type VDS_VOLUME_PLEX_STATUS = i32;
pub type VDS_VOLUME_PLEX_TYPE = i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct VDS_VOLUME_PROP {
    pub id: VDS_OBJECT_ID,
    pub r#type: VDS_VOLUME_TYPE,
    pub status: VDS_VOLUME_STATUS,
    pub health: VDS_HEALTH,
    pub TransitionState: VDS_TRANSITION_STATE,
    pub ullSize: u64,
    pub ulFlags: u32,
    pub RecommendedFileSystemType: VDS_FILE_SYSTEM_TYPE,
    pub pwszName: windows_sys::core::PWSTR,
}
impl Default for VDS_VOLUME_PROP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct VDS_VOLUME_PROP2 {
    pub id: VDS_OBJECT_ID,
    pub r#type: VDS_VOLUME_TYPE,
    pub status: VDS_VOLUME_STATUS,
    pub health: VDS_HEALTH,
    pub TransitionState: VDS_TRANSITION_STATE,
    pub ullSize: u64,
    pub ulFlags: u32,
    pub RecommendedFileSystemType: VDS_FILE_SYSTEM_TYPE,
    pub cbUniqueId: u32,
    pub pwszName: windows_sys::core::PWSTR,
    pub pUniqueId: *mut u8,
}
impl Default for VDS_VOLUME_PROP2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type VDS_VOLUME_STATUS = i32;
pub type VDS_VOLUME_TYPE = i32;
pub const VDS_VPS_FAILED: VDS_VOLUME_PLEX_STATUS = 5;
pub const VDS_VPS_NO_MEDIA: VDS_VOLUME_PLEX_STATUS = 3;
pub const VDS_VPS_ONLINE: VDS_VOLUME_PLEX_STATUS = 1;
pub const VDS_VPS_UNKNOWN: VDS_VOLUME_PLEX_STATUS = 0;
pub const VDS_VPT_PARITY: VDS_VOLUME_PLEX_TYPE = 14;
pub const VDS_VPT_SIMPLE: VDS_VOLUME_PLEX_TYPE = 10;
pub const VDS_VPT_SPAN: VDS_VOLUME_PLEX_TYPE = 11;
pub const VDS_VPT_STRIPE: VDS_VOLUME_PLEX_TYPE = 12;
pub const VDS_VPT_UNKNOWN: VDS_VOLUME_PLEX_TYPE = 0;
pub const VDS_VSF_1_0: VDS_VERSION_SUPPORT_FLAG = 1;
pub const VDS_VSF_1_1: VDS_VERSION_SUPPORT_FLAG = 2;
pub const VDS_VSF_2_0: VDS_VERSION_SUPPORT_FLAG = 4;
pub const VDS_VSF_2_1: VDS_VERSION_SUPPORT_FLAG = 8;
pub const VDS_VSF_3_0: VDS_VERSION_SUPPORT_FLAG = 16;
pub const VDS_VST_ADDED: VDS_VDISK_STATE = 1;
pub const VDS_VST_ATTACHED: VDS_VDISK_STATE = 5;
pub const VDS_VST_ATTACHED_NOT_OPEN: VDS_VDISK_STATE = 4;
pub const VDS_VST_ATTACH_PENDING: VDS_VDISK_STATE = 3;
pub const VDS_VST_COMPACTING: VDS_VDISK_STATE = 7;
pub const VDS_VST_DELETED: VDS_VDISK_STATE = 10;
pub const VDS_VST_DETACH_PENDING: VDS_VDISK_STATE = 6;
pub const VDS_VST_EXPANDING: VDS_VDISK_STATE = 9;
pub const VDS_VST_MAX: VDS_VDISK_STATE = 11;
pub const VDS_VST_MERGING: VDS_VDISK_STATE = 8;
pub const VDS_VST_OPEN: VDS_VDISK_STATE = 2;
pub const VDS_VST_UNKNOWN: VDS_VDISK_STATE = 0;
pub const VDS_VS_FAILED: VDS_VOLUME_STATUS = 5;
pub const VDS_VS_NO_MEDIA: VDS_VOLUME_STATUS = 3;
pub const VDS_VS_OFFLINE: VDS_VOLUME_STATUS = 4;
pub const VDS_VS_ONLINE: VDS_VOLUME_STATUS = 1;
pub const VDS_VS_UNKNOWN: VDS_VOLUME_STATUS = 0;
pub const VDS_VT_MIRROR: VDS_VOLUME_TYPE = 13;
pub const VDS_VT_PARITY: VDS_VOLUME_TYPE = 14;
pub const VDS_VT_SIMPLE: VDS_VOLUME_TYPE = 10;
pub const VDS_VT_SPAN: VDS_VOLUME_TYPE = 11;
pub const VDS_VT_STRIPE: VDS_VOLUME_TYPE = 12;
pub const VDS_VT_UNKNOWN: VDS_VOLUME_TYPE = 0;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct VDS_WWN {
    pub rguchWwn: [u8; 8],
}
impl Default for VDS_WWN {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type __VDS_PARTITION_STYLE = i32;
