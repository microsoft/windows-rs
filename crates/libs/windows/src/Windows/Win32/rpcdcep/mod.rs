#[inline]
pub unsafe fn I_RpcAllocate(size: u32) -> *mut core::ffi::c_void {
    windows_core::link!("rpcrt4.dll" "system" fn I_RpcAllocate(size : u32) -> *mut core::ffi::c_void);
    unsafe { I_RpcAllocate(size) }
}
#[cfg(all(feature = "Win32_rpc", feature = "Win32_rpcdce"))]
#[inline]
pub unsafe fn I_RpcBindingCopy(sourcebinding: super::rpcdce::RPC_BINDING_HANDLE, destinationbinding: *mut super::rpcdce::RPC_BINDING_HANDLE) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn I_RpcBindingCopy(sourcebinding : super::rpcdce::RPC_BINDING_HANDLE, destinationbinding : *mut super::rpcdce::RPC_BINDING_HANDLE) -> super::rpc::RPC_STATUS);
    unsafe { I_RpcBindingCopy(sourcebinding, destinationbinding as _) }
}
#[cfg(all(feature = "Win32_rpc", feature = "Win32_rpcdce"))]
#[inline]
pub unsafe fn I_RpcBindingCreateNP(servername: *const u16, servicename: *const u16, networkoptions: *const u16, binding: *mut super::rpcdce::RPC_BINDING_HANDLE) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn I_RpcBindingCreateNP(servername : *const u16, servicename : *const u16, networkoptions : *const u16, binding : *mut super::rpcdce::RPC_BINDING_HANDLE) -> super::rpc::RPC_STATUS);
    unsafe { I_RpcBindingCreateNP(servername, servicename, networkoptions, binding as _) }
}
#[cfg(all(feature = "Win32_rpc", feature = "Win32_rpcdce"))]
#[inline]
pub unsafe fn I_RpcBindingHandleToAsyncHandle(binding: super::rpcdce::RPC_BINDING_HANDLE, asynchandle: *mut *mut core::ffi::c_void) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn I_RpcBindingHandleToAsyncHandle(binding : super::rpcdce::RPC_BINDING_HANDLE, asynchandle : *mut *mut core::ffi::c_void) -> super::rpc::RPC_STATUS);
    unsafe { I_RpcBindingHandleToAsyncHandle(binding, asynchandle as _) }
}
#[cfg(all(feature = "Win32_rpc", feature = "Win32_rpcdce"))]
#[inline]
pub unsafe fn I_RpcBindingInqDynamicEndpointA(binding: super::rpcdce::RPC_BINDING_HANDLE, dynamicendpoint: *mut super::rpcdce::RPC_CSTR) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn I_RpcBindingInqDynamicEndpointA(binding : super::rpcdce::RPC_BINDING_HANDLE, dynamicendpoint : *mut super::rpcdce::RPC_CSTR) -> super::rpc::RPC_STATUS);
    unsafe { I_RpcBindingInqDynamicEndpointA(binding, dynamicendpoint as _) }
}
#[cfg(all(feature = "Win32_rpc", feature = "Win32_rpcdce"))]
#[inline]
pub unsafe fn I_RpcBindingInqDynamicEndpointW(binding: super::rpcdce::RPC_BINDING_HANDLE, dynamicendpoint: *mut super::rpcdce::RPC_WSTR) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn I_RpcBindingInqDynamicEndpointW(binding : super::rpcdce::RPC_BINDING_HANDLE, dynamicendpoint : *mut super::rpcdce::RPC_WSTR) -> super::rpc::RPC_STATUS);
    unsafe { I_RpcBindingInqDynamicEndpointW(binding, dynamicendpoint as _) }
}
#[cfg(all(feature = "Win32_rpc", feature = "Win32_rpcdce"))]
#[inline]
pub unsafe fn I_RpcBindingInqLocalClientPID(binding: super::rpcdce::RPC_BINDING_HANDLE, pid: *mut u32) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn I_RpcBindingInqLocalClientPID(binding : super::rpcdce::RPC_BINDING_HANDLE, pid : *mut u32) -> super::rpc::RPC_STATUS);
    unsafe { I_RpcBindingInqLocalClientPID(binding, pid as _) }
}
#[cfg(all(feature = "Win32_rpc", feature = "Win32_rpcdce"))]
#[inline]
pub unsafe fn I_RpcBindingInqMarshalledTargetInfo(binding: super::rpcdce::RPC_BINDING_HANDLE, marshalledtargetinfosize: *mut u32, marshalledtargetinfo: *mut super::rpcdce::RPC_CSTR) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn I_RpcBindingInqMarshalledTargetInfo(binding : super::rpcdce::RPC_BINDING_HANDLE, marshalledtargetinfosize : *mut u32, marshalledtargetinfo : *mut super::rpcdce::RPC_CSTR) -> super::rpc::RPC_STATUS);
    unsafe { I_RpcBindingInqMarshalledTargetInfo(binding, marshalledtargetinfosize as _, marshalledtargetinfo as _) }
}
#[cfg(all(feature = "Win32_rpc", feature = "Win32_rpcdce"))]
#[inline]
pub unsafe fn I_RpcBindingInqSecurityContext(binding: super::rpcdce::RPC_BINDING_HANDLE, securitycontexthandle: *mut *mut core::ffi::c_void) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn I_RpcBindingInqSecurityContext(binding : super::rpcdce::RPC_BINDING_HANDLE, securitycontexthandle : *mut *mut core::ffi::c_void) -> super::rpc::RPC_STATUS);
    unsafe { I_RpcBindingInqSecurityContext(binding, securitycontexthandle as _) }
}
#[cfg(all(feature = "Win32_rpc", feature = "Win32_rpcdce"))]
#[inline]
pub unsafe fn I_RpcBindingInqSecurityContextKeyInfo(binding: Option<super::rpcdce::RPC_BINDING_HANDLE>, keyinfo: *mut core::ffi::c_void) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn I_RpcBindingInqSecurityContextKeyInfo(binding : super::rpcdce::RPC_BINDING_HANDLE, keyinfo : *mut core::ffi::c_void) -> super::rpc::RPC_STATUS);
    unsafe { I_RpcBindingInqSecurityContextKeyInfo(binding.unwrap_or(core::mem::zeroed()) as _, keyinfo as _) }
}
#[cfg(all(feature = "Win32_rpc", feature = "Win32_rpcdce"))]
#[inline]
pub unsafe fn I_RpcBindingInqTransportType(binding: super::rpcdce::RPC_BINDING_HANDLE, r#type: *mut u32) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn I_RpcBindingInqTransportType(binding : super::rpcdce::RPC_BINDING_HANDLE, r#type : *mut u32) -> super::rpc::RPC_STATUS);
    unsafe { I_RpcBindingInqTransportType(binding, r#type as _) }
}
#[cfg(all(feature = "Win32_rpc", feature = "Win32_rpcdce"))]
#[inline]
pub unsafe fn I_RpcBindingInqWireIdForSnego(binding: super::rpcdce::RPC_BINDING_HANDLE, wireid: *mut u8) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn I_RpcBindingInqWireIdForSnego(binding : super::rpcdce::RPC_BINDING_HANDLE, wireid : *mut u8) -> super::rpc::RPC_STATUS);
    unsafe { I_RpcBindingInqWireIdForSnego(binding, wireid as _) }
}
#[cfg(all(feature = "Win32_rpc", feature = "Win32_rpcdce"))]
#[inline]
pub unsafe fn I_RpcBindingIsClientLocal(bindinghandle: super::rpcdce::RPC_BINDING_HANDLE, clientlocalflag: *mut u32) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn I_RpcBindingIsClientLocal(bindinghandle : super::rpcdce::RPC_BINDING_HANDLE, clientlocalflag : *mut u32) -> super::rpc::RPC_STATUS);
    unsafe { I_RpcBindingIsClientLocal(bindinghandle, clientlocalflag as _) }
}
#[cfg(all(feature = "Win32_rpc", feature = "Win32_rpcdce"))]
#[inline]
pub unsafe fn I_RpcBindingIsServerLocal(binding: super::rpcdce::RPC_BINDING_HANDLE, serverlocalflag: *mut u32) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn I_RpcBindingIsServerLocal(binding : super::rpcdce::RPC_BINDING_HANDLE, serverlocalflag : *mut u32) -> super::rpc::RPC_STATUS);
    unsafe { I_RpcBindingIsServerLocal(binding, serverlocalflag as _) }
}
#[cfg(all(feature = "Win32_rpc", feature = "Win32_rpcdce"))]
#[inline]
pub unsafe fn I_RpcBindingSetPrivateOption(hbinding: super::rpcdce::RPC_BINDING_HANDLE, option: u32, optionvalue: usize) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn I_RpcBindingSetPrivateOption(hbinding : super::rpcdce::RPC_BINDING_HANDLE, option : u32, optionvalue : usize) -> super::rpc::RPC_STATUS);
    unsafe { I_RpcBindingSetPrivateOption(hbinding, option, optionvalue) }
}
#[cfg(all(feature = "Win32_rpc", feature = "Win32_rpcdce"))]
#[inline]
pub unsafe fn I_RpcBindingToStaticStringBindingW(binding: super::rpcdce::RPC_BINDING_HANDLE, stringbinding: *mut *mut u16) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn I_RpcBindingToStaticStringBindingW(binding : super::rpcdce::RPC_BINDING_HANDLE, stringbinding : *mut *mut u16) -> super::rpc::RPC_STATUS);
    unsafe { I_RpcBindingToStaticStringBindingW(binding, stringbinding as _) }
}
#[inline]
pub unsafe fn I_RpcClearMutex(mutex: I_RPC_MUTEX) {
    windows_core::link!("rpcrt4.dll" "system" fn I_RpcClearMutex(mutex : I_RPC_MUTEX));
    unsafe { I_RpcClearMutex(mutex) }
}
#[inline]
pub unsafe fn I_RpcDeleteMutex(mutex: I_RPC_MUTEX) {
    windows_core::link!("rpcrt4.dll" "system" fn I_RpcDeleteMutex(mutex : I_RPC_MUTEX));
    unsafe { I_RpcDeleteMutex(mutex) }
}
#[inline]
pub unsafe fn I_RpcFree(object: *mut core::ffi::c_void) {
    windows_core::link!("rpcrt4.dll" "system" fn I_RpcFree(object : *mut core::ffi::c_void));
    unsafe { I_RpcFree(object as _) }
}
#[cfg(all(feature = "Win32_rpc", feature = "Win32_rpcdce"))]
#[inline]
pub unsafe fn I_RpcFreeBuffer(message: *mut RPC_MESSAGE) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn I_RpcFreeBuffer(message : *mut RPC_MESSAGE) -> super::rpc::RPC_STATUS);
    unsafe { I_RpcFreeBuffer(message as _) }
}
#[cfg(all(feature = "Win32_rpc", feature = "Win32_rpcdce"))]
#[inline]
pub unsafe fn I_RpcFreePipeBuffer(message: *mut RPC_MESSAGE) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn I_RpcFreePipeBuffer(message : *mut RPC_MESSAGE) -> super::rpc::RPC_STATUS);
    unsafe { I_RpcFreePipeBuffer(message as _) }
}
#[cfg(all(feature = "Win32_rpc", feature = "Win32_rpcdce"))]
#[inline]
pub unsafe fn I_RpcGetBuffer(message: *mut RPC_MESSAGE) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn I_RpcGetBuffer(message : *mut RPC_MESSAGE) -> super::rpc::RPC_STATUS);
    unsafe { I_RpcGetBuffer(message as _) }
}
#[cfg(all(feature = "Win32_rpc", feature = "Win32_rpcdce"))]
#[inline]
pub unsafe fn I_RpcGetBufferWithObject(message: *mut RPC_MESSAGE, objectuuid: *mut windows_core::GUID) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn I_RpcGetBufferWithObject(message : *mut RPC_MESSAGE, objectuuid : *mut windows_core::GUID) -> super::rpc::RPC_STATUS);
    unsafe { I_RpcGetBufferWithObject(message as _, objectuuid as _) }
}
#[cfg(all(feature = "Win32_rpc", feature = "Win32_rpcdce"))]
#[inline]
pub unsafe fn I_RpcGetCurrentCallHandle() -> super::rpcdce::RPC_BINDING_HANDLE {
    windows_core::link!("rpcrt4.dll" "system" fn I_RpcGetCurrentCallHandle() -> super::rpcdce::RPC_BINDING_HANDLE);
    unsafe { I_RpcGetCurrentCallHandle() }
}
#[cfg(feature = "Win32_rpc")]
#[inline]
pub unsafe fn I_RpcGetDefaultSD(ppsecuritydescriptor: *mut *mut core::ffi::c_void) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn I_RpcGetDefaultSD(ppsecuritydescriptor : *mut *mut core::ffi::c_void) -> super::rpc::RPC_STATUS);
    unsafe { I_RpcGetDefaultSD(ppsecuritydescriptor as _) }
}
#[cfg(feature = "Win32_rpc")]
#[inline]
pub unsafe fn I_RpcGetExtendedError() -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn I_RpcGetExtendedError() -> super::rpc::RPC_STATUS);
    unsafe { I_RpcGetExtendedError() }
}
#[cfg(all(feature = "Win32_rpc", feature = "Win32_rpcdce"))]
#[inline]
pub unsafe fn I_RpcIfInqTransferSyntaxes(rpcifhandle: super::rpcdce::RPC_IF_HANDLE, transfersyntaxes: *mut RPC_TRANSFER_SYNTAX, transfersyntaxsize: u32, transfersyntaxcount: *mut u32) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn I_RpcIfInqTransferSyntaxes(rpcifhandle : super::rpcdce::RPC_IF_HANDLE, transfersyntaxes : *mut RPC_TRANSFER_SYNTAX, transfersyntaxsize : u32, transfersyntaxcount : *mut u32) -> super::rpc::RPC_STATUS);
    unsafe { I_RpcIfInqTransferSyntaxes(rpcifhandle, transfersyntaxes as _, transfersyntaxsize, transfersyntaxcount as _) }
}
#[cfg(feature = "Win32_rpc")]
#[inline]
pub unsafe fn I_RpcMapWin32Status(status: super::rpc::RPC_STATUS) -> i32 {
    windows_core::link!("rpcrt4.dll" "system" fn I_RpcMapWin32Status(status : super::rpc::RPC_STATUS) -> i32);
    unsafe { I_RpcMapWin32Status(status) }
}
#[cfg(feature = "Win32_rpc")]
#[inline]
pub unsafe fn I_RpcMgmtEnableDedicatedThreadPool() -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn I_RpcMgmtEnableDedicatedThreadPool() -> super::rpc::RPC_STATUS);
    unsafe { I_RpcMgmtEnableDedicatedThreadPool() }
}
#[cfg(all(feature = "Win32_rpc", feature = "Win32_rpcdce"))]
#[inline]
pub unsafe fn I_RpcNegotiateTransferSyntax(message: *mut RPC_MESSAGE) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn I_RpcNegotiateTransferSyntax(message : *mut RPC_MESSAGE) -> super::rpc::RPC_STATUS);
    unsafe { I_RpcNegotiateTransferSyntax(message as _) }
}
#[cfg(all(feature = "Win32_rpc", feature = "Win32_rpcdce"))]
#[inline]
pub unsafe fn I_RpcNsBindingSetEntryNameA(binding: super::rpcdce::RPC_BINDING_HANDLE, entrynamesyntax: u32, entryname: *const u8) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn I_RpcNsBindingSetEntryNameA(binding : super::rpcdce::RPC_BINDING_HANDLE, entrynamesyntax : u32, entryname : *const u8) -> super::rpc::RPC_STATUS);
    unsafe { I_RpcNsBindingSetEntryNameA(binding, entrynamesyntax, entryname) }
}
#[cfg(all(feature = "Win32_rpc", feature = "Win32_rpcdce"))]
#[inline]
pub unsafe fn I_RpcNsBindingSetEntryNameW(binding: super::rpcdce::RPC_BINDING_HANDLE, entrynamesyntax: u32, entryname: *const u16) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn I_RpcNsBindingSetEntryNameW(binding : super::rpcdce::RPC_BINDING_HANDLE, entrynamesyntax : u32, entryname : *const u16) -> super::rpc::RPC_STATUS);
    unsafe { I_RpcNsBindingSetEntryNameW(binding, entrynamesyntax, entryname) }
}
#[cfg(all(feature = "Win32_rpc", feature = "Win32_rpcdce"))]
#[inline]
pub unsafe fn I_RpcNsInterfaceExported(entrynamesyntax: u32, entryname: *mut u16, rpcinterfaceinformation: *mut RPC_SERVER_INTERFACE) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn I_RpcNsInterfaceExported(entrynamesyntax : u32, entryname : *mut u16, rpcinterfaceinformation : *mut RPC_SERVER_INTERFACE) -> super::rpc::RPC_STATUS);
    unsafe { I_RpcNsInterfaceExported(entrynamesyntax, entryname as _, rpcinterfaceinformation as _) }
}
#[cfg(all(feature = "Win32_rpc", feature = "Win32_rpcdce"))]
#[inline]
pub unsafe fn I_RpcNsInterfaceUnexported(entrynamesyntax: u32, entryname: *mut u16, rpcinterfaceinformation: *mut RPC_SERVER_INTERFACE) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn I_RpcNsInterfaceUnexported(entrynamesyntax : u32, entryname : *mut u16, rpcinterfaceinformation : *mut RPC_SERVER_INTERFACE) -> super::rpc::RPC_STATUS);
    unsafe { I_RpcNsInterfaceUnexported(entrynamesyntax, entryname as _, rpcinterfaceinformation as _) }
}
#[cfg(all(feature = "Win32_rpc", feature = "Win32_rpcdce"))]
#[inline]
pub unsafe fn I_RpcOpenClientProcess(binding: Option<super::rpcdce::RPC_BINDING_HANDLE>, desiredaccess: u32, clientprocess: *mut *mut core::ffi::c_void) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn I_RpcOpenClientProcess(binding : super::rpcdce::RPC_BINDING_HANDLE, desiredaccess : u32, clientprocess : *mut *mut core::ffi::c_void) -> super::rpc::RPC_STATUS);
    unsafe { I_RpcOpenClientProcess(binding.unwrap_or(core::mem::zeroed()) as _, desiredaccess, clientprocess as _) }
}
#[inline]
pub unsafe fn I_RpcPauseExecution(milliseconds: u32) {
    windows_core::link!("rpcrt4.dll" "system" fn I_RpcPauseExecution(milliseconds : u32));
    unsafe { I_RpcPauseExecution(milliseconds) }
}
#[cfg(all(feature = "Win32_rpc", feature = "Win32_rpcdce"))]
#[inline]
pub unsafe fn I_RpcReallocPipeBuffer(message: *const RPC_MESSAGE, newsize: u32) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn I_RpcReallocPipeBuffer(message : *const RPC_MESSAGE, newsize : u32) -> super::rpc::RPC_STATUS);
    unsafe { I_RpcReallocPipeBuffer(message, newsize) }
}
#[cfg(all(feature = "Win32_rpc", feature = "Win32_rpcdce"))]
#[inline]
pub unsafe fn I_RpcReceive(message: *mut RPC_MESSAGE, size: u32) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn I_RpcReceive(message : *mut RPC_MESSAGE, size : u32) -> super::rpc::RPC_STATUS);
    unsafe { I_RpcReceive(message as _, size) }
}
#[cfg(all(feature = "Win32_rpc", feature = "Win32_rpcdce"))]
#[inline]
pub unsafe fn I_RpcRecordCalloutFailure(rpcstatus: super::rpc::RPC_STATUS, calloutstate: *mut RDR_CALLOUT_STATE, dllname: *mut u16) {
    windows_core::link!("rpcrt4.dll" "system" fn I_RpcRecordCalloutFailure(rpcstatus : super::rpc::RPC_STATUS, calloutstate : *mut RDR_CALLOUT_STATE, dllname : *mut u16));
    unsafe { I_RpcRecordCalloutFailure(rpcstatus, calloutstate as _, dllname as _) }
}
#[inline]
pub unsafe fn I_RpcRequestMutex() -> I_RPC_MUTEX {
    windows_core::link!("rpcrt4.dll" "system" fn I_RpcRequestMutex(mutex : *mut I_RPC_MUTEX));
    unsafe {
        let mut result__ = core::mem::zeroed();
        I_RpcRequestMutex(&mut result__);
        result__
    }
}
#[cfg(all(feature = "Win32_rpc", feature = "Win32_rpcdce"))]
#[inline]
pub unsafe fn I_RpcSend(message: *mut RPC_MESSAGE) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn I_RpcSend(message : *mut RPC_MESSAGE) -> super::rpc::RPC_STATUS);
    unsafe { I_RpcSend(message as _) }
}
#[cfg(all(feature = "Win32_rpc", feature = "Win32_rpcdce"))]
#[inline]
pub unsafe fn I_RpcSendReceive(message: *mut RPC_MESSAGE) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn I_RpcSendReceive(message : *mut RPC_MESSAGE) -> super::rpc::RPC_STATUS);
    unsafe { I_RpcSendReceive(message as _) }
}
#[cfg(all(feature = "Win32_rpc", feature = "Win32_rpcdce"))]
#[inline]
pub unsafe fn I_RpcServerCheckClientRestriction(context: super::rpcdce::RPC_BINDING_HANDLE) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn I_RpcServerCheckClientRestriction(context : super::rpcdce::RPC_BINDING_HANDLE) -> super::rpc::RPC_STATUS);
    unsafe { I_RpcServerCheckClientRestriction(context) }
}
#[inline]
pub unsafe fn I_RpcServerDisableExceptionFilter() -> i32 {
    windows_core::link!("rpcrt4.dll" "system" fn I_RpcServerDisableExceptionFilter() -> i32);
    unsafe { I_RpcServerDisableExceptionFilter() }
}
#[cfg(all(feature = "Win32_rpc", feature = "Win32_rpcdce"))]
#[inline]
pub unsafe fn I_RpcServerGetAssociationID(binding: Option<super::rpcdce::RPC_BINDING_HANDLE>, associationid: *mut u32) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn I_RpcServerGetAssociationID(binding : super::rpcdce::RPC_BINDING_HANDLE, associationid : *mut u32) -> super::rpc::RPC_STATUS);
    unsafe { I_RpcServerGetAssociationID(binding.unwrap_or(core::mem::zeroed()) as _, associationid as _) }
}
#[inline]
pub unsafe fn I_RpcServerInqAddressChangeFn() -> RPC_ADDRESS_CHANGE_FN {
    windows_core::link!("rpcrt4.dll" "system" fn I_RpcServerInqAddressChangeFn() -> RPC_ADDRESS_CHANGE_FN);
    unsafe { I_RpcServerInqAddressChangeFn() }
}
#[cfg(all(feature = "Win32_rpc", feature = "Win32_rpcdce"))]
#[inline]
pub unsafe fn I_RpcServerInqLocalConnAddress(binding: super::rpcdce::RPC_BINDING_HANDLE, buffer: *mut core::ffi::c_void, buffersize: *mut u32, addressformat: *mut u32) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn I_RpcServerInqLocalConnAddress(binding : super::rpcdce::RPC_BINDING_HANDLE, buffer : *mut core::ffi::c_void, buffersize : *mut u32, addressformat : *mut u32) -> super::rpc::RPC_STATUS);
    unsafe { I_RpcServerInqLocalConnAddress(binding, buffer as _, buffersize as _, addressformat as _) }
}
#[cfg(all(feature = "Win32_rpc", feature = "Win32_rpcdce"))]
#[inline]
pub unsafe fn I_RpcServerInqRemoteConnAddress(binding: super::rpcdce::RPC_BINDING_HANDLE, buffer: *mut core::ffi::c_void, buffersize: *mut u32, addressformat: *mut u32) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn I_RpcServerInqRemoteConnAddress(binding : super::rpcdce::RPC_BINDING_HANDLE, buffer : *mut core::ffi::c_void, buffersize : *mut u32, addressformat : *mut u32) -> super::rpc::RPC_STATUS);
    unsafe { I_RpcServerInqRemoteConnAddress(binding, buffer as _, buffersize as _, addressformat as _) }
}
#[cfg(feature = "Win32_rpc")]
#[inline]
pub unsafe fn I_RpcServerInqTransportType(r#type: *mut u32) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn I_RpcServerInqTransportType(r#type : *mut u32) -> super::rpc::RPC_STATUS);
    unsafe { I_RpcServerInqTransportType(r#type as _) }
}
#[cfg(feature = "Win32_rpc")]
#[inline]
pub unsafe fn I_RpcServerRegisterForwardFunction(pforwardfunction: RPC_FORWARD_FUNCTION) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn I_RpcServerRegisterForwardFunction(pforwardfunction : RPC_FORWARD_FUNCTION) -> super::rpc::RPC_STATUS);
    unsafe { I_RpcServerRegisterForwardFunction(pforwardfunction) }
}
#[cfg(feature = "Win32_rpc")]
#[inline]
pub unsafe fn I_RpcServerSetAddressChangeFn(paddresschangefn: RPC_ADDRESS_CHANGE_FN) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn I_RpcServerSetAddressChangeFn(paddresschangefn : RPC_ADDRESS_CHANGE_FN) -> super::rpc::RPC_STATUS);
    unsafe { I_RpcServerSetAddressChangeFn(paddresschangefn) }
}
#[cfg(all(feature = "Win32_rpc", feature = "Win32_rpcdce"))]
#[inline]
pub unsafe fn I_RpcServerStartService(protseq: *const u16, endpoint: *const u16, ifspec: super::rpcdce::RPC_IF_HANDLE) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn I_RpcServerStartService(protseq : *const u16, endpoint : *const u16, ifspec : super::rpcdce::RPC_IF_HANDLE) -> super::rpc::RPC_STATUS);
    unsafe { I_RpcServerStartService(protseq, endpoint, ifspec) }
}
#[cfg(all(feature = "Win32_rpc", feature = "Win32_rpcdce"))]
#[inline]
pub unsafe fn I_RpcServerSubscribeForDisconnectNotification(binding: Option<super::rpcdce::RPC_BINDING_HANDLE>, hevent: Option<*const core::ffi::c_void>) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn I_RpcServerSubscribeForDisconnectNotification(binding : super::rpcdce::RPC_BINDING_HANDLE, hevent : *const core::ffi::c_void) -> super::rpc::RPC_STATUS);
    unsafe { I_RpcServerSubscribeForDisconnectNotification(binding.unwrap_or(core::mem::zeroed()) as _, hevent.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "Win32_rpc", feature = "Win32_rpcdce"))]
