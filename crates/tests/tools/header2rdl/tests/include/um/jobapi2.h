/********************************************************************************
*                                                                               *
* jobapiset2.h -- ApiSet Contract for api-ms-win-core-job-l2                     *
*                                                                               *
* Copyright (c) Microsoft Corporation. All rights reserved.                     *
*                                                                               *
********************************************************************************/

#pragma once
#ifndef _JOBAPISET2_H_
#define _JOBAPISET2_H_

#include <apiset.h>
#include <apisetcconv.h>
#include <minwindef.h>
#include <minwinbase.h>

#ifdef __cplusplus
extern "C" {
#endif

#pragma region Desktop Family or OneCore Family

#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

typedef struct JOBOBJECT_IO_RATE_CONTROL_INFORMATION {
    LONG64 MaxIops;
    LONG64 MaxBandwidth;
    LONG64 ReservationIops;
    PCWSTR VolumeName;
    ULONG BaseIoSize;
    ULONG ControlFlags;
} JOBOBJECT_IO_RATE_CONTROL_INFORMATION;

WINBASEAPI
HANDLE
WINAPI
CreateJobObjectW(
    _In_opt_ LPSECURITY_ATTRIBUTES lpJobAttributes,
    _In_opt_ LPCWSTR lpName
    );

WINBASEAPI
VOID
WINAPI
FreeMemoryJobObject(
    _In_ _Frees_ptr_ VOID* Buffer
    );

WINBASEAPI
HANDLE
WINAPI
OpenJobObjectW(
    _In_ DWORD dwDesiredAccess,
    _In_ BOOL bInheritHandle,
    _In_ LPCWSTR lpName
    );

WINBASEAPI
BOOL
WINAPI
AssignProcessToJobObject(
    _In_ HANDLE hJob,
    _In_ HANDLE hProcess
    );

WINBASEAPI
BOOL
WINAPI
TerminateJobObject(
    _In_ HANDLE hJob,
    _In_ UINT uExitCode
    );

WINBASEAPI
BOOL
WINAPI
SetInformationJobObject(
    _In_ HANDLE hJob,
    _In_ JOBOBJECTINFOCLASS JobObjectInformationClass,
    _In_reads_bytes_(cbJobObjectInformationLength) LPVOID lpJobObjectInformation,
    _In_ DWORD cbJobObjectInformationLength
    );

WINBASEAPI
DWORD
WINAPI
SetIoRateControlInformationJobObject(
    _In_ HANDLE hJob,
    _In_ JOBOBJECT_IO_RATE_CONTROL_INFORMATION* IoRateControlInfo
    );

WINBASEAPI
BOOL
WINAPI
QueryInformationJobObject(
    _In_opt_ HANDLE hJob,
    _In_ JOBOBJECTINFOCLASS JobObjectInformationClass,
    _Out_writes_bytes_to_(cbJobObjectInformationLength, *lpReturnLength) LPVOID lpJobObjectInformation,
    _In_ DWORD cbJobObjectInformationLength,
    _Out_opt_ LPDWORD lpReturnLength
    );

WINBASEAPI
DWORD
WINAPI
QueryIoRateControlInformationJobObject(
    _In_opt_ HANDLE hJob,
    _In_opt_ PCWSTR VolumeName,
    _Outptr_result_buffer_(*InfoBlockCount) JOBOBJECT_IO_RATE_CONTROL_INFORMATION** InfoBlocks,
    _Out_ ULONG* InfoBlockCount
    );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM) */

#pragma endregion

#ifdef __cplusplus
}
#endif

#endif // _JOBAPISET2_H_
