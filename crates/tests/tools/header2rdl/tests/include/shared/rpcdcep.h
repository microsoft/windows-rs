#include <winapifamily.h>

/*++

Copyright (c) Microsoft Corporation. All rights reserved.

Module Name:

    rpcdcep.h

Abstract:

    This module contains the private RPC runtime APIs for use by the
    stubs and by support libraries.  Applications must not call these
    routines.

--*/

//@[contract("rpcdcep"), comment("MVI_tracked - https://osgwiki.com/wiki/Microsoft_Virus_Initiative")]; 

#ifndef __RPCDCEP_H__
#define __RPCDCEP_H__

#if _MSC_VER > 1000
#pragma once
#endif

#ifdef __cplusplus
extern "C" {
#endif

#if _MSC_VER >= 1200
#pragma warning(push)
#pragma warning(disable:4668) // #if not_defined treated as #if 0
#pragma warning(disable:4820) // padding added after data member
#endif

#pragma region Application Family or OneCore Family or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

typedef struct _RPC_VERSION {
    unsigned short MajorVersion;
    unsigned short MinorVersion;
} RPC_VERSION;

typedef struct _RPC_SYNTAX_IDENTIFIER {
    GUID SyntaxGUID;
    RPC_VERSION SyntaxVersion;
} RPC_SYNTAX_IDENTIFIER, __RPC_FAR * PRPC_SYNTAX_IDENTIFIER;

typedef struct _RPC_MESSAGE
{
    RPC_BINDING_HANDLE Handle;
    unsigned long DataRepresentation;
    void __RPC_FAR * Buffer;
    unsigned int BufferLength;
    unsigned int ProcNum;
    PRPC_SYNTAX_IDENTIFIER TransferSyntax;
    void __RPC_FAR * RpcInterfaceInformation;
    void __RPC_FAR * ReservedForRuntime;
    RPC_MGR_EPV __RPC_FAR * ManagerEpv;
    void __RPC_FAR * ImportContext;
    unsigned long RpcFlags;
} RPC_MESSAGE, __RPC_FAR * PRPC_MESSAGE;

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

typedef RPC_STATUS
RPC_ENTRY RPC_FORWARD_FUNCTION(
                       IN UUID             __RPC_FAR * InterfaceId,
                       IN RPC_VERSION      __RPC_FAR * InterfaceVersion,
                       IN UUID             __RPC_FAR * ObjectId,
                       IN unsigned char         __RPC_FAR * Rpcpro,
                       IN void __RPC_FAR * __RPC_FAR * ppDestEndpoint);

enum RPC_ADDRESS_CHANGE_TYPE
{
    PROTOCOL_NOT_LOADED = 1,
    PROTOCOL_LOADED,
    PROTOCOL_ADDRESS_CHANGE
};

typedef void
RPC_ENTRY RPC_ADDRESS_CHANGE_FN(
                        IN void * arg
                        );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#pragma region Application Family or OneCore Family or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

/*
*  New context handle flavors.
*/
#define RPC_CONTEXT_HANDLE_DEFAULT_GUARD    ((void *)(ULONG_PTR)0xFFFFF00D)

#define RPC_CONTEXT_HANDLE_DEFAULT_FLAGS    0x00000000UL
#define RPC_CONTEXT_HANDLE_FLAGS            0x30000000UL
#define RPC_CONTEXT_HANDLE_SERIALIZE        0x10000000UL
#define RPC_CONTEXT_HANDLE_DONT_SERIALIZE   0x20000000UL
#if (NTDDI_VERSION >= NTDDI_VISTA)
#define RPC_TYPE_STRICT_CONTEXT_HANDLE      0x40000000UL
#endif // (NTDDI_VERSION >= NTDDI_VISTA)
#if (NTDDI_VERSION >= NTDDI_WIN10_RS5)
#define RPC_TYPE_DISCONNECT_EVENT_CONTEXT_HANDLE  0x80000000UL
#endif // (NTDDI_VERSION >= NTDDI_WIN10_RS5)

/*
 * Types of function calls for datagram rpc
 */

#define RPC_NCA_FLAGS_DEFAULT       0x00000000  /* 0b000...000 */
#define RPC_NCA_FLAGS_IDEMPOTENT    0x00000001  /* 0b000...001 */
#define RPC_NCA_FLAGS_BROADCAST     0x00000002  /* 0b000...010 */
#define RPC_NCA_FLAGS_MAYBE         0x00000004  /* 0b000...100 */

#if (NTDDI_VERSION >= NTDDI_VISTA)
#define RPCFLG_HAS_GUARANTEE        0x00000010UL
#endif // (NTDDI_VERSION >= NTDDI_VISTA)

#define RPCFLG_WINRT_REMOTE_ASYNC   0x00000020UL

/* Flags used in RpcFlag field of RPC_MESSAGE structure */

#define RPC_BUFFER_COMPLETE         0x00001000 /* used by pipes */
#define RPC_BUFFER_PARTIAL          0x00002000 /* used by pipes */
#define RPC_BUFFER_EXTRA            0x00004000 /* used by pipes */
#define RPC_BUFFER_ASYNC            0x00008000 /* used by async rpc */
#define RPC_BUFFER_NONOTIFY         0x00010000 /* used by async pipes */

#define RPCFLG_MESSAGE              0x01000000UL
#define RPCFLG_AUTO_COMPLETE        0x08000000UL
#define RPCFLG_LOCAL_CALL           0x10000000UL
#define RPCFLG_INPUT_SYNCHRONOUS    0x20000000UL
#define RPCFLG_ASYNCHRONOUS         0x40000000UL
#define RPCFLG_NON_NDR              0x80000000UL

#if (NTDDI_VERSION >= NTDDI_WINXP)
#define RPCFLG_HAS_MULTI_SYNTAXES   0x02000000UL
#define RPCFLG_HAS_CALLBACK         0x04000000UL
#endif // (NTDDI_VERSION >= NTDDI_WXP)

#if (NTDDI_VERSION >= NTDDI_VISTA)
// These two bits will hold the combination of
// anonymous/admin/authenticate/mixed mode
#define RPCFLG_ACCESSIBILITY_BIT1   0x00100000UL
#define RPCFLG_ACCESSIBILITY_BIT2   0x00200000UL
#define RPCFLG_ACCESS_LOCAL         0x00400000UL

// This goes to MIDL_STUB_DESC only
#define NDR_CUSTOM_OR_DEFAULT_ALLOCATOR 0x10000000UL
#define NDR_DEFAULT_ALLOCATOR           0x20000000UL
#endif // (NTDDI_VERSION >= NTDDI_VISTA)

#if (NTDDI_VERSION >= NTDDI_WIN8)
// NDR64 on ARM includes ARM parameter layout info
#define RPCFLG_NDR64_CONTAINS_ARM_LAYOUT 0x04000000UL
#endif

#if (NTDDI_VERSION >= NTDDI_WINBLUE)
/*
 * Used by COM to inform RPC that even though this is an
 * async RPC call, there is a thread waiting for the call
 * to complete. Essentially, from COM perspective, this is a
 * sync call. This flag will be passed down to ALPC in order
 * to count the wake charges properly. 
 */

#define RPCFLG_SENDER_WAITING_FOR_REPLY 0x00800000UL 

#endif // (NTDDI_VERSION >= NTDDI_WINBLUE)

#define RPC_FLAGS_VALID_BIT 0x00008000

typedef
void
(__RPC_STUB __RPC_FAR * RPC_DISPATCH_FUNCTION) (
    IN OUT PRPC_MESSAGE Message
    );

typedef struct {
    unsigned int DispatchTableCount;
    RPC_DISPATCH_FUNCTION __RPC_FAR * DispatchTable;
    LONG_PTR                          Reserved;
} RPC_DISPATCH_TABLE, __RPC_FAR * PRPC_DISPATCH_TABLE;

typedef struct _RPC_PROTSEQ_ENDPOINT
{
    unsigned char __RPC_FAR * RpcProtocolSequence;
    unsigned char __RPC_FAR * Endpoint;
} RPC_PROTSEQ_ENDPOINT, __RPC_FAR * PRPC_PROTSEQ_ENDPOINT;

/*
Both of these types MUST start with the InterfaceId and TransferSyntax.
Look at RpcIfInqId and I_RpcIfInqTransferSyntaxes to see why.
*/
#define NT351_INTERFACE_SIZE 0x40
#define RPC_INTERFACE_HAS_PIPES           0x0001

typedef struct _RPC_SERVER_INTERFACE
{
    unsigned int Length;
    RPC_SYNTAX_IDENTIFIER InterfaceId;
    RPC_SYNTAX_IDENTIFIER TransferSyntax;
    PRPC_DISPATCH_TABLE DispatchTable;
    unsigned int RpcProtseqEndpointCount;
    PRPC_PROTSEQ_ENDPOINT RpcProtseqEndpoint;
    RPC_MGR_EPV __RPC_FAR *DefaultManagerEpv;
    void const __RPC_FAR *InterpreterInfo;
    unsigned int Flags ;
} RPC_SERVER_INTERFACE, __RPC_FAR * PRPC_SERVER_INTERFACE;

typedef struct _RPC_CLIENT_INTERFACE
{
    unsigned int Length;
    RPC_SYNTAX_IDENTIFIER   InterfaceId;
    RPC_SYNTAX_IDENTIFIER   TransferSyntax;
    PRPC_DISPATCH_TABLE     DispatchTable;
    unsigned int            RpcProtseqEndpointCount;
    PRPC_PROTSEQ_ENDPOINT   RpcProtseqEndpoint;
    ULONG_PTR               Reserved;
    void const __RPC_FAR *  InterpreterInfo;
    unsigned int Flags ;
} RPC_CLIENT_INTERFACE, __RPC_FAR * PRPC_CLIENT_INTERFACE;

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

#if (NTDDI_VERSION >= NTDDI_WINXP)
RPCRTAPI
RPC_STATUS
RPC_ENTRY
I_RpcNegotiateTransferSyntax (
    IN OUT RPC_MESSAGE __RPC_FAR * Message
    );
#endif // (NTDDI_VERSION >= NTDDI_WINXP)

RPCRTAPI
_Must_inspect_result_
RPC_STATUS
RPC_ENTRY
I_RpcGetBuffer (
    IN OUT RPC_MESSAGE __RPC_FAR * Message
    );

RPCRTAPI
RPC_STATUS
RPC_ENTRY
I_RpcGetBufferWithObject (
    IN OUT RPC_MESSAGE __RPC_FAR * Message,
    IN UUID * ObjectUuid
    );

RPCRTAPI
_Must_inspect_result_
RPC_STATUS
RPC_ENTRY
I_RpcSendReceive (
    IN OUT RPC_MESSAGE __RPC_FAR * Message
    );

RPCRTAPI
_Must_inspect_result_
RPC_STATUS
RPC_ENTRY
I_RpcFreeBuffer (
    IN OUT RPC_MESSAGE __RPC_FAR * Message
    );

RPCRTAPI
_Must_inspect_result_
RPC_STATUS
RPC_ENTRY
I_RpcSend (
    IN OUT PRPC_MESSAGE Message
    );

RPCRTAPI
_Must_inspect_result_
RPC_STATUS
RPC_ENTRY
I_RpcReceive (
    IN OUT PRPC_MESSAGE Message,
    IN unsigned int Size
    );

RPCRTAPI
_Must_inspect_result_
RPC_STATUS
RPC_ENTRY
I_RpcFreePipeBuffer (
    IN OUT RPC_MESSAGE __RPC_FAR * Message
    );

RPCRTAPI
_Must_inspect_result_
RPC_STATUS
RPC_ENTRY
I_RpcReallocPipeBuffer (
    _In_ PRPC_MESSAGE Message,
     unsigned int NewSize
    );

typedef void * I_RPC_MUTEX;

RPCRTAPI
void
RPC_ENTRY
I_RpcRequestMutex (
    IN OUT I_RPC_MUTEX * Mutex
    );

RPCRTAPI
void
RPC_ENTRY
I_RpcClearMutex (
    IN I_RPC_MUTEX Mutex
    );

RPCRTAPI
void
RPC_ENTRY
I_RpcDeleteMutex (
    IN I_RPC_MUTEX Mutex
    );

RPCRTAPI
void __RPC_FAR *
RPC_ENTRY
I_RpcAllocate (
    IN unsigned int Size
    );

RPCRTAPI
void
RPC_ENTRY
I_RpcFree (
    IN void __RPC_FAR * Object
    );

#define RPC_SYSTEM_HANDLE_FREE_UNRETRIEVED 1
#define RPC_SYSTEM_HANDLE_FREE_RETRIEVED 2
#define RPC_SYSTEM_HANDLE_FREE_ALL 3
#define RPC_SYSTEM_HANDLE_FREE_ERROR_ON_CLOSE 4

RPCRTAPI
unsigned long
RPC_ENTRY
I_RpcFreeSystemHandleCollection (
    _Inout_ void * CallObj,
    _In_ unsigned long FreeFlags
    );

RPCRTAPI
RPC_STATUS
RPC_ENTRY
I_RpcSetSystemHandle (
    _In_ void * Handle,
    _In_ unsigned char Type,
    _In_ unsigned long AccessMask,
    _Inout_ void * CallObj,
    _Out_ unsigned long * HandleIndex
    );

RPCRTAPI
RPC_STATUS
RPC_ENTRY
I_RpcGetSystemHandle (
    _Out_writes_bytes_(sizeof(HANDLE)) unsigned char* pMemory,
    _In_ unsigned char Type,
    _In_ unsigned long AccessMask,
    _In_ unsigned long HandleIndex,
    _Inout_ void * CallObj
    );

RPCRTAPI
void
RPC_ENTRY
I_RpcFreeSystemHandle (
    _In_ unsigned char Type,
    _In_ void * Handle
    );

RPCRTAPI
void
RPC_ENTRY
I_RpcPauseExecution (
    IN unsigned long Milliseconds
    );

RPCRTAPI
RPC_STATUS
RPC_ENTRY
I_RpcGetExtendedError (
    void
    );


typedef enum _LRPC_SYSTEM_HANDLE_MARSHAL_DIRECTION
{
    MarshalDirectionMarshal,
    MarshalDirectionUnmarshal
}LRPC_SYSTEM_HANDLE_MARSHAL_DIRECTION;

RPCRTAPI
RPC_STATUS
RPC_ENTRY
I_RpcSystemHandleTypeSpecificWork (
    _Inout_ void * Handle,
    _In_ unsigned char ActualType,
    _In_ unsigned char IdlType,
    _In_ LRPC_SYSTEM_HANDLE_MARSHAL_DIRECTION MarshalDirection
    );

typedef
void
(__RPC_USER __RPC_FAR * PRPC_RUNDOWN) (
    void __RPC_FAR * AssociationContext
    );

RPCRTAPI
_Must_inspect_result_
RPC_STATUS
RPC_ENTRY
I_RpcMonitorAssociation (
    IN RPC_BINDING_HANDLE Handle,
    IN PRPC_RUNDOWN RundownRoutine,
    IN void * Context
    );

RPCRTAPI
_Must_inspect_result_
RPC_STATUS
RPC_ENTRY
I_RpcStopMonitorAssociation (
    IN RPC_BINDING_HANDLE Handle
    );

RPCRTAPI
RPC_BINDING_HANDLE
RPC_ENTRY
I_RpcGetCurrentCallHandle(
    void
    );

RPCRTAPI
_Must_inspect_result_
RPC_STATUS
RPC_ENTRY
I_RpcGetAssociationContext (
    IN RPC_BINDING_HANDLE BindingHandle,
    OUT void __RPC_FAR * __RPC_FAR * AssociationContext
    );

RPCRTAPI
void *
RPC_ENTRY
I_RpcGetServerContextList (
    IN RPC_BINDING_HANDLE BindingHandle
    );

RPCRTAPI
void
RPC_ENTRY
I_RpcSetServerContextList (
    IN RPC_BINDING_HANDLE BindingHandle,
    OUT void * ServerContextList
    );

RPCRTAPI
RPC_STATUS
RPC_ENTRY
I_RpcNsInterfaceExported (
    IN unsigned long EntryNameSyntax,
    IN unsigned short *EntryName,
    IN RPC_SERVER_INTERFACE * RpcInterfaceInformation
    );

RPCRTAPI
RPC_STATUS
RPC_ENTRY
I_RpcNsInterfaceUnexported (
    IN unsigned long EntryNameSyntax,
    IN unsigned short *EntryName,
    IN RPC_SERVER_INTERFACE * RpcInterfaceInformation
    );

RPCRTAPI
RPC_STATUS
RPC_ENTRY
I_RpcBindingToStaticStringBindingW (
    IN RPC_BINDING_HANDLE Binding,
    OUT unsigned short **StringBinding
    );

RPCRTAPI
RPC_STATUS
RPC_ENTRY
I_RpcBindingInqSecurityContext (
    IN RPC_BINDING_HANDLE Binding,
    OUT void **SecurityContextHandle
    );

#if (NTDDI_VERSION >= NTDDI_VISTA)
typedef struct _RPC_SEC_CONTEXT_KEY_INFO
{
    unsigned long EncryptAlgorithm;
    unsigned long KeySize;
    unsigned long SignatureAlgorithm;
}
RPC_SEC_CONTEXT_KEY_INFO, *PRPC_SEC_CONTEXT_KEY_INFO;

RPCRTAPI
RPC_STATUS
RPC_ENTRY
I_RpcBindingInqSecurityContextKeyInfo (
    _In_opt_ RPC_BINDING_HANDLE Binding,
    _Inout_ void *KeyInfo
    );

#endif // (NTDDI_VERSION >= NTDDI_VISTA)

RPCRTAPI
RPC_STATUS
RPC_ENTRY
I_RpcBindingInqWireIdForSnego (
    _In_ RPC_BINDING_HANDLE Binding,
    _Out_ unsigned char * WireId
    );

#if (NTDDI_VERSION >= NTDDI_WS03)
RPCRTAPI
RPC_STATUS
RPC_ENTRY
I_RpcBindingInqMarshalledTargetInfo (
    _In_ RPC_BINDING_HANDLE Binding,
    _Out_ unsigned long * MarshalledTargetInfoSize,
    _Outptr_result_bytebuffer_(* MarshalledTargetInfoSize) RPC_CSTR * MarshalledTargetInfo
    );
#endif // (NTDDI_VERSION >= NTDDI_WS03)

#if (NTDDI_VERSION >= NTDDI_WINXP)
RPCRTAPI
RPC_STATUS
RPC_ENTRY
I_RpcBindingInqLocalClientPID (
    IN RPC_BINDING_HANDLE Binding,
    OUT unsigned long *Pid
    );

RPCRTAPI
RPC_STATUS
RPC_ENTRY
I_RpcBindingHandleToAsyncHandle (
    IN RPC_BINDING_HANDLE Binding,
    OUT void **AsyncHandle
    );
#endif // (NTDDI_VERSION >= NTDDI_WINXP)

#ifdef RPC_UNICODE_SUPPORTED

RPCRTAPI
_Must_inspect_result_
RPC_STATUS
RPC_ENTRY
I_RpcNsBindingSetEntryNameW (
    _In_ RPC_BINDING_HANDLE Binding,
    _In_ unsigned long EntryNameSyntax,
    _In_ RPC_WSTR EntryName
    );

RPCRTAPI
_Must_inspect_result_
RPC_STATUS
RPC_ENTRY
I_RpcNsBindingSetEntryNameA (
    _In_ RPC_BINDING_HANDLE Binding,
    _In_ unsigned long EntryNameSyntax,
    _In_ RPC_CSTR EntryName
    );

RPCRTAPI
RPC_STATUS
RPC_ENTRY
I_RpcServerUseProtseqEp2A (
    _In_opt_ RPC_CSTR NetworkAddress,
    _In_ RPC_CSTR Protseq,
    _In_ unsigned int MaxCalls,
    _In_ RPC_CSTR Endpoint,
    _In_opt_ void __RPC_FAR * SecurityDescriptor,
    _In_ void * Policy
    );

RPCRTAPI
RPC_STATUS
RPC_ENTRY
I_RpcServerUseProtseqEp2W (
    _In_opt_ RPC_WSTR NetworkAddress,
    _In_ RPC_WSTR Protseq,
    _In_ unsigned int MaxCalls,
    _In_ RPC_WSTR Endpoint,
    _In_opt_ void __RPC_FAR * SecurityDescriptor,
    _In_ void * Policy
    );

RPCRTAPI
RPC_STATUS
RPC_ENTRY
I_RpcServerUseProtseq2W (
    _In_opt_ RPC_WSTR NetworkAddress,
    _In_ RPC_WSTR Protseq,
    _In_ unsigned int MaxCalls,
    _In_opt_ void __RPC_FAR * SecurityDescriptor,
    _In_ void * Policy
    );

RPCRTAPI
RPC_STATUS
RPC_ENTRY
I_RpcServerUseProtseq2A (
    _In_opt_ RPC_CSTR NetworkAddress,
    _In_ RPC_CSTR Protseq,
    _In_ unsigned int MaxCalls,
    _In_opt_ void __RPC_FAR * SecurityDescriptor,
    _In_ void * Policy
    );


#ifdef UNICODE
#define I_RpcNsBindingSetEntryName I_RpcNsBindingSetEntryNameW
#define I_RpcServerUseProtseqEp2 I_RpcServerUseProtseqEp2W
#define I_RpcServerUseProtseq2 I_RpcServerUseProtseq2W
#else
#define I_RpcNsBindingSetEntryName I_RpcNsBindingSetEntryNameA
#define I_RpcServerUseProtseqEp2 I_RpcServerUseProtseqEp2A
#define I_RpcServerUseProtseq2 I_RpcServerUseProtseq2A
#endif

#else

RPCRTAPI
RPC_STATUS
RPC_ENTRY
I_RpcNsBindingSetEntryName (
    _In_ RPC_BINDING_HANDLE Binding,
    _In_ unsigned long EntryNameSyntax,
    _In_ RPC_CSTR EntryName
    );

RPCRTAPI
RPC_STATUS
RPC_ENTRY
I_RpcServerUseProtseq2 (
    _In_ RPC_CSTR NetworkAddress,
    _In_ RPC_CSTR Protseq,
    _In_ unsigned int MaxCalls,
    _In_ void __RPC_FAR * SecurityDescriptor,
    _In_ void * Policy
    );

RPCRTAPI
RPC_STATUS
RPC_ENTRY
I_RpcServerUseProtseqEp2 (
    _In_ RPC_CSTR NetworkAddress,
    _In_ RPC_CSTR Protseq,
    _In_ unsigned int MaxCalls,
    _In_ RPC_CSTR Endpoint,
    _In_ void __RPC_FAR *SecurityDescriptor,
    _In_ void *Policy
    );
#endif

RPCRTAPI
RPC_STATUS
RPC_ENTRY
I_RpcServerStartService (
    _In_ RPC_WSTR Protseq,
    _In_ RPC_WSTR Endpoint,
    _In_ RPC_IF_HANDLE IfSpec
    );

#ifdef RPC_UNICODE_SUPPORTED

RPCRTAPI
_Must_inspect_result_
RPC_STATUS
RPC_ENTRY
I_RpcBindingInqDynamicEndpointW (
    _In_ RPC_BINDING_HANDLE Binding,
    _Outptr_result_maybenull_ RPC_WSTR __RPC_FAR *DynamicEndpoint
    );

RPCRTAPI
_Must_inspect_result_
RPC_STATUS
RPC_ENTRY
I_RpcBindingInqDynamicEndpointA (
    _In_ RPC_BINDING_HANDLE Binding,
    _Outptr_result_maybenull_ RPC_CSTR __RPC_FAR *DynamicEndpoint
    );

#ifdef UNICODE
#define I_RpcBindingInqDynamicEndpoint I_RpcBindingInqDynamicEndpointW
#else
#define I_RpcBindingInqDynamicEndpoint I_RpcBindingInqDynamicEndpointA
#endif

#else

RPCRTAPI
_Must_inspect_result_
RPC_STATUS
RPC_ENTRY
I_RpcBindingInqDynamicEndpoint (
    _In_ RPC_BINDING_HANDLE Binding,
    _Outptr_result_maybenull_ RPC_CSTR __RPC_FAR *DynamicEndpoint
    );

#endif

#if (NTDDI_VERSION >= NTDDI_WINXP)
RPCRTAPI
RPC_STATUS
RPC_ENTRY
I_RpcServerCheckClientRestriction (
    IN RPC_BINDING_HANDLE Context
    );
#endif // (NTDDI_VERSION >= NTDDI_WINXP)

#define TRANSPORT_TYPE_CN        0x01
#define TRANSPORT_TYPE_DG        0x02
#define TRANSPORT_TYPE_LPC       0x04
#define TRANSPORT_TYPE_WMSG      0x08

//@[comment("MVI_tracked")]
RPCRTAPI
_Must_inspect_result_
RPC_STATUS
RPC_ENTRY
I_RpcBindingInqTransportType (
    IN RPC_BINDING_HANDLE Binding,
    OUT unsigned int __RPC_FAR * Type
    );

typedef struct _RPC_TRANSFER_SYNTAX
{
    UUID Uuid;
    unsigned short VersMajor;
    unsigned short VersMinor;
} RPC_TRANSFER_SYNTAX;

RPCRTAPI
_Must_inspect_result_
RPC_STATUS
RPC_ENTRY
I_RpcIfInqTransferSyntaxes (
    IN RPC_IF_HANDLE RpcIfHandle,
    OUT RPC_TRANSFER_SYNTAX __RPC_FAR * TransferSyntaxes,
    IN unsigned int TransferSyntaxSize,
    OUT unsigned int __RPC_FAR * TransferSyntaxCount
    );

RPCRTAPI
_Must_inspect_result_
RPC_STATUS
RPC_ENTRY
I_UuidCreate (
    OUT UUID __RPC_FAR * Uuid
    );

RPCRTAPI
void 
RPC_ENTRY 
I_RpcUninitializeNdrOle (
    void
    );

RPCRTAPI
_Must_inspect_result_
RPC_STATUS
RPC_ENTRY
I_RpcBindingCopy (
    IN RPC_BINDING_HANDLE SourceBinding,
    OUT RPC_BINDING_HANDLE __RPC_FAR * DestinationBinding
    );

RPCRTAPI
RPC_STATUS
RPC_ENTRY
I_RpcBindingIsClientLocal (
    IN OPTIONAL RPC_BINDING_HANDLE BindingHandle,
    OUT unsigned int __RPC_FAR * ClientLocalFlag
    );

RPCRTAPI
RPC_STATUS
RPC_ENTRY
I_RpcBindingInqConnId (
    IN RPC_BINDING_HANDLE Binding,
    OUT void **ConnId,
    OUT int *pfFirstCall
    );

RPCRTAPI
RPC_STATUS
RPC_ENTRY
I_RpcBindingCreateNP (
    _In_ RPC_WSTR ServerName,
    _In_ RPC_WSTR ServiceName,
    _In_ RPC_WSTR NetworkOptions,
    _Out_ RPC_BINDING_HANDLE *Binding
    );

RPCRTAPI
void
RPC_ENTRY
I_RpcSsDontSerializeContext (
    void
    );

RPCRTAPI
_Must_inspect_result_
RPC_STATUS
RPC_ENTRY
I_RpcLaunchDatagramReceiveThread(
    void __RPC_FAR * pAddress
    );

RPCRTAPI
_Must_inspect_result_
RPC_STATUS
RPC_ENTRY
I_RpcServerRegisterForwardFunction (
    IN RPC_FORWARD_FUNCTION  * pForwardFunction
    );

RPC_ADDRESS_CHANGE_FN * RPC_ENTRY
I_RpcServerInqAddressChangeFn(
    void
    );

RPC_STATUS RPC_ENTRY
I_RpcServerSetAddressChangeFn(
    IN RPC_ADDRESS_CHANGE_FN * pAddressChangeFn
    );

#if (NTDDI_VERSION >= NTDDI_WINXP)
/* The return buffer will contain SOCKADDR_IN for IPv4 */
#define RPC_P_ADDR_FORMAT_TCP_IPV4      1

/* The return buffer will contain SOCKADDR_STORAGE for IPv6 */
#define RPC_P_ADDR_FORMAT_TCP_IPV6      2

RPCRTAPI
RPC_STATUS
RPC_ENTRY
I_RpcServerInqLocalConnAddress (
    IN RPC_BINDING_HANDLE Binding,
    IN OUT void *Buffer,
    IN OUT unsigned long *BufferSize,
    OUT unsigned long *AddressFormat
    );

RPCRTAPI
RPC_STATUS
RPC_ENTRY
I_RpcServerInqRemoteConnAddress (
    IN RPC_BINDING_HANDLE Binding,
    IN OUT void *Buffer,
    IN OUT unsigned long *BufferSize,
    OUT unsigned long *AddressFormat
    );

RPCRTAPI
void
RPC_ENTRY
I_RpcSessionStrictContextHandle (
    void
    );

RPCRTAPI
RPC_STATUS
RPC_ENTRY
I_RpcTurnOnEEInfoPropagation (
    void
    );
#endif // (NTDDI_VERSION >= NTDDI_WINXP)

RPCRTAPI
_Must_inspect_result_
RPC_STATUS
RPC_ENTRY
I_RpcConnectionInqSockBuffSize(
  OUT unsigned long __RPC_FAR * RecvBuffSize,
  OUT unsigned long __RPC_FAR * SendBuffSize
  );

RPCRTAPI
_Must_inspect_result_
RPC_STATUS
RPC_ENTRY
I_RpcConnectionSetSockBuffSize(
   IN unsigned long RecvBuffSize,
   IN unsigned long SendBuffSize
   );

typedef
void
(*RPCLT_PDU_FILTER_FUNC) (
    IN void *Buffer,
    IN unsigned int BufferLength,
    IN int fDatagram
    );

typedef
void
(__cdecl *RPC_SETFILTER_FUNC) (
    IN RPCLT_PDU_FILTER_FUNC pfnFilter
    );

#ifndef WINNT
RPCRTAPI
_Must_inspect_result_
RPC_STATUS
RPC_ENTRY
I_RpcServerStartListening(
    void * hWnd
    );

RPCRTAPI
_Must_inspect_result_
RPC_STATUS
RPC_ENTRY
I_RpcServerStopListening(
    void
    );

typedef RPC_STATUS (*RPC_BLOCKING_FN) (
    IN void * hWnd,
    IN void * Context,
    IN OPTIONAL void * hSyncEvent
    );

RPCRTAPI
_Must_inspect_result_
RPC_STATUS
RPC_ENTRY
I_RpcBindingSetAsync(
    IN RPC_BINDING_HANDLE Binding,
    IN RPC_BLOCKING_FN BlockingFn,
    IN unsigned long ServerTid
    );

RPCRTAPI
_Must_inspect_result_
RPC_STATUS
RPC_ENTRY
I_RpcSetThreadParams(
    IN int fClientFree,
    IN OPTIONAL void *Context,
    IN OPTIONAL void * hWndClient
    );

RPCRTAPI
unsigned int
RPC_ENTRY
I_RpcWindowProc(
    IN void * hWnd,
    IN unsigned int Message,
    IN unsigned int wParam,
    IN unsigned long lParam
    );

RPCRTAPI
_Must_inspect_result_
RPC_STATUS
RPC_ENTRY
I_RpcServerUnregisterEndpointA (
    _In_ RPC_CSTR Protseq,
    _In_ RPC_CSTR Endpoint
    );

RPCRTAPI
_Must_inspect_result_
RPC_STATUS
RPC_ENTRY
I_RpcServerUnregisterEndpointW (
    _In_ RPC_WSTR Protseq,
    _In_ RPC_WSTR Endpoint
    );

#ifdef UNICODE
#define I_RpcServerUnregisterEndpoint I_RpcServerUnregisterEndpointW
#else
#define I_RpcServerUnregisterEndpoint I_RpcServerUnregisterEndpointA
#endif
#endif // WINNT

RPCRTAPI
RPC_STATUS
RPC_ENTRY
I_RpcServerInqTransportType(
    OUT unsigned int __RPC_FAR * Type
    );

RPCRTAPI
long
RPC_ENTRY
I_RpcMapWin32Status (
    IN RPC_STATUS Status
    );

#if (NTDDI_VERSION >= NTDDI_WS03)

#define RPC_C_OPT_SESSION_ID                (6)
#define RPC_C_OPT_COOKIE_AUTH                  (7)
#define RPC_C_OPT_RESOURCE_TYPE_UUID        (8)

typedef struct _RPC_C_OPT_COOKIE_AUTH_DESCRIPTOR
{
    unsigned long BufferSize;   // Length of the buffer in bytes
    _Field_size_bytes_(BufferSize) char *Buffer;               // Zero-terminated string
} RPC_C_OPT_COOKIE_AUTH_DESCRIPTOR;

typedef struct _RDR_CALLOUT_STATE
{
    // LastError is used for debugging only - don't use it for processing
    RPC_STATUS LastError;
    void *LastEEInfo;

    RPC_HTTP_REDIRECTOR_STAGE LastCalledStage;

    // the information accumulated throughout the stages
    unsigned short *ServerName;   // Allocated with MemAllocate
    unsigned short *ServerPort;   // Allocated with MemAllocate
    unsigned short *RemoteUser;   // Allocated with MemAllocate
    unsigned short *AuthType;     // Allocated with MemAllocate
    unsigned char ResourceTypePresent;
    unsigned char SessionIdPresent;
    unsigned char InterfacePresent;
    UUID ResourceType;
    UUID SessionId;
    RPC_SYNTAX_IDENTIFIER Interface;
    void *CertContext;            // Contains PCCERT_CONTEXT if cert is used for authentication.
                                  // NULL otherwise. If allocated, CertFreeCertificateContext
                                  // needs to be called on it when done
} RDR_CALLOUT_STATE;
#endif // (NTDDI_VERSION >= NTDDI_WS03)

#if (NTDDI_VERSION >= NTDDI_WINXP)
typedef RPC_STATUS
(RPC_ENTRY *I_RpcProxyIsValidMachineFn)
        (
        _In_ RPC_WSTR Machine,
        _In_ RPC_WSTR DotMachine,
        _In_ unsigned long PortNumber
        );

typedef RPC_STATUS
(RPC_ENTRY *I_RpcProxyGetClientAddressFn)
        (
        IN void *Context,
        OUT char *Buffer,
        OUT unsigned long *BufferLength
        );

typedef RPC_STATUS
(RPC_ENTRY *I_RpcProxyGetConnectionTimeoutFn)
        (
        OUT unsigned long *ConnectionTimeout
        );

#if (NTDDI_VERSION >= NTDDI_WS03)
typedef RPC_STATUS
(RPC_ENTRY *I_RpcPerformCalloutFn)
    (
    IN void *Context,
    IN RDR_CALLOUT_STATE *CallOutState,
    IN RPC_HTTP_REDIRECTOR_STAGE Stage
    );

typedef void
(RPC_ENTRY *I_RpcFreeCalloutStateFn)
    (
    IN RDR_CALLOUT_STATE *CallOutState
    );

typedef RPC_STATUS
(RPC_ENTRY *I_RpcProxyGetClientSessionAndResourceUUID)
        (
        _In_ void *Context,
        _Out_opt_ int *SessionIdPresent,
        _Out_opt_ UUID *SessionId,
        _Out_ int *ResourceIdPresent,
        _Out_ UUID *ResourceId
        );

#endif // (NTDDI_VERSION >= NTDDI_WS03)

#if (NTDDI_VERSION >= NTDDI_VISTA)
typedef RPC_STATUS
(RPC_ENTRY *I_RpcProxyFilterIfFn)
    (
    _In_ void *Context,
    _In_ UUID *IfUuid,
    _In_ unsigned short IfMajorVersion,
    _Out_ int *fAllow
    );

typedef enum RpcProxyPerfCounters
{
    RpcCurrentUniqueUser = 1,    //Current Number of Unique Users
    RpcBackEndConnectionAttempts, // Number of Back-End Connection Attempts per Second
    RpcBackEndConnectionFailed, // Number of Failed Back-End Connection Attempts per Second
    RpcRequestsPerSecond, //RPC/HTTP Requests per Second
    RpcIncomingConnections, //Current Number of Incoming RPC over HTTP Connections
    RpcIncomingBandwidth, // Total Incoming Bandwidth from Back-End Servers
    RpcOutgoingBandwidth, // Total Outgoing Bandwidth to Back-End Servers
    RpcAttemptedLbsDecisions, // Attempted RPC Load Balancing Decisions per Second
    RpcFailedLbsDecisions, // Failed RPC Load Balancing Decisions per Second
    RpcAttemptedLbsMessages, //Attempted RPC Load Balancing Broker Requests per Second
    RpcFailedLbsMessages, // Failed RPC Load Balancing Broker Requests per Second
    RpcLastCounter
} RpcPerfCounters;

typedef void
(RPC_ENTRY *I_RpcProxyUpdatePerfCounterFn)
    (
    _In_ RpcPerfCounters Counter,
    _In_ int ModifyTrend,
    _In_ unsigned long Size
    );

 typedef void
(RPC_ENTRY *I_RpcProxyUpdatePerfCounterBackendServerFn)
    (
    _In_ unsigned short* MachineName,
    _In_ int IsConnectEvent
    );

#endif // (NTDDI_VERSION >= NTDDI_VISTA)

#define RPC_PROXY_CONNECTION_TYPE_IN_PROXY    0
#define RPC_PROXY_CONNECTION_TYPE_OUT_PROXY   1

#if (NTDDI_VERSION >= NTDDI_WS03)
typedef struct tagI_RpcProxyCallbackInterface
{
    I_RpcProxyIsValidMachineFn IsValidMachineFn;
    I_RpcProxyGetClientAddressFn GetClientAddressFn;
    I_RpcProxyGetConnectionTimeoutFn GetConnectionTimeoutFn;
    I_RpcPerformCalloutFn PerformCalloutFn;
    I_RpcFreeCalloutStateFn FreeCalloutStateFn;
    I_RpcProxyGetClientSessionAndResourceUUID GetClientSessionAndResourceUUIDFn;
#if (NTDDI_VERSION >= NTDDI_VISTA)
    I_RpcProxyFilterIfFn ProxyFilterIfFn;
    I_RpcProxyUpdatePerfCounterFn RpcProxyUpdatePerfCounterFn;
    I_RpcProxyUpdatePerfCounterBackendServerFn RpcProxyUpdatePerfCounterBackendServerFn;
#endif // (NTDDI_VERSION >= NTDDI_VISTA)
} I_RpcProxyCallbackInterface;

RPCRTAPI
RPC_STATUS
RPC_ENTRY
I_RpcProxyNewConnection (
    IN unsigned long ConnectionType,
    IN unsigned short *ServerAddress,
    IN unsigned short *ServerPort,
    IN unsigned short *MinConnTimeout,
    IN void *ConnectionParameter,
    IN OPTIONAL RDR_CALLOUT_STATE *CallOutState,
    IN I_RpcProxyCallbackInterface *ProxyCallbackInterface
    );

#else

typedef struct tagI_RpcProxyCallbackInterface
{
    I_RpcProxyIsValidMachineFn IsValidMachineFn;
    I_RpcProxyGetClientAddressFn GetClientAddressFn;
    I_RpcProxyGetConnectionTimeoutFn GetConnectionTimeoutFn;
} I_RpcProxyCallbackInterface;

RPCRTAPI
RPC_STATUS
RPC_ENTRY
I_RpcProxyNewConnection (
    IN unsigned long ConnectionType,
    IN unsigned short *ServerAddress,
    IN unsigned short *ServerPort,
    IN void *ConnectionParameter,
    IN I_RpcProxyCallbackInterface *ProxyCallbackInterface
    );
#endif // (NTDDI_VERSION >= NTDDI_WS03)

#endif // (NTDDI_VERSION >= NTDDI_WINXP)

#if (NTDDI_VERSION >= NTDDI_WS03)
RPCRTAPI
RPC_STATUS
RPC_ENTRY
I_RpcReplyToClientWithStatus (
    IN void *ConnectionParameter,
    IN RPC_STATUS RpcStatus
    );

RPCRTAPI
void
RPC_ENTRY
I_RpcRecordCalloutFailure (
    IN RPC_STATUS RpcStatus,
    IN RDR_CALLOUT_STATE *CallOutState,
    IN unsigned short *DllName
    );
#endif // (NTDDI_VERSION >= NTDDI_WS03)

#if (NTDDI_VERSION >= NTDDI_WIN7)
RPCRTAPI
RPC_STATUS
RPC_ENTRY
I_RpcMgmtEnableDedicatedThreadPool (
    void
    );
#endif // (NTDDI_VERSION >= NTDDI_WIN7)

#if (NTDDI_VERSION >= NTDDI_WIN7)
RPCRTAPI
RPC_STATUS
RPC_ENTRY
I_RpcGetDefaultSD(
    _Out_ void __RPC_FAR ** ppSecurityDescriptor
    );
#endif // (NTDDI_VERSION >= NTDDI_WIN7)

#if (NTDDI_VERSION >= NTDDI_VISTA)
RPCRTAPI
RPC_STATUS
RPC_ENTRY
I_RpcOpenClientProcess(
    _In_opt_ RPC_BINDING_HANDLE Binding,
    _In_ unsigned long DesiredAccess,
    _Outptr_ void** ClientProcess
    );
#endif // (NTDDI_VERSION >= NTDDI_VISTA)

#if (NTDDI_VERSION >= NTDDI_WIN8)
RPCRTAPI
RPC_STATUS
RPC_ENTRY
I_RpcBindingIsServerLocal(
    _In_ RPC_BINDING_HANDLE Binding,
    _Out_ unsigned int * ServerLocalFlag
    );

RPC_STATUS RPC_ENTRY
I_RpcBindingSetPrivateOption (
    _In_ RPC_BINDING_HANDLE hBinding,
    _In_ unsigned long      option,
    _In_ ULONG_PTR          optionValue
    );

#define RPC_C_OPT_PRIVATE_SUPPRESS_WAKE     1
#define RPC_C_OPT_PRIVATE_DO_NOT_DISTURB    2
#define RPC_C_OPT_PRIVATE_BREAK_ON_SUSPEND  3

#endif // (NTDDI_VERSION >= NTDDI_WIN8)

#if (NTDDI_VERSION >= NTDDI_WINBLUE)
RPC_STATUS
RPC_ENTRY
I_RpcServerSubscribeForDisconnectNotification (
    _In_opt_ RPC_BINDING_HANDLE Binding,
    _In_opt_ void *hEvent
    );

RPC_STATUS
RPC_ENTRY
I_RpcServerGetAssociationID (
    _In_opt_ RPC_BINDING_HANDLE Binding,
    _Out_ unsigned long *AssociationID
    );

RPCRTAPI
long
RPC_ENTRY
I_RpcServerDisableExceptionFilter (
    void
    );

#endif // (NTDDI_VERSION >= NTDDI_WINBLUE)

#if (NTDDI_VERSION >= NTDDI_WIN10_RS5)
RPC_STATUS
RPC_ENTRY
I_RpcServerSubscribeForDisconnectNotification2 (
    _In_opt_ RPC_BINDING_HANDLE Binding,
    _In_ void *hEvent,
    _Out_ UUID *SubscriptionId
    );

RPC_STATUS
RPC_ENTRY
I_RpcServerUnsubscribeForDisconnectNotification (
    _In_opt_ RPC_BINDING_HANDLE Binding,
    _In_ UUID SubscriptionId
    );

#endif // (NTDDI_VERSION >= NTDDI_WIN10_RS5)

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#if _MSC_VER >= 1200
#pragma warning(pop)
#endif

#ifdef __cplusplus
}
#endif

#if defined(__cplusplus)
#define I_RRPCUNINITIALIZENDROLE_EXPORT_NAME reinterpret_cast<LPCSTR>(static_cast<ULONG_PTR>(1000))
#else
#define I_RRPCUNINITIALIZENDROLE_EXPORT_NAME ((LPCSTR)(ULONG_PTR)1000)
#endif

#endif /* __RPCDCEP_H__ */