#[inline]
pub unsafe fn I_RpcServerSubscribeForDisconnectNotification2(binding: Option<super::rpcdce::RPC_BINDING_HANDLE>, hevent: *const core::ffi::c_void, subscriptionid: *mut windows_core::GUID) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn I_RpcServerSubscribeForDisconnectNotification2(binding : super::rpcdce::RPC_BINDING_HANDLE, hevent : *const core::ffi::c_void, subscriptionid : *mut windows_core::GUID) -> super::rpc::RPC_STATUS);
    unsafe { I_RpcServerSubscribeForDisconnectNotification2(binding.unwrap_or(core::mem::zeroed()) as _, hevent, subscriptionid as _) }
}
#[cfg(all(feature = "Win32_rpc", feature = "Win32_rpcdce"))]
#[inline]
pub unsafe fn I_RpcServerUnsubscribeForDisconnectNotification(binding: Option<super::rpcdce::RPC_BINDING_HANDLE>, subscriptionid: windows_core::GUID) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn I_RpcServerUnsubscribeForDisconnectNotification(binding : super::rpcdce::RPC_BINDING_HANDLE, subscriptionid : windows_core::GUID) -> super::rpc::RPC_STATUS);
    unsafe { I_RpcServerUnsubscribeForDisconnectNotification(binding.unwrap_or(core::mem::zeroed()) as _, core::mem::transmute(subscriptionid)) }
}
#[cfg(feature = "Win32_rpc")]
#[inline]
pub unsafe fn I_RpcServerUseProtseq2A(networkaddress: Option<*const u8>, protseq: *const u8, maxcalls: u32, securitydescriptor: Option<*const core::ffi::c_void>, policy: *const core::ffi::c_void) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn I_RpcServerUseProtseq2A(networkaddress : *const u8, protseq : *const u8, maxcalls : u32, securitydescriptor : *const core::ffi::c_void, policy : *const core::ffi::c_void) -> super::rpc::RPC_STATUS);
    unsafe { I_RpcServerUseProtseq2A(networkaddress.unwrap_or(core::mem::zeroed()) as _, protseq, maxcalls, securitydescriptor.unwrap_or(core::mem::zeroed()) as _, policy) }
}
#[cfg(feature = "Win32_rpc")]
#[inline]
pub unsafe fn I_RpcServerUseProtseq2W(networkaddress: Option<*const u16>, protseq: *const u16, maxcalls: u32, securitydescriptor: Option<*const core::ffi::c_void>, policy: *const core::ffi::c_void) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn I_RpcServerUseProtseq2W(networkaddress : *const u16, protseq : *const u16, maxcalls : u32, securitydescriptor : *const core::ffi::c_void, policy : *const core::ffi::c_void) -> super::rpc::RPC_STATUS);
    unsafe { I_RpcServerUseProtseq2W(networkaddress.unwrap_or(core::mem::zeroed()) as _, protseq, maxcalls, securitydescriptor.unwrap_or(core::mem::zeroed()) as _, policy) }
}
#[cfg(feature = "Win32_rpc")]
#[inline]
pub unsafe fn I_RpcServerUseProtseqEp2A(networkaddress: Option<*const u8>, protseq: *const u8, maxcalls: u32, endpoint: *const u8, securitydescriptor: Option<*const core::ffi::c_void>, policy: *const core::ffi::c_void) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn I_RpcServerUseProtseqEp2A(networkaddress : *const u8, protseq : *const u8, maxcalls : u32, endpoint : *const u8, securitydescriptor : *const core::ffi::c_void, policy : *const core::ffi::c_void) -> super::rpc::RPC_STATUS);
    unsafe { I_RpcServerUseProtseqEp2A(networkaddress.unwrap_or(core::mem::zeroed()) as _, protseq, maxcalls, endpoint, securitydescriptor.unwrap_or(core::mem::zeroed()) as _, policy) }
}
#[cfg(feature = "Win32_rpc")]
#[inline]
pub unsafe fn I_RpcServerUseProtseqEp2W(networkaddress: Option<*const u16>, protseq: *const u16, maxcalls: u32, endpoint: *const u16, securitydescriptor: Option<*const core::ffi::c_void>, policy: *const core::ffi::c_void) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn I_RpcServerUseProtseqEp2W(networkaddress : *const u16, protseq : *const u16, maxcalls : u32, endpoint : *const u16, securitydescriptor : *const core::ffi::c_void, policy : *const core::ffi::c_void) -> super::rpc::RPC_STATUS);
    unsafe { I_RpcServerUseProtseqEp2W(networkaddress.unwrap_or(core::mem::zeroed()) as _, protseq, maxcalls, endpoint, securitydescriptor.unwrap_or(core::mem::zeroed()) as _, policy) }
}
#[inline]
pub unsafe fn I_RpcSessionStrictContextHandle() {
    windows_core::link!("rpcrt4.dll" "system" fn I_RpcSessionStrictContextHandle());
    unsafe { I_RpcSessionStrictContextHandle() }
}
#[inline]
pub unsafe fn I_RpcSsDontSerializeContext() {
    windows_core::link!("rpcrt4.dll" "system" fn I_RpcSsDontSerializeContext());
    unsafe { I_RpcSsDontSerializeContext() }
}
#[cfg(feature = "Win32_rpc")]
#[inline]
pub unsafe fn I_RpcSystemHandleTypeSpecificWork(handle: *mut core::ffi::c_void, actualtype: u8, idltype: u8, marshaldirection: LRPC_SYSTEM_HANDLE_MARSHAL_DIRECTION) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn I_RpcSystemHandleTypeSpecificWork(handle : *mut core::ffi::c_void, actualtype : u8, idltype : u8, marshaldirection : LRPC_SYSTEM_HANDLE_MARSHAL_DIRECTION) -> super::rpc::RPC_STATUS);
    unsafe { I_RpcSystemHandleTypeSpecificWork(handle as _, actualtype, idltype, marshaldirection) }
}
#[cfg(feature = "Win32_rpc")]
#[inline]
pub unsafe fn I_RpcTurnOnEEInfoPropagation() -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn I_RpcTurnOnEEInfoPropagation() -> super::rpc::RPC_STATUS);
    unsafe { I_RpcTurnOnEEInfoPropagation() }
}
#[cfg(feature = "Win32_rpc")]
#[inline]
pub unsafe fn I_UuidCreate(uuid: *mut windows_core::GUID) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn I_UuidCreate(uuid : *mut windows_core::GUID) -> super::rpc::RPC_STATUS);
    unsafe { I_UuidCreate(uuid as _) }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct I_RPC_MUTEX(pub *mut core::ffi::c_void);
