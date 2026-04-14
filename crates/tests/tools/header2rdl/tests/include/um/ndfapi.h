// Copyright (C) Microsoft Corporation. All rights reserved.
//
// This file contains declaration for Network Diagnostics Framework (NDF) client API

#pragma once
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

#include <specstrings.h>
#include <ndattrib.h>

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

#ifndef __CSADDR_DEFINED__
struct SOCKET_ADDRESS_LIST;
#endif // __CSADDR_DEFINED__

typedef void* NDFHANDLE;

#if defined(FKG_FORCED_USAGE) || defined(BUILD_WINDOWS)
#define NOT_BUILD_WINDOWS_DEPRECATE_NDFAPI_STDAPI(message) STDAPI
#define NOT_BUILD_WINDOWS_DEPRECATE_NDFAPI(message)
#else
#define NOT_BUILD_WINDOWS_DEPRECATE_NDFAPI_STDAPI(message) DEPRECATED_STDAPI(message)
#define NOT_BUILD_WINDOWS_DEPRECATE_NDFAPI(message) __declspec(deprecated(message))
#endif

NOT_BUILD_WINDOWS_DEPRECATE_NDFAPI_STDAPI("NdfCreateIncident is deprecated and might not work on all platforms. For more info, see MSDN.")
NdfCreateIncident(_In_ LPCWSTR helperClassName, ULONG celt, _In_reads_(celt) HELPER_ATTRIBUTE* attributes, _Outptr_ NDFHANDLE* handle);

NOT_BUILD_WINDOWS_DEPRECATE_NDFAPI_STDAPI("NdfCreateWinSockIncident is deprecated and might not work on all platforms. For more info, see MSDN.")
NdfCreateWinSockIncident(SOCKET sock, _In_opt_ LPCWSTR host, USHORT port, _In_opt_ LPCWSTR appId, _In_opt_ SID* userId, _Outptr_ NDFHANDLE* handle);

NOT_BUILD_WINDOWS_DEPRECATE_NDFAPI_STDAPI("NdfCreateWebIncident is deprecated and might not work on all platforms. For more info, see MSDN.")
NdfCreateWebIncident(_In_ LPCWSTR url, _Outptr_ NDFHANDLE* handle);

NOT_BUILD_WINDOWS_DEPRECATE_NDFAPI_STDAPI("NdfCreateWebIncidentEx is deprecated and might not work on all platforms. For more info, see MSDN.")
NdfCreateWebIncidentEx(_In_ LPCWSTR url, BOOL useWinHTTP, _In_opt_ LPWSTR moduleName, _Outptr_ NDFHANDLE* handle);

NOT_BUILD_WINDOWS_DEPRECATE_NDFAPI_STDAPI("NdfCreateSharingIncident is deprecated and might not work on all platforms. For more info, see MSDN.")
NdfCreateSharingIncident(_In_ LPCWSTR UNCPath, _Outptr_ NDFHANDLE* handle);

NOT_BUILD_WINDOWS_DEPRECATE_NDFAPI_STDAPI("NdfCreateDNSIncident is deprecated and might not work on all platforms. For more info, see MSDN.")
NdfCreateDNSIncident(_In_ LPCWSTR hostname, WORD queryType, _Outptr_ NDFHANDLE* handle);

NOT_BUILD_WINDOWS_DEPRECATE_NDFAPI_STDAPI("NdfCreateConnectivityIncident is deprecated and might not work on all platforms. For more info, see MSDN.")
NdfCreateConnectivityIncident(_Outptr_ NDFHANDLE* handle);

NOT_BUILD_WINDOWS_DEPRECATE_NDFAPI_STDAPI("NdfCreateNetConnectionIncident is deprecated and might not work on all platforms. For more info, see MSDN.")
NdfCreateNetConnectionIncident(_Outptr_ NDFHANDLE* handle, GUID id);

NOT_BUILD_WINDOWS_DEPRECATE_NDFAPI_STDAPI("NdfCreatePnrpIncident is deprecated and might not work on all platforms. For more info, see MSDN.")
NdfCreatePnrpIncident(_In_ LPCWSTR cloudname, _In_opt_ LPCWSTR peername, _In_ BOOL diagnosePublish, _In_opt_ LPCWSTR appId, _Outptr_ NDFHANDLE* handle);

#define NDF_INBOUND_FLAG_EDGETRAVERSAL 0x00001
#define NDF_INBOUND_FLAG_HEALTHCHECK 0x00002

