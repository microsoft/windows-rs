/********************************************************************************
*                                                                               *
* powerbase.h -- ApiSet Contract for api-ms-win-power-base-l1-1-0               *
*                                                                               *
* Copyright (c) Microsoft Corporation. All rights reserved.                     *
*                                                                               *
********************************************************************************/

#ifdef _MSC_VER
#if (_MSC_VER > 1000)
#pragma once
#endif // _MSC_VER > 1000
#endif // _MSC_VER

#ifndef _POWERBASE_H_
#define _POWERBASE_H_

#include <apiset.h>
#include <apisetcconv.h>

#ifdef _CONTRACT_GEN
#include <nt.h>
#include <ntrtl.h>
#include <nturtl.h>
#include <minwindef.h>
#endif

#ifdef __cplusplus
extern "C" {
#endif

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

#ifndef _HPOWERNOTIFY_DEF_
#define _HPOWERNOTIFY_DEF_

typedef PVOID HPOWERNOTIFY, *PHPOWERNOTIFY;

#endif // _HPOWERNOTIFY_DEF_

#ifndef NT_SUCCESS
#define NTSTATUS LONG
#define _OVERRIDE_NTSTATUS_
#endif

NTSTATUS
WINAPI
CallNtPowerInformation(
    _In_ POWER_INFORMATION_LEVEL InformationLevel,
    _In_reads_bytes_opt_(InputBufferLength) PVOID InputBuffer,
    _In_ ULONG InputBufferLength,
    _Out_writes_bytes_opt_(OutputBufferLength) PVOID OutputBuffer,
    _In_ ULONG OutputBufferLength
    );

#ifdef _OVERRIDE_NTSTATUS_
#undef NTSTATUS
#endif

BOOLEAN
WINAPI
GetPwrCapabilities(
    _Out_ PSYSTEM_POWER_CAPABILITIES lpspc
    );

#if (NTDDI_VERSION >= NTDDI_WIN8)
POWER_PLATFORM_ROLE
WINAPI
PowerDeterminePlatformRoleEx(
    _In_ ULONG Version
    );

#endif

#if (NTDDI_VERSION >= NTDDI_WIN8)
DWORD
WINAPI
PowerRegisterSuspendResumeNotification(
    _In_ DWORD Flags,
    _In_ HANDLE Recipient,
    _Out_ PHPOWERNOTIFY RegistrationHandle
    );

#endif

#if (NTDDI_VERSION >= NTDDI_WIN8)
DWORD
WINAPI
PowerUnregisterSuspendResumeNotification(
    _Inout_ HPOWERNOTIFY RegistrationHandle
    );

#endif

#endif // WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)
#pragma endregion

#ifdef __cplusplus
}
#endif

#endif // _POWERBASE_H_
