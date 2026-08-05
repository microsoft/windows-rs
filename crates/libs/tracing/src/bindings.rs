windows_link::link!("advapi32.dll" "system" fn EventRegister(providerid : *const GUID, enablecallback : PENABLECALLBACK, callbackcontext : *const core::ffi::c_void, reghandle : *mut u64) -> u32);
windows_link::link!("advapi32.dll" "system" fn EventSetInformation(reghandle : REGHANDLE, informationclass : EVENT_INFO_CLASS, eventinformation : *const core::ffi::c_void, informationlength : u32) -> u32);
windows_link::link!("advapi32.dll" "system" fn EventUnregister(reghandle : REGHANDLE) -> u32);
windows_link::link!("advapi32.dll" "system" fn EventWriteTransfer(reghandle : REGHANDLE, eventdescriptor : *const EVENT_DESCRIPTOR, activityid : *const GUID, relatedactivityid : *const GUID, userdatacount : u32, userdata : *const EVENT_DATA_DESCRIPTOR) -> u32);
pub const EVENT_CONTROL_CODE_DISABLE_PROVIDER: i32 = 0;
pub const EVENT_CONTROL_CODE_ENABLE_PROVIDER: i32 = 1;
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
#[derive(Clone, Copy, Default)]
pub struct EVENT_DATA_DESCRIPTOR_0_0 {
    pub Type: u8,
    pub Reserved1: u8,
    pub Reserved2: u16,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
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
#[derive(Clone, Copy, Default)]
pub struct EVENT_FILTER_DESCRIPTOR {
    pub Ptr: u64,
    pub Size: u32,
    pub Type: u32,
}
pub type EVENT_INFO_CLASS = i32;
pub const EventProviderSetTraits: EVENT_INFO_CLASS = 2;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct GUID {
    pub data1: u32,
    pub data2: u16,
    pub data3: u16,
    pub data4: [u8; 8],
}
pub type PENABLECALLBACK = Option<
    unsafe extern "system" fn(
        sourceid: *const GUID,
        isenabled: u32,
        level: u8,
        matchanykeyword: u64,
        matchallkeyword: u64,
        filterdata: *const EVENT_FILTER_DESCRIPTOR,
        callbackcontext: *mut core::ffi::c_void,
    ),
>;
pub type REGHANDLE = u64;
