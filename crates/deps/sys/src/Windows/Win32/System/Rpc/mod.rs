#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn DceErrorInqTextA();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn DceErrorInqTextW();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn IUnknown_AddRef_Proxy();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn IUnknown_QueryInterface_Proxy();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn IUnknown_Release_Proxy();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn I_RpcAllocate();
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn I_RpcAsyncAbortCall();
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn I_RpcAsyncSetHandle();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn I_RpcBindingCopy();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn I_RpcBindingCreateNP();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn I_RpcBindingHandleToAsyncHandle();
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn I_RpcBindingInqClientTokenAttributes();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn I_RpcBindingInqDynamicEndpointA();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn I_RpcBindingInqDynamicEndpointW();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn I_RpcBindingInqLocalClientPID();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn I_RpcBindingInqMarshalledTargetInfo();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn I_RpcBindingInqSecurityContext();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn I_RpcBindingInqSecurityContextKeyInfo();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn I_RpcBindingInqTransportType();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn I_RpcBindingInqWireIdForSnego();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn I_RpcBindingIsClientLocal();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn I_RpcBindingIsServerLocal();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn I_RpcBindingSetPrivateOption();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn I_RpcBindingToStaticStringBindingW();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn I_RpcClearMutex();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn I_RpcDeleteMutex();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn I_RpcExceptionFilter();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn I_RpcFree();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn I_RpcFreeBuffer();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn I_RpcFreePipeBuffer();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn I_RpcGetBuffer();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn I_RpcGetBufferWithObject();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn I_RpcGetCurrentCallHandle();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn I_RpcGetDefaultSD();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn I_RpcGetExtendedError();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn I_RpcIfInqTransferSyntaxes();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn I_RpcMapWin32Status();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn I_RpcMgmtEnableDedicatedThreadPool();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn I_RpcNegotiateTransferSyntax();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn I_RpcNsBindingSetEntryNameA();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn I_RpcNsBindingSetEntryNameW();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn I_RpcNsGetBuffer();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn I_RpcNsInterfaceExported();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn I_RpcNsInterfaceUnexported();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn I_RpcNsRaiseException();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn I_RpcNsSendReceive();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn I_RpcOpenClientProcess();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn I_RpcPauseExecution();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn I_RpcReBindBuffer();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn I_RpcReallocPipeBuffer();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn I_RpcReceive();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn I_RpcRecordCalloutFailure();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn I_RpcRequestMutex();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn I_RpcSend();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn I_RpcSendReceive();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn I_RpcServerCheckClientRestriction();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn I_RpcServerDisableExceptionFilter();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn I_RpcServerGetAssociationID();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn I_RpcServerInqAddressChangeFn();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn I_RpcServerInqLocalConnAddress();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn I_RpcServerInqRemoteConnAddress();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn I_RpcServerInqTransportType();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn I_RpcServerRegisterForwardFunction();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn I_RpcServerSetAddressChangeFn();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn I_RpcServerStartService();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn I_RpcServerSubscribeForDisconnectNotification();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn I_RpcServerSubscribeForDisconnectNotification2();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn I_RpcServerUnsubscribeForDisconnectNotification();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn I_RpcServerUseProtseq2A();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn I_RpcServerUseProtseq2W();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn I_RpcServerUseProtseqEp2A();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn I_RpcServerUseProtseqEp2W();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn I_RpcSessionStrictContextHandle();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn I_RpcSsDontSerializeContext();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn I_RpcSystemHandleTypeSpecificWork();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn I_RpcTurnOnEEInfoPropagation();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn I_UuidCreate();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn MesBufferHandleReset();
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MesDecodeBufferHandleCreate();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn MesDecodeIncrementalHandleCreate();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn MesEncodeDynBufferHandleCreate();
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MesEncodeFixedBufferHandleCreate();
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MesEncodeIncrementalHandleCreate();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn MesHandleFree();
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MesIncrementalHandleReset();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn MesInqProcEncodingId();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn NDRCContextBinding();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn NDRCContextMarshall();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn NDRCContextUnmarshall();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn NDRSContextMarshall();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn NDRSContextMarshall2();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn NDRSContextMarshallEx();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn NDRSContextUnmarshall();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn NDRSContextUnmarshall2();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn NDRSContextUnmarshallEx();
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn Ndr64AsyncClientCall();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn Ndr64AsyncServerCall64();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn Ndr64AsyncServerCallAll();
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn Ndr64DcomAsyncClientCall();
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn Ndr64DcomAsyncStubCall();
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrAllocate();
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrAsyncClientCall();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn NdrAsyncServerCall();
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrByteCountPointerBufferSize();
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrByteCountPointerFree();
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrByteCountPointerMarshall();
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrByteCountPointerUnmarshall();
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrClearOutParameters();
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrClientCall2();
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrClientCall3();
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrClientContextMarshall();
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrClientContextUnmarshall();
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrClientInitialize();
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrClientInitializeNew();
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrComplexArrayBufferSize();
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrComplexArrayFree();
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrComplexArrayMarshall();
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrComplexArrayMemorySize();
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrComplexArrayUnmarshall();
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrComplexStructBufferSize();
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrComplexStructFree();
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrComplexStructMarshall();
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrComplexStructMemorySize();
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrComplexStructUnmarshall();
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrConformantArrayBufferSize();
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrConformantArrayFree();
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrConformantArrayMarshall();
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrConformantArrayMemorySize();
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrConformantArrayUnmarshall();
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrConformantStringBufferSize();
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrConformantStringMarshall();
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrConformantStringMemorySize();
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrConformantStringUnmarshall();
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrConformantStructBufferSize();
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrConformantStructFree();
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrConformantStructMarshall();
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrConformantStructMemorySize();
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrConformantStructUnmarshall();
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrConformantVaryingArrayBufferSize();
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrConformantVaryingArrayFree();
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrConformantVaryingArrayMarshall();
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrConformantVaryingArrayMemorySize();
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrConformantVaryingArrayUnmarshall();
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrConformantVaryingStructBufferSize();
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrConformantVaryingStructFree();
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrConformantVaryingStructMarshall();
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrConformantVaryingStructMemorySize();
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrConformantVaryingStructUnmarshall();
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrContextHandleInitialize();
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrContextHandleSize();
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrConvert();
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrConvert2();
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrCorrelationFree();
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrCorrelationInitialize();
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrCorrelationPass();
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrCreateServerInterfaceFromStub();
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrDcomAsyncClientCall();
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrDcomAsyncStubCall();
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrEncapsulatedUnionBufferSize();
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrEncapsulatedUnionFree();
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrEncapsulatedUnionMarshall();
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrEncapsulatedUnionMemorySize();
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrEncapsulatedUnionUnmarshall();
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrFixedArrayBufferSize();
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrFixedArrayFree();
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrFixedArrayMarshall();
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrFixedArrayMemorySize();
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrFixedArrayUnmarshall();
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrFreeBuffer();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn NdrFullPointerXlatFree();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn NdrFullPointerXlatInit();
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrGetBuffer();
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrGetDcomProtocolVersion();
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrGetUserMarshalInfo();
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrInterfacePointerBufferSize();
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrInterfacePointerFree();
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrInterfacePointerMarshall();
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrInterfacePointerMemorySize();
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrInterfacePointerUnmarshall();
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrMapCommAndFaultStatus();
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrMesProcEncodeDecode();
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrMesProcEncodeDecode2();
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrMesProcEncodeDecode3();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn NdrMesSimpleTypeAlignSize();
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrMesSimpleTypeAlignSizeAll();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn NdrMesSimpleTypeDecode();
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrMesSimpleTypeDecodeAll();
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrMesSimpleTypeEncode();
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrMesSimpleTypeEncodeAll();
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrMesTypeAlignSize();
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrMesTypeAlignSize2();
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrMesTypeAlignSize3();
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrMesTypeDecode();
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrMesTypeDecode2();
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrMesTypeDecode3();
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrMesTypeEncode();
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrMesTypeEncode2();
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrMesTypeEncode3();
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrMesTypeFree2();
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrMesTypeFree3();
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrNonConformantStringBufferSize();
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrNonConformantStringMarshall();
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrNonConformantStringMemorySize();
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrNonConformantStringUnmarshall();
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrNonEncapsulatedUnionBufferSize();
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrNonEncapsulatedUnionFree();
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrNonEncapsulatedUnionMarshall();
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrNonEncapsulatedUnionMemorySize();
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrNonEncapsulatedUnionUnmarshall();
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrNsGetBuffer();
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrNsSendReceive();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn NdrOleAllocate();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn NdrOleFree();
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrPartialIgnoreClientBufferSize();
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrPartialIgnoreClientMarshall();
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrPartialIgnoreServerInitialize();
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrPartialIgnoreServerUnmarshall();
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrPointerBufferSize();
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrPointerFree();
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrPointerMarshall();
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrPointerMemorySize();
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrPointerUnmarshall();
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrRangeUnmarshall();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn NdrRpcSmClientAllocate();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn NdrRpcSmClientFree();
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrRpcSmSetClientToOsf();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn NdrRpcSsDefaultAllocate();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn NdrRpcSsDefaultFree();
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrRpcSsDisableAllocate();
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrRpcSsEnableAllocate();
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrSendReceive();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn NdrServerCall2();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn NdrServerCallAll();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn NdrServerCallNdr64();
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrServerContextMarshall();
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrServerContextNewMarshall();
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrServerContextNewUnmarshall();
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrServerContextUnmarshall();
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrServerInitialize();
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrServerInitializeMarshall();
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrServerInitializeNew();
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrServerInitializePartial();
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrServerInitializeUnmarshall();
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrSimpleStructBufferSize();
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrSimpleStructFree();
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrSimpleStructMarshall();
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrSimpleStructMemorySize();
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrSimpleStructUnmarshall();
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrSimpleTypeMarshall();
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrSimpleTypeUnmarshall();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn NdrStubCall2();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn NdrStubCall3();
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrUserMarshalBufferSize();
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrUserMarshalFree();
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrUserMarshalMarshall();
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrUserMarshalMemorySize();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn NdrUserMarshalSimpleTypeConvert();
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrUserMarshalUnmarshall();
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrVaryingArrayBufferSize();
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrVaryingArrayFree();
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrVaryingArrayMarshall();
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrVaryingArrayMemorySize();
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrVaryingArrayUnmarshall();
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrXmitOrRepAsBufferSize();
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrXmitOrRepAsFree();
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrXmitOrRepAsMarshall();
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrXmitOrRepAsMemorySize();
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn NdrXmitOrRepAsUnmarshall();
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn RpcAsyncAbortCall();
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn RpcAsyncCancelCall();
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn RpcAsyncCompleteCall();
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn RpcAsyncGetCallStatus();
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn RpcAsyncInitializeHandle();
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn RpcAsyncRegisterInfo();
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn RpcBindingBind();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcBindingCopy();
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn RpcBindingCreateA();
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn RpcBindingCreateW();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcBindingFree();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcBindingFromStringBindingA();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcBindingFromStringBindingW();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcBindingInqAuthClientA();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcBindingInqAuthClientExA();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcBindingInqAuthClientExW();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcBindingInqAuthClientW();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcBindingInqAuthInfoA();
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn RpcBindingInqAuthInfoExA();
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn RpcBindingInqAuthInfoExW();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcBindingInqAuthInfoW();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcBindingInqMaxCalls();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcBindingInqObject();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcBindingInqOption();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcBindingReset();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcBindingServerFromClient();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcBindingSetAuthInfoA();
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn RpcBindingSetAuthInfoExA();
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn RpcBindingSetAuthInfoExW();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcBindingSetAuthInfoW();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcBindingSetObject();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcBindingSetOption();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcBindingToStringBindingA();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcBindingToStringBindingW();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcBindingUnbind();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcBindingVectorFree();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcCancelThread();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcCancelThreadEx();
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_Foundation`, `Win32_Security_Cryptography`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
    pub fn RpcCertGeneratePrincipalNameA();
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_Foundation`, `Win32_Security_Cryptography`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
    pub fn RpcCertGeneratePrincipalNameW();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcEpRegisterA();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcEpRegisterNoReplaceA();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcEpRegisterNoReplaceW();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcEpRegisterW();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcEpResolveBinding();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcEpUnregister();
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RpcErrorAddRecord();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcErrorClearInformation();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcErrorEndEnumeration();
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RpcErrorGetNextRecord();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcErrorGetNumberOfRecords();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcErrorLoadErrorInfo();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcErrorResetEnumeration();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcErrorSaveErrorInfo();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcErrorStartEnumeration();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcExceptionFilter();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcFreeAuthorizationContext();
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RpcGetAuthorizationContextForClient();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcIfIdVectorFree();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcIfInqId();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcImpersonateClient();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcImpersonateClient2();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcImpersonateClientContainer();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcMgmtEnableIdleCleanup();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcMgmtEpEltInqBegin();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcMgmtEpEltInqDone();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcMgmtEpEltInqNextA();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcMgmtEpEltInqNextW();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcMgmtEpUnregister();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcMgmtInqComTimeout();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcMgmtInqDefaultProtectLevel();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcMgmtInqIfIds();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcMgmtInqServerPrincNameA();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcMgmtInqServerPrincNameW();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcMgmtInqStats();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcMgmtIsServerListening();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcMgmtSetAuthorizationFn();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcMgmtSetCancelTimeout();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcMgmtSetComTimeout();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcMgmtSetServerStackSize();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcMgmtStatsVectorFree();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcMgmtStopServerListening();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcMgmtWaitServerListen();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcNetworkInqProtseqsA();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcNetworkInqProtseqsW();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcNetworkIsProtseqValidA();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcNetworkIsProtseqValidW();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcNsBindingExportA();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcNsBindingExportPnPA();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcNsBindingExportPnPW();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcNsBindingExportW();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcNsBindingImportBeginA();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcNsBindingImportBeginW();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcNsBindingImportDone();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcNsBindingImportNext();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcNsBindingInqEntryNameA();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcNsBindingInqEntryNameW();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcNsBindingLookupBeginA();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcNsBindingLookupBeginW();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcNsBindingLookupDone();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcNsBindingLookupNext();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcNsBindingSelect();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcNsBindingUnexportA();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcNsBindingUnexportPnPA();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcNsBindingUnexportPnPW();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcNsBindingUnexportW();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcNsEntryExpandNameA();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcNsEntryExpandNameW();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcNsEntryObjectInqBeginA();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcNsEntryObjectInqBeginW();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcNsEntryObjectInqDone();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcNsEntryObjectInqNext();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcNsGroupDeleteA();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcNsGroupDeleteW();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcNsGroupMbrAddA();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcNsGroupMbrAddW();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcNsGroupMbrInqBeginA();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcNsGroupMbrInqBeginW();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcNsGroupMbrInqDone();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcNsGroupMbrInqNextA();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcNsGroupMbrInqNextW();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcNsGroupMbrRemoveA();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcNsGroupMbrRemoveW();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcNsMgmtBindingUnexportA();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcNsMgmtBindingUnexportW();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcNsMgmtEntryCreateA();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcNsMgmtEntryCreateW();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcNsMgmtEntryDeleteA();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcNsMgmtEntryDeleteW();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcNsMgmtEntryInqIfIdsA();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcNsMgmtEntryInqIfIdsW();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcNsMgmtHandleSetExpAge();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcNsMgmtInqExpAge();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcNsMgmtSetExpAge();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcNsProfileDeleteA();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcNsProfileDeleteW();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcNsProfileEltAddA();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcNsProfileEltAddW();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcNsProfileEltInqBeginA();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcNsProfileEltInqBeginW();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcNsProfileEltInqDone();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcNsProfileEltInqNextA();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcNsProfileEltInqNextW();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcNsProfileEltRemoveA();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcNsProfileEltRemoveW();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcObjectInqType();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcObjectSetInqFn();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcObjectSetType();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcProtseqVectorFreeA();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcProtseqVectorFreeW();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcRaiseException();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcRevertContainerImpersonation();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcRevertToSelf();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcRevertToSelfEx();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcServerCompleteSecurityCallback();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcServerInqBindingHandle();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcServerInqBindings();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcServerInqBindingsEx();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcServerInqCallAttributesA();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcServerInqCallAttributesW();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcServerInqDefaultPrincNameA();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcServerInqDefaultPrincNameW();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcServerInqIf();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcServerInterfaceGroupActivate();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcServerInterfaceGroupClose();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcServerInterfaceGroupCreateA();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcServerInterfaceGroupCreateW();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcServerInterfaceGroupDeactivate();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcServerInterfaceGroupInqBindings();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcServerListen();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcServerRegisterAuthInfoA();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcServerRegisterAuthInfoW();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcServerRegisterIf();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcServerRegisterIf2();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcServerRegisterIf3();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcServerRegisterIfEx();
    #[doc = "*Required features: `Win32_System_Rpc`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn RpcServerSubscribeForNotification();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcServerTestCancel();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcServerUnregisterIf();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcServerUnregisterIfEx();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcServerUnsubscribeForNotification();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcServerUseAllProtseqs();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcServerUseAllProtseqsEx();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcServerUseAllProtseqsIf();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcServerUseAllProtseqsIfEx();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcServerUseProtseqA();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcServerUseProtseqEpA();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcServerUseProtseqEpExA();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcServerUseProtseqEpExW();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcServerUseProtseqEpW();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcServerUseProtseqExA();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcServerUseProtseqExW();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcServerUseProtseqIfA();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcServerUseProtseqIfExA();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcServerUseProtseqIfExW();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcServerUseProtseqIfW();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcServerUseProtseqW();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcServerYield();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcSmAllocate();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcSmClientFree();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcSmDestroyClientContext();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcSmDisableAllocate();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcSmEnableAllocate();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcSmFree();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcSmGetThreadHandle();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcSmSetClientAllocFree();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcSmSetThreadHandle();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcSmSwapClientAllocFree();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcSsAllocate();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcSsContextLockExclusive();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcSsContextLockShared();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcSsDestroyClientContext();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcSsDisableAllocate();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcSsDontSerializeContext();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcSsEnableAllocate();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcSsFree();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcSsGetContextBinding();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcSsGetThreadHandle();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcSsSetClientAllocFree();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcSsSetThreadHandle();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcSsSwapClientAllocFree();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcStringBindingComposeA();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcStringBindingComposeW();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcStringBindingParseA();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcStringBindingParseW();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcStringFreeA();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcStringFreeW();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcTestCancel();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn RpcUserFree();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn UuidCompare();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn UuidCreate();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn UuidCreateNil();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn UuidCreateSequential();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn UuidEqual();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn UuidFromStringA();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn UuidFromStringW();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn UuidHash();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn UuidIsNil();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn UuidToStringA();
    #[doc = "*Required features: `Win32_System_Rpc`*"]
    pub fn UuidToStringW();
}
