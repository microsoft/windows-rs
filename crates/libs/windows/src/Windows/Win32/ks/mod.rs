#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn KsCreateAllocator(connectionhandle: super::HANDLE, allocatorframing: PKSALLOCATOR_FRAMING, allocatorhandle: super::PHANDLE) -> u32 {
    windows_core::link!("ksuser.dll" "system" fn KsCreateAllocator(connectionhandle : super::HANDLE, allocatorframing : PKSALLOCATOR_FRAMING, allocatorhandle : super::PHANDLE) -> u32);
    unsafe { KsCreateAllocator(connectionhandle, allocatorframing, allocatorhandle as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn KsCreateAllocator2(connectionhandle: super::HANDLE, allocatorframing: PKSALLOCATOR_FRAMING, allocatorhandle: super::PHANDLE) -> windows_core::HRESULT {
    windows_core::link!("ksuser.dll" "system" fn KsCreateAllocator2(connectionhandle : super::HANDLE, allocatorframing : PKSALLOCATOR_FRAMING, allocatorhandle : super::PHANDLE) -> windows_core::HRESULT);
    unsafe { KsCreateAllocator2(connectionhandle, allocatorframing, allocatorhandle as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn KsCreateClock(connectionhandle: super::HANDLE, clockcreate: PKSCLOCK_CREATE, clockhandle: super::PHANDLE) -> u32 {
    windows_core::link!("ksuser.dll" "system" fn KsCreateClock(connectionhandle : super::HANDLE, clockcreate : PKSCLOCK_CREATE, clockhandle : super::PHANDLE) -> u32);
    unsafe { KsCreateClock(connectionhandle, clockcreate, clockhandle as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn KsCreateClock2(connectionhandle: super::HANDLE, clockcreate: PKSCLOCK_CREATE, clockhandle: super::PHANDLE) -> windows_core::HRESULT {
    windows_core::link!("ksuser.dll" "system" fn KsCreateClock2(connectionhandle : super::HANDLE, clockcreate : PKSCLOCK_CREATE, clockhandle : super::PHANDLE) -> windows_core::HRESULT);
    unsafe { KsCreateClock2(connectionhandle, clockcreate, clockhandle as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn KsCreatePin(filterhandle: super::HANDLE, connect: PKSPIN_CONNECT, desiredaccess: super::ACCESS_MASK, connectionhandle: super::PHANDLE) -> u32 {
    windows_core::link!("ksuser.dll" "system" fn KsCreatePin(filterhandle : super::HANDLE, connect : PKSPIN_CONNECT, desiredaccess : super::ACCESS_MASK, connectionhandle : super::PHANDLE) -> u32);
    unsafe { KsCreatePin(filterhandle, connect, desiredaccess, connectionhandle as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn KsCreatePin2(filterhandle: super::HANDLE, connect: PKSPIN_CONNECT, desiredaccess: super::ACCESS_MASK, connectionhandle: super::PHANDLE) -> windows_core::HRESULT {
    windows_core::link!("ksuser.dll" "system" fn KsCreatePin2(filterhandle : super::HANDLE, connect : PKSPIN_CONNECT, desiredaccess : super::ACCESS_MASK, connectionhandle : super::PHANDLE) -> windows_core::HRESULT);
    unsafe { KsCreatePin2(filterhandle, connect, desiredaccess, connectionhandle as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn KsCreateTopologyNode(parenthandle: super::HANDLE, nodecreate: PKSNODE_CREATE, desiredaccess: super::ACCESS_MASK, nodehandle: super::PHANDLE) -> u32 {
    windows_core::link!("ksuser.dll" "system" fn KsCreateTopologyNode(parenthandle : super::HANDLE, nodecreate : PKSNODE_CREATE, desiredaccess : super::ACCESS_MASK, nodehandle : super::PHANDLE) -> u32);
    unsafe { KsCreateTopologyNode(parenthandle, nodecreate, desiredaccess, nodehandle as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn KsCreateTopologyNode2(parenthandle: super::HANDLE, nodecreate: PKSNODE_CREATE, desiredaccess: super::ACCESS_MASK, nodehandle: super::PHANDLE) -> windows_core::HRESULT {
    windows_core::link!("ksuser.dll" "system" fn KsCreateTopologyNode2(parenthandle : super::HANDLE, nodecreate : PKSNODE_CREATE, desiredaccess : super::ACCESS_MASK, nodehandle : super::PHANDLE) -> windows_core::HRESULT);
    unsafe { KsCreateTopologyNode2(parenthandle, nodecreate, desiredaccess, nodehandle as _) }
}
pub const GUID_NULL: windows_core::GUID = windows_core::GUID::from_u128(0x00000000_0000_0000_0000_000000000000);
pub const IOCTL_KS_DISABLE_EVENT: i32 = 3080203;
pub const IOCTL_KS_ENABLE_EVENT: i32 = 3080199;
pub const IOCTL_KS_METHOD: i32 = 3080207;
pub const IOCTL_KS_PROPERTY: i32 = 3080195;
pub const IOCTL_KS_READ_STREAM: i32 = 3096599;
pub const IOCTL_KS_RESET_STATE: i32 = 3080219;
pub const IOCTL_KS_WRITE_STREAM: i32 = 3112979;
pub const KSALLOCATOR_FLAG_2D_BUFFER_REQUIRED: i32 = 32768;
pub const KSALLOCATOR_FLAG_ALLOCATOR_EXISTS: i32 = 2048;
pub const KSALLOCATOR_FLAG_ATTENTION_STEPPING: i32 = 8192;
pub const KSALLOCATOR_FLAG_CAN_ALLOCATE: i32 = 64;
pub const KSALLOCATOR_FLAG_CYCLE: i32 = 1024;
pub const KSALLOCATOR_FLAG_DEVICE_SPECIFIC: i32 = 32;
pub const KSALLOCATOR_FLAG_ENABLE_CACHED_MDL: i32 = 16384;
pub const KSALLOCATOR_FLAG_INDEPENDENT_RANGES: i32 = 4096;
pub const KSALLOCATOR_FLAG_INSIST_ON_FRAMESIZE_RATIO: i32 = 128;
pub const KSALLOCATOR_FLAG_MULTIPLE_OUTPUT: i32 = 512;
pub const KSALLOCATOR_FLAG_NO_FRAME_INTEGRITY: i32 = 256;
pub const KSALLOCATOR_FLAG_PARTIAL_READ_SUPPORT: i32 = 16;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KSALLOCATOR_FRAMING {
    pub Anonymous: KSALLOCATOR_FRAMING_0,
    pub PoolType: u32,
    pub Frames: u32,
    pub FrameSize: u32,
    pub Anonymous2: KSALLOCATOR_FRAMING_1,
    pub Reserved: u32,
}
impl Default for KSALLOCATOR_FRAMING {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union KSALLOCATOR_FRAMING_0 {
    pub OptionsFlags: u32,
    pub RequirementsFlags: u32,
}
impl Default for KSALLOCATOR_FRAMING_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union KSALLOCATOR_FRAMING_1 {
    pub FileAlignment: u32,
    pub FramePitch: i32,
}
impl Default for KSALLOCATOR_FRAMING_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KSALLOCATOR_FRAMING_EX {
    pub CountItems: u32,
    pub PinFlags: u32,
    pub OutputCompression: KS_COMPRESSION,
    pub PinWeight: u32,
    pub FramingItem: [KS_FRAMING_ITEM; 1],
}
impl Default for KSALLOCATOR_FRAMING_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const KSALLOCATOR_OPTIONF_COMPATIBLE: i32 = 1;
pub const KSALLOCATOR_OPTIONF_SYSTEM_MEMORY: i32 = 2;
pub const KSALLOCATOR_OPTIONF_VALID: i32 = 3;
pub const KSALLOCATOR_REQUIREMENTF_FRAME_INTEGRITY: i32 = 4;
pub const KSALLOCATOR_REQUIREMENTF_INPLACE_MODIFIER: i32 = 1;
pub const KSALLOCATOR_REQUIREMENTF_MUST_ALLOCATE: i32 = 8;
pub const KSALLOCATOR_REQUIREMENTF_PREFERENCES_ONLY: u32 = 2147483648;
pub const KSALLOCATOR_REQUIREMENTF_SYSTEM_MEMORY: i32 = 2;
pub const KSALLOCATOR_REQUIREMENTF_SYSTEM_MEMORY_CUSTOM_ALLOCATION: i32 = 16;
pub const KSALL_NODES: u32 = 4294967295;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSATTRIBUTE {
    pub Size: u32,
    pub Flags: u32,
    pub Attribute: windows_core::GUID,
}
pub const KSATTRIBUTE_REQUIRED: i32 = 1;
pub const KSCATEGORY_BRIDGE: windows_core::GUID = windows_core::GUID::from_u128(0x085aff00_62ce_11cf_a5d6_28db04c10000);
pub const KSCATEGORY_CAPTURE: windows_core::GUID = windows_core::GUID::from_u128(0x65e8773d_8f56_11d0_a3b9_00a0c9223196);
pub const KSCATEGORY_CLOCK: windows_core::GUID = windows_core::GUID::from_u128(0x53172480_4791_11d0_a5d6_28db04c10000);
pub const KSCATEGORY_COMMUNICATIONSTRANSFORM: windows_core::GUID = windows_core::GUID::from_u128(0xcf1dda2c_9743_11d0_a3ee_00a0c9223196);
pub const KSCATEGORY_DATACOMPRESSOR: windows_core::GUID = windows_core::GUID::from_u128(0x1e84c900_7e70_11d0_a5d6_28db04c10000);
pub const KSCATEGORY_DATADECOMPRESSOR: windows_core::GUID = windows_core::GUID::from_u128(0x2721ae20_7e70_11d0_a5d6_28db04c10000);
pub const KSCATEGORY_DATATRANSFORM: windows_core::GUID = windows_core::GUID::from_u128(0x2eb07ea0_7e70_11d0_a5d6_28db04c10000);
pub const KSCATEGORY_FILESYSTEM: windows_core::GUID = windows_core::GUID::from_u128(0x760fed5e_9357_11d0_a3cc_00a0c9223196);
pub const KSCATEGORY_INTERFACETRANSFORM: windows_core::GUID = windows_core::GUID::from_u128(0xcf1dda2d_9743_11d0_a3ee_00a0c9223196);
pub const KSCATEGORY_MEDIUMTRANSFORM: windows_core::GUID = windows_core::GUID::from_u128(0xcf1dda2e_9743_11d0_a3ee_00a0c9223196);
pub const KSCATEGORY_MIXER: windows_core::GUID = windows_core::GUID::from_u128(0xad809c00_7b88_11d0_a5d6_28db04c10000);
pub const KSCATEGORY_NETWORK_CAMERA: windows_core::GUID = windows_core::GUID::from_u128(0xb8238652_b500_41eb_b4f3_4234f7f5ae99);
pub const KSCATEGORY_PROXY: windows_core::GUID = windows_core::GUID::from_u128(0x97ebaaca_95bd_11d0_a3ea_00a0c9223196);
pub const KSCATEGORY_QUALITY: windows_core::GUID = windows_core::GUID::from_u128(0x97ebaacb_95bd_11d0_a3ea_00a0c9223196);
pub const KSCATEGORY_RENDER: windows_core::GUID = windows_core::GUID::from_u128(0x65e8773e_8f56_11d0_a3b9_00a0c9223196);
pub const KSCATEGORY_SENSOR_CAMERA: windows_core::GUID = windows_core::GUID::from_u128(0x24e552d7_6523_47f7_a647_d3465bf1f5ca);
pub const KSCATEGORY_SENSOR_GROUP: windows_core::GUID = windows_core::GUID::from_u128(0x669c7214_0a88_4311_a7f3_4e79820e33bd);
pub const KSCATEGORY_SPLITTER: windows_core::GUID = windows_core::GUID::from_u128(0x0a4252a0_7e70_11d0_a5d6_28db04c10000);
pub const KSCATEGORY_VIDEO_CAMERA: windows_core::GUID = windows_core::GUID::from_u128(0xe5323777_f976_4f5b_9b55_b94699c46e44);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSCLOCK_CREATE {
    pub CreateFlags: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSCOMPONENTID {
    pub Manufacturer: windows_core::GUID,
    pub Product: windows_core::GUID,
    pub Component: windows_core::GUID,
    pub Name: windows_core::GUID,
    pub Version: u32,
    pub Revision: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSCORRELATED_TIME {
    pub Time: i64,
    pub SystemTime: i64,
}
pub const KSDATAFORMAT_ATTRIBUTES: i32 = 2;
pub const KSDATAFORMAT_BIT_ATTRIBUTES: i32 = 1;
pub const KSDATAFORMAT_BIT_TEMPORAL_COMPRESSION: i32 = 0;
pub const KSDATAFORMAT_SPECIFIER_FILEHANDLE: windows_core::GUID = windows_core::GUID::from_u128(0x65e8773c_8f56_11d0_a3b9_00a0c9223196);
pub const KSDATAFORMAT_SPECIFIER_FILENAME: windows_core::GUID = windows_core::GUID::from_u128(0xaa797b40_e974_11cf_a5d6_28db04c10000);
pub const KSDATAFORMAT_SPECIFIER_NONE: windows_core::GUID = windows_core::GUID::from_u128(0x0f6417d6_c318_11d0_a43f_00a0c9223196);
pub const KSDATAFORMAT_SUBTYPE_NONE: windows_core::GUID = windows_core::GUID::from_u128(0xe436eb8e_524f_11ce_9f53_0020af0ba770);
pub const KSDATAFORMAT_TEMPORAL_COMPRESSION: i32 = 1;
pub const KSDATAFORMAT_TYPE_STREAM: windows_core::GUID = windows_core::GUID::from_u128(0xe436eb83_524f_11ce_9f53_0020af0ba770);
#[repr(C)]
#[derive(Clone, Copy)]
pub union KSDATARANGE {
    pub Anonymous: KSDATARANGE_0,
    pub Alignment: i64,
}
impl Default for KSDATARANGE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSDATARANGE_0 {
    pub FormatSize: u32,
    pub Flags: u32,
    pub SampleSize: u32,
    pub Reserved: u32,
    pub MajorFormat: windows_core::GUID,
    pub SubFormat: windows_core::GUID,
    pub Specifier: windows_core::GUID,
}
pub const KSDATARANGE_ATTRIBUTES: i32 = 2;
pub const KSDATARANGE_BIT_ATTRIBUTES: i32 = 1;
pub const KSDATARANGE_BIT_REQUIRED_ATTRIBUTES: i32 = 2;
pub const KSDATARANGE_REQUIRED_ATTRIBUTES: i32 = 4;
pub type KSDEGRADE = KSIDENTIFIER;
pub const KSDEGRADESETID_Standard: windows_core::GUID = windows_core::GUID::from_u128(0x9f564180_704c_11d0_a5d6_28db04c10000);
pub type KSDEGRADE_STANDARD = i32;
pub const KSDEGRADE_STANDARD_COMPUTATION: KSDEGRADE_STANDARD = 2;
pub const KSDEGRADE_STANDARD_QUALITY: KSDEGRADE_STANDARD = 1;
pub const KSDEGRADE_STANDARD_SAMPLE: KSDEGRADE_STANDARD = 0;
pub const KSDEGRADE_STANDARD_SKIP: KSDEGRADE_STANDARD = 3;
pub type KSDEVICE_THERMAL_STATE = i32;
pub const KSDEVICE_THERMAL_STATE_HIGH: KSDEVICE_THERMAL_STATE = 1;
pub const KSDEVICE_THERMAL_STATE_LOW: KSDEVICE_THERMAL_STATE = 0;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSERROR {
    pub Context: *mut core::ffi::c_void,
    pub Status: u32,
}
pub type KSEVENT = KSIDENTIFIER;
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct KSEVENTDATA {
    pub NotificationType: u32,
    pub Anonymous: KSEVENTDATA_0,
}
#[cfg(feature = "winnt")]
impl Default for KSEVENTDATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub union KSEVENTDATA_0 {
    pub EventHandle: KSEVENTDATA_0_0,
    pub SemaphoreHandle: KSEVENTDATA_0_1,
    pub Alignment: KSEVENTDATA_0_2,
}
#[cfg(feature = "winnt")]
impl Default for KSEVENTDATA_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct KSEVENTDATA_0_0 {
    pub Event: super::HANDLE,
    pub Reserved: [usize; 2],
}
#[cfg(feature = "winnt")]
impl Default for KSEVENTDATA_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSEVENTDATA_0_1 {
    pub Semaphore: super::HANDLE,
    pub Reserved: u32,
    pub Adjustment: i32,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct KSEVENTDATA_0_2 {
    pub Unused: *mut core::ffi::c_void,
    pub Alignment: [isize; 2],
}
#[cfg(feature = "winnt")]
impl Default for KSEVENTDATA_0_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const KSEVENTF_EVENT_HANDLE: i32 = 1;
pub const KSEVENTF_SEMAPHORE_HANDLE: i32 = 2;
pub const KSEVENTSETID_Clock: windows_core::GUID = windows_core::GUID::from_u128(0x364d8e20_62c7_11cf_a5d6_28db04c10000);
pub const KSEVENTSETID_Connection: windows_core::GUID = windows_core::GUID::from_u128(0x7f4bcbe0_9ea5_11cf_a5d6_28db04c10000);
pub const KSEVENTSETID_Device: windows_core::GUID = windows_core::GUID::from_u128(0x288296ec_9f94_41b4_a153_aa31aeecb33f);
pub const KSEVENTSETID_PinCapsChange: windows_core::GUID = windows_core::GUID::from_u128(0xdd4f192e_3b78_49ad_a534_2c315b822000);
pub const KSEVENTSETID_StreamAllocator: windows_core::GUID = windows_core::GUID::from_u128(0x75d95571_073c_11d0_a161_0020afd156e4);
pub const KSEVENTSETID_VolumeLimit: windows_core::GUID = windows_core::GUID::from_u128(0xda168465_3a7c_4858_9d4a_3e8e24701aef);
pub const KSEVENT_CLOCK_INTERVAL_MARK: KSEVENT_CLOCK_POSITION = 0;
pub type KSEVENT_CLOCK_POSITION = i32;
pub const KSEVENT_CLOCK_POSITION_MARK: KSEVENT_CLOCK_POSITION = 1;
pub type KSEVENT_CONNECTION = i32;
pub const KSEVENT_CONNECTION_DATADISCONTINUITY: KSEVENT_CONNECTION = 1;
pub const KSEVENT_CONNECTION_ENDOFSTREAM: KSEVENT_CONNECTION = 4;
pub const KSEVENT_CONNECTION_POSITIONUPDATE: KSEVENT_CONNECTION = 0;
pub const KSEVENT_CONNECTION_PRIORITY: KSEVENT_CONNECTION = 3;
pub const KSEVENT_CONNECTION_TIMEDISCONTINUITY: KSEVENT_CONNECTION = 2;
pub type KSEVENT_DEVICE = i32;
pub const KSEVENT_DEVICE_LOST: KSEVENT_DEVICE = 0;
pub const KSEVENT_DEVICE_PREEMPTED: KSEVENT_DEVICE = 1;
pub const KSEVENT_DEVICE_THERMAL_HIGH: KSEVENT_DEVICE = 2;
pub const KSEVENT_DEVICE_THERMAL_LOW: KSEVENT_DEVICE = 3;
pub type KSEVENT_PINCAPS_CHANGENOTIFICATIONS = i32;
pub const KSEVENT_PINCAPS_FORMATCHANGE: KSEVENT_PINCAPS_CHANGENOTIFICATIONS = 0;
pub const KSEVENT_PINCAPS_INVALIDATECLIENTS: KSEVENT_PINCAPS_CHANGENOTIFICATIONS = 2;
pub const KSEVENT_PINCAPS_JACKINFOCHANGE: KSEVENT_PINCAPS_CHANGENOTIFICATIONS = 1;
pub type KSEVENT_STREAMALLOCATOR = i32;
pub const KSEVENT_STREAMALLOCATOR_FREEFRAME: KSEVENT_STREAMALLOCATOR = 1;
pub const KSEVENT_STREAMALLOCATOR_INTERNAL_FREEFRAME: KSEVENT_STREAMALLOCATOR = 0;
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct KSEVENT_TIME_INTERVAL {
    pub EventData: KSEVENTDATA,
    pub TimeBase: i64,
    pub Interval: i64,
}
#[cfg(feature = "winnt")]
impl Default for KSEVENT_TIME_INTERVAL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct KSEVENT_TIME_MARK {
    pub EventData: KSEVENTDATA,
    pub MarkTime: i64,
}
#[cfg(feature = "winnt")]
impl Default for KSEVENT_TIME_MARK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const KSEVENT_TYPE_BASICSUPPORT: i32 = 512;
pub const KSEVENT_TYPE_ENABLE: i32 = 1;
pub const KSEVENT_TYPE_ENABLEBUFFERED: i32 = 4;
pub const KSEVENT_TYPE_ONESHOT: i32 = 2;
pub const KSEVENT_TYPE_QUERYBUFFER: i32 = 1024;
pub const KSEVENT_TYPE_SETSUPPORT: i32 = 256;
pub const KSEVENT_TYPE_TOPOLOGY: i32 = 268435456;
pub type KSEVENT_VOLUMELIMIT = i32;
pub const KSEVENT_VOLUMELIMIT_CHANGED: KSEVENT_VOLUMELIMIT = 0;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KSE_NODE {
    pub Event: KSEVENT,
    pub NodeId: u32,
    pub Reserved: u32,
}
impl Default for KSE_NODE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KSE_PIN {
    pub Event: KSEVENT,
    pub PinId: u32,
    pub Reserved: u32,
}
impl Default for KSE_PIN {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const KSFILTER_NODE: u32 = 4294967295;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSFRAMETIME {
    pub Duration: i64,
    pub FrameFlags: u32,
    pub Reserved: u32,
}
pub const KSFRAMETIME_VARIABLESIZE: i32 = 1;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KSIDENTIFIER {
    pub Anonymous: KSIDENTIFIER_0,
}
impl Default for KSIDENTIFIER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union KSIDENTIFIER_0 {
    pub Anonymous: KSIDENTIFIER_0_0,
    pub Alignment: i64,
}
impl Default for KSIDENTIFIER_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSIDENTIFIER_0_0 {
    pub Set: windows_core::GUID,
    pub Id: u32,
    pub Flags: u32,
}
pub const KSINSTANCE_INDETERMINATE: u32 = 4294967295;
pub const KSINTERFACESETID_FileIo: windows_core::GUID = windows_core::GUID::from_u128(0x8c6f932c_e771_11d0_b8ff_00a0c9223196);
pub const KSINTERFACESETID_Standard: windows_core::GUID = windows_core::GUID::from_u128(0x1a8766a0_62ce_11cf_a5d6_28db04c10000);
pub type KSINTERFACE_FILEIO = i32;
pub const KSINTERFACE_FILEIO_STREAMING: KSINTERFACE_FILEIO = 0;
pub type KSINTERFACE_STANDARD = i32;
pub const KSINTERFACE_STANDARD_CONTROL: KSINTERFACE_STANDARD = 2;
pub const KSINTERFACE_STANDARD_LOOPED_STREAMING: KSINTERFACE_STANDARD = 1;
pub const KSINTERFACE_STANDARD_STREAMING: KSINTERFACE_STANDARD = 0;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSINTERVAL {
    pub TimeBase: i64,
    pub Interval: i64,
}
pub const KSMEDIUMSETID_Standard: windows_core::GUID = windows_core::GUID::from_u128(0x4747b320_62ce_11cf_a5d6_28db04c10000);
pub const KSMEDIUM_STANDARD_DEVIO: i32 = 0;
pub const KSMEDIUM_TYPE_ANYINSTANCE: i32 = 0;
pub const KSMEMORY_TYPE_DEVICE_UNKNOWN: windows_core::GUID = windows_core::GUID::from_u128(0x091bb639_603f_11d1_b067_00a0c9062802);
pub const KSMEMORY_TYPE_KERNEL_NONPAGED: windows_core::GUID = windows_core::GUID::from_u128(0x4a6d5fc4_7895_11d1_b069_00a0c9062802);
pub const KSMEMORY_TYPE_KERNEL_PAGED: windows_core::GUID = windows_core::GUID::from_u128(0xd833f8f8_7894_11d1_b069_00a0c9062802);
pub const KSMEMORY_TYPE_SYSTEM: windows_core::GUID = windows_core::GUID::from_u128(0x091bb638_603f_11d1_b067_00a0c9062802);
pub const KSMEMORY_TYPE_USER: windows_core::GUID = windows_core::GUID::from_u128(0x8cb0fc28_7893_11d1_b069_00a0c9062802);
pub type KSMETHOD = KSIDENTIFIER;
pub const KSMETHODSETID_StreamAllocator: windows_core::GUID = windows_core::GUID::from_u128(0xcf6e4341_ec87_11cf_a130_0020afd156e4);
pub const KSMETHODSETID_StreamIo: windows_core::GUID = windows_core::GUID::from_u128(0x65d003ca_1523_11d2_b27a_00a0c9223196);
pub type KSMETHOD_STREAMALLOCATOR = i32;
pub const KSMETHOD_STREAMALLOCATOR_ALLOC: KSMETHOD_STREAMALLOCATOR = 0;
pub const KSMETHOD_STREAMALLOCATOR_FREE: KSMETHOD_STREAMALLOCATOR = 1;
pub type KSMETHOD_STREAMIO = i32;
pub const KSMETHOD_STREAMIO_READ: KSMETHOD_STREAMIO = 0;
pub const KSMETHOD_STREAMIO_WRITE: KSMETHOD_STREAMIO = 1;
pub const KSMETHOD_TYPE_BASICSUPPORT: i32 = 512;
pub const KSMETHOD_TYPE_MODIFY: i32 = 3;
pub const KSMETHOD_TYPE_NONE: i32 = 0;
pub const KSMETHOD_TYPE_READ: i32 = 1;
pub const KSMETHOD_TYPE_SEND: i32 = 1;
pub const KSMETHOD_TYPE_SETSUPPORT: i32 = 256;
pub const KSMETHOD_TYPE_SOURCE: i32 = 4;
pub const KSMETHOD_TYPE_TOPOLOGY: i32 = 268435456;
pub const KSMETHOD_TYPE_WRITE: i32 = 2;
pub const KSMFT_CATEGORY_AUDIO_DECODER: windows_core::GUID = windows_core::GUID::from_u128(0x9ea73fb4_ef7a_4559_8d5d_719d8f0426c7);
pub const KSMFT_CATEGORY_AUDIO_EFFECT: windows_core::GUID = windows_core::GUID::from_u128(0x11064c48_3648_4ed0_932e_05ce8ac811b7);
pub const KSMFT_CATEGORY_AUDIO_ENCODER: windows_core::GUID = windows_core::GUID::from_u128(0x91c64bd0_f91e_4d8c_9276_db248279d975);
pub const KSMFT_CATEGORY_DEMULTIPLEXER: windows_core::GUID = windows_core::GUID::from_u128(0xa8700a7a_939b_44c5_99d7_76226b23b3f1);
pub const KSMFT_CATEGORY_MULTIPLEXER: windows_core::GUID = windows_core::GUID::from_u128(0x059c561e_05ae_4b61_b69d_55b61ee54a7b);
pub const KSMFT_CATEGORY_OTHER: windows_core::GUID = windows_core::GUID::from_u128(0x90175d57_b7ea_4901_aeb3_933a8747756f);
pub const KSMFT_CATEGORY_VIDEO_DECODER: windows_core::GUID = windows_core::GUID::from_u128(0xd6c02d4b_6833_45b4_971a_05a4b04bab91);
pub const KSMFT_CATEGORY_VIDEO_EFFECT: windows_core::GUID = windows_core::GUID::from_u128(0x12e17c21_532c_4a6e_8a1c_40825a736397);
pub const KSMFT_CATEGORY_VIDEO_ENCODER: windows_core::GUID = windows_core::GUID::from_u128(0xf79eac7d_e545_4387_bdee_d647d7bde42a);
pub const KSMFT_CATEGORY_VIDEO_PROCESSOR: windows_core::GUID = windows_core::GUID::from_u128(0x302ea3fc_aa5f_47f9_9f7a_c2188bb16302);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSMULTIPLE_ITEM {
    pub Size: u32,
    pub Count: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KSM_NODE {
    pub Method: KSMETHOD,
    pub NodeId: u32,
    pub Reserved: u32,
}
impl Default for KSM_NODE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const KSNAME_Allocator: windows_core::GUID = windows_core::GUID::from_u128(0x642f5d00_4791_11d0_a5d6_28db04c10000);
pub const KSNAME_Clock: windows_core::GUID = windows_core::GUID::from_u128(0x53172480_4791_11d0_a5d6_28db04c10000);
pub const KSNAME_Filter: windows_core::GUID = windows_core::GUID::from_u128(0x9b365890_165f_11d0_a195_0020afd156e4);
pub const KSNAME_Pin: windows_core::GUID = windows_core::GUID::from_u128(0x146f1a80_4791_11d0_a5d6_28db04c10000);
pub const KSNAME_TopologyNode: windows_core::GUID = windows_core::GUID::from_u128(0x0621061a_ee75_11d0_b915_00a0c9223196);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSNODE_CREATE {
    pub CreateFlags: u32,
    pub Node: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSPIN_CINSTANCES {
    pub PossibleCount: u32,
    pub CurrentCount: u32,
}
pub type KSPIN_COMMUNICATION = i32;
pub const KSPIN_COMMUNICATION_BOTH: KSPIN_COMMUNICATION = 3;
pub const KSPIN_COMMUNICATION_BRIDGE: KSPIN_COMMUNICATION = 4;
pub const KSPIN_COMMUNICATION_NONE: KSPIN_COMMUNICATION = 0;
pub const KSPIN_COMMUNICATION_SINK: KSPIN_COMMUNICATION = 1;
pub const KSPIN_COMMUNICATION_SOURCE: KSPIN_COMMUNICATION = 2;
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct KSPIN_CONNECT {
    pub Interface: KSPIN_INTERFACE,
    pub Medium: KSPIN_MEDIUM,
    pub PinId: u32,
    pub PinToHandle: super::HANDLE,
    pub Priority: KSPRIORITY,
}
#[cfg(feature = "winnt")]
impl Default for KSPIN_CONNECT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type KSPIN_DATAFLOW = i32;
pub const KSPIN_DATAFLOW_IN: KSPIN_DATAFLOW = 1;
pub const KSPIN_DATAFLOW_OUT: KSPIN_DATAFLOW = 2;
pub type KSPIN_INTERFACE = KSIDENTIFIER;
pub type KSPIN_MDL_CACHING_EVENT = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSPIN_MDL_CACHING_NOTIFICATION {
    pub Event: KSPIN_MDL_CACHING_EVENT,
    pub Buffer: *mut core::ffi::c_void,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSPIN_MDL_CACHING_NOTIFICATION32 {
    pub Event: KSPIN_MDL_CACHING_EVENT,
    pub Buffer: u32,
}
pub const KSPIN_MDL_CACHING_NOTIFY_ADDSAMPLE: KSPIN_MDL_CACHING_EVENT = 3;
pub const KSPIN_MDL_CACHING_NOTIFY_CLEANALL_NOWAIT: KSPIN_MDL_CACHING_EVENT = 2;
pub const KSPIN_MDL_CACHING_NOTIFY_CLEANALL_WAIT: KSPIN_MDL_CACHING_EVENT = 1;
pub const KSPIN_MDL_CACHING_NOTIFY_CLEANUP: KSPIN_MDL_CACHING_EVENT = 0;
pub type KSPIN_MEDIUM = KSIDENTIFIER;
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct KSPIN_PHYSICALCONNECTION {
    pub Size: u32,
    pub Pin: u32,
    pub SymbolicLinkName: [u16; 1],
}
impl Default for KSPIN_PHYSICALCONNECTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type KSPPROPERTY_ALLOCATOR_MDLCACHING = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSPRIORITY {
    pub PriorityClass: u32,
    pub PrioritySubClass: u32,
}
pub const KSPRIORITY_EXCLUSIVE: u32 = 4294967295;
pub const KSPRIORITY_HIGH: u32 = 2147483648;
pub const KSPRIORITY_LOW: i32 = 1;
pub const KSPRIORITY_NORMAL: i32 = 1073741824;
pub type KSPROPERTY = KSIDENTIFIER;
pub const KSPROPERTY_ALLOCATOR_CLEANUP_CACHEDMDLPAGES: KSPPROPERTY_ALLOCATOR_MDLCACHING = 1;
#[repr(C)]
#[derive(Clone, Copy)]
pub union KSPROPERTY_BOUNDS_LONG {
    pub Anonymous: KSPROPERTY_BOUNDS_LONG_0,
    pub Anonymous2: KSPROPERTY_BOUNDS_LONG_1,
}
impl Default for KSPROPERTY_BOUNDS_LONG {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSPROPERTY_BOUNDS_LONG_0 {
    pub SignedMinimum: i32,
    pub SignedMaximum: i32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSPROPERTY_BOUNDS_LONG_1 {
    pub UnsignedMinimum: u32,
    pub UnsignedMaximum: u32,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub union KSPROPERTY_BOUNDS_LONGLONG {
    pub Anonymous: KSPROPERTY_BOUNDS_LONGLONG_0,
    pub Anonymous2: KSPROPERTY_BOUNDS_LONGLONG_1,
}
#[cfg(feature = "winnt")]
impl Default for KSPROPERTY_BOUNDS_LONGLONG {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSPROPERTY_BOUNDS_LONGLONG_0 {
    pub SignedMinimum: i64,
    pub SignedMaximum: i64,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSPROPERTY_BOUNDS_LONGLONG_1 {
    pub UnsignedMinimum: super::DWORDLONG,
    pub UnsignedMaximum: super::DWORDLONG,
}
pub type KSPROPERTY_CLOCK = i32;
pub const KSPROPERTY_CLOCK_CORRELATEDPHYSICALTIME: KSPROPERTY_CLOCK = 3;
pub const KSPROPERTY_CLOCK_CORRELATEDTIME: KSPROPERTY_CLOCK = 2;
pub const KSPROPERTY_CLOCK_PHYSICALTIME: KSPROPERTY_CLOCK = 1;
pub const KSPROPERTY_CLOCK_RESOLUTION: KSPROPERTY_CLOCK = 4;
pub const KSPROPERTY_CLOCK_STATE: KSPROPERTY_CLOCK = 5;
pub const KSPROPERTY_CLOCK_TIME: KSPROPERTY_CLOCK = 0;
pub type KSPROPERTY_CONNECTION = i32;
pub const KSPROPERTY_CONNECTION_ACQUIREORDERING: KSPROPERTY_CONNECTION = 5;
pub const KSPROPERTY_CONNECTION_ALLOCATORFRAMING: KSPROPERTY_CONNECTION = 3;
pub const KSPROPERTY_CONNECTION_ALLOCATORFRAMING_EX: KSPROPERTY_CONNECTION = 6;
pub const KSPROPERTY_CONNECTION_DATAFORMAT: KSPROPERTY_CONNECTION = 2;
pub const KSPROPERTY_CONNECTION_PRIORITY: KSPROPERTY_CONNECTION = 1;
pub const KSPROPERTY_CONNECTION_PROPOSEDATAFORMAT: KSPROPERTY_CONNECTION = 4;
pub const KSPROPERTY_CONNECTION_STARTAT: KSPROPERTY_CONNECTION = 7;
pub const KSPROPERTY_CONNECTION_STATE: KSPROPERTY_CONNECTION = 0;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KSPROPERTY_DESCRIPTION {
    pub AccessFlags: u32,
    pub DescriptionSize: u32,
    pub PropTypeSet: KSIDENTIFIER,
    pub MembersListCount: u32,
    pub Reserved: u32,
}
impl Default for KSPROPERTY_DESCRIPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type KSPROPERTY_GENERAL = i32;
pub const KSPROPERTY_GENERAL_COMPONENTID: KSPROPERTY_GENERAL = 0;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSPROPERTY_MEDIAAVAILABLE {
    pub Earliest: i64,
    pub Latest: i64,
}
pub type KSPROPERTY_MEDIASEEKING = i32;
pub const KSPROPERTY_MEDIASEEKING_AVAILABLE: KSPROPERTY_MEDIASEEKING = 7;
pub const KSPROPERTY_MEDIASEEKING_CAPABILITIES: KSPROPERTY_MEDIASEEKING = 0;
pub const KSPROPERTY_MEDIASEEKING_CONVERTTIMEFORMAT: KSPROPERTY_MEDIASEEKING = 9;
pub const KSPROPERTY_MEDIASEEKING_DURATION: KSPROPERTY_MEDIASEEKING = 6;
pub const KSPROPERTY_MEDIASEEKING_FORMATS: KSPROPERTY_MEDIASEEKING = 1;
pub const KSPROPERTY_MEDIASEEKING_POSITION: KSPROPERTY_MEDIASEEKING = 3;
pub const KSPROPERTY_MEDIASEEKING_POSITIONS: KSPROPERTY_MEDIASEEKING = 5;
pub const KSPROPERTY_MEDIASEEKING_PREROLL: KSPROPERTY_MEDIASEEKING = 8;
pub const KSPROPERTY_MEDIASEEKING_STOPPOSITION: KSPROPERTY_MEDIASEEKING = 4;
pub const KSPROPERTY_MEDIASEEKING_TIMEFORMAT: KSPROPERTY_MEDIASEEKING = 2;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSPROPERTY_MEMBERSHEADER {
    pub MembersFlags: u32,
    pub MembersSize: u32,
    pub MembersCount: u32,
    pub Flags: u32,
}
pub const KSPROPERTY_MEMBER_FLAG_BASICSUPPORT_MULTICHANNEL: i32 = 2;
pub const KSPROPERTY_MEMBER_FLAG_BASICSUPPORT_UNIFORM: i32 = 4;
pub const KSPROPERTY_MEMBER_FLAG_DEFAULT: i32 = 1;
pub const KSPROPERTY_MEMBER_RANGES: i32 = 1;
pub const KSPROPERTY_MEMBER_STEPPEDRANGES: i32 = 2;
pub const KSPROPERTY_MEMBER_VALUES: i32 = 3;
pub const KSPROPERTY_MEMORY_TRANSPORT: i32 = 1;
pub type KSPROPERTY_PIN = i32;
pub const KSPROPERTY_PIN_CATEGORY: KSPROPERTY_PIN = 11;
pub const KSPROPERTY_PIN_CINSTANCES: KSPROPERTY_PIN = 0;
pub const KSPROPERTY_PIN_COMMUNICATION: KSPROPERTY_PIN = 7;
pub const KSPROPERTY_PIN_CONSTRAINEDDATARANGES: KSPROPERTY_PIN = 13;
pub const KSPROPERTY_PIN_CTYPES: KSPROPERTY_PIN = 1;
pub const KSPROPERTY_PIN_DATAFLOW: KSPROPERTY_PIN = 2;
pub const KSPROPERTY_PIN_DATAINTERSECTION: KSPROPERTY_PIN = 4;
pub const KSPROPERTY_PIN_DATARANGES: KSPROPERTY_PIN = 3;
pub const KSPROPERTY_PIN_FLAGS_ATTRIBUTE_RANGE_AWARE: i32 = 1;
pub const KSPROPERTY_PIN_FLAGS_MASK: i32 = 1;
pub const KSPROPERTY_PIN_GLOBALCINSTANCES: KSPROPERTY_PIN = 8;
pub const KSPROPERTY_PIN_INTERFACES: KSPROPERTY_PIN = 5;
pub const KSPROPERTY_PIN_MEDIUMS: KSPROPERTY_PIN = 6;
pub const KSPROPERTY_PIN_MODEDATAFORMATS: KSPROPERTY_PIN = 16;
pub const KSPROPERTY_PIN_NAME: KSPROPERTY_PIN = 12;
pub const KSPROPERTY_PIN_NECESSARYINSTANCES: KSPROPERTY_PIN = 9;
pub const KSPROPERTY_PIN_PHYSICALCONNECTION: KSPROPERTY_PIN = 10;
pub const KSPROPERTY_PIN_PROPOSEDATAFORMAT: KSPROPERTY_PIN = 14;
pub const KSPROPERTY_PIN_PROPOSEDATAFORMAT2: KSPROPERTY_PIN = 15;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSPROPERTY_POSITIONS {
    pub Current: i64,
    pub Stop: i64,
    pub CurrentFlags: KS_SEEKING_FLAGS,
    pub StopFlags: KS_SEEKING_FLAGS,
}
pub type KSPROPERTY_QUALITY = i32;
pub const KSPROPERTY_QUALITY_ERROR: KSPROPERTY_QUALITY = 1;
pub const KSPROPERTY_QUALITY_REPORT: KSPROPERTY_QUALITY = 0;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KSPROPERTY_SERIAL {
    pub PropTypeSet: KSIDENTIFIER,
    pub Id: u32,
    pub PropertyLength: u32,
}
impl Default for KSPROPERTY_SERIAL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct KSPROPERTY_SERIALHDR {
    pub PropertySet: windows_core::GUID,
    pub Count: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KSPROPERTY_STEPPING_LONG {
    pub SteppingDelta: u32,
    pub Reserved: u32,
    pub Bounds: KSPROPERTY_BOUNDS_LONG,
}
impl Default for KSPROPERTY_STEPPING_LONG {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct KSPROPERTY_STEPPING_LONGLONG {
    pub SteppingDelta: super::DWORDLONG,
    pub Bounds: KSPROPERTY_BOUNDS_LONGLONG,
}
#[cfg(feature = "winnt")]
impl Default for KSPROPERTY_STEPPING_LONGLONG {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type KSPROPERTY_STREAM = i32;
pub type KSPROPERTY_STREAMINTERFACE = i32;
pub const KSPROPERTY_STREAMINTERFACE_HEADERSIZE: KSPROPERTY_STREAMINTERFACE = 0;
pub const KSPROPERTY_STREAM_ALLOCATOR: KSPROPERTY_STREAM = 0;
pub const KSPROPERTY_STREAM_DEGRADATION: KSPROPERTY_STREAM = 2;
pub const KSPROPERTY_STREAM_FRAMETIME: KSPROPERTY_STREAM = 7;
pub const KSPROPERTY_STREAM_MASTERCLOCK: KSPROPERTY_STREAM = 3;
pub const KSPROPERTY_STREAM_PIPE_ID: KSPROPERTY_STREAM = 10;
pub const KSPROPERTY_STREAM_PRESENTATIONEXTENT: KSPROPERTY_STREAM = 6;
pub const KSPROPERTY_STREAM_PRESENTATIONTIME: KSPROPERTY_STREAM = 5;
pub const KSPROPERTY_STREAM_QUALITY: KSPROPERTY_STREAM = 1;
pub const KSPROPERTY_STREAM_RATE: KSPROPERTY_STREAM = 9;
pub const KSPROPERTY_STREAM_RATECAPABILITY: KSPROPERTY_STREAM = 8;
pub const KSPROPERTY_STREAM_TIMEFORMAT: KSPROPERTY_STREAM = 4;
pub type KSPROPERTY_TOPOLOGY = i32;
pub const KSPROPERTY_TOPOLOGY_CATEGORIES: KSPROPERTY_TOPOLOGY = 0;
pub const KSPROPERTY_TOPOLOGY_CONNECTIONS: KSPROPERTY_TOPOLOGY = 2;
pub const KSPROPERTY_TOPOLOGY_NAME: KSPROPERTY_TOPOLOGY = 3;
pub const KSPROPERTY_TOPOLOGY_NODES: KSPROPERTY_TOPOLOGY = 1;
pub const KSPROPERTY_TYPE_BASICSUPPORT: i32 = 512;
pub const KSPROPERTY_TYPE_COPYPAYLOAD: u32 = 2147483648;
pub const KSPROPERTY_TYPE_DEFAULTVALUES: i32 = 65536;
pub const KSPROPERTY_TYPE_FSFILTERSCOPE: i32 = 1073741824;
pub const KSPROPERTY_TYPE_GET: i32 = 1;
pub const KSPROPERTY_TYPE_GETPAYLOADSIZE: i32 = 4;
pub const KSPROPERTY_TYPE_HIGHPRIORITY: i32 = 134217728;
pub const KSPROPERTY_TYPE_RELATIONS: i32 = 1024;
pub const KSPROPERTY_TYPE_SERIALIZERAW: i32 = 8192;
pub const KSPROPERTY_TYPE_SERIALIZESET: i32 = 2048;
pub const KSPROPERTY_TYPE_SERIALIZESIZE: i32 = 32768;
pub const KSPROPERTY_TYPE_SET: i32 = 2;
pub const KSPROPERTY_TYPE_SETSUPPORT: i32 = 256;
pub const KSPROPERTY_TYPE_TOPOLOGY: i32 = 268435456;
pub const KSPROPERTY_TYPE_UNSERIALIZERAW: i32 = 16384;
pub const KSPROPERTY_TYPE_UNSERIALIZESET: i32 = 4096;
pub const KSPROPSETID_Clock: windows_core::GUID = windows_core::GUID::from_u128(0xdf12a4c0_ac17_11cf_a5d6_28db04c10000);
pub const KSPROPSETID_Connection: windows_core::GUID = windows_core::GUID::from_u128(0x1d58c920_ac9b_11cf_a5d6_28db04c10000);
pub const KSPROPSETID_General: windows_core::GUID = windows_core::GUID::from_u128(0x1464eda5_6a8f_11d1_9aa7_00a0c9223196);
pub const KSPROPSETID_MediaSeeking: windows_core::GUID = windows_core::GUID::from_u128(0xee904f0c_d09b_11d0_abe9_00a0c9223196);
pub const KSPROPSETID_MemoryTransport: windows_core::GUID = windows_core::GUID::from_u128(0x0a3d1c5d_5243_4819_9ed0_aee8044cee2b);
pub const KSPROPSETID_Pin: windows_core::GUID = windows_core::GUID::from_u128(0x8c134960_51ad_11cf_878a_94f801c10000);
pub const KSPROPSETID_PinMDLCacheClearProp: windows_core::GUID = windows_core::GUID::from_u128(0xbd718a7b_97fc_40c7_88ce_d3ff06f55b16);
pub const KSPROPSETID_Quality: windows_core::GUID = windows_core::GUID::from_u128(0xd16ad380_ac1a_11cf_a5d6_28db04c10000);
pub const KSPROPSETID_Stream: windows_core::GUID = windows_core::GUID::from_u128(0x65aaba60_98ae_11cf_a10d_0020afd156e4);
pub const KSPROPSETID_StreamAllocator: windows_core::GUID = windows_core::GUID::from_u128(0xcf6e4342_ec87_11cf_a130_0020afd156e4);
pub const KSPROPSETID_StreamInterface: windows_core::GUID = windows_core::GUID::from_u128(0x1fdd8ee1_9cd3_11d0_82aa_0000f822fe8a);
pub const KSPROPSETID_Topology: windows_core::GUID = windows_core::GUID::from_u128(0x720d4ac0_7533_11d0_a5d6_28db04c10000);
pub const KSPROPTYPESETID_General: windows_core::GUID = windows_core::GUID::from_u128(0x97e99ba0_bdea_11cf_a5d6_28db04c10000);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KSP_NODE {
    pub Property: KSPROPERTY,
    pub NodeId: u32,
    pub Reserved: u32,
}
impl Default for KSP_NODE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KSP_PIN {
    pub Property: KSPROPERTY,
    pub PinId: u32,
    pub Anonymous: KSP_PIN_0,
}
impl Default for KSP_PIN {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union KSP_PIN_0 {
    pub Reserved: u32,
    pub Flags: u32,
}
impl Default for KSP_PIN_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KSP_TIMEFORMAT {
    pub Property: KSPROPERTY,
    pub SourceFormat: windows_core::GUID,
    pub TargetFormat: windows_core::GUID,
    pub Time: i64,
}
impl Default for KSP_TIMEFORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSQUALITY {
    pub Context: *mut core::ffi::c_void,
    pub Proportion: u32,
    pub DeltaTime: i64,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSQUALITY_MANAGER {
    pub QualityManager: super::HANDLE,
    pub Context: *mut core::ffi::c_void,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct KSQUERYBUFFER {
    pub Event: KSEVENT,
    pub EventData: PKSEVENTDATA,
    pub Reserved: *mut core::ffi::c_void,
}
#[cfg(feature = "winnt")]
impl Default for KSQUERYBUFFER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KSRATE {
    pub PresentationStart: i64,
    pub Duration: i64,
    pub Interface: KSPIN_INTERFACE,
    pub Rate: i32,
    pub Flags: u32,
}
impl Default for KSRATE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KSRATE_CAPABILITY {
    pub Property: KSPROPERTY,
    pub Rate: KSRATE,
}
impl Default for KSRATE_CAPABILITY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const KSRATE_NOPRESENTATIONDURATION: i32 = 2;
pub const KSRATE_NOPRESENTATIONSTART: i32 = 1;
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct KSRELATIVEEVENT {
    pub Size: u32,
    pub Flags: u32,
    pub Anonymous: KSRELATIVEEVENT_0,
    pub Reserved: *mut core::ffi::c_void,
    pub Event: KSEVENT,
    pub EventData: KSEVENTDATA,
}
#[cfg(feature = "winnt")]
impl Default for KSRELATIVEEVENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub union KSRELATIVEEVENT_0 {
    pub ObjectHandle: super::HANDLE,
    pub ObjectPointer: *mut core::ffi::c_void,
}
#[cfg(feature = "winnt")]
impl Default for KSRELATIVEEVENT_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const KSRELATIVEEVENT_FLAG_HANDLE: i32 = 1;
pub const KSRELATIVEEVENT_FLAG_POINTER: i32 = 2;
pub type KSRESET = i32;
pub const KSRESET_BEGIN: KSRESET = 0;
pub const KSRESET_END: KSRESET = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSRESOLUTION {
    pub Granularity: i64,
    pub Error: i64,
}
pub type KSSTATE = i32;
pub const KSSTATE_ACQUIRE: KSSTATE = 1;
pub const KSSTATE_PAUSE: KSSTATE = 2;
pub const KSSTATE_RUN: KSSTATE = 3;
pub const KSSTATE_STOP: KSSTATE = 0;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KSSTREAMALLOCATOR_STATUS {
    pub Framing: KSALLOCATOR_FRAMING,
    pub AllocatedFrames: u32,
    pub Reserved: u32,
}
impl Default for KSSTREAMALLOCATOR_STATUS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KSSTREAMALLOCATOR_STATUS_EX {
    pub Framing: KSALLOCATOR_FRAMING_EX,
    pub AllocatedFrames: u32,
    pub Reserved: u32,
}
impl Default for KSSTREAMALLOCATOR_STATUS_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSSTREAM_HEADER {
    pub Size: u32,
    pub TypeSpecificFlags: u32,
    pub PresentationTime: KSTIME,
    pub Duration: i64,
    pub FrameExtent: u32,
    pub DataUsed: u32,
    pub Data: *mut core::ffi::c_void,
    pub OptionsFlags: u32,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSSTREAM_HEADER {
    pub Size: u32,
    pub TypeSpecificFlags: u32,
    pub PresentationTime: KSTIME,
    pub Duration: i64,
    pub FrameExtent: u32,
    pub DataUsed: u32,
    pub Data: *mut core::ffi::c_void,
    pub OptionsFlags: u32,
    pub Reserved: u32,
}
pub const KSSTREAM_HEADER_OPTIONSF_BUFFEREDTRANSFER: i32 = 1024;
pub const KSSTREAM_HEADER_OPTIONSF_DATADISCONTINUITY: i32 = 4;
pub const KSSTREAM_HEADER_OPTIONSF_DURATIONVALID: i32 = 256;
pub const KSSTREAM_HEADER_OPTIONSF_ENDOFPHOTOSEQUENCE: i32 = 8192;
pub const KSSTREAM_HEADER_OPTIONSF_ENDOFSTREAM: i32 = 512;
pub const KSSTREAM_HEADER_OPTIONSF_FLUSHONPAUSE: i32 = 128;
pub const KSSTREAM_HEADER_OPTIONSF_FRAMEINFO: i32 = 16384;
pub const KSSTREAM_HEADER_OPTIONSF_LOOPEDDATA: u32 = 2147483648;
pub const KSSTREAM_HEADER_OPTIONSF_METADATA: i32 = 4096;
pub const KSSTREAM_HEADER_OPTIONSF_PERSIST_SAMPLE: i32 = 32768;
pub const KSSTREAM_HEADER_OPTIONSF_PREROLL: i32 = 2;
pub const KSSTREAM_HEADER_OPTIONSF_SAMPLE_PERSISTED: i32 = 65536;
pub const KSSTREAM_HEADER_OPTIONSF_SECUREBUFFERTRANSFER: i32 = 262144;
pub const KSSTREAM_HEADER_OPTIONSF_SPLICEPOINT: i32 = 1;
pub const KSSTREAM_HEADER_OPTIONSF_TIMEDISCONTINUITY: i32 = 64;
pub const KSSTREAM_HEADER_OPTIONSF_TIMEVALID: i32 = 16;
pub const KSSTREAM_HEADER_OPTIONSF_TYPECHANGED: i32 = 8;
pub const KSSTREAM_HEADER_OPTIONSF_VRAM_DATA_TRANSFER: i32 = 2048;
pub const KSSTREAM_HEADER_TRACK_COMPLETION_NUMBERS: i32 = 131072;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSSTREAM_METADATA_INFO {
    pub BufferSize: u32,
    pub UsedSize: u32,
    pub Data: *mut core::ffi::c_void,
    pub SystemVa: *mut core::ffi::c_void,
    pub Flags: u32,
    pub Reserved: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KSSTREAM_UVC_METADATA {
    pub StartOfFrameTimestamp: KSSTREAM_UVC_METADATATYPE_TIMESTAMP,
    pub EndOfFrameTimestamp: KSSTREAM_UVC_METADATATYPE_TIMESTAMP,
}
impl Default for KSSTREAM_UVC_METADATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KSSTREAM_UVC_METADATATYPE_TIMESTAMP {
    pub PresentationTimeStamp: u32,
    pub SourceClockReference: u32,
    pub Anonymous: KSSTREAM_UVC_METADATATYPE_TIMESTAMP_0,
    pub Reserved0: u16,
    pub Reserved1: u32,
}
impl Default for KSSTREAM_UVC_METADATATYPE_TIMESTAMP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union KSSTREAM_UVC_METADATATYPE_TIMESTAMP_0 {
    pub Anonymous: KSSTREAM_UVC_METADATATYPE_TIMESTAMP_0_0,
    pub SCRToken: u16,
}
impl Default for KSSTREAM_UVC_METADATATYPE_TIMESTAMP_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSSTREAM_UVC_METADATATYPE_TIMESTAMP_0_0 {
    pub _bitfield: u16,
}
impl KSSTREAM_UVC_METADATATYPE_TIMESTAMP_0_0 {
    pub fn Counter(&self) -> u16 {
        (self._bitfield << 5) >> 5
    }
    pub fn set_Counter(&mut self, value: u16) {
        self._bitfield = (self._bitfield & !2047) | (value & 2047);
    }
    pub fn Reserved(&self) -> u16 {
        self._bitfield >> 11
    }
    pub fn set_Reserved(&mut self, value: u16) {
        self._bitfield = (self._bitfield & !(31 << 11)) | ((value & 31) << 11);
    }
}
pub const KSSTREAM_UVC_SECURE_ATTRIBUTE_SIZE: i32 = 8192;
pub const KSSTRING_Allocator: windows_core::PCWSTR = windows_core::w!("{642F5D00-4791-11D0-A5D6-28DB04C10000}");
pub const KSSTRING_AllocatorEx: windows_core::PCWSTR = windows_core::w!("{091BB63B-603F-11D1-B067-00A0C9062802}");
pub const KSSTRING_Clock: windows_core::PCWSTR = windows_core::w!("{53172480-4791-11D0-A5D6-28DB04C10000}");
pub const KSSTRING_Filter: windows_core::PCWSTR = windows_core::w!("{9B365890-165F-11D0-A195-0020AFD156E4}");
pub const KSSTRING_Pin: windows_core::PCWSTR = windows_core::w!("{146F1A80-4791-11D0-A5D6-28DB04C10000}");
pub const KSSTRING_TopologyNode: windows_core::PCWSTR = windows_core::w!("{0621061A-EE75-11D0-B915-00A0C9223196}");
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSTIME {
    pub Time: i64,
    pub Numerator: u32,
    pub Denominator: u32,
}
pub const KSTIME_FORMAT_BYTE: windows_core::GUID = windows_core::GUID::from_u128(0x7b785571_8c82_11cf_bc0c_00aa00ac74f6);
pub const KSTIME_FORMAT_FIELD: windows_core::GUID = windows_core::GUID::from_u128(0x7b785573_8c82_11cf_bc0c_00aa00ac74f6);
pub const KSTIME_FORMAT_FRAME: windows_core::GUID = windows_core::GUID::from_u128(0x7b785570_8c82_11cf_bc0c_00aa00ac74f6);
pub const KSTIME_FORMAT_MEDIA_TIME: windows_core::GUID = windows_core::GUID::from_u128(0x7b785574_8c82_11cf_bc0c_00aa00ac74f6);
pub const KSTIME_FORMAT_SAMPLE: windows_core::GUID = windows_core::GUID::from_u128(0x7b785572_8c82_11cf_bc0c_00aa00ac74f6);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSTOPOLOGY {
    pub CategoriesCount: u32,
    pub Categories: *const windows_core::GUID,
    pub TopologyNodesCount: u32,
    pub TopologyNodes: *const windows_core::GUID,
    pub TopologyConnectionsCount: u32,
    pub TopologyConnections: *const KSTOPOLOGY_CONNECTION,
    pub TopologyNodesNames: *const windows_core::GUID,
    pub Reserved: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KSTOPOLOGY_CONNECTION {
    pub FromNode: u32,
    pub FromNodePin: u32,
    pub ToNode: u32,
    pub ToNodePin: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KS_COMPRESSION {
    pub RatioNumerator: u32,
    pub RatioDenominator: u32,
    pub RatioConstantMargin: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KS_FRAMING_ITEM {
    pub MemoryType: windows_core::GUID,
    pub BusType: windows_core::GUID,
    pub MemoryFlags: u32,
    pub BusFlags: u32,
    pub Flags: u32,
    pub Frames: u32,
    pub Anonymous: KS_FRAMING_ITEM_0,
    pub MemoryTypeWeight: u32,
    pub PhysicalRange: KS_FRAMING_RANGE,
    pub FramingRange: KS_FRAMING_RANGE_WEIGHTED,
}
impl Default for KS_FRAMING_ITEM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union KS_FRAMING_ITEM_0 {
    pub FileAlignment: u32,
    pub FramePitch: i32,
}
impl Default for KS_FRAMING_ITEM_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KS_FRAMING_RANGE {
    pub MinFrameSize: u32,
    pub MaxFrameSize: u32,
    pub Stepping: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KS_FRAMING_RANGE_WEIGHTED {
    pub Range: KS_FRAMING_RANGE,
    pub InPlaceWeight: u32,
    pub NotInPlaceWeight: u32,
}
pub const KS_SEEKING_AbsolutePositioning: KS_SEEKING_FLAGS = 1;
pub type KS_SEEKING_CAPABILITIES = i32;
pub const KS_SEEKING_CanGetCurrentPos: KS_SEEKING_CAPABILITIES = 8;
pub const KS_SEEKING_CanGetDuration: KS_SEEKING_CAPABILITIES = 32;
pub const KS_SEEKING_CanGetStopPos: KS_SEEKING_CAPABILITIES = 16;
pub const KS_SEEKING_CanPlayBackwards: KS_SEEKING_CAPABILITIES = 64;
pub const KS_SEEKING_CanSeekAbsolute: KS_SEEKING_CAPABILITIES = 1;
pub const KS_SEEKING_CanSeekBackwards: KS_SEEKING_CAPABILITIES = 4;
pub const KS_SEEKING_CanSeekForwards: KS_SEEKING_CAPABILITIES = 2;
pub type KS_SEEKING_FLAGS = i32;
pub const KS_SEEKING_IncrementalPositioning: KS_SEEKING_FLAGS = 3;
pub const KS_SEEKING_NoPositioning: KS_SEEKING_FLAGS = 0;
pub const KS_SEEKING_PositioningBitsMask: KS_SEEKING_FLAGS = 3;
pub const KS_SEEKING_RelativePositioning: KS_SEEKING_FLAGS = 2;
pub const KS_SEEKING_ReturnTime: KS_SEEKING_FLAGS = 8;
pub const KS_SEEKING_SeekToKeyFrame: KS_SEEKING_FLAGS = 4;
#[repr(C)]
#[derive(Clone, Copy)]
pub union MF_MDL_SHARED_PAYLOAD_KEY {
    pub combined: MF_MDL_SHARED_PAYLOAD_KEY_0,
    pub GMDLHandle: windows_core::GUID,
}
impl Default for MF_MDL_SHARED_PAYLOAD_KEY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MF_MDL_SHARED_PAYLOAD_KEY_0 {
    pub pHandle: u32,
    pub fHandle: u32,
    pub uPayload: u64,
}
pub const NANOSECONDS: i32 = 10000000;
pub type PKSALLOCATOR_FRAMING = *mut KSALLOCATOR_FRAMING;
pub type PKSALLOCATOR_FRAMING_EX = *mut KSALLOCATOR_FRAMING_EX;
pub type PKSATTRIBUTE = *mut KSATTRIBUTE;
pub type PKSCLOCK_CREATE = *mut KSCLOCK_CREATE;
pub type PKSCOMPONENTID = *mut KSCOMPONENTID;
pub type PKSCORRELATED_TIME = *mut KSCORRELATED_TIME;
pub type PKSDATAFORMAT = *mut KSDATARANGE;
pub type PKSDATARANGE = *mut KSDATARANGE;
pub type PKSDEGRADE = *mut KSIDENTIFIER;
pub type PKSERROR = *mut KSERROR;
pub type PKSEVENT = *mut KSIDENTIFIER;
#[cfg(feature = "winnt")]
pub type PKSEVENTDATA = *mut KSEVENTDATA;
#[cfg(feature = "winnt")]
pub type PKSEVENT_TIME_INTERVAL = *mut KSEVENT_TIME_INTERVAL;
#[cfg(feature = "winnt")]
pub type PKSEVENT_TIME_MARK = *mut KSEVENT_TIME_MARK;
pub type PKSE_NODE = *mut KSE_NODE;
pub type PKSE_PIN = *mut KSE_PIN;
pub type PKSFRAMETIME = *mut KSFRAMETIME;
pub type PKSIDENTIFIER = *mut KSIDENTIFIER;
pub type PKSINTERVAL = *mut KSINTERVAL;
pub type PKSMETHOD = *mut KSIDENTIFIER;
pub type PKSMULTIPLE_ITEM = *mut KSMULTIPLE_ITEM;
pub type PKSM_NODE = *mut KSM_NODE;
pub type PKSNODE_CREATE = *mut KSNODE_CREATE;
pub type PKSPIN_CINSTANCES = *mut KSPIN_CINSTANCES;
pub type PKSPIN_COMMUNICATION = *mut KSPIN_COMMUNICATION;
#[cfg(feature = "winnt")]
pub type PKSPIN_CONNECT = *mut KSPIN_CONNECT;
pub type PKSPIN_DATAFLOW = *mut KSPIN_DATAFLOW;
pub type PKSPIN_INTERFACE = *mut KSIDENTIFIER;
pub type PKSPIN_MDL_CACHING_NOTIFICATION = *mut KSPIN_MDL_CACHING_NOTIFICATION;
pub type PKSPIN_MDL_CACHING_NOTIFICATION32 = *mut KSPIN_MDL_CACHING_NOTIFICATION32;
pub type PKSPIN_MEDIUM = *mut KSIDENTIFIER;
pub type PKSPIN_PHYSICALCONNECTION = *mut KSPIN_PHYSICALCONNECTION;
pub type PKSPRIORITY = *mut KSPRIORITY;
pub type PKSPROPERTY = *mut KSIDENTIFIER;
pub type PKSPROPERTY_BOUNDS_LONG = *mut KSPROPERTY_BOUNDS_LONG;
#[cfg(feature = "winnt")]
pub type PKSPROPERTY_BOUNDS_LONGLONG = *mut KSPROPERTY_BOUNDS_LONGLONG;
pub type PKSPROPERTY_DESCRIPTION = *mut KSPROPERTY_DESCRIPTION;
pub type PKSPROPERTY_MEDIAAVAILABLE = *mut KSPROPERTY_MEDIAAVAILABLE;
pub type PKSPROPERTY_MEMBERSHEADER = *mut KSPROPERTY_MEMBERSHEADER;
pub type PKSPROPERTY_POSITIONS = *mut KSPROPERTY_POSITIONS;
pub type PKSPROPERTY_SERIAL = *mut KSPROPERTY_SERIAL;
pub type PKSPROPERTY_SERIALHDR = *mut KSPROPERTY_SERIALHDR;
pub type PKSPROPERTY_STEPPING_LONG = *mut KSPROPERTY_STEPPING_LONG;
#[cfg(feature = "winnt")]
pub type PKSPROPERTY_STEPPING_LONGLONG = *mut KSPROPERTY_STEPPING_LONGLONG;
pub type PKSP_NODE = *mut KSP_NODE;
pub type PKSP_PIN = *mut KSP_PIN;
pub type PKSP_TIMEFORMAT = *mut KSP_TIMEFORMAT;
pub type PKSQUALITY = *mut KSQUALITY;
#[cfg(feature = "winnt")]
pub type PKSQUALITY_MANAGER = *mut KSQUALITY_MANAGER;
#[cfg(feature = "winnt")]
pub type PKSQUERYBUFFER = *mut KSQUERYBUFFER;
pub type PKSRATE = *mut KSRATE;
pub type PKSRATE_CAPABILITY = *mut KSRATE_CAPABILITY;
pub type PKSRESOLUTION = *mut KSRESOLUTION;
pub type PKSSTATE = *mut KSSTATE;
pub type PKSSTREAMALLOCATOR_STATUS = *mut KSSTREAMALLOCATOR_STATUS;
pub type PKSSTREAMALLOCATOR_STATUS_EX = *mut KSSTREAMALLOCATOR_STATUS_EX;
pub type PKSSTREAM_HEADER = *mut KSSTREAM_HEADER;
pub type PKSSTREAM_METADATA_INFO = *mut KSSTREAM_METADATA_INFO;
pub type PKSSTREAM_UVC_METADATA = *mut KSSTREAM_UVC_METADATA;
pub type PKSSTREAM_UVC_METADATATYPE_TIMESTAMP = *mut KSSTREAM_UVC_METADATATYPE_TIMESTAMP;
pub type PKSTIME = *mut KSTIME;
pub type PKSTOPOLOGY = *mut KSTOPOLOGY;
pub type PKSTOPOLOGY_CONNECTION = *mut KSTOPOLOGY_CONNECTION;
pub type PKSWORKER = *mut core::ffi::c_void;
pub type PKS_COMPRESSION = *mut KS_COMPRESSION;
pub type PKS_FRAMING_ITEM = *mut KS_FRAMING_ITEM;
pub type PKS_FRAMING_RANGE = *mut KS_FRAMING_RANGE;
pub type PKS_FRAMING_RANGE_WEIGHTED = *mut KS_FRAMING_RANGE_WEIGHTED;
pub type PMF_MDL_SHARED_PAYLOAD_KEY = *mut MF_MDL_SHARED_PAYLOAD_KEY;
