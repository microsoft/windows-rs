#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_rpc", feature = "Win32_windef", feature = "Win32_winnt"))]
windows_link::link!("rpcrt4.dll" "system" fn I_RpcAsyncAbortCall(pasync : *const RPC_ASYNC_STATE, exceptioncode : u32) -> super::rpc::RPC_STATUS);
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_rpc", feature = "Win32_rpcdce", feature = "Win32_rpcdcep", feature = "Win32_windef", feature = "Win32_winnt"))]
windows_link::link!("rpcrt4.dll" "system" fn I_RpcAsyncSetHandle(message : *const super::rpcdcep::RPC_MESSAGE, pasync : *const RPC_ASYNC_STATE) -> super::rpc::RPC_STATUS);
#[cfg(all(feature = "Win32_rpc", feature = "Win32_rpcdce", feature = "Win32_winnt"))]
windows_link::link!("rpcrt4.dll" "system" fn I_RpcBindingInqClientTokenAttributes(binding : super::rpcdce::RPC_BINDING_HANDLE, tokenid : *mut super::winnt::LUID, authenticationid : *mut super::winnt::LUID, modifiedid : *mut super::winnt::LUID) -> super::rpc::RPC_STATUS);
windows_link::link!("rpcrt4.dll" "system" fn I_RpcExceptionFilter(exceptioncode : u32) -> i32);
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_rpc", feature = "Win32_windef", feature = "Win32_winnt"))]
windows_link::link!("rpcrt4.dll" "system" fn RpcAsyncAbortCall(pasync : *mut RPC_ASYNC_STATE, exceptioncode : u32) -> super::rpc::RPC_STATUS);
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_rpc", feature = "Win32_windef", feature = "Win32_winnt"))]
windows_link::link!("rpcrt4.dll" "system" fn RpcAsyncCancelCall(pasync : *mut RPC_ASYNC_STATE, fabort : windows_sys::core::BOOL) -> super::rpc::RPC_STATUS);
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_rpc", feature = "Win32_windef", feature = "Win32_winnt"))]
windows_link::link!("rpcrt4.dll" "system" fn RpcAsyncCompleteCall(pasync : *mut RPC_ASYNC_STATE, reply : *mut core::ffi::c_void) -> super::rpc::RPC_STATUS);
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_rpc", feature = "Win32_windef", feature = "Win32_winnt"))]
windows_link::link!("rpcrt4.dll" "system" fn RpcAsyncGetCallStatus(pasync : *const RPC_ASYNC_STATE) -> super::rpc::RPC_STATUS);
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_rpc", feature = "Win32_windef", feature = "Win32_winnt"))]
windows_link::link!("rpcrt4.dll" "system" fn RpcAsyncInitializeHandle(pasync : *mut RPC_ASYNC_STATE, size : u32) -> super::rpc::RPC_STATUS);
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_rpc", feature = "Win32_windef", feature = "Win32_winnt"))]
windows_link::link!("rpcrt4.dll" "system" fn RpcAsyncRegisterInfo(pasync : *const RPC_ASYNC_STATE) -> super::rpc::RPC_STATUS);
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_rpc", feature = "Win32_rpcdce", feature = "Win32_windef", feature = "Win32_winnt"))]
windows_link::link!("rpcrt4.dll" "system" fn RpcBindingBind(pasync : *const RPC_ASYNC_STATE, binding : super::rpcdce::RPC_BINDING_HANDLE, ifspec : super::rpcdce::RPC_IF_HANDLE) -> super::rpc::RPC_STATUS);
#[cfg(all(feature = "Win32_rpc", feature = "Win32_rpcdce"))]
windows_link::link!("rpcrt4.dll" "system" fn RpcBindingUnbind(binding : super::rpcdce::RPC_BINDING_HANDLE) -> super::rpc::RPC_STATUS);
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_minwindef", feature = "Win32_rpc"))]
windows_link::link!("rpcrt4.dll" "system" fn RpcErrorAddRecord(errorinfo : *const RPC_EXTENDED_ERROR_INFO) -> super::rpc::RPC_STATUS);
windows_link::link!("rpcrt4.dll" "system" fn RpcErrorClearInformation());
#[cfg(feature = "Win32_rpc")]
windows_link::link!("rpcrt4.dll" "system" fn RpcErrorEndEnumeration(enumhandle : *mut RPC_ERROR_ENUM_HANDLE) -> super::rpc::RPC_STATUS);
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_minwindef", feature = "Win32_rpc"))]
windows_link::link!("rpcrt4.dll" "system" fn RpcErrorGetNextRecord(enumhandle : *const RPC_ERROR_ENUM_HANDLE, copystrings : windows_sys::core::BOOL, errorinfo : *mut RPC_EXTENDED_ERROR_INFO) -> super::rpc::RPC_STATUS);
#[cfg(feature = "Win32_rpc")]
windows_link::link!("rpcrt4.dll" "system" fn RpcErrorGetNumberOfRecords(enumhandle : *const RPC_ERROR_ENUM_HANDLE, records : *mut i32) -> super::rpc::RPC_STATUS);
#[cfg(feature = "Win32_rpc")]
windows_link::link!("rpcrt4.dll" "system" fn RpcErrorLoadErrorInfo(errorblob : *const core::ffi::c_void, blobsize : usize, enumhandle : *mut RPC_ERROR_ENUM_HANDLE) -> super::rpc::RPC_STATUS);
#[cfg(feature = "Win32_rpc")]
windows_link::link!("rpcrt4.dll" "system" fn RpcErrorResetEnumeration(enumhandle : *mut RPC_ERROR_ENUM_HANDLE) -> super::rpc::RPC_STATUS);
#[cfg(feature = "Win32_rpc")]
windows_link::link!("rpcrt4.dll" "system" fn RpcErrorSaveErrorInfo(enumhandle : *const RPC_ERROR_ENUM_HANDLE, errorblob : *mut *mut core::ffi::c_void, blobsize : *mut usize) -> super::rpc::RPC_STATUS);
#[cfg(feature = "Win32_rpc")]
windows_link::link!("rpcrt4.dll" "system" fn RpcErrorStartEnumeration(enumhandle : *mut RPC_ERROR_ENUM_HANDLE) -> super::rpc::RPC_STATUS);
#[cfg(feature = "Win32_rpc")]
windows_link::link!("rpcrt4.dll" "system" fn RpcFreeAuthorizationContext(pauthzclientcontext : *mut *mut core::ffi::c_void) -> super::rpc::RPC_STATUS);
#[cfg(all(feature = "Win32_rpc", feature = "Win32_rpcdce", feature = "Win32_winnt"))]
windows_link::link!("rpcrt4.dll" "system" fn RpcGetAuthorizationContextForClient(clientbinding : super::rpcdce::RPC_BINDING_HANDLE, impersonateonreturn : windows_sys::core::BOOL, reserved1 : *const core::ffi::c_void, pexpirationtime : *const i64, reserved2 : super::winnt::LUID, reserved3 : u32, reserved4 : *const core::ffi::c_void, pauthzclientcontext : *mut *mut core::ffi::c_void) -> super::rpc::RPC_STATUS);
#[cfg(all(feature = "Win32_rpc", feature = "Win32_rpcdce"))]
windows_link::link!("rpcrt4.dll" "system" fn RpcServerInqCallAttributesA(clientbinding : super::rpcdce::RPC_BINDING_HANDLE, rpccallattributes : *mut core::ffi::c_void) -> super::rpc::RPC_STATUS);
#[cfg(all(feature = "Win32_rpc", feature = "Win32_rpcdce"))]
windows_link::link!("rpcrt4.dll" "system" fn RpcServerInqCallAttributesW(clientbinding : super::rpcdce::RPC_BINDING_HANDLE, rpccallattributes : *mut core::ffi::c_void) -> super::rpc::RPC_STATUS);
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_rpc", feature = "Win32_rpcdce", feature = "Win32_windef", feature = "Win32_winnt"))]
windows_link::link!("rpcrt4.dll" "system" fn RpcServerSubscribeForNotification(binding : super::rpcdce::RPC_BINDING_HANDLE, notification : RPC_NOTIFICATIONS, notificationtype : RPC_NOTIFICATION_TYPES, notificationinfo : *const RPC_ASYNC_NOTIFICATION_INFO) -> super::rpc::RPC_STATUS);
#[cfg(all(feature = "Win32_rpc", feature = "Win32_rpcdce"))]
windows_link::link!("rpcrt4.dll" "system" fn RpcServerUnsubscribeForNotification(binding : super::rpcdce::RPC_BINDING_HANDLE, notification : RPC_NOTIFICATIONS, notificationsqueued : *mut u32) -> super::rpc::RPC_STATUS);
#[cfg(all(feature = "Win32_rpc", feature = "Win32_rpcdce"))]
windows_link::link!("rpcrt4.dll" "system" fn RpcSsContextLockExclusive(serverbindinghandle : super::rpcdce::RPC_BINDING_HANDLE, usercontext : *const core::ffi::c_void) -> super::rpc::RPC_STATUS);
#[cfg(all(feature = "Win32_rpc", feature = "Win32_rpcdce"))]
windows_link::link!("rpcrt4.dll" "system" fn RpcSsContextLockShared(serverbindinghandle : super::rpcdce::RPC_BINDING_HANDLE, usercontext : *const core::ffi::c_void) -> super::rpc::RPC_STATUS);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct BinaryParam {
    pub Buffer: *mut core::ffi::c_void,
    pub Size: i16,
}
impl Default for BinaryParam {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const EEInfoGCCOM: u32 = 11;
pub const EEInfoGCFRS: u32 = 12;
pub const EEInfoNextRecordsMissing: u32 = 2;
pub const EEInfoPreviousRecordsMissing: u32 = 1;
pub const EEInfoUseFileTime: u32 = 4;
pub type ExtendedErrorParamTypes = i32;
pub const MaxNumberOfEEInfoParams: u32 = 4;
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_windef", feature = "Win32_winnt"))]
pub type PFN_RPCNOTIFICATION_ROUTINE = Option<unsafe extern "system" fn(pasync: *mut RPC_ASYNC_STATE, context: *mut core::ffi::c_void, event: RPC_ASYNC_EVENT)>;
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_windef", feature = "Win32_winnt"))]
pub type PRPC_ASYNC_NOTIFICATION_INFO = *mut RPC_ASYNC_NOTIFICATION_INFO;
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_windef", feature = "Win32_winnt"))]
pub type PRPC_ASYNC_STATE = *mut RPC_ASYNC_STATE;
pub type PRPC_CALL_LOCAL_ADDRESS_V1 = *mut RPC_CALL_LOCAL_ADDRESS_V1;
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_windef", feature = "Win32_winnt"))]
pub type RPCNOTIFICATION_ROUTINE = Option<unsafe extern "system" fn(pasync: *mut RPC_ASYNC_STATE, context: *mut core::ffi::c_void, event: RPC_ASYNC_EVENT)>;
pub type RPC_ASYNC_EVENT = i32;
#[repr(C)]
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_windef", feature = "Win32_winnt"))]
#[derive(Clone, Copy)]
pub union RPC_ASYNC_NOTIFICATION_INFO {
    pub APC: RPC_ASYNC_NOTIFICATION_INFO_0,
    pub IOC: RPC_ASYNC_NOTIFICATION_INFO_1,
    pub HWND: RPC_ASYNC_NOTIFICATION_INFO_2,
    pub hEvent: super::winnt::HANDLE,
    pub NotificationRoutine: PFN_RPCNOTIFICATION_ROUTINE,
}
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_windef", feature = "Win32_winnt"))]
impl Default for RPC_ASYNC_NOTIFICATION_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_windef", feature = "Win32_winnt"))]
#[derive(Clone, Copy)]
pub struct RPC_ASYNC_NOTIFICATION_INFO_0 {
    pub NotificationRoutine: PFN_RPCNOTIFICATION_ROUTINE,
    pub hThread: super::winnt::HANDLE,
}
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_windef", feature = "Win32_winnt"))]
impl Default for RPC_ASYNC_NOTIFICATION_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_windef", feature = "Win32_winnt"))]
#[derive(Clone, Copy)]
pub struct RPC_ASYNC_NOTIFICATION_INFO_1 {
    pub hIOPort: super::winnt::HANDLE,
    pub dwNumberOfBytesTransferred: u32,
    pub dwCompletionKey: usize,
    pub lpOverlapped: super::minwinbase::LPOVERLAPPED,
}
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_windef", feature = "Win32_winnt"))]
impl Default for RPC_ASYNC_NOTIFICATION_INFO_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_windef", feature = "Win32_winnt"))]
#[derive(Clone, Copy)]
pub struct RPC_ASYNC_NOTIFICATION_INFO_2 {
    pub hWnd: super::windef::HWND,
    pub Msg: u32,
}
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_windef", feature = "Win32_winnt"))]
impl Default for RPC_ASYNC_NOTIFICATION_INFO_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_windef", feature = "Win32_winnt"))]
#[derive(Clone, Copy)]
pub struct RPC_ASYNC_STATE {
    pub Size: u32,
    pub Signature: u32,
    pub Lock: i32,
    pub Flags: u32,
    pub StubInfo: *mut core::ffi::c_void,
    pub UserInfo: *mut core::ffi::c_void,
    pub RuntimeInfo: *mut core::ffi::c_void,
    pub Event: RPC_ASYNC_EVENT,
    pub NotificationType: RPC_NOTIFICATION_TYPES,
    pub u: RPC_ASYNC_NOTIFICATION_INFO,
    pub Reserved: [isize; 4],
}
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_windef", feature = "Win32_winnt"))]
impl Default for RPC_ASYNC_STATE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_winnt")]
pub type RPC_CALL_ATTRIBUTES = RPC_CALL_ATTRIBUTES_V3_A;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct RPC_CALL_ATTRIBUTES_V1_A {
    pub Version: u32,
    pub Flags: u32,
    pub ServerPrincipalNameBufferLength: u32,
    pub ServerPrincipalName: *mut u8,
    pub ClientPrincipalNameBufferLength: u32,
    pub ClientPrincipalName: *mut u8,
    pub AuthenticationLevel: u32,
    pub AuthenticationService: u32,
    pub NullSession: windows_sys::core::BOOL,
}
impl Default for RPC_CALL_ATTRIBUTES_V1_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct RPC_CALL_ATTRIBUTES_V1_W {
    pub Version: u32,
    pub Flags: u32,
    pub ServerPrincipalNameBufferLength: u32,
    pub ServerPrincipalName: *mut u16,
    pub ClientPrincipalNameBufferLength: u32,
    pub ClientPrincipalName: *mut u16,
    pub AuthenticationLevel: u32,
    pub AuthenticationService: u32,
    pub NullSession: windows_sys::core::BOOL,
}
impl Default for RPC_CALL_ATTRIBUTES_V1_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy)]
pub struct RPC_CALL_ATTRIBUTES_V2_A {
    pub Version: u32,
    pub Flags: u32,
    pub ServerPrincipalNameBufferLength: u32,
    pub ServerPrincipalName: *mut u8,
    pub ClientPrincipalNameBufferLength: u32,
    pub ClientPrincipalName: *mut u8,
    pub AuthenticationLevel: u32,
    pub AuthenticationService: u32,
    pub NullSession: windows_sys::core::BOOL,
    pub KernelModeCaller: windows_sys::core::BOOL,
    pub ProtocolSequence: u32,
    pub IsClientLocal: u32,
    pub ClientPID: super::winnt::HANDLE,
    pub CallStatus: u32,
    pub CallType: RpcCallType,
    pub CallLocalAddress: *mut RPC_CALL_LOCAL_ADDRESS_V1,
    pub OpNum: u16,
    pub InterfaceUuid: windows_sys::core::GUID,
}
#[cfg(feature = "Win32_winnt")]
impl Default for RPC_CALL_ATTRIBUTES_V2_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy)]
pub struct RPC_CALL_ATTRIBUTES_V2_W {
    pub Version: u32,
    pub Flags: u32,
    pub ServerPrincipalNameBufferLength: u32,
    pub ServerPrincipalName: *mut u16,
    pub ClientPrincipalNameBufferLength: u32,
    pub ClientPrincipalName: *mut u16,
    pub AuthenticationLevel: u32,
    pub AuthenticationService: u32,
    pub NullSession: windows_sys::core::BOOL,
    pub KernelModeCaller: windows_sys::core::BOOL,
    pub ProtocolSequence: u32,
    pub IsClientLocal: RpcCallClientLocality,
    pub ClientPID: super::winnt::HANDLE,
    pub CallStatus: u32,
    pub CallType: RpcCallType,
    pub CallLocalAddress: *mut RPC_CALL_LOCAL_ADDRESS_V1,
    pub OpNum: u16,
    pub InterfaceUuid: windows_sys::core::GUID,
}
#[cfg(feature = "Win32_winnt")]
impl Default for RPC_CALL_ATTRIBUTES_V2_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy)]
pub struct RPC_CALL_ATTRIBUTES_V3_A {
    pub Version: u32,
    pub Flags: u32,
    pub ServerPrincipalNameBufferLength: u32,
    pub ServerPrincipalName: *mut u8,
    pub ClientPrincipalNameBufferLength: u32,
    pub ClientPrincipalName: *mut u8,
    pub AuthenticationLevel: u32,
    pub AuthenticationService: u32,
    pub NullSession: windows_sys::core::BOOL,
    pub KernelModeCaller: windows_sys::core::BOOL,
    pub ProtocolSequence: u32,
    pub IsClientLocal: u32,
    pub ClientPID: super::winnt::HANDLE,
    pub CallStatus: u32,
    pub CallType: RpcCallType,
    pub CallLocalAddress: *mut RPC_CALL_LOCAL_ADDRESS_V1,
    pub OpNum: u16,
    pub InterfaceUuid: windows_sys::core::GUID,
    pub ClientIdentifierBufferLength: u32,
    pub ClientIdentifier: *mut u8,
}
#[cfg(feature = "Win32_winnt")]
impl Default for RPC_CALL_ATTRIBUTES_V3_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy)]
pub struct RPC_CALL_ATTRIBUTES_V3_W {
    pub Version: u32,
    pub Flags: u32,
    pub ServerPrincipalNameBufferLength: u32,
    pub ServerPrincipalName: *mut u16,
    pub ClientPrincipalNameBufferLength: u32,
    pub ClientPrincipalName: *mut u16,
    pub AuthenticationLevel: u32,
    pub AuthenticationService: u32,
    pub NullSession: windows_sys::core::BOOL,
    pub KernelModeCaller: windows_sys::core::BOOL,
    pub ProtocolSequence: u32,
    pub IsClientLocal: RpcCallClientLocality,
    pub ClientPID: super::winnt::HANDLE,
    pub CallStatus: u32,
    pub CallType: RpcCallType,
    pub CallLocalAddress: *mut RPC_CALL_LOCAL_ADDRESS_V1,
    pub OpNum: u16,
    pub InterfaceUuid: windows_sys::core::GUID,
    pub ClientIdentifierBufferLength: u32,
    pub ClientIdentifier: *mut u8,
}
#[cfg(feature = "Win32_winnt")]
impl Default for RPC_CALL_ATTRIBUTES_V3_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const RPC_CALL_ATTRIBUTES_VERSION: u32 = 3;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct RPC_CALL_LOCAL_ADDRESS_V1 {
    pub Version: u32,
    pub Buffer: *mut core::ffi::c_void,
    pub BufferSize: u32,
    pub AddressFormat: RpcLocalAddressFormat,
}
impl Default for RPC_CALL_LOCAL_ADDRESS_V1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const RPC_CALL_STATUS_CANCELLED: u32 = 1;
pub const RPC_CALL_STATUS_DISCONNECTED: u32 = 2;
pub const RPC_C_INFINITE_TIMEOUT: i32 = -1;
pub const RPC_C_NOTIFY_ON_SEND_COMPLETE: u32 = 1;
pub const RPC_EEINFO_VERSION: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct RPC_EE_INFO_PARAM {
    pub ParameterType: ExtendedErrorParamTypes,
    pub u: RPC_EE_INFO_PARAM_0,
}
impl Default for RPC_EE_INFO_PARAM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union RPC_EE_INFO_PARAM_0 {
    pub AnsiString: windows_sys::core::PSTR,
    pub UnicodeString: windows_sys::core::PWSTR,
    pub LVal: i32,
    pub SVal: i16,
    pub PVal: u64,
    pub BVal: BinaryParam,
}
impl Default for RPC_EE_INFO_PARAM_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct RPC_ERROR_ENUM_HANDLE {
    pub Signature: u32,
    pub CurrentPos: *mut core::ffi::c_void,
    pub Head: *mut core::ffi::c_void,
}
impl Default for RPC_ERROR_ENUM_HANDLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_minwindef"))]
#[derive(Clone, Copy)]
pub struct RPC_EXTENDED_ERROR_INFO {
    pub Version: u32,
    pub ComputerName: windows_sys::core::PWSTR,
    pub ProcessID: u32,
    pub u: RPC_EXTENDED_ERROR_INFO_0,
    pub GeneratingComponent: u32,
    pub Status: u32,
    pub DetectionLocation: u16,
    pub Flags: u16,
    pub NumberOfParameters: i32,
    pub Parameters: [RPC_EE_INFO_PARAM; 4],
}
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_minwindef"))]
impl Default for RPC_EXTENDED_ERROR_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_minwindef"))]
#[derive(Clone, Copy)]
pub union RPC_EXTENDED_ERROR_INFO_0 {
    pub SystemTime: super::minwinbase::SYSTEMTIME,
    pub FileTime: super::minwindef::FILETIME,
}
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_minwindef"))]
impl Default for RPC_EXTENDED_ERROR_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type RPC_NOTIFICATIONS = i32;
pub type RPC_NOTIFICATION_TYPES = i32;
pub const RPC_QUERY_CALL_LOCAL_ADDRESS: u32 = 8;
pub const RPC_QUERY_CLIENT_ID: u32 = 128;
pub const RPC_QUERY_CLIENT_PID: u32 = 16;
pub const RPC_QUERY_CLIENT_PRINCIPAL_NAME: u32 = 4;
pub const RPC_QUERY_IS_CLIENT_LOCAL: u32 = 32;
pub const RPC_QUERY_NO_AUTH_REQUIRED: u32 = 64;
pub const RPC_QUERY_SERVER_PRINCIPAL_NAME: u32 = 2;
pub type RpcCallClientLocality = i32;
pub const RpcCallComplete: RPC_ASYNC_EVENT = 0;
pub type RpcCallType = i32;
pub const RpcClientCancel: RPC_ASYNC_EVENT = 4;
pub const RpcClientDisconnect: RPC_ASYNC_EVENT = 3;
pub type RpcLocalAddressFormat = i32;
pub const RpcNotificationCallCancel: RPC_NOTIFICATIONS = 2;
pub const RpcNotificationCallNone: RPC_NOTIFICATIONS = 0;
pub const RpcNotificationCallStatusChange: u32 = 1;
pub const RpcNotificationClientDisconnect: RPC_NOTIFICATIONS = 1;
pub const RpcNotificationTypeApc: RPC_NOTIFICATION_TYPES = 2;
pub const RpcNotificationTypeCallback: RPC_NOTIFICATION_TYPES = 5;
pub const RpcNotificationTypeEvent: RPC_NOTIFICATION_TYPES = 1;
pub const RpcNotificationTypeHwnd: RPC_NOTIFICATION_TYPES = 4;
pub const RpcNotificationTypeIoc: RPC_NOTIFICATION_TYPES = 3;
pub const RpcNotificationTypeNone: RPC_NOTIFICATION_TYPES = 0;
pub const RpcReceiveComplete: RPC_ASYNC_EVENT = 2;
pub const RpcSendComplete: RPC_ASYNC_EVENT = 1;
pub const eeptAnsiString: ExtendedErrorParamTypes = 1;
pub const eeptBinary: ExtendedErrorParamTypes = 7;
pub const eeptLongVal: ExtendedErrorParamTypes = 3;
pub const eeptNone: ExtendedErrorParamTypes = 6;
pub const eeptPointerVal: ExtendedErrorParamTypes = 5;
pub const eeptShortVal: ExtendedErrorParamTypes = 4;
pub const eeptUnicodeString: ExtendedErrorParamTypes = 2;
pub const rcclClientUnknownLocality: RpcCallClientLocality = 3;
pub const rcclInvalid: RpcCallClientLocality = 0;
pub const rcclLocal: RpcCallClientLocality = 1;
pub const rcclRemote: RpcCallClientLocality = 2;
pub const rctGuaranteed: RpcCallType = 3;
pub const rctInvalid: RpcCallType = 0;
pub const rctNormal: RpcCallType = 1;
pub const rctTraining: RpcCallType = 2;
pub const rlafIPv4: RpcLocalAddressFormat = 1;
pub const rlafIPv6: RpcLocalAddressFormat = 2;
pub const rlafInvalid: RpcLocalAddressFormat = 0;
