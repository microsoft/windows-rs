//
// Copyright (C) Microsoft Corporation. All rights reserved.
//
// Contents: On Demand Connection Route Helper API Prototypes and Definitions
//

#ifndef _ONDEMANDCONNROUTEHELPER_H
#define _ONDEMANDCONNROUTEHELPER_H

#include <winapifamily.h>
#include <unknwn.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

#if NTDDI_VERSION >= NTDDI_WINBLUE

#ifdef _MSC_VER
#pragma once
#endif

STDAPI OnDemandGetRoutingHint(_In_ PCWSTR destinationHostName, _Out_ DWORD* interfaceIndex);

typedef void(WINAPI* ONDEMAND_NOTIFICATION_CALLBACK)(_In_opt_ void*);

STDAPI OnDemandRegisterNotification(_In_ ONDEMAND_NOTIFICATION_CALLBACK callback, _In_opt_ void* callbackContext, _Out_ HANDLE* registrationHandle);

STDAPI OnDemandUnRegisterNotification(_In_ HANDLE registrationHandle);

#endif // NTDDI_VERSION >= NTDDI_WINBLUE

#if NTDDI_VERSION >= NTDDI_WINTHRESHOLD

#ifdef _MSC_VER
#pragma once
#endif

#define NET_INTERFACE_FLAG_NONE 0x00000000
#define NET_INTERFACE_FLAG_CONNECT_IF_NEEDED 0x00000001

typedef struct _NET_INTERFACE_CONTEXT
{
    ULONG InterfaceIndex;
    LPWSTR ConfigurationName;
} NET_INTERFACE_CONTEXT;

typedef struct _NET_INTERFACE_CONTEXT_TABLE
{
    HANDLE InterfaceContextHandle;
    UINT NumberOfEntries;
    NET_INTERFACE_CONTEXT* InterfaceContextArray;
} NET_INTERFACE_CONTEXT_TABLE;

//
// Retrieve an interface context table for the given hostname and connection profile filter.
//
// Parameters:
//
// HostName                             - Optional destination hostname.
// ProxyName                            - Optional HTTP proxy name.
// Flags                                - NET_INTERFACE_FLAG_NONE: Default behavior.
//                                        NET_INTERFACE_FLAG_CONNECT_IF_NEEDED: Flags to indicate whether the underlying connection
//                                                                              should be activated or not.
// ConnectionProfileFilterRawData       - Optional pointer to the connection profile filter blob (byte cast of wcm_selection_filters)
// ConnectionProfileFilterRawDataSize   - Size in bytes of ConnectionProfileFilterRawData
// InterfaceContextTable                - On output, set to the list of NET_INTERFACE_CONTEXT structures containing interface indices
//                                        and configuration names that can be used for the hostname and filter.
//
// Return:
//
// S_OK             - Returned if connections that satisfy the parameters and internal policies exists. InterfaceContextTable will contain a
//                    list of interfaces indices and configuration names of those connections.
//                    When S_OK is returned, FreeInterfaceContextTable should be called to release the context table.
//
// S_FALSE          - Returned to indicate that any connection (or default interface) can be used for this hostname and filter. And the interface
//                    context table will be nullptr in this case since the caller can use the default route (any route) to satisfy the
//                    requirements.
//
// E_NOTFOUND       - Returned if no connection is currently available or existing connections don't meet the connection filter and the
//                    internal policy for the host. The exact return code would be HRESULT(ERROR_NOT_FOUND).
//
// E_INVALIDARG     - Returned if the caller passes an invalid argument. Returned if the caller uses an unsupported flag, bad connection filter data,
//                    incorrect size or NULL interface context table pointer.
//
// E_OUTOFMEMORY    - Returned if there is not enough memory to complete the operation.
//
// FAILED(HRESULT)  - Returned in case of failures that are outside the control of the implementation.
//
STDAPI GetInterfaceContextTableForHostName(
    _In_opt_ PCWSTR HostName,
    _In_opt_ PCWSTR ProxyName,
    _In_ DWORD Flags,
    _In_reads_bytes_opt_(ConnectionProfileFilterRawDataSize) BYTE* ConnectionProfileFilterRawData,
    _In_ DWORD ConnectionProfileFilterRawDataSize,
    _Outptr_ NET_INTERFACE_CONTEXT_TABLE** InterfaceContextTable);

//
// Frees the interface context table retrieved from GetInterfaceContextTableForHostName. Internally, this also
// deactivates the connections if needed.
//
// Parameters:
//
// InterfaceContextTable                - Interface context table retrieved using GetInterfaceContextTableForHostName.
//
STDAPI_(void) FreeInterfaceContextTable(_In_ NET_INTERFACE_CONTEXT_TABLE* InterfaceContextTable);

#endif // NTDDI_VERSION >= NTDDI_WINTHRESHOLD

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif // _ONDEMANDCONNROUTEHELPER_H
