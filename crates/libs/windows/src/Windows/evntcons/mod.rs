#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn EventAccessControl(guid: *const windows_core::GUID, operation: u32, sid: super::winnt::PSID, rights: u32, allowordeny: bool) -> u32 {
    windows_core::link!("advapi32.dll" "system" fn EventAccessControl(guid : *const windows_core::GUID, operation : u32, sid : super::winnt::PSID, rights : u32, allowordeny : bool) -> u32);
    unsafe { EventAccessControl(guid, operation, sid, rights, allowordeny) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn EventAccessQuery(guid: *const windows_core::GUID, buffer: Option<super::winnt::PSECURITY_DESCRIPTOR>, buffersize: *mut u32) -> u32 {
    windows_core::link!("advapi32.dll" "system" fn EventAccessQuery(guid : *const windows_core::GUID, buffer : super::winnt::PSECURITY_DESCRIPTOR, buffersize : *mut u32) -> u32);
    unsafe { EventAccessQuery(guid, buffer.unwrap_or(core::mem::zeroed()) as _, buffersize as _) }
}
#[inline]
pub unsafe fn EventAccessRemove(guid: *const windows_core::GUID) -> u32 {
    windows_core::link!("advapi32.dll" "system" fn EventAccessRemove(guid : *const windows_core::GUID) -> u32);
    unsafe { EventAccessRemove(guid) }
}
pub type ETW_PROVIDER_TRAIT_TYPE = i32;
pub type EVENTSECURITYOPERATION = i32;
pub const EVENT_ENABLE_PROPERTY_ENABLE_KEYWORD_0: u32 = 64;
pub const EVENT_ENABLE_PROPERTY_ENABLE_SILOS: u32 = 1024;
pub const EVENT_ENABLE_PROPERTY_EVENT_KEY: u32 = 256;
pub const EVENT_ENABLE_PROPERTY_EXCLUDE_INPRIVATE: u32 = 512;
pub const EVENT_ENABLE_PROPERTY_IGNORE_KEYWORD_0: u32 = 16;
pub const EVENT_ENABLE_PROPERTY_PROCESS_START_KEY: u32 = 128;
pub const EVENT_ENABLE_PROPERTY_PROVIDER_GROUP: u32 = 32;
pub const EVENT_ENABLE_PROPERTY_PSM_KEY: u32 = 8;
pub const EVENT_ENABLE_PROPERTY_SID: u32 = 1;
pub const EVENT_ENABLE_PROPERTY_SOURCE_CONTAINER_TRACKING: u32 = 2048;
pub const EVENT_ENABLE_PROPERTY_STACK_TRACE: u32 = 4;
pub const EVENT_ENABLE_PROPERTY_TS_ID: u32 = 2;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EVENT_EXTENDED_ITEM_EVENT_KEY {
    pub Key: u64,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EVENT_EXTENDED_ITEM_INSTANCE {
    pub InstanceId: u32,
    pub ParentInstanceId: u32,
    pub ParentGuid: windows_core::GUID,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EVENT_EXTENDED_ITEM_PEBS_INDEX {
    pub PebsIndex: u64,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EVENT_EXTENDED_ITEM_PMC_COUNTERS {
    pub Counter: [u64; 1],
}
impl Default for EVENT_EXTENDED_ITEM_PMC_COUNTERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EVENT_EXTENDED_ITEM_PROCESS_START_KEY {
    pub ProcessStartKey: u64,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EVENT_EXTENDED_ITEM_RELATED_ACTIVITYID {
    pub RelatedActivityId: windows_core::GUID,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EVENT_EXTENDED_ITEM_STACK_KEY32 {
    pub MatchId: u64,
    pub StackKey: u32,
    pub Padding: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EVENT_EXTENDED_ITEM_STACK_KEY64 {
    pub MatchId: u64,
    pub StackKey: u64,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EVENT_EXTENDED_ITEM_STACK_TRACE32 {
    pub MatchId: u64,
    pub Address: [u32; 1],
}
impl Default for EVENT_EXTENDED_ITEM_STACK_TRACE32 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EVENT_EXTENDED_ITEM_STACK_TRACE64 {
    pub MatchId: u64,
    pub Address: [u64; 1],
}
impl Default for EVENT_EXTENDED_ITEM_STACK_TRACE64 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EVENT_EXTENDED_ITEM_TS_ID {
    pub SessionId: u32,
}
#[repr(C)]
#[cfg(feature = "evntprov")]
#[derive(Clone, Copy)]
pub struct EVENT_HEADER {
    pub Size: u16,
    pub HeaderType: u16,
    pub Flags: u16,
    pub EventProperty: u16,
    pub ThreadId: u32,
    pub ProcessId: u32,
    pub TimeStamp: i64,
    pub ProviderId: windows_core::GUID,
    pub EventDescriptor: super::evntprov::EVENT_DESCRIPTOR,
    pub Anonymous: EVENT_HEADER_0,
    pub ActivityId: windows_core::GUID,
}
#[cfg(feature = "evntprov")]
impl Default for EVENT_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "evntprov")]
#[derive(Clone, Copy)]
pub union EVENT_HEADER_0 {
    pub Anonymous: EVENT_HEADER_0_0,
    pub ProcessorTime: u64,
}
#[cfg(feature = "evntprov")]
impl Default for EVENT_HEADER_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "evntprov")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EVENT_HEADER_0_0 {
    pub KernelTime: u32,
    pub UserTime: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EVENT_HEADER_EXTENDED_DATA_ITEM {
    pub Reserved1: u16,
    pub ExtType: u16,
    pub Anonymous: EVENT_HEADER_EXTENDED_DATA_ITEM_0,
    pub DataSize: u16,
    pub DataPtr: u64,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EVENT_HEADER_EXTENDED_DATA_ITEM_0 {
    pub _bitfield: u16,
}
pub const EVENT_HEADER_EXT_TYPE_CONTAINER_ID: u32 = 16;
pub const EVENT_HEADER_EXT_TYPE_CONTROL_GUID: u32 = 14;
pub const EVENT_HEADER_EXT_TYPE_EVENT_KEY: u32 = 10;
pub const EVENT_HEADER_EXT_TYPE_EVENT_SCHEMA_TL: u32 = 11;
pub const EVENT_HEADER_EXT_TYPE_INSTANCE_INFO: u32 = 4;
pub const EVENT_HEADER_EXT_TYPE_MAX: u32 = 19;
pub const EVENT_HEADER_EXT_TYPE_PEBS_INDEX: u32 = 7;
pub const EVENT_HEADER_EXT_TYPE_PMC_COUNTERS: u32 = 8;
pub const EVENT_HEADER_EXT_TYPE_PROCESS_START_KEY: u32 = 13;
pub const EVENT_HEADER_EXT_TYPE_PROV_TRAITS: u32 = 12;
pub const EVENT_HEADER_EXT_TYPE_PSM_KEY: u32 = 9;
pub const EVENT_HEADER_EXT_TYPE_QPC_DELTA: u32 = 15;
pub const EVENT_HEADER_EXT_TYPE_RELATED_ACTIVITYID: u32 = 1;
pub const EVENT_HEADER_EXT_TYPE_SID: u32 = 2;
pub const EVENT_HEADER_EXT_TYPE_STACK_KEY32: u32 = 17;
pub const EVENT_HEADER_EXT_TYPE_STACK_KEY64: u32 = 18;
pub const EVENT_HEADER_EXT_TYPE_STACK_TRACE32: u32 = 5;
pub const EVENT_HEADER_EXT_TYPE_STACK_TRACE64: u32 = 6;
pub const EVENT_HEADER_EXT_TYPE_TS_ID: u32 = 3;
pub const EVENT_HEADER_FLAG_32_BIT_HEADER: u32 = 32;
pub const EVENT_HEADER_FLAG_64_BIT_HEADER: u32 = 64;
pub const EVENT_HEADER_FLAG_CLASSIC_HEADER: u32 = 256;
pub const EVENT_HEADER_FLAG_DECODE_GUID: u32 = 128;
pub const EVENT_HEADER_FLAG_EXTENDED_INFO: u32 = 1;
pub const EVENT_HEADER_FLAG_NO_CPUTIME: u32 = 16;
pub const EVENT_HEADER_FLAG_PRIVATE_SESSION: u32 = 2;
pub const EVENT_HEADER_FLAG_PROCESSOR_INDEX: u32 = 512;
pub const EVENT_HEADER_FLAG_RESERVED1: u32 = 1024;
pub const EVENT_HEADER_FLAG_STRING_ONLY: u32 = 4;
pub const EVENT_HEADER_FLAG_TRACE_MESSAGE: u32 = 8;
pub const EVENT_HEADER_PROPERTY_FORWARDED_XML: u32 = 2;
pub const EVENT_HEADER_PROPERTY_LEGACY_EVENTLOG: u32 = 4;
pub const EVENT_HEADER_PROPERTY_RELOGGABLE: u32 = 8;
pub const EVENT_HEADER_PROPERTY_XML: u32 = 1;
#[repr(C)]
#[cfg(all(feature = "evntprov", feature = "evntrace"))]
#[derive(Clone, Copy)]
pub struct EVENT_RECORD {
    pub EventHeader: EVENT_HEADER,
    pub BufferContext: super::evntrace::ETW_BUFFER_CONTEXT,
    pub ExtendedDataCount: u16,
    pub UserDataLength: u16,
    pub ExtendedData: PEVENT_HEADER_EXTENDED_DATA_ITEM,
    pub UserData: *mut core::ffi::c_void,
    pub UserContext: *mut core::ffi::c_void,
}
#[cfg(all(feature = "evntprov", feature = "evntrace"))]
impl Default for EVENT_RECORD {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const EtwProviderTraitDecodeGuid: ETW_PROVIDER_TRAIT_TYPE = 2;
pub const EtwProviderTraitTypeGroup: ETW_PROVIDER_TRAIT_TYPE = 1;
pub const EtwProviderTraitTypeMax: ETW_PROVIDER_TRAIT_TYPE = 3;
pub const EventSecurityAddDACL: EVENTSECURITYOPERATION = 2;
pub const EventSecurityAddSACL: EVENTSECURITYOPERATION = 3;
pub const EventSecurityMax: EVENTSECURITYOPERATION = 4;
pub const EventSecuritySetDACL: EVENTSECURITYOPERATION = 0;
pub const EventSecuritySetSACL: EVENTSECURITYOPERATION = 1;
#[cfg(all(feature = "evntprov", feature = "evntrace"))]
pub type PCEVENT_RECORD = *const EVENT_RECORD;
pub type PEVENT_EXTENDED_ITEM_EVENT_KEY = *mut EVENT_EXTENDED_ITEM_EVENT_KEY;
pub type PEVENT_EXTENDED_ITEM_INSTANCE = *mut EVENT_EXTENDED_ITEM_INSTANCE;
pub type PEVENT_EXTENDED_ITEM_PEBS_INDEX = *mut EVENT_EXTENDED_ITEM_PEBS_INDEX;
pub type PEVENT_EXTENDED_ITEM_PMC_COUNTERS = *mut EVENT_EXTENDED_ITEM_PMC_COUNTERS;
pub type PEVENT_EXTENDED_ITEM_PROCESS_START_KEY = *mut EVENT_EXTENDED_ITEM_PROCESS_START_KEY;
pub type PEVENT_EXTENDED_ITEM_RELATED_ACTIVITYID = *mut EVENT_EXTENDED_ITEM_RELATED_ACTIVITYID;
pub type PEVENT_EXTENDED_ITEM_STACK_KEY32 = *mut EVENT_EXTENDED_ITEM_STACK_KEY32;
pub type PEVENT_EXTENDED_ITEM_STACK_KEY64 = *mut EVENT_EXTENDED_ITEM_STACK_KEY64;
pub type PEVENT_EXTENDED_ITEM_STACK_TRACE32 = *mut EVENT_EXTENDED_ITEM_STACK_TRACE32;
pub type PEVENT_EXTENDED_ITEM_STACK_TRACE64 = *mut EVENT_EXTENDED_ITEM_STACK_TRACE64;
pub type PEVENT_EXTENDED_ITEM_TS_ID = *mut EVENT_EXTENDED_ITEM_TS_ID;
#[cfg(feature = "evntprov")]
pub type PEVENT_HEADER = *mut EVENT_HEADER;
pub type PEVENT_HEADER_EXTENDED_DATA_ITEM = *mut EVENT_HEADER_EXTENDED_DATA_ITEM;
pub const PROCESS_TRACE_MODE_EVENT_RECORD: u32 = 268435456;
pub const PROCESS_TRACE_MODE_RAW_TIMESTAMP: u32 = 4096;
pub const PROCESS_TRACE_MODE_REAL_TIME: u32 = 256;
