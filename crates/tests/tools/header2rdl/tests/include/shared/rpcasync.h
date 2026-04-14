#include <winapifamily.h>

/*++

Copyright (c) Microsoft Corporation. All rights reserved.

Module Name:

    rpcasync.h

Abstract:

    This module contains the RPC runtime APIs needed to use
    [async] RPC features.

--*/

#ifndef __RPCASYNC_H__
#define __RPCASYNC_H__

#if _MSC_VER > 1000
#pragma once
#endif

#if defined(__RPC_WIN64__)
#include <pshpack8.h>
#endif

#ifdef __cplusplus
extern "C" {
#endif

#pragma region Application Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)


#if defined(_KRPCENV_)

#include <ntddk.h>

#endif // _KRPCENV_

#if defined(RPC_NO_WINDOWS_H) || defined(_KRPCENV_)

// We've gotta have windef.h--can't live without it.

#include <windef.h>

#endif // RPC_NO_WINDOWS_H || _KRPCENV_

#if _MSC_VER >= 1200
#pragma warning(push)
#pragma warning(disable:4820) // padding added after data member
#endif

#define RPC_ASYNC_VERSION_1_0     sizeof(RPC_ASYNC_STATE)

typedef
enum _RPC_NOTIFICATION_TYPES
{
    RpcNotificationTypeNone,
    RpcNotificationTypeEvent,
#if !defined(_KRPCENV_)
    RpcNotificationTypeApc,
    RpcNotificationTypeIoc,
    RpcNotificationTypeHwnd,
#endif
    RpcNotificationTypeCallback
} RPC_NOTIFICATION_TYPES;

#if (NTDDI_VERSION >= NTDDI_VISTA)
typedef
enum _RPC_ASYNC_EVENT {
    RpcCallComplete,
    RpcSendComplete,
    RpcReceiveComplete,
    RpcClientDisconnect,
    RpcClientCancel
    } RPC_ASYNC_EVENT;
    
#else // (NTDDI_VERSION >= NTDDI_VISTA)

typedef
enum _RPC_ASYNC_EVENT {
    RpcCallComplete,
    RpcSendComplete,
    RpcReceiveComplete,
    RpcClientDisconnect
    } RPC_ASYNC_EVENT;    
#endif // (NTDDI_VERSION >= NTDDI_VISTA)

struct _RPC_ASYNC_STATE;

typedef void RPC_ENTRY
RPCNOTIFICATION_ROUTINE (
                  struct _RPC_ASYNC_STATE *pAsync,
                  void *Context,
                  RPC_ASYNC_EVENT Event);
typedef RPCNOTIFICATION_ROUTINE *PFN_RPCNOTIFICATION_ROUTINE;

typedef union _RPC_ASYNC_NOTIFICATION_INFO {
#if !defined(_KRPCENV_)
    //
    // Notification by APC
    //
    struct {
        PFN_RPCNOTIFICATION_ROUTINE NotificationRoutine;
        HANDLE hThread;
        } APC;


#if !defined(RPC_NO_WINDOWS_H) || defined (_WINBASE_)

    //
    // Notification by IO completion port
    //
    struct {
        HANDLE hIOPort;
        DWORD dwNumberOfBytesTransferred;
        DWORD_PTR dwCompletionKey;
        LPOVERLAPPED lpOverlapped;
        } IOC;

#endif // !RPC_NO_WINDOWS_H || _WINBASE_
#if !defined(RPC_NO_WINDOWS_H)
    //
    // Notification by window message
    //
    struct {
        HWND hWnd;
        UINT Msg;
        } HWND;

#endif // RPC_NO_WINDOWS_H
#endif // _KRPCENV_


    #if !defined(_KRPCENV_)
    //
    // Notification by event
    //
    HANDLE hEvent;
    #else   // _KRPCENV_
    //
    // Notification by event
    //
    PKEVENT Event;
    #endif  // _KRPCENV_

    //
    // Notification by callback function
    //
    // This option is available only to OLE
    //
    PFN_RPCNOTIFICATION_ROUTINE NotificationRoutine;
} RPC_ASYNC_NOTIFICATION_INFO, *PRPC_ASYNC_NOTIFICATION_INFO;

typedef struct _RPC_ASYNC_STATE {
    unsigned int    Size; // size of this structure
    unsigned long   Signature;
    long   Lock;
    unsigned long   Flags;
    void           *StubInfo;
    void           *UserInfo;
    void           *RuntimeInfo;
    RPC_ASYNC_EVENT Event;

    RPC_NOTIFICATION_TYPES NotificationType;
    RPC_ASYNC_NOTIFICATION_INFO u;

    LONG_PTR Reserved[4];
    } RPC_ASYNC_STATE, *PRPC_ASYNC_STATE;

// Possible values for Flags
#define RPC_C_NOTIFY_ON_SEND_COMPLETE      0x1
#define RPC_C_INFINITE_TIMEOUT             INFINITE

#define RpcAsyncGetCallHandle(pAsync) (((PRPC_ASYNC_STATE) pAsync)->RuntimeInfo)

#if !defined(_KRPCENV_)
RPCRTAPI
_Must_inspect_result_
RPC_STATUS
RPC_ENTRY
RpcAsyncRegisterInfo (
    _In_ PRPC_ASYNC_STATE pAsync
    ) ;
#endif

RPCRTAPI
_Must_inspect_result_
RPC_STATUS
RPC_ENTRY
RpcAsyncInitializeHandle (
    _Out_writes_bytes_(Size) PRPC_ASYNC_STATE pAsync,
    _In_ unsigned int     Size
    );

RPCRTAPI
_Must_inspect_result_
RPC_STATUS
RPC_ENTRY
RpcAsyncGetCallStatus (
    _In_ PRPC_ASYNC_STATE pAsync
    ) ;

RPCRTAPI
_Must_inspect_result_
RPC_STATUS
RPC_ENTRY
RpcAsyncCompleteCall (
    _Inout_ PRPC_ASYNC_STATE pAsync,
    _Out_opt_ void *Reply
    ) ;

RPCRTAPI
_Must_inspect_result_
RPC_STATUS
RPC_ENTRY
RpcAsyncAbortCall (
    _Inout_ PRPC_ASYNC_STATE pAsync,
    _In_ unsigned long ExceptionCode
    ) ;

RPCRTAPI
_Must_inspect_result_
RPC_STATUS
RPC_ENTRY
RpcAsyncCancelCall (
    _Inout_ PRPC_ASYNC_STATE pAsync,
    _In_ BOOL fAbort
    ) ;

#if (NTDDI_VERSION >= NTDDI_WINXP)

// EEINFO needs k-mode or needs winbase.h
#if !defined(RPC_NO_WINDOWS_H) || defined(_KRPCENV_) || defined(_WINBASE_)

typedef enum tagExtendedErrorParamTypes
{
    eeptAnsiString = 1,
    eeptUnicodeString,
    eeptLongVal,
    eeptShortVal,
    eeptPointerVal,
    eeptNone,
    eeptBinary
} ExtendedErrorParamTypes;

#define MaxNumberOfEEInfoParams    4
#define RPC_EEINFO_VERSION         1

typedef struct tagBinaryParam
{
    void *Buffer;
    short Size;
} BinaryParam;

typedef struct tagRPC_EE_INFO_PARAM
{
    ExtendedErrorParamTypes ParameterType;
    union
        {
        LPSTR AnsiString;
        LPWSTR UnicodeString;
        long LVal;
        short SVal;
        ULONGLONG PVal;
        BinaryParam BVal;
        } u;
} RPC_EE_INFO_PARAM;

#define EEInfoPreviousRecordsMissing     1
#define EEInfoNextRecordsMissing         2
#define EEInfoUseFileTime                4

#define EEInfoGCCOM                     11
#if (NTDDI_VERSION >= NTDDI_WS03)
#define EEInfoGCFRS                     12
#endif // (NTDDI_VERSION >= NTDDI_WS03)

typedef struct tagRPC_EXTENDED_ERROR_INFO
{
    ULONG Version;
    LPWSTR ComputerName;
    ULONG ProcessID;
    union
        {
#if !defined(_KRPCENV_)
        SYSTEMTIME SystemTime;
        FILETIME FileTime;
#else    // _KRPCENV_
        LARGE_INTEGER KernelTime;
#endif   // _KRPCENV_
        } u;
    ULONG GeneratingComponent;
    ULONG Status;
    USHORT DetectionLocation;
    USHORT Flags;
    int NumberOfParameters;
    RPC_EE_INFO_PARAM Parameters[MaxNumberOfEEInfoParams];
} RPC_EXTENDED_ERROR_INFO;

typedef struct tagRPC_ERROR_ENUM_HANDLE
{
    ULONG Signature;
    void *CurrentPos;
    void *Head;
} RPC_ERROR_ENUM_HANDLE;

RPCRTAPI
RPC_STATUS 
RPC_ENTRY
RpcErrorStartEnumeration (
    _Out_ RPC_ERROR_ENUM_HANDLE *EnumHandle
    );

RPCRTAPI
RPC_STATUS 
RPC_ENTRY
RpcErrorGetNextRecord (
    _In_ RPC_ERROR_ENUM_HANDLE *EnumHandle, 
    _In_ BOOL CopyStrings, 
    _Out_ RPC_EXTENDED_ERROR_INFO *ErrorInfo
    );

RPCRTAPI
RPC_STATUS 
RPC_ENTRY
RpcErrorEndEnumeration (
    _Inout_ RPC_ERROR_ENUM_HANDLE *EnumHandle
    );

RPCRTAPI
RPC_STATUS 
RPC_ENTRY
RpcErrorResetEnumeration (
    _Inout_ RPC_ERROR_ENUM_HANDLE *EnumHandle
    );

RPCRTAPI
RPC_STATUS 
RPC_ENTRY
RpcErrorGetNumberOfRecords (
    _In_ RPC_ERROR_ENUM_HANDLE *EnumHandle, 
    _Out_ int *Records
    );

RPCRTAPI
RPC_STATUS 
RPC_ENTRY
RpcErrorSaveErrorInfo (
    _In_ RPC_ERROR_ENUM_HANDLE *EnumHandle, 
    _Outptr_result_bytebuffer_(*BlobSize) PVOID *ErrorBlob, 
    _Out_ size_t *BlobSize
    );

RPCRTAPI
RPC_STATUS 
RPC_ENTRY
RpcErrorLoadErrorInfo (
    _In_reads_bytes_(BlobSize) PVOID ErrorBlob, 
    _In_ size_t BlobSize, 
    _Out_ RPC_ERROR_ENUM_HANDLE *EnumHandle
    );

RPCRTAPI
RPC_STATUS 
RPC_ENTRY
RpcErrorAddRecord (
    _In_ RPC_EXTENDED_ERROR_INFO *ErrorInfo
    );

RPCRTAPI
void 
RPC_ENTRY
RpcErrorClearInformation (
    void
    );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

#endif // !RPC_NO_WINDOWS_H || _KRPCENV_ || _WINBASE_

#if !defined(_KRPCENV_)

RPCRTAPI
RPC_STATUS
RPC_ENTRY
RpcAsyncCleanupThread (
    _In_ DWORD dwTimeout
    );

RPCRTAPI
RPC_STATUS
RPC_ENTRY
RpcGetAuthorizationContextForClient (
    _In_opt_ RPC_BINDING_HANDLE ClientBinding,
    _In_ BOOL ImpersonateOnReturn,
    _In_opt_ PVOID Reserved1,
    _In_opt_ PLARGE_INTEGER pExpirationTime,
    _In_ LUID Reserved2,
    _In_ DWORD Reserved3,
    _In_opt_ PVOID Reserved4,
    _Out_ PVOID *pAuthzClientContext
    );

RPCRTAPI
RPC_STATUS 
RPC_ENTRY
RpcFreeAuthorizationContext (
    _Inout_ PVOID *pAuthzClientContext
    );

RPCRTAPI
RPC_STATUS
RPC_ENTRY
RpcSsContextLockExclusive (
    _In_opt_ RPC_BINDING_HANDLE ServerBindingHandle,
    _In_ PVOID UserContext
    );

RPCRTAPI
RPC_STATUS
RPC_ENTRY
RpcSsContextLockShared (
    _In_ RPC_BINDING_HANDLE ServerBindingHandle,
    _In_ PVOID UserContext
    );

#if (NTDDI_VERSION >= NTDDI_VISTA)
typedef enum tagRpcLocalAddressFormat
{
    rlafInvalid = 0,
    rlafIPv4,
    rlafIPv6
} RpcLocalAddressFormat;

typedef struct _RPC_CALL_LOCAL_ADDRESS_V1
{
    unsigned int Version;
    void *Buffer;
    unsigned long BufferSize;
    RpcLocalAddressFormat AddressFormat;
} RPC_CALL_LOCAL_ADDRESS_V1, *PRPC_CALL_LOCAL_ADDRESS_V1;

#if ( (NTDDI_VERSION >= NTDDI_VISTA) && (NTDDI_VERSION < NTDDI_WIN8) )
#define RPC_CALL_ATTRIBUTES_VERSION                         (2)
#endif // ( (NTDDI_VERSION >= NTDDI_VISTA) && (NTDDI_VERSION < NTDDI_WIN8) )

#define RPC_QUERY_SERVER_PRINCIPAL_NAME                     (0x02)
#define RPC_QUERY_CLIENT_PRINCIPAL_NAME                     (0x04)
#define RPC_QUERY_CALL_LOCAL_ADDRESS                        (0x08)
#define RPC_QUERY_CLIENT_PID                                (0x10)
#define RPC_QUERY_IS_CLIENT_LOCAL                           (0x20)
#define RPC_QUERY_NO_AUTH_REQUIRED                          (0x40)
#endif // (NTDDI_VERSION >= NTDDI_VISTA)

#if (NTDDI_VERSION >= NTDDI_WIN8)
#define RPC_CALL_ATTRIBUTES_VERSION                         (3)
#define RPC_QUERY_CLIENT_ID                                 (0x80)
#endif // (NTDDI_VERSION >= NTDDI_WIN8)

#if ( (NTDDI_VERSION >= NTDDI_WINXP) && (NTDDI_VERSION < NTDDI_VISTA) )
#define RPC_CALL_ATTRIBUTES_VERSION                         (1)
#define RPC_QUERY_SERVER_PRINCIPAL_NAME                     (2)
#define RPC_QUERY_CLIENT_PRINCIPAL_NAME                     (4)
#endif // ( (NTDDI_VERSION >= NTDDI_WINXP) && (NTDDI_VERSION < NTDDI_VISTA) )

typedef struct tagRPC_CALL_ATTRIBUTES_V1_W
{
    unsigned int Version;
    unsigned long Flags;
    unsigned long ServerPrincipalNameBufferLength;
    unsigned short *ServerPrincipalName;
    unsigned long ClientPrincipalNameBufferLength;
    unsigned short *ClientPrincipalName;
    unsigned long AuthenticationLevel;
    unsigned long AuthenticationService;
    BOOL NullSession;
} RPC_CALL_ATTRIBUTES_V1_W;

typedef struct tagRPC_CALL_ATTRIBUTES_V1_A
{
    unsigned int Version;
    unsigned long Flags;
    unsigned long ServerPrincipalNameBufferLength;
    unsigned char *ServerPrincipalName;
    unsigned long ClientPrincipalNameBufferLength;
    unsigned char *ClientPrincipalName;
    unsigned long AuthenticationLevel;
    unsigned long AuthenticationService;
    BOOL NullSession;
} RPC_CALL_ATTRIBUTES_V1_A;

#if (NTDDI_VERSION >= NTDDI_VISTA)
// CallStatus bit fields
#define RPC_CALL_STATUS_CANCELLED          0x01
#define RPC_CALL_STATUS_DISCONNECTED       0x02

typedef enum tagRpcCallType
{
    rctInvalid = 0,
    rctNormal,
    rctTraining,
    rctGuaranteed
} RpcCallType;

typedef enum tagRpcCallClientLocality
{
    rcclInvalid = 0,
    rcclLocal,
    rcclRemote,
    rcclClientUnknownLocality
} RpcCallClientLocality;


typedef struct tagRPC_CALL_ATTRIBUTES_V2_W
{
    unsigned int Version;
    unsigned long Flags;
    unsigned long ServerPrincipalNameBufferLength;
    unsigned short *ServerPrincipalName;
    unsigned long ClientPrincipalNameBufferLength;
    unsigned short *ClientPrincipalName;
    unsigned long AuthenticationLevel;
    unsigned long AuthenticationService;
    BOOL NullSession;
    BOOL KernelModeCaller;
    unsigned long ProtocolSequence;
    RpcCallClientLocality IsClientLocal;
    HANDLE ClientPID; 
    unsigned long CallStatus;
    RpcCallType CallType;
    RPC_CALL_LOCAL_ADDRESS_V1 *CallLocalAddress;	// one of the CallLocalAddress flavors
    unsigned short OpNum;
    UUID InterfaceUuid;        
} RPC_CALL_ATTRIBUTES_V2_W;
    
typedef struct tagRPC_CALL_ATTRIBUTES_V2_A
{
    unsigned int Version;
    unsigned long Flags;
    unsigned long ServerPrincipalNameBufferLength;
    unsigned char *ServerPrincipalName;
    unsigned long ClientPrincipalNameBufferLength;
    unsigned char *ClientPrincipalName;
    unsigned long AuthenticationLevel;
    unsigned long AuthenticationService;
    BOOL NullSession;
    BOOL KernelModeCaller;
    unsigned long ProtocolSequence;
    unsigned long IsClientLocal;
    HANDLE ClientPID; 
    unsigned long CallStatus;
    RpcCallType CallType;
    RPC_CALL_LOCAL_ADDRESS_V1 *CallLocalAddress;	// one of the CallLocalAddress flavors
    unsigned short OpNum;
    UUID InterfaceUuid;    
} RPC_CALL_ATTRIBUTES_V2_A;
#endif // (NTDDI_VERSION >= NTDDI_VISTA)

#if (NTDDI_VERSION >= NTDDI_WIN8)
typedef struct tagRPC_CALL_ATTRIBUTES_V3_W
{
    unsigned int Version;
    unsigned long Flags;
    unsigned long ServerPrincipalNameBufferLength;
    unsigned short *ServerPrincipalName;
    unsigned long ClientPrincipalNameBufferLength;
    unsigned short *ClientPrincipalName;
    unsigned long AuthenticationLevel;
    unsigned long AuthenticationService;
    BOOL NullSession;
    BOOL KernelModeCaller;
    unsigned long ProtocolSequence;
    RpcCallClientLocality IsClientLocal;
    HANDLE ClientPID; 
    unsigned long CallStatus;
    RpcCallType CallType;
    RPC_CALL_LOCAL_ADDRESS_V1 *CallLocalAddress;	// one of the CallLocalAddress flavors
    unsigned short OpNum;
    UUID InterfaceUuid;
    unsigned long          ClientIdentifierBufferLength;
    unsigned char          *ClientIdentifier;
} RPC_CALL_ATTRIBUTES_V3_W;
    
typedef struct tagRPC_CALL_ATTRIBUTES_V3_A
{
    unsigned int Version;
    unsigned long Flags;
    unsigned long ServerPrincipalNameBufferLength;
    unsigned char *ServerPrincipalName;
    unsigned long ClientPrincipalNameBufferLength;
    unsigned char *ClientPrincipalName;
    unsigned long AuthenticationLevel;
    unsigned long AuthenticationService;
    BOOL NullSession;
    BOOL KernelModeCaller;
    unsigned long ProtocolSequence;
    unsigned long IsClientLocal;
    HANDLE ClientPID; 
    unsigned long CallStatus;
    RpcCallType CallType;
    RPC_CALL_LOCAL_ADDRESS_V1 *CallLocalAddress;	// one of the CallLocalAddress flavors
    unsigned short OpNum;
    UUID InterfaceUuid;    
    unsigned long          ClientIdentifierBufferLength;
    unsigned char          *ClientIdentifier;
} RPC_CALL_ATTRIBUTES_V3_A;

#endif // (NTDDI_VERSION >= NTDDI_WIN8)

RPCRTAPI
RPC_STATUS
RPC_ENTRY
RpcServerInqCallAttributesW (
    _In_opt_ RPC_BINDING_HANDLE ClientBinding,
    _Inout_ void *RpcCallAttributes
    );

RPCRTAPI
RPC_STATUS
RPC_ENTRY
RpcServerInqCallAttributesA (
    _In_opt_ RPC_BINDING_HANDLE ClientBinding,
    _Inout_ void *RpcCallAttributes
    );

#ifdef UNICODE
#  define RPC_CALL_ATTRIBUTES_V1 RPC_CALL_ATTRIBUTES_V1_W
#if (NTDDI_VERSION >= NTDDI_VISTA)
#  define RPC_CALL_ATTRIBUTES_V2 RPC_CALL_ATTRIBUTES_V2_W
#endif // (NTDDI_VERSION >= NTDDI_VISTA)
#if (NTDDI_VERSION >= NTDDI_WIN8)
#  define RPC_CALL_ATTRIBUTES_V3 RPC_CALL_ATTRIBUTES_V3_W
#endif // (NTDDI_VERSION >= NTDDI_VISTA)
#  define RpcServerInqCallAttributes RpcServerInqCallAttributesW
#else
#  define RPC_CALL_ATTRIBUTES_V1 RPC_CALL_ATTRIBUTES_V1_A
#if (NTDDI_VERSION >= NTDDI_VISTA)
#  define RPC_CALL_ATTRIBUTES_V2 RPC_CALL_ATTRIBUTES_V2_A
#endif // (NTDDI_VERSION >= NTDDI_VISTA)
#if (NTDDI_VERSION >= NTDDI_WIN8)
#  define RPC_CALL_ATTRIBUTES_V3 RPC_CALL_ATTRIBUTES_V3_A
#endif // (NTDDI_VERSION >= NTDDI_VISTA)
#  define RpcServerInqCallAttributes RpcServerInqCallAttributesA
#endif // !UNICODE

#if (NTDDI_VERSION >= NTDDI_WIN8)
typedef RPC_CALL_ATTRIBUTES_V3 RPC_CALL_ATTRIBUTES;
#elif (NTDDI_VERSION >= NTDDI_VISTA)
typedef RPC_CALL_ATTRIBUTES_V2 RPC_CALL_ATTRIBUTES;
#else 
typedef RPC_CALL_ATTRIBUTES_V1 RPC_CALL_ATTRIBUTES;
#endif // (NTDDI_VERSION >= NTDDI_VISTA)

#if (NTDDI_VERSION >= NTDDI_VISTA)
// can be used as flags in a bitmask too
typedef enum _RPC_NOTIFICATIONS
{
    RpcNotificationCallNone = 0,
    RpcNotificationClientDisconnect = 1,
    RpcNotificationCallCancel = 2      // internal for Windows
} RPC_NOTIFICATIONS;

#define RpcNotificationCallStatusChange    RpcNotificationClientDisconnect

RPCRTAPI
RPC_STATUS 
RPC_ENTRY 
RpcServerSubscribeForNotification (
    _In_opt_ RPC_BINDING_HANDLE Binding,
    _In_ RPC_NOTIFICATIONS Notification,
    _In_ RPC_NOTIFICATION_TYPES NotificationType,
    _In_ RPC_ASYNC_NOTIFICATION_INFO *NotificationInfo
    );

RPCRTAPI
RPC_STATUS 
RPC_ENTRY 
RpcServerUnsubscribeForNotification (
    _In_opt_ RPC_BINDING_HANDLE Binding,
    _In_ RPC_NOTIFICATIONS Notification,
    _Out_ unsigned long *NotificationsQueued
    );

#endif // (NTDDI_VERSION >= NTDDI_VISTA)
#endif // _KRPCENV_
 
#if (NTDDI_VERSION >= NTDDI_VISTA)  

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#pragma region Application Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)

RPCRTAPI
RPC_STATUS 
RPC_ENTRY 
RpcBindingBind (
    _In_opt_ PRPC_ASYNC_STATE pAsync,
    _In_ RPC_BINDING_HANDLE Binding,
    _In_ RPC_IF_HANDLE IfSpec    
    );

RPCRTAPI
RPC_STATUS 
RPC_ENTRY 
RpcBindingUnbind (
    _In_ RPC_BINDING_HANDLE Binding
    );

#endif // (NTDDI_VERSION >= NTDDI_VISTA)


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

#endif // (NTDDI_VERSION >= NTDDI_WINXP)
//
// Internal APIs
//
_Must_inspect_result_
RPC_STATUS RPC_ENTRY
I_RpcAsyncSetHandle (
    _In_  PRPC_MESSAGE Message,
    _In_  PRPC_ASYNC_STATE pAsync
    );

_Must_inspect_result_
RPC_STATUS RPC_ENTRY
I_RpcAsyncAbortCall (
    _In_ PRPC_ASYNC_STATE pAsync,
    _In_ unsigned long ExceptionCode
    ) ;

#if (NTDDI_VERSION >= NTDDI_WINXP)
int 
RPC_ENTRY
I_RpcExceptionFilter (
    _In_ unsigned long ExceptionCode
    );
#endif // (NTDDI_VERSION >= NTDDI_WINXP)

#if (NTDDI_VERSION >= NTDDI_WIN7)

RPCRTAPI
RPC_STATUS
RPC_ENTRY
I_RpcBindingInqClientTokenAttributes (
    _In_ RPC_BINDING_HANDLE Binding,
    _Out_opt_ LUID * TokenId,
    _Out_opt_ LUID * AuthenticationId,
    _Out_opt_ LUID * ModifiedId
    );
    
#endif // (NTDDI_VERSION >= NTDDI_WIN7)

#if _MSC_VER >= 1200
#pragma warning(pop)
#endif

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#ifdef __cplusplus
}
#endif

#if defined(__RPC_WIN64__)
#include <poppack.h>
#endif

#endif /* __RPCASYNC_H__ */

