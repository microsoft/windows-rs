#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct OFFSETINSTANCEDATAANDLENGTH {
    pub OffsetInstanceData: u32,
    pub LengthInstanceData: u32,
}
pub type POFFSETINSTANCEDATAANDLENGTH = *mut OFFSETINSTANCEDATAANDLENGTH;
pub type PWMIREGGUID = PWMIREGGUIDW;
pub type PWMIREGGUIDW = *mut WMIREGGUIDW;
pub type PWMIREGINFO = PWMIREGINFOW;
pub type PWMIREGINFOW = *mut WMIREGINFOW;
#[cfg(feature = "winnt")]
pub type PWNODE_ALL_DATA = *mut WNODE_ALL_DATA;
#[cfg(feature = "winnt")]
pub type PWNODE_EVENT_ITEM = *mut WNODE_EVENT_ITEM;
#[cfg(feature = "winnt")]
pub type PWNODE_EVENT_REFERENCE = *mut WNODE_EVENT_REFERENCE;
#[cfg(feature = "winnt")]
pub type PWNODE_HEADER = *mut WNODE_HEADER;
#[cfg(feature = "winnt")]
pub type PWNODE_METHOD_ITEM = *mut WNODE_METHOD_ITEM;
#[cfg(feature = "winnt")]
pub type PWNODE_SINGLE_INSTANCE = *mut WNODE_SINGLE_INSTANCE;
#[cfg(feature = "winnt")]
pub type PWNODE_SINGLE_ITEM = *mut WNODE_SINGLE_ITEM;
#[cfg(feature = "winnt")]
pub type PWNODE_TOO_SMALL = *mut WNODE_TOO_SMALL;
pub const TRACELOG_ACCESS_KERNEL_LOGGER: i32 = 256;
pub const TRACELOG_ACCESS_REALTIME: i32 = 1024;
pub const TRACELOG_CREATE_INPROC: i32 = 512;
pub const TRACELOG_CREATE_ONDISK: i32 = 64;
pub const TRACELOG_CREATE_REALTIME: i32 = 32;
pub const TRACELOG_GUID_ENABLE: i32 = 128;
pub const TRACELOG_JOIN_GROUP: i32 = 4096;
pub const TRACELOG_LOG_EVENT: i32 = 512;
pub const TRACELOG_REGISTER_GUIDS: i32 = 2048;
pub type WMIDPREQUESTCODE = i32;
pub const WMIGUID_ALL_ACCESS: i32 = 1187839;
pub const WMIGUID_ALL_ACCESS_RS1: i32 = 1187839;
pub const WMIGUID_ALL_ACCESS_WIN2K: i32 = 133119;
pub const WMIGUID_ALL_ACCESS_WINXP: i32 = 1183743;
pub const WMIGUID_EXECUTE: i32 = 16;
pub const WMIGUID_NOTIFICATION: i32 = 4;
pub const WMIGUID_QUERY: i32 = 1;
pub const WMIGUID_READ_DESCRIPTION: i32 = 8;
pub const WMIGUID_SET: i32 = 2;
pub type WMIREGGUID = WMIREGGUIDW;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WMIREGGUIDW {
    pub Guid: windows_sys::core::GUID,
    pub Flags: u32,
    pub InstanceCount: u32,
    pub Anonymous: WMIREGGUIDW_0,
}
impl Default for WMIREGGUIDW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union WMIREGGUIDW_0 {
    pub InstanceNameList: u32,
    pub BaseNameOffset: u32,
    pub Pdo: usize,
    pub InstanceInfo: usize,
}
impl Default for WMIREGGUIDW_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type WMIREGINFO = WMIREGINFOW;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WMIREGINFOW {
    pub BufferSize: u32,
    pub NextWmiRegInfo: u32,
    pub RegistryPath: u32,
    pub MofResourceName: u32,
    pub GuidCount: u32,
    pub WmiRegGuid: [WMIREGGUIDW; 0],
}
impl Default for WMIREGINFOW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WMIREG_FLAG_EVENT_ONLY_GUID: i32 = 64;
pub const WMIREG_FLAG_EXPENSIVE: i32 = 1;
pub const WMIREG_FLAG_INSTANCE_BASENAME: i32 = 8;
pub const WMIREG_FLAG_INSTANCE_LIST: i32 = 4;
pub const WMIREG_FLAG_INSTANCE_PDO: i32 = 32;
pub const WMIREG_FLAG_REMOVE_GUID: i32 = 65536;
pub const WMIREG_FLAG_RESERVED1: i32 = 131072;
pub const WMIREG_FLAG_RESERVED2: i32 = 262144;
pub const WMIREG_FLAG_TRACED_GUID: i32 = 524288;
pub const WMIREG_FLAG_TRACE_CONTROL_GUID: i32 = 4096;
pub const WMI_CAPTURE_STATE: WMIDPREQUESTCODE = 10;
pub const WMI_DISABLE_COLLECTION: WMIDPREQUESTCODE = 7;
pub const WMI_DISABLE_EVENTS: WMIDPREQUESTCODE = 5;
pub const WMI_ENABLE_COLLECTION: WMIDPREQUESTCODE = 6;
pub const WMI_ENABLE_EVENTS: WMIDPREQUESTCODE = 4;
pub const WMI_EXECUTE_METHOD: WMIDPREQUESTCODE = 9;
pub const WMI_GET_ALL_DATA: WMIDPREQUESTCODE = 0;
pub const WMI_GET_SINGLE_INSTANCE: WMIDPREQUESTCODE = 1;
pub const WMI_GLOBAL_LOGGER_ID: i32 = 1;
pub const WMI_GUIDTYPE_DATA: i32 = 2;
pub const WMI_GUIDTYPE_EVENT: i32 = 3;
pub const WMI_GUIDTYPE_TRACE: i32 = 1;
pub const WMI_GUIDTYPE_TRACECONTROL: i32 = 0;
pub const WMI_REGINFO: WMIDPREQUESTCODE = 8;
pub const WMI_SET_SINGLE_INSTANCE: WMIDPREQUESTCODE = 2;
pub const WMI_SET_SINGLE_ITEM: WMIDPREQUESTCODE = 3;
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct WNODE_ALL_DATA {
    pub WnodeHeader: WNODE_HEADER,
    pub DataBlockOffset: u32,
    pub InstanceCount: u32,
    pub OffsetInstanceNameOffsets: u32,
    pub Anonymous: WNODE_ALL_DATA_0,
}
#[cfg(feature = "winnt")]
impl Default for WNODE_ALL_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub union WNODE_ALL_DATA_0 {
    pub FixedInstanceSize: u32,
    pub OffsetInstanceDataAndLength: [OFFSETINSTANCEDATAANDLENGTH; 0],
}
#[cfg(feature = "winnt")]
impl Default for WNODE_ALL_DATA_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct WNODE_EVENT_ITEM {
    pub WnodeHeader: WNODE_HEADER,
}
#[cfg(feature = "winnt")]
impl Default for WNODE_EVENT_ITEM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct WNODE_EVENT_REFERENCE {
    pub WnodeHeader: WNODE_HEADER,
    pub TargetGuid: windows_sys::core::GUID,
    pub TargetDataBlockSize: u32,
    pub Anonymous: WNODE_EVENT_REFERENCE_0,
}
#[cfg(feature = "winnt")]
impl Default for WNODE_EVENT_REFERENCE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub union WNODE_EVENT_REFERENCE_0 {
    pub TargetInstanceIndex: u32,
    pub TargetInstanceName: [u16; 0],
}
#[cfg(feature = "winnt")]
impl Default for WNODE_EVENT_REFERENCE_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WNODE_FLAG_ALL_DATA: i32 = 1;
pub const WNODE_FLAG_ANSI_INSTANCENAMES: i32 = 16384;
pub const WNODE_FLAG_EVENT_ITEM: i32 = 8;
pub const WNODE_FLAG_EVENT_REFERENCE: i32 = 8192;
pub const WNODE_FLAG_FIXED_INSTANCE_SIZE: i32 = 16;
pub const WNODE_FLAG_INSTANCES_SAME: i32 = 64;
pub const WNODE_FLAG_INTERNAL: i32 = 256;
pub const WNODE_FLAG_LOG_WNODE: i32 = 262144;
pub const WNODE_FLAG_METHOD_ITEM: i32 = 32768;
pub const WNODE_FLAG_NO_HEADER: i32 = 2097152;
pub const WNODE_FLAG_PDO_INSTANCE_NAMES: i32 = 65536;
pub const WNODE_FLAG_PERSIST_EVENT: i32 = 1024;
pub const WNODE_FLAG_SEND_DATA_BLOCK: i32 = 4194304;
pub const WNODE_FLAG_SEVERITY_MASK: u32 = 4278190080;
pub const WNODE_FLAG_SINGLE_INSTANCE: i32 = 2;
pub const WNODE_FLAG_SINGLE_ITEM: i32 = 4;
pub const WNODE_FLAG_STATIC_INSTANCE_NAMES: i32 = 128;
pub const WNODE_FLAG_TOO_SMALL: i32 = 32;
pub const WNODE_FLAG_TRACED_GUID: i32 = 131072;
pub const WNODE_FLAG_USE_GUID_PTR: i32 = 524288;
pub const WNODE_FLAG_USE_MOF_PTR: i32 = 1048576;
pub const WNODE_FLAG_USE_TIMESTAMP: i32 = 512;
pub const WNODE_FLAG_VERSIONED_PROPERTIES: i32 = 8388608;
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct WNODE_HEADER {
    pub BufferSize: u32,
    pub ProviderId: u32,
    pub Anonymous: WNODE_HEADER_0,
    pub Anonymous2: WNODE_HEADER_1,
    pub Guid: windows_sys::core::GUID,
    pub ClientContext: u32,
    pub Flags: u32,
}
#[cfg(feature = "winnt")]
impl Default for WNODE_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub union WNODE_HEADER_0 {
    pub HistoricalContext: u64,
    pub Anonymous: WNODE_HEADER_0_0,
}
#[cfg(feature = "winnt")]
impl Default for WNODE_HEADER_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Default)]
pub struct WNODE_HEADER_0_0 {
    pub Version: u32,
    pub Linkage: u32,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub union WNODE_HEADER_1 {
    pub CountLost: u32,
    pub KernelHandle: super::HANDLE,
    pub TimeStamp: i64,
}
#[cfg(feature = "winnt")]
impl Default for WNODE_HEADER_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct WNODE_METHOD_ITEM {
    pub WnodeHeader: WNODE_HEADER,
    pub OffsetInstanceName: u32,
    pub InstanceIndex: u32,
    pub MethodId: u32,
    pub DataBlockOffset: u32,
    pub SizeDataBlock: u32,
    pub VariableData: [u8; 0],
}
#[cfg(feature = "winnt")]
impl Default for WNODE_METHOD_ITEM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct WNODE_SINGLE_INSTANCE {
    pub WnodeHeader: WNODE_HEADER,
    pub OffsetInstanceName: u32,
    pub InstanceIndex: u32,
    pub DataBlockOffset: u32,
    pub SizeDataBlock: u32,
    pub VariableData: [u8; 0],
}
#[cfg(feature = "winnt")]
impl Default for WNODE_SINGLE_INSTANCE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct WNODE_SINGLE_ITEM {
    pub WnodeHeader: WNODE_HEADER,
    pub OffsetInstanceName: u32,
    pub InstanceIndex: u32,
    pub ItemId: u32,
    pub DataBlockOffset: u32,
    pub SizeDataItem: u32,
    pub VariableData: [u8; 0],
}
#[cfg(feature = "winnt")]
impl Default for WNODE_SINGLE_ITEM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct WNODE_TOO_SMALL {
    pub WnodeHeader: WNODE_HEADER,
    pub SizeNeeded: u32,
}
#[cfg(feature = "winnt")]
impl Default for WNODE_TOO_SMALL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
