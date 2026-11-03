#[cfg(all(feature = "guiddef", feature = "shared"))]
#[inline]
pub unsafe fn EventActivityIdControl(controlcode: super::ULONG, activityid: super::LPGUID) -> super::ULONG {
    windows_core::link!("advapi32.dll" "system" fn EventActivityIdControl(controlcode : super::ULONG, activityid : super::LPGUID) -> super::ULONG);
    unsafe { EventActivityIdControl(controlcode, activityid as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn EventEnabled(reghandle: REGHANDLE, eventdescriptor: PCEVENT_DESCRIPTOR) -> super::BOOLEAN {
    windows_core::link!("advapi32.dll" "system" fn EventEnabled(reghandle : REGHANDLE, eventdescriptor : PCEVENT_DESCRIPTOR) -> super::BOOLEAN);
    unsafe { EventEnabled(reghandle, eventdescriptor) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn EventProviderEnabled(reghandle: REGHANDLE, level: super::UCHAR, keyword: super::ULONGLONG) -> super::BOOLEAN {
    windows_core::link!("advapi32.dll" "system" fn EventProviderEnabled(reghandle : REGHANDLE, level : super::UCHAR, keyword : super::ULONGLONG) -> super::BOOLEAN);
    unsafe { EventProviderEnabled(reghandle, level, keyword) }
}
#[cfg(all(feature = "guiddef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn EventRegister(providerid: super::LPCGUID, enablecallback: PENABLECALLBACK, callbackcontext: Option<super::PVOID>, reghandle: PREGHANDLE) -> super::ULONG {
    windows_core::link!("advapi32.dll" "system" fn EventRegister(providerid : super::LPCGUID, enablecallback : PENABLECALLBACK, callbackcontext : super::PVOID, reghandle : PREGHANDLE) -> super::ULONG);
    unsafe { EventRegister(providerid, enablecallback, callbackcontext.unwrap_or(core::mem::zeroed()) as _, reghandle as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn EventSetInformation(reghandle: REGHANDLE, informationclass: EVENT_INFO_CLASS, eventinformation: super::PVOID, informationlength: super::ULONG) -> super::ULONG {
    windows_core::link!("advapi32.dll" "system" fn EventSetInformation(reghandle : REGHANDLE, informationclass : EVENT_INFO_CLASS, eventinformation : super::PVOID, informationlength : super::ULONG) -> super::ULONG);
    unsafe { EventSetInformation(reghandle, informationclass, eventinformation, informationlength) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn EventUnregister(reghandle: REGHANDLE) -> super::ULONG {
    windows_core::link!("advapi32.dll" "system" fn EventUnregister(reghandle : REGHANDLE) -> super::ULONG);
    unsafe { EventUnregister(reghandle) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn EventWrite(reghandle: REGHANDLE, eventdescriptor: PCEVENT_DESCRIPTOR, userdatacount: super::ULONG, userdata: Option<PEVENT_DATA_DESCRIPTOR>) -> super::ULONG {
    windows_core::link!("advapi32.dll" "system" fn EventWrite(reghandle : REGHANDLE, eventdescriptor : PCEVENT_DESCRIPTOR, userdatacount : super::ULONG, userdata : PEVENT_DATA_DESCRIPTOR) -> super::ULONG);
    unsafe { EventWrite(reghandle, eventdescriptor, userdatacount, userdata.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "guiddef", feature = "shared"))]
#[inline]
pub unsafe fn EventWriteEx(reghandle: REGHANDLE, eventdescriptor: PCEVENT_DESCRIPTOR, filter: super::ULONG64, flags: super::ULONG, activityid: Option<super::LPCGUID>, relatedactivityid: Option<super::LPCGUID>, userdatacount: super::ULONG, userdata: Option<PEVENT_DATA_DESCRIPTOR>) -> super::ULONG {
    windows_core::link!("advapi32.dll" "system" fn EventWriteEx(reghandle : REGHANDLE, eventdescriptor : PCEVENT_DESCRIPTOR, filter : super::ULONG64, flags : super::ULONG, activityid : super::LPCGUID, relatedactivityid : super::LPCGUID, userdatacount : super::ULONG, userdata : PEVENT_DATA_DESCRIPTOR) -> super::ULONG);
    unsafe { EventWriteEx(reghandle, eventdescriptor, filter, flags, activityid.unwrap_or(core::mem::zeroed()) as _, relatedactivityid.unwrap_or(core::mem::zeroed()) as _, userdatacount, userdata.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn EventWriteString<P3>(reghandle: REGHANDLE, level: super::UCHAR, keyword: super::ULONGLONG, string: P3) -> super::ULONG
where
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn EventWriteString(reghandle : REGHANDLE, level : super::UCHAR, keyword : super::ULONGLONG, string : windows_core::PCWSTR) -> super::ULONG);
    unsafe { EventWriteString(reghandle, level, keyword, string.param().abi()) }
}
#[cfg(all(feature = "guiddef", feature = "shared"))]
#[inline]
pub unsafe fn EventWriteTransfer(reghandle: REGHANDLE, eventdescriptor: PCEVENT_DESCRIPTOR, activityid: Option<super::LPCGUID>, relatedactivityid: Option<super::LPCGUID>, userdatacount: super::ULONG, userdata: Option<PEVENT_DATA_DESCRIPTOR>) -> super::ULONG {
    windows_core::link!("advapi32.dll" "system" fn EventWriteTransfer(reghandle : REGHANDLE, eventdescriptor : PCEVENT_DESCRIPTOR, activityid : super::LPCGUID, relatedactivityid : super::LPCGUID, userdatacount : super::ULONG, userdata : PEVENT_DATA_DESCRIPTOR) -> super::ULONG);
    unsafe { EventWriteTransfer(reghandle, eventdescriptor, activityid.unwrap_or(core::mem::zeroed()) as _, relatedactivityid.unwrap_or(core::mem::zeroed()) as _, userdatacount, userdata.unwrap_or(core::mem::zeroed()) as _) }
}
pub const EVENT_ACTIVITY_CTRL_CREATE_ID: i32 = 3;
pub const EVENT_ACTIVITY_CTRL_CREATE_SET_ID: i32 = 5;
pub const EVENT_ACTIVITY_CTRL_GET_ID: i32 = 1;
pub const EVENT_ACTIVITY_CTRL_GET_SET_ID: i32 = 4;
pub const EVENT_ACTIVITY_CTRL_SET_ID: i32 = 2;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct EVENT_DATA_DESCRIPTOR {
    pub Ptr: super::ULONGLONG,
    pub Size: super::ULONG,
    pub Anonymous: EVENT_DATA_DESCRIPTOR_0,
}
#[cfg(feature = "shared")]
impl Default for EVENT_DATA_DESCRIPTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union EVENT_DATA_DESCRIPTOR_0 {
    pub Reserved: super::ULONG,
    pub Anonymous: EVENT_DATA_DESCRIPTOR_0_0,
}
#[cfg(feature = "shared")]
impl Default for EVENT_DATA_DESCRIPTOR_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EVENT_DATA_DESCRIPTOR_0_0 {
    pub Type: super::UCHAR,
    pub Reserved1: super::UCHAR,
    pub Reserved2: super::USHORT,
}
pub const EVENT_DATA_DESCRIPTOR_TYPE_EVENT_METADATA: i32 = 1;
pub const EVENT_DATA_DESCRIPTOR_TYPE_NONE: i32 = 0;
pub const EVENT_DATA_DESCRIPTOR_TYPE_PROVIDER_METADATA: i32 = 2;
pub const EVENT_DATA_DESCRIPTOR_TYPE_RESERVED1: i32 = 4;
pub const EVENT_DATA_DESCRIPTOR_TYPE_TIMESTAMP_OVERRIDE: i32 = 3;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EVENT_DESCRIPTOR {
    pub Id: super::USHORT,
    pub Version: super::UCHAR,
    pub Channel: super::UCHAR,
    pub Level: super::UCHAR,
    pub Opcode: super::UCHAR,
    pub Task: super::USHORT,
    pub Keyword: super::ULONGLONG,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EVENT_FILTER_DESCRIPTOR {
    pub Ptr: super::ULONGLONG,
    pub Size: super::ULONG,
    pub Type: super::ULONG,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EVENT_FILTER_EVENT_ID {
    pub FilterIn: super::BOOLEAN,
    pub Reserved: super::UCHAR,
    pub Count: super::USHORT,
    pub Events: [super::USHORT; 1],
}
#[cfg(feature = "shared")]
impl Default for EVENT_FILTER_EVENT_ID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EVENT_FILTER_EVENT_NAME {
    pub MatchAnyKeyword: super::ULONGLONG,
    pub MatchAllKeyword: super::ULONGLONG,
    pub Level: super::UCHAR,
    pub FilterIn: super::BOOLEAN,
    pub NameCount: super::USHORT,
    pub Names: [super::UCHAR; 1],
}
#[cfg(feature = "shared")]
impl Default for EVENT_FILTER_EVENT_NAME {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EVENT_FILTER_HEADER {
    pub Id: super::USHORT,
    pub Version: super::UCHAR,
    pub Reserved: [super::UCHAR; 5],
    pub InstanceId: super::ULONGLONG,
    pub Size: super::ULONG,
    pub NextOffset: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for EVENT_FILTER_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EVENT_FILTER_LEVEL_KW {
    pub MatchAnyKeyword: super::ULONGLONG,
    pub MatchAllKeyword: super::ULONGLONG,
    pub Level: super::UCHAR,
    pub FilterIn: super::BOOLEAN,
}
pub const EVENT_FILTER_TYPE_CONTAINER: u32 = 2147516416;
pub const EVENT_FILTER_TYPE_EVENT_ID: u32 = 2147484160;
pub const EVENT_FILTER_TYPE_EVENT_NAME: u32 = 2147484672;
pub const EVENT_FILTER_TYPE_EXECUTABLE_NAME: u32 = 2147483656;
pub const EVENT_FILTER_TYPE_NONE: i32 = 0;
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
pub const EVENT_MAX_LEVEL: i32 = 255;
pub const EVENT_MIN_LEVEL: i32 = 0;
pub const EVENT_WRITE_FLAG_INPRIVATE: i32 = 2;
pub const EVENT_WRITE_FLAG_NO_FAULTING: i32 = 1;
pub const EventProviderBinaryTrackInfo: EVENT_INFO_CLASS = 0;
pub const EventProviderSetReserved1: EVENT_INFO_CLASS = 1;
pub const EventProviderSetReserved2: EVENT_INFO_CLASS = 4;
pub const EventProviderSetTraits: EVENT_INFO_CLASS = 2;
pub const EventProviderUseDescriptorType: EVENT_INFO_CLASS = 3;
pub const MAX_EVENT_DATA_DESCRIPTORS: i32 = 128;
pub const MAX_EVENT_FILTERS_COUNT: i32 = 13;
pub const MAX_EVENT_FILTER_DATA_SIZE: i32 = 1024;
pub const MAX_EVENT_FILTER_EVENT_ID_COUNT: i32 = 64;
pub const MAX_EVENT_FILTER_EVENT_NAME_SIZE: i32 = 4096;
pub const MAX_EVENT_FILTER_PAYLOAD_SIZE: i32 = 4096;
pub const MAX_EVENT_FILTER_PID_COUNT: i32 = 8;
pub const MaxEventInfo: EVENT_INFO_CLASS = 5;
#[cfg(feature = "shared")]
pub type PCEVENT_DESCRIPTOR = *const EVENT_DESCRIPTOR;
#[cfg(all(feature = "guiddef", feature = "shared", feature = "winnt"))]
pub type PENABLECALLBACK = Option<unsafe extern "system" fn(sourceid: super::LPCGUID, isenabled: super::ULONG, level: super::UCHAR, matchanykeyword: super::ULONGLONG, matchallkeyword: super::ULONGLONG, filterdata: PEVENT_FILTER_DESCRIPTOR, callbackcontext: super::PVOID)>;
#[cfg(feature = "shared")]
pub type PEVENT_DATA_DESCRIPTOR = *mut EVENT_DATA_DESCRIPTOR;
#[cfg(feature = "shared")]
pub type PEVENT_DESCRIPTOR = *mut EVENT_DESCRIPTOR;
#[cfg(feature = "shared")]
pub type PEVENT_FILTER_DESCRIPTOR = *mut EVENT_FILTER_DESCRIPTOR;
#[cfg(feature = "shared")]
pub type PEVENT_FILTER_EVENT_ID = *mut EVENT_FILTER_EVENT_ID;
#[cfg(feature = "shared")]
pub type PEVENT_FILTER_EVENT_NAME = *mut EVENT_FILTER_EVENT_NAME;
#[cfg(feature = "shared")]
pub type PEVENT_FILTER_HEADER = *mut EVENT_FILTER_HEADER;
#[cfg(feature = "shared")]
pub type PEVENT_FILTER_LEVEL_KW = *mut EVENT_FILTER_LEVEL_KW;
pub type PREGHANDLE = *mut u64;
pub type REGHANDLE = u64;
