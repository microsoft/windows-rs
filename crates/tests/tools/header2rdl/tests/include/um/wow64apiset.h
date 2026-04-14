/********************************************************************************
*                                                                               *
* wow64app.h - ApiSet Contract for api-ms-win-core-wow64-l1                     *
*                                                                               *
* Copyright (c) Microsoft Corporation. All rights reserved.                     *
*                                                                               *
********************************************************************************/

#ifdef _MSC_VER
#pragma once
#endif // _MSC_VER

#ifndef _WOW64APISET_H_
#define _WOW64APISET_H_

#include <apiset.h>
#include <apisetcconv.h>
#include <minwindef.h>
#include <minwinbase.h>

#ifdef __cplusplus
extern "C" {
#endif

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

// RC warns because "WINBASE_DECLARE_GET_SYSTEM_WOW64_DIRECTORY" is a bit long.
#if !defined(RC_INVOKED)
#if _WIN32_WINNT >= 0x0501 || defined(WINBASE_DECLARE_GET_SYSTEM_WOW64_DIRECTORY)

WINBASEAPI
BOOLEAN
WINAPI
Wow64EnableWow64FsRedirection(
    _In_ BOOLEAN Wow64FsEnableRedirection
    );

WINBASEAPI
BOOL
WINAPI
Wow64DisableWow64FsRedirection(
    _Out_ PVOID* OldValue
    );

WINBASEAPI
BOOL
WINAPI
Wow64RevertWow64FsRedirection(
    _In_ PVOID OlValue
    );

#endif // _WIN32_WINNT >= 0x0501
#endif // !defined(RC_INVOKED)

#endif // WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)
#pragma endregion

#pragma region Application Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)

#if (_WIN32_WINNT >= 0x0501)

WINBASEAPI
BOOL
WINAPI
IsWow64Process(
    _In_ HANDLE hProcess,
    _Out_ PBOOL Wow64Process
    );

#endif // _WIN32_WINNT >= 0x0501

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

// RC warns because "WINBASE_DECLARE_GET_SYSTEM_WOW64_DIRECTORY" is a bit long.
#if !defined(RC_INVOKED)
#if _WIN32_WINNT >= 0x0501 || defined(WINBASE_DECLARE_GET_SYSTEM_WOW64_DIRECTORY)

WINBASEAPI
_Must_inspect_result_
_Success_(return != 0 && return < uSize)
UINT
WINAPI
GetSystemWow64DirectoryA(
    _Out_writes_to_opt_(uSize,return + 1) LPSTR lpBuffer,
    _In_ UINT uSize
    );

WINBASEAPI
_Must_inspect_result_
_Success_(return != 0 && return < uSize)
UINT
WINAPI
GetSystemWow64DirectoryW(
    _Out_writes_to_opt_(uSize,return + 1) LPWSTR lpBuffer,
    _In_ UINT uSize
    );

#ifdef UNICODE
#define GetSystemWow64Directory  GetSystemWow64DirectoryW
#else
#define GetSystemWow64Directory  GetSystemWow64DirectoryA
#endif // !UNICODE

#endif // _WIN32_WINNT >= 0x0501
#endif // !defined(RC_INVOKED)

#if (_WIN32_WINNT >= 0x0A00)

WINBASEAPI
USHORT
WINAPI
Wow64SetThreadDefaultGuestMachine(
    _In_ USHORT Machine
    );

#endif // _WIN32_WINNT >= 0x0A00

#endif // WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)
#pragma endregion

#pragma region Application Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)

#if (_WIN32_WINNT >= 0x0A00)

WINBASEAPI
BOOL
WINAPI
IsWow64Process2(
    _In_ HANDLE hProcess,
    _Out_ USHORT* pProcessMachine,
    _Out_opt_ USHORT* pNativeMachine
    );

#endif // _WIN32_WINNT >= 0x0A00

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

#if (_WIN32_WINNT >= 0x0A00)

WINBASEAPI
_Must_inspect_result_
_Success_(return != 0 && return < uSize)
UINT
WINAPI
GetSystemWow64Directory2A(
    _Out_writes_to_opt_(uSize,return + 1) LPSTR lpBuffer,
    _In_ UINT uSize,
    _In_ WORD ImageFileMachineType
    );

WINBASEAPI
_Must_inspect_result_
_Success_(return != 0 && return < uSize)
UINT
WINAPI
GetSystemWow64Directory2W(
    _Out_writes_to_opt_(uSize,return + 1) LPWSTR lpBuffer,
    _In_ UINT uSize,
    _In_ WORD ImageFileMachineType
    );

#ifdef UNICODE
#define GetSystemWow64Directory2  GetSystemWow64Directory2W
#else
#define GetSystemWow64Directory2  GetSystemWow64Directory2A
#endif // !UNICODE

#endif // _WIN32_WINNT >= 0x0A00

#if (_WIN32_WINNT >= 0x0A00)

WINBASEAPI
_Must_inspect_result_
HRESULT
WINAPI
IsWow64GuestMachineSupported(
    _In_ USHORT WowGuestMachine,
    _Out_ BOOL* MachineIsSupported
    );

#endif // _WIN32_WINNT >= 0x0A00

#if (_WIN32_WINNT >= 0x0600)

WINBASEAPI
BOOL
WINAPI
Wow64GetThreadContext(
    _In_ HANDLE hThread,
    _Inout_ PWOW64_CONTEXT lpContext
    );

WINBASEAPI
BOOL
WINAPI
Wow64SetThreadContext(
    _In_ HANDLE hThread,
    _In_ CONST WOW64_CONTEXT* lpContext
    );

WINBASEAPI
DWORD
WINAPI
Wow64SuspendThread(
    _In_ HANDLE hThread
    );

#endif // (_WIN32_WINNT >= 0x0600)

#endif // WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)
#pragma endregion

#ifdef __cplusplus
}
#endif

#endif // _WOW64APISET_H_