#ifdef __CSADDR_DEFINED__
NOT_BUILD_WINDOWS_DEPRECATE_NDFAPI_STDAPI("NdfCreateInboundIncident is deprecated and might not work on all platforms. For more info, see MSDN.")
NdfCreateInboundIncident(
    _In_opt_ LPCWSTR applicationID,
    _In_opt_ LPCWSTR serviceID,
    _In_opt_ SID* userID,
    _In_opt_ const SOCKADDR_STORAGE* localTarget,
    IPPROTO protocol,
    DWORD dwFlags,
    _Outptr_ NDFHANDLE* handle);
#endif

NOT_BUILD_WINDOWS_DEPRECATE_NDFAPI("NdfCreateGroupingIncident is deprecated and might not work on all platforms. For more info, see MSDN.")
HRESULT WINAPI NdfCreateGroupingIncident(
    _In_opt_ LPCWSTR CloudName,
    _In_opt_ LPCWSTR GroupName,
    _In_opt_ LPCWSTR Identity,
    _In_opt_ LPCWSTR Invitation,
    _In_opt_ SOCKET_ADDRESS_LIST* Addresses,
    _In_opt_ LPCWSTR appId,
    _Outptr_ NDFHANDLE* handle);

NOT_BUILD_WINDOWS_DEPRECATE_NDFAPI_STDAPI("NdfExecuteDiagnosis is deprecated and might not work on all platforms. For more info, see MSDN.")
NdfExecuteDiagnosis(_In_ NDFHANDLE handle, _In_opt_ HWND hwnd);

NOT_BUILD_WINDOWS_DEPRECATE_NDFAPI_STDAPI("NdfCloseIncident is deprecated and might not work on all platforms. For more info, see MSDN.")
NdfCloseIncident(NDFHANDLE handle);

// Available flags for NdfDiagnoseIncident
#define NDF_ADD_CAPTURE_TRACE 0x0001
#define NDF_APPLY_INCLUSION_LIST_FILTER 0x0002

// UI-less diagnosis API
#ifdef __cplusplus
NOT_BUILD_WINDOWS_DEPRECATE_NDFAPI_STDAPI("NdfDiagnoseIncident is deprecated and might not work on all platforms. For more info, see MSDN.")
NdfDiagnoseIncident(
    _In_ NDFHANDLE Handle,
    _Out_ ULONG* RootCauseCount,
    _Outptr_result_buffer_(*RootCauseCount) RootCauseInfo** RootCauses,
    DWORD dwWait = INFINITE,
    DWORD dwFlags = 0);
#else
NOT_BUILD_WINDOWS_DEPRECATE_NDFAPI_STDAPI("NdfDiagnoseIncident is deprecated and might not work on all platforms. For more info, see MSDN.")
NdfDiagnoseIncident(
    _In_ NDFHANDLE Handle,
    _Out_ ULONG* RootCauseCount,
    _Outptr_result_buffer_(*RootCauseCount) RootCauseInfo** RootCauses,
    DWORD dwWait,
    DWORD dwFlags);

#endif

// UI-less repair API
#ifdef __cplusplus
NOT_BUILD_WINDOWS_DEPRECATE_NDFAPI_STDAPI("NdfRepairIncident is deprecated and might not work on all platforms. For more info, see MSDN.")
NdfRepairIncident(_In_ NDFHANDLE Handle, _In_ RepairInfoEx* RepairEx, DWORD dwWait = INFINITE);
#else
NOT_BUILD_WINDOWS_DEPRECATE_NDFAPI_STDAPI("NdfRepairIncident is deprecated and might not work on all platforms. For more info, see MSDN.")
NdfRepairIncident(_In_ NDFHANDLE Handle, _In_ RepairInfoEx* RepairEx, DWORD dwWait);
#endif

// UI-less cancel API
NOT_BUILD_WINDOWS_DEPRECATE_NDFAPI_STDAPI("NdfCancelIncident is deprecated and might not work on all platforms. For more info, see MSDN.")
NdfCancelIncident(_In_ NDFHANDLE Handle);

NOT_BUILD_WINDOWS_DEPRECATE_NDFAPI_STDAPI("NdfGetTraceFile is deprecated and might not work on all platforms. For more info, see MSDN.")
NdfGetTraceFile(_In_ NDFHANDLE Handle, _Outptr_ LPCWSTR* TraceFileLocation);

#ifdef __cplusplus
}
#endif // defined(__cplusplus)

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion
