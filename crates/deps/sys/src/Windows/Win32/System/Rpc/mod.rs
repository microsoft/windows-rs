#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn DceErrorInqTextA(rpcstatus: RPC_STATUS, errortext: *mut u8) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn DceErrorInqTextW(rpcstatus: RPC_STATUS, errortext: *mut u16) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn IUnknown_AddRef_Proxy(this: ::windows_sys::core::IUnknown) -> u32;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn IUnknown_QueryInterface_Proxy(this: ::windows_sys::core::IUnknown, riid: *const ::windows_sys::core::GUID, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn IUnknown_Release_Proxy(this: ::windows_sys::core::IUnknown) -> u32;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn I_RpcAllocate(size: u32) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn I_RpcAsyncAbortCall(pasync: *const RPC_ASYNC_STATE, exceptioncode: u32) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn I_RpcAsyncSetHandle(message: *const RPC_MESSAGE, pasync: *const RPC_ASYNC_STATE) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn I_RpcBindingCopy(sourcebinding: *mut ::core::ffi::c_void, destinationbinding: *mut *mut ::core::ffi::c_void) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn I_RpcBindingCreateNP(servername: *const u16, servicename: *const u16, networkoptions: *const u16, binding: *mut *mut ::core::ffi::c_void) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn I_RpcBindingHandleToAsyncHandle(binding: *mut ::core::ffi::c_void, asynchandle: *mut *mut ::core::ffi::c_void) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn I_RpcBindingInqClientTokenAttributes(binding: *const ::core::ffi::c_void, tokenid: *mut super::super::Foundation::LUID, authenticationid: *mut super::super::Foundation::LUID, modifiedid: *mut super::super::Foundation::LUID) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn I_RpcBindingInqDynamicEndpointA(binding: *const ::core::ffi::c_void, dynamicendpoint: *mut *mut u8) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn I_RpcBindingInqDynamicEndpointW(binding: *const ::core::ffi::c_void, dynamicendpoint: *mut *mut u16) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn I_RpcBindingInqLocalClientPID(binding: *mut ::core::ffi::c_void, pid: *mut u32) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn I_RpcBindingInqMarshalledTargetInfo(binding: *const ::core::ffi::c_void, marshalledtargetinfosize: *mut u32, marshalledtargetinfo: *mut *mut u8) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn I_RpcBindingInqSecurityContext(binding: *mut ::core::ffi::c_void, securitycontexthandle: *mut *mut ::core::ffi::c_void) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn I_RpcBindingInqSecurityContextKeyInfo(binding: *const ::core::ffi::c_void, keyinfo: *mut ::core::ffi::c_void) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn I_RpcBindingInqTransportType(binding: *mut ::core::ffi::c_void, r#type: *mut u32) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn I_RpcBindingInqWireIdForSnego(binding: *const ::core::ffi::c_void, wireid: *mut u8) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn I_RpcBindingIsClientLocal(bindinghandle: *mut ::core::ffi::c_void, clientlocalflag: *mut u32) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn I_RpcBindingIsServerLocal(binding: *const ::core::ffi::c_void, serverlocalflag: *mut u32) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn I_RpcBindingSetPrivateOption(hbinding: *const ::core::ffi::c_void, option: u32, optionvalue: usize) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn I_RpcBindingToStaticStringBindingW(binding: *mut ::core::ffi::c_void, stringbinding: *mut *mut u16) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn I_RpcClearMutex(mutex: *mut ::core::ffi::c_void);
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn I_RpcDeleteMutex(mutex: *mut ::core::ffi::c_void);
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn I_RpcExceptionFilter(exceptioncode: u32) -> i32;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn I_RpcFree(object: *mut ::core::ffi::c_void);
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn I_RpcFreeBuffer(message: *mut RPC_MESSAGE) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn I_RpcFreePipeBuffer(message: *mut RPC_MESSAGE) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn I_RpcGetBuffer(message: *mut RPC_MESSAGE) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn I_RpcGetBufferWithObject(message: *mut RPC_MESSAGE, objectuuid: *mut ::windows_sys::core::GUID) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn I_RpcGetCurrentCallHandle() -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn I_RpcGetDefaultSD(ppsecuritydescriptor: *mut *mut ::core::ffi::c_void) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn I_RpcGetExtendedError() -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn I_RpcIfInqTransferSyntaxes(rpcifhandle: *mut ::core::ffi::c_void, transfersyntaxes: *mut RPC_TRANSFER_SYNTAX, transfersyntaxsize: u32, transfersyntaxcount: *mut u32) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn I_RpcMapWin32Status(status: RPC_STATUS) -> i32;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn I_RpcMgmtEnableDedicatedThreadPool() -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn I_RpcNegotiateTransferSyntax(message: *mut RPC_MESSAGE) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn I_RpcNsBindingSetEntryNameA(binding: *const ::core::ffi::c_void, entrynamesyntax: u32, entryname: *const u8) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn I_RpcNsBindingSetEntryNameW(binding: *const ::core::ffi::c_void, entrynamesyntax: u32, entryname: *const u16) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn I_RpcNsGetBuffer(message: *mut RPC_MESSAGE) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn I_RpcNsInterfaceExported(entrynamesyntax: u32, entryname: *mut u16, rpcinterfaceinformation: *mut RPC_SERVER_INTERFACE) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn I_RpcNsInterfaceUnexported(entrynamesyntax: u32, entryname: *mut u16, rpcinterfaceinformation: *mut RPC_SERVER_INTERFACE) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn I_RpcNsRaiseException(message: *mut RPC_MESSAGE, status: RPC_STATUS);
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn I_RpcNsSendReceive(message: *mut RPC_MESSAGE, handle: *mut *mut ::core::ffi::c_void) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn I_RpcOpenClientProcess(binding: *const ::core::ffi::c_void, desiredaccess: u32, clientprocess: *mut *mut ::core::ffi::c_void) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn I_RpcPauseExecution(milliseconds: u32);
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn I_RpcReBindBuffer(message: *mut RPC_MESSAGE) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn I_RpcReallocPipeBuffer(message: *const RPC_MESSAGE, newsize: u32) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn I_RpcReceive(message: *mut RPC_MESSAGE, size: u32) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn I_RpcRecordCalloutFailure(rpcstatus: RPC_STATUS, calloutstate: *mut RDR_CALLOUT_STATE, dllname: *mut u16);
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn I_RpcRequestMutex(mutex: *mut *mut ::core::ffi::c_void);
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn I_RpcSend(message: *mut RPC_MESSAGE) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn I_RpcSendReceive(message: *mut RPC_MESSAGE) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn I_RpcServerCheckClientRestriction(context: *mut ::core::ffi::c_void) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn I_RpcServerDisableExceptionFilter() -> i32;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn I_RpcServerGetAssociationID(binding: *const ::core::ffi::c_void, associationid: *mut u32) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn I_RpcServerInqAddressChangeFn() -> *mut ::core::option::Option<RPC_ADDRESS_CHANGE_FN>;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn I_RpcServerInqLocalConnAddress(binding: *mut ::core::ffi::c_void, buffer: *mut ::core::ffi::c_void, buffersize: *mut u32, addressformat: *mut u32) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn I_RpcServerInqRemoteConnAddress(binding: *mut ::core::ffi::c_void, buffer: *mut ::core::ffi::c_void, buffersize: *mut u32, addressformat: *mut u32) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn I_RpcServerInqTransportType(r#type: *mut u32) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn I_RpcServerRegisterForwardFunction(pforwardfunction: *mut RPC_FORWARD_FUNCTION) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn I_RpcServerSetAddressChangeFn(paddresschangefn: *mut RPC_ADDRESS_CHANGE_FN) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn I_RpcServerStartService(protseq: *const u16, endpoint: *const u16, ifspec: *const ::core::ffi::c_void) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn I_RpcServerSubscribeForDisconnectNotification(binding: *const ::core::ffi::c_void, hevent: *const ::core::ffi::c_void) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn I_RpcServerSubscribeForDisconnectNotification2(binding: *const ::core::ffi::c_void, hevent: *const ::core::ffi::c_void, subscriptionid: *mut ::windows_sys::core::GUID) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn I_RpcServerUnsubscribeForDisconnectNotification(binding: *const ::core::ffi::c_void, subscriptionid: ::windows_sys::core::GUID) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn I_RpcServerUseProtseq2A(networkaddress: *const u8, protseq: *const u8, maxcalls: u32, securitydescriptor: *const ::core::ffi::c_void, policy: *const ::core::ffi::c_void) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn I_RpcServerUseProtseq2W(networkaddress: *const u16, protseq: *const u16, maxcalls: u32, securitydescriptor: *const ::core::ffi::c_void, policy: *const ::core::ffi::c_void) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn I_RpcServerUseProtseqEp2A(networkaddress: *const u8, protseq: *const u8, maxcalls: u32, endpoint: *const u8, securitydescriptor: *const ::core::ffi::c_void, policy: *const ::core::ffi::c_void) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn I_RpcServerUseProtseqEp2W(networkaddress: *const u16, protseq: *const u16, maxcalls: u32, endpoint: *const u16, securitydescriptor: *const ::core::ffi::c_void, policy: *const ::core::ffi::c_void) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn I_RpcSessionStrictContextHandle();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn I_RpcSsDontSerializeContext();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn I_RpcSystemHandleTypeSpecificWork(handle: *mut ::core::ffi::c_void, actualtype: u8, idltype: u8, marshaldirection: LRPC_SYSTEM_HANDLE_MARSHAL_DIRECTION) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn I_RpcTurnOnEEInfoPropagation() -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn I_UuidCreate(uuid: *mut ::windows_sys::core::GUID) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn MesBufferHandleReset(handle: *const ::core::ffi::c_void, handlestyle: u32, operation: MIDL_ES_CODE, pbuffer: *const *const i8, buffersize: u32, pencodedsize: *mut u32) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MesDecodeBufferHandleCreate(buffer: super::super::Foundation::PSTR, buffersize: u32, phandle: *mut *mut ::core::ffi::c_void) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn MesDecodeIncrementalHandleCreate(userstate: *mut ::core::ffi::c_void, readfn: MIDL_ES_READ, phandle: *mut *mut ::core::ffi::c_void) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn MesEncodeDynBufferHandleCreate(pbuffer: *mut *mut i8, pencodedsize: *mut u32, phandle: *mut *mut ::core::ffi::c_void) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MesEncodeFixedBufferHandleCreate(pbuffer: super::super::Foundation::PSTR, buffersize: u32, pencodedsize: *mut u32, phandle: *mut *mut ::core::ffi::c_void) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MesEncodeIncrementalHandleCreate(userstate: *mut ::core::ffi::c_void, allocfn: MIDL_ES_ALLOC, writefn: MIDL_ES_WRITE, phandle: *mut *mut ::core::ffi::c_void) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn MesHandleFree(handle: *mut ::core::ffi::c_void) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MesIncrementalHandleReset(handle: *mut ::core::ffi::c_void, userstate: *mut ::core::ffi::c_void, allocfn: MIDL_ES_ALLOC, writefn: MIDL_ES_WRITE, readfn: MIDL_ES_READ, operation: MIDL_ES_CODE) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn MesInqProcEncodingId(handle: *mut ::core::ffi::c_void, pinterfaceid: *mut RPC_SYNTAX_IDENTIFIER, pprocnum: *mut u32) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn NDRCContextBinding(ccontext: isize) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn NDRCContextMarshall(ccontext: isize, pbuff: *mut ::core::ffi::c_void);
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn NDRCContextUnmarshall(pccontext: *mut isize, hbinding: *const ::core::ffi::c_void, pbuff: *const ::core::ffi::c_void, datarepresentation: u32);
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn NDRSContextMarshall(ccontext: *const NDR_SCONTEXT_1, pbuff: *mut ::core::ffi::c_void, userrundownin: NDR_RUNDOWN);
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn NDRSContextMarshall2(bindinghandle: *const ::core::ffi::c_void, ccontext: *const NDR_SCONTEXT_1, pbuff: *mut ::core::ffi::c_void, userrundownin: NDR_RUNDOWN, ctxguard: *const ::core::ffi::c_void, flags: u32);
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn NDRSContextMarshallEx(bindinghandle: *const ::core::ffi::c_void, ccontext: *const NDR_SCONTEXT_1, pbuff: *mut ::core::ffi::c_void, userrundownin: NDR_RUNDOWN);
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn NDRSContextUnmarshall(pbuff: *const ::core::ffi::c_void, datarepresentation: u32) -> *mut NDR_SCONTEXT_1;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn NDRSContextUnmarshall2(bindinghandle: *const ::core::ffi::c_void, pbuff: *const ::core::ffi::c_void, datarepresentation: u32, ctxguard: *const ::core::ffi::c_void, flags: u32) -> *mut NDR_SCONTEXT_1;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn NDRSContextUnmarshallEx(bindinghandle: *const ::core::ffi::c_void, pbuff: *const ::core::ffi::c_void, datarepresentation: u32) -> *mut NDR_SCONTEXT_1;
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn Ndr64AsyncClientCall(pproxyinfo: *mut MIDL_STUBLESS_PROXY_INFO, nprocnum: u32, preturnvalue: *mut ::core::ffi::c_void) -> CLIENT_CALL_RETURN;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn Ndr64AsyncServerCall64(prpcmsg: *mut RPC_MESSAGE);
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn Ndr64AsyncServerCallAll(prpcmsg: *mut RPC_MESSAGE);
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn Ndr64DcomAsyncClientCall(pproxyinfo: *mut MIDL_STUBLESS_PROXY_INFO, nprocnum: u32, preturnvalue: *mut ::core::ffi::c_void) -> CLIENT_CALL_RETURN;
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn Ndr64DcomAsyncStubCall(pthis: super::Com::IRpcStubBuffer, pchannel: super::Com::IRpcChannelBuffer, prpcmsg: *mut RPC_MESSAGE, pdwstubphase: *mut u32) -> i32;
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrAllocate(pstubmsg: *mut MIDL_STUB_MESSAGE, len: usize) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrAsyncClientCall(pstubdescriptor: *mut MIDL_STUB_DESC, pformat: *mut u8) -> CLIENT_CALL_RETURN;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn NdrAsyncServerCall(prpcmsg: *mut RPC_MESSAGE);
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrByteCountPointerBufferSize(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8);
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrByteCountPointerFree(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8);
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrByteCountPointerMarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) -> *mut u8;
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrByteCountPointerUnmarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, ppmemory: *mut *mut u8, pformat: *mut u8, fmustalloc: u8) -> *mut u8;
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrClearOutParameters(pstubmsg: *mut MIDL_STUB_MESSAGE, pformat: *mut u8, argaddr: *mut ::core::ffi::c_void);
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrClientCall2(pstubdescriptor: *mut MIDL_STUB_DESC, pformat: *mut u8) -> CLIENT_CALL_RETURN;
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrClientCall3(pproxyinfo: *mut MIDL_STUBLESS_PROXY_INFO, nprocnum: u32, preturnvalue: *mut ::core::ffi::c_void) -> CLIENT_CALL_RETURN;
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrClientContextMarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, contexthandle: isize, fcheck: i32);
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrClientContextUnmarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, pcontexthandle: *mut isize, bindhandle: *mut ::core::ffi::c_void);
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrClientInitialize(prpcmsg: *mut RPC_MESSAGE, pstubmsg: *mut MIDL_STUB_MESSAGE, pstubdescriptor: *mut MIDL_STUB_DESC, procnum: u32);
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrClientInitializeNew(prpcmsg: *mut RPC_MESSAGE, pstubmsg: *mut MIDL_STUB_MESSAGE, pstubdescriptor: *mut MIDL_STUB_DESC, procnum: u32);
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrComplexArrayBufferSize(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8);
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrComplexArrayFree(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8);
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrComplexArrayMarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) -> *mut u8;
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrComplexArrayMemorySize(pstubmsg: *mut MIDL_STUB_MESSAGE, pformat: *mut u8) -> u32;
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrComplexArrayUnmarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, ppmemory: *mut *mut u8, pformat: *mut u8, fmustalloc: u8) -> *mut u8;
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrComplexStructBufferSize(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8);
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrComplexStructFree(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8);
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrComplexStructMarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) -> *mut u8;
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrComplexStructMemorySize(pstubmsg: *mut MIDL_STUB_MESSAGE, pformat: *mut u8) -> u32;
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrComplexStructUnmarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, ppmemory: *mut *mut u8, pformat: *mut u8, fmustalloc: u8) -> *mut u8;
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrConformantArrayBufferSize(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8);
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrConformantArrayFree(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8);
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrConformantArrayMarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) -> *mut u8;
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrConformantArrayMemorySize(pstubmsg: *mut MIDL_STUB_MESSAGE, pformat: *mut u8) -> u32;
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrConformantArrayUnmarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, ppmemory: *mut *mut u8, pformat: *mut u8, fmustalloc: u8) -> *mut u8;
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrConformantStringBufferSize(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8);
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrConformantStringMarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) -> *mut u8;
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrConformantStringMemorySize(pstubmsg: *mut MIDL_STUB_MESSAGE, pformat: *mut u8) -> u32;
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrConformantStringUnmarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, ppmemory: *mut *mut u8, pformat: *mut u8, fmustalloc: u8) -> *mut u8;
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrConformantStructBufferSize(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8);
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrConformantStructFree(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8);
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrConformantStructMarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) -> *mut u8;
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrConformantStructMemorySize(pstubmsg: *mut MIDL_STUB_MESSAGE, pformat: *mut u8) -> u32;
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrConformantStructUnmarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, ppmemory: *mut *mut u8, pformat: *mut u8, fmustalloc: u8) -> *mut u8;
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrConformantVaryingArrayBufferSize(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8);
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrConformantVaryingArrayFree(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8);
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrConformantVaryingArrayMarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) -> *mut u8;
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrConformantVaryingArrayMemorySize(pstubmsg: *mut MIDL_STUB_MESSAGE, pformat: *mut u8) -> u32;
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrConformantVaryingArrayUnmarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, ppmemory: *mut *mut u8, pformat: *mut u8, fmustalloc: u8) -> *mut u8;
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrConformantVaryingStructBufferSize(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8);
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrConformantVaryingStructFree(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8);
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrConformantVaryingStructMarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) -> *mut u8;
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrConformantVaryingStructMemorySize(pstubmsg: *mut MIDL_STUB_MESSAGE, pformat: *mut u8) -> u32;
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrConformantVaryingStructUnmarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, ppmemory: *mut *mut u8, pformat: *mut u8, fmustalloc: u8) -> *mut u8;
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrContextHandleInitialize(pstubmsg: *const MIDL_STUB_MESSAGE, pformat: *const u8) -> *mut NDR_SCONTEXT_1;
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrContextHandleSize(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8);
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrConvert(pstubmsg: *mut MIDL_STUB_MESSAGE, pformat: *mut u8);
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrConvert2(pstubmsg: *mut MIDL_STUB_MESSAGE, pformat: *mut u8, numberparams: i32);
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrCorrelationFree(pstubmsg: *mut MIDL_STUB_MESSAGE);
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrCorrelationInitialize(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut ::core::ffi::c_void, cachesize: u32, flags: u32);
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrCorrelationPass(pstubmsg: *mut MIDL_STUB_MESSAGE);
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrCreateServerInterfaceFromStub(pstub: super::Com::IRpcStubBuffer, pserverif: *mut RPC_SERVER_INTERFACE) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrDcomAsyncClientCall(pstubdescriptor: *mut MIDL_STUB_DESC, pformat: *mut u8) -> CLIENT_CALL_RETURN;
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrDcomAsyncStubCall(pthis: super::Com::IRpcStubBuffer, pchannel: super::Com::IRpcChannelBuffer, prpcmsg: *mut RPC_MESSAGE, pdwstubphase: *mut u32) -> i32;
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrEncapsulatedUnionBufferSize(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8);
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrEncapsulatedUnionFree(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8);
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrEncapsulatedUnionMarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) -> *mut u8;
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrEncapsulatedUnionMemorySize(pstubmsg: *mut MIDL_STUB_MESSAGE, pformat: *mut u8) -> u32;
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrEncapsulatedUnionUnmarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, ppmemory: *mut *mut u8, pformat: *mut u8, fmustalloc: u8) -> *mut u8;
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrFixedArrayBufferSize(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8);
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrFixedArrayFree(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8);
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrFixedArrayMarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) -> *mut u8;
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrFixedArrayMemorySize(pstubmsg: *mut MIDL_STUB_MESSAGE, pformat: *mut u8) -> u32;
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrFixedArrayUnmarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, ppmemory: *mut *mut u8, pformat: *mut u8, fmustalloc: u8) -> *mut u8;
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrFreeBuffer(pstubmsg: *mut MIDL_STUB_MESSAGE);
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn NdrFullPointerXlatFree(pxlattables: *mut FULL_PTR_XLAT_TABLES);
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn NdrFullPointerXlatInit(numberofpointers: u32, xlatside: XLAT_SIDE) -> *mut FULL_PTR_XLAT_TABLES;
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrGetBuffer(pstubmsg: *mut MIDL_STUB_MESSAGE, bufferlength: u32, handle: *mut ::core::ffi::c_void) -> *mut u8;
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrGetDcomProtocolVersion(pstubmsg: *mut MIDL_STUB_MESSAGE, pversion: *mut RPC_VERSION) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrGetUserMarshalInfo(pflags: *const u32, informationlevel: u32, pmarshalinfo: *mut NDR_USER_MARSHAL_INFO) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrInterfacePointerBufferSize(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8);
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrInterfacePointerFree(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8);
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrInterfacePointerMarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) -> *mut u8;
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrInterfacePointerMemorySize(pstubmsg: *mut MIDL_STUB_MESSAGE, pformat: *mut u8) -> u32;
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrInterfacePointerUnmarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, ppmemory: *mut *mut u8, pformat: *mut u8, fmustalloc: u8) -> *mut u8;
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrMapCommAndFaultStatus(pstubmsg: *mut MIDL_STUB_MESSAGE, pcommstatus: *mut u32, pfaultstatus: *mut u32, status: RPC_STATUS) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrMesProcEncodeDecode(handle: *mut ::core::ffi::c_void, pstubdesc: *const MIDL_STUB_DESC, pformatstring: *mut u8);
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrMesProcEncodeDecode2(handle: *mut ::core::ffi::c_void, pstubdesc: *const MIDL_STUB_DESC, pformatstring: *mut u8) -> CLIENT_CALL_RETURN;
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrMesProcEncodeDecode3(handle: *mut ::core::ffi::c_void, pproxyinfo: *const MIDL_STUBLESS_PROXY_INFO, nprocnum: u32, preturnvalue: *mut ::core::ffi::c_void) -> CLIENT_CALL_RETURN;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn NdrMesSimpleTypeAlignSize(param0: *mut ::core::ffi::c_void) -> usize;
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrMesSimpleTypeAlignSizeAll(handle: *mut ::core::ffi::c_void, pproxyinfo: *const MIDL_STUBLESS_PROXY_INFO) -> usize;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn NdrMesSimpleTypeDecode(handle: *mut ::core::ffi::c_void, pobject: *mut ::core::ffi::c_void, size: i16);
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrMesSimpleTypeDecodeAll(handle: *mut ::core::ffi::c_void, pproxyinfo: *const MIDL_STUBLESS_PROXY_INFO, pobject: *mut ::core::ffi::c_void, size: i16);
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrMesSimpleTypeEncode(handle: *mut ::core::ffi::c_void, pstubdesc: *const MIDL_STUB_DESC, pobject: *const ::core::ffi::c_void, size: i16);
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrMesSimpleTypeEncodeAll(handle: *mut ::core::ffi::c_void, pproxyinfo: *const MIDL_STUBLESS_PROXY_INFO, pobject: *const ::core::ffi::c_void, size: i16);
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrMesTypeAlignSize(handle: *mut ::core::ffi::c_void, pstubdesc: *const MIDL_STUB_DESC, pformatstring: *mut u8, pobject: *const ::core::ffi::c_void) -> usize;
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrMesTypeAlignSize2(handle: *mut ::core::ffi::c_void, ppicklinginfo: *const MIDL_TYPE_PICKLING_INFO, pstubdesc: *const MIDL_STUB_DESC, pformatstring: *mut u8, pobject: *const ::core::ffi::c_void) -> usize;
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrMesTypeAlignSize3(handle: *mut ::core::ffi::c_void, ppicklinginfo: *const MIDL_TYPE_PICKLING_INFO, pproxyinfo: *const MIDL_STUBLESS_PROXY_INFO, arrtypeoffset: *const *const u32, ntypeindex: u32, pobject: *const ::core::ffi::c_void) -> usize;
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrMesTypeDecode(handle: *mut ::core::ffi::c_void, pstubdesc: *const MIDL_STUB_DESC, pformatstring: *mut u8, pobject: *mut ::core::ffi::c_void);
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrMesTypeDecode2(handle: *mut ::core::ffi::c_void, ppicklinginfo: *const MIDL_TYPE_PICKLING_INFO, pstubdesc: *const MIDL_STUB_DESC, pformatstring: *mut u8, pobject: *mut ::core::ffi::c_void);
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrMesTypeDecode3(handle: *mut ::core::ffi::c_void, ppicklinginfo: *const MIDL_TYPE_PICKLING_INFO, pproxyinfo: *const MIDL_STUBLESS_PROXY_INFO, arrtypeoffset: *const *const u32, ntypeindex: u32, pobject: *mut ::core::ffi::c_void);
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrMesTypeEncode(handle: *mut ::core::ffi::c_void, pstubdesc: *const MIDL_STUB_DESC, pformatstring: *mut u8, pobject: *const ::core::ffi::c_void);
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrMesTypeEncode2(handle: *mut ::core::ffi::c_void, ppicklinginfo: *const MIDL_TYPE_PICKLING_INFO, pstubdesc: *const MIDL_STUB_DESC, pformatstring: *mut u8, pobject: *const ::core::ffi::c_void);
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrMesTypeEncode3(handle: *mut ::core::ffi::c_void, ppicklinginfo: *const MIDL_TYPE_PICKLING_INFO, pproxyinfo: *const MIDL_STUBLESS_PROXY_INFO, arrtypeoffset: *const *const u32, ntypeindex: u32, pobject: *const ::core::ffi::c_void);
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrMesTypeFree2(handle: *mut ::core::ffi::c_void, ppicklinginfo: *const MIDL_TYPE_PICKLING_INFO, pstubdesc: *const MIDL_STUB_DESC, pformatstring: *mut u8, pobject: *mut ::core::ffi::c_void);
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrMesTypeFree3(handle: *mut ::core::ffi::c_void, ppicklinginfo: *const MIDL_TYPE_PICKLING_INFO, pproxyinfo: *const MIDL_STUBLESS_PROXY_INFO, arrtypeoffset: *const *const u32, ntypeindex: u32, pobject: *mut ::core::ffi::c_void);
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrNonConformantStringBufferSize(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8);
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrNonConformantStringMarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) -> *mut u8;
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrNonConformantStringMemorySize(pstubmsg: *mut MIDL_STUB_MESSAGE, pformat: *mut u8) -> u32;
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrNonConformantStringUnmarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, ppmemory: *mut *mut u8, pformat: *mut u8, fmustalloc: u8) -> *mut u8;
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrNonEncapsulatedUnionBufferSize(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8);
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrNonEncapsulatedUnionFree(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8);
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrNonEncapsulatedUnionMarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) -> *mut u8;
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrNonEncapsulatedUnionMemorySize(pstubmsg: *mut MIDL_STUB_MESSAGE, pformat: *mut u8) -> u32;
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrNonEncapsulatedUnionUnmarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, ppmemory: *mut *mut u8, pformat: *mut u8, fmustalloc: u8) -> *mut u8;
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrNsGetBuffer(pstubmsg: *mut MIDL_STUB_MESSAGE, bufferlength: u32, handle: *mut ::core::ffi::c_void) -> *mut u8;
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrNsSendReceive(pstubmsg: *mut MIDL_STUB_MESSAGE, pbufferend: *mut u8, pautohandle: *mut *mut ::core::ffi::c_void) -> *mut u8;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn NdrOleAllocate(size: usize) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn NdrOleFree(nodetofree: *const ::core::ffi::c_void);
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrPartialIgnoreClientBufferSize(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut ::core::ffi::c_void);
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrPartialIgnoreClientMarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut ::core::ffi::c_void);
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrPartialIgnoreServerInitialize(pstubmsg: *mut MIDL_STUB_MESSAGE, ppmemory: *mut *mut ::core::ffi::c_void, pformat: *mut u8);
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrPartialIgnoreServerUnmarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, ppmemory: *mut *mut ::core::ffi::c_void);
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrPointerBufferSize(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8);
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrPointerFree(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8);
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrPointerMarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) -> *mut u8;
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrPointerMemorySize(pstubmsg: *mut MIDL_STUB_MESSAGE, pformat: *mut u8) -> u32;
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrPointerUnmarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, ppmemory: *mut *mut u8, pformat: *mut u8, fmustalloc: u8) -> *mut u8;
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrRangeUnmarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, ppmemory: *mut *mut u8, pformat: *mut u8, fmustalloc: u8) -> *mut u8;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn NdrRpcSmClientAllocate(size: usize) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn NdrRpcSmClientFree(nodetofree: *const ::core::ffi::c_void);
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrRpcSmSetClientToOsf(pmessage: *mut MIDL_STUB_MESSAGE);
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn NdrRpcSsDefaultAllocate(size: usize) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn NdrRpcSsDefaultFree(nodetofree: *const ::core::ffi::c_void);
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrRpcSsDisableAllocate(pmessage: *mut MIDL_STUB_MESSAGE);
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrRpcSsEnableAllocate(pmessage: *mut MIDL_STUB_MESSAGE);
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrSendReceive(pstubmsg: *mut MIDL_STUB_MESSAGE, pbufferend: *mut u8) -> *mut u8;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn NdrServerCall2(prpcmsg: *mut RPC_MESSAGE);
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn NdrServerCallAll(prpcmsg: *mut RPC_MESSAGE);
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn NdrServerCallNdr64(prpcmsg: *mut RPC_MESSAGE);
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrServerContextMarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, contexthandle: *mut NDR_SCONTEXT_1, rundownroutine: NDR_RUNDOWN);
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrServerContextNewMarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, contexthandle: *mut NDR_SCONTEXT_1, rundownroutine: NDR_RUNDOWN, pformat: *mut u8);
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrServerContextNewUnmarshall(pstubmsg: *const MIDL_STUB_MESSAGE, pformat: *const u8) -> *mut NDR_SCONTEXT_1;
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrServerContextUnmarshall(pstubmsg: *mut MIDL_STUB_MESSAGE) -> *mut NDR_SCONTEXT_1;
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrServerInitialize(prpcmsg: *mut RPC_MESSAGE, pstubmsg: *mut MIDL_STUB_MESSAGE, pstubdescriptor: *mut MIDL_STUB_DESC) -> *mut u8;
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrServerInitializeMarshall(prpcmsg: *mut RPC_MESSAGE, pstubmsg: *mut MIDL_STUB_MESSAGE);
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrServerInitializeNew(prpcmsg: *mut RPC_MESSAGE, pstubmsg: *mut MIDL_STUB_MESSAGE, pstubdescriptor: *mut MIDL_STUB_DESC) -> *mut u8;
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrServerInitializePartial(prpcmsg: *mut RPC_MESSAGE, pstubmsg: *mut MIDL_STUB_MESSAGE, pstubdescriptor: *mut MIDL_STUB_DESC, requestedbuffersize: u32);
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrServerInitializeUnmarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, pstubdescriptor: *mut MIDL_STUB_DESC, prpcmsg: *mut RPC_MESSAGE) -> *mut u8;
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrSimpleStructBufferSize(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8);
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrSimpleStructFree(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8);
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrSimpleStructMarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) -> *mut u8;
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrSimpleStructMemorySize(pstubmsg: *mut MIDL_STUB_MESSAGE, pformat: *mut u8) -> u32;
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrSimpleStructUnmarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, ppmemory: *mut *mut u8, pformat: *mut u8, fmustalloc: u8) -> *mut u8;
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrSimpleTypeMarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, formatchar: u8);
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrSimpleTypeUnmarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, formatchar: u8);
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn NdrStubCall2(pthis: *mut ::core::ffi::c_void, pchannel: *mut ::core::ffi::c_void, prpcmsg: *mut RPC_MESSAGE, pdwstubphase: *mut u32) -> i32;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn NdrStubCall3(pthis: *mut ::core::ffi::c_void, pchannel: *mut ::core::ffi::c_void, prpcmsg: *mut RPC_MESSAGE, pdwstubphase: *mut u32) -> i32;
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrUserMarshalBufferSize(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8);
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrUserMarshalFree(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8);
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrUserMarshalMarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) -> *mut u8;
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrUserMarshalMemorySize(pstubmsg: *mut MIDL_STUB_MESSAGE, pformat: *mut u8) -> u32;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn NdrUserMarshalSimpleTypeConvert(pflags: *mut u32, pbuffer: *mut u8, formatchar: u8) -> *mut u8;
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrUserMarshalUnmarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, ppmemory: *mut *mut u8, pformat: *mut u8, fmustalloc: u8) -> *mut u8;
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrVaryingArrayBufferSize(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8);
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrVaryingArrayFree(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8);
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrVaryingArrayMarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) -> *mut u8;
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrVaryingArrayMemorySize(pstubmsg: *mut MIDL_STUB_MESSAGE, pformat: *mut u8) -> u32;
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrVaryingArrayUnmarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, ppmemory: *mut *mut u8, pformat: *mut u8, fmustalloc: u8) -> *mut u8;
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrXmitOrRepAsBufferSize(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8);
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrXmitOrRepAsFree(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8);
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrXmitOrRepAsMarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) -> *mut u8;
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrXmitOrRepAsMemorySize(pstubmsg: *mut MIDL_STUB_MESSAGE, pformat: *mut u8) -> u32;
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrXmitOrRepAsUnmarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, ppmemory: *mut *mut u8, pformat: *mut u8, fmustalloc: u8) -> *mut u8;
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn RpcAsyncAbortCall(pasync: *mut RPC_ASYNC_STATE, exceptioncode: u32) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn RpcAsyncCancelCall(pasync: *mut RPC_ASYNC_STATE, fabort: super::super::Foundation::BOOL) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn RpcAsyncCompleteCall(pasync: *mut RPC_ASYNC_STATE, reply: *mut ::core::ffi::c_void) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn RpcAsyncGetCallStatus(pasync: *const RPC_ASYNC_STATE) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn RpcAsyncInitializeHandle(pasync: *mut RPC_ASYNC_STATE, size: u32) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn RpcAsyncRegisterInfo(pasync: *const RPC_ASYNC_STATE) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn RpcBindingBind(pasync: *const RPC_ASYNC_STATE, binding: *const ::core::ffi::c_void, ifspec: *const ::core::ffi::c_void) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcBindingCopy(sourcebinding: *const ::core::ffi::c_void, destinationbinding: *mut *mut ::core::ffi::c_void) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn RpcBindingCreateA(template: *const RPC_BINDING_HANDLE_TEMPLATE_V1_A, security: *const RPC_BINDING_HANDLE_SECURITY_V1_A, options: *const RPC_BINDING_HANDLE_OPTIONS_V1, binding: *mut *mut ::core::ffi::c_void) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn RpcBindingCreateW(template: *const RPC_BINDING_HANDLE_TEMPLATE_V1_W, security: *const RPC_BINDING_HANDLE_SECURITY_V1_W, options: *const RPC_BINDING_HANDLE_OPTIONS_V1, binding: *mut *mut ::core::ffi::c_void) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcBindingFree(binding: *mut *mut ::core::ffi::c_void) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcBindingFromStringBindingA(stringbinding: *const u8, binding: *mut *mut ::core::ffi::c_void) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcBindingFromStringBindingW(stringbinding: *const u16, binding: *mut *mut ::core::ffi::c_void) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcBindingInqAuthClientA(clientbinding: *const ::core::ffi::c_void, privs: *mut *mut ::core::ffi::c_void, serverprincname: *mut *mut u8, authnlevel: *mut u32, authnsvc: *mut u32, authzsvc: *mut u32) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcBindingInqAuthClientExA(clientbinding: *const ::core::ffi::c_void, privs: *mut *mut ::core::ffi::c_void, serverprincname: *mut *mut u8, authnlevel: *mut u32, authnsvc: *mut u32, authzsvc: *mut u32, flags: u32) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcBindingInqAuthClientExW(clientbinding: *const ::core::ffi::c_void, privs: *mut *mut ::core::ffi::c_void, serverprincname: *mut *mut u16, authnlevel: *mut u32, authnsvc: *mut u32, authzsvc: *mut u32, flags: u32) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcBindingInqAuthClientW(clientbinding: *const ::core::ffi::c_void, privs: *mut *mut ::core::ffi::c_void, serverprincname: *mut *mut u16, authnlevel: *mut u32, authnsvc: *mut u32, authzsvc: *mut u32) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcBindingInqAuthInfoA(binding: *const ::core::ffi::c_void, serverprincname: *mut *mut u8, authnlevel: *mut u32, authnsvc: *mut u32, authidentity: *mut *mut ::core::ffi::c_void, authzsvc: *mut u32) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn RpcBindingInqAuthInfoExA(binding: *const ::core::ffi::c_void, serverprincname: *mut *mut u8, authnlevel: *mut u32, authnsvc: *mut u32, authidentity: *mut *mut ::core::ffi::c_void, authzsvc: *mut u32, rpcqosversion: u32, securityqos: *mut RPC_SECURITY_QOS) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn RpcBindingInqAuthInfoExW(binding: *const ::core::ffi::c_void, serverprincname: *mut *mut u16, authnlevel: *mut u32, authnsvc: *mut u32, authidentity: *mut *mut ::core::ffi::c_void, authzsvc: *mut u32, rpcqosversion: u32, securityqos: *mut RPC_SECURITY_QOS) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcBindingInqAuthInfoW(binding: *const ::core::ffi::c_void, serverprincname: *mut *mut u16, authnlevel: *mut u32, authnsvc: *mut u32, authidentity: *mut *mut ::core::ffi::c_void, authzsvc: *mut u32) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcBindingInqMaxCalls(binding: *const ::core::ffi::c_void, maxcalls: *mut u32) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcBindingInqObject(binding: *const ::core::ffi::c_void, objectuuid: *mut ::windows_sys::core::GUID) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcBindingInqOption(hbinding: *const ::core::ffi::c_void, option: u32, poptionvalue: *mut usize) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcBindingReset(binding: *const ::core::ffi::c_void) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcBindingServerFromClient(clientbinding: *const ::core::ffi::c_void, serverbinding: *mut *mut ::core::ffi::c_void) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcBindingSetAuthInfoA(binding: *const ::core::ffi::c_void, serverprincname: *const u8, authnlevel: u32, authnsvc: u32, authidentity: *const ::core::ffi::c_void, authzsvc: u32) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn RpcBindingSetAuthInfoExA(binding: *const ::core::ffi::c_void, serverprincname: *const u8, authnlevel: u32, authnsvc: u32, authidentity: *const ::core::ffi::c_void, authzsvc: u32, securityqos: *const RPC_SECURITY_QOS) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn RpcBindingSetAuthInfoExW(binding: *const ::core::ffi::c_void, serverprincname: *const u16, authnlevel: u32, authnsvc: u32, authidentity: *const ::core::ffi::c_void, authzsvc: u32, securityqos: *const RPC_SECURITY_QOS) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcBindingSetAuthInfoW(binding: *const ::core::ffi::c_void, serverprincname: *const u16, authnlevel: u32, authnsvc: u32, authidentity: *const ::core::ffi::c_void, authzsvc: u32) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcBindingSetObject(binding: *const ::core::ffi::c_void, objectuuid: *const ::windows_sys::core::GUID) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcBindingSetOption(hbinding: *const ::core::ffi::c_void, option: u32, optionvalue: usize) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcBindingToStringBindingA(binding: *const ::core::ffi::c_void, stringbinding: *mut *mut u8) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcBindingToStringBindingW(binding: *const ::core::ffi::c_void, stringbinding: *mut *mut u16) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcBindingUnbind(binding: *const ::core::ffi::c_void) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcBindingVectorFree(bindingvector: *mut *mut RPC_BINDING_VECTOR) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcCancelThread(thread: *const ::core::ffi::c_void) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcCancelThreadEx(thread: *const ::core::ffi::c_void, timeout: i32) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_Foundation`, `Win32_Security_Cryptography`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
    pub fn RpcCertGeneratePrincipalNameA(context: *const super::super::Security::Cryptography::CERT_CONTEXT, flags: u32, pbuffer: *mut *mut u8) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_Foundation`, `Win32_Security_Cryptography`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
    pub fn RpcCertGeneratePrincipalNameW(context: *const super::super::Security::Cryptography::CERT_CONTEXT, flags: u32, pbuffer: *mut *mut u16) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcEpRegisterA(ifspec: *const ::core::ffi::c_void, bindingvector: *const RPC_BINDING_VECTOR, uuidvector: *const UUID_VECTOR, annotation: *const u8) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcEpRegisterNoReplaceA(ifspec: *const ::core::ffi::c_void, bindingvector: *const RPC_BINDING_VECTOR, uuidvector: *const UUID_VECTOR, annotation: *const u8) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcEpRegisterNoReplaceW(ifspec: *const ::core::ffi::c_void, bindingvector: *const RPC_BINDING_VECTOR, uuidvector: *const UUID_VECTOR, annotation: *const u16) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcEpRegisterW(ifspec: *const ::core::ffi::c_void, bindingvector: *const RPC_BINDING_VECTOR, uuidvector: *const UUID_VECTOR, annotation: *const u16) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcEpResolveBinding(binding: *const ::core::ffi::c_void, ifspec: *const ::core::ffi::c_void) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcEpUnregister(ifspec: *const ::core::ffi::c_void, bindingvector: *const RPC_BINDING_VECTOR, uuidvector: *const UUID_VECTOR) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RpcErrorAddRecord(errorinfo: *const RPC_EXTENDED_ERROR_INFO) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcErrorClearInformation();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcErrorEndEnumeration(enumhandle: *mut RPC_ERROR_ENUM_HANDLE) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RpcErrorGetNextRecord(enumhandle: *const RPC_ERROR_ENUM_HANDLE, copystrings: super::super::Foundation::BOOL, errorinfo: *mut RPC_EXTENDED_ERROR_INFO) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcErrorGetNumberOfRecords(enumhandle: *const RPC_ERROR_ENUM_HANDLE, records: *mut i32) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcErrorLoadErrorInfo(errorblob: *const ::core::ffi::c_void, blobsize: usize, enumhandle: *mut RPC_ERROR_ENUM_HANDLE) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcErrorResetEnumeration(enumhandle: *mut RPC_ERROR_ENUM_HANDLE) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcErrorSaveErrorInfo(enumhandle: *const RPC_ERROR_ENUM_HANDLE, errorblob: *mut *mut ::core::ffi::c_void, blobsize: *mut usize) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcErrorStartEnumeration(enumhandle: *mut RPC_ERROR_ENUM_HANDLE) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcExceptionFilter(exceptioncode: u32) -> i32;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcFreeAuthorizationContext(pauthzclientcontext: *mut *mut ::core::ffi::c_void) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RpcGetAuthorizationContextForClient(clientbinding: *const ::core::ffi::c_void, impersonateonreturn: super::super::Foundation::BOOL, reserved1: *const ::core::ffi::c_void, pexpirationtime: *const i64, reserved2: super::super::Foundation::LUID, reserved3: u32, reserved4: *const ::core::ffi::c_void, pauthzclientcontext: *mut *mut ::core::ffi::c_void) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcIfIdVectorFree(ifidvector: *mut *mut RPC_IF_ID_VECTOR) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcIfInqId(rpcifhandle: *const ::core::ffi::c_void, rpcifid: *mut RPC_IF_ID) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcImpersonateClient(bindinghandle: *const ::core::ffi::c_void) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcImpersonateClient2(bindinghandle: *const ::core::ffi::c_void) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcImpersonateClientContainer(bindinghandle: *const ::core::ffi::c_void) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcMgmtEnableIdleCleanup() -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcMgmtEpEltInqBegin(epbinding: *const ::core::ffi::c_void, inquirytype: u32, ifid: *const RPC_IF_ID, versoption: u32, objectuuid: *const ::windows_sys::core::GUID, inquirycontext: *mut *mut *mut ::core::ffi::c_void) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcMgmtEpEltInqDone(inquirycontext: *mut *mut *mut ::core::ffi::c_void) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcMgmtEpEltInqNextA(inquirycontext: *const *const ::core::ffi::c_void, ifid: *mut RPC_IF_ID, binding: *mut *mut ::core::ffi::c_void, objectuuid: *mut ::windows_sys::core::GUID, annotation: *mut *mut u8) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcMgmtEpEltInqNextW(inquirycontext: *const *const ::core::ffi::c_void, ifid: *mut RPC_IF_ID, binding: *mut *mut ::core::ffi::c_void, objectuuid: *mut ::windows_sys::core::GUID, annotation: *mut *mut u16) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcMgmtEpUnregister(epbinding: *const ::core::ffi::c_void, ifid: *const RPC_IF_ID, binding: *const ::core::ffi::c_void, objectuuid: *const ::windows_sys::core::GUID) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcMgmtInqComTimeout(binding: *const ::core::ffi::c_void, timeout: *mut u32) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcMgmtInqDefaultProtectLevel(authnsvc: u32, authnlevel: *mut u32) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcMgmtInqIfIds(binding: *const ::core::ffi::c_void, ifidvector: *mut *mut RPC_IF_ID_VECTOR) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcMgmtInqServerPrincNameA(binding: *const ::core::ffi::c_void, authnsvc: u32, serverprincname: *mut *mut u8) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcMgmtInqServerPrincNameW(binding: *const ::core::ffi::c_void, authnsvc: u32, serverprincname: *mut *mut u16) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcMgmtInqStats(binding: *const ::core::ffi::c_void, statistics: *mut *mut RPC_STATS_VECTOR) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcMgmtIsServerListening(binding: *const ::core::ffi::c_void) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcMgmtSetAuthorizationFn(authorizationfn: RPC_MGMT_AUTHORIZATION_FN) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcMgmtSetCancelTimeout(timeout: i32) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcMgmtSetComTimeout(binding: *const ::core::ffi::c_void, timeout: u32) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcMgmtSetServerStackSize(threadstacksize: u32) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcMgmtStatsVectorFree(statsvector: *mut *mut RPC_STATS_VECTOR) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcMgmtStopServerListening(binding: *const ::core::ffi::c_void) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcMgmtWaitServerListen() -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcNetworkInqProtseqsA(protseqvector: *mut *mut RPC_PROTSEQ_VECTORA) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcNetworkInqProtseqsW(protseqvector: *mut *mut RPC_PROTSEQ_VECTORW) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcNetworkIsProtseqValidA(protseq: *const u8) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcNetworkIsProtseqValidW(protseq: *const u16) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcNsBindingExportA(entrynamesyntax: u32, entryname: *const u8, ifspec: *const ::core::ffi::c_void, bindingvec: *const RPC_BINDING_VECTOR, objectuuidvec: *const UUID_VECTOR) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcNsBindingExportPnPA(entrynamesyntax: u32, entryname: *const u8, ifspec: *const ::core::ffi::c_void, objectvector: *const UUID_VECTOR) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcNsBindingExportPnPW(entrynamesyntax: u32, entryname: *const u16, ifspec: *const ::core::ffi::c_void, objectvector: *const UUID_VECTOR) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcNsBindingExportW(entrynamesyntax: u32, entryname: *const u16, ifspec: *const ::core::ffi::c_void, bindingvec: *const RPC_BINDING_VECTOR, objectuuidvec: *const UUID_VECTOR) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcNsBindingImportBeginA(entrynamesyntax: u32, entryname: *const u8, ifspec: *const ::core::ffi::c_void, objuuid: *const ::windows_sys::core::GUID, importcontext: *mut *mut ::core::ffi::c_void) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcNsBindingImportBeginW(entrynamesyntax: u32, entryname: *const u16, ifspec: *const ::core::ffi::c_void, objuuid: *const ::windows_sys::core::GUID, importcontext: *mut *mut ::core::ffi::c_void) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcNsBindingImportDone(importcontext: *mut *mut ::core::ffi::c_void) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcNsBindingImportNext(importcontext: *mut ::core::ffi::c_void, binding: *mut *mut ::core::ffi::c_void) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcNsBindingInqEntryNameA(binding: *const ::core::ffi::c_void, entrynamesyntax: u32, entryname: *mut *mut u8) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcNsBindingInqEntryNameW(binding: *const ::core::ffi::c_void, entrynamesyntax: u32, entryname: *mut *mut u16) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcNsBindingLookupBeginA(entrynamesyntax: u32, entryname: *const u8, ifspec: *const ::core::ffi::c_void, objuuid: *const ::windows_sys::core::GUID, bindingmaxcount: u32, lookupcontext: *mut *mut ::core::ffi::c_void) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcNsBindingLookupBeginW(entrynamesyntax: u32, entryname: *const u16, ifspec: *const ::core::ffi::c_void, objuuid: *const ::windows_sys::core::GUID, bindingmaxcount: u32, lookupcontext: *mut *mut ::core::ffi::c_void) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcNsBindingLookupDone(lookupcontext: *mut *mut ::core::ffi::c_void) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcNsBindingLookupNext(lookupcontext: *mut ::core::ffi::c_void, bindingvec: *mut *mut RPC_BINDING_VECTOR) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcNsBindingSelect(bindingvec: *mut RPC_BINDING_VECTOR, binding: *mut *mut ::core::ffi::c_void) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcNsBindingUnexportA(entrynamesyntax: u32, entryname: *const u8, ifspec: *const ::core::ffi::c_void, objectuuidvec: *const UUID_VECTOR) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcNsBindingUnexportPnPA(entrynamesyntax: u32, entryname: *const u8, ifspec: *const ::core::ffi::c_void, objectvector: *const UUID_VECTOR) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcNsBindingUnexportPnPW(entrynamesyntax: u32, entryname: *const u16, ifspec: *const ::core::ffi::c_void, objectvector: *const UUID_VECTOR) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcNsBindingUnexportW(entrynamesyntax: u32, entryname: *const u16, ifspec: *const ::core::ffi::c_void, objectuuidvec: *const UUID_VECTOR) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcNsEntryExpandNameA(entrynamesyntax: u32, entryname: *const u8, expandedname: *mut *mut u8) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcNsEntryExpandNameW(entrynamesyntax: u32, entryname: *const u16, expandedname: *mut *mut u16) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcNsEntryObjectInqBeginA(entrynamesyntax: u32, entryname: *const u8, inquirycontext: *mut *mut ::core::ffi::c_void) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcNsEntryObjectInqBeginW(entrynamesyntax: u32, entryname: *const u16, inquirycontext: *mut *mut ::core::ffi::c_void) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcNsEntryObjectInqDone(inquirycontext: *mut *mut ::core::ffi::c_void) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcNsEntryObjectInqNext(inquirycontext: *mut ::core::ffi::c_void, objuuid: *mut ::windows_sys::core::GUID) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcNsGroupDeleteA(groupnamesyntax: GROUP_NAME_SYNTAX, groupname: *const u8) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcNsGroupDeleteW(groupnamesyntax: GROUP_NAME_SYNTAX, groupname: *const u16) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcNsGroupMbrAddA(groupnamesyntax: u32, groupname: *const u8, membernamesyntax: u32, membername: *const u8) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcNsGroupMbrAddW(groupnamesyntax: u32, groupname: *const u16, membernamesyntax: u32, membername: *const u16) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcNsGroupMbrInqBeginA(groupnamesyntax: u32, groupname: *const u8, membernamesyntax: u32, inquirycontext: *mut *mut ::core::ffi::c_void) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcNsGroupMbrInqBeginW(groupnamesyntax: u32, groupname: *const u16, membernamesyntax: u32, inquirycontext: *mut *mut ::core::ffi::c_void) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcNsGroupMbrInqDone(inquirycontext: *mut *mut ::core::ffi::c_void) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcNsGroupMbrInqNextA(inquirycontext: *mut ::core::ffi::c_void, membername: *mut *mut u8) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcNsGroupMbrInqNextW(inquirycontext: *mut ::core::ffi::c_void, membername: *mut *mut u16) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcNsGroupMbrRemoveA(groupnamesyntax: u32, groupname: *const u8, membernamesyntax: u32, membername: *const u8) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcNsGroupMbrRemoveW(groupnamesyntax: u32, groupname: *const u16, membernamesyntax: u32, membername: *const u16) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcNsMgmtBindingUnexportA(entrynamesyntax: u32, entryname: *const u8, ifid: *const RPC_IF_ID, versoption: u32, objectuuidvec: *const UUID_VECTOR) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcNsMgmtBindingUnexportW(entrynamesyntax: u32, entryname: *const u16, ifid: *const RPC_IF_ID, versoption: u32, objectuuidvec: *const UUID_VECTOR) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcNsMgmtEntryCreateA(entrynamesyntax: u32, entryname: *const u8) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcNsMgmtEntryCreateW(entrynamesyntax: u32, entryname: *const u16) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcNsMgmtEntryDeleteA(entrynamesyntax: u32, entryname: *const u8) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcNsMgmtEntryDeleteW(entrynamesyntax: u32, entryname: *const u16) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcNsMgmtEntryInqIfIdsA(entrynamesyntax: u32, entryname: *const u8, ifidvec: *mut *mut RPC_IF_ID_VECTOR) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcNsMgmtEntryInqIfIdsW(entrynamesyntax: u32, entryname: *const u16, ifidvec: *mut *mut RPC_IF_ID_VECTOR) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcNsMgmtHandleSetExpAge(nshandle: *mut ::core::ffi::c_void, expirationage: u32) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcNsMgmtInqExpAge(expirationage: *mut u32) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcNsMgmtSetExpAge(expirationage: u32) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcNsProfileDeleteA(profilenamesyntax: u32, profilename: *const u8) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcNsProfileDeleteW(profilenamesyntax: u32, profilename: *const u16) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcNsProfileEltAddA(profilenamesyntax: u32, profilename: *const u8, ifid: *const RPC_IF_ID, membernamesyntax: u32, membername: *const u8, priority: u32, annotation: *const u8) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcNsProfileEltAddW(profilenamesyntax: u32, profilename: *const u16, ifid: *const RPC_IF_ID, membernamesyntax: u32, membername: *const u16, priority: u32, annotation: *const u16) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcNsProfileEltInqBeginA(profilenamesyntax: u32, profilename: *const u8, inquirytype: u32, ifid: *const RPC_IF_ID, versoption: u32, membernamesyntax: u32, membername: *const u8, inquirycontext: *mut *mut ::core::ffi::c_void) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcNsProfileEltInqBeginW(profilenamesyntax: u32, profilename: *const u16, inquirytype: u32, ifid: *const RPC_IF_ID, versoption: u32, membernamesyntax: u32, membername: *const u16, inquirycontext: *mut *mut ::core::ffi::c_void) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcNsProfileEltInqDone(inquirycontext: *mut *mut ::core::ffi::c_void) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcNsProfileEltInqNextA(inquirycontext: *const ::core::ffi::c_void, ifid: *mut RPC_IF_ID, membername: *mut *mut u8, priority: *mut u32, annotation: *mut *mut u8) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcNsProfileEltInqNextW(inquirycontext: *const ::core::ffi::c_void, ifid: *mut RPC_IF_ID, membername: *mut *mut u16, priority: *mut u32, annotation: *mut *mut u16) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcNsProfileEltRemoveA(profilenamesyntax: u32, profilename: *const u8, ifid: *const RPC_IF_ID, membernamesyntax: u32, membername: *const u8) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcNsProfileEltRemoveW(profilenamesyntax: u32, profilename: *const u16, ifid: *const RPC_IF_ID, membernamesyntax: u32, membername: *const u16) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcObjectInqType(objuuid: *const ::windows_sys::core::GUID, typeuuid: *mut ::windows_sys::core::GUID) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcObjectSetInqFn(inquiryfn: RPC_OBJECT_INQ_FN) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcObjectSetType(objuuid: *const ::windows_sys::core::GUID, typeuuid: *const ::windows_sys::core::GUID) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcProtseqVectorFreeA(protseqvector: *mut *mut RPC_PROTSEQ_VECTORA) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcProtseqVectorFreeW(protseqvector: *mut *mut RPC_PROTSEQ_VECTORW) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcRaiseException(exception: RPC_STATUS);
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcRevertContainerImpersonation() -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcRevertToSelf() -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcRevertToSelfEx(bindinghandle: *const ::core::ffi::c_void) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcServerCompleteSecurityCallback(bindinghandle: *const ::core::ffi::c_void, status: RPC_STATUS) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcServerInqBindingHandle(binding: *mut *mut ::core::ffi::c_void) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcServerInqBindings(bindingvector: *mut *mut RPC_BINDING_VECTOR) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcServerInqBindingsEx(securitydescriptor: *const ::core::ffi::c_void, bindingvector: *mut *mut RPC_BINDING_VECTOR) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcServerInqCallAttributesA(clientbinding: *const ::core::ffi::c_void, rpccallattributes: *mut ::core::ffi::c_void) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcServerInqCallAttributesW(clientbinding: *const ::core::ffi::c_void, rpccallattributes: *mut ::core::ffi::c_void) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcServerInqDefaultPrincNameA(authnsvc: u32, princname: *mut *mut u8) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcServerInqDefaultPrincNameW(authnsvc: u32, princname: *mut *mut u16) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcServerInqIf(ifspec: *const ::core::ffi::c_void, mgrtypeuuid: *const ::windows_sys::core::GUID, mgrepv: *mut *mut ::core::ffi::c_void) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcServerInterfaceGroupActivate(ifgroup: *const ::core::ffi::c_void) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcServerInterfaceGroupClose(ifgroup: *const ::core::ffi::c_void) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcServerInterfaceGroupCreateA(interfaces: *const RPC_INTERFACE_TEMPLATEA, numifs: u32, endpoints: *const RPC_ENDPOINT_TEMPLATEA, numendpoints: u32, idleperiod: u32, idlecallbackfn: RPC_INTERFACE_GROUP_IDLE_CALLBACK_FN, idlecallbackcontext: *const ::core::ffi::c_void, ifgroup: *mut *mut ::core::ffi::c_void) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcServerInterfaceGroupCreateW(interfaces: *const RPC_INTERFACE_TEMPLATEW, numifs: u32, endpoints: *const RPC_ENDPOINT_TEMPLATEW, numendpoints: u32, idleperiod: u32, idlecallbackfn: RPC_INTERFACE_GROUP_IDLE_CALLBACK_FN, idlecallbackcontext: *const ::core::ffi::c_void, ifgroup: *mut *mut ::core::ffi::c_void) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcServerInterfaceGroupDeactivate(ifgroup: *const ::core::ffi::c_void, forcedeactivation: u32) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcServerInterfaceGroupInqBindings(ifgroup: *const ::core::ffi::c_void, bindingvector: *mut *mut RPC_BINDING_VECTOR) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcServerListen(minimumcallthreads: u32, maxcalls: u32, dontwait: u32) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcServerRegisterAuthInfoA(serverprincname: *const u8, authnsvc: u32, getkeyfn: RPC_AUTH_KEY_RETRIEVAL_FN, arg: *const ::core::ffi::c_void) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcServerRegisterAuthInfoW(serverprincname: *const u16, authnsvc: u32, getkeyfn: RPC_AUTH_KEY_RETRIEVAL_FN, arg: *const ::core::ffi::c_void) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcServerRegisterIf(ifspec: *const ::core::ffi::c_void, mgrtypeuuid: *const ::windows_sys::core::GUID, mgrepv: *const ::core::ffi::c_void) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcServerRegisterIf2(ifspec: *const ::core::ffi::c_void, mgrtypeuuid: *const ::windows_sys::core::GUID, mgrepv: *const ::core::ffi::c_void, flags: u32, maxcalls: u32, maxrpcsize: u32, ifcallbackfn: RPC_IF_CALLBACK_FN) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcServerRegisterIf3(ifspec: *const ::core::ffi::c_void, mgrtypeuuid: *const ::windows_sys::core::GUID, mgrepv: *const ::core::ffi::c_void, flags: u32, maxcalls: u32, maxrpcsize: u32, ifcallback: RPC_IF_CALLBACK_FN, securitydescriptor: *const ::core::ffi::c_void) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcServerRegisterIfEx(ifspec: *const ::core::ffi::c_void, mgrtypeuuid: *const ::windows_sys::core::GUID, mgrepv: *const ::core::ffi::c_void, flags: u32, maxcalls: u32, ifcallback: RPC_IF_CALLBACK_FN) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn RpcServerSubscribeForNotification(binding: *const ::core::ffi::c_void, notification: RPC_NOTIFICATIONS, notificationtype: RPC_NOTIFICATION_TYPES, notificationinfo: *const RPC_ASYNC_NOTIFICATION_INFO) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcServerTestCancel(bindinghandle: *const ::core::ffi::c_void) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcServerUnregisterIf(ifspec: *const ::core::ffi::c_void, mgrtypeuuid: *const ::windows_sys::core::GUID, waitforcallstocomplete: u32) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcServerUnregisterIfEx(ifspec: *const ::core::ffi::c_void, mgrtypeuuid: *const ::windows_sys::core::GUID, rundowncontexthandles: i32) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcServerUnsubscribeForNotification(binding: *const ::core::ffi::c_void, notification: RPC_NOTIFICATIONS, notificationsqueued: *mut u32) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcServerUseAllProtseqs(maxcalls: u32, securitydescriptor: *const ::core::ffi::c_void) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcServerUseAllProtseqsEx(maxcalls: u32, securitydescriptor: *const ::core::ffi::c_void, policy: *const RPC_POLICY) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcServerUseAllProtseqsIf(maxcalls: u32, ifspec: *const ::core::ffi::c_void, securitydescriptor: *const ::core::ffi::c_void) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcServerUseAllProtseqsIfEx(maxcalls: u32, ifspec: *const ::core::ffi::c_void, securitydescriptor: *const ::core::ffi::c_void, policy: *const RPC_POLICY) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcServerUseProtseqA(protseq: *const u8, maxcalls: u32, securitydescriptor: *const ::core::ffi::c_void) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcServerUseProtseqEpA(protseq: *const u8, maxcalls: u32, endpoint: *const u8, securitydescriptor: *const ::core::ffi::c_void) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcServerUseProtseqEpExA(protseq: *const u8, maxcalls: u32, endpoint: *const u8, securitydescriptor: *const ::core::ffi::c_void, policy: *const RPC_POLICY) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcServerUseProtseqEpExW(protseq: *const u16, maxcalls: u32, endpoint: *const u16, securitydescriptor: *const ::core::ffi::c_void, policy: *const RPC_POLICY) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcServerUseProtseqEpW(protseq: *const u16, maxcalls: u32, endpoint: *const u16, securitydescriptor: *const ::core::ffi::c_void) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcServerUseProtseqExA(protseq: *const u8, maxcalls: u32, securitydescriptor: *const ::core::ffi::c_void, policy: *const RPC_POLICY) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcServerUseProtseqExW(protseq: *const u16, maxcalls: u32, securitydescriptor: *const ::core::ffi::c_void, policy: *const RPC_POLICY) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcServerUseProtseqIfA(protseq: *const u8, maxcalls: u32, ifspec: *const ::core::ffi::c_void, securitydescriptor: *const ::core::ffi::c_void) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcServerUseProtseqIfExA(protseq: *const u8, maxcalls: u32, ifspec: *const ::core::ffi::c_void, securitydescriptor: *const ::core::ffi::c_void, policy: *const RPC_POLICY) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcServerUseProtseqIfExW(protseq: *const u16, maxcalls: u32, ifspec: *const ::core::ffi::c_void, securitydescriptor: *const ::core::ffi::c_void, policy: *const RPC_POLICY) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcServerUseProtseqIfW(protseq: *const u16, maxcalls: u32, ifspec: *const ::core::ffi::c_void, securitydescriptor: *const ::core::ffi::c_void) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcServerUseProtseqW(protseq: *const u16, maxcalls: u32, securitydescriptor: *const ::core::ffi::c_void) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcServerYield();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcSmAllocate(size: usize, pstatus: *mut RPC_STATUS) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcSmClientFree(pnodetofree: *const ::core::ffi::c_void) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcSmDestroyClientContext(contexthandle: *const *const ::core::ffi::c_void) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcSmDisableAllocate() -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcSmEnableAllocate() -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcSmFree(nodetofree: *const ::core::ffi::c_void) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcSmGetThreadHandle(pstatus: *mut RPC_STATUS) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcSmSetClientAllocFree(clientalloc: RPC_CLIENT_ALLOC, clientfree: RPC_CLIENT_FREE) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcSmSetThreadHandle(id: *const ::core::ffi::c_void) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcSmSwapClientAllocFree(clientalloc: RPC_CLIENT_ALLOC, clientfree: RPC_CLIENT_FREE, oldclientalloc: *mut RPC_CLIENT_ALLOC, oldclientfree: *mut RPC_CLIENT_FREE) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcSsAllocate(size: usize) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcSsContextLockExclusive(serverbindinghandle: *const ::core::ffi::c_void, usercontext: *const ::core::ffi::c_void) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcSsContextLockShared(serverbindinghandle: *const ::core::ffi::c_void, usercontext: *const ::core::ffi::c_void) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcSsDestroyClientContext(contexthandle: *const *const ::core::ffi::c_void);
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcSsDisableAllocate();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcSsDontSerializeContext();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcSsEnableAllocate();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcSsFree(nodetofree: *const ::core::ffi::c_void);
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcSsGetContextBinding(contexthandle: *const ::core::ffi::c_void, binding: *mut *mut ::core::ffi::c_void) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcSsGetThreadHandle() -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcSsSetClientAllocFree(clientalloc: RPC_CLIENT_ALLOC, clientfree: RPC_CLIENT_FREE);
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcSsSetThreadHandle(id: *const ::core::ffi::c_void);
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcSsSwapClientAllocFree(clientalloc: RPC_CLIENT_ALLOC, clientfree: RPC_CLIENT_FREE, oldclientalloc: *mut RPC_CLIENT_ALLOC, oldclientfree: *mut RPC_CLIENT_FREE);
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcStringBindingComposeA(objuuid: *const u8, protseq: *const u8, networkaddr: *const u8, endpoint: *const u8, options: *const u8, stringbinding: *mut *mut u8) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcStringBindingComposeW(objuuid: *const u16, protseq: *const u16, networkaddr: *const u16, endpoint: *const u16, options: *const u16, stringbinding: *mut *mut u16) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcStringBindingParseA(stringbinding: *const u8, objuuid: *mut *mut u8, protseq: *mut *mut u8, networkaddr: *mut *mut u8, endpoint: *mut *mut u8, networkoptions: *mut *mut u8) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcStringBindingParseW(stringbinding: *const u16, objuuid: *mut *mut u16, protseq: *mut *mut u16, networkaddr: *mut *mut u16, endpoint: *mut *mut u16, networkoptions: *mut *mut u16) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcStringFreeA(string: *mut *mut u8) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcStringFreeW(string: *mut *mut u16) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcTestCancel() -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcUserFree(asynchandle: *mut ::core::ffi::c_void, pbuffer: *mut ::core::ffi::c_void);
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn UuidCompare(uuid1: *const ::windows_sys::core::GUID, uuid2: *const ::windows_sys::core::GUID, status: *mut RPC_STATUS) -> i32;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn UuidCreate(uuid: *mut ::windows_sys::core::GUID) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn UuidCreateNil(niluuid: *mut ::windows_sys::core::GUID) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn UuidCreateSequential(uuid: *mut ::windows_sys::core::GUID) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn UuidEqual(uuid1: *const ::windows_sys::core::GUID, uuid2: *const ::windows_sys::core::GUID, status: *mut RPC_STATUS) -> i32;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn UuidFromStringA(stringuuid: *const u8, uuid: *mut ::windows_sys::core::GUID) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn UuidFromStringW(stringuuid: *const u16, uuid: *mut ::windows_sys::core::GUID) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn UuidHash(uuid: *const ::windows_sys::core::GUID, status: *mut RPC_STATUS) -> u16;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn UuidIsNil(uuid: *const ::windows_sys::core::GUID, status: *mut RPC_STATUS) -> i32;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn UuidToStringA(uuid: *const ::windows_sys::core::GUID, stringuuid: *mut *mut u8) -> RPC_STATUS;
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn UuidToStringW(uuid: *const ::windows_sys::core::GUID, stringuuid: *mut *mut u16) -> RPC_STATUS;
}
pub struct ARRAY_INFO(i32);
pub struct BinaryParam(i32);
pub struct CLIENT_CALL_RETURN(i32);
pub struct COMM_FAULT_OFFSETS(i32);
pub struct CS_TAG_GETTING_ROUTINE(i32);
pub struct CS_TYPE_FROM_NETCS_ROUTINE(i32);
pub struct CS_TYPE_LOCAL_SIZE_ROUTINE(i32);
pub struct CS_TYPE_NET_SIZE_ROUTINE(i32);
pub struct CS_TYPE_TO_NETCS_ROUTINE(i32);
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const DCE_C_ERROR_STRING_LEN: u32 = 256u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const EEInfoGCCOM: u32 = 11u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const EEInfoGCFRS: u32 = 12u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const EEInfoNextRecordsMissing: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const EEInfoPreviousRecordsMissing: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const EEInfoUseFileTime: u32 = 4u32;
pub struct EXPR_EVAL(i32);
pub struct EXPR_TOKEN(i32);
pub struct ExtendedErrorParamTypes(i32);
pub struct FULL_PTR_XLAT_TABLES(i32);
pub struct GENERIC_BINDING_INFO(i32);
pub struct GENERIC_BINDING_ROUTINE(i32);
pub struct GENERIC_BINDING_ROUTINE_PAIR(i32);
pub struct GENERIC_UNBIND_ROUTINE(i32);
pub struct GROUP_NAME_SYNTAX(i32);
pub struct IDL_CS_CONVERT(i32);
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const INVALID_FRAGMENT_ID: u32 = 0u32;
pub struct I_RpcFreeCalloutStateFn(i32);
pub struct I_RpcPerformCalloutFn(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct I_RpcProxyCallbackInterface(i32);
pub struct I_RpcProxyFilterIfFn(i32);
pub struct I_RpcProxyGetClientAddressFn(i32);
pub struct I_RpcProxyGetClientSessionAndResourceUUID(i32);
pub struct I_RpcProxyGetConnectionTimeoutFn(i32);
pub struct I_RpcProxyIsValidMachineFn(i32);
pub struct I_RpcProxyUpdatePerfCounterBackendServerFn(i32);
pub struct I_RpcProxyUpdatePerfCounterFn(i32);
pub struct LRPC_SYSTEM_HANDLE_MARSHAL_DIRECTION(i32);
pub struct MALLOC_FREE_STRUCT(i32);
pub struct MIDL_ES_ALLOC(i32);
pub struct MIDL_ES_CODE(i32);
pub struct MIDL_ES_HANDLE_STYLE(i32);
pub struct MIDL_ES_READ(i32);
pub struct MIDL_ES_WRITE(i32);
pub struct MIDL_FORMAT_STRING(i32);
pub struct MIDL_INTERCEPTION_INFO(i32);
pub struct MIDL_INTERFACE_METHOD_PROPERTIES(i32);
pub struct MIDL_METHOD_PROPERTY(i32);
pub struct MIDL_METHOD_PROPERTY_MAP(i32);
#[cfg(feature = "Win32_System_Com")]
pub struct MIDL_SERVER_INFO(i32);
#[cfg(feature = "Win32_System_Com")]
pub struct MIDL_STUBLESS_PROXY_INFO(i32);
#[cfg(feature = "Win32_System_Com")]
pub struct MIDL_STUB_DESC(i32);
#[cfg(feature = "Win32_System_Com")]
pub struct MIDL_STUB_MESSAGE(i32);
pub struct MIDL_SYNTAX_INFO(i32);
pub struct MIDL_TYPE_PICKLING_INFO(i32);
#[cfg(feature = "Win32_System_Com")]
pub struct MIDL_WINRT_TYPE_SERIALIZATION_INFO(i32);
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const MIDL_WINRT_TYPE_SERIALIZATION_INFO_CURRENT_VERSION: i32 = 1i32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const MaxNumberOfEEInfoParams: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const MidlInterceptionInfoVersionOne: i32 = 1i32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const MidlWinrtTypeSerializationInfoVersionOne: i32 = 1i32;
pub struct NDR64_ARRAY_ELEMENT_INFO(i32);
pub struct NDR64_ARRAY_FLAGS(i32);
pub struct NDR64_BINDINGS(i32);
pub struct NDR64_BIND_AND_NOTIFY_EXTENSION(i32);
pub struct NDR64_BIND_CONTEXT(i32);
pub struct NDR64_BIND_GENERIC(i32);
pub struct NDR64_BIND_PRIMITIVE(i32);
pub struct NDR64_BOGUS_ARRAY_HEADER_FORMAT(i32);
pub struct NDR64_BOGUS_STRUCTURE_HEADER_FORMAT(i32);
pub struct NDR64_BUFFER_ALIGN_FORMAT(i32);
pub struct NDR64_CONFORMANT_STRING_FORMAT(i32);
pub struct NDR64_CONF_ARRAY_HEADER_FORMAT(i32);
pub struct NDR64_CONF_BOGUS_STRUCTURE_HEADER_FORMAT(i32);
pub struct NDR64_CONF_STRUCTURE_HEADER_FORMAT(i32);
pub struct NDR64_CONF_VAR_ARRAY_HEADER_FORMAT(i32);
pub struct NDR64_CONF_VAR_BOGUS_ARRAY_HEADER_FORMAT(i32);
pub struct NDR64_CONSTANT_IID_FORMAT(i32);
pub struct NDR64_CONTEXT_HANDLE_FLAGS(i32);
pub struct NDR64_CONTEXT_HANDLE_FORMAT(i32);
pub struct NDR64_EMBEDDED_COMPLEX_FORMAT(i32);
pub struct NDR64_ENCAPSULATED_UNION(i32);
pub struct NDR64_EXPR_CONST32(i32);
pub struct NDR64_EXPR_CONST64(i32);
pub struct NDR64_EXPR_NOOP(i32);
pub struct NDR64_EXPR_OPERATOR(i32);
pub struct NDR64_EXPR_VAR(i32);
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const NDR64_FC_AUTO_HANDLE: u32 = 3u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const NDR64_FC_BIND_GENERIC: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const NDR64_FC_BIND_PRIMITIVE: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const NDR64_FC_CALLBACK_HANDLE: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const NDR64_FC_EXPLICIT_HANDLE: u32 = 0u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const NDR64_FC_NO_HANDLE: u32 = 5u32;
pub struct NDR64_FIXED_REPEAT_FORMAT(i32);
pub struct NDR64_FIX_ARRAY_HEADER_FORMAT(i32);
pub struct NDR64_IID_FLAGS(i32);
pub struct NDR64_IID_FORMAT(i32);
pub struct NDR64_MEMPAD_FORMAT(i32);
pub struct NDR64_NON_CONFORMANT_STRING_FORMAT(i32);
pub struct NDR64_NON_ENCAPSULATED_UNION(i32);
pub struct NDR64_NO_REPEAT_FORMAT(i32);
pub struct NDR64_PARAM_FLAGS(i32);
pub struct NDR64_PARAM_FORMAT(i32);
pub struct NDR64_PIPE_FLAGS(i32);
pub struct NDR64_PIPE_FORMAT(i32);
pub struct NDR64_POINTER_FORMAT(i32);
pub struct NDR64_POINTER_INSTANCE_HEADER_FORMAT(i32);
pub struct NDR64_POINTER_REPEAT_FLAGS(i32);
pub struct NDR64_PROC_FLAGS(i32);
pub struct NDR64_PROC_FORMAT(i32);
pub struct NDR64_RANGED_STRING_FORMAT(i32);
pub struct NDR64_RANGE_FORMAT(i32);
pub struct NDR64_RANGE_PIPE_FORMAT(i32);
pub struct NDR64_REPEAT_FORMAT(i32);
pub struct NDR64_RPC_FLAGS(i32);
pub struct NDR64_SIMPLE_MEMBER_FORMAT(i32);
pub struct NDR64_SIMPLE_REGION_FORMAT(i32);
pub struct NDR64_SIZED_CONFORMANT_STRING_FORMAT(i32);
pub struct NDR64_STRING_FLAGS(i32);
pub struct NDR64_STRING_HEADER_FORMAT(i32);
pub struct NDR64_STRUCTURE_FLAGS(i32);
pub struct NDR64_STRUCTURE_HEADER_FORMAT(i32);
pub struct NDR64_SYSTEM_HANDLE_FORMAT(i32);
pub struct NDR64_TRANSMIT_AS_FLAGS(i32);
pub struct NDR64_TRANSMIT_AS_FORMAT(i32);
pub struct NDR64_TYPE_STRICT_CONTEXT_HANDLE(i32);
pub struct NDR64_UNION_ARM(i32);
pub struct NDR64_UNION_ARM_SELECTOR(i32);
pub struct NDR64_USER_MARSHAL_FLAGS(i32);
pub struct NDR64_USER_MARSHAL_FORMAT(i32);
pub struct NDR64_VAR_ARRAY_HEADER_FORMAT(i32);
pub struct NDR_ALLOC_ALL_NODES_CONTEXT(i32);
pub struct NDR_CS_ROUTINES(i32);
pub struct NDR_CS_SIZE_CONVERT_ROUTINES(i32);
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const NDR_CUSTOM_OR_DEFAULT_ALLOCATOR: u32 = 268435456u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const NDR_DEFAULT_ALLOCATOR: u32 = 536870912u32;
pub struct NDR_EXPR_DESC(i32);
pub struct NDR_NOTIFY2_ROUTINE(i32);
pub struct NDR_NOTIFY_ROUTINE(i32);
pub struct NDR_POINTER_QUEUE_STATE(i32);
pub struct NDR_RUNDOWN(i32);
pub struct NDR_SCONTEXT_1(i32);
#[cfg(feature = "Win32_System_Com")]
pub struct NDR_USER_MARSHAL_INFO(i32);
#[cfg(feature = "Win32_System_Com")]
pub struct NDR_USER_MARSHAL_INFO_LEVEL1(i32);
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const NT351_INTERFACE_SIZE: u32 = 64u32;
pub struct PFN_RPCNOTIFICATION_ROUTINE(i32);
pub struct PROXY_PHASE(i32);
pub struct PRPC_RUNDOWN(i32);
pub struct RDR_CALLOUT_STATE(i32);
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPCFLG_ACCESSIBILITY_BIT1: u32 = 1048576u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPCFLG_ACCESSIBILITY_BIT2: u32 = 2097152u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPCFLG_ACCESS_LOCAL: u32 = 4194304u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPCFLG_ASYNCHRONOUS: u32 = 1073741824u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPCFLG_AUTO_COMPLETE: u32 = 134217728u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPCFLG_HAS_CALLBACK: u32 = 67108864u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPCFLG_HAS_GUARANTEE: u32 = 16u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPCFLG_HAS_MULTI_SYNTAXES: u32 = 33554432u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPCFLG_INPUT_SYNCHRONOUS: u32 = 536870912u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPCFLG_LOCAL_CALL: u32 = 268435456u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPCFLG_MESSAGE: u32 = 16777216u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPCFLG_NDR64_CONTAINS_ARM_LAYOUT: u32 = 67108864u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPCFLG_NON_NDR: u32 = 2147483648u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPCFLG_SENDER_WAITING_FOR_REPLY: u32 = 8388608u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPCFLG_WINRT_REMOTE_ASYNC: u32 = 32u32;
pub struct RPCLT_PDU_FILTER_FUNC(i32);
pub struct RPC_ADDRESS_CHANGE_FN(i32);
pub struct RPC_ADDRESS_CHANGE_TYPE(i32);
pub struct RPC_ASYNC_EVENT(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub struct RPC_ASYNC_NOTIFICATION_INFO(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub struct RPC_ASYNC_STATE(i32);
pub struct RPC_AUTH_KEY_RETRIEVAL_FN(i32);
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_BHO_EXCLUSIVE_AND_GUARANTEED: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_BHT_OBJECT_UUID_VALID: u32 = 1u32;
pub struct RPC_BINDING_HANDLE_OPTIONS_FLAGS(i32);
pub struct RPC_BINDING_HANDLE_OPTIONS_V1(i32);
#[cfg(feature = "Win32_System_Com")]
pub struct RPC_BINDING_HANDLE_SECURITY_V1_A(i32);
#[cfg(feature = "Win32_System_Com")]
pub struct RPC_BINDING_HANDLE_SECURITY_V1_W(i32);
pub struct RPC_BINDING_HANDLE_TEMPLATE_V1_A(i32);
pub struct RPC_BINDING_HANDLE_TEMPLATE_V1_W(i32);
pub struct RPC_BINDING_VECTOR(i32);
pub struct RPC_BLOCKING_FN(i32);
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_BUFFER_ASYNC: u32 = 32768u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_BUFFER_COMPLETE: u32 = 4096u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_BUFFER_EXTRA: u32 = 16384u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_BUFFER_NONOTIFY: u32 = 65536u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_BUFFER_PARTIAL: u32 = 8192u32;
#[cfg(feature = "Win32_Foundation")]
pub struct RPC_CALL_ATTRIBUTES_V1_A(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct RPC_CALL_ATTRIBUTES_V1_W(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct RPC_CALL_ATTRIBUTES_V2_A(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct RPC_CALL_ATTRIBUTES_V2_W(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct RPC_CALL_ATTRIBUTES_V3_A(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct RPC_CALL_ATTRIBUTES_V3_W(i32);
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_CALL_ATTRIBUTES_VERSION: u32 = 2u32;
pub struct RPC_CALL_LOCAL_ADDRESS_V1(i32);
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_CALL_STATUS_CANCELLED: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_CALL_STATUS_DISCONNECTED: u32 = 2u32;
pub struct RPC_CLIENT_ALLOC(i32);
pub struct RPC_CLIENT_FREE(i32);
pub struct RPC_CLIENT_INFORMATION1(i32);
pub struct RPC_CLIENT_INTERFACE(i32);
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_CONTEXT_HANDLE_DEFAULT_FLAGS: u32 = 0u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_CONTEXT_HANDLE_DONT_SERIALIZE: u32 = 536870912u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_CONTEXT_HANDLE_FLAGS: u32 = 805306368u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_CONTEXT_HANDLE_SERIALIZE: u32 = 268435456u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_AUTHN_CLOUD_AP: u32 = 36u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_AUTHN_DCE_PRIVATE: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_AUTHN_DCE_PUBLIC: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_AUTHN_DEC_PUBLIC: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_AUTHN_DEFAULT: i32 = -1i32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_AUTHN_DIGEST: u32 = 21u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_AUTHN_DPA: u32 = 17u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_AUTHN_GSS_KERBEROS: u32 = 16u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_AUTHN_GSS_NEGOTIATE: u32 = 9u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_AUTHN_GSS_SCHANNEL: u32 = 14u32;
pub struct RPC_C_AUTHN_INFO_TYPE(i32);
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_AUTHN_KERNEL: u32 = 20u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_AUTHN_LIVEXP_SSP: u32 = 35u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_AUTHN_LIVE_SSP: u32 = 32u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_AUTHN_MQ: u32 = 100u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_AUTHN_MSN: u32 = 18u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_AUTHN_MSONLINE: u32 = 82u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_AUTHN_NEGO_EXTENDER: u32 = 30u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_AUTHN_NONE: u32 = 0u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_AUTHN_PKU2U: u32 = 31u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_AUTHN_WINNT: u32 = 10u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_AUTHZ_DCE: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_AUTHZ_DEFAULT: u32 = 4294967295u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_AUTHZ_NAME: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_AUTHZ_NONE: u32 = 0u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_BINDING_DEFAULT_TIMEOUT: u32 = 5u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_BINDING_INFINITE_TIMEOUT: u32 = 10u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_BINDING_MAX_TIMEOUT: u32 = 9u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_BINDING_MIN_TIMEOUT: u32 = 0u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_BIND_TO_ALL_NICS: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_CANCEL_INFINITE_TIMEOUT: i32 = -1i32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_DONT_FAIL: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_EP_ALL_ELTS: u32 = 0u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_EP_MATCH_BY_BOTH: u32 = 3u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_EP_MATCH_BY_IF: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_EP_MATCH_BY_OBJ: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_FULL_CERT_CHAIN: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_HTTP_AUTHN_SCHEME_BASIC: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_HTTP_AUTHN_SCHEME_CERT: u32 = 65536u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_HTTP_AUTHN_SCHEME_DIGEST: u32 = 8u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_HTTP_AUTHN_SCHEME_NEGOTIATE: u32 = 16u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_HTTP_AUTHN_SCHEME_NTLM: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_HTTP_AUTHN_SCHEME_PASSPORT: u32 = 4u32;
pub struct RPC_C_HTTP_AUTHN_TARGET(i32);
pub struct RPC_C_HTTP_FLAGS(i32);
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_LISTEN_MAX_CALLS_DEFAULT: u32 = 1234u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_MGMT_INQ_IF_IDS: u32 = 0u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_MGMT_INQ_PRINC_NAME: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_MGMT_INQ_STATS: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_MGMT_IS_SERVER_LISTEN: u32 = 3u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_MGMT_STOP_SERVER_LISTEN: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_MQ_AUTHN_LEVEL_NONE: u32 = 0u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_MQ_AUTHN_LEVEL_PKT_INTEGRITY: u32 = 8u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_MQ_AUTHN_LEVEL_PKT_PRIVACY: u32 = 16u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_MQ_CLEAR_ON_OPEN: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_MQ_EXPRESS: u32 = 0u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_MQ_JOURNAL_ALWAYS: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_MQ_JOURNAL_DEADLETTER: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_MQ_JOURNAL_NONE: u32 = 0u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_MQ_PERMANENT: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_MQ_RECOVERABLE: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_MQ_TEMPORARY: u32 = 0u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_MQ_USE_EXISTING_SECURITY: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_NOTIFY_ON_SEND_COMPLETE: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_NS_DEFAULT_EXP_AGE: i32 = -1i32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_OPT_ASYNC_BLOCK: u32 = 15u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_OPT_BINDING_NONCAUSAL: u32 = 9u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_OPT_CALL_TIMEOUT: u32 = 12u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_OPT_COOKIE_AUTH: u32 = 7u32;
#[cfg(feature = "Win32_Foundation")]
pub struct RPC_C_OPT_COOKIE_AUTH_DESCRIPTOR(i32);
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_OPT_DONT_LINGER: u32 = 13u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_OPT_MAX_OPTIONS: u32 = 12u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_OPT_MQ_ACKNOWLEDGE: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_OPT_MQ_AUTHN_LEVEL: u32 = 6u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_OPT_MQ_AUTHN_SERVICE: u32 = 5u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_OPT_MQ_DELIVERY: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_OPT_MQ_JOURNAL: u32 = 3u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_OPT_MQ_PRIORITY: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_OPT_MQ_TIME_TO_BE_RECEIVED: u32 = 8u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_OPT_MQ_TIME_TO_REACH_QUEUE: u32 = 7u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_OPT_OPTIMIZE_TIME: u32 = 16u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_OPT_PRIVATE_BREAK_ON_SUSPEND: u32 = 3u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_OPT_PRIVATE_DO_NOT_DISTURB: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_OPT_PRIVATE_SUPPRESS_WAKE: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_OPT_RESOURCE_TYPE_UUID: u32 = 8u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_OPT_SECURITY_CALLBACK: u32 = 10u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_OPT_SESSION_ID: u32 = 6u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_OPT_TRANS_SEND_BUFFER_SIZE: u32 = 5u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_OPT_TRUST_PEER: u32 = 14u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_OPT_UNIQUE_BINDING: u32 = 11u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_PARM_BUFFER_LENGTH: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_PARM_MAX_PACKET_LENGTH: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_PROFILE_ALL_ELT: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_PROFILE_ALL_ELTS: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_PROFILE_DEFAULT_ELT: u32 = 0u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_PROFILE_MATCH_BY_BOTH: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_PROFILE_MATCH_BY_IF: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_PROFILE_MATCH_BY_MBR: u32 = 3u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_PROTSEQ_MAX_REQS_DEFAULT: u32 = 10u32;
pub struct RPC_C_QOS_CAPABILITIES(i32);
pub struct RPC_C_QOS_IDENTITY(i32);
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_RPCHTTP_USE_LOAD_BALANCE: u32 = 8u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_SECURITY_QOS_VERSION: i32 = 1i32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_SECURITY_QOS_VERSION_1: i32 = 1i32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_SECURITY_QOS_VERSION_2: i32 = 2i32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_SECURITY_QOS_VERSION_3: i32 = 3i32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_SECURITY_QOS_VERSION_4: i32 = 4i32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_SECURITY_QOS_VERSION_5: i32 = 5i32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_STATS_CALLS_IN: u32 = 0u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_STATS_CALLS_OUT: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_STATS_PKTS_IN: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_STATS_PKTS_OUT: u32 = 3u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_TRY_ENFORCE_MAX_CALLS: u32 = 16u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_USE_INTERNET_PORT: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_USE_INTRANET_PORT: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_VERS_ALL: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_VERS_COMPATIBLE: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_VERS_EXACT: u32 = 3u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_VERS_MAJOR_ONLY: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_VERS_UPTO: u32 = 5u32;
pub struct RPC_DISPATCH_FUNCTION(i32);
pub struct RPC_DISPATCH_TABLE(i32);
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_EEINFO_VERSION: u32 = 1u32;
#[cfg(feature = "Win32_Foundation")]
pub struct RPC_EE_INFO_PARAM(i32);
pub struct RPC_ENDPOINT_TEMPLATEA(i32);
pub struct RPC_ENDPOINT_TEMPLATEW(i32);
pub struct RPC_ERROR_ENUM_HANDLE(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct RPC_EXTENDED_ERROR_INFO(i32);
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_FLAGS_VALID_BIT: u32 = 32768u32;
pub struct RPC_FORWARD_FUNCTION(i32);
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_FW_IF_FLAG_DCOM: u32 = 1u32;
pub struct RPC_HTTP_PROXY_FREE_STRING(i32);
pub struct RPC_HTTP_REDIRECTOR_STAGE(i32);
pub struct RPC_HTTP_TRANSPORT_CREDENTIALS_A(i32);
pub struct RPC_HTTP_TRANSPORT_CREDENTIALS_V2_A(i32);
pub struct RPC_HTTP_TRANSPORT_CREDENTIALS_V2_W(i32);
pub struct RPC_HTTP_TRANSPORT_CREDENTIALS_V3_A(i32);
pub struct RPC_HTTP_TRANSPORT_CREDENTIALS_V3_W(i32);
pub struct RPC_HTTP_TRANSPORT_CREDENTIALS_W(i32);
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_IF_ALLOW_CALLBACKS_WITH_NO_AUTH: u32 = 16u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_IF_ALLOW_LOCAL_ONLY: u32 = 32u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_IF_ALLOW_SECURE_ONLY: u32 = 8u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_IF_ALLOW_UNKNOWN_AUTHORITY: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_IF_ASYNC_CALLBACK: u32 = 256u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_IF_AUTOLISTEN: u32 = 1u32;
pub struct RPC_IF_CALLBACK_FN(i32);
pub struct RPC_IF_ID(i32);
pub struct RPC_IF_ID_VECTOR(i32);
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_IF_OLE: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_IF_SEC_CACHE_PER_PROC: u32 = 128u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_IF_SEC_NO_CACHE: u32 = 64u32;
pub struct RPC_IMPORT_CONTEXT_P(i32);
pub struct RPC_INTERFACE_GROUP_IDLE_CALLBACK_FN(i32);
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_INTERFACE_HAS_PIPES: u32 = 1u32;
pub struct RPC_INTERFACE_TEMPLATEA(i32);
pub struct RPC_INTERFACE_TEMPLATEW(i32);
pub struct RPC_MESSAGE(i32);
pub struct RPC_MGMT_AUTHORIZATION_FN(i32);
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_NCA_FLAGS_BROADCAST: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_NCA_FLAGS_DEFAULT: u32 = 0u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_NCA_FLAGS_IDEMPOTENT: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_NCA_FLAGS_MAYBE: u32 = 4u32;
pub struct RPC_NEW_HTTP_PROXY_CHANNEL(i32);
pub struct RPC_NOTIFICATIONS(i32);
pub struct RPC_NOTIFICATION_TYPES(i32);
pub struct RPC_OBJECT_INQ_FN(i32);
pub struct RPC_POLICY(i32);
pub struct RPC_PROTSEQ_ENDPOINT(i32);
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_PROTSEQ_HTTP: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_PROTSEQ_LRPC: u32 = 3u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_PROTSEQ_NMP: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_PROTSEQ_TCP: u32 = 1u32;
pub struct RPC_PROTSEQ_VECTORA(i32);
pub struct RPC_PROTSEQ_VECTORW(i32);
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_PROXY_CONNECTION_TYPE_IN_PROXY: u32 = 0u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_PROXY_CONNECTION_TYPE_OUT_PROXY: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_P_ADDR_FORMAT_TCP_IPV4: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_P_ADDR_FORMAT_TCP_IPV6: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_QUERY_CALL_LOCAL_ADDRESS: u32 = 8u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_QUERY_CLIENT_ID: u32 = 128u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_QUERY_CLIENT_PID: u32 = 16u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_QUERY_CLIENT_PRINCIPAL_NAME: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_QUERY_IS_CLIENT_LOCAL: u32 = 32u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_QUERY_NO_AUTH_REQUIRED: u32 = 64u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_QUERY_SERVER_PRINCIPAL_NAME: u32 = 2u32;
pub struct RPC_SECURITY_CALLBACK_FN(i32);
#[cfg(feature = "Win32_System_Com")]
pub struct RPC_SECURITY_QOS(i32);
#[cfg(feature = "Win32_System_Com")]
pub struct RPC_SECURITY_QOS_V2_A(i32);
#[cfg(feature = "Win32_System_Com")]
pub struct RPC_SECURITY_QOS_V2_W(i32);
#[cfg(feature = "Win32_System_Com")]
pub struct RPC_SECURITY_QOS_V3_A(i32);
#[cfg(feature = "Win32_System_Com")]
pub struct RPC_SECURITY_QOS_V3_W(i32);
#[cfg(feature = "Win32_System_Com")]
pub struct RPC_SECURITY_QOS_V4_A(i32);
#[cfg(feature = "Win32_System_Com")]
pub struct RPC_SECURITY_QOS_V4_W(i32);
#[cfg(feature = "Win32_System_Com")]
pub struct RPC_SECURITY_QOS_V5_A(i32);
#[cfg(feature = "Win32_System_Com")]
pub struct RPC_SECURITY_QOS_V5_W(i32);
pub struct RPC_SEC_CONTEXT_KEY_INFO(i32);
pub struct RPC_SERVER_INTERFACE(i32);
pub struct RPC_SETFILTER_FUNC(i32);
pub struct RPC_STATS_VECTOR(i32);
pub struct RPC_STATUS(i32);
pub struct RPC_SYNTAX_IDENTIFIER(i32);
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_SYSTEM_HANDLE_FREE_ALL: u32 = 3u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_SYSTEM_HANDLE_FREE_ERROR_ON_CLOSE: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_SYSTEM_HANDLE_FREE_RETRIEVED: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_SYSTEM_HANDLE_FREE_UNRETRIEVED: u32 = 1u32;
pub struct RPC_TRANSFER_SYNTAX(i32);
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_TYPE_DISCONNECT_EVENT_CONTEXT_HANDLE: u32 = 2147483648u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_TYPE_STRICT_CONTEXT_HANDLE: u32 = 1073741824u32;
pub struct RPC_VERSION(i32);
pub struct RpcCallClientLocality(i32);
pub struct RpcCallType(i32);
pub struct RpcLocalAddressFormat(i32);
pub struct RpcProxyPerfCounters(i32);
pub struct SCONTEXT_QUEUE(i32);
pub struct SEC_WINNT_AUTH_IDENTITY(i32);
pub struct SEC_WINNT_AUTH_IDENTITY_A(i32);
pub struct SEC_WINNT_AUTH_IDENTITY_W(i32);
pub struct SERVER_ROUTINE(i32);
pub struct STUB_PHASE(i32);
pub struct STUB_THUNK(i32);
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const TARGET_IS_NT100_OR_LATER: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const TARGET_IS_NT351_OR_WIN95_OR_LATER: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const TARGET_IS_NT40_OR_LATER: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const TARGET_IS_NT50_OR_LATER: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const TARGET_IS_NT51_OR_LATER: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const TARGET_IS_NT60_OR_LATER: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const TARGET_IS_NT61_OR_LATER: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const TARGET_IS_NT62_OR_LATER: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const TARGET_IS_NT63_OR_LATER: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const TRANSPORT_TYPE_CN: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const TRANSPORT_TYPE_DG: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const TRANSPORT_TYPE_LPC: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const TRANSPORT_TYPE_WMSG: u32 = 8u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const USER_CALL_IS_ASYNC: u32 = 256u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const USER_CALL_NEW_CORRELATION_DESC: u32 = 512u32;
#[cfg(feature = "Win32_System_Com")]
pub struct USER_MARSHAL_CB(i32);
pub struct USER_MARSHAL_CB_TYPE(i32);
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const USER_MARSHAL_FC_BYTE: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const USER_MARSHAL_FC_CHAR: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const USER_MARSHAL_FC_DOUBLE: u32 = 12u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const USER_MARSHAL_FC_FLOAT: u32 = 10u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const USER_MARSHAL_FC_HYPER: u32 = 11u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const USER_MARSHAL_FC_LONG: u32 = 8u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const USER_MARSHAL_FC_SHORT: u32 = 6u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const USER_MARSHAL_FC_SMALL: u32 = 3u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const USER_MARSHAL_FC_ULONG: u32 = 9u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const USER_MARSHAL_FC_USHORT: u32 = 7u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const USER_MARSHAL_FC_USMALL: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const USER_MARSHAL_FC_WCHAR: u32 = 5u32;
pub struct USER_MARSHAL_FREEING_ROUTINE(i32);
pub struct USER_MARSHAL_MARSHALLING_ROUTINE(i32);
pub struct USER_MARSHAL_ROUTINE_QUADRUPLE(i32);
pub struct USER_MARSHAL_SIZING_ROUTINE(i32);
pub struct USER_MARSHAL_UNMARSHALLING_ROUTINE(i32);
pub struct UUID_VECTOR(i32);
pub struct XLAT_SIDE(i32);
pub struct XMIT_HELPER_ROUTINE(i32);
#[cfg(feature = "Win32_System_Com")]
pub struct XMIT_ROUTINE_QUINTUPLE(i32);
pub struct _NDR_ASYNC_MESSAGE(i32);
pub struct _NDR_CORRELATION_INFO(i32);
pub struct _NDR_PROC_CONTEXT(i32);
pub struct _NDR_SCONTEXT(i32);
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const __RPCPROXY_H_VERSION__: u32 = 475u32;
pub struct system_handle_t(i32);
