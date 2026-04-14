/********************************************************************************
*                                                                               *
* srpapi.h -- ApiSet Contract for ext-ms-win-security-srp-l1                    *
*                                                                               *
* Copyright (c) Microsoft Corporation. All rights reserved.                     *
*                                                                               *
********************************************************************************/

#ifndef _APISETSRPEXT_
#define _APISETSRPEXT_

#include <apiset.h>
#include <apisetcconv.h>

#ifdef _CONTRACT_GEN                                   // Header(s) needed for contract generation only.
#include <nt.h>
#include <ntrtl.h>
#include <nturtl.h>
#include <apiquery.h>
#include <Windows.h>
#include <nlsp.h>
#include <nls.h>
#include <setupapi.h>
#include <cfgmgr32.h>
#include <winsafer.h>
#endif

#include <minwindef.h>
#include <minwinbase.h>

#if _MSC_VER > 1000
#pragma once
#endif

#define HR_PROC_NOT_FOUND HRESULT_FROM_WIN32(ERROR_PROC_NOT_FOUND)

#ifdef __cplusplus
extern "C" {
#endif

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

#if (_WIN32_WINNT >= _WIN32_WINNT_WINTHRESHOLD)

typedef struct HTHREAD_NETWORK_CONTEXT
{
    DWORD ThreadId;
    HANDLE ThreadContext;
} HTHREAD_NETWORK_CONTEXT;

STDAPI
SrpCreateThreadNetworkContext(
    _In_ PCWSTR enterpriseId,
    _Out_ HTHREAD_NETWORK_CONTEXT* threadNetworkContext
    );

STDAPI
SrpCloseThreadNetworkContext(
    _Inout_ HTHREAD_NETWORK_CONTEXT* threadNetworkContext
    );

STDAPI
SrpSetTokenEnterpriseId(
    _In_ HANDLE tokenHandle,
    _In_opt_ PCWSTR enterpriseId
    );

STDAPI
SrpGetEnterpriseIds(
    _In_ HANDLE tokenHandle,
    _Inout_opt_ PULONG numberOfBytes,
    _Out_writes_bytes_opt_(*numberOfBytes) PCWSTR* enterpriseIds,
    _Out_ PULONG enterpriseIdCount
    );

STDAPI
SrpEnablePermissiveModeFileEncryption(
    _In_opt_ PCWSTR enterpriseId
    );

STDAPI
SrpDisablePermissiveModeFileEncryption(
    VOID
    );

#endif /* _WIN32_WINNT >= _WIN32_WINNT_WINTHRESHOLD */

typedef enum
{
    ENTERPRISE_POLICY_NONE        = 0x0,
    ENTERPRISE_POLICY_ALLOWED     = 0x1,
    ENTERPRISE_POLICY_ENLIGHTENED = 0x2,
    ENTERPRISE_POLICY_EXEMPT      = 0x4
} ENTERPRISE_DATA_POLICIES;

DEFINE_ENUM_FLAG_OPERATORS(ENTERPRISE_DATA_POLICIES);

STDAPI
SrpGetEnterprisePolicy(
    _In_ HANDLE tokenHandle,
    _Out_ ENTERPRISE_DATA_POLICIES* policyFlags
    );

NTSTATUS
SrpIsTokenService(
    _In_ HANDLE TokenHandle,
    _Out_ BOOLEAN* IsTokenService
    );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP)

#if (_WIN32_WINNT >= _WIN32_WINNT_WINTHRESHOLD)

#include <appmodel.h>

STDAPI
SrpDoesPolicyAllowAppExecution(
    _In_ const PACKAGE_ID* packageId,
    _Out_ BOOL* isAllowed
    );

struct _SRP_REQUEST;
typedef struct _SRP_REQUEST *PSRP_REQUEST;

NTSTATUS
SrpIsAllowed(
    _In_ PSRP_REQUEST FileInfo
    );

#endif /* _WIN32_WINNT >= _WIN32_WINNT_WINTHRESHOLD */

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP) */
#pragma endregion

typedef enum
{
    SRPHOSTING_TYPE_NONE = 0,
    SRPHOSTING_TYPE_WINHTTP = 1,
    SRPHOSTING_TYPE_WININET = 2
} SRPHOSTING_TYPE;

typedef enum
{
   SRPHOSTING_VERSION1 = 1
}  SRPHOSTING_VERSION;

STDAPI
SrpHostingInitialize(
    _In_ SRPHOSTING_VERSION Version,
    _In_ SRPHOSTING_TYPE Type,
    _In_ VOID* pvData,
    _In_ ULONG cbData
);

VOID
SrpHostingTerminate(
    _In_ SRPHOSTING_TYPE Type
);

#ifdef __cplusplus
}
#endif

#endif // APISETSRPEXT

#ifndef ext_ms_win_security_srp_l1_1_1_query_routines
#define ext_ms_win_security_srp_l1_1_1_query_routines

//
//Private Extension API Query Routines
//

#ifdef __cplusplus
extern "C" {
#endif

#if (_WIN32_WINNT >= _WIN32_WINNT_WINTHRESHOLD)

BOOLEAN
__stdcall
IsSrpCreateThreadNetworkContextPresent(
    VOID
    );

BOOLEAN
__stdcall
IsSrpCloseThreadNetworkContextPresent(
    VOID
    );

BOOLEAN
__stdcall
IsSrpSetTokenEnterpriseIdPresent(
    VOID
    );

BOOLEAN
__stdcall
IsSrpGetEnterpriseIdsPresent(
    VOID
    );

BOOLEAN
__stdcall
IsSrpEnablePermissiveModeFileEncryptionPresent(
    VOID
    );

BOOLEAN
__stdcall
IsSrpDisablePermissiveModeFileEncryptionPresent(
    VOID
    );

#endif /* _WIN32_WINNT >= _WIN32_WINNT_WINTHRESHOLD */

BOOLEAN
__stdcall
IsSrpGetEnterprisePolicyPresent(
    VOID
    );

BOOLEAN
__stdcall
IsSrpIsTokenServicePresent(
    VOID
    );

#if (_WIN32_WINNT >= _WIN32_WINNT_WINTHRESHOLD)

BOOLEAN
__stdcall
IsSrpDoesPolicyAllowAppExecutionPresent(
    VOID
    );

BOOLEAN
__stdcall
IsSrpIsAllowedPresent(
    VOID
    );

#endif /* _WIN32_WINNT >= _WIN32_WINNT_WINTHRESHOLD */

#ifdef __cplusplus
}
#endif

#endif // endof guard

