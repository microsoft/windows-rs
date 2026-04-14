/*++

Copyright (c) 2006 Microsoft Corporation

Module Name:

    wdstpdi.h

Abstract:

    This module defines the structures and functions that compose the interface
    that content providers must expose to the multicast server.
 
Environment:

    User Mode

--*/

#ifndef _WDSTPDI_H
#define _WDSTPDI_H
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


#ifdef __cplusplus
extern "C"
{
#endif

#define WDSTRANSPORTPROVIDERAPI        __stdcall 
#define WDSMCSAPI                      __stdcall 

//---Defines-----------------------------------------------------

#define MC_SERVER_CURRENT_VERSION 1
#define TRANSPORTPROVIDER_CURRENT_VERSION 1

#define TRANSPORT_INVALID_HANDLE (HANDLE)-1

typedef enum _TRANSPORTPROVIDER_CALLBACK_ID
{
    WDS_TRANSPORTPROVIDER_CREATE_INSTANCE = 0,
    WDS_TRANSPORTPROVIDER_COMPARE_CONTENT,
    WDS_TRANSPORTPROVIDER_OPEN_CONTENT,
    WDS_TRANSPORTPROVIDER_USER_ACCESS_CHECK,
    WDS_TRANSPORTPROVIDER_GET_CONTENT_SIZE,
    WDS_TRANSPORTPROVIDER_READ_CONTENT,
    WDS_TRANSPORTPROVIDER_CLOSE_CONTENT,
    WDS_TRANSPORTPROVIDER_CLOSE_INSTANCE,
    WDS_TRANSPORTPROVIDER_SHUTDOWN,
    WDS_TRANSPORTPROVIDER_DUMP_STATE,
    WDS_TRANSPORTPROVIDER_REFRESH_SETTINGS,
    WDS_TRANSPORTPROVIDER_GET_CONTENT_METADATA,

    WDS_TRANSPORTPROVIDER_MAX_CALLBACKS,
} TRANSPORTPROVIDER_CALLBACK_ID, *PTRANSPORTPROVIDER_CALLBACK_ID;

typedef ULONG WDS_MC_SEVERITY;

#define WDS_MC_TRACE_VERBOSE           0x00010000
#define WDS_MC_TRACE_INFO              0x00020000
#define WDS_MC_TRACE_WARNING           0x00040000
#define WDS_MC_TRACE_ERROR             0x00080000
#define WDS_MC_TRACE_FATAL             0x00100000

//---Structs-----------------------------------------------------

typedef struct _WDS_TRANSPORTPROVIDER_INIT_PARAMS
{
    //
    // The length of this structure.
    //

    ULONG ulLength;

    //
    // The multicast server's version.
    //

    ULONG ulMcServerVersion;
    
    //
    // An open handle to the registry key where this provider should
    // store and retrieve its settings.
    //
    
    HKEY hRegistryKey;

    //
    // A handle that the provider can use to uniquely identify
    // itself in calls to the multicast server.
    //

    HANDLE hProvider;
} WDS_TRANSPORTPROVIDER_INIT_PARAMS, *PWDS_TRANSPORTPROVIDER_INIT_PARAMS;

typedef struct _WDS_TRANSPORTPROVIDER_SETTINGS
{
    //
    // The length of this structure.
    //

    ULONG ulLength;

    //
    // The version of the api that this provider implements.
    //

    ULONG ulProviderVersion;
} WDS_TRANSPORTPROVIDER_SETTINGS, *PWDS_TRANSPORTPROVIDER_SETTINGS;

//---Functions the provider must implement (required callbacks)---

HRESULT
WDSTRANSPORTPROVIDERAPI
WdsTransportProviderInitialize(
    _In_ PWDS_TRANSPORTPROVIDER_INIT_PARAMS pInParameters,
    _Out_writes_bytes_(ulLength) PWDS_TRANSPORTPROVIDER_SETTINGS pSettings,
    _In_ ULONG ulLength
);

HRESULT
WDSTRANSPORTPROVIDERAPI
WdsTransportProviderCreateInstance(
    _In_ PCWSTR pwszConfigString,
    _Out_ PHANDLE phInstance
);

HRESULT
WDSTRANSPORTPROVIDERAPI
WdsTransportProviderCompareContent(
    _In_ HANDLE hInstance,
    _In_ PCWSTR pwszContentName,
    _In_ HANDLE hContent,
    _Out_ PBOOL pbContentMatches
);

HRESULT
WDSTRANSPORTPROVIDERAPI
WdsTransportProviderOpenContent(
    _In_ HANDLE hInstance,
    _In_ PCWSTR pwszContentName,
    _Out_ PHANDLE phContent
);

HRESULT
WDSTRANSPORTPROVIDERAPI
WdsTransportProviderGetContentMetadata(
    _In_ HANDLE hContent,
    _Out_writes_bytes_(*pulLength) PVOID* pvData,
    _Out_ PULONG pulLength
);

HRESULT
WDSTRANSPORTPROVIDERAPI
WdsTransportProviderUserAccessCheck(
    _In_ HANDLE hContent,
    _In_ HANDLE hUserToken,
    _Out_ PBOOL pbAccessAllowed
);

HRESULT
WDSTRANSPORTPROVIDERAPI
WdsTransportProviderGetContentSize(
    _In_ HANDLE hContent,
    _Out_ PULARGE_INTEGER pContentSize
);

HRESULT
WDSTRANSPORTPROVIDERAPI
WdsTransportProviderReadContent(
    _In_ HANDLE hContent,
    _In_reads_bytes_(ulBytesToRead) PVOID pBuffer,
    _In_ ULONG ulBytesToRead,
    _In_ PULARGE_INTEGER pContentOffset,
    _In_ PVOID pvUserData
);

HRESULT
WDSTRANSPORTPROVIDERAPI
WdsTransportProviderCloseContent(
    _In_ HANDLE hContent
);

HRESULT
WDSTRANSPORTPROVIDERAPI
WdsTransportProviderCloseInstance(
    _In_ HANDLE hInstance
);

HRESULT
WDSTRANSPORTPROVIDERAPI
WdsTransportProviderShutdown(
);

//---Functions the provider may implement (optional callbacks)-------

HRESULT
WDSTRANSPORTPROVIDERAPI
WdsTransportProviderDumpState(
);

HRESULT
WDSTRANSPORTPROVIDERAPI
WdsTransportProviderRefreshSettings(
);

//---Functions provided by the multicast server----------------------

HRESULT
WDSMCSAPI
WdsTransportServerRegisterCallback(
    _In_ HANDLE hProvider,
    _In_ TRANSPORTPROVIDER_CALLBACK_ID CallbackId,
    _In_ PVOID pfnCallback
);

HRESULT
WDSMCSAPI
WdsTransportServerCompleteRead(
    _In_ HANDLE hProvider,
    _In_ ULONG ulBytesRead,
    _In_ PVOID pvUserData,
    _In_ HRESULT hReadResult
);

HRESULT
WDSMCSAPI
WdsTransportServerTrace(
    _In_ HANDLE hProvider,
    _In_ WDS_MC_SEVERITY Severity,
    _In_ LPCWSTR pwszFormat,
    ...
);

HRESULT
WDSMCSAPI
WdsTransportServerTraceV(
    _In_ HANDLE hProvider,
    _In_ WDS_MC_SEVERITY Severity,
    _In_ LPCWSTR pwszFormat,
    _In_ va_list Params
);

PVOID
WDSMCSAPI
WdsTransportServerAllocateBuffer(
    _In_ HANDLE hProvider,
    _In_ ULONG ulBufferSize
);

HRESULT
WDSMCSAPI
WdsTransportServerFreeBuffer(
    _In_ HANDLE hProvider,
    _In_ PVOID pvBuffer
);

#ifdef __cplusplus
}
#endif


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif

