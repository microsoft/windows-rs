/********************************************************************************
*                                                                               *
* netiso.h -- ApiSet Contract for api-ms-win-netsec-isolation-l1                *
*                                                                               *
* Copyright (c) Microsoft Corporation. All rights reserved.                     *
*                                                                               *
********************************************************************************/

#pragma once

#ifndef _NETISO_H_
#define _NETISO_H_

#include <apiset.h>
#include <apisetcconv.h>
#include <minwindef.h>
#include <minwinbase.h>

#if NTDDI_VERSION >= NTDDI_WIN8

#ifdef __cplusplus
extern "C" {
#endif

#pragma region Desktop Family or Firewall Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_MPSSVC)

#ifndef __NET_ISOLATION_TYPES__
#define __NET_ISOLATION_TYPES__

typedef enum NETISO_FLAG
{
    NETISO_FLAG_FORCE_COMPUTE_BINARIES = 0x1,
    NETISO_FLAG_MAX = 0x2
} NETISO_FLAG;

typedef enum _INET_FIREWALL_AC_CREATION_TYPE
{
    INET_FIREWALL_AC_NONE	 		= 0x00,
    INET_FIREWALL_AC_PACKAGE_ID_ONLY 		= 0x01,
    INET_FIREWALL_AC_BINARY			= 0x02,
    INET_FIREWALL_AC_MAX			= 0x04
} INET_FIREWALL_AC_CREATION_TYPE;

typedef enum _INET_FIREWALL_AC_CHANGE_TYPE
{
    INET_FIREWALL_AC_CHANGE_INVALID	= 0,
    INET_FIREWALL_AC_CHANGE_CREATE 	= 1,
    INET_FIREWALL_AC_CHANGE_DELETE	= 2,
    INET_FIREWALL_AC_CHANGE_MAX	= 3
} INET_FIREWALL_AC_CHANGE_TYPE;

typedef struct _INET_FIREWALL_AC_CAPABILITIES
{
    DWORD count;
    SID_AND_ATTRIBUTES * capabilities;
} INET_FIREWALL_AC_CAPABILITIES, *PINET_FIREWALL_AC_CAPABILITIES;

typedef struct _INET_FIREWALL_AC_BINARIES
{
    DWORD count;
    LPWSTR *binaries;
} INET_FIREWALL_AC_BINARIES, *PINET_FIREWALL_AC_BINARIES;

typedef struct _INET_FIREWALL_AC_CHANGE
{
    INET_FIREWALL_AC_CHANGE_TYPE changeType;
    INET_FIREWALL_AC_CREATION_TYPE createType;
    SID *appContainerSid;
    SID* userSid;
    LPWSTR displayName;

    union
    {
        INET_FIREWALL_AC_CAPABILITIES capabilities;
        INET_FIREWALL_AC_BINARIES binaries;
    } u;

} INET_FIREWALL_AC_CHANGE, *PINET_FIREWALL_AC_CHANGE;

typedef struct _INET_FIREWALL_APP_CONTAINER
{
    SID *appContainerSid;
    SID *userSid;
    LPWSTR appContainerName;
    LPWSTR displayName;
    LPWSTR description;
    INET_FIREWALL_AC_CAPABILITIES capabilities;
    INET_FIREWALL_AC_BINARIES binaries;
    LPWSTR workingDirectory;
    LPWSTR packageFullName;
} INET_FIREWALL_APP_CONTAINER, *PINET_FIREWALL_APP_CONTAINER;

#endif //__NET_ISOLATION_TYPES__
typedef void (CALLBACK *PAC_CHANGES_CALLBACK_FN) (
    _In_opt_ void *context,
    _In_ const INET_FIREWALL_AC_CHANGE *pChange
    );

HRESULT
WINAPI
NetworkIsolationSetupAppContainerBinaries(
    _In_ PSID applicationContainerSid,
    _In_ LPCWSTR packageFullName,
    _In_ LPCWSTR packageFolder,
    _In_ LPCWSTR displayName,
    _In_ BOOL bBinariesFullyComputed,
    _In_reads_(binariesCount) LPCWSTR* binaries,
    _In_ DWORD binariesCount
    );

DWORD
WINAPI
NetworkIsolationRegisterForAppContainerChanges(
    _In_ DWORD flags,
    _In_ PAC_CHANGES_CALLBACK_FN callback,
    _In_opt_ PVOID context,
    _Out_ HANDLE* registrationObject
    );

DWORD
WINAPI
NetworkIsolationUnregisterForAppContainerChanges(
    _In_ HANDLE registrationObject
    );

//APISET
//DLOAD_RET(ERROR_NOT_SUPPORTED)
//HRESULT
//WINAPI
//NetworkIsolationEnumerateAppContainerRules(
//    API_SAL("_Outptr_") IEnumVARIANT ** newEnum
//    );

DWORD
WINAPI
NetworkIsolationFreeAppContainers(
    _In_ PINET_FIREWALL_APP_CONTAINER pPublicAppCs
    );

DWORD
WINAPI
NetworkIsolationEnumAppContainers(
    DWORD Flags,
    _Out_ DWORD* pdwNumPublicAppCs,
    _Outptr_result_buffer_(*pdwNumPublicAppCs) PINET_FIREWALL_APP_CONTAINER* ppPublicAppCs
    );

DWORD
WINAPI
NetworkIsolationGetAppContainerConfig(
    _Out_ DWORD* pdwNumPublicAppCs,
    _Outptr_result_buffer_(*pdwNumPublicAppCs) PSID_AND_ATTRIBUTES* appContainerSids
    );

DWORD
WINAPI
NetworkIsolationSetAppContainerConfig(
    _In_ DWORD dwNumPublicAppCs,
    _In_reads_(dwNumPublicAppCs) PSID_AND_ATTRIBUTES appContainerSids
    );

#ifndef __NET_ISOLATION_DIAG_TYPES__
#define __NET_ISOLATION_DIAG_TYPES__
typedef enum _NETISO_ERROR_TYPE
{
    NETISO_ERROR_TYPE_NONE                      = 0x00,
    NETISO_ERROR_TYPE_PRIVATE_NETWORK           = 0x01,
    NETISO_ERROR_TYPE_INTERNET_CLIENT           = 0x02,
    NETISO_ERROR_TYPE_INTERNET_CLIENT_SERVER    = 0x03,
    NETISO_ERROR_TYPE_MAX                       = 0x04
} NETISO_ERROR_TYPE;
#endif //__NET_ISOLATION_DIAG_TYPES__

DWORD
NetworkIsolationDiagnoseConnectFailure(
    __in LPCWSTR wszServerName
    );

DWORD
WINAPI
NetworkIsolationDiagnoseConnectFailureAndGetInfo(
    _In_ LPCWSTR wszServerName,
    _Out_ NETISO_ERROR_TYPE* netIsoError
    );

#endif // WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_MPSSVC)
#pragma endregion

#ifdef __cplusplus
}
#endif

#endif // NTDDI

#endif // _NETISO_H_
