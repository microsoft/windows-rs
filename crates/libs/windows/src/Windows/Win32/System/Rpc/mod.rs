#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn DceErrorInqTextA(rpcstatus: RPC_STATUS, errortext: &mut [u8; 256]) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn DceErrorInqTextA ( rpcstatus : RPC_STATUS , errortext : *mut u8 ) -> RPC_STATUS );
    DceErrorInqTextA(rpcstatus, ::core::mem::transmute(errortext.as_ptr()))
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn DceErrorInqTextW(rpcstatus: RPC_STATUS, errortext: &mut [u16; 256]) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn DceErrorInqTextW ( rpcstatus : RPC_STATUS , errortext : *mut u16 ) -> RPC_STATUS );
    DceErrorInqTextW(rpcstatus, ::core::mem::transmute(errortext.as_ptr()))
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn IUnknown_AddRef_Proxy<P0>(this: P0) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
{
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn IUnknown_AddRef_Proxy ( this : * mut::core::ffi::c_void ) -> u32 );
    IUnknown_AddRef_Proxy(this.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn IUnknown_QueryInterface_Proxy<P0>(this: P0, riid: *const ::windows::core::GUID, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
{
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn IUnknown_QueryInterface_Proxy ( this : * mut::core::ffi::c_void , riid : *const :: windows::core::GUID , ppvobject : *mut *mut ::core::ffi::c_void ) -> :: windows::core::HRESULT );
    IUnknown_QueryInterface_Proxy(this.into_param().abi(), riid, ppvobject).ok()
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn IUnknown_Release_Proxy<P0>(this: P0) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
{
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn IUnknown_Release_Proxy ( this : * mut::core::ffi::c_void ) -> u32 );
    IUnknown_Release_Proxy(this.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn I_RpcAllocate(size: u32) -> *mut ::core::ffi::c_void {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn I_RpcAllocate ( size : u32 ) -> *mut ::core::ffi::c_void );
    I_RpcAllocate(size)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[inline]
pub unsafe fn I_RpcAsyncAbortCall(pasync: *const RPC_ASYNC_STATE, exceptioncode: u32) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn I_RpcAsyncAbortCall ( pasync : *const RPC_ASYNC_STATE , exceptioncode : u32 ) -> RPC_STATUS );
    I_RpcAsyncAbortCall(pasync, exceptioncode)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[inline]
pub unsafe fn I_RpcAsyncSetHandle(message: *const RPC_MESSAGE, pasync: *const RPC_ASYNC_STATE) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn I_RpcAsyncSetHandle ( message : *const RPC_MESSAGE , pasync : *const RPC_ASYNC_STATE ) -> RPC_STATUS );
    I_RpcAsyncSetHandle(message, pasync)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn I_RpcBindingCopy(sourcebinding: *mut ::core::ffi::c_void, destinationbinding: *mut *mut ::core::ffi::c_void) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn I_RpcBindingCopy ( sourcebinding : *mut ::core::ffi::c_void , destinationbinding : *mut *mut ::core::ffi::c_void ) -> RPC_STATUS );
    I_RpcBindingCopy(sourcebinding, destinationbinding)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn I_RpcBindingCreateNP(servername: *const u16, servicename: *const u16, networkoptions: *const u16, binding: *mut *mut ::core::ffi::c_void) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn I_RpcBindingCreateNP ( servername : *const u16 , servicename : *const u16 , networkoptions : *const u16 , binding : *mut *mut ::core::ffi::c_void ) -> RPC_STATUS );
    I_RpcBindingCreateNP(servername, servicename, networkoptions, binding)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn I_RpcBindingHandleToAsyncHandle(binding: *mut ::core::ffi::c_void, asynchandle: *mut *mut ::core::ffi::c_void) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn I_RpcBindingHandleToAsyncHandle ( binding : *mut ::core::ffi::c_void , asynchandle : *mut *mut ::core::ffi::c_void ) -> RPC_STATUS );
    I_RpcBindingHandleToAsyncHandle(binding, asynchandle)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn I_RpcBindingInqClientTokenAttributes(binding: *const ::core::ffi::c_void, tokenid: ::core::option::Option<*mut super::super::Foundation::LUID>, authenticationid: ::core::option::Option<*mut super::super::Foundation::LUID>, modifiedid: ::core::option::Option<*mut super::super::Foundation::LUID>) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn I_RpcBindingInqClientTokenAttributes ( binding : *const ::core::ffi::c_void , tokenid : *mut super::super::Foundation:: LUID , authenticationid : *mut super::super::Foundation:: LUID , modifiedid : *mut super::super::Foundation:: LUID ) -> RPC_STATUS );
    I_RpcBindingInqClientTokenAttributes(binding, ::core::mem::transmute(tokenid.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(authenticationid.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(modifiedid.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn I_RpcBindingInqDynamicEndpointA(binding: *const ::core::ffi::c_void, dynamicendpoint: *mut *mut u8) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn I_RpcBindingInqDynamicEndpointA ( binding : *const ::core::ffi::c_void , dynamicendpoint : *mut *mut u8 ) -> RPC_STATUS );
    I_RpcBindingInqDynamicEndpointA(binding, dynamicendpoint)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn I_RpcBindingInqDynamicEndpointW(binding: *const ::core::ffi::c_void, dynamicendpoint: *mut *mut u16) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn I_RpcBindingInqDynamicEndpointW ( binding : *const ::core::ffi::c_void , dynamicendpoint : *mut *mut u16 ) -> RPC_STATUS );
    I_RpcBindingInqDynamicEndpointW(binding, dynamicendpoint)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn I_RpcBindingInqLocalClientPID(binding: *mut ::core::ffi::c_void, pid: *mut u32) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn I_RpcBindingInqLocalClientPID ( binding : *mut ::core::ffi::c_void , pid : *mut u32 ) -> RPC_STATUS );
    I_RpcBindingInqLocalClientPID(binding, pid)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn I_RpcBindingInqMarshalledTargetInfo(binding: *const ::core::ffi::c_void, marshalledtargetinfosize: *mut u32, marshalledtargetinfo: *mut *mut u8) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn I_RpcBindingInqMarshalledTargetInfo ( binding : *const ::core::ffi::c_void , marshalledtargetinfosize : *mut u32 , marshalledtargetinfo : *mut *mut u8 ) -> RPC_STATUS );
    I_RpcBindingInqMarshalledTargetInfo(binding, marshalledtargetinfosize, marshalledtargetinfo)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn I_RpcBindingInqSecurityContext(binding: *mut ::core::ffi::c_void, securitycontexthandle: *mut *mut ::core::ffi::c_void) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn I_RpcBindingInqSecurityContext ( binding : *mut ::core::ffi::c_void , securitycontexthandle : *mut *mut ::core::ffi::c_void ) -> RPC_STATUS );
    I_RpcBindingInqSecurityContext(binding, securitycontexthandle)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn I_RpcBindingInqSecurityContextKeyInfo(binding: ::core::option::Option<*const ::core::ffi::c_void>, keyinfo: *mut ::core::ffi::c_void) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn I_RpcBindingInqSecurityContextKeyInfo ( binding : *const ::core::ffi::c_void , keyinfo : *mut ::core::ffi::c_void ) -> RPC_STATUS );
    I_RpcBindingInqSecurityContextKeyInfo(::core::mem::transmute(binding.unwrap_or(::std::ptr::null())), keyinfo)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn I_RpcBindingInqTransportType(binding: *mut ::core::ffi::c_void, r#type: *mut u32) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn I_RpcBindingInqTransportType ( binding : *mut ::core::ffi::c_void , r#type : *mut u32 ) -> RPC_STATUS );
    I_RpcBindingInqTransportType(binding, r#type)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn I_RpcBindingInqWireIdForSnego(binding: *const ::core::ffi::c_void, wireid: *mut u8) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn I_RpcBindingInqWireIdForSnego ( binding : *const ::core::ffi::c_void , wireid : *mut u8 ) -> RPC_STATUS );
    I_RpcBindingInqWireIdForSnego(binding, wireid)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn I_RpcBindingIsClientLocal(bindinghandle: *mut ::core::ffi::c_void, clientlocalflag: *mut u32) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn I_RpcBindingIsClientLocal ( bindinghandle : *mut ::core::ffi::c_void , clientlocalflag : *mut u32 ) -> RPC_STATUS );
    I_RpcBindingIsClientLocal(bindinghandle, clientlocalflag)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn I_RpcBindingIsServerLocal(binding: *const ::core::ffi::c_void, serverlocalflag: *mut u32) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn I_RpcBindingIsServerLocal ( binding : *const ::core::ffi::c_void , serverlocalflag : *mut u32 ) -> RPC_STATUS );
    I_RpcBindingIsServerLocal(binding, serverlocalflag)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn I_RpcBindingSetPrivateOption(hbinding: *const ::core::ffi::c_void, option: u32, optionvalue: usize) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn I_RpcBindingSetPrivateOption ( hbinding : *const ::core::ffi::c_void , option : u32 , optionvalue : usize ) -> RPC_STATUS );
    I_RpcBindingSetPrivateOption(hbinding, option, optionvalue)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn I_RpcBindingToStaticStringBindingW(binding: *mut ::core::ffi::c_void, stringbinding: *mut *mut u16) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn I_RpcBindingToStaticStringBindingW ( binding : *mut ::core::ffi::c_void , stringbinding : *mut *mut u16 ) -> RPC_STATUS );
    I_RpcBindingToStaticStringBindingW(binding, stringbinding)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn I_RpcClearMutex(mutex: *mut ::core::ffi::c_void) {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn I_RpcClearMutex ( mutex : *mut ::core::ffi::c_void ) -> ( ) );
    I_RpcClearMutex(mutex)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn I_RpcDeleteMutex(mutex: *mut ::core::ffi::c_void) {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn I_RpcDeleteMutex ( mutex : *mut ::core::ffi::c_void ) -> ( ) );
    I_RpcDeleteMutex(mutex)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn I_RpcExceptionFilter(exceptioncode: u32) -> i32 {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn I_RpcExceptionFilter ( exceptioncode : u32 ) -> i32 );
    I_RpcExceptionFilter(exceptioncode)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn I_RpcFree(object: *mut ::core::ffi::c_void) {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn I_RpcFree ( object : *mut ::core::ffi::c_void ) -> ( ) );
    I_RpcFree(object)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn I_RpcFreeBuffer(message: *mut RPC_MESSAGE) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn I_RpcFreeBuffer ( message : *mut RPC_MESSAGE ) -> RPC_STATUS );
    I_RpcFreeBuffer(message)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn I_RpcFreePipeBuffer(message: *mut RPC_MESSAGE) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn I_RpcFreePipeBuffer ( message : *mut RPC_MESSAGE ) -> RPC_STATUS );
    I_RpcFreePipeBuffer(message)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn I_RpcGetBuffer(message: *mut RPC_MESSAGE) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn I_RpcGetBuffer ( message : *mut RPC_MESSAGE ) -> RPC_STATUS );
    I_RpcGetBuffer(message)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn I_RpcGetBufferWithObject(message: *mut RPC_MESSAGE, objectuuid: *mut ::windows::core::GUID) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn I_RpcGetBufferWithObject ( message : *mut RPC_MESSAGE , objectuuid : *mut :: windows::core::GUID ) -> RPC_STATUS );
    I_RpcGetBufferWithObject(message, objectuuid)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn I_RpcGetCurrentCallHandle() -> *mut ::core::ffi::c_void {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn I_RpcGetCurrentCallHandle ( ) -> *mut ::core::ffi::c_void );
    I_RpcGetCurrentCallHandle()
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn I_RpcGetDefaultSD(ppsecuritydescriptor: *mut *mut ::core::ffi::c_void) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn I_RpcGetDefaultSD ( ppsecuritydescriptor : *mut *mut ::core::ffi::c_void ) -> RPC_STATUS );
    I_RpcGetDefaultSD(ppsecuritydescriptor)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn I_RpcGetExtendedError() -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn I_RpcGetExtendedError ( ) -> RPC_STATUS );
    I_RpcGetExtendedError()
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn I_RpcIfInqTransferSyntaxes(rpcifhandle: *mut ::core::ffi::c_void, transfersyntaxes: *mut RPC_TRANSFER_SYNTAX, transfersyntaxsize: u32, transfersyntaxcount: *mut u32) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn I_RpcIfInqTransferSyntaxes ( rpcifhandle : *mut ::core::ffi::c_void , transfersyntaxes : *mut RPC_TRANSFER_SYNTAX , transfersyntaxsize : u32 , transfersyntaxcount : *mut u32 ) -> RPC_STATUS );
    I_RpcIfInqTransferSyntaxes(rpcifhandle, transfersyntaxes, transfersyntaxsize, transfersyntaxcount)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn I_RpcMapWin32Status(status: RPC_STATUS) -> i32 {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn I_RpcMapWin32Status ( status : RPC_STATUS ) -> i32 );
    I_RpcMapWin32Status(status)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn I_RpcMgmtEnableDedicatedThreadPool() -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn I_RpcMgmtEnableDedicatedThreadPool ( ) -> RPC_STATUS );
    I_RpcMgmtEnableDedicatedThreadPool()
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn I_RpcNegotiateTransferSyntax(message: *mut RPC_MESSAGE) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn I_RpcNegotiateTransferSyntax ( message : *mut RPC_MESSAGE ) -> RPC_STATUS );
    I_RpcNegotiateTransferSyntax(message)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn I_RpcNsBindingSetEntryNameA(binding: *const ::core::ffi::c_void, entrynamesyntax: u32, entryname: *const u8) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn I_RpcNsBindingSetEntryNameA ( binding : *const ::core::ffi::c_void , entrynamesyntax : u32 , entryname : *const u8 ) -> RPC_STATUS );
    I_RpcNsBindingSetEntryNameA(binding, entrynamesyntax, entryname)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn I_RpcNsBindingSetEntryNameW(binding: *const ::core::ffi::c_void, entrynamesyntax: u32, entryname: *const u16) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn I_RpcNsBindingSetEntryNameW ( binding : *const ::core::ffi::c_void , entrynamesyntax : u32 , entryname : *const u16 ) -> RPC_STATUS );
    I_RpcNsBindingSetEntryNameW(binding, entrynamesyntax, entryname)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn I_RpcNsGetBuffer(message: *mut RPC_MESSAGE) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcns4.dll""system" fn I_RpcNsGetBuffer ( message : *mut RPC_MESSAGE ) -> RPC_STATUS );
    I_RpcNsGetBuffer(message)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn I_RpcNsInterfaceExported(entrynamesyntax: u32, entryname: *const u16, rpcinterfaceinformation: *const RPC_SERVER_INTERFACE) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn I_RpcNsInterfaceExported ( entrynamesyntax : u32 , entryname : *const u16 , rpcinterfaceinformation : *const RPC_SERVER_INTERFACE ) -> RPC_STATUS );
    I_RpcNsInterfaceExported(entrynamesyntax, entryname, rpcinterfaceinformation)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn I_RpcNsInterfaceUnexported(entrynamesyntax: u32, entryname: *mut u16, rpcinterfaceinformation: *mut RPC_SERVER_INTERFACE) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn I_RpcNsInterfaceUnexported ( entrynamesyntax : u32 , entryname : *mut u16 , rpcinterfaceinformation : *mut RPC_SERVER_INTERFACE ) -> RPC_STATUS );
    I_RpcNsInterfaceUnexported(entrynamesyntax, entryname, rpcinterfaceinformation)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn I_RpcNsRaiseException(message: *mut RPC_MESSAGE, status: RPC_STATUS) {
    ::windows::imp::link ! ( "rpcns4.dll""system" fn I_RpcNsRaiseException ( message : *mut RPC_MESSAGE , status : RPC_STATUS ) -> ( ) );
    I_RpcNsRaiseException(message, status)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn I_RpcNsSendReceive(message: *mut RPC_MESSAGE, handle: *mut *mut ::core::ffi::c_void) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcns4.dll""system" fn I_RpcNsSendReceive ( message : *mut RPC_MESSAGE , handle : *mut *mut ::core::ffi::c_void ) -> RPC_STATUS );
    I_RpcNsSendReceive(message, handle)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn I_RpcOpenClientProcess(binding: ::core::option::Option<*const ::core::ffi::c_void>, desiredaccess: u32, clientprocess: *mut *mut ::core::ffi::c_void) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn I_RpcOpenClientProcess ( binding : *const ::core::ffi::c_void , desiredaccess : u32 , clientprocess : *mut *mut ::core::ffi::c_void ) -> RPC_STATUS );
    I_RpcOpenClientProcess(::core::mem::transmute(binding.unwrap_or(::std::ptr::null())), desiredaccess, clientprocess)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn I_RpcPauseExecution(milliseconds: u32) {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn I_RpcPauseExecution ( milliseconds : u32 ) -> ( ) );
    I_RpcPauseExecution(milliseconds)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn I_RpcReBindBuffer(message: *mut RPC_MESSAGE) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcns4.dll""system" fn I_RpcReBindBuffer ( message : *mut RPC_MESSAGE ) -> RPC_STATUS );
    I_RpcReBindBuffer(message)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn I_RpcReallocPipeBuffer(message: *const RPC_MESSAGE, newsize: u32) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn I_RpcReallocPipeBuffer ( message : *const RPC_MESSAGE , newsize : u32 ) -> RPC_STATUS );
    I_RpcReallocPipeBuffer(message, newsize)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn I_RpcReceive(message: *mut RPC_MESSAGE, size: u32) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn I_RpcReceive ( message : *mut RPC_MESSAGE , size : u32 ) -> RPC_STATUS );
    I_RpcReceive(message, size)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn I_RpcRecordCalloutFailure(rpcstatus: RPC_STATUS, calloutstate: *mut RDR_CALLOUT_STATE, dllname: *mut u16) {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn I_RpcRecordCalloutFailure ( rpcstatus : RPC_STATUS , calloutstate : *mut RDR_CALLOUT_STATE , dllname : *mut u16 ) -> ( ) );
    I_RpcRecordCalloutFailure(rpcstatus, calloutstate, dllname)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn I_RpcRequestMutex(mutex: *mut *mut ::core::ffi::c_void) {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn I_RpcRequestMutex ( mutex : *mut *mut ::core::ffi::c_void ) -> ( ) );
    I_RpcRequestMutex(mutex)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn I_RpcSend(message: *mut RPC_MESSAGE) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn I_RpcSend ( message : *mut RPC_MESSAGE ) -> RPC_STATUS );
    I_RpcSend(message)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn I_RpcSendReceive(message: *mut RPC_MESSAGE) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn I_RpcSendReceive ( message : *mut RPC_MESSAGE ) -> RPC_STATUS );
    I_RpcSendReceive(message)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn I_RpcServerCheckClientRestriction(context: *mut ::core::ffi::c_void) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn I_RpcServerCheckClientRestriction ( context : *mut ::core::ffi::c_void ) -> RPC_STATUS );
    I_RpcServerCheckClientRestriction(context)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn I_RpcServerDisableExceptionFilter() -> i32 {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn I_RpcServerDisableExceptionFilter ( ) -> i32 );
    I_RpcServerDisableExceptionFilter()
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn I_RpcServerGetAssociationID(binding: ::core::option::Option<*const ::core::ffi::c_void>, associationid: *mut u32) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn I_RpcServerGetAssociationID ( binding : *const ::core::ffi::c_void , associationid : *mut u32 ) -> RPC_STATUS );
    I_RpcServerGetAssociationID(::core::mem::transmute(binding.unwrap_or(::std::ptr::null())), associationid)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn I_RpcServerInqAddressChangeFn() -> *mut RPC_ADDRESS_CHANGE_FN {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn I_RpcServerInqAddressChangeFn ( ) -> *mut RPC_ADDRESS_CHANGE_FN );
    I_RpcServerInqAddressChangeFn()
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn I_RpcServerInqLocalConnAddress(binding: *mut ::core::ffi::c_void, buffer: *mut ::core::ffi::c_void, buffersize: *mut u32, addressformat: *mut u32) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn I_RpcServerInqLocalConnAddress ( binding : *mut ::core::ffi::c_void , buffer : *mut ::core::ffi::c_void , buffersize : *mut u32 , addressformat : *mut u32 ) -> RPC_STATUS );
    I_RpcServerInqLocalConnAddress(binding, buffer, buffersize, addressformat)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn I_RpcServerInqRemoteConnAddress(binding: *mut ::core::ffi::c_void, buffer: *mut ::core::ffi::c_void, buffersize: *mut u32, addressformat: *mut u32) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn I_RpcServerInqRemoteConnAddress ( binding : *mut ::core::ffi::c_void , buffer : *mut ::core::ffi::c_void , buffersize : *mut u32 , addressformat : *mut u32 ) -> RPC_STATUS );
    I_RpcServerInqRemoteConnAddress(binding, buffer, buffersize, addressformat)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn I_RpcServerInqTransportType(r#type: *mut u32) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn I_RpcServerInqTransportType ( r#type : *mut u32 ) -> RPC_STATUS );
    I_RpcServerInqTransportType(r#type)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn I_RpcServerRegisterForwardFunction(pforwardfunction: *mut RPC_FORWARD_FUNCTION) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn I_RpcServerRegisterForwardFunction ( pforwardfunction : *mut RPC_FORWARD_FUNCTION ) -> RPC_STATUS );
    I_RpcServerRegisterForwardFunction(pforwardfunction)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn I_RpcServerSetAddressChangeFn(paddresschangefn: *mut RPC_ADDRESS_CHANGE_FN) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn I_RpcServerSetAddressChangeFn ( paddresschangefn : *mut RPC_ADDRESS_CHANGE_FN ) -> RPC_STATUS );
    I_RpcServerSetAddressChangeFn(paddresschangefn)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn I_RpcServerStartService(protseq: *const u16, endpoint: *const u16, ifspec: *const ::core::ffi::c_void) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn I_RpcServerStartService ( protseq : *const u16 , endpoint : *const u16 , ifspec : *const ::core::ffi::c_void ) -> RPC_STATUS );
    I_RpcServerStartService(protseq, endpoint, ifspec)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn I_RpcServerSubscribeForDisconnectNotification(binding: ::core::option::Option<*const ::core::ffi::c_void>, hevent: ::core::option::Option<*const ::core::ffi::c_void>) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn I_RpcServerSubscribeForDisconnectNotification ( binding : *const ::core::ffi::c_void , hevent : *const ::core::ffi::c_void ) -> RPC_STATUS );
    I_RpcServerSubscribeForDisconnectNotification(::core::mem::transmute(binding.unwrap_or(::std::ptr::null())), ::core::mem::transmute(hevent.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn I_RpcServerSubscribeForDisconnectNotification2(binding: ::core::option::Option<*const ::core::ffi::c_void>, hevent: *const ::core::ffi::c_void, subscriptionid: *mut ::windows::core::GUID) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn I_RpcServerSubscribeForDisconnectNotification2 ( binding : *const ::core::ffi::c_void , hevent : *const ::core::ffi::c_void , subscriptionid : *mut :: windows::core::GUID ) -> RPC_STATUS );
    I_RpcServerSubscribeForDisconnectNotification2(::core::mem::transmute(binding.unwrap_or(::std::ptr::null())), hevent, subscriptionid)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn I_RpcServerUnsubscribeForDisconnectNotification(binding: ::core::option::Option<*const ::core::ffi::c_void>, subscriptionid: ::windows::core::GUID) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn I_RpcServerUnsubscribeForDisconnectNotification ( binding : *const ::core::ffi::c_void , subscriptionid : :: windows::core::GUID ) -> RPC_STATUS );
    I_RpcServerUnsubscribeForDisconnectNotification(::core::mem::transmute(binding.unwrap_or(::std::ptr::null())), ::core::mem::transmute(subscriptionid))
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn I_RpcServerUseProtseq2A(networkaddress: ::core::option::Option<*const u8>, protseq: *const u8, maxcalls: u32, securitydescriptor: ::core::option::Option<*const ::core::ffi::c_void>, policy: *const ::core::ffi::c_void) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn I_RpcServerUseProtseq2A ( networkaddress : *const u8 , protseq : *const u8 , maxcalls : u32 , securitydescriptor : *const ::core::ffi::c_void , policy : *const ::core::ffi::c_void ) -> RPC_STATUS );
    I_RpcServerUseProtseq2A(::core::mem::transmute(networkaddress.unwrap_or(::std::ptr::null())), protseq, maxcalls, ::core::mem::transmute(securitydescriptor.unwrap_or(::std::ptr::null())), policy)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn I_RpcServerUseProtseq2W(networkaddress: ::core::option::Option<*const u16>, protseq: *const u16, maxcalls: u32, securitydescriptor: ::core::option::Option<*const ::core::ffi::c_void>, policy: *const ::core::ffi::c_void) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn I_RpcServerUseProtseq2W ( networkaddress : *const u16 , protseq : *const u16 , maxcalls : u32 , securitydescriptor : *const ::core::ffi::c_void , policy : *const ::core::ffi::c_void ) -> RPC_STATUS );
    I_RpcServerUseProtseq2W(::core::mem::transmute(networkaddress.unwrap_or(::std::ptr::null())), protseq, maxcalls, ::core::mem::transmute(securitydescriptor.unwrap_or(::std::ptr::null())), policy)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn I_RpcServerUseProtseqEp2A(networkaddress: ::core::option::Option<*const u8>, protseq: *const u8, maxcalls: u32, endpoint: *const u8, securitydescriptor: ::core::option::Option<*const ::core::ffi::c_void>, policy: *const ::core::ffi::c_void) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn I_RpcServerUseProtseqEp2A ( networkaddress : *const u8 , protseq : *const u8 , maxcalls : u32 , endpoint : *const u8 , securitydescriptor : *const ::core::ffi::c_void , policy : *const ::core::ffi::c_void ) -> RPC_STATUS );
    I_RpcServerUseProtseqEp2A(::core::mem::transmute(networkaddress.unwrap_or(::std::ptr::null())), protseq, maxcalls, endpoint, ::core::mem::transmute(securitydescriptor.unwrap_or(::std::ptr::null())), policy)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn I_RpcServerUseProtseqEp2W(networkaddress: ::core::option::Option<*const u16>, protseq: *const u16, maxcalls: u32, endpoint: *const u16, securitydescriptor: ::core::option::Option<*const ::core::ffi::c_void>, policy: *const ::core::ffi::c_void) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn I_RpcServerUseProtseqEp2W ( networkaddress : *const u16 , protseq : *const u16 , maxcalls : u32 , endpoint : *const u16 , securitydescriptor : *const ::core::ffi::c_void , policy : *const ::core::ffi::c_void ) -> RPC_STATUS );
    I_RpcServerUseProtseqEp2W(::core::mem::transmute(networkaddress.unwrap_or(::std::ptr::null())), protseq, maxcalls, endpoint, ::core::mem::transmute(securitydescriptor.unwrap_or(::std::ptr::null())), policy)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn I_RpcSessionStrictContextHandle() {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn I_RpcSessionStrictContextHandle ( ) -> ( ) );
    I_RpcSessionStrictContextHandle()
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn I_RpcSsDontSerializeContext() {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn I_RpcSsDontSerializeContext ( ) -> ( ) );
    I_RpcSsDontSerializeContext()
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn I_RpcSystemHandleTypeSpecificWork(handle: *mut ::core::ffi::c_void, actualtype: u8, idltype: u8, marshaldirection: LRPC_SYSTEM_HANDLE_MARSHAL_DIRECTION) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn I_RpcSystemHandleTypeSpecificWork ( handle : *mut ::core::ffi::c_void , actualtype : u8 , idltype : u8 , marshaldirection : LRPC_SYSTEM_HANDLE_MARSHAL_DIRECTION ) -> RPC_STATUS );
    I_RpcSystemHandleTypeSpecificWork(handle, actualtype, idltype, marshaldirection)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn I_RpcTurnOnEEInfoPropagation() -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn I_RpcTurnOnEEInfoPropagation ( ) -> RPC_STATUS );
    I_RpcTurnOnEEInfoPropagation()
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn I_UuidCreate(uuid: *mut ::windows::core::GUID) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn I_UuidCreate ( uuid : *mut :: windows::core::GUID ) -> RPC_STATUS );
    I_UuidCreate(uuid)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn MesBufferHandleReset(handle: *const ::core::ffi::c_void, handlestyle: u32, operation: MIDL_ES_CODE, pbuffer: ::core::option::Option<&[u8]>, pencodedsize: ::core::option::Option<*mut u32>) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn MesBufferHandleReset ( handle : *const ::core::ffi::c_void , handlestyle : u32 , operation : MIDL_ES_CODE , pbuffer : *const *const i8 , buffersize : u32 , pencodedsize : *mut u32 ) -> RPC_STATUS );
    MesBufferHandleReset(handle, handlestyle, operation, ::core::mem::transmute(pbuffer.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), pbuffer.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(pencodedsize.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn MesDecodeBufferHandleCreate(buffer: &[u8], phandle: *mut *mut ::core::ffi::c_void) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn MesDecodeBufferHandleCreate ( buffer : :: windows::core::PCSTR , buffersize : u32 , phandle : *mut *mut ::core::ffi::c_void ) -> RPC_STATUS );
    MesDecodeBufferHandleCreate(::core::mem::transmute(buffer.as_ptr()), buffer.len() as _, phandle)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn MesDecodeIncrementalHandleCreate(userstate: *mut ::core::ffi::c_void, readfn: MIDL_ES_READ, phandle: *mut *mut ::core::ffi::c_void) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn MesDecodeIncrementalHandleCreate ( userstate : *mut ::core::ffi::c_void , readfn : MIDL_ES_READ , phandle : *mut *mut ::core::ffi::c_void ) -> RPC_STATUS );
    MesDecodeIncrementalHandleCreate(userstate, readfn, phandle)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn MesEncodeDynBufferHandleCreate(pbuffer: *mut *mut i8, pencodedsize: *mut u32, phandle: *mut *mut ::core::ffi::c_void) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn MesEncodeDynBufferHandleCreate ( pbuffer : *mut *mut i8 , pencodedsize : *mut u32 , phandle : *mut *mut ::core::ffi::c_void ) -> RPC_STATUS );
    MesEncodeDynBufferHandleCreate(pbuffer, pencodedsize, phandle)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn MesEncodeFixedBufferHandleCreate(pbuffer: &mut [u8], pencodedsize: *mut u32, phandle: *mut *mut ::core::ffi::c_void) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn MesEncodeFixedBufferHandleCreate ( pbuffer : :: windows::core::PSTR , buffersize : u32 , pencodedsize : *mut u32 , phandle : *mut *mut ::core::ffi::c_void ) -> RPC_STATUS );
    MesEncodeFixedBufferHandleCreate(::core::mem::transmute(pbuffer.as_ptr()), pbuffer.len() as _, pencodedsize, phandle)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn MesEncodeIncrementalHandleCreate(userstate: *mut ::core::ffi::c_void, allocfn: MIDL_ES_ALLOC, writefn: MIDL_ES_WRITE, phandle: *mut *mut ::core::ffi::c_void) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn MesEncodeIncrementalHandleCreate ( userstate : *mut ::core::ffi::c_void , allocfn : MIDL_ES_ALLOC , writefn : MIDL_ES_WRITE , phandle : *mut *mut ::core::ffi::c_void ) -> RPC_STATUS );
    MesEncodeIncrementalHandleCreate(userstate, allocfn, writefn, phandle)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn MesHandleFree(handle: *mut ::core::ffi::c_void) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn MesHandleFree ( handle : *mut ::core::ffi::c_void ) -> RPC_STATUS );
    MesHandleFree(handle)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn MesIncrementalHandleReset(handle: *mut ::core::ffi::c_void, userstate: *mut ::core::ffi::c_void, allocfn: MIDL_ES_ALLOC, writefn: MIDL_ES_WRITE, readfn: MIDL_ES_READ, operation: MIDL_ES_CODE) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn MesIncrementalHandleReset ( handle : *mut ::core::ffi::c_void , userstate : *mut ::core::ffi::c_void , allocfn : MIDL_ES_ALLOC , writefn : MIDL_ES_WRITE , readfn : MIDL_ES_READ , operation : MIDL_ES_CODE ) -> RPC_STATUS );
    MesIncrementalHandleReset(handle, userstate, allocfn, writefn, readfn, operation)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn MesInqProcEncodingId(handle: *mut ::core::ffi::c_void, pinterfaceid: *mut RPC_SYNTAX_IDENTIFIER, pprocnum: *mut u32) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn MesInqProcEncodingId ( handle : *mut ::core::ffi::c_void , pinterfaceid : *mut RPC_SYNTAX_IDENTIFIER , pprocnum : *mut u32 ) -> RPC_STATUS );
    MesInqProcEncodingId(handle, pinterfaceid, pprocnum)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn NDRCContextBinding(ccontext: isize) -> *mut ::core::ffi::c_void {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn NDRCContextBinding ( ccontext : isize ) -> *mut ::core::ffi::c_void );
    NDRCContextBinding(ccontext)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn NDRCContextMarshall(ccontext: isize, pbuff: *mut ::core::ffi::c_void) {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn NDRCContextMarshall ( ccontext : isize , pbuff : *mut ::core::ffi::c_void ) -> ( ) );
    NDRCContextMarshall(ccontext, pbuff)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn NDRCContextUnmarshall(pccontext: ::core::option::Option<*mut isize>, hbinding: *const ::core::ffi::c_void, pbuff: *const ::core::ffi::c_void, datarepresentation: u32) {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn NDRCContextUnmarshall ( pccontext : *mut isize , hbinding : *const ::core::ffi::c_void , pbuff : *const ::core::ffi::c_void , datarepresentation : u32 ) -> ( ) );
    NDRCContextUnmarshall(::core::mem::transmute(pccontext.unwrap_or(::std::ptr::null_mut())), hbinding, pbuff, datarepresentation)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn NDRSContextMarshall(ccontext: *const NDR_SCONTEXT, pbuff: *mut ::core::ffi::c_void, userrundownin: NDR_RUNDOWN) {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn NDRSContextMarshall ( ccontext : *const NDR_SCONTEXT , pbuff : *mut ::core::ffi::c_void , userrundownin : NDR_RUNDOWN ) -> ( ) );
    NDRSContextMarshall(ccontext, pbuff, userrundownin)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn NDRSContextMarshall2(bindinghandle: *const ::core::ffi::c_void, ccontext: *const NDR_SCONTEXT, pbuff: *mut ::core::ffi::c_void, userrundownin: NDR_RUNDOWN, ctxguard: ::core::option::Option<*const ::core::ffi::c_void>, flags: u32) {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn NDRSContextMarshall2 ( bindinghandle : *const ::core::ffi::c_void , ccontext : *const NDR_SCONTEXT , pbuff : *mut ::core::ffi::c_void , userrundownin : NDR_RUNDOWN , ctxguard : *const ::core::ffi::c_void , flags : u32 ) -> ( ) );
    NDRSContextMarshall2(bindinghandle, ccontext, pbuff, userrundownin, ::core::mem::transmute(ctxguard.unwrap_or(::std::ptr::null())), flags)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn NDRSContextMarshallEx(bindinghandle: *const ::core::ffi::c_void, ccontext: *const NDR_SCONTEXT, pbuff: *mut ::core::ffi::c_void, userrundownin: NDR_RUNDOWN) {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn NDRSContextMarshallEx ( bindinghandle : *const ::core::ffi::c_void , ccontext : *const NDR_SCONTEXT , pbuff : *mut ::core::ffi::c_void , userrundownin : NDR_RUNDOWN ) -> ( ) );
    NDRSContextMarshallEx(bindinghandle, ccontext, pbuff, userrundownin)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn NDRSContextUnmarshall(pbuff: *const ::core::ffi::c_void, datarepresentation: u32) -> *mut NDR_SCONTEXT {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn NDRSContextUnmarshall ( pbuff : *const ::core::ffi::c_void , datarepresentation : u32 ) -> *mut NDR_SCONTEXT );
    NDRSContextUnmarshall(pbuff, datarepresentation)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn NDRSContextUnmarshall2(bindinghandle: *const ::core::ffi::c_void, pbuff: ::core::option::Option<*const ::core::ffi::c_void>, datarepresentation: u32, ctxguard: ::core::option::Option<*const ::core::ffi::c_void>, flags: u32) -> *mut NDR_SCONTEXT {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn NDRSContextUnmarshall2 ( bindinghandle : *const ::core::ffi::c_void , pbuff : *const ::core::ffi::c_void , datarepresentation : u32 , ctxguard : *const ::core::ffi::c_void , flags : u32 ) -> *mut NDR_SCONTEXT );
    NDRSContextUnmarshall2(bindinghandle, ::core::mem::transmute(pbuff.unwrap_or(::std::ptr::null())), datarepresentation, ::core::mem::transmute(ctxguard.unwrap_or(::std::ptr::null())), flags)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn NDRSContextUnmarshallEx(bindinghandle: *const ::core::ffi::c_void, pbuff: *const ::core::ffi::c_void, datarepresentation: u32) -> *mut NDR_SCONTEXT {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn NDRSContextUnmarshallEx ( bindinghandle : *const ::core::ffi::c_void , pbuff : *const ::core::ffi::c_void , datarepresentation : u32 ) -> *mut NDR_SCONTEXT );
    NDRSContextUnmarshallEx(bindinghandle, pbuff, datarepresentation)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn Ndr64AsyncClientCall(pproxyinfo: *mut MIDL_STUBLESS_PROXY_INFO, nprocnum: u32, preturnvalue: *mut ::core::ffi::c_void) -> CLIENT_CALL_RETURN {
    ::windows::imp::link ! ( "rpcrt4.dll""cdecl" fn Ndr64AsyncClientCall ( pproxyinfo : *mut MIDL_STUBLESS_PROXY_INFO , nprocnum : u32 , preturnvalue : *mut ::core::ffi::c_void ) -> CLIENT_CALL_RETURN );
    Ndr64AsyncClientCall(pproxyinfo, nprocnum, preturnvalue)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn Ndr64AsyncServerCall64(prpcmsg: *mut RPC_MESSAGE) {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn Ndr64AsyncServerCall64 ( prpcmsg : *mut RPC_MESSAGE ) -> ( ) );
    Ndr64AsyncServerCall64(prpcmsg)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn Ndr64AsyncServerCallAll(prpcmsg: *mut RPC_MESSAGE) {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn Ndr64AsyncServerCallAll ( prpcmsg : *mut RPC_MESSAGE ) -> ( ) );
    Ndr64AsyncServerCallAll(prpcmsg)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn Ndr64DcomAsyncClientCall(pproxyinfo: *mut MIDL_STUBLESS_PROXY_INFO, nprocnum: u32, preturnvalue: *mut ::core::ffi::c_void) -> CLIENT_CALL_RETURN {
    ::windows::imp::link ! ( "rpcrt4.dll""cdecl" fn Ndr64DcomAsyncClientCall ( pproxyinfo : *mut MIDL_STUBLESS_PROXY_INFO , nprocnum : u32 , preturnvalue : *mut ::core::ffi::c_void ) -> CLIENT_CALL_RETURN );
    Ndr64DcomAsyncClientCall(pproxyinfo, nprocnum, preturnvalue)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn Ndr64DcomAsyncStubCall<P0, P1>(pthis: P0, pchannel: P1, prpcmsg: *mut RPC_MESSAGE, pdwstubphase: *mut u32) -> i32
where
    P0: ::windows::core::IntoParam<super::Com::IRpcStubBuffer>,
    P1: ::windows::core::IntoParam<super::Com::IRpcChannelBuffer>,
{
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn Ndr64DcomAsyncStubCall ( pthis : * mut::core::ffi::c_void , pchannel : * mut::core::ffi::c_void , prpcmsg : *mut RPC_MESSAGE , pdwstubphase : *mut u32 ) -> i32 );
    Ndr64DcomAsyncStubCall(pthis.into_param().abi(), pchannel.into_param().abi(), prpcmsg, pdwstubphase)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrAllocate(pstubmsg: *mut MIDL_STUB_MESSAGE, len: usize) -> *mut ::core::ffi::c_void {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn NdrAllocate ( pstubmsg : *mut MIDL_STUB_MESSAGE , len : usize ) -> *mut ::core::ffi::c_void );
    NdrAllocate(pstubmsg, len)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrAsyncClientCall(pstubdescriptor: *mut MIDL_STUB_DESC, pformat: *mut u8) -> CLIENT_CALL_RETURN {
    ::windows::imp::link ! ( "rpcrt4.dll""cdecl" fn NdrAsyncClientCall ( pstubdescriptor : *mut MIDL_STUB_DESC , pformat : *mut u8 ) -> CLIENT_CALL_RETURN );
    NdrAsyncClientCall(pstubdescriptor, pformat)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn NdrAsyncServerCall(prpcmsg: *mut RPC_MESSAGE) {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn NdrAsyncServerCall ( prpcmsg : *mut RPC_MESSAGE ) -> ( ) );
    NdrAsyncServerCall(prpcmsg)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrByteCountPointerBufferSize(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn NdrByteCountPointerBufferSize ( pstubmsg : *mut MIDL_STUB_MESSAGE , pmemory : *mut u8 , pformat : *mut u8 ) -> ( ) );
    NdrByteCountPointerBufferSize(pstubmsg, pmemory, pformat)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrByteCountPointerFree(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn NdrByteCountPointerFree ( pstubmsg : *mut MIDL_STUB_MESSAGE , pmemory : *mut u8 , pformat : *mut u8 ) -> ( ) );
    NdrByteCountPointerFree(pstubmsg, pmemory, pformat)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrByteCountPointerMarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) -> *mut u8 {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn NdrByteCountPointerMarshall ( pstubmsg : *mut MIDL_STUB_MESSAGE , pmemory : *mut u8 , pformat : *mut u8 ) -> *mut u8 );
    NdrByteCountPointerMarshall(pstubmsg, pmemory, pformat)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrByteCountPointerUnmarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, ppmemory: *mut *mut u8, pformat: *mut u8, fmustalloc: u8) -> *mut u8 {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn NdrByteCountPointerUnmarshall ( pstubmsg : *mut MIDL_STUB_MESSAGE , ppmemory : *mut *mut u8 , pformat : *mut u8 , fmustalloc : u8 ) -> *mut u8 );
    NdrByteCountPointerUnmarshall(pstubmsg, ppmemory, pformat, fmustalloc)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrClearOutParameters(pstubmsg: *mut MIDL_STUB_MESSAGE, pformat: *mut u8, argaddr: *mut ::core::ffi::c_void) {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn NdrClearOutParameters ( pstubmsg : *mut MIDL_STUB_MESSAGE , pformat : *mut u8 , argaddr : *mut ::core::ffi::c_void ) -> ( ) );
    NdrClearOutParameters(pstubmsg, pformat, argaddr)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrClientCall2(pstubdescriptor: *mut MIDL_STUB_DESC, pformat: *mut u8) -> CLIENT_CALL_RETURN {
    ::windows::imp::link ! ( "rpcrt4.dll""cdecl" fn NdrClientCall2 ( pstubdescriptor : *mut MIDL_STUB_DESC , pformat : *mut u8 ) -> CLIENT_CALL_RETURN );
    NdrClientCall2(pstubdescriptor, pformat)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrClientCall3(pproxyinfo: *mut MIDL_STUBLESS_PROXY_INFO, nprocnum: u32, preturnvalue: *mut ::core::ffi::c_void) -> CLIENT_CALL_RETURN {
    ::windows::imp::link ! ( "rpcrt4.dll""cdecl" fn NdrClientCall3 ( pproxyinfo : *mut MIDL_STUBLESS_PROXY_INFO , nprocnum : u32 , preturnvalue : *mut ::core::ffi::c_void ) -> CLIENT_CALL_RETURN );
    NdrClientCall3(pproxyinfo, nprocnum, preturnvalue)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrClientContextMarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, contexthandle: isize, fcheck: i32) {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn NdrClientContextMarshall ( pstubmsg : *mut MIDL_STUB_MESSAGE , contexthandle : isize , fcheck : i32 ) -> ( ) );
    NdrClientContextMarshall(pstubmsg, contexthandle, fcheck)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrClientContextUnmarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, pcontexthandle: *mut isize, bindhandle: *mut ::core::ffi::c_void) {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn NdrClientContextUnmarshall ( pstubmsg : *mut MIDL_STUB_MESSAGE , pcontexthandle : *mut isize , bindhandle : *mut ::core::ffi::c_void ) -> ( ) );
    NdrClientContextUnmarshall(pstubmsg, pcontexthandle, bindhandle)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrClientInitialize(prpcmsg: *mut RPC_MESSAGE, pstubmsg: *mut MIDL_STUB_MESSAGE, pstubdescriptor: *mut MIDL_STUB_DESC, procnum: u32) {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn NdrClientInitialize ( prpcmsg : *mut RPC_MESSAGE , pstubmsg : *mut MIDL_STUB_MESSAGE , pstubdescriptor : *mut MIDL_STUB_DESC , procnum : u32 ) -> ( ) );
    NdrClientInitialize(prpcmsg, pstubmsg, pstubdescriptor, procnum)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrClientInitializeNew(prpcmsg: *mut RPC_MESSAGE, pstubmsg: *mut MIDL_STUB_MESSAGE, pstubdescriptor: *mut MIDL_STUB_DESC, procnum: u32) {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn NdrClientInitializeNew ( prpcmsg : *mut RPC_MESSAGE , pstubmsg : *mut MIDL_STUB_MESSAGE , pstubdescriptor : *mut MIDL_STUB_DESC , procnum : u32 ) -> ( ) );
    NdrClientInitializeNew(prpcmsg, pstubmsg, pstubdescriptor, procnum)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrComplexArrayBufferSize(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn NdrComplexArrayBufferSize ( pstubmsg : *mut MIDL_STUB_MESSAGE , pmemory : *mut u8 , pformat : *mut u8 ) -> ( ) );
    NdrComplexArrayBufferSize(pstubmsg, pmemory, pformat)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrComplexArrayFree(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn NdrComplexArrayFree ( pstubmsg : *mut MIDL_STUB_MESSAGE , pmemory : *mut u8 , pformat : *mut u8 ) -> ( ) );
    NdrComplexArrayFree(pstubmsg, pmemory, pformat)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrComplexArrayMarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) -> *mut u8 {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn NdrComplexArrayMarshall ( pstubmsg : *mut MIDL_STUB_MESSAGE , pmemory : *mut u8 , pformat : *mut u8 ) -> *mut u8 );
    NdrComplexArrayMarshall(pstubmsg, pmemory, pformat)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrComplexArrayMemorySize(pstubmsg: *mut MIDL_STUB_MESSAGE, pformat: *mut u8) -> u32 {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn NdrComplexArrayMemorySize ( pstubmsg : *mut MIDL_STUB_MESSAGE , pformat : *mut u8 ) -> u32 );
    NdrComplexArrayMemorySize(pstubmsg, pformat)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrComplexArrayUnmarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, ppmemory: *mut *mut u8, pformat: *mut u8, fmustalloc: u8) -> *mut u8 {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn NdrComplexArrayUnmarshall ( pstubmsg : *mut MIDL_STUB_MESSAGE , ppmemory : *mut *mut u8 , pformat : *mut u8 , fmustalloc : u8 ) -> *mut u8 );
    NdrComplexArrayUnmarshall(pstubmsg, ppmemory, pformat, fmustalloc)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrComplexStructBufferSize(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn NdrComplexStructBufferSize ( pstubmsg : *mut MIDL_STUB_MESSAGE , pmemory : *mut u8 , pformat : *mut u8 ) -> ( ) );
    NdrComplexStructBufferSize(pstubmsg, pmemory, pformat)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrComplexStructFree(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn NdrComplexStructFree ( pstubmsg : *mut MIDL_STUB_MESSAGE , pmemory : *mut u8 , pformat : *mut u8 ) -> ( ) );
    NdrComplexStructFree(pstubmsg, pmemory, pformat)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrComplexStructMarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) -> *mut u8 {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn NdrComplexStructMarshall ( pstubmsg : *mut MIDL_STUB_MESSAGE , pmemory : *mut u8 , pformat : *mut u8 ) -> *mut u8 );
    NdrComplexStructMarshall(pstubmsg, pmemory, pformat)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrComplexStructMemorySize(pstubmsg: *mut MIDL_STUB_MESSAGE, pformat: *mut u8) -> u32 {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn NdrComplexStructMemorySize ( pstubmsg : *mut MIDL_STUB_MESSAGE , pformat : *mut u8 ) -> u32 );
    NdrComplexStructMemorySize(pstubmsg, pformat)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrComplexStructUnmarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, ppmemory: *mut *mut u8, pformat: *mut u8, fmustalloc: u8) -> *mut u8 {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn NdrComplexStructUnmarshall ( pstubmsg : *mut MIDL_STUB_MESSAGE , ppmemory : *mut *mut u8 , pformat : *mut u8 , fmustalloc : u8 ) -> *mut u8 );
    NdrComplexStructUnmarshall(pstubmsg, ppmemory, pformat, fmustalloc)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrConformantArrayBufferSize(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn NdrConformantArrayBufferSize ( pstubmsg : *mut MIDL_STUB_MESSAGE , pmemory : *mut u8 , pformat : *mut u8 ) -> ( ) );
    NdrConformantArrayBufferSize(pstubmsg, pmemory, pformat)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrConformantArrayFree(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn NdrConformantArrayFree ( pstubmsg : *mut MIDL_STUB_MESSAGE , pmemory : *mut u8 , pformat : *mut u8 ) -> ( ) );
    NdrConformantArrayFree(pstubmsg, pmemory, pformat)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrConformantArrayMarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) -> *mut u8 {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn NdrConformantArrayMarshall ( pstubmsg : *mut MIDL_STUB_MESSAGE , pmemory : *mut u8 , pformat : *mut u8 ) -> *mut u8 );
    NdrConformantArrayMarshall(pstubmsg, pmemory, pformat)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrConformantArrayMemorySize(pstubmsg: *mut MIDL_STUB_MESSAGE, pformat: *mut u8) -> u32 {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn NdrConformantArrayMemorySize ( pstubmsg : *mut MIDL_STUB_MESSAGE , pformat : *mut u8 ) -> u32 );
    NdrConformantArrayMemorySize(pstubmsg, pformat)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrConformantArrayUnmarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, ppmemory: *mut *mut u8, pformat: *mut u8, fmustalloc: u8) -> *mut u8 {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn NdrConformantArrayUnmarshall ( pstubmsg : *mut MIDL_STUB_MESSAGE , ppmemory : *mut *mut u8 , pformat : *mut u8 , fmustalloc : u8 ) -> *mut u8 );
    NdrConformantArrayUnmarshall(pstubmsg, ppmemory, pformat, fmustalloc)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrConformantStringBufferSize(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn NdrConformantStringBufferSize ( pstubmsg : *mut MIDL_STUB_MESSAGE , pmemory : *mut u8 , pformat : *mut u8 ) -> ( ) );
    NdrConformantStringBufferSize(pstubmsg, pmemory, pformat)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrConformantStringMarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) -> *mut u8 {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn NdrConformantStringMarshall ( pstubmsg : *mut MIDL_STUB_MESSAGE , pmemory : *mut u8 , pformat : *mut u8 ) -> *mut u8 );
    NdrConformantStringMarshall(pstubmsg, pmemory, pformat)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrConformantStringMemorySize(pstubmsg: *mut MIDL_STUB_MESSAGE, pformat: *mut u8) -> u32 {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn NdrConformantStringMemorySize ( pstubmsg : *mut MIDL_STUB_MESSAGE , pformat : *mut u8 ) -> u32 );
    NdrConformantStringMemorySize(pstubmsg, pformat)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrConformantStringUnmarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, ppmemory: *mut *mut u8, pformat: *mut u8, fmustalloc: u8) -> *mut u8 {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn NdrConformantStringUnmarshall ( pstubmsg : *mut MIDL_STUB_MESSAGE , ppmemory : *mut *mut u8 , pformat : *mut u8 , fmustalloc : u8 ) -> *mut u8 );
    NdrConformantStringUnmarshall(pstubmsg, ppmemory, pformat, fmustalloc)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrConformantStructBufferSize(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn NdrConformantStructBufferSize ( pstubmsg : *mut MIDL_STUB_MESSAGE , pmemory : *mut u8 , pformat : *mut u8 ) -> ( ) );
    NdrConformantStructBufferSize(pstubmsg, pmemory, pformat)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrConformantStructFree(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn NdrConformantStructFree ( pstubmsg : *mut MIDL_STUB_MESSAGE , pmemory : *mut u8 , pformat : *mut u8 ) -> ( ) );
    NdrConformantStructFree(pstubmsg, pmemory, pformat)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrConformantStructMarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) -> *mut u8 {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn NdrConformantStructMarshall ( pstubmsg : *mut MIDL_STUB_MESSAGE , pmemory : *mut u8 , pformat : *mut u8 ) -> *mut u8 );
    NdrConformantStructMarshall(pstubmsg, pmemory, pformat)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrConformantStructMemorySize(pstubmsg: *mut MIDL_STUB_MESSAGE, pformat: *mut u8) -> u32 {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn NdrConformantStructMemorySize ( pstubmsg : *mut MIDL_STUB_MESSAGE , pformat : *mut u8 ) -> u32 );
    NdrConformantStructMemorySize(pstubmsg, pformat)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrConformantStructUnmarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, ppmemory: *mut *mut u8, pformat: *mut u8, fmustalloc: u8) -> *mut u8 {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn NdrConformantStructUnmarshall ( pstubmsg : *mut MIDL_STUB_MESSAGE , ppmemory : *mut *mut u8 , pformat : *mut u8 , fmustalloc : u8 ) -> *mut u8 );
    NdrConformantStructUnmarshall(pstubmsg, ppmemory, pformat, fmustalloc)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrConformantVaryingArrayBufferSize(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn NdrConformantVaryingArrayBufferSize ( pstubmsg : *mut MIDL_STUB_MESSAGE , pmemory : *mut u8 , pformat : *mut u8 ) -> ( ) );
    NdrConformantVaryingArrayBufferSize(pstubmsg, pmemory, pformat)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrConformantVaryingArrayFree(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn NdrConformantVaryingArrayFree ( pstubmsg : *mut MIDL_STUB_MESSAGE , pmemory : *mut u8 , pformat : *mut u8 ) -> ( ) );
    NdrConformantVaryingArrayFree(pstubmsg, pmemory, pformat)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrConformantVaryingArrayMarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) -> *mut u8 {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn NdrConformantVaryingArrayMarshall ( pstubmsg : *mut MIDL_STUB_MESSAGE , pmemory : *mut u8 , pformat : *mut u8 ) -> *mut u8 );
    NdrConformantVaryingArrayMarshall(pstubmsg, pmemory, pformat)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrConformantVaryingArrayMemorySize(pstubmsg: *mut MIDL_STUB_MESSAGE, pformat: *mut u8) -> u32 {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn NdrConformantVaryingArrayMemorySize ( pstubmsg : *mut MIDL_STUB_MESSAGE , pformat : *mut u8 ) -> u32 );
    NdrConformantVaryingArrayMemorySize(pstubmsg, pformat)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrConformantVaryingArrayUnmarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, ppmemory: *mut *mut u8, pformat: *mut u8, fmustalloc: u8) -> *mut u8 {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn NdrConformantVaryingArrayUnmarshall ( pstubmsg : *mut MIDL_STUB_MESSAGE , ppmemory : *mut *mut u8 , pformat : *mut u8 , fmustalloc : u8 ) -> *mut u8 );
    NdrConformantVaryingArrayUnmarshall(pstubmsg, ppmemory, pformat, fmustalloc)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrConformantVaryingStructBufferSize(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn NdrConformantVaryingStructBufferSize ( pstubmsg : *mut MIDL_STUB_MESSAGE , pmemory : *mut u8 , pformat : *mut u8 ) -> ( ) );
    NdrConformantVaryingStructBufferSize(pstubmsg, pmemory, pformat)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrConformantVaryingStructFree(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn NdrConformantVaryingStructFree ( pstubmsg : *mut MIDL_STUB_MESSAGE , pmemory : *mut u8 , pformat : *mut u8 ) -> ( ) );
    NdrConformantVaryingStructFree(pstubmsg, pmemory, pformat)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrConformantVaryingStructMarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) -> *mut u8 {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn NdrConformantVaryingStructMarshall ( pstubmsg : *mut MIDL_STUB_MESSAGE , pmemory : *mut u8 , pformat : *mut u8 ) -> *mut u8 );
    NdrConformantVaryingStructMarshall(pstubmsg, pmemory, pformat)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrConformantVaryingStructMemorySize(pstubmsg: *mut MIDL_STUB_MESSAGE, pformat: *mut u8) -> u32 {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn NdrConformantVaryingStructMemorySize ( pstubmsg : *mut MIDL_STUB_MESSAGE , pformat : *mut u8 ) -> u32 );
    NdrConformantVaryingStructMemorySize(pstubmsg, pformat)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrConformantVaryingStructUnmarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, ppmemory: *mut *mut u8, pformat: *mut u8, fmustalloc: u8) -> *mut u8 {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn NdrConformantVaryingStructUnmarshall ( pstubmsg : *mut MIDL_STUB_MESSAGE , ppmemory : *mut *mut u8 , pformat : *mut u8 , fmustalloc : u8 ) -> *mut u8 );
    NdrConformantVaryingStructUnmarshall(pstubmsg, ppmemory, pformat, fmustalloc)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrContextHandleInitialize(pstubmsg: *const MIDL_STUB_MESSAGE, pformat: *const u8) -> *mut NDR_SCONTEXT {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn NdrContextHandleInitialize ( pstubmsg : *const MIDL_STUB_MESSAGE , pformat : *const u8 ) -> *mut NDR_SCONTEXT );
    NdrContextHandleInitialize(pstubmsg, pformat)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrContextHandleSize(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn NdrContextHandleSize ( pstubmsg : *mut MIDL_STUB_MESSAGE , pmemory : *mut u8 , pformat : *mut u8 ) -> ( ) );
    NdrContextHandleSize(pstubmsg, pmemory, pformat)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrConvert(pstubmsg: *mut MIDL_STUB_MESSAGE, pformat: *mut u8) {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn NdrConvert ( pstubmsg : *mut MIDL_STUB_MESSAGE , pformat : *mut u8 ) -> ( ) );
    NdrConvert(pstubmsg, pformat)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrConvert2(pstubmsg: *mut MIDL_STUB_MESSAGE, pformat: *mut u8, numberparams: i32) {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn NdrConvert2 ( pstubmsg : *mut MIDL_STUB_MESSAGE , pformat : *mut u8 , numberparams : i32 ) -> ( ) );
    NdrConvert2(pstubmsg, pformat, numberparams)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrCorrelationFree(pstubmsg: *mut MIDL_STUB_MESSAGE) {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn NdrCorrelationFree ( pstubmsg : *mut MIDL_STUB_MESSAGE ) -> ( ) );
    NdrCorrelationFree(pstubmsg)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrCorrelationInitialize(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut ::core::ffi::c_void, cachesize: u32, flags: u32) {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn NdrCorrelationInitialize ( pstubmsg : *mut MIDL_STUB_MESSAGE , pmemory : *mut ::core::ffi::c_void , cachesize : u32 , flags : u32 ) -> ( ) );
    NdrCorrelationInitialize(pstubmsg, pmemory, cachesize, flags)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrCorrelationPass(pstubmsg: *mut MIDL_STUB_MESSAGE) {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn NdrCorrelationPass ( pstubmsg : *mut MIDL_STUB_MESSAGE ) -> ( ) );
    NdrCorrelationPass(pstubmsg)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrCreateServerInterfaceFromStub<P0>(pstub: P0, pserverif: *mut RPC_SERVER_INTERFACE) -> RPC_STATUS
where
    P0: ::windows::core::IntoParam<super::Com::IRpcStubBuffer>,
{
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn NdrCreateServerInterfaceFromStub ( pstub : * mut::core::ffi::c_void , pserverif : *mut RPC_SERVER_INTERFACE ) -> RPC_STATUS );
    NdrCreateServerInterfaceFromStub(pstub.into_param().abi(), pserverif)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrDcomAsyncClientCall(pstubdescriptor: *mut MIDL_STUB_DESC, pformat: *mut u8) -> CLIENT_CALL_RETURN {
    ::windows::imp::link ! ( "rpcrt4.dll""cdecl" fn NdrDcomAsyncClientCall ( pstubdescriptor : *mut MIDL_STUB_DESC , pformat : *mut u8 ) -> CLIENT_CALL_RETURN );
    NdrDcomAsyncClientCall(pstubdescriptor, pformat)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrDcomAsyncStubCall<P0, P1>(pthis: P0, pchannel: P1, prpcmsg: *mut RPC_MESSAGE, pdwstubphase: *mut u32) -> i32
where
    P0: ::windows::core::IntoParam<super::Com::IRpcStubBuffer>,
    P1: ::windows::core::IntoParam<super::Com::IRpcChannelBuffer>,
{
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn NdrDcomAsyncStubCall ( pthis : * mut::core::ffi::c_void , pchannel : * mut::core::ffi::c_void , prpcmsg : *mut RPC_MESSAGE , pdwstubphase : *mut u32 ) -> i32 );
    NdrDcomAsyncStubCall(pthis.into_param().abi(), pchannel.into_param().abi(), prpcmsg, pdwstubphase)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrEncapsulatedUnionBufferSize(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn NdrEncapsulatedUnionBufferSize ( pstubmsg : *mut MIDL_STUB_MESSAGE , pmemory : *mut u8 , pformat : *mut u8 ) -> ( ) );
    NdrEncapsulatedUnionBufferSize(pstubmsg, pmemory, pformat)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrEncapsulatedUnionFree(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn NdrEncapsulatedUnionFree ( pstubmsg : *mut MIDL_STUB_MESSAGE , pmemory : *mut u8 , pformat : *mut u8 ) -> ( ) );
    NdrEncapsulatedUnionFree(pstubmsg, pmemory, pformat)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrEncapsulatedUnionMarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) -> *mut u8 {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn NdrEncapsulatedUnionMarshall ( pstubmsg : *mut MIDL_STUB_MESSAGE , pmemory : *mut u8 , pformat : *mut u8 ) -> *mut u8 );
    NdrEncapsulatedUnionMarshall(pstubmsg, pmemory, pformat)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrEncapsulatedUnionMemorySize(pstubmsg: *mut MIDL_STUB_MESSAGE, pformat: *mut u8) -> u32 {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn NdrEncapsulatedUnionMemorySize ( pstubmsg : *mut MIDL_STUB_MESSAGE , pformat : *mut u8 ) -> u32 );
    NdrEncapsulatedUnionMemorySize(pstubmsg, pformat)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrEncapsulatedUnionUnmarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, ppmemory: *mut *mut u8, pformat: *mut u8, fmustalloc: u8) -> *mut u8 {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn NdrEncapsulatedUnionUnmarshall ( pstubmsg : *mut MIDL_STUB_MESSAGE , ppmemory : *mut *mut u8 , pformat : *mut u8 , fmustalloc : u8 ) -> *mut u8 );
    NdrEncapsulatedUnionUnmarshall(pstubmsg, ppmemory, pformat, fmustalloc)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrFixedArrayBufferSize(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn NdrFixedArrayBufferSize ( pstubmsg : *mut MIDL_STUB_MESSAGE , pmemory : *mut u8 , pformat : *mut u8 ) -> ( ) );
    NdrFixedArrayBufferSize(pstubmsg, pmemory, pformat)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrFixedArrayFree(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn NdrFixedArrayFree ( pstubmsg : *mut MIDL_STUB_MESSAGE , pmemory : *mut u8 , pformat : *mut u8 ) -> ( ) );
    NdrFixedArrayFree(pstubmsg, pmemory, pformat)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrFixedArrayMarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) -> *mut u8 {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn NdrFixedArrayMarshall ( pstubmsg : *mut MIDL_STUB_MESSAGE , pmemory : *mut u8 , pformat : *mut u8 ) -> *mut u8 );
    NdrFixedArrayMarshall(pstubmsg, pmemory, pformat)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrFixedArrayMemorySize(pstubmsg: *mut MIDL_STUB_MESSAGE, pformat: *mut u8) -> u32 {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn NdrFixedArrayMemorySize ( pstubmsg : *mut MIDL_STUB_MESSAGE , pformat : *mut u8 ) -> u32 );
    NdrFixedArrayMemorySize(pstubmsg, pformat)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrFixedArrayUnmarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, ppmemory: *mut *mut u8, pformat: *mut u8, fmustalloc: u8) -> *mut u8 {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn NdrFixedArrayUnmarshall ( pstubmsg : *mut MIDL_STUB_MESSAGE , ppmemory : *mut *mut u8 , pformat : *mut u8 , fmustalloc : u8 ) -> *mut u8 );
    NdrFixedArrayUnmarshall(pstubmsg, ppmemory, pformat, fmustalloc)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrFreeBuffer(pstubmsg: *mut MIDL_STUB_MESSAGE) {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn NdrFreeBuffer ( pstubmsg : *mut MIDL_STUB_MESSAGE ) -> ( ) );
    NdrFreeBuffer(pstubmsg)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn NdrFullPointerXlatFree(pxlattables: *mut FULL_PTR_XLAT_TABLES) {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn NdrFullPointerXlatFree ( pxlattables : *mut FULL_PTR_XLAT_TABLES ) -> ( ) );
    NdrFullPointerXlatFree(pxlattables)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn NdrFullPointerXlatInit(numberofpointers: u32, xlatside: XLAT_SIDE) -> *mut FULL_PTR_XLAT_TABLES {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn NdrFullPointerXlatInit ( numberofpointers : u32 , xlatside : XLAT_SIDE ) -> *mut FULL_PTR_XLAT_TABLES );
    NdrFullPointerXlatInit(numberofpointers, xlatside)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrGetBuffer(pstubmsg: *mut MIDL_STUB_MESSAGE, bufferlength: u32, handle: *mut ::core::ffi::c_void) -> *mut u8 {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn NdrGetBuffer ( pstubmsg : *mut MIDL_STUB_MESSAGE , bufferlength : u32 , handle : *mut ::core::ffi::c_void ) -> *mut u8 );
    NdrGetBuffer(pstubmsg, bufferlength, handle)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrGetDcomProtocolVersion(pstubmsg: *mut MIDL_STUB_MESSAGE, pversion: *mut RPC_VERSION) -> ::windows::core::Result<()> {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn NdrGetDcomProtocolVersion ( pstubmsg : *mut MIDL_STUB_MESSAGE , pversion : *mut RPC_VERSION ) -> :: windows::core::HRESULT );
    NdrGetDcomProtocolVersion(pstubmsg, pversion).ok()
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrGetUserMarshalInfo(pflags: *const u32, informationlevel: u32, pmarshalinfo: *mut NDR_USER_MARSHAL_INFO) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn NdrGetUserMarshalInfo ( pflags : *const u32 , informationlevel : u32 , pmarshalinfo : *mut NDR_USER_MARSHAL_INFO ) -> RPC_STATUS );
    NdrGetUserMarshalInfo(pflags, informationlevel, pmarshalinfo)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrInterfacePointerBufferSize(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn NdrInterfacePointerBufferSize ( pstubmsg : *mut MIDL_STUB_MESSAGE , pmemory : *mut u8 , pformat : *mut u8 ) -> ( ) );
    NdrInterfacePointerBufferSize(pstubmsg, pmemory, pformat)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrInterfacePointerFree(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn NdrInterfacePointerFree ( pstubmsg : *mut MIDL_STUB_MESSAGE , pmemory : *mut u8 , pformat : *mut u8 ) -> ( ) );
    NdrInterfacePointerFree(pstubmsg, pmemory, pformat)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrInterfacePointerMarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) -> *mut u8 {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn NdrInterfacePointerMarshall ( pstubmsg : *mut MIDL_STUB_MESSAGE , pmemory : *mut u8 , pformat : *mut u8 ) -> *mut u8 );
    NdrInterfacePointerMarshall(pstubmsg, pmemory, pformat)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrInterfacePointerMemorySize(pstubmsg: *mut MIDL_STUB_MESSAGE, pformat: *mut u8) -> u32 {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn NdrInterfacePointerMemorySize ( pstubmsg : *mut MIDL_STUB_MESSAGE , pformat : *mut u8 ) -> u32 );
    NdrInterfacePointerMemorySize(pstubmsg, pformat)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrInterfacePointerUnmarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, ppmemory: *mut *mut u8, pformat: *mut u8, fmustalloc: u8) -> *mut u8 {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn NdrInterfacePointerUnmarshall ( pstubmsg : *mut MIDL_STUB_MESSAGE , ppmemory : *mut *mut u8 , pformat : *mut u8 , fmustalloc : u8 ) -> *mut u8 );
    NdrInterfacePointerUnmarshall(pstubmsg, ppmemory, pformat, fmustalloc)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrMapCommAndFaultStatus(pstubmsg: *mut MIDL_STUB_MESSAGE, pcommstatus: *mut u32, pfaultstatus: *mut u32, status: RPC_STATUS) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn NdrMapCommAndFaultStatus ( pstubmsg : *mut MIDL_STUB_MESSAGE , pcommstatus : *mut u32 , pfaultstatus : *mut u32 , status : RPC_STATUS ) -> RPC_STATUS );
    NdrMapCommAndFaultStatus(pstubmsg, pcommstatus, pfaultstatus, status)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrMesProcEncodeDecode(handle: *mut ::core::ffi::c_void, pstubdesc: *const MIDL_STUB_DESC, pformatstring: *mut u8) {
    ::windows::imp::link ! ( "rpcrt4.dll""cdecl" fn NdrMesProcEncodeDecode ( handle : *mut ::core::ffi::c_void , pstubdesc : *const MIDL_STUB_DESC , pformatstring : *mut u8 ) -> ( ) );
    NdrMesProcEncodeDecode(handle, pstubdesc, pformatstring)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrMesProcEncodeDecode2(handle: *mut ::core::ffi::c_void, pstubdesc: *const MIDL_STUB_DESC, pformatstring: *mut u8) -> CLIENT_CALL_RETURN {
    ::windows::imp::link ! ( "rpcrt4.dll""cdecl" fn NdrMesProcEncodeDecode2 ( handle : *mut ::core::ffi::c_void , pstubdesc : *const MIDL_STUB_DESC , pformatstring : *mut u8 ) -> CLIENT_CALL_RETURN );
    NdrMesProcEncodeDecode2(handle, pstubdesc, pformatstring)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrMesProcEncodeDecode3(handle: *mut ::core::ffi::c_void, pproxyinfo: *const MIDL_STUBLESS_PROXY_INFO, nprocnum: u32, preturnvalue: *mut ::core::ffi::c_void) -> CLIENT_CALL_RETURN {
    ::windows::imp::link ! ( "rpcrt4.dll""cdecl" fn NdrMesProcEncodeDecode3 ( handle : *mut ::core::ffi::c_void , pproxyinfo : *const MIDL_STUBLESS_PROXY_INFO , nprocnum : u32 , preturnvalue : *mut ::core::ffi::c_void ) -> CLIENT_CALL_RETURN );
    NdrMesProcEncodeDecode3(handle, pproxyinfo, nprocnum, preturnvalue)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn NdrMesSimpleTypeAlignSize(param0: *mut ::core::ffi::c_void) -> usize {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn NdrMesSimpleTypeAlignSize ( param0 : *mut ::core::ffi::c_void ) -> usize );
    NdrMesSimpleTypeAlignSize(param0)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrMesSimpleTypeAlignSizeAll(handle: *mut ::core::ffi::c_void, pproxyinfo: *const MIDL_STUBLESS_PROXY_INFO) -> usize {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn NdrMesSimpleTypeAlignSizeAll ( handle : *mut ::core::ffi::c_void , pproxyinfo : *const MIDL_STUBLESS_PROXY_INFO ) -> usize );
    NdrMesSimpleTypeAlignSizeAll(handle, pproxyinfo)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn NdrMesSimpleTypeDecode(handle: *mut ::core::ffi::c_void, pobject: *mut ::core::ffi::c_void, size: i16) {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn NdrMesSimpleTypeDecode ( handle : *mut ::core::ffi::c_void , pobject : *mut ::core::ffi::c_void , size : i16 ) -> ( ) );
    NdrMesSimpleTypeDecode(handle, pobject, size)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrMesSimpleTypeDecodeAll(handle: *mut ::core::ffi::c_void, pproxyinfo: *const MIDL_STUBLESS_PROXY_INFO, pobject: *mut ::core::ffi::c_void, size: i16) {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn NdrMesSimpleTypeDecodeAll ( handle : *mut ::core::ffi::c_void , pproxyinfo : *const MIDL_STUBLESS_PROXY_INFO , pobject : *mut ::core::ffi::c_void , size : i16 ) -> ( ) );
    NdrMesSimpleTypeDecodeAll(handle, pproxyinfo, pobject, size)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrMesSimpleTypeEncode(handle: *mut ::core::ffi::c_void, pstubdesc: *const MIDL_STUB_DESC, pobject: *const ::core::ffi::c_void, size: i16) {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn NdrMesSimpleTypeEncode ( handle : *mut ::core::ffi::c_void , pstubdesc : *const MIDL_STUB_DESC , pobject : *const ::core::ffi::c_void , size : i16 ) -> ( ) );
    NdrMesSimpleTypeEncode(handle, pstubdesc, pobject, size)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrMesSimpleTypeEncodeAll(handle: *mut ::core::ffi::c_void, pproxyinfo: *const MIDL_STUBLESS_PROXY_INFO, pobject: *const ::core::ffi::c_void, size: i16) {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn NdrMesSimpleTypeEncodeAll ( handle : *mut ::core::ffi::c_void , pproxyinfo : *const MIDL_STUBLESS_PROXY_INFO , pobject : *const ::core::ffi::c_void , size : i16 ) -> ( ) );
    NdrMesSimpleTypeEncodeAll(handle, pproxyinfo, pobject, size)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrMesTypeAlignSize(handle: *mut ::core::ffi::c_void, pstubdesc: *const MIDL_STUB_DESC, pformatstring: *mut u8, pobject: *const ::core::ffi::c_void) -> usize {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn NdrMesTypeAlignSize ( handle : *mut ::core::ffi::c_void , pstubdesc : *const MIDL_STUB_DESC , pformatstring : *mut u8 , pobject : *const ::core::ffi::c_void ) -> usize );
    NdrMesTypeAlignSize(handle, pstubdesc, pformatstring, pobject)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrMesTypeAlignSize2(handle: *mut ::core::ffi::c_void, ppicklinginfo: *const MIDL_TYPE_PICKLING_INFO, pstubdesc: *const MIDL_STUB_DESC, pformatstring: *mut u8, pobject: *const ::core::ffi::c_void) -> usize {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn NdrMesTypeAlignSize2 ( handle : *mut ::core::ffi::c_void , ppicklinginfo : *const MIDL_TYPE_PICKLING_INFO , pstubdesc : *const MIDL_STUB_DESC , pformatstring : *mut u8 , pobject : *const ::core::ffi::c_void ) -> usize );
    NdrMesTypeAlignSize2(handle, ppicklinginfo, pstubdesc, pformatstring, pobject)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrMesTypeAlignSize3(handle: *mut ::core::ffi::c_void, ppicklinginfo: *const MIDL_TYPE_PICKLING_INFO, pproxyinfo: *const MIDL_STUBLESS_PROXY_INFO, arrtypeoffset: *const *const u32, ntypeindex: u32, pobject: *const ::core::ffi::c_void) -> usize {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn NdrMesTypeAlignSize3 ( handle : *mut ::core::ffi::c_void , ppicklinginfo : *const MIDL_TYPE_PICKLING_INFO , pproxyinfo : *const MIDL_STUBLESS_PROXY_INFO , arrtypeoffset : *const *const u32 , ntypeindex : u32 , pobject : *const ::core::ffi::c_void ) -> usize );
    NdrMesTypeAlignSize3(handle, ppicklinginfo, pproxyinfo, arrtypeoffset, ntypeindex, pobject)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrMesTypeDecode(handle: *mut ::core::ffi::c_void, pstubdesc: *const MIDL_STUB_DESC, pformatstring: *mut u8, pobject: *mut ::core::ffi::c_void) {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn NdrMesTypeDecode ( handle : *mut ::core::ffi::c_void , pstubdesc : *const MIDL_STUB_DESC , pformatstring : *mut u8 , pobject : *mut ::core::ffi::c_void ) -> ( ) );
    NdrMesTypeDecode(handle, pstubdesc, pformatstring, pobject)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrMesTypeDecode2(handle: *mut ::core::ffi::c_void, ppicklinginfo: *const MIDL_TYPE_PICKLING_INFO, pstubdesc: *const MIDL_STUB_DESC, pformatstring: *mut u8, pobject: *mut ::core::ffi::c_void) {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn NdrMesTypeDecode2 ( handle : *mut ::core::ffi::c_void , ppicklinginfo : *const MIDL_TYPE_PICKLING_INFO , pstubdesc : *const MIDL_STUB_DESC , pformatstring : *mut u8 , pobject : *mut ::core::ffi::c_void ) -> ( ) );
    NdrMesTypeDecode2(handle, ppicklinginfo, pstubdesc, pformatstring, pobject)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrMesTypeDecode3(handle: *mut ::core::ffi::c_void, ppicklinginfo: *const MIDL_TYPE_PICKLING_INFO, pproxyinfo: *const MIDL_STUBLESS_PROXY_INFO, arrtypeoffset: *const *const u32, ntypeindex: u32, pobject: *mut ::core::ffi::c_void) {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn NdrMesTypeDecode3 ( handle : *mut ::core::ffi::c_void , ppicklinginfo : *const MIDL_TYPE_PICKLING_INFO , pproxyinfo : *const MIDL_STUBLESS_PROXY_INFO , arrtypeoffset : *const *const u32 , ntypeindex : u32 , pobject : *mut ::core::ffi::c_void ) -> ( ) );
    NdrMesTypeDecode3(handle, ppicklinginfo, pproxyinfo, arrtypeoffset, ntypeindex, pobject)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrMesTypeEncode(handle: *mut ::core::ffi::c_void, pstubdesc: *const MIDL_STUB_DESC, pformatstring: *mut u8, pobject: *const ::core::ffi::c_void) {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn NdrMesTypeEncode ( handle : *mut ::core::ffi::c_void , pstubdesc : *const MIDL_STUB_DESC , pformatstring : *mut u8 , pobject : *const ::core::ffi::c_void ) -> ( ) );
    NdrMesTypeEncode(handle, pstubdesc, pformatstring, pobject)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrMesTypeEncode2(handle: *mut ::core::ffi::c_void, ppicklinginfo: *const MIDL_TYPE_PICKLING_INFO, pstubdesc: *const MIDL_STUB_DESC, pformatstring: *mut u8, pobject: *const ::core::ffi::c_void) {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn NdrMesTypeEncode2 ( handle : *mut ::core::ffi::c_void , ppicklinginfo : *const MIDL_TYPE_PICKLING_INFO , pstubdesc : *const MIDL_STUB_DESC , pformatstring : *mut u8 , pobject : *const ::core::ffi::c_void ) -> ( ) );
    NdrMesTypeEncode2(handle, ppicklinginfo, pstubdesc, pformatstring, pobject)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrMesTypeEncode3(handle: *mut ::core::ffi::c_void, ppicklinginfo: *const MIDL_TYPE_PICKLING_INFO, pproxyinfo: *const MIDL_STUBLESS_PROXY_INFO, arrtypeoffset: *const *const u32, ntypeindex: u32, pobject: *const ::core::ffi::c_void) {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn NdrMesTypeEncode3 ( handle : *mut ::core::ffi::c_void , ppicklinginfo : *const MIDL_TYPE_PICKLING_INFO , pproxyinfo : *const MIDL_STUBLESS_PROXY_INFO , arrtypeoffset : *const *const u32 , ntypeindex : u32 , pobject : *const ::core::ffi::c_void ) -> ( ) );
    NdrMesTypeEncode3(handle, ppicklinginfo, pproxyinfo, arrtypeoffset, ntypeindex, pobject)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrMesTypeFree2(handle: *mut ::core::ffi::c_void, ppicklinginfo: *const MIDL_TYPE_PICKLING_INFO, pstubdesc: *const MIDL_STUB_DESC, pformatstring: *mut u8, pobject: *mut ::core::ffi::c_void) {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn NdrMesTypeFree2 ( handle : *mut ::core::ffi::c_void , ppicklinginfo : *const MIDL_TYPE_PICKLING_INFO , pstubdesc : *const MIDL_STUB_DESC , pformatstring : *mut u8 , pobject : *mut ::core::ffi::c_void ) -> ( ) );
    NdrMesTypeFree2(handle, ppicklinginfo, pstubdesc, pformatstring, pobject)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrMesTypeFree3(handle: *mut ::core::ffi::c_void, ppicklinginfo: *const MIDL_TYPE_PICKLING_INFO, pproxyinfo: *const MIDL_STUBLESS_PROXY_INFO, arrtypeoffset: *const *const u32, ntypeindex: u32, pobject: *mut ::core::ffi::c_void) {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn NdrMesTypeFree3 ( handle : *mut ::core::ffi::c_void , ppicklinginfo : *const MIDL_TYPE_PICKLING_INFO , pproxyinfo : *const MIDL_STUBLESS_PROXY_INFO , arrtypeoffset : *const *const u32 , ntypeindex : u32 , pobject : *mut ::core::ffi::c_void ) -> ( ) );
    NdrMesTypeFree3(handle, ppicklinginfo, pproxyinfo, arrtypeoffset, ntypeindex, pobject)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrNonConformantStringBufferSize(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn NdrNonConformantStringBufferSize ( pstubmsg : *mut MIDL_STUB_MESSAGE , pmemory : *mut u8 , pformat : *mut u8 ) -> ( ) );
    NdrNonConformantStringBufferSize(pstubmsg, pmemory, pformat)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrNonConformantStringMarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) -> *mut u8 {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn NdrNonConformantStringMarshall ( pstubmsg : *mut MIDL_STUB_MESSAGE , pmemory : *mut u8 , pformat : *mut u8 ) -> *mut u8 );
    NdrNonConformantStringMarshall(pstubmsg, pmemory, pformat)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrNonConformantStringMemorySize(pstubmsg: *mut MIDL_STUB_MESSAGE, pformat: *mut u8) -> u32 {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn NdrNonConformantStringMemorySize ( pstubmsg : *mut MIDL_STUB_MESSAGE , pformat : *mut u8 ) -> u32 );
    NdrNonConformantStringMemorySize(pstubmsg, pformat)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrNonConformantStringUnmarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, ppmemory: *mut *mut u8, pformat: *mut u8, fmustalloc: u8) -> *mut u8 {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn NdrNonConformantStringUnmarshall ( pstubmsg : *mut MIDL_STUB_MESSAGE , ppmemory : *mut *mut u8 , pformat : *mut u8 , fmustalloc : u8 ) -> *mut u8 );
    NdrNonConformantStringUnmarshall(pstubmsg, ppmemory, pformat, fmustalloc)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrNonEncapsulatedUnionBufferSize(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn NdrNonEncapsulatedUnionBufferSize ( pstubmsg : *mut MIDL_STUB_MESSAGE , pmemory : *mut u8 , pformat : *mut u8 ) -> ( ) );
    NdrNonEncapsulatedUnionBufferSize(pstubmsg, pmemory, pformat)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrNonEncapsulatedUnionFree(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn NdrNonEncapsulatedUnionFree ( pstubmsg : *mut MIDL_STUB_MESSAGE , pmemory : *mut u8 , pformat : *mut u8 ) -> ( ) );
    NdrNonEncapsulatedUnionFree(pstubmsg, pmemory, pformat)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrNonEncapsulatedUnionMarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) -> *mut u8 {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn NdrNonEncapsulatedUnionMarshall ( pstubmsg : *mut MIDL_STUB_MESSAGE , pmemory : *mut u8 , pformat : *mut u8 ) -> *mut u8 );
    NdrNonEncapsulatedUnionMarshall(pstubmsg, pmemory, pformat)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrNonEncapsulatedUnionMemorySize(pstubmsg: *mut MIDL_STUB_MESSAGE, pformat: *mut u8) -> u32 {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn NdrNonEncapsulatedUnionMemorySize ( pstubmsg : *mut MIDL_STUB_MESSAGE , pformat : *mut u8 ) -> u32 );
    NdrNonEncapsulatedUnionMemorySize(pstubmsg, pformat)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrNonEncapsulatedUnionUnmarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, ppmemory: *mut *mut u8, pformat: *mut u8, fmustalloc: u8) -> *mut u8 {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn NdrNonEncapsulatedUnionUnmarshall ( pstubmsg : *mut MIDL_STUB_MESSAGE , ppmemory : *mut *mut u8 , pformat : *mut u8 , fmustalloc : u8 ) -> *mut u8 );
    NdrNonEncapsulatedUnionUnmarshall(pstubmsg, ppmemory, pformat, fmustalloc)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrNsGetBuffer(pstubmsg: *mut MIDL_STUB_MESSAGE, bufferlength: u32, handle: *mut ::core::ffi::c_void) -> *mut u8 {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn NdrNsGetBuffer ( pstubmsg : *mut MIDL_STUB_MESSAGE , bufferlength : u32 , handle : *mut ::core::ffi::c_void ) -> *mut u8 );
    NdrNsGetBuffer(pstubmsg, bufferlength, handle)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrNsSendReceive(pstubmsg: *mut MIDL_STUB_MESSAGE, pbufferend: *mut u8, pautohandle: *mut *mut ::core::ffi::c_void) -> *mut u8 {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn NdrNsSendReceive ( pstubmsg : *mut MIDL_STUB_MESSAGE , pbufferend : *mut u8 , pautohandle : *mut *mut ::core::ffi::c_void ) -> *mut u8 );
    NdrNsSendReceive(pstubmsg, pbufferend, pautohandle)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn NdrOleAllocate(size: usize) -> *mut ::core::ffi::c_void {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn NdrOleAllocate ( size : usize ) -> *mut ::core::ffi::c_void );
    NdrOleAllocate(size)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn NdrOleFree(nodetofree: *const ::core::ffi::c_void) {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn NdrOleFree ( nodetofree : *const ::core::ffi::c_void ) -> ( ) );
    NdrOleFree(nodetofree)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrPartialIgnoreClientBufferSize(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut ::core::ffi::c_void) {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn NdrPartialIgnoreClientBufferSize ( pstubmsg : *mut MIDL_STUB_MESSAGE , pmemory : *mut ::core::ffi::c_void ) -> ( ) );
    NdrPartialIgnoreClientBufferSize(pstubmsg, pmemory)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrPartialIgnoreClientMarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut ::core::ffi::c_void) {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn NdrPartialIgnoreClientMarshall ( pstubmsg : *mut MIDL_STUB_MESSAGE , pmemory : *mut ::core::ffi::c_void ) -> ( ) );
    NdrPartialIgnoreClientMarshall(pstubmsg, pmemory)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrPartialIgnoreServerInitialize(pstubmsg: *mut MIDL_STUB_MESSAGE, ppmemory: *mut *mut ::core::ffi::c_void, pformat: *mut u8) {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn NdrPartialIgnoreServerInitialize ( pstubmsg : *mut MIDL_STUB_MESSAGE , ppmemory : *mut *mut ::core::ffi::c_void , pformat : *mut u8 ) -> ( ) );
    NdrPartialIgnoreServerInitialize(pstubmsg, ppmemory, pformat)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrPartialIgnoreServerUnmarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, ppmemory: *mut *mut ::core::ffi::c_void) {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn NdrPartialIgnoreServerUnmarshall ( pstubmsg : *mut MIDL_STUB_MESSAGE , ppmemory : *mut *mut ::core::ffi::c_void ) -> ( ) );
    NdrPartialIgnoreServerUnmarshall(pstubmsg, ppmemory)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrPointerBufferSize(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn NdrPointerBufferSize ( pstubmsg : *mut MIDL_STUB_MESSAGE , pmemory : *mut u8 , pformat : *mut u8 ) -> ( ) );
    NdrPointerBufferSize(pstubmsg, pmemory, pformat)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrPointerFree(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn NdrPointerFree ( pstubmsg : *mut MIDL_STUB_MESSAGE , pmemory : *mut u8 , pformat : *mut u8 ) -> ( ) );
    NdrPointerFree(pstubmsg, pmemory, pformat)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrPointerMarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) -> *mut u8 {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn NdrPointerMarshall ( pstubmsg : *mut MIDL_STUB_MESSAGE , pmemory : *mut u8 , pformat : *mut u8 ) -> *mut u8 );
    NdrPointerMarshall(pstubmsg, pmemory, pformat)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrPointerMemorySize(pstubmsg: *mut MIDL_STUB_MESSAGE, pformat: *mut u8) -> u32 {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn NdrPointerMemorySize ( pstubmsg : *mut MIDL_STUB_MESSAGE , pformat : *mut u8 ) -> u32 );
    NdrPointerMemorySize(pstubmsg, pformat)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrPointerUnmarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, ppmemory: *mut *mut u8, pformat: *mut u8, fmustalloc: u8) -> *mut u8 {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn NdrPointerUnmarshall ( pstubmsg : *mut MIDL_STUB_MESSAGE , ppmemory : *mut *mut u8 , pformat : *mut u8 , fmustalloc : u8 ) -> *mut u8 );
    NdrPointerUnmarshall(pstubmsg, ppmemory, pformat, fmustalloc)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrRangeUnmarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, ppmemory: *mut *mut u8, pformat: *mut u8, fmustalloc: u8) -> *mut u8 {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn NdrRangeUnmarshall ( pstubmsg : *mut MIDL_STUB_MESSAGE , ppmemory : *mut *mut u8 , pformat : *mut u8 , fmustalloc : u8 ) -> *mut u8 );
    NdrRangeUnmarshall(pstubmsg, ppmemory, pformat, fmustalloc)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn NdrRpcSmClientAllocate(size: usize) -> *mut ::core::ffi::c_void {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn NdrRpcSmClientAllocate ( size : usize ) -> *mut ::core::ffi::c_void );
    NdrRpcSmClientAllocate(size)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn NdrRpcSmClientFree(nodetofree: *const ::core::ffi::c_void) {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn NdrRpcSmClientFree ( nodetofree : *const ::core::ffi::c_void ) -> ( ) );
    NdrRpcSmClientFree(nodetofree)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrRpcSmSetClientToOsf(pmessage: *mut MIDL_STUB_MESSAGE) {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn NdrRpcSmSetClientToOsf ( pmessage : *mut MIDL_STUB_MESSAGE ) -> ( ) );
    NdrRpcSmSetClientToOsf(pmessage)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn NdrRpcSsDefaultAllocate(size: usize) -> *mut ::core::ffi::c_void {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn NdrRpcSsDefaultAllocate ( size : usize ) -> *mut ::core::ffi::c_void );
    NdrRpcSsDefaultAllocate(size)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn NdrRpcSsDefaultFree(nodetofree: *const ::core::ffi::c_void) {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn NdrRpcSsDefaultFree ( nodetofree : *const ::core::ffi::c_void ) -> ( ) );
    NdrRpcSsDefaultFree(nodetofree)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrRpcSsDisableAllocate(pmessage: *mut MIDL_STUB_MESSAGE) {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn NdrRpcSsDisableAllocate ( pmessage : *mut MIDL_STUB_MESSAGE ) -> ( ) );
    NdrRpcSsDisableAllocate(pmessage)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrRpcSsEnableAllocate(pmessage: *mut MIDL_STUB_MESSAGE) {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn NdrRpcSsEnableAllocate ( pmessage : *mut MIDL_STUB_MESSAGE ) -> ( ) );
    NdrRpcSsEnableAllocate(pmessage)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrSendReceive(pstubmsg: *mut MIDL_STUB_MESSAGE, pbufferend: *mut u8) -> *mut u8 {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn NdrSendReceive ( pstubmsg : *mut MIDL_STUB_MESSAGE , pbufferend : *mut u8 ) -> *mut u8 );
    NdrSendReceive(pstubmsg, pbufferend)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn NdrServerCall2(prpcmsg: *mut RPC_MESSAGE) {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn NdrServerCall2 ( prpcmsg : *mut RPC_MESSAGE ) -> ( ) );
    NdrServerCall2(prpcmsg)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn NdrServerCallAll(prpcmsg: *mut RPC_MESSAGE) {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn NdrServerCallAll ( prpcmsg : *mut RPC_MESSAGE ) -> ( ) );
    NdrServerCallAll(prpcmsg)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn NdrServerCallNdr64(prpcmsg: *mut RPC_MESSAGE) {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn NdrServerCallNdr64 ( prpcmsg : *mut RPC_MESSAGE ) -> ( ) );
    NdrServerCallNdr64(prpcmsg)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrServerContextMarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, contexthandle: *mut NDR_SCONTEXT, rundownroutine: NDR_RUNDOWN) {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn NdrServerContextMarshall ( pstubmsg : *mut MIDL_STUB_MESSAGE , contexthandle : *mut NDR_SCONTEXT , rundownroutine : NDR_RUNDOWN ) -> ( ) );
    NdrServerContextMarshall(pstubmsg, contexthandle, rundownroutine)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrServerContextNewMarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, contexthandle: *mut NDR_SCONTEXT, rundownroutine: NDR_RUNDOWN, pformat: *mut u8) {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn NdrServerContextNewMarshall ( pstubmsg : *mut MIDL_STUB_MESSAGE , contexthandle : *mut NDR_SCONTEXT , rundownroutine : NDR_RUNDOWN , pformat : *mut u8 ) -> ( ) );
    NdrServerContextNewMarshall(pstubmsg, contexthandle, rundownroutine, pformat)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrServerContextNewUnmarshall(pstubmsg: *const MIDL_STUB_MESSAGE, pformat: *const u8) -> *mut NDR_SCONTEXT {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn NdrServerContextNewUnmarshall ( pstubmsg : *const MIDL_STUB_MESSAGE , pformat : *const u8 ) -> *mut NDR_SCONTEXT );
    NdrServerContextNewUnmarshall(pstubmsg, pformat)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrServerContextUnmarshall(pstubmsg: *mut MIDL_STUB_MESSAGE) -> *mut NDR_SCONTEXT {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn NdrServerContextUnmarshall ( pstubmsg : *mut MIDL_STUB_MESSAGE ) -> *mut NDR_SCONTEXT );
    NdrServerContextUnmarshall(pstubmsg)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrServerInitialize(prpcmsg: *mut RPC_MESSAGE, pstubmsg: *mut MIDL_STUB_MESSAGE, pstubdescriptor: *mut MIDL_STUB_DESC) -> *mut u8 {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn NdrServerInitialize ( prpcmsg : *mut RPC_MESSAGE , pstubmsg : *mut MIDL_STUB_MESSAGE , pstubdescriptor : *mut MIDL_STUB_DESC ) -> *mut u8 );
    NdrServerInitialize(prpcmsg, pstubmsg, pstubdescriptor)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrServerInitializeMarshall(prpcmsg: *mut RPC_MESSAGE, pstubmsg: *mut MIDL_STUB_MESSAGE) {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn NdrServerInitializeMarshall ( prpcmsg : *mut RPC_MESSAGE , pstubmsg : *mut MIDL_STUB_MESSAGE ) -> ( ) );
    NdrServerInitializeMarshall(prpcmsg, pstubmsg)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrServerInitializeNew(prpcmsg: *mut RPC_MESSAGE, pstubmsg: *mut MIDL_STUB_MESSAGE, pstubdescriptor: *mut MIDL_STUB_DESC) -> *mut u8 {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn NdrServerInitializeNew ( prpcmsg : *mut RPC_MESSAGE , pstubmsg : *mut MIDL_STUB_MESSAGE , pstubdescriptor : *mut MIDL_STUB_DESC ) -> *mut u8 );
    NdrServerInitializeNew(prpcmsg, pstubmsg, pstubdescriptor)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrServerInitializePartial(prpcmsg: *mut RPC_MESSAGE, pstubmsg: *mut MIDL_STUB_MESSAGE, pstubdescriptor: *mut MIDL_STUB_DESC, requestedbuffersize: u32) {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn NdrServerInitializePartial ( prpcmsg : *mut RPC_MESSAGE , pstubmsg : *mut MIDL_STUB_MESSAGE , pstubdescriptor : *mut MIDL_STUB_DESC , requestedbuffersize : u32 ) -> ( ) );
    NdrServerInitializePartial(prpcmsg, pstubmsg, pstubdescriptor, requestedbuffersize)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrServerInitializeUnmarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, pstubdescriptor: *mut MIDL_STUB_DESC, prpcmsg: *mut RPC_MESSAGE) -> *mut u8 {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn NdrServerInitializeUnmarshall ( pstubmsg : *mut MIDL_STUB_MESSAGE , pstubdescriptor : *mut MIDL_STUB_DESC , prpcmsg : *mut RPC_MESSAGE ) -> *mut u8 );
    NdrServerInitializeUnmarshall(pstubmsg, pstubdescriptor, prpcmsg)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrSimpleStructBufferSize(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn NdrSimpleStructBufferSize ( pstubmsg : *mut MIDL_STUB_MESSAGE , pmemory : *mut u8 , pformat : *mut u8 ) -> ( ) );
    NdrSimpleStructBufferSize(pstubmsg, pmemory, pformat)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrSimpleStructFree(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn NdrSimpleStructFree ( pstubmsg : *mut MIDL_STUB_MESSAGE , pmemory : *mut u8 , pformat : *mut u8 ) -> ( ) );
    NdrSimpleStructFree(pstubmsg, pmemory, pformat)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrSimpleStructMarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) -> *mut u8 {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn NdrSimpleStructMarshall ( pstubmsg : *mut MIDL_STUB_MESSAGE , pmemory : *mut u8 , pformat : *mut u8 ) -> *mut u8 );
    NdrSimpleStructMarshall(pstubmsg, pmemory, pformat)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrSimpleStructMemorySize(pstubmsg: *mut MIDL_STUB_MESSAGE, pformat: *mut u8) -> u32 {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn NdrSimpleStructMemorySize ( pstubmsg : *mut MIDL_STUB_MESSAGE , pformat : *mut u8 ) -> u32 );
    NdrSimpleStructMemorySize(pstubmsg, pformat)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrSimpleStructUnmarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, ppmemory: *mut *mut u8, pformat: *mut u8, fmustalloc: u8) -> *mut u8 {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn NdrSimpleStructUnmarshall ( pstubmsg : *mut MIDL_STUB_MESSAGE , ppmemory : *mut *mut u8 , pformat : *mut u8 , fmustalloc : u8 ) -> *mut u8 );
    NdrSimpleStructUnmarshall(pstubmsg, ppmemory, pformat, fmustalloc)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrSimpleTypeMarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, formatchar: u8) {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn NdrSimpleTypeMarshall ( pstubmsg : *mut MIDL_STUB_MESSAGE , pmemory : *mut u8 , formatchar : u8 ) -> ( ) );
    NdrSimpleTypeMarshall(pstubmsg, pmemory, formatchar)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrSimpleTypeUnmarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, formatchar: u8) {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn NdrSimpleTypeUnmarshall ( pstubmsg : *mut MIDL_STUB_MESSAGE , pmemory : *mut u8 , formatchar : u8 ) -> ( ) );
    NdrSimpleTypeUnmarshall(pstubmsg, pmemory, formatchar)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn NdrStubCall2(pthis: *mut ::core::ffi::c_void, pchannel: *mut ::core::ffi::c_void, prpcmsg: *mut RPC_MESSAGE, pdwstubphase: *mut u32) -> i32 {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn NdrStubCall2 ( pthis : *mut ::core::ffi::c_void , pchannel : *mut ::core::ffi::c_void , prpcmsg : *mut RPC_MESSAGE , pdwstubphase : *mut u32 ) -> i32 );
    NdrStubCall2(pthis, pchannel, prpcmsg, pdwstubphase)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn NdrStubCall3(pthis: *mut ::core::ffi::c_void, pchannel: *mut ::core::ffi::c_void, prpcmsg: *mut RPC_MESSAGE, pdwstubphase: *mut u32) -> i32 {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn NdrStubCall3 ( pthis : *mut ::core::ffi::c_void , pchannel : *mut ::core::ffi::c_void , prpcmsg : *mut RPC_MESSAGE , pdwstubphase : *mut u32 ) -> i32 );
    NdrStubCall3(pthis, pchannel, prpcmsg, pdwstubphase)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrUserMarshalBufferSize(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn NdrUserMarshalBufferSize ( pstubmsg : *mut MIDL_STUB_MESSAGE , pmemory : *mut u8 , pformat : *mut u8 ) -> ( ) );
    NdrUserMarshalBufferSize(pstubmsg, pmemory, pformat)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrUserMarshalFree(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn NdrUserMarshalFree ( pstubmsg : *mut MIDL_STUB_MESSAGE , pmemory : *mut u8 , pformat : *mut u8 ) -> ( ) );
    NdrUserMarshalFree(pstubmsg, pmemory, pformat)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrUserMarshalMarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) -> *mut u8 {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn NdrUserMarshalMarshall ( pstubmsg : *mut MIDL_STUB_MESSAGE , pmemory : *mut u8 , pformat : *mut u8 ) -> *mut u8 );
    NdrUserMarshalMarshall(pstubmsg, pmemory, pformat)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrUserMarshalMemorySize(pstubmsg: *mut MIDL_STUB_MESSAGE, pformat: *mut u8) -> u32 {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn NdrUserMarshalMemorySize ( pstubmsg : *mut MIDL_STUB_MESSAGE , pformat : *mut u8 ) -> u32 );
    NdrUserMarshalMemorySize(pstubmsg, pformat)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn NdrUserMarshalSimpleTypeConvert(pflags: *mut u32, pbuffer: *mut u8, formatchar: u8) -> *mut u8 {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn NdrUserMarshalSimpleTypeConvert ( pflags : *mut u32 , pbuffer : *mut u8 , formatchar : u8 ) -> *mut u8 );
    NdrUserMarshalSimpleTypeConvert(pflags, pbuffer, formatchar)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrUserMarshalUnmarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, ppmemory: *mut *mut u8, pformat: *mut u8, fmustalloc: u8) -> *mut u8 {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn NdrUserMarshalUnmarshall ( pstubmsg : *mut MIDL_STUB_MESSAGE , ppmemory : *mut *mut u8 , pformat : *mut u8 , fmustalloc : u8 ) -> *mut u8 );
    NdrUserMarshalUnmarshall(pstubmsg, ppmemory, pformat, fmustalloc)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrVaryingArrayBufferSize(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn NdrVaryingArrayBufferSize ( pstubmsg : *mut MIDL_STUB_MESSAGE , pmemory : *mut u8 , pformat : *mut u8 ) -> ( ) );
    NdrVaryingArrayBufferSize(pstubmsg, pmemory, pformat)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrVaryingArrayFree(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn NdrVaryingArrayFree ( pstubmsg : *mut MIDL_STUB_MESSAGE , pmemory : *mut u8 , pformat : *mut u8 ) -> ( ) );
    NdrVaryingArrayFree(pstubmsg, pmemory, pformat)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrVaryingArrayMarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) -> *mut u8 {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn NdrVaryingArrayMarshall ( pstubmsg : *mut MIDL_STUB_MESSAGE , pmemory : *mut u8 , pformat : *mut u8 ) -> *mut u8 );
    NdrVaryingArrayMarshall(pstubmsg, pmemory, pformat)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrVaryingArrayMemorySize(pstubmsg: *mut MIDL_STUB_MESSAGE, pformat: *mut u8) -> u32 {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn NdrVaryingArrayMemorySize ( pstubmsg : *mut MIDL_STUB_MESSAGE , pformat : *mut u8 ) -> u32 );
    NdrVaryingArrayMemorySize(pstubmsg, pformat)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrVaryingArrayUnmarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, ppmemory: *mut *mut u8, pformat: *mut u8, fmustalloc: u8) -> *mut u8 {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn NdrVaryingArrayUnmarshall ( pstubmsg : *mut MIDL_STUB_MESSAGE , ppmemory : *mut *mut u8 , pformat : *mut u8 , fmustalloc : u8 ) -> *mut u8 );
    NdrVaryingArrayUnmarshall(pstubmsg, ppmemory, pformat, fmustalloc)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrXmitOrRepAsBufferSize(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn NdrXmitOrRepAsBufferSize ( pstubmsg : *mut MIDL_STUB_MESSAGE , pmemory : *mut u8 , pformat : *mut u8 ) -> ( ) );
    NdrXmitOrRepAsBufferSize(pstubmsg, pmemory, pformat)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrXmitOrRepAsFree(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn NdrXmitOrRepAsFree ( pstubmsg : *mut MIDL_STUB_MESSAGE , pmemory : *mut u8 , pformat : *mut u8 ) -> ( ) );
    NdrXmitOrRepAsFree(pstubmsg, pmemory, pformat)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrXmitOrRepAsMarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) -> *mut u8 {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn NdrXmitOrRepAsMarshall ( pstubmsg : *mut MIDL_STUB_MESSAGE , pmemory : *mut u8 , pformat : *mut u8 ) -> *mut u8 );
    NdrXmitOrRepAsMarshall(pstubmsg, pmemory, pformat)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrXmitOrRepAsMemorySize(pstubmsg: *mut MIDL_STUB_MESSAGE, pformat: *mut u8) -> u32 {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn NdrXmitOrRepAsMemorySize ( pstubmsg : *mut MIDL_STUB_MESSAGE , pformat : *mut u8 ) -> u32 );
    NdrXmitOrRepAsMemorySize(pstubmsg, pformat)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrXmitOrRepAsUnmarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, ppmemory: *mut *mut u8, pformat: *mut u8, fmustalloc: u8) -> *mut u8 {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn NdrXmitOrRepAsUnmarshall ( pstubmsg : *mut MIDL_STUB_MESSAGE , ppmemory : *mut *mut u8 , pformat : *mut u8 , fmustalloc : u8 ) -> *mut u8 );
    NdrXmitOrRepAsUnmarshall(pstubmsg, ppmemory, pformat, fmustalloc)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[inline]
pub unsafe fn RpcAsyncAbortCall(pasync: *mut RPC_ASYNC_STATE, exceptioncode: u32) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn RpcAsyncAbortCall ( pasync : *mut RPC_ASYNC_STATE , exceptioncode : u32 ) -> RPC_STATUS );
    RpcAsyncAbortCall(pasync, exceptioncode)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[inline]
pub unsafe fn RpcAsyncCancelCall<P0>(pasync: *mut RPC_ASYNC_STATE, fabort: P0) -> RPC_STATUS
where
    P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
{
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn RpcAsyncCancelCall ( pasync : *mut RPC_ASYNC_STATE , fabort : super::super::Foundation:: BOOL ) -> RPC_STATUS );
    RpcAsyncCancelCall(pasync, fabort.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[inline]
pub unsafe fn RpcAsyncCompleteCall(pasync: *mut RPC_ASYNC_STATE, reply: ::core::option::Option<*mut ::core::ffi::c_void>) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn RpcAsyncCompleteCall ( pasync : *mut RPC_ASYNC_STATE , reply : *mut ::core::ffi::c_void ) -> RPC_STATUS );
    RpcAsyncCompleteCall(pasync, ::core::mem::transmute(reply.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[inline]
pub unsafe fn RpcAsyncGetCallStatus(pasync: *const RPC_ASYNC_STATE) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn RpcAsyncGetCallStatus ( pasync : *const RPC_ASYNC_STATE ) -> RPC_STATUS );
    RpcAsyncGetCallStatus(pasync)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[inline]
pub unsafe fn RpcAsyncInitializeHandle(pasync: *mut RPC_ASYNC_STATE, size: u32) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn RpcAsyncInitializeHandle ( pasync : *mut RPC_ASYNC_STATE , size : u32 ) -> RPC_STATUS );
    RpcAsyncInitializeHandle(pasync, size)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[inline]
pub unsafe fn RpcAsyncRegisterInfo(pasync: *const RPC_ASYNC_STATE) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn RpcAsyncRegisterInfo ( pasync : *const RPC_ASYNC_STATE ) -> RPC_STATUS );
    RpcAsyncRegisterInfo(pasync)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[inline]
pub unsafe fn RpcBindingBind(pasync: ::core::option::Option<*const RPC_ASYNC_STATE>, binding: *const ::core::ffi::c_void, ifspec: *const ::core::ffi::c_void) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn RpcBindingBind ( pasync : *const RPC_ASYNC_STATE , binding : *const ::core::ffi::c_void , ifspec : *const ::core::ffi::c_void ) -> RPC_STATUS );
    RpcBindingBind(::core::mem::transmute(pasync.unwrap_or(::std::ptr::null())), binding, ifspec)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcBindingCopy(sourcebinding: *const ::core::ffi::c_void, destinationbinding: *mut *mut ::core::ffi::c_void) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn RpcBindingCopy ( sourcebinding : *const ::core::ffi::c_void , destinationbinding : *mut *mut ::core::ffi::c_void ) -> RPC_STATUS );
    RpcBindingCopy(sourcebinding, destinationbinding)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn RpcBindingCreateA(template: *const RPC_BINDING_HANDLE_TEMPLATE_V1_A, security: ::core::option::Option<*const RPC_BINDING_HANDLE_SECURITY_V1_A>, options: ::core::option::Option<*const RPC_BINDING_HANDLE_OPTIONS_V1>, binding: *mut *mut ::core::ffi::c_void) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn RpcBindingCreateA ( template : *const RPC_BINDING_HANDLE_TEMPLATE_V1_A , security : *const RPC_BINDING_HANDLE_SECURITY_V1_A , options : *const RPC_BINDING_HANDLE_OPTIONS_V1 , binding : *mut *mut ::core::ffi::c_void ) -> RPC_STATUS );
    RpcBindingCreateA(template, ::core::mem::transmute(security.unwrap_or(::std::ptr::null())), ::core::mem::transmute(options.unwrap_or(::std::ptr::null())), binding)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn RpcBindingCreateW(template: *const RPC_BINDING_HANDLE_TEMPLATE_V1_W, security: ::core::option::Option<*const RPC_BINDING_HANDLE_SECURITY_V1_W>, options: ::core::option::Option<*const RPC_BINDING_HANDLE_OPTIONS_V1>, binding: *mut *mut ::core::ffi::c_void) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn RpcBindingCreateW ( template : *const RPC_BINDING_HANDLE_TEMPLATE_V1_W , security : *const RPC_BINDING_HANDLE_SECURITY_V1_W , options : *const RPC_BINDING_HANDLE_OPTIONS_V1 , binding : *mut *mut ::core::ffi::c_void ) -> RPC_STATUS );
    RpcBindingCreateW(template, ::core::mem::transmute(security.unwrap_or(::std::ptr::null())), ::core::mem::transmute(options.unwrap_or(::std::ptr::null())), binding)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcBindingFree(binding: *mut *mut ::core::ffi::c_void) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn RpcBindingFree ( binding : *mut *mut ::core::ffi::c_void ) -> RPC_STATUS );
    RpcBindingFree(binding)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcBindingFromStringBindingA(stringbinding: *const u8, binding: *mut *mut ::core::ffi::c_void) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn RpcBindingFromStringBindingA ( stringbinding : *const u8 , binding : *mut *mut ::core::ffi::c_void ) -> RPC_STATUS );
    RpcBindingFromStringBindingA(stringbinding, binding)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcBindingFromStringBindingW(stringbinding: *const u16, binding: *mut *mut ::core::ffi::c_void) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn RpcBindingFromStringBindingW ( stringbinding : *const u16 , binding : *mut *mut ::core::ffi::c_void ) -> RPC_STATUS );
    RpcBindingFromStringBindingW(stringbinding, binding)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcBindingInqAuthClientA(clientbinding: ::core::option::Option<*const ::core::ffi::c_void>, privs: *mut *mut ::core::ffi::c_void, serverprincname: ::core::option::Option<*mut *mut u8>, authnlevel: ::core::option::Option<*mut u32>, authnsvc: ::core::option::Option<*mut u32>, authzsvc: ::core::option::Option<*mut u32>) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn RpcBindingInqAuthClientA ( clientbinding : *const ::core::ffi::c_void , privs : *mut *mut ::core::ffi::c_void , serverprincname : *mut *mut u8 , authnlevel : *mut u32 , authnsvc : *mut u32 , authzsvc : *mut u32 ) -> RPC_STATUS );
    RpcBindingInqAuthClientA(::core::mem::transmute(clientbinding.unwrap_or(::std::ptr::null())), privs, ::core::mem::transmute(serverprincname.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(authnlevel.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(authnsvc.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(authzsvc.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcBindingInqAuthClientExA(clientbinding: ::core::option::Option<*const ::core::ffi::c_void>, privs: *mut *mut ::core::ffi::c_void, serverprincname: ::core::option::Option<*mut *mut u8>, authnlevel: ::core::option::Option<*mut u32>, authnsvc: ::core::option::Option<*mut u32>, authzsvc: ::core::option::Option<*mut u32>, flags: u32) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn RpcBindingInqAuthClientExA ( clientbinding : *const ::core::ffi::c_void , privs : *mut *mut ::core::ffi::c_void , serverprincname : *mut *mut u8 , authnlevel : *mut u32 , authnsvc : *mut u32 , authzsvc : *mut u32 , flags : u32 ) -> RPC_STATUS );
    RpcBindingInqAuthClientExA(::core::mem::transmute(clientbinding.unwrap_or(::std::ptr::null())), privs, ::core::mem::transmute(serverprincname.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(authnlevel.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(authnsvc.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(authzsvc.unwrap_or(::std::ptr::null_mut())), flags)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcBindingInqAuthClientExW(clientbinding: ::core::option::Option<*const ::core::ffi::c_void>, privs: *mut *mut ::core::ffi::c_void, serverprincname: ::core::option::Option<*mut *mut u16>, authnlevel: ::core::option::Option<*mut u32>, authnsvc: ::core::option::Option<*mut u32>, authzsvc: ::core::option::Option<*mut u32>, flags: u32) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn RpcBindingInqAuthClientExW ( clientbinding : *const ::core::ffi::c_void , privs : *mut *mut ::core::ffi::c_void , serverprincname : *mut *mut u16 , authnlevel : *mut u32 , authnsvc : *mut u32 , authzsvc : *mut u32 , flags : u32 ) -> RPC_STATUS );
    RpcBindingInqAuthClientExW(::core::mem::transmute(clientbinding.unwrap_or(::std::ptr::null())), privs, ::core::mem::transmute(serverprincname.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(authnlevel.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(authnsvc.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(authzsvc.unwrap_or(::std::ptr::null_mut())), flags)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcBindingInqAuthClientW(clientbinding: ::core::option::Option<*const ::core::ffi::c_void>, privs: *mut *mut ::core::ffi::c_void, serverprincname: ::core::option::Option<*mut *mut u16>, authnlevel: ::core::option::Option<*mut u32>, authnsvc: ::core::option::Option<*mut u32>, authzsvc: ::core::option::Option<*mut u32>) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn RpcBindingInqAuthClientW ( clientbinding : *const ::core::ffi::c_void , privs : *mut *mut ::core::ffi::c_void , serverprincname : *mut *mut u16 , authnlevel : *mut u32 , authnsvc : *mut u32 , authzsvc : *mut u32 ) -> RPC_STATUS );
    RpcBindingInqAuthClientW(::core::mem::transmute(clientbinding.unwrap_or(::std::ptr::null())), privs, ::core::mem::transmute(serverprincname.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(authnlevel.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(authnsvc.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(authzsvc.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcBindingInqAuthInfoA(binding: *const ::core::ffi::c_void, serverprincname: ::core::option::Option<*mut *mut u8>, authnlevel: ::core::option::Option<*mut u32>, authnsvc: ::core::option::Option<*mut u32>, authidentity: ::core::option::Option<*mut *mut ::core::ffi::c_void>, authzsvc: ::core::option::Option<*mut u32>) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn RpcBindingInqAuthInfoA ( binding : *const ::core::ffi::c_void , serverprincname : *mut *mut u8 , authnlevel : *mut u32 , authnsvc : *mut u32 , authidentity : *mut *mut ::core::ffi::c_void , authzsvc : *mut u32 ) -> RPC_STATUS );
    RpcBindingInqAuthInfoA(binding, ::core::mem::transmute(serverprincname.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(authnlevel.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(authnsvc.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(authidentity.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(authzsvc.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn RpcBindingInqAuthInfoExA(binding: *const ::core::ffi::c_void, serverprincname: ::core::option::Option<*mut *mut u8>, authnlevel: ::core::option::Option<*mut u32>, authnsvc: ::core::option::Option<*mut u32>, authidentity: ::core::option::Option<*mut *mut ::core::ffi::c_void>, authzsvc: ::core::option::Option<*mut u32>, rpcqosversion: u32, securityqos: ::core::option::Option<*mut RPC_SECURITY_QOS>) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn RpcBindingInqAuthInfoExA ( binding : *const ::core::ffi::c_void , serverprincname : *mut *mut u8 , authnlevel : *mut u32 , authnsvc : *mut u32 , authidentity : *mut *mut ::core::ffi::c_void , authzsvc : *mut u32 , rpcqosversion : u32 , securityqos : *mut RPC_SECURITY_QOS ) -> RPC_STATUS );
    RpcBindingInqAuthInfoExA(binding, ::core::mem::transmute(serverprincname.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(authnlevel.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(authnsvc.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(authidentity.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(authzsvc.unwrap_or(::std::ptr::null_mut())), rpcqosversion, ::core::mem::transmute(securityqos.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn RpcBindingInqAuthInfoExW(binding: *const ::core::ffi::c_void, serverprincname: ::core::option::Option<*mut *mut u16>, authnlevel: ::core::option::Option<*mut u32>, authnsvc: ::core::option::Option<*mut u32>, authidentity: ::core::option::Option<*mut *mut ::core::ffi::c_void>, authzsvc: ::core::option::Option<*mut u32>, rpcqosversion: u32, securityqos: ::core::option::Option<*mut RPC_SECURITY_QOS>) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn RpcBindingInqAuthInfoExW ( binding : *const ::core::ffi::c_void , serverprincname : *mut *mut u16 , authnlevel : *mut u32 , authnsvc : *mut u32 , authidentity : *mut *mut ::core::ffi::c_void , authzsvc : *mut u32 , rpcqosversion : u32 , securityqos : *mut RPC_SECURITY_QOS ) -> RPC_STATUS );
    RpcBindingInqAuthInfoExW(binding, ::core::mem::transmute(serverprincname.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(authnlevel.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(authnsvc.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(authidentity.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(authzsvc.unwrap_or(::std::ptr::null_mut())), rpcqosversion, ::core::mem::transmute(securityqos.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcBindingInqAuthInfoW(binding: *const ::core::ffi::c_void, serverprincname: ::core::option::Option<*mut *mut u16>, authnlevel: ::core::option::Option<*mut u32>, authnsvc: ::core::option::Option<*mut u32>, authidentity: ::core::option::Option<*mut *mut ::core::ffi::c_void>, authzsvc: ::core::option::Option<*mut u32>) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn RpcBindingInqAuthInfoW ( binding : *const ::core::ffi::c_void , serverprincname : *mut *mut u16 , authnlevel : *mut u32 , authnsvc : *mut u32 , authidentity : *mut *mut ::core::ffi::c_void , authzsvc : *mut u32 ) -> RPC_STATUS );
    RpcBindingInqAuthInfoW(binding, ::core::mem::transmute(serverprincname.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(authnlevel.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(authnsvc.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(authidentity.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(authzsvc.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcBindingInqMaxCalls(binding: *const ::core::ffi::c_void, maxcalls: *mut u32) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn RpcBindingInqMaxCalls ( binding : *const ::core::ffi::c_void , maxcalls : *mut u32 ) -> RPC_STATUS );
    RpcBindingInqMaxCalls(binding, maxcalls)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcBindingInqObject(binding: *const ::core::ffi::c_void, objectuuid: *mut ::windows::core::GUID) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn RpcBindingInqObject ( binding : *const ::core::ffi::c_void , objectuuid : *mut :: windows::core::GUID ) -> RPC_STATUS );
    RpcBindingInqObject(binding, objectuuid)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcBindingInqOption(hbinding: *const ::core::ffi::c_void, option: u32, poptionvalue: *mut usize) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn RpcBindingInqOption ( hbinding : *const ::core::ffi::c_void , option : u32 , poptionvalue : *mut usize ) -> RPC_STATUS );
    RpcBindingInqOption(hbinding, option, poptionvalue)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcBindingReset(binding: *const ::core::ffi::c_void) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn RpcBindingReset ( binding : *const ::core::ffi::c_void ) -> RPC_STATUS );
    RpcBindingReset(binding)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcBindingServerFromClient(clientbinding: ::core::option::Option<*const ::core::ffi::c_void>, serverbinding: *mut *mut ::core::ffi::c_void) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn RpcBindingServerFromClient ( clientbinding : *const ::core::ffi::c_void , serverbinding : *mut *mut ::core::ffi::c_void ) -> RPC_STATUS );
    RpcBindingServerFromClient(::core::mem::transmute(clientbinding.unwrap_or(::std::ptr::null())), serverbinding)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcBindingSetAuthInfoA(binding: *const ::core::ffi::c_void, serverprincname: ::core::option::Option<*const u8>, authnlevel: u32, authnsvc: u32, authidentity: ::core::option::Option<*const ::core::ffi::c_void>, authzsvc: u32) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn RpcBindingSetAuthInfoA ( binding : *const ::core::ffi::c_void , serverprincname : *const u8 , authnlevel : u32 , authnsvc : u32 , authidentity : *const ::core::ffi::c_void , authzsvc : u32 ) -> RPC_STATUS );
    RpcBindingSetAuthInfoA(binding, ::core::mem::transmute(serverprincname.unwrap_or(::std::ptr::null())), authnlevel, authnsvc, ::core::mem::transmute(authidentity.unwrap_or(::std::ptr::null())), authzsvc)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn RpcBindingSetAuthInfoExA(binding: *const ::core::ffi::c_void, serverprincname: ::core::option::Option<*const u8>, authnlevel: u32, authnsvc: u32, authidentity: ::core::option::Option<*const ::core::ffi::c_void>, authzsvc: u32, securityqos: ::core::option::Option<*const RPC_SECURITY_QOS>) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn RpcBindingSetAuthInfoExA ( binding : *const ::core::ffi::c_void , serverprincname : *const u8 , authnlevel : u32 , authnsvc : u32 , authidentity : *const ::core::ffi::c_void , authzsvc : u32 , securityqos : *const RPC_SECURITY_QOS ) -> RPC_STATUS );
    RpcBindingSetAuthInfoExA(binding, ::core::mem::transmute(serverprincname.unwrap_or(::std::ptr::null())), authnlevel, authnsvc, ::core::mem::transmute(authidentity.unwrap_or(::std::ptr::null())), authzsvc, ::core::mem::transmute(securityqos.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn RpcBindingSetAuthInfoExW(binding: *const ::core::ffi::c_void, serverprincname: ::core::option::Option<*const u16>, authnlevel: u32, authnsvc: u32, authidentity: ::core::option::Option<*const ::core::ffi::c_void>, authzsvc: u32, securityqos: ::core::option::Option<*const RPC_SECURITY_QOS>) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn RpcBindingSetAuthInfoExW ( binding : *const ::core::ffi::c_void , serverprincname : *const u16 , authnlevel : u32 , authnsvc : u32 , authidentity : *const ::core::ffi::c_void , authzsvc : u32 , securityqos : *const RPC_SECURITY_QOS ) -> RPC_STATUS );
    RpcBindingSetAuthInfoExW(binding, ::core::mem::transmute(serverprincname.unwrap_or(::std::ptr::null())), authnlevel, authnsvc, ::core::mem::transmute(authidentity.unwrap_or(::std::ptr::null())), authzsvc, ::core::mem::transmute(securityqos.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcBindingSetAuthInfoW(binding: *const ::core::ffi::c_void, serverprincname: ::core::option::Option<*const u16>, authnlevel: u32, authnsvc: u32, authidentity: ::core::option::Option<*const ::core::ffi::c_void>, authzsvc: u32) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn RpcBindingSetAuthInfoW ( binding : *const ::core::ffi::c_void , serverprincname : *const u16 , authnlevel : u32 , authnsvc : u32 , authidentity : *const ::core::ffi::c_void , authzsvc : u32 ) -> RPC_STATUS );
    RpcBindingSetAuthInfoW(binding, ::core::mem::transmute(serverprincname.unwrap_or(::std::ptr::null())), authnlevel, authnsvc, ::core::mem::transmute(authidentity.unwrap_or(::std::ptr::null())), authzsvc)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcBindingSetObject(binding: *const ::core::ffi::c_void, objectuuid: *const ::windows::core::GUID) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn RpcBindingSetObject ( binding : *const ::core::ffi::c_void , objectuuid : *const :: windows::core::GUID ) -> RPC_STATUS );
    RpcBindingSetObject(binding, objectuuid)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcBindingSetOption(hbinding: *const ::core::ffi::c_void, option: u32, optionvalue: usize) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn RpcBindingSetOption ( hbinding : *const ::core::ffi::c_void , option : u32 , optionvalue : usize ) -> RPC_STATUS );
    RpcBindingSetOption(hbinding, option, optionvalue)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcBindingToStringBindingA(binding: *const ::core::ffi::c_void, stringbinding: *mut *mut u8) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn RpcBindingToStringBindingA ( binding : *const ::core::ffi::c_void , stringbinding : *mut *mut u8 ) -> RPC_STATUS );
    RpcBindingToStringBindingA(binding, stringbinding)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcBindingToStringBindingW(binding: *const ::core::ffi::c_void, stringbinding: *mut *mut u16) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn RpcBindingToStringBindingW ( binding : *const ::core::ffi::c_void , stringbinding : *mut *mut u16 ) -> RPC_STATUS );
    RpcBindingToStringBindingW(binding, stringbinding)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcBindingUnbind(binding: *const ::core::ffi::c_void) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn RpcBindingUnbind ( binding : *const ::core::ffi::c_void ) -> RPC_STATUS );
    RpcBindingUnbind(binding)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcBindingVectorFree(bindingvector: *mut *mut RPC_BINDING_VECTOR) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn RpcBindingVectorFree ( bindingvector : *mut *mut RPC_BINDING_VECTOR ) -> RPC_STATUS );
    RpcBindingVectorFree(bindingvector)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcCancelThread(thread: *const ::core::ffi::c_void) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn RpcCancelThread ( thread : *const ::core::ffi::c_void ) -> RPC_STATUS );
    RpcCancelThread(thread)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcCancelThreadEx(thread: *const ::core::ffi::c_void, timeout: i32) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn RpcCancelThreadEx ( thread : *const ::core::ffi::c_void , timeout : i32 ) -> RPC_STATUS );
    RpcCancelThreadEx(thread, timeout)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_Foundation\"`, `\"Win32_Security_Cryptography\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
#[inline]
pub unsafe fn RpcCertGeneratePrincipalNameA(context: *const super::super::Security::Cryptography::CERT_CONTEXT, flags: u32, pbuffer: *mut *mut u8) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn RpcCertGeneratePrincipalNameA ( context : *const super::super::Security::Cryptography:: CERT_CONTEXT , flags : u32 , pbuffer : *mut *mut u8 ) -> RPC_STATUS );
    RpcCertGeneratePrincipalNameA(context, flags, pbuffer)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_Foundation\"`, `\"Win32_Security_Cryptography\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
#[inline]
pub unsafe fn RpcCertGeneratePrincipalNameW(context: *const super::super::Security::Cryptography::CERT_CONTEXT, flags: u32, pbuffer: *mut *mut u16) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn RpcCertGeneratePrincipalNameW ( context : *const super::super::Security::Cryptography:: CERT_CONTEXT , flags : u32 , pbuffer : *mut *mut u16 ) -> RPC_STATUS );
    RpcCertGeneratePrincipalNameW(context, flags, pbuffer)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcEpRegisterA(ifspec: *const ::core::ffi::c_void, bindingvector: *const RPC_BINDING_VECTOR, uuidvector: ::core::option::Option<*const UUID_VECTOR>, annotation: ::core::option::Option<*const u8>) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn RpcEpRegisterA ( ifspec : *const ::core::ffi::c_void , bindingvector : *const RPC_BINDING_VECTOR , uuidvector : *const UUID_VECTOR , annotation : *const u8 ) -> RPC_STATUS );
    RpcEpRegisterA(ifspec, bindingvector, ::core::mem::transmute(uuidvector.unwrap_or(::std::ptr::null())), ::core::mem::transmute(annotation.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcEpRegisterNoReplaceA(ifspec: *const ::core::ffi::c_void, bindingvector: *const RPC_BINDING_VECTOR, uuidvector: ::core::option::Option<*const UUID_VECTOR>, annotation: ::core::option::Option<*const u8>) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn RpcEpRegisterNoReplaceA ( ifspec : *const ::core::ffi::c_void , bindingvector : *const RPC_BINDING_VECTOR , uuidvector : *const UUID_VECTOR , annotation : *const u8 ) -> RPC_STATUS );
    RpcEpRegisterNoReplaceA(ifspec, bindingvector, ::core::mem::transmute(uuidvector.unwrap_or(::std::ptr::null())), ::core::mem::transmute(annotation.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcEpRegisterNoReplaceW(ifspec: *const ::core::ffi::c_void, bindingvector: *const RPC_BINDING_VECTOR, uuidvector: ::core::option::Option<*const UUID_VECTOR>, annotation: ::core::option::Option<*const u16>) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn RpcEpRegisterNoReplaceW ( ifspec : *const ::core::ffi::c_void , bindingvector : *const RPC_BINDING_VECTOR , uuidvector : *const UUID_VECTOR , annotation : *const u16 ) -> RPC_STATUS );
    RpcEpRegisterNoReplaceW(ifspec, bindingvector, ::core::mem::transmute(uuidvector.unwrap_or(::std::ptr::null())), ::core::mem::transmute(annotation.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcEpRegisterW(ifspec: *const ::core::ffi::c_void, bindingvector: *const RPC_BINDING_VECTOR, uuidvector: ::core::option::Option<*const UUID_VECTOR>, annotation: ::core::option::Option<*const u16>) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn RpcEpRegisterW ( ifspec : *const ::core::ffi::c_void , bindingvector : *const RPC_BINDING_VECTOR , uuidvector : *const UUID_VECTOR , annotation : *const u16 ) -> RPC_STATUS );
    RpcEpRegisterW(ifspec, bindingvector, ::core::mem::transmute(uuidvector.unwrap_or(::std::ptr::null())), ::core::mem::transmute(annotation.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcEpResolveBinding(binding: *const ::core::ffi::c_void, ifspec: *const ::core::ffi::c_void) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn RpcEpResolveBinding ( binding : *const ::core::ffi::c_void , ifspec : *const ::core::ffi::c_void ) -> RPC_STATUS );
    RpcEpResolveBinding(binding, ifspec)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcEpUnregister(ifspec: *const ::core::ffi::c_void, bindingvector: *const RPC_BINDING_VECTOR, uuidvector: ::core::option::Option<*const UUID_VECTOR>) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn RpcEpUnregister ( ifspec : *const ::core::ffi::c_void , bindingvector : *const RPC_BINDING_VECTOR , uuidvector : *const UUID_VECTOR ) -> RPC_STATUS );
    RpcEpUnregister(ifspec, bindingvector, ::core::mem::transmute(uuidvector.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RpcErrorAddRecord(errorinfo: *const RPC_EXTENDED_ERROR_INFO) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn RpcErrorAddRecord ( errorinfo : *const RPC_EXTENDED_ERROR_INFO ) -> RPC_STATUS );
    RpcErrorAddRecord(errorinfo)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcErrorClearInformation() {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn RpcErrorClearInformation ( ) -> ( ) );
    RpcErrorClearInformation()
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcErrorEndEnumeration(enumhandle: *mut RPC_ERROR_ENUM_HANDLE) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn RpcErrorEndEnumeration ( enumhandle : *mut RPC_ERROR_ENUM_HANDLE ) -> RPC_STATUS );
    RpcErrorEndEnumeration(enumhandle)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RpcErrorGetNextRecord<P0>(enumhandle: *const RPC_ERROR_ENUM_HANDLE, copystrings: P0, errorinfo: *mut RPC_EXTENDED_ERROR_INFO) -> RPC_STATUS
where
    P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
{
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn RpcErrorGetNextRecord ( enumhandle : *const RPC_ERROR_ENUM_HANDLE , copystrings : super::super::Foundation:: BOOL , errorinfo : *mut RPC_EXTENDED_ERROR_INFO ) -> RPC_STATUS );
    RpcErrorGetNextRecord(enumhandle, copystrings.into_param().abi(), errorinfo)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcErrorGetNumberOfRecords(enumhandle: *const RPC_ERROR_ENUM_HANDLE, records: *mut i32) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn RpcErrorGetNumberOfRecords ( enumhandle : *const RPC_ERROR_ENUM_HANDLE , records : *mut i32 ) -> RPC_STATUS );
    RpcErrorGetNumberOfRecords(enumhandle, records)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcErrorLoadErrorInfo(errorblob: *const ::core::ffi::c_void, blobsize: usize, enumhandle: *mut RPC_ERROR_ENUM_HANDLE) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn RpcErrorLoadErrorInfo ( errorblob : *const ::core::ffi::c_void , blobsize : usize , enumhandle : *mut RPC_ERROR_ENUM_HANDLE ) -> RPC_STATUS );
    RpcErrorLoadErrorInfo(errorblob, blobsize, enumhandle)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcErrorResetEnumeration(enumhandle: *mut RPC_ERROR_ENUM_HANDLE) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn RpcErrorResetEnumeration ( enumhandle : *mut RPC_ERROR_ENUM_HANDLE ) -> RPC_STATUS );
    RpcErrorResetEnumeration(enumhandle)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcErrorSaveErrorInfo(enumhandle: *const RPC_ERROR_ENUM_HANDLE, errorblob: *mut *mut ::core::ffi::c_void, blobsize: *mut usize) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn RpcErrorSaveErrorInfo ( enumhandle : *const RPC_ERROR_ENUM_HANDLE , errorblob : *mut *mut ::core::ffi::c_void , blobsize : *mut usize ) -> RPC_STATUS );
    RpcErrorSaveErrorInfo(enumhandle, errorblob, blobsize)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcErrorStartEnumeration(enumhandle: *mut RPC_ERROR_ENUM_HANDLE) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn RpcErrorStartEnumeration ( enumhandle : *mut RPC_ERROR_ENUM_HANDLE ) -> RPC_STATUS );
    RpcErrorStartEnumeration(enumhandle)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcExceptionFilter(exceptioncode: u32) -> i32 {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn RpcExceptionFilter ( exceptioncode : u32 ) -> i32 );
    RpcExceptionFilter(exceptioncode)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcFreeAuthorizationContext(pauthzclientcontext: *mut *mut ::core::ffi::c_void) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn RpcFreeAuthorizationContext ( pauthzclientcontext : *mut *mut ::core::ffi::c_void ) -> RPC_STATUS );
    RpcFreeAuthorizationContext(pauthzclientcontext)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RpcGetAuthorizationContextForClient<P0>(clientbinding: ::core::option::Option<*const ::core::ffi::c_void>, impersonateonreturn: P0, reserved1: ::core::option::Option<*const ::core::ffi::c_void>, pexpirationtime: ::core::option::Option<*const i64>, reserved2: super::super::Foundation::LUID, reserved3: u32, reserved4: ::core::option::Option<*const ::core::ffi::c_void>, pauthzclientcontext: *mut *mut ::core::ffi::c_void) -> RPC_STATUS
where
    P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
{
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn RpcGetAuthorizationContextForClient ( clientbinding : *const ::core::ffi::c_void , impersonateonreturn : super::super::Foundation:: BOOL , reserved1 : *const ::core::ffi::c_void , pexpirationtime : *const i64 , reserved2 : super::super::Foundation:: LUID , reserved3 : u32 , reserved4 : *const ::core::ffi::c_void , pauthzclientcontext : *mut *mut ::core::ffi::c_void ) -> RPC_STATUS );
    RpcGetAuthorizationContextForClient(::core::mem::transmute(clientbinding.unwrap_or(::std::ptr::null())), impersonateonreturn.into_param().abi(), ::core::mem::transmute(reserved1.unwrap_or(::std::ptr::null())), ::core::mem::transmute(pexpirationtime.unwrap_or(::std::ptr::null())), ::core::mem::transmute(reserved2), reserved3, ::core::mem::transmute(reserved4.unwrap_or(::std::ptr::null())), pauthzclientcontext)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcIfIdVectorFree(ifidvector: *mut *mut RPC_IF_ID_VECTOR) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcns4.dll""system" fn RpcIfIdVectorFree ( ifidvector : *mut *mut RPC_IF_ID_VECTOR ) -> RPC_STATUS );
    RpcIfIdVectorFree(ifidvector)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcIfInqId(rpcifhandle: *const ::core::ffi::c_void, rpcifid: *mut RPC_IF_ID) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn RpcIfInqId ( rpcifhandle : *const ::core::ffi::c_void , rpcifid : *mut RPC_IF_ID ) -> RPC_STATUS );
    RpcIfInqId(rpcifhandle, rpcifid)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcImpersonateClient(bindinghandle: ::core::option::Option<*const ::core::ffi::c_void>) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn RpcImpersonateClient ( bindinghandle : *const ::core::ffi::c_void ) -> RPC_STATUS );
    RpcImpersonateClient(::core::mem::transmute(bindinghandle.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcImpersonateClient2(bindinghandle: ::core::option::Option<*const ::core::ffi::c_void>) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn RpcImpersonateClient2 ( bindinghandle : *const ::core::ffi::c_void ) -> RPC_STATUS );
    RpcImpersonateClient2(::core::mem::transmute(bindinghandle.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcImpersonateClientContainer(bindinghandle: ::core::option::Option<*const ::core::ffi::c_void>) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn RpcImpersonateClientContainer ( bindinghandle : *const ::core::ffi::c_void ) -> RPC_STATUS );
    RpcImpersonateClientContainer(::core::mem::transmute(bindinghandle.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcMgmtEnableIdleCleanup() -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn RpcMgmtEnableIdleCleanup ( ) -> RPC_STATUS );
    RpcMgmtEnableIdleCleanup()
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcMgmtEpEltInqBegin(epbinding: ::core::option::Option<*const ::core::ffi::c_void>, inquirytype: u32, ifid: ::core::option::Option<*const RPC_IF_ID>, versoption: u32, objectuuid: ::core::option::Option<*const ::windows::core::GUID>, inquirycontext: *mut *mut *mut ::core::ffi::c_void) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn RpcMgmtEpEltInqBegin ( epbinding : *const ::core::ffi::c_void , inquirytype : u32 , ifid : *const RPC_IF_ID , versoption : u32 , objectuuid : *const :: windows::core::GUID , inquirycontext : *mut *mut *mut ::core::ffi::c_void ) -> RPC_STATUS );
    RpcMgmtEpEltInqBegin(::core::mem::transmute(epbinding.unwrap_or(::std::ptr::null())), inquirytype, ::core::mem::transmute(ifid.unwrap_or(::std::ptr::null())), versoption, ::core::mem::transmute(objectuuid.unwrap_or(::std::ptr::null())), inquirycontext)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcMgmtEpEltInqDone(inquirycontext: *mut *mut *mut ::core::ffi::c_void) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn RpcMgmtEpEltInqDone ( inquirycontext : *mut *mut *mut ::core::ffi::c_void ) -> RPC_STATUS );
    RpcMgmtEpEltInqDone(inquirycontext)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcMgmtEpEltInqNextA(inquirycontext: *const *const ::core::ffi::c_void, ifid: *mut RPC_IF_ID, binding: ::core::option::Option<*mut *mut ::core::ffi::c_void>, objectuuid: ::core::option::Option<*mut ::windows::core::GUID>, annotation: ::core::option::Option<*mut *mut u8>) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn RpcMgmtEpEltInqNextA ( inquirycontext : *const *const ::core::ffi::c_void , ifid : *mut RPC_IF_ID , binding : *mut *mut ::core::ffi::c_void , objectuuid : *mut :: windows::core::GUID , annotation : *mut *mut u8 ) -> RPC_STATUS );
    RpcMgmtEpEltInqNextA(inquirycontext, ifid, ::core::mem::transmute(binding.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(objectuuid.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(annotation.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcMgmtEpEltInqNextW(inquirycontext: *const *const ::core::ffi::c_void, ifid: *mut RPC_IF_ID, binding: ::core::option::Option<*mut *mut ::core::ffi::c_void>, objectuuid: ::core::option::Option<*mut ::windows::core::GUID>, annotation: ::core::option::Option<*mut *mut u16>) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn RpcMgmtEpEltInqNextW ( inquirycontext : *const *const ::core::ffi::c_void , ifid : *mut RPC_IF_ID , binding : *mut *mut ::core::ffi::c_void , objectuuid : *mut :: windows::core::GUID , annotation : *mut *mut u16 ) -> RPC_STATUS );
    RpcMgmtEpEltInqNextW(inquirycontext, ifid, ::core::mem::transmute(binding.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(objectuuid.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(annotation.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcMgmtEpUnregister(epbinding: ::core::option::Option<*const ::core::ffi::c_void>, ifid: *const RPC_IF_ID, binding: *const ::core::ffi::c_void, objectuuid: ::core::option::Option<*const ::windows::core::GUID>) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn RpcMgmtEpUnregister ( epbinding : *const ::core::ffi::c_void , ifid : *const RPC_IF_ID , binding : *const ::core::ffi::c_void , objectuuid : *const :: windows::core::GUID ) -> RPC_STATUS );
    RpcMgmtEpUnregister(::core::mem::transmute(epbinding.unwrap_or(::std::ptr::null())), ifid, binding, ::core::mem::transmute(objectuuid.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcMgmtInqComTimeout(binding: *const ::core::ffi::c_void, timeout: *mut u32) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn RpcMgmtInqComTimeout ( binding : *const ::core::ffi::c_void , timeout : *mut u32 ) -> RPC_STATUS );
    RpcMgmtInqComTimeout(binding, timeout)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcMgmtInqDefaultProtectLevel(authnsvc: u32, authnlevel: *mut u32) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn RpcMgmtInqDefaultProtectLevel ( authnsvc : u32 , authnlevel : *mut u32 ) -> RPC_STATUS );
    RpcMgmtInqDefaultProtectLevel(authnsvc, authnlevel)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcMgmtInqIfIds(binding: ::core::option::Option<*const ::core::ffi::c_void>, ifidvector: *mut *mut RPC_IF_ID_VECTOR) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn RpcMgmtInqIfIds ( binding : *const ::core::ffi::c_void , ifidvector : *mut *mut RPC_IF_ID_VECTOR ) -> RPC_STATUS );
    RpcMgmtInqIfIds(::core::mem::transmute(binding.unwrap_or(::std::ptr::null())), ifidvector)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcMgmtInqServerPrincNameA(binding: ::core::option::Option<*const ::core::ffi::c_void>, authnsvc: u32, serverprincname: *mut *mut u8) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn RpcMgmtInqServerPrincNameA ( binding : *const ::core::ffi::c_void , authnsvc : u32 , serverprincname : *mut *mut u8 ) -> RPC_STATUS );
    RpcMgmtInqServerPrincNameA(::core::mem::transmute(binding.unwrap_or(::std::ptr::null())), authnsvc, serverprincname)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcMgmtInqServerPrincNameW(binding: ::core::option::Option<*const ::core::ffi::c_void>, authnsvc: u32, serverprincname: *mut *mut u16) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn RpcMgmtInqServerPrincNameW ( binding : *const ::core::ffi::c_void , authnsvc : u32 , serverprincname : *mut *mut u16 ) -> RPC_STATUS );
    RpcMgmtInqServerPrincNameW(::core::mem::transmute(binding.unwrap_or(::std::ptr::null())), authnsvc, serverprincname)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcMgmtInqStats(binding: ::core::option::Option<*const ::core::ffi::c_void>, statistics: *mut *mut RPC_STATS_VECTOR) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn RpcMgmtInqStats ( binding : *const ::core::ffi::c_void , statistics : *mut *mut RPC_STATS_VECTOR ) -> RPC_STATUS );
    RpcMgmtInqStats(::core::mem::transmute(binding.unwrap_or(::std::ptr::null())), statistics)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcMgmtIsServerListening(binding: ::core::option::Option<*const ::core::ffi::c_void>) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn RpcMgmtIsServerListening ( binding : *const ::core::ffi::c_void ) -> RPC_STATUS );
    RpcMgmtIsServerListening(::core::mem::transmute(binding.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcMgmtSetAuthorizationFn(authorizationfn: RPC_MGMT_AUTHORIZATION_FN) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn RpcMgmtSetAuthorizationFn ( authorizationfn : RPC_MGMT_AUTHORIZATION_FN ) -> RPC_STATUS );
    RpcMgmtSetAuthorizationFn(authorizationfn)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcMgmtSetCancelTimeout(timeout: i32) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn RpcMgmtSetCancelTimeout ( timeout : i32 ) -> RPC_STATUS );
    RpcMgmtSetCancelTimeout(timeout)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcMgmtSetComTimeout(binding: *const ::core::ffi::c_void, timeout: u32) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn RpcMgmtSetComTimeout ( binding : *const ::core::ffi::c_void , timeout : u32 ) -> RPC_STATUS );
    RpcMgmtSetComTimeout(binding, timeout)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcMgmtSetServerStackSize(threadstacksize: u32) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn RpcMgmtSetServerStackSize ( threadstacksize : u32 ) -> RPC_STATUS );
    RpcMgmtSetServerStackSize(threadstacksize)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcMgmtStatsVectorFree(statsvector: *mut *mut RPC_STATS_VECTOR) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn RpcMgmtStatsVectorFree ( statsvector : *mut *mut RPC_STATS_VECTOR ) -> RPC_STATUS );
    RpcMgmtStatsVectorFree(statsvector)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcMgmtStopServerListening(binding: ::core::option::Option<*const ::core::ffi::c_void>) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn RpcMgmtStopServerListening ( binding : *const ::core::ffi::c_void ) -> RPC_STATUS );
    RpcMgmtStopServerListening(::core::mem::transmute(binding.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcMgmtWaitServerListen() -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn RpcMgmtWaitServerListen ( ) -> RPC_STATUS );
    RpcMgmtWaitServerListen()
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcNetworkInqProtseqsA(protseqvector: *mut *mut RPC_PROTSEQ_VECTORA) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn RpcNetworkInqProtseqsA ( protseqvector : *mut *mut RPC_PROTSEQ_VECTORA ) -> RPC_STATUS );
    RpcNetworkInqProtseqsA(protseqvector)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcNetworkInqProtseqsW(protseqvector: *mut *mut RPC_PROTSEQ_VECTORW) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn RpcNetworkInqProtseqsW ( protseqvector : *mut *mut RPC_PROTSEQ_VECTORW ) -> RPC_STATUS );
    RpcNetworkInqProtseqsW(protseqvector)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcNetworkIsProtseqValidA(protseq: *const u8) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn RpcNetworkIsProtseqValidA ( protseq : *const u8 ) -> RPC_STATUS );
    RpcNetworkIsProtseqValidA(protseq)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcNetworkIsProtseqValidW(protseq: *const u16) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn RpcNetworkIsProtseqValidW ( protseq : *const u16 ) -> RPC_STATUS );
    RpcNetworkIsProtseqValidW(protseq)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcNsBindingExportA(entrynamesyntax: u32, entryname: ::core::option::Option<*const u8>, ifspec: ::core::option::Option<*const ::core::ffi::c_void>, bindingvec: ::core::option::Option<*const RPC_BINDING_VECTOR>, objectuuidvec: ::core::option::Option<*const UUID_VECTOR>) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcns4.dll""system" fn RpcNsBindingExportA ( entrynamesyntax : u32 , entryname : *const u8 , ifspec : *const ::core::ffi::c_void , bindingvec : *const RPC_BINDING_VECTOR , objectuuidvec : *const UUID_VECTOR ) -> RPC_STATUS );
    RpcNsBindingExportA(entrynamesyntax, ::core::mem::transmute(entryname.unwrap_or(::std::ptr::null())), ::core::mem::transmute(ifspec.unwrap_or(::std::ptr::null())), ::core::mem::transmute(bindingvec.unwrap_or(::std::ptr::null())), ::core::mem::transmute(objectuuidvec.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcNsBindingExportPnPA(entrynamesyntax: u32, entryname: ::core::option::Option<*const u8>, ifspec: ::core::option::Option<*const ::core::ffi::c_void>, objectvector: ::core::option::Option<*const UUID_VECTOR>) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcns4.dll""system" fn RpcNsBindingExportPnPA ( entrynamesyntax : u32 , entryname : *const u8 , ifspec : *const ::core::ffi::c_void , objectvector : *const UUID_VECTOR ) -> RPC_STATUS );
    RpcNsBindingExportPnPA(entrynamesyntax, ::core::mem::transmute(entryname.unwrap_or(::std::ptr::null())), ::core::mem::transmute(ifspec.unwrap_or(::std::ptr::null())), ::core::mem::transmute(objectvector.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcNsBindingExportPnPW(entrynamesyntax: u32, entryname: ::core::option::Option<*const u16>, ifspec: ::core::option::Option<*const ::core::ffi::c_void>, objectvector: ::core::option::Option<*const UUID_VECTOR>) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcns4.dll""system" fn RpcNsBindingExportPnPW ( entrynamesyntax : u32 , entryname : *const u16 , ifspec : *const ::core::ffi::c_void , objectvector : *const UUID_VECTOR ) -> RPC_STATUS );
    RpcNsBindingExportPnPW(entrynamesyntax, ::core::mem::transmute(entryname.unwrap_or(::std::ptr::null())), ::core::mem::transmute(ifspec.unwrap_or(::std::ptr::null())), ::core::mem::transmute(objectvector.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcNsBindingExportW(entrynamesyntax: u32, entryname: ::core::option::Option<*const u16>, ifspec: ::core::option::Option<*const ::core::ffi::c_void>, bindingvec: ::core::option::Option<*const RPC_BINDING_VECTOR>, objectuuidvec: ::core::option::Option<*const UUID_VECTOR>) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcns4.dll""system" fn RpcNsBindingExportW ( entrynamesyntax : u32 , entryname : *const u16 , ifspec : *const ::core::ffi::c_void , bindingvec : *const RPC_BINDING_VECTOR , objectuuidvec : *const UUID_VECTOR ) -> RPC_STATUS );
    RpcNsBindingExportW(entrynamesyntax, ::core::mem::transmute(entryname.unwrap_or(::std::ptr::null())), ::core::mem::transmute(ifspec.unwrap_or(::std::ptr::null())), ::core::mem::transmute(bindingvec.unwrap_or(::std::ptr::null())), ::core::mem::transmute(objectuuidvec.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcNsBindingImportBeginA(entrynamesyntax: u32, entryname: ::core::option::Option<*const u8>, ifspec: ::core::option::Option<*const ::core::ffi::c_void>, objuuid: ::core::option::Option<*const ::windows::core::GUID>, importcontext: *mut *mut ::core::ffi::c_void) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcns4.dll""system" fn RpcNsBindingImportBeginA ( entrynamesyntax : u32 , entryname : *const u8 , ifspec : *const ::core::ffi::c_void , objuuid : *const :: windows::core::GUID , importcontext : *mut *mut ::core::ffi::c_void ) -> RPC_STATUS );
    RpcNsBindingImportBeginA(entrynamesyntax, ::core::mem::transmute(entryname.unwrap_or(::std::ptr::null())), ::core::mem::transmute(ifspec.unwrap_or(::std::ptr::null())), ::core::mem::transmute(objuuid.unwrap_or(::std::ptr::null())), importcontext)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcNsBindingImportBeginW(entrynamesyntax: u32, entryname: ::core::option::Option<*const u16>, ifspec: ::core::option::Option<*const ::core::ffi::c_void>, objuuid: ::core::option::Option<*const ::windows::core::GUID>, importcontext: *mut *mut ::core::ffi::c_void) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcns4.dll""system" fn RpcNsBindingImportBeginW ( entrynamesyntax : u32 , entryname : *const u16 , ifspec : *const ::core::ffi::c_void , objuuid : *const :: windows::core::GUID , importcontext : *mut *mut ::core::ffi::c_void ) -> RPC_STATUS );
    RpcNsBindingImportBeginW(entrynamesyntax, ::core::mem::transmute(entryname.unwrap_or(::std::ptr::null())), ::core::mem::transmute(ifspec.unwrap_or(::std::ptr::null())), ::core::mem::transmute(objuuid.unwrap_or(::std::ptr::null())), importcontext)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcNsBindingImportDone(importcontext: *mut *mut ::core::ffi::c_void) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcns4.dll""system" fn RpcNsBindingImportDone ( importcontext : *mut *mut ::core::ffi::c_void ) -> RPC_STATUS );
    RpcNsBindingImportDone(importcontext)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcNsBindingImportNext(importcontext: *mut ::core::ffi::c_void, binding: *mut *mut ::core::ffi::c_void) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcns4.dll""system" fn RpcNsBindingImportNext ( importcontext : *mut ::core::ffi::c_void , binding : *mut *mut ::core::ffi::c_void ) -> RPC_STATUS );
    RpcNsBindingImportNext(importcontext, binding)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcNsBindingInqEntryNameA(binding: *const ::core::ffi::c_void, entrynamesyntax: u32, entryname: *mut *mut u8) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn RpcNsBindingInqEntryNameA ( binding : *const ::core::ffi::c_void , entrynamesyntax : u32 , entryname : *mut *mut u8 ) -> RPC_STATUS );
    RpcNsBindingInqEntryNameA(binding, entrynamesyntax, entryname)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcNsBindingInqEntryNameW(binding: *const ::core::ffi::c_void, entrynamesyntax: u32, entryname: *mut *mut u16) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn RpcNsBindingInqEntryNameW ( binding : *const ::core::ffi::c_void , entrynamesyntax : u32 , entryname : *mut *mut u16 ) -> RPC_STATUS );
    RpcNsBindingInqEntryNameW(binding, entrynamesyntax, entryname)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcNsBindingLookupBeginA(entrynamesyntax: u32, entryname: ::core::option::Option<*const u8>, ifspec: ::core::option::Option<*const ::core::ffi::c_void>, objuuid: ::core::option::Option<*const ::windows::core::GUID>, bindingmaxcount: u32, lookupcontext: *mut *mut ::core::ffi::c_void) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcns4.dll""system" fn RpcNsBindingLookupBeginA ( entrynamesyntax : u32 , entryname : *const u8 , ifspec : *const ::core::ffi::c_void , objuuid : *const :: windows::core::GUID , bindingmaxcount : u32 , lookupcontext : *mut *mut ::core::ffi::c_void ) -> RPC_STATUS );
    RpcNsBindingLookupBeginA(entrynamesyntax, ::core::mem::transmute(entryname.unwrap_or(::std::ptr::null())), ::core::mem::transmute(ifspec.unwrap_or(::std::ptr::null())), ::core::mem::transmute(objuuid.unwrap_or(::std::ptr::null())), bindingmaxcount, lookupcontext)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcNsBindingLookupBeginW(entrynamesyntax: u32, entryname: ::core::option::Option<*const u16>, ifspec: ::core::option::Option<*const ::core::ffi::c_void>, objuuid: ::core::option::Option<*const ::windows::core::GUID>, bindingmaxcount: u32, lookupcontext: *mut *mut ::core::ffi::c_void) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcns4.dll""system" fn RpcNsBindingLookupBeginW ( entrynamesyntax : u32 , entryname : *const u16 , ifspec : *const ::core::ffi::c_void , objuuid : *const :: windows::core::GUID , bindingmaxcount : u32 , lookupcontext : *mut *mut ::core::ffi::c_void ) -> RPC_STATUS );
    RpcNsBindingLookupBeginW(entrynamesyntax, ::core::mem::transmute(entryname.unwrap_or(::std::ptr::null())), ::core::mem::transmute(ifspec.unwrap_or(::std::ptr::null())), ::core::mem::transmute(objuuid.unwrap_or(::std::ptr::null())), bindingmaxcount, lookupcontext)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcNsBindingLookupDone(lookupcontext: *mut *mut ::core::ffi::c_void) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcns4.dll""system" fn RpcNsBindingLookupDone ( lookupcontext : *mut *mut ::core::ffi::c_void ) -> RPC_STATUS );
    RpcNsBindingLookupDone(lookupcontext)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcNsBindingLookupNext(lookupcontext: *mut ::core::ffi::c_void, bindingvec: *mut *mut RPC_BINDING_VECTOR) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcns4.dll""system" fn RpcNsBindingLookupNext ( lookupcontext : *mut ::core::ffi::c_void , bindingvec : *mut *mut RPC_BINDING_VECTOR ) -> RPC_STATUS );
    RpcNsBindingLookupNext(lookupcontext, bindingvec)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcNsBindingSelect(bindingvec: *mut RPC_BINDING_VECTOR, binding: *mut *mut ::core::ffi::c_void) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcns4.dll""system" fn RpcNsBindingSelect ( bindingvec : *mut RPC_BINDING_VECTOR , binding : *mut *mut ::core::ffi::c_void ) -> RPC_STATUS );
    RpcNsBindingSelect(bindingvec, binding)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcNsBindingUnexportA(entrynamesyntax: u32, entryname: ::core::option::Option<*const u8>, ifspec: ::core::option::Option<*const ::core::ffi::c_void>, objectuuidvec: ::core::option::Option<*const UUID_VECTOR>) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcns4.dll""system" fn RpcNsBindingUnexportA ( entrynamesyntax : u32 , entryname : *const u8 , ifspec : *const ::core::ffi::c_void , objectuuidvec : *const UUID_VECTOR ) -> RPC_STATUS );
    RpcNsBindingUnexportA(entrynamesyntax, ::core::mem::transmute(entryname.unwrap_or(::std::ptr::null())), ::core::mem::transmute(ifspec.unwrap_or(::std::ptr::null())), ::core::mem::transmute(objectuuidvec.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcNsBindingUnexportPnPA(entrynamesyntax: u32, entryname: ::core::option::Option<*const u8>, ifspec: ::core::option::Option<*const ::core::ffi::c_void>, objectvector: ::core::option::Option<*const UUID_VECTOR>) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcns4.dll""system" fn RpcNsBindingUnexportPnPA ( entrynamesyntax : u32 , entryname : *const u8 , ifspec : *const ::core::ffi::c_void , objectvector : *const UUID_VECTOR ) -> RPC_STATUS );
    RpcNsBindingUnexportPnPA(entrynamesyntax, ::core::mem::transmute(entryname.unwrap_or(::std::ptr::null())), ::core::mem::transmute(ifspec.unwrap_or(::std::ptr::null())), ::core::mem::transmute(objectvector.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcNsBindingUnexportPnPW(entrynamesyntax: u32, entryname: ::core::option::Option<*const u16>, ifspec: ::core::option::Option<*const ::core::ffi::c_void>, objectvector: ::core::option::Option<*const UUID_VECTOR>) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcns4.dll""system" fn RpcNsBindingUnexportPnPW ( entrynamesyntax : u32 , entryname : *const u16 , ifspec : *const ::core::ffi::c_void , objectvector : *const UUID_VECTOR ) -> RPC_STATUS );
    RpcNsBindingUnexportPnPW(entrynamesyntax, ::core::mem::transmute(entryname.unwrap_or(::std::ptr::null())), ::core::mem::transmute(ifspec.unwrap_or(::std::ptr::null())), ::core::mem::transmute(objectvector.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcNsBindingUnexportW(entrynamesyntax: u32, entryname: ::core::option::Option<*const u16>, ifspec: ::core::option::Option<*const ::core::ffi::c_void>, objectuuidvec: ::core::option::Option<*const UUID_VECTOR>) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcns4.dll""system" fn RpcNsBindingUnexportW ( entrynamesyntax : u32 , entryname : *const u16 , ifspec : *const ::core::ffi::c_void , objectuuidvec : *const UUID_VECTOR ) -> RPC_STATUS );
    RpcNsBindingUnexportW(entrynamesyntax, ::core::mem::transmute(entryname.unwrap_or(::std::ptr::null())), ::core::mem::transmute(ifspec.unwrap_or(::std::ptr::null())), ::core::mem::transmute(objectuuidvec.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcNsEntryExpandNameA(entrynamesyntax: u32, entryname: *const u8, expandedname: *mut *mut u8) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcns4.dll""system" fn RpcNsEntryExpandNameA ( entrynamesyntax : u32 , entryname : *const u8 , expandedname : *mut *mut u8 ) -> RPC_STATUS );
    RpcNsEntryExpandNameA(entrynamesyntax, entryname, expandedname)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcNsEntryExpandNameW(entrynamesyntax: u32, entryname: *const u16, expandedname: *mut *mut u16) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcns4.dll""system" fn RpcNsEntryExpandNameW ( entrynamesyntax : u32 , entryname : *const u16 , expandedname : *mut *mut u16 ) -> RPC_STATUS );
    RpcNsEntryExpandNameW(entrynamesyntax, entryname, expandedname)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcNsEntryObjectInqBeginA(entrynamesyntax: u32, entryname: *const u8, inquirycontext: *mut *mut ::core::ffi::c_void) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcns4.dll""system" fn RpcNsEntryObjectInqBeginA ( entrynamesyntax : u32 , entryname : *const u8 , inquirycontext : *mut *mut ::core::ffi::c_void ) -> RPC_STATUS );
    RpcNsEntryObjectInqBeginA(entrynamesyntax, entryname, inquirycontext)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcNsEntryObjectInqBeginW(entrynamesyntax: u32, entryname: *const u16, inquirycontext: *mut *mut ::core::ffi::c_void) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcns4.dll""system" fn RpcNsEntryObjectInqBeginW ( entrynamesyntax : u32 , entryname : *const u16 , inquirycontext : *mut *mut ::core::ffi::c_void ) -> RPC_STATUS );
    RpcNsEntryObjectInqBeginW(entrynamesyntax, entryname, inquirycontext)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcNsEntryObjectInqDone(inquirycontext: *mut *mut ::core::ffi::c_void) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcns4.dll""system" fn RpcNsEntryObjectInqDone ( inquirycontext : *mut *mut ::core::ffi::c_void ) -> RPC_STATUS );
    RpcNsEntryObjectInqDone(inquirycontext)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcNsEntryObjectInqNext(inquirycontext: *mut ::core::ffi::c_void, objuuid: *mut ::windows::core::GUID) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcns4.dll""system" fn RpcNsEntryObjectInqNext ( inquirycontext : *mut ::core::ffi::c_void , objuuid : *mut :: windows::core::GUID ) -> RPC_STATUS );
    RpcNsEntryObjectInqNext(inquirycontext, objuuid)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcNsGroupDeleteA(groupnamesyntax: GROUP_NAME_SYNTAX, groupname: ::core::option::Option<*const u8>) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcns4.dll""system" fn RpcNsGroupDeleteA ( groupnamesyntax : GROUP_NAME_SYNTAX , groupname : *const u8 ) -> RPC_STATUS );
    RpcNsGroupDeleteA(groupnamesyntax, ::core::mem::transmute(groupname.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcNsGroupDeleteW(groupnamesyntax: GROUP_NAME_SYNTAX, groupname: ::core::option::Option<*const u16>) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcns4.dll""system" fn RpcNsGroupDeleteW ( groupnamesyntax : GROUP_NAME_SYNTAX , groupname : *const u16 ) -> RPC_STATUS );
    RpcNsGroupDeleteW(groupnamesyntax, ::core::mem::transmute(groupname.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcNsGroupMbrAddA(groupnamesyntax: u32, groupname: *const u8, membernamesyntax: u32, membername: *const u8) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcns4.dll""system" fn RpcNsGroupMbrAddA ( groupnamesyntax : u32 , groupname : *const u8 , membernamesyntax : u32 , membername : *const u8 ) -> RPC_STATUS );
    RpcNsGroupMbrAddA(groupnamesyntax, groupname, membernamesyntax, membername)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcNsGroupMbrAddW(groupnamesyntax: u32, groupname: *const u16, membernamesyntax: u32, membername: *const u16) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcns4.dll""system" fn RpcNsGroupMbrAddW ( groupnamesyntax : u32 , groupname : *const u16 , membernamesyntax : u32 , membername : *const u16 ) -> RPC_STATUS );
    RpcNsGroupMbrAddW(groupnamesyntax, groupname, membernamesyntax, membername)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcNsGroupMbrInqBeginA(groupnamesyntax: u32, groupname: *const u8, membernamesyntax: u32, inquirycontext: *mut *mut ::core::ffi::c_void) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcns4.dll""system" fn RpcNsGroupMbrInqBeginA ( groupnamesyntax : u32 , groupname : *const u8 , membernamesyntax : u32 , inquirycontext : *mut *mut ::core::ffi::c_void ) -> RPC_STATUS );
    RpcNsGroupMbrInqBeginA(groupnamesyntax, groupname, membernamesyntax, inquirycontext)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcNsGroupMbrInqBeginW(groupnamesyntax: u32, groupname: *const u16, membernamesyntax: u32, inquirycontext: *mut *mut ::core::ffi::c_void) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcns4.dll""system" fn RpcNsGroupMbrInqBeginW ( groupnamesyntax : u32 , groupname : *const u16 , membernamesyntax : u32 , inquirycontext : *mut *mut ::core::ffi::c_void ) -> RPC_STATUS );
    RpcNsGroupMbrInqBeginW(groupnamesyntax, groupname, membernamesyntax, inquirycontext)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcNsGroupMbrInqDone(inquirycontext: *mut *mut ::core::ffi::c_void) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcns4.dll""system" fn RpcNsGroupMbrInqDone ( inquirycontext : *mut *mut ::core::ffi::c_void ) -> RPC_STATUS );
    RpcNsGroupMbrInqDone(inquirycontext)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcNsGroupMbrInqNextA(inquirycontext: *mut ::core::ffi::c_void, membername: *mut *mut u8) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcns4.dll""system" fn RpcNsGroupMbrInqNextA ( inquirycontext : *mut ::core::ffi::c_void , membername : *mut *mut u8 ) -> RPC_STATUS );
    RpcNsGroupMbrInqNextA(inquirycontext, membername)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcNsGroupMbrInqNextW(inquirycontext: *mut ::core::ffi::c_void, membername: *mut *mut u16) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcns4.dll""system" fn RpcNsGroupMbrInqNextW ( inquirycontext : *mut ::core::ffi::c_void , membername : *mut *mut u16 ) -> RPC_STATUS );
    RpcNsGroupMbrInqNextW(inquirycontext, membername)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcNsGroupMbrRemoveA(groupnamesyntax: u32, groupname: *const u8, membernamesyntax: u32, membername: *const u8) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcns4.dll""system" fn RpcNsGroupMbrRemoveA ( groupnamesyntax : u32 , groupname : *const u8 , membernamesyntax : u32 , membername : *const u8 ) -> RPC_STATUS );
    RpcNsGroupMbrRemoveA(groupnamesyntax, groupname, membernamesyntax, membername)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcNsGroupMbrRemoveW(groupnamesyntax: u32, groupname: *const u16, membernamesyntax: u32, membername: *const u16) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcns4.dll""system" fn RpcNsGroupMbrRemoveW ( groupnamesyntax : u32 , groupname : *const u16 , membernamesyntax : u32 , membername : *const u16 ) -> RPC_STATUS );
    RpcNsGroupMbrRemoveW(groupnamesyntax, groupname, membernamesyntax, membername)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcNsMgmtBindingUnexportA(entrynamesyntax: u32, entryname: *const u8, ifid: ::core::option::Option<*const RPC_IF_ID>, versoption: u32, objectuuidvec: ::core::option::Option<*const UUID_VECTOR>) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcns4.dll""system" fn RpcNsMgmtBindingUnexportA ( entrynamesyntax : u32 , entryname : *const u8 , ifid : *const RPC_IF_ID , versoption : u32 , objectuuidvec : *const UUID_VECTOR ) -> RPC_STATUS );
    RpcNsMgmtBindingUnexportA(entrynamesyntax, entryname, ::core::mem::transmute(ifid.unwrap_or(::std::ptr::null())), versoption, ::core::mem::transmute(objectuuidvec.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcNsMgmtBindingUnexportW(entrynamesyntax: u32, entryname: *const u16, ifid: ::core::option::Option<*const RPC_IF_ID>, versoption: u32, objectuuidvec: ::core::option::Option<*const UUID_VECTOR>) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcns4.dll""system" fn RpcNsMgmtBindingUnexportW ( entrynamesyntax : u32 , entryname : *const u16 , ifid : *const RPC_IF_ID , versoption : u32 , objectuuidvec : *const UUID_VECTOR ) -> RPC_STATUS );
    RpcNsMgmtBindingUnexportW(entrynamesyntax, entryname, ::core::mem::transmute(ifid.unwrap_or(::std::ptr::null())), versoption, ::core::mem::transmute(objectuuidvec.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcNsMgmtEntryCreateA(entrynamesyntax: u32, entryname: *const u8) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcns4.dll""system" fn RpcNsMgmtEntryCreateA ( entrynamesyntax : u32 , entryname : *const u8 ) -> RPC_STATUS );
    RpcNsMgmtEntryCreateA(entrynamesyntax, entryname)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcNsMgmtEntryCreateW(entrynamesyntax: u32, entryname: *const u16) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcns4.dll""system" fn RpcNsMgmtEntryCreateW ( entrynamesyntax : u32 , entryname : *const u16 ) -> RPC_STATUS );
    RpcNsMgmtEntryCreateW(entrynamesyntax, entryname)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcNsMgmtEntryDeleteA(entrynamesyntax: u32, entryname: *const u8) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcns4.dll""system" fn RpcNsMgmtEntryDeleteA ( entrynamesyntax : u32 , entryname : *const u8 ) -> RPC_STATUS );
    RpcNsMgmtEntryDeleteA(entrynamesyntax, entryname)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcNsMgmtEntryDeleteW(entrynamesyntax: u32, entryname: *const u16) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcns4.dll""system" fn RpcNsMgmtEntryDeleteW ( entrynamesyntax : u32 , entryname : *const u16 ) -> RPC_STATUS );
    RpcNsMgmtEntryDeleteW(entrynamesyntax, entryname)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcNsMgmtEntryInqIfIdsA(entrynamesyntax: u32, entryname: *const u8, ifidvec: *mut *mut RPC_IF_ID_VECTOR) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcns4.dll""system" fn RpcNsMgmtEntryInqIfIdsA ( entrynamesyntax : u32 , entryname : *const u8 , ifidvec : *mut *mut RPC_IF_ID_VECTOR ) -> RPC_STATUS );
    RpcNsMgmtEntryInqIfIdsA(entrynamesyntax, entryname, ifidvec)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcNsMgmtEntryInqIfIdsW(entrynamesyntax: u32, entryname: *const u16, ifidvec: *mut *mut RPC_IF_ID_VECTOR) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcns4.dll""system" fn RpcNsMgmtEntryInqIfIdsW ( entrynamesyntax : u32 , entryname : *const u16 , ifidvec : *mut *mut RPC_IF_ID_VECTOR ) -> RPC_STATUS );
    RpcNsMgmtEntryInqIfIdsW(entrynamesyntax, entryname, ifidvec)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcNsMgmtHandleSetExpAge(nshandle: *mut ::core::ffi::c_void, expirationage: u32) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcns4.dll""system" fn RpcNsMgmtHandleSetExpAge ( nshandle : *mut ::core::ffi::c_void , expirationage : u32 ) -> RPC_STATUS );
    RpcNsMgmtHandleSetExpAge(nshandle, expirationage)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcNsMgmtInqExpAge(expirationage: *mut u32) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcns4.dll""system" fn RpcNsMgmtInqExpAge ( expirationage : *mut u32 ) -> RPC_STATUS );
    RpcNsMgmtInqExpAge(expirationage)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcNsMgmtSetExpAge(expirationage: u32) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcns4.dll""system" fn RpcNsMgmtSetExpAge ( expirationage : u32 ) -> RPC_STATUS );
    RpcNsMgmtSetExpAge(expirationage)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcNsProfileDeleteA(profilenamesyntax: u32, profilename: *const u8) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcns4.dll""system" fn RpcNsProfileDeleteA ( profilenamesyntax : u32 , profilename : *const u8 ) -> RPC_STATUS );
    RpcNsProfileDeleteA(profilenamesyntax, profilename)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcNsProfileDeleteW(profilenamesyntax: u32, profilename: *const u16) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcns4.dll""system" fn RpcNsProfileDeleteW ( profilenamesyntax : u32 , profilename : *const u16 ) -> RPC_STATUS );
    RpcNsProfileDeleteW(profilenamesyntax, profilename)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcNsProfileEltAddA(profilenamesyntax: u32, profilename: *const u8, ifid: ::core::option::Option<*const RPC_IF_ID>, membernamesyntax: u32, membername: *const u8, priority: u32, annotation: ::core::option::Option<*const u8>) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcns4.dll""system" fn RpcNsProfileEltAddA ( profilenamesyntax : u32 , profilename : *const u8 , ifid : *const RPC_IF_ID , membernamesyntax : u32 , membername : *const u8 , priority : u32 , annotation : *const u8 ) -> RPC_STATUS );
    RpcNsProfileEltAddA(profilenamesyntax, profilename, ::core::mem::transmute(ifid.unwrap_or(::std::ptr::null())), membernamesyntax, membername, priority, ::core::mem::transmute(annotation.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcNsProfileEltAddW(profilenamesyntax: u32, profilename: *const u16, ifid: ::core::option::Option<*const RPC_IF_ID>, membernamesyntax: u32, membername: *const u16, priority: u32, annotation: ::core::option::Option<*const u16>) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcns4.dll""system" fn RpcNsProfileEltAddW ( profilenamesyntax : u32 , profilename : *const u16 , ifid : *const RPC_IF_ID , membernamesyntax : u32 , membername : *const u16 , priority : u32 , annotation : *const u16 ) -> RPC_STATUS );
    RpcNsProfileEltAddW(profilenamesyntax, profilename, ::core::mem::transmute(ifid.unwrap_or(::std::ptr::null())), membernamesyntax, membername, priority, ::core::mem::transmute(annotation.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcNsProfileEltInqBeginA(profilenamesyntax: u32, profilename: *const u8, inquirytype: u32, ifid: ::core::option::Option<*const RPC_IF_ID>, versoption: u32, membernamesyntax: u32, membername: ::core::option::Option<*const u8>, inquirycontext: *mut *mut ::core::ffi::c_void) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcns4.dll""system" fn RpcNsProfileEltInqBeginA ( profilenamesyntax : u32 , profilename : *const u8 , inquirytype : u32 , ifid : *const RPC_IF_ID , versoption : u32 , membernamesyntax : u32 , membername : *const u8 , inquirycontext : *mut *mut ::core::ffi::c_void ) -> RPC_STATUS );
    RpcNsProfileEltInqBeginA(profilenamesyntax, profilename, inquirytype, ::core::mem::transmute(ifid.unwrap_or(::std::ptr::null())), versoption, membernamesyntax, ::core::mem::transmute(membername.unwrap_or(::std::ptr::null())), inquirycontext)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcNsProfileEltInqBeginW(profilenamesyntax: u32, profilename: *const u16, inquirytype: u32, ifid: ::core::option::Option<*const RPC_IF_ID>, versoption: u32, membernamesyntax: u32, membername: ::core::option::Option<*const u16>, inquirycontext: *mut *mut ::core::ffi::c_void) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcns4.dll""system" fn RpcNsProfileEltInqBeginW ( profilenamesyntax : u32 , profilename : *const u16 , inquirytype : u32 , ifid : *const RPC_IF_ID , versoption : u32 , membernamesyntax : u32 , membername : *const u16 , inquirycontext : *mut *mut ::core::ffi::c_void ) -> RPC_STATUS );
    RpcNsProfileEltInqBeginW(profilenamesyntax, profilename, inquirytype, ::core::mem::transmute(ifid.unwrap_or(::std::ptr::null())), versoption, membernamesyntax, ::core::mem::transmute(membername.unwrap_or(::std::ptr::null())), inquirycontext)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcNsProfileEltInqDone(inquirycontext: *mut *mut ::core::ffi::c_void) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcns4.dll""system" fn RpcNsProfileEltInqDone ( inquirycontext : *mut *mut ::core::ffi::c_void ) -> RPC_STATUS );
    RpcNsProfileEltInqDone(inquirycontext)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcNsProfileEltInqNextA(inquirycontext: *const ::core::ffi::c_void, ifid: ::core::option::Option<*mut RPC_IF_ID>, membername: *mut *mut u8, priority: *mut u32, annotation: *mut *mut u8) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcns4.dll""system" fn RpcNsProfileEltInqNextA ( inquirycontext : *const ::core::ffi::c_void , ifid : *mut RPC_IF_ID , membername : *mut *mut u8 , priority : *mut u32 , annotation : *mut *mut u8 ) -> RPC_STATUS );
    RpcNsProfileEltInqNextA(inquirycontext, ::core::mem::transmute(ifid.unwrap_or(::std::ptr::null_mut())), membername, priority, annotation)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcNsProfileEltInqNextW(inquirycontext: *const ::core::ffi::c_void, ifid: ::core::option::Option<*mut RPC_IF_ID>, membername: *mut *mut u16, priority: *mut u32, annotation: *mut *mut u16) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcns4.dll""system" fn RpcNsProfileEltInqNextW ( inquirycontext : *const ::core::ffi::c_void , ifid : *mut RPC_IF_ID , membername : *mut *mut u16 , priority : *mut u32 , annotation : *mut *mut u16 ) -> RPC_STATUS );
    RpcNsProfileEltInqNextW(inquirycontext, ::core::mem::transmute(ifid.unwrap_or(::std::ptr::null_mut())), membername, priority, annotation)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcNsProfileEltRemoveA(profilenamesyntax: u32, profilename: *const u8, ifid: ::core::option::Option<*const RPC_IF_ID>, membernamesyntax: u32, membername: *const u8) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcns4.dll""system" fn RpcNsProfileEltRemoveA ( profilenamesyntax : u32 , profilename : *const u8 , ifid : *const RPC_IF_ID , membernamesyntax : u32 , membername : *const u8 ) -> RPC_STATUS );
    RpcNsProfileEltRemoveA(profilenamesyntax, profilename, ::core::mem::transmute(ifid.unwrap_or(::std::ptr::null())), membernamesyntax, membername)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcNsProfileEltRemoveW(profilenamesyntax: u32, profilename: *const u16, ifid: ::core::option::Option<*const RPC_IF_ID>, membernamesyntax: u32, membername: *const u16) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcns4.dll""system" fn RpcNsProfileEltRemoveW ( profilenamesyntax : u32 , profilename : *const u16 , ifid : *const RPC_IF_ID , membernamesyntax : u32 , membername : *const u16 ) -> RPC_STATUS );
    RpcNsProfileEltRemoveW(profilenamesyntax, profilename, ::core::mem::transmute(ifid.unwrap_or(::std::ptr::null())), membernamesyntax, membername)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcObjectInqType(objuuid: *const ::windows::core::GUID, typeuuid: ::core::option::Option<*mut ::windows::core::GUID>) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn RpcObjectInqType ( objuuid : *const :: windows::core::GUID , typeuuid : *mut :: windows::core::GUID ) -> RPC_STATUS );
    RpcObjectInqType(objuuid, ::core::mem::transmute(typeuuid.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcObjectSetInqFn(inquiryfn: RPC_OBJECT_INQ_FN) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn RpcObjectSetInqFn ( inquiryfn : RPC_OBJECT_INQ_FN ) -> RPC_STATUS );
    RpcObjectSetInqFn(inquiryfn)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcObjectSetType(objuuid: *const ::windows::core::GUID, typeuuid: ::core::option::Option<*const ::windows::core::GUID>) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn RpcObjectSetType ( objuuid : *const :: windows::core::GUID , typeuuid : *const :: windows::core::GUID ) -> RPC_STATUS );
    RpcObjectSetType(objuuid, ::core::mem::transmute(typeuuid.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcProtseqVectorFreeA(protseqvector: *mut *mut RPC_PROTSEQ_VECTORA) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn RpcProtseqVectorFreeA ( protseqvector : *mut *mut RPC_PROTSEQ_VECTORA ) -> RPC_STATUS );
    RpcProtseqVectorFreeA(protseqvector)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcProtseqVectorFreeW(protseqvector: *mut *mut RPC_PROTSEQ_VECTORW) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn RpcProtseqVectorFreeW ( protseqvector : *mut *mut RPC_PROTSEQ_VECTORW ) -> RPC_STATUS );
    RpcProtseqVectorFreeW(protseqvector)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcRaiseException(exception: RPC_STATUS) {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn RpcRaiseException ( exception : RPC_STATUS ) -> ( ) );
    RpcRaiseException(exception)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcRevertContainerImpersonation() -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn RpcRevertContainerImpersonation ( ) -> RPC_STATUS );
    RpcRevertContainerImpersonation()
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcRevertToSelf() -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn RpcRevertToSelf ( ) -> RPC_STATUS );
    RpcRevertToSelf()
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcRevertToSelfEx(bindinghandle: ::core::option::Option<*const ::core::ffi::c_void>) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn RpcRevertToSelfEx ( bindinghandle : *const ::core::ffi::c_void ) -> RPC_STATUS );
    RpcRevertToSelfEx(::core::mem::transmute(bindinghandle.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcServerCompleteSecurityCallback(bindinghandle: *const ::core::ffi::c_void, status: RPC_STATUS) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn RpcServerCompleteSecurityCallback ( bindinghandle : *const ::core::ffi::c_void , status : RPC_STATUS ) -> RPC_STATUS );
    RpcServerCompleteSecurityCallback(bindinghandle, status)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcServerInqBindingHandle(binding: *mut *mut ::core::ffi::c_void) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn RpcServerInqBindingHandle ( binding : *mut *mut ::core::ffi::c_void ) -> RPC_STATUS );
    RpcServerInqBindingHandle(binding)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcServerInqBindings(bindingvector: *mut *mut RPC_BINDING_VECTOR) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn RpcServerInqBindings ( bindingvector : *mut *mut RPC_BINDING_VECTOR ) -> RPC_STATUS );
    RpcServerInqBindings(bindingvector)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcServerInqBindingsEx(securitydescriptor: ::core::option::Option<*const ::core::ffi::c_void>, bindingvector: *mut *mut RPC_BINDING_VECTOR) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn RpcServerInqBindingsEx ( securitydescriptor : *const ::core::ffi::c_void , bindingvector : *mut *mut RPC_BINDING_VECTOR ) -> RPC_STATUS );
    RpcServerInqBindingsEx(::core::mem::transmute(securitydescriptor.unwrap_or(::std::ptr::null())), bindingvector)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcServerInqCallAttributesA(clientbinding: ::core::option::Option<*const ::core::ffi::c_void>, rpccallattributes: *mut ::core::ffi::c_void) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn RpcServerInqCallAttributesA ( clientbinding : *const ::core::ffi::c_void , rpccallattributes : *mut ::core::ffi::c_void ) -> RPC_STATUS );
    RpcServerInqCallAttributesA(::core::mem::transmute(clientbinding.unwrap_or(::std::ptr::null())), rpccallattributes)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcServerInqCallAttributesW(clientbinding: ::core::option::Option<*const ::core::ffi::c_void>, rpccallattributes: *mut ::core::ffi::c_void) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn RpcServerInqCallAttributesW ( clientbinding : *const ::core::ffi::c_void , rpccallattributes : *mut ::core::ffi::c_void ) -> RPC_STATUS );
    RpcServerInqCallAttributesW(::core::mem::transmute(clientbinding.unwrap_or(::std::ptr::null())), rpccallattributes)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcServerInqDefaultPrincNameA(authnsvc: u32, princname: *mut *mut u8) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn RpcServerInqDefaultPrincNameA ( authnsvc : u32 , princname : *mut *mut u8 ) -> RPC_STATUS );
    RpcServerInqDefaultPrincNameA(authnsvc, princname)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcServerInqDefaultPrincNameW(authnsvc: u32, princname: *mut *mut u16) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn RpcServerInqDefaultPrincNameW ( authnsvc : u32 , princname : *mut *mut u16 ) -> RPC_STATUS );
    RpcServerInqDefaultPrincNameW(authnsvc, princname)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcServerInqIf(ifspec: *const ::core::ffi::c_void, mgrtypeuuid: ::core::option::Option<*const ::windows::core::GUID>, mgrepv: *mut *mut ::core::ffi::c_void) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn RpcServerInqIf ( ifspec : *const ::core::ffi::c_void , mgrtypeuuid : *const :: windows::core::GUID , mgrepv : *mut *mut ::core::ffi::c_void ) -> RPC_STATUS );
    RpcServerInqIf(ifspec, ::core::mem::transmute(mgrtypeuuid.unwrap_or(::std::ptr::null())), mgrepv)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcServerInterfaceGroupActivate(ifgroup: *const ::core::ffi::c_void) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn RpcServerInterfaceGroupActivate ( ifgroup : *const ::core::ffi::c_void ) -> RPC_STATUS );
    RpcServerInterfaceGroupActivate(ifgroup)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcServerInterfaceGroupClose(ifgroup: *const ::core::ffi::c_void) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn RpcServerInterfaceGroupClose ( ifgroup : *const ::core::ffi::c_void ) -> RPC_STATUS );
    RpcServerInterfaceGroupClose(ifgroup)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcServerInterfaceGroupCreateA(interfaces: &[RPC_INTERFACE_TEMPLATEA], endpoints: &[RPC_ENDPOINT_TEMPLATEA], idleperiod: u32, idlecallbackfn: RPC_INTERFACE_GROUP_IDLE_CALLBACK_FN, idlecallbackcontext: *const ::core::ffi::c_void, ifgroup: *mut *mut ::core::ffi::c_void) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn RpcServerInterfaceGroupCreateA ( interfaces : *const RPC_INTERFACE_TEMPLATEA , numifs : u32 , endpoints : *const RPC_ENDPOINT_TEMPLATEA , numendpoints : u32 , idleperiod : u32 , idlecallbackfn : RPC_INTERFACE_GROUP_IDLE_CALLBACK_FN , idlecallbackcontext : *const ::core::ffi::c_void , ifgroup : *mut *mut ::core::ffi::c_void ) -> RPC_STATUS );
    RpcServerInterfaceGroupCreateA(::core::mem::transmute(interfaces.as_ptr()), interfaces.len() as _, ::core::mem::transmute(endpoints.as_ptr()), endpoints.len() as _, idleperiod, idlecallbackfn, idlecallbackcontext, ifgroup)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcServerInterfaceGroupCreateW(interfaces: &[RPC_INTERFACE_TEMPLATEW], endpoints: &[RPC_ENDPOINT_TEMPLATEW], idleperiod: u32, idlecallbackfn: RPC_INTERFACE_GROUP_IDLE_CALLBACK_FN, idlecallbackcontext: *const ::core::ffi::c_void, ifgroup: *mut *mut ::core::ffi::c_void) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn RpcServerInterfaceGroupCreateW ( interfaces : *const RPC_INTERFACE_TEMPLATEW , numifs : u32 , endpoints : *const RPC_ENDPOINT_TEMPLATEW , numendpoints : u32 , idleperiod : u32 , idlecallbackfn : RPC_INTERFACE_GROUP_IDLE_CALLBACK_FN , idlecallbackcontext : *const ::core::ffi::c_void , ifgroup : *mut *mut ::core::ffi::c_void ) -> RPC_STATUS );
    RpcServerInterfaceGroupCreateW(::core::mem::transmute(interfaces.as_ptr()), interfaces.len() as _, ::core::mem::transmute(endpoints.as_ptr()), endpoints.len() as _, idleperiod, idlecallbackfn, idlecallbackcontext, ifgroup)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcServerInterfaceGroupDeactivate(ifgroup: *const ::core::ffi::c_void, forcedeactivation: u32) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn RpcServerInterfaceGroupDeactivate ( ifgroup : *const ::core::ffi::c_void , forcedeactivation : u32 ) -> RPC_STATUS );
    RpcServerInterfaceGroupDeactivate(ifgroup, forcedeactivation)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcServerInterfaceGroupInqBindings(ifgroup: *const ::core::ffi::c_void, bindingvector: *mut *mut RPC_BINDING_VECTOR) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn RpcServerInterfaceGroupInqBindings ( ifgroup : *const ::core::ffi::c_void , bindingvector : *mut *mut RPC_BINDING_VECTOR ) -> RPC_STATUS );
    RpcServerInterfaceGroupInqBindings(ifgroup, bindingvector)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcServerListen(minimumcallthreads: u32, maxcalls: u32, dontwait: u32) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn RpcServerListen ( minimumcallthreads : u32 , maxcalls : u32 , dontwait : u32 ) -> RPC_STATUS );
    RpcServerListen(minimumcallthreads, maxcalls, dontwait)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcServerRegisterAuthInfoA(serverprincname: ::core::option::Option<*const u8>, authnsvc: u32, getkeyfn: RPC_AUTH_KEY_RETRIEVAL_FN, arg: ::core::option::Option<*const ::core::ffi::c_void>) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn RpcServerRegisterAuthInfoA ( serverprincname : *const u8 , authnsvc : u32 , getkeyfn : RPC_AUTH_KEY_RETRIEVAL_FN , arg : *const ::core::ffi::c_void ) -> RPC_STATUS );
    RpcServerRegisterAuthInfoA(::core::mem::transmute(serverprincname.unwrap_or(::std::ptr::null())), authnsvc, getkeyfn, ::core::mem::transmute(arg.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcServerRegisterAuthInfoW(serverprincname: ::core::option::Option<*const u16>, authnsvc: u32, getkeyfn: RPC_AUTH_KEY_RETRIEVAL_FN, arg: ::core::option::Option<*const ::core::ffi::c_void>) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn RpcServerRegisterAuthInfoW ( serverprincname : *const u16 , authnsvc : u32 , getkeyfn : RPC_AUTH_KEY_RETRIEVAL_FN , arg : *const ::core::ffi::c_void ) -> RPC_STATUS );
    RpcServerRegisterAuthInfoW(::core::mem::transmute(serverprincname.unwrap_or(::std::ptr::null())), authnsvc, getkeyfn, ::core::mem::transmute(arg.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcServerRegisterIf(ifspec: *const ::core::ffi::c_void, mgrtypeuuid: ::core::option::Option<*const ::windows::core::GUID>, mgrepv: ::core::option::Option<*const ::core::ffi::c_void>) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn RpcServerRegisterIf ( ifspec : *const ::core::ffi::c_void , mgrtypeuuid : *const :: windows::core::GUID , mgrepv : *const ::core::ffi::c_void ) -> RPC_STATUS );
    RpcServerRegisterIf(ifspec, ::core::mem::transmute(mgrtypeuuid.unwrap_or(::std::ptr::null())), ::core::mem::transmute(mgrepv.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcServerRegisterIf2(ifspec: *const ::core::ffi::c_void, mgrtypeuuid: ::core::option::Option<*const ::windows::core::GUID>, mgrepv: ::core::option::Option<*const ::core::ffi::c_void>, flags: u32, maxcalls: u32, maxrpcsize: u32, ifcallbackfn: RPC_IF_CALLBACK_FN) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn RpcServerRegisterIf2 ( ifspec : *const ::core::ffi::c_void , mgrtypeuuid : *const :: windows::core::GUID , mgrepv : *const ::core::ffi::c_void , flags : u32 , maxcalls : u32 , maxrpcsize : u32 , ifcallbackfn : RPC_IF_CALLBACK_FN ) -> RPC_STATUS );
    RpcServerRegisterIf2(ifspec, ::core::mem::transmute(mgrtypeuuid.unwrap_or(::std::ptr::null())), ::core::mem::transmute(mgrepv.unwrap_or(::std::ptr::null())), flags, maxcalls, maxrpcsize, ifcallbackfn)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcServerRegisterIf3(ifspec: *const ::core::ffi::c_void, mgrtypeuuid: ::core::option::Option<*const ::windows::core::GUID>, mgrepv: ::core::option::Option<*const ::core::ffi::c_void>, flags: u32, maxcalls: u32, maxrpcsize: u32, ifcallback: RPC_IF_CALLBACK_FN, securitydescriptor: ::core::option::Option<*const ::core::ffi::c_void>) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn RpcServerRegisterIf3 ( ifspec : *const ::core::ffi::c_void , mgrtypeuuid : *const :: windows::core::GUID , mgrepv : *const ::core::ffi::c_void , flags : u32 , maxcalls : u32 , maxrpcsize : u32 , ifcallback : RPC_IF_CALLBACK_FN , securitydescriptor : *const ::core::ffi::c_void ) -> RPC_STATUS );
    RpcServerRegisterIf3(ifspec, ::core::mem::transmute(mgrtypeuuid.unwrap_or(::std::ptr::null())), ::core::mem::transmute(mgrepv.unwrap_or(::std::ptr::null())), flags, maxcalls, maxrpcsize, ifcallback, ::core::mem::transmute(securitydescriptor.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcServerRegisterIfEx(ifspec: *const ::core::ffi::c_void, mgrtypeuuid: ::core::option::Option<*const ::windows::core::GUID>, mgrepv: ::core::option::Option<*const ::core::ffi::c_void>, flags: u32, maxcalls: u32, ifcallback: RPC_IF_CALLBACK_FN) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn RpcServerRegisterIfEx ( ifspec : *const ::core::ffi::c_void , mgrtypeuuid : *const :: windows::core::GUID , mgrepv : *const ::core::ffi::c_void , flags : u32 , maxcalls : u32 , ifcallback : RPC_IF_CALLBACK_FN ) -> RPC_STATUS );
    RpcServerRegisterIfEx(ifspec, ::core::mem::transmute(mgrtypeuuid.unwrap_or(::std::ptr::null())), ::core::mem::transmute(mgrepv.unwrap_or(::std::ptr::null())), flags, maxcalls, ifcallback)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[inline]
pub unsafe fn RpcServerSubscribeForNotification(binding: ::core::option::Option<*const ::core::ffi::c_void>, notification: RPC_NOTIFICATIONS, notificationtype: RPC_NOTIFICATION_TYPES, notificationinfo: *const RPC_ASYNC_NOTIFICATION_INFO) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn RpcServerSubscribeForNotification ( binding : *const ::core::ffi::c_void , notification : RPC_NOTIFICATIONS , notificationtype : RPC_NOTIFICATION_TYPES , notificationinfo : *const RPC_ASYNC_NOTIFICATION_INFO ) -> RPC_STATUS );
    RpcServerSubscribeForNotification(::core::mem::transmute(binding.unwrap_or(::std::ptr::null())), notification, notificationtype, notificationinfo)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcServerTestCancel(bindinghandle: ::core::option::Option<*const ::core::ffi::c_void>) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn RpcServerTestCancel ( bindinghandle : *const ::core::ffi::c_void ) -> RPC_STATUS );
    RpcServerTestCancel(::core::mem::transmute(bindinghandle.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcServerUnregisterIf(ifspec: ::core::option::Option<*const ::core::ffi::c_void>, mgrtypeuuid: ::core::option::Option<*const ::windows::core::GUID>, waitforcallstocomplete: u32) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn RpcServerUnregisterIf ( ifspec : *const ::core::ffi::c_void , mgrtypeuuid : *const :: windows::core::GUID , waitforcallstocomplete : u32 ) -> RPC_STATUS );
    RpcServerUnregisterIf(::core::mem::transmute(ifspec.unwrap_or(::std::ptr::null())), ::core::mem::transmute(mgrtypeuuid.unwrap_or(::std::ptr::null())), waitforcallstocomplete)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcServerUnregisterIfEx(ifspec: ::core::option::Option<*const ::core::ffi::c_void>, mgrtypeuuid: ::core::option::Option<*const ::windows::core::GUID>, rundowncontexthandles: i32) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn RpcServerUnregisterIfEx ( ifspec : *const ::core::ffi::c_void , mgrtypeuuid : *const :: windows::core::GUID , rundowncontexthandles : i32 ) -> RPC_STATUS );
    RpcServerUnregisterIfEx(::core::mem::transmute(ifspec.unwrap_or(::std::ptr::null())), ::core::mem::transmute(mgrtypeuuid.unwrap_or(::std::ptr::null())), rundowncontexthandles)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcServerUnsubscribeForNotification(binding: ::core::option::Option<*const ::core::ffi::c_void>, notification: RPC_NOTIFICATIONS, notificationsqueued: *mut u32) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn RpcServerUnsubscribeForNotification ( binding : *const ::core::ffi::c_void , notification : RPC_NOTIFICATIONS , notificationsqueued : *mut u32 ) -> RPC_STATUS );
    RpcServerUnsubscribeForNotification(::core::mem::transmute(binding.unwrap_or(::std::ptr::null())), notification, notificationsqueued)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcServerUseAllProtseqs(maxcalls: u32, securitydescriptor: ::core::option::Option<*const ::core::ffi::c_void>) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn RpcServerUseAllProtseqs ( maxcalls : u32 , securitydescriptor : *const ::core::ffi::c_void ) -> RPC_STATUS );
    RpcServerUseAllProtseqs(maxcalls, ::core::mem::transmute(securitydescriptor.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcServerUseAllProtseqsEx(maxcalls: u32, securitydescriptor: ::core::option::Option<*const ::core::ffi::c_void>, policy: *const RPC_POLICY) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn RpcServerUseAllProtseqsEx ( maxcalls : u32 , securitydescriptor : *const ::core::ffi::c_void , policy : *const RPC_POLICY ) -> RPC_STATUS );
    RpcServerUseAllProtseqsEx(maxcalls, ::core::mem::transmute(securitydescriptor.unwrap_or(::std::ptr::null())), policy)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcServerUseAllProtseqsIf(maxcalls: u32, ifspec: *const ::core::ffi::c_void, securitydescriptor: ::core::option::Option<*const ::core::ffi::c_void>) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn RpcServerUseAllProtseqsIf ( maxcalls : u32 , ifspec : *const ::core::ffi::c_void , securitydescriptor : *const ::core::ffi::c_void ) -> RPC_STATUS );
    RpcServerUseAllProtseqsIf(maxcalls, ifspec, ::core::mem::transmute(securitydescriptor.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcServerUseAllProtseqsIfEx(maxcalls: u32, ifspec: *const ::core::ffi::c_void, securitydescriptor: ::core::option::Option<*const ::core::ffi::c_void>, policy: *const RPC_POLICY) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn RpcServerUseAllProtseqsIfEx ( maxcalls : u32 , ifspec : *const ::core::ffi::c_void , securitydescriptor : *const ::core::ffi::c_void , policy : *const RPC_POLICY ) -> RPC_STATUS );
    RpcServerUseAllProtseqsIfEx(maxcalls, ifspec, ::core::mem::transmute(securitydescriptor.unwrap_or(::std::ptr::null())), policy)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcServerUseProtseqA(protseq: *const u8, maxcalls: u32, securitydescriptor: ::core::option::Option<*const ::core::ffi::c_void>) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn RpcServerUseProtseqA ( protseq : *const u8 , maxcalls : u32 , securitydescriptor : *const ::core::ffi::c_void ) -> RPC_STATUS );
    RpcServerUseProtseqA(protseq, maxcalls, ::core::mem::transmute(securitydescriptor.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcServerUseProtseqEpA(protseq: *const u8, maxcalls: u32, endpoint: *const u8, securitydescriptor: ::core::option::Option<*const ::core::ffi::c_void>) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn RpcServerUseProtseqEpA ( protseq : *const u8 , maxcalls : u32 , endpoint : *const u8 , securitydescriptor : *const ::core::ffi::c_void ) -> RPC_STATUS );
    RpcServerUseProtseqEpA(protseq, maxcalls, endpoint, ::core::mem::transmute(securitydescriptor.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcServerUseProtseqEpExA(protseq: *const u8, maxcalls: u32, endpoint: *const u8, securitydescriptor: ::core::option::Option<*const ::core::ffi::c_void>, policy: *const RPC_POLICY) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn RpcServerUseProtseqEpExA ( protseq : *const u8 , maxcalls : u32 , endpoint : *const u8 , securitydescriptor : *const ::core::ffi::c_void , policy : *const RPC_POLICY ) -> RPC_STATUS );
    RpcServerUseProtseqEpExA(protseq, maxcalls, endpoint, ::core::mem::transmute(securitydescriptor.unwrap_or(::std::ptr::null())), policy)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcServerUseProtseqEpExW(protseq: *const u16, maxcalls: u32, endpoint: *const u16, securitydescriptor: ::core::option::Option<*const ::core::ffi::c_void>, policy: *const RPC_POLICY) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn RpcServerUseProtseqEpExW ( protseq : *const u16 , maxcalls : u32 , endpoint : *const u16 , securitydescriptor : *const ::core::ffi::c_void , policy : *const RPC_POLICY ) -> RPC_STATUS );
    RpcServerUseProtseqEpExW(protseq, maxcalls, endpoint, ::core::mem::transmute(securitydescriptor.unwrap_or(::std::ptr::null())), policy)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcServerUseProtseqEpW(protseq: *const u16, maxcalls: u32, endpoint: *const u16, securitydescriptor: ::core::option::Option<*const ::core::ffi::c_void>) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn RpcServerUseProtseqEpW ( protseq : *const u16 , maxcalls : u32 , endpoint : *const u16 , securitydescriptor : *const ::core::ffi::c_void ) -> RPC_STATUS );
    RpcServerUseProtseqEpW(protseq, maxcalls, endpoint, ::core::mem::transmute(securitydescriptor.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcServerUseProtseqExA(protseq: *const u8, maxcalls: u32, securitydescriptor: ::core::option::Option<*const ::core::ffi::c_void>, policy: *const RPC_POLICY) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn RpcServerUseProtseqExA ( protseq : *const u8 , maxcalls : u32 , securitydescriptor : *const ::core::ffi::c_void , policy : *const RPC_POLICY ) -> RPC_STATUS );
    RpcServerUseProtseqExA(protseq, maxcalls, ::core::mem::transmute(securitydescriptor.unwrap_or(::std::ptr::null())), policy)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcServerUseProtseqExW(protseq: *const u16, maxcalls: u32, securitydescriptor: ::core::option::Option<*const ::core::ffi::c_void>, policy: *const RPC_POLICY) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn RpcServerUseProtseqExW ( protseq : *const u16 , maxcalls : u32 , securitydescriptor : *const ::core::ffi::c_void , policy : *const RPC_POLICY ) -> RPC_STATUS );
    RpcServerUseProtseqExW(protseq, maxcalls, ::core::mem::transmute(securitydescriptor.unwrap_or(::std::ptr::null())), policy)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcServerUseProtseqIfA(protseq: *const u8, maxcalls: u32, ifspec: *const ::core::ffi::c_void, securitydescriptor: ::core::option::Option<*const ::core::ffi::c_void>) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn RpcServerUseProtseqIfA ( protseq : *const u8 , maxcalls : u32 , ifspec : *const ::core::ffi::c_void , securitydescriptor : *const ::core::ffi::c_void ) -> RPC_STATUS );
    RpcServerUseProtseqIfA(protseq, maxcalls, ifspec, ::core::mem::transmute(securitydescriptor.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcServerUseProtseqIfExA(protseq: *const u8, maxcalls: u32, ifspec: *const ::core::ffi::c_void, securitydescriptor: ::core::option::Option<*const ::core::ffi::c_void>, policy: *const RPC_POLICY) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn RpcServerUseProtseqIfExA ( protseq : *const u8 , maxcalls : u32 , ifspec : *const ::core::ffi::c_void , securitydescriptor : *const ::core::ffi::c_void , policy : *const RPC_POLICY ) -> RPC_STATUS );
    RpcServerUseProtseqIfExA(protseq, maxcalls, ifspec, ::core::mem::transmute(securitydescriptor.unwrap_or(::std::ptr::null())), policy)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcServerUseProtseqIfExW(protseq: *const u16, maxcalls: u32, ifspec: *const ::core::ffi::c_void, securitydescriptor: ::core::option::Option<*const ::core::ffi::c_void>, policy: *const RPC_POLICY) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn RpcServerUseProtseqIfExW ( protseq : *const u16 , maxcalls : u32 , ifspec : *const ::core::ffi::c_void , securitydescriptor : *const ::core::ffi::c_void , policy : *const RPC_POLICY ) -> RPC_STATUS );
    RpcServerUseProtseqIfExW(protseq, maxcalls, ifspec, ::core::mem::transmute(securitydescriptor.unwrap_or(::std::ptr::null())), policy)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcServerUseProtseqIfW(protseq: *const u16, maxcalls: u32, ifspec: *const ::core::ffi::c_void, securitydescriptor: ::core::option::Option<*const ::core::ffi::c_void>) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn RpcServerUseProtseqIfW ( protseq : *const u16 , maxcalls : u32 , ifspec : *const ::core::ffi::c_void , securitydescriptor : *const ::core::ffi::c_void ) -> RPC_STATUS );
    RpcServerUseProtseqIfW(protseq, maxcalls, ifspec, ::core::mem::transmute(securitydescriptor.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcServerUseProtseqW(protseq: *const u16, maxcalls: u32, securitydescriptor: ::core::option::Option<*const ::core::ffi::c_void>) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn RpcServerUseProtseqW ( protseq : *const u16 , maxcalls : u32 , securitydescriptor : *const ::core::ffi::c_void ) -> RPC_STATUS );
    RpcServerUseProtseqW(protseq, maxcalls, ::core::mem::transmute(securitydescriptor.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcServerYield() {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn RpcServerYield ( ) -> ( ) );
    RpcServerYield()
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcSmAllocate(size: usize, pstatus: *mut RPC_STATUS) -> *mut ::core::ffi::c_void {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn RpcSmAllocate ( size : usize , pstatus : *mut RPC_STATUS ) -> *mut ::core::ffi::c_void );
    RpcSmAllocate(size, pstatus)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcSmClientFree(pnodetofree: *const ::core::ffi::c_void) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn RpcSmClientFree ( pnodetofree : *const ::core::ffi::c_void ) -> RPC_STATUS );
    RpcSmClientFree(pnodetofree)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcSmDestroyClientContext(contexthandle: *const *const ::core::ffi::c_void) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn RpcSmDestroyClientContext ( contexthandle : *const *const ::core::ffi::c_void ) -> RPC_STATUS );
    RpcSmDestroyClientContext(contexthandle)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcSmDisableAllocate() -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn RpcSmDisableAllocate ( ) -> RPC_STATUS );
    RpcSmDisableAllocate()
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcSmEnableAllocate() -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn RpcSmEnableAllocate ( ) -> RPC_STATUS );
    RpcSmEnableAllocate()
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcSmFree(nodetofree: *const ::core::ffi::c_void) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn RpcSmFree ( nodetofree : *const ::core::ffi::c_void ) -> RPC_STATUS );
    RpcSmFree(nodetofree)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcSmGetThreadHandle(pstatus: *mut RPC_STATUS) -> *mut ::core::ffi::c_void {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn RpcSmGetThreadHandle ( pstatus : *mut RPC_STATUS ) -> *mut ::core::ffi::c_void );
    RpcSmGetThreadHandle(pstatus)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcSmSetClientAllocFree(clientalloc: RPC_CLIENT_ALLOC, clientfree: RPC_CLIENT_FREE) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn RpcSmSetClientAllocFree ( clientalloc : RPC_CLIENT_ALLOC , clientfree : RPC_CLIENT_FREE ) -> RPC_STATUS );
    RpcSmSetClientAllocFree(clientalloc, clientfree)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcSmSetThreadHandle(id: *const ::core::ffi::c_void) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn RpcSmSetThreadHandle ( id : *const ::core::ffi::c_void ) -> RPC_STATUS );
    RpcSmSetThreadHandle(id)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcSmSwapClientAllocFree(clientalloc: RPC_CLIENT_ALLOC, clientfree: RPC_CLIENT_FREE, oldclientalloc: *mut RPC_CLIENT_ALLOC, oldclientfree: *mut RPC_CLIENT_FREE) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn RpcSmSwapClientAllocFree ( clientalloc : RPC_CLIENT_ALLOC , clientfree : RPC_CLIENT_FREE , oldclientalloc : *mut RPC_CLIENT_ALLOC , oldclientfree : *mut RPC_CLIENT_FREE ) -> RPC_STATUS );
    RpcSmSwapClientAllocFree(clientalloc, clientfree, oldclientalloc, oldclientfree)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcSsAllocate(size: usize) -> *mut ::core::ffi::c_void {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn RpcSsAllocate ( size : usize ) -> *mut ::core::ffi::c_void );
    RpcSsAllocate(size)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcSsContextLockExclusive(serverbindinghandle: ::core::option::Option<*const ::core::ffi::c_void>, usercontext: *const ::core::ffi::c_void) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn RpcSsContextLockExclusive ( serverbindinghandle : *const ::core::ffi::c_void , usercontext : *const ::core::ffi::c_void ) -> RPC_STATUS );
    RpcSsContextLockExclusive(::core::mem::transmute(serverbindinghandle.unwrap_or(::std::ptr::null())), usercontext)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcSsContextLockShared(serverbindinghandle: *const ::core::ffi::c_void, usercontext: *const ::core::ffi::c_void) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn RpcSsContextLockShared ( serverbindinghandle : *const ::core::ffi::c_void , usercontext : *const ::core::ffi::c_void ) -> RPC_STATUS );
    RpcSsContextLockShared(serverbindinghandle, usercontext)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcSsDestroyClientContext(contexthandle: *const *const ::core::ffi::c_void) {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn RpcSsDestroyClientContext ( contexthandle : *const *const ::core::ffi::c_void ) -> ( ) );
    RpcSsDestroyClientContext(contexthandle)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcSsDisableAllocate() {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn RpcSsDisableAllocate ( ) -> ( ) );
    RpcSsDisableAllocate()
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcSsDontSerializeContext() {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn RpcSsDontSerializeContext ( ) -> ( ) );
    RpcSsDontSerializeContext()
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcSsEnableAllocate() {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn RpcSsEnableAllocate ( ) -> ( ) );
    RpcSsEnableAllocate()
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcSsFree(nodetofree: *const ::core::ffi::c_void) {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn RpcSsFree ( nodetofree : *const ::core::ffi::c_void ) -> ( ) );
    RpcSsFree(nodetofree)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcSsGetContextBinding(contexthandle: *const ::core::ffi::c_void, binding: *mut *mut ::core::ffi::c_void) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn RpcSsGetContextBinding ( contexthandle : *const ::core::ffi::c_void , binding : *mut *mut ::core::ffi::c_void ) -> RPC_STATUS );
    RpcSsGetContextBinding(contexthandle, binding)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcSsGetThreadHandle() -> *mut ::core::ffi::c_void {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn RpcSsGetThreadHandle ( ) -> *mut ::core::ffi::c_void );
    RpcSsGetThreadHandle()
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcSsSetClientAllocFree(clientalloc: RPC_CLIENT_ALLOC, clientfree: RPC_CLIENT_FREE) {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn RpcSsSetClientAllocFree ( clientalloc : RPC_CLIENT_ALLOC , clientfree : RPC_CLIENT_FREE ) -> ( ) );
    RpcSsSetClientAllocFree(clientalloc, clientfree)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcSsSetThreadHandle(id: *const ::core::ffi::c_void) {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn RpcSsSetThreadHandle ( id : *const ::core::ffi::c_void ) -> ( ) );
    RpcSsSetThreadHandle(id)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcSsSwapClientAllocFree(clientalloc: RPC_CLIENT_ALLOC, clientfree: RPC_CLIENT_FREE, oldclientalloc: *mut RPC_CLIENT_ALLOC, oldclientfree: *mut RPC_CLIENT_FREE) {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn RpcSsSwapClientAllocFree ( clientalloc : RPC_CLIENT_ALLOC , clientfree : RPC_CLIENT_FREE , oldclientalloc : *mut RPC_CLIENT_ALLOC , oldclientfree : *mut RPC_CLIENT_FREE ) -> ( ) );
    RpcSsSwapClientAllocFree(clientalloc, clientfree, oldclientalloc, oldclientfree)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcStringBindingComposeA(objuuid: ::core::option::Option<*const u8>, protseq: ::core::option::Option<*const u8>, networkaddr: ::core::option::Option<*const u8>, endpoint: ::core::option::Option<*const u8>, options: ::core::option::Option<*const u8>, stringbinding: ::core::option::Option<*mut *mut u8>) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn RpcStringBindingComposeA ( objuuid : *const u8 , protseq : *const u8 , networkaddr : *const u8 , endpoint : *const u8 , options : *const u8 , stringbinding : *mut *mut u8 ) -> RPC_STATUS );
    RpcStringBindingComposeA(::core::mem::transmute(objuuid.unwrap_or(::std::ptr::null())), ::core::mem::transmute(protseq.unwrap_or(::std::ptr::null())), ::core::mem::transmute(networkaddr.unwrap_or(::std::ptr::null())), ::core::mem::transmute(endpoint.unwrap_or(::std::ptr::null())), ::core::mem::transmute(options.unwrap_or(::std::ptr::null())), ::core::mem::transmute(stringbinding.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcStringBindingComposeW(objuuid: ::core::option::Option<*const u16>, protseq: ::core::option::Option<*const u16>, networkaddr: ::core::option::Option<*const u16>, endpoint: ::core::option::Option<*const u16>, options: ::core::option::Option<*const u16>, stringbinding: ::core::option::Option<*mut *mut u16>) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn RpcStringBindingComposeW ( objuuid : *const u16 , protseq : *const u16 , networkaddr : *const u16 , endpoint : *const u16 , options : *const u16 , stringbinding : *mut *mut u16 ) -> RPC_STATUS );
    RpcStringBindingComposeW(::core::mem::transmute(objuuid.unwrap_or(::std::ptr::null())), ::core::mem::transmute(protseq.unwrap_or(::std::ptr::null())), ::core::mem::transmute(networkaddr.unwrap_or(::std::ptr::null())), ::core::mem::transmute(endpoint.unwrap_or(::std::ptr::null())), ::core::mem::transmute(options.unwrap_or(::std::ptr::null())), ::core::mem::transmute(stringbinding.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcStringBindingParseA(stringbinding: *const u8, objuuid: ::core::option::Option<*mut *mut u8>, protseq: ::core::option::Option<*mut *mut u8>, networkaddr: ::core::option::Option<*mut *mut u8>, endpoint: ::core::option::Option<*mut *mut u8>, networkoptions: ::core::option::Option<*mut *mut u8>) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn RpcStringBindingParseA ( stringbinding : *const u8 , objuuid : *mut *mut u8 , protseq : *mut *mut u8 , networkaddr : *mut *mut u8 , endpoint : *mut *mut u8 , networkoptions : *mut *mut u8 ) -> RPC_STATUS );
    RpcStringBindingParseA(stringbinding, ::core::mem::transmute(objuuid.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(protseq.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(networkaddr.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(endpoint.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(networkoptions.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcStringBindingParseW(stringbinding: *const u16, objuuid: ::core::option::Option<*mut *mut u16>, protseq: ::core::option::Option<*mut *mut u16>, networkaddr: ::core::option::Option<*mut *mut u16>, endpoint: ::core::option::Option<*mut *mut u16>, networkoptions: ::core::option::Option<*mut *mut u16>) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn RpcStringBindingParseW ( stringbinding : *const u16 , objuuid : *mut *mut u16 , protseq : *mut *mut u16 , networkaddr : *mut *mut u16 , endpoint : *mut *mut u16 , networkoptions : *mut *mut u16 ) -> RPC_STATUS );
    RpcStringBindingParseW(stringbinding, ::core::mem::transmute(objuuid.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(protseq.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(networkaddr.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(endpoint.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(networkoptions.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcStringFreeA(string: *mut *mut u8) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn RpcStringFreeA ( string : *mut *mut u8 ) -> RPC_STATUS );
    RpcStringFreeA(string)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcStringFreeW(string: *mut *mut u16) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn RpcStringFreeW ( string : *mut *mut u16 ) -> RPC_STATUS );
    RpcStringFreeW(string)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcTestCancel() -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn RpcTestCancel ( ) -> RPC_STATUS );
    RpcTestCancel()
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcUserFree(asynchandle: *mut ::core::ffi::c_void, pbuffer: *mut ::core::ffi::c_void) {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn RpcUserFree ( asynchandle : *mut ::core::ffi::c_void , pbuffer : *mut ::core::ffi::c_void ) -> ( ) );
    RpcUserFree(asynchandle, pbuffer)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn UuidCompare(uuid1: *const ::windows::core::GUID, uuid2: *const ::windows::core::GUID, status: *mut RPC_STATUS) -> i32 {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn UuidCompare ( uuid1 : *const :: windows::core::GUID , uuid2 : *const :: windows::core::GUID , status : *mut RPC_STATUS ) -> i32 );
    UuidCompare(uuid1, uuid2, status)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn UuidCreate(uuid: *mut ::windows::core::GUID) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn UuidCreate ( uuid : *mut :: windows::core::GUID ) -> RPC_STATUS );
    UuidCreate(uuid)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn UuidCreateNil(niluuid: *mut ::windows::core::GUID) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn UuidCreateNil ( niluuid : *mut :: windows::core::GUID ) -> RPC_STATUS );
    UuidCreateNil(niluuid)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn UuidCreateSequential(uuid: *mut ::windows::core::GUID) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn UuidCreateSequential ( uuid : *mut :: windows::core::GUID ) -> RPC_STATUS );
    UuidCreateSequential(uuid)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn UuidEqual(uuid1: *const ::windows::core::GUID, uuid2: *const ::windows::core::GUID, status: *mut RPC_STATUS) -> i32 {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn UuidEqual ( uuid1 : *const :: windows::core::GUID , uuid2 : *const :: windows::core::GUID , status : *mut RPC_STATUS ) -> i32 );
    UuidEqual(uuid1, uuid2, status)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn UuidFromStringA(stringuuid: ::core::option::Option<*const u8>, uuid: *mut ::windows::core::GUID) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn UuidFromStringA ( stringuuid : *const u8 , uuid : *mut :: windows::core::GUID ) -> RPC_STATUS );
    UuidFromStringA(::core::mem::transmute(stringuuid.unwrap_or(::std::ptr::null())), uuid)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn UuidFromStringW(stringuuid: ::core::option::Option<*const u16>, uuid: *mut ::windows::core::GUID) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn UuidFromStringW ( stringuuid : *const u16 , uuid : *mut :: windows::core::GUID ) -> RPC_STATUS );
    UuidFromStringW(::core::mem::transmute(stringuuid.unwrap_or(::std::ptr::null())), uuid)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn UuidHash(uuid: *const ::windows::core::GUID, status: *mut RPC_STATUS) -> u16 {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn UuidHash ( uuid : *const :: windows::core::GUID , status : *mut RPC_STATUS ) -> u16 );
    UuidHash(uuid, status)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn UuidIsNil(uuid: *const ::windows::core::GUID, status: *mut RPC_STATUS) -> i32 {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn UuidIsNil ( uuid : *const :: windows::core::GUID , status : *mut RPC_STATUS ) -> i32 );
    UuidIsNil(uuid, status)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn UuidToStringA(uuid: *const ::windows::core::GUID, stringuuid: *mut *mut u8) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn UuidToStringA ( uuid : *const :: windows::core::GUID , stringuuid : *mut *mut u8 ) -> RPC_STATUS );
    UuidToStringA(uuid, stringuuid)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn UuidToStringW(uuid: *const ::windows::core::GUID, stringuuid: *mut *mut u16) -> RPC_STATUS {
    ::windows::imp::link ! ( "rpcrt4.dll""system" fn UuidToStringW ( uuid : *const :: windows::core::GUID , stringuuid : *mut *mut u16 ) -> RPC_STATUS );
    UuidToStringW(uuid, stringuuid)
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const DCE_C_ERROR_STRING_LEN: u32 = 256u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const EEInfoGCCOM: u32 = 11u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const EEInfoGCFRS: u32 = 12u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const EEInfoNextRecordsMissing: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const EEInfoPreviousRecordsMissing: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const EEInfoUseFileTime: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const INVALID_FRAGMENT_ID: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const MIDL_WINRT_TYPE_SERIALIZATION_INFO_CURRENT_VERSION: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const MaxNumberOfEEInfoParams: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const MidlInterceptionInfoVersionOne: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const MidlWinrtTypeSerializationInfoVersionOne: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const NDR64_FC_AUTO_HANDLE: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const NDR64_FC_BIND_GENERIC: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const NDR64_FC_BIND_PRIMITIVE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const NDR64_FC_CALLBACK_HANDLE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const NDR64_FC_EXPLICIT_HANDLE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const NDR64_FC_NO_HANDLE: u32 = 5u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const NDR_CUSTOM_OR_DEFAULT_ALLOCATOR: u32 = 268435456u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const NDR_DEFAULT_ALLOCATOR: u32 = 536870912u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const NT351_INTERFACE_SIZE: u32 = 64u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPCFLG_ACCESSIBILITY_BIT1: u32 = 1048576u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPCFLG_ACCESSIBILITY_BIT2: u32 = 2097152u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPCFLG_ACCESS_LOCAL: u32 = 4194304u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPCFLG_ASYNCHRONOUS: u32 = 1073741824u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPCFLG_AUTO_COMPLETE: u32 = 134217728u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPCFLG_HAS_CALLBACK: u32 = 67108864u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPCFLG_HAS_GUARANTEE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPCFLG_HAS_MULTI_SYNTAXES: u32 = 33554432u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPCFLG_INPUT_SYNCHRONOUS: u32 = 536870912u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPCFLG_LOCAL_CALL: u32 = 268435456u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPCFLG_MESSAGE: u32 = 16777216u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPCFLG_NDR64_CONTAINS_ARM_LAYOUT: u32 = 67108864u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPCFLG_NON_NDR: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPCFLG_SENDER_WAITING_FOR_REPLY: u32 = 8388608u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPCFLG_WINRT_REMOTE_ASYNC: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_BHO_EXCLUSIVE_AND_GUARANTEED: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_BHT_OBJECT_UUID_VALID: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_BUFFER_ASYNC: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_BUFFER_COMPLETE: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_BUFFER_EXTRA: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_BUFFER_NONOTIFY: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_BUFFER_PARTIAL: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_CALL_ATTRIBUTES_VERSION: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_CALL_STATUS_CANCELLED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_CALL_STATUS_DISCONNECTED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_CONTEXT_HANDLE_DEFAULT_FLAGS: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_CONTEXT_HANDLE_DONT_SERIALIZE: u32 = 536870912u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_CONTEXT_HANDLE_FLAGS: u32 = 805306368u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_CONTEXT_HANDLE_SERIALIZE: u32 = 268435456u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_AUTHN_CLOUD_AP: u32 = 36u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_AUTHN_DCE_PRIVATE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_AUTHN_DCE_PUBLIC: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_AUTHN_DEC_PUBLIC: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_AUTHN_DEFAULT: i32 = -1i32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_AUTHN_DIGEST: u32 = 21u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_AUTHN_DPA: u32 = 17u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_AUTHN_GSS_KERBEROS: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_AUTHN_GSS_NEGOTIATE: u32 = 9u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_AUTHN_GSS_SCHANNEL: u32 = 14u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_AUTHN_KERNEL: u32 = 20u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_AUTHN_LIVEXP_SSP: u32 = 35u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_AUTHN_LIVE_SSP: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_AUTHN_MQ: u32 = 100u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_AUTHN_MSN: u32 = 18u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_AUTHN_MSONLINE: u32 = 82u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_AUTHN_NEGO_EXTENDER: u32 = 30u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_AUTHN_NONE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_AUTHN_PKU2U: u32 = 31u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_AUTHN_WINNT: u32 = 10u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_AUTHZ_DCE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_AUTHZ_DEFAULT: u32 = 4294967295u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_AUTHZ_NAME: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_AUTHZ_NONE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_BINDING_DEFAULT_TIMEOUT: u32 = 5u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_BINDING_INFINITE_TIMEOUT: u32 = 10u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_BINDING_MAX_TIMEOUT: u32 = 9u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_BINDING_MIN_TIMEOUT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_BIND_TO_ALL_NICS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_CANCEL_INFINITE_TIMEOUT: i32 = -1i32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_DONT_FAIL: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_EP_ALL_ELTS: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_EP_MATCH_BY_BOTH: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_EP_MATCH_BY_IF: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_EP_MATCH_BY_OBJ: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_FULL_CERT_CHAIN: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_HTTP_AUTHN_SCHEME_BASIC: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_HTTP_AUTHN_SCHEME_CERT: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_HTTP_AUTHN_SCHEME_DIGEST: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_HTTP_AUTHN_SCHEME_NEGOTIATE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_HTTP_AUTHN_SCHEME_NTLM: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_HTTP_AUTHN_SCHEME_PASSPORT: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_LISTEN_MAX_CALLS_DEFAULT: u32 = 1234u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_MGMT_INQ_IF_IDS: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_MGMT_INQ_PRINC_NAME: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_MGMT_INQ_STATS: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_MGMT_IS_SERVER_LISTEN: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_MGMT_STOP_SERVER_LISTEN: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_MQ_AUTHN_LEVEL_NONE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_MQ_AUTHN_LEVEL_PKT_INTEGRITY: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_MQ_AUTHN_LEVEL_PKT_PRIVACY: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_MQ_CLEAR_ON_OPEN: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_MQ_EXPRESS: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_MQ_JOURNAL_ALWAYS: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_MQ_JOURNAL_DEADLETTER: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_MQ_JOURNAL_NONE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_MQ_PERMANENT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_MQ_RECOVERABLE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_MQ_TEMPORARY: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_MQ_USE_EXISTING_SECURITY: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_NOTIFY_ON_SEND_COMPLETE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_NS_DEFAULT_EXP_AGE: i32 = -1i32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_OPT_ASYNC_BLOCK: u32 = 15u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_OPT_BINDING_NONCAUSAL: u32 = 9u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_OPT_CALL_TIMEOUT: u32 = 12u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_OPT_COOKIE_AUTH: u32 = 7u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_OPT_DONT_LINGER: u32 = 13u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_OPT_MAX_OPTIONS: u32 = 12u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_OPT_MQ_ACKNOWLEDGE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_OPT_MQ_AUTHN_LEVEL: u32 = 6u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_OPT_MQ_AUTHN_SERVICE: u32 = 5u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_OPT_MQ_DELIVERY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_OPT_MQ_JOURNAL: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_OPT_MQ_PRIORITY: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_OPT_MQ_TIME_TO_BE_RECEIVED: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_OPT_MQ_TIME_TO_REACH_QUEUE: u32 = 7u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_OPT_OPTIMIZE_TIME: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_OPT_PRIVATE_BREAK_ON_SUSPEND: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_OPT_PRIVATE_DO_NOT_DISTURB: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_OPT_PRIVATE_SUPPRESS_WAKE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_OPT_RESOURCE_TYPE_UUID: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_OPT_SECURITY_CALLBACK: u32 = 10u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_OPT_SESSION_ID: u32 = 6u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_OPT_TRANS_SEND_BUFFER_SIZE: u32 = 5u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_OPT_TRUST_PEER: u32 = 14u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_OPT_UNIQUE_BINDING: u32 = 11u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_PARM_BUFFER_LENGTH: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_PARM_MAX_PACKET_LENGTH: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_PROFILE_ALL_ELT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_PROFILE_ALL_ELTS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_PROFILE_DEFAULT_ELT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_PROFILE_MATCH_BY_BOTH: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_PROFILE_MATCH_BY_IF: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_PROFILE_MATCH_BY_MBR: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_PROTSEQ_MAX_REQS_DEFAULT: u32 = 10u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_RPCHTTP_USE_LOAD_BALANCE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_SECURITY_QOS_VERSION: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_SECURITY_QOS_VERSION_1: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_SECURITY_QOS_VERSION_2: i32 = 2i32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_SECURITY_QOS_VERSION_3: i32 = 3i32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_SECURITY_QOS_VERSION_4: i32 = 4i32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_SECURITY_QOS_VERSION_5: i32 = 5i32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_STATS_CALLS_IN: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_STATS_CALLS_OUT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_STATS_PKTS_IN: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_STATS_PKTS_OUT: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_TRY_ENFORCE_MAX_CALLS: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_USE_INTERNET_PORT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_USE_INTRANET_PORT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_VERS_ALL: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_VERS_COMPATIBLE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_VERS_EXACT: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_VERS_MAJOR_ONLY: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_VERS_UPTO: u32 = 5u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_EEINFO_VERSION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_FLAGS_VALID_BIT: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_FW_IF_FLAG_DCOM: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_IF_ALLOW_CALLBACKS_WITH_NO_AUTH: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_IF_ALLOW_LOCAL_ONLY: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_IF_ALLOW_SECURE_ONLY: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_IF_ALLOW_UNKNOWN_AUTHORITY: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_IF_ASYNC_CALLBACK: u32 = 256u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_IF_AUTOLISTEN: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_IF_OLE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_IF_SEC_CACHE_PER_PROC: u32 = 128u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_IF_SEC_NO_CACHE: u32 = 64u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_INTERFACE_HAS_PIPES: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_NCA_FLAGS_BROADCAST: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_NCA_FLAGS_DEFAULT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_NCA_FLAGS_IDEMPOTENT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_NCA_FLAGS_MAYBE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_PROTSEQ_HTTP: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_PROTSEQ_LRPC: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_PROTSEQ_NMP: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_PROTSEQ_TCP: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_PROXY_CONNECTION_TYPE_IN_PROXY: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_PROXY_CONNECTION_TYPE_OUT_PROXY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_P_ADDR_FORMAT_TCP_IPV4: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_P_ADDR_FORMAT_TCP_IPV6: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_QUERY_CALL_LOCAL_ADDRESS: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_QUERY_CLIENT_ID: u32 = 128u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_QUERY_CLIENT_PID: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_QUERY_CLIENT_PRINCIPAL_NAME: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_QUERY_IS_CLIENT_LOCAL: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_QUERY_NO_AUTH_REQUIRED: u32 = 64u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_QUERY_SERVER_PRINCIPAL_NAME: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_SYSTEM_HANDLE_FREE_ALL: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_SYSTEM_HANDLE_FREE_ERROR_ON_CLOSE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_SYSTEM_HANDLE_FREE_RETRIEVED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_SYSTEM_HANDLE_FREE_UNRETRIEVED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_TYPE_DISCONNECT_EVENT_CONTEXT_HANDLE: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_TYPE_STRICT_CONTEXT_HANDLE: u32 = 1073741824u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const TARGET_IS_NT100_OR_LATER: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const TARGET_IS_NT351_OR_WIN95_OR_LATER: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const TARGET_IS_NT40_OR_LATER: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const TARGET_IS_NT50_OR_LATER: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const TARGET_IS_NT51_OR_LATER: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const TARGET_IS_NT60_OR_LATER: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const TARGET_IS_NT61_OR_LATER: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const TARGET_IS_NT62_OR_LATER: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const TARGET_IS_NT63_OR_LATER: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const TRANSPORT_TYPE_CN: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const TRANSPORT_TYPE_DG: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const TRANSPORT_TYPE_LPC: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const TRANSPORT_TYPE_WMSG: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const USER_CALL_IS_ASYNC: u32 = 256u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const USER_CALL_NEW_CORRELATION_DESC: u32 = 512u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const USER_MARSHAL_FC_BYTE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const USER_MARSHAL_FC_CHAR: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const USER_MARSHAL_FC_DOUBLE: u32 = 12u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const USER_MARSHAL_FC_FLOAT: u32 = 10u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const USER_MARSHAL_FC_HYPER: u32 = 11u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const USER_MARSHAL_FC_LONG: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const USER_MARSHAL_FC_SHORT: u32 = 6u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const USER_MARSHAL_FC_SMALL: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const USER_MARSHAL_FC_ULONG: u32 = 9u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const USER_MARSHAL_FC_USHORT: u32 = 7u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const USER_MARSHAL_FC_USMALL: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const USER_MARSHAL_FC_WCHAR: u32 = 5u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const __RPCPROXY_H_VERSION__: u32 = 475u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const cbNDRContext: u32 = 20u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct EXPR_TOKEN(pub i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const FC_EXPR_START: EXPR_TOKEN = EXPR_TOKEN(0i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const FC_EXPR_ILLEGAL: EXPR_TOKEN = EXPR_TOKEN(0i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const FC_EXPR_CONST32: EXPR_TOKEN = EXPR_TOKEN(1i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const FC_EXPR_CONST64: EXPR_TOKEN = EXPR_TOKEN(2i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const FC_EXPR_VAR: EXPR_TOKEN = EXPR_TOKEN(3i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const FC_EXPR_OPER: EXPR_TOKEN = EXPR_TOKEN(4i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const FC_EXPR_NOOP: EXPR_TOKEN = EXPR_TOKEN(5i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const FC_EXPR_END: EXPR_TOKEN = EXPR_TOKEN(6i32);
impl ::core::marker::Copy for EXPR_TOKEN {}
impl ::core::clone::Clone for EXPR_TOKEN {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EXPR_TOKEN {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for EXPR_TOKEN {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for EXPR_TOKEN {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EXPR_TOKEN").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ExtendedErrorParamTypes(pub i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const eeptAnsiString: ExtendedErrorParamTypes = ExtendedErrorParamTypes(1i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const eeptUnicodeString: ExtendedErrorParamTypes = ExtendedErrorParamTypes(2i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const eeptLongVal: ExtendedErrorParamTypes = ExtendedErrorParamTypes(3i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const eeptShortVal: ExtendedErrorParamTypes = ExtendedErrorParamTypes(4i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const eeptPointerVal: ExtendedErrorParamTypes = ExtendedErrorParamTypes(5i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const eeptNone: ExtendedErrorParamTypes = ExtendedErrorParamTypes(6i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const eeptBinary: ExtendedErrorParamTypes = ExtendedErrorParamTypes(7i32);
impl ::core::marker::Copy for ExtendedErrorParamTypes {}
impl ::core::clone::Clone for ExtendedErrorParamTypes {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ExtendedErrorParamTypes {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for ExtendedErrorParamTypes {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for ExtendedErrorParamTypes {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ExtendedErrorParamTypes").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct GROUP_NAME_SYNTAX(pub u32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_NS_SYNTAX_DEFAULT: GROUP_NAME_SYNTAX = GROUP_NAME_SYNTAX(0u32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_NS_SYNTAX_DCE: GROUP_NAME_SYNTAX = GROUP_NAME_SYNTAX(3u32);
impl ::core::marker::Copy for GROUP_NAME_SYNTAX {}
impl ::core::clone::Clone for GROUP_NAME_SYNTAX {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GROUP_NAME_SYNTAX {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for GROUP_NAME_SYNTAX {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for GROUP_NAME_SYNTAX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GROUP_NAME_SYNTAX").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct IDL_CS_CONVERT(pub i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const IDL_CS_NO_CONVERT: IDL_CS_CONVERT = IDL_CS_CONVERT(0i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const IDL_CS_IN_PLACE_CONVERT: IDL_CS_CONVERT = IDL_CS_CONVERT(1i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const IDL_CS_NEW_BUFFER_CONVERT: IDL_CS_CONVERT = IDL_CS_CONVERT(2i32);
impl ::core::marker::Copy for IDL_CS_CONVERT {}
impl ::core::clone::Clone for IDL_CS_CONVERT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for IDL_CS_CONVERT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for IDL_CS_CONVERT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for IDL_CS_CONVERT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDL_CS_CONVERT").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct LRPC_SYSTEM_HANDLE_MARSHAL_DIRECTION(pub i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const MarshalDirectionMarshal: LRPC_SYSTEM_HANDLE_MARSHAL_DIRECTION = LRPC_SYSTEM_HANDLE_MARSHAL_DIRECTION(0i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const MarshalDirectionUnmarshal: LRPC_SYSTEM_HANDLE_MARSHAL_DIRECTION = LRPC_SYSTEM_HANDLE_MARSHAL_DIRECTION(1i32);
impl ::core::marker::Copy for LRPC_SYSTEM_HANDLE_MARSHAL_DIRECTION {}
impl ::core::clone::Clone for LRPC_SYSTEM_HANDLE_MARSHAL_DIRECTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for LRPC_SYSTEM_HANDLE_MARSHAL_DIRECTION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for LRPC_SYSTEM_HANDLE_MARSHAL_DIRECTION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for LRPC_SYSTEM_HANDLE_MARSHAL_DIRECTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LRPC_SYSTEM_HANDLE_MARSHAL_DIRECTION").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MIDL_ES_CODE(pub i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const MES_ENCODE: MIDL_ES_CODE = MIDL_ES_CODE(0i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const MES_DECODE: MIDL_ES_CODE = MIDL_ES_CODE(1i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const MES_ENCODE_NDR64: MIDL_ES_CODE = MIDL_ES_CODE(2i32);
impl ::core::marker::Copy for MIDL_ES_CODE {}
impl ::core::clone::Clone for MIDL_ES_CODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MIDL_ES_CODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for MIDL_ES_CODE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for MIDL_ES_CODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MIDL_ES_CODE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MIDL_ES_HANDLE_STYLE(pub i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const MES_INCREMENTAL_HANDLE: MIDL_ES_HANDLE_STYLE = MIDL_ES_HANDLE_STYLE(0i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const MES_FIXED_BUFFER_HANDLE: MIDL_ES_HANDLE_STYLE = MIDL_ES_HANDLE_STYLE(1i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const MES_DYNAMIC_BUFFER_HANDLE: MIDL_ES_HANDLE_STYLE = MIDL_ES_HANDLE_STYLE(2i32);
impl ::core::marker::Copy for MIDL_ES_HANDLE_STYLE {}
impl ::core::clone::Clone for MIDL_ES_HANDLE_STYLE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MIDL_ES_HANDLE_STYLE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for MIDL_ES_HANDLE_STYLE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for MIDL_ES_HANDLE_STYLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MIDL_ES_HANDLE_STYLE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PROXY_PHASE(pub i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const PROXY_CALCSIZE: PROXY_PHASE = PROXY_PHASE(0i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const PROXY_GETBUFFER: PROXY_PHASE = PROXY_PHASE(1i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const PROXY_MARSHAL: PROXY_PHASE = PROXY_PHASE(2i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const PROXY_SENDRECEIVE: PROXY_PHASE = PROXY_PHASE(3i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const PROXY_UNMARSHAL: PROXY_PHASE = PROXY_PHASE(4i32);
impl ::core::marker::Copy for PROXY_PHASE {}
impl ::core::clone::Clone for PROXY_PHASE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PROXY_PHASE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for PROXY_PHASE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for PROXY_PHASE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PROXY_PHASE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct RPC_ADDRESS_CHANGE_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const PROTOCOL_NOT_LOADED: RPC_ADDRESS_CHANGE_TYPE = RPC_ADDRESS_CHANGE_TYPE(1i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const PROTOCOL_LOADED: RPC_ADDRESS_CHANGE_TYPE = RPC_ADDRESS_CHANGE_TYPE(2i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const PROTOCOL_ADDRESS_CHANGE: RPC_ADDRESS_CHANGE_TYPE = RPC_ADDRESS_CHANGE_TYPE(3i32);
impl ::core::marker::Copy for RPC_ADDRESS_CHANGE_TYPE {}
impl ::core::clone::Clone for RPC_ADDRESS_CHANGE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RPC_ADDRESS_CHANGE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for RPC_ADDRESS_CHANGE_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for RPC_ADDRESS_CHANGE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RPC_ADDRESS_CHANGE_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct RPC_ASYNC_EVENT(pub i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RpcCallComplete: RPC_ASYNC_EVENT = RPC_ASYNC_EVENT(0i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RpcSendComplete: RPC_ASYNC_EVENT = RPC_ASYNC_EVENT(1i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RpcReceiveComplete: RPC_ASYNC_EVENT = RPC_ASYNC_EVENT(2i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RpcClientDisconnect: RPC_ASYNC_EVENT = RPC_ASYNC_EVENT(3i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RpcClientCancel: RPC_ASYNC_EVENT = RPC_ASYNC_EVENT(4i32);
impl ::core::marker::Copy for RPC_ASYNC_EVENT {}
impl ::core::clone::Clone for RPC_ASYNC_EVENT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RPC_ASYNC_EVENT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for RPC_ASYNC_EVENT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for RPC_ASYNC_EVENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RPC_ASYNC_EVENT").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct RPC_BINDING_HANDLE_OPTIONS_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_BHO_NONCAUSAL: RPC_BINDING_HANDLE_OPTIONS_FLAGS = RPC_BINDING_HANDLE_OPTIONS_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_BHO_DONTLINGER: RPC_BINDING_HANDLE_OPTIONS_FLAGS = RPC_BINDING_HANDLE_OPTIONS_FLAGS(2u32);
impl ::core::marker::Copy for RPC_BINDING_HANDLE_OPTIONS_FLAGS {}
impl ::core::clone::Clone for RPC_BINDING_HANDLE_OPTIONS_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RPC_BINDING_HANDLE_OPTIONS_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for RPC_BINDING_HANDLE_OPTIONS_FLAGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for RPC_BINDING_HANDLE_OPTIONS_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RPC_BINDING_HANDLE_OPTIONS_FLAGS").field(&self.0).finish()
    }
}
impl RPC_BINDING_HANDLE_OPTIONS_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for RPC_BINDING_HANDLE_OPTIONS_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for RPC_BINDING_HANDLE_OPTIONS_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for RPC_BINDING_HANDLE_OPTIONS_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for RPC_BINDING_HANDLE_OPTIONS_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for RPC_BINDING_HANDLE_OPTIONS_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct RPC_C_AUTHN_INFO_TYPE(pub u32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_AUTHN_INFO_NONE: RPC_C_AUTHN_INFO_TYPE = RPC_C_AUTHN_INFO_TYPE(0u32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_AUTHN_INFO_TYPE_HTTP: RPC_C_AUTHN_INFO_TYPE = RPC_C_AUTHN_INFO_TYPE(1u32);
impl ::core::marker::Copy for RPC_C_AUTHN_INFO_TYPE {}
impl ::core::clone::Clone for RPC_C_AUTHN_INFO_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RPC_C_AUTHN_INFO_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for RPC_C_AUTHN_INFO_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for RPC_C_AUTHN_INFO_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RPC_C_AUTHN_INFO_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct RPC_C_HTTP_AUTHN_TARGET(pub u32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_HTTP_AUTHN_TARGET_SERVER: RPC_C_HTTP_AUTHN_TARGET = RPC_C_HTTP_AUTHN_TARGET(1u32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_HTTP_AUTHN_TARGET_PROXY: RPC_C_HTTP_AUTHN_TARGET = RPC_C_HTTP_AUTHN_TARGET(2u32);
impl ::core::marker::Copy for RPC_C_HTTP_AUTHN_TARGET {}
impl ::core::clone::Clone for RPC_C_HTTP_AUTHN_TARGET {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RPC_C_HTTP_AUTHN_TARGET {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for RPC_C_HTTP_AUTHN_TARGET {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for RPC_C_HTTP_AUTHN_TARGET {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RPC_C_HTTP_AUTHN_TARGET").field(&self.0).finish()
    }
}
impl RPC_C_HTTP_AUTHN_TARGET {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for RPC_C_HTTP_AUTHN_TARGET {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for RPC_C_HTTP_AUTHN_TARGET {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for RPC_C_HTTP_AUTHN_TARGET {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for RPC_C_HTTP_AUTHN_TARGET {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for RPC_C_HTTP_AUTHN_TARGET {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct RPC_C_HTTP_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_HTTP_FLAG_USE_SSL: RPC_C_HTTP_FLAGS = RPC_C_HTTP_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_HTTP_FLAG_USE_FIRST_AUTH_SCHEME: RPC_C_HTTP_FLAGS = RPC_C_HTTP_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_HTTP_FLAG_IGNORE_CERT_CN_INVALID: RPC_C_HTTP_FLAGS = RPC_C_HTTP_FLAGS(8u32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_HTTP_FLAG_ENABLE_CERT_REVOCATION_CHECK: RPC_C_HTTP_FLAGS = RPC_C_HTTP_FLAGS(16u32);
impl ::core::marker::Copy for RPC_C_HTTP_FLAGS {}
impl ::core::clone::Clone for RPC_C_HTTP_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RPC_C_HTTP_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for RPC_C_HTTP_FLAGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for RPC_C_HTTP_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RPC_C_HTTP_FLAGS").field(&self.0).finish()
    }
}
impl RPC_C_HTTP_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for RPC_C_HTTP_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for RPC_C_HTTP_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for RPC_C_HTTP_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for RPC_C_HTTP_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for RPC_C_HTTP_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct RPC_C_QOS_CAPABILITIES(pub u32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_QOS_CAPABILITIES_DEFAULT: RPC_C_QOS_CAPABILITIES = RPC_C_QOS_CAPABILITIES(0u32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_QOS_CAPABILITIES_MUTUAL_AUTH: RPC_C_QOS_CAPABILITIES = RPC_C_QOS_CAPABILITIES(1u32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_QOS_CAPABILITIES_MAKE_FULLSIC: RPC_C_QOS_CAPABILITIES = RPC_C_QOS_CAPABILITIES(2u32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_QOS_CAPABILITIES_ANY_AUTHORITY: RPC_C_QOS_CAPABILITIES = RPC_C_QOS_CAPABILITIES(4u32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_QOS_CAPABILITIES_IGNORE_DELEGATE_FAILURE: RPC_C_QOS_CAPABILITIES = RPC_C_QOS_CAPABILITIES(8u32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_QOS_CAPABILITIES_LOCAL_MA_HINT: RPC_C_QOS_CAPABILITIES = RPC_C_QOS_CAPABILITIES(16u32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_QOS_CAPABILITIES_SCHANNEL_FULL_AUTH_IDENTITY: RPC_C_QOS_CAPABILITIES = RPC_C_QOS_CAPABILITIES(32u32);
impl ::core::marker::Copy for RPC_C_QOS_CAPABILITIES {}
impl ::core::clone::Clone for RPC_C_QOS_CAPABILITIES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RPC_C_QOS_CAPABILITIES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for RPC_C_QOS_CAPABILITIES {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for RPC_C_QOS_CAPABILITIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RPC_C_QOS_CAPABILITIES").field(&self.0).finish()
    }
}
impl RPC_C_QOS_CAPABILITIES {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for RPC_C_QOS_CAPABILITIES {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for RPC_C_QOS_CAPABILITIES {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for RPC_C_QOS_CAPABILITIES {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for RPC_C_QOS_CAPABILITIES {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for RPC_C_QOS_CAPABILITIES {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct RPC_C_QOS_IDENTITY(pub u32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_QOS_IDENTITY_STATIC: RPC_C_QOS_IDENTITY = RPC_C_QOS_IDENTITY(0u32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_QOS_IDENTITY_DYNAMIC: RPC_C_QOS_IDENTITY = RPC_C_QOS_IDENTITY(1u32);
impl ::core::marker::Copy for RPC_C_QOS_IDENTITY {}
impl ::core::clone::Clone for RPC_C_QOS_IDENTITY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RPC_C_QOS_IDENTITY {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for RPC_C_QOS_IDENTITY {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for RPC_C_QOS_IDENTITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RPC_C_QOS_IDENTITY").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct RPC_HTTP_REDIRECTOR_STAGE(pub i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPCHTTP_RS_REDIRECT: RPC_HTTP_REDIRECTOR_STAGE = RPC_HTTP_REDIRECTOR_STAGE(1i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPCHTTP_RS_ACCESS_1: RPC_HTTP_REDIRECTOR_STAGE = RPC_HTTP_REDIRECTOR_STAGE(2i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPCHTTP_RS_SESSION: RPC_HTTP_REDIRECTOR_STAGE = RPC_HTTP_REDIRECTOR_STAGE(3i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPCHTTP_RS_ACCESS_2: RPC_HTTP_REDIRECTOR_STAGE = RPC_HTTP_REDIRECTOR_STAGE(4i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPCHTTP_RS_INTERFACE: RPC_HTTP_REDIRECTOR_STAGE = RPC_HTTP_REDIRECTOR_STAGE(5i32);
impl ::core::marker::Copy for RPC_HTTP_REDIRECTOR_STAGE {}
impl ::core::clone::Clone for RPC_HTTP_REDIRECTOR_STAGE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RPC_HTTP_REDIRECTOR_STAGE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for RPC_HTTP_REDIRECTOR_STAGE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for RPC_HTTP_REDIRECTOR_STAGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RPC_HTTP_REDIRECTOR_STAGE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct RPC_NOTIFICATIONS(pub i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RpcNotificationCallNone: RPC_NOTIFICATIONS = RPC_NOTIFICATIONS(0i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RpcNotificationClientDisconnect: RPC_NOTIFICATIONS = RPC_NOTIFICATIONS(1i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RpcNotificationCallCancel: RPC_NOTIFICATIONS = RPC_NOTIFICATIONS(2i32);
impl ::core::marker::Copy for RPC_NOTIFICATIONS {}
impl ::core::clone::Clone for RPC_NOTIFICATIONS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RPC_NOTIFICATIONS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for RPC_NOTIFICATIONS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for RPC_NOTIFICATIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RPC_NOTIFICATIONS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct RPC_NOTIFICATION_TYPES(pub i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RpcNotificationTypeNone: RPC_NOTIFICATION_TYPES = RPC_NOTIFICATION_TYPES(0i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RpcNotificationTypeEvent: RPC_NOTIFICATION_TYPES = RPC_NOTIFICATION_TYPES(1i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RpcNotificationTypeApc: RPC_NOTIFICATION_TYPES = RPC_NOTIFICATION_TYPES(2i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RpcNotificationTypeIoc: RPC_NOTIFICATION_TYPES = RPC_NOTIFICATION_TYPES(3i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RpcNotificationTypeHwnd: RPC_NOTIFICATION_TYPES = RPC_NOTIFICATION_TYPES(4i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RpcNotificationTypeCallback: RPC_NOTIFICATION_TYPES = RPC_NOTIFICATION_TYPES(5i32);
impl ::core::marker::Copy for RPC_NOTIFICATION_TYPES {}
impl ::core::clone::Clone for RPC_NOTIFICATION_TYPES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RPC_NOTIFICATION_TYPES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for RPC_NOTIFICATION_TYPES {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for RPC_NOTIFICATION_TYPES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RPC_NOTIFICATION_TYPES").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct RPC_STATUS(pub i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_S_INVALID_STRING_BINDING: RPC_STATUS = RPC_STATUS(1700i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_S_WRONG_KIND_OF_BINDING: RPC_STATUS = RPC_STATUS(1701i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_S_INVALID_BINDING: RPC_STATUS = RPC_STATUS(1702i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_S_PROTSEQ_NOT_SUPPORTED: RPC_STATUS = RPC_STATUS(1703i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_S_INVALID_RPC_PROTSEQ: RPC_STATUS = RPC_STATUS(1704i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_S_INVALID_STRING_UUID: RPC_STATUS = RPC_STATUS(1705i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_S_INVALID_ENDPOINT_FORMAT: RPC_STATUS = RPC_STATUS(1706i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_S_INVALID_NET_ADDR: RPC_STATUS = RPC_STATUS(1707i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_S_NO_ENDPOINT_FOUND: RPC_STATUS = RPC_STATUS(1708i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_S_INVALID_TIMEOUT: RPC_STATUS = RPC_STATUS(1709i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_S_OBJECT_NOT_FOUND: RPC_STATUS = RPC_STATUS(1710i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_S_ALREADY_REGISTERED: RPC_STATUS = RPC_STATUS(1711i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_S_TYPE_ALREADY_REGISTERED: RPC_STATUS = RPC_STATUS(1712i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_S_ALREADY_LISTENING: RPC_STATUS = RPC_STATUS(1713i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_S_NO_PROTSEQS_REGISTERED: RPC_STATUS = RPC_STATUS(1714i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_S_NOT_LISTENING: RPC_STATUS = RPC_STATUS(1715i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_S_UNKNOWN_MGR_TYPE: RPC_STATUS = RPC_STATUS(1716i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_S_UNKNOWN_IF: RPC_STATUS = RPC_STATUS(1717i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_S_NO_BINDINGS: RPC_STATUS = RPC_STATUS(1718i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_S_NO_PROTSEQS: RPC_STATUS = RPC_STATUS(1719i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_S_CANT_CREATE_ENDPOINT: RPC_STATUS = RPC_STATUS(1720i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_S_OUT_OF_RESOURCES: RPC_STATUS = RPC_STATUS(1721i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_S_SERVER_UNAVAILABLE: RPC_STATUS = RPC_STATUS(1722i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_S_SERVER_TOO_BUSY: RPC_STATUS = RPC_STATUS(1723i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_S_INVALID_NETWORK_OPTIONS: RPC_STATUS = RPC_STATUS(1724i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_S_NO_CALL_ACTIVE: RPC_STATUS = RPC_STATUS(1725i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_S_CALL_FAILED: RPC_STATUS = RPC_STATUS(1726i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_S_CALL_FAILED_DNE: RPC_STATUS = RPC_STATUS(1727i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_S_PROTOCOL_ERROR: RPC_STATUS = RPC_STATUS(1728i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_S_PROXY_ACCESS_DENIED: RPC_STATUS = RPC_STATUS(1729i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_S_UNSUPPORTED_TRANS_SYN: RPC_STATUS = RPC_STATUS(1730i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_S_UNSUPPORTED_TYPE: RPC_STATUS = RPC_STATUS(1732i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_S_INVALID_TAG: RPC_STATUS = RPC_STATUS(1733i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_S_INVALID_BOUND: RPC_STATUS = RPC_STATUS(1734i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_S_NO_ENTRY_NAME: RPC_STATUS = RPC_STATUS(1735i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_S_INVALID_NAME_SYNTAX: RPC_STATUS = RPC_STATUS(1736i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_S_UNSUPPORTED_NAME_SYNTAX: RPC_STATUS = RPC_STATUS(1737i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_S_UUID_NO_ADDRESS: RPC_STATUS = RPC_STATUS(1739i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_S_DUPLICATE_ENDPOINT: RPC_STATUS = RPC_STATUS(1740i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_S_UNKNOWN_AUTHN_TYPE: RPC_STATUS = RPC_STATUS(1741i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_S_MAX_CALLS_TOO_SMALL: RPC_STATUS = RPC_STATUS(1742i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_S_STRING_TOO_LONG: RPC_STATUS = RPC_STATUS(1743i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_S_PROTSEQ_NOT_FOUND: RPC_STATUS = RPC_STATUS(1744i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_S_PROCNUM_OUT_OF_RANGE: RPC_STATUS = RPC_STATUS(1745i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_S_BINDING_HAS_NO_AUTH: RPC_STATUS = RPC_STATUS(1746i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_S_UNKNOWN_AUTHN_SERVICE: RPC_STATUS = RPC_STATUS(1747i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_S_UNKNOWN_AUTHN_LEVEL: RPC_STATUS = RPC_STATUS(1748i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_S_INVALID_AUTH_IDENTITY: RPC_STATUS = RPC_STATUS(1749i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_S_UNKNOWN_AUTHZ_SERVICE: RPC_STATUS = RPC_STATUS(1750i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const EPT_S_INVALID_ENTRY: RPC_STATUS = RPC_STATUS(1751i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const EPT_S_CANT_PERFORM_OP: RPC_STATUS = RPC_STATUS(1752i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const EPT_S_NOT_REGISTERED: RPC_STATUS = RPC_STATUS(1753i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_S_NOTHING_TO_EXPORT: RPC_STATUS = RPC_STATUS(1754i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_S_INCOMPLETE_NAME: RPC_STATUS = RPC_STATUS(1755i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_S_INVALID_VERS_OPTION: RPC_STATUS = RPC_STATUS(1756i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_S_NO_MORE_MEMBERS: RPC_STATUS = RPC_STATUS(1757i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_S_NOT_ALL_OBJS_UNEXPORTED: RPC_STATUS = RPC_STATUS(1758i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_S_INTERFACE_NOT_FOUND: RPC_STATUS = RPC_STATUS(1759i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_S_ENTRY_ALREADY_EXISTS: RPC_STATUS = RPC_STATUS(1760i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_S_ENTRY_NOT_FOUND: RPC_STATUS = RPC_STATUS(1761i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_S_NAME_SERVICE_UNAVAILABLE: RPC_STATUS = RPC_STATUS(1762i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_S_INVALID_NAF_ID: RPC_STATUS = RPC_STATUS(1763i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_S_CANNOT_SUPPORT: RPC_STATUS = RPC_STATUS(1764i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_S_NO_CONTEXT_AVAILABLE: RPC_STATUS = RPC_STATUS(1765i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_S_INTERNAL_ERROR: RPC_STATUS = RPC_STATUS(1766i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_S_ZERO_DIVIDE: RPC_STATUS = RPC_STATUS(1767i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_S_ADDRESS_ERROR: RPC_STATUS = RPC_STATUS(1768i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_S_FP_DIV_ZERO: RPC_STATUS = RPC_STATUS(1769i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_S_FP_UNDERFLOW: RPC_STATUS = RPC_STATUS(1770i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_S_FP_OVERFLOW: RPC_STATUS = RPC_STATUS(1771i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_S_CALL_IN_PROGRESS: RPC_STATUS = RPC_STATUS(1791i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_S_NO_MORE_BINDINGS: RPC_STATUS = RPC_STATUS(1806i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_S_NO_INTERFACES: RPC_STATUS = RPC_STATUS(1817i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_S_CALL_CANCELLED: RPC_STATUS = RPC_STATUS(1818i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_S_BINDING_INCOMPLETE: RPC_STATUS = RPC_STATUS(1819i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_S_COMM_FAILURE: RPC_STATUS = RPC_STATUS(1820i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_S_UNSUPPORTED_AUTHN_LEVEL: RPC_STATUS = RPC_STATUS(1821i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_S_NO_PRINC_NAME: RPC_STATUS = RPC_STATUS(1822i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_S_NOT_RPC_ERROR: RPC_STATUS = RPC_STATUS(1823i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_S_UUID_LOCAL_ONLY: RPC_STATUS = RPC_STATUS(1824i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_S_SEC_PKG_ERROR: RPC_STATUS = RPC_STATUS(1825i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_S_NOT_CANCELLED: RPC_STATUS = RPC_STATUS(1826i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_S_COOKIE_AUTH_FAILED: RPC_STATUS = RPC_STATUS(1833i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_S_DO_NOT_DISTURB: RPC_STATUS = RPC_STATUS(1834i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_S_SYSTEM_HANDLE_COUNT_EXCEEDED: RPC_STATUS = RPC_STATUS(1835i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_S_SYSTEM_HANDLE_TYPE_MISMATCH: RPC_STATUS = RPC_STATUS(1836i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_S_GROUP_MEMBER_NOT_FOUND: RPC_STATUS = RPC_STATUS(1898i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const EPT_S_CANT_CREATE: RPC_STATUS = RPC_STATUS(1899i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_S_INVALID_OBJECT: RPC_STATUS = RPC_STATUS(1900i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_S_SEND_INCOMPLETE: RPC_STATUS = RPC_STATUS(1913i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_S_INVALID_ASYNC_HANDLE: RPC_STATUS = RPC_STATUS(1914i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_S_INVALID_ASYNC_CALL: RPC_STATUS = RPC_STATUS(1915i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_S_ENTRY_TYPE_MISMATCH: RPC_STATUS = RPC_STATUS(1922i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_S_NOT_ALL_OBJS_EXPORTED: RPC_STATUS = RPC_STATUS(1923i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_S_INTERFACE_NOT_EXPORTED: RPC_STATUS = RPC_STATUS(1924i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_S_PROFILE_NOT_ADDED: RPC_STATUS = RPC_STATUS(1925i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_S_PRF_ELT_NOT_ADDED: RPC_STATUS = RPC_STATUS(1926i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_S_PRF_ELT_NOT_REMOVED: RPC_STATUS = RPC_STATUS(1927i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_S_GRP_ELT_NOT_ADDED: RPC_STATUS = RPC_STATUS(1928i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_S_GRP_ELT_NOT_REMOVED: RPC_STATUS = RPC_STATUS(1929i32);
impl ::core::marker::Copy for RPC_STATUS {}
impl ::core::clone::Clone for RPC_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RPC_STATUS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for RPC_STATUS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for RPC_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RPC_STATUS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct RpcCallClientLocality(pub i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const rcclInvalid: RpcCallClientLocality = RpcCallClientLocality(0i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const rcclLocal: RpcCallClientLocality = RpcCallClientLocality(1i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const rcclRemote: RpcCallClientLocality = RpcCallClientLocality(2i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const rcclClientUnknownLocality: RpcCallClientLocality = RpcCallClientLocality(3i32);
impl ::core::marker::Copy for RpcCallClientLocality {}
impl ::core::clone::Clone for RpcCallClientLocality {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RpcCallClientLocality {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for RpcCallClientLocality {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for RpcCallClientLocality {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RpcCallClientLocality").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct RpcCallType(pub i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const rctInvalid: RpcCallType = RpcCallType(0i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const rctNormal: RpcCallType = RpcCallType(1i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const rctTraining: RpcCallType = RpcCallType(2i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const rctGuaranteed: RpcCallType = RpcCallType(3i32);
impl ::core::marker::Copy for RpcCallType {}
impl ::core::clone::Clone for RpcCallType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RpcCallType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for RpcCallType {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for RpcCallType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RpcCallType").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct RpcLocalAddressFormat(pub i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const rlafInvalid: RpcLocalAddressFormat = RpcLocalAddressFormat(0i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const rlafIPv4: RpcLocalAddressFormat = RpcLocalAddressFormat(1i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const rlafIPv6: RpcLocalAddressFormat = RpcLocalAddressFormat(2i32);
impl ::core::marker::Copy for RpcLocalAddressFormat {}
impl ::core::clone::Clone for RpcLocalAddressFormat {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RpcLocalAddressFormat {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for RpcLocalAddressFormat {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for RpcLocalAddressFormat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RpcLocalAddressFormat").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct RpcPerfCounters(pub i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RpcCurrentUniqueUser: RpcPerfCounters = RpcPerfCounters(1i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RpcBackEndConnectionAttempts: RpcPerfCounters = RpcPerfCounters(2i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RpcBackEndConnectionFailed: RpcPerfCounters = RpcPerfCounters(3i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RpcRequestsPerSecond: RpcPerfCounters = RpcPerfCounters(4i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RpcIncomingConnections: RpcPerfCounters = RpcPerfCounters(5i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RpcIncomingBandwidth: RpcPerfCounters = RpcPerfCounters(6i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RpcOutgoingBandwidth: RpcPerfCounters = RpcPerfCounters(7i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RpcAttemptedLbsDecisions: RpcPerfCounters = RpcPerfCounters(8i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RpcFailedLbsDecisions: RpcPerfCounters = RpcPerfCounters(9i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RpcAttemptedLbsMessages: RpcPerfCounters = RpcPerfCounters(10i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RpcFailedLbsMessages: RpcPerfCounters = RpcPerfCounters(11i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RpcLastCounter: RpcPerfCounters = RpcPerfCounters(12i32);
impl ::core::marker::Copy for RpcPerfCounters {}
impl ::core::clone::Clone for RpcPerfCounters {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RpcPerfCounters {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for RpcPerfCounters {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for RpcPerfCounters {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RpcPerfCounters").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SEC_WINNT_AUTH_IDENTITY(pub u32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const SEC_WINNT_AUTH_IDENTITY_ANSI: SEC_WINNT_AUTH_IDENTITY = SEC_WINNT_AUTH_IDENTITY(1u32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const SEC_WINNT_AUTH_IDENTITY_UNICODE: SEC_WINNT_AUTH_IDENTITY = SEC_WINNT_AUTH_IDENTITY(2u32);
impl ::core::marker::Copy for SEC_WINNT_AUTH_IDENTITY {}
impl ::core::clone::Clone for SEC_WINNT_AUTH_IDENTITY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SEC_WINNT_AUTH_IDENTITY {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for SEC_WINNT_AUTH_IDENTITY {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for SEC_WINNT_AUTH_IDENTITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SEC_WINNT_AUTH_IDENTITY").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct STUB_PHASE(pub i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const STUB_UNMARSHAL: STUB_PHASE = STUB_PHASE(0i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const STUB_CALL_SERVER: STUB_PHASE = STUB_PHASE(1i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const STUB_MARSHAL: STUB_PHASE = STUB_PHASE(2i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const STUB_CALL_SERVER_NO_HRESULT: STUB_PHASE = STUB_PHASE(3i32);
impl ::core::marker::Copy for STUB_PHASE {}
impl ::core::clone::Clone for STUB_PHASE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for STUB_PHASE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for STUB_PHASE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for STUB_PHASE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("STUB_PHASE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct USER_MARSHAL_CB_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const USER_MARSHAL_CB_BUFFER_SIZE: USER_MARSHAL_CB_TYPE = USER_MARSHAL_CB_TYPE(0i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const USER_MARSHAL_CB_MARSHALL: USER_MARSHAL_CB_TYPE = USER_MARSHAL_CB_TYPE(1i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const USER_MARSHAL_CB_UNMARSHALL: USER_MARSHAL_CB_TYPE = USER_MARSHAL_CB_TYPE(2i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const USER_MARSHAL_CB_FREE: USER_MARSHAL_CB_TYPE = USER_MARSHAL_CB_TYPE(3i32);
impl ::core::marker::Copy for USER_MARSHAL_CB_TYPE {}
impl ::core::clone::Clone for USER_MARSHAL_CB_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for USER_MARSHAL_CB_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for USER_MARSHAL_CB_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for USER_MARSHAL_CB_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("USER_MARSHAL_CB_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct XLAT_SIDE(pub i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const XLAT_SERVER: XLAT_SIDE = XLAT_SIDE(1i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const XLAT_CLIENT: XLAT_SIDE = XLAT_SIDE(2i32);
impl ::core::marker::Copy for XLAT_SIDE {}
impl ::core::clone::Clone for XLAT_SIDE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for XLAT_SIDE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for XLAT_SIDE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for XLAT_SIDE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XLAT_SIDE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct system_handle_t(pub i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const SYSTEM_HANDLE_FILE: system_handle_t = system_handle_t(0i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const SYSTEM_HANDLE_SEMAPHORE: system_handle_t = system_handle_t(1i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const SYSTEM_HANDLE_EVENT: system_handle_t = system_handle_t(2i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const SYSTEM_HANDLE_MUTEX: system_handle_t = system_handle_t(3i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const SYSTEM_HANDLE_PROCESS: system_handle_t = system_handle_t(4i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const SYSTEM_HANDLE_TOKEN: system_handle_t = system_handle_t(5i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const SYSTEM_HANDLE_SECTION: system_handle_t = system_handle_t(6i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const SYSTEM_HANDLE_REG_KEY: system_handle_t = system_handle_t(7i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const SYSTEM_HANDLE_THREAD: system_handle_t = system_handle_t(8i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const SYSTEM_HANDLE_COMPOSITION_OBJECT: system_handle_t = system_handle_t(9i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const SYSTEM_HANDLE_SOCKET: system_handle_t = system_handle_t(10i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const SYSTEM_HANDLE_JOB: system_handle_t = system_handle_t(11i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const SYSTEM_HANDLE_PIPE: system_handle_t = system_handle_t(12i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const SYSTEM_HANDLE_MAX: system_handle_t = system_handle_t(12i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const SYSTEM_HANDLE_INVALID: system_handle_t = system_handle_t(255i32);
impl ::core::marker::Copy for system_handle_t {}
impl ::core::clone::Clone for system_handle_t {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for system_handle_t {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for system_handle_t {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for system_handle_t {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("system_handle_t").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct ARRAY_INFO {
    pub Dimension: i32,
    pub BufferConformanceMark: *mut u32,
    pub BufferVarianceMark: *mut u32,
    pub MaxCountArray: *mut u32,
    pub OffsetArray: *mut u32,
    pub ActualCountArray: *mut u32,
}
impl ::core::marker::Copy for ARRAY_INFO {}
impl ::core::clone::Clone for ARRAY_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ARRAY_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ARRAY_INFO").field("Dimension", &self.Dimension).field("BufferConformanceMark", &self.BufferConformanceMark).field("BufferVarianceMark", &self.BufferVarianceMark).field("MaxCountArray", &self.MaxCountArray).field("OffsetArray", &self.OffsetArray).field("ActualCountArray", &self.ActualCountArray).finish()
    }
}
impl ::windows::core::TypeKind for ARRAY_INFO {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for ARRAY_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.Dimension == other.Dimension && self.BufferConformanceMark == other.BufferConformanceMark && self.BufferVarianceMark == other.BufferVarianceMark && self.MaxCountArray == other.MaxCountArray && self.OffsetArray == other.OffsetArray && self.ActualCountArray == other.ActualCountArray
    }
}
impl ::core::cmp::Eq for ARRAY_INFO {}
impl ::core::default::Default for ARRAY_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct BinaryParam {
    pub Buffer: *mut ::core::ffi::c_void,
    pub Size: i16,
}
impl ::core::marker::Copy for BinaryParam {}
impl ::core::clone::Clone for BinaryParam {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for BinaryParam {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("BinaryParam").field("Buffer", &self.Buffer).field("Size", &self.Size).finish()
    }
}
impl ::windows::core::TypeKind for BinaryParam {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for BinaryParam {
    fn eq(&self, other: &Self) -> bool {
        self.Buffer == other.Buffer && self.Size == other.Size
    }
}
impl ::core::cmp::Eq for BinaryParam {}
impl ::core::default::Default for BinaryParam {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub union CLIENT_CALL_RETURN {
    pub Pointer: *mut ::core::ffi::c_void,
    pub Simple: isize,
}
impl ::core::marker::Copy for CLIENT_CALL_RETURN {}
impl ::core::clone::Clone for CLIENT_CALL_RETURN {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for CLIENT_CALL_RETURN {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for CLIENT_CALL_RETURN {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct COMM_FAULT_OFFSETS {
    pub CommOffset: i16,
    pub FaultOffset: i16,
}
impl ::core::marker::Copy for COMM_FAULT_OFFSETS {}
impl ::core::clone::Clone for COMM_FAULT_OFFSETS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for COMM_FAULT_OFFSETS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("COMM_FAULT_OFFSETS").field("CommOffset", &self.CommOffset).field("FaultOffset", &self.FaultOffset).finish()
    }
}
impl ::windows::core::TypeKind for COMM_FAULT_OFFSETS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for COMM_FAULT_OFFSETS {
    fn eq(&self, other: &Self) -> bool {
        self.CommOffset == other.CommOffset && self.FaultOffset == other.FaultOffset
    }
}
impl ::core::cmp::Eq for COMM_FAULT_OFFSETS {}
impl ::core::default::Default for COMM_FAULT_OFFSETS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct FULL_PTR_XLAT_TABLES {
    pub RefIdToPointer: *mut ::core::ffi::c_void,
    pub PointerToRefId: *mut ::core::ffi::c_void,
    pub NextRefId: u32,
    pub XlatSide: XLAT_SIDE,
}
impl ::core::marker::Copy for FULL_PTR_XLAT_TABLES {}
impl ::core::clone::Clone for FULL_PTR_XLAT_TABLES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for FULL_PTR_XLAT_TABLES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FULL_PTR_XLAT_TABLES").field("RefIdToPointer", &self.RefIdToPointer).field("PointerToRefId", &self.PointerToRefId).field("NextRefId", &self.NextRefId).field("XlatSide", &self.XlatSide).finish()
    }
}
impl ::windows::core::TypeKind for FULL_PTR_XLAT_TABLES {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for FULL_PTR_XLAT_TABLES {
    fn eq(&self, other: &Self) -> bool {
        self.RefIdToPointer == other.RefIdToPointer && self.PointerToRefId == other.PointerToRefId && self.NextRefId == other.NextRefId && self.XlatSide == other.XlatSide
    }
}
impl ::core::cmp::Eq for FULL_PTR_XLAT_TABLES {}
impl ::core::default::Default for FULL_PTR_XLAT_TABLES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct GENERIC_BINDING_INFO {
    pub pObj: *mut ::core::ffi::c_void,
    pub Size: u32,
    pub pfnBind: GENERIC_BINDING_ROUTINE,
    pub pfnUnbind: GENERIC_UNBIND_ROUTINE,
}
impl ::core::marker::Copy for GENERIC_BINDING_INFO {}
impl ::core::clone::Clone for GENERIC_BINDING_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for GENERIC_BINDING_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GENERIC_BINDING_INFO").field("pObj", &self.pObj).field("Size", &self.Size).finish()
    }
}
impl ::windows::core::TypeKind for GENERIC_BINDING_INFO {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for GENERIC_BINDING_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct GENERIC_BINDING_ROUTINE_PAIR {
    pub pfnBind: GENERIC_BINDING_ROUTINE,
    pub pfnUnbind: GENERIC_UNBIND_ROUTINE,
}
impl ::core::marker::Copy for GENERIC_BINDING_ROUTINE_PAIR {}
impl ::core::clone::Clone for GENERIC_BINDING_ROUTINE_PAIR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for GENERIC_BINDING_ROUTINE_PAIR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GENERIC_BINDING_ROUTINE_PAIR").finish()
    }
}
impl ::windows::core::TypeKind for GENERIC_BINDING_ROUTINE_PAIR {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for GENERIC_BINDING_ROUTINE_PAIR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
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
impl ::core::marker::Copy for I_RpcProxyCallbackInterface {}
impl ::core::clone::Clone for I_RpcProxyCallbackInterface {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for I_RpcProxyCallbackInterface {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("I_RpcProxyCallbackInterface").finish()
    }
}
impl ::windows::core::TypeKind for I_RpcProxyCallbackInterface {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for I_RpcProxyCallbackInterface {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct MALLOC_FREE_STRUCT {
    pub pfnAllocate: isize,
    pub pfnFree: isize,
}
impl ::core::marker::Copy for MALLOC_FREE_STRUCT {}
impl ::core::clone::Clone for MALLOC_FREE_STRUCT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MALLOC_FREE_STRUCT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MALLOC_FREE_STRUCT").field("pfnAllocate", &self.pfnAllocate).field("pfnFree", &self.pfnFree).finish()
    }
}
impl ::windows::core::TypeKind for MALLOC_FREE_STRUCT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for MALLOC_FREE_STRUCT {
    fn eq(&self, other: &Self) -> bool {
        self.pfnAllocate == other.pfnAllocate && self.pfnFree == other.pfnFree
    }
}
impl ::core::cmp::Eq for MALLOC_FREE_STRUCT {}
impl ::core::default::Default for MALLOC_FREE_STRUCT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct MIDL_FORMAT_STRING {
    pub Pad: i16,
    pub Format: [u8; 1],
}
impl ::core::marker::Copy for MIDL_FORMAT_STRING {}
impl ::core::clone::Clone for MIDL_FORMAT_STRING {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MIDL_FORMAT_STRING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIDL_FORMAT_STRING").field("Pad", &self.Pad).field("Format", &self.Format).finish()
    }
}
impl ::windows::core::TypeKind for MIDL_FORMAT_STRING {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for MIDL_FORMAT_STRING {
    fn eq(&self, other: &Self) -> bool {
        self.Pad == other.Pad && self.Format == other.Format
    }
}
impl ::core::cmp::Eq for MIDL_FORMAT_STRING {}
impl ::core::default::Default for MIDL_FORMAT_STRING {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct MIDL_INTERCEPTION_INFO {
    pub Version: u32,
    pub ProcString: *mut u8,
    pub ProcFormatOffsetTable: *const u16,
    pub ProcCount: u32,
    pub TypeString: *mut u8,
}
impl ::core::marker::Copy for MIDL_INTERCEPTION_INFO {}
impl ::core::clone::Clone for MIDL_INTERCEPTION_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MIDL_INTERCEPTION_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIDL_INTERCEPTION_INFO").field("Version", &self.Version).field("ProcString", &self.ProcString).field("ProcFormatOffsetTable", &self.ProcFormatOffsetTable).field("ProcCount", &self.ProcCount).field("TypeString", &self.TypeString).finish()
    }
}
impl ::windows::core::TypeKind for MIDL_INTERCEPTION_INFO {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for MIDL_INTERCEPTION_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.ProcString == other.ProcString && self.ProcFormatOffsetTable == other.ProcFormatOffsetTable && self.ProcCount == other.ProcCount && self.TypeString == other.TypeString
    }
}
impl ::core::cmp::Eq for MIDL_INTERCEPTION_INFO {}
impl ::core::default::Default for MIDL_INTERCEPTION_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct MIDL_INTERFACE_METHOD_PROPERTIES {
    pub MethodCount: u16,
    pub MethodProperties: *const *const MIDL_METHOD_PROPERTY_MAP,
}
impl ::core::marker::Copy for MIDL_INTERFACE_METHOD_PROPERTIES {}
impl ::core::clone::Clone for MIDL_INTERFACE_METHOD_PROPERTIES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MIDL_INTERFACE_METHOD_PROPERTIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIDL_INTERFACE_METHOD_PROPERTIES").field("MethodCount", &self.MethodCount).field("MethodProperties", &self.MethodProperties).finish()
    }
}
impl ::windows::core::TypeKind for MIDL_INTERFACE_METHOD_PROPERTIES {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for MIDL_INTERFACE_METHOD_PROPERTIES {
    fn eq(&self, other: &Self) -> bool {
        self.MethodCount == other.MethodCount && self.MethodProperties == other.MethodProperties
    }
}
impl ::core::cmp::Eq for MIDL_INTERFACE_METHOD_PROPERTIES {}
impl ::core::default::Default for MIDL_INTERFACE_METHOD_PROPERTIES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct MIDL_METHOD_PROPERTY {
    pub Id: u32,
    pub Value: usize,
}
impl ::core::marker::Copy for MIDL_METHOD_PROPERTY {}
impl ::core::clone::Clone for MIDL_METHOD_PROPERTY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MIDL_METHOD_PROPERTY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIDL_METHOD_PROPERTY").field("Id", &self.Id).field("Value", &self.Value).finish()
    }
}
impl ::windows::core::TypeKind for MIDL_METHOD_PROPERTY {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for MIDL_METHOD_PROPERTY {
    fn eq(&self, other: &Self) -> bool {
        self.Id == other.Id && self.Value == other.Value
    }
}
impl ::core::cmp::Eq for MIDL_METHOD_PROPERTY {}
impl ::core::default::Default for MIDL_METHOD_PROPERTY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct MIDL_METHOD_PROPERTY_MAP {
    pub Count: u32,
    pub Properties: *const MIDL_METHOD_PROPERTY,
}
impl ::core::marker::Copy for MIDL_METHOD_PROPERTY_MAP {}
impl ::core::clone::Clone for MIDL_METHOD_PROPERTY_MAP {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MIDL_METHOD_PROPERTY_MAP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIDL_METHOD_PROPERTY_MAP").field("Count", &self.Count).field("Properties", &self.Properties).finish()
    }
}
impl ::windows::core::TypeKind for MIDL_METHOD_PROPERTY_MAP {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for MIDL_METHOD_PROPERTY_MAP {
    fn eq(&self, other: &Self) -> bool {
        self.Count == other.Count && self.Properties == other.Properties
    }
}
impl ::core::cmp::Eq for MIDL_METHOD_PROPERTY_MAP {}
impl ::core::default::Default for MIDL_METHOD_PROPERTY_MAP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub struct MIDL_SERVER_INFO {
    pub pStubDesc: *mut MIDL_STUB_DESC,
    pub DispatchTable: *const SERVER_ROUTINE,
    pub ProcString: *mut u8,
    pub FmtStringOffset: *const u16,
    pub ThunkTable: *const STUB_THUNK,
    pub pTransferSyntax: *mut RPC_SYNTAX_IDENTIFIER,
    pub nCount: usize,
    pub pSyntaxInfo: *mut MIDL_SYNTAX_INFO,
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::marker::Copy for MIDL_SERVER_INFO {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for MIDL_SERVER_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for MIDL_SERVER_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIDL_SERVER_INFO").field("pStubDesc", &self.pStubDesc).field("DispatchTable", &self.DispatchTable).field("ProcString", &self.ProcString).field("FmtStringOffset", &self.FmtStringOffset).field("ThunkTable", &self.ThunkTable).field("pTransferSyntax", &self.pTransferSyntax).field("nCount", &self.nCount).field("pSyntaxInfo", &self.pSyntaxInfo).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::TypeKind for MIDL_SERVER_INFO {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for MIDL_SERVER_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.pStubDesc == other.pStubDesc && self.DispatchTable == other.DispatchTable && self.ProcString == other.ProcString && self.FmtStringOffset == other.FmtStringOffset && self.ThunkTable == other.ThunkTable && self.pTransferSyntax == other.pTransferSyntax && self.nCount == other.nCount && self.pSyntaxInfo == other.pSyntaxInfo
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for MIDL_SERVER_INFO {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::default::Default for MIDL_SERVER_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub struct MIDL_STUBLESS_PROXY_INFO {
    pub pStubDesc: *mut MIDL_STUB_DESC,
    pub ProcFormatString: *mut u8,
    pub FormatStringOffset: *const u16,
    pub pTransferSyntax: *mut RPC_SYNTAX_IDENTIFIER,
    pub nCount: usize,
    pub pSyntaxInfo: *mut MIDL_SYNTAX_INFO,
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::marker::Copy for MIDL_STUBLESS_PROXY_INFO {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for MIDL_STUBLESS_PROXY_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for MIDL_STUBLESS_PROXY_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIDL_STUBLESS_PROXY_INFO").field("pStubDesc", &self.pStubDesc).field("ProcFormatString", &self.ProcFormatString).field("FormatStringOffset", &self.FormatStringOffset).field("pTransferSyntax", &self.pTransferSyntax).field("nCount", &self.nCount).field("pSyntaxInfo", &self.pSyntaxInfo).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::TypeKind for MIDL_STUBLESS_PROXY_INFO {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for MIDL_STUBLESS_PROXY_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.pStubDesc == other.pStubDesc && self.ProcFormatString == other.ProcFormatString && self.FormatStringOffset == other.FormatStringOffset && self.pTransferSyntax == other.pTransferSyntax && self.nCount == other.nCount && self.pSyntaxInfo == other.pSyntaxInfo
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for MIDL_STUBLESS_PROXY_INFO {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::default::Default for MIDL_STUBLESS_PROXY_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub struct MIDL_STUB_DESC {
    pub RpcInterfaceInformation: *mut ::core::ffi::c_void,
    pub pfnAllocate: isize,
    pub pfnFree: isize,
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
    pub ProxyServerInfo: *mut ::core::ffi::c_void,
    pub pExprInfo: *const NDR_EXPR_DESC,
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::marker::Copy for MIDL_STUB_DESC {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for MIDL_STUB_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::TypeKind for MIDL_STUB_DESC {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::default::Default for MIDL_STUB_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub union MIDL_STUB_DESC_0 {
    pub pAutoHandle: *mut *mut ::core::ffi::c_void,
    pub pPrimitiveHandle: *mut *mut ::core::ffi::c_void,
    pub pGenericBindingInfo: *mut GENERIC_BINDING_INFO,
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::marker::Copy for MIDL_STUB_DESC_0 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for MIDL_STUB_DESC_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::TypeKind for MIDL_STUB_DESC_0 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::default::Default for MIDL_STUB_DESC_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub struct MIDL_STUB_MESSAGE {
    pub RpcMsg: *mut RPC_MESSAGE,
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
    pub pfnAllocate: isize,
    pub pfnFree: isize,
    pub StackTop: *mut u8,
    pub pPresentedType: *mut u8,
    pub pTransmitType: *mut u8,
    pub SavedHandle: *mut ::core::ffi::c_void,
    pub StubDesc: *const MIDL_STUB_DESC,
    pub FullPtrXlatTables: *mut FULL_PTR_XLAT_TABLES,
    pub FullPtrRefId: u32,
    pub PointerLength: u32,
    pub _bitfield: i32,
    pub dwDestContext: u32,
    pub pvDestContext: *mut ::core::ffi::c_void,
    pub SavedContextHandles: *mut *mut NDR_SCONTEXT,
    pub ParamNumber: i32,
    pub pRpcChannelBuffer: ::std::mem::ManuallyDrop<::core::option::Option<super::Com::IRpcChannelBuffer>>,
    pub pArrayInfo: *mut ARRAY_INFO,
    pub SizePtrCountArray: *mut u32,
    pub SizePtrOffsetArray: *mut u32,
    pub SizePtrLengthArray: *mut u32,
    pub pArgQueue: *mut ::core::ffi::c_void,
    pub dwStubPhase: u32,
    pub LowStackMark: *mut ::core::ffi::c_void,
    pub pAsyncMsg: *mut _NDR_ASYNC_MESSAGE,
    pub pCorrInfo: *mut _NDR_CORRELATION_INFO,
    pub pCorrMemory: *mut u8,
    pub pMemoryList: *mut ::core::ffi::c_void,
    pub pCSInfo: isize,
    pub ConformanceMark: *mut u8,
    pub VarianceMark: *mut u8,
    pub Unused: isize,
    pub pContext: *mut _NDR_PROC_CONTEXT,
    pub ContextHandleHash: *mut ::core::ffi::c_void,
    pub pUserMarshalList: *mut ::core::ffi::c_void,
    pub Reserved51_3: isize,
    pub Reserved51_4: isize,
    pub Reserved51_5: isize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for MIDL_STUB_MESSAGE {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for MIDL_STUB_MESSAGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIDL_STUB_MESSAGE")
            .field("RpcMsg", &self.RpcMsg)
            .field("Buffer", &self.Buffer)
            .field("BufferStart", &self.BufferStart)
            .field("BufferEnd", &self.BufferEnd)
            .field("BufferMark", &self.BufferMark)
            .field("BufferLength", &self.BufferLength)
            .field("MemorySize", &self.MemorySize)
            .field("Memory", &self.Memory)
            .field("IsClient", &self.IsClient)
            .field("Pad", &self.Pad)
            .field("uFlags2", &self.uFlags2)
            .field("ReuseBuffer", &self.ReuseBuffer)
            .field("pAllocAllNodesContext", &self.pAllocAllNodesContext)
            .field("pPointerQueueState", &self.pPointerQueueState)
            .field("IgnoreEmbeddedPointers", &self.IgnoreEmbeddedPointers)
            .field("PointerBufferMark", &self.PointerBufferMark)
            .field("CorrDespIncrement", &self.CorrDespIncrement)
            .field("uFlags", &self.uFlags)
            .field("UniquePtrCount", &self.UniquePtrCount)
            .field("MaxCount", &self.MaxCount)
            .field("Offset", &self.Offset)
            .field("ActualCount", &self.ActualCount)
            .field("pfnAllocate", &self.pfnAllocate)
            .field("pfnFree", &self.pfnFree)
            .field("StackTop", &self.StackTop)
            .field("pPresentedType", &self.pPresentedType)
            .field("pTransmitType", &self.pTransmitType)
            .field("SavedHandle", &self.SavedHandle)
            .field("StubDesc", &self.StubDesc)
            .field("FullPtrXlatTables", &self.FullPtrXlatTables)
            .field("FullPtrRefId", &self.FullPtrRefId)
            .field("PointerLength", &self.PointerLength)
            .field("_bitfield", &self._bitfield)
            .field("dwDestContext", &self.dwDestContext)
            .field("pvDestContext", &self.pvDestContext)
            .field("SavedContextHandles", &self.SavedContextHandles)
            .field("ParamNumber", &self.ParamNumber)
            .field("pRpcChannelBuffer", &self.pRpcChannelBuffer)
            .field("pArrayInfo", &self.pArrayInfo)
            .field("SizePtrCountArray", &self.SizePtrCountArray)
            .field("SizePtrOffsetArray", &self.SizePtrOffsetArray)
            .field("SizePtrLengthArray", &self.SizePtrLengthArray)
            .field("pArgQueue", &self.pArgQueue)
            .field("dwStubPhase", &self.dwStubPhase)
            .field("LowStackMark", &self.LowStackMark)
            .field("pAsyncMsg", &self.pAsyncMsg)
            .field("pCorrInfo", &self.pCorrInfo)
            .field("pCorrMemory", &self.pCorrMemory)
            .field("pMemoryList", &self.pMemoryList)
            .field("pCSInfo", &self.pCSInfo)
            .field("ConformanceMark", &self.ConformanceMark)
            .field("VarianceMark", &self.VarianceMark)
            .field("Unused", &self.Unused)
            .field("pContext", &self.pContext)
            .field("ContextHandleHash", &self.ContextHandleHash)
            .field("pUserMarshalList", &self.pUserMarshalList)
            .field("Reserved51_3", &self.Reserved51_3)
            .field("Reserved51_4", &self.Reserved51_4)
            .field("Reserved51_5", &self.Reserved51_5)
            .finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::TypeKind for MIDL_STUB_MESSAGE {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for MIDL_STUB_MESSAGE {
    fn eq(&self, other: &Self) -> bool {
        self.RpcMsg == other.RpcMsg
            && self.Buffer == other.Buffer
            && self.BufferStart == other.BufferStart
            && self.BufferEnd == other.BufferEnd
            && self.BufferMark == other.BufferMark
            && self.BufferLength == other.BufferLength
            && self.MemorySize == other.MemorySize
            && self.Memory == other.Memory
            && self.IsClient == other.IsClient
            && self.Pad == other.Pad
            && self.uFlags2 == other.uFlags2
            && self.ReuseBuffer == other.ReuseBuffer
            && self.pAllocAllNodesContext == other.pAllocAllNodesContext
            && self.pPointerQueueState == other.pPointerQueueState
            && self.IgnoreEmbeddedPointers == other.IgnoreEmbeddedPointers
            && self.PointerBufferMark == other.PointerBufferMark
            && self.CorrDespIncrement == other.CorrDespIncrement
            && self.uFlags == other.uFlags
            && self.UniquePtrCount == other.UniquePtrCount
            && self.MaxCount == other.MaxCount
            && self.Offset == other.Offset
            && self.ActualCount == other.ActualCount
            && self.pfnAllocate == other.pfnAllocate
            && self.pfnFree == other.pfnFree
            && self.StackTop == other.StackTop
            && self.pPresentedType == other.pPresentedType
            && self.pTransmitType == other.pTransmitType
            && self.SavedHandle == other.SavedHandle
            && self.StubDesc == other.StubDesc
            && self.FullPtrXlatTables == other.FullPtrXlatTables
            && self.FullPtrRefId == other.FullPtrRefId
            && self.PointerLength == other.PointerLength
            && self._bitfield == other._bitfield
            && self.dwDestContext == other.dwDestContext
            && self.pvDestContext == other.pvDestContext
            && self.SavedContextHandles == other.SavedContextHandles
            && self.ParamNumber == other.ParamNumber
            && self.pRpcChannelBuffer == other.pRpcChannelBuffer
            && self.pArrayInfo == other.pArrayInfo
            && self.SizePtrCountArray == other.SizePtrCountArray
            && self.SizePtrOffsetArray == other.SizePtrOffsetArray
            && self.SizePtrLengthArray == other.SizePtrLengthArray
            && self.pArgQueue == other.pArgQueue
            && self.dwStubPhase == other.dwStubPhase
            && self.LowStackMark == other.LowStackMark
            && self.pAsyncMsg == other.pAsyncMsg
            && self.pCorrInfo == other.pCorrInfo
            && self.pCorrMemory == other.pCorrMemory
            && self.pMemoryList == other.pMemoryList
            && self.pCSInfo == other.pCSInfo
            && self.ConformanceMark == other.ConformanceMark
            && self.VarianceMark == other.VarianceMark
            && self.Unused == other.Unused
            && self.pContext == other.pContext
            && self.ContextHandleHash == other.ContextHandleHash
            && self.pUserMarshalList == other.pUserMarshalList
            && self.Reserved51_3 == other.Reserved51_3
            && self.Reserved51_4 == other.Reserved51_4
            && self.Reserved51_5 == other.Reserved51_5
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for MIDL_STUB_MESSAGE {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::default::Default for MIDL_STUB_MESSAGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct MIDL_SYNTAX_INFO {
    pub TransferSyntax: RPC_SYNTAX_IDENTIFIER,
    pub DispatchTable: *mut RPC_DISPATCH_TABLE,
    pub ProcString: *mut u8,
    pub FmtStringOffset: *const u16,
    pub TypeString: *mut u8,
    pub aUserMarshalQuadruple: *const ::core::ffi::c_void,
    pub pMethodProperties: *const MIDL_INTERFACE_METHOD_PROPERTIES,
    pub pReserved2: usize,
}
impl ::core::marker::Copy for MIDL_SYNTAX_INFO {}
impl ::core::clone::Clone for MIDL_SYNTAX_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MIDL_SYNTAX_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIDL_SYNTAX_INFO").field("TransferSyntax", &self.TransferSyntax).field("DispatchTable", &self.DispatchTable).field("ProcString", &self.ProcString).field("FmtStringOffset", &self.FmtStringOffset).field("TypeString", &self.TypeString).field("aUserMarshalQuadruple", &self.aUserMarshalQuadruple).field("pMethodProperties", &self.pMethodProperties).field("pReserved2", &self.pReserved2).finish()
    }
}
impl ::windows::core::TypeKind for MIDL_SYNTAX_INFO {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for MIDL_SYNTAX_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.TransferSyntax == other.TransferSyntax && self.DispatchTable == other.DispatchTable && self.ProcString == other.ProcString && self.FmtStringOffset == other.FmtStringOffset && self.TypeString == other.TypeString && self.aUserMarshalQuadruple == other.aUserMarshalQuadruple && self.pMethodProperties == other.pMethodProperties && self.pReserved2 == other.pReserved2
    }
}
impl ::core::cmp::Eq for MIDL_SYNTAX_INFO {}
impl ::core::default::Default for MIDL_SYNTAX_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct MIDL_TYPE_PICKLING_INFO {
    pub Version: u32,
    pub Flags: u32,
    pub Reserved: [usize; 3],
}
impl ::core::marker::Copy for MIDL_TYPE_PICKLING_INFO {}
impl ::core::clone::Clone for MIDL_TYPE_PICKLING_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MIDL_TYPE_PICKLING_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIDL_TYPE_PICKLING_INFO").field("Version", &self.Version).field("Flags", &self.Flags).field("Reserved", &self.Reserved).finish()
    }
}
impl ::windows::core::TypeKind for MIDL_TYPE_PICKLING_INFO {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for MIDL_TYPE_PICKLING_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.Flags == other.Flags && self.Reserved == other.Reserved
    }
}
impl ::core::cmp::Eq for MIDL_TYPE_PICKLING_INFO {}
impl ::core::default::Default for MIDL_TYPE_PICKLING_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub struct MIDL_WINRT_TYPE_SERIALIZATION_INFO {
    pub Version: u32,
    pub TypeFormatString: *mut u8,
    pub FormatStringSize: u16,
    pub TypeOffset: u16,
    pub StubDesc: *mut MIDL_STUB_DESC,
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::marker::Copy for MIDL_WINRT_TYPE_SERIALIZATION_INFO {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for MIDL_WINRT_TYPE_SERIALIZATION_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for MIDL_WINRT_TYPE_SERIALIZATION_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIDL_WINRT_TYPE_SERIALIZATION_INFO").field("Version", &self.Version).field("TypeFormatString", &self.TypeFormatString).field("FormatStringSize", &self.FormatStringSize).field("TypeOffset", &self.TypeOffset).field("StubDesc", &self.StubDesc).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::TypeKind for MIDL_WINRT_TYPE_SERIALIZATION_INFO {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for MIDL_WINRT_TYPE_SERIALIZATION_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.TypeFormatString == other.TypeFormatString && self.FormatStringSize == other.FormatStringSize && self.TypeOffset == other.TypeOffset && self.StubDesc == other.StubDesc
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for MIDL_WINRT_TYPE_SERIALIZATION_INFO {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::default::Default for MIDL_WINRT_TYPE_SERIALIZATION_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct NDR64_ARRAY_ELEMENT_INFO {
    pub ElementMemSize: u32,
    pub Element: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for NDR64_ARRAY_ELEMENT_INFO {}
impl ::core::clone::Clone for NDR64_ARRAY_ELEMENT_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDR64_ARRAY_ELEMENT_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDR64_ARRAY_ELEMENT_INFO").field("ElementMemSize", &self.ElementMemSize).field("Element", &self.Element).finish()
    }
}
impl ::windows::core::TypeKind for NDR64_ARRAY_ELEMENT_INFO {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for NDR64_ARRAY_ELEMENT_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.ElementMemSize == other.ElementMemSize && self.Element == other.Element
    }
}
impl ::core::cmp::Eq for NDR64_ARRAY_ELEMENT_INFO {}
impl ::core::default::Default for NDR64_ARRAY_ELEMENT_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct NDR64_ARRAY_FLAGS {
    pub _bitfield: u8,
}
impl ::core::marker::Copy for NDR64_ARRAY_FLAGS {}
impl ::core::clone::Clone for NDR64_ARRAY_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDR64_ARRAY_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDR64_ARRAY_FLAGS").field("_bitfield", &self._bitfield).finish()
    }
}
impl ::windows::core::TypeKind for NDR64_ARRAY_FLAGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for NDR64_ARRAY_FLAGS {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
impl ::core::cmp::Eq for NDR64_ARRAY_FLAGS {}
impl ::core::default::Default for NDR64_ARRAY_FLAGS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub union NDR64_BINDINGS {
    pub Primitive: NDR64_BIND_PRIMITIVE,
    pub Generic: NDR64_BIND_GENERIC,
    pub Context: NDR64_BIND_CONTEXT,
}
impl ::core::marker::Copy for NDR64_BINDINGS {}
impl ::core::clone::Clone for NDR64_BINDINGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for NDR64_BINDINGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for NDR64_BINDINGS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct NDR64_BIND_AND_NOTIFY_EXTENSION {
    pub Binding: NDR64_BIND_CONTEXT,
    pub NotifyIndex: u16,
}
impl ::core::marker::Copy for NDR64_BIND_AND_NOTIFY_EXTENSION {}
impl ::core::clone::Clone for NDR64_BIND_AND_NOTIFY_EXTENSION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDR64_BIND_AND_NOTIFY_EXTENSION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDR64_BIND_AND_NOTIFY_EXTENSION").field("Binding", &self.Binding).field("NotifyIndex", &self.NotifyIndex).finish()
    }
}
impl ::windows::core::TypeKind for NDR64_BIND_AND_NOTIFY_EXTENSION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for NDR64_BIND_AND_NOTIFY_EXTENSION {
    fn eq(&self, other: &Self) -> bool {
        self.Binding == other.Binding && self.NotifyIndex == other.NotifyIndex
    }
}
impl ::core::cmp::Eq for NDR64_BIND_AND_NOTIFY_EXTENSION {}
impl ::core::default::Default for NDR64_BIND_AND_NOTIFY_EXTENSION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct NDR64_BIND_CONTEXT {
    pub HandleType: u8,
    pub Flags: u8,
    pub StackOffset: u16,
    pub RoutineIndex: u8,
    pub Ordinal: u8,
}
impl ::core::marker::Copy for NDR64_BIND_CONTEXT {}
impl ::core::clone::Clone for NDR64_BIND_CONTEXT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDR64_BIND_CONTEXT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDR64_BIND_CONTEXT").field("HandleType", &self.HandleType).field("Flags", &self.Flags).field("StackOffset", &self.StackOffset).field("RoutineIndex", &self.RoutineIndex).field("Ordinal", &self.Ordinal).finish()
    }
}
impl ::windows::core::TypeKind for NDR64_BIND_CONTEXT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for NDR64_BIND_CONTEXT {
    fn eq(&self, other: &Self) -> bool {
        self.HandleType == other.HandleType && self.Flags == other.Flags && self.StackOffset == other.StackOffset && self.RoutineIndex == other.RoutineIndex && self.Ordinal == other.Ordinal
    }
}
impl ::core::cmp::Eq for NDR64_BIND_CONTEXT {}
impl ::core::default::Default for NDR64_BIND_CONTEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct NDR64_BIND_GENERIC {
    pub HandleType: u8,
    pub Flags: u8,
    pub StackOffset: u16,
    pub RoutineIndex: u8,
    pub Size: u8,
}
impl ::core::marker::Copy for NDR64_BIND_GENERIC {}
impl ::core::clone::Clone for NDR64_BIND_GENERIC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDR64_BIND_GENERIC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDR64_BIND_GENERIC").field("HandleType", &self.HandleType).field("Flags", &self.Flags).field("StackOffset", &self.StackOffset).field("RoutineIndex", &self.RoutineIndex).field("Size", &self.Size).finish()
    }
}
impl ::windows::core::TypeKind for NDR64_BIND_GENERIC {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for NDR64_BIND_GENERIC {
    fn eq(&self, other: &Self) -> bool {
        self.HandleType == other.HandleType && self.Flags == other.Flags && self.StackOffset == other.StackOffset && self.RoutineIndex == other.RoutineIndex && self.Size == other.Size
    }
}
impl ::core::cmp::Eq for NDR64_BIND_GENERIC {}
impl ::core::default::Default for NDR64_BIND_GENERIC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct NDR64_BIND_PRIMITIVE {
    pub HandleType: u8,
    pub Flags: u8,
    pub StackOffset: u16,
    pub Reserved: u16,
}
impl ::core::marker::Copy for NDR64_BIND_PRIMITIVE {}
impl ::core::clone::Clone for NDR64_BIND_PRIMITIVE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDR64_BIND_PRIMITIVE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDR64_BIND_PRIMITIVE").field("HandleType", &self.HandleType).field("Flags", &self.Flags).field("StackOffset", &self.StackOffset).field("Reserved", &self.Reserved).finish()
    }
}
impl ::windows::core::TypeKind for NDR64_BIND_PRIMITIVE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for NDR64_BIND_PRIMITIVE {
    fn eq(&self, other: &Self) -> bool {
        self.HandleType == other.HandleType && self.Flags == other.Flags && self.StackOffset == other.StackOffset && self.Reserved == other.Reserved
    }
}
impl ::core::cmp::Eq for NDR64_BIND_PRIMITIVE {}
impl ::core::default::Default for NDR64_BIND_PRIMITIVE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct NDR64_BOGUS_ARRAY_HEADER_FORMAT {
    pub FormatCode: u8,
    pub Alignment: u8,
    pub Flags: NDR64_ARRAY_FLAGS,
    pub NumberDims: u8,
    pub NumberElements: u32,
    pub Element: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for NDR64_BOGUS_ARRAY_HEADER_FORMAT {}
impl ::core::clone::Clone for NDR64_BOGUS_ARRAY_HEADER_FORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDR64_BOGUS_ARRAY_HEADER_FORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDR64_BOGUS_ARRAY_HEADER_FORMAT").field("FormatCode", &self.FormatCode).field("Alignment", &self.Alignment).field("Flags", &self.Flags).field("NumberDims", &self.NumberDims).field("NumberElements", &self.NumberElements).field("Element", &self.Element).finish()
    }
}
impl ::windows::core::TypeKind for NDR64_BOGUS_ARRAY_HEADER_FORMAT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for NDR64_BOGUS_ARRAY_HEADER_FORMAT {
    fn eq(&self, other: &Self) -> bool {
        self.FormatCode == other.FormatCode && self.Alignment == other.Alignment && self.Flags == other.Flags && self.NumberDims == other.NumberDims && self.NumberElements == other.NumberElements && self.Element == other.Element
    }
}
impl ::core::cmp::Eq for NDR64_BOGUS_ARRAY_HEADER_FORMAT {}
impl ::core::default::Default for NDR64_BOGUS_ARRAY_HEADER_FORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct NDR64_BOGUS_STRUCTURE_HEADER_FORMAT {
    pub FormatCode: u8,
    pub Alignment: u8,
    pub Flags: NDR64_STRUCTURE_FLAGS,
    pub Reserve: u8,
    pub MemorySize: u32,
    pub OriginalMemberLayout: *mut ::core::ffi::c_void,
    pub OriginalPointerLayout: *mut ::core::ffi::c_void,
    pub PointerLayout: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for NDR64_BOGUS_STRUCTURE_HEADER_FORMAT {}
impl ::core::clone::Clone for NDR64_BOGUS_STRUCTURE_HEADER_FORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDR64_BOGUS_STRUCTURE_HEADER_FORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDR64_BOGUS_STRUCTURE_HEADER_FORMAT").field("FormatCode", &self.FormatCode).field("Alignment", &self.Alignment).field("Flags", &self.Flags).field("Reserve", &self.Reserve).field("MemorySize", &self.MemorySize).field("OriginalMemberLayout", &self.OriginalMemberLayout).field("OriginalPointerLayout", &self.OriginalPointerLayout).field("PointerLayout", &self.PointerLayout).finish()
    }
}
impl ::windows::core::TypeKind for NDR64_BOGUS_STRUCTURE_HEADER_FORMAT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for NDR64_BOGUS_STRUCTURE_HEADER_FORMAT {
    fn eq(&self, other: &Self) -> bool {
        self.FormatCode == other.FormatCode && self.Alignment == other.Alignment && self.Flags == other.Flags && self.Reserve == other.Reserve && self.MemorySize == other.MemorySize && self.OriginalMemberLayout == other.OriginalMemberLayout && self.OriginalPointerLayout == other.OriginalPointerLayout && self.PointerLayout == other.PointerLayout
    }
}
impl ::core::cmp::Eq for NDR64_BOGUS_STRUCTURE_HEADER_FORMAT {}
impl ::core::default::Default for NDR64_BOGUS_STRUCTURE_HEADER_FORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct NDR64_BUFFER_ALIGN_FORMAT {
    pub FormatCode: u8,
    pub Alignment: u8,
    pub Reserved: u16,
    pub Reserved2: u32,
}
impl ::core::marker::Copy for NDR64_BUFFER_ALIGN_FORMAT {}
impl ::core::clone::Clone for NDR64_BUFFER_ALIGN_FORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDR64_BUFFER_ALIGN_FORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDR64_BUFFER_ALIGN_FORMAT").field("FormatCode", &self.FormatCode).field("Alignment", &self.Alignment).field("Reserved", &self.Reserved).field("Reserved2", &self.Reserved2).finish()
    }
}
impl ::windows::core::TypeKind for NDR64_BUFFER_ALIGN_FORMAT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for NDR64_BUFFER_ALIGN_FORMAT {
    fn eq(&self, other: &Self) -> bool {
        self.FormatCode == other.FormatCode && self.Alignment == other.Alignment && self.Reserved == other.Reserved && self.Reserved2 == other.Reserved2
    }
}
impl ::core::cmp::Eq for NDR64_BUFFER_ALIGN_FORMAT {}
impl ::core::default::Default for NDR64_BUFFER_ALIGN_FORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct NDR64_CONFORMANT_STRING_FORMAT {
    pub Header: NDR64_STRING_HEADER_FORMAT,
}
impl ::core::marker::Copy for NDR64_CONFORMANT_STRING_FORMAT {}
impl ::core::clone::Clone for NDR64_CONFORMANT_STRING_FORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDR64_CONFORMANT_STRING_FORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDR64_CONFORMANT_STRING_FORMAT").field("Header", &self.Header).finish()
    }
}
impl ::windows::core::TypeKind for NDR64_CONFORMANT_STRING_FORMAT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for NDR64_CONFORMANT_STRING_FORMAT {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header
    }
}
impl ::core::cmp::Eq for NDR64_CONFORMANT_STRING_FORMAT {}
impl ::core::default::Default for NDR64_CONFORMANT_STRING_FORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct NDR64_CONF_ARRAY_HEADER_FORMAT {
    pub FormatCode: u8,
    pub Alignment: u8,
    pub Flags: NDR64_ARRAY_FLAGS,
    pub Reserved: u8,
    pub ElementSize: u32,
    pub ConfDescriptor: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for NDR64_CONF_ARRAY_HEADER_FORMAT {}
impl ::core::clone::Clone for NDR64_CONF_ARRAY_HEADER_FORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDR64_CONF_ARRAY_HEADER_FORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDR64_CONF_ARRAY_HEADER_FORMAT").field("FormatCode", &self.FormatCode).field("Alignment", &self.Alignment).field("Flags", &self.Flags).field("Reserved", &self.Reserved).field("ElementSize", &self.ElementSize).field("ConfDescriptor", &self.ConfDescriptor).finish()
    }
}
impl ::windows::core::TypeKind for NDR64_CONF_ARRAY_HEADER_FORMAT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for NDR64_CONF_ARRAY_HEADER_FORMAT {
    fn eq(&self, other: &Self) -> bool {
        self.FormatCode == other.FormatCode && self.Alignment == other.Alignment && self.Flags == other.Flags && self.Reserved == other.Reserved && self.ElementSize == other.ElementSize && self.ConfDescriptor == other.ConfDescriptor
    }
}
impl ::core::cmp::Eq for NDR64_CONF_ARRAY_HEADER_FORMAT {}
impl ::core::default::Default for NDR64_CONF_ARRAY_HEADER_FORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct NDR64_CONF_BOGUS_STRUCTURE_HEADER_FORMAT {
    pub FormatCode: u8,
    pub Alignment: u8,
    pub Flags: NDR64_STRUCTURE_FLAGS,
    pub Dimensions: u8,
    pub MemorySize: u32,
    pub OriginalMemberLayout: *mut ::core::ffi::c_void,
    pub OriginalPointerLayout: *mut ::core::ffi::c_void,
    pub PointerLayout: *mut ::core::ffi::c_void,
    pub ConfArrayDescription: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for NDR64_CONF_BOGUS_STRUCTURE_HEADER_FORMAT {}
impl ::core::clone::Clone for NDR64_CONF_BOGUS_STRUCTURE_HEADER_FORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDR64_CONF_BOGUS_STRUCTURE_HEADER_FORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDR64_CONF_BOGUS_STRUCTURE_HEADER_FORMAT").field("FormatCode", &self.FormatCode).field("Alignment", &self.Alignment).field("Flags", &self.Flags).field("Dimensions", &self.Dimensions).field("MemorySize", &self.MemorySize).field("OriginalMemberLayout", &self.OriginalMemberLayout).field("OriginalPointerLayout", &self.OriginalPointerLayout).field("PointerLayout", &self.PointerLayout).field("ConfArrayDescription", &self.ConfArrayDescription).finish()
    }
}
impl ::windows::core::TypeKind for NDR64_CONF_BOGUS_STRUCTURE_HEADER_FORMAT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for NDR64_CONF_BOGUS_STRUCTURE_HEADER_FORMAT {
    fn eq(&self, other: &Self) -> bool {
        self.FormatCode == other.FormatCode && self.Alignment == other.Alignment && self.Flags == other.Flags && self.Dimensions == other.Dimensions && self.MemorySize == other.MemorySize && self.OriginalMemberLayout == other.OriginalMemberLayout && self.OriginalPointerLayout == other.OriginalPointerLayout && self.PointerLayout == other.PointerLayout && self.ConfArrayDescription == other.ConfArrayDescription
    }
}
impl ::core::cmp::Eq for NDR64_CONF_BOGUS_STRUCTURE_HEADER_FORMAT {}
impl ::core::default::Default for NDR64_CONF_BOGUS_STRUCTURE_HEADER_FORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct NDR64_CONF_STRUCTURE_HEADER_FORMAT {
    pub FormatCode: u8,
    pub Alignment: u8,
    pub Flags: NDR64_STRUCTURE_FLAGS,
    pub Reserve: u8,
    pub MemorySize: u32,
    pub ArrayDescription: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for NDR64_CONF_STRUCTURE_HEADER_FORMAT {}
impl ::core::clone::Clone for NDR64_CONF_STRUCTURE_HEADER_FORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDR64_CONF_STRUCTURE_HEADER_FORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDR64_CONF_STRUCTURE_HEADER_FORMAT").field("FormatCode", &self.FormatCode).field("Alignment", &self.Alignment).field("Flags", &self.Flags).field("Reserve", &self.Reserve).field("MemorySize", &self.MemorySize).field("ArrayDescription", &self.ArrayDescription).finish()
    }
}
impl ::windows::core::TypeKind for NDR64_CONF_STRUCTURE_HEADER_FORMAT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for NDR64_CONF_STRUCTURE_HEADER_FORMAT {
    fn eq(&self, other: &Self) -> bool {
        self.FormatCode == other.FormatCode && self.Alignment == other.Alignment && self.Flags == other.Flags && self.Reserve == other.Reserve && self.MemorySize == other.MemorySize && self.ArrayDescription == other.ArrayDescription
    }
}
impl ::core::cmp::Eq for NDR64_CONF_STRUCTURE_HEADER_FORMAT {}
impl ::core::default::Default for NDR64_CONF_STRUCTURE_HEADER_FORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct NDR64_CONF_VAR_ARRAY_HEADER_FORMAT {
    pub FormatCode: u8,
    pub Alignment: u8,
    pub Flags: NDR64_ARRAY_FLAGS,
    pub Reserved: u8,
    pub ElementSize: u32,
    pub ConfDescriptor: *mut ::core::ffi::c_void,
    pub VarDescriptor: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for NDR64_CONF_VAR_ARRAY_HEADER_FORMAT {}
impl ::core::clone::Clone for NDR64_CONF_VAR_ARRAY_HEADER_FORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDR64_CONF_VAR_ARRAY_HEADER_FORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDR64_CONF_VAR_ARRAY_HEADER_FORMAT").field("FormatCode", &self.FormatCode).field("Alignment", &self.Alignment).field("Flags", &self.Flags).field("Reserved", &self.Reserved).field("ElementSize", &self.ElementSize).field("ConfDescriptor", &self.ConfDescriptor).field("VarDescriptor", &self.VarDescriptor).finish()
    }
}
impl ::windows::core::TypeKind for NDR64_CONF_VAR_ARRAY_HEADER_FORMAT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for NDR64_CONF_VAR_ARRAY_HEADER_FORMAT {
    fn eq(&self, other: &Self) -> bool {
        self.FormatCode == other.FormatCode && self.Alignment == other.Alignment && self.Flags == other.Flags && self.Reserved == other.Reserved && self.ElementSize == other.ElementSize && self.ConfDescriptor == other.ConfDescriptor && self.VarDescriptor == other.VarDescriptor
    }
}
impl ::core::cmp::Eq for NDR64_CONF_VAR_ARRAY_HEADER_FORMAT {}
impl ::core::default::Default for NDR64_CONF_VAR_ARRAY_HEADER_FORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct NDR64_CONF_VAR_BOGUS_ARRAY_HEADER_FORMAT {
    pub FixedArrayFormat: NDR64_BOGUS_ARRAY_HEADER_FORMAT,
    pub ConfDescription: *mut ::core::ffi::c_void,
    pub VarDescription: *mut ::core::ffi::c_void,
    pub OffsetDescription: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for NDR64_CONF_VAR_BOGUS_ARRAY_HEADER_FORMAT {}
impl ::core::clone::Clone for NDR64_CONF_VAR_BOGUS_ARRAY_HEADER_FORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDR64_CONF_VAR_BOGUS_ARRAY_HEADER_FORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDR64_CONF_VAR_BOGUS_ARRAY_HEADER_FORMAT").field("FixedArrayFormat", &self.FixedArrayFormat).field("ConfDescription", &self.ConfDescription).field("VarDescription", &self.VarDescription).field("OffsetDescription", &self.OffsetDescription).finish()
    }
}
impl ::windows::core::TypeKind for NDR64_CONF_VAR_BOGUS_ARRAY_HEADER_FORMAT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for NDR64_CONF_VAR_BOGUS_ARRAY_HEADER_FORMAT {
    fn eq(&self, other: &Self) -> bool {
        self.FixedArrayFormat == other.FixedArrayFormat && self.ConfDescription == other.ConfDescription && self.VarDescription == other.VarDescription && self.OffsetDescription == other.OffsetDescription
    }
}
impl ::core::cmp::Eq for NDR64_CONF_VAR_BOGUS_ARRAY_HEADER_FORMAT {}
impl ::core::default::Default for NDR64_CONF_VAR_BOGUS_ARRAY_HEADER_FORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct NDR64_CONSTANT_IID_FORMAT {
    pub FormatCode: u8,
    pub Flags: u8,
    pub Reserved: u16,
    pub Guid: ::windows::core::GUID,
}
impl ::core::marker::Copy for NDR64_CONSTANT_IID_FORMAT {}
impl ::core::clone::Clone for NDR64_CONSTANT_IID_FORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDR64_CONSTANT_IID_FORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDR64_CONSTANT_IID_FORMAT").field("FormatCode", &self.FormatCode).field("Flags", &self.Flags).field("Reserved", &self.Reserved).field("Guid", &self.Guid).finish()
    }
}
impl ::windows::core::TypeKind for NDR64_CONSTANT_IID_FORMAT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for NDR64_CONSTANT_IID_FORMAT {
    fn eq(&self, other: &Self) -> bool {
        self.FormatCode == other.FormatCode && self.Flags == other.Flags && self.Reserved == other.Reserved && self.Guid == other.Guid
    }
}
impl ::core::cmp::Eq for NDR64_CONSTANT_IID_FORMAT {}
impl ::core::default::Default for NDR64_CONSTANT_IID_FORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct NDR64_CONTEXT_HANDLE_FLAGS {
    pub _bitfield: u8,
}
impl ::core::marker::Copy for NDR64_CONTEXT_HANDLE_FLAGS {}
impl ::core::clone::Clone for NDR64_CONTEXT_HANDLE_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDR64_CONTEXT_HANDLE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDR64_CONTEXT_HANDLE_FLAGS").field("_bitfield", &self._bitfield).finish()
    }
}
impl ::windows::core::TypeKind for NDR64_CONTEXT_HANDLE_FLAGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for NDR64_CONTEXT_HANDLE_FLAGS {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
impl ::core::cmp::Eq for NDR64_CONTEXT_HANDLE_FLAGS {}
impl ::core::default::Default for NDR64_CONTEXT_HANDLE_FLAGS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct NDR64_CONTEXT_HANDLE_FORMAT {
    pub FormatCode: u8,
    pub ContextFlags: u8,
    pub RundownRoutineIndex: u8,
    pub Ordinal: u8,
}
impl ::core::marker::Copy for NDR64_CONTEXT_HANDLE_FORMAT {}
impl ::core::clone::Clone for NDR64_CONTEXT_HANDLE_FORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDR64_CONTEXT_HANDLE_FORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDR64_CONTEXT_HANDLE_FORMAT").field("FormatCode", &self.FormatCode).field("ContextFlags", &self.ContextFlags).field("RundownRoutineIndex", &self.RundownRoutineIndex).field("Ordinal", &self.Ordinal).finish()
    }
}
impl ::windows::core::TypeKind for NDR64_CONTEXT_HANDLE_FORMAT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for NDR64_CONTEXT_HANDLE_FORMAT {
    fn eq(&self, other: &Self) -> bool {
        self.FormatCode == other.FormatCode && self.ContextFlags == other.ContextFlags && self.RundownRoutineIndex == other.RundownRoutineIndex && self.Ordinal == other.Ordinal
    }
}
impl ::core::cmp::Eq for NDR64_CONTEXT_HANDLE_FORMAT {}
impl ::core::default::Default for NDR64_CONTEXT_HANDLE_FORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct NDR64_EMBEDDED_COMPLEX_FORMAT {
    pub FormatCode: u8,
    pub Reserve1: u8,
    pub Reserve2: u16,
    pub Type: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for NDR64_EMBEDDED_COMPLEX_FORMAT {}
impl ::core::clone::Clone for NDR64_EMBEDDED_COMPLEX_FORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDR64_EMBEDDED_COMPLEX_FORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDR64_EMBEDDED_COMPLEX_FORMAT").field("FormatCode", &self.FormatCode).field("Reserve1", &self.Reserve1).field("Reserve2", &self.Reserve2).field("Type", &self.Type).finish()
    }
}
impl ::windows::core::TypeKind for NDR64_EMBEDDED_COMPLEX_FORMAT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for NDR64_EMBEDDED_COMPLEX_FORMAT {
    fn eq(&self, other: &Self) -> bool {
        self.FormatCode == other.FormatCode && self.Reserve1 == other.Reserve1 && self.Reserve2 == other.Reserve2 && self.Type == other.Type
    }
}
impl ::core::cmp::Eq for NDR64_EMBEDDED_COMPLEX_FORMAT {}
impl ::core::default::Default for NDR64_EMBEDDED_COMPLEX_FORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct NDR64_ENCAPSULATED_UNION {
    pub FormatCode: u8,
    pub Alignment: u8,
    pub Flags: u8,
    pub SwitchType: u8,
    pub MemoryOffset: u32,
    pub MemorySize: u32,
    pub Reserved: u32,
}
impl ::core::marker::Copy for NDR64_ENCAPSULATED_UNION {}
impl ::core::clone::Clone for NDR64_ENCAPSULATED_UNION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDR64_ENCAPSULATED_UNION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDR64_ENCAPSULATED_UNION").field("FormatCode", &self.FormatCode).field("Alignment", &self.Alignment).field("Flags", &self.Flags).field("SwitchType", &self.SwitchType).field("MemoryOffset", &self.MemoryOffset).field("MemorySize", &self.MemorySize).field("Reserved", &self.Reserved).finish()
    }
}
impl ::windows::core::TypeKind for NDR64_ENCAPSULATED_UNION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for NDR64_ENCAPSULATED_UNION {
    fn eq(&self, other: &Self) -> bool {
        self.FormatCode == other.FormatCode && self.Alignment == other.Alignment && self.Flags == other.Flags && self.SwitchType == other.SwitchType && self.MemoryOffset == other.MemoryOffset && self.MemorySize == other.MemorySize && self.Reserved == other.Reserved
    }
}
impl ::core::cmp::Eq for NDR64_ENCAPSULATED_UNION {}
impl ::core::default::Default for NDR64_ENCAPSULATED_UNION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct NDR64_EXPR_CONST32 {
    pub ExprType: u8,
    pub Reserved: u8,
    pub Reserved1: u16,
    pub ConstValue: u32,
}
impl ::core::marker::Copy for NDR64_EXPR_CONST32 {}
impl ::core::clone::Clone for NDR64_EXPR_CONST32 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDR64_EXPR_CONST32 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDR64_EXPR_CONST32").field("ExprType", &self.ExprType).field("Reserved", &self.Reserved).field("Reserved1", &self.Reserved1).field("ConstValue", &self.ConstValue).finish()
    }
}
impl ::windows::core::TypeKind for NDR64_EXPR_CONST32 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for NDR64_EXPR_CONST32 {
    fn eq(&self, other: &Self) -> bool {
        self.ExprType == other.ExprType && self.Reserved == other.Reserved && self.Reserved1 == other.Reserved1 && self.ConstValue == other.ConstValue
    }
}
impl ::core::cmp::Eq for NDR64_EXPR_CONST32 {}
impl ::core::default::Default for NDR64_EXPR_CONST32 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct NDR64_EXPR_CONST64 {
    pub ExprType: u8,
    pub Reserved: u8,
    pub Reserved1: u16,
    pub ConstValue: i64,
}
impl ::core::marker::Copy for NDR64_EXPR_CONST64 {}
impl ::core::clone::Clone for NDR64_EXPR_CONST64 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDR64_EXPR_CONST64 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDR64_EXPR_CONST64").field("ExprType", &self.ExprType).field("Reserved", &self.Reserved).field("Reserved1", &self.Reserved1).field("ConstValue", &self.ConstValue).finish()
    }
}
impl ::windows::core::TypeKind for NDR64_EXPR_CONST64 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for NDR64_EXPR_CONST64 {
    fn eq(&self, other: &Self) -> bool {
        self.ExprType == other.ExprType && self.Reserved == other.Reserved && self.Reserved1 == other.Reserved1 && self.ConstValue == other.ConstValue
    }
}
impl ::core::cmp::Eq for NDR64_EXPR_CONST64 {}
impl ::core::default::Default for NDR64_EXPR_CONST64 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct NDR64_EXPR_NOOP {
    pub ExprType: u8,
    pub Size: u8,
    pub Reserved: u16,
}
impl ::core::marker::Copy for NDR64_EXPR_NOOP {}
impl ::core::clone::Clone for NDR64_EXPR_NOOP {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDR64_EXPR_NOOP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDR64_EXPR_NOOP").field("ExprType", &self.ExprType).field("Size", &self.Size).field("Reserved", &self.Reserved).finish()
    }
}
impl ::windows::core::TypeKind for NDR64_EXPR_NOOP {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for NDR64_EXPR_NOOP {
    fn eq(&self, other: &Self) -> bool {
        self.ExprType == other.ExprType && self.Size == other.Size && self.Reserved == other.Reserved
    }
}
impl ::core::cmp::Eq for NDR64_EXPR_NOOP {}
impl ::core::default::Default for NDR64_EXPR_NOOP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct NDR64_EXPR_OPERATOR {
    pub ExprType: u8,
    pub Operator: u8,
    pub CastType: u8,
    pub Reserved: u8,
}
impl ::core::marker::Copy for NDR64_EXPR_OPERATOR {}
impl ::core::clone::Clone for NDR64_EXPR_OPERATOR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDR64_EXPR_OPERATOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDR64_EXPR_OPERATOR").field("ExprType", &self.ExprType).field("Operator", &self.Operator).field("CastType", &self.CastType).field("Reserved", &self.Reserved).finish()
    }
}
impl ::windows::core::TypeKind for NDR64_EXPR_OPERATOR {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for NDR64_EXPR_OPERATOR {
    fn eq(&self, other: &Self) -> bool {
        self.ExprType == other.ExprType && self.Operator == other.Operator && self.CastType == other.CastType && self.Reserved == other.Reserved
    }
}
impl ::core::cmp::Eq for NDR64_EXPR_OPERATOR {}
impl ::core::default::Default for NDR64_EXPR_OPERATOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct NDR64_EXPR_VAR {
    pub ExprType: u8,
    pub VarType: u8,
    pub Reserved: u16,
    pub Offset: u32,
}
impl ::core::marker::Copy for NDR64_EXPR_VAR {}
impl ::core::clone::Clone for NDR64_EXPR_VAR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDR64_EXPR_VAR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDR64_EXPR_VAR").field("ExprType", &self.ExprType).field("VarType", &self.VarType).field("Reserved", &self.Reserved).field("Offset", &self.Offset).finish()
    }
}
impl ::windows::core::TypeKind for NDR64_EXPR_VAR {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for NDR64_EXPR_VAR {
    fn eq(&self, other: &Self) -> bool {
        self.ExprType == other.ExprType && self.VarType == other.VarType && self.Reserved == other.Reserved && self.Offset == other.Offset
    }
}
impl ::core::cmp::Eq for NDR64_EXPR_VAR {}
impl ::core::default::Default for NDR64_EXPR_VAR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct NDR64_FIXED_REPEAT_FORMAT {
    pub RepeatFormat: NDR64_REPEAT_FORMAT,
    pub Iterations: u32,
    pub Reserved: u32,
}
impl ::core::marker::Copy for NDR64_FIXED_REPEAT_FORMAT {}
impl ::core::clone::Clone for NDR64_FIXED_REPEAT_FORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDR64_FIXED_REPEAT_FORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDR64_FIXED_REPEAT_FORMAT").field("RepeatFormat", &self.RepeatFormat).field("Iterations", &self.Iterations).field("Reserved", &self.Reserved).finish()
    }
}
impl ::windows::core::TypeKind for NDR64_FIXED_REPEAT_FORMAT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for NDR64_FIXED_REPEAT_FORMAT {
    fn eq(&self, other: &Self) -> bool {
        self.RepeatFormat == other.RepeatFormat && self.Iterations == other.Iterations && self.Reserved == other.Reserved
    }
}
impl ::core::cmp::Eq for NDR64_FIXED_REPEAT_FORMAT {}
impl ::core::default::Default for NDR64_FIXED_REPEAT_FORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct NDR64_FIX_ARRAY_HEADER_FORMAT {
    pub FormatCode: u8,
    pub Alignment: u8,
    pub Flags: NDR64_ARRAY_FLAGS,
    pub Reserved: u8,
    pub TotalSize: u32,
}
impl ::core::marker::Copy for NDR64_FIX_ARRAY_HEADER_FORMAT {}
impl ::core::clone::Clone for NDR64_FIX_ARRAY_HEADER_FORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDR64_FIX_ARRAY_HEADER_FORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDR64_FIX_ARRAY_HEADER_FORMAT").field("FormatCode", &self.FormatCode).field("Alignment", &self.Alignment).field("Flags", &self.Flags).field("Reserved", &self.Reserved).field("TotalSize", &self.TotalSize).finish()
    }
}
impl ::windows::core::TypeKind for NDR64_FIX_ARRAY_HEADER_FORMAT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for NDR64_FIX_ARRAY_HEADER_FORMAT {
    fn eq(&self, other: &Self) -> bool {
        self.FormatCode == other.FormatCode && self.Alignment == other.Alignment && self.Flags == other.Flags && self.Reserved == other.Reserved && self.TotalSize == other.TotalSize
    }
}
impl ::core::cmp::Eq for NDR64_FIX_ARRAY_HEADER_FORMAT {}
impl ::core::default::Default for NDR64_FIX_ARRAY_HEADER_FORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct NDR64_IID_FLAGS {
    pub _bitfield: u8,
}
impl ::core::marker::Copy for NDR64_IID_FLAGS {}
impl ::core::clone::Clone for NDR64_IID_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDR64_IID_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDR64_IID_FLAGS").field("_bitfield", &self._bitfield).finish()
    }
}
impl ::windows::core::TypeKind for NDR64_IID_FLAGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for NDR64_IID_FLAGS {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
impl ::core::cmp::Eq for NDR64_IID_FLAGS {}
impl ::core::default::Default for NDR64_IID_FLAGS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct NDR64_IID_FORMAT {
    pub FormatCode: u8,
    pub Flags: u8,
    pub Reserved: u16,
    pub IIDDescriptor: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for NDR64_IID_FORMAT {}
impl ::core::clone::Clone for NDR64_IID_FORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDR64_IID_FORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDR64_IID_FORMAT").field("FormatCode", &self.FormatCode).field("Flags", &self.Flags).field("Reserved", &self.Reserved).field("IIDDescriptor", &self.IIDDescriptor).finish()
    }
}
impl ::windows::core::TypeKind for NDR64_IID_FORMAT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for NDR64_IID_FORMAT {
    fn eq(&self, other: &Self) -> bool {
        self.FormatCode == other.FormatCode && self.Flags == other.Flags && self.Reserved == other.Reserved && self.IIDDescriptor == other.IIDDescriptor
    }
}
impl ::core::cmp::Eq for NDR64_IID_FORMAT {}
impl ::core::default::Default for NDR64_IID_FORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct NDR64_MEMPAD_FORMAT {
    pub FormatCode: u8,
    pub Reserve1: u8,
    pub MemPad: u16,
    pub Reserved2: u32,
}
impl ::core::marker::Copy for NDR64_MEMPAD_FORMAT {}
impl ::core::clone::Clone for NDR64_MEMPAD_FORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDR64_MEMPAD_FORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDR64_MEMPAD_FORMAT").field("FormatCode", &self.FormatCode).field("Reserve1", &self.Reserve1).field("MemPad", &self.MemPad).field("Reserved2", &self.Reserved2).finish()
    }
}
impl ::windows::core::TypeKind for NDR64_MEMPAD_FORMAT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for NDR64_MEMPAD_FORMAT {
    fn eq(&self, other: &Self) -> bool {
        self.FormatCode == other.FormatCode && self.Reserve1 == other.Reserve1 && self.MemPad == other.MemPad && self.Reserved2 == other.Reserved2
    }
}
impl ::core::cmp::Eq for NDR64_MEMPAD_FORMAT {}
impl ::core::default::Default for NDR64_MEMPAD_FORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct NDR64_NON_CONFORMANT_STRING_FORMAT {
    pub Header: NDR64_STRING_HEADER_FORMAT,
    pub TotalSize: u32,
}
impl ::core::marker::Copy for NDR64_NON_CONFORMANT_STRING_FORMAT {}
impl ::core::clone::Clone for NDR64_NON_CONFORMANT_STRING_FORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDR64_NON_CONFORMANT_STRING_FORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDR64_NON_CONFORMANT_STRING_FORMAT").field("Header", &self.Header).field("TotalSize", &self.TotalSize).finish()
    }
}
impl ::windows::core::TypeKind for NDR64_NON_CONFORMANT_STRING_FORMAT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for NDR64_NON_CONFORMANT_STRING_FORMAT {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.TotalSize == other.TotalSize
    }
}
impl ::core::cmp::Eq for NDR64_NON_CONFORMANT_STRING_FORMAT {}
impl ::core::default::Default for NDR64_NON_CONFORMANT_STRING_FORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct NDR64_NON_ENCAPSULATED_UNION {
    pub FormatCode: u8,
    pub Alignment: u8,
    pub Flags: u8,
    pub SwitchType: u8,
    pub MemorySize: u32,
    pub Switch: *mut ::core::ffi::c_void,
    pub Reserved: u32,
}
impl ::core::marker::Copy for NDR64_NON_ENCAPSULATED_UNION {}
impl ::core::clone::Clone for NDR64_NON_ENCAPSULATED_UNION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDR64_NON_ENCAPSULATED_UNION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDR64_NON_ENCAPSULATED_UNION").field("FormatCode", &self.FormatCode).field("Alignment", &self.Alignment).field("Flags", &self.Flags).field("SwitchType", &self.SwitchType).field("MemorySize", &self.MemorySize).field("Switch", &self.Switch).field("Reserved", &self.Reserved).finish()
    }
}
impl ::windows::core::TypeKind for NDR64_NON_ENCAPSULATED_UNION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for NDR64_NON_ENCAPSULATED_UNION {
    fn eq(&self, other: &Self) -> bool {
        self.FormatCode == other.FormatCode && self.Alignment == other.Alignment && self.Flags == other.Flags && self.SwitchType == other.SwitchType && self.MemorySize == other.MemorySize && self.Switch == other.Switch && self.Reserved == other.Reserved
    }
}
impl ::core::cmp::Eq for NDR64_NON_ENCAPSULATED_UNION {}
impl ::core::default::Default for NDR64_NON_ENCAPSULATED_UNION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct NDR64_NO_REPEAT_FORMAT {
    pub FormatCode: u8,
    pub Flags: u8,
    pub Reserved1: u16,
    pub Reserved2: u32,
}
impl ::core::marker::Copy for NDR64_NO_REPEAT_FORMAT {}
impl ::core::clone::Clone for NDR64_NO_REPEAT_FORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDR64_NO_REPEAT_FORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDR64_NO_REPEAT_FORMAT").field("FormatCode", &self.FormatCode).field("Flags", &self.Flags).field("Reserved1", &self.Reserved1).field("Reserved2", &self.Reserved2).finish()
    }
}
impl ::windows::core::TypeKind for NDR64_NO_REPEAT_FORMAT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for NDR64_NO_REPEAT_FORMAT {
    fn eq(&self, other: &Self) -> bool {
        self.FormatCode == other.FormatCode && self.Flags == other.Flags && self.Reserved1 == other.Reserved1 && self.Reserved2 == other.Reserved2
    }
}
impl ::core::cmp::Eq for NDR64_NO_REPEAT_FORMAT {}
impl ::core::default::Default for NDR64_NO_REPEAT_FORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct NDR64_PARAM_FLAGS {
    pub _bitfield: u16,
}
impl ::core::marker::Copy for NDR64_PARAM_FLAGS {}
impl ::core::clone::Clone for NDR64_PARAM_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDR64_PARAM_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDR64_PARAM_FLAGS").field("_bitfield", &self._bitfield).finish()
    }
}
impl ::windows::core::TypeKind for NDR64_PARAM_FLAGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for NDR64_PARAM_FLAGS {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
impl ::core::cmp::Eq for NDR64_PARAM_FLAGS {}
impl ::core::default::Default for NDR64_PARAM_FLAGS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct NDR64_PARAM_FORMAT {
    pub Type: *mut ::core::ffi::c_void,
    pub Attributes: NDR64_PARAM_FLAGS,
    pub Reserved: u16,
    pub StackOffset: u32,
}
impl ::core::marker::Copy for NDR64_PARAM_FORMAT {}
impl ::core::clone::Clone for NDR64_PARAM_FORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDR64_PARAM_FORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDR64_PARAM_FORMAT").field("Type", &self.Type).field("Attributes", &self.Attributes).field("Reserved", &self.Reserved).field("StackOffset", &self.StackOffset).finish()
    }
}
impl ::windows::core::TypeKind for NDR64_PARAM_FORMAT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for NDR64_PARAM_FORMAT {
    fn eq(&self, other: &Self) -> bool {
        self.Type == other.Type && self.Attributes == other.Attributes && self.Reserved == other.Reserved && self.StackOffset == other.StackOffset
    }
}
impl ::core::cmp::Eq for NDR64_PARAM_FORMAT {}
impl ::core::default::Default for NDR64_PARAM_FORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct NDR64_PIPE_FLAGS {
    pub _bitfield: u8,
}
impl ::core::marker::Copy for NDR64_PIPE_FLAGS {}
impl ::core::clone::Clone for NDR64_PIPE_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDR64_PIPE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDR64_PIPE_FLAGS").field("_bitfield", &self._bitfield).finish()
    }
}
impl ::windows::core::TypeKind for NDR64_PIPE_FLAGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for NDR64_PIPE_FLAGS {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
impl ::core::cmp::Eq for NDR64_PIPE_FLAGS {}
impl ::core::default::Default for NDR64_PIPE_FLAGS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct NDR64_PIPE_FORMAT {
    pub FormatCode: u8,
    pub Flags: u8,
    pub Alignment: u8,
    pub Reserved: u8,
    pub Type: *mut ::core::ffi::c_void,
    pub MemorySize: u32,
    pub BufferSize: u32,
}
impl ::core::marker::Copy for NDR64_PIPE_FORMAT {}
impl ::core::clone::Clone for NDR64_PIPE_FORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDR64_PIPE_FORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDR64_PIPE_FORMAT").field("FormatCode", &self.FormatCode).field("Flags", &self.Flags).field("Alignment", &self.Alignment).field("Reserved", &self.Reserved).field("Type", &self.Type).field("MemorySize", &self.MemorySize).field("BufferSize", &self.BufferSize).finish()
    }
}
impl ::windows::core::TypeKind for NDR64_PIPE_FORMAT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for NDR64_PIPE_FORMAT {
    fn eq(&self, other: &Self) -> bool {
        self.FormatCode == other.FormatCode && self.Flags == other.Flags && self.Alignment == other.Alignment && self.Reserved == other.Reserved && self.Type == other.Type && self.MemorySize == other.MemorySize && self.BufferSize == other.BufferSize
    }
}
impl ::core::cmp::Eq for NDR64_PIPE_FORMAT {}
impl ::core::default::Default for NDR64_PIPE_FORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct NDR64_POINTER_FORMAT {
    pub FormatCode: u8,
    pub Flags: u8,
    pub Reserved: u16,
    pub Pointee: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for NDR64_POINTER_FORMAT {}
impl ::core::clone::Clone for NDR64_POINTER_FORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDR64_POINTER_FORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDR64_POINTER_FORMAT").field("FormatCode", &self.FormatCode).field("Flags", &self.Flags).field("Reserved", &self.Reserved).field("Pointee", &self.Pointee).finish()
    }
}
impl ::windows::core::TypeKind for NDR64_POINTER_FORMAT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for NDR64_POINTER_FORMAT {
    fn eq(&self, other: &Self) -> bool {
        self.FormatCode == other.FormatCode && self.Flags == other.Flags && self.Reserved == other.Reserved && self.Pointee == other.Pointee
    }
}
impl ::core::cmp::Eq for NDR64_POINTER_FORMAT {}
impl ::core::default::Default for NDR64_POINTER_FORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct NDR64_POINTER_INSTANCE_HEADER_FORMAT {
    pub Offset: u32,
    pub Reserved: u32,
}
impl ::core::marker::Copy for NDR64_POINTER_INSTANCE_HEADER_FORMAT {}
impl ::core::clone::Clone for NDR64_POINTER_INSTANCE_HEADER_FORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDR64_POINTER_INSTANCE_HEADER_FORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDR64_POINTER_INSTANCE_HEADER_FORMAT").field("Offset", &self.Offset).field("Reserved", &self.Reserved).finish()
    }
}
impl ::windows::core::TypeKind for NDR64_POINTER_INSTANCE_HEADER_FORMAT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for NDR64_POINTER_INSTANCE_HEADER_FORMAT {
    fn eq(&self, other: &Self) -> bool {
        self.Offset == other.Offset && self.Reserved == other.Reserved
    }
}
impl ::core::cmp::Eq for NDR64_POINTER_INSTANCE_HEADER_FORMAT {}
impl ::core::default::Default for NDR64_POINTER_INSTANCE_HEADER_FORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct NDR64_POINTER_REPEAT_FLAGS {
    pub _bitfield: u8,
}
impl ::core::marker::Copy for NDR64_POINTER_REPEAT_FLAGS {}
impl ::core::clone::Clone for NDR64_POINTER_REPEAT_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDR64_POINTER_REPEAT_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDR64_POINTER_REPEAT_FLAGS").field("_bitfield", &self._bitfield).finish()
    }
}
impl ::windows::core::TypeKind for NDR64_POINTER_REPEAT_FLAGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for NDR64_POINTER_REPEAT_FLAGS {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
impl ::core::cmp::Eq for NDR64_POINTER_REPEAT_FLAGS {}
impl ::core::default::Default for NDR64_POINTER_REPEAT_FLAGS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct NDR64_PROC_FLAGS {
    pub _bitfield: u32,
}
impl ::core::marker::Copy for NDR64_PROC_FLAGS {}
impl ::core::clone::Clone for NDR64_PROC_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDR64_PROC_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDR64_PROC_FLAGS").field("_bitfield", &self._bitfield).finish()
    }
}
impl ::windows::core::TypeKind for NDR64_PROC_FLAGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for NDR64_PROC_FLAGS {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
impl ::core::cmp::Eq for NDR64_PROC_FLAGS {}
impl ::core::default::Default for NDR64_PROC_FLAGS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct NDR64_PROC_FORMAT {
    pub Flags: u32,
    pub StackSize: u32,
    pub ConstantClientBufferSize: u32,
    pub ConstantServerBufferSize: u32,
    pub RpcFlags: u16,
    pub FloatDoubleMask: u16,
    pub NumberOfParams: u16,
    pub ExtensionSize: u16,
}
impl ::core::marker::Copy for NDR64_PROC_FORMAT {}
impl ::core::clone::Clone for NDR64_PROC_FORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDR64_PROC_FORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDR64_PROC_FORMAT").field("Flags", &self.Flags).field("StackSize", &self.StackSize).field("ConstantClientBufferSize", &self.ConstantClientBufferSize).field("ConstantServerBufferSize", &self.ConstantServerBufferSize).field("RpcFlags", &self.RpcFlags).field("FloatDoubleMask", &self.FloatDoubleMask).field("NumberOfParams", &self.NumberOfParams).field("ExtensionSize", &self.ExtensionSize).finish()
    }
}
impl ::windows::core::TypeKind for NDR64_PROC_FORMAT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for NDR64_PROC_FORMAT {
    fn eq(&self, other: &Self) -> bool {
        self.Flags == other.Flags && self.StackSize == other.StackSize && self.ConstantClientBufferSize == other.ConstantClientBufferSize && self.ConstantServerBufferSize == other.ConstantServerBufferSize && self.RpcFlags == other.RpcFlags && self.FloatDoubleMask == other.FloatDoubleMask && self.NumberOfParams == other.NumberOfParams && self.ExtensionSize == other.ExtensionSize
    }
}
impl ::core::cmp::Eq for NDR64_PROC_FORMAT {}
impl ::core::default::Default for NDR64_PROC_FORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct NDR64_RANGED_STRING_FORMAT {
    pub Header: NDR64_STRING_HEADER_FORMAT,
    pub Reserved: u32,
    pub Min: u64,
    pub Max: u64,
}
impl ::core::marker::Copy for NDR64_RANGED_STRING_FORMAT {}
impl ::core::clone::Clone for NDR64_RANGED_STRING_FORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDR64_RANGED_STRING_FORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDR64_RANGED_STRING_FORMAT").field("Header", &self.Header).field("Reserved", &self.Reserved).field("Min", &self.Min).field("Max", &self.Max).finish()
    }
}
impl ::windows::core::TypeKind for NDR64_RANGED_STRING_FORMAT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for NDR64_RANGED_STRING_FORMAT {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.Reserved == other.Reserved && self.Min == other.Min && self.Max == other.Max
    }
}
impl ::core::cmp::Eq for NDR64_RANGED_STRING_FORMAT {}
impl ::core::default::Default for NDR64_RANGED_STRING_FORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct NDR64_RANGE_FORMAT {
    pub FormatCode: u8,
    pub RangeType: u8,
    pub Reserved: u16,
    pub MinValue: i64,
    pub MaxValue: i64,
}
impl ::core::marker::Copy for NDR64_RANGE_FORMAT {}
impl ::core::clone::Clone for NDR64_RANGE_FORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDR64_RANGE_FORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDR64_RANGE_FORMAT").field("FormatCode", &self.FormatCode).field("RangeType", &self.RangeType).field("Reserved", &self.Reserved).field("MinValue", &self.MinValue).field("MaxValue", &self.MaxValue).finish()
    }
}
impl ::windows::core::TypeKind for NDR64_RANGE_FORMAT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for NDR64_RANGE_FORMAT {
    fn eq(&self, other: &Self) -> bool {
        self.FormatCode == other.FormatCode && self.RangeType == other.RangeType && self.Reserved == other.Reserved && self.MinValue == other.MinValue && self.MaxValue == other.MaxValue
    }
}
impl ::core::cmp::Eq for NDR64_RANGE_FORMAT {}
impl ::core::default::Default for NDR64_RANGE_FORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct NDR64_RANGE_PIPE_FORMAT {
    pub FormatCode: u8,
    pub Flags: u8,
    pub Alignment: u8,
    pub Reserved: u8,
    pub Type: *mut ::core::ffi::c_void,
    pub MemorySize: u32,
    pub BufferSize: u32,
    pub MinValue: u32,
    pub MaxValue: u32,
}
impl ::core::marker::Copy for NDR64_RANGE_PIPE_FORMAT {}
impl ::core::clone::Clone for NDR64_RANGE_PIPE_FORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDR64_RANGE_PIPE_FORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDR64_RANGE_PIPE_FORMAT").field("FormatCode", &self.FormatCode).field("Flags", &self.Flags).field("Alignment", &self.Alignment).field("Reserved", &self.Reserved).field("Type", &self.Type).field("MemorySize", &self.MemorySize).field("BufferSize", &self.BufferSize).field("MinValue", &self.MinValue).field("MaxValue", &self.MaxValue).finish()
    }
}
impl ::windows::core::TypeKind for NDR64_RANGE_PIPE_FORMAT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for NDR64_RANGE_PIPE_FORMAT {
    fn eq(&self, other: &Self) -> bool {
        self.FormatCode == other.FormatCode && self.Flags == other.Flags && self.Alignment == other.Alignment && self.Reserved == other.Reserved && self.Type == other.Type && self.MemorySize == other.MemorySize && self.BufferSize == other.BufferSize && self.MinValue == other.MinValue && self.MaxValue == other.MaxValue
    }
}
impl ::core::cmp::Eq for NDR64_RANGE_PIPE_FORMAT {}
impl ::core::default::Default for NDR64_RANGE_PIPE_FORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct NDR64_REPEAT_FORMAT {
    pub FormatCode: u8,
    pub Flags: NDR64_POINTER_REPEAT_FLAGS,
    pub Reserved: u16,
    pub Increment: u32,
    pub OffsetToArray: u32,
    pub NumberOfPointers: u32,
}
impl ::core::marker::Copy for NDR64_REPEAT_FORMAT {}
impl ::core::clone::Clone for NDR64_REPEAT_FORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDR64_REPEAT_FORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDR64_REPEAT_FORMAT").field("FormatCode", &self.FormatCode).field("Flags", &self.Flags).field("Reserved", &self.Reserved).field("Increment", &self.Increment).field("OffsetToArray", &self.OffsetToArray).field("NumberOfPointers", &self.NumberOfPointers).finish()
    }
}
impl ::windows::core::TypeKind for NDR64_REPEAT_FORMAT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for NDR64_REPEAT_FORMAT {
    fn eq(&self, other: &Self) -> bool {
        self.FormatCode == other.FormatCode && self.Flags == other.Flags && self.Reserved == other.Reserved && self.Increment == other.Increment && self.OffsetToArray == other.OffsetToArray && self.NumberOfPointers == other.NumberOfPointers
    }
}
impl ::core::cmp::Eq for NDR64_REPEAT_FORMAT {}
impl ::core::default::Default for NDR64_REPEAT_FORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct NDR64_RPC_FLAGS {
    pub _bitfield: u16,
}
impl ::core::marker::Copy for NDR64_RPC_FLAGS {}
impl ::core::clone::Clone for NDR64_RPC_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDR64_RPC_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDR64_RPC_FLAGS").field("_bitfield", &self._bitfield).finish()
    }
}
impl ::windows::core::TypeKind for NDR64_RPC_FLAGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for NDR64_RPC_FLAGS {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
impl ::core::cmp::Eq for NDR64_RPC_FLAGS {}
impl ::core::default::Default for NDR64_RPC_FLAGS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct NDR64_SIMPLE_MEMBER_FORMAT {
    pub FormatCode: u8,
    pub Reserved1: u8,
    pub Reserved2: u16,
    pub Reserved3: u32,
}
impl ::core::marker::Copy for NDR64_SIMPLE_MEMBER_FORMAT {}
impl ::core::clone::Clone for NDR64_SIMPLE_MEMBER_FORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDR64_SIMPLE_MEMBER_FORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDR64_SIMPLE_MEMBER_FORMAT").field("FormatCode", &self.FormatCode).field("Reserved1", &self.Reserved1).field("Reserved2", &self.Reserved2).field("Reserved3", &self.Reserved3).finish()
    }
}
impl ::windows::core::TypeKind for NDR64_SIMPLE_MEMBER_FORMAT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for NDR64_SIMPLE_MEMBER_FORMAT {
    fn eq(&self, other: &Self) -> bool {
        self.FormatCode == other.FormatCode && self.Reserved1 == other.Reserved1 && self.Reserved2 == other.Reserved2 && self.Reserved3 == other.Reserved3
    }
}
impl ::core::cmp::Eq for NDR64_SIMPLE_MEMBER_FORMAT {}
impl ::core::default::Default for NDR64_SIMPLE_MEMBER_FORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct NDR64_SIMPLE_REGION_FORMAT {
    pub FormatCode: u8,
    pub Alignment: u8,
    pub RegionSize: u16,
    pub Reserved: u32,
}
impl ::core::marker::Copy for NDR64_SIMPLE_REGION_FORMAT {}
impl ::core::clone::Clone for NDR64_SIMPLE_REGION_FORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDR64_SIMPLE_REGION_FORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDR64_SIMPLE_REGION_FORMAT").field("FormatCode", &self.FormatCode).field("Alignment", &self.Alignment).field("RegionSize", &self.RegionSize).field("Reserved", &self.Reserved).finish()
    }
}
impl ::windows::core::TypeKind for NDR64_SIMPLE_REGION_FORMAT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for NDR64_SIMPLE_REGION_FORMAT {
    fn eq(&self, other: &Self) -> bool {
        self.FormatCode == other.FormatCode && self.Alignment == other.Alignment && self.RegionSize == other.RegionSize && self.Reserved == other.Reserved
    }
}
impl ::core::cmp::Eq for NDR64_SIMPLE_REGION_FORMAT {}
impl ::core::default::Default for NDR64_SIMPLE_REGION_FORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct NDR64_SIZED_CONFORMANT_STRING_FORMAT {
    pub Header: NDR64_STRING_HEADER_FORMAT,
    pub SizeDescription: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for NDR64_SIZED_CONFORMANT_STRING_FORMAT {}
impl ::core::clone::Clone for NDR64_SIZED_CONFORMANT_STRING_FORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDR64_SIZED_CONFORMANT_STRING_FORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDR64_SIZED_CONFORMANT_STRING_FORMAT").field("Header", &self.Header).field("SizeDescription", &self.SizeDescription).finish()
    }
}
impl ::windows::core::TypeKind for NDR64_SIZED_CONFORMANT_STRING_FORMAT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for NDR64_SIZED_CONFORMANT_STRING_FORMAT {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.SizeDescription == other.SizeDescription
    }
}
impl ::core::cmp::Eq for NDR64_SIZED_CONFORMANT_STRING_FORMAT {}
impl ::core::default::Default for NDR64_SIZED_CONFORMANT_STRING_FORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct NDR64_STRING_FLAGS {
    pub _bitfield: u8,
}
impl ::core::marker::Copy for NDR64_STRING_FLAGS {}
impl ::core::clone::Clone for NDR64_STRING_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDR64_STRING_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDR64_STRING_FLAGS").field("_bitfield", &self._bitfield).finish()
    }
}
impl ::windows::core::TypeKind for NDR64_STRING_FLAGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for NDR64_STRING_FLAGS {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
impl ::core::cmp::Eq for NDR64_STRING_FLAGS {}
impl ::core::default::Default for NDR64_STRING_FLAGS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct NDR64_STRING_HEADER_FORMAT {
    pub FormatCode: u8,
    pub Flags: NDR64_STRING_FLAGS,
    pub ElementSize: u16,
}
impl ::core::marker::Copy for NDR64_STRING_HEADER_FORMAT {}
impl ::core::clone::Clone for NDR64_STRING_HEADER_FORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDR64_STRING_HEADER_FORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDR64_STRING_HEADER_FORMAT").field("FormatCode", &self.FormatCode).field("Flags", &self.Flags).field("ElementSize", &self.ElementSize).finish()
    }
}
impl ::windows::core::TypeKind for NDR64_STRING_HEADER_FORMAT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for NDR64_STRING_HEADER_FORMAT {
    fn eq(&self, other: &Self) -> bool {
        self.FormatCode == other.FormatCode && self.Flags == other.Flags && self.ElementSize == other.ElementSize
    }
}
impl ::core::cmp::Eq for NDR64_STRING_HEADER_FORMAT {}
impl ::core::default::Default for NDR64_STRING_HEADER_FORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct NDR64_STRUCTURE_FLAGS {
    pub _bitfield: u8,
}
impl ::core::marker::Copy for NDR64_STRUCTURE_FLAGS {}
impl ::core::clone::Clone for NDR64_STRUCTURE_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDR64_STRUCTURE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDR64_STRUCTURE_FLAGS").field("_bitfield", &self._bitfield).finish()
    }
}
impl ::windows::core::TypeKind for NDR64_STRUCTURE_FLAGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for NDR64_STRUCTURE_FLAGS {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
impl ::core::cmp::Eq for NDR64_STRUCTURE_FLAGS {}
impl ::core::default::Default for NDR64_STRUCTURE_FLAGS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct NDR64_STRUCTURE_HEADER_FORMAT {
    pub FormatCode: u8,
    pub Alignment: u8,
    pub Flags: NDR64_STRUCTURE_FLAGS,
    pub Reserve: u8,
    pub MemorySize: u32,
}
impl ::core::marker::Copy for NDR64_STRUCTURE_HEADER_FORMAT {}
impl ::core::clone::Clone for NDR64_STRUCTURE_HEADER_FORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDR64_STRUCTURE_HEADER_FORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDR64_STRUCTURE_HEADER_FORMAT").field("FormatCode", &self.FormatCode).field("Alignment", &self.Alignment).field("Flags", &self.Flags).field("Reserve", &self.Reserve).field("MemorySize", &self.MemorySize).finish()
    }
}
impl ::windows::core::TypeKind for NDR64_STRUCTURE_HEADER_FORMAT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for NDR64_STRUCTURE_HEADER_FORMAT {
    fn eq(&self, other: &Self) -> bool {
        self.FormatCode == other.FormatCode && self.Alignment == other.Alignment && self.Flags == other.Flags && self.Reserve == other.Reserve && self.MemorySize == other.MemorySize
    }
}
impl ::core::cmp::Eq for NDR64_STRUCTURE_HEADER_FORMAT {}
impl ::core::default::Default for NDR64_STRUCTURE_HEADER_FORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct NDR64_SYSTEM_HANDLE_FORMAT {
    pub FormatCode: u8,
    pub HandleType: u8,
    pub DesiredAccess: u32,
}
impl ::core::marker::Copy for NDR64_SYSTEM_HANDLE_FORMAT {}
impl ::core::clone::Clone for NDR64_SYSTEM_HANDLE_FORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDR64_SYSTEM_HANDLE_FORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDR64_SYSTEM_HANDLE_FORMAT").field("FormatCode", &self.FormatCode).field("HandleType", &self.HandleType).field("DesiredAccess", &self.DesiredAccess).finish()
    }
}
impl ::windows::core::TypeKind for NDR64_SYSTEM_HANDLE_FORMAT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for NDR64_SYSTEM_HANDLE_FORMAT {
    fn eq(&self, other: &Self) -> bool {
        self.FormatCode == other.FormatCode && self.HandleType == other.HandleType && self.DesiredAccess == other.DesiredAccess
    }
}
impl ::core::cmp::Eq for NDR64_SYSTEM_HANDLE_FORMAT {}
impl ::core::default::Default for NDR64_SYSTEM_HANDLE_FORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct NDR64_TRANSMIT_AS_FLAGS {
    pub _bitfield: u8,
}
impl ::core::marker::Copy for NDR64_TRANSMIT_AS_FLAGS {}
impl ::core::clone::Clone for NDR64_TRANSMIT_AS_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDR64_TRANSMIT_AS_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDR64_TRANSMIT_AS_FLAGS").field("_bitfield", &self._bitfield).finish()
    }
}
impl ::windows::core::TypeKind for NDR64_TRANSMIT_AS_FLAGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for NDR64_TRANSMIT_AS_FLAGS {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
impl ::core::cmp::Eq for NDR64_TRANSMIT_AS_FLAGS {}
impl ::core::default::Default for NDR64_TRANSMIT_AS_FLAGS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct NDR64_TRANSMIT_AS_FORMAT {
    pub FormatCode: u8,
    pub Flags: u8,
    pub RoutineIndex: u16,
    pub TransmittedTypeWireAlignment: u16,
    pub MemoryAlignment: u16,
    pub PresentedTypeMemorySize: u32,
    pub TransmittedTypeBufferSize: u32,
    pub TransmittedType: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for NDR64_TRANSMIT_AS_FORMAT {}
impl ::core::clone::Clone for NDR64_TRANSMIT_AS_FORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDR64_TRANSMIT_AS_FORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDR64_TRANSMIT_AS_FORMAT").field("FormatCode", &self.FormatCode).field("Flags", &self.Flags).field("RoutineIndex", &self.RoutineIndex).field("TransmittedTypeWireAlignment", &self.TransmittedTypeWireAlignment).field("MemoryAlignment", &self.MemoryAlignment).field("PresentedTypeMemorySize", &self.PresentedTypeMemorySize).field("TransmittedTypeBufferSize", &self.TransmittedTypeBufferSize).field("TransmittedType", &self.TransmittedType).finish()
    }
}
impl ::windows::core::TypeKind for NDR64_TRANSMIT_AS_FORMAT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for NDR64_TRANSMIT_AS_FORMAT {
    fn eq(&self, other: &Self) -> bool {
        self.FormatCode == other.FormatCode && self.Flags == other.Flags && self.RoutineIndex == other.RoutineIndex && self.TransmittedTypeWireAlignment == other.TransmittedTypeWireAlignment && self.MemoryAlignment == other.MemoryAlignment && self.PresentedTypeMemorySize == other.PresentedTypeMemorySize && self.TransmittedTypeBufferSize == other.TransmittedTypeBufferSize && self.TransmittedType == other.TransmittedType
    }
}
impl ::core::cmp::Eq for NDR64_TRANSMIT_AS_FORMAT {}
impl ::core::default::Default for NDR64_TRANSMIT_AS_FORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct NDR64_TYPE_STRICT_CONTEXT_HANDLE {
    pub FormatCode: u8,
    pub RealFormatCode: u8,
    pub Reserved: u16,
    pub Type: *mut ::core::ffi::c_void,
    pub CtxtFlags: u32,
    pub CtxtID: u32,
}
impl ::core::marker::Copy for NDR64_TYPE_STRICT_CONTEXT_HANDLE {}
impl ::core::clone::Clone for NDR64_TYPE_STRICT_CONTEXT_HANDLE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDR64_TYPE_STRICT_CONTEXT_HANDLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDR64_TYPE_STRICT_CONTEXT_HANDLE").field("FormatCode", &self.FormatCode).field("RealFormatCode", &self.RealFormatCode).field("Reserved", &self.Reserved).field("Type", &self.Type).field("CtxtFlags", &self.CtxtFlags).field("CtxtID", &self.CtxtID).finish()
    }
}
impl ::windows::core::TypeKind for NDR64_TYPE_STRICT_CONTEXT_HANDLE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for NDR64_TYPE_STRICT_CONTEXT_HANDLE {
    fn eq(&self, other: &Self) -> bool {
        self.FormatCode == other.FormatCode && self.RealFormatCode == other.RealFormatCode && self.Reserved == other.Reserved && self.Type == other.Type && self.CtxtFlags == other.CtxtFlags && self.CtxtID == other.CtxtID
    }
}
impl ::core::cmp::Eq for NDR64_TYPE_STRICT_CONTEXT_HANDLE {}
impl ::core::default::Default for NDR64_TYPE_STRICT_CONTEXT_HANDLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct NDR64_UNION_ARM {
    pub CaseValue: i64,
    pub Type: *mut ::core::ffi::c_void,
    pub Reserved: u32,
}
impl ::core::marker::Copy for NDR64_UNION_ARM {}
impl ::core::clone::Clone for NDR64_UNION_ARM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDR64_UNION_ARM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDR64_UNION_ARM").field("CaseValue", &self.CaseValue).field("Type", &self.Type).field("Reserved", &self.Reserved).finish()
    }
}
impl ::windows::core::TypeKind for NDR64_UNION_ARM {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for NDR64_UNION_ARM {
    fn eq(&self, other: &Self) -> bool {
        self.CaseValue == other.CaseValue && self.Type == other.Type && self.Reserved == other.Reserved
    }
}
impl ::core::cmp::Eq for NDR64_UNION_ARM {}
impl ::core::default::Default for NDR64_UNION_ARM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct NDR64_UNION_ARM_SELECTOR {
    pub Reserved1: u8,
    pub Alignment: u8,
    pub Reserved2: u16,
    pub Arms: u32,
}
impl ::core::marker::Copy for NDR64_UNION_ARM_SELECTOR {}
impl ::core::clone::Clone for NDR64_UNION_ARM_SELECTOR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDR64_UNION_ARM_SELECTOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDR64_UNION_ARM_SELECTOR").field("Reserved1", &self.Reserved1).field("Alignment", &self.Alignment).field("Reserved2", &self.Reserved2).field("Arms", &self.Arms).finish()
    }
}
impl ::windows::core::TypeKind for NDR64_UNION_ARM_SELECTOR {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for NDR64_UNION_ARM_SELECTOR {
    fn eq(&self, other: &Self) -> bool {
        self.Reserved1 == other.Reserved1 && self.Alignment == other.Alignment && self.Reserved2 == other.Reserved2 && self.Arms == other.Arms
    }
}
impl ::core::cmp::Eq for NDR64_UNION_ARM_SELECTOR {}
impl ::core::default::Default for NDR64_UNION_ARM_SELECTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct NDR64_USER_MARSHAL_FLAGS {
    pub _bitfield: u8,
}
impl ::core::marker::Copy for NDR64_USER_MARSHAL_FLAGS {}
impl ::core::clone::Clone for NDR64_USER_MARSHAL_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDR64_USER_MARSHAL_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDR64_USER_MARSHAL_FLAGS").field("_bitfield", &self._bitfield).finish()
    }
}
impl ::windows::core::TypeKind for NDR64_USER_MARSHAL_FLAGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for NDR64_USER_MARSHAL_FLAGS {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
impl ::core::cmp::Eq for NDR64_USER_MARSHAL_FLAGS {}
impl ::core::default::Default for NDR64_USER_MARSHAL_FLAGS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct NDR64_USER_MARSHAL_FORMAT {
    pub FormatCode: u8,
    pub Flags: u8,
    pub RoutineIndex: u16,
    pub TransmittedTypeWireAlignment: u16,
    pub MemoryAlignment: u16,
    pub UserTypeMemorySize: u32,
    pub TransmittedTypeBufferSize: u32,
    pub TransmittedType: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for NDR64_USER_MARSHAL_FORMAT {}
impl ::core::clone::Clone for NDR64_USER_MARSHAL_FORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDR64_USER_MARSHAL_FORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDR64_USER_MARSHAL_FORMAT").field("FormatCode", &self.FormatCode).field("Flags", &self.Flags).field("RoutineIndex", &self.RoutineIndex).field("TransmittedTypeWireAlignment", &self.TransmittedTypeWireAlignment).field("MemoryAlignment", &self.MemoryAlignment).field("UserTypeMemorySize", &self.UserTypeMemorySize).field("TransmittedTypeBufferSize", &self.TransmittedTypeBufferSize).field("TransmittedType", &self.TransmittedType).finish()
    }
}
impl ::windows::core::TypeKind for NDR64_USER_MARSHAL_FORMAT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for NDR64_USER_MARSHAL_FORMAT {
    fn eq(&self, other: &Self) -> bool {
        self.FormatCode == other.FormatCode && self.Flags == other.Flags && self.RoutineIndex == other.RoutineIndex && self.TransmittedTypeWireAlignment == other.TransmittedTypeWireAlignment && self.MemoryAlignment == other.MemoryAlignment && self.UserTypeMemorySize == other.UserTypeMemorySize && self.TransmittedTypeBufferSize == other.TransmittedTypeBufferSize && self.TransmittedType == other.TransmittedType
    }
}
impl ::core::cmp::Eq for NDR64_USER_MARSHAL_FORMAT {}
impl ::core::default::Default for NDR64_USER_MARSHAL_FORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct NDR64_VAR_ARRAY_HEADER_FORMAT {
    pub FormatCode: u8,
    pub Alignment: u8,
    pub Flags: NDR64_ARRAY_FLAGS,
    pub Reserved: u8,
    pub TotalSize: u32,
    pub ElementSize: u32,
    pub VarDescriptor: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for NDR64_VAR_ARRAY_HEADER_FORMAT {}
impl ::core::clone::Clone for NDR64_VAR_ARRAY_HEADER_FORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDR64_VAR_ARRAY_HEADER_FORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDR64_VAR_ARRAY_HEADER_FORMAT").field("FormatCode", &self.FormatCode).field("Alignment", &self.Alignment).field("Flags", &self.Flags).field("Reserved", &self.Reserved).field("TotalSize", &self.TotalSize).field("ElementSize", &self.ElementSize).field("VarDescriptor", &self.VarDescriptor).finish()
    }
}
impl ::windows::core::TypeKind for NDR64_VAR_ARRAY_HEADER_FORMAT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for NDR64_VAR_ARRAY_HEADER_FORMAT {
    fn eq(&self, other: &Self) -> bool {
        self.FormatCode == other.FormatCode && self.Alignment == other.Alignment && self.Flags == other.Flags && self.Reserved == other.Reserved && self.TotalSize == other.TotalSize && self.ElementSize == other.ElementSize && self.VarDescriptor == other.VarDescriptor
    }
}
impl ::core::cmp::Eq for NDR64_VAR_ARRAY_HEADER_FORMAT {}
impl ::core::default::Default for NDR64_VAR_ARRAY_HEADER_FORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct NDR_ALLOC_ALL_NODES_CONTEXT(pub u8);
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct NDR_CS_ROUTINES {
    pub pSizeConvertRoutines: *mut NDR_CS_SIZE_CONVERT_ROUTINES,
    pub pTagGettingRoutines: *mut CS_TAG_GETTING_ROUTINE,
}
impl ::core::marker::Copy for NDR_CS_ROUTINES {}
impl ::core::clone::Clone for NDR_CS_ROUTINES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDR_CS_ROUTINES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDR_CS_ROUTINES").field("pSizeConvertRoutines", &self.pSizeConvertRoutines).field("pTagGettingRoutines", &self.pTagGettingRoutines).finish()
    }
}
impl ::windows::core::TypeKind for NDR_CS_ROUTINES {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for NDR_CS_ROUTINES {
    fn eq(&self, other: &Self) -> bool {
        self.pSizeConvertRoutines == other.pSizeConvertRoutines && self.pTagGettingRoutines == other.pTagGettingRoutines
    }
}
impl ::core::cmp::Eq for NDR_CS_ROUTINES {}
impl ::core::default::Default for NDR_CS_ROUTINES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct NDR_CS_SIZE_CONVERT_ROUTINES {
    pub pfnNetSize: CS_TYPE_NET_SIZE_ROUTINE,
    pub pfnToNetCs: CS_TYPE_TO_NETCS_ROUTINE,
    pub pfnLocalSize: CS_TYPE_LOCAL_SIZE_ROUTINE,
    pub pfnFromNetCs: CS_TYPE_FROM_NETCS_ROUTINE,
}
impl ::core::marker::Copy for NDR_CS_SIZE_CONVERT_ROUTINES {}
impl ::core::clone::Clone for NDR_CS_SIZE_CONVERT_ROUTINES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDR_CS_SIZE_CONVERT_ROUTINES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDR_CS_SIZE_CONVERT_ROUTINES").finish()
    }
}
impl ::windows::core::TypeKind for NDR_CS_SIZE_CONVERT_ROUTINES {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for NDR_CS_SIZE_CONVERT_ROUTINES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct NDR_EXPR_DESC {
    pub pOffset: *const u16,
    pub pFormatExpr: *mut u8,
}
impl ::core::marker::Copy for NDR_EXPR_DESC {}
impl ::core::clone::Clone for NDR_EXPR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDR_EXPR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDR_EXPR_DESC").field("pOffset", &self.pOffset).field("pFormatExpr", &self.pFormatExpr).finish()
    }
}
impl ::windows::core::TypeKind for NDR_EXPR_DESC {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for NDR_EXPR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.pOffset == other.pOffset && self.pFormatExpr == other.pFormatExpr
    }
}
impl ::core::cmp::Eq for NDR_EXPR_DESC {}
impl ::core::default::Default for NDR_EXPR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct NDR_POINTER_QUEUE_STATE(pub u8);
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct NDR_SCONTEXT {
    pub pad: [*mut ::core::ffi::c_void; 2],
    pub userContext: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for NDR_SCONTEXT {}
impl ::core::clone::Clone for NDR_SCONTEXT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDR_SCONTEXT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDR_SCONTEXT").field("pad", &self.pad).field("userContext", &self.userContext).finish()
    }
}
impl ::windows::core::TypeKind for NDR_SCONTEXT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for NDR_SCONTEXT {
    fn eq(&self, other: &Self) -> bool {
        self.pad == other.pad && self.userContext == other.userContext
    }
}
impl ::core::cmp::Eq for NDR_SCONTEXT {}
impl ::core::default::Default for NDR_SCONTEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub struct NDR_USER_MARSHAL_INFO {
    pub InformationLevel: u32,
    pub Anonymous: NDR_USER_MARSHAL_INFO_0,
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for NDR_USER_MARSHAL_INFO {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::TypeKind for NDR_USER_MARSHAL_INFO {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::default::Default for NDR_USER_MARSHAL_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub union NDR_USER_MARSHAL_INFO_0 {
    pub Level1: ::std::mem::ManuallyDrop<NDR_USER_MARSHAL_INFO_LEVEL1>,
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for NDR_USER_MARSHAL_INFO_0 {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::TypeKind for NDR_USER_MARSHAL_INFO_0 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::default::Default for NDR_USER_MARSHAL_INFO_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub struct NDR_USER_MARSHAL_INFO_LEVEL1 {
    pub Buffer: *mut ::core::ffi::c_void,
    pub BufferSize: u32,
    pub pfnAllocate: isize,
    pub pfnFree: isize,
    pub pRpcChannelBuffer: ::std::mem::ManuallyDrop<::core::option::Option<super::Com::IRpcChannelBuffer>>,
    pub Reserved: [usize; 5],
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for NDR_USER_MARSHAL_INFO_LEVEL1 {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for NDR_USER_MARSHAL_INFO_LEVEL1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDR_USER_MARSHAL_INFO_LEVEL1").field("Buffer", &self.Buffer).field("BufferSize", &self.BufferSize).field("pfnAllocate", &self.pfnAllocate).field("pfnFree", &self.pfnFree).field("pRpcChannelBuffer", &self.pRpcChannelBuffer).field("Reserved", &self.Reserved).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::TypeKind for NDR_USER_MARSHAL_INFO_LEVEL1 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for NDR_USER_MARSHAL_INFO_LEVEL1 {
    fn eq(&self, other: &Self) -> bool {
        self.Buffer == other.Buffer && self.BufferSize == other.BufferSize && self.pfnAllocate == other.pfnAllocate && self.pfnFree == other.pfnFree && self.pRpcChannelBuffer == other.pRpcChannelBuffer && self.Reserved == other.Reserved
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for NDR_USER_MARSHAL_INFO_LEVEL1 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::default::Default for NDR_USER_MARSHAL_INFO_LEVEL1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct RDR_CALLOUT_STATE {
    pub LastError: RPC_STATUS,
    pub LastEEInfo: *mut ::core::ffi::c_void,
    pub LastCalledStage: RPC_HTTP_REDIRECTOR_STAGE,
    pub ServerName: *mut u16,
    pub ServerPort: *mut u16,
    pub RemoteUser: *mut u16,
    pub AuthType: *mut u16,
    pub ResourceTypePresent: u8,
    pub SessionIdPresent: u8,
    pub InterfacePresent: u8,
    pub ResourceType: ::windows::core::GUID,
    pub SessionId: ::windows::core::GUID,
    pub Interface: RPC_SYNTAX_IDENTIFIER,
    pub CertContext: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for RDR_CALLOUT_STATE {}
impl ::core::clone::Clone for RDR_CALLOUT_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RDR_CALLOUT_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RDR_CALLOUT_STATE")
            .field("LastError", &self.LastError)
            .field("LastEEInfo", &self.LastEEInfo)
            .field("LastCalledStage", &self.LastCalledStage)
            .field("ServerName", &self.ServerName)
            .field("ServerPort", &self.ServerPort)
            .field("RemoteUser", &self.RemoteUser)
            .field("AuthType", &self.AuthType)
            .field("ResourceTypePresent", &self.ResourceTypePresent)
            .field("SessionIdPresent", &self.SessionIdPresent)
            .field("InterfacePresent", &self.InterfacePresent)
            .field("ResourceType", &self.ResourceType)
            .field("SessionId", &self.SessionId)
            .field("Interface", &self.Interface)
            .field("CertContext", &self.CertContext)
            .finish()
    }
}
impl ::windows::core::TypeKind for RDR_CALLOUT_STATE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for RDR_CALLOUT_STATE {
    fn eq(&self, other: &Self) -> bool {
        self.LastError == other.LastError && self.LastEEInfo == other.LastEEInfo && self.LastCalledStage == other.LastCalledStage && self.ServerName == other.ServerName && self.ServerPort == other.ServerPort && self.RemoteUser == other.RemoteUser && self.AuthType == other.AuthType && self.ResourceTypePresent == other.ResourceTypePresent && self.SessionIdPresent == other.SessionIdPresent && self.InterfacePresent == other.InterfacePresent && self.ResourceType == other.ResourceType && self.SessionId == other.SessionId && self.Interface == other.Interface && self.CertContext == other.CertContext
    }
}
impl ::core::cmp::Eq for RDR_CALLOUT_STATE {}
impl ::core::default::Default for RDR_CALLOUT_STATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub union RPC_ASYNC_NOTIFICATION_INFO {
    pub APC: RPC_ASYNC_NOTIFICATION_INFO_0,
    pub IOC: RPC_ASYNC_NOTIFICATION_INFO_1,
    pub IntPtr: RPC_ASYNC_NOTIFICATION_INFO_2,
    pub hEvent: super::super::Foundation::HANDLE,
    pub NotificationRoutine: PFN_RPCNOTIFICATION_ROUTINE,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
impl ::core::marker::Copy for RPC_ASYNC_NOTIFICATION_INFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
impl ::core::clone::Clone for RPC_ASYNC_NOTIFICATION_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
impl ::windows::core::TypeKind for RPC_ASYNC_NOTIFICATION_INFO {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
impl ::core::default::Default for RPC_ASYNC_NOTIFICATION_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub struct RPC_ASYNC_NOTIFICATION_INFO_0 {
    pub NotificationRoutine: PFN_RPCNOTIFICATION_ROUTINE,
    pub hThread: super::super::Foundation::HANDLE,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
impl ::core::marker::Copy for RPC_ASYNC_NOTIFICATION_INFO_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
impl ::core::clone::Clone for RPC_ASYNC_NOTIFICATION_INFO_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
impl ::core::fmt::Debug for RPC_ASYNC_NOTIFICATION_INFO_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RPC_ASYNC_NOTIFICATION_INFO_0").field("hThread", &self.hThread).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
impl ::windows::core::TypeKind for RPC_ASYNC_NOTIFICATION_INFO_0 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
impl ::core::default::Default for RPC_ASYNC_NOTIFICATION_INFO_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub struct RPC_ASYNC_NOTIFICATION_INFO_1 {
    pub hIOPort: super::super::Foundation::HANDLE,
    pub dwNumberOfBytesTransferred: u32,
    pub dwCompletionKey: usize,
    pub lpOverlapped: *mut super::IO::OVERLAPPED,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
impl ::core::marker::Copy for RPC_ASYNC_NOTIFICATION_INFO_1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
impl ::core::clone::Clone for RPC_ASYNC_NOTIFICATION_INFO_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
impl ::core::fmt::Debug for RPC_ASYNC_NOTIFICATION_INFO_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RPC_ASYNC_NOTIFICATION_INFO_1").field("hIOPort", &self.hIOPort).field("dwNumberOfBytesTransferred", &self.dwNumberOfBytesTransferred).field("dwCompletionKey", &self.dwCompletionKey).field("lpOverlapped", &self.lpOverlapped).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
impl ::windows::core::TypeKind for RPC_ASYNC_NOTIFICATION_INFO_1 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
impl ::core::cmp::PartialEq for RPC_ASYNC_NOTIFICATION_INFO_1 {
    fn eq(&self, other: &Self) -> bool {
        self.hIOPort == other.hIOPort && self.dwNumberOfBytesTransferred == other.dwNumberOfBytesTransferred && self.dwCompletionKey == other.dwCompletionKey && self.lpOverlapped == other.lpOverlapped
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
impl ::core::cmp::Eq for RPC_ASYNC_NOTIFICATION_INFO_1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
impl ::core::default::Default for RPC_ASYNC_NOTIFICATION_INFO_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub struct RPC_ASYNC_NOTIFICATION_INFO_2 {
    pub hWnd: super::super::Foundation::HWND,
    pub Msg: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
impl ::core::marker::Copy for RPC_ASYNC_NOTIFICATION_INFO_2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
impl ::core::clone::Clone for RPC_ASYNC_NOTIFICATION_INFO_2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
impl ::core::fmt::Debug for RPC_ASYNC_NOTIFICATION_INFO_2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RPC_ASYNC_NOTIFICATION_INFO_2").field("hWnd", &self.hWnd).field("Msg", &self.Msg).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
impl ::windows::core::TypeKind for RPC_ASYNC_NOTIFICATION_INFO_2 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
impl ::core::cmp::PartialEq for RPC_ASYNC_NOTIFICATION_INFO_2 {
    fn eq(&self, other: &Self) -> bool {
        self.hWnd == other.hWnd && self.Msg == other.Msg
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
impl ::core::cmp::Eq for RPC_ASYNC_NOTIFICATION_INFO_2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
impl ::core::default::Default for RPC_ASYNC_NOTIFICATION_INFO_2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub struct RPC_ASYNC_STATE {
    pub Size: u32,
    pub Signature: u32,
    pub Lock: i32,
    pub Flags: u32,
    pub StubInfo: *mut ::core::ffi::c_void,
    pub UserInfo: *mut ::core::ffi::c_void,
    pub RuntimeInfo: *mut ::core::ffi::c_void,
    pub Event: RPC_ASYNC_EVENT,
    pub NotificationType: RPC_NOTIFICATION_TYPES,
    pub u: RPC_ASYNC_NOTIFICATION_INFO,
    pub Reserved: [isize; 4],
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
impl ::core::marker::Copy for RPC_ASYNC_STATE {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
impl ::core::clone::Clone for RPC_ASYNC_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
impl ::windows::core::TypeKind for RPC_ASYNC_STATE {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
impl ::core::default::Default for RPC_ASYNC_STATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct RPC_BINDING_HANDLE_OPTIONS_V1 {
    pub Version: u32,
    pub Flags: RPC_BINDING_HANDLE_OPTIONS_FLAGS,
    pub ComTimeout: u32,
    pub CallTimeout: u32,
}
impl ::core::marker::Copy for RPC_BINDING_HANDLE_OPTIONS_V1 {}
impl ::core::clone::Clone for RPC_BINDING_HANDLE_OPTIONS_V1 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RPC_BINDING_HANDLE_OPTIONS_V1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RPC_BINDING_HANDLE_OPTIONS_V1").field("Version", &self.Version).field("Flags", &self.Flags).field("ComTimeout", &self.ComTimeout).field("CallTimeout", &self.CallTimeout).finish()
    }
}
impl ::windows::core::TypeKind for RPC_BINDING_HANDLE_OPTIONS_V1 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for RPC_BINDING_HANDLE_OPTIONS_V1 {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.Flags == other.Flags && self.ComTimeout == other.ComTimeout && self.CallTimeout == other.CallTimeout
    }
}
impl ::core::cmp::Eq for RPC_BINDING_HANDLE_OPTIONS_V1 {}
impl ::core::default::Default for RPC_BINDING_HANDLE_OPTIONS_V1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub struct RPC_BINDING_HANDLE_SECURITY_V1_A {
    pub Version: u32,
    pub ServerPrincName: *mut u8,
    pub AuthnLevel: u32,
    pub AuthnSvc: u32,
    pub AuthIdentity: *mut SEC_WINNT_AUTH_IDENTITY_A,
    pub SecurityQos: *mut RPC_SECURITY_QOS,
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::marker::Copy for RPC_BINDING_HANDLE_SECURITY_V1_A {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for RPC_BINDING_HANDLE_SECURITY_V1_A {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for RPC_BINDING_HANDLE_SECURITY_V1_A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RPC_BINDING_HANDLE_SECURITY_V1_A").field("Version", &self.Version).field("ServerPrincName", &self.ServerPrincName).field("AuthnLevel", &self.AuthnLevel).field("AuthnSvc", &self.AuthnSvc).field("AuthIdentity", &self.AuthIdentity).field("SecurityQos", &self.SecurityQos).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::TypeKind for RPC_BINDING_HANDLE_SECURITY_V1_A {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for RPC_BINDING_HANDLE_SECURITY_V1_A {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.ServerPrincName == other.ServerPrincName && self.AuthnLevel == other.AuthnLevel && self.AuthnSvc == other.AuthnSvc && self.AuthIdentity == other.AuthIdentity && self.SecurityQos == other.SecurityQos
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for RPC_BINDING_HANDLE_SECURITY_V1_A {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::default::Default for RPC_BINDING_HANDLE_SECURITY_V1_A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub struct RPC_BINDING_HANDLE_SECURITY_V1_W {
    pub Version: u32,
    pub ServerPrincName: *mut u16,
    pub AuthnLevel: u32,
    pub AuthnSvc: u32,
    pub AuthIdentity: *mut SEC_WINNT_AUTH_IDENTITY_W,
    pub SecurityQos: *mut RPC_SECURITY_QOS,
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::marker::Copy for RPC_BINDING_HANDLE_SECURITY_V1_W {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for RPC_BINDING_HANDLE_SECURITY_V1_W {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for RPC_BINDING_HANDLE_SECURITY_V1_W {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RPC_BINDING_HANDLE_SECURITY_V1_W").field("Version", &self.Version).field("ServerPrincName", &self.ServerPrincName).field("AuthnLevel", &self.AuthnLevel).field("AuthnSvc", &self.AuthnSvc).field("AuthIdentity", &self.AuthIdentity).field("SecurityQos", &self.SecurityQos).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::TypeKind for RPC_BINDING_HANDLE_SECURITY_V1_W {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for RPC_BINDING_HANDLE_SECURITY_V1_W {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.ServerPrincName == other.ServerPrincName && self.AuthnLevel == other.AuthnLevel && self.AuthnSvc == other.AuthnSvc && self.AuthIdentity == other.AuthIdentity && self.SecurityQos == other.SecurityQos
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for RPC_BINDING_HANDLE_SECURITY_V1_W {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::default::Default for RPC_BINDING_HANDLE_SECURITY_V1_W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct RPC_BINDING_HANDLE_TEMPLATE_V1_A {
    pub Version: u32,
    pub Flags: u32,
    pub ProtocolSequence: u32,
    pub NetworkAddress: *mut u8,
    pub StringEndpoint: *mut u8,
    pub u1: RPC_BINDING_HANDLE_TEMPLATE_V1_A_0,
    pub ObjectUuid: ::windows::core::GUID,
}
impl ::core::marker::Copy for RPC_BINDING_HANDLE_TEMPLATE_V1_A {}
impl ::core::clone::Clone for RPC_BINDING_HANDLE_TEMPLATE_V1_A {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for RPC_BINDING_HANDLE_TEMPLATE_V1_A {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for RPC_BINDING_HANDLE_TEMPLATE_V1_A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub union RPC_BINDING_HANDLE_TEMPLATE_V1_A_0 {
    pub Reserved: *mut u8,
}
impl ::core::marker::Copy for RPC_BINDING_HANDLE_TEMPLATE_V1_A_0 {}
impl ::core::clone::Clone for RPC_BINDING_HANDLE_TEMPLATE_V1_A_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for RPC_BINDING_HANDLE_TEMPLATE_V1_A_0 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for RPC_BINDING_HANDLE_TEMPLATE_V1_A_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct RPC_BINDING_HANDLE_TEMPLATE_V1_W {
    pub Version: u32,
    pub Flags: u32,
    pub ProtocolSequence: u32,
    pub NetworkAddress: *mut u16,
    pub StringEndpoint: *mut u16,
    pub u1: RPC_BINDING_HANDLE_TEMPLATE_V1_W_0,
    pub ObjectUuid: ::windows::core::GUID,
}
impl ::core::marker::Copy for RPC_BINDING_HANDLE_TEMPLATE_V1_W {}
impl ::core::clone::Clone for RPC_BINDING_HANDLE_TEMPLATE_V1_W {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for RPC_BINDING_HANDLE_TEMPLATE_V1_W {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for RPC_BINDING_HANDLE_TEMPLATE_V1_W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub union RPC_BINDING_HANDLE_TEMPLATE_V1_W_0 {
    pub Reserved: *mut u16,
}
impl ::core::marker::Copy for RPC_BINDING_HANDLE_TEMPLATE_V1_W_0 {}
impl ::core::clone::Clone for RPC_BINDING_HANDLE_TEMPLATE_V1_W_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for RPC_BINDING_HANDLE_TEMPLATE_V1_W_0 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for RPC_BINDING_HANDLE_TEMPLATE_V1_W_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct RPC_BINDING_VECTOR {
    pub Count: u32,
    pub BindingH: [*mut ::core::ffi::c_void; 1],
}
impl ::core::marker::Copy for RPC_BINDING_VECTOR {}
impl ::core::clone::Clone for RPC_BINDING_VECTOR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RPC_BINDING_VECTOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RPC_BINDING_VECTOR").field("Count", &self.Count).field("BindingH", &self.BindingH).finish()
    }
}
impl ::windows::core::TypeKind for RPC_BINDING_VECTOR {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for RPC_BINDING_VECTOR {
    fn eq(&self, other: &Self) -> bool {
        self.Count == other.Count && self.BindingH == other.BindingH
    }
}
impl ::core::cmp::Eq for RPC_BINDING_VECTOR {}
impl ::core::default::Default for RPC_BINDING_VECTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct RPC_CALL_ATTRIBUTES_V1_A {
    pub Version: u32,
    pub Flags: u32,
    pub ServerPrincipalNameBufferLength: u32,
    pub ServerPrincipalName: *mut u8,
    pub ClientPrincipalNameBufferLength: u32,
    pub ClientPrincipalName: *mut u8,
    pub AuthenticationLevel: u32,
    pub AuthenticationService: u32,
    pub NullSession: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for RPC_CALL_ATTRIBUTES_V1_A {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for RPC_CALL_ATTRIBUTES_V1_A {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for RPC_CALL_ATTRIBUTES_V1_A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RPC_CALL_ATTRIBUTES_V1_A")
            .field("Version", &self.Version)
            .field("Flags", &self.Flags)
            .field("ServerPrincipalNameBufferLength", &self.ServerPrincipalNameBufferLength)
            .field("ServerPrincipalName", &self.ServerPrincipalName)
            .field("ClientPrincipalNameBufferLength", &self.ClientPrincipalNameBufferLength)
            .field("ClientPrincipalName", &self.ClientPrincipalName)
            .field("AuthenticationLevel", &self.AuthenticationLevel)
            .field("AuthenticationService", &self.AuthenticationService)
            .field("NullSession", &self.NullSession)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for RPC_CALL_ATTRIBUTES_V1_A {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for RPC_CALL_ATTRIBUTES_V1_A {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.Flags == other.Flags && self.ServerPrincipalNameBufferLength == other.ServerPrincipalNameBufferLength && self.ServerPrincipalName == other.ServerPrincipalName && self.ClientPrincipalNameBufferLength == other.ClientPrincipalNameBufferLength && self.ClientPrincipalName == other.ClientPrincipalName && self.AuthenticationLevel == other.AuthenticationLevel && self.AuthenticationService == other.AuthenticationService && self.NullSession == other.NullSession
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for RPC_CALL_ATTRIBUTES_V1_A {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for RPC_CALL_ATTRIBUTES_V1_A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct RPC_CALL_ATTRIBUTES_V1_W {
    pub Version: u32,
    pub Flags: u32,
    pub ServerPrincipalNameBufferLength: u32,
    pub ServerPrincipalName: *mut u16,
    pub ClientPrincipalNameBufferLength: u32,
    pub ClientPrincipalName: *mut u16,
    pub AuthenticationLevel: u32,
    pub AuthenticationService: u32,
    pub NullSession: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for RPC_CALL_ATTRIBUTES_V1_W {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for RPC_CALL_ATTRIBUTES_V1_W {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for RPC_CALL_ATTRIBUTES_V1_W {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RPC_CALL_ATTRIBUTES_V1_W")
            .field("Version", &self.Version)
            .field("Flags", &self.Flags)
            .field("ServerPrincipalNameBufferLength", &self.ServerPrincipalNameBufferLength)
            .field("ServerPrincipalName", &self.ServerPrincipalName)
            .field("ClientPrincipalNameBufferLength", &self.ClientPrincipalNameBufferLength)
            .field("ClientPrincipalName", &self.ClientPrincipalName)
            .field("AuthenticationLevel", &self.AuthenticationLevel)
            .field("AuthenticationService", &self.AuthenticationService)
            .field("NullSession", &self.NullSession)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for RPC_CALL_ATTRIBUTES_V1_W {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for RPC_CALL_ATTRIBUTES_V1_W {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.Flags == other.Flags && self.ServerPrincipalNameBufferLength == other.ServerPrincipalNameBufferLength && self.ServerPrincipalName == other.ServerPrincipalName && self.ClientPrincipalNameBufferLength == other.ClientPrincipalNameBufferLength && self.ClientPrincipalName == other.ClientPrincipalName && self.AuthenticationLevel == other.AuthenticationLevel && self.AuthenticationService == other.AuthenticationService && self.NullSession == other.NullSession
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for RPC_CALL_ATTRIBUTES_V1_W {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for RPC_CALL_ATTRIBUTES_V1_W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct RPC_CALL_ATTRIBUTES_V2_A {
    pub Version: u32,
    pub Flags: u32,
    pub ServerPrincipalNameBufferLength: u32,
    pub ServerPrincipalName: *mut u8,
    pub ClientPrincipalNameBufferLength: u32,
    pub ClientPrincipalName: *mut u8,
    pub AuthenticationLevel: u32,
    pub AuthenticationService: u32,
    pub NullSession: super::super::Foundation::BOOL,
    pub KernelModeCaller: super::super::Foundation::BOOL,
    pub ProtocolSequence: u32,
    pub IsClientLocal: u32,
    pub ClientPID: super::super::Foundation::HANDLE,
    pub CallStatus: u32,
    pub CallType: RpcCallType,
    pub CallLocalAddress: *mut RPC_CALL_LOCAL_ADDRESS_V1,
    pub OpNum: u16,
    pub InterfaceUuid: ::windows::core::GUID,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for RPC_CALL_ATTRIBUTES_V2_A {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for RPC_CALL_ATTRIBUTES_V2_A {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for RPC_CALL_ATTRIBUTES_V2_A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RPC_CALL_ATTRIBUTES_V2_A")
            .field("Version", &self.Version)
            .field("Flags", &self.Flags)
            .field("ServerPrincipalNameBufferLength", &self.ServerPrincipalNameBufferLength)
            .field("ServerPrincipalName", &self.ServerPrincipalName)
            .field("ClientPrincipalNameBufferLength", &self.ClientPrincipalNameBufferLength)
            .field("ClientPrincipalName", &self.ClientPrincipalName)
            .field("AuthenticationLevel", &self.AuthenticationLevel)
            .field("AuthenticationService", &self.AuthenticationService)
            .field("NullSession", &self.NullSession)
            .field("KernelModeCaller", &self.KernelModeCaller)
            .field("ProtocolSequence", &self.ProtocolSequence)
            .field("IsClientLocal", &self.IsClientLocal)
            .field("ClientPID", &self.ClientPID)
            .field("CallStatus", &self.CallStatus)
            .field("CallType", &self.CallType)
            .field("CallLocalAddress", &self.CallLocalAddress)
            .field("OpNum", &self.OpNum)
            .field("InterfaceUuid", &self.InterfaceUuid)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for RPC_CALL_ATTRIBUTES_V2_A {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for RPC_CALL_ATTRIBUTES_V2_A {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version
            && self.Flags == other.Flags
            && self.ServerPrincipalNameBufferLength == other.ServerPrincipalNameBufferLength
            && self.ServerPrincipalName == other.ServerPrincipalName
            && self.ClientPrincipalNameBufferLength == other.ClientPrincipalNameBufferLength
            && self.ClientPrincipalName == other.ClientPrincipalName
            && self.AuthenticationLevel == other.AuthenticationLevel
            && self.AuthenticationService == other.AuthenticationService
            && self.NullSession == other.NullSession
            && self.KernelModeCaller == other.KernelModeCaller
            && self.ProtocolSequence == other.ProtocolSequence
            && self.IsClientLocal == other.IsClientLocal
            && self.ClientPID == other.ClientPID
            && self.CallStatus == other.CallStatus
            && self.CallType == other.CallType
            && self.CallLocalAddress == other.CallLocalAddress
            && self.OpNum == other.OpNum
            && self.InterfaceUuid == other.InterfaceUuid
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for RPC_CALL_ATTRIBUTES_V2_A {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for RPC_CALL_ATTRIBUTES_V2_A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct RPC_CALL_ATTRIBUTES_V2_W {
    pub Version: u32,
    pub Flags: u32,
    pub ServerPrincipalNameBufferLength: u32,
    pub ServerPrincipalName: *mut u16,
    pub ClientPrincipalNameBufferLength: u32,
    pub ClientPrincipalName: *mut u16,
    pub AuthenticationLevel: u32,
    pub AuthenticationService: u32,
    pub NullSession: super::super::Foundation::BOOL,
    pub KernelModeCaller: super::super::Foundation::BOOL,
    pub ProtocolSequence: u32,
    pub IsClientLocal: RpcCallClientLocality,
    pub ClientPID: super::super::Foundation::HANDLE,
    pub CallStatus: u32,
    pub CallType: RpcCallType,
    pub CallLocalAddress: *mut RPC_CALL_LOCAL_ADDRESS_V1,
    pub OpNum: u16,
    pub InterfaceUuid: ::windows::core::GUID,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for RPC_CALL_ATTRIBUTES_V2_W {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for RPC_CALL_ATTRIBUTES_V2_W {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for RPC_CALL_ATTRIBUTES_V2_W {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RPC_CALL_ATTRIBUTES_V2_W")
            .field("Version", &self.Version)
            .field("Flags", &self.Flags)
            .field("ServerPrincipalNameBufferLength", &self.ServerPrincipalNameBufferLength)
            .field("ServerPrincipalName", &self.ServerPrincipalName)
            .field("ClientPrincipalNameBufferLength", &self.ClientPrincipalNameBufferLength)
            .field("ClientPrincipalName", &self.ClientPrincipalName)
            .field("AuthenticationLevel", &self.AuthenticationLevel)
            .field("AuthenticationService", &self.AuthenticationService)
            .field("NullSession", &self.NullSession)
            .field("KernelModeCaller", &self.KernelModeCaller)
            .field("ProtocolSequence", &self.ProtocolSequence)
            .field("IsClientLocal", &self.IsClientLocal)
            .field("ClientPID", &self.ClientPID)
            .field("CallStatus", &self.CallStatus)
            .field("CallType", &self.CallType)
            .field("CallLocalAddress", &self.CallLocalAddress)
            .field("OpNum", &self.OpNum)
            .field("InterfaceUuid", &self.InterfaceUuid)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for RPC_CALL_ATTRIBUTES_V2_W {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for RPC_CALL_ATTRIBUTES_V2_W {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version
            && self.Flags == other.Flags
            && self.ServerPrincipalNameBufferLength == other.ServerPrincipalNameBufferLength
            && self.ServerPrincipalName == other.ServerPrincipalName
            && self.ClientPrincipalNameBufferLength == other.ClientPrincipalNameBufferLength
            && self.ClientPrincipalName == other.ClientPrincipalName
            && self.AuthenticationLevel == other.AuthenticationLevel
            && self.AuthenticationService == other.AuthenticationService
            && self.NullSession == other.NullSession
            && self.KernelModeCaller == other.KernelModeCaller
            && self.ProtocolSequence == other.ProtocolSequence
            && self.IsClientLocal == other.IsClientLocal
            && self.ClientPID == other.ClientPID
            && self.CallStatus == other.CallStatus
            && self.CallType == other.CallType
            && self.CallLocalAddress == other.CallLocalAddress
            && self.OpNum == other.OpNum
            && self.InterfaceUuid == other.InterfaceUuid
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for RPC_CALL_ATTRIBUTES_V2_W {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for RPC_CALL_ATTRIBUTES_V2_W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct RPC_CALL_ATTRIBUTES_V3_A {
    pub Version: u32,
    pub Flags: u32,
    pub ServerPrincipalNameBufferLength: u32,
    pub ServerPrincipalName: *mut u8,
    pub ClientPrincipalNameBufferLength: u32,
    pub ClientPrincipalName: *mut u8,
    pub AuthenticationLevel: u32,
    pub AuthenticationService: u32,
    pub NullSession: super::super::Foundation::BOOL,
    pub KernelModeCaller: super::super::Foundation::BOOL,
    pub ProtocolSequence: u32,
    pub IsClientLocal: u32,
    pub ClientPID: super::super::Foundation::HANDLE,
    pub CallStatus: u32,
    pub CallType: RpcCallType,
    pub CallLocalAddress: *mut RPC_CALL_LOCAL_ADDRESS_V1,
    pub OpNum: u16,
    pub InterfaceUuid: ::windows::core::GUID,
    pub ClientIdentifierBufferLength: u32,
    pub ClientIdentifier: *mut u8,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for RPC_CALL_ATTRIBUTES_V3_A {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for RPC_CALL_ATTRIBUTES_V3_A {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for RPC_CALL_ATTRIBUTES_V3_A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RPC_CALL_ATTRIBUTES_V3_A")
            .field("Version", &self.Version)
            .field("Flags", &self.Flags)
            .field("ServerPrincipalNameBufferLength", &self.ServerPrincipalNameBufferLength)
            .field("ServerPrincipalName", &self.ServerPrincipalName)
            .field("ClientPrincipalNameBufferLength", &self.ClientPrincipalNameBufferLength)
            .field("ClientPrincipalName", &self.ClientPrincipalName)
            .field("AuthenticationLevel", &self.AuthenticationLevel)
            .field("AuthenticationService", &self.AuthenticationService)
            .field("NullSession", &self.NullSession)
            .field("KernelModeCaller", &self.KernelModeCaller)
            .field("ProtocolSequence", &self.ProtocolSequence)
            .field("IsClientLocal", &self.IsClientLocal)
            .field("ClientPID", &self.ClientPID)
            .field("CallStatus", &self.CallStatus)
            .field("CallType", &self.CallType)
            .field("CallLocalAddress", &self.CallLocalAddress)
            .field("OpNum", &self.OpNum)
            .field("InterfaceUuid", &self.InterfaceUuid)
            .field("ClientIdentifierBufferLength", &self.ClientIdentifierBufferLength)
            .field("ClientIdentifier", &self.ClientIdentifier)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for RPC_CALL_ATTRIBUTES_V3_A {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for RPC_CALL_ATTRIBUTES_V3_A {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version
            && self.Flags == other.Flags
            && self.ServerPrincipalNameBufferLength == other.ServerPrincipalNameBufferLength
            && self.ServerPrincipalName == other.ServerPrincipalName
            && self.ClientPrincipalNameBufferLength == other.ClientPrincipalNameBufferLength
            && self.ClientPrincipalName == other.ClientPrincipalName
            && self.AuthenticationLevel == other.AuthenticationLevel
            && self.AuthenticationService == other.AuthenticationService
            && self.NullSession == other.NullSession
            && self.KernelModeCaller == other.KernelModeCaller
            && self.ProtocolSequence == other.ProtocolSequence
            && self.IsClientLocal == other.IsClientLocal
            && self.ClientPID == other.ClientPID
            && self.CallStatus == other.CallStatus
            && self.CallType == other.CallType
            && self.CallLocalAddress == other.CallLocalAddress
            && self.OpNum == other.OpNum
            && self.InterfaceUuid == other.InterfaceUuid
            && self.ClientIdentifierBufferLength == other.ClientIdentifierBufferLength
            && self.ClientIdentifier == other.ClientIdentifier
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for RPC_CALL_ATTRIBUTES_V3_A {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for RPC_CALL_ATTRIBUTES_V3_A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct RPC_CALL_ATTRIBUTES_V3_W {
    pub Version: u32,
    pub Flags: u32,
    pub ServerPrincipalNameBufferLength: u32,
    pub ServerPrincipalName: *mut u16,
    pub ClientPrincipalNameBufferLength: u32,
    pub ClientPrincipalName: *mut u16,
    pub AuthenticationLevel: u32,
    pub AuthenticationService: u32,
    pub NullSession: super::super::Foundation::BOOL,
    pub KernelModeCaller: super::super::Foundation::BOOL,
    pub ProtocolSequence: u32,
    pub IsClientLocal: RpcCallClientLocality,
    pub ClientPID: super::super::Foundation::HANDLE,
    pub CallStatus: u32,
    pub CallType: RpcCallType,
    pub CallLocalAddress: *mut RPC_CALL_LOCAL_ADDRESS_V1,
    pub OpNum: u16,
    pub InterfaceUuid: ::windows::core::GUID,
    pub ClientIdentifierBufferLength: u32,
    pub ClientIdentifier: *mut u8,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for RPC_CALL_ATTRIBUTES_V3_W {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for RPC_CALL_ATTRIBUTES_V3_W {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for RPC_CALL_ATTRIBUTES_V3_W {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RPC_CALL_ATTRIBUTES_V3_W")
            .field("Version", &self.Version)
            .field("Flags", &self.Flags)
            .field("ServerPrincipalNameBufferLength", &self.ServerPrincipalNameBufferLength)
            .field("ServerPrincipalName", &self.ServerPrincipalName)
            .field("ClientPrincipalNameBufferLength", &self.ClientPrincipalNameBufferLength)
            .field("ClientPrincipalName", &self.ClientPrincipalName)
            .field("AuthenticationLevel", &self.AuthenticationLevel)
            .field("AuthenticationService", &self.AuthenticationService)
            .field("NullSession", &self.NullSession)
            .field("KernelModeCaller", &self.KernelModeCaller)
            .field("ProtocolSequence", &self.ProtocolSequence)
            .field("IsClientLocal", &self.IsClientLocal)
            .field("ClientPID", &self.ClientPID)
            .field("CallStatus", &self.CallStatus)
            .field("CallType", &self.CallType)
            .field("CallLocalAddress", &self.CallLocalAddress)
            .field("OpNum", &self.OpNum)
            .field("InterfaceUuid", &self.InterfaceUuid)
            .field("ClientIdentifierBufferLength", &self.ClientIdentifierBufferLength)
            .field("ClientIdentifier", &self.ClientIdentifier)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for RPC_CALL_ATTRIBUTES_V3_W {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for RPC_CALL_ATTRIBUTES_V3_W {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version
            && self.Flags == other.Flags
            && self.ServerPrincipalNameBufferLength == other.ServerPrincipalNameBufferLength
            && self.ServerPrincipalName == other.ServerPrincipalName
            && self.ClientPrincipalNameBufferLength == other.ClientPrincipalNameBufferLength
            && self.ClientPrincipalName == other.ClientPrincipalName
            && self.AuthenticationLevel == other.AuthenticationLevel
            && self.AuthenticationService == other.AuthenticationService
            && self.NullSession == other.NullSession
            && self.KernelModeCaller == other.KernelModeCaller
            && self.ProtocolSequence == other.ProtocolSequence
            && self.IsClientLocal == other.IsClientLocal
            && self.ClientPID == other.ClientPID
            && self.CallStatus == other.CallStatus
            && self.CallType == other.CallType
            && self.CallLocalAddress == other.CallLocalAddress
            && self.OpNum == other.OpNum
            && self.InterfaceUuid == other.InterfaceUuid
            && self.ClientIdentifierBufferLength == other.ClientIdentifierBufferLength
            && self.ClientIdentifier == other.ClientIdentifier
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for RPC_CALL_ATTRIBUTES_V3_W {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for RPC_CALL_ATTRIBUTES_V3_W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct RPC_CALL_LOCAL_ADDRESS_V1 {
    pub Version: u32,
    pub Buffer: *mut ::core::ffi::c_void,
    pub BufferSize: u32,
    pub AddressFormat: RpcLocalAddressFormat,
}
impl ::core::marker::Copy for RPC_CALL_LOCAL_ADDRESS_V1 {}
impl ::core::clone::Clone for RPC_CALL_LOCAL_ADDRESS_V1 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RPC_CALL_LOCAL_ADDRESS_V1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RPC_CALL_LOCAL_ADDRESS_V1").field("Version", &self.Version).field("Buffer", &self.Buffer).field("BufferSize", &self.BufferSize).field("AddressFormat", &self.AddressFormat).finish()
    }
}
impl ::windows::core::TypeKind for RPC_CALL_LOCAL_ADDRESS_V1 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for RPC_CALL_LOCAL_ADDRESS_V1 {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.Buffer == other.Buffer && self.BufferSize == other.BufferSize && self.AddressFormat == other.AddressFormat
    }
}
impl ::core::cmp::Eq for RPC_CALL_LOCAL_ADDRESS_V1 {}
impl ::core::default::Default for RPC_CALL_LOCAL_ADDRESS_V1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct RPC_CLIENT_INFORMATION1 {
    pub UserName: *mut u8,
    pub ComputerName: *mut u8,
    pub Privilege: u16,
    pub AuthFlags: u32,
}
impl ::core::marker::Copy for RPC_CLIENT_INFORMATION1 {}
impl ::core::clone::Clone for RPC_CLIENT_INFORMATION1 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RPC_CLIENT_INFORMATION1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RPC_CLIENT_INFORMATION1").field("UserName", &self.UserName).field("ComputerName", &self.ComputerName).field("Privilege", &self.Privilege).field("AuthFlags", &self.AuthFlags).finish()
    }
}
impl ::windows::core::TypeKind for RPC_CLIENT_INFORMATION1 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for RPC_CLIENT_INFORMATION1 {
    fn eq(&self, other: &Self) -> bool {
        self.UserName == other.UserName && self.ComputerName == other.ComputerName && self.Privilege == other.Privilege && self.AuthFlags == other.AuthFlags
    }
}
impl ::core::cmp::Eq for RPC_CLIENT_INFORMATION1 {}
impl ::core::default::Default for RPC_CLIENT_INFORMATION1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct RPC_CLIENT_INTERFACE {
    pub Length: u32,
    pub InterfaceId: RPC_SYNTAX_IDENTIFIER,
    pub TransferSyntax: RPC_SYNTAX_IDENTIFIER,
    pub DispatchTable: *mut RPC_DISPATCH_TABLE,
    pub RpcProtseqEndpointCount: u32,
    pub RpcProtseqEndpoint: *mut RPC_PROTSEQ_ENDPOINT,
    pub Reserved: usize,
    pub InterpreterInfo: *const ::core::ffi::c_void,
    pub Flags: u32,
}
impl ::core::marker::Copy for RPC_CLIENT_INTERFACE {}
impl ::core::clone::Clone for RPC_CLIENT_INTERFACE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RPC_CLIENT_INTERFACE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RPC_CLIENT_INTERFACE").field("Length", &self.Length).field("InterfaceId", &self.InterfaceId).field("TransferSyntax", &self.TransferSyntax).field("DispatchTable", &self.DispatchTable).field("RpcProtseqEndpointCount", &self.RpcProtseqEndpointCount).field("RpcProtseqEndpoint", &self.RpcProtseqEndpoint).field("Reserved", &self.Reserved).field("InterpreterInfo", &self.InterpreterInfo).field("Flags", &self.Flags).finish()
    }
}
impl ::windows::core::TypeKind for RPC_CLIENT_INTERFACE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for RPC_CLIENT_INTERFACE {
    fn eq(&self, other: &Self) -> bool {
        self.Length == other.Length && self.InterfaceId == other.InterfaceId && self.TransferSyntax == other.TransferSyntax && self.DispatchTable == other.DispatchTable && self.RpcProtseqEndpointCount == other.RpcProtseqEndpointCount && self.RpcProtseqEndpoint == other.RpcProtseqEndpoint && self.Reserved == other.Reserved && self.InterpreterInfo == other.InterpreterInfo && self.Flags == other.Flags
    }
}
impl ::core::cmp::Eq for RPC_CLIENT_INTERFACE {}
impl ::core::default::Default for RPC_CLIENT_INTERFACE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct RPC_C_OPT_COOKIE_AUTH_DESCRIPTOR {
    pub BufferSize: u32,
    pub Buffer: ::windows::core::PSTR,
}
impl ::core::marker::Copy for RPC_C_OPT_COOKIE_AUTH_DESCRIPTOR {}
impl ::core::clone::Clone for RPC_C_OPT_COOKIE_AUTH_DESCRIPTOR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RPC_C_OPT_COOKIE_AUTH_DESCRIPTOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RPC_C_OPT_COOKIE_AUTH_DESCRIPTOR").field("BufferSize", &self.BufferSize).field("Buffer", &self.Buffer).finish()
    }
}
impl ::windows::core::TypeKind for RPC_C_OPT_COOKIE_AUTH_DESCRIPTOR {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for RPC_C_OPT_COOKIE_AUTH_DESCRIPTOR {
    fn eq(&self, other: &Self) -> bool {
        self.BufferSize == other.BufferSize && self.Buffer == other.Buffer
    }
}
impl ::core::cmp::Eq for RPC_C_OPT_COOKIE_AUTH_DESCRIPTOR {}
impl ::core::default::Default for RPC_C_OPT_COOKIE_AUTH_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct RPC_DISPATCH_TABLE {
    pub DispatchTableCount: u32,
    pub DispatchTable: RPC_DISPATCH_FUNCTION,
    pub Reserved: isize,
}
impl ::core::marker::Copy for RPC_DISPATCH_TABLE {}
impl ::core::clone::Clone for RPC_DISPATCH_TABLE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RPC_DISPATCH_TABLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RPC_DISPATCH_TABLE").field("DispatchTableCount", &self.DispatchTableCount).field("Reserved", &self.Reserved).finish()
    }
}
impl ::windows::core::TypeKind for RPC_DISPATCH_TABLE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for RPC_DISPATCH_TABLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct RPC_EE_INFO_PARAM {
    pub ParameterType: ExtendedErrorParamTypes,
    pub u: RPC_EE_INFO_PARAM_0,
}
impl ::core::marker::Copy for RPC_EE_INFO_PARAM {}
impl ::core::clone::Clone for RPC_EE_INFO_PARAM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for RPC_EE_INFO_PARAM {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for RPC_EE_INFO_PARAM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub union RPC_EE_INFO_PARAM_0 {
    pub AnsiString: ::windows::core::PSTR,
    pub UnicodeString: ::windows::core::PWSTR,
    pub LVal: i32,
    pub SVal: i16,
    pub PVal: u64,
    pub BVal: BinaryParam,
}
impl ::core::marker::Copy for RPC_EE_INFO_PARAM_0 {}
impl ::core::clone::Clone for RPC_EE_INFO_PARAM_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for RPC_EE_INFO_PARAM_0 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for RPC_EE_INFO_PARAM_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct RPC_ENDPOINT_TEMPLATEA {
    pub Version: u32,
    pub ProtSeq: *mut u8,
    pub Endpoint: *mut u8,
    pub SecurityDescriptor: *mut ::core::ffi::c_void,
    pub Backlog: u32,
}
impl ::core::marker::Copy for RPC_ENDPOINT_TEMPLATEA {}
impl ::core::clone::Clone for RPC_ENDPOINT_TEMPLATEA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RPC_ENDPOINT_TEMPLATEA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RPC_ENDPOINT_TEMPLATEA").field("Version", &self.Version).field("ProtSeq", &self.ProtSeq).field("Endpoint", &self.Endpoint).field("SecurityDescriptor", &self.SecurityDescriptor).field("Backlog", &self.Backlog).finish()
    }
}
impl ::windows::core::TypeKind for RPC_ENDPOINT_TEMPLATEA {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for RPC_ENDPOINT_TEMPLATEA {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.ProtSeq == other.ProtSeq && self.Endpoint == other.Endpoint && self.SecurityDescriptor == other.SecurityDescriptor && self.Backlog == other.Backlog
    }
}
impl ::core::cmp::Eq for RPC_ENDPOINT_TEMPLATEA {}
impl ::core::default::Default for RPC_ENDPOINT_TEMPLATEA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct RPC_ENDPOINT_TEMPLATEW {
    pub Version: u32,
    pub ProtSeq: *mut u16,
    pub Endpoint: *mut u16,
    pub SecurityDescriptor: *mut ::core::ffi::c_void,
    pub Backlog: u32,
}
impl ::core::marker::Copy for RPC_ENDPOINT_TEMPLATEW {}
impl ::core::clone::Clone for RPC_ENDPOINT_TEMPLATEW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RPC_ENDPOINT_TEMPLATEW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RPC_ENDPOINT_TEMPLATEW").field("Version", &self.Version).field("ProtSeq", &self.ProtSeq).field("Endpoint", &self.Endpoint).field("SecurityDescriptor", &self.SecurityDescriptor).field("Backlog", &self.Backlog).finish()
    }
}
impl ::windows::core::TypeKind for RPC_ENDPOINT_TEMPLATEW {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for RPC_ENDPOINT_TEMPLATEW {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.ProtSeq == other.ProtSeq && self.Endpoint == other.Endpoint && self.SecurityDescriptor == other.SecurityDescriptor && self.Backlog == other.Backlog
    }
}
impl ::core::cmp::Eq for RPC_ENDPOINT_TEMPLATEW {}
impl ::core::default::Default for RPC_ENDPOINT_TEMPLATEW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct RPC_ERROR_ENUM_HANDLE {
    pub Signature: u32,
    pub CurrentPos: *mut ::core::ffi::c_void,
    pub Head: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for RPC_ERROR_ENUM_HANDLE {}
impl ::core::clone::Clone for RPC_ERROR_ENUM_HANDLE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RPC_ERROR_ENUM_HANDLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RPC_ERROR_ENUM_HANDLE").field("Signature", &self.Signature).field("CurrentPos", &self.CurrentPos).field("Head", &self.Head).finish()
    }
}
impl ::windows::core::TypeKind for RPC_ERROR_ENUM_HANDLE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for RPC_ERROR_ENUM_HANDLE {
    fn eq(&self, other: &Self) -> bool {
        self.Signature == other.Signature && self.CurrentPos == other.CurrentPos && self.Head == other.Head
    }
}
impl ::core::cmp::Eq for RPC_ERROR_ENUM_HANDLE {}
impl ::core::default::Default for RPC_ERROR_ENUM_HANDLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct RPC_EXTENDED_ERROR_INFO {
    pub Version: u32,
    pub ComputerName: ::windows::core::PWSTR,
    pub ProcessID: u32,
    pub u: RPC_EXTENDED_ERROR_INFO_0,
    pub GeneratingComponent: u32,
    pub Status: u32,
    pub DetectionLocation: u16,
    pub Flags: u16,
    pub NumberOfParameters: i32,
    pub Parameters: [RPC_EE_INFO_PARAM; 4],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for RPC_EXTENDED_ERROR_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for RPC_EXTENDED_ERROR_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for RPC_EXTENDED_ERROR_INFO {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for RPC_EXTENDED_ERROR_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union RPC_EXTENDED_ERROR_INFO_0 {
    pub SystemTime: super::super::Foundation::SYSTEMTIME,
    pub FileTime: super::super::Foundation::FILETIME,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for RPC_EXTENDED_ERROR_INFO_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for RPC_EXTENDED_ERROR_INFO_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for RPC_EXTENDED_ERROR_INFO_0 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for RPC_EXTENDED_ERROR_INFO_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct RPC_HTTP_TRANSPORT_CREDENTIALS_A {
    pub TransportCredentials: *mut SEC_WINNT_AUTH_IDENTITY_A,
    pub Flags: RPC_C_HTTP_FLAGS,
    pub AuthenticationTarget: RPC_C_HTTP_AUTHN_TARGET,
    pub NumberOfAuthnSchemes: u32,
    pub AuthnSchemes: *mut u32,
    pub ServerCertificateSubject: *mut u8,
}
impl ::core::marker::Copy for RPC_HTTP_TRANSPORT_CREDENTIALS_A {}
impl ::core::clone::Clone for RPC_HTTP_TRANSPORT_CREDENTIALS_A {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RPC_HTTP_TRANSPORT_CREDENTIALS_A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RPC_HTTP_TRANSPORT_CREDENTIALS_A").field("TransportCredentials", &self.TransportCredentials).field("Flags", &self.Flags).field("AuthenticationTarget", &self.AuthenticationTarget).field("NumberOfAuthnSchemes", &self.NumberOfAuthnSchemes).field("AuthnSchemes", &self.AuthnSchemes).field("ServerCertificateSubject", &self.ServerCertificateSubject).finish()
    }
}
impl ::windows::core::TypeKind for RPC_HTTP_TRANSPORT_CREDENTIALS_A {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for RPC_HTTP_TRANSPORT_CREDENTIALS_A {
    fn eq(&self, other: &Self) -> bool {
        self.TransportCredentials == other.TransportCredentials && self.Flags == other.Flags && self.AuthenticationTarget == other.AuthenticationTarget && self.NumberOfAuthnSchemes == other.NumberOfAuthnSchemes && self.AuthnSchemes == other.AuthnSchemes && self.ServerCertificateSubject == other.ServerCertificateSubject
    }
}
impl ::core::cmp::Eq for RPC_HTTP_TRANSPORT_CREDENTIALS_A {}
impl ::core::default::Default for RPC_HTTP_TRANSPORT_CREDENTIALS_A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct RPC_HTTP_TRANSPORT_CREDENTIALS_V2_A {
    pub TransportCredentials: *mut SEC_WINNT_AUTH_IDENTITY_A,
    pub Flags: RPC_C_HTTP_FLAGS,
    pub AuthenticationTarget: RPC_C_HTTP_AUTHN_TARGET,
    pub NumberOfAuthnSchemes: u32,
    pub AuthnSchemes: *mut u32,
    pub ServerCertificateSubject: *mut u8,
    pub ProxyCredentials: *mut SEC_WINNT_AUTH_IDENTITY_A,
    pub NumberOfProxyAuthnSchemes: u32,
    pub ProxyAuthnSchemes: *mut u32,
}
impl ::core::marker::Copy for RPC_HTTP_TRANSPORT_CREDENTIALS_V2_A {}
impl ::core::clone::Clone for RPC_HTTP_TRANSPORT_CREDENTIALS_V2_A {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RPC_HTTP_TRANSPORT_CREDENTIALS_V2_A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RPC_HTTP_TRANSPORT_CREDENTIALS_V2_A")
            .field("TransportCredentials", &self.TransportCredentials)
            .field("Flags", &self.Flags)
            .field("AuthenticationTarget", &self.AuthenticationTarget)
            .field("NumberOfAuthnSchemes", &self.NumberOfAuthnSchemes)
            .field("AuthnSchemes", &self.AuthnSchemes)
            .field("ServerCertificateSubject", &self.ServerCertificateSubject)
            .field("ProxyCredentials", &self.ProxyCredentials)
            .field("NumberOfProxyAuthnSchemes", &self.NumberOfProxyAuthnSchemes)
            .field("ProxyAuthnSchemes", &self.ProxyAuthnSchemes)
            .finish()
    }
}
impl ::windows::core::TypeKind for RPC_HTTP_TRANSPORT_CREDENTIALS_V2_A {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for RPC_HTTP_TRANSPORT_CREDENTIALS_V2_A {
    fn eq(&self, other: &Self) -> bool {
        self.TransportCredentials == other.TransportCredentials && self.Flags == other.Flags && self.AuthenticationTarget == other.AuthenticationTarget && self.NumberOfAuthnSchemes == other.NumberOfAuthnSchemes && self.AuthnSchemes == other.AuthnSchemes && self.ServerCertificateSubject == other.ServerCertificateSubject && self.ProxyCredentials == other.ProxyCredentials && self.NumberOfProxyAuthnSchemes == other.NumberOfProxyAuthnSchemes && self.ProxyAuthnSchemes == other.ProxyAuthnSchemes
    }
}
impl ::core::cmp::Eq for RPC_HTTP_TRANSPORT_CREDENTIALS_V2_A {}
impl ::core::default::Default for RPC_HTTP_TRANSPORT_CREDENTIALS_V2_A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct RPC_HTTP_TRANSPORT_CREDENTIALS_V2_W {
    pub TransportCredentials: *mut SEC_WINNT_AUTH_IDENTITY_W,
    pub Flags: RPC_C_HTTP_FLAGS,
    pub AuthenticationTarget: RPC_C_HTTP_AUTHN_TARGET,
    pub NumberOfAuthnSchemes: u32,
    pub AuthnSchemes: *mut u32,
    pub ServerCertificateSubject: *mut u16,
    pub ProxyCredentials: *mut SEC_WINNT_AUTH_IDENTITY_W,
    pub NumberOfProxyAuthnSchemes: u32,
    pub ProxyAuthnSchemes: *mut u32,
}
impl ::core::marker::Copy for RPC_HTTP_TRANSPORT_CREDENTIALS_V2_W {}
impl ::core::clone::Clone for RPC_HTTP_TRANSPORT_CREDENTIALS_V2_W {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RPC_HTTP_TRANSPORT_CREDENTIALS_V2_W {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RPC_HTTP_TRANSPORT_CREDENTIALS_V2_W")
            .field("TransportCredentials", &self.TransportCredentials)
            .field("Flags", &self.Flags)
            .field("AuthenticationTarget", &self.AuthenticationTarget)
            .field("NumberOfAuthnSchemes", &self.NumberOfAuthnSchemes)
            .field("AuthnSchemes", &self.AuthnSchemes)
            .field("ServerCertificateSubject", &self.ServerCertificateSubject)
            .field("ProxyCredentials", &self.ProxyCredentials)
            .field("NumberOfProxyAuthnSchemes", &self.NumberOfProxyAuthnSchemes)
            .field("ProxyAuthnSchemes", &self.ProxyAuthnSchemes)
            .finish()
    }
}
impl ::windows::core::TypeKind for RPC_HTTP_TRANSPORT_CREDENTIALS_V2_W {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for RPC_HTTP_TRANSPORT_CREDENTIALS_V2_W {
    fn eq(&self, other: &Self) -> bool {
        self.TransportCredentials == other.TransportCredentials && self.Flags == other.Flags && self.AuthenticationTarget == other.AuthenticationTarget && self.NumberOfAuthnSchemes == other.NumberOfAuthnSchemes && self.AuthnSchemes == other.AuthnSchemes && self.ServerCertificateSubject == other.ServerCertificateSubject && self.ProxyCredentials == other.ProxyCredentials && self.NumberOfProxyAuthnSchemes == other.NumberOfProxyAuthnSchemes && self.ProxyAuthnSchemes == other.ProxyAuthnSchemes
    }
}
impl ::core::cmp::Eq for RPC_HTTP_TRANSPORT_CREDENTIALS_V2_W {}
impl ::core::default::Default for RPC_HTTP_TRANSPORT_CREDENTIALS_V2_W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct RPC_HTTP_TRANSPORT_CREDENTIALS_V3_A {
    pub TransportCredentials: *mut ::core::ffi::c_void,
    pub Flags: RPC_C_HTTP_FLAGS,
    pub AuthenticationTarget: RPC_C_HTTP_AUTHN_TARGET,
    pub NumberOfAuthnSchemes: u32,
    pub AuthnSchemes: *mut u32,
    pub ServerCertificateSubject: *mut u8,
    pub ProxyCredentials: *mut ::core::ffi::c_void,
    pub NumberOfProxyAuthnSchemes: u32,
    pub ProxyAuthnSchemes: *mut u32,
}
impl ::core::marker::Copy for RPC_HTTP_TRANSPORT_CREDENTIALS_V3_A {}
impl ::core::clone::Clone for RPC_HTTP_TRANSPORT_CREDENTIALS_V3_A {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RPC_HTTP_TRANSPORT_CREDENTIALS_V3_A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RPC_HTTP_TRANSPORT_CREDENTIALS_V3_A")
            .field("TransportCredentials", &self.TransportCredentials)
            .field("Flags", &self.Flags)
            .field("AuthenticationTarget", &self.AuthenticationTarget)
            .field("NumberOfAuthnSchemes", &self.NumberOfAuthnSchemes)
            .field("AuthnSchemes", &self.AuthnSchemes)
            .field("ServerCertificateSubject", &self.ServerCertificateSubject)
            .field("ProxyCredentials", &self.ProxyCredentials)
            .field("NumberOfProxyAuthnSchemes", &self.NumberOfProxyAuthnSchemes)
            .field("ProxyAuthnSchemes", &self.ProxyAuthnSchemes)
            .finish()
    }
}
impl ::windows::core::TypeKind for RPC_HTTP_TRANSPORT_CREDENTIALS_V3_A {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for RPC_HTTP_TRANSPORT_CREDENTIALS_V3_A {
    fn eq(&self, other: &Self) -> bool {
        self.TransportCredentials == other.TransportCredentials && self.Flags == other.Flags && self.AuthenticationTarget == other.AuthenticationTarget && self.NumberOfAuthnSchemes == other.NumberOfAuthnSchemes && self.AuthnSchemes == other.AuthnSchemes && self.ServerCertificateSubject == other.ServerCertificateSubject && self.ProxyCredentials == other.ProxyCredentials && self.NumberOfProxyAuthnSchemes == other.NumberOfProxyAuthnSchemes && self.ProxyAuthnSchemes == other.ProxyAuthnSchemes
    }
}
impl ::core::cmp::Eq for RPC_HTTP_TRANSPORT_CREDENTIALS_V3_A {}
impl ::core::default::Default for RPC_HTTP_TRANSPORT_CREDENTIALS_V3_A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct RPC_HTTP_TRANSPORT_CREDENTIALS_V3_W {
    pub TransportCredentials: *mut ::core::ffi::c_void,
    pub Flags: RPC_C_HTTP_FLAGS,
    pub AuthenticationTarget: RPC_C_HTTP_AUTHN_TARGET,
    pub NumberOfAuthnSchemes: u32,
    pub AuthnSchemes: *mut u32,
    pub ServerCertificateSubject: *mut u16,
    pub ProxyCredentials: *mut ::core::ffi::c_void,
    pub NumberOfProxyAuthnSchemes: u32,
    pub ProxyAuthnSchemes: *mut u32,
}
impl ::core::marker::Copy for RPC_HTTP_TRANSPORT_CREDENTIALS_V3_W {}
impl ::core::clone::Clone for RPC_HTTP_TRANSPORT_CREDENTIALS_V3_W {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RPC_HTTP_TRANSPORT_CREDENTIALS_V3_W {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RPC_HTTP_TRANSPORT_CREDENTIALS_V3_W")
            .field("TransportCredentials", &self.TransportCredentials)
            .field("Flags", &self.Flags)
            .field("AuthenticationTarget", &self.AuthenticationTarget)
            .field("NumberOfAuthnSchemes", &self.NumberOfAuthnSchemes)
            .field("AuthnSchemes", &self.AuthnSchemes)
            .field("ServerCertificateSubject", &self.ServerCertificateSubject)
            .field("ProxyCredentials", &self.ProxyCredentials)
            .field("NumberOfProxyAuthnSchemes", &self.NumberOfProxyAuthnSchemes)
            .field("ProxyAuthnSchemes", &self.ProxyAuthnSchemes)
            .finish()
    }
}
impl ::windows::core::TypeKind for RPC_HTTP_TRANSPORT_CREDENTIALS_V3_W {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for RPC_HTTP_TRANSPORT_CREDENTIALS_V3_W {
    fn eq(&self, other: &Self) -> bool {
        self.TransportCredentials == other.TransportCredentials && self.Flags == other.Flags && self.AuthenticationTarget == other.AuthenticationTarget && self.NumberOfAuthnSchemes == other.NumberOfAuthnSchemes && self.AuthnSchemes == other.AuthnSchemes && self.ServerCertificateSubject == other.ServerCertificateSubject && self.ProxyCredentials == other.ProxyCredentials && self.NumberOfProxyAuthnSchemes == other.NumberOfProxyAuthnSchemes && self.ProxyAuthnSchemes == other.ProxyAuthnSchemes
    }
}
impl ::core::cmp::Eq for RPC_HTTP_TRANSPORT_CREDENTIALS_V3_W {}
impl ::core::default::Default for RPC_HTTP_TRANSPORT_CREDENTIALS_V3_W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct RPC_HTTP_TRANSPORT_CREDENTIALS_W {
    pub TransportCredentials: *mut SEC_WINNT_AUTH_IDENTITY_W,
    pub Flags: RPC_C_HTTP_FLAGS,
    pub AuthenticationTarget: RPC_C_HTTP_AUTHN_TARGET,
    pub NumberOfAuthnSchemes: u32,
    pub AuthnSchemes: *mut u32,
    pub ServerCertificateSubject: *mut u16,
}
impl ::core::marker::Copy for RPC_HTTP_TRANSPORT_CREDENTIALS_W {}
impl ::core::clone::Clone for RPC_HTTP_TRANSPORT_CREDENTIALS_W {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RPC_HTTP_TRANSPORT_CREDENTIALS_W {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RPC_HTTP_TRANSPORT_CREDENTIALS_W").field("TransportCredentials", &self.TransportCredentials).field("Flags", &self.Flags).field("AuthenticationTarget", &self.AuthenticationTarget).field("NumberOfAuthnSchemes", &self.NumberOfAuthnSchemes).field("AuthnSchemes", &self.AuthnSchemes).field("ServerCertificateSubject", &self.ServerCertificateSubject).finish()
    }
}
impl ::windows::core::TypeKind for RPC_HTTP_TRANSPORT_CREDENTIALS_W {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for RPC_HTTP_TRANSPORT_CREDENTIALS_W {
    fn eq(&self, other: &Self) -> bool {
        self.TransportCredentials == other.TransportCredentials && self.Flags == other.Flags && self.AuthenticationTarget == other.AuthenticationTarget && self.NumberOfAuthnSchemes == other.NumberOfAuthnSchemes && self.AuthnSchemes == other.AuthnSchemes && self.ServerCertificateSubject == other.ServerCertificateSubject
    }
}
impl ::core::cmp::Eq for RPC_HTTP_TRANSPORT_CREDENTIALS_W {}
impl ::core::default::Default for RPC_HTTP_TRANSPORT_CREDENTIALS_W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct RPC_IF_ID {
    pub Uuid: ::windows::core::GUID,
    pub VersMajor: u16,
    pub VersMinor: u16,
}
impl ::core::marker::Copy for RPC_IF_ID {}
impl ::core::clone::Clone for RPC_IF_ID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RPC_IF_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RPC_IF_ID").field("Uuid", &self.Uuid).field("VersMajor", &self.VersMajor).field("VersMinor", &self.VersMinor).finish()
    }
}
impl ::windows::core::TypeKind for RPC_IF_ID {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for RPC_IF_ID {
    fn eq(&self, other: &Self) -> bool {
        self.Uuid == other.Uuid && self.VersMajor == other.VersMajor && self.VersMinor == other.VersMinor
    }
}
impl ::core::cmp::Eq for RPC_IF_ID {}
impl ::core::default::Default for RPC_IF_ID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct RPC_IF_ID_VECTOR {
    pub Count: u32,
    pub IfId: [*mut RPC_IF_ID; 1],
}
impl ::core::marker::Copy for RPC_IF_ID_VECTOR {}
impl ::core::clone::Clone for RPC_IF_ID_VECTOR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RPC_IF_ID_VECTOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RPC_IF_ID_VECTOR").field("Count", &self.Count).field("IfId", &self.IfId).finish()
    }
}
impl ::windows::core::TypeKind for RPC_IF_ID_VECTOR {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for RPC_IF_ID_VECTOR {
    fn eq(&self, other: &Self) -> bool {
        self.Count == other.Count && self.IfId == other.IfId
    }
}
impl ::core::cmp::Eq for RPC_IF_ID_VECTOR {}
impl ::core::default::Default for RPC_IF_ID_VECTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct RPC_IMPORT_CONTEXT_P {
    pub LookupContext: *mut ::core::ffi::c_void,
    pub ProposedHandle: *mut ::core::ffi::c_void,
    pub Bindings: *mut RPC_BINDING_VECTOR,
}
impl ::core::marker::Copy for RPC_IMPORT_CONTEXT_P {}
impl ::core::clone::Clone for RPC_IMPORT_CONTEXT_P {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RPC_IMPORT_CONTEXT_P {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RPC_IMPORT_CONTEXT_P").field("LookupContext", &self.LookupContext).field("ProposedHandle", &self.ProposedHandle).field("Bindings", &self.Bindings).finish()
    }
}
impl ::windows::core::TypeKind for RPC_IMPORT_CONTEXT_P {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for RPC_IMPORT_CONTEXT_P {
    fn eq(&self, other: &Self) -> bool {
        self.LookupContext == other.LookupContext && self.ProposedHandle == other.ProposedHandle && self.Bindings == other.Bindings
    }
}
impl ::core::cmp::Eq for RPC_IMPORT_CONTEXT_P {}
impl ::core::default::Default for RPC_IMPORT_CONTEXT_P {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct RPC_INTERFACE_TEMPLATEA {
    pub Version: u32,
    pub IfSpec: *mut ::core::ffi::c_void,
    pub MgrTypeUuid: *mut ::windows::core::GUID,
    pub MgrEpv: *mut ::core::ffi::c_void,
    pub Flags: u32,
    pub MaxCalls: u32,
    pub MaxRpcSize: u32,
    pub IfCallback: RPC_IF_CALLBACK_FN,
    pub UuidVector: *mut UUID_VECTOR,
    pub Annotation: *mut u8,
    pub SecurityDescriptor: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for RPC_INTERFACE_TEMPLATEA {}
impl ::core::clone::Clone for RPC_INTERFACE_TEMPLATEA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RPC_INTERFACE_TEMPLATEA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RPC_INTERFACE_TEMPLATEA").field("Version", &self.Version).field("IfSpec", &self.IfSpec).field("MgrTypeUuid", &self.MgrTypeUuid).field("MgrEpv", &self.MgrEpv).field("Flags", &self.Flags).field("MaxCalls", &self.MaxCalls).field("MaxRpcSize", &self.MaxRpcSize).field("UuidVector", &self.UuidVector).field("Annotation", &self.Annotation).field("SecurityDescriptor", &self.SecurityDescriptor).finish()
    }
}
impl ::windows::core::TypeKind for RPC_INTERFACE_TEMPLATEA {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for RPC_INTERFACE_TEMPLATEA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct RPC_INTERFACE_TEMPLATEW {
    pub Version: u32,
    pub IfSpec: *mut ::core::ffi::c_void,
    pub MgrTypeUuid: *mut ::windows::core::GUID,
    pub MgrEpv: *mut ::core::ffi::c_void,
    pub Flags: u32,
    pub MaxCalls: u32,
    pub MaxRpcSize: u32,
    pub IfCallback: RPC_IF_CALLBACK_FN,
    pub UuidVector: *mut UUID_VECTOR,
    pub Annotation: *mut u16,
    pub SecurityDescriptor: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for RPC_INTERFACE_TEMPLATEW {}
impl ::core::clone::Clone for RPC_INTERFACE_TEMPLATEW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RPC_INTERFACE_TEMPLATEW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RPC_INTERFACE_TEMPLATEW").field("Version", &self.Version).field("IfSpec", &self.IfSpec).field("MgrTypeUuid", &self.MgrTypeUuid).field("MgrEpv", &self.MgrEpv).field("Flags", &self.Flags).field("MaxCalls", &self.MaxCalls).field("MaxRpcSize", &self.MaxRpcSize).field("UuidVector", &self.UuidVector).field("Annotation", &self.Annotation).field("SecurityDescriptor", &self.SecurityDescriptor).finish()
    }
}
impl ::windows::core::TypeKind for RPC_INTERFACE_TEMPLATEW {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for RPC_INTERFACE_TEMPLATEW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct RPC_MESSAGE {
    pub Handle: *mut ::core::ffi::c_void,
    pub DataRepresentation: u32,
    pub Buffer: *mut ::core::ffi::c_void,
    pub BufferLength: u32,
    pub ProcNum: u32,
    pub TransferSyntax: *mut RPC_SYNTAX_IDENTIFIER,
    pub RpcInterfaceInformation: *mut ::core::ffi::c_void,
    pub ReservedForRuntime: *mut ::core::ffi::c_void,
    pub ManagerEpv: *mut ::core::ffi::c_void,
    pub ImportContext: *mut ::core::ffi::c_void,
    pub RpcFlags: u32,
}
impl ::core::marker::Copy for RPC_MESSAGE {}
impl ::core::clone::Clone for RPC_MESSAGE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RPC_MESSAGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RPC_MESSAGE")
            .field("Handle", &self.Handle)
            .field("DataRepresentation", &self.DataRepresentation)
            .field("Buffer", &self.Buffer)
            .field("BufferLength", &self.BufferLength)
            .field("ProcNum", &self.ProcNum)
            .field("TransferSyntax", &self.TransferSyntax)
            .field("RpcInterfaceInformation", &self.RpcInterfaceInformation)
            .field("ReservedForRuntime", &self.ReservedForRuntime)
            .field("ManagerEpv", &self.ManagerEpv)
            .field("ImportContext", &self.ImportContext)
            .field("RpcFlags", &self.RpcFlags)
            .finish()
    }
}
impl ::windows::core::TypeKind for RPC_MESSAGE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for RPC_MESSAGE {
    fn eq(&self, other: &Self) -> bool {
        self.Handle == other.Handle && self.DataRepresentation == other.DataRepresentation && self.Buffer == other.Buffer && self.BufferLength == other.BufferLength && self.ProcNum == other.ProcNum && self.TransferSyntax == other.TransferSyntax && self.RpcInterfaceInformation == other.RpcInterfaceInformation && self.ReservedForRuntime == other.ReservedForRuntime && self.ManagerEpv == other.ManagerEpv && self.ImportContext == other.ImportContext && self.RpcFlags == other.RpcFlags
    }
}
impl ::core::cmp::Eq for RPC_MESSAGE {}
impl ::core::default::Default for RPC_MESSAGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct RPC_POLICY {
    pub Length: u32,
    pub EndpointFlags: u32,
    pub NICFlags: u32,
}
impl ::core::marker::Copy for RPC_POLICY {}
impl ::core::clone::Clone for RPC_POLICY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RPC_POLICY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RPC_POLICY").field("Length", &self.Length).field("EndpointFlags", &self.EndpointFlags).field("NICFlags", &self.NICFlags).finish()
    }
}
impl ::windows::core::TypeKind for RPC_POLICY {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for RPC_POLICY {
    fn eq(&self, other: &Self) -> bool {
        self.Length == other.Length && self.EndpointFlags == other.EndpointFlags && self.NICFlags == other.NICFlags
    }
}
impl ::core::cmp::Eq for RPC_POLICY {}
impl ::core::default::Default for RPC_POLICY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct RPC_PROTSEQ_ENDPOINT {
    pub RpcProtocolSequence: *mut u8,
    pub Endpoint: *mut u8,
}
impl ::core::marker::Copy for RPC_PROTSEQ_ENDPOINT {}
impl ::core::clone::Clone for RPC_PROTSEQ_ENDPOINT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RPC_PROTSEQ_ENDPOINT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RPC_PROTSEQ_ENDPOINT").field("RpcProtocolSequence", &self.RpcProtocolSequence).field("Endpoint", &self.Endpoint).finish()
    }
}
impl ::windows::core::TypeKind for RPC_PROTSEQ_ENDPOINT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for RPC_PROTSEQ_ENDPOINT {
    fn eq(&self, other: &Self) -> bool {
        self.RpcProtocolSequence == other.RpcProtocolSequence && self.Endpoint == other.Endpoint
    }
}
impl ::core::cmp::Eq for RPC_PROTSEQ_ENDPOINT {}
impl ::core::default::Default for RPC_PROTSEQ_ENDPOINT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct RPC_PROTSEQ_VECTORA {
    pub Count: u32,
    pub Protseq: [*mut u8; 1],
}
impl ::core::marker::Copy for RPC_PROTSEQ_VECTORA {}
impl ::core::clone::Clone for RPC_PROTSEQ_VECTORA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RPC_PROTSEQ_VECTORA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RPC_PROTSEQ_VECTORA").field("Count", &self.Count).field("Protseq", &self.Protseq).finish()
    }
}
impl ::windows::core::TypeKind for RPC_PROTSEQ_VECTORA {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for RPC_PROTSEQ_VECTORA {
    fn eq(&self, other: &Self) -> bool {
        self.Count == other.Count && self.Protseq == other.Protseq
    }
}
impl ::core::cmp::Eq for RPC_PROTSEQ_VECTORA {}
impl ::core::default::Default for RPC_PROTSEQ_VECTORA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct RPC_PROTSEQ_VECTORW {
    pub Count: u32,
    pub Protseq: [*mut u16; 1],
}
impl ::core::marker::Copy for RPC_PROTSEQ_VECTORW {}
impl ::core::clone::Clone for RPC_PROTSEQ_VECTORW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RPC_PROTSEQ_VECTORW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RPC_PROTSEQ_VECTORW").field("Count", &self.Count).field("Protseq", &self.Protseq).finish()
    }
}
impl ::windows::core::TypeKind for RPC_PROTSEQ_VECTORW {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for RPC_PROTSEQ_VECTORW {
    fn eq(&self, other: &Self) -> bool {
        self.Count == other.Count && self.Protseq == other.Protseq
    }
}
impl ::core::cmp::Eq for RPC_PROTSEQ_VECTORW {}
impl ::core::default::Default for RPC_PROTSEQ_VECTORW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub struct RPC_SECURITY_QOS {
    pub Version: u32,
    pub Capabilities: RPC_C_QOS_CAPABILITIES,
    pub IdentityTracking: RPC_C_QOS_IDENTITY,
    pub ImpersonationType: super::Com::RPC_C_IMP_LEVEL,
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::marker::Copy for RPC_SECURITY_QOS {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for RPC_SECURITY_QOS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for RPC_SECURITY_QOS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RPC_SECURITY_QOS").field("Version", &self.Version).field("Capabilities", &self.Capabilities).field("IdentityTracking", &self.IdentityTracking).field("ImpersonationType", &self.ImpersonationType).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::TypeKind for RPC_SECURITY_QOS {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for RPC_SECURITY_QOS {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.Capabilities == other.Capabilities && self.IdentityTracking == other.IdentityTracking && self.ImpersonationType == other.ImpersonationType
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for RPC_SECURITY_QOS {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::default::Default for RPC_SECURITY_QOS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub struct RPC_SECURITY_QOS_V2_A {
    pub Version: u32,
    pub Capabilities: RPC_C_QOS_CAPABILITIES,
    pub IdentityTracking: RPC_C_QOS_IDENTITY,
    pub ImpersonationType: super::Com::RPC_C_IMP_LEVEL,
    pub AdditionalSecurityInfoType: RPC_C_AUTHN_INFO_TYPE,
    pub u: RPC_SECURITY_QOS_V2_A_0,
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::marker::Copy for RPC_SECURITY_QOS_V2_A {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for RPC_SECURITY_QOS_V2_A {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::TypeKind for RPC_SECURITY_QOS_V2_A {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::default::Default for RPC_SECURITY_QOS_V2_A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub union RPC_SECURITY_QOS_V2_A_0 {
    pub HttpCredentials: *mut RPC_HTTP_TRANSPORT_CREDENTIALS_A,
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::marker::Copy for RPC_SECURITY_QOS_V2_A_0 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for RPC_SECURITY_QOS_V2_A_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::TypeKind for RPC_SECURITY_QOS_V2_A_0 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::default::Default for RPC_SECURITY_QOS_V2_A_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub struct RPC_SECURITY_QOS_V2_W {
    pub Version: u32,
    pub Capabilities: RPC_C_QOS_CAPABILITIES,
    pub IdentityTracking: RPC_C_QOS_IDENTITY,
    pub ImpersonationType: super::Com::RPC_C_IMP_LEVEL,
    pub AdditionalSecurityInfoType: RPC_C_AUTHN_INFO_TYPE,
    pub u: RPC_SECURITY_QOS_V2_W_0,
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::marker::Copy for RPC_SECURITY_QOS_V2_W {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for RPC_SECURITY_QOS_V2_W {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::TypeKind for RPC_SECURITY_QOS_V2_W {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::default::Default for RPC_SECURITY_QOS_V2_W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub union RPC_SECURITY_QOS_V2_W_0 {
    pub HttpCredentials: *mut RPC_HTTP_TRANSPORT_CREDENTIALS_W,
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::marker::Copy for RPC_SECURITY_QOS_V2_W_0 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for RPC_SECURITY_QOS_V2_W_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::TypeKind for RPC_SECURITY_QOS_V2_W_0 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::default::Default for RPC_SECURITY_QOS_V2_W_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub struct RPC_SECURITY_QOS_V3_A {
    pub Version: u32,
    pub Capabilities: RPC_C_QOS_CAPABILITIES,
    pub IdentityTracking: RPC_C_QOS_IDENTITY,
    pub ImpersonationType: super::Com::RPC_C_IMP_LEVEL,
    pub AdditionalSecurityInfoType: RPC_C_AUTHN_INFO_TYPE,
    pub u: RPC_SECURITY_QOS_V3_A_0,
    pub Sid: *mut ::core::ffi::c_void,
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::marker::Copy for RPC_SECURITY_QOS_V3_A {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for RPC_SECURITY_QOS_V3_A {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::TypeKind for RPC_SECURITY_QOS_V3_A {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::default::Default for RPC_SECURITY_QOS_V3_A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub union RPC_SECURITY_QOS_V3_A_0 {
    pub HttpCredentials: *mut RPC_HTTP_TRANSPORT_CREDENTIALS_A,
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::marker::Copy for RPC_SECURITY_QOS_V3_A_0 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for RPC_SECURITY_QOS_V3_A_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::TypeKind for RPC_SECURITY_QOS_V3_A_0 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::default::Default for RPC_SECURITY_QOS_V3_A_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub struct RPC_SECURITY_QOS_V3_W {
    pub Version: u32,
    pub Capabilities: RPC_C_QOS_CAPABILITIES,
    pub IdentityTracking: RPC_C_QOS_IDENTITY,
    pub ImpersonationType: super::Com::RPC_C_IMP_LEVEL,
    pub AdditionalSecurityInfoType: RPC_C_AUTHN_INFO_TYPE,
    pub u: RPC_SECURITY_QOS_V3_W_0,
    pub Sid: *mut ::core::ffi::c_void,
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::marker::Copy for RPC_SECURITY_QOS_V3_W {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for RPC_SECURITY_QOS_V3_W {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::TypeKind for RPC_SECURITY_QOS_V3_W {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::default::Default for RPC_SECURITY_QOS_V3_W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub union RPC_SECURITY_QOS_V3_W_0 {
    pub HttpCredentials: *mut RPC_HTTP_TRANSPORT_CREDENTIALS_W,
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::marker::Copy for RPC_SECURITY_QOS_V3_W_0 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for RPC_SECURITY_QOS_V3_W_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::TypeKind for RPC_SECURITY_QOS_V3_W_0 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::default::Default for RPC_SECURITY_QOS_V3_W_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub struct RPC_SECURITY_QOS_V4_A {
    pub Version: u32,
    pub Capabilities: RPC_C_QOS_CAPABILITIES,
    pub IdentityTracking: RPC_C_QOS_IDENTITY,
    pub ImpersonationType: super::Com::RPC_C_IMP_LEVEL,
    pub AdditionalSecurityInfoType: RPC_C_AUTHN_INFO_TYPE,
    pub u: RPC_SECURITY_QOS_V4_A_0,
    pub Sid: *mut ::core::ffi::c_void,
    pub EffectiveOnly: u32,
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::marker::Copy for RPC_SECURITY_QOS_V4_A {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for RPC_SECURITY_QOS_V4_A {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::TypeKind for RPC_SECURITY_QOS_V4_A {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::default::Default for RPC_SECURITY_QOS_V4_A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub union RPC_SECURITY_QOS_V4_A_0 {
    pub HttpCredentials: *mut RPC_HTTP_TRANSPORT_CREDENTIALS_A,
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::marker::Copy for RPC_SECURITY_QOS_V4_A_0 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for RPC_SECURITY_QOS_V4_A_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::TypeKind for RPC_SECURITY_QOS_V4_A_0 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::default::Default for RPC_SECURITY_QOS_V4_A_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub struct RPC_SECURITY_QOS_V4_W {
    pub Version: u32,
    pub Capabilities: RPC_C_QOS_CAPABILITIES,
    pub IdentityTracking: RPC_C_QOS_IDENTITY,
    pub ImpersonationType: super::Com::RPC_C_IMP_LEVEL,
    pub AdditionalSecurityInfoType: RPC_C_AUTHN_INFO_TYPE,
    pub u: RPC_SECURITY_QOS_V4_W_0,
    pub Sid: *mut ::core::ffi::c_void,
    pub EffectiveOnly: u32,
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::marker::Copy for RPC_SECURITY_QOS_V4_W {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for RPC_SECURITY_QOS_V4_W {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::TypeKind for RPC_SECURITY_QOS_V4_W {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::default::Default for RPC_SECURITY_QOS_V4_W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub union RPC_SECURITY_QOS_V4_W_0 {
    pub HttpCredentials: *mut RPC_HTTP_TRANSPORT_CREDENTIALS_W,
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::marker::Copy for RPC_SECURITY_QOS_V4_W_0 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for RPC_SECURITY_QOS_V4_W_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::TypeKind for RPC_SECURITY_QOS_V4_W_0 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::default::Default for RPC_SECURITY_QOS_V4_W_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub struct RPC_SECURITY_QOS_V5_A {
    pub Version: u32,
    pub Capabilities: RPC_C_QOS_CAPABILITIES,
    pub IdentityTracking: RPC_C_QOS_IDENTITY,
    pub ImpersonationType: super::Com::RPC_C_IMP_LEVEL,
    pub AdditionalSecurityInfoType: RPC_C_AUTHN_INFO_TYPE,
    pub u: RPC_SECURITY_QOS_V5_A_0,
    pub Sid: *mut ::core::ffi::c_void,
    pub EffectiveOnly: u32,
    pub ServerSecurityDescriptor: *mut ::core::ffi::c_void,
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::marker::Copy for RPC_SECURITY_QOS_V5_A {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for RPC_SECURITY_QOS_V5_A {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::TypeKind for RPC_SECURITY_QOS_V5_A {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::default::Default for RPC_SECURITY_QOS_V5_A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub union RPC_SECURITY_QOS_V5_A_0 {
    pub HttpCredentials: *mut RPC_HTTP_TRANSPORT_CREDENTIALS_A,
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::marker::Copy for RPC_SECURITY_QOS_V5_A_0 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for RPC_SECURITY_QOS_V5_A_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::TypeKind for RPC_SECURITY_QOS_V5_A_0 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::default::Default for RPC_SECURITY_QOS_V5_A_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub struct RPC_SECURITY_QOS_V5_W {
    pub Version: u32,
    pub Capabilities: RPC_C_QOS_CAPABILITIES,
    pub IdentityTracking: RPC_C_QOS_IDENTITY,
    pub ImpersonationType: super::Com::RPC_C_IMP_LEVEL,
    pub AdditionalSecurityInfoType: RPC_C_AUTHN_INFO_TYPE,
    pub u: RPC_SECURITY_QOS_V5_W_0,
    pub Sid: *mut ::core::ffi::c_void,
    pub EffectiveOnly: u32,
    pub ServerSecurityDescriptor: *mut ::core::ffi::c_void,
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::marker::Copy for RPC_SECURITY_QOS_V5_W {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for RPC_SECURITY_QOS_V5_W {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::TypeKind for RPC_SECURITY_QOS_V5_W {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::default::Default for RPC_SECURITY_QOS_V5_W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub union RPC_SECURITY_QOS_V5_W_0 {
    pub HttpCredentials: *mut RPC_HTTP_TRANSPORT_CREDENTIALS_W,
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::marker::Copy for RPC_SECURITY_QOS_V5_W_0 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for RPC_SECURITY_QOS_V5_W_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::TypeKind for RPC_SECURITY_QOS_V5_W_0 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::default::Default for RPC_SECURITY_QOS_V5_W_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct RPC_SEC_CONTEXT_KEY_INFO {
    pub EncryptAlgorithm: u32,
    pub KeySize: u32,
    pub SignatureAlgorithm: u32,
}
impl ::core::marker::Copy for RPC_SEC_CONTEXT_KEY_INFO {}
impl ::core::clone::Clone for RPC_SEC_CONTEXT_KEY_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RPC_SEC_CONTEXT_KEY_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RPC_SEC_CONTEXT_KEY_INFO").field("EncryptAlgorithm", &self.EncryptAlgorithm).field("KeySize", &self.KeySize).field("SignatureAlgorithm", &self.SignatureAlgorithm).finish()
    }
}
impl ::windows::core::TypeKind for RPC_SEC_CONTEXT_KEY_INFO {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for RPC_SEC_CONTEXT_KEY_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.EncryptAlgorithm == other.EncryptAlgorithm && self.KeySize == other.KeySize && self.SignatureAlgorithm == other.SignatureAlgorithm
    }
}
impl ::core::cmp::Eq for RPC_SEC_CONTEXT_KEY_INFO {}
impl ::core::default::Default for RPC_SEC_CONTEXT_KEY_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct RPC_SERVER_INTERFACE {
    pub Length: u32,
    pub InterfaceId: RPC_SYNTAX_IDENTIFIER,
    pub TransferSyntax: RPC_SYNTAX_IDENTIFIER,
    pub DispatchTable: *mut RPC_DISPATCH_TABLE,
    pub RpcProtseqEndpointCount: u32,
    pub RpcProtseqEndpoint: *mut RPC_PROTSEQ_ENDPOINT,
    pub DefaultManagerEpv: *mut ::core::ffi::c_void,
    pub InterpreterInfo: *const ::core::ffi::c_void,
    pub Flags: u32,
}
impl ::core::marker::Copy for RPC_SERVER_INTERFACE {}
impl ::core::clone::Clone for RPC_SERVER_INTERFACE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RPC_SERVER_INTERFACE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RPC_SERVER_INTERFACE").field("Length", &self.Length).field("InterfaceId", &self.InterfaceId).field("TransferSyntax", &self.TransferSyntax).field("DispatchTable", &self.DispatchTable).field("RpcProtseqEndpointCount", &self.RpcProtseqEndpointCount).field("RpcProtseqEndpoint", &self.RpcProtseqEndpoint).field("DefaultManagerEpv", &self.DefaultManagerEpv).field("InterpreterInfo", &self.InterpreterInfo).field("Flags", &self.Flags).finish()
    }
}
impl ::windows::core::TypeKind for RPC_SERVER_INTERFACE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for RPC_SERVER_INTERFACE {
    fn eq(&self, other: &Self) -> bool {
        self.Length == other.Length && self.InterfaceId == other.InterfaceId && self.TransferSyntax == other.TransferSyntax && self.DispatchTable == other.DispatchTable && self.RpcProtseqEndpointCount == other.RpcProtseqEndpointCount && self.RpcProtseqEndpoint == other.RpcProtseqEndpoint && self.DefaultManagerEpv == other.DefaultManagerEpv && self.InterpreterInfo == other.InterpreterInfo && self.Flags == other.Flags
    }
}
impl ::core::cmp::Eq for RPC_SERVER_INTERFACE {}
impl ::core::default::Default for RPC_SERVER_INTERFACE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct RPC_STATS_VECTOR {
    pub Count: u32,
    pub Stats: [u32; 1],
}
impl ::core::marker::Copy for RPC_STATS_VECTOR {}
impl ::core::clone::Clone for RPC_STATS_VECTOR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RPC_STATS_VECTOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RPC_STATS_VECTOR").field("Count", &self.Count).field("Stats", &self.Stats).finish()
    }
}
impl ::windows::core::TypeKind for RPC_STATS_VECTOR {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for RPC_STATS_VECTOR {
    fn eq(&self, other: &Self) -> bool {
        self.Count == other.Count && self.Stats == other.Stats
    }
}
impl ::core::cmp::Eq for RPC_STATS_VECTOR {}
impl ::core::default::Default for RPC_STATS_VECTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct RPC_SYNTAX_IDENTIFIER {
    pub SyntaxGUID: ::windows::core::GUID,
    pub SyntaxVersion: RPC_VERSION,
}
impl ::core::marker::Copy for RPC_SYNTAX_IDENTIFIER {}
impl ::core::clone::Clone for RPC_SYNTAX_IDENTIFIER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RPC_SYNTAX_IDENTIFIER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RPC_SYNTAX_IDENTIFIER").field("SyntaxGUID", &self.SyntaxGUID).field("SyntaxVersion", &self.SyntaxVersion).finish()
    }
}
impl ::windows::core::TypeKind for RPC_SYNTAX_IDENTIFIER {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for RPC_SYNTAX_IDENTIFIER {
    fn eq(&self, other: &Self) -> bool {
        self.SyntaxGUID == other.SyntaxGUID && self.SyntaxVersion == other.SyntaxVersion
    }
}
impl ::core::cmp::Eq for RPC_SYNTAX_IDENTIFIER {}
impl ::core::default::Default for RPC_SYNTAX_IDENTIFIER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct RPC_TRANSFER_SYNTAX {
    pub Uuid: ::windows::core::GUID,
    pub VersMajor: u16,
    pub VersMinor: u16,
}
impl ::core::marker::Copy for RPC_TRANSFER_SYNTAX {}
impl ::core::clone::Clone for RPC_TRANSFER_SYNTAX {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RPC_TRANSFER_SYNTAX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RPC_TRANSFER_SYNTAX").field("Uuid", &self.Uuid).field("VersMajor", &self.VersMajor).field("VersMinor", &self.VersMinor).finish()
    }
}
impl ::windows::core::TypeKind for RPC_TRANSFER_SYNTAX {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for RPC_TRANSFER_SYNTAX {
    fn eq(&self, other: &Self) -> bool {
        self.Uuid == other.Uuid && self.VersMajor == other.VersMajor && self.VersMinor == other.VersMinor
    }
}
impl ::core::cmp::Eq for RPC_TRANSFER_SYNTAX {}
impl ::core::default::Default for RPC_TRANSFER_SYNTAX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct RPC_VERSION {
    pub MajorVersion: u16,
    pub MinorVersion: u16,
}
impl ::core::marker::Copy for RPC_VERSION {}
impl ::core::clone::Clone for RPC_VERSION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RPC_VERSION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RPC_VERSION").field("MajorVersion", &self.MajorVersion).field("MinorVersion", &self.MinorVersion).finish()
    }
}
impl ::windows::core::TypeKind for RPC_VERSION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for RPC_VERSION {
    fn eq(&self, other: &Self) -> bool {
        self.MajorVersion == other.MajorVersion && self.MinorVersion == other.MinorVersion
    }
}
impl ::core::cmp::Eq for RPC_VERSION {}
impl ::core::default::Default for RPC_VERSION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct SCONTEXT_QUEUE {
    pub NumberOfObjects: u32,
    pub ArrayOfObjects: *mut *mut NDR_SCONTEXT,
}
impl ::core::marker::Copy for SCONTEXT_QUEUE {}
impl ::core::clone::Clone for SCONTEXT_QUEUE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SCONTEXT_QUEUE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCONTEXT_QUEUE").field("NumberOfObjects", &self.NumberOfObjects).field("ArrayOfObjects", &self.ArrayOfObjects).finish()
    }
}
impl ::windows::core::TypeKind for SCONTEXT_QUEUE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SCONTEXT_QUEUE {
    fn eq(&self, other: &Self) -> bool {
        self.NumberOfObjects == other.NumberOfObjects && self.ArrayOfObjects == other.ArrayOfObjects
    }
}
impl ::core::cmp::Eq for SCONTEXT_QUEUE {}
impl ::core::default::Default for SCONTEXT_QUEUE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct SEC_WINNT_AUTH_IDENTITY_A {
    pub User: *mut u8,
    pub UserLength: u32,
    pub Domain: *mut u8,
    pub DomainLength: u32,
    pub Password: *mut u8,
    pub PasswordLength: u32,
    pub Flags: SEC_WINNT_AUTH_IDENTITY,
}
impl ::core::marker::Copy for SEC_WINNT_AUTH_IDENTITY_A {}
impl ::core::clone::Clone for SEC_WINNT_AUTH_IDENTITY_A {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SEC_WINNT_AUTH_IDENTITY_A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SEC_WINNT_AUTH_IDENTITY_A").field("User", &self.User).field("UserLength", &self.UserLength).field("Domain", &self.Domain).field("DomainLength", &self.DomainLength).field("Password", &self.Password).field("PasswordLength", &self.PasswordLength).field("Flags", &self.Flags).finish()
    }
}
impl ::windows::core::TypeKind for SEC_WINNT_AUTH_IDENTITY_A {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SEC_WINNT_AUTH_IDENTITY_A {
    fn eq(&self, other: &Self) -> bool {
        self.User == other.User && self.UserLength == other.UserLength && self.Domain == other.Domain && self.DomainLength == other.DomainLength && self.Password == other.Password && self.PasswordLength == other.PasswordLength && self.Flags == other.Flags
    }
}
impl ::core::cmp::Eq for SEC_WINNT_AUTH_IDENTITY_A {}
impl ::core::default::Default for SEC_WINNT_AUTH_IDENTITY_A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct SEC_WINNT_AUTH_IDENTITY_W {
    pub User: *mut u16,
    pub UserLength: u32,
    pub Domain: *mut u16,
    pub DomainLength: u32,
    pub Password: *mut u16,
    pub PasswordLength: u32,
    pub Flags: SEC_WINNT_AUTH_IDENTITY,
}
impl ::core::marker::Copy for SEC_WINNT_AUTH_IDENTITY_W {}
impl ::core::clone::Clone for SEC_WINNT_AUTH_IDENTITY_W {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SEC_WINNT_AUTH_IDENTITY_W {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SEC_WINNT_AUTH_IDENTITY_W").field("User", &self.User).field("UserLength", &self.UserLength).field("Domain", &self.Domain).field("DomainLength", &self.DomainLength).field("Password", &self.Password).field("PasswordLength", &self.PasswordLength).field("Flags", &self.Flags).finish()
    }
}
impl ::windows::core::TypeKind for SEC_WINNT_AUTH_IDENTITY_W {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SEC_WINNT_AUTH_IDENTITY_W {
    fn eq(&self, other: &Self) -> bool {
        self.User == other.User && self.UserLength == other.UserLength && self.Domain == other.Domain && self.DomainLength == other.DomainLength && self.Password == other.Password && self.PasswordLength == other.PasswordLength && self.Flags == other.Flags
    }
}
impl ::core::cmp::Eq for SEC_WINNT_AUTH_IDENTITY_W {}
impl ::core::default::Default for SEC_WINNT_AUTH_IDENTITY_W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub struct USER_MARSHAL_CB {
    pub Flags: u32,
    pub pStubMsg: *mut MIDL_STUB_MESSAGE,
    pub pReserve: *mut u8,
    pub Signature: u32,
    pub CBType: USER_MARSHAL_CB_TYPE,
    pub pFormat: *mut u8,
    pub pTypeFormat: *mut u8,
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::marker::Copy for USER_MARSHAL_CB {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for USER_MARSHAL_CB {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for USER_MARSHAL_CB {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("USER_MARSHAL_CB").field("Flags", &self.Flags).field("pStubMsg", &self.pStubMsg).field("pReserve", &self.pReserve).field("Signature", &self.Signature).field("CBType", &self.CBType).field("pFormat", &self.pFormat).field("pTypeFormat", &self.pTypeFormat).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::TypeKind for USER_MARSHAL_CB {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for USER_MARSHAL_CB {
    fn eq(&self, other: &Self) -> bool {
        self.Flags == other.Flags && self.pStubMsg == other.pStubMsg && self.pReserve == other.pReserve && self.Signature == other.Signature && self.CBType == other.CBType && self.pFormat == other.pFormat && self.pTypeFormat == other.pTypeFormat
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for USER_MARSHAL_CB {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::default::Default for USER_MARSHAL_CB {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct USER_MARSHAL_ROUTINE_QUADRUPLE {
    pub pfnBufferSize: USER_MARSHAL_SIZING_ROUTINE,
    pub pfnMarshall: USER_MARSHAL_MARSHALLING_ROUTINE,
    pub pfnUnmarshall: USER_MARSHAL_UNMARSHALLING_ROUTINE,
    pub pfnFree: USER_MARSHAL_FREEING_ROUTINE,
}
impl ::core::marker::Copy for USER_MARSHAL_ROUTINE_QUADRUPLE {}
impl ::core::clone::Clone for USER_MARSHAL_ROUTINE_QUADRUPLE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for USER_MARSHAL_ROUTINE_QUADRUPLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("USER_MARSHAL_ROUTINE_QUADRUPLE").finish()
    }
}
impl ::windows::core::TypeKind for USER_MARSHAL_ROUTINE_QUADRUPLE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for USER_MARSHAL_ROUTINE_QUADRUPLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct UUID_VECTOR {
    pub Count: u32,
    pub Uuid: [*mut ::windows::core::GUID; 1],
}
impl ::core::marker::Copy for UUID_VECTOR {}
impl ::core::clone::Clone for UUID_VECTOR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for UUID_VECTOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("UUID_VECTOR").field("Count", &self.Count).field("Uuid", &self.Uuid).finish()
    }
}
impl ::windows::core::TypeKind for UUID_VECTOR {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for UUID_VECTOR {
    fn eq(&self, other: &Self) -> bool {
        self.Count == other.Count && self.Uuid == other.Uuid
    }
}
impl ::core::cmp::Eq for UUID_VECTOR {}
impl ::core::default::Default for UUID_VECTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub struct XMIT_ROUTINE_QUINTUPLE {
    pub pfnTranslateToXmit: XMIT_HELPER_ROUTINE,
    pub pfnTranslateFromXmit: XMIT_HELPER_ROUTINE,
    pub pfnFreeXmit: XMIT_HELPER_ROUTINE,
    pub pfnFreeInst: XMIT_HELPER_ROUTINE,
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::marker::Copy for XMIT_ROUTINE_QUINTUPLE {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for XMIT_ROUTINE_QUINTUPLE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for XMIT_ROUTINE_QUINTUPLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("XMIT_ROUTINE_QUINTUPLE").finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::TypeKind for XMIT_ROUTINE_QUINTUPLE {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::default::Default for XMIT_ROUTINE_QUINTUPLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct _NDR_ASYNC_MESSAGE(pub u8);
#[repr(C)]
pub struct _NDR_CORRELATION_INFO(pub u8);
#[repr(C)]
pub struct _NDR_PROC_CONTEXT(pub u8);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub type CS_TAG_GETTING_ROUTINE = ::core::option::Option<unsafe extern "system" fn(hbinding: *mut ::core::ffi::c_void, fserverside: i32, pulsendingtag: *mut u32, puldesiredreceivingtag: *mut u32, pulreceivingtag: *mut u32, pstatus: *mut u32) -> ()>;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub type CS_TYPE_FROM_NETCS_ROUTINE = ::core::option::Option<unsafe extern "system" fn(hbinding: *mut ::core::ffi::c_void, ulnetworkcodeset: u32, pnetworkdata: *mut u8, ulnetworkdatalength: u32, ullocalbuffersize: u32, plocaldata: *mut ::core::ffi::c_void, pullocaldatalength: *mut u32, pstatus: *mut u32) -> ()>;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub type CS_TYPE_LOCAL_SIZE_ROUTINE = ::core::option::Option<unsafe extern "system" fn(hbinding: *mut ::core::ffi::c_void, ulnetworkcodeset: u32, ulnetworkbuffersize: u32, conversiontype: *mut IDL_CS_CONVERT, pullocalbuffersize: *mut u32, pstatus: *mut u32) -> ()>;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub type CS_TYPE_NET_SIZE_ROUTINE = ::core::option::Option<unsafe extern "system" fn(hbinding: *mut ::core::ffi::c_void, ulnetworkcodeset: u32, ullocalbuffersize: u32, conversiontype: *mut IDL_CS_CONVERT, pulnetworkbuffersize: *mut u32, pstatus: *mut u32) -> ()>;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub type CS_TYPE_TO_NETCS_ROUTINE = ::core::option::Option<unsafe extern "system" fn(hbinding: *mut ::core::ffi::c_void, ulnetworkcodeset: u32, plocaldata: *mut ::core::ffi::c_void, ullocaldatalength: u32, pnetworkdata: *mut u8, pulnetworkdatalength: *mut u32, pstatus: *mut u32) -> ()>;
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub type EXPR_EVAL = ::core::option::Option<unsafe extern "system" fn(param0: *mut MIDL_STUB_MESSAGE) -> ()>;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub type GENERIC_BINDING_ROUTINE = ::core::option::Option<unsafe extern "system" fn(param0: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void>;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub type GENERIC_UNBIND_ROUTINE = ::core::option::Option<unsafe extern "system" fn(param0: *mut ::core::ffi::c_void, param1: *mut u8) -> ()>;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub type I_RpcFreeCalloutStateFn = ::core::option::Option<unsafe extern "system" fn(calloutstate: *mut RDR_CALLOUT_STATE) -> ()>;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub type I_RpcPerformCalloutFn = ::core::option::Option<unsafe extern "system" fn(context: *mut ::core::ffi::c_void, calloutstate: *mut RDR_CALLOUT_STATE, stage: RPC_HTTP_REDIRECTOR_STAGE) -> RPC_STATUS>;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub type I_RpcProxyFilterIfFn = ::core::option::Option<unsafe extern "system" fn(context: *const ::core::ffi::c_void, ifuuid: *const ::windows::core::GUID, ifmajorversion: u16, fallow: *mut i32) -> RPC_STATUS>;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub type I_RpcProxyGetClientAddressFn = ::core::option::Option<unsafe extern "system" fn(context: *mut ::core::ffi::c_void, buffer: ::windows::core::PCSTR, bufferlength: *mut u32) -> RPC_STATUS>;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub type I_RpcProxyGetClientSessionAndResourceUUID = ::core::option::Option<unsafe extern "system" fn(context: *const ::core::ffi::c_void, sessionidpresent: *mut i32, sessionid: *mut ::windows::core::GUID, resourceidpresent: *mut i32, resourceid: *mut ::windows::core::GUID) -> RPC_STATUS>;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub type I_RpcProxyGetConnectionTimeoutFn = ::core::option::Option<unsafe extern "system" fn(connectiontimeout: *mut u32) -> RPC_STATUS>;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub type I_RpcProxyIsValidMachineFn = ::core::option::Option<unsafe extern "system" fn(machine: *const u16, dotmachine: *const u16, portnumber: u32) -> RPC_STATUS>;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub type I_RpcProxyUpdatePerfCounterBackendServerFn = ::core::option::Option<unsafe extern "system" fn(machinename: *const u16, isconnectevent: i32) -> ()>;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub type I_RpcProxyUpdatePerfCounterFn = ::core::option::Option<unsafe extern "system" fn(counter: RpcPerfCounters, modifytrend: i32, size: u32) -> ()>;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub type MIDL_ES_ALLOC = ::core::option::Option<unsafe extern "system" fn(state: *mut ::core::ffi::c_void, pbuffer: *mut *mut i8, psize: *mut u32) -> ()>;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub type MIDL_ES_READ = ::core::option::Option<unsafe extern "system" fn(state: *mut ::core::ffi::c_void, pbuffer: *mut *mut i8, psize: *mut u32) -> ()>;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub type MIDL_ES_WRITE = ::core::option::Option<unsafe extern "system" fn(state: *mut ::core::ffi::c_void, buffer: ::windows::core::PCSTR, size: u32) -> ()>;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub type NDR_NOTIFY2_ROUTINE = ::core::option::Option<unsafe extern "system" fn(flag: u8) -> ()>;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub type NDR_NOTIFY_ROUTINE = ::core::option::Option<unsafe extern "system" fn() -> ()>;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub type NDR_RUNDOWN = ::core::option::Option<unsafe extern "system" fn(context: *mut ::core::ffi::c_void) -> ()>;
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PFN_RPCNOTIFICATION_ROUTINE = ::core::option::Option<unsafe extern "system" fn(pasync: *mut RPC_ASYNC_STATE, context: *mut ::core::ffi::c_void, event: RPC_ASYNC_EVENT) -> ()>;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub type PRPC_RUNDOWN = ::core::option::Option<unsafe extern "system" fn(associationcontext: *mut ::core::ffi::c_void) -> ()>;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub type RPCLT_PDU_FILTER_FUNC = ::core::option::Option<unsafe extern "system" fn(buffer: *mut ::core::ffi::c_void, bufferlength: u32, fdatagram: i32) -> ()>;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub type RPC_ADDRESS_CHANGE_FN = ::core::option::Option<unsafe extern "system" fn(arg: *mut ::core::ffi::c_void) -> ()>;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub type RPC_AUTH_KEY_RETRIEVAL_FN = ::core::option::Option<unsafe extern "system" fn(arg: *const ::core::ffi::c_void, serverprincname: *const u16, keyver: u32, key: *mut *mut ::core::ffi::c_void, status: *mut RPC_STATUS) -> ()>;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub type RPC_BLOCKING_FN = ::core::option::Option<unsafe extern "system" fn(hwnd: *mut ::core::ffi::c_void, context: *mut ::core::ffi::c_void, hsyncevent: *mut ::core::ffi::c_void) -> RPC_STATUS>;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub type RPC_CLIENT_ALLOC = ::core::option::Option<unsafe extern "system" fn(size: usize) -> *mut ::core::ffi::c_void>;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub type RPC_CLIENT_FREE = ::core::option::Option<unsafe extern "system" fn(ptr: *const ::core::ffi::c_void) -> ()>;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub type RPC_DISPATCH_FUNCTION = ::core::option::Option<unsafe extern "system" fn(message: *mut RPC_MESSAGE) -> ()>;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub type RPC_FORWARD_FUNCTION = ::core::option::Option<unsafe extern "system" fn(interfaceid: *mut ::windows::core::GUID, interfaceversion: *mut RPC_VERSION, objectid: *mut ::windows::core::GUID, rpcpro: *mut u8, ppdestendpoint: *mut *mut ::core::ffi::c_void) -> RPC_STATUS>;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub type RPC_HTTP_PROXY_FREE_STRING = ::core::option::Option<unsafe extern "system" fn(string: *const u16) -> ()>;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub type RPC_IF_CALLBACK_FN = ::core::option::Option<unsafe extern "system" fn(interfaceuuid: *const ::core::ffi::c_void, context: *const ::core::ffi::c_void) -> RPC_STATUS>;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub type RPC_INTERFACE_GROUP_IDLE_CALLBACK_FN = ::core::option::Option<unsafe extern "system" fn(ifgroup: *const ::core::ffi::c_void, idlecallbackcontext: *const ::core::ffi::c_void, isgroupidle: u32) -> ()>;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub type RPC_MGMT_AUTHORIZATION_FN = ::core::option::Option<unsafe extern "system" fn(clientbinding: *const ::core::ffi::c_void, requestedmgmtoperation: u32, status: *mut RPC_STATUS) -> i32>;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub type RPC_NEW_HTTP_PROXY_CHANNEL = ::core::option::Option<unsafe extern "system" fn(redirectorstage: RPC_HTTP_REDIRECTOR_STAGE, servername: *const u16, serverport: *const u16, remoteuser: *const u16, authtype: *const u16, resourceuuid: *mut ::core::ffi::c_void, sessionid: *mut ::core::ffi::c_void, interface: *const ::core::ffi::c_void, reserved: *const ::core::ffi::c_void, flags: u32, newservername: *mut *mut u16, newserverport: *mut *mut u16) -> RPC_STATUS>;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub type RPC_OBJECT_INQ_FN = ::core::option::Option<unsafe extern "system" fn(objectuuid: *const ::windows::core::GUID, typeuuid: *mut ::windows::core::GUID, status: *mut RPC_STATUS) -> ()>;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub type RPC_SECURITY_CALLBACK_FN = ::core::option::Option<unsafe extern "system" fn(context: *const ::core::ffi::c_void) -> ()>;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub type RPC_SETFILTER_FUNC = ::core::option::Option<unsafe extern "system" fn(pfnfilter: RPCLT_PDU_FILTER_FUNC) -> ()>;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub type SERVER_ROUTINE = ::core::option::Option<unsafe extern "system" fn() -> i32>;
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub type STUB_THUNK = ::core::option::Option<unsafe extern "system" fn(param0: *mut MIDL_STUB_MESSAGE) -> ()>;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub type USER_MARSHAL_FREEING_ROUTINE = ::core::option::Option<unsafe extern "system" fn(param0: *mut u32, param1: *mut ::core::ffi::c_void) -> ()>;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub type USER_MARSHAL_MARSHALLING_ROUTINE = ::core::option::Option<unsafe extern "system" fn(param0: *mut u32, param1: *mut u8, param2: *mut ::core::ffi::c_void) -> *mut u8>;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub type USER_MARSHAL_SIZING_ROUTINE = ::core::option::Option<unsafe extern "system" fn(param0: *mut u32, param1: u32, param2: *mut ::core::ffi::c_void) -> u32>;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub type USER_MARSHAL_UNMARSHALLING_ROUTINE = ::core::option::Option<unsafe extern "system" fn(param0: *mut u32, param1: *mut u8, param2: *mut ::core::ffi::c_void) -> *mut u8>;
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub type XMIT_HELPER_ROUTINE = ::core::option::Option<unsafe extern "system" fn(param0: *mut MIDL_STUB_MESSAGE) -> ()>;
#[cfg(feature = "implement")]
::core::include!("impl.rs");
