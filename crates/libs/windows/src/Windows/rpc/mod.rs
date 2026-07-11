#[inline]
pub unsafe fn DceErrorInqTextA(rpcstatus: RPC_STATUS, errortext: *mut u8) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn DceErrorInqTextA(rpcstatus : RPC_STATUS, errortext : *mut u8) -> RPC_STATUS);
    unsafe { DceErrorInqTextA(rpcstatus, errortext as _) }
}
#[inline]
pub unsafe fn DceErrorInqTextW(rpcstatus: RPC_STATUS, errortext: *mut u16) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn DceErrorInqTextW(rpcstatus : RPC_STATUS, errortext : *mut u16) -> RPC_STATUS);
    unsafe { DceErrorInqTextW(rpcstatus, errortext as _) }
}
#[inline]
pub unsafe fn I_RpcAllocate(size: u32) -> *mut core::ffi::c_void {
    windows_core::link!("rpcrt4.dll" "system" fn I_RpcAllocate(size : u32) -> *mut core::ffi::c_void);
    unsafe { I_RpcAllocate(size) }
}
#[cfg(all(feature = "minwinbase", feature = "windef", feature = "winnt"))]
#[inline]
pub unsafe fn I_RpcAsyncAbortCall(pasync: *const RPC_ASYNC_STATE, exceptioncode: u32) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn I_RpcAsyncAbortCall(pasync : *const RPC_ASYNC_STATE, exceptioncode : u32) -> RPC_STATUS);
    unsafe { I_RpcAsyncAbortCall(pasync, exceptioncode) }
}
#[cfg(all(feature = "minwinbase", feature = "windef", feature = "winnt"))]
#[inline]
pub unsafe fn I_RpcAsyncSetHandle(message: *const RPC_MESSAGE, pasync: *const RPC_ASYNC_STATE) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn I_RpcAsyncSetHandle(message : *const RPC_MESSAGE, pasync : *const RPC_ASYNC_STATE) -> RPC_STATUS);
    unsafe { I_RpcAsyncSetHandle(message, pasync) }
}
#[inline]
pub unsafe fn I_RpcBindingCopy(sourcebinding: RPC_BINDING_HANDLE, destinationbinding: *mut RPC_BINDING_HANDLE) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn I_RpcBindingCopy(sourcebinding : RPC_BINDING_HANDLE, destinationbinding : *mut RPC_BINDING_HANDLE) -> RPC_STATUS);
    unsafe { I_RpcBindingCopy(sourcebinding, destinationbinding as _) }
}
#[inline]
pub unsafe fn I_RpcBindingCreateNP(servername: *const u16, servicename: *const u16, networkoptions: *const u16, binding: *mut RPC_BINDING_HANDLE) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn I_RpcBindingCreateNP(servername : *const u16, servicename : *const u16, networkoptions : *const u16, binding : *mut RPC_BINDING_HANDLE) -> RPC_STATUS);
    unsafe { I_RpcBindingCreateNP(servername, servicename, networkoptions, binding as _) }
}
#[inline]
pub unsafe fn I_RpcBindingHandleToAsyncHandle(binding: RPC_BINDING_HANDLE, asynchandle: *mut *mut core::ffi::c_void) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn I_RpcBindingHandleToAsyncHandle(binding : RPC_BINDING_HANDLE, asynchandle : *mut *mut core::ffi::c_void) -> RPC_STATUS);
    unsafe { I_RpcBindingHandleToAsyncHandle(binding, asynchandle as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn I_RpcBindingInqClientTokenAttributes(binding: RPC_BINDING_HANDLE, tokenid: Option<*mut super::winnt::LUID>, authenticationid: Option<*mut super::winnt::LUID>, modifiedid: Option<*mut super::winnt::LUID>) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn I_RpcBindingInqClientTokenAttributes(binding : RPC_BINDING_HANDLE, tokenid : *mut super::winnt::LUID, authenticationid : *mut super::winnt::LUID, modifiedid : *mut super::winnt::LUID) -> RPC_STATUS);
    unsafe { I_RpcBindingInqClientTokenAttributes(binding, tokenid.unwrap_or(core::mem::zeroed()) as _, authenticationid.unwrap_or(core::mem::zeroed()) as _, modifiedid.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn I_RpcBindingInqDynamicEndpointA(binding: RPC_BINDING_HANDLE, dynamicendpoint: *mut RPC_CSTR) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn I_RpcBindingInqDynamicEndpointA(binding : RPC_BINDING_HANDLE, dynamicendpoint : *mut RPC_CSTR) -> RPC_STATUS);
    unsafe { I_RpcBindingInqDynamicEndpointA(binding, dynamicendpoint as _) }
}
#[inline]
pub unsafe fn I_RpcBindingInqDynamicEndpointW(binding: RPC_BINDING_HANDLE, dynamicendpoint: *mut RPC_WSTR) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn I_RpcBindingInqDynamicEndpointW(binding : RPC_BINDING_HANDLE, dynamicendpoint : *mut RPC_WSTR) -> RPC_STATUS);
    unsafe { I_RpcBindingInqDynamicEndpointW(binding, dynamicendpoint as _) }
}
#[inline]
pub unsafe fn I_RpcBindingInqLocalClientPID(binding: RPC_BINDING_HANDLE, pid: *mut u32) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn I_RpcBindingInqLocalClientPID(binding : RPC_BINDING_HANDLE, pid : *mut u32) -> RPC_STATUS);
    unsafe { I_RpcBindingInqLocalClientPID(binding, pid as _) }
}
#[inline]
pub unsafe fn I_RpcBindingInqMarshalledTargetInfo(binding: RPC_BINDING_HANDLE, marshalledtargetinfosize: *mut u32, marshalledtargetinfo: *mut RPC_CSTR) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn I_RpcBindingInqMarshalledTargetInfo(binding : RPC_BINDING_HANDLE, marshalledtargetinfosize : *mut u32, marshalledtargetinfo : *mut RPC_CSTR) -> RPC_STATUS);
    unsafe { I_RpcBindingInqMarshalledTargetInfo(binding, marshalledtargetinfosize as _, marshalledtargetinfo as _) }
}
#[inline]
pub unsafe fn I_RpcBindingInqSecurityContext(binding: RPC_BINDING_HANDLE, securitycontexthandle: *mut *mut core::ffi::c_void) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn I_RpcBindingInqSecurityContext(binding : RPC_BINDING_HANDLE, securitycontexthandle : *mut *mut core::ffi::c_void) -> RPC_STATUS);
    unsafe { I_RpcBindingInqSecurityContext(binding, securitycontexthandle as _) }
}
#[inline]
pub unsafe fn I_RpcBindingInqSecurityContextKeyInfo(binding: Option<RPC_BINDING_HANDLE>, keyinfo: *mut core::ffi::c_void) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn I_RpcBindingInqSecurityContextKeyInfo(binding : RPC_BINDING_HANDLE, keyinfo : *mut core::ffi::c_void) -> RPC_STATUS);
    unsafe { I_RpcBindingInqSecurityContextKeyInfo(binding.unwrap_or(core::mem::zeroed()) as _, keyinfo as _) }
}
#[inline]
pub unsafe fn I_RpcBindingInqTransportType(binding: RPC_BINDING_HANDLE, r#type: *mut u32) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn I_RpcBindingInqTransportType(binding : RPC_BINDING_HANDLE, r#type : *mut u32) -> RPC_STATUS);
    unsafe { I_RpcBindingInqTransportType(binding, r#type as _) }
}
#[inline]
pub unsafe fn I_RpcBindingInqWireIdForSnego(binding: RPC_BINDING_HANDLE, wireid: *mut u8) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn I_RpcBindingInqWireIdForSnego(binding : RPC_BINDING_HANDLE, wireid : *mut u8) -> RPC_STATUS);
    unsafe { I_RpcBindingInqWireIdForSnego(binding, wireid as _) }
}
#[inline]
pub unsafe fn I_RpcBindingIsClientLocal(bindinghandle: RPC_BINDING_HANDLE, clientlocalflag: *mut u32) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn I_RpcBindingIsClientLocal(bindinghandle : RPC_BINDING_HANDLE, clientlocalflag : *mut u32) -> RPC_STATUS);
    unsafe { I_RpcBindingIsClientLocal(bindinghandle, clientlocalflag as _) }
}
#[inline]
pub unsafe fn I_RpcBindingIsServerLocal(binding: RPC_BINDING_HANDLE, serverlocalflag: *mut u32) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn I_RpcBindingIsServerLocal(binding : RPC_BINDING_HANDLE, serverlocalflag : *mut u32) -> RPC_STATUS);
    unsafe { I_RpcBindingIsServerLocal(binding, serverlocalflag as _) }
}
#[inline]
pub unsafe fn I_RpcBindingSetPrivateOption(hbinding: RPC_BINDING_HANDLE, option: u32, optionvalue: usize) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn I_RpcBindingSetPrivateOption(hbinding : RPC_BINDING_HANDLE, option : u32, optionvalue : usize) -> RPC_STATUS);
    unsafe { I_RpcBindingSetPrivateOption(hbinding, option, optionvalue) }
}
#[inline]
pub unsafe fn I_RpcBindingToStaticStringBindingW(binding: RPC_BINDING_HANDLE, stringbinding: *mut *mut u16) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn I_RpcBindingToStaticStringBindingW(binding : RPC_BINDING_HANDLE, stringbinding : *mut *mut u16) -> RPC_STATUS);
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
pub unsafe fn I_RpcExceptionFilter(exceptioncode: u32) -> i32 {
    windows_core::link!("rpcrt4.dll" "system" fn I_RpcExceptionFilter(exceptioncode : u32) -> i32);
    unsafe { I_RpcExceptionFilter(exceptioncode) }
}
#[inline]
pub unsafe fn I_RpcFree(object: *mut core::ffi::c_void) {
    windows_core::link!("rpcrt4.dll" "system" fn I_RpcFree(object : *mut core::ffi::c_void));
    unsafe { I_RpcFree(object as _) }
}
#[inline]
pub unsafe fn I_RpcFreeBuffer(message: *mut RPC_MESSAGE) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn I_RpcFreeBuffer(message : *mut RPC_MESSAGE) -> RPC_STATUS);
    unsafe { I_RpcFreeBuffer(message as _) }
}
#[inline]
pub unsafe fn I_RpcFreePipeBuffer(message: *mut RPC_MESSAGE) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn I_RpcFreePipeBuffer(message : *mut RPC_MESSAGE) -> RPC_STATUS);
    unsafe { I_RpcFreePipeBuffer(message as _) }
}
#[inline]
pub unsafe fn I_RpcGetBuffer(message: *mut RPC_MESSAGE) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn I_RpcGetBuffer(message : *mut RPC_MESSAGE) -> RPC_STATUS);
    unsafe { I_RpcGetBuffer(message as _) }
}
#[inline]
pub unsafe fn I_RpcGetBufferWithObject(message: *mut RPC_MESSAGE, objectuuid: *mut windows_core::GUID) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn I_RpcGetBufferWithObject(message : *mut RPC_MESSAGE, objectuuid : *mut windows_core::GUID) -> RPC_STATUS);
    unsafe { I_RpcGetBufferWithObject(message as _, objectuuid as _) }
}
#[inline]
pub unsafe fn I_RpcGetCurrentCallHandle() -> RPC_BINDING_HANDLE {
    windows_core::link!("rpcrt4.dll" "system" fn I_RpcGetCurrentCallHandle() -> RPC_BINDING_HANDLE);
    unsafe { I_RpcGetCurrentCallHandle() }
}
#[inline]
pub unsafe fn I_RpcGetDefaultSD(ppsecuritydescriptor: *mut *mut core::ffi::c_void) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn I_RpcGetDefaultSD(ppsecuritydescriptor : *mut *mut core::ffi::c_void) -> RPC_STATUS);
    unsafe { I_RpcGetDefaultSD(ppsecuritydescriptor as _) }
}
#[inline]
pub unsafe fn I_RpcGetExtendedError() -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn I_RpcGetExtendedError() -> RPC_STATUS);
    unsafe { I_RpcGetExtendedError() }
}
#[inline]
pub unsafe fn I_RpcIfInqTransferSyntaxes(rpcifhandle: RPC_IF_HANDLE, transfersyntaxes: *mut RPC_TRANSFER_SYNTAX, transfersyntaxsize: u32, transfersyntaxcount: *mut u32) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn I_RpcIfInqTransferSyntaxes(rpcifhandle : RPC_IF_HANDLE, transfersyntaxes : *mut RPC_TRANSFER_SYNTAX, transfersyntaxsize : u32, transfersyntaxcount : *mut u32) -> RPC_STATUS);
    unsafe { I_RpcIfInqTransferSyntaxes(rpcifhandle, transfersyntaxes as _, transfersyntaxsize, transfersyntaxcount as _) }
}
#[inline]
pub unsafe fn I_RpcMapWin32Status(status: RPC_STATUS) -> i32 {
    windows_core::link!("rpcrt4.dll" "system" fn I_RpcMapWin32Status(status : RPC_STATUS) -> i32);
    unsafe { I_RpcMapWin32Status(status) }
}
#[inline]
pub unsafe fn I_RpcMgmtEnableDedicatedThreadPool() -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn I_RpcMgmtEnableDedicatedThreadPool() -> RPC_STATUS);
    unsafe { I_RpcMgmtEnableDedicatedThreadPool() }
}
#[inline]
pub unsafe fn I_RpcNegotiateTransferSyntax(message: *mut RPC_MESSAGE) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn I_RpcNegotiateTransferSyntax(message : *mut RPC_MESSAGE) -> RPC_STATUS);
    unsafe { I_RpcNegotiateTransferSyntax(message as _) }
}
#[inline]
pub unsafe fn I_RpcNsBindingSetEntryNameA(binding: RPC_BINDING_HANDLE, entrynamesyntax: u32, entryname: *const u8) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn I_RpcNsBindingSetEntryNameA(binding : RPC_BINDING_HANDLE, entrynamesyntax : u32, entryname : *const u8) -> RPC_STATUS);
    unsafe { I_RpcNsBindingSetEntryNameA(binding, entrynamesyntax, entryname) }
}
#[inline]
pub unsafe fn I_RpcNsBindingSetEntryNameW(binding: RPC_BINDING_HANDLE, entrynamesyntax: u32, entryname: *const u16) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn I_RpcNsBindingSetEntryNameW(binding : RPC_BINDING_HANDLE, entrynamesyntax : u32, entryname : *const u16) -> RPC_STATUS);
    unsafe { I_RpcNsBindingSetEntryNameW(binding, entrynamesyntax, entryname) }
}
#[inline]
pub unsafe fn I_RpcNsGetBuffer(message: *mut RPC_MESSAGE) -> RPC_STATUS {
    windows_core::link!("rpcns4.dll" "system" fn I_RpcNsGetBuffer(message : *mut RPC_MESSAGE) -> RPC_STATUS);
    unsafe { I_RpcNsGetBuffer(message as _) }
}
#[inline]
pub unsafe fn I_RpcNsInterfaceExported(entrynamesyntax: u32, entryname: *mut u16, rpcinterfaceinformation: *mut RPC_SERVER_INTERFACE) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn I_RpcNsInterfaceExported(entrynamesyntax : u32, entryname : *mut u16, rpcinterfaceinformation : *mut RPC_SERVER_INTERFACE) -> RPC_STATUS);
    unsafe { I_RpcNsInterfaceExported(entrynamesyntax, entryname as _, rpcinterfaceinformation as _) }
}
#[inline]
pub unsafe fn I_RpcNsInterfaceUnexported(entrynamesyntax: u32, entryname: *mut u16, rpcinterfaceinformation: *mut RPC_SERVER_INTERFACE) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn I_RpcNsInterfaceUnexported(entrynamesyntax : u32, entryname : *mut u16, rpcinterfaceinformation : *mut RPC_SERVER_INTERFACE) -> RPC_STATUS);
    unsafe { I_RpcNsInterfaceUnexported(entrynamesyntax, entryname as _, rpcinterfaceinformation as _) }
}
#[inline]
pub unsafe fn I_RpcNsRaiseException(message: *mut RPC_MESSAGE, status: RPC_STATUS) {
    windows_core::link!("rpcns4.dll" "system" fn I_RpcNsRaiseException(message : *mut RPC_MESSAGE, status : RPC_STATUS));
    unsafe { I_RpcNsRaiseException(message as _, status) }
}
#[inline]
pub unsafe fn I_RpcNsSendReceive(message: *mut RPC_MESSAGE, handle: *mut RPC_BINDING_HANDLE) -> RPC_STATUS {
    windows_core::link!("rpcns4.dll" "system" fn I_RpcNsSendReceive(message : *mut RPC_MESSAGE, handle : *mut RPC_BINDING_HANDLE) -> RPC_STATUS);
    unsafe { I_RpcNsSendReceive(message as _, handle as _) }
}
#[inline]
pub unsafe fn I_RpcOpenClientProcess(binding: Option<RPC_BINDING_HANDLE>, desiredaccess: u32, clientprocess: *mut *mut core::ffi::c_void) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn I_RpcOpenClientProcess(binding : RPC_BINDING_HANDLE, desiredaccess : u32, clientprocess : *mut *mut core::ffi::c_void) -> RPC_STATUS);
    unsafe { I_RpcOpenClientProcess(binding.unwrap_or(core::mem::zeroed()) as _, desiredaccess, clientprocess as _) }
}
#[inline]
pub unsafe fn I_RpcPauseExecution(milliseconds: u32) {
    windows_core::link!("rpcrt4.dll" "system" fn I_RpcPauseExecution(milliseconds : u32));
    unsafe { I_RpcPauseExecution(milliseconds) }
}
#[inline]
pub unsafe fn I_RpcReBindBuffer(message: *mut RPC_MESSAGE) -> RPC_STATUS {
    windows_core::link!("rpcns4.dll" "system" fn I_RpcReBindBuffer(message : *mut RPC_MESSAGE) -> RPC_STATUS);
    unsafe { I_RpcReBindBuffer(message as _) }
}
#[inline]
pub unsafe fn I_RpcReallocPipeBuffer(message: *const RPC_MESSAGE, newsize: u32) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn I_RpcReallocPipeBuffer(message : *const RPC_MESSAGE, newsize : u32) -> RPC_STATUS);
    unsafe { I_RpcReallocPipeBuffer(message, newsize) }
}
#[inline]
pub unsafe fn I_RpcReceive(message: *mut RPC_MESSAGE, size: u32) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn I_RpcReceive(message : *mut RPC_MESSAGE, size : u32) -> RPC_STATUS);
    unsafe { I_RpcReceive(message as _, size) }
}
#[inline]
pub unsafe fn I_RpcRecordCalloutFailure(rpcstatus: RPC_STATUS, calloutstate: *mut RDR_CALLOUT_STATE, dllname: *mut u16) {
    windows_core::link!("rpcrt4.dll" "system" fn I_RpcRecordCalloutFailure(rpcstatus : RPC_STATUS, calloutstate : *mut RDR_CALLOUT_STATE, dllname : *mut u16));
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
#[inline]
pub unsafe fn I_RpcSend(message: *mut RPC_MESSAGE) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn I_RpcSend(message : *mut RPC_MESSAGE) -> RPC_STATUS);
    unsafe { I_RpcSend(message as _) }
}
#[inline]
pub unsafe fn I_RpcSendReceive(message: *mut RPC_MESSAGE) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn I_RpcSendReceive(message : *mut RPC_MESSAGE) -> RPC_STATUS);
    unsafe { I_RpcSendReceive(message as _) }
}
#[inline]
pub unsafe fn I_RpcServerCheckClientRestriction(context: RPC_BINDING_HANDLE) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn I_RpcServerCheckClientRestriction(context : RPC_BINDING_HANDLE) -> RPC_STATUS);
    unsafe { I_RpcServerCheckClientRestriction(context) }
}
#[inline]
pub unsafe fn I_RpcServerDisableExceptionFilter() -> i32 {
    windows_core::link!("rpcrt4.dll" "system" fn I_RpcServerDisableExceptionFilter() -> i32);
    unsafe { I_RpcServerDisableExceptionFilter() }
}
#[inline]
pub unsafe fn I_RpcServerGetAssociationID(binding: Option<RPC_BINDING_HANDLE>, associationid: *mut u32) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn I_RpcServerGetAssociationID(binding : RPC_BINDING_HANDLE, associationid : *mut u32) -> RPC_STATUS);
    unsafe { I_RpcServerGetAssociationID(binding.unwrap_or(core::mem::zeroed()) as _, associationid as _) }
}
#[inline]
pub unsafe fn I_RpcServerInqAddressChangeFn() -> RPC_ADDRESS_CHANGE_FN {
    windows_core::link!("rpcrt4.dll" "system" fn I_RpcServerInqAddressChangeFn() -> RPC_ADDRESS_CHANGE_FN);
    unsafe { I_RpcServerInqAddressChangeFn() }
}
#[inline]
pub unsafe fn I_RpcServerInqLocalConnAddress(binding: RPC_BINDING_HANDLE, buffer: *mut core::ffi::c_void, buffersize: *mut u32, addressformat: *mut u32) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn I_RpcServerInqLocalConnAddress(binding : RPC_BINDING_HANDLE, buffer : *mut core::ffi::c_void, buffersize : *mut u32, addressformat : *mut u32) -> RPC_STATUS);
    unsafe { I_RpcServerInqLocalConnAddress(binding, buffer as _, buffersize as _, addressformat as _) }
}
#[inline]
pub unsafe fn I_RpcServerInqRemoteConnAddress(binding: RPC_BINDING_HANDLE, buffer: *mut core::ffi::c_void, buffersize: *mut u32, addressformat: *mut u32) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn I_RpcServerInqRemoteConnAddress(binding : RPC_BINDING_HANDLE, buffer : *mut core::ffi::c_void, buffersize : *mut u32, addressformat : *mut u32) -> RPC_STATUS);
    unsafe { I_RpcServerInqRemoteConnAddress(binding, buffer as _, buffersize as _, addressformat as _) }
}
#[inline]
pub unsafe fn I_RpcServerInqTransportType(r#type: *mut u32) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn I_RpcServerInqTransportType(r#type : *mut u32) -> RPC_STATUS);
    unsafe { I_RpcServerInqTransportType(r#type as _) }
}
#[inline]
pub unsafe fn I_RpcServerRegisterForwardFunction(pforwardfunction: RPC_FORWARD_FUNCTION) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn I_RpcServerRegisterForwardFunction(pforwardfunction : RPC_FORWARD_FUNCTION) -> RPC_STATUS);
    unsafe { I_RpcServerRegisterForwardFunction(pforwardfunction) }
}
#[inline]
pub unsafe fn I_RpcServerSetAddressChangeFn(paddresschangefn: RPC_ADDRESS_CHANGE_FN) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn I_RpcServerSetAddressChangeFn(paddresschangefn : RPC_ADDRESS_CHANGE_FN) -> RPC_STATUS);
    unsafe { I_RpcServerSetAddressChangeFn(paddresschangefn) }
}
#[inline]
pub unsafe fn I_RpcServerStartService(protseq: *const u16, endpoint: *const u16, ifspec: RPC_IF_HANDLE) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn I_RpcServerStartService(protseq : *const u16, endpoint : *const u16, ifspec : RPC_IF_HANDLE) -> RPC_STATUS);
    unsafe { I_RpcServerStartService(protseq, endpoint, ifspec) }
}
#[inline]
pub unsafe fn I_RpcServerSubscribeForDisconnectNotification(binding: Option<RPC_BINDING_HANDLE>, hevent: Option<*const core::ffi::c_void>) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn I_RpcServerSubscribeForDisconnectNotification(binding : RPC_BINDING_HANDLE, hevent : *const core::ffi::c_void) -> RPC_STATUS);
    unsafe { I_RpcServerSubscribeForDisconnectNotification(binding.unwrap_or(core::mem::zeroed()) as _, hevent.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn I_RpcServerSubscribeForDisconnectNotification2(binding: Option<RPC_BINDING_HANDLE>, hevent: *const core::ffi::c_void, subscriptionid: *mut windows_core::GUID) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn I_RpcServerSubscribeForDisconnectNotification2(binding : RPC_BINDING_HANDLE, hevent : *const core::ffi::c_void, subscriptionid : *mut windows_core::GUID) -> RPC_STATUS);
    unsafe { I_RpcServerSubscribeForDisconnectNotification2(binding.unwrap_or(core::mem::zeroed()) as _, hevent, subscriptionid as _) }
}
#[inline]
pub unsafe fn I_RpcServerUnsubscribeForDisconnectNotification(binding: Option<RPC_BINDING_HANDLE>, subscriptionid: windows_core::GUID) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn I_RpcServerUnsubscribeForDisconnectNotification(binding : RPC_BINDING_HANDLE, subscriptionid : windows_core::GUID) -> RPC_STATUS);
    unsafe { I_RpcServerUnsubscribeForDisconnectNotification(binding.unwrap_or(core::mem::zeroed()) as _, core::mem::transmute(subscriptionid)) }
}
#[inline]
pub unsafe fn I_RpcServerUseProtseq2A(networkaddress: Option<*const u8>, protseq: *const u8, maxcalls: u32, securitydescriptor: Option<*const core::ffi::c_void>, policy: *const core::ffi::c_void) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn I_RpcServerUseProtseq2A(networkaddress : *const u8, protseq : *const u8, maxcalls : u32, securitydescriptor : *const core::ffi::c_void, policy : *const core::ffi::c_void) -> RPC_STATUS);
    unsafe { I_RpcServerUseProtseq2A(networkaddress.unwrap_or(core::mem::zeroed()) as _, protseq, maxcalls, securitydescriptor.unwrap_or(core::mem::zeroed()) as _, policy) }
}
#[inline]
pub unsafe fn I_RpcServerUseProtseq2W(networkaddress: Option<*const u16>, protseq: *const u16, maxcalls: u32, securitydescriptor: Option<*const core::ffi::c_void>, policy: *const core::ffi::c_void) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn I_RpcServerUseProtseq2W(networkaddress : *const u16, protseq : *const u16, maxcalls : u32, securitydescriptor : *const core::ffi::c_void, policy : *const core::ffi::c_void) -> RPC_STATUS);
    unsafe { I_RpcServerUseProtseq2W(networkaddress.unwrap_or(core::mem::zeroed()) as _, protseq, maxcalls, securitydescriptor.unwrap_or(core::mem::zeroed()) as _, policy) }
}
#[inline]
pub unsafe fn I_RpcServerUseProtseqEp2A(networkaddress: Option<*const u8>, protseq: *const u8, maxcalls: u32, endpoint: *const u8, securitydescriptor: Option<*const core::ffi::c_void>, policy: *const core::ffi::c_void) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn I_RpcServerUseProtseqEp2A(networkaddress : *const u8, protseq : *const u8, maxcalls : u32, endpoint : *const u8, securitydescriptor : *const core::ffi::c_void, policy : *const core::ffi::c_void) -> RPC_STATUS);
    unsafe { I_RpcServerUseProtseqEp2A(networkaddress.unwrap_or(core::mem::zeroed()) as _, protseq, maxcalls, endpoint, securitydescriptor.unwrap_or(core::mem::zeroed()) as _, policy) }
}
#[inline]
pub unsafe fn I_RpcServerUseProtseqEp2W(networkaddress: Option<*const u16>, protseq: *const u16, maxcalls: u32, endpoint: *const u16, securitydescriptor: Option<*const core::ffi::c_void>, policy: *const core::ffi::c_void) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn I_RpcServerUseProtseqEp2W(networkaddress : *const u16, protseq : *const u16, maxcalls : u32, endpoint : *const u16, securitydescriptor : *const core::ffi::c_void, policy : *const core::ffi::c_void) -> RPC_STATUS);
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
#[inline]
pub unsafe fn I_RpcSystemHandleTypeSpecificWork(handle: *mut core::ffi::c_void, actualtype: u8, idltype: u8, marshaldirection: LRPC_SYSTEM_HANDLE_MARSHAL_DIRECTION) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn I_RpcSystemHandleTypeSpecificWork(handle : *mut core::ffi::c_void, actualtype : u8, idltype : u8, marshaldirection : LRPC_SYSTEM_HANDLE_MARSHAL_DIRECTION) -> RPC_STATUS);
    unsafe { I_RpcSystemHandleTypeSpecificWork(handle as _, actualtype, idltype, marshaldirection) }
}
#[inline]
pub unsafe fn I_RpcTurnOnEEInfoPropagation() -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn I_RpcTurnOnEEInfoPropagation() -> RPC_STATUS);
    unsafe { I_RpcTurnOnEEInfoPropagation() }
}
#[inline]
pub unsafe fn I_UuidCreate(uuid: *mut windows_core::GUID) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn I_UuidCreate(uuid : *mut windows_core::GUID) -> RPC_STATUS);
    unsafe { I_UuidCreate(uuid as _) }
}
#[inline]
pub unsafe fn NDRCContextBinding(ccontext: NDR_CCONTEXT) -> RPC_BINDING_HANDLE {
    windows_core::link!("rpcrt4.dll" "system" fn NDRCContextBinding(ccontext : NDR_CCONTEXT) -> RPC_BINDING_HANDLE);
    unsafe { NDRCContextBinding(ccontext) }
}
#[inline]
pub unsafe fn NDRCContextMarshall(ccontext: Option<NDR_CCONTEXT>, pbuff: *mut core::ffi::c_void) {
    windows_core::link!("rpcrt4.dll" "system" fn NDRCContextMarshall(ccontext : NDR_CCONTEXT, pbuff : *mut core::ffi::c_void));
    unsafe { NDRCContextMarshall(ccontext.unwrap_or(core::mem::zeroed()) as _, pbuff as _) }
}
#[inline]
pub unsafe fn NDRCContextUnmarshall(pccontext: Option<*mut NDR_CCONTEXT>, hbinding: RPC_BINDING_HANDLE, pbuff: *const core::ffi::c_void, datarepresentation: u32) {
    windows_core::link!("rpcrt4.dll" "system" fn NDRCContextUnmarshall(pccontext : *mut NDR_CCONTEXT, hbinding : RPC_BINDING_HANDLE, pbuff : *const core::ffi::c_void, datarepresentation : u32));
    unsafe { NDRCContextUnmarshall(pccontext.unwrap_or(core::mem::zeroed()) as _, hbinding, pbuff, datarepresentation) }
}
#[inline]
pub unsafe fn NDRSContextMarshall(ccontext: *const _NDR_SCONTEXT, pbuff: *mut core::ffi::c_void, userrundownin: NDR_RUNDOWN) {
    windows_core::link!("rpcrt4.dll" "system" fn NDRSContextMarshall(ccontext : *const _NDR_SCONTEXT, pbuff : *mut core::ffi::c_void, userrundownin : NDR_RUNDOWN));
    unsafe { NDRSContextMarshall(ccontext, pbuff as _, userrundownin) }
}
#[inline]
pub unsafe fn NDRSContextMarshall2(bindinghandle: RPC_BINDING_HANDLE, ccontext: *const _NDR_SCONTEXT, pbuff: *mut core::ffi::c_void, userrundownin: NDR_RUNDOWN, ctxguard: Option<*const core::ffi::c_void>, flags: u32) {
    windows_core::link!("rpcrt4.dll" "system" fn NDRSContextMarshall2(bindinghandle : RPC_BINDING_HANDLE, ccontext : *const _NDR_SCONTEXT, pbuff : *mut core::ffi::c_void, userrundownin : NDR_RUNDOWN, ctxguard : *const core::ffi::c_void, flags : u32));
    unsafe { NDRSContextMarshall2(bindinghandle, ccontext, pbuff as _, userrundownin, ctxguard.unwrap_or(core::mem::zeroed()) as _, flags) }
}
#[inline]
pub unsafe fn NDRSContextMarshallEx(bindinghandle: RPC_BINDING_HANDLE, ccontext: *const _NDR_SCONTEXT, pbuff: *mut core::ffi::c_void, userrundownin: NDR_RUNDOWN) {
    windows_core::link!("rpcrt4.dll" "system" fn NDRSContextMarshallEx(bindinghandle : RPC_BINDING_HANDLE, ccontext : *const _NDR_SCONTEXT, pbuff : *mut core::ffi::c_void, userrundownin : NDR_RUNDOWN));
    unsafe { NDRSContextMarshallEx(bindinghandle, ccontext, pbuff as _, userrundownin) }
}
#[inline]
pub unsafe fn NDRSContextUnmarshall(pbuff: *const core::ffi::c_void, datarepresentation: u32) -> NDR_SCONTEXT {
    windows_core::link!("rpcrt4.dll" "system" fn NDRSContextUnmarshall(pbuff : *const core::ffi::c_void, datarepresentation : u32) -> NDR_SCONTEXT);
    unsafe { NDRSContextUnmarshall(pbuff, datarepresentation) }
}
#[inline]
pub unsafe fn NDRSContextUnmarshall2(bindinghandle: RPC_BINDING_HANDLE, pbuff: Option<*const core::ffi::c_void>, datarepresentation: u32, ctxguard: Option<*const core::ffi::c_void>, flags: u32) -> NDR_SCONTEXT {
    windows_core::link!("rpcrt4.dll" "system" fn NDRSContextUnmarshall2(bindinghandle : RPC_BINDING_HANDLE, pbuff : *const core::ffi::c_void, datarepresentation : u32, ctxguard : *const core::ffi::c_void, flags : u32) -> NDR_SCONTEXT);
    unsafe { NDRSContextUnmarshall2(bindinghandle, pbuff.unwrap_or(core::mem::zeroed()) as _, datarepresentation, ctxguard.unwrap_or(core::mem::zeroed()) as _, flags) }
}
#[inline]
pub unsafe fn NDRSContextUnmarshallEx(bindinghandle: RPC_BINDING_HANDLE, pbuff: *const core::ffi::c_void, datarepresentation: u32) -> NDR_SCONTEXT {
    windows_core::link!("rpcrt4.dll" "system" fn NDRSContextUnmarshallEx(bindinghandle : RPC_BINDING_HANDLE, pbuff : *const core::ffi::c_void, datarepresentation : u32) -> NDR_SCONTEXT);
    unsafe { NDRSContextUnmarshallEx(bindinghandle, pbuff, datarepresentation) }
}
#[cfg(feature = "objidlbase")]
#[inline]
pub unsafe fn Ndr64AsyncClientCall(pproxyinfo: *mut MIDL_STUBLESS_PROXY_INFO, nprocnum: u32, preturnvalue: *mut core::ffi::c_void) -> CLIENT_CALL_RETURN {
    windows_core::link!("rpcrt4.dll" "C" fn Ndr64AsyncClientCall(pproxyinfo : *mut MIDL_STUBLESS_PROXY_INFO, nprocnum : u32, preturnvalue : *mut core::ffi::c_void) -> CLIENT_CALL_RETURN);
    unsafe { Ndr64AsyncClientCall(pproxyinfo as _, nprocnum, preturnvalue as _) }
}
#[inline]
pub unsafe fn Ndr64AsyncServerCall64(prpcmsg: *mut RPC_MESSAGE) {
    windows_core::link!("rpcrt4.dll" "system" fn Ndr64AsyncServerCall64(prpcmsg : *mut RPC_MESSAGE));
    unsafe { Ndr64AsyncServerCall64(prpcmsg as _) }
}
#[inline]
pub unsafe fn Ndr64AsyncServerCallAll(prpcmsg: *mut RPC_MESSAGE) {
    windows_core::link!("rpcrt4.dll" "system" fn Ndr64AsyncServerCallAll(prpcmsg : *mut RPC_MESSAGE));
    unsafe { Ndr64AsyncServerCallAll(prpcmsg as _) }
}
#[cfg(feature = "objidlbase")]
#[inline]
pub unsafe fn Ndr64DcomAsyncClientCall(pproxyinfo: *mut MIDL_STUBLESS_PROXY_INFO, nprocnum: u32, preturnvalue: *mut core::ffi::c_void) -> CLIENT_CALL_RETURN {
    windows_core::link!("rpcrt4.dll" "C" fn Ndr64DcomAsyncClientCall(pproxyinfo : *mut MIDL_STUBLESS_PROXY_INFO, nprocnum : u32, preturnvalue : *mut core::ffi::c_void) -> CLIENT_CALL_RETURN);
    unsafe { Ndr64DcomAsyncClientCall(pproxyinfo as _, nprocnum, preturnvalue as _) }
}
#[cfg(feature = "objidlbase")]
#[inline]
pub unsafe fn Ndr64DcomAsyncStubCall<P0, P1>(pthis: P0, pchannel: P1, prpcmsg: *mut RPC_MESSAGE, pdwstubphase: *mut u32) -> i32
where
    P0: windows_core::Param<super::objidlbase::IRpcStubBuffer>,
    P1: windows_core::Param<super::objidlbase::IRpcChannelBuffer>,
{
    windows_core::link!("rpcrt4.dll" "system" fn Ndr64DcomAsyncStubCall(pthis : *mut core::ffi::c_void, pchannel : *mut core::ffi::c_void, prpcmsg : *mut RPC_MESSAGE, pdwstubphase : *mut u32) -> i32);
    unsafe { Ndr64DcomAsyncStubCall(pthis.param().abi(), pchannel.param().abi(), prpcmsg as _, pdwstubphase as _) }
}
#[cfg(feature = "objidlbase")]
#[inline]
pub unsafe fn NdrAllocate(pstubmsg: *mut MIDL_STUB_MESSAGE, len: usize) -> *mut core::ffi::c_void {
    windows_core::link!("rpcrt4.dll" "system" fn NdrAllocate(pstubmsg : *mut MIDL_STUB_MESSAGE, len : usize) -> *mut core::ffi::c_void);
    unsafe { NdrAllocate(core::mem::transmute(pstubmsg), len) }
}
#[cfg(feature = "objidlbase")]
#[inline]
pub unsafe fn NdrAsyncClientCall(pstubdescriptor: *const MIDL_STUB_DESC, pformat: *const u8) -> CLIENT_CALL_RETURN {
    windows_core::link!("rpcrt4.dll" "C" fn NdrAsyncClientCall(pstubdescriptor : *const MIDL_STUB_DESC, pformat : *const u8) -> CLIENT_CALL_RETURN);
    unsafe { NdrAsyncClientCall(pstubdescriptor, pformat) }
}
#[inline]
pub unsafe fn NdrAsyncServerCall(prpcmsg: *mut RPC_MESSAGE) {
    windows_core::link!("rpcrt4.dll" "system" fn NdrAsyncServerCall(prpcmsg : *mut RPC_MESSAGE));
    unsafe { NdrAsyncServerCall(prpcmsg as _) }
}
#[cfg(feature = "objidlbase")]
#[inline]
pub unsafe fn NdrByteCountPointerBufferSize(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *const u8) {
    windows_core::link!("rpcrt4.dll" "system" fn NdrByteCountPointerBufferSize(pstubmsg : *mut MIDL_STUB_MESSAGE, pmemory : *mut u8, pformat : *const u8));
    unsafe { NdrByteCountPointerBufferSize(core::mem::transmute(pstubmsg), pmemory as _, pformat) }
}
#[cfg(feature = "objidlbase")]
#[inline]
pub unsafe fn NdrByteCountPointerFree(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *const u8) {
    windows_core::link!("rpcrt4.dll" "system" fn NdrByteCountPointerFree(pstubmsg : *mut MIDL_STUB_MESSAGE, pmemory : *mut u8, pformat : *const u8));
    unsafe { NdrByteCountPointerFree(core::mem::transmute(pstubmsg), pmemory as _, pformat) }
}
#[cfg(feature = "objidlbase")]
#[inline]
pub unsafe fn NdrByteCountPointerMarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *const u8) -> *mut u8 {
    windows_core::link!("rpcrt4.dll" "system" fn NdrByteCountPointerMarshall(pstubmsg : *mut MIDL_STUB_MESSAGE, pmemory : *mut u8, pformat : *const u8) -> *mut u8);
    unsafe { NdrByteCountPointerMarshall(core::mem::transmute(pstubmsg), pmemory as _, pformat) }
}
#[cfg(feature = "objidlbase")]
#[inline]
pub unsafe fn NdrByteCountPointerUnmarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, ppmemory: *mut *mut u8, pformat: *const u8, fmustalloc: u8) -> *mut u8 {
    windows_core::link!("rpcrt4.dll" "system" fn NdrByteCountPointerUnmarshall(pstubmsg : *mut MIDL_STUB_MESSAGE, ppmemory : *mut *mut u8, pformat : *const u8, fmustalloc : u8) -> *mut u8);
    unsafe { NdrByteCountPointerUnmarshall(core::mem::transmute(pstubmsg), ppmemory as _, pformat, fmustalloc) }
}
#[cfg(feature = "objidlbase")]
#[inline]
pub unsafe fn NdrClearOutParameters(pstubmsg: *mut MIDL_STUB_MESSAGE, pformat: *const u8, argaddr: *mut core::ffi::c_void) {
    windows_core::link!("rpcrt4.dll" "system" fn NdrClearOutParameters(pstubmsg : *mut MIDL_STUB_MESSAGE, pformat : *const u8, argaddr : *mut core::ffi::c_void));
    unsafe { NdrClearOutParameters(core::mem::transmute(pstubmsg), pformat, argaddr as _) }
}
#[cfg(feature = "objidlbase")]
#[inline]
pub unsafe fn NdrClientCall2(pstubdescriptor: *const MIDL_STUB_DESC, pformat: *const u8) -> CLIENT_CALL_RETURN {
    windows_core::link!("rpcrt4.dll" "C" fn NdrClientCall2(pstubdescriptor : *const MIDL_STUB_DESC, pformat : *const u8) -> CLIENT_CALL_RETURN);
    unsafe { NdrClientCall2(pstubdescriptor, pformat) }
}
#[cfg(feature = "objidlbase")]
#[inline]
pub unsafe fn NdrClientCall3(pproxyinfo: *mut MIDL_STUBLESS_PROXY_INFO, nprocnum: u32, preturnvalue: *mut core::ffi::c_void) -> CLIENT_CALL_RETURN {
    windows_core::link!("rpcrt4.dll" "C" fn NdrClientCall3(pproxyinfo : *mut MIDL_STUBLESS_PROXY_INFO, nprocnum : u32, preturnvalue : *mut core::ffi::c_void) -> CLIENT_CALL_RETURN);
    unsafe { NdrClientCall3(pproxyinfo as _, nprocnum, preturnvalue as _) }
}
#[cfg(feature = "objidlbase")]
#[inline]
pub unsafe fn NdrClientContextMarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, contexthandle: NDR_CCONTEXT, fcheck: i32) {
    windows_core::link!("rpcrt4.dll" "system" fn NdrClientContextMarshall(pstubmsg : *mut MIDL_STUB_MESSAGE, contexthandle : NDR_CCONTEXT, fcheck : i32));
    unsafe { NdrClientContextMarshall(core::mem::transmute(pstubmsg), contexthandle, fcheck) }
}
#[cfg(feature = "objidlbase")]
#[inline]
pub unsafe fn NdrClientContextUnmarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, pcontexthandle: *mut NDR_CCONTEXT, bindhandle: RPC_BINDING_HANDLE) {
    windows_core::link!("rpcrt4.dll" "system" fn NdrClientContextUnmarshall(pstubmsg : *mut MIDL_STUB_MESSAGE, pcontexthandle : *mut NDR_CCONTEXT, bindhandle : RPC_BINDING_HANDLE));
    unsafe { NdrClientContextUnmarshall(core::mem::transmute(pstubmsg), pcontexthandle as _, bindhandle) }
}
#[cfg(feature = "objidlbase")]
#[inline]
pub unsafe fn NdrClientInitialize(prpcmsg: *mut RPC_MESSAGE, pstubmsg: *mut MIDL_STUB_MESSAGE, pstubdescriptor: *const MIDL_STUB_DESC, procnum: u32) {
    windows_core::link!("rpcrt4.dll" "system" fn NdrClientInitialize(prpcmsg : *mut RPC_MESSAGE, pstubmsg : *mut MIDL_STUB_MESSAGE, pstubdescriptor : *const MIDL_STUB_DESC, procnum : u32));
    unsafe { NdrClientInitialize(prpcmsg as _, core::mem::transmute(pstubmsg), pstubdescriptor, procnum) }
}
#[cfg(feature = "objidlbase")]
#[inline]
pub unsafe fn NdrClientInitializeNew(prpcmsg: *mut RPC_MESSAGE, pstubmsg: *mut MIDL_STUB_MESSAGE, pstubdescriptor: *const MIDL_STUB_DESC, procnum: u32) {
    windows_core::link!("rpcrt4.dll" "system" fn NdrClientInitializeNew(prpcmsg : *mut RPC_MESSAGE, pstubmsg : *mut MIDL_STUB_MESSAGE, pstubdescriptor : *const MIDL_STUB_DESC, procnum : u32));
    unsafe { NdrClientInitializeNew(prpcmsg as _, core::mem::transmute(pstubmsg), pstubdescriptor, procnum) }
}
#[cfg(feature = "objidlbase")]
#[inline]
pub unsafe fn NdrComplexArrayBufferSize(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *const u8) {
    windows_core::link!("rpcrt4.dll" "system" fn NdrComplexArrayBufferSize(pstubmsg : *mut MIDL_STUB_MESSAGE, pmemory : *mut u8, pformat : *const u8));
    unsafe { NdrComplexArrayBufferSize(core::mem::transmute(pstubmsg), pmemory as _, pformat) }
}
#[cfg(feature = "objidlbase")]
#[inline]
pub unsafe fn NdrComplexArrayFree(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *const u8) {
    windows_core::link!("rpcrt4.dll" "system" fn NdrComplexArrayFree(pstubmsg : *mut MIDL_STUB_MESSAGE, pmemory : *mut u8, pformat : *const u8));
    unsafe { NdrComplexArrayFree(core::mem::transmute(pstubmsg), pmemory as _, pformat) }
}
#[cfg(feature = "objidlbase")]
#[inline]
pub unsafe fn NdrComplexArrayMarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *const u8) -> *mut u8 {
    windows_core::link!("rpcrt4.dll" "system" fn NdrComplexArrayMarshall(pstubmsg : *mut MIDL_STUB_MESSAGE, pmemory : *mut u8, pformat : *const u8) -> *mut u8);
    unsafe { NdrComplexArrayMarshall(core::mem::transmute(pstubmsg), pmemory as _, pformat) }
}
#[cfg(feature = "objidlbase")]
#[inline]
pub unsafe fn NdrComplexArrayMemorySize(pstubmsg: *mut MIDL_STUB_MESSAGE, pformat: *const u8) -> u32 {
    windows_core::link!("rpcrt4.dll" "system" fn NdrComplexArrayMemorySize(pstubmsg : *mut MIDL_STUB_MESSAGE, pformat : *const u8) -> u32);
    unsafe { NdrComplexArrayMemorySize(core::mem::transmute(pstubmsg), pformat) }
}
#[cfg(feature = "objidlbase")]
#[inline]
pub unsafe fn NdrComplexArrayUnmarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, ppmemory: *mut *mut u8, pformat: *const u8, fmustalloc: u8) -> *mut u8 {
    windows_core::link!("rpcrt4.dll" "system" fn NdrComplexArrayUnmarshall(pstubmsg : *mut MIDL_STUB_MESSAGE, ppmemory : *mut *mut u8, pformat : *const u8, fmustalloc : u8) -> *mut u8);
    unsafe { NdrComplexArrayUnmarshall(core::mem::transmute(pstubmsg), ppmemory as _, pformat, fmustalloc) }
}
#[cfg(feature = "objidlbase")]
#[inline]
pub unsafe fn NdrComplexStructBufferSize(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *const u8) {
    windows_core::link!("rpcrt4.dll" "system" fn NdrComplexStructBufferSize(pstubmsg : *mut MIDL_STUB_MESSAGE, pmemory : *mut u8, pformat : *const u8));
    unsafe { NdrComplexStructBufferSize(core::mem::transmute(pstubmsg), pmemory as _, pformat) }
}
#[cfg(feature = "objidlbase")]
#[inline]
pub unsafe fn NdrComplexStructFree(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *const u8) {
    windows_core::link!("rpcrt4.dll" "system" fn NdrComplexStructFree(pstubmsg : *mut MIDL_STUB_MESSAGE, pmemory : *mut u8, pformat : *const u8));
    unsafe { NdrComplexStructFree(core::mem::transmute(pstubmsg), pmemory as _, pformat) }
}
#[cfg(feature = "objidlbase")]
#[inline]
pub unsafe fn NdrComplexStructMarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *const u8) -> *mut u8 {
    windows_core::link!("rpcrt4.dll" "system" fn NdrComplexStructMarshall(pstubmsg : *mut MIDL_STUB_MESSAGE, pmemory : *mut u8, pformat : *const u8) -> *mut u8);
    unsafe { NdrComplexStructMarshall(core::mem::transmute(pstubmsg), pmemory as _, pformat) }
}
#[cfg(feature = "objidlbase")]
#[inline]
pub unsafe fn NdrComplexStructMemorySize(pstubmsg: *mut MIDL_STUB_MESSAGE, pformat: *const u8) -> u32 {
    windows_core::link!("rpcrt4.dll" "system" fn NdrComplexStructMemorySize(pstubmsg : *mut MIDL_STUB_MESSAGE, pformat : *const u8) -> u32);
    unsafe { NdrComplexStructMemorySize(core::mem::transmute(pstubmsg), pformat) }
}
#[cfg(feature = "objidlbase")]
#[inline]
pub unsafe fn NdrComplexStructUnmarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, ppmemory: *mut *mut u8, pformat: *const u8, fmustalloc: u8) -> *mut u8 {
    windows_core::link!("rpcrt4.dll" "system" fn NdrComplexStructUnmarshall(pstubmsg : *mut MIDL_STUB_MESSAGE, ppmemory : *mut *mut u8, pformat : *const u8, fmustalloc : u8) -> *mut u8);
    unsafe { NdrComplexStructUnmarshall(core::mem::transmute(pstubmsg), ppmemory as _, pformat, fmustalloc) }
}
#[cfg(feature = "objidlbase")]
#[inline]
pub unsafe fn NdrConformantArrayBufferSize(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *const u8) {
    windows_core::link!("rpcrt4.dll" "system" fn NdrConformantArrayBufferSize(pstubmsg : *mut MIDL_STUB_MESSAGE, pmemory : *mut u8, pformat : *const u8));
    unsafe { NdrConformantArrayBufferSize(core::mem::transmute(pstubmsg), pmemory as _, pformat) }
}
#[cfg(feature = "objidlbase")]
#[inline]
pub unsafe fn NdrConformantArrayFree(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *const u8) {
    windows_core::link!("rpcrt4.dll" "system" fn NdrConformantArrayFree(pstubmsg : *mut MIDL_STUB_MESSAGE, pmemory : *mut u8, pformat : *const u8));
    unsafe { NdrConformantArrayFree(core::mem::transmute(pstubmsg), pmemory as _, pformat) }
}
#[cfg(feature = "objidlbase")]
#[inline]
pub unsafe fn NdrConformantArrayMarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *const u8) -> *mut u8 {
    windows_core::link!("rpcrt4.dll" "system" fn NdrConformantArrayMarshall(pstubmsg : *mut MIDL_STUB_MESSAGE, pmemory : *mut u8, pformat : *const u8) -> *mut u8);
    unsafe { NdrConformantArrayMarshall(core::mem::transmute(pstubmsg), pmemory as _, pformat) }
}
#[cfg(feature = "objidlbase")]
#[inline]
pub unsafe fn NdrConformantArrayMemorySize(pstubmsg: *mut MIDL_STUB_MESSAGE, pformat: *const u8) -> u32 {
    windows_core::link!("rpcrt4.dll" "system" fn NdrConformantArrayMemorySize(pstubmsg : *mut MIDL_STUB_MESSAGE, pformat : *const u8) -> u32);
    unsafe { NdrConformantArrayMemorySize(core::mem::transmute(pstubmsg), pformat) }
}
#[cfg(feature = "objidlbase")]
#[inline]
pub unsafe fn NdrConformantArrayUnmarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, ppmemory: *mut *mut u8, pformat: *const u8, fmustalloc: u8) -> *mut u8 {
    windows_core::link!("rpcrt4.dll" "system" fn NdrConformantArrayUnmarshall(pstubmsg : *mut MIDL_STUB_MESSAGE, ppmemory : *mut *mut u8, pformat : *const u8, fmustalloc : u8) -> *mut u8);
    unsafe { NdrConformantArrayUnmarshall(core::mem::transmute(pstubmsg), ppmemory as _, pformat, fmustalloc) }
}
#[cfg(feature = "objidlbase")]
#[inline]
pub unsafe fn NdrConformantStringBufferSize(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *const u8) {
    windows_core::link!("rpcrt4.dll" "system" fn NdrConformantStringBufferSize(pstubmsg : *mut MIDL_STUB_MESSAGE, pmemory : *mut u8, pformat : *const u8));
    unsafe { NdrConformantStringBufferSize(core::mem::transmute(pstubmsg), pmemory as _, pformat) }
}
#[cfg(feature = "objidlbase")]
#[inline]
pub unsafe fn NdrConformantStringMarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *const u8) -> *mut u8 {
    windows_core::link!("rpcrt4.dll" "system" fn NdrConformantStringMarshall(pstubmsg : *mut MIDL_STUB_MESSAGE, pmemory : *mut u8, pformat : *const u8) -> *mut u8);
    unsafe { NdrConformantStringMarshall(core::mem::transmute(pstubmsg), pmemory as _, pformat) }
}
#[cfg(feature = "objidlbase")]
#[inline]
pub unsafe fn NdrConformantStringMemorySize(pstubmsg: *mut MIDL_STUB_MESSAGE, pformat: *const u8) -> u32 {
    windows_core::link!("rpcrt4.dll" "system" fn NdrConformantStringMemorySize(pstubmsg : *mut MIDL_STUB_MESSAGE, pformat : *const u8) -> u32);
    unsafe { NdrConformantStringMemorySize(core::mem::transmute(pstubmsg), pformat) }
}
#[cfg(feature = "objidlbase")]
#[inline]
pub unsafe fn NdrConformantStringUnmarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, ppmemory: *mut *mut u8, pformat: *const u8, fmustalloc: u8) -> *mut u8 {
    windows_core::link!("rpcrt4.dll" "system" fn NdrConformantStringUnmarshall(pstubmsg : *mut MIDL_STUB_MESSAGE, ppmemory : *mut *mut u8, pformat : *const u8, fmustalloc : u8) -> *mut u8);
    unsafe { NdrConformantStringUnmarshall(core::mem::transmute(pstubmsg), ppmemory as _, pformat, fmustalloc) }
}
#[cfg(feature = "objidlbase")]
#[inline]
pub unsafe fn NdrConformantStructBufferSize(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *const u8) {
    windows_core::link!("rpcrt4.dll" "system" fn NdrConformantStructBufferSize(pstubmsg : *mut MIDL_STUB_MESSAGE, pmemory : *mut u8, pformat : *const u8));
    unsafe { NdrConformantStructBufferSize(core::mem::transmute(pstubmsg), pmemory as _, pformat) }
}
#[cfg(feature = "objidlbase")]
#[inline]
pub unsafe fn NdrConformantStructFree(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *const u8) {
    windows_core::link!("rpcrt4.dll" "system" fn NdrConformantStructFree(pstubmsg : *mut MIDL_STUB_MESSAGE, pmemory : *mut u8, pformat : *const u8));
    unsafe { NdrConformantStructFree(core::mem::transmute(pstubmsg), pmemory as _, pformat) }
}
#[cfg(feature = "objidlbase")]
#[inline]
pub unsafe fn NdrConformantStructMarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *const u8) -> *mut u8 {
    windows_core::link!("rpcrt4.dll" "system" fn NdrConformantStructMarshall(pstubmsg : *mut MIDL_STUB_MESSAGE, pmemory : *mut u8, pformat : *const u8) -> *mut u8);
    unsafe { NdrConformantStructMarshall(core::mem::transmute(pstubmsg), pmemory as _, pformat) }
}
#[cfg(feature = "objidlbase")]
#[inline]
pub unsafe fn NdrConformantStructMemorySize(pstubmsg: *mut MIDL_STUB_MESSAGE, pformat: *const u8) -> u32 {
    windows_core::link!("rpcrt4.dll" "system" fn NdrConformantStructMemorySize(pstubmsg : *mut MIDL_STUB_MESSAGE, pformat : *const u8) -> u32);
    unsafe { NdrConformantStructMemorySize(core::mem::transmute(pstubmsg), pformat) }
}
#[cfg(feature = "objidlbase")]
#[inline]
pub unsafe fn NdrConformantStructUnmarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, ppmemory: *mut *mut u8, pformat: *const u8, fmustalloc: u8) -> *mut u8 {
    windows_core::link!("rpcrt4.dll" "system" fn NdrConformantStructUnmarshall(pstubmsg : *mut MIDL_STUB_MESSAGE, ppmemory : *mut *mut u8, pformat : *const u8, fmustalloc : u8) -> *mut u8);
    unsafe { NdrConformantStructUnmarshall(core::mem::transmute(pstubmsg), ppmemory as _, pformat, fmustalloc) }
}
#[cfg(feature = "objidlbase")]
#[inline]
pub unsafe fn NdrConformantVaryingArrayBufferSize(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *const u8) {
    windows_core::link!("rpcrt4.dll" "system" fn NdrConformantVaryingArrayBufferSize(pstubmsg : *mut MIDL_STUB_MESSAGE, pmemory : *mut u8, pformat : *const u8));
    unsafe { NdrConformantVaryingArrayBufferSize(core::mem::transmute(pstubmsg), pmemory as _, pformat) }
}
#[cfg(feature = "objidlbase")]
#[inline]
pub unsafe fn NdrConformantVaryingArrayFree(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *const u8) {
    windows_core::link!("rpcrt4.dll" "system" fn NdrConformantVaryingArrayFree(pstubmsg : *mut MIDL_STUB_MESSAGE, pmemory : *mut u8, pformat : *const u8));
    unsafe { NdrConformantVaryingArrayFree(core::mem::transmute(pstubmsg), pmemory as _, pformat) }
}
#[cfg(feature = "objidlbase")]
#[inline]
pub unsafe fn NdrConformantVaryingArrayMarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *const u8) -> *mut u8 {
    windows_core::link!("rpcrt4.dll" "system" fn NdrConformantVaryingArrayMarshall(pstubmsg : *mut MIDL_STUB_MESSAGE, pmemory : *mut u8, pformat : *const u8) -> *mut u8);
    unsafe { NdrConformantVaryingArrayMarshall(core::mem::transmute(pstubmsg), pmemory as _, pformat) }
}
#[cfg(feature = "objidlbase")]
#[inline]
pub unsafe fn NdrConformantVaryingArrayMemorySize(pstubmsg: *mut MIDL_STUB_MESSAGE, pformat: *const u8) -> u32 {
    windows_core::link!("rpcrt4.dll" "system" fn NdrConformantVaryingArrayMemorySize(pstubmsg : *mut MIDL_STUB_MESSAGE, pformat : *const u8) -> u32);
    unsafe { NdrConformantVaryingArrayMemorySize(core::mem::transmute(pstubmsg), pformat) }
}
#[cfg(feature = "objidlbase")]
#[inline]
pub unsafe fn NdrConformantVaryingArrayUnmarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, ppmemory: *mut *mut u8, pformat: *const u8, fmustalloc: u8) -> *mut u8 {
    windows_core::link!("rpcrt4.dll" "system" fn NdrConformantVaryingArrayUnmarshall(pstubmsg : *mut MIDL_STUB_MESSAGE, ppmemory : *mut *mut u8, pformat : *const u8, fmustalloc : u8) -> *mut u8);
    unsafe { NdrConformantVaryingArrayUnmarshall(core::mem::transmute(pstubmsg), ppmemory as _, pformat, fmustalloc) }
}
#[cfg(feature = "objidlbase")]
#[inline]
pub unsafe fn NdrConformantVaryingStructBufferSize(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *const u8) {
    windows_core::link!("rpcrt4.dll" "system" fn NdrConformantVaryingStructBufferSize(pstubmsg : *mut MIDL_STUB_MESSAGE, pmemory : *mut u8, pformat : *const u8));
    unsafe { NdrConformantVaryingStructBufferSize(core::mem::transmute(pstubmsg), pmemory as _, pformat) }
}
#[cfg(feature = "objidlbase")]
#[inline]
pub unsafe fn NdrConformantVaryingStructFree(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *const u8) {
    windows_core::link!("rpcrt4.dll" "system" fn NdrConformantVaryingStructFree(pstubmsg : *mut MIDL_STUB_MESSAGE, pmemory : *mut u8, pformat : *const u8));
    unsafe { NdrConformantVaryingStructFree(core::mem::transmute(pstubmsg), pmemory as _, pformat) }
}
#[cfg(feature = "objidlbase")]
#[inline]
pub unsafe fn NdrConformantVaryingStructMarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *const u8) -> *mut u8 {
    windows_core::link!("rpcrt4.dll" "system" fn NdrConformantVaryingStructMarshall(pstubmsg : *mut MIDL_STUB_MESSAGE, pmemory : *mut u8, pformat : *const u8) -> *mut u8);
    unsafe { NdrConformantVaryingStructMarshall(core::mem::transmute(pstubmsg), pmemory as _, pformat) }
}
#[cfg(feature = "objidlbase")]
#[inline]
pub unsafe fn NdrConformantVaryingStructMemorySize(pstubmsg: *mut MIDL_STUB_MESSAGE, pformat: *const u8) -> u32 {
    windows_core::link!("rpcrt4.dll" "system" fn NdrConformantVaryingStructMemorySize(pstubmsg : *mut MIDL_STUB_MESSAGE, pformat : *const u8) -> u32);
    unsafe { NdrConformantVaryingStructMemorySize(core::mem::transmute(pstubmsg), pformat) }
}
#[cfg(feature = "objidlbase")]
#[inline]
pub unsafe fn NdrConformantVaryingStructUnmarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, ppmemory: *mut *mut u8, pformat: *const u8, fmustalloc: u8) -> *mut u8 {
    windows_core::link!("rpcrt4.dll" "system" fn NdrConformantVaryingStructUnmarshall(pstubmsg : *mut MIDL_STUB_MESSAGE, ppmemory : *mut *mut u8, pformat : *const u8, fmustalloc : u8) -> *mut u8);
    unsafe { NdrConformantVaryingStructUnmarshall(core::mem::transmute(pstubmsg), ppmemory as _, pformat, fmustalloc) }
}
#[cfg(feature = "objidlbase")]
#[inline]
pub unsafe fn NdrContextHandleInitialize(pstubmsg: *const MIDL_STUB_MESSAGE, pformat: *const u8) -> NDR_SCONTEXT {
    windows_core::link!("rpcrt4.dll" "system" fn NdrContextHandleInitialize(pstubmsg : *const MIDL_STUB_MESSAGE, pformat : *const u8) -> NDR_SCONTEXT);
    unsafe { NdrContextHandleInitialize(core::mem::transmute(pstubmsg), pformat) }
}
#[cfg(feature = "objidlbase")]
#[inline]
pub unsafe fn NdrContextHandleSize(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *const u8) {
    windows_core::link!("rpcrt4.dll" "system" fn NdrContextHandleSize(pstubmsg : *mut MIDL_STUB_MESSAGE, pmemory : *mut u8, pformat : *const u8));
    unsafe { NdrContextHandleSize(core::mem::transmute(pstubmsg), pmemory as _, pformat) }
}
#[cfg(feature = "objidlbase")]
#[inline]
pub unsafe fn NdrConvert(pstubmsg: *mut MIDL_STUB_MESSAGE, pformat: *const u8) {
    windows_core::link!("rpcrt4.dll" "system" fn NdrConvert(pstubmsg : *mut MIDL_STUB_MESSAGE, pformat : *const u8));
    unsafe { NdrConvert(core::mem::transmute(pstubmsg), pformat) }
}
#[cfg(feature = "objidlbase")]
#[inline]
pub unsafe fn NdrConvert2(pstubmsg: *mut MIDL_STUB_MESSAGE, pformat: *const u8, numberparams: i32) {
    windows_core::link!("rpcrt4.dll" "system" fn NdrConvert2(pstubmsg : *mut MIDL_STUB_MESSAGE, pformat : *const u8, numberparams : i32));
    unsafe { NdrConvert2(core::mem::transmute(pstubmsg), pformat, numberparams) }
}
#[cfg(feature = "objidlbase")]
#[inline]
pub unsafe fn NdrCorrelationFree(pstubmsg: *mut MIDL_STUB_MESSAGE) {
    windows_core::link!("rpcrt4.dll" "system" fn NdrCorrelationFree(pstubmsg : *mut MIDL_STUB_MESSAGE));
    unsafe { NdrCorrelationFree(core::mem::transmute(pstubmsg)) }
}
#[cfg(feature = "objidlbase")]
#[inline]
pub unsafe fn NdrCorrelationInitialize(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut core::ffi::c_void, cachesize: u32, flags: u32) {
    windows_core::link!("rpcrt4.dll" "system" fn NdrCorrelationInitialize(pstubmsg : *mut MIDL_STUB_MESSAGE, pmemory : *mut core::ffi::c_void, cachesize : u32, flags : u32));
    unsafe { NdrCorrelationInitialize(core::mem::transmute(pstubmsg), pmemory as _, cachesize, flags) }
}
#[cfg(feature = "objidlbase")]
#[inline]
pub unsafe fn NdrCorrelationPass(pstubmsg: *mut MIDL_STUB_MESSAGE) {
    windows_core::link!("rpcrt4.dll" "system" fn NdrCorrelationPass(pstubmsg : *mut MIDL_STUB_MESSAGE));
    unsafe { NdrCorrelationPass(core::mem::transmute(pstubmsg)) }
}
#[cfg(feature = "objidlbase")]
#[inline]
pub unsafe fn NdrCreateServerInterfaceFromStub<P0>(pstub: P0, pserverif: *mut RPC_SERVER_INTERFACE) -> RPC_STATUS
where
    P0: windows_core::Param<super::objidlbase::IRpcStubBuffer>,
{
    windows_core::link!("rpcrt4.dll" "system" fn NdrCreateServerInterfaceFromStub(pstub : *mut core::ffi::c_void, pserverif : *mut RPC_SERVER_INTERFACE) -> RPC_STATUS);
    unsafe { NdrCreateServerInterfaceFromStub(pstub.param().abi(), pserverif as _) }
}
#[cfg(feature = "objidlbase")]
#[inline]
pub unsafe fn NdrDcomAsyncClientCall(pstubdescriptor: *const MIDL_STUB_DESC, pformat: *const u8) -> CLIENT_CALL_RETURN {
    windows_core::link!("rpcrt4.dll" "C" fn NdrDcomAsyncClientCall(pstubdescriptor : *const MIDL_STUB_DESC, pformat : *const u8) -> CLIENT_CALL_RETURN);
    unsafe { NdrDcomAsyncClientCall(pstubdescriptor, pformat) }
}
#[cfg(feature = "objidlbase")]
#[inline]
pub unsafe fn NdrDcomAsyncStubCall<P0, P1>(pthis: P0, pchannel: P1, prpcmsg: *mut RPC_MESSAGE, pdwstubphase: *mut u32) -> i32
where
    P0: windows_core::Param<super::objidlbase::IRpcStubBuffer>,
    P1: windows_core::Param<super::objidlbase::IRpcChannelBuffer>,
{
    windows_core::link!("rpcrt4.dll" "system" fn NdrDcomAsyncStubCall(pthis : *mut core::ffi::c_void, pchannel : *mut core::ffi::c_void, prpcmsg : *mut RPC_MESSAGE, pdwstubphase : *mut u32) -> i32);
    unsafe { NdrDcomAsyncStubCall(pthis.param().abi(), pchannel.param().abi(), prpcmsg as _, pdwstubphase as _) }
}
#[cfg(feature = "objidlbase")]
#[inline]
pub unsafe fn NdrEncapsulatedUnionBufferSize(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *const u8) {
    windows_core::link!("rpcrt4.dll" "system" fn NdrEncapsulatedUnionBufferSize(pstubmsg : *mut MIDL_STUB_MESSAGE, pmemory : *mut u8, pformat : *const u8));
    unsafe { NdrEncapsulatedUnionBufferSize(core::mem::transmute(pstubmsg), pmemory as _, pformat) }
}
#[cfg(feature = "objidlbase")]
#[inline]
pub unsafe fn NdrEncapsulatedUnionFree(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *const u8) {
    windows_core::link!("rpcrt4.dll" "system" fn NdrEncapsulatedUnionFree(pstubmsg : *mut MIDL_STUB_MESSAGE, pmemory : *mut u8, pformat : *const u8));
    unsafe { NdrEncapsulatedUnionFree(core::mem::transmute(pstubmsg), pmemory as _, pformat) }
}
#[cfg(feature = "objidlbase")]
#[inline]
pub unsafe fn NdrEncapsulatedUnionMarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *const u8) -> *mut u8 {
    windows_core::link!("rpcrt4.dll" "system" fn NdrEncapsulatedUnionMarshall(pstubmsg : *mut MIDL_STUB_MESSAGE, pmemory : *mut u8, pformat : *const u8) -> *mut u8);
    unsafe { NdrEncapsulatedUnionMarshall(core::mem::transmute(pstubmsg), pmemory as _, pformat) }
}
#[cfg(feature = "objidlbase")]
#[inline]
pub unsafe fn NdrEncapsulatedUnionMemorySize(pstubmsg: *mut MIDL_STUB_MESSAGE, pformat: *const u8) -> u32 {
    windows_core::link!("rpcrt4.dll" "system" fn NdrEncapsulatedUnionMemorySize(pstubmsg : *mut MIDL_STUB_MESSAGE, pformat : *const u8) -> u32);
    unsafe { NdrEncapsulatedUnionMemorySize(core::mem::transmute(pstubmsg), pformat) }
}
#[cfg(feature = "objidlbase")]
#[inline]
pub unsafe fn NdrEncapsulatedUnionUnmarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, ppmemory: *mut *mut u8, pformat: *const u8, fmustalloc: u8) -> *mut u8 {
    windows_core::link!("rpcrt4.dll" "system" fn NdrEncapsulatedUnionUnmarshall(pstubmsg : *mut MIDL_STUB_MESSAGE, ppmemory : *mut *mut u8, pformat : *const u8, fmustalloc : u8) -> *mut u8);
    unsafe { NdrEncapsulatedUnionUnmarshall(core::mem::transmute(pstubmsg), ppmemory as _, pformat, fmustalloc) }
}
#[cfg(feature = "objidlbase")]
#[inline]
pub unsafe fn NdrFixedArrayBufferSize(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *const u8) {
    windows_core::link!("rpcrt4.dll" "system" fn NdrFixedArrayBufferSize(pstubmsg : *mut MIDL_STUB_MESSAGE, pmemory : *mut u8, pformat : *const u8));
    unsafe { NdrFixedArrayBufferSize(core::mem::transmute(pstubmsg), pmemory as _, pformat) }
}
#[cfg(feature = "objidlbase")]
#[inline]
pub unsafe fn NdrFixedArrayFree(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *const u8) {
    windows_core::link!("rpcrt4.dll" "system" fn NdrFixedArrayFree(pstubmsg : *mut MIDL_STUB_MESSAGE, pmemory : *mut u8, pformat : *const u8));
    unsafe { NdrFixedArrayFree(core::mem::transmute(pstubmsg), pmemory as _, pformat) }
}
#[cfg(feature = "objidlbase")]
#[inline]
pub unsafe fn NdrFixedArrayMarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *const u8) -> *mut u8 {
    windows_core::link!("rpcrt4.dll" "system" fn NdrFixedArrayMarshall(pstubmsg : *mut MIDL_STUB_MESSAGE, pmemory : *mut u8, pformat : *const u8) -> *mut u8);
    unsafe { NdrFixedArrayMarshall(core::mem::transmute(pstubmsg), pmemory as _, pformat) }
}
#[cfg(feature = "objidlbase")]
#[inline]
pub unsafe fn NdrFixedArrayMemorySize(pstubmsg: *mut MIDL_STUB_MESSAGE, pformat: *const u8) -> u32 {
    windows_core::link!("rpcrt4.dll" "system" fn NdrFixedArrayMemorySize(pstubmsg : *mut MIDL_STUB_MESSAGE, pformat : *const u8) -> u32);
    unsafe { NdrFixedArrayMemorySize(core::mem::transmute(pstubmsg), pformat) }
}
#[cfg(feature = "objidlbase")]
#[inline]
pub unsafe fn NdrFixedArrayUnmarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, ppmemory: *mut *mut u8, pformat: *const u8, fmustalloc: u8) -> *mut u8 {
    windows_core::link!("rpcrt4.dll" "system" fn NdrFixedArrayUnmarshall(pstubmsg : *mut MIDL_STUB_MESSAGE, ppmemory : *mut *mut u8, pformat : *const u8, fmustalloc : u8) -> *mut u8);
    unsafe { NdrFixedArrayUnmarshall(core::mem::transmute(pstubmsg), ppmemory as _, pformat, fmustalloc) }
}
#[cfg(feature = "objidlbase")]
#[inline]
pub unsafe fn NdrFreeBuffer(pstubmsg: *mut MIDL_STUB_MESSAGE) {
    windows_core::link!("rpcrt4.dll" "system" fn NdrFreeBuffer(pstubmsg : *mut MIDL_STUB_MESSAGE));
    unsafe { NdrFreeBuffer(core::mem::transmute(pstubmsg)) }
}
#[inline]
pub unsafe fn NdrFullPointerXlatFree() -> FULL_PTR_XLAT_TABLES {
    windows_core::link!("rpcrt4.dll" "system" fn NdrFullPointerXlatFree(pxlattables : *mut FULL_PTR_XLAT_TABLES));
    unsafe {
        let mut result__ = core::mem::zeroed();
        NdrFullPointerXlatFree(&mut result__);
        result__
    }
}
#[inline]
pub unsafe fn NdrFullPointerXlatInit(numberofpointers: u32, xlatside: XLAT_SIDE) -> PFULL_PTR_XLAT_TABLES {
    windows_core::link!("rpcrt4.dll" "system" fn NdrFullPointerXlatInit(numberofpointers : u32, xlatside : XLAT_SIDE) -> PFULL_PTR_XLAT_TABLES);
    unsafe { NdrFullPointerXlatInit(numberofpointers, xlatside) }
}
#[cfg(feature = "objidlbase")]
#[inline]
pub unsafe fn NdrGetBuffer(pstubmsg: *mut MIDL_STUB_MESSAGE, bufferlength: u32, handle: RPC_BINDING_HANDLE) -> *mut u8 {
    windows_core::link!("rpcrt4.dll" "system" fn NdrGetBuffer(pstubmsg : *mut MIDL_STUB_MESSAGE, bufferlength : u32, handle : RPC_BINDING_HANDLE) -> *mut u8);
    unsafe { NdrGetBuffer(core::mem::transmute(pstubmsg), bufferlength, handle) }
}
#[cfg(feature = "objidlbase")]
#[inline]
pub unsafe fn NdrGetDcomProtocolVersion(pstubmsg: *mut MIDL_STUB_MESSAGE, pversion: *mut RPC_VERSION) -> windows_core::HRESULT {
    windows_core::link!("rpcrt4.dll" "system" fn NdrGetDcomProtocolVersion(pstubmsg : *mut MIDL_STUB_MESSAGE, pversion : *mut RPC_VERSION) -> windows_core::HRESULT);
    unsafe { NdrGetDcomProtocolVersion(core::mem::transmute(pstubmsg), pversion as _) }
}
#[cfg(feature = "objidlbase")]
#[inline]
pub unsafe fn NdrGetUserMarshalInfo(pflags: *const u32, informationlevel: u32, pmarshalinfo: *mut NDR_USER_MARSHAL_INFO) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn NdrGetUserMarshalInfo(pflags : *const u32, informationlevel : u32, pmarshalinfo : *mut NDR_USER_MARSHAL_INFO) -> RPC_STATUS);
    unsafe { NdrGetUserMarshalInfo(pflags, informationlevel, core::mem::transmute(pmarshalinfo)) }
}
#[cfg(feature = "objidlbase")]
#[inline]
pub unsafe fn NdrInterfacePointerBufferSize(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *const u8) {
    windows_core::link!("rpcrt4.dll" "system" fn NdrInterfacePointerBufferSize(pstubmsg : *mut MIDL_STUB_MESSAGE, pmemory : *mut u8, pformat : *const u8));
    unsafe { NdrInterfacePointerBufferSize(core::mem::transmute(pstubmsg), pmemory as _, pformat) }
}
#[cfg(feature = "objidlbase")]
#[inline]
pub unsafe fn NdrInterfacePointerFree(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *const u8) {
    windows_core::link!("rpcrt4.dll" "system" fn NdrInterfacePointerFree(pstubmsg : *mut MIDL_STUB_MESSAGE, pmemory : *mut u8, pformat : *const u8));
    unsafe { NdrInterfacePointerFree(core::mem::transmute(pstubmsg), pmemory as _, pformat) }
}
#[cfg(feature = "objidlbase")]
#[inline]
pub unsafe fn NdrInterfacePointerMarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *const u8) -> *mut u8 {
    windows_core::link!("rpcrt4.dll" "system" fn NdrInterfacePointerMarshall(pstubmsg : *mut MIDL_STUB_MESSAGE, pmemory : *mut u8, pformat : *const u8) -> *mut u8);
    unsafe { NdrInterfacePointerMarshall(core::mem::transmute(pstubmsg), pmemory as _, pformat) }
}
#[cfg(feature = "objidlbase")]
#[inline]
pub unsafe fn NdrInterfacePointerMemorySize(pstubmsg: *mut MIDL_STUB_MESSAGE, pformat: *const u8) -> u32 {
    windows_core::link!("rpcrt4.dll" "system" fn NdrInterfacePointerMemorySize(pstubmsg : *mut MIDL_STUB_MESSAGE, pformat : *const u8) -> u32);
    unsafe { NdrInterfacePointerMemorySize(core::mem::transmute(pstubmsg), pformat) }
}
#[cfg(feature = "objidlbase")]
#[inline]
pub unsafe fn NdrInterfacePointerUnmarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, ppmemory: *mut *mut u8, pformat: *const u8, fmustalloc: u8) -> *mut u8 {
    windows_core::link!("rpcrt4.dll" "system" fn NdrInterfacePointerUnmarshall(pstubmsg : *mut MIDL_STUB_MESSAGE, ppmemory : *mut *mut u8, pformat : *const u8, fmustalloc : u8) -> *mut u8);
    unsafe { NdrInterfacePointerUnmarshall(core::mem::transmute(pstubmsg), ppmemory as _, pformat, fmustalloc) }
}
#[cfg(feature = "objidlbase")]
#[inline]
pub unsafe fn NdrMapCommAndFaultStatus(pstubmsg: *mut MIDL_STUB_MESSAGE, pcommstatus: *mut u32, pfaultstatus: *mut u32, status: RPC_STATUS) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn NdrMapCommAndFaultStatus(pstubmsg : *mut MIDL_STUB_MESSAGE, pcommstatus : *mut u32, pfaultstatus : *mut u32, status : RPC_STATUS) -> RPC_STATUS);
    unsafe { NdrMapCommAndFaultStatus(core::mem::transmute(pstubmsg), pcommstatus as _, pfaultstatus as _, status) }
}
#[cfg(feature = "objidlbase")]
#[inline]
pub unsafe fn NdrNonConformantStringBufferSize(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *const u8) {
    windows_core::link!("rpcrt4.dll" "system" fn NdrNonConformantStringBufferSize(pstubmsg : *mut MIDL_STUB_MESSAGE, pmemory : *mut u8, pformat : *const u8));
    unsafe { NdrNonConformantStringBufferSize(core::mem::transmute(pstubmsg), pmemory as _, pformat) }
}
#[cfg(feature = "objidlbase")]
#[inline]
pub unsafe fn NdrNonConformantStringMarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *const u8) -> *mut u8 {
    windows_core::link!("rpcrt4.dll" "system" fn NdrNonConformantStringMarshall(pstubmsg : *mut MIDL_STUB_MESSAGE, pmemory : *mut u8, pformat : *const u8) -> *mut u8);
    unsafe { NdrNonConformantStringMarshall(core::mem::transmute(pstubmsg), pmemory as _, pformat) }
}
#[cfg(feature = "objidlbase")]
#[inline]
pub unsafe fn NdrNonConformantStringMemorySize(pstubmsg: *mut MIDL_STUB_MESSAGE, pformat: *const u8) -> u32 {
    windows_core::link!("rpcrt4.dll" "system" fn NdrNonConformantStringMemorySize(pstubmsg : *mut MIDL_STUB_MESSAGE, pformat : *const u8) -> u32);
    unsafe { NdrNonConformantStringMemorySize(core::mem::transmute(pstubmsg), pformat) }
}
#[cfg(feature = "objidlbase")]
#[inline]
pub unsafe fn NdrNonConformantStringUnmarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, ppmemory: *mut *mut u8, pformat: *const u8, fmustalloc: u8) -> *mut u8 {
    windows_core::link!("rpcrt4.dll" "system" fn NdrNonConformantStringUnmarshall(pstubmsg : *mut MIDL_STUB_MESSAGE, ppmemory : *mut *mut u8, pformat : *const u8, fmustalloc : u8) -> *mut u8);
    unsafe { NdrNonConformantStringUnmarshall(core::mem::transmute(pstubmsg), ppmemory as _, pformat, fmustalloc) }
}
#[cfg(feature = "objidlbase")]
#[inline]
pub unsafe fn NdrNonEncapsulatedUnionBufferSize(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *const u8) {
    windows_core::link!("rpcrt4.dll" "system" fn NdrNonEncapsulatedUnionBufferSize(pstubmsg : *mut MIDL_STUB_MESSAGE, pmemory : *mut u8, pformat : *const u8));
    unsafe { NdrNonEncapsulatedUnionBufferSize(core::mem::transmute(pstubmsg), pmemory as _, pformat) }
}
#[cfg(feature = "objidlbase")]
#[inline]
pub unsafe fn NdrNonEncapsulatedUnionFree(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *const u8) {
    windows_core::link!("rpcrt4.dll" "system" fn NdrNonEncapsulatedUnionFree(pstubmsg : *mut MIDL_STUB_MESSAGE, pmemory : *mut u8, pformat : *const u8));
    unsafe { NdrNonEncapsulatedUnionFree(core::mem::transmute(pstubmsg), pmemory as _, pformat) }
}
#[cfg(feature = "objidlbase")]
#[inline]
pub unsafe fn NdrNonEncapsulatedUnionMarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *const u8) -> *mut u8 {
    windows_core::link!("rpcrt4.dll" "system" fn NdrNonEncapsulatedUnionMarshall(pstubmsg : *mut MIDL_STUB_MESSAGE, pmemory : *mut u8, pformat : *const u8) -> *mut u8);
    unsafe { NdrNonEncapsulatedUnionMarshall(core::mem::transmute(pstubmsg), pmemory as _, pformat) }
}
#[cfg(feature = "objidlbase")]
#[inline]
pub unsafe fn NdrNonEncapsulatedUnionMemorySize(pstubmsg: *mut MIDL_STUB_MESSAGE, pformat: *const u8) -> u32 {
    windows_core::link!("rpcrt4.dll" "system" fn NdrNonEncapsulatedUnionMemorySize(pstubmsg : *mut MIDL_STUB_MESSAGE, pformat : *const u8) -> u32);
    unsafe { NdrNonEncapsulatedUnionMemorySize(core::mem::transmute(pstubmsg), pformat) }
}
#[cfg(feature = "objidlbase")]
#[inline]
pub unsafe fn NdrNonEncapsulatedUnionUnmarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, ppmemory: *mut *mut u8, pformat: *const u8, fmustalloc: u8) -> *mut u8 {
    windows_core::link!("rpcrt4.dll" "system" fn NdrNonEncapsulatedUnionUnmarshall(pstubmsg : *mut MIDL_STUB_MESSAGE, ppmemory : *mut *mut u8, pformat : *const u8, fmustalloc : u8) -> *mut u8);
    unsafe { NdrNonEncapsulatedUnionUnmarshall(core::mem::transmute(pstubmsg), ppmemory as _, pformat, fmustalloc) }
}
#[cfg(feature = "objidlbase")]
#[inline]
pub unsafe fn NdrNsGetBuffer(pstubmsg: *mut MIDL_STUB_MESSAGE, bufferlength: u32, handle: RPC_BINDING_HANDLE) -> *mut u8 {
    windows_core::link!("rpcrt4.dll" "system" fn NdrNsGetBuffer(pstubmsg : *mut MIDL_STUB_MESSAGE, bufferlength : u32, handle : RPC_BINDING_HANDLE) -> *mut u8);
    unsafe { NdrNsGetBuffer(core::mem::transmute(pstubmsg), bufferlength, handle) }
}
#[cfg(feature = "objidlbase")]
#[inline]
pub unsafe fn NdrNsSendReceive(pstubmsg: *mut MIDL_STUB_MESSAGE, pbufferend: *mut u8, pautohandle: *mut RPC_BINDING_HANDLE) -> *mut u8 {
    windows_core::link!("rpcrt4.dll" "system" fn NdrNsSendReceive(pstubmsg : *mut MIDL_STUB_MESSAGE, pbufferend : *mut u8, pautohandle : *mut RPC_BINDING_HANDLE) -> *mut u8);
    unsafe { NdrNsSendReceive(core::mem::transmute(pstubmsg), pbufferend as _, pautohandle as _) }
}
#[inline]
pub unsafe fn NdrOleAllocate(size: usize) -> *mut core::ffi::c_void {
    windows_core::link!("rpcrt4.dll" "system" fn NdrOleAllocate(size : usize) -> *mut core::ffi::c_void);
    unsafe { NdrOleAllocate(size) }
}
#[inline]
pub unsafe fn NdrOleFree(nodetofree: *const core::ffi::c_void) {
    windows_core::link!("rpcrt4.dll" "system" fn NdrOleFree(nodetofree : *const core::ffi::c_void));
    unsafe { NdrOleFree(nodetofree) }
}
#[cfg(feature = "objidlbase")]
#[inline]
pub unsafe fn NdrPartialIgnoreClientBufferSize(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut core::ffi::c_void) {
    windows_core::link!("rpcrt4.dll" "system" fn NdrPartialIgnoreClientBufferSize(pstubmsg : *mut MIDL_STUB_MESSAGE, pmemory : *mut core::ffi::c_void));
    unsafe { NdrPartialIgnoreClientBufferSize(core::mem::transmute(pstubmsg), pmemory as _) }
}
#[cfg(feature = "objidlbase")]
#[inline]
pub unsafe fn NdrPartialIgnoreClientMarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut core::ffi::c_void) {
    windows_core::link!("rpcrt4.dll" "system" fn NdrPartialIgnoreClientMarshall(pstubmsg : *mut MIDL_STUB_MESSAGE, pmemory : *mut core::ffi::c_void));
    unsafe { NdrPartialIgnoreClientMarshall(core::mem::transmute(pstubmsg), pmemory as _) }
}
#[cfg(feature = "objidlbase")]
#[inline]
pub unsafe fn NdrPartialIgnoreServerInitialize(pstubmsg: *mut MIDL_STUB_MESSAGE, ppmemory: *mut *mut core::ffi::c_void, pformat: *const u8) {
    windows_core::link!("rpcrt4.dll" "system" fn NdrPartialIgnoreServerInitialize(pstubmsg : *mut MIDL_STUB_MESSAGE, ppmemory : *mut *mut core::ffi::c_void, pformat : *const u8));
    unsafe { NdrPartialIgnoreServerInitialize(core::mem::transmute(pstubmsg), ppmemory as _, pformat) }
}
#[cfg(feature = "objidlbase")]
#[inline]
pub unsafe fn NdrPartialIgnoreServerUnmarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, ppmemory: *mut *mut core::ffi::c_void) {
    windows_core::link!("rpcrt4.dll" "system" fn NdrPartialIgnoreServerUnmarshall(pstubmsg : *mut MIDL_STUB_MESSAGE, ppmemory : *mut *mut core::ffi::c_void));
    unsafe { NdrPartialIgnoreServerUnmarshall(core::mem::transmute(pstubmsg), ppmemory as _) }
}
#[cfg(feature = "objidlbase")]
#[inline]
pub unsafe fn NdrPointerBufferSize(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *const u8) {
    windows_core::link!("rpcrt4.dll" "system" fn NdrPointerBufferSize(pstubmsg : *mut MIDL_STUB_MESSAGE, pmemory : *mut u8, pformat : *const u8));
    unsafe { NdrPointerBufferSize(core::mem::transmute(pstubmsg), pmemory as _, pformat) }
}
#[cfg(feature = "objidlbase")]
#[inline]
pub unsafe fn NdrPointerFree(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *const u8) {
    windows_core::link!("rpcrt4.dll" "system" fn NdrPointerFree(pstubmsg : *mut MIDL_STUB_MESSAGE, pmemory : *mut u8, pformat : *const u8));
    unsafe { NdrPointerFree(core::mem::transmute(pstubmsg), pmemory as _, pformat) }
}
#[cfg(feature = "objidlbase")]
#[inline]
pub unsafe fn NdrPointerMarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *const u8) -> *mut u8 {
    windows_core::link!("rpcrt4.dll" "system" fn NdrPointerMarshall(pstubmsg : *mut MIDL_STUB_MESSAGE, pmemory : *mut u8, pformat : *const u8) -> *mut u8);
    unsafe { NdrPointerMarshall(core::mem::transmute(pstubmsg), pmemory as _, pformat) }
}
#[cfg(feature = "objidlbase")]
#[inline]
pub unsafe fn NdrPointerMemorySize(pstubmsg: *mut MIDL_STUB_MESSAGE, pformat: *const u8) -> u32 {
    windows_core::link!("rpcrt4.dll" "system" fn NdrPointerMemorySize(pstubmsg : *mut MIDL_STUB_MESSAGE, pformat : *const u8) -> u32);
    unsafe { NdrPointerMemorySize(core::mem::transmute(pstubmsg), pformat) }
}
#[cfg(feature = "objidlbase")]
#[inline]
pub unsafe fn NdrPointerUnmarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, ppmemory: *mut *mut u8, pformat: *const u8, fmustalloc: u8) -> *mut u8 {
    windows_core::link!("rpcrt4.dll" "system" fn NdrPointerUnmarshall(pstubmsg : *mut MIDL_STUB_MESSAGE, ppmemory : *mut *mut u8, pformat : *const u8, fmustalloc : u8) -> *mut u8);
    unsafe { NdrPointerUnmarshall(core::mem::transmute(pstubmsg), ppmemory as _, pformat, fmustalloc) }
}
#[cfg(feature = "objidlbase")]
#[inline]
pub unsafe fn NdrRangeUnmarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, ppmemory: *mut *mut u8, pformat: *const u8, fmustalloc: u8) -> *mut u8 {
    windows_core::link!("rpcrt4.dll" "system" fn NdrRangeUnmarshall(pstubmsg : *mut MIDL_STUB_MESSAGE, ppmemory : *mut *mut u8, pformat : *const u8, fmustalloc : u8) -> *mut u8);
    unsafe { NdrRangeUnmarshall(core::mem::transmute(pstubmsg), ppmemory as _, pformat, fmustalloc) }
}
#[inline]
pub unsafe fn NdrRpcSmClientAllocate(size: usize) -> *mut core::ffi::c_void {
    windows_core::link!("rpcrt4.dll" "system" fn NdrRpcSmClientAllocate(size : usize) -> *mut core::ffi::c_void);
    unsafe { NdrRpcSmClientAllocate(size) }
}
#[inline]
pub unsafe fn NdrRpcSmClientFree(nodetofree: *const core::ffi::c_void) {
    windows_core::link!("rpcrt4.dll" "system" fn NdrRpcSmClientFree(nodetofree : *const core::ffi::c_void));
    unsafe { NdrRpcSmClientFree(nodetofree) }
}
#[cfg(feature = "objidlbase")]
#[inline]
pub unsafe fn NdrRpcSmSetClientToOsf(pmessage: *mut MIDL_STUB_MESSAGE) {
    windows_core::link!("rpcrt4.dll" "system" fn NdrRpcSmSetClientToOsf(pmessage : *mut MIDL_STUB_MESSAGE));
    unsafe { NdrRpcSmSetClientToOsf(core::mem::transmute(pmessage)) }
}
#[inline]
pub unsafe fn NdrRpcSsDefaultAllocate(size: usize) -> *mut core::ffi::c_void {
    windows_core::link!("rpcrt4.dll" "system" fn NdrRpcSsDefaultAllocate(size : usize) -> *mut core::ffi::c_void);
    unsafe { NdrRpcSsDefaultAllocate(size) }
}
#[inline]
pub unsafe fn NdrRpcSsDefaultFree(nodetofree: *const core::ffi::c_void) {
    windows_core::link!("rpcrt4.dll" "system" fn NdrRpcSsDefaultFree(nodetofree : *const core::ffi::c_void));
    unsafe { NdrRpcSsDefaultFree(nodetofree) }
}
#[cfg(feature = "objidlbase")]
#[inline]
pub unsafe fn NdrRpcSsDisableAllocate(pmessage: *mut MIDL_STUB_MESSAGE) {
    windows_core::link!("rpcrt4.dll" "system" fn NdrRpcSsDisableAllocate(pmessage : *mut MIDL_STUB_MESSAGE));
    unsafe { NdrRpcSsDisableAllocate(core::mem::transmute(pmessage)) }
}
#[cfg(feature = "objidlbase")]
#[inline]
pub unsafe fn NdrRpcSsEnableAllocate(pmessage: *mut MIDL_STUB_MESSAGE) {
    windows_core::link!("rpcrt4.dll" "system" fn NdrRpcSsEnableAllocate(pmessage : *mut MIDL_STUB_MESSAGE));
    unsafe { NdrRpcSsEnableAllocate(core::mem::transmute(pmessage)) }
}
#[cfg(feature = "objidlbase")]
#[inline]
pub unsafe fn NdrSendReceive(pstubmsg: *mut MIDL_STUB_MESSAGE, pbufferend: *mut u8) -> *mut u8 {
    windows_core::link!("rpcrt4.dll" "system" fn NdrSendReceive(pstubmsg : *mut MIDL_STUB_MESSAGE, pbufferend : *mut u8) -> *mut u8);
    unsafe { NdrSendReceive(core::mem::transmute(pstubmsg), pbufferend as _) }
}
#[inline]
pub unsafe fn NdrServerCall2(prpcmsg: *mut RPC_MESSAGE) {
    windows_core::link!("rpcrt4.dll" "system" fn NdrServerCall2(prpcmsg : *mut RPC_MESSAGE));
    unsafe { NdrServerCall2(prpcmsg as _) }
}
#[inline]
pub unsafe fn NdrServerCallAll(prpcmsg: *mut RPC_MESSAGE) {
    windows_core::link!("rpcrt4.dll" "system" fn NdrServerCallAll(prpcmsg : *mut RPC_MESSAGE));
    unsafe { NdrServerCallAll(prpcmsg as _) }
}
#[inline]
pub unsafe fn NdrServerCallNdr64(prpcmsg: *mut RPC_MESSAGE) {
    windows_core::link!("rpcrt4.dll" "system" fn NdrServerCallNdr64(prpcmsg : *mut RPC_MESSAGE));
    unsafe { NdrServerCallNdr64(prpcmsg as _) }
}
#[cfg(feature = "objidlbase")]
#[inline]
pub unsafe fn NdrServerContextMarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, contexthandle: *mut _NDR_SCONTEXT, rundownroutine: NDR_RUNDOWN) {
    windows_core::link!("rpcrt4.dll" "system" fn NdrServerContextMarshall(pstubmsg : *mut MIDL_STUB_MESSAGE, contexthandle : *mut _NDR_SCONTEXT, rundownroutine : NDR_RUNDOWN));
    unsafe { NdrServerContextMarshall(core::mem::transmute(pstubmsg), contexthandle as _, rundownroutine) }
}
#[cfg(feature = "objidlbase")]
#[inline]
pub unsafe fn NdrServerContextNewMarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, contexthandle: *mut _NDR_SCONTEXT, rundownroutine: NDR_RUNDOWN, pformat: *const u8) {
    windows_core::link!("rpcrt4.dll" "system" fn NdrServerContextNewMarshall(pstubmsg : *mut MIDL_STUB_MESSAGE, contexthandle : *mut _NDR_SCONTEXT, rundownroutine : NDR_RUNDOWN, pformat : *const u8));
    unsafe { NdrServerContextNewMarshall(core::mem::transmute(pstubmsg), contexthandle as _, rundownroutine, pformat) }
}
#[cfg(feature = "objidlbase")]
#[inline]
pub unsafe fn NdrServerContextNewUnmarshall(pstubmsg: *const MIDL_STUB_MESSAGE, pformat: *const u8) -> NDR_SCONTEXT {
    windows_core::link!("rpcrt4.dll" "system" fn NdrServerContextNewUnmarshall(pstubmsg : *const MIDL_STUB_MESSAGE, pformat : *const u8) -> NDR_SCONTEXT);
    unsafe { NdrServerContextNewUnmarshall(core::mem::transmute(pstubmsg), pformat) }
}
#[cfg(feature = "objidlbase")]
#[inline]
pub unsafe fn NdrServerContextUnmarshall(pstubmsg: *mut MIDL_STUB_MESSAGE) -> NDR_SCONTEXT {
    windows_core::link!("rpcrt4.dll" "system" fn NdrServerContextUnmarshall(pstubmsg : *mut MIDL_STUB_MESSAGE) -> NDR_SCONTEXT);
    unsafe { NdrServerContextUnmarshall(core::mem::transmute(pstubmsg)) }
}
#[cfg(feature = "objidlbase")]
#[inline]
pub unsafe fn NdrServerInitialize(prpcmsg: *mut RPC_MESSAGE, pstubmsg: *mut MIDL_STUB_MESSAGE, pstubdescriptor: *const MIDL_STUB_DESC) -> *mut u8 {
    windows_core::link!("rpcrt4.dll" "system" fn NdrServerInitialize(prpcmsg : *mut RPC_MESSAGE, pstubmsg : *mut MIDL_STUB_MESSAGE, pstubdescriptor : *const MIDL_STUB_DESC) -> *mut u8);
    unsafe { NdrServerInitialize(prpcmsg as _, core::mem::transmute(pstubmsg), pstubdescriptor) }
}
#[cfg(feature = "objidlbase")]
#[inline]
pub unsafe fn NdrServerInitializeMarshall(prpcmsg: *mut RPC_MESSAGE, pstubmsg: *mut MIDL_STUB_MESSAGE) {
    windows_core::link!("rpcrt4.dll" "system" fn NdrServerInitializeMarshall(prpcmsg : *mut RPC_MESSAGE, pstubmsg : *mut MIDL_STUB_MESSAGE));
    unsafe { NdrServerInitializeMarshall(prpcmsg as _, core::mem::transmute(pstubmsg)) }
}
#[cfg(feature = "objidlbase")]
#[inline]
pub unsafe fn NdrServerInitializeNew(prpcmsg: *mut RPC_MESSAGE, pstubmsg: *mut MIDL_STUB_MESSAGE, pstubdescriptor: *const MIDL_STUB_DESC) -> *mut u8 {
    windows_core::link!("rpcrt4.dll" "system" fn NdrServerInitializeNew(prpcmsg : *mut RPC_MESSAGE, pstubmsg : *mut MIDL_STUB_MESSAGE, pstubdescriptor : *const MIDL_STUB_DESC) -> *mut u8);
    unsafe { NdrServerInitializeNew(prpcmsg as _, core::mem::transmute(pstubmsg), pstubdescriptor) }
}
#[cfg(feature = "objidlbase")]
#[inline]
pub unsafe fn NdrServerInitializePartial(prpcmsg: *mut RPC_MESSAGE, pstubmsg: *mut MIDL_STUB_MESSAGE, pstubdescriptor: *const MIDL_STUB_DESC, requestedbuffersize: u32) {
    windows_core::link!("rpcrt4.dll" "system" fn NdrServerInitializePartial(prpcmsg : *mut RPC_MESSAGE, pstubmsg : *mut MIDL_STUB_MESSAGE, pstubdescriptor : *const MIDL_STUB_DESC, requestedbuffersize : u32));
    unsafe { NdrServerInitializePartial(prpcmsg as _, core::mem::transmute(pstubmsg), pstubdescriptor, requestedbuffersize) }
}
#[cfg(feature = "objidlbase")]
#[inline]
pub unsafe fn NdrServerInitializeUnmarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, pstubdescriptor: *const MIDL_STUB_DESC, prpcmsg: *mut RPC_MESSAGE) -> *mut u8 {
    windows_core::link!("rpcrt4.dll" "system" fn NdrServerInitializeUnmarshall(pstubmsg : *mut MIDL_STUB_MESSAGE, pstubdescriptor : *const MIDL_STUB_DESC, prpcmsg : *mut RPC_MESSAGE) -> *mut u8);
    unsafe { NdrServerInitializeUnmarshall(core::mem::transmute(pstubmsg), pstubdescriptor, prpcmsg as _) }
}
#[cfg(feature = "objidlbase")]
#[inline]
pub unsafe fn NdrSimpleStructBufferSize(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *const u8) {
    windows_core::link!("rpcrt4.dll" "system" fn NdrSimpleStructBufferSize(pstubmsg : *mut MIDL_STUB_MESSAGE, pmemory : *mut u8, pformat : *const u8));
    unsafe { NdrSimpleStructBufferSize(core::mem::transmute(pstubmsg), pmemory as _, pformat) }
}
#[cfg(feature = "objidlbase")]
#[inline]
pub unsafe fn NdrSimpleStructFree(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *const u8) {
    windows_core::link!("rpcrt4.dll" "system" fn NdrSimpleStructFree(pstubmsg : *mut MIDL_STUB_MESSAGE, pmemory : *mut u8, pformat : *const u8));
    unsafe { NdrSimpleStructFree(core::mem::transmute(pstubmsg), pmemory as _, pformat) }
}
#[cfg(feature = "objidlbase")]
#[inline]
pub unsafe fn NdrSimpleStructMarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *const u8) -> *mut u8 {
    windows_core::link!("rpcrt4.dll" "system" fn NdrSimpleStructMarshall(pstubmsg : *mut MIDL_STUB_MESSAGE, pmemory : *mut u8, pformat : *const u8) -> *mut u8);
    unsafe { NdrSimpleStructMarshall(core::mem::transmute(pstubmsg), pmemory as _, pformat) }
}
#[cfg(feature = "objidlbase")]
#[inline]
pub unsafe fn NdrSimpleStructMemorySize(pstubmsg: *mut MIDL_STUB_MESSAGE, pformat: *const u8) -> u32 {
    windows_core::link!("rpcrt4.dll" "system" fn NdrSimpleStructMemorySize(pstubmsg : *mut MIDL_STUB_MESSAGE, pformat : *const u8) -> u32);
    unsafe { NdrSimpleStructMemorySize(core::mem::transmute(pstubmsg), pformat) }
}
#[cfg(feature = "objidlbase")]
#[inline]
pub unsafe fn NdrSimpleStructUnmarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, ppmemory: *mut *mut u8, pformat: *const u8, fmustalloc: u8) -> *mut u8 {
    windows_core::link!("rpcrt4.dll" "system" fn NdrSimpleStructUnmarshall(pstubmsg : *mut MIDL_STUB_MESSAGE, ppmemory : *mut *mut u8, pformat : *const u8, fmustalloc : u8) -> *mut u8);
    unsafe { NdrSimpleStructUnmarshall(core::mem::transmute(pstubmsg), ppmemory as _, pformat, fmustalloc) }
}
#[cfg(feature = "objidlbase")]
#[inline]
pub unsafe fn NdrSimpleTypeMarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, formatchar: u8) {
    windows_core::link!("rpcrt4.dll" "system" fn NdrSimpleTypeMarshall(pstubmsg : *mut MIDL_STUB_MESSAGE, pmemory : *mut u8, formatchar : u8));
    unsafe { NdrSimpleTypeMarshall(core::mem::transmute(pstubmsg), pmemory as _, formatchar) }
}
#[cfg(feature = "objidlbase")]
#[inline]
pub unsafe fn NdrSimpleTypeUnmarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, formatchar: u8) {
    windows_core::link!("rpcrt4.dll" "system" fn NdrSimpleTypeUnmarshall(pstubmsg : *mut MIDL_STUB_MESSAGE, pmemory : *mut u8, formatchar : u8));
    unsafe { NdrSimpleTypeUnmarshall(core::mem::transmute(pstubmsg), pmemory as _, formatchar) }
}
#[inline]
pub unsafe fn NdrStubCall2(pthis: *mut core::ffi::c_void, pchannel: *mut core::ffi::c_void, prpcmsg: *mut RPC_MESSAGE, pdwstubphase: *mut u32) -> i32 {
    windows_core::link!("rpcrt4.dll" "system" fn NdrStubCall2(pthis : *mut core::ffi::c_void, pchannel : *mut core::ffi::c_void, prpcmsg : *mut RPC_MESSAGE, pdwstubphase : *mut u32) -> i32);
    unsafe { NdrStubCall2(pthis as _, pchannel as _, prpcmsg as _, pdwstubphase as _) }
}
#[inline]
pub unsafe fn NdrStubCall3(pthis: *mut core::ffi::c_void, pchannel: *mut core::ffi::c_void, prpcmsg: *mut RPC_MESSAGE, pdwstubphase: *mut u32) -> i32 {
    windows_core::link!("rpcrt4.dll" "system" fn NdrStubCall3(pthis : *mut core::ffi::c_void, pchannel : *mut core::ffi::c_void, prpcmsg : *mut RPC_MESSAGE, pdwstubphase : *mut u32) -> i32);
    unsafe { NdrStubCall3(pthis as _, pchannel as _, prpcmsg as _, pdwstubphase as _) }
}
#[cfg(feature = "objidlbase")]
#[inline]
pub unsafe fn NdrUserMarshalBufferSize(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *const u8) {
    windows_core::link!("rpcrt4.dll" "system" fn NdrUserMarshalBufferSize(pstubmsg : *mut MIDL_STUB_MESSAGE, pmemory : *mut u8, pformat : *const u8));
    unsafe { NdrUserMarshalBufferSize(core::mem::transmute(pstubmsg), pmemory as _, pformat) }
}
#[cfg(feature = "objidlbase")]
#[inline]
pub unsafe fn NdrUserMarshalFree(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *const u8) {
    windows_core::link!("rpcrt4.dll" "system" fn NdrUserMarshalFree(pstubmsg : *mut MIDL_STUB_MESSAGE, pmemory : *mut u8, pformat : *const u8));
    unsafe { NdrUserMarshalFree(core::mem::transmute(pstubmsg), pmemory as _, pformat) }
}
#[cfg(feature = "objidlbase")]
#[inline]
pub unsafe fn NdrUserMarshalMarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *const u8) -> *mut u8 {
    windows_core::link!("rpcrt4.dll" "system" fn NdrUserMarshalMarshall(pstubmsg : *mut MIDL_STUB_MESSAGE, pmemory : *mut u8, pformat : *const u8) -> *mut u8);
    unsafe { NdrUserMarshalMarshall(core::mem::transmute(pstubmsg), pmemory as _, pformat) }
}
#[cfg(feature = "objidlbase")]
#[inline]
pub unsafe fn NdrUserMarshalMemorySize(pstubmsg: *mut MIDL_STUB_MESSAGE, pformat: *const u8) -> u32 {
    windows_core::link!("rpcrt4.dll" "system" fn NdrUserMarshalMemorySize(pstubmsg : *mut MIDL_STUB_MESSAGE, pformat : *const u8) -> u32);
    unsafe { NdrUserMarshalMemorySize(core::mem::transmute(pstubmsg), pformat) }
}
#[inline]
pub unsafe fn NdrUserMarshalSimpleTypeConvert(pflags: *mut u32, pbuffer: *mut u8, formatchar: u8) -> *mut u8 {
    windows_core::link!("rpcrt4.dll" "system" fn NdrUserMarshalSimpleTypeConvert(pflags : *mut u32, pbuffer : *mut u8, formatchar : u8) -> *mut u8);
    unsafe { NdrUserMarshalSimpleTypeConvert(pflags as _, pbuffer as _, formatchar) }
}
#[cfg(feature = "objidlbase")]
#[inline]
pub unsafe fn NdrUserMarshalUnmarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, ppmemory: *mut *mut u8, pformat: *const u8, fmustalloc: u8) -> *mut u8 {
    windows_core::link!("rpcrt4.dll" "system" fn NdrUserMarshalUnmarshall(pstubmsg : *mut MIDL_STUB_MESSAGE, ppmemory : *mut *mut u8, pformat : *const u8, fmustalloc : u8) -> *mut u8);
    unsafe { NdrUserMarshalUnmarshall(core::mem::transmute(pstubmsg), ppmemory as _, pformat, fmustalloc) }
}
#[cfg(feature = "objidlbase")]
#[inline]
pub unsafe fn NdrVaryingArrayBufferSize(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *const u8) {
    windows_core::link!("rpcrt4.dll" "system" fn NdrVaryingArrayBufferSize(pstubmsg : *mut MIDL_STUB_MESSAGE, pmemory : *mut u8, pformat : *const u8));
    unsafe { NdrVaryingArrayBufferSize(core::mem::transmute(pstubmsg), pmemory as _, pformat) }
}
#[cfg(feature = "objidlbase")]
#[inline]
pub unsafe fn NdrVaryingArrayFree(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *const u8) {
    windows_core::link!("rpcrt4.dll" "system" fn NdrVaryingArrayFree(pstubmsg : *mut MIDL_STUB_MESSAGE, pmemory : *mut u8, pformat : *const u8));
    unsafe { NdrVaryingArrayFree(core::mem::transmute(pstubmsg), pmemory as _, pformat) }
}
#[cfg(feature = "objidlbase")]
#[inline]
pub unsafe fn NdrVaryingArrayMarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *const u8) -> *mut u8 {
    windows_core::link!("rpcrt4.dll" "system" fn NdrVaryingArrayMarshall(pstubmsg : *mut MIDL_STUB_MESSAGE, pmemory : *mut u8, pformat : *const u8) -> *mut u8);
    unsafe { NdrVaryingArrayMarshall(core::mem::transmute(pstubmsg), pmemory as _, pformat) }
}
#[cfg(feature = "objidlbase")]
#[inline]
pub unsafe fn NdrVaryingArrayMemorySize(pstubmsg: *mut MIDL_STUB_MESSAGE, pformat: *const u8) -> u32 {
    windows_core::link!("rpcrt4.dll" "system" fn NdrVaryingArrayMemorySize(pstubmsg : *mut MIDL_STUB_MESSAGE, pformat : *const u8) -> u32);
    unsafe { NdrVaryingArrayMemorySize(core::mem::transmute(pstubmsg), pformat) }
}
#[cfg(feature = "objidlbase")]
#[inline]
pub unsafe fn NdrVaryingArrayUnmarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, ppmemory: *mut *mut u8, pformat: *const u8, fmustalloc: u8) -> *mut u8 {
    windows_core::link!("rpcrt4.dll" "system" fn NdrVaryingArrayUnmarshall(pstubmsg : *mut MIDL_STUB_MESSAGE, ppmemory : *mut *mut u8, pformat : *const u8, fmustalloc : u8) -> *mut u8);
    unsafe { NdrVaryingArrayUnmarshall(core::mem::transmute(pstubmsg), ppmemory as _, pformat, fmustalloc) }
}
#[cfg(feature = "objidlbase")]
#[inline]
pub unsafe fn NdrXmitOrRepAsBufferSize(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *const u8) {
    windows_core::link!("rpcrt4.dll" "system" fn NdrXmitOrRepAsBufferSize(pstubmsg : *mut MIDL_STUB_MESSAGE, pmemory : *mut u8, pformat : *const u8));
    unsafe { NdrXmitOrRepAsBufferSize(core::mem::transmute(pstubmsg), pmemory as _, pformat) }
}
#[cfg(feature = "objidlbase")]
#[inline]
pub unsafe fn NdrXmitOrRepAsFree(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *const u8) {
    windows_core::link!("rpcrt4.dll" "system" fn NdrXmitOrRepAsFree(pstubmsg : *mut MIDL_STUB_MESSAGE, pmemory : *mut u8, pformat : *const u8));
    unsafe { NdrXmitOrRepAsFree(core::mem::transmute(pstubmsg), pmemory as _, pformat) }
}
#[cfg(feature = "objidlbase")]
#[inline]
pub unsafe fn NdrXmitOrRepAsMarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *const u8) -> *mut u8 {
    windows_core::link!("rpcrt4.dll" "system" fn NdrXmitOrRepAsMarshall(pstubmsg : *mut MIDL_STUB_MESSAGE, pmemory : *mut u8, pformat : *const u8) -> *mut u8);
    unsafe { NdrXmitOrRepAsMarshall(core::mem::transmute(pstubmsg), pmemory as _, pformat) }
}
#[cfg(feature = "objidlbase")]
#[inline]
pub unsafe fn NdrXmitOrRepAsMemorySize(pstubmsg: *mut MIDL_STUB_MESSAGE, pformat: *const u8) -> u32 {
    windows_core::link!("rpcrt4.dll" "system" fn NdrXmitOrRepAsMemorySize(pstubmsg : *mut MIDL_STUB_MESSAGE, pformat : *const u8) -> u32);
    unsafe { NdrXmitOrRepAsMemorySize(core::mem::transmute(pstubmsg), pformat) }
}
#[cfg(feature = "objidlbase")]
#[inline]
pub unsafe fn NdrXmitOrRepAsUnmarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, ppmemory: *mut *mut u8, pformat: *const u8, fmustalloc: u8) -> *mut u8 {
    windows_core::link!("rpcrt4.dll" "system" fn NdrXmitOrRepAsUnmarshall(pstubmsg : *mut MIDL_STUB_MESSAGE, ppmemory : *mut *mut u8, pformat : *const u8, fmustalloc : u8) -> *mut u8);
    unsafe { NdrXmitOrRepAsUnmarshall(core::mem::transmute(pstubmsg), ppmemory as _, pformat, fmustalloc) }
}
#[cfg(all(feature = "minwinbase", feature = "windef", feature = "winnt"))]
#[inline]
pub unsafe fn RpcAsyncAbortCall(pasync: *mut RPC_ASYNC_STATE, exceptioncode: u32) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcAsyncAbortCall(pasync : *mut RPC_ASYNC_STATE, exceptioncode : u32) -> RPC_STATUS);
    unsafe { RpcAsyncAbortCall(pasync as _, exceptioncode) }
}
#[cfg(all(feature = "minwinbase", feature = "windef", feature = "winnt"))]
#[inline]
pub unsafe fn RpcAsyncCancelCall(pasync: *mut RPC_ASYNC_STATE, fabort: bool) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcAsyncCancelCall(pasync : *mut RPC_ASYNC_STATE, fabort : windows_core::BOOL) -> RPC_STATUS);
    unsafe { RpcAsyncCancelCall(pasync as _, fabort.into()) }
}
#[cfg(all(feature = "minwinbase", feature = "windef", feature = "winnt"))]
#[inline]
pub unsafe fn RpcAsyncCompleteCall(pasync: *mut RPC_ASYNC_STATE, reply: Option<*mut core::ffi::c_void>) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcAsyncCompleteCall(pasync : *mut RPC_ASYNC_STATE, reply : *mut core::ffi::c_void) -> RPC_STATUS);
    unsafe { RpcAsyncCompleteCall(pasync as _, reply.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwinbase", feature = "windef", feature = "winnt"))]
#[inline]
pub unsafe fn RpcAsyncGetCallStatus(pasync: *const RPC_ASYNC_STATE) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcAsyncGetCallStatus(pasync : *const RPC_ASYNC_STATE) -> RPC_STATUS);
    unsafe { RpcAsyncGetCallStatus(pasync) }
}
#[cfg(all(feature = "minwinbase", feature = "windef", feature = "winnt"))]
#[inline]
pub unsafe fn RpcAsyncInitializeHandle(pasync: *mut RPC_ASYNC_STATE, size: u32) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcAsyncInitializeHandle(pasync : *mut RPC_ASYNC_STATE, size : u32) -> RPC_STATUS);
    unsafe { RpcAsyncInitializeHandle(pasync as _, size) }
}
#[cfg(all(feature = "minwinbase", feature = "windef", feature = "winnt"))]
#[inline]
pub unsafe fn RpcAsyncRegisterInfo(pasync: *const RPC_ASYNC_STATE) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcAsyncRegisterInfo(pasync : *const RPC_ASYNC_STATE) -> RPC_STATUS);
    unsafe { RpcAsyncRegisterInfo(pasync) }
}
#[cfg(all(feature = "minwinbase", feature = "windef", feature = "winnt"))]
#[inline]
pub unsafe fn RpcBindingBind(pasync: Option<*const RPC_ASYNC_STATE>, binding: RPC_BINDING_HANDLE, ifspec: RPC_IF_HANDLE) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcBindingBind(pasync : *const RPC_ASYNC_STATE, binding : RPC_BINDING_HANDLE, ifspec : RPC_IF_HANDLE) -> RPC_STATUS);
    unsafe { RpcBindingBind(pasync.unwrap_or(core::mem::zeroed()) as _, binding, ifspec) }
}
#[inline]
pub unsafe fn RpcBindingCopy(sourcebinding: RPC_BINDING_HANDLE, destinationbinding: *mut RPC_BINDING_HANDLE) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcBindingCopy(sourcebinding : RPC_BINDING_HANDLE, destinationbinding : *mut RPC_BINDING_HANDLE) -> RPC_STATUS);
    unsafe { RpcBindingCopy(sourcebinding, destinationbinding as _) }
}
#[inline]
pub unsafe fn RpcBindingCreateA(template: *const RPC_BINDING_HANDLE_TEMPLATE_V1_A, security: Option<*const RPC_BINDING_HANDLE_SECURITY_V1_A>, options: Option<*const RPC_BINDING_HANDLE_OPTIONS_V1>, binding: *mut RPC_BINDING_HANDLE) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcBindingCreateA(template : *const RPC_BINDING_HANDLE_TEMPLATE_V1_A, security : *const RPC_BINDING_HANDLE_SECURITY_V1_A, options : *const RPC_BINDING_HANDLE_OPTIONS_V1, binding : *mut RPC_BINDING_HANDLE) -> RPC_STATUS);
    unsafe { RpcBindingCreateA(template, security.unwrap_or(core::mem::zeroed()) as _, options.unwrap_or(core::mem::zeroed()) as _, binding as _) }
}
#[inline]
pub unsafe fn RpcBindingCreateW(template: *const RPC_BINDING_HANDLE_TEMPLATE_V1_W, security: Option<*const RPC_BINDING_HANDLE_SECURITY_V1_W>, options: Option<*const RPC_BINDING_HANDLE_OPTIONS_V1>, binding: *mut RPC_BINDING_HANDLE) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcBindingCreateW(template : *const RPC_BINDING_HANDLE_TEMPLATE_V1_W, security : *const RPC_BINDING_HANDLE_SECURITY_V1_W, options : *const RPC_BINDING_HANDLE_OPTIONS_V1, binding : *mut RPC_BINDING_HANDLE) -> RPC_STATUS);
    unsafe { RpcBindingCreateW(template, security.unwrap_or(core::mem::zeroed()) as _, options.unwrap_or(core::mem::zeroed()) as _, binding as _) }
}
#[inline]
pub unsafe fn RpcBindingFree(binding: *mut RPC_BINDING_HANDLE) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcBindingFree(binding : *mut RPC_BINDING_HANDLE) -> RPC_STATUS);
    unsafe { RpcBindingFree(binding as _) }
}
#[inline]
pub unsafe fn RpcBindingFromStringBindingA(stringbinding: *const u8, binding: *mut RPC_BINDING_HANDLE) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcBindingFromStringBindingA(stringbinding : *const u8, binding : *mut RPC_BINDING_HANDLE) -> RPC_STATUS);
    unsafe { RpcBindingFromStringBindingA(stringbinding, binding as _) }
}
#[inline]
pub unsafe fn RpcBindingFromStringBindingW(stringbinding: *const u16, binding: *mut RPC_BINDING_HANDLE) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcBindingFromStringBindingW(stringbinding : *const u16, binding : *mut RPC_BINDING_HANDLE) -> RPC_STATUS);
    unsafe { RpcBindingFromStringBindingW(stringbinding, binding as _) }
}
#[inline]
pub unsafe fn RpcBindingInqAuthClientA(clientbinding: Option<RPC_BINDING_HANDLE>, privs: *mut RPC_AUTHZ_HANDLE, serverprincname: *mut RPC_CSTR, authnlevel: Option<*mut u32>, authnsvc: Option<*mut u32>, authzsvc: Option<*mut u32>) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcBindingInqAuthClientA(clientbinding : RPC_BINDING_HANDLE, privs : *mut RPC_AUTHZ_HANDLE, serverprincname : *mut RPC_CSTR, authnlevel : *mut u32, authnsvc : *mut u32, authzsvc : *mut u32) -> RPC_STATUS);
    unsafe { RpcBindingInqAuthClientA(clientbinding.unwrap_or(core::mem::zeroed()) as _, privs as _, serverprincname as _, authnlevel.unwrap_or(core::mem::zeroed()) as _, authnsvc.unwrap_or(core::mem::zeroed()) as _, authzsvc.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn RpcBindingInqAuthClientExA(clientbinding: Option<RPC_BINDING_HANDLE>, privs: *mut RPC_AUTHZ_HANDLE, serverprincname: *mut RPC_CSTR, authnlevel: Option<*mut u32>, authnsvc: Option<*mut u32>, authzsvc: Option<*mut u32>, flags: u32) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcBindingInqAuthClientExA(clientbinding : RPC_BINDING_HANDLE, privs : *mut RPC_AUTHZ_HANDLE, serverprincname : *mut RPC_CSTR, authnlevel : *mut u32, authnsvc : *mut u32, authzsvc : *mut u32, flags : u32) -> RPC_STATUS);
    unsafe { RpcBindingInqAuthClientExA(clientbinding.unwrap_or(core::mem::zeroed()) as _, privs as _, serverprincname as _, authnlevel.unwrap_or(core::mem::zeroed()) as _, authnsvc.unwrap_or(core::mem::zeroed()) as _, authzsvc.unwrap_or(core::mem::zeroed()) as _, flags) }
}
#[inline]
pub unsafe fn RpcBindingInqAuthClientExW(clientbinding: Option<RPC_BINDING_HANDLE>, privs: *mut RPC_AUTHZ_HANDLE, serverprincname: *mut RPC_WSTR, authnlevel: Option<*mut u32>, authnsvc: Option<*mut u32>, authzsvc: Option<*mut u32>, flags: u32) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcBindingInqAuthClientExW(clientbinding : RPC_BINDING_HANDLE, privs : *mut RPC_AUTHZ_HANDLE, serverprincname : *mut RPC_WSTR, authnlevel : *mut u32, authnsvc : *mut u32, authzsvc : *mut u32, flags : u32) -> RPC_STATUS);
    unsafe { RpcBindingInqAuthClientExW(clientbinding.unwrap_or(core::mem::zeroed()) as _, privs as _, serverprincname as _, authnlevel.unwrap_or(core::mem::zeroed()) as _, authnsvc.unwrap_or(core::mem::zeroed()) as _, authzsvc.unwrap_or(core::mem::zeroed()) as _, flags) }
}
#[inline]
pub unsafe fn RpcBindingInqAuthClientW(clientbinding: Option<RPC_BINDING_HANDLE>, privs: *mut RPC_AUTHZ_HANDLE, serverprincname: *mut RPC_WSTR, authnlevel: Option<*mut u32>, authnsvc: Option<*mut u32>, authzsvc: Option<*mut u32>) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcBindingInqAuthClientW(clientbinding : RPC_BINDING_HANDLE, privs : *mut RPC_AUTHZ_HANDLE, serverprincname : *mut RPC_WSTR, authnlevel : *mut u32, authnsvc : *mut u32, authzsvc : *mut u32) -> RPC_STATUS);
    unsafe { RpcBindingInqAuthClientW(clientbinding.unwrap_or(core::mem::zeroed()) as _, privs as _, serverprincname as _, authnlevel.unwrap_or(core::mem::zeroed()) as _, authnsvc.unwrap_or(core::mem::zeroed()) as _, authzsvc.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn RpcBindingInqAuthInfoA(binding: RPC_BINDING_HANDLE, serverprincname: *mut RPC_CSTR, authnlevel: Option<*mut u32>, authnsvc: Option<*mut u32>, authidentity: Option<*mut RPC_AUTH_IDENTITY_HANDLE>, authzsvc: Option<*mut u32>) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcBindingInqAuthInfoA(binding : RPC_BINDING_HANDLE, serverprincname : *mut RPC_CSTR, authnlevel : *mut u32, authnsvc : *mut u32, authidentity : *mut RPC_AUTH_IDENTITY_HANDLE, authzsvc : *mut u32) -> RPC_STATUS);
    unsafe { RpcBindingInqAuthInfoA(binding, serverprincname as _, authnlevel.unwrap_or(core::mem::zeroed()) as _, authnsvc.unwrap_or(core::mem::zeroed()) as _, authidentity.unwrap_or(core::mem::zeroed()) as _, authzsvc.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn RpcBindingInqAuthInfoExA(binding: RPC_BINDING_HANDLE, serverprincname: *mut RPC_CSTR, authnlevel: Option<*mut u32>, authnsvc: Option<*mut u32>, authidentity: Option<*mut RPC_AUTH_IDENTITY_HANDLE>, authzsvc: Option<*mut u32>, rpcqosversion: u32, securityqos: Option<*mut RPC_SECURITY_QOS>) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcBindingInqAuthInfoExA(binding : RPC_BINDING_HANDLE, serverprincname : *mut RPC_CSTR, authnlevel : *mut u32, authnsvc : *mut u32, authidentity : *mut RPC_AUTH_IDENTITY_HANDLE, authzsvc : *mut u32, rpcqosversion : u32, securityqos : *mut RPC_SECURITY_QOS) -> RPC_STATUS);
    unsafe { RpcBindingInqAuthInfoExA(binding, serverprincname as _, authnlevel.unwrap_or(core::mem::zeroed()) as _, authnsvc.unwrap_or(core::mem::zeroed()) as _, authidentity.unwrap_or(core::mem::zeroed()) as _, authzsvc.unwrap_or(core::mem::zeroed()) as _, rpcqosversion, securityqos.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn RpcBindingInqAuthInfoExW(binding: RPC_BINDING_HANDLE, serverprincname: *mut RPC_WSTR, authnlevel: Option<*mut u32>, authnsvc: Option<*mut u32>, authidentity: Option<*mut RPC_AUTH_IDENTITY_HANDLE>, authzsvc: Option<*mut u32>, rpcqosversion: u32, securityqos: Option<*mut RPC_SECURITY_QOS>) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcBindingInqAuthInfoExW(binding : RPC_BINDING_HANDLE, serverprincname : *mut RPC_WSTR, authnlevel : *mut u32, authnsvc : *mut u32, authidentity : *mut RPC_AUTH_IDENTITY_HANDLE, authzsvc : *mut u32, rpcqosversion : u32, securityqos : *mut RPC_SECURITY_QOS) -> RPC_STATUS);
    unsafe { RpcBindingInqAuthInfoExW(binding, serverprincname as _, authnlevel.unwrap_or(core::mem::zeroed()) as _, authnsvc.unwrap_or(core::mem::zeroed()) as _, authidentity.unwrap_or(core::mem::zeroed()) as _, authzsvc.unwrap_or(core::mem::zeroed()) as _, rpcqosversion, securityqos.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn RpcBindingInqAuthInfoW(binding: RPC_BINDING_HANDLE, serverprincname: *mut RPC_WSTR, authnlevel: Option<*mut u32>, authnsvc: Option<*mut u32>, authidentity: Option<*mut RPC_AUTH_IDENTITY_HANDLE>, authzsvc: Option<*mut u32>) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcBindingInqAuthInfoW(binding : RPC_BINDING_HANDLE, serverprincname : *mut RPC_WSTR, authnlevel : *mut u32, authnsvc : *mut u32, authidentity : *mut RPC_AUTH_IDENTITY_HANDLE, authzsvc : *mut u32) -> RPC_STATUS);
    unsafe { RpcBindingInqAuthInfoW(binding, serverprincname as _, authnlevel.unwrap_or(core::mem::zeroed()) as _, authnsvc.unwrap_or(core::mem::zeroed()) as _, authidentity.unwrap_or(core::mem::zeroed()) as _, authzsvc.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn RpcBindingInqMaxCalls(binding: RPC_BINDING_HANDLE, maxcalls: *mut u32) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcBindingInqMaxCalls(binding : RPC_BINDING_HANDLE, maxcalls : *mut u32) -> RPC_STATUS);
    unsafe { RpcBindingInqMaxCalls(binding, maxcalls as _) }
}
#[inline]
pub unsafe fn RpcBindingInqObject(binding: RPC_BINDING_HANDLE, objectuuid: *mut windows_core::GUID) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcBindingInqObject(binding : RPC_BINDING_HANDLE, objectuuid : *mut windows_core::GUID) -> RPC_STATUS);
    unsafe { RpcBindingInqObject(binding, objectuuid as _) }
}
#[inline]
pub unsafe fn RpcBindingInqOption(hbinding: RPC_BINDING_HANDLE, option: u32, poptionvalue: *mut usize) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcBindingInqOption(hbinding : RPC_BINDING_HANDLE, option : u32, poptionvalue : *mut usize) -> RPC_STATUS);
    unsafe { RpcBindingInqOption(hbinding, option, poptionvalue as _) }
}
#[inline]
pub unsafe fn RpcBindingReset(binding: RPC_BINDING_HANDLE) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcBindingReset(binding : RPC_BINDING_HANDLE) -> RPC_STATUS);
    unsafe { RpcBindingReset(binding) }
}
#[inline]
pub unsafe fn RpcBindingServerFromClient(clientbinding: Option<RPC_BINDING_HANDLE>, serverbinding: *mut RPC_BINDING_HANDLE) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcBindingServerFromClient(clientbinding : RPC_BINDING_HANDLE, serverbinding : *mut RPC_BINDING_HANDLE) -> RPC_STATUS);
    unsafe { RpcBindingServerFromClient(clientbinding.unwrap_or(core::mem::zeroed()) as _, serverbinding as _) }
}
#[inline]
pub unsafe fn RpcBindingSetAuthInfoA(binding: RPC_BINDING_HANDLE, serverprincname: Option<*const u8>, authnlevel: u32, authnsvc: u32, authidentity: Option<RPC_AUTH_IDENTITY_HANDLE>, authzsvc: u32) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcBindingSetAuthInfoA(binding : RPC_BINDING_HANDLE, serverprincname : *const u8, authnlevel : u32, authnsvc : u32, authidentity : RPC_AUTH_IDENTITY_HANDLE, authzsvc : u32) -> RPC_STATUS);
    unsafe { RpcBindingSetAuthInfoA(binding, serverprincname.unwrap_or(core::mem::zeroed()) as _, authnlevel, authnsvc, authidentity.unwrap_or(core::mem::zeroed()) as _, authzsvc) }
}
#[inline]
pub unsafe fn RpcBindingSetAuthInfoExA(binding: RPC_BINDING_HANDLE, serverprincname: Option<*const u8>, authnlevel: u32, authnsvc: u32, authidentity: Option<RPC_AUTH_IDENTITY_HANDLE>, authzsvc: u32, securityqos: Option<*const RPC_SECURITY_QOS>) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcBindingSetAuthInfoExA(binding : RPC_BINDING_HANDLE, serverprincname : *const u8, authnlevel : u32, authnsvc : u32, authidentity : RPC_AUTH_IDENTITY_HANDLE, authzsvc : u32, securityqos : *const RPC_SECURITY_QOS) -> RPC_STATUS);
    unsafe { RpcBindingSetAuthInfoExA(binding, serverprincname.unwrap_or(core::mem::zeroed()) as _, authnlevel, authnsvc, authidentity.unwrap_or(core::mem::zeroed()) as _, authzsvc, securityqos.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn RpcBindingSetAuthInfoExW(binding: RPC_BINDING_HANDLE, serverprincname: Option<*const u16>, authnlevel: u32, authnsvc: u32, authidentity: Option<RPC_AUTH_IDENTITY_HANDLE>, authzsvc: u32, securityqos: Option<*const RPC_SECURITY_QOS>) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcBindingSetAuthInfoExW(binding : RPC_BINDING_HANDLE, serverprincname : *const u16, authnlevel : u32, authnsvc : u32, authidentity : RPC_AUTH_IDENTITY_HANDLE, authzsvc : u32, securityqos : *const RPC_SECURITY_QOS) -> RPC_STATUS);
    unsafe { RpcBindingSetAuthInfoExW(binding, serverprincname.unwrap_or(core::mem::zeroed()) as _, authnlevel, authnsvc, authidentity.unwrap_or(core::mem::zeroed()) as _, authzsvc, securityqos.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn RpcBindingSetAuthInfoW(binding: RPC_BINDING_HANDLE, serverprincname: Option<*const u16>, authnlevel: u32, authnsvc: u32, authidentity: Option<RPC_AUTH_IDENTITY_HANDLE>, authzsvc: u32) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcBindingSetAuthInfoW(binding : RPC_BINDING_HANDLE, serverprincname : *const u16, authnlevel : u32, authnsvc : u32, authidentity : RPC_AUTH_IDENTITY_HANDLE, authzsvc : u32) -> RPC_STATUS);
    unsafe { RpcBindingSetAuthInfoW(binding, serverprincname.unwrap_or(core::mem::zeroed()) as _, authnlevel, authnsvc, authidentity.unwrap_or(core::mem::zeroed()) as _, authzsvc) }
}
#[inline]
pub unsafe fn RpcBindingSetObject(binding: RPC_BINDING_HANDLE, objectuuid: *const windows_core::GUID) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcBindingSetObject(binding : RPC_BINDING_HANDLE, objectuuid : *const windows_core::GUID) -> RPC_STATUS);
    unsafe { RpcBindingSetObject(binding, objectuuid) }
}
#[inline]
pub unsafe fn RpcBindingSetOption(hbinding: RPC_BINDING_HANDLE, option: u32, optionvalue: usize) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcBindingSetOption(hbinding : RPC_BINDING_HANDLE, option : u32, optionvalue : usize) -> RPC_STATUS);
    unsafe { RpcBindingSetOption(hbinding, option, optionvalue) }
}
#[inline]
pub unsafe fn RpcBindingToStringBindingA(binding: RPC_BINDING_HANDLE, stringbinding: *mut RPC_CSTR) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcBindingToStringBindingA(binding : RPC_BINDING_HANDLE, stringbinding : *mut RPC_CSTR) -> RPC_STATUS);
    unsafe { RpcBindingToStringBindingA(binding, stringbinding as _) }
}
#[inline]
pub unsafe fn RpcBindingToStringBindingW(binding: RPC_BINDING_HANDLE, stringbinding: *mut RPC_WSTR) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcBindingToStringBindingW(binding : RPC_BINDING_HANDLE, stringbinding : *mut RPC_WSTR) -> RPC_STATUS);
    unsafe { RpcBindingToStringBindingW(binding, stringbinding as _) }
}
#[inline]
pub unsafe fn RpcBindingUnbind(binding: RPC_BINDING_HANDLE) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcBindingUnbind(binding : RPC_BINDING_HANDLE) -> RPC_STATUS);
    unsafe { RpcBindingUnbind(binding) }
}
#[inline]
pub unsafe fn RpcBindingVectorFree(bindingvector: *mut *mut RPC_BINDING_VECTOR) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcBindingVectorFree(bindingvector : *mut *mut RPC_BINDING_VECTOR) -> RPC_STATUS);
    unsafe { RpcBindingVectorFree(bindingvector as _) }
}
#[inline]
pub unsafe fn RpcCancelThread(thread: *const core::ffi::c_void) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcCancelThread(thread : *const core::ffi::c_void) -> RPC_STATUS);
    unsafe { RpcCancelThread(thread) }
}
#[inline]
pub unsafe fn RpcCancelThreadEx(thread: *const core::ffi::c_void, timeout: i32) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcCancelThreadEx(thread : *const core::ffi::c_void, timeout : i32) -> RPC_STATUS);
    unsafe { RpcCancelThreadEx(thread, timeout) }
}
#[inline]
pub unsafe fn RpcEpRegisterA(ifspec: RPC_IF_HANDLE, bindingvector: *const RPC_BINDING_VECTOR, uuidvector: Option<*const UUID_VECTOR>, annotation: Option<*const u8>) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcEpRegisterA(ifspec : RPC_IF_HANDLE, bindingvector : *const RPC_BINDING_VECTOR, uuidvector : *const UUID_VECTOR, annotation : *const u8) -> RPC_STATUS);
    unsafe { RpcEpRegisterA(ifspec, bindingvector, uuidvector.unwrap_or(core::mem::zeroed()) as _, annotation.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn RpcEpRegisterNoReplaceA(ifspec: RPC_IF_HANDLE, bindingvector: *const RPC_BINDING_VECTOR, uuidvector: Option<*const UUID_VECTOR>, annotation: Option<*const u8>) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcEpRegisterNoReplaceA(ifspec : RPC_IF_HANDLE, bindingvector : *const RPC_BINDING_VECTOR, uuidvector : *const UUID_VECTOR, annotation : *const u8) -> RPC_STATUS);
    unsafe { RpcEpRegisterNoReplaceA(ifspec, bindingvector, uuidvector.unwrap_or(core::mem::zeroed()) as _, annotation.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn RpcEpRegisterNoReplaceW(ifspec: RPC_IF_HANDLE, bindingvector: *const RPC_BINDING_VECTOR, uuidvector: Option<*const UUID_VECTOR>, annotation: Option<*const u16>) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcEpRegisterNoReplaceW(ifspec : RPC_IF_HANDLE, bindingvector : *const RPC_BINDING_VECTOR, uuidvector : *const UUID_VECTOR, annotation : *const u16) -> RPC_STATUS);
    unsafe { RpcEpRegisterNoReplaceW(ifspec, bindingvector, uuidvector.unwrap_or(core::mem::zeroed()) as _, annotation.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn RpcEpRegisterW(ifspec: RPC_IF_HANDLE, bindingvector: *const RPC_BINDING_VECTOR, uuidvector: Option<*const UUID_VECTOR>, annotation: Option<*const u16>) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcEpRegisterW(ifspec : RPC_IF_HANDLE, bindingvector : *const RPC_BINDING_VECTOR, uuidvector : *const UUID_VECTOR, annotation : *const u16) -> RPC_STATUS);
    unsafe { RpcEpRegisterW(ifspec, bindingvector, uuidvector.unwrap_or(core::mem::zeroed()) as _, annotation.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn RpcEpResolveBinding(binding: RPC_BINDING_HANDLE, ifspec: RPC_IF_HANDLE) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcEpResolveBinding(binding : RPC_BINDING_HANDLE, ifspec : RPC_IF_HANDLE) -> RPC_STATUS);
    unsafe { RpcEpResolveBinding(binding, ifspec) }
}
#[inline]
pub unsafe fn RpcEpUnregister(ifspec: RPC_IF_HANDLE, bindingvector: *const RPC_BINDING_VECTOR, uuidvector: Option<*const UUID_VECTOR>) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcEpUnregister(ifspec : RPC_IF_HANDLE, bindingvector : *const RPC_BINDING_VECTOR, uuidvector : *const UUID_VECTOR) -> RPC_STATUS);
    unsafe { RpcEpUnregister(ifspec, bindingvector, uuidvector.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwinbase", feature = "minwindef"))]
#[inline]
pub unsafe fn RpcErrorAddRecord(errorinfo: *const RPC_EXTENDED_ERROR_INFO) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcErrorAddRecord(errorinfo : *const RPC_EXTENDED_ERROR_INFO) -> RPC_STATUS);
    unsafe { RpcErrorAddRecord(errorinfo) }
}
#[inline]
pub unsafe fn RpcErrorClearInformation() {
    windows_core::link!("rpcrt4.dll" "system" fn RpcErrorClearInformation());
    unsafe { RpcErrorClearInformation() }
}
#[inline]
pub unsafe fn RpcErrorEndEnumeration(enumhandle: *mut RPC_ERROR_ENUM_HANDLE) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcErrorEndEnumeration(enumhandle : *mut RPC_ERROR_ENUM_HANDLE) -> RPC_STATUS);
    unsafe { RpcErrorEndEnumeration(enumhandle as _) }
}
#[cfg(all(feature = "minwinbase", feature = "minwindef"))]
#[inline]
pub unsafe fn RpcErrorGetNextRecord(enumhandle: *const RPC_ERROR_ENUM_HANDLE, copystrings: bool, errorinfo: *mut RPC_EXTENDED_ERROR_INFO) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcErrorGetNextRecord(enumhandle : *const RPC_ERROR_ENUM_HANDLE, copystrings : windows_core::BOOL, errorinfo : *mut RPC_EXTENDED_ERROR_INFO) -> RPC_STATUS);
    unsafe { RpcErrorGetNextRecord(enumhandle, copystrings.into(), errorinfo as _) }
}
#[inline]
pub unsafe fn RpcErrorGetNumberOfRecords(enumhandle: *const RPC_ERROR_ENUM_HANDLE, records: *mut i32) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcErrorGetNumberOfRecords(enumhandle : *const RPC_ERROR_ENUM_HANDLE, records : *mut i32) -> RPC_STATUS);
    unsafe { RpcErrorGetNumberOfRecords(enumhandle, records as _) }
}
#[inline]
pub unsafe fn RpcErrorLoadErrorInfo(errorblob: *const core::ffi::c_void, blobsize: usize, enumhandle: *mut RPC_ERROR_ENUM_HANDLE) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcErrorLoadErrorInfo(errorblob : *const core::ffi::c_void, blobsize : usize, enumhandle : *mut RPC_ERROR_ENUM_HANDLE) -> RPC_STATUS);
    unsafe { RpcErrorLoadErrorInfo(errorblob, blobsize, enumhandle as _) }
}
#[inline]
pub unsafe fn RpcErrorResetEnumeration(enumhandle: *mut RPC_ERROR_ENUM_HANDLE) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcErrorResetEnumeration(enumhandle : *mut RPC_ERROR_ENUM_HANDLE) -> RPC_STATUS);
    unsafe { RpcErrorResetEnumeration(enumhandle as _) }
}
#[inline]
pub unsafe fn RpcErrorSaveErrorInfo(enumhandle: *const RPC_ERROR_ENUM_HANDLE, errorblob: *mut *mut core::ffi::c_void, blobsize: *mut usize) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcErrorSaveErrorInfo(enumhandle : *const RPC_ERROR_ENUM_HANDLE, errorblob : *mut *mut core::ffi::c_void, blobsize : *mut usize) -> RPC_STATUS);
    unsafe { RpcErrorSaveErrorInfo(enumhandle, errorblob as _, blobsize as _) }
}
#[inline]
pub unsafe fn RpcErrorStartEnumeration(enumhandle: *mut RPC_ERROR_ENUM_HANDLE) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcErrorStartEnumeration(enumhandle : *mut RPC_ERROR_ENUM_HANDLE) -> RPC_STATUS);
    unsafe { RpcErrorStartEnumeration(enumhandle as _) }
}
#[inline]
pub unsafe fn RpcExceptionFilter(exceptioncode: u32) -> i32 {
    windows_core::link!("rpcrt4.dll" "system" fn RpcExceptionFilter(exceptioncode : u32) -> i32);
    unsafe { RpcExceptionFilter(exceptioncode) }
}
#[inline]
pub unsafe fn RpcFreeAuthorizationContext(pauthzclientcontext: *mut *mut core::ffi::c_void) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcFreeAuthorizationContext(pauthzclientcontext : *mut *mut core::ffi::c_void) -> RPC_STATUS);
    unsafe { RpcFreeAuthorizationContext(pauthzclientcontext as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn RpcGetAuthorizationContextForClient(clientbinding: Option<RPC_BINDING_HANDLE>, impersonateonreturn: bool, reserved1: Option<*const core::ffi::c_void>, pexpirationtime: Option<*const i64>, reserved2: super::winnt::LUID, reserved3: u32, reserved4: Option<*const core::ffi::c_void>, pauthzclientcontext: *mut *mut core::ffi::c_void) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcGetAuthorizationContextForClient(clientbinding : RPC_BINDING_HANDLE, impersonateonreturn : windows_core::BOOL, reserved1 : *const core::ffi::c_void, pexpirationtime : *const i64, reserved2 : super::winnt::LUID, reserved3 : u32, reserved4 : *const core::ffi::c_void, pauthzclientcontext : *mut *mut core::ffi::c_void) -> RPC_STATUS);
    unsafe { RpcGetAuthorizationContextForClient(clientbinding.unwrap_or(core::mem::zeroed()) as _, impersonateonreturn.into(), reserved1.unwrap_or(core::mem::zeroed()) as _, pexpirationtime.unwrap_or(core::mem::zeroed()) as _, core::mem::transmute(reserved2), reserved3, reserved4.unwrap_or(core::mem::zeroed()) as _, pauthzclientcontext as _) }
}
#[inline]
pub unsafe fn RpcIfIdVectorFree(ifidvector: *mut *mut RPC_IF_ID_VECTOR) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcIfIdVectorFree(ifidvector : *mut *mut RPC_IF_ID_VECTOR) -> RPC_STATUS);
    unsafe { RpcIfIdVectorFree(ifidvector as _) }
}
#[inline]
pub unsafe fn RpcIfInqId(rpcifhandle: RPC_IF_HANDLE, rpcifid: *mut RPC_IF_ID) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcIfInqId(rpcifhandle : RPC_IF_HANDLE, rpcifid : *mut RPC_IF_ID) -> RPC_STATUS);
    unsafe { RpcIfInqId(rpcifhandle, rpcifid as _) }
}
#[inline]
pub unsafe fn RpcImpersonateClient(bindinghandle: Option<RPC_BINDING_HANDLE>) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcImpersonateClient(bindinghandle : RPC_BINDING_HANDLE) -> RPC_STATUS);
    unsafe { RpcImpersonateClient(bindinghandle.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn RpcImpersonateClient2(bindinghandle: Option<RPC_BINDING_HANDLE>) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcImpersonateClient2(bindinghandle : RPC_BINDING_HANDLE) -> RPC_STATUS);
    unsafe { RpcImpersonateClient2(bindinghandle.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn RpcImpersonateClientContainer(bindinghandle: Option<RPC_BINDING_HANDLE>) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcImpersonateClientContainer(bindinghandle : RPC_BINDING_HANDLE) -> RPC_STATUS);
    unsafe { RpcImpersonateClientContainer(bindinghandle.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn RpcMgmtEnableIdleCleanup() -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcMgmtEnableIdleCleanup() -> RPC_STATUS);
    unsafe { RpcMgmtEnableIdleCleanup() }
}
#[inline]
pub unsafe fn RpcMgmtEpEltInqBegin(epbinding: Option<RPC_BINDING_HANDLE>, inquirytype: u32, ifid: Option<*const RPC_IF_ID>, versoption: Option<u32>, objectuuid: Option<*const windows_core::GUID>, inquirycontext: *mut RPC_EP_INQ_HANDLE) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcMgmtEpEltInqBegin(epbinding : RPC_BINDING_HANDLE, inquirytype : u32, ifid : *const RPC_IF_ID, versoption : u32, objectuuid : *const windows_core::GUID, inquirycontext : *mut RPC_EP_INQ_HANDLE) -> RPC_STATUS);
    unsafe { RpcMgmtEpEltInqBegin(epbinding.unwrap_or(core::mem::zeroed()) as _, inquirytype, ifid.unwrap_or(core::mem::zeroed()) as _, versoption.unwrap_or(core::mem::zeroed()) as _, objectuuid.unwrap_or(core::mem::zeroed()) as _, inquirycontext as _) }
}
#[inline]
pub unsafe fn RpcMgmtEpEltInqDone(inquirycontext: *mut RPC_EP_INQ_HANDLE) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcMgmtEpEltInqDone(inquirycontext : *mut RPC_EP_INQ_HANDLE) -> RPC_STATUS);
    unsafe { RpcMgmtEpEltInqDone(inquirycontext as _) }
}
#[inline]
pub unsafe fn RpcMgmtEpEltInqNextA(inquirycontext: *const I_RPC_HANDLE, ifid: *mut RPC_IF_ID, binding: Option<*mut RPC_BINDING_HANDLE>, objectuuid: Option<*mut windows_core::GUID>, annotation: *mut RPC_CSTR) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcMgmtEpEltInqNextA(inquirycontext : *const I_RPC_HANDLE, ifid : *mut RPC_IF_ID, binding : *mut RPC_BINDING_HANDLE, objectuuid : *mut windows_core::GUID, annotation : *mut RPC_CSTR) -> RPC_STATUS);
    unsafe { RpcMgmtEpEltInqNextA(inquirycontext, ifid as _, binding.unwrap_or(core::mem::zeroed()) as _, objectuuid.unwrap_or(core::mem::zeroed()) as _, annotation as _) }
}
#[inline]
pub unsafe fn RpcMgmtEpEltInqNextW(inquirycontext: *const I_RPC_HANDLE, ifid: *mut RPC_IF_ID, binding: Option<*mut RPC_BINDING_HANDLE>, objectuuid: Option<*mut windows_core::GUID>, annotation: *mut RPC_WSTR) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcMgmtEpEltInqNextW(inquirycontext : *const I_RPC_HANDLE, ifid : *mut RPC_IF_ID, binding : *mut RPC_BINDING_HANDLE, objectuuid : *mut windows_core::GUID, annotation : *mut RPC_WSTR) -> RPC_STATUS);
    unsafe { RpcMgmtEpEltInqNextW(inquirycontext, ifid as _, binding.unwrap_or(core::mem::zeroed()) as _, objectuuid.unwrap_or(core::mem::zeroed()) as _, annotation as _) }
}
#[inline]
pub unsafe fn RpcMgmtEpUnregister(epbinding: Option<RPC_BINDING_HANDLE>, ifid: *const RPC_IF_ID, binding: RPC_BINDING_HANDLE, objectuuid: Option<*const windows_core::GUID>) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcMgmtEpUnregister(epbinding : RPC_BINDING_HANDLE, ifid : *const RPC_IF_ID, binding : RPC_BINDING_HANDLE, objectuuid : *const windows_core::GUID) -> RPC_STATUS);
    unsafe { RpcMgmtEpUnregister(epbinding.unwrap_or(core::mem::zeroed()) as _, ifid, binding, objectuuid.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn RpcMgmtInqComTimeout(binding: RPC_BINDING_HANDLE, timeout: *mut u32) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcMgmtInqComTimeout(binding : RPC_BINDING_HANDLE, timeout : *mut u32) -> RPC_STATUS);
    unsafe { RpcMgmtInqComTimeout(binding, timeout as _) }
}
#[inline]
pub unsafe fn RpcMgmtInqDefaultProtectLevel(authnsvc: u32, authnlevel: *mut u32) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcMgmtInqDefaultProtectLevel(authnsvc : u32, authnlevel : *mut u32) -> RPC_STATUS);
    unsafe { RpcMgmtInqDefaultProtectLevel(authnsvc, authnlevel as _) }
}
#[inline]
pub unsafe fn RpcMgmtInqIfIds(binding: Option<RPC_BINDING_HANDLE>, ifidvector: *mut *mut RPC_IF_ID_VECTOR) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcMgmtInqIfIds(binding : RPC_BINDING_HANDLE, ifidvector : *mut *mut RPC_IF_ID_VECTOR) -> RPC_STATUS);
    unsafe { RpcMgmtInqIfIds(binding.unwrap_or(core::mem::zeroed()) as _, ifidvector as _) }
}
#[inline]
pub unsafe fn RpcMgmtInqServerPrincNameA(binding: Option<RPC_BINDING_HANDLE>, authnsvc: u32, serverprincname: *mut RPC_CSTR) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcMgmtInqServerPrincNameA(binding : RPC_BINDING_HANDLE, authnsvc : u32, serverprincname : *mut RPC_CSTR) -> RPC_STATUS);
    unsafe { RpcMgmtInqServerPrincNameA(binding.unwrap_or(core::mem::zeroed()) as _, authnsvc, serverprincname as _) }
}
#[inline]
pub unsafe fn RpcMgmtInqServerPrincNameW(binding: Option<RPC_BINDING_HANDLE>, authnsvc: u32, serverprincname: *mut RPC_WSTR) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcMgmtInqServerPrincNameW(binding : RPC_BINDING_HANDLE, authnsvc : u32, serverprincname : *mut RPC_WSTR) -> RPC_STATUS);
    unsafe { RpcMgmtInqServerPrincNameW(binding.unwrap_or(core::mem::zeroed()) as _, authnsvc, serverprincname as _) }
}
#[inline]
pub unsafe fn RpcMgmtInqStats(binding: Option<RPC_BINDING_HANDLE>, statistics: *mut *mut RPC_STATS_VECTOR) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcMgmtInqStats(binding : RPC_BINDING_HANDLE, statistics : *mut *mut RPC_STATS_VECTOR) -> RPC_STATUS);
    unsafe { RpcMgmtInqStats(binding.unwrap_or(core::mem::zeroed()) as _, statistics as _) }
}
#[inline]
pub unsafe fn RpcMgmtIsServerListening(binding: Option<RPC_BINDING_HANDLE>) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcMgmtIsServerListening(binding : RPC_BINDING_HANDLE) -> RPC_STATUS);
    unsafe { RpcMgmtIsServerListening(binding.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn RpcMgmtSetAuthorizationFn(authorizationfn: RPC_MGMT_AUTHORIZATION_FN) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcMgmtSetAuthorizationFn(authorizationfn : RPC_MGMT_AUTHORIZATION_FN) -> RPC_STATUS);
    unsafe { RpcMgmtSetAuthorizationFn(authorizationfn) }
}
#[inline]
pub unsafe fn RpcMgmtSetCancelTimeout(timeout: i32) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcMgmtSetCancelTimeout(timeout : i32) -> RPC_STATUS);
    unsafe { RpcMgmtSetCancelTimeout(timeout) }
}
#[inline]
pub unsafe fn RpcMgmtSetComTimeout(binding: RPC_BINDING_HANDLE, timeout: u32) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcMgmtSetComTimeout(binding : RPC_BINDING_HANDLE, timeout : u32) -> RPC_STATUS);
    unsafe { RpcMgmtSetComTimeout(binding, timeout) }
}
#[inline]
pub unsafe fn RpcMgmtSetServerStackSize(threadstacksize: u32) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcMgmtSetServerStackSize(threadstacksize : u32) -> RPC_STATUS);
    unsafe { RpcMgmtSetServerStackSize(threadstacksize) }
}
#[inline]
pub unsafe fn RpcMgmtStatsVectorFree(statsvector: *mut *mut RPC_STATS_VECTOR) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcMgmtStatsVectorFree(statsvector : *mut *mut RPC_STATS_VECTOR) -> RPC_STATUS);
    unsafe { RpcMgmtStatsVectorFree(statsvector as _) }
}
#[inline]
pub unsafe fn RpcMgmtStopServerListening(binding: Option<RPC_BINDING_HANDLE>) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcMgmtStopServerListening(binding : RPC_BINDING_HANDLE) -> RPC_STATUS);
    unsafe { RpcMgmtStopServerListening(binding.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn RpcMgmtWaitServerListen() -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcMgmtWaitServerListen() -> RPC_STATUS);
    unsafe { RpcMgmtWaitServerListen() }
}
#[inline]
pub unsafe fn RpcNetworkInqProtseqsA(protseqvector: *mut *mut RPC_PROTSEQ_VECTORA) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcNetworkInqProtseqsA(protseqvector : *mut *mut RPC_PROTSEQ_VECTORA) -> RPC_STATUS);
    unsafe { RpcNetworkInqProtseqsA(protseqvector as _) }
}
#[inline]
pub unsafe fn RpcNetworkInqProtseqsW(protseqvector: *mut *mut RPC_PROTSEQ_VECTORW) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcNetworkInqProtseqsW(protseqvector : *mut *mut RPC_PROTSEQ_VECTORW) -> RPC_STATUS);
    unsafe { RpcNetworkInqProtseqsW(protseqvector as _) }
}
#[inline]
pub unsafe fn RpcNetworkIsProtseqValidA(protseq: *const u8) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcNetworkIsProtseqValidA(protseq : *const u8) -> RPC_STATUS);
    unsafe { RpcNetworkIsProtseqValidA(protseq) }
}
#[inline]
pub unsafe fn RpcNetworkIsProtseqValidW(protseq: *const u16) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcNetworkIsProtseqValidW(protseq : *const u16) -> RPC_STATUS);
    unsafe { RpcNetworkIsProtseqValidW(protseq) }
}
#[inline]
pub unsafe fn RpcNsBindingExportA(entrynamesyntax: u32, entryname: Option<*const u8>, ifspec: Option<RPC_IF_HANDLE>, bindingvec: Option<*const RPC_BINDING_VECTOR>, objectuuidvec: Option<*const UUID_VECTOR>) -> RPC_STATUS {
    windows_core::link!("rpcns4.dll" "system" fn RpcNsBindingExportA(entrynamesyntax : u32, entryname : *const u8, ifspec : RPC_IF_HANDLE, bindingvec : *const RPC_BINDING_VECTOR, objectuuidvec : *const UUID_VECTOR) -> RPC_STATUS);
    unsafe { RpcNsBindingExportA(entrynamesyntax, entryname.unwrap_or(core::mem::zeroed()) as _, ifspec.unwrap_or(core::mem::zeroed()) as _, bindingvec.unwrap_or(core::mem::zeroed()) as _, objectuuidvec.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn RpcNsBindingExportPnPA(entrynamesyntax: u32, entryname: Option<*const u8>, ifspec: Option<RPC_IF_HANDLE>, objectvector: Option<*const UUID_VECTOR>) -> RPC_STATUS {
    windows_core::link!("rpcns4.dll" "system" fn RpcNsBindingExportPnPA(entrynamesyntax : u32, entryname : *const u8, ifspec : RPC_IF_HANDLE, objectvector : *const UUID_VECTOR) -> RPC_STATUS);
    unsafe { RpcNsBindingExportPnPA(entrynamesyntax, entryname.unwrap_or(core::mem::zeroed()) as _, ifspec.unwrap_or(core::mem::zeroed()) as _, objectvector.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn RpcNsBindingExportPnPW(entrynamesyntax: u32, entryname: Option<*const u16>, ifspec: Option<RPC_IF_HANDLE>, objectvector: Option<*const UUID_VECTOR>) -> RPC_STATUS {
    windows_core::link!("rpcns4.dll" "system" fn RpcNsBindingExportPnPW(entrynamesyntax : u32, entryname : *const u16, ifspec : RPC_IF_HANDLE, objectvector : *const UUID_VECTOR) -> RPC_STATUS);
    unsafe { RpcNsBindingExportPnPW(entrynamesyntax, entryname.unwrap_or(core::mem::zeroed()) as _, ifspec.unwrap_or(core::mem::zeroed()) as _, objectvector.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn RpcNsBindingExportW(entrynamesyntax: u32, entryname: Option<*const u16>, ifspec: Option<RPC_IF_HANDLE>, bindingvec: Option<*const RPC_BINDING_VECTOR>, objectuuidvec: Option<*const UUID_VECTOR>) -> RPC_STATUS {
    windows_core::link!("rpcns4.dll" "system" fn RpcNsBindingExportW(entrynamesyntax : u32, entryname : *const u16, ifspec : RPC_IF_HANDLE, bindingvec : *const RPC_BINDING_VECTOR, objectuuidvec : *const UUID_VECTOR) -> RPC_STATUS);
    unsafe { RpcNsBindingExportW(entrynamesyntax, entryname.unwrap_or(core::mem::zeroed()) as _, ifspec.unwrap_or(core::mem::zeroed()) as _, bindingvec.unwrap_or(core::mem::zeroed()) as _, objectuuidvec.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn RpcNsBindingImportBeginA(entrynamesyntax: u32, entryname: Option<*const u8>, ifspec: Option<RPC_IF_HANDLE>, objuuid: Option<*const windows_core::GUID>, importcontext: *mut RPC_NS_HANDLE) -> RPC_STATUS {
    windows_core::link!("rpcns4.dll" "system" fn RpcNsBindingImportBeginA(entrynamesyntax : u32, entryname : *const u8, ifspec : RPC_IF_HANDLE, objuuid : *const windows_core::GUID, importcontext : *mut RPC_NS_HANDLE) -> RPC_STATUS);
    unsafe { RpcNsBindingImportBeginA(entrynamesyntax, entryname.unwrap_or(core::mem::zeroed()) as _, ifspec.unwrap_or(core::mem::zeroed()) as _, objuuid.unwrap_or(core::mem::zeroed()) as _, importcontext as _) }
}
#[inline]
pub unsafe fn RpcNsBindingImportBeginW(entrynamesyntax: u32, entryname: Option<*const u16>, ifspec: Option<RPC_IF_HANDLE>, objuuid: Option<*const windows_core::GUID>, importcontext: *mut RPC_NS_HANDLE) -> RPC_STATUS {
    windows_core::link!("rpcns4.dll" "system" fn RpcNsBindingImportBeginW(entrynamesyntax : u32, entryname : *const u16, ifspec : RPC_IF_HANDLE, objuuid : *const windows_core::GUID, importcontext : *mut RPC_NS_HANDLE) -> RPC_STATUS);
    unsafe { RpcNsBindingImportBeginW(entrynamesyntax, entryname.unwrap_or(core::mem::zeroed()) as _, ifspec.unwrap_or(core::mem::zeroed()) as _, objuuid.unwrap_or(core::mem::zeroed()) as _, importcontext as _) }
}
#[inline]
pub unsafe fn RpcNsBindingImportDone(importcontext: *mut RPC_NS_HANDLE) -> RPC_STATUS {
    windows_core::link!("rpcns4.dll" "system" fn RpcNsBindingImportDone(importcontext : *mut RPC_NS_HANDLE) -> RPC_STATUS);
    unsafe { RpcNsBindingImportDone(importcontext as _) }
}
#[inline]
pub unsafe fn RpcNsBindingImportNext(importcontext: RPC_NS_HANDLE, binding: *mut RPC_BINDING_HANDLE) -> RPC_STATUS {
    windows_core::link!("rpcns4.dll" "system" fn RpcNsBindingImportNext(importcontext : RPC_NS_HANDLE, binding : *mut RPC_BINDING_HANDLE) -> RPC_STATUS);
    unsafe { RpcNsBindingImportNext(importcontext, binding as _) }
}
#[inline]
pub unsafe fn RpcNsBindingInqEntryNameA(binding: RPC_BINDING_HANDLE, entrynamesyntax: u32, entryname: *mut RPC_CSTR) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcNsBindingInqEntryNameA(binding : RPC_BINDING_HANDLE, entrynamesyntax : u32, entryname : *mut RPC_CSTR) -> RPC_STATUS);
    unsafe { RpcNsBindingInqEntryNameA(binding, entrynamesyntax, entryname as _) }
}
#[inline]
pub unsafe fn RpcNsBindingInqEntryNameW(binding: RPC_BINDING_HANDLE, entrynamesyntax: u32, entryname: *mut RPC_WSTR) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcNsBindingInqEntryNameW(binding : RPC_BINDING_HANDLE, entrynamesyntax : u32, entryname : *mut RPC_WSTR) -> RPC_STATUS);
    unsafe { RpcNsBindingInqEntryNameW(binding, entrynamesyntax, entryname as _) }
}
#[inline]
pub unsafe fn RpcNsBindingLookupBeginA(entrynamesyntax: u32, entryname: Option<*const u8>, ifspec: Option<RPC_IF_HANDLE>, objuuid: Option<*const windows_core::GUID>, bindingmaxcount: u32, lookupcontext: *mut RPC_NS_HANDLE) -> RPC_STATUS {
    windows_core::link!("rpcns4.dll" "system" fn RpcNsBindingLookupBeginA(entrynamesyntax : u32, entryname : *const u8, ifspec : RPC_IF_HANDLE, objuuid : *const windows_core::GUID, bindingmaxcount : u32, lookupcontext : *mut RPC_NS_HANDLE) -> RPC_STATUS);
    unsafe { RpcNsBindingLookupBeginA(entrynamesyntax, entryname.unwrap_or(core::mem::zeroed()) as _, ifspec.unwrap_or(core::mem::zeroed()) as _, objuuid.unwrap_or(core::mem::zeroed()) as _, bindingmaxcount, lookupcontext as _) }
}
#[inline]
pub unsafe fn RpcNsBindingLookupBeginW(entrynamesyntax: u32, entryname: Option<*const u16>, ifspec: Option<RPC_IF_HANDLE>, objuuid: Option<*const windows_core::GUID>, bindingmaxcount: u32, lookupcontext: *mut RPC_NS_HANDLE) -> RPC_STATUS {
    windows_core::link!("rpcns4.dll" "system" fn RpcNsBindingLookupBeginW(entrynamesyntax : u32, entryname : *const u16, ifspec : RPC_IF_HANDLE, objuuid : *const windows_core::GUID, bindingmaxcount : u32, lookupcontext : *mut RPC_NS_HANDLE) -> RPC_STATUS);
    unsafe { RpcNsBindingLookupBeginW(entrynamesyntax, entryname.unwrap_or(core::mem::zeroed()) as _, ifspec.unwrap_or(core::mem::zeroed()) as _, objuuid.unwrap_or(core::mem::zeroed()) as _, bindingmaxcount, lookupcontext as _) }
}
#[inline]
pub unsafe fn RpcNsBindingLookupDone(lookupcontext: *mut RPC_NS_HANDLE) -> RPC_STATUS {
    windows_core::link!("rpcns4.dll" "system" fn RpcNsBindingLookupDone(lookupcontext : *mut RPC_NS_HANDLE) -> RPC_STATUS);
    unsafe { RpcNsBindingLookupDone(lookupcontext as _) }
}
#[inline]
pub unsafe fn RpcNsBindingLookupNext(lookupcontext: RPC_NS_HANDLE, bindingvec: *mut *mut RPC_BINDING_VECTOR) -> RPC_STATUS {
    windows_core::link!("rpcns4.dll" "system" fn RpcNsBindingLookupNext(lookupcontext : RPC_NS_HANDLE, bindingvec : *mut *mut RPC_BINDING_VECTOR) -> RPC_STATUS);
    unsafe { RpcNsBindingLookupNext(lookupcontext, bindingvec as _) }
}
#[inline]
pub unsafe fn RpcNsBindingSelect(bindingvec: *mut RPC_BINDING_VECTOR, binding: *mut RPC_BINDING_HANDLE) -> RPC_STATUS {
    windows_core::link!("rpcns4.dll" "system" fn RpcNsBindingSelect(bindingvec : *mut RPC_BINDING_VECTOR, binding : *mut RPC_BINDING_HANDLE) -> RPC_STATUS);
    unsafe { RpcNsBindingSelect(bindingvec as _, binding as _) }
}
#[inline]
pub unsafe fn RpcNsBindingUnexportA(entrynamesyntax: u32, entryname: Option<*const u8>, ifspec: Option<RPC_IF_HANDLE>, objectuuidvec: Option<*const UUID_VECTOR>) -> RPC_STATUS {
    windows_core::link!("rpcns4.dll" "system" fn RpcNsBindingUnexportA(entrynamesyntax : u32, entryname : *const u8, ifspec : RPC_IF_HANDLE, objectuuidvec : *const UUID_VECTOR) -> RPC_STATUS);
    unsafe { RpcNsBindingUnexportA(entrynamesyntax, entryname.unwrap_or(core::mem::zeroed()) as _, ifspec.unwrap_or(core::mem::zeroed()) as _, objectuuidvec.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn RpcNsBindingUnexportPnPA(entrynamesyntax: u32, entryname: Option<*const u8>, ifspec: Option<RPC_IF_HANDLE>, objectvector: Option<*const UUID_VECTOR>) -> RPC_STATUS {
    windows_core::link!("rpcns4.dll" "system" fn RpcNsBindingUnexportPnPA(entrynamesyntax : u32, entryname : *const u8, ifspec : RPC_IF_HANDLE, objectvector : *const UUID_VECTOR) -> RPC_STATUS);
    unsafe { RpcNsBindingUnexportPnPA(entrynamesyntax, entryname.unwrap_or(core::mem::zeroed()) as _, ifspec.unwrap_or(core::mem::zeroed()) as _, objectvector.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn RpcNsBindingUnexportPnPW(entrynamesyntax: u32, entryname: Option<*const u16>, ifspec: Option<RPC_IF_HANDLE>, objectvector: Option<*const UUID_VECTOR>) -> RPC_STATUS {
    windows_core::link!("rpcns4.dll" "system" fn RpcNsBindingUnexportPnPW(entrynamesyntax : u32, entryname : *const u16, ifspec : RPC_IF_HANDLE, objectvector : *const UUID_VECTOR) -> RPC_STATUS);
    unsafe { RpcNsBindingUnexportPnPW(entrynamesyntax, entryname.unwrap_or(core::mem::zeroed()) as _, ifspec.unwrap_or(core::mem::zeroed()) as _, objectvector.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn RpcNsBindingUnexportW(entrynamesyntax: u32, entryname: Option<*const u16>, ifspec: Option<RPC_IF_HANDLE>, objectuuidvec: Option<*const UUID_VECTOR>) -> RPC_STATUS {
    windows_core::link!("rpcns4.dll" "system" fn RpcNsBindingUnexportW(entrynamesyntax : u32, entryname : *const u16, ifspec : RPC_IF_HANDLE, objectuuidvec : *const UUID_VECTOR) -> RPC_STATUS);
    unsafe { RpcNsBindingUnexportW(entrynamesyntax, entryname.unwrap_or(core::mem::zeroed()) as _, ifspec.unwrap_or(core::mem::zeroed()) as _, objectuuidvec.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn RpcNsEntryExpandNameA(entrynamesyntax: u32, entryname: *const u8, expandedname: *mut RPC_CSTR) -> RPC_STATUS {
    windows_core::link!("rpcns4.dll" "system" fn RpcNsEntryExpandNameA(entrynamesyntax : u32, entryname : *const u8, expandedname : *mut RPC_CSTR) -> RPC_STATUS);
    unsafe { RpcNsEntryExpandNameA(entrynamesyntax, entryname, expandedname as _) }
}
#[inline]
pub unsafe fn RpcNsEntryExpandNameW(entrynamesyntax: u32, entryname: *const u16, expandedname: *mut RPC_WSTR) -> RPC_STATUS {
    windows_core::link!("rpcns4.dll" "system" fn RpcNsEntryExpandNameW(entrynamesyntax : u32, entryname : *const u16, expandedname : *mut RPC_WSTR) -> RPC_STATUS);
    unsafe { RpcNsEntryExpandNameW(entrynamesyntax, entryname, expandedname as _) }
}
#[inline]
pub unsafe fn RpcNsEntryObjectInqBeginA(entrynamesyntax: u32, entryname: *const u8, inquirycontext: *mut RPC_NS_HANDLE) -> RPC_STATUS {
    windows_core::link!("rpcns4.dll" "system" fn RpcNsEntryObjectInqBeginA(entrynamesyntax : u32, entryname : *const u8, inquirycontext : *mut RPC_NS_HANDLE) -> RPC_STATUS);
    unsafe { RpcNsEntryObjectInqBeginA(entrynamesyntax, entryname, inquirycontext as _) }
}
#[inline]
pub unsafe fn RpcNsEntryObjectInqBeginW(entrynamesyntax: u32, entryname: *const u16, inquirycontext: *mut RPC_NS_HANDLE) -> RPC_STATUS {
    windows_core::link!("rpcns4.dll" "system" fn RpcNsEntryObjectInqBeginW(entrynamesyntax : u32, entryname : *const u16, inquirycontext : *mut RPC_NS_HANDLE) -> RPC_STATUS);
    unsafe { RpcNsEntryObjectInqBeginW(entrynamesyntax, entryname, inquirycontext as _) }
}
#[inline]
pub unsafe fn RpcNsEntryObjectInqDone(inquirycontext: *mut RPC_NS_HANDLE) -> RPC_STATUS {
    windows_core::link!("rpcns4.dll" "system" fn RpcNsEntryObjectInqDone(inquirycontext : *mut RPC_NS_HANDLE) -> RPC_STATUS);
    unsafe { RpcNsEntryObjectInqDone(inquirycontext as _) }
}
#[inline]
pub unsafe fn RpcNsEntryObjectInqNext(inquirycontext: RPC_NS_HANDLE, objuuid: *mut windows_core::GUID) -> RPC_STATUS {
    windows_core::link!("rpcns4.dll" "system" fn RpcNsEntryObjectInqNext(inquirycontext : RPC_NS_HANDLE, objuuid : *mut windows_core::GUID) -> RPC_STATUS);
    unsafe { RpcNsEntryObjectInqNext(inquirycontext, objuuid as _) }
}
#[inline]
pub unsafe fn RpcNsGroupDeleteA(groupnamesyntax: u32, groupname: Option<*const u8>) -> RPC_STATUS {
    windows_core::link!("rpcns4.dll" "system" fn RpcNsGroupDeleteA(groupnamesyntax : u32, groupname : *const u8) -> RPC_STATUS);
    unsafe { RpcNsGroupDeleteA(groupnamesyntax, groupname.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn RpcNsGroupDeleteW(groupnamesyntax: u32, groupname: Option<*const u16>) -> RPC_STATUS {
    windows_core::link!("rpcns4.dll" "system" fn RpcNsGroupDeleteW(groupnamesyntax : u32, groupname : *const u16) -> RPC_STATUS);
    unsafe { RpcNsGroupDeleteW(groupnamesyntax, groupname.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn RpcNsGroupMbrAddA(groupnamesyntax: u32, groupname: *const u8, membernamesyntax: u32, membername: *const u8) -> RPC_STATUS {
    windows_core::link!("rpcns4.dll" "system" fn RpcNsGroupMbrAddA(groupnamesyntax : u32, groupname : *const u8, membernamesyntax : u32, membername : *const u8) -> RPC_STATUS);
    unsafe { RpcNsGroupMbrAddA(groupnamesyntax, groupname, membernamesyntax, membername) }
}
#[inline]
pub unsafe fn RpcNsGroupMbrAddW(groupnamesyntax: u32, groupname: *const u16, membernamesyntax: u32, membername: *const u16) -> RPC_STATUS {
    windows_core::link!("rpcns4.dll" "system" fn RpcNsGroupMbrAddW(groupnamesyntax : u32, groupname : *const u16, membernamesyntax : u32, membername : *const u16) -> RPC_STATUS);
    unsafe { RpcNsGroupMbrAddW(groupnamesyntax, groupname, membernamesyntax, membername) }
}
#[inline]
pub unsafe fn RpcNsGroupMbrInqBeginA(groupnamesyntax: u32, groupname: *const u8, membernamesyntax: u32, inquirycontext: *mut RPC_NS_HANDLE) -> RPC_STATUS {
    windows_core::link!("rpcns4.dll" "system" fn RpcNsGroupMbrInqBeginA(groupnamesyntax : u32, groupname : *const u8, membernamesyntax : u32, inquirycontext : *mut RPC_NS_HANDLE) -> RPC_STATUS);
    unsafe { RpcNsGroupMbrInqBeginA(groupnamesyntax, groupname, membernamesyntax, inquirycontext as _) }
}
#[inline]
pub unsafe fn RpcNsGroupMbrInqBeginW(groupnamesyntax: u32, groupname: *const u16, membernamesyntax: u32, inquirycontext: *mut RPC_NS_HANDLE) -> RPC_STATUS {
    windows_core::link!("rpcns4.dll" "system" fn RpcNsGroupMbrInqBeginW(groupnamesyntax : u32, groupname : *const u16, membernamesyntax : u32, inquirycontext : *mut RPC_NS_HANDLE) -> RPC_STATUS);
    unsafe { RpcNsGroupMbrInqBeginW(groupnamesyntax, groupname, membernamesyntax, inquirycontext as _) }
}
#[inline]
pub unsafe fn RpcNsGroupMbrInqDone(inquirycontext: *mut RPC_NS_HANDLE) -> RPC_STATUS {
    windows_core::link!("rpcns4.dll" "system" fn RpcNsGroupMbrInqDone(inquirycontext : *mut RPC_NS_HANDLE) -> RPC_STATUS);
    unsafe { RpcNsGroupMbrInqDone(inquirycontext as _) }
}
#[inline]
pub unsafe fn RpcNsGroupMbrInqNextA(inquirycontext: RPC_NS_HANDLE, membername: *mut RPC_CSTR) -> RPC_STATUS {
    windows_core::link!("rpcns4.dll" "system" fn RpcNsGroupMbrInqNextA(inquirycontext : RPC_NS_HANDLE, membername : *mut RPC_CSTR) -> RPC_STATUS);
    unsafe { RpcNsGroupMbrInqNextA(inquirycontext as _, membername as _) }
}
#[inline]
pub unsafe fn RpcNsGroupMbrInqNextW(inquirycontext: RPC_NS_HANDLE, membername: *mut RPC_WSTR) -> RPC_STATUS {
    windows_core::link!("rpcns4.dll" "system" fn RpcNsGroupMbrInqNextW(inquirycontext : RPC_NS_HANDLE, membername : *mut RPC_WSTR) -> RPC_STATUS);
    unsafe { RpcNsGroupMbrInqNextW(inquirycontext as _, membername as _) }
}
#[inline]
pub unsafe fn RpcNsGroupMbrRemoveA(groupnamesyntax: u32, groupname: *const u8, membernamesyntax: u32, membername: *const u8) -> RPC_STATUS {
    windows_core::link!("rpcns4.dll" "system" fn RpcNsGroupMbrRemoveA(groupnamesyntax : u32, groupname : *const u8, membernamesyntax : u32, membername : *const u8) -> RPC_STATUS);
    unsafe { RpcNsGroupMbrRemoveA(groupnamesyntax, groupname, membernamesyntax, membername) }
}
#[inline]
pub unsafe fn RpcNsGroupMbrRemoveW(groupnamesyntax: u32, groupname: *const u16, membernamesyntax: u32, membername: *const u16) -> RPC_STATUS {
    windows_core::link!("rpcns4.dll" "system" fn RpcNsGroupMbrRemoveW(groupnamesyntax : u32, groupname : *const u16, membernamesyntax : u32, membername : *const u16) -> RPC_STATUS);
    unsafe { RpcNsGroupMbrRemoveW(groupnamesyntax, groupname, membernamesyntax, membername) }
}
#[inline]
pub unsafe fn RpcNsMgmtBindingUnexportA(entrynamesyntax: u32, entryname: *const u8, ifid: Option<*const RPC_IF_ID>, versoption: u32, objectuuidvec: Option<*const UUID_VECTOR>) -> RPC_STATUS {
    windows_core::link!("rpcns4.dll" "system" fn RpcNsMgmtBindingUnexportA(entrynamesyntax : u32, entryname : *const u8, ifid : *const RPC_IF_ID, versoption : u32, objectuuidvec : *const UUID_VECTOR) -> RPC_STATUS);
    unsafe { RpcNsMgmtBindingUnexportA(entrynamesyntax, entryname, ifid.unwrap_or(core::mem::zeroed()) as _, versoption, objectuuidvec.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn RpcNsMgmtBindingUnexportW(entrynamesyntax: u32, entryname: *const u16, ifid: Option<*const RPC_IF_ID>, versoption: u32, objectuuidvec: Option<*const UUID_VECTOR>) -> RPC_STATUS {
    windows_core::link!("rpcns4.dll" "system" fn RpcNsMgmtBindingUnexportW(entrynamesyntax : u32, entryname : *const u16, ifid : *const RPC_IF_ID, versoption : u32, objectuuidvec : *const UUID_VECTOR) -> RPC_STATUS);
    unsafe { RpcNsMgmtBindingUnexportW(entrynamesyntax, entryname, ifid.unwrap_or(core::mem::zeroed()) as _, versoption, objectuuidvec.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn RpcNsMgmtEntryCreateA(entrynamesyntax: u32, entryname: *const u8) -> RPC_STATUS {
    windows_core::link!("rpcns4.dll" "system" fn RpcNsMgmtEntryCreateA(entrynamesyntax : u32, entryname : *const u8) -> RPC_STATUS);
    unsafe { RpcNsMgmtEntryCreateA(entrynamesyntax, entryname) }
}
#[inline]
pub unsafe fn RpcNsMgmtEntryCreateW(entrynamesyntax: u32, entryname: *const u16) -> RPC_STATUS {
    windows_core::link!("rpcns4.dll" "system" fn RpcNsMgmtEntryCreateW(entrynamesyntax : u32, entryname : *const u16) -> RPC_STATUS);
    unsafe { RpcNsMgmtEntryCreateW(entrynamesyntax, entryname) }
}
#[inline]
pub unsafe fn RpcNsMgmtEntryDeleteA(entrynamesyntax: u32, entryname: *const u8) -> RPC_STATUS {
    windows_core::link!("rpcns4.dll" "system" fn RpcNsMgmtEntryDeleteA(entrynamesyntax : u32, entryname : *const u8) -> RPC_STATUS);
    unsafe { RpcNsMgmtEntryDeleteA(entrynamesyntax, entryname) }
}
#[inline]
pub unsafe fn RpcNsMgmtEntryDeleteW(entrynamesyntax: u32, entryname: *const u16) -> RPC_STATUS {
    windows_core::link!("rpcns4.dll" "system" fn RpcNsMgmtEntryDeleteW(entrynamesyntax : u32, entryname : *const u16) -> RPC_STATUS);
    unsafe { RpcNsMgmtEntryDeleteW(entrynamesyntax, entryname) }
}
#[inline]
pub unsafe fn RpcNsMgmtEntryInqIfIdsA(entrynamesyntax: u32, entryname: *const u8, ifidvec: *mut *mut RPC_IF_ID_VECTOR) -> RPC_STATUS {
    windows_core::link!("rpcns4.dll" "system" fn RpcNsMgmtEntryInqIfIdsA(entrynamesyntax : u32, entryname : *const u8, ifidvec : *mut *mut RPC_IF_ID_VECTOR) -> RPC_STATUS);
    unsafe { RpcNsMgmtEntryInqIfIdsA(entrynamesyntax, entryname, ifidvec as _) }
}
#[inline]
pub unsafe fn RpcNsMgmtEntryInqIfIdsW(entrynamesyntax: u32, entryname: *const u16, ifidvec: *mut *mut RPC_IF_ID_VECTOR) -> RPC_STATUS {
    windows_core::link!("rpcns4.dll" "system" fn RpcNsMgmtEntryInqIfIdsW(entrynamesyntax : u32, entryname : *const u16, ifidvec : *mut *mut RPC_IF_ID_VECTOR) -> RPC_STATUS);
    unsafe { RpcNsMgmtEntryInqIfIdsW(entrynamesyntax, entryname, ifidvec as _) }
}
#[inline]
pub unsafe fn RpcNsMgmtHandleSetExpAge(nshandle: RPC_NS_HANDLE, expirationage: u32) -> RPC_STATUS {
    windows_core::link!("rpcns4.dll" "system" fn RpcNsMgmtHandleSetExpAge(nshandle : RPC_NS_HANDLE, expirationage : u32) -> RPC_STATUS);
    unsafe { RpcNsMgmtHandleSetExpAge(nshandle, expirationage) }
}
#[inline]
pub unsafe fn RpcNsMgmtInqExpAge(expirationage: *mut u32) -> RPC_STATUS {
    windows_core::link!("rpcns4.dll" "system" fn RpcNsMgmtInqExpAge(expirationage : *mut u32) -> RPC_STATUS);
    unsafe { RpcNsMgmtInqExpAge(expirationage as _) }
}
#[inline]
pub unsafe fn RpcNsMgmtSetExpAge(expirationage: u32) -> RPC_STATUS {
    windows_core::link!("rpcns4.dll" "system" fn RpcNsMgmtSetExpAge(expirationage : u32) -> RPC_STATUS);
    unsafe { RpcNsMgmtSetExpAge(expirationage) }
}
#[inline]
pub unsafe fn RpcNsProfileDeleteA(profilenamesyntax: u32, profilename: *const u8) -> RPC_STATUS {
    windows_core::link!("rpcns4.dll" "system" fn RpcNsProfileDeleteA(profilenamesyntax : u32, profilename : *const u8) -> RPC_STATUS);
    unsafe { RpcNsProfileDeleteA(profilenamesyntax, profilename) }
}
#[inline]
pub unsafe fn RpcNsProfileDeleteW(profilenamesyntax: u32, profilename: *const u16) -> RPC_STATUS {
    windows_core::link!("rpcns4.dll" "system" fn RpcNsProfileDeleteW(profilenamesyntax : u32, profilename : *const u16) -> RPC_STATUS);
    unsafe { RpcNsProfileDeleteW(profilenamesyntax, profilename) }
}
#[inline]
pub unsafe fn RpcNsProfileEltAddA(profilenamesyntax: u32, profilename: *const u8, ifid: Option<*const RPC_IF_ID>, membernamesyntax: u32, membername: *const u8, priority: u32, annotation: Option<*const u8>) -> RPC_STATUS {
    windows_core::link!("rpcns4.dll" "system" fn RpcNsProfileEltAddA(profilenamesyntax : u32, profilename : *const u8, ifid : *const RPC_IF_ID, membernamesyntax : u32, membername : *const u8, priority : u32, annotation : *const u8) -> RPC_STATUS);
    unsafe { RpcNsProfileEltAddA(profilenamesyntax, profilename, ifid.unwrap_or(core::mem::zeroed()) as _, membernamesyntax, membername, priority, annotation.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn RpcNsProfileEltAddW(profilenamesyntax: u32, profilename: *const u16, ifid: Option<*const RPC_IF_ID>, membernamesyntax: u32, membername: *const u16, priority: u32, annotation: Option<*const u16>) -> RPC_STATUS {
    windows_core::link!("rpcns4.dll" "system" fn RpcNsProfileEltAddW(profilenamesyntax : u32, profilename : *const u16, ifid : *const RPC_IF_ID, membernamesyntax : u32, membername : *const u16, priority : u32, annotation : *const u16) -> RPC_STATUS);
    unsafe { RpcNsProfileEltAddW(profilenamesyntax, profilename, ifid.unwrap_or(core::mem::zeroed()) as _, membernamesyntax, membername, priority, annotation.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn RpcNsProfileEltInqBeginA(profilenamesyntax: u32, profilename: *const u8, inquirytype: u32, ifid: Option<*const RPC_IF_ID>, versoption: u32, membernamesyntax: u32, membername: Option<*const u8>, inquirycontext: *mut RPC_NS_HANDLE) -> RPC_STATUS {
    windows_core::link!("rpcns4.dll" "system" fn RpcNsProfileEltInqBeginA(profilenamesyntax : u32, profilename : *const u8, inquirytype : u32, ifid : *const RPC_IF_ID, versoption : u32, membernamesyntax : u32, membername : *const u8, inquirycontext : *mut RPC_NS_HANDLE) -> RPC_STATUS);
    unsafe { RpcNsProfileEltInqBeginA(profilenamesyntax, profilename, inquirytype, ifid.unwrap_or(core::mem::zeroed()) as _, versoption, membernamesyntax, membername.unwrap_or(core::mem::zeroed()) as _, inquirycontext as _) }
}
#[inline]
pub unsafe fn RpcNsProfileEltInqBeginW(profilenamesyntax: u32, profilename: *const u16, inquirytype: u32, ifid: Option<*const RPC_IF_ID>, versoption: u32, membernamesyntax: u32, membername: Option<*const u16>, inquirycontext: *mut RPC_NS_HANDLE) -> RPC_STATUS {
    windows_core::link!("rpcns4.dll" "system" fn RpcNsProfileEltInqBeginW(profilenamesyntax : u32, profilename : *const u16, inquirytype : u32, ifid : *const RPC_IF_ID, versoption : u32, membernamesyntax : u32, membername : *const u16, inquirycontext : *mut RPC_NS_HANDLE) -> RPC_STATUS);
    unsafe { RpcNsProfileEltInqBeginW(profilenamesyntax, profilename, inquirytype, ifid.unwrap_or(core::mem::zeroed()) as _, versoption, membernamesyntax, membername.unwrap_or(core::mem::zeroed()) as _, inquirycontext as _) }
}
#[inline]
pub unsafe fn RpcNsProfileEltInqDone(inquirycontext: *mut RPC_NS_HANDLE) -> RPC_STATUS {
    windows_core::link!("rpcns4.dll" "system" fn RpcNsProfileEltInqDone(inquirycontext : *mut RPC_NS_HANDLE) -> RPC_STATUS);
    unsafe { RpcNsProfileEltInqDone(inquirycontext as _) }
}
#[inline]
pub unsafe fn RpcNsProfileEltInqNextA(inquirycontext: RPC_NS_HANDLE, ifid: Option<*mut RPC_IF_ID>, membername: *mut RPC_CSTR, priority: *mut u32, annotation: *mut RPC_CSTR) -> RPC_STATUS {
    windows_core::link!("rpcns4.dll" "system" fn RpcNsProfileEltInqNextA(inquirycontext : RPC_NS_HANDLE, ifid : *mut RPC_IF_ID, membername : *mut RPC_CSTR, priority : *mut u32, annotation : *mut RPC_CSTR) -> RPC_STATUS);
    unsafe { RpcNsProfileEltInqNextA(inquirycontext, ifid.unwrap_or(core::mem::zeroed()) as _, membername as _, priority as _, annotation as _) }
}
#[inline]
pub unsafe fn RpcNsProfileEltInqNextW(inquirycontext: RPC_NS_HANDLE, ifid: Option<*mut RPC_IF_ID>, membername: *mut RPC_WSTR, priority: *mut u32, annotation: *mut RPC_WSTR) -> RPC_STATUS {
    windows_core::link!("rpcns4.dll" "system" fn RpcNsProfileEltInqNextW(inquirycontext : RPC_NS_HANDLE, ifid : *mut RPC_IF_ID, membername : *mut RPC_WSTR, priority : *mut u32, annotation : *mut RPC_WSTR) -> RPC_STATUS);
    unsafe { RpcNsProfileEltInqNextW(inquirycontext, ifid.unwrap_or(core::mem::zeroed()) as _, membername as _, priority as _, annotation as _) }
}
#[inline]
pub unsafe fn RpcNsProfileEltRemoveA(profilenamesyntax: u32, profilename: *const u8, ifid: Option<*const RPC_IF_ID>, membernamesyntax: u32, membername: *const u8) -> RPC_STATUS {
    windows_core::link!("rpcns4.dll" "system" fn RpcNsProfileEltRemoveA(profilenamesyntax : u32, profilename : *const u8, ifid : *const RPC_IF_ID, membernamesyntax : u32, membername : *const u8) -> RPC_STATUS);
    unsafe { RpcNsProfileEltRemoveA(profilenamesyntax, profilename, ifid.unwrap_or(core::mem::zeroed()) as _, membernamesyntax, membername) }
}
#[inline]
pub unsafe fn RpcNsProfileEltRemoveW(profilenamesyntax: u32, profilename: *const u16, ifid: Option<*const RPC_IF_ID>, membernamesyntax: u32, membername: *const u16) -> RPC_STATUS {
    windows_core::link!("rpcns4.dll" "system" fn RpcNsProfileEltRemoveW(profilenamesyntax : u32, profilename : *const u16, ifid : *const RPC_IF_ID, membernamesyntax : u32, membername : *const u16) -> RPC_STATUS);
    unsafe { RpcNsProfileEltRemoveW(profilenamesyntax, profilename, ifid.unwrap_or(core::mem::zeroed()) as _, membernamesyntax, membername) }
}
#[inline]
pub unsafe fn RpcObjectInqType(objuuid: *const windows_core::GUID, typeuuid: Option<*mut windows_core::GUID>) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcObjectInqType(objuuid : *const windows_core::GUID, typeuuid : *mut windows_core::GUID) -> RPC_STATUS);
    unsafe { RpcObjectInqType(objuuid, typeuuid.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn RpcObjectSetInqFn(inquiryfn: RPC_OBJECT_INQ_FN) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcObjectSetInqFn(inquiryfn : RPC_OBJECT_INQ_FN) -> RPC_STATUS);
    unsafe { RpcObjectSetInqFn(inquiryfn) }
}
#[inline]
pub unsafe fn RpcObjectSetType(objuuid: *const windows_core::GUID, typeuuid: Option<*const windows_core::GUID>) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcObjectSetType(objuuid : *const windows_core::GUID, typeuuid : *const windows_core::GUID) -> RPC_STATUS);
    unsafe { RpcObjectSetType(objuuid, typeuuid.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn RpcProtseqVectorFreeA(protseqvector: *mut *mut RPC_PROTSEQ_VECTORA) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcProtseqVectorFreeA(protseqvector : *mut *mut RPC_PROTSEQ_VECTORA) -> RPC_STATUS);
    unsafe { RpcProtseqVectorFreeA(protseqvector as _) }
}
#[inline]
pub unsafe fn RpcProtseqVectorFreeW(protseqvector: *mut *mut RPC_PROTSEQ_VECTORW) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcProtseqVectorFreeW(protseqvector : *mut *mut RPC_PROTSEQ_VECTORW) -> RPC_STATUS);
    unsafe { RpcProtseqVectorFreeW(protseqvector as _) }
}
#[inline]
pub unsafe fn RpcRaiseException(exception: RPC_STATUS) -> ! {
    windows_core::link!("rpcrt4.dll" "system" fn RpcRaiseException(exception : RPC_STATUS) -> !);
    unsafe { RpcRaiseException(exception) }
}
#[inline]
pub unsafe fn RpcRevertContainerImpersonation() -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcRevertContainerImpersonation() -> RPC_STATUS);
    unsafe { RpcRevertContainerImpersonation() }
}
#[inline]
pub unsafe fn RpcRevertToSelf() -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcRevertToSelf() -> RPC_STATUS);
    unsafe { RpcRevertToSelf() }
}
#[inline]
pub unsafe fn RpcRevertToSelfEx(bindinghandle: Option<RPC_BINDING_HANDLE>) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcRevertToSelfEx(bindinghandle : RPC_BINDING_HANDLE) -> RPC_STATUS);
    unsafe { RpcRevertToSelfEx(bindinghandle.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn RpcServerCompleteSecurityCallback(bindinghandle: RPC_BINDING_HANDLE, status: RPC_STATUS) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcServerCompleteSecurityCallback(bindinghandle : RPC_BINDING_HANDLE, status : RPC_STATUS) -> RPC_STATUS);
    unsafe { RpcServerCompleteSecurityCallback(bindinghandle, status) }
}
#[inline]
pub unsafe fn RpcServerInqBindingHandle(binding: *mut RPC_BINDING_HANDLE) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcServerInqBindingHandle(binding : *mut RPC_BINDING_HANDLE) -> RPC_STATUS);
    unsafe { RpcServerInqBindingHandle(binding as _) }
}
#[inline]
pub unsafe fn RpcServerInqBindings(bindingvector: *mut *mut RPC_BINDING_VECTOR) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcServerInqBindings(bindingvector : *mut *mut RPC_BINDING_VECTOR) -> RPC_STATUS);
    unsafe { RpcServerInqBindings(bindingvector as _) }
}
#[inline]
pub unsafe fn RpcServerInqBindingsEx(securitydescriptor: Option<*const core::ffi::c_void>, bindingvector: *mut *mut RPC_BINDING_VECTOR) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcServerInqBindingsEx(securitydescriptor : *const core::ffi::c_void, bindingvector : *mut *mut RPC_BINDING_VECTOR) -> RPC_STATUS);
    unsafe { RpcServerInqBindingsEx(securitydescriptor.unwrap_or(core::mem::zeroed()) as _, bindingvector as _) }
}
#[inline]
pub unsafe fn RpcServerInqCallAttributesA(clientbinding: Option<RPC_BINDING_HANDLE>, rpccallattributes: *mut core::ffi::c_void) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcServerInqCallAttributesA(clientbinding : RPC_BINDING_HANDLE, rpccallattributes : *mut core::ffi::c_void) -> RPC_STATUS);
    unsafe { RpcServerInqCallAttributesA(clientbinding.unwrap_or(core::mem::zeroed()) as _, rpccallattributes as _) }
}
#[inline]
pub unsafe fn RpcServerInqCallAttributesW(clientbinding: Option<RPC_BINDING_HANDLE>, rpccallattributes: *mut core::ffi::c_void) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcServerInqCallAttributesW(clientbinding : RPC_BINDING_HANDLE, rpccallattributes : *mut core::ffi::c_void) -> RPC_STATUS);
    unsafe { RpcServerInqCallAttributesW(clientbinding.unwrap_or(core::mem::zeroed()) as _, rpccallattributes as _) }
}
#[inline]
pub unsafe fn RpcServerInqDefaultPrincNameA(authnsvc: u32, princname: *mut RPC_CSTR) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcServerInqDefaultPrincNameA(authnsvc : u32, princname : *mut RPC_CSTR) -> RPC_STATUS);
    unsafe { RpcServerInqDefaultPrincNameA(authnsvc, princname as _) }
}
#[inline]
pub unsafe fn RpcServerInqDefaultPrincNameW(authnsvc: u32, princname: *mut RPC_WSTR) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcServerInqDefaultPrincNameW(authnsvc : u32, princname : *mut RPC_WSTR) -> RPC_STATUS);
    unsafe { RpcServerInqDefaultPrincNameW(authnsvc, princname as _) }
}
#[inline]
pub unsafe fn RpcServerInqIf(ifspec: RPC_IF_HANDLE, mgrtypeuuid: Option<*const windows_core::GUID>, mgrepv: *mut *mut core::ffi::c_void) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcServerInqIf(ifspec : RPC_IF_HANDLE, mgrtypeuuid : *const windows_core::GUID, mgrepv : *mut *mut core::ffi::c_void) -> RPC_STATUS);
    unsafe { RpcServerInqIf(ifspec, mgrtypeuuid.unwrap_or(core::mem::zeroed()) as _, mgrepv as _) }
}
#[inline]
pub unsafe fn RpcServerInterfaceGroupActivate(ifgroup: RPC_INTERFACE_GROUP) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcServerInterfaceGroupActivate(ifgroup : RPC_INTERFACE_GROUP) -> RPC_STATUS);
    unsafe { RpcServerInterfaceGroupActivate(ifgroup) }
}
#[inline]
pub unsafe fn RpcServerInterfaceGroupClose(ifgroup: RPC_INTERFACE_GROUP) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcServerInterfaceGroupClose(ifgroup : RPC_INTERFACE_GROUP) -> RPC_STATUS);
    unsafe { RpcServerInterfaceGroupClose(ifgroup) }
}
#[inline]
pub unsafe fn RpcServerInterfaceGroupCreateA(interfaces: &[RPC_INTERFACE_TEMPLATEA], endpoints: &[RPC_ENDPOINT_TEMPLATEA], idleperiod: u32, idlecallbackfn: *const core::ffi::c_void, idlecallbackcontext: *const core::ffi::c_void, ifgroup: *mut *mut core::ffi::c_void) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcServerInterfaceGroupCreateA(interfaces : *const RPC_INTERFACE_TEMPLATEA, numifs : u32, endpoints : *const RPC_ENDPOINT_TEMPLATEA, numendpoints : u32, idleperiod : u32, idlecallbackfn : *const core::ffi::c_void, idlecallbackcontext : *const core::ffi::c_void, ifgroup : *mut *mut core::ffi::c_void) -> RPC_STATUS);
    unsafe { RpcServerInterfaceGroupCreateA(core::mem::transmute(interfaces.as_ptr()), interfaces.len().try_into().unwrap(), core::mem::transmute(endpoints.as_ptr()), endpoints.len().try_into().unwrap(), idleperiod, idlecallbackfn, idlecallbackcontext, ifgroup as _) }
}
#[inline]
pub unsafe fn RpcServerInterfaceGroupCreateW(interfaces: &[RPC_INTERFACE_TEMPLATEW], endpoints: &[RPC_ENDPOINT_TEMPLATEW], idleperiod: u32, idlecallbackfn: *const core::ffi::c_void, idlecallbackcontext: *const core::ffi::c_void, ifgroup: *mut *mut core::ffi::c_void) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcServerInterfaceGroupCreateW(interfaces : *const RPC_INTERFACE_TEMPLATEW, numifs : u32, endpoints : *const RPC_ENDPOINT_TEMPLATEW, numendpoints : u32, idleperiod : u32, idlecallbackfn : *const core::ffi::c_void, idlecallbackcontext : *const core::ffi::c_void, ifgroup : *mut *mut core::ffi::c_void) -> RPC_STATUS);
    unsafe { RpcServerInterfaceGroupCreateW(core::mem::transmute(interfaces.as_ptr()), interfaces.len().try_into().unwrap(), core::mem::transmute(endpoints.as_ptr()), endpoints.len().try_into().unwrap(), idleperiod, idlecallbackfn, idlecallbackcontext, ifgroup as _) }
}
#[inline]
pub unsafe fn RpcServerInterfaceGroupDeactivate(ifgroup: RPC_INTERFACE_GROUP, forcedeactivation: u32) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcServerInterfaceGroupDeactivate(ifgroup : RPC_INTERFACE_GROUP, forcedeactivation : u32) -> RPC_STATUS);
    unsafe { RpcServerInterfaceGroupDeactivate(ifgroup, forcedeactivation) }
}
#[inline]
pub unsafe fn RpcServerInterfaceGroupInqBindings(ifgroup: RPC_INTERFACE_GROUP, bindingvector: *mut *mut RPC_BINDING_VECTOR) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcServerInterfaceGroupInqBindings(ifgroup : RPC_INTERFACE_GROUP, bindingvector : *mut *mut RPC_BINDING_VECTOR) -> RPC_STATUS);
    unsafe { RpcServerInterfaceGroupInqBindings(ifgroup, bindingvector as _) }
}
#[inline]
pub unsafe fn RpcServerListen(minimumcallthreads: u32, maxcalls: u32, dontwait: u32) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcServerListen(minimumcallthreads : u32, maxcalls : u32, dontwait : u32) -> RPC_STATUS);
    unsafe { RpcServerListen(minimumcallthreads, maxcalls, dontwait) }
}
#[inline]
pub unsafe fn RpcServerRegisterAuthInfoA(serverprincname: Option<*const u8>, authnsvc: u32, getkeyfn: RPC_AUTH_KEY_RETRIEVAL_FN, arg: Option<*const core::ffi::c_void>) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcServerRegisterAuthInfoA(serverprincname : *const u8, authnsvc : u32, getkeyfn : RPC_AUTH_KEY_RETRIEVAL_FN, arg : *const core::ffi::c_void) -> RPC_STATUS);
    unsafe { RpcServerRegisterAuthInfoA(serverprincname.unwrap_or(core::mem::zeroed()) as _, authnsvc, getkeyfn, arg.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn RpcServerRegisterAuthInfoW(serverprincname: Option<*const u16>, authnsvc: u32, getkeyfn: RPC_AUTH_KEY_RETRIEVAL_FN, arg: Option<*const core::ffi::c_void>) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcServerRegisterAuthInfoW(serverprincname : *const u16, authnsvc : u32, getkeyfn : RPC_AUTH_KEY_RETRIEVAL_FN, arg : *const core::ffi::c_void) -> RPC_STATUS);
    unsafe { RpcServerRegisterAuthInfoW(serverprincname.unwrap_or(core::mem::zeroed()) as _, authnsvc, getkeyfn, arg.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn RpcServerRegisterIf(ifspec: RPC_IF_HANDLE, mgrtypeuuid: Option<*const windows_core::GUID>, mgrepv: Option<*const core::ffi::c_void>) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcServerRegisterIf(ifspec : RPC_IF_HANDLE, mgrtypeuuid : *const windows_core::GUID, mgrepv : *const core::ffi::c_void) -> RPC_STATUS);
    unsafe { RpcServerRegisterIf(ifspec, mgrtypeuuid.unwrap_or(core::mem::zeroed()) as _, mgrepv.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn RpcServerRegisterIf2(ifspec: RPC_IF_HANDLE, mgrtypeuuid: Option<*const windows_core::GUID>, mgrepv: Option<*const core::ffi::c_void>, flags: u32, maxcalls: u32, maxrpcsize: u32, ifcallbackfn: RPC_IF_CALLBACK_FN) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcServerRegisterIf2(ifspec : RPC_IF_HANDLE, mgrtypeuuid : *const windows_core::GUID, mgrepv : *const core::ffi::c_void, flags : u32, maxcalls : u32, maxrpcsize : u32, ifcallbackfn : RPC_IF_CALLBACK_FN) -> RPC_STATUS);
    unsafe { RpcServerRegisterIf2(ifspec, mgrtypeuuid.unwrap_or(core::mem::zeroed()) as _, mgrepv.unwrap_or(core::mem::zeroed()) as _, flags, maxcalls, maxrpcsize, ifcallbackfn) }
}
#[inline]
pub unsafe fn RpcServerRegisterIf3(ifspec: RPC_IF_HANDLE, mgrtypeuuid: Option<*const windows_core::GUID>, mgrepv: Option<*const core::ffi::c_void>, flags: u32, maxcalls: u32, maxrpcsize: u32, ifcallback: RPC_IF_CALLBACK_FN, securitydescriptor: Option<*const core::ffi::c_void>) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcServerRegisterIf3(ifspec : RPC_IF_HANDLE, mgrtypeuuid : *const windows_core::GUID, mgrepv : *const core::ffi::c_void, flags : u32, maxcalls : u32, maxrpcsize : u32, ifcallback : RPC_IF_CALLBACK_FN, securitydescriptor : *const core::ffi::c_void) -> RPC_STATUS);
    unsafe { RpcServerRegisterIf3(ifspec, mgrtypeuuid.unwrap_or(core::mem::zeroed()) as _, mgrepv.unwrap_or(core::mem::zeroed()) as _, flags, maxcalls, maxrpcsize, ifcallback, securitydescriptor.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn RpcServerRegisterIfEx(ifspec: RPC_IF_HANDLE, mgrtypeuuid: Option<*const windows_core::GUID>, mgrepv: Option<*const core::ffi::c_void>, flags: u32, maxcalls: u32, ifcallback: RPC_IF_CALLBACK_FN) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcServerRegisterIfEx(ifspec : RPC_IF_HANDLE, mgrtypeuuid : *const windows_core::GUID, mgrepv : *const core::ffi::c_void, flags : u32, maxcalls : u32, ifcallback : RPC_IF_CALLBACK_FN) -> RPC_STATUS);
    unsafe { RpcServerRegisterIfEx(ifspec, mgrtypeuuid.unwrap_or(core::mem::zeroed()) as _, mgrepv.unwrap_or(core::mem::zeroed()) as _, flags, maxcalls, ifcallback) }
}
#[cfg(all(feature = "minwinbase", feature = "windef", feature = "winnt"))]
#[inline]
pub unsafe fn RpcServerSubscribeForNotification(binding: Option<RPC_BINDING_HANDLE>, notification: RPC_NOTIFICATIONS, notificationtype: RPC_NOTIFICATION_TYPES, notificationinfo: *const RPC_ASYNC_NOTIFICATION_INFO) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcServerSubscribeForNotification(binding : RPC_BINDING_HANDLE, notification : RPC_NOTIFICATIONS, notificationtype : RPC_NOTIFICATION_TYPES, notificationinfo : *const RPC_ASYNC_NOTIFICATION_INFO) -> RPC_STATUS);
    unsafe { RpcServerSubscribeForNotification(binding.unwrap_or(core::mem::zeroed()) as _, notification, notificationtype, notificationinfo) }
}
#[inline]
pub unsafe fn RpcServerTestCancel(bindinghandle: Option<RPC_BINDING_HANDLE>) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcServerTestCancel(bindinghandle : RPC_BINDING_HANDLE) -> RPC_STATUS);
    unsafe { RpcServerTestCancel(bindinghandle.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn RpcServerUnregisterIf(ifspec: Option<RPC_IF_HANDLE>, mgrtypeuuid: Option<*const windows_core::GUID>, waitforcallstocomplete: u32) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcServerUnregisterIf(ifspec : RPC_IF_HANDLE, mgrtypeuuid : *const windows_core::GUID, waitforcallstocomplete : u32) -> RPC_STATUS);
    unsafe { RpcServerUnregisterIf(ifspec.unwrap_or(core::mem::zeroed()) as _, mgrtypeuuid.unwrap_or(core::mem::zeroed()) as _, waitforcallstocomplete) }
}
#[inline]
pub unsafe fn RpcServerUnregisterIfEx(ifspec: Option<RPC_IF_HANDLE>, mgrtypeuuid: Option<*const windows_core::GUID>, rundowncontexthandles: i32) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcServerUnregisterIfEx(ifspec : RPC_IF_HANDLE, mgrtypeuuid : *const windows_core::GUID, rundowncontexthandles : i32) -> RPC_STATUS);
    unsafe { RpcServerUnregisterIfEx(ifspec.unwrap_or(core::mem::zeroed()) as _, mgrtypeuuid.unwrap_or(core::mem::zeroed()) as _, rundowncontexthandles) }
}
#[inline]
pub unsafe fn RpcServerUnsubscribeForNotification(binding: Option<RPC_BINDING_HANDLE>, notification: RPC_NOTIFICATIONS, notificationsqueued: *mut u32) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcServerUnsubscribeForNotification(binding : RPC_BINDING_HANDLE, notification : RPC_NOTIFICATIONS, notificationsqueued : *mut u32) -> RPC_STATUS);
    unsafe { RpcServerUnsubscribeForNotification(binding.unwrap_or(core::mem::zeroed()) as _, notification, notificationsqueued as _) }
}
#[inline]
pub unsafe fn RpcServerUseAllProtseqs(maxcalls: u32, securitydescriptor: Option<*const core::ffi::c_void>) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcServerUseAllProtseqs(maxcalls : u32, securitydescriptor : *const core::ffi::c_void) -> RPC_STATUS);
    unsafe { RpcServerUseAllProtseqs(maxcalls, securitydescriptor.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn RpcServerUseAllProtseqsEx(maxcalls: u32, securitydescriptor: Option<*const core::ffi::c_void>, policy: *const RPC_POLICY) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcServerUseAllProtseqsEx(maxcalls : u32, securitydescriptor : *const core::ffi::c_void, policy : *const RPC_POLICY) -> RPC_STATUS);
    unsafe { RpcServerUseAllProtseqsEx(maxcalls, securitydescriptor.unwrap_or(core::mem::zeroed()) as _, policy) }
}
#[inline]
pub unsafe fn RpcServerUseAllProtseqsIf(maxcalls: u32, ifspec: RPC_IF_HANDLE, securitydescriptor: Option<*const core::ffi::c_void>) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcServerUseAllProtseqsIf(maxcalls : u32, ifspec : RPC_IF_HANDLE, securitydescriptor : *const core::ffi::c_void) -> RPC_STATUS);
    unsafe { RpcServerUseAllProtseqsIf(maxcalls, ifspec, securitydescriptor.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn RpcServerUseAllProtseqsIfEx(maxcalls: u32, ifspec: RPC_IF_HANDLE, securitydescriptor: Option<*const core::ffi::c_void>, policy: *const RPC_POLICY) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcServerUseAllProtseqsIfEx(maxcalls : u32, ifspec : RPC_IF_HANDLE, securitydescriptor : *const core::ffi::c_void, policy : *const RPC_POLICY) -> RPC_STATUS);
    unsafe { RpcServerUseAllProtseqsIfEx(maxcalls, ifspec, securitydescriptor.unwrap_or(core::mem::zeroed()) as _, policy) }
}
#[inline]
pub unsafe fn RpcServerUseProtseqA(protseq: *const u8, maxcalls: u32, securitydescriptor: Option<*const core::ffi::c_void>) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcServerUseProtseqA(protseq : *const u8, maxcalls : u32, securitydescriptor : *const core::ffi::c_void) -> RPC_STATUS);
    unsafe { RpcServerUseProtseqA(protseq, maxcalls, securitydescriptor.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn RpcServerUseProtseqEpA(protseq: *const u8, maxcalls: u32, endpoint: *const u8, securitydescriptor: Option<*const core::ffi::c_void>) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcServerUseProtseqEpA(protseq : *const u8, maxcalls : u32, endpoint : *const u8, securitydescriptor : *const core::ffi::c_void) -> RPC_STATUS);
    unsafe { RpcServerUseProtseqEpA(protseq, maxcalls, endpoint, securitydescriptor.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn RpcServerUseProtseqEpExA(protseq: *const u8, maxcalls: u32, endpoint: *const u8, securitydescriptor: Option<*const core::ffi::c_void>, policy: *const RPC_POLICY) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcServerUseProtseqEpExA(protseq : *const u8, maxcalls : u32, endpoint : *const u8, securitydescriptor : *const core::ffi::c_void, policy : *const RPC_POLICY) -> RPC_STATUS);
    unsafe { RpcServerUseProtseqEpExA(protseq, maxcalls, endpoint, securitydescriptor.unwrap_or(core::mem::zeroed()) as _, policy) }
}
#[inline]
pub unsafe fn RpcServerUseProtseqEpExW(protseq: *const u16, maxcalls: u32, endpoint: *const u16, securitydescriptor: Option<*const core::ffi::c_void>, policy: *const RPC_POLICY) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcServerUseProtseqEpExW(protseq : *const u16, maxcalls : u32, endpoint : *const u16, securitydescriptor : *const core::ffi::c_void, policy : *const RPC_POLICY) -> RPC_STATUS);
    unsafe { RpcServerUseProtseqEpExW(protseq, maxcalls, endpoint, securitydescriptor.unwrap_or(core::mem::zeroed()) as _, policy) }
}
#[inline]
pub unsafe fn RpcServerUseProtseqEpW(protseq: *const u16, maxcalls: u32, endpoint: *const u16, securitydescriptor: Option<*const core::ffi::c_void>) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcServerUseProtseqEpW(protseq : *const u16, maxcalls : u32, endpoint : *const u16, securitydescriptor : *const core::ffi::c_void) -> RPC_STATUS);
    unsafe { RpcServerUseProtseqEpW(protseq, maxcalls, endpoint, securitydescriptor.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn RpcServerUseProtseqExA(protseq: *const u8, maxcalls: u32, securitydescriptor: Option<*const core::ffi::c_void>, policy: *const RPC_POLICY) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcServerUseProtseqExA(protseq : *const u8, maxcalls : u32, securitydescriptor : *const core::ffi::c_void, policy : *const RPC_POLICY) -> RPC_STATUS);
    unsafe { RpcServerUseProtseqExA(protseq, maxcalls, securitydescriptor.unwrap_or(core::mem::zeroed()) as _, policy) }
}
#[inline]
pub unsafe fn RpcServerUseProtseqExW(protseq: *const u16, maxcalls: u32, securitydescriptor: Option<*const core::ffi::c_void>, policy: *const RPC_POLICY) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcServerUseProtseqExW(protseq : *const u16, maxcalls : u32, securitydescriptor : *const core::ffi::c_void, policy : *const RPC_POLICY) -> RPC_STATUS);
    unsafe { RpcServerUseProtseqExW(protseq, maxcalls, securitydescriptor.unwrap_or(core::mem::zeroed()) as _, policy) }
}
#[inline]
pub unsafe fn RpcServerUseProtseqIfA(protseq: *const u8, maxcalls: u32, ifspec: RPC_IF_HANDLE, securitydescriptor: Option<*const core::ffi::c_void>) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcServerUseProtseqIfA(protseq : *const u8, maxcalls : u32, ifspec : RPC_IF_HANDLE, securitydescriptor : *const core::ffi::c_void) -> RPC_STATUS);
    unsafe { RpcServerUseProtseqIfA(protseq, maxcalls, ifspec, securitydescriptor.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn RpcServerUseProtseqIfExA(protseq: *const u8, maxcalls: u32, ifspec: RPC_IF_HANDLE, securitydescriptor: Option<*const core::ffi::c_void>, policy: *const RPC_POLICY) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcServerUseProtseqIfExA(protseq : *const u8, maxcalls : u32, ifspec : RPC_IF_HANDLE, securitydescriptor : *const core::ffi::c_void, policy : *const RPC_POLICY) -> RPC_STATUS);
    unsafe { RpcServerUseProtseqIfExA(protseq, maxcalls, ifspec, securitydescriptor.unwrap_or(core::mem::zeroed()) as _, policy) }
}
#[inline]
pub unsafe fn RpcServerUseProtseqIfExW(protseq: *const u16, maxcalls: u32, ifspec: RPC_IF_HANDLE, securitydescriptor: Option<*const core::ffi::c_void>, policy: *const RPC_POLICY) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcServerUseProtseqIfExW(protseq : *const u16, maxcalls : u32, ifspec : RPC_IF_HANDLE, securitydescriptor : *const core::ffi::c_void, policy : *const RPC_POLICY) -> RPC_STATUS);
    unsafe { RpcServerUseProtseqIfExW(protseq, maxcalls, ifspec, securitydescriptor.unwrap_or(core::mem::zeroed()) as _, policy) }
}
#[inline]
pub unsafe fn RpcServerUseProtseqIfW(protseq: *const u16, maxcalls: u32, ifspec: RPC_IF_HANDLE, securitydescriptor: Option<*const core::ffi::c_void>) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcServerUseProtseqIfW(protseq : *const u16, maxcalls : u32, ifspec : RPC_IF_HANDLE, securitydescriptor : *const core::ffi::c_void) -> RPC_STATUS);
    unsafe { RpcServerUseProtseqIfW(protseq, maxcalls, ifspec, securitydescriptor.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn RpcServerUseProtseqW(protseq: *const u16, maxcalls: u32, securitydescriptor: Option<*const core::ffi::c_void>) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcServerUseProtseqW(protseq : *const u16, maxcalls : u32, securitydescriptor : *const core::ffi::c_void) -> RPC_STATUS);
    unsafe { RpcServerUseProtseqW(protseq, maxcalls, securitydescriptor.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn RpcServerYield() {
    windows_core::link!("rpcrt4.dll" "system" fn RpcServerYield());
    unsafe { RpcServerYield() }
}
#[inline]
pub unsafe fn RpcSmAllocate(size: usize, pstatus: *mut RPC_STATUS) -> *mut core::ffi::c_void {
    windows_core::link!("rpcrt4.dll" "system" fn RpcSmAllocate(size : usize, pstatus : *mut RPC_STATUS) -> *mut core::ffi::c_void);
    unsafe { RpcSmAllocate(size, pstatus as _) }
}
#[inline]
pub unsafe fn RpcSmClientFree(pnodetofree: *const core::ffi::c_void) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcSmClientFree(pnodetofree : *const core::ffi::c_void) -> RPC_STATUS);
    unsafe { RpcSmClientFree(pnodetofree) }
}
#[inline]
pub unsafe fn RpcSmDestroyClientContext(contexthandle: *const *const core::ffi::c_void) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcSmDestroyClientContext(contexthandle : *const *const core::ffi::c_void) -> RPC_STATUS);
    unsafe { RpcSmDestroyClientContext(contexthandle) }
}
#[inline]
pub unsafe fn RpcSmDisableAllocate() -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcSmDisableAllocate() -> RPC_STATUS);
    unsafe { RpcSmDisableAllocate() }
}
#[inline]
pub unsafe fn RpcSmEnableAllocate() -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcSmEnableAllocate() -> RPC_STATUS);
    unsafe { RpcSmEnableAllocate() }
}
#[inline]
pub unsafe fn RpcSmFree(nodetofree: *const core::ffi::c_void) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcSmFree(nodetofree : *const core::ffi::c_void) -> RPC_STATUS);
    unsafe { RpcSmFree(nodetofree) }
}
#[inline]
pub unsafe fn RpcSmGetThreadHandle(pstatus: *mut RPC_STATUS) -> RPC_SS_THREAD_HANDLE {
    windows_core::link!("rpcrt4.dll" "system" fn RpcSmGetThreadHandle(pstatus : *mut RPC_STATUS) -> RPC_SS_THREAD_HANDLE);
    unsafe { RpcSmGetThreadHandle(pstatus as _) }
}
#[inline]
pub unsafe fn RpcSmSetClientAllocFree(clientalloc: RPC_CLIENT_ALLOC, clientfree: RPC_CLIENT_FREE) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcSmSetClientAllocFree(clientalloc : RPC_CLIENT_ALLOC, clientfree : RPC_CLIENT_FREE) -> RPC_STATUS);
    unsafe { RpcSmSetClientAllocFree(clientalloc, clientfree) }
}
#[inline]
pub unsafe fn RpcSmSetThreadHandle(id: RPC_SS_THREAD_HANDLE) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcSmSetThreadHandle(id : RPC_SS_THREAD_HANDLE) -> RPC_STATUS);
    unsafe { RpcSmSetThreadHandle(id) }
}
#[inline]
pub unsafe fn RpcSmSwapClientAllocFree(clientalloc: RPC_CLIENT_ALLOC, clientfree: RPC_CLIENT_FREE, oldclientalloc: *mut RPC_CLIENT_ALLOC, oldclientfree: *mut RPC_CLIENT_FREE) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcSmSwapClientAllocFree(clientalloc : RPC_CLIENT_ALLOC, clientfree : RPC_CLIENT_FREE, oldclientalloc : *mut RPC_CLIENT_ALLOC, oldclientfree : *mut RPC_CLIENT_FREE) -> RPC_STATUS);
    unsafe { RpcSmSwapClientAllocFree(clientalloc, clientfree, oldclientalloc as _, oldclientfree as _) }
}
#[inline]
pub unsafe fn RpcSsAllocate(size: usize) -> *mut core::ffi::c_void {
    windows_core::link!("rpcrt4.dll" "system" fn RpcSsAllocate(size : usize) -> *mut core::ffi::c_void);
    unsafe { RpcSsAllocate(size) }
}
#[inline]
pub unsafe fn RpcSsContextLockExclusive(serverbindinghandle: Option<RPC_BINDING_HANDLE>, usercontext: *const core::ffi::c_void) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcSsContextLockExclusive(serverbindinghandle : RPC_BINDING_HANDLE, usercontext : *const core::ffi::c_void) -> RPC_STATUS);
    unsafe { RpcSsContextLockExclusive(serverbindinghandle.unwrap_or(core::mem::zeroed()) as _, usercontext) }
}
#[inline]
pub unsafe fn RpcSsContextLockShared(serverbindinghandle: RPC_BINDING_HANDLE, usercontext: *const core::ffi::c_void) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcSsContextLockShared(serverbindinghandle : RPC_BINDING_HANDLE, usercontext : *const core::ffi::c_void) -> RPC_STATUS);
    unsafe { RpcSsContextLockShared(serverbindinghandle, usercontext) }
}
#[inline]
pub unsafe fn RpcSsDestroyClientContext(contexthandle: *const *const core::ffi::c_void) {
    windows_core::link!("rpcrt4.dll" "system" fn RpcSsDestroyClientContext(contexthandle : *const *const core::ffi::c_void));
    unsafe { RpcSsDestroyClientContext(contexthandle) }
}
#[inline]
pub unsafe fn RpcSsDisableAllocate() {
    windows_core::link!("rpcrt4.dll" "system" fn RpcSsDisableAllocate());
    unsafe { RpcSsDisableAllocate() }
}
#[inline]
pub unsafe fn RpcSsDontSerializeContext() {
    windows_core::link!("rpcrt4.dll" "system" fn RpcSsDontSerializeContext());
    unsafe { RpcSsDontSerializeContext() }
}
#[inline]
pub unsafe fn RpcSsEnableAllocate() {
    windows_core::link!("rpcrt4.dll" "system" fn RpcSsEnableAllocate());
    unsafe { RpcSsEnableAllocate() }
}
#[inline]
pub unsafe fn RpcSsFree(nodetofree: *const core::ffi::c_void) {
    windows_core::link!("rpcrt4.dll" "system" fn RpcSsFree(nodetofree : *const core::ffi::c_void));
    unsafe { RpcSsFree(nodetofree) }
}
#[inline]
pub unsafe fn RpcSsGetContextBinding(contexthandle: *const core::ffi::c_void, binding: *mut RPC_BINDING_HANDLE) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcSsGetContextBinding(contexthandle : *const core::ffi::c_void, binding : *mut RPC_BINDING_HANDLE) -> RPC_STATUS);
    unsafe { RpcSsGetContextBinding(contexthandle, binding as _) }
}
#[inline]
pub unsafe fn RpcSsGetThreadHandle() -> RPC_SS_THREAD_HANDLE {
    windows_core::link!("rpcrt4.dll" "system" fn RpcSsGetThreadHandle() -> RPC_SS_THREAD_HANDLE);
    unsafe { RpcSsGetThreadHandle() }
}
#[inline]
pub unsafe fn RpcSsSetClientAllocFree(clientalloc: RPC_CLIENT_ALLOC, clientfree: RPC_CLIENT_FREE) {
    windows_core::link!("rpcrt4.dll" "system" fn RpcSsSetClientAllocFree(clientalloc : RPC_CLIENT_ALLOC, clientfree : RPC_CLIENT_FREE));
    unsafe { RpcSsSetClientAllocFree(clientalloc, clientfree) }
}
#[inline]
pub unsafe fn RpcSsSetThreadHandle(id: RPC_SS_THREAD_HANDLE) {
    windows_core::link!("rpcrt4.dll" "system" fn RpcSsSetThreadHandle(id : RPC_SS_THREAD_HANDLE));
    unsafe { RpcSsSetThreadHandle(id) }
}
#[inline]
pub unsafe fn RpcSsSwapClientAllocFree(clientalloc: RPC_CLIENT_ALLOC, clientfree: RPC_CLIENT_FREE, oldclientalloc: *mut RPC_CLIENT_ALLOC, oldclientfree: *mut RPC_CLIENT_FREE) {
    windows_core::link!("rpcrt4.dll" "system" fn RpcSsSwapClientAllocFree(clientalloc : RPC_CLIENT_ALLOC, clientfree : RPC_CLIENT_FREE, oldclientalloc : *mut RPC_CLIENT_ALLOC, oldclientfree : *mut RPC_CLIENT_FREE));
    unsafe { RpcSsSwapClientAllocFree(clientalloc, clientfree, oldclientalloc as _, oldclientfree as _) }
}
#[inline]
pub unsafe fn RpcStringBindingComposeA(objuuid: Option<*const u8>, protseq: Option<*const u8>, networkaddr: Option<*const u8>, endpoint: Option<*const u8>, options: Option<*const u8>, stringbinding: *mut RPC_CSTR) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcStringBindingComposeA(objuuid : *const u8, protseq : *const u8, networkaddr : *const u8, endpoint : *const u8, options : *const u8, stringbinding : *mut RPC_CSTR) -> RPC_STATUS);
    unsafe { RpcStringBindingComposeA(objuuid.unwrap_or(core::mem::zeroed()) as _, protseq.unwrap_or(core::mem::zeroed()) as _, networkaddr.unwrap_or(core::mem::zeroed()) as _, endpoint.unwrap_or(core::mem::zeroed()) as _, options.unwrap_or(core::mem::zeroed()) as _, stringbinding as _) }
}
#[inline]
pub unsafe fn RpcStringBindingComposeW(objuuid: Option<*const u16>, protseq: Option<*const u16>, networkaddr: Option<*const u16>, endpoint: Option<*const u16>, options: Option<*const u16>, stringbinding: *mut RPC_WSTR) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcStringBindingComposeW(objuuid : *const u16, protseq : *const u16, networkaddr : *const u16, endpoint : *const u16, options : *const u16, stringbinding : *mut RPC_WSTR) -> RPC_STATUS);
    unsafe { RpcStringBindingComposeW(objuuid.unwrap_or(core::mem::zeroed()) as _, protseq.unwrap_or(core::mem::zeroed()) as _, networkaddr.unwrap_or(core::mem::zeroed()) as _, endpoint.unwrap_or(core::mem::zeroed()) as _, options.unwrap_or(core::mem::zeroed()) as _, stringbinding as _) }
}
#[inline]
pub unsafe fn RpcStringBindingParseA(stringbinding: *const u8, objuuid: *mut RPC_CSTR, protseq: *mut RPC_CSTR, networkaddr: *mut RPC_CSTR, endpoint: *mut RPC_CSTR, networkoptions: *mut RPC_CSTR) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcStringBindingParseA(stringbinding : *const u8, objuuid : *mut RPC_CSTR, protseq : *mut RPC_CSTR, networkaddr : *mut RPC_CSTR, endpoint : *mut RPC_CSTR, networkoptions : *mut RPC_CSTR) -> RPC_STATUS);
    unsafe { RpcStringBindingParseA(stringbinding, objuuid as _, protseq as _, networkaddr as _, endpoint as _, networkoptions as _) }
}
#[inline]
pub unsafe fn RpcStringBindingParseW(stringbinding: *const u16, objuuid: *mut RPC_WSTR, protseq: *mut RPC_WSTR, networkaddr: *mut RPC_WSTR, endpoint: *mut RPC_WSTR, networkoptions: *mut RPC_WSTR) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcStringBindingParseW(stringbinding : *const u16, objuuid : *mut RPC_WSTR, protseq : *mut RPC_WSTR, networkaddr : *mut RPC_WSTR, endpoint : *mut RPC_WSTR, networkoptions : *mut RPC_WSTR) -> RPC_STATUS);
    unsafe { RpcStringBindingParseW(stringbinding, objuuid as _, protseq as _, networkaddr as _, endpoint as _, networkoptions as _) }
}
#[inline]
pub unsafe fn RpcStringFreeA(string: *mut RPC_CSTR) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcStringFreeA(string : *mut RPC_CSTR) -> RPC_STATUS);
    unsafe { RpcStringFreeA(string as _) }
}
#[inline]
pub unsafe fn RpcStringFreeW(string: *mut RPC_WSTR) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcStringFreeW(string : *mut RPC_WSTR) -> RPC_STATUS);
    unsafe { RpcStringFreeW(string as _) }
}
#[inline]
pub unsafe fn RpcTestCancel() -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcTestCancel() -> RPC_STATUS);
    unsafe { RpcTestCancel() }
}
#[inline]
pub unsafe fn RpcUserFree(asynchandle: handle_t, pbuffer: *mut core::ffi::c_void) {
    windows_core::link!("rpcrt4.dll" "system" fn RpcUserFree(asynchandle : handle_t, pbuffer : *mut core::ffi::c_void));
    unsafe { RpcUserFree(asynchandle, pbuffer as _) }
}
#[inline]
pub unsafe fn UuidCompare(uuid1: *const windows_core::GUID, uuid2: *const windows_core::GUID, status: *mut RPC_STATUS) -> i32 {
    windows_core::link!("rpcrt4.dll" "system" fn UuidCompare(uuid1 : *const windows_core::GUID, uuid2 : *const windows_core::GUID, status : *mut RPC_STATUS) -> i32);
    unsafe { UuidCompare(uuid1, uuid2, status as _) }
}
#[inline]
pub unsafe fn UuidCreate(uuid: *mut windows_core::GUID) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn UuidCreate(uuid : *mut windows_core::GUID) -> RPC_STATUS);
    unsafe { UuidCreate(uuid as _) }
}
#[inline]
pub unsafe fn UuidCreateNil(niluuid: *mut windows_core::GUID) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn UuidCreateNil(niluuid : *mut windows_core::GUID) -> RPC_STATUS);
    unsafe { UuidCreateNil(niluuid as _) }
}
#[inline]
pub unsafe fn UuidCreateSequential(uuid: *mut windows_core::GUID) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn UuidCreateSequential(uuid : *mut windows_core::GUID) -> RPC_STATUS);
    unsafe { UuidCreateSequential(uuid as _) }
}
#[inline]
pub unsafe fn UuidEqual(uuid1: *const windows_core::GUID, uuid2: *const windows_core::GUID, status: *mut RPC_STATUS) -> i32 {
    windows_core::link!("rpcrt4.dll" "system" fn UuidEqual(uuid1 : *const windows_core::GUID, uuid2 : *const windows_core::GUID, status : *mut RPC_STATUS) -> i32);
    unsafe { UuidEqual(uuid1, uuid2, status as _) }
}
#[inline]
pub unsafe fn UuidFromStringA(stringuuid: Option<*const u8>, uuid: *mut windows_core::GUID) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn UuidFromStringA(stringuuid : *const u8, uuid : *mut windows_core::GUID) -> RPC_STATUS);
    unsafe { UuidFromStringA(stringuuid.unwrap_or(core::mem::zeroed()) as _, uuid as _) }
}
#[inline]
pub unsafe fn UuidFromStringW(stringuuid: Option<*const u16>, uuid: *mut windows_core::GUID) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn UuidFromStringW(stringuuid : *const u16, uuid : *mut windows_core::GUID) -> RPC_STATUS);
    unsafe { UuidFromStringW(stringuuid.unwrap_or(core::mem::zeroed()) as _, uuid as _) }
}
#[inline]
pub unsafe fn UuidHash(uuid: *const windows_core::GUID, status: *mut RPC_STATUS) -> u16 {
    windows_core::link!("rpcrt4.dll" "system" fn UuidHash(uuid : *const windows_core::GUID, status : *mut RPC_STATUS) -> u16);
    unsafe { UuidHash(uuid, status as _) }
}
#[inline]
pub unsafe fn UuidIsNil(uuid: *const windows_core::GUID, status: *mut RPC_STATUS) -> i32 {
    windows_core::link!("rpcrt4.dll" "system" fn UuidIsNil(uuid : *const windows_core::GUID, status : *mut RPC_STATUS) -> i32);
    unsafe { UuidIsNil(uuid, status as _) }
}
#[inline]
pub unsafe fn UuidToStringA(uuid: *const windows_core::GUID, stringuuid: *mut RPC_CSTR) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn UuidToStringA(uuid : *const windows_core::GUID, stringuuid : *mut RPC_CSTR) -> RPC_STATUS);
    unsafe { UuidToStringA(uuid, stringuuid as _) }
}
#[inline]
pub unsafe fn UuidToStringW(uuid: *const windows_core::GUID, stringuuid: *mut RPC_WSTR) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn UuidToStringW(uuid : *const windows_core::GUID, stringuuid : *mut RPC_WSTR) -> RPC_STATUS);
    unsafe { UuidToStringW(uuid, stringuuid as _) }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ARRAY_INFO {
    pub Dimension: i32,
    pub BufferConformanceMark: *mut u32,
    pub BufferVarianceMark: *mut u32,
    pub MaxCountArray: *mut u32,
    pub OffsetArray: *mut u32,
    pub ActualCountArray: *mut u32,
}
impl Default for ARRAY_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BinaryParam {
    pub Buffer: *mut core::ffi::c_void,
    pub Size: i16,
}
impl Default for BinaryParam {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union CLIENT_CALL_RETURN {
    pub Pointer: *mut core::ffi::c_void,
    pub Simple: isize,
}
impl Default for CLIENT_CALL_RETURN {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct COMM_FAULT_OFFSETS {
    pub CommOffset: i16,
    pub FaultOffset: i16,
}
pub type CS_TAG_GETTING_ROUTINE = Option<unsafe extern "system" fn(hbinding: RPC_BINDING_HANDLE, fserverside: i32, pulsendingtag: *mut u32, puldesiredreceivingtag: *mut u32, pulreceivingtag: *mut u32, pstatus: *mut error_status_t)>;
pub type CS_TYPE_FROM_NETCS_ROUTINE = Option<unsafe extern "system" fn(hbinding: RPC_BINDING_HANDLE, ulnetworkcodeset: u32, pnetworkdata: *mut byte, ulnetworkdatalength: u32, ullocalbuffersize: u32, plocaldata: *mut core::ffi::c_void, pullocaldatalength: *mut u32, pstatus: *mut error_status_t)>;
pub type CS_TYPE_LOCAL_SIZE_ROUTINE = Option<unsafe extern "system" fn(hbinding: RPC_BINDING_HANDLE, ulnetworkcodeset: u32, ulnetworkbuffersize: u32, conversiontype: *mut IDL_CS_CONVERT, pullocalbuffersize: *mut u32, pstatus: *mut error_status_t)>;
pub type CS_TYPE_NET_SIZE_ROUTINE = Option<unsafe extern "system" fn(hbinding: RPC_BINDING_HANDLE, ulnetworkcodeset: u32, ullocalbuffersize: u32, conversiontype: *mut IDL_CS_CONVERT, pulnetworkbuffersize: *mut u32, pstatus: *mut error_status_t)>;
pub type CS_TYPE_TO_NETCS_ROUTINE = Option<unsafe extern "system" fn(hbinding: RPC_BINDING_HANDLE, ulnetworkcodeset: u32, plocaldata: *mut core::ffi::c_void, ullocaldatalength: u32, pnetworkdata: *mut byte, pulnetworkdatalength: *mut u32, pstatus: *mut error_status_t)>;
pub const DCE_C_ERROR_STRING_LEN: u32 = 256;
pub const EEInfoGCCOM: u32 = 11;
pub const EEInfoGCFRS: u32 = 12;
pub const EEInfoNextRecordsMissing: u32 = 2;
pub const EEInfoPreviousRecordsMissing: u32 = 1;
pub const EEInfoUseFileTime: u32 = 4;
#[cfg(feature = "objidlbase")]
pub type EXPR_EVAL = Option<unsafe extern "system" fn(param0: *mut MIDL_STUB_MESSAGE)>;
pub type ExtendedErrorParamTypes = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FULL_PTR_XLAT_TABLES {
    pub RefIdToPointer: *mut core::ffi::c_void,
    pub PointerToRefId: *mut core::ffi::c_void,
    pub NextRefId: u32,
    pub XlatSide: XLAT_SIDE,
}
impl Default for FULL_PTR_XLAT_TABLES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct GENERIC_BINDING_INFO {
    pub pObj: *mut core::ffi::c_void,
    pub Size: u32,
    pub pfnBind: GENERIC_BINDING_ROUTINE,
    pub pfnUnbind: GENERIC_UNBIND_ROUTINE,
}
impl Default for GENERIC_BINDING_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type GENERIC_BINDING_ROUTINE = Option<unsafe extern "system" fn(param0: *mut core::ffi::c_void) -> *mut core::ffi::c_void>;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct GENERIC_BINDING_ROUTINE_PAIR {
    pub pfnBind: GENERIC_BINDING_ROUTINE,
    pub pfnUnbind: GENERIC_UNBIND_ROUTINE,
}
pub type GENERIC_UNBIND_ROUTINE = Option<unsafe extern "system" fn(param0: *mut core::ffi::c_void, param1: *mut u8)>;
pub type IDL_CS_CONVERT = i32;
pub const IDL_CS_IN_PLACE_CONVERT: IDL_CS_CONVERT = 1;
pub const IDL_CS_NEW_BUFFER_CONVERT: IDL_CS_CONVERT = 2;
pub const IDL_CS_NO_CONVERT: IDL_CS_CONVERT = 0;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct I_RPC_HANDLE(pub *mut core::ffi::c_void);
impl I_RPC_HANDLE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for I_RPC_HANDLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
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
pub type I_RpcFreeCalloutStateFn = Option<unsafe extern "system" fn(calloutstate: *mut RDR_CALLOUT_STATE)>;
pub type I_RpcPerformCalloutFn = Option<unsafe extern "system" fn(context: *mut core::ffi::c_void, calloutstate: *mut RDR_CALLOUT_STATE, stage: RPC_HTTP_REDIRECTOR_STAGE) -> RPC_STATUS>;
#[repr(C)]
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
pub type I_RpcProxyFilterIfFn = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, ifuuid: *const windows_core::GUID, ifmajorversion: u16, fallow: *mut i32) -> RPC_STATUS>;
pub type I_RpcProxyGetClientAddressFn = Option<unsafe extern "system" fn(context: *mut core::ffi::c_void, buffer: *mut i8, bufferlength: *mut u32) -> RPC_STATUS>;
pub type I_RpcProxyGetClientSessionAndResourceUUID = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, sessionidpresent: *mut i32, sessionid: *mut windows_core::GUID, resourceidpresent: *mut i32, resourceid: *mut windows_core::GUID) -> RPC_STATUS>;
pub type I_RpcProxyGetConnectionTimeoutFn = Option<unsafe extern "system" fn(connectiontimeout: *mut u32) -> RPC_STATUS>;
pub type I_RpcProxyIsValidMachineFn = Option<unsafe extern "system" fn(machine: *const u16, dotmachine: *const u16, portnumber: u32) -> RPC_STATUS>;
pub type I_RpcProxyUpdatePerfCounterBackendServerFn = Option<unsafe extern "system" fn(machinename: *const u16, isconnectevent: i32)>;
pub type I_RpcProxyUpdatePerfCounterFn = Option<unsafe extern "system" fn(counter: RpcPerfCounters, modifytrend: i32, size: u32)>;
pub type LRPC_SYSTEM_HANDLE_MARSHAL_DIRECTION = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MALLOC_FREE_STRUCT {
    pub pfnAllocate: *mut u8,
    pub pfnFree: *mut u8,
}
impl Default for MALLOC_FREE_STRUCT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MIDL_FORMAT_STRING {
    pub Pad: i16,
    pub Format: [u8; 0],
}
impl Default for MIDL_FORMAT_STRING {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MIDL_INTERCEPTION_INFO {
    pub Version: u32,
    pub ProcString: PFORMAT_STRING,
    pub ProcFormatOffsetTable: *const u16,
    pub ProcCount: u32,
    pub TypeString: PFORMAT_STRING,
}
impl Default for MIDL_INTERCEPTION_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MIDL_INTERFACE_METHOD_PROPERTIES {
    pub MethodCount: u16,
    pub MethodProperties: *const *const MIDL_METHOD_PROPERTY_MAP,
}
impl Default for MIDL_INTERFACE_METHOD_PROPERTIES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MIDL_METHOD_PROPERTY {
    pub Id: u32,
    pub Value: usize,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MIDL_METHOD_PROPERTY_MAP {
    pub Count: u32,
    pub Properties: *const MIDL_METHOD_PROPERTY,
}
impl Default for MIDL_METHOD_PROPERTY_MAP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "objidlbase")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MIDL_SERVER_INFO {
    pub pStubDesc: PMIDL_STUB_DESC,
    pub DispatchTable: *const SERVER_ROUTINE,
    pub ProcString: PFORMAT_STRING,
    pub FmtStringOffset: *const u16,
    pub ThunkTable: *const STUB_THUNK,
    pub pTransferSyntax: PRPC_SYNTAX_IDENTIFIER,
    pub nCount: usize,
    pub pSyntaxInfo: PMIDL_SYNTAX_INFO,
}
#[cfg(feature = "objidlbase")]
impl Default for MIDL_SERVER_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "objidlbase")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MIDL_STUBLESS_PROXY_INFO {
    pub pStubDesc: PMIDL_STUB_DESC,
    pub ProcFormatString: PFORMAT_STRING,
    pub FormatStringOffset: *const u16,
    pub pTransferSyntax: PRPC_SYNTAX_IDENTIFIER,
    pub nCount: usize,
    pub pSyntaxInfo: PMIDL_SYNTAX_INFO,
}
#[cfg(feature = "objidlbase")]
impl Default for MIDL_STUBLESS_PROXY_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "objidlbase")]
#[derive(Clone, Copy)]
pub struct MIDL_STUB_DESC {
    pub RpcInterfaceInformation: *mut core::ffi::c_void,
    pub pfnAllocate: *mut u8,
    pub pfnFree: *mut u8,
    pub IMPLICIT_HANDLE_INFO: MIDL_STUB_DESC_0,
    pub apfnNdrRundownRoutines: *const NDR_RUNDOWN,
    pub aGenericBindingRoutinePairs: *const GENERIC_BINDING_ROUTINE_PAIR,
    pub apfnExprEval: *const EXPR_EVAL,
    pub aXmitQuintuple: *const XMIT_ROUTINE_QUINTUPLE,
    pub pFormatTypes: *const u8,
    pub fCheckBounds: i32,
    pub Version: u32,
    pub pMallocFreeStruct: *mut MALLOC_FREE_STRUCT,
    pub MIDLVersion: i32,
    pub CommFaultOffsets: *const COMM_FAULT_OFFSETS,
    pub aUserMarshalQuadruple: *const USER_MARSHAL_ROUTINE_QUADRUPLE,
    pub NotifyRoutineTable: *const NDR_NOTIFY_ROUTINE,
    pub mFlags: usize,
    pub CsRoutineTables: *const NDR_CS_ROUTINES,
    pub ProxyServerInfo: *mut core::ffi::c_void,
    pub pExprInfo: *const NDR_EXPR_DESC,
}
#[cfg(feature = "objidlbase")]
impl Default for MIDL_STUB_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "objidlbase")]
#[derive(Clone, Copy)]
pub union MIDL_STUB_DESC_0 {
    pub pAutoHandle: *mut handle_t,
    pub pPrimitiveHandle: *mut handle_t,
    pub pGenericBindingInfo: PGENERIC_BINDING_INFO,
}
#[cfg(feature = "objidlbase")]
impl Default for MIDL_STUB_DESC_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "objidlbase")]
#[derive(Clone, Debug, PartialEq)]
pub struct MIDL_STUB_MESSAGE {
    pub RpcMsg: PRPC_MESSAGE,
    pub Buffer: *mut u8,
    pub BufferStart: *mut u8,
    pub BufferEnd: *mut u8,
    pub BufferMark: *mut u8,
    pub BufferLength: u32,
    pub MemorySize: u32,
    pub Memory: *mut u8,
    pub IsClient: u8,
    pub Pad: u8,
    pub uFlags2: u16,
    pub ReuseBuffer: i32,
    pub pAllocAllNodesContext: *mut NDR_ALLOC_ALL_NODES_CONTEXT,
    pub pPointerQueueState: *mut NDR_POINTER_QUEUE_STATE,
    pub IgnoreEmbeddedPointers: i32,
    pub PointerBufferMark: *mut u8,
    pub CorrDespIncrement: u8,
    pub uFlags: u8,
    pub UniquePtrCount: u16,
    pub MaxCount: usize,
    pub Offset: u32,
    pub ActualCount: u32,
    pub pfnAllocate: *mut u8,
    pub pfnFree: *mut u8,
    pub StackTop: *mut u8,
    pub pPresentedType: *mut u8,
    pub pTransmitType: *mut u8,
    pub SavedHandle: handle_t,
    pub StubDesc: *const MIDL_STUB_DESC,
    pub FullPtrXlatTables: *mut FULL_PTR_XLAT_TABLES,
    pub FullPtrRefId: u32,
    pub PointerLength: u32,
    pub _bitfield: u32,
    pub dwDestContext: u32,
    pub pvDestContext: *mut core::ffi::c_void,
    pub SavedContextHandles: *mut NDR_SCONTEXT,
    pub ParamNumber: i32,
    pub pRpcChannelBuffer: core::mem::ManuallyDrop<Option<super::objidlbase::IRpcChannelBuffer>>,
    pub pArrayInfo: PARRAY_INFO,
    pub SizePtrCountArray: *mut u32,
    pub SizePtrOffsetArray: *mut u32,
    pub SizePtrLengthArray: *mut u32,
    pub pArgQueue: *mut core::ffi::c_void,
    pub dwStubPhase: u32,
    pub LowStackMark: *mut core::ffi::c_void,
    pub pAsyncMsg: PNDR_ASYNC_MESSAGE,
    pub pCorrInfo: PNDR_CORRELATION_INFO,
    pub pCorrMemory: *mut u8,
    pub pMemoryList: *mut core::ffi::c_void,
    pub pCSInfo: isize,
    pub ConformanceMark: *mut u8,
    pub VarianceMark: *mut u8,
    pub Unused: isize,
    pub pContext: *mut _NDR_PROC_CONTEXT,
    pub ContextHandleHash: *mut core::ffi::c_void,
    pub pUserMarshalList: *mut core::ffi::c_void,
    pub pFullPtrFormat: *mut u8,
    pub Reserved51_4: isize,
    pub Reserved51_5: isize,
}
#[cfg(feature = "objidlbase")]
impl Default for MIDL_STUB_MESSAGE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MIDL_SYNTAX_INFO {
    pub TransferSyntax: RPC_SYNTAX_IDENTIFIER,
    pub DispatchTable: *mut RPC_DISPATCH_TABLE,
    pub ProcString: PFORMAT_STRING,
    pub FmtStringOffset: *const u16,
    pub TypeString: PFORMAT_STRING,
    pub aUserMarshalQuadruple: *const core::ffi::c_void,
    pub pMethodProperties: *const MIDL_INTERFACE_METHOD_PROPERTIES,
    pub pReserved2: usize,
}
impl Default for MIDL_SYNTAX_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "objidlbase")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MIDL_WINRT_TYPE_SERIALIZATION_INFO {
    pub Version: u32,
    pub TypeFormatString: PFORMAT_STRING,
    pub FormatStringSize: u16,
    pub TypeOffset: u16,
    pub StubDesc: PMIDL_STUB_DESC,
}
pub const MIDL_WINRT_TYPE_SERIALIZATION_INFO_CURRENT_VERSION: u32 = 1;
pub const MarshalDirectionMarshal: LRPC_SYSTEM_HANDLE_MARSHAL_DIRECTION = 0;
pub const MarshalDirectionUnmarshal: LRPC_SYSTEM_HANDLE_MARSHAL_DIRECTION = 1;
pub const MaxNumberOfEEInfoParams: u32 = 4;
pub const MidlInterceptionInfoVersionOne: i32 = 1;
pub const MidlWinrtTypeSerializationInfoVersionOne: i32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NDR_ALLOC_ALL_NODES_CONTEXT(pub u8);
pub const NDR_ASCII_CHAR: u32 = 0;
pub const NDR_BIG_ENDIAN: u32 = 0;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NDR_CCONTEXT(pub *mut core::ffi::c_void);
impl NDR_CCONTEXT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for NDR_CCONTEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const NDR_CHAR_REP_MASK: u32 = 15;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NDR_CS_ROUTINES {
    pub pSizeConvertRoutines: *mut NDR_CS_SIZE_CONVERT_ROUTINES,
    pub pTagGettingRoutines: *mut CS_TAG_GETTING_ROUTINE,
}
impl Default for NDR_CS_ROUTINES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct NDR_CS_SIZE_CONVERT_ROUTINES {
    pub pfnNetSize: CS_TYPE_NET_SIZE_ROUTINE,
    pub pfnToNetCs: CS_TYPE_TO_NETCS_ROUTINE,
    pub pfnLocalSize: CS_TYPE_LOCAL_SIZE_ROUTINE,
    pub pfnFromNetCs: CS_TYPE_FROM_NETCS_ROUTINE,
}
pub const NDR_CUSTOM_OR_DEFAULT_ALLOCATOR: u32 = 268435456;
pub const NDR_DEFAULT_ALLOCATOR: u32 = 536870912;
pub const NDR_EBCDIC_CHAR: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NDR_EXPR_DESC {
    pub pOffset: *const u16,
    pub pFormatExpr: PFORMAT_STRING,
}
impl Default for NDR_EXPR_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const NDR_FLOAT_REP_MASK: u32 = 65280;
pub const NDR_IBM_FLOAT: u32 = 768;
pub const NDR_IEEE_FLOAT: u32 = 0;
pub const NDR_INT_REP_MASK: u32 = 240;
pub const NDR_LITTLE_ENDIAN: u32 = 16;
pub const NDR_LOCAL_DATA_REPRESENTATION: u32 = 16;
pub const NDR_LOCAL_ENDIAN: u32 = 16;
pub type NDR_NOTIFY2_ROUTINE = Option<unsafe extern "system" fn(flag: boolean)>;
pub type NDR_NOTIFY_ROUTINE = Option<unsafe extern "system" fn()>;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NDR_POINTER_QUEUE_STATE(pub u8);
pub type NDR_RUNDOWN = Option<unsafe extern "system" fn(context: *mut core::ffi::c_void)>;
pub type NDR_SCONTEXT = *mut _NDR_SCONTEXT;
#[repr(C)]
#[cfg(feature = "objidlbase")]
pub struct NDR_USER_MARSHAL_INFO {
    pub InformationLevel: u32,
    pub Anonymous: NDR_USER_MARSHAL_INFO_0,
}
#[cfg(feature = "objidlbase")]
impl Clone for NDR_USER_MARSHAL_INFO {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
#[cfg(feature = "objidlbase")]
impl Default for NDR_USER_MARSHAL_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "objidlbase")]
pub union NDR_USER_MARSHAL_INFO_0 {
    pub Level1: core::mem::ManuallyDrop<NDR_USER_MARSHAL_INFO_LEVEL1>,
}
#[cfg(feature = "objidlbase")]
impl Clone for NDR_USER_MARSHAL_INFO_0 {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
#[cfg(feature = "objidlbase")]
impl Default for NDR_USER_MARSHAL_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "objidlbase")]
#[derive(Clone, Debug, PartialEq)]
pub struct NDR_USER_MARSHAL_INFO_LEVEL1 {
    pub Buffer: *mut core::ffi::c_void,
    pub BufferSize: u32,
    pub pfnAllocate: *mut u8,
    pub pfnFree: *mut u8,
    pub pRpcChannelBuffer: core::mem::ManuallyDrop<Option<super::objidlbase::IRpcChannelBuffer>>,
    pub Reserved: [usize; 5],
}
#[cfg(feature = "objidlbase")]
impl Default for NDR_USER_MARSHAL_INFO_LEVEL1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const NDR_VAX_FLOAT: u32 = 256;
pub const NT351_INTERFACE_SIZE: u32 = 64;
pub type PARAM_OFFSETTABLE = *mut u16;
pub type PARRAY_INFO = *mut ARRAY_INFO;
#[cfg(all(feature = "minwinbase", feature = "windef", feature = "winnt"))]
pub type PFN_RPCNOTIFICATION_ROUTINE = Option<unsafe extern "system" fn(pasync: *mut RPC_ASYNC_STATE, context: *mut core::ffi::c_void, event: RPC_ASYNC_EVENT)>;
pub type PFORMAT_STRING = *const u8;
pub type PFULL_PTR_XLAT_TABLES = *mut FULL_PTR_XLAT_TABLES;
pub type PGENERIC_BINDING_INFO = *mut GENERIC_BINDING_INFO;
pub type PGENERIC_BINDING_ROUTINE_PAIR = *mut GENERIC_BINDING_ROUTINE_PAIR;
pub type PMIDL_INTERCEPTION_INFO = *mut MIDL_INTERCEPTION_INFO;
pub type PMIDL_METHOD_PROPERTY = *mut MIDL_METHOD_PROPERTY;
pub type PMIDL_METHOD_PROPERTY_MAP = *mut MIDL_METHOD_PROPERTY_MAP;
#[cfg(feature = "objidlbase")]
pub type PMIDL_SERVER_INFO = *mut MIDL_SERVER_INFO;
#[cfg(feature = "objidlbase")]
pub type PMIDL_STUBLESS_PROXY_INFO = *mut MIDL_STUBLESS_PROXY_INFO;
#[cfg(feature = "objidlbase")]
pub type PMIDL_STUB_DESC = *const MIDL_STUB_DESC;
#[cfg(feature = "objidlbase")]
pub type PMIDL_STUB_MESSAGE = *mut MIDL_STUB_MESSAGE;
pub type PMIDL_SYNTAX_INFO = *mut MIDL_SYNTAX_INFO;
#[cfg(feature = "objidlbase")]
pub type PMIDL_WINRT_TYPE_SERIALIZATION_INFO = *mut MIDL_WINRT_TYPE_SERIALIZATION_INFO;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PMIDL_XMIT_TYPE(pub *mut core::ffi::c_void);
impl PMIDL_XMIT_TYPE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PMIDL_XMIT_TYPE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type PNDR_ASYNC_MESSAGE = *mut _NDR_ASYNC_MESSAGE;
pub type PNDR_CORRELATION_INFO = *mut _NDR_CORRELATION_INFO;
pub type PPARAM_OFFSETTABLE = *mut u16;
pub const PROTOCOL_ADDRESS_CHANGE: RPC_ADDRESS_CHANGE_TYPE = 3;
pub const PROTOCOL_LOADED: RPC_ADDRESS_CHANGE_TYPE = 2;
pub const PROTOCOL_NOT_LOADED: RPC_ADDRESS_CHANGE_TYPE = 1;
pub const PROXY_CALCSIZE: PROXY_PHASE = 0;
pub const PROXY_GETBUFFER: PROXY_PHASE = 1;
pub const PROXY_MARSHAL: PROXY_PHASE = 2;
pub type PROXY_PHASE = i32;
pub const PROXY_SENDRECEIVE: PROXY_PHASE = 3;
pub const PROXY_UNMARSHAL: PROXY_PHASE = 4;
#[cfg(all(feature = "minwinbase", feature = "windef", feature = "winnt"))]
pub type PRPC_ASYNC_NOTIFICATION_INFO = *mut RPC_ASYNC_NOTIFICATION_INFO;
#[cfg(all(feature = "minwinbase", feature = "windef", feature = "winnt"))]
pub type PRPC_ASYNC_STATE = *mut RPC_ASYNC_STATE;
pub type PRPC_BINDING_HANDLE_OPTIONS_V1 = *mut RPC_BINDING_HANDLE_OPTIONS_V1;
pub type PRPC_BINDING_HANDLE_SECURITY_V1_A = *mut RPC_BINDING_HANDLE_SECURITY_V1_A;
pub type PRPC_BINDING_HANDLE_SECURITY_V1_W = *mut RPC_BINDING_HANDLE_SECURITY_V1_W;
pub type PRPC_BINDING_HANDLE_TEMPLATE_V1_A = *mut RPC_BINDING_HANDLE_TEMPLATE_V1_A;
pub type PRPC_BINDING_HANDLE_TEMPLATE_V1_W = *mut RPC_BINDING_HANDLE_TEMPLATE_V1_W;
pub type PRPC_CALL_LOCAL_ADDRESS_V1 = *mut RPC_CALL_LOCAL_ADDRESS_V1;
pub type PRPC_CLIENT_INFORMATION1 = *mut RPC_CLIENT_INFORMATION1;
pub type PRPC_CLIENT_INTERFACE = *mut RPC_CLIENT_INTERFACE;
pub type PRPC_DISPATCH_TABLE = *mut RPC_DISPATCH_TABLE;
pub type PRPC_ENDPOINT_TEMPLATEA = *mut RPC_ENDPOINT_TEMPLATEA;
pub type PRPC_ENDPOINT_TEMPLATEW = *mut RPC_ENDPOINT_TEMPLATEW;
pub type PRPC_HTTP_TRANSPORT_CREDENTIALS_A = *mut RPC_HTTP_TRANSPORT_CREDENTIALS_A;
pub type PRPC_HTTP_TRANSPORT_CREDENTIALS_V2_A = *mut RPC_HTTP_TRANSPORT_CREDENTIALS_V2_A;
pub type PRPC_HTTP_TRANSPORT_CREDENTIALS_V2_W = *mut RPC_HTTP_TRANSPORT_CREDENTIALS_V2_W;
pub type PRPC_HTTP_TRANSPORT_CREDENTIALS_V3_A = *mut RPC_HTTP_TRANSPORT_CREDENTIALS_V3_A;
pub type PRPC_HTTP_TRANSPORT_CREDENTIALS_V3_W = *mut RPC_HTTP_TRANSPORT_CREDENTIALS_V3_W;
pub type PRPC_HTTP_TRANSPORT_CREDENTIALS_W = *mut RPC_HTTP_TRANSPORT_CREDENTIALS_W;
pub type PRPC_IMPORT_CONTEXT_P = *mut RPC_IMPORT_CONTEXT_P;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PRPC_INTERFACE_GROUP(pub *mut *mut core::ffi::c_void);
impl PRPC_INTERFACE_GROUP {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PRPC_INTERFACE_GROUP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type PRPC_INTERFACE_TEMPLATEA = *mut RPC_INTERFACE_TEMPLATEA;
pub type PRPC_INTERFACE_TEMPLATEW = *mut RPC_INTERFACE_TEMPLATEW;
pub type PRPC_MESSAGE = *mut RPC_MESSAGE;
pub type PRPC_POLICY = *mut RPC_POLICY;
pub type PRPC_PROTSEQ_ENDPOINT = *mut RPC_PROTSEQ_ENDPOINT;
pub type PRPC_RUNDOWN = Option<unsafe extern "system" fn(associationcontext: *mut core::ffi::c_void)>;
pub type PRPC_SECURITY_QOS = *mut RPC_SECURITY_QOS;
pub type PRPC_SECURITY_QOS_V2_A = *mut RPC_SECURITY_QOS_V2_A;
pub type PRPC_SECURITY_QOS_V2_W = *mut RPC_SECURITY_QOS_V2_W;
pub type PRPC_SECURITY_QOS_V3_A = *mut RPC_SECURITY_QOS_V3_A;
pub type PRPC_SECURITY_QOS_V3_W = *mut RPC_SECURITY_QOS_V3_W;
pub type PRPC_SECURITY_QOS_V4_A = *mut RPC_SECURITY_QOS_V4_A;
pub type PRPC_SECURITY_QOS_V4_W = *mut RPC_SECURITY_QOS_V4_W;
pub type PRPC_SECURITY_QOS_V5_A = *mut RPC_SECURITY_QOS_V5_A;
pub type PRPC_SECURITY_QOS_V5_W = *mut RPC_SECURITY_QOS_V5_W;
pub type PRPC_SEC_CONTEXT_KEY_INFO = *mut RPC_SEC_CONTEXT_KEY_INFO;
pub type PRPC_SERVER_INTERFACE = *mut RPC_SERVER_INTERFACE;
pub type PRPC_SYNTAX_IDENTIFIER = *mut RPC_SYNTAX_IDENTIFIER;
pub type PSCONTEXT_QUEUE = *mut SCONTEXT_QUEUE;
pub type PSEC_WINNT_AUTH_IDENTITY_A = *mut SEC_WINNT_AUTH_IDENTITY_A;
pub type PSEC_WINNT_AUTH_IDENTITY_W = *mut SEC_WINNT_AUTH_IDENTITY_W;
#[cfg(feature = "objidlbase")]
pub type PXMIT_ROUTINE_QUINTUPLE = *mut XMIT_ROUTINE_QUINTUPLE;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RDR_CALLOUT_STATE {
    pub LastError: RPC_STATUS,
    pub LastEEInfo: *mut core::ffi::c_void,
    pub LastCalledStage: RPC_HTTP_REDIRECTOR_STAGE,
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
pub const RPCHTTP_RS_ACCESS_1: RPC_HTTP_REDIRECTOR_STAGE = 2;
pub const RPCHTTP_RS_ACCESS_2: RPC_HTTP_REDIRECTOR_STAGE = 4;
pub const RPCHTTP_RS_INTERFACE: RPC_HTTP_REDIRECTOR_STAGE = 5;
pub const RPCHTTP_RS_REDIRECT: RPC_HTTP_REDIRECTOR_STAGE = 1;
pub const RPCHTTP_RS_SESSION: RPC_HTTP_REDIRECTOR_STAGE = 3;
pub type RPCLT_PDU_FILTER_FUNC = Option<unsafe extern "system" fn(buffer: *mut core::ffi::c_void, bufferlength: u32, fdatagram: i32)>;
#[cfg(all(feature = "minwinbase", feature = "windef", feature = "winnt"))]
pub type RPCNOTIFICATION_ROUTINE = Option<unsafe extern "system" fn(pasync: *mut RPC_ASYNC_STATE, context: *mut core::ffi::c_void, event: RPC_ASYNC_EVENT)>;
pub type RPC_ADDRESS_CHANGE_FN = Option<unsafe extern "system" fn(arg: *mut core::ffi::c_void)>;
pub type RPC_ADDRESS_CHANGE_TYPE = i32;
pub type RPC_ASYNC_EVENT = i32;
#[repr(C)]
#[cfg(all(feature = "minwinbase", feature = "windef", feature = "winnt"))]
#[derive(Clone, Copy)]
pub union RPC_ASYNC_NOTIFICATION_INFO {
    pub APC: RPC_ASYNC_NOTIFICATION_INFO_0,
    pub IOC: RPC_ASYNC_NOTIFICATION_INFO_1,
    pub HWND: RPC_ASYNC_NOTIFICATION_INFO_2,
    pub hEvent: super::winnt::HANDLE,
    pub NotificationRoutine: PFN_RPCNOTIFICATION_ROUTINE,
}
#[cfg(all(feature = "minwinbase", feature = "windef", feature = "winnt"))]
impl Default for RPC_ASYNC_NOTIFICATION_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "minwinbase", feature = "windef", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default)]
pub struct RPC_ASYNC_NOTIFICATION_INFO_0 {
    pub NotificationRoutine: PFN_RPCNOTIFICATION_ROUTINE,
    pub hThread: super::winnt::HANDLE,
}
#[repr(C)]
#[cfg(all(feature = "minwinbase", feature = "windef", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct RPC_ASYNC_NOTIFICATION_INFO_1 {
    pub hIOPort: super::winnt::HANDLE,
    pub dwNumberOfBytesTransferred: u32,
    pub dwCompletionKey: usize,
    pub lpOverlapped: super::minwinbase::LPOVERLAPPED,
}
#[repr(C)]
#[cfg(all(feature = "minwinbase", feature = "windef", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct RPC_ASYNC_NOTIFICATION_INFO_2 {
    pub hWnd: super::windef::HWND,
    pub Msg: u32,
}
#[repr(C)]
#[cfg(all(feature = "minwinbase", feature = "windef", feature = "winnt"))]
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
#[cfg(all(feature = "minwinbase", feature = "windef", feature = "winnt"))]
impl Default for RPC_ASYNC_STATE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RPC_AUTHZ_HANDLE(pub *mut core::ffi::c_void);
impl RPC_AUTHZ_HANDLE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for RPC_AUTHZ_HANDLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RPC_AUTH_IDENTITY_HANDLE(pub *mut core::ffi::c_void);
impl RPC_AUTH_IDENTITY_HANDLE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for RPC_AUTH_IDENTITY_HANDLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type RPC_AUTH_KEY_RETRIEVAL_FN = Option<unsafe extern "system" fn(arg: *const core::ffi::c_void, serverprincname: *const u16, keyver: u32, key: *mut *mut core::ffi::c_void, status: *mut RPC_STATUS)>;
pub const RPC_BHO_DONTLINGER: u32 = 2;
pub const RPC_BHO_EXCLUSIVE_AND_GUARANTEED: u32 = 4;
pub const RPC_BHO_NONCAUSAL: u32 = 1;
pub const RPC_BHT_OBJECT_UUID_VALID: u32 = 1;
pub type RPC_BINDING_HANDLE = I_RPC_HANDLE;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct RPC_BINDING_HANDLE_OPTIONS_V1 {
    pub Version: u32,
    pub Flags: u32,
    pub ComTimeout: u32,
    pub CallTimeout: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RPC_BINDING_HANDLE_SECURITY_V1_A {
    pub Version: u32,
    pub ServerPrincName: *mut u8,
    pub AuthnLevel: u32,
    pub AuthnSvc: u32,
    pub AuthIdentity: *mut SEC_WINNT_AUTH_IDENTITY_A,
    pub SecurityQos: *mut RPC_SECURITY_QOS,
}
impl Default for RPC_BINDING_HANDLE_SECURITY_V1_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RPC_BINDING_HANDLE_SECURITY_V1_W {
    pub Version: u32,
    pub ServerPrincName: *mut u16,
    pub AuthnLevel: u32,
    pub AuthnSvc: u32,
    pub AuthIdentity: *mut SEC_WINNT_AUTH_IDENTITY_W,
    pub SecurityQos: *mut RPC_SECURITY_QOS,
}
impl Default for RPC_BINDING_HANDLE_SECURITY_V1_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct RPC_BINDING_HANDLE_TEMPLATE_V1_A {
    pub Version: u32,
    pub Flags: u32,
    pub ProtocolSequence: u32,
    pub NetworkAddress: *mut u8,
    pub StringEndpoint: *mut u8,
    pub u1: RPC_BINDING_HANDLE_TEMPLATE_V1_A_0,
    pub ObjectUuid: windows_core::GUID,
}
impl Default for RPC_BINDING_HANDLE_TEMPLATE_V1_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union RPC_BINDING_HANDLE_TEMPLATE_V1_A_0 {
    pub Reserved: *mut u8,
}
impl Default for RPC_BINDING_HANDLE_TEMPLATE_V1_A_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct RPC_BINDING_HANDLE_TEMPLATE_V1_W {
    pub Version: u32,
    pub Flags: u32,
    pub ProtocolSequence: u32,
    pub NetworkAddress: *mut u16,
    pub StringEndpoint: *mut u16,
    pub u1: RPC_BINDING_HANDLE_TEMPLATE_V1_W_0,
    pub ObjectUuid: windows_core::GUID,
}
impl Default for RPC_BINDING_HANDLE_TEMPLATE_V1_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union RPC_BINDING_HANDLE_TEMPLATE_V1_W_0 {
    pub Reserved: *mut u16,
}
impl Default for RPC_BINDING_HANDLE_TEMPLATE_V1_W_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RPC_BINDING_VECTOR {
    pub Count: u32,
    pub BindingH: [RPC_BINDING_HANDLE; 1],
}
impl Default for RPC_BINDING_VECTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type RPC_BLOCKING_FN = Option<unsafe extern "system" fn(hwnd: *mut core::ffi::c_void, context: *mut core::ffi::c_void, hsyncevent: *mut core::ffi::c_void) -> RPC_STATUS>;
pub const RPC_BUFFER_ASYNC: u32 = 32768;
pub const RPC_BUFFER_COMPLETE: u32 = 4096;
pub const RPC_BUFFER_EXTRA: u32 = 16384;
pub const RPC_BUFFER_NONOTIFY: u32 = 65536;
pub const RPC_BUFFER_PARTIAL: u32 = 8192;
pub type RPC_BUFPTR = *mut u8;
#[cfg(feature = "winnt")]
pub type RPC_CALL_ATTRIBUTES = RPC_CALL_ATTRIBUTES_V3_A;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RPC_CALL_ATTRIBUTES_V1_A {
    pub Version: u32,
    pub Flags: u32,
    pub ServerPrincipalNameBufferLength: u32,
    pub ServerPrincipalName: *mut u8,
    pub ClientPrincipalNameBufferLength: u32,
    pub ClientPrincipalName: *mut u8,
    pub AuthenticationLevel: u32,
    pub AuthenticationService: u32,
    pub NullSession: windows_core::BOOL,
}
impl Default for RPC_CALL_ATTRIBUTES_V1_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RPC_CALL_ATTRIBUTES_V1_W {
    pub Version: u32,
    pub Flags: u32,
    pub ServerPrincipalNameBufferLength: u32,
    pub ServerPrincipalName: *mut u16,
    pub ClientPrincipalNameBufferLength: u32,
    pub ClientPrincipalName: *mut u16,
    pub AuthenticationLevel: u32,
    pub AuthenticationService: u32,
    pub NullSession: windows_core::BOOL,
}
impl Default for RPC_CALL_ATTRIBUTES_V1_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RPC_CALL_ATTRIBUTES_V2_A {
    pub Version: u32,
    pub Flags: u32,
    pub ServerPrincipalNameBufferLength: u32,
    pub ServerPrincipalName: *mut u8,
    pub ClientPrincipalNameBufferLength: u32,
    pub ClientPrincipalName: *mut u8,
    pub AuthenticationLevel: u32,
    pub AuthenticationService: u32,
    pub NullSession: windows_core::BOOL,
    pub KernelModeCaller: windows_core::BOOL,
    pub ProtocolSequence: u32,
    pub IsClientLocal: u32,
    pub ClientPID: super::winnt::HANDLE,
    pub CallStatus: u32,
    pub CallType: RpcCallType,
    pub CallLocalAddress: *mut RPC_CALL_LOCAL_ADDRESS_V1,
    pub OpNum: u16,
    pub InterfaceUuid: windows_core::GUID,
}
#[cfg(feature = "winnt")]
impl Default for RPC_CALL_ATTRIBUTES_V2_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RPC_CALL_ATTRIBUTES_V2_W {
    pub Version: u32,
    pub Flags: u32,
    pub ServerPrincipalNameBufferLength: u32,
    pub ServerPrincipalName: *mut u16,
    pub ClientPrincipalNameBufferLength: u32,
    pub ClientPrincipalName: *mut u16,
    pub AuthenticationLevel: u32,
    pub AuthenticationService: u32,
    pub NullSession: windows_core::BOOL,
    pub KernelModeCaller: windows_core::BOOL,
    pub ProtocolSequence: u32,
    pub IsClientLocal: RpcCallClientLocality,
    pub ClientPID: super::winnt::HANDLE,
    pub CallStatus: u32,
    pub CallType: RpcCallType,
    pub CallLocalAddress: *mut RPC_CALL_LOCAL_ADDRESS_V1,
    pub OpNum: u16,
    pub InterfaceUuid: windows_core::GUID,
}
#[cfg(feature = "winnt")]
impl Default for RPC_CALL_ATTRIBUTES_V2_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RPC_CALL_ATTRIBUTES_V3_A {
    pub Version: u32,
    pub Flags: u32,
    pub ServerPrincipalNameBufferLength: u32,
    pub ServerPrincipalName: *mut u8,
    pub ClientPrincipalNameBufferLength: u32,
    pub ClientPrincipalName: *mut u8,
    pub AuthenticationLevel: u32,
    pub AuthenticationService: u32,
    pub NullSession: windows_core::BOOL,
    pub KernelModeCaller: windows_core::BOOL,
    pub ProtocolSequence: u32,
    pub IsClientLocal: u32,
    pub ClientPID: super::winnt::HANDLE,
    pub CallStatus: u32,
    pub CallType: RpcCallType,
    pub CallLocalAddress: *mut RPC_CALL_LOCAL_ADDRESS_V1,
    pub OpNum: u16,
    pub InterfaceUuid: windows_core::GUID,
    pub ClientIdentifierBufferLength: u32,
    pub ClientIdentifier: *mut u8,
}
#[cfg(feature = "winnt")]
impl Default for RPC_CALL_ATTRIBUTES_V3_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RPC_CALL_ATTRIBUTES_V3_W {
    pub Version: u32,
    pub Flags: u32,
    pub ServerPrincipalNameBufferLength: u32,
    pub ServerPrincipalName: *mut u16,
    pub ClientPrincipalNameBufferLength: u32,
    pub ClientPrincipalName: *mut u16,
    pub AuthenticationLevel: u32,
    pub AuthenticationService: u32,
    pub NullSession: windows_core::BOOL,
    pub KernelModeCaller: windows_core::BOOL,
    pub ProtocolSequence: u32,
    pub IsClientLocal: RpcCallClientLocality,
    pub ClientPID: super::winnt::HANDLE,
    pub CallStatus: u32,
    pub CallType: RpcCallType,
    pub CallLocalAddress: *mut RPC_CALL_LOCAL_ADDRESS_V1,
    pub OpNum: u16,
    pub InterfaceUuid: windows_core::GUID,
    pub ClientIdentifierBufferLength: u32,
    pub ClientIdentifier: *mut u8,
}
#[cfg(feature = "winnt")]
impl Default for RPC_CALL_ATTRIBUTES_V3_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const RPC_CALL_ATTRIBUTES_VERSION: u32 = 3;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
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
pub type RPC_CLIENT_ALLOC = Option<unsafe extern "system" fn(size: usize) -> *mut core::ffi::c_void>;
pub type RPC_CLIENT_FREE = Option<unsafe extern "system" fn(ptr: *const core::ffi::c_void)>;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RPC_CLIENT_INFORMATION1 {
    pub UserName: *mut u8,
    pub ComputerName: *mut u8,
    pub Privilege: u16,
    pub AuthFlags: u32,
}
impl Default for RPC_CLIENT_INFORMATION1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
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
impl Default for RPC_CLIENT_INTERFACE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const RPC_CONTEXT_HANDLE_DEFAULT_FLAGS: u32 = 0;
pub const RPC_CONTEXT_HANDLE_DONT_SERIALIZE: u32 = 536870912;
pub const RPC_CONTEXT_HANDLE_FLAGS: u32 = 805306368;
pub const RPC_CONTEXT_HANDLE_SERIALIZE: u32 = 268435456;
pub type RPC_CSTR = *mut u8;
pub type RPC_CWSTR = *const u16;
pub const RPC_C_AUTHN_CLOUD_AP: u32 = 36;
pub const RPC_C_AUTHN_DCE_PRIVATE: u32 = 1;
pub const RPC_C_AUTHN_DCE_PUBLIC: u32 = 2;
pub const RPC_C_AUTHN_DEC_PUBLIC: u32 = 4;
pub const RPC_C_AUTHN_DEFAULT: u32 = 4294967295;
pub const RPC_C_AUTHN_DIGEST: u32 = 21;
pub const RPC_C_AUTHN_DPA: u32 = 17;
pub const RPC_C_AUTHN_GSS_KERBEROS: u32 = 16;
pub const RPC_C_AUTHN_GSS_NEGOTIATE: u32 = 9;
pub const RPC_C_AUTHN_GSS_SCHANNEL: u32 = 14;
pub const RPC_C_AUTHN_INFO_TYPE_HTTP: u32 = 1;
pub const RPC_C_AUTHN_KERNEL: u32 = 20;
pub const RPC_C_AUTHN_LEVEL_CALL: u32 = 3;
pub const RPC_C_AUTHN_LEVEL_CONNECT: u32 = 2;
pub const RPC_C_AUTHN_LEVEL_DEFAULT: u32 = 0;
pub const RPC_C_AUTHN_LEVEL_NONE: u32 = 1;
pub const RPC_C_AUTHN_LEVEL_PKT: u32 = 4;
pub const RPC_C_AUTHN_LEVEL_PKT_INTEGRITY: u32 = 5;
pub const RPC_C_AUTHN_LEVEL_PKT_PRIVACY: u32 = 6;
pub const RPC_C_AUTHN_LIVEXP_SSP: u32 = 35;
pub const RPC_C_AUTHN_LIVE_SSP: u32 = 32;
pub const RPC_C_AUTHN_MQ: u32 = 100;
pub const RPC_C_AUTHN_MSN: u32 = 18;
pub const RPC_C_AUTHN_MSONLINE: u32 = 82;
pub const RPC_C_AUTHN_NEGO_EXTENDER: u32 = 30;
pub const RPC_C_AUTHN_NONE: u32 = 0;
pub const RPC_C_AUTHN_PKU2U: u32 = 31;
pub const RPC_C_AUTHN_WINNT: u32 = 10;
pub const RPC_C_AUTHZ_DCE: u32 = 2;
pub const RPC_C_AUTHZ_DEFAULT: u32 = 4294967295;
pub const RPC_C_AUTHZ_NAME: u32 = 1;
pub const RPC_C_AUTHZ_NONE: u32 = 0;
pub const RPC_C_BINDING_DEFAULT_TIMEOUT: u32 = 5;
pub const RPC_C_BINDING_INFINITE_TIMEOUT: u32 = 10;
pub const RPC_C_BINDING_MAX_TIMEOUT: u32 = 9;
pub const RPC_C_BINDING_MIN_TIMEOUT: u32 = 0;
pub const RPC_C_BIND_TO_ALL_NICS: u32 = 1;
pub const RPC_C_CANCEL_INFINITE_TIMEOUT: i32 = -1;
pub const RPC_C_DONT_FAIL: u32 = 4;
pub const RPC_C_EP_ALL_ELTS: u32 = 0;
pub const RPC_C_EP_MATCH_BY_BOTH: u32 = 3;
pub const RPC_C_EP_MATCH_BY_IF: u32 = 1;
pub const RPC_C_EP_MATCH_BY_OBJ: u32 = 2;
pub const RPC_C_FULL_CERT_CHAIN: u32 = 1;
pub const RPC_C_HTTP_AUTHN_SCHEME_BASIC: u32 = 1;
pub const RPC_C_HTTP_AUTHN_SCHEME_CERT: u32 = 65536;
pub const RPC_C_HTTP_AUTHN_SCHEME_DIGEST: u32 = 8;
pub const RPC_C_HTTP_AUTHN_SCHEME_NEGOTIATE: u32 = 16;
pub const RPC_C_HTTP_AUTHN_SCHEME_NTLM: u32 = 2;
pub const RPC_C_HTTP_AUTHN_SCHEME_PASSPORT: u32 = 4;
pub const RPC_C_HTTP_AUTHN_TARGET_PROXY: u32 = 2;
pub const RPC_C_HTTP_AUTHN_TARGET_SERVER: u32 = 1;
pub const RPC_C_HTTP_FLAG_ENABLE_CERT_REVOCATION_CHECK: u32 = 16;
pub const RPC_C_HTTP_FLAG_IGNORE_CERT_CN_INVALID: u32 = 8;
pub const RPC_C_HTTP_FLAG_USE_FIRST_AUTH_SCHEME: u32 = 2;
pub const RPC_C_HTTP_FLAG_USE_SSL: u32 = 1;
pub const RPC_C_IMP_LEVEL_ANONYMOUS: u32 = 1;
pub const RPC_C_IMP_LEVEL_DEFAULT: u32 = 0;
pub const RPC_C_IMP_LEVEL_DELEGATE: u32 = 4;
pub const RPC_C_IMP_LEVEL_IDENTIFY: u32 = 2;
pub const RPC_C_IMP_LEVEL_IMPERSONATE: u32 = 3;
pub const RPC_C_INFINITE_TIMEOUT: i32 = -1;
pub const RPC_C_LISTEN_MAX_CALLS_DEFAULT: u32 = 1234;
pub const RPC_C_MGMT_INQ_IF_IDS: u32 = 0;
pub const RPC_C_MGMT_INQ_PRINC_NAME: u32 = 1;
pub const RPC_C_MGMT_INQ_STATS: u32 = 2;
pub const RPC_C_MGMT_IS_SERVER_LISTEN: u32 = 3;
pub const RPC_C_MGMT_STOP_SERVER_LISTEN: u32 = 4;
pub const RPC_C_NOTIFY_ON_SEND_COMPLETE: u32 = 1;
pub const RPC_C_NS_DEFAULT_EXP_AGE: i32 = -1;
pub const RPC_C_NS_SYNTAX_DCE: u32 = 3;
pub const RPC_C_NS_SYNTAX_DEFAULT: u32 = 0;
pub const RPC_C_OPT_ASYNC_BLOCK: u32 = 15;
pub const RPC_C_OPT_BINDING_NONCAUSAL: u32 = 9;
pub const RPC_C_OPT_CALL_TIMEOUT: u32 = 12;
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
pub const RPC_C_OPT_DONT_LINGER: u32 = 13;
pub const RPC_C_OPT_MAX_OPTIONS: u32 = 17;
pub const RPC_C_OPT_OPTIMIZE_TIME: u32 = 16;
pub const RPC_C_OPT_PRIVATE_BREAK_ON_SUSPEND: u32 = 3;
pub const RPC_C_OPT_PRIVATE_DO_NOT_DISTURB: u32 = 2;
pub const RPC_C_OPT_PRIVATE_SUPPRESS_WAKE: u32 = 1;
pub const RPC_C_OPT_RESOURCE_TYPE_UUID: u32 = 8;
pub const RPC_C_OPT_SECURITY_CALLBACK: u32 = 10;
pub const RPC_C_OPT_SESSION_ID: u32 = 6;
pub const RPC_C_OPT_TRANS_SEND_BUFFER_SIZE: u32 = 5;
pub const RPC_C_OPT_TRUST_PEER: u32 = 14;
pub const RPC_C_OPT_UNIQUE_BINDING: u32 = 11;
pub const RPC_C_PARM_BUFFER_LENGTH: u32 = 2;
pub const RPC_C_PARM_MAX_PACKET_LENGTH: u32 = 1;
pub const RPC_C_PROFILE_ALL_ELT: u32 = 1;
pub const RPC_C_PROFILE_ALL_ELTS: u32 = 1;
pub const RPC_C_PROFILE_DEFAULT_ELT: u32 = 0;
pub const RPC_C_PROFILE_MATCH_BY_BOTH: u32 = 4;
pub const RPC_C_PROFILE_MATCH_BY_IF: u32 = 2;
pub const RPC_C_PROFILE_MATCH_BY_MBR: u32 = 3;
pub const RPC_C_PROTECT_LEVEL_CALL: u32 = 3;
pub const RPC_C_PROTECT_LEVEL_CONNECT: u32 = 2;
pub const RPC_C_PROTECT_LEVEL_DEFAULT: u32 = 0;
pub const RPC_C_PROTECT_LEVEL_NONE: u32 = 1;
pub const RPC_C_PROTECT_LEVEL_PKT: u32 = 4;
pub const RPC_C_PROTECT_LEVEL_PKT_INTEGRITY: u32 = 5;
pub const RPC_C_PROTECT_LEVEL_PKT_PRIVACY: u32 = 6;
pub const RPC_C_PROTSEQ_MAX_REQS_DEFAULT: u32 = 10;
pub const RPC_C_QOS_CAPABILITIES_ANY_AUTHORITY: u32 = 4;
pub const RPC_C_QOS_CAPABILITIES_DEFAULT: u32 = 0;
pub const RPC_C_QOS_CAPABILITIES_IGNORE_DELEGATE_FAILURE: u32 = 8;
pub const RPC_C_QOS_CAPABILITIES_LOCAL_MA_HINT: u32 = 16;
pub const RPC_C_QOS_CAPABILITIES_MAKE_FULLSIC: u32 = 2;
pub const RPC_C_QOS_CAPABILITIES_MUTUAL_AUTH: u32 = 1;
pub const RPC_C_QOS_CAPABILITIES_SCHANNEL_FULL_AUTH_IDENTITY: u32 = 32;
pub const RPC_C_QOS_IDENTITY_DYNAMIC: u32 = 1;
pub const RPC_C_QOS_IDENTITY_STATIC: u32 = 0;
pub const RPC_C_RPCHTTP_USE_LOAD_BALANCE: u32 = 8;
pub const RPC_C_SECURITY_QOS_VERSION: u32 = 1;
pub const RPC_C_SECURITY_QOS_VERSION_1: u32 = 1;
pub const RPC_C_SECURITY_QOS_VERSION_2: u32 = 2;
pub const RPC_C_SECURITY_QOS_VERSION_3: u32 = 3;
pub const RPC_C_SECURITY_QOS_VERSION_4: u32 = 4;
pub const RPC_C_SECURITY_QOS_VERSION_5: u32 = 5;
pub const RPC_C_STATS_CALLS_IN: u32 = 0;
pub const RPC_C_STATS_CALLS_OUT: u32 = 1;
pub const RPC_C_STATS_PKTS_IN: u32 = 2;
pub const RPC_C_STATS_PKTS_OUT: u32 = 3;
pub const RPC_C_TRY_ENFORCE_MAX_CALLS: u32 = 16;
pub const RPC_C_USE_INTERNET_PORT: u32 = 1;
pub const RPC_C_USE_INTRANET_PORT: u32 = 2;
pub const RPC_C_VERS_ALL: u32 = 1;
pub const RPC_C_VERS_COMPATIBLE: u32 = 2;
pub const RPC_C_VERS_EXACT: u32 = 3;
pub const RPC_C_VERS_MAJOR_ONLY: u32 = 4;
pub const RPC_C_VERS_UPTO: u32 = 5;
pub type RPC_DISPATCH_FUNCTION = Option<unsafe extern "system" fn(message: *mut RPC_MESSAGE)>;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RPC_DISPATCH_TABLE {
    pub DispatchTableCount: u32,
    pub DispatchTable: *mut RPC_DISPATCH_FUNCTION,
    pub Reserved: isize,
}
impl Default for RPC_DISPATCH_TABLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
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
    pub AnsiString: windows_core::PSTR,
    pub UnicodeString: windows_core::PWSTR,
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
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RPC_ENDPOINT_TEMPLATEA {
    pub Version: u32,
    pub ProtSeq: RPC_CSTR,
    pub Endpoint: RPC_CSTR,
    pub SecurityDescriptor: *mut core::ffi::c_void,
    pub Backlog: u32,
}
impl Default for RPC_ENDPOINT_TEMPLATEA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RPC_ENDPOINT_TEMPLATEW {
    pub Version: u32,
    pub ProtSeq: RPC_WSTR,
    pub Endpoint: RPC_WSTR,
    pub SecurityDescriptor: *mut core::ffi::c_void,
    pub Backlog: u32,
}
impl Default for RPC_ENDPOINT_TEMPLATEW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type RPC_EP_INQ_HANDLE = *mut I_RPC_HANDLE;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[cfg(all(feature = "minwinbase", feature = "minwindef"))]
#[derive(Clone, Copy)]
pub struct RPC_EXTENDED_ERROR_INFO {
    pub Version: u32,
    pub ComputerName: windows_core::PWSTR,
    pub ProcessID: u32,
    pub u: RPC_EXTENDED_ERROR_INFO_0,
    pub GeneratingComponent: u32,
    pub Status: u32,
    pub DetectionLocation: u16,
    pub Flags: u16,
    pub NumberOfParameters: i32,
    pub Parameters: [RPC_EE_INFO_PARAM; 4],
}
#[cfg(all(feature = "minwinbase", feature = "minwindef"))]
impl Default for RPC_EXTENDED_ERROR_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "minwinbase", feature = "minwindef"))]
#[derive(Clone, Copy)]
pub union RPC_EXTENDED_ERROR_INFO_0 {
    pub SystemTime: super::minwinbase::SYSTEMTIME,
    pub FileTime: super::minwindef::FILETIME,
}
#[cfg(all(feature = "minwinbase", feature = "minwindef"))]
impl Default for RPC_EXTENDED_ERROR_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const RPC_FLAGS_VALID_BIT: u32 = 32768;
pub type RPC_FORWARD_FUNCTION = Option<unsafe extern "system" fn(interfaceid: *mut windows_core::GUID, interfaceversion: *mut RPC_VERSION, objectid: *mut windows_core::GUID, rpcpro: *mut u8, ppdestendpoint: *mut *mut core::ffi::c_void) -> RPC_STATUS>;
pub const RPC_FW_IF_FLAG_DCOM: u32 = 1;
pub type RPC_HTTP_PROXY_FREE_STRING = Option<unsafe extern "system" fn(string: *const u16)>;
pub type RPC_HTTP_REDIRECTOR_STAGE = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RPC_HTTP_TRANSPORT_CREDENTIALS_A {
    pub TransportCredentials: *mut SEC_WINNT_AUTH_IDENTITY_A,
    pub Flags: u32,
    pub AuthenticationTarget: u32,
    pub NumberOfAuthnSchemes: u32,
    pub AuthnSchemes: *mut u32,
    pub ServerCertificateSubject: *mut u8,
}
impl Default for RPC_HTTP_TRANSPORT_CREDENTIALS_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RPC_HTTP_TRANSPORT_CREDENTIALS_V2_A {
    pub TransportCredentials: *mut SEC_WINNT_AUTH_IDENTITY_A,
    pub Flags: u32,
    pub AuthenticationTarget: u32,
    pub NumberOfAuthnSchemes: u32,
    pub AuthnSchemes: *mut u32,
    pub ServerCertificateSubject: *mut u8,
    pub ProxyCredentials: *mut SEC_WINNT_AUTH_IDENTITY_A,
    pub NumberOfProxyAuthnSchemes: u32,
    pub ProxyAuthnSchemes: *mut u32,
}
impl Default for RPC_HTTP_TRANSPORT_CREDENTIALS_V2_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RPC_HTTP_TRANSPORT_CREDENTIALS_V2_W {
    pub TransportCredentials: *mut SEC_WINNT_AUTH_IDENTITY_W,
    pub Flags: u32,
    pub AuthenticationTarget: u32,
    pub NumberOfAuthnSchemes: u32,
    pub AuthnSchemes: *mut u32,
    pub ServerCertificateSubject: *mut u16,
    pub ProxyCredentials: *mut SEC_WINNT_AUTH_IDENTITY_W,
    pub NumberOfProxyAuthnSchemes: u32,
    pub ProxyAuthnSchemes: *mut u32,
}
impl Default for RPC_HTTP_TRANSPORT_CREDENTIALS_V2_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RPC_HTTP_TRANSPORT_CREDENTIALS_V3_A {
    pub TransportCredentials: RPC_AUTH_IDENTITY_HANDLE,
    pub Flags: u32,
    pub AuthenticationTarget: u32,
    pub NumberOfAuthnSchemes: u32,
    pub AuthnSchemes: *mut u32,
    pub ServerCertificateSubject: *mut u8,
    pub ProxyCredentials: RPC_AUTH_IDENTITY_HANDLE,
    pub NumberOfProxyAuthnSchemes: u32,
    pub ProxyAuthnSchemes: *mut u32,
}
impl Default for RPC_HTTP_TRANSPORT_CREDENTIALS_V3_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RPC_HTTP_TRANSPORT_CREDENTIALS_V3_W {
    pub TransportCredentials: RPC_AUTH_IDENTITY_HANDLE,
    pub Flags: u32,
    pub AuthenticationTarget: u32,
    pub NumberOfAuthnSchemes: u32,
    pub AuthnSchemes: *mut u32,
    pub ServerCertificateSubject: *mut u16,
    pub ProxyCredentials: RPC_AUTH_IDENTITY_HANDLE,
    pub NumberOfProxyAuthnSchemes: u32,
    pub ProxyAuthnSchemes: *mut u32,
}
impl Default for RPC_HTTP_TRANSPORT_CREDENTIALS_V3_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RPC_HTTP_TRANSPORT_CREDENTIALS_W {
    pub TransportCredentials: *mut SEC_WINNT_AUTH_IDENTITY_W,
    pub Flags: u32,
    pub AuthenticationTarget: u32,
    pub NumberOfAuthnSchemes: u32,
    pub AuthnSchemes: *mut u32,
    pub ServerCertificateSubject: *mut u16,
}
impl Default for RPC_HTTP_TRANSPORT_CREDENTIALS_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const RPC_IF_ALLOW_CALLBACKS_WITH_NO_AUTH: u32 = 16;
pub const RPC_IF_ALLOW_LOCAL_ONLY: u32 = 32;
pub const RPC_IF_ALLOW_SECURE_ONLY: u32 = 8;
pub const RPC_IF_ALLOW_UNKNOWN_AUTHORITY: u32 = 4;
pub const RPC_IF_ASYNC_CALLBACK: u32 = 256;
pub const RPC_IF_AUTOLISTEN: u32 = 1;
pub type RPC_IF_CALLBACK_FN = Option<unsafe extern "system" fn(interfaceuuid: RPC_IF_HANDLE, context: *const core::ffi::c_void) -> RPC_STATUS>;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RPC_IF_HANDLE(pub *mut core::ffi::c_void);
impl RPC_IF_HANDLE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for RPC_IF_HANDLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct RPC_IF_ID {
    pub Uuid: windows_core::GUID,
    pub VersMajor: u16,
    pub VersMinor: u16,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RPC_IF_ID_VECTOR {
    pub Count: u32,
    pub IfId: [*mut RPC_IF_ID; 1],
}
impl Default for RPC_IF_ID_VECTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const RPC_IF_OLE: u32 = 2;
pub const RPC_IF_SEC_CACHE_PER_PROC: u32 = 128;
pub const RPC_IF_SEC_NO_CACHE: u32 = 64;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RPC_IMPORT_CONTEXT_P {
    pub LookupContext: RPC_NS_HANDLE,
    pub ProposedHandle: RPC_BINDING_HANDLE,
    pub Bindings: *mut RPC_BINDING_VECTOR,
}
impl Default for RPC_IMPORT_CONTEXT_P {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RPC_INTERFACE_GROUP(pub *mut core::ffi::c_void);
impl RPC_INTERFACE_GROUP {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for RPC_INTERFACE_GROUP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type RPC_INTERFACE_GROUP_IDLE_CALLBACK_FN = Option<unsafe extern "system" fn(ifgroup: RPC_INTERFACE_GROUP, idlecallbackcontext: *const core::ffi::c_void, isgroupidle: u32)>;
pub const RPC_INTERFACE_HAS_PIPES: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct RPC_INTERFACE_TEMPLATEA {
    pub Version: u32,
    pub IfSpec: RPC_IF_HANDLE,
    pub MgrTypeUuid: *mut windows_core::GUID,
    pub MgrEpv: *mut core::ffi::c_void,
    pub Flags: u32,
    pub MaxCalls: u32,
    pub MaxRpcSize: u32,
    pub IfCallback: RPC_IF_CALLBACK_FN,
    pub UuidVector: *mut UUID_VECTOR,
    pub Annotation: RPC_CSTR,
    pub SecurityDescriptor: *mut core::ffi::c_void,
}
impl Default for RPC_INTERFACE_TEMPLATEA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct RPC_INTERFACE_TEMPLATEW {
    pub Version: u32,
    pub IfSpec: RPC_IF_HANDLE,
    pub MgrTypeUuid: *mut windows_core::GUID,
    pub MgrEpv: *mut core::ffi::c_void,
    pub Flags: u32,
    pub MaxCalls: u32,
    pub MaxRpcSize: u32,
    pub IfCallback: RPC_IF_CALLBACK_FN,
    pub UuidVector: *mut UUID_VECTOR,
    pub Annotation: RPC_WSTR,
    pub SecurityDescriptor: *mut core::ffi::c_void,
}
impl Default for RPC_INTERFACE_TEMPLATEW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct RPC_LENGTH(pub u32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RPC_MESSAGE {
    pub Handle: RPC_BINDING_HANDLE,
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
impl Default for RPC_MESSAGE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type RPC_MGMT_AUTHORIZATION_FN = Option<unsafe extern "system" fn(clientbinding: RPC_BINDING_HANDLE, requestedmgmtoperation: u32, status: *mut RPC_STATUS) -> i32>;
pub const RPC_NCA_FLAGS_BROADCAST: u32 = 2;
pub const RPC_NCA_FLAGS_DEFAULT: u32 = 0;
pub const RPC_NCA_FLAGS_IDEMPOTENT: u32 = 1;
pub const RPC_NCA_FLAGS_MAYBE: u32 = 4;
pub type RPC_NEW_HTTP_PROXY_CHANNEL = Option<unsafe extern "system" fn(redirectorstage: RPC_HTTP_REDIRECTOR_STAGE, servername: *const u16, serverport: *const u16, remoteuser: *const u16, authtype: *const u16, resourceuuid: *mut core::ffi::c_void, sessionid: *mut core::ffi::c_void, interface: *const core::ffi::c_void, reserved: *const core::ffi::c_void, flags: u32, newservername: *mut RPC_WSTR, newserverport: *mut RPC_WSTR) -> RPC_STATUS>;
pub type RPC_NOTIFICATIONS = i32;
pub type RPC_NOTIFICATION_TYPES = i32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RPC_NS_HANDLE(pub *mut core::ffi::c_void);
impl RPC_NS_HANDLE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for RPC_NS_HANDLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type RPC_OBJECT_INQ_FN = Option<unsafe extern "system" fn(objectuuid: *const windows_core::GUID, typeuuid: *mut windows_core::GUID, status: *mut RPC_STATUS)>;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct RPC_POLICY {
    pub Length: u32,
    pub EndpointFlags: u32,
    pub NICFlags: u32,
}
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
pub const RPC_PROTSEQ_HTTP: u32 = 4;
pub const RPC_PROTSEQ_LRPC: u32 = 3;
pub const RPC_PROTSEQ_NMP: u32 = 2;
pub const RPC_PROTSEQ_TCP: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RPC_PROTSEQ_VECTORA {
    pub Count: u32,
    pub Protseq: [*mut u8; 1],
}
impl Default for RPC_PROTSEQ_VECTORA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RPC_PROTSEQ_VECTORW {
    pub Count: u32,
    pub Protseq: [*mut u16; 1],
}
impl Default for RPC_PROTSEQ_VECTORW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const RPC_PROXY_CONNECTION_TYPE_IN_PROXY: u32 = 0;
pub const RPC_PROXY_CONNECTION_TYPE_OUT_PROXY: u32 = 1;
pub const RPC_P_ADDR_FORMAT_TCP_IPV4: u32 = 1;
pub const RPC_P_ADDR_FORMAT_TCP_IPV6: u32 = 2;
pub const RPC_QUERY_CALL_LOCAL_ADDRESS: u32 = 8;
pub const RPC_QUERY_CLIENT_ID: u32 = 128;
pub const RPC_QUERY_CLIENT_PID: u32 = 16;
pub const RPC_QUERY_CLIENT_PRINCIPAL_NAME: u32 = 4;
pub const RPC_QUERY_IS_CLIENT_LOCAL: u32 = 32;
pub const RPC_QUERY_NO_AUTH_REQUIRED: u32 = 64;
pub const RPC_QUERY_SERVER_PRINCIPAL_NAME: u32 = 2;
pub type RPC_SECURITY_CALLBACK_FN = Option<unsafe extern "system" fn(context: *const core::ffi::c_void)>;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct RPC_SECURITY_QOS {
    pub Version: u32,
    pub Capabilities: u32,
    pub IdentityTracking: u32,
    pub ImpersonationType: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct RPC_SECURITY_QOS_V2_A {
    pub Version: u32,
    pub Capabilities: u32,
    pub IdentityTracking: u32,
    pub ImpersonationType: u32,
    pub AdditionalSecurityInfoType: u32,
    pub u: RPC_SECURITY_QOS_V2_A_0,
}
impl Default for RPC_SECURITY_QOS_V2_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union RPC_SECURITY_QOS_V2_A_0 {
    pub HttpCredentials: *mut RPC_HTTP_TRANSPORT_CREDENTIALS_A,
}
impl Default for RPC_SECURITY_QOS_V2_A_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct RPC_SECURITY_QOS_V2_W {
    pub Version: u32,
    pub Capabilities: u32,
    pub IdentityTracking: u32,
    pub ImpersonationType: u32,
    pub AdditionalSecurityInfoType: u32,
    pub u: RPC_SECURITY_QOS_V2_W_0,
}
impl Default for RPC_SECURITY_QOS_V2_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union RPC_SECURITY_QOS_V2_W_0 {
    pub HttpCredentials: *mut RPC_HTTP_TRANSPORT_CREDENTIALS_W,
}
impl Default for RPC_SECURITY_QOS_V2_W_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct RPC_SECURITY_QOS_V3_A {
    pub Version: u32,
    pub Capabilities: u32,
    pub IdentityTracking: u32,
    pub ImpersonationType: u32,
    pub AdditionalSecurityInfoType: u32,
    pub u: RPC_SECURITY_QOS_V3_A_0,
    pub Sid: *mut core::ffi::c_void,
}
impl Default for RPC_SECURITY_QOS_V3_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union RPC_SECURITY_QOS_V3_A_0 {
    pub HttpCredentials: *mut RPC_HTTP_TRANSPORT_CREDENTIALS_A,
}
impl Default for RPC_SECURITY_QOS_V3_A_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct RPC_SECURITY_QOS_V3_W {
    pub Version: u32,
    pub Capabilities: u32,
    pub IdentityTracking: u32,
    pub ImpersonationType: u32,
    pub AdditionalSecurityInfoType: u32,
    pub u: RPC_SECURITY_QOS_V3_W_0,
    pub Sid: *mut core::ffi::c_void,
}
impl Default for RPC_SECURITY_QOS_V3_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union RPC_SECURITY_QOS_V3_W_0 {
    pub HttpCredentials: *mut RPC_HTTP_TRANSPORT_CREDENTIALS_W,
}
impl Default for RPC_SECURITY_QOS_V3_W_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct RPC_SECURITY_QOS_V4_A {
    pub Version: u32,
    pub Capabilities: u32,
    pub IdentityTracking: u32,
    pub ImpersonationType: u32,
    pub AdditionalSecurityInfoType: u32,
    pub u: RPC_SECURITY_QOS_V4_A_0,
    pub Sid: *mut core::ffi::c_void,
    pub EffectiveOnly: u32,
}
impl Default for RPC_SECURITY_QOS_V4_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union RPC_SECURITY_QOS_V4_A_0 {
    pub HttpCredentials: *mut RPC_HTTP_TRANSPORT_CREDENTIALS_A,
}
impl Default for RPC_SECURITY_QOS_V4_A_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct RPC_SECURITY_QOS_V4_W {
    pub Version: u32,
    pub Capabilities: u32,
    pub IdentityTracking: u32,
    pub ImpersonationType: u32,
    pub AdditionalSecurityInfoType: u32,
    pub u: RPC_SECURITY_QOS_V4_W_0,
    pub Sid: *mut core::ffi::c_void,
    pub EffectiveOnly: u32,
}
impl Default for RPC_SECURITY_QOS_V4_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union RPC_SECURITY_QOS_V4_W_0 {
    pub HttpCredentials: *mut RPC_HTTP_TRANSPORT_CREDENTIALS_W,
}
impl Default for RPC_SECURITY_QOS_V4_W_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct RPC_SECURITY_QOS_V5_A {
    pub Version: u32,
    pub Capabilities: u32,
    pub IdentityTracking: u32,
    pub ImpersonationType: u32,
    pub AdditionalSecurityInfoType: u32,
    pub u: RPC_SECURITY_QOS_V5_A_0,
    pub Sid: *mut core::ffi::c_void,
    pub EffectiveOnly: u32,
    pub ServerSecurityDescriptor: *mut core::ffi::c_void,
}
impl Default for RPC_SECURITY_QOS_V5_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union RPC_SECURITY_QOS_V5_A_0 {
    pub HttpCredentials: *mut RPC_HTTP_TRANSPORT_CREDENTIALS_A,
}
impl Default for RPC_SECURITY_QOS_V5_A_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct RPC_SECURITY_QOS_V5_W {
    pub Version: u32,
    pub Capabilities: u32,
    pub IdentityTracking: u32,
    pub ImpersonationType: u32,
    pub AdditionalSecurityInfoType: u32,
    pub u: RPC_SECURITY_QOS_V5_W_0,
    pub Sid: *mut core::ffi::c_void,
    pub EffectiveOnly: u32,
    pub ServerSecurityDescriptor: *mut core::ffi::c_void,
}
impl Default for RPC_SECURITY_QOS_V5_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union RPC_SECURITY_QOS_V5_W_0 {
    pub HttpCredentials: *mut RPC_HTTP_TRANSPORT_CREDENTIALS_W,
}
impl Default for RPC_SECURITY_QOS_V5_W_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct RPC_SEC_CONTEXT_KEY_INFO {
    pub EncryptAlgorithm: u32,
    pub KeySize: u32,
    pub SignatureAlgorithm: u32,
}
#[repr(C)]
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
impl Default for RPC_SERVER_INTERFACE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type RPC_SETFILTER_FUNC = Option<unsafe extern "C" fn(pfnfilter: RPCLT_PDU_FILTER_FUNC)>;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RPC_SS_THREAD_HANDLE(pub *mut core::ffi::c_void);
impl RPC_SS_THREAD_HANDLE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for RPC_SS_THREAD_HANDLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RPC_STATS_VECTOR {
    pub Count: u32,
    pub Stats: [u32; 1],
}
impl Default for RPC_STATS_VECTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct RPC_STATUS(pub i32);
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
pub const RPC_S_ACCESS_DENIED: u32 = 5;
pub const RPC_S_ASYNC_CALL_PENDING: u32 = 997;
pub const RPC_S_BUFFER_TOO_SMALL: u32 = 122;
pub const RPC_S_INVALID_ARG: u32 = 87;
pub const RPC_S_INVALID_LEVEL: u32 = 87;
pub const RPC_S_INVALID_SECURITY_DESC: u32 = 1338;
pub const RPC_S_NOT_ENOUGH_QUOTA: u32 = 1816;
pub const RPC_S_OK: u32 = 0;
pub const RPC_S_OUT_OF_MEMORY: u32 = 14;
pub const RPC_S_OUT_OF_THREADS: u32 = 164;
pub const RPC_S_RUNTIME_UNINITIALIZED: u32 = 1;
pub const RPC_S_SERVER_OUT_OF_MEMORY: u32 = 1130;
pub const RPC_S_TIMEOUT: u32 = 1460;
pub const RPC_S_UNKNOWN_PRINCIPAL: u32 = 1332;
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
pub type RPC_WSTR = *mut u16;
pub const RPC_X_ENUM_VALUE_TOO_LARGE: u32 = 1781;
pub const RPC_X_INVALID_BOUND: u32 = 1734;
pub const RPC_X_INVALID_BUFFER: u32 = 1784;
pub const RPC_X_INVALID_PIPE_OPERATION: u32 = 1831;
pub const RPC_X_INVALID_TAG: u32 = 1733;
pub const RPC_X_NO_MEMORY: u32 = 14;
pub const RPC_X_PIPE_APP_MEMORY: u32 = 14;
pub const RPC_X_SS_CONTEXT_MISMATCH: u32 = 6;
pub const RpcAttemptedLbsDecisions: RpcPerfCounters = 8;
pub const RpcAttemptedLbsMessages: RpcPerfCounters = 10;
pub const RpcBackEndConnectionAttempts: RpcPerfCounters = 2;
pub const RpcBackEndConnectionFailed: RpcPerfCounters = 3;
pub type RpcCallClientLocality = i32;
pub const RpcCallComplete: RPC_ASYNC_EVENT = 0;
pub type RpcCallType = i32;
pub const RpcClientCancel: RPC_ASYNC_EVENT = 4;
pub const RpcClientDisconnect: RPC_ASYNC_EVENT = 3;
pub const RpcCurrentUniqueUser: RpcPerfCounters = 1;
pub const RpcFailedLbsDecisions: RpcPerfCounters = 9;
pub const RpcFailedLbsMessages: RpcPerfCounters = 11;
pub const RpcIncomingBandwidth: RpcPerfCounters = 6;
pub const RpcIncomingConnections: RpcPerfCounters = 5;
pub const RpcLastCounter: RpcPerfCounters = 12;
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
pub const RpcOutgoingBandwidth: RpcPerfCounters = 7;
pub type RpcPerfCounters = i32;
pub const RpcReceiveComplete: RPC_ASYNC_EVENT = 2;
pub const RpcRequestsPerSecond: RpcPerfCounters = 4;
pub const RpcSendComplete: RPC_ASYNC_EVENT = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SCONTEXT_QUEUE {
    pub NumberOfObjects: u32,
    pub ArrayOfObjects: *mut NDR_SCONTEXT,
}
impl Default for SCONTEXT_QUEUE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SEC_WINNT_AUTH_IDENTITY_A {
    pub User: *mut u8,
    pub UserLength: u32,
    pub Domain: *mut u8,
    pub DomainLength: u32,
    pub Password: *mut u8,
    pub PasswordLength: u32,
    pub Flags: u32,
}
impl Default for SEC_WINNT_AUTH_IDENTITY_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SEC_WINNT_AUTH_IDENTITY_ANSI: u32 = 1;
pub const SEC_WINNT_AUTH_IDENTITY_UNICODE: u32 = 2;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SEC_WINNT_AUTH_IDENTITY_W {
    pub User: *mut u16,
    pub UserLength: u32,
    pub Domain: *mut u16,
    pub DomainLength: u32,
    pub Password: *mut u16,
    pub PasswordLength: u32,
    pub Flags: u32,
}
impl Default for SEC_WINNT_AUTH_IDENTITY_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type SERVER_ROUTINE = Option<unsafe extern "system" fn() -> i32>;
pub const STUB_CALL_SERVER: STUB_PHASE = 1;
pub const STUB_CALL_SERVER_NO_HRESULT: STUB_PHASE = 3;
pub const STUB_MARSHAL: STUB_PHASE = 2;
pub type STUB_PHASE = i32;
#[cfg(feature = "objidlbase")]
pub type STUB_THUNK = Option<unsafe extern "system" fn(param0: *mut MIDL_STUB_MESSAGE)>;
pub const STUB_UNMARSHAL: STUB_PHASE = 0;
pub const SYSTEM_HANDLE_COMPOSITION_OBJECT: system_handle_t = 9;
pub const SYSTEM_HANDLE_EVENT: system_handle_t = 2;
pub const SYSTEM_HANDLE_FILE: system_handle_t = 0;
pub const SYSTEM_HANDLE_INVALID: system_handle_t = 255;
pub const SYSTEM_HANDLE_JOB: system_handle_t = 11;
pub const SYSTEM_HANDLE_MAX: system_handle_t = 12;
pub const SYSTEM_HANDLE_MUTEX: system_handle_t = 3;
pub const SYSTEM_HANDLE_PIPE: system_handle_t = 12;
pub const SYSTEM_HANDLE_PROCESS: system_handle_t = 4;
pub const SYSTEM_HANDLE_REG_KEY: system_handle_t = 7;
pub const SYSTEM_HANDLE_SECTION: system_handle_t = 6;
pub const SYSTEM_HANDLE_SEMAPHORE: system_handle_t = 1;
pub const SYSTEM_HANDLE_SOCKET: system_handle_t = 10;
pub const SYSTEM_HANDLE_THREAD: system_handle_t = 8;
pub const SYSTEM_HANDLE_TOKEN: system_handle_t = 5;
pub const TARGET_IS_NT100_OR_LATER: u32 = 1;
pub const TARGET_IS_NT1012_OR_LATER: u32 = 1;
pub const TARGET_IS_NT102_OR_LATER: u32 = 1;
pub const TARGET_IS_NT351_OR_WIN95_OR_LATER: u32 = 1;
pub const TARGET_IS_NT40_OR_LATER: u32 = 1;
pub const TARGET_IS_NT50_OR_LATER: u32 = 1;
pub const TARGET_IS_NT51_OR_LATER: u32 = 1;
pub const TARGET_IS_NT60_OR_LATER: u32 = 1;
pub const TARGET_IS_NT61_OR_LATER: u32 = 1;
pub const TARGET_IS_NT62_OR_LATER: u32 = 1;
pub const TARGET_IS_NT63_OR_LATER: u32 = 1;
pub const TRANSPORT_TYPE_CN: u32 = 1;
pub const TRANSPORT_TYPE_DG: u32 = 2;
pub const TRANSPORT_TYPE_LPC: u32 = 4;
pub const TRANSPORT_TYPE_WMSG: u32 = 8;
pub const USER_CALL_IS_ASYNC: u32 = 256;
pub const USER_CALL_NEW_CORRELATION_DESC: u32 = 512;
#[repr(C)]
#[cfg(feature = "objidlbase")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct USER_MARSHAL_CB {
    pub Flags: u32,
    pub pStubMsg: PMIDL_STUB_MESSAGE,
    pub pReserve: PFORMAT_STRING,
    pub Signature: u32,
    pub CBType: USER_MARSHAL_CB_TYPE,
    pub pFormat: PFORMAT_STRING,
    pub pTypeFormat: PFORMAT_STRING,
}
pub const USER_MARSHAL_CB_BUFFER_SIZE: USER_MARSHAL_CB_TYPE = 0;
pub const USER_MARSHAL_CB_FREE: USER_MARSHAL_CB_TYPE = 3;
pub const USER_MARSHAL_CB_MARSHALL: USER_MARSHAL_CB_TYPE = 1;
pub const USER_MARSHAL_CB_SIGNATURE: u32 = 1431523907;
pub type USER_MARSHAL_CB_TYPE = i32;
pub const USER_MARSHAL_CB_UNMARSHALL: USER_MARSHAL_CB_TYPE = 2;
pub const USER_MARSHAL_FC_BYTE: u32 = 1;
pub const USER_MARSHAL_FC_CHAR: u32 = 2;
pub const USER_MARSHAL_FC_DOUBLE: u32 = 12;
pub const USER_MARSHAL_FC_FLOAT: u32 = 10;
pub const USER_MARSHAL_FC_HYPER: u32 = 11;
pub const USER_MARSHAL_FC_LONG: u32 = 8;
pub const USER_MARSHAL_FC_SHORT: u32 = 6;
pub const USER_MARSHAL_FC_SMALL: u32 = 3;
pub const USER_MARSHAL_FC_ULONG: u32 = 9;
pub const USER_MARSHAL_FC_USHORT: u32 = 7;
pub const USER_MARSHAL_FC_USMALL: u32 = 4;
pub const USER_MARSHAL_FC_WCHAR: u32 = 5;
pub type USER_MARSHAL_FREEING_ROUTINE = Option<unsafe extern "system" fn(param0: *mut u32, param1: *mut core::ffi::c_void)>;
pub type USER_MARSHAL_MARSHALLING_ROUTINE = Option<unsafe extern "system" fn(param0: *mut u32, param1: *mut u8, param2: *mut core::ffi::c_void) -> *mut u8>;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct USER_MARSHAL_ROUTINE_QUADRUPLE {
    pub pfnBufferSize: USER_MARSHAL_SIZING_ROUTINE,
    pub pfnMarshall: USER_MARSHAL_MARSHALLING_ROUTINE,
    pub pfnUnmarshall: USER_MARSHAL_UNMARSHALLING_ROUTINE,
    pub pfnFree: USER_MARSHAL_FREEING_ROUTINE,
}
pub type USER_MARSHAL_SIZING_ROUTINE = Option<unsafe extern "system" fn(param0: *mut u32, param1: u32, param2: *mut core::ffi::c_void) -> u32>;
pub type USER_MARSHAL_UNMARSHALLING_ROUTINE = Option<unsafe extern "system" fn(param0: *mut u32, param1: *mut u8, param2: *mut core::ffi::c_void) -> *mut u8>;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct UUID_VECTOR {
    pub Count: u32,
    pub Uuid: [*mut windows_core::GUID; 1],
}
impl Default for UUID_VECTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const XLAT_CLIENT: XLAT_SIDE = 2;
pub const XLAT_SERVER: XLAT_SIDE = 1;
pub type XLAT_SIDE = i32;
#[cfg(feature = "objidlbase")]
pub type XMIT_HELPER_ROUTINE = Option<unsafe extern "system" fn(param0: *mut MIDL_STUB_MESSAGE)>;
#[repr(C)]
#[cfg(feature = "objidlbase")]
#[derive(Clone, Copy, Debug, Default)]
pub struct XMIT_ROUTINE_QUINTUPLE {
    pub pfnTranslateToXmit: XMIT_HELPER_ROUTINE,
    pub pfnTranslateFromXmit: XMIT_HELPER_ROUTINE,
    pub pfnFreeXmit: XMIT_HELPER_ROUTINE,
    pub pfnFreeInst: XMIT_HELPER_ROUTINE,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct _NDR_ASYNC_MESSAGE(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct _NDR_CORRELATION_INFO(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct _NDR_PROC_CONTEXT(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct _NDR_SCONTEXT {
    pub pad: [*mut core::ffi::c_void; 2],
    pub userContext: *mut core::ffi::c_void,
}
impl Default for _NDR_SCONTEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct boolean(pub u8);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct byte(pub u8);
pub const cbNDRContext: u32 = 20;
pub type cs_byte = byte;
pub const eeptAnsiString: ExtendedErrorParamTypes = 1;
pub const eeptBinary: ExtendedErrorParamTypes = 7;
pub const eeptLongVal: ExtendedErrorParamTypes = 3;
pub const eeptNone: ExtendedErrorParamTypes = 6;
pub const eeptPointerVal: ExtendedErrorParamTypes = 5;
pub const eeptShortVal: ExtendedErrorParamTypes = 4;
pub const eeptUnicodeString: ExtendedErrorParamTypes = 2;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct error_status_t(pub u32);
pub type handle_t = RPC_BINDING_HANDLE;
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
pub type system_handle_t = i32;
