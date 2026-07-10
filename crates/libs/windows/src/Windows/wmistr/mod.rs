#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct OFFSETINSTANCEDATAANDLENGTH {
    pub OffsetInstanceData: u32,
    pub LengthInstanceData: u32,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct POFFSETINSTANCEDATAANDLENGTH(pub *mut OFFSETINSTANCEDATAANDLENGTH);
impl POFFSETINSTANCEDATAANDLENGTH {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for POFFSETINSTANCEDATAANDLENGTH {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct PWMIREGGUID(pub PWMIREGGUIDW);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PWMIREGGUIDW(pub *mut WMIREGGUIDW);
impl PWMIREGGUIDW {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PWMIREGGUIDW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct PWMIREGINFO(pub PWMIREGINFOW);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PWMIREGINFOW(pub *mut WMIREGINFOW);
impl PWMIREGINFOW {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PWMIREGINFOW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "winnt")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PWNODE_ALL_DATA(pub *mut WNODE_ALL_DATA);
#[cfg(feature = "winnt")]
impl PWNODE_ALL_DATA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "winnt")]
impl Default for PWNODE_ALL_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "winnt")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PWNODE_EVENT_ITEM(pub *mut WNODE_EVENT_ITEM);
#[cfg(feature = "winnt")]
impl PWNODE_EVENT_ITEM {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "winnt")]
impl Default for PWNODE_EVENT_ITEM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "winnt")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PWNODE_EVENT_REFERENCE(pub *mut WNODE_EVENT_REFERENCE);
#[cfg(feature = "winnt")]
impl PWNODE_EVENT_REFERENCE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "winnt")]
impl Default for PWNODE_EVENT_REFERENCE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "winnt")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PWNODE_HEADER(pub *mut WNODE_HEADER);
#[cfg(feature = "winnt")]
impl PWNODE_HEADER {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "winnt")]
impl Default for PWNODE_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "winnt")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PWNODE_METHOD_ITEM(pub *mut WNODE_METHOD_ITEM);
#[cfg(feature = "winnt")]
impl PWNODE_METHOD_ITEM {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "winnt")]
impl Default for PWNODE_METHOD_ITEM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "winnt")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PWNODE_SINGLE_INSTANCE(pub *mut WNODE_SINGLE_INSTANCE);
#[cfg(feature = "winnt")]
impl PWNODE_SINGLE_INSTANCE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "winnt")]
impl Default for PWNODE_SINGLE_INSTANCE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "winnt")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PWNODE_SINGLE_ITEM(pub *mut WNODE_SINGLE_ITEM);
#[cfg(feature = "winnt")]
impl PWNODE_SINGLE_ITEM {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "winnt")]
impl Default for PWNODE_SINGLE_ITEM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "winnt")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PWNODE_TOO_SMALL(pub *mut WNODE_TOO_SMALL);
#[cfg(feature = "winnt")]
impl PWNODE_TOO_SMALL {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "winnt")]
impl Default for PWNODE_TOO_SMALL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const TRACELOG_ACCESS_KERNEL_LOGGER: u32 = 256;
pub const TRACELOG_ACCESS_REALTIME: u32 = 1024;
pub const TRACELOG_CREATE_INPROC: u32 = 512;
pub const TRACELOG_CREATE_ONDISK: u32 = 64;
pub const TRACELOG_CREATE_REALTIME: u32 = 32;
pub const TRACELOG_GUID_ENABLE: u32 = 128;
pub const TRACELOG_JOIN_GROUP: u32 = 4096;
pub const TRACELOG_LOG_EVENT: u32 = 512;
pub const TRACELOG_REGISTER_GUIDS: u32 = 2048;
pub type WMIDPREQUESTCODE = i32;
pub const WMIGUID_ALL_ACCESS: u32 = 1187839;
pub const WMIGUID_ALL_ACCESS_RS1: u32 = 1187839;
pub const WMIGUID_ALL_ACCESS_WIN2K: u32 = 133119;
pub const WMIGUID_ALL_ACCESS_WINXP: u32 = 1183743;
pub const WMIGUID_EXECUTE: u32 = 16;
pub const WMIGUID_NOTIFICATION: u32 = 4;
pub const WMIGUID_QUERY: u32 = 1;
pub const WMIGUID_READ_DESCRIPTION: u32 = 8;
pub const WMIGUID_SET: u32 = 2;
pub type WMIREGGUID = WMIREGGUIDW;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WMIREGGUIDW {
    pub Guid: windows_core::GUID,
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
pub const WMIREG_FLAG_EVENT_ONLY_GUID: u32 = 64;
pub const WMIREG_FLAG_EXPENSIVE: u32 = 1;
pub const WMIREG_FLAG_INSTANCE_BASENAME: u32 = 8;
pub const WMIREG_FLAG_INSTANCE_LIST: u32 = 4;
pub const WMIREG_FLAG_INSTANCE_PDO: u32 = 32;
pub const WMIREG_FLAG_REMOVE_GUID: u32 = 65536;
pub const WMIREG_FLAG_RESERVED1: u32 = 131072;
pub const WMIREG_FLAG_RESERVED2: u32 = 262144;
pub const WMIREG_FLAG_TRACED_GUID: u32 = 524288;
pub const WMIREG_FLAG_TRACE_CONTROL_GUID: u32 = 4096;
pub const WMI_CAPTURE_STATE: WMIDPREQUESTCODE = 10;
pub const WMI_DISABLE_COLLECTION: WMIDPREQUESTCODE = 7;
pub const WMI_DISABLE_EVENTS: WMIDPREQUESTCODE = 5;
pub const WMI_ENABLE_COLLECTION: WMIDPREQUESTCODE = 6;
pub const WMI_ENABLE_EVENTS: WMIDPREQUESTCODE = 4;
pub const WMI_EXECUTE_METHOD: WMIDPREQUESTCODE = 9;
pub const WMI_GET_ALL_DATA: WMIDPREQUESTCODE = 0;
pub const WMI_GET_SINGLE_INSTANCE: WMIDPREQUESTCODE = 1;
pub const WMI_GLOBAL_LOGGER_ID: u32 = 1;
pub const WMI_GUIDTYPE_DATA: u32 = 2;
pub const WMI_GUIDTYPE_EVENT: u32 = 3;
pub const WMI_GUIDTYPE_TRACE: u32 = 1;
pub const WMI_GUIDTYPE_TRACECONTROL: u32 = 0;
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
    pub TargetGuid: windows_core::GUID,
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
pub const WNODE_FLAG_ALL_DATA: u32 = 1;
pub const WNODE_FLAG_ANSI_INSTANCENAMES: u32 = 16384;
pub const WNODE_FLAG_EVENT_ITEM: u32 = 8;
pub const WNODE_FLAG_EVENT_REFERENCE: u32 = 8192;
pub const WNODE_FLAG_FIXED_INSTANCE_SIZE: u32 = 16;
pub const WNODE_FLAG_INSTANCES_SAME: u32 = 64;
pub const WNODE_FLAG_INTERNAL: u32 = 256;
pub const WNODE_FLAG_LOG_WNODE: u32 = 262144;
pub const WNODE_FLAG_METHOD_ITEM: u32 = 32768;
pub const WNODE_FLAG_NO_HEADER: u32 = 2097152;
pub const WNODE_FLAG_PDO_INSTANCE_NAMES: u32 = 65536;
pub const WNODE_FLAG_PERSIST_EVENT: u32 = 1024;
pub const WNODE_FLAG_SEND_DATA_BLOCK: u32 = 4194304;
pub const WNODE_FLAG_SEVERITY_MASK: u32 = 4278190080;
pub const WNODE_FLAG_SINGLE_INSTANCE: u32 = 2;
pub const WNODE_FLAG_SINGLE_ITEM: u32 = 4;
pub const WNODE_FLAG_STATIC_INSTANCE_NAMES: u32 = 128;
pub const WNODE_FLAG_TOO_SMALL: u32 = 32;
pub const WNODE_FLAG_TRACED_GUID: u32 = 131072;
pub const WNODE_FLAG_USE_GUID_PTR: u32 = 524288;
pub const WNODE_FLAG_USE_MOF_PTR: u32 = 1048576;
pub const WNODE_FLAG_USE_TIMESTAMP: u32 = 512;
pub const WNODE_FLAG_VERSIONED_PROPERTIES: u32 = 8388608;
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct WNODE_HEADER {
    pub BufferSize: u32,
    pub ProviderId: u32,
    pub Anonymous: WNODE_HEADER_0,
    pub Anonymous2: WNODE_HEADER_1,
    pub Guid: windows_core::GUID,
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WNODE_HEADER_0_0 {
    pub Version: u32,
    pub Linkage: u32,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub union WNODE_HEADER_1 {
    pub CountLost: u32,
    pub KernelHandle: super::winnt::HANDLE,
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
