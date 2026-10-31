#[inline]
pub unsafe fn CloseThreadWaitChainSession(wcthandle: HWCT) {
    windows_core::link!("advapi32.dll" "system" fn CloseThreadWaitChainSession(wcthandle : HWCT));
    unsafe { CloseThreadWaitChainSession(wcthandle) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn GetThreadWaitChain(wcthandle: HWCT, context: Option<super::DWORD_PTR>, flags: super::DWORD, threadid: super::DWORD, nodecount: super::LPDWORD, nodeinfoarray: PWAITCHAIN_NODE_INFO, iscycle: super::LPBOOL) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn GetThreadWaitChain(wcthandle : HWCT, context : super::DWORD_PTR, flags : super::DWORD, threadid : super::DWORD, nodecount : super::LPDWORD, nodeinfoarray : PWAITCHAIN_NODE_INFO, iscycle : super::LPBOOL) -> windows_core::BOOL);
    unsafe { GetThreadWaitChain(wcthandle, context.unwrap_or(core::mem::zeroed()) as _, flags, threadid, nodecount as _, nodeinfoarray as _, iscycle as _) }
}
#[must_use]
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn OpenThreadWaitChainSession(flags: super::DWORD, callback: PWAITCHAINCALLBACK) -> HWCT {
    windows_core::link!("advapi32.dll" "system" fn OpenThreadWaitChainSession(flags : super::DWORD, callback : PWAITCHAINCALLBACK) -> HWCT);
    unsafe { OpenThreadWaitChainSession(flags, callback) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn RegisterWaitChainCOMCallback(callstatecallback: PCOGETCALLSTATE, activationstatecallback: PCOGETACTIVATIONSTATE) {
    windows_core::link!("advapi32.dll" "system" fn RegisterWaitChainCOMCallback(callstatecallback : PCOGETCALLSTATE, activationstatecallback : PCOGETACTIVATIONSTATE));
    unsafe { RegisterWaitChainCOMCallback(callstatecallback, activationstatecallback) }
}
pub type HWCT = *mut core::ffi::c_void;
#[cfg(feature = "shared")]
pub type PCOGETACTIVATIONSTATE = Option<unsafe extern "C" fn(param0: windows_core::GUID, param1: super::DWORD, param2: *mut super::DWORD) -> windows_core::HRESULT>;
#[cfg(feature = "minwindef")]
pub type PCOGETCALLSTATE = Option<unsafe extern "C" fn(param0: i32, param1: super::PULONG) -> windows_core::HRESULT>;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
pub type PWAITCHAINCALLBACK = Option<unsafe extern "system" fn(wcthandle: HWCT, context: super::DWORD_PTR, callbackstatus: super::DWORD, nodecount: super::LPDWORD, nodeinfoarray: PWAITCHAIN_NODE_INFO, iscycle: super::LPBOOL)>;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PWAITCHAIN_NODE_INFO = *mut WAITCHAIN_NODE_INFO;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct WAITCHAIN_NODE_INFO {
    pub ObjectType: WCT_OBJECT_TYPE,
    pub ObjectStatus: WCT_OBJECT_STATUS,
    pub Anonymous: WAITCHAIN_NODE_INFO_0,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for WAITCHAIN_NODE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub union WAITCHAIN_NODE_INFO_0 {
    pub LockObject: WAITCHAIN_NODE_INFO_0_0,
    pub ThreadObject: WAITCHAIN_NODE_INFO_0_1,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for WAITCHAIN_NODE_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct WAITCHAIN_NODE_INFO_0_0 {
    pub ObjectName: [super::WCHAR; 128],
    pub Timeout: super::LARGE_INTEGER,
    pub Alertable: windows_core::BOOL,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for WAITCHAIN_NODE_INFO_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WAITCHAIN_NODE_INFO_0_1 {
    pub ProcessId: super::DWORD,
    pub ThreadId: super::DWORD,
    pub WaitTime: super::DWORD,
    pub ContextSwitches: super::DWORD,
}
pub const WCTP_GETINFO_ALL_FLAGS: i32 = 7;
pub const WCTP_OPEN_ALL_FLAGS: i32 = 1;
pub const WCT_ASYNC_OPEN_FLAG: i32 = 1;
pub const WCT_MAX_NODE_COUNT: i32 = 16;
pub const WCT_NETWORK_IO_FLAG: i32 = 8;
pub type WCT_OBJECT_STATUS = i32;
pub type WCT_OBJECT_TYPE = i32;
pub const WCT_OBJNAME_LENGTH: i32 = 128;
pub const WCT_OUT_OF_PROC_COM_FLAG: i32 = 2;
pub const WCT_OUT_OF_PROC_CS_FLAG: i32 = 4;
pub const WCT_OUT_OF_PROC_FLAG: i32 = 1;
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
