/*++

Copyright (c) 1991-1999  Microsoft Corporation

Module Name:

    nspapip.h

Abstract:

    Internel Name Space Provider API prototypes and manifests.  This
    header file should only be included by name space providers.  NSPAPI
    users should include only nspapi.h See the "Windows NT NameSpace
    Provider Specification" document for details.

Environment:

    User Mode -Win32

Notes:

--*/

#ifndef _NSPAPIP_INCLUDED
#define _NSPAPIP_INCLUDED

#if _MSC_VER > 1000
#pragma once
#endif
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


//
// Standard priority values for the dwPriority field of NS_ROUTINE.
//
#define NS_STANDARD_FAST_PRIORITY   (500)
#define NS_MAX_FAST_PRIORITY        (999)
#define NS_STANDARD_PRIORITY        (2000)

//
// Indices for the alpfnFunctions array field of NS_ROUTINE.
//
#define NSPAPI_GET_ADDRESS_BY_NAME  (0x00000000)
#define NSPAPI_GET_SERVICE          (0x00000001)
#define NSPAPI_SET_SERVICE          (0x00000002)

//
// Structures used by the provider interface.
//

typedef
INT
(APIENTRY *LPGET_ADDR_BY_NAME_PROC) (
    _In_     LPGUID          lpServiceType,
    _In_z_   LPWSTR          lpServiceName,
    _In_     LPDWORD         lpdwProtocols,
    _In_     DWORD           dwResolution,
    _Out_writes_bytes_(*lpdwBufferLength)      LPVOID      lpCsaddrBuffer,
    _Inout_  LPDWORD     lpdwBufferLength,
    _Out_writes_bytes_(*lpdwAliasBufferlength) LPWSTR          lpAliasBuffer,
    _Inout_  LPDWORD         lpdwAliasBufferLength,
    _In_     HANDLE          hCancellationEvent
    );

typedef struct _NS_ROUTINE {
    DWORD        dwFunctionCount;
    LPFN_NSPAPI *alpfnFunctions;
    DWORD        dwNameSpace;
    DWORD        dwPriority;
} NS_ROUTINE, *PNS_ROUTINE, * FAR LPNS_ROUTINE;

typedef
DWORD

(APIENTRY *LPLOAD_NAME_SPACE_PROC) (
    _Inout_ LPDWORD         lpdwVersion,
    _Inout_updates_bytes_(*lpdwBufferLength) LPNS_ROUTINE    nsrBuffer,
    _Inout_ LPDWORD         lpdwBufferLength
    );

typedef
INT
(APIENTRY *LPGET_SERVICE_PROC) (
    _In_     LPGUID          lpServiceType,
    _In_z_   LPWSTR          lpServiceName,
    _In_     DWORD           dwProperties,
    _In_     BOOL            fUnicodeBlob,
    _Out_writes_bytes_(*lpdwBufferLen)    LPSERVICE_INFO  lpServiceInfo,
    _Inout_  LPDWORD         lpdwBufferLen
    );

typedef
DWORD
(APIENTRY *LPSET_SERVICE_PROC) (
    _In_     DWORD           dwOperation,
    _In_     DWORD           dwFlags,
    _In_     BOOL            fUnicodeBlob,
    _In_     LPSERVICE_INFO  lpServiceInfo
    );

//
// Internal Functions
//
DWORD
APIENTRY
NPGetService (
    _In_     LPGUID          lpServiceType,
    _In_z_   LPWSTR          lpServiceName,
    _In_     DWORD           dwProperties,
    _In_     BOOL            fUnicodeBlob,
    _Out_writes_bytes_(*lpdwBufferLen)    LPSERVICE_INFO  lpServiceInfo,
    _Inout_  LPDWORD         lpdwBufferLen
    );

DWORD
APIENTRY
NPSetService (
    _In_     DWORD           dwOperation,
    _In_     DWORD           dwFlags,
    _In_     BOOL            fUnicodeBlob,
    _In_     LPSERVICE_INFO  lpServiceInfo
    );

INT
APIENTRY
NPGetAddressByName (
    _In_     LPGUID          lpServiceType,
    _In_z_   LPWSTR          lpServiceName,
    _In_     LPDWORD         lpdwProtocols,
    _In_     DWORD           dwResolution,
    _Out_writes_bytes_(*lpdwBufferLength) LPVOID          lpCsaddrBuffer,
    _Inout_  LPDWORD         lpdwBufferLength,
    _Out_writes_bytes_(*lpdwAliasBufferlength) LPWSTR          lpAliasBuffer,
    _Inout_  LPDWORD         lpdwAliasBufferLength,
    _In_     HANDLE          hCancellationEvent
    );

INT
APIENTRY
NPLoadNameSpaces (
    _Inout_ LPDWORD         lpdwVersion,
    _Inout_updates_bytes_(*lpdwBufferLength) LPNS_ROUTINE    nsrBuffer,
    _Inout_ LPDWORD         lpdwBufferLength
    );


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif  // _NSPAPIP_INCLUDED
