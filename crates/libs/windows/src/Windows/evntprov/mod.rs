#[inline]
pub unsafe fn EventActivityIdControl(controlcode: u32, activityid: *mut windows_core::GUID) -> u32 {
    windows_core::link!("advapi32.dll" "system" fn EventActivityIdControl(controlcode : u32, activityid : *mut windows_core::GUID) -> u32);
    unsafe { EventActivityIdControl(controlcode, activityid as _) }
}
#[inline]
pub unsafe fn EventEnabled(reghandle: REGHANDLE, eventdescriptor: *const EVENT_DESCRIPTOR) -> bool {
    windows_core::link!("advapi32.dll" "system" fn EventEnabled(reghandle : REGHANDLE, eventdescriptor : *const EVENT_DESCRIPTOR) -> bool);
    unsafe { EventEnabled(reghandle, eventdescriptor) }
}
#[inline]
pub unsafe fn EventProviderEnabled(reghandle: REGHANDLE, level: u8, keyword: u64) -> bool {
    windows_core::link!("advapi32.dll" "system" fn EventProviderEnabled(reghandle : REGHANDLE, level : u8, keyword : u64) -> bool);
    unsafe { EventProviderEnabled(reghandle, level, keyword) }
}
#[inline]
pub unsafe fn EventRegister(providerid: *const windows_core::GUID, enablecallback: PENABLECALLBACK, callbackcontext: Option<*const core::ffi::c_void>, reghandle: *mut u64) -> u32 {
    windows_core::link!("advapi32.dll" "system" fn EventRegister(providerid : *const windows_core::GUID, enablecallback : PENABLECALLBACK, callbackcontext : *const core::ffi::c_void, reghandle : *mut u64) -> u32);
    unsafe { EventRegister(providerid, enablecallback, callbackcontext.unwrap_or(core::mem::zeroed()) as _, reghandle as _) }
}
#[inline]
pub unsafe fn EventSetInformation(reghandle: REGHANDLE, informationclass: EVENT_INFO_CLASS, eventinformation: *const core::ffi::c_void, informationlength: u32) -> u32 {
    windows_core::link!("advapi32.dll" "system" fn EventSetInformation(reghandle : REGHANDLE, informationclass : EVENT_INFO_CLASS, eventinformation : *const core::ffi::c_void, informationlength : u32) -> u32);
    unsafe { EventSetInformation(reghandle, informationclass, eventinformation, informationlength) }
}
#[inline]
pub unsafe fn EventUnregister(reghandle: REGHANDLE) -> u32 {
    windows_core::link!("advapi32.dll" "system" fn EventUnregister(reghandle : REGHANDLE) -> u32);
    unsafe { EventUnregister(reghandle) }
}
#[inline]
pub unsafe fn EventWrite(reghandle: REGHANDLE, eventdescriptor: *const EVENT_DESCRIPTOR, userdata: Option<&[EVENT_DATA_DESCRIPTOR]>) -> u32 {
    windows_core::link!("advapi32.dll" "system" fn EventWrite(reghandle : REGHANDLE, eventdescriptor : *const EVENT_DESCRIPTOR, userdatacount : u32, userdata : *const EVENT_DATA_DESCRIPTOR) -> u32);
    unsafe { EventWrite(reghandle, eventdescriptor, userdata.map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(userdata.map_or(core::ptr::null(), |slice| slice.as_ptr()))) }
}
#[inline]
pub unsafe fn EventWriteEx(reghandle: REGHANDLE, eventdescriptor: *const EVENT_DESCRIPTOR, filter: u64, flags: u32, activityid: Option<*const windows_core::GUID>, relatedactivityid: Option<*const windows_core::GUID>, userdata: Option<&[EVENT_DATA_DESCRIPTOR]>) -> u32 {
    windows_core::link!("advapi32.dll" "system" fn EventWriteEx(reghandle : REGHANDLE, eventdescriptor : *const EVENT_DESCRIPTOR, filter : u64, flags : u32, activityid : *const windows_core::GUID, relatedactivityid : *const windows_core::GUID, userdatacount : u32, userdata : *const EVENT_DATA_DESCRIPTOR) -> u32);
    unsafe { EventWriteEx(reghandle, eventdescriptor, filter, flags, activityid.unwrap_or(core::mem::zeroed()) as _, relatedactivityid.unwrap_or(core::mem::zeroed()) as _, userdata.map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(userdata.map_or(core::ptr::null(), |slice| slice.as_ptr()))) }
}
#[inline]
pub unsafe fn EventWriteString<P3>(reghandle: REGHANDLE, level: u8, keyword: u64, string: P3) -> u32
where
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn EventWriteString(reghandle : REGHANDLE, level : u8, keyword : u64, string : windows_core::PCWSTR) -> u32);
    unsafe { EventWriteString(reghandle, level, keyword, string.param().abi()) }
}
#[inline]
pub unsafe fn EventWriteTransfer(reghandle: REGHANDLE, eventdescriptor: *const EVENT_DESCRIPTOR, activityid: Option<*const windows_core::GUID>, relatedactivityid: Option<*const windows_core::GUID>, userdata: Option<&[EVENT_DATA_DESCRIPTOR]>) -> u32 {
    windows_core::link!("advapi32.dll" "system" fn EventWriteTransfer(reghandle : REGHANDLE, eventdescriptor : *const EVENT_DESCRIPTOR, activityid : *const windows_core::GUID, relatedactivityid : *const windows_core::GUID, userdatacount : u32, userdata : *const EVENT_DATA_DESCRIPTOR) -> u32);
    unsafe { EventWriteTransfer(reghandle, eventdescriptor, activityid.unwrap_or(core::mem::zeroed()) as _, relatedactivityid.unwrap_or(core::mem::zeroed()) as _, userdata.map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(userdata.map_or(core::ptr::null(), |slice| slice.as_ptr()))) }
}
pub const EVENT_ACTIVITY_CTRL_CREATE_ID: u32 = 3;
pub const EVENT_ACTIVITY_CTRL_CREATE_SET_ID: u32 = 5;
pub const EVENT_ACTIVITY_CTRL_GET_ID: u32 = 1;
pub const EVENT_ACTIVITY_CTRL_GET_SET_ID: u32 = 4;
pub const EVENT_ACTIVITY_CTRL_SET_ID: u32 = 2;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct EVENT_DATA_DESCRIPTOR {
    pub Ptr: u64,
    pub Size: u32,
    pub Anonymous: EVENT_DATA_DESCRIPTOR_0,
}
impl Default for EVENT_DATA_DESCRIPTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union EVENT_DATA_DESCRIPTOR_0 {
    pub Reserved: u32,
    pub Anonymous: EVENT_DATA_DESCRIPTOR_0_0,
}
impl Default for EVENT_DATA_DESCRIPTOR_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct EVENT_DATA_DESCRIPTOR_0_0 {
    pub Type: u8,
    pub Reserved1: u8,
    pub Reserved2: u16,
}
pub const EVENT_DATA_DESCRIPTOR_TYPE_EVENT_METADATA: u32 = 1;
pub const EVENT_DATA_DESCRIPTOR_TYPE_NONE: u32 = 0;
pub const EVENT_DATA_DESCRIPTOR_TYPE_PROVIDER_METADATA: u32 = 2;
pub const EVENT_DATA_DESCRIPTOR_TYPE_RESERVED1: u32 = 4;
pub const EVENT_DATA_DESCRIPTOR_TYPE_TIMESTAMP_OVERRIDE: u32 = 3;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct EVENT_DESCRIPTOR {
    pub Id: u16,
    pub Version: u8,
    pub Channel: u8,
    pub Level: u8,
    pub Opcode: u8,
    pub Task: u16,
    pub Keyword: u64,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct EVENT_FILTER_DESCRIPTOR {
    pub Ptr: u64,
    pub Size: u32,
    pub Type: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EVENT_FILTER_EVENT_ID {
    pub FilterIn: bool,
    pub Reserved: u8,
    pub Count: u16,
    pub Events: [u16; 1],
}
impl Default for EVENT_FILTER_EVENT_ID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EVENT_FILTER_EVENT_NAME {
    pub MatchAnyKeyword: u64,
    pub MatchAllKeyword: u64,
    pub Level: u8,
    pub FilterIn: bool,
    pub NameCount: u16,
    pub Names: [u8; 1],
}
impl Default for EVENT_FILTER_EVENT_NAME {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EVENT_FILTER_HEADER {
    pub Id: u16,
    pub Version: u8,
    pub Reserved: [u8; 5],
    pub InstanceId: u64,
    pub Size: u32,
    pub NextOffset: u32,
}
impl Default for EVENT_FILTER_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct EVENT_FILTER_LEVEL_KW {
    pub MatchAnyKeyword: u64,
    pub MatchAllKeyword: u64,
    pub Level: u8,
    pub FilterIn: bool,
}
pub const EVENT_FILTER_TYPE_CONTAINER: u32 = 2147516416;
pub const EVENT_FILTER_TYPE_EVENT_ID: u32 = 2147484160;
pub const EVENT_FILTER_TYPE_EVENT_NAME: u32 = 2147484672;
pub const EVENT_FILTER_TYPE_EXECUTABLE_NAME: u32 = 2147483656;
pub const EVENT_FILTER_TYPE_NONE: u32 = 0;
pub const EVENT_FILTER_TYPE_PACKAGE_APP_ID: u32 = 2147483680;
pub const EVENT_FILTER_TYPE_PACKAGE_ID: u32 = 2147483664;
pub const EVENT_FILTER_TYPE_PAYLOAD: u32 = 2147483904;
pub const EVENT_FILTER_TYPE_PID: u32 = 2147483652;
pub const EVENT_FILTER_TYPE_SCHEMATIZED: u32 = 2147483648;
pub const EVENT_FILTER_TYPE_STACKWALK: u32 = 2147487744;
pub const EVENT_FILTER_TYPE_STACKWALK_LEVEL_KW: u32 = 2147500032;
pub const EVENT_FILTER_TYPE_STACKWALK_NAME: u32 = 2147491840;
pub const EVENT_FILTER_TYPE_SYSTEM_FLAGS: u32 = 2147483649;
pub const EVENT_FILTER_TYPE_TRACEHANDLE: u32 = 2147483650;
pub type EVENT_INFO_CLASS = i32;
pub const EVENT_MAX_LEVEL: u32 = 255;
pub const EVENT_MIN_LEVEL: u32 = 0;
pub const EVENT_WRITE_FLAG_INPRIVATE: u32 = 2;
pub const EVENT_WRITE_FLAG_NO_FAULTING: u32 = 1;
pub const EventProviderBinaryTrackInfo: EVENT_INFO_CLASS = 0;
pub const EventProviderSetReserved1: EVENT_INFO_CLASS = 1;
pub const EventProviderSetReserved2: EVENT_INFO_CLASS = 4;
pub const EventProviderSetTraits: EVENT_INFO_CLASS = 2;
pub const EventProviderUseDescriptorType: EVENT_INFO_CLASS = 3;
pub const MAX_EVENT_DATA_DESCRIPTORS: u32 = 128;
pub const MAX_EVENT_FILTERS_COUNT: u32 = 13;
pub const MAX_EVENT_FILTER_DATA_SIZE: u32 = 1024;
pub const MAX_EVENT_FILTER_EVENT_ID_COUNT: u32 = 64;
pub const MAX_EVENT_FILTER_EVENT_NAME_SIZE: u32 = 4096;
pub const MAX_EVENT_FILTER_PAYLOAD_SIZE: u32 = 4096;
pub const MAX_EVENT_FILTER_PID_COUNT: u32 = 8;
pub const MaxEventInfo: EVENT_INFO_CLASS = 5;
pub type PCEVENT_DESCRIPTOR = *const EVENT_DESCRIPTOR;
pub type PENABLECALLBACK = Option<unsafe extern "system" fn(sourceid: *const windows_core::GUID, isenabled: u32, level: u8, matchanykeyword: u64, matchallkeyword: u64, filterdata: *const EVENT_FILTER_DESCRIPTOR, callbackcontext: *mut core::ffi::c_void)>;
pub type PEVENT_DATA_DESCRIPTOR = *mut EVENT_DATA_DESCRIPTOR;
pub type PEVENT_DESCRIPTOR = *mut EVENT_DESCRIPTOR;
pub type PEVENT_FILTER_EVENT_ID = *mut EVENT_FILTER_EVENT_ID;
pub type PEVENT_FILTER_EVENT_NAME = *mut EVENT_FILTER_EVENT_NAME;
pub type PEVENT_FILTER_HEADER = *mut EVENT_FILTER_HEADER;
pub type PEVENT_FILTER_LEVEL_KW = *mut EVENT_FILTER_LEVEL_KW;
pub type PREGHANDLE = *mut u64;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct REGHANDLE(pub u64);