impl I_RPC_MUTEX {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for I_RPC_MUTEX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_rpc", feature = "Win32_rpcdce"))]
pub type I_RpcFreeCalloutStateFn = Option<unsafe extern "system" fn(calloutstate: *mut RDR_CALLOUT_STATE)>;
#[cfg(all(feature = "Win32_rpc", feature = "Win32_rpcdce"))]
pub type I_RpcPerformCalloutFn = Option<unsafe extern "system" fn(context: *mut core::ffi::c_void, calloutstate: *mut RDR_CALLOUT_STATE, stage: super::rpcdce::RPC_HTTP_REDIRECTOR_STAGE) -> super::rpc::RPC_STATUS>;
#[repr(C)]
#[cfg(all(feature = "Win32_rpc", feature = "Win32_rpcdce"))]
#[derive(Clone, Copy, Debug, Default)]
pub struct I_RpcProxyCallbackInterface {
    pub IsValidMachineFn: I_RpcProxyIsValidMachineFn,
    pub GetClientAddressFn: I_RpcProxyGetClientAddressFn,
    pub GetConnectionTimeoutFn: I_RpcProxyGetConnectionTimeoutFn,
    pub PerformCalloutFn: I_RpcPerformCalloutFn,
    pub FreeCalloutStateFn: I_RpcFreeCalloutStateFn,
    pub GetClientSessionAndResourceUUIDFn: I_RpcProxyGetClientSessionAndResourceUUID,
    pub ProxyFilterIfFn: I_RpcProxyFilterIfFn,
    pub RpcProxyUpdatePerfCounterFn: I_RpcProxyUpdatePerfCounterFn,
    pub RpcProxyUpdatePerfCounterBackendServerFn: I_RpcProxyUpdatePerfCounterBackendServerFn,
}
#[cfg(feature = "Win32_rpc")]
pub type I_RpcProxyFilterIfFn = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, ifuuid: *const windows_core::GUID, ifmajorversion: u16, fallow: *mut i32) -> super::rpc::RPC_STATUS>;
#[cfg(feature = "Win32_rpc")]
pub type I_RpcProxyGetClientAddressFn = Option<unsafe extern "system" fn(context: *mut core::ffi::c_void, buffer: *mut i8, bufferlength: *mut u32) -> super::rpc::RPC_STATUS>;
#[cfg(feature = "Win32_rpc")]
pub type I_RpcProxyGetClientSessionAndResourceUUID = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, sessionidpresent: *mut i32, sessionid: *mut windows_core::GUID, resourceidpresent: *mut i32, resourceid: *mut windows_core::GUID) -> super::rpc::RPC_STATUS>;
#[cfg(feature = "Win32_rpc")]
pub type I_RpcProxyGetConnectionTimeoutFn = Option<unsafe extern "system" fn(connectiontimeout: *mut u32) -> super::rpc::RPC_STATUS>;
#[cfg(feature = "Win32_rpc")]
pub type I_RpcProxyIsValidMachineFn = Option<unsafe extern "system" fn(machine: *const u16, dotmachine: *const u16, portnumber: u32) -> super::rpc::RPC_STATUS>;
pub type I_RpcProxyUpdatePerfCounterBackendServerFn = Option<unsafe extern "system" fn(machinename: *const u16, isconnectevent: i32)>;
pub type I_RpcProxyUpdatePerfCounterFn = Option<unsafe extern "system" fn(counter: RpcPerfCounters, modifytrend: i32, size: u32)>;
pub type LRPC_SYSTEM_HANDLE_MARSHAL_DIRECTION = i32;
pub const MarshalDirectionMarshal: LRPC_SYSTEM_HANDLE_MARSHAL_DIRECTION = 0;
pub const MarshalDirectionUnmarshal: LRPC_SYSTEM_HANDLE_MARSHAL_DIRECTION = 1;
pub const NDR_CUSTOM_OR_DEFAULT_ALLOCATOR: u32 = 268435456;
pub const NDR_DEFAULT_ALLOCATOR: u32 = 536870912;
pub const NT351_INTERFACE_SIZE: u32 = 64;
pub const PROTOCOL_ADDRESS_CHANGE: RPC_ADDRESS_CHANGE_TYPE = 3;
pub const PROTOCOL_LOADED: RPC_ADDRESS_CHANGE_TYPE = 2;
pub const PROTOCOL_NOT_LOADED: RPC_ADDRESS_CHANGE_TYPE = 1;
#[cfg(all(feature = "Win32_rpc", feature = "Win32_rpcdce"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PRPC_CLIENT_INTERFACE(pub *mut RPC_CLIENT_INTERFACE);
#[cfg(all(feature = "Win32_rpc", feature = "Win32_rpcdce"))]
impl PRPC_CLIENT_INTERFACE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "Win32_rpc", feature = "Win32_rpcdce"))]
impl Default for PRPC_CLIENT_INTERFACE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_rpc", feature = "Win32_rpcdce"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PRPC_DISPATCH_TABLE(pub *mut RPC_DISPATCH_TABLE);
#[cfg(all(feature = "Win32_rpc", feature = "Win32_rpcdce"))]
impl PRPC_DISPATCH_TABLE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "Win32_rpc", feature = "Win32_rpcdce"))]
impl Default for PRPC_DISPATCH_TABLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_rpc", feature = "Win32_rpcdce"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PRPC_MESSAGE(pub *mut RPC_MESSAGE);
#[cfg(all(feature = "Win32_rpc", feature = "Win32_rpcdce"))]
impl PRPC_MESSAGE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "Win32_rpc", feature = "Win32_rpcdce"))]
impl Default for PRPC_MESSAGE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PRPC_PROTSEQ_ENDPOINT(pub *mut RPC_PROTSEQ_ENDPOINT);
impl PRPC_PROTSEQ_ENDPOINT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PRPC_PROTSEQ_ENDPOINT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type PRPC_RUNDOWN = Option<unsafe extern "system" fn(associationcontext: *mut core::ffi::c_void)>;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PRPC_SEC_CONTEXT_KEY_INFO(pub *mut RPC_SEC_CONTEXT_KEY_INFO);
impl PRPC_SEC_CONTEXT_KEY_INFO {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PRPC_SEC_CONTEXT_KEY_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_rpc", feature = "Win32_rpcdce"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PRPC_SERVER_INTERFACE(pub *mut RPC_SERVER_INTERFACE);
#[cfg(all(feature = "Win32_rpc", feature = "Win32_rpcdce"))]
impl PRPC_SERVER_INTERFACE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "Win32_rpc", feature = "Win32_rpcdce"))]
impl Default for PRPC_SERVER_INTERFACE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PRPC_SYNTAX_IDENTIFIER(pub *mut RPC_SYNTAX_IDENTIFIER);
impl PRPC_SYNTAX_IDENTIFIER {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PRPC_SYNTAX_IDENTIFIER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_rpc", feature = "Win32_rpcdce"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RDR_CALLOUT_STATE {
    pub LastError: super::rpc::RPC_STATUS,
    pub LastEEInfo: *mut core::ffi::c_void,
    pub LastCalledStage: super::rpcdce::RPC_HTTP_REDIRECTOR_STAGE,
    pub ServerName: *mut u16,
    pub ServerPort: *mut u16,
    pub RemoteUser: *mut u16,
    pub AuthType: *mut u16,
    pub ResourceTypePresent: u8,
    pub SessionIdPresent: u8,
    pub InterfacePresent: u8,
    pub ResourceType: windows_core::GUID,
    pub SessionId: windows_core::GUID,
    pub Interface: RPC_SYNTAX_IDENTIFIER,
    pub CertContext: *mut core::ffi::c_void,
}
#[cfg(all(feature = "Win32_rpc", feature = "Win32_rpcdce"))]
impl Default for RDR_CALLOUT_STATE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const RPCFLG_ACCESSIBILITY_BIT1: u32 = 1048576;
pub const RPCFLG_ACCESSIBILITY_BIT2: u32 = 2097152;
pub const RPCFLG_ACCESS_LOCAL: u32 = 4194304;
pub const RPCFLG_ASYNCHRONOUS: u32 = 1073741824;
pub const RPCFLG_AUTO_COMPLETE: u32 = 134217728;
pub const RPCFLG_HAS_CALLBACK: u32 = 67108864;
pub const RPCFLG_HAS_GUARANTEE: u32 = 16;
pub const RPCFLG_HAS_MULTI_SYNTAXES: u32 = 33554432;
pub const RPCFLG_INPUT_SYNCHRONOUS: u32 = 536870912;
pub const RPCFLG_LOCAL_CALL: u32 = 268435456;
pub const RPCFLG_MESSAGE: u32 = 16777216;
pub const RPCFLG_NDR64_CONTAINS_ARM_LAYOUT: u32 = 67108864;
pub const RPCFLG_NON_NDR: u32 = 2147483648;
pub const RPCFLG_SENDER_WAITING_FOR_REPLY: u32 = 8388608;
pub const RPCFLG_WINRT_REMOTE_ASYNC: u32 = 32;
pub type RPCLT_PDU_FILTER_FUNC = Option<unsafe extern "system" fn(buffer: *mut core::ffi::c_void, bufferlength: u32, fdatagram: i32)>;
pub type RPC_ADDRESS_CHANGE_FN = Option<unsafe extern "system" fn(arg: *mut core::ffi::c_void)>;
pub type RPC_ADDRESS_CHANGE_TYPE = i32;
#[cfg(feature = "Win32_rpc")]
pub type RPC_BLOCKING_FN = Option<unsafe extern "system" fn(hwnd: *mut core::ffi::c_void, context: *mut core::ffi::c_void, hsyncevent: *mut core::ffi::c_void) -> super::rpc::RPC_STATUS>;
pub const RPC_BUFFER_ASYNC: u32 = 32768;
pub const RPC_BUFFER_COMPLETE: u32 = 4096;
pub const RPC_BUFFER_EXTRA: u32 = 16384;
pub const RPC_BUFFER_NONOTIFY: u32 = 65536;
pub const RPC_BUFFER_PARTIAL: u32 = 8192;
#[repr(C)]
#[cfg(all(feature = "Win32_rpc", feature = "Win32_rpcdce"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RPC_CLIENT_INTERFACE {
    pub Length: u32,
    pub InterfaceId: RPC_SYNTAX_IDENTIFIER,
    pub TransferSyntax: RPC_SYNTAX_IDENTIFIER,
    pub DispatchTable: PRPC_DISPATCH_TABLE,
    pub RpcProtseqEndpointCount: u32,
    pub RpcProtseqEndpoint: PRPC_PROTSEQ_ENDPOINT,
    pub Reserved: usize,
    pub InterpreterInfo: *const core::ffi::c_void,
    pub Flags: u32,
}
#[cfg(all(feature = "Win32_rpc", feature = "Win32_rpcdce"))]
impl Default for RPC_CLIENT_INTERFACE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const RPC_CONTEXT_HANDLE_DEFAULT_FLAGS: u32 = 0;
pub const RPC_CONTEXT_HANDLE_DONT_SERIALIZE: u32 = 536870912;
pub const RPC_CONTEXT_HANDLE_FLAGS: u32 = 805306368;
pub const RPC_CONTEXT_HANDLE_SERIALIZE: u32 = 268435456;
pub const RPC_C_OPT_COOKIE_AUTH: u32 = 7;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RPC_C_OPT_COOKIE_AUTH_DESCRIPTOR {
    pub BufferSize: u32,
    pub Buffer: *mut i8,
}
impl Default for RPC_C_OPT_COOKIE_AUTH_DESCRIPTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const RPC_C_OPT_PRIVATE_BREAK_ON_SUSPEND: u32 = 3;
pub const RPC_C_OPT_PRIVATE_DO_NOT_DISTURB: u32 = 2;
pub const RPC_C_OPT_PRIVATE_SUPPRESS_WAKE: u32 = 1;
pub const RPC_C_OPT_RESOURCE_TYPE_UUID: u32 = 8;
pub const RPC_C_OPT_SESSION_ID: u32 = 6;
#[cfg(all(feature = "Win32_rpc", feature = "Win32_rpcdce"))]
pub type RPC_DISPATCH_FUNCTION = Option<unsafe extern "system" fn(message: *mut RPC_MESSAGE)>;
#[repr(C)]
#[cfg(all(feature = "Win32_rpc", feature = "Win32_rpcdce"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RPC_DISPATCH_TABLE {
    pub DispatchTableCount: u32,
    pub DispatchTable: *mut RPC_DISPATCH_FUNCTION,
    pub Reserved: isize,
}
#[cfg(all(feature = "Win32_rpc", feature = "Win32_rpcdce"))]
impl Default for RPC_DISPATCH_TABLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const RPC_FLAGS_VALID_BIT: u32 = 32768;
#[cfg(feature = "Win32_rpc")]
pub type RPC_FORWARD_FUNCTION = Option<unsafe extern "system" fn(interfaceid: *mut windows_core::GUID, interfaceversion: *mut RPC_VERSION, objectid: *mut windows_core::GUID, rpcpro: *mut u8, ppdestendpoint: *mut *mut core::ffi::c_void) -> super::rpc::RPC_STATUS>;
pub const RPC_INTERFACE_HAS_PIPES: u32 = 1;
#[repr(C)]
#[cfg(all(feature = "Win32_rpc", feature = "Win32_rpcdce"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RPC_MESSAGE {
    pub Handle: super::rpcdce::RPC_BINDING_HANDLE,
    pub DataRepresentation: u32,
    pub Buffer: *mut core::ffi::c_void,
    pub BufferLength: u32,
    pub ProcNum: u32,
    pub TransferSyntax: PRPC_SYNTAX_IDENTIFIER,
    pub RpcInterfaceInformation: *mut core::ffi::c_void,
    pub ReservedForRuntime: *mut core::ffi::c_void,
    pub ManagerEpv: *mut core::ffi::c_void,
    pub ImportContext: *mut core::ffi::c_void,
    pub RpcFlags: u32,
}
#[cfg(all(feature = "Win32_rpc", feature = "Win32_rpcdce"))]
impl Default for RPC_MESSAGE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const RPC_NCA_FLAGS_BROADCAST: u32 = 2;
pub const RPC_NCA_FLAGS_DEFAULT: u32 = 0;
pub const RPC_NCA_FLAGS_IDEMPOTENT: u32 = 1;
pub const RPC_NCA_FLAGS_MAYBE: u32 = 4;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RPC_PROTSEQ_ENDPOINT {
    pub RpcProtocolSequence: *mut u8,
    pub Endpoint: *mut u8,
}
impl Default for RPC_PROTSEQ_ENDPOINT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const RPC_PROXY_CONNECTION_TYPE_IN_PROXY: u32 = 0;
pub const RPC_PROXY_CONNECTION_TYPE_OUT_PROXY: u32 = 1;
pub const RPC_P_ADDR_FORMAT_TCP_IPV4: u32 = 1;
pub const RPC_P_ADDR_FORMAT_TCP_IPV6: u32 = 2;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct RPC_SEC_CONTEXT_KEY_INFO {
    pub EncryptAlgorithm: u32,
    pub KeySize: u32,
    pub SignatureAlgorithm: u32,
}
#[repr(C)]
#[cfg(all(feature = "Win32_rpc", feature = "Win32_rpcdce"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RPC_SERVER_INTERFACE {
    pub Length: u32,
    pub InterfaceId: RPC_SYNTAX_IDENTIFIER,
    pub TransferSyntax: RPC_SYNTAX_IDENTIFIER,
    pub DispatchTable: PRPC_DISPATCH_TABLE,
    pub RpcProtseqEndpointCount: u32,
    pub RpcProtseqEndpoint: PRPC_PROTSEQ_ENDPOINT,
    pub DefaultManagerEpv: *mut core::ffi::c_void,
    pub InterpreterInfo: *const core::ffi::c_void,
    pub Flags: u32,
}
#[cfg(all(feature = "Win32_rpc", feature = "Win32_rpcdce"))]
impl Default for RPC_SERVER_INTERFACE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type RPC_SETFILTER_FUNC = Option<unsafe extern "C" fn(pfnfilter: RPCLT_PDU_FILTER_FUNC)>;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct RPC_SYNTAX_IDENTIFIER {
    pub SyntaxGUID: windows_core::GUID,
    pub SyntaxVersion: RPC_VERSION,
}
pub const RPC_SYSTEM_HANDLE_FREE_ALL: u32 = 3;
pub const RPC_SYSTEM_HANDLE_FREE_ERROR_ON_CLOSE: u32 = 4;
pub const RPC_SYSTEM_HANDLE_FREE_RETRIEVED: u32 = 2;
pub const RPC_SYSTEM_HANDLE_FREE_UNRETRIEVED: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct RPC_TRANSFER_SYNTAX {
    pub Uuid: windows_core::GUID,
    pub VersMajor: u16,
    pub VersMinor: u16,
}
pub const RPC_TYPE_DISCONNECT_EVENT_CONTEXT_HANDLE: u32 = 2147483648;
pub const RPC_TYPE_STRICT_CONTEXT_HANDLE: u32 = 1073741824;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct RPC_VERSION {
    pub MajorVersion: u16,
    pub MinorVersion: u16,
}
pub const RpcAttemptedLbsDecisions: RpcPerfCounters = 8;
pub const RpcAttemptedLbsMessages: RpcPerfCounters = 10;
pub const RpcBackEndConnectionAttempts: RpcPerfCounters = 2;
pub const RpcBackEndConnectionFailed: RpcPerfCounters = 3;
pub const RpcCurrentUniqueUser: RpcPerfCounters = 1;
pub const RpcFailedLbsDecisions: RpcPerfCounters = 9;
pub const RpcFailedLbsMessages: RpcPerfCounters = 11;
pub const RpcIncomingBandwidth: RpcPerfCounters = 6;
pub const RpcIncomingConnections: RpcPerfCounters = 5;
pub const RpcLastCounter: RpcPerfCounters = 12;
pub const RpcOutgoingBandwidth: RpcPerfCounters = 7;
pub type RpcPerfCounters = i32;
pub const RpcRequestsPerSecond: RpcPerfCounters = 4;
pub const TRANSPORT_TYPE_CN: u32 = 1;
pub const TRANSPORT_TYPE_DG: u32 = 2;
pub const TRANSPORT_TYPE_LPC: u32 = 4;
pub const TRANSPORT_TYPE_WMSG: u32 = 8;
