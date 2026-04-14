/********************************************************************************
*                                                                               *
* enclaveapi.h -- ApiSet Contract for api-ms-win-core-enclave-l1-1-0            *
*                                                                               *
* Copyright (c) Microsoft Corporation. All rights reserved.                     *
*                                                                               *
********************************************************************************/

#ifdef _MSC_VER
#pragma once
#endif // _MSC_VER

#ifndef _ENCLAVEAPI_H_
#define _ENCLAVEAPI_H_

#include <apiset.h>
#include <apisetcconv.h>
#include <minwindef.h>
#include <minwinbase.h>

#ifdef __cplusplus
extern "C" {
#endif

#pragma region Desktop Family or OneCore Or App Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_APP)

WINBASEAPI
_Success_(return != FALSE)
BOOL
WINAPI
IsEnclaveTypeSupported(
    _In_ DWORD flEnclaveType
    );

WINBASEAPI
_Ret_maybenull_
_Post_writable_byte_size_(dwSize)
LPVOID
WINAPI
CreateEnclave(
    _In_ HANDLE hProcess,
    _In_opt_ LPVOID lpAddress,
    _In_ SIZE_T dwSize,
    _In_ SIZE_T dwInitialCommitment,
    _In_ DWORD flEnclaveType,
    _In_reads_bytes_(dwInfoLength) LPCVOID lpEnclaveInformation,
    _In_ DWORD dwInfoLength,
    _Out_opt_ LPDWORD lpEnclaveError
    );

WINBASEAPI
_Success_(return != FALSE)
BOOL
WINAPI
LoadEnclaveData(
    _In_ HANDLE hProcess,
    _In_ LPVOID lpAddress,
    _In_reads_bytes_(nSize) LPCVOID lpBuffer,
    _In_ SIZE_T nSize,
    _In_ DWORD flProtect,
    _In_reads_bytes_(dwInfoLength) LPCVOID lpPageInformation,
    _In_ DWORD dwInfoLength,
    _Out_ PSIZE_T lpNumberOfBytesWritten,
    _Out_opt_ LPDWORD lpEnclaveError
    );

WINBASEAPI
_Success_(return != FALSE)
BOOL
WINAPI
InitializeEnclave(
    _In_ HANDLE hProcess,
    _In_ LPVOID lpAddress,
    _In_reads_bytes_(dwInfoLength) LPCVOID lpEnclaveInformation,
    _In_ DWORD dwInfoLength,
    _Out_opt_ LPDWORD lpEnclaveError
    );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_APP) */
#pragma endregion

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

WINBASEAPI
_Success_(return != FALSE)
BOOL
WINAPI
LoadEnclaveImageA(
    _In_ LPVOID lpEnclaveAddress,
    _In_ LPCSTR lpImageName
    );

WINBASEAPI
_Success_(return != FALSE)
BOOL
WINAPI
LoadEnclaveImageW(
    _In_ LPVOID lpEnclaveAddress,
    _In_ LPCWSTR lpImageName
    );

#ifdef UNICODE
#define LoadEnclaveImage  LoadEnclaveImageW
#else
#define LoadEnclaveImage  LoadEnclaveImageA
#endif // !UNICODE

WINBASEAPI
_Success_(return != FALSE)
BOOL
WINAPI
CallEnclave(
    _In_ LPENCLAVE_ROUTINE lpRoutine,
    _In_ LPVOID lpParameter,
    _In_ BOOL fWaitForThread,
    _Out_ LPVOID* lpReturnValue
    );

WINBASEAPI
_Success_(return != FALSE)
BOOL
WINAPI
TerminateEnclave(
    _In_ LPVOID lpAddress,
    _In_ BOOL fWait
    );

WINBASEAPI
_Success_(return != FALSE)
BOOL
WINAPI
DeleteEnclave(
    _In_ LPVOID lpAddress
    );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#ifdef __cplusplus
}
#endif

#endif // _ENCLAVEAPI_H_
