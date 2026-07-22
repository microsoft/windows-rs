windows_link::link!("advapi32.dll" "system" fn CloseThreadWaitChainSession(wcthandle : HWCT));
windows_link::link!("advapi32.dll" "system" fn GetThreadWaitChain(wcthandle : HWCT, context : usize, flags : u32, threadid : u32, nodecount : *mut u32, nodeinfoarray : *mut WAITCHAIN_NODE_INFO, iscycle : *mut windows_sys::core::BOOL) -> windows_sys::core::BOOL);
windows_link::link!("advapi32.dll" "system" fn OpenThreadWaitChainSession(flags : u32, callback : PWAITCHAINCALLBACK) -> HWCT);
windows_link::link!("advapi32.dll" "system" fn RegisterWaitChainCOMCallback(callstatecallback : PCOGETCALLSTATE, activationstatecallback : PCOGETACTIVATIONSTATE));
pub type HWCT = *mut core::ffi::c_void;
pub type PCOGETACTIVATIONSTATE = Option<unsafe extern "system" fn(param0: windows_sys::core::GUID, param1: u32, param2: *mut u32) -> windows_sys::core::HRESULT>;
pub type PCOGETCALLSTATE = Option<unsafe extern "system" fn(param0: i32, param1: *mut u32) -> windows_sys::core::HRESULT>;
pub type PWAITCHAINCALLBACK = Option<unsafe extern "system" fn(wcthandle: HWCT, context: usize, callbackstatus: u32, nodecount: *mut u32, nodeinfoarray: *mut WAITCHAIN_NODE_INFO, iscycle: *mut windows_sys::core::BOOL)>;
pub type PWAITCHAIN_NODE_INFO = *mut WAITCHAIN_NODE_INFO;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WAITCHAIN_NODE_INFO {
    pub ObjectType: WCT_OBJECT_TYPE,
    pub ObjectStatus: WCT_OBJECT_STATUS,
    pub Anonymous: WAITCHAIN_NODE_INFO_0,
}
impl Default for WAITCHAIN_NODE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union WAITCHAIN_NODE_INFO_0 {
    pub LockObject: WAITCHAIN_NODE_INFO_0_0,
    pub ThreadObject: WAITCHAIN_NODE_INFO_0_1,
}
impl Default for WAITCHAIN_NODE_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WAITCHAIN_NODE_INFO_0_0 {
    pub ObjectName: [u16; 128],
    pub Timeout: i64,
    pub Alertable: windows_sys::core::BOOL,
}
impl Default for WAITCHAIN_NODE_INFO_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct WAITCHAIN_NODE_INFO_0_1 {
    pub ProcessId: u32,
    pub ThreadId: u32,
    pub WaitTime: u32,
    pub ContextSwitches: u32,
}
pub const WCTP_GETINFO_ALL_FLAGS: u32 = 7;
pub const WCTP_OPEN_ALL_FLAGS: u32 = 1;
pub const WCT_ASYNC_OPEN_FLAG: u32 = 1;
pub const WCT_MAX_NODE_COUNT: u32 = 16;
pub const WCT_NETWORK_IO_FLAG: u32 = 8;
pub type WCT_OBJECT_STATUS = i32;
pub type WCT_OBJECT_TYPE = i32;
pub const WCT_OBJNAME_LENGTH: u32 = 128;
pub const WCT_OUT_OF_PROC_COM_FLAG: u32 = 2;
pub const WCT_OUT_OF_PROC_CS_FLAG: u32 = 4;
pub const WCT_OUT_OF_PROC_FLAG: u32 = 1;
pub const WctAlpcType: WCT_OBJECT_TYPE = 4;
pub const WctComActivationType: WCT_OBJECT_TYPE = 9;
pub const WctComType: WCT_OBJECT_TYPE = 5;
pub const WctCriticalSectionType: WCT_OBJECT_TYPE = 1;
pub const WctMaxType: WCT_OBJECT_TYPE = 13;
pub const WctMutexType: WCT_OBJECT_TYPE = 3;
pub const WctProcessWaitType: WCT_OBJECT_TYPE = 7;
pub const WctSendMessageType: WCT_OBJECT_TYPE = 2;
pub const WctSmbIoType: WCT_OBJECT_TYPE = 12;
pub const WctSocketIoType: WCT_OBJECT_TYPE = 11;
pub const WctStatusAbandoned: WCT_OBJECT_STATUS = 8;
pub const WctStatusBlocked: WCT_OBJECT_STATUS = 3;
pub const WctStatusError: WCT_OBJECT_STATUS = 10;
pub const WctStatusMax: WCT_OBJECT_STATUS = 11;
pub const WctStatusNoAccess: WCT_OBJECT_STATUS = 1;
pub const WctStatusNotOwned: WCT_OBJECT_STATUS = 7;
pub const WctStatusOwned: WCT_OBJECT_STATUS = 6;
pub const WctStatusPidOnly: WCT_OBJECT_STATUS = 4;
pub const WctStatusPidOnlyRpcss: WCT_OBJECT_STATUS = 5;
pub const WctStatusRunning: WCT_OBJECT_STATUS = 2;
pub const WctStatusUnknown: WCT_OBJECT_STATUS = 9;
pub const WctThreadType: WCT_OBJECT_TYPE = 8;
pub const WctThreadWaitType: WCT_OBJECT_TYPE = 6;
pub const WctUnknownType: WCT_OBJECT_TYPE = 10;
