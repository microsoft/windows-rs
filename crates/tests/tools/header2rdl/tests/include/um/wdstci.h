/*++

Copyright (c) 2005 Microsoft Corporation

Module Name:

    wdstci.h

Abstract:

    This module defines the structures and functions that compose the interface
    between the content receiver and the transport client.

Environment:

    User Mode

--*/

#ifndef _WDSTCI_H
#define _WDSTCI_H
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


#include <windows.h>

#ifdef __cplusplus
extern "C"
{
#endif

#define WDSTCIAPI                      __stdcall

//----------------------------------------------------------------- Defines.

#define WDS_TRANSPORTCLIENT_CURRENT_API_VERSION 1

//
// Protocol types
//

#define WDS_TRANSPORTCLIENT_PROTOCOL_MULTICAST       0x00000001

//
// Authentication levels
//

#define WDS_TRANSPORTCLIENT_AUTH    0x1
#define WDS_TRANSPORTCLIENT_NO_AUTH 0x2

typedef enum _TRANSPORTCLIENT_CALLBACK_ID
{
    WDS_TRANSPORTCLIENT_SESSION_START = 0,
    WDS_TRANSPORTCLIENT_RECEIVE_CONTENTS,
    WDS_TRANSPORTCLIENT_SESSION_COMPLETE,
    WDS_TRANSPORTCLIENT_RECEIVE_METADATA,
    WDS_TRANSPORTCLIENT_SESSION_STARTEX,
    WDS_TRANSPORTCLIENT_SESSION_NEGOTIATE,

    WDS_TRANSPORTCLIENT_MAX_CALLBACKS,
} TRANSPORTCLIENT_CALLBACK_ID, *PTRANSPORTCLIENT_CALLBACK_ID;

typedef struct _TRANSPORTCLIENT_SESSION_INFO
{
    //
    // The length of this structure in bytes.
    //

    ULONG ulStructureLength;

    //
    // The size of the file, in bytes.
    //

    ULARGE_INTEGER ullFileSize;

    //
    // The size of a receive block, in bytes.
    //

    ULONG ulBlockSize;

} TRANSPORTCLIENT_SESSION_INFO, *PTRANSPORTCLIENT_SESSION_INFO;

#define WDS_TRANSPORTCLIENT_NO_CACHE                0

//
// Download status.
//
#define WDS_TRANSPORTCLIENT_STATUS_IN_PROGRESS      0x0001
#define WDS_TRANSPORTCLIENT_STATUS_SUCCESS          0x0002
#define WDS_TRANSPORTCLIENT_STATUS_FAILURE          0x0003

//----------------------------------------------------------------- Callbacks.

typedef
VOID
(CALLBACK *PFN_WdsTransportClientSessionStart) (
    _In_ HANDLE hSessionKey,
    _In_ PVOID pCallerData,
    _In_ PULARGE_INTEGER ullFileSize
    );

typedef
VOID
(CALLBACK *PFN_WdsTransportClientSessionStartEx) (
    _In_ HANDLE hSessionKey,
    _In_ PVOID pCallerData,
    _In_ PTRANSPORTCLIENT_SESSION_INFO Info
    );

typedef
VOID
(CALLBACK *PFN_WdsTransportClientReceiveMetadata) (
    _In_ HANDLE hSessionKey,
    _In_ PVOID pCallerData,
    _In_reads_bytes_(ulSize) PVOID pMetadata,
    _In_ ULONG ulSize
    );

typedef
VOID
(CALLBACK *PFN_WdsTransportClientReceiveContents) (
    _In_ HANDLE hSessionKey,
    _In_ PVOID pCallerData,
    _In_reads_bytes_(ulSize) PVOID pContents,
    _In_ ULONG ulSize,
    _In_ PULARGE_INTEGER pullContentOffset
    );

typedef
VOID
(CALLBACK *PFN_WdsTransportClientSessionComplete) (
    _In_ HANDLE hSessionKey,
    _In_ PVOID pCallerData,
    _In_ DWORD dwError
    );

typedef
VOID
(CALLBACK *PFN_WdsTransportClientSessionNegotiate) (
    _In_ HANDLE hSessionKey,
    _In_ PVOID pCallerData,
    _In_ PTRANSPORTCLIENT_SESSION_INFO pInfo,
    _In_ HANDLE hNegotiateKey
    );

//----------------------------------------------------------------- Structures.

typedef struct _WDS_TRANSPORTCLIENT_REQUEST
{
    //
    // The length of this structure.
    //
    ULONG ulLength;

    //
    // The version of the api that the caller is built against.  Callers should
    // always set this field to WDS_TRANSPORTCLIENT_CURRENT_API_VERSION.
    //
    ULONG ulApiVersion;

    //
    // The level of authentication to send to the server.
    //
    ULONG ulAuthLevel;

    //
    // Server name.
    //
    LPCWSTR pwszServer;

    //
    // Namespace of the object to retrieve.
    //
    LPCWSTR pwszNamespace;

    //
    // Specifies the name of the object to retrieve.  Object names are
    // provider dependant.
    //
    LPCWSTR pwszObjectName;

    //
    // Specifies how much data in bytes the consumer can store in its queue.  Once
    // this threshold is reached, the client will not send any more writes to
    // the consumer until some memory is released with
    // WdsTransportClientCompleteWrite.
    //
    ULONG ulCacheSize;

    //
    // Specifies the protocol to be used for this transfer.
    //
    ULONG ulProtocol;

    //
    // Protocol specific information.
    //
    PVOID pvProtocolData;

    //
    // The length of the protocol data pointed to by pvProtocolData.
    //
    ULONG ulProtocolDataLength;
} WDS_TRANSPORTCLIENT_REQUEST, *PWDS_TRANSPORTCLIENT_REQUEST;

typedef struct _WDS_TRANSPORTCLIENT_CALLBACKS
{
    PFN_WdsTransportClientSessionStart SessionStart;
    PFN_WdsTransportClientSessionStartEx SessionStartEx;
    PFN_WdsTransportClientReceiveContents ReceiveContents;
    PFN_WdsTransportClientReceiveMetadata ReceiveMetadata;
    PFN_WdsTransportClientSessionComplete SessionComplete;
    PFN_WdsTransportClientSessionNegotiate SessionNegotiate;
} WDS_TRANSPORTCLIENT_CALLBACKS, *PWDS_TRANSPORTCLIENT_CALLBACKS;

//----------------------------------------------------------------- Functions.

DWORD
WDSTCIAPI
WdsTransportClientInitialize (
    VOID
    );

DWORD
WDSTCIAPI
WdsTransportClientInitializeSession (
    _In_ PWDS_TRANSPORTCLIENT_REQUEST pSessionRequest,
    _In_ PVOID pCallerData,
    _Out_ PHANDLE hSessionKey
    );

DWORD
WDSTCIAPI
WdsTransportClientRegisterCallback (
    _In_ HANDLE hSessionKey,
    _In_ TRANSPORTCLIENT_CALLBACK_ID CallbackId,
    _In_ PVOID pfnCallback
    );

DWORD
WDSTCIAPI
WdsTransportClientStartSession (
    _In_ HANDLE hSessionKey
    );

DWORD
WDSTCIAPI
WdsTransportClientCompleteReceive (
    _In_ HANDLE hSessionKey,
    _In_ ULONG ulSize,
    _In_ PULARGE_INTEGER pullOffset
    );

DWORD
WDSTCIAPI
WdsTransportClientCancelSession (
    _In_ HANDLE hSessionKey
    );

DWORD
WDSTCIAPI
WdsTransportClientCancelSessionEx (
    _In_ HANDLE hSessionKey,
    _In_ DWORD dwErrorCode
    );

DWORD
WDSTCIAPI
WdsTransportClientWaitForCompletion (
    _In_ HANDLE hSessionKey,
    _In_ ULONG uTimeout
    );

DWORD
WDSTCIAPI
WdsTransportClientQueryStatus (
    _In_ HANDLE hSessionKey,
    _Out_ PULONG puStatus,
    _Out_ PULONG puErrorCode
    );

DWORD
WDSTCIAPI
WdsTransportClientCloseSession (
    _In_ HANDLE hSessionKey
    );

DWORD
WDSTCIAPI
WdsTransportClientAddRefBuffer (
    _In_ PVOID pvBuffer
    );

DWORD
WDSTCIAPI
WdsTransportClientReleaseBuffer (
    _In_ PVOID pvBuffer
    );

DWORD
WDSTCIAPI
WdsTransportClientShutdown (
    VOID
    );

DWORD
WDSTCIAPI
WdsTransportClientNegotiateRequiredDataRange (
    _In_ HANDLE hSessionKey,
    _In_ HANDLE hNegotiateKey,
    _In_ ULARGE_INTEGER ullDataRangeStart,
    _In_ ULARGE_INTEGER ullDataRangeSize
    );

#ifdef __cplusplus
}
#endif


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif
