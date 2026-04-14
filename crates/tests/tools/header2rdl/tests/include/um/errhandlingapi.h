/********************************************************************************
*                                                                               *
* errhandlingapi.h - ApiSet Contract for api-ms-win-core-errorhandling-l1       *
*                                                                               *
* Copyright (c) Microsoft Corporation. All rights reserved.                     *
*                                                                               *
********************************************************************************/

#ifdef _MSC_VER
#pragma once
#endif // _MSC_VER

#ifndef _ERRHANDLING_H_
#define _ERRHANDLING_H_

#include <apiset.h>
#include <apisetcconv.h>
#include <minwindef.h>

#ifdef __cplusplus
extern "C" {
#endif

#pragma region Application Family or OneCore Family or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

//
// Typedefs
//

typedef LONG (WINAPI *PTOP_LEVEL_EXCEPTION_FILTER)(
    _In_ struct _EXCEPTION_POINTERS *ExceptionInfo
    );

typedef PTOP_LEVEL_EXCEPTION_FILTER LPTOP_LEVEL_EXCEPTION_FILTER;

//
// Prototypes
//

WINBASEAPI
VOID
WINAPI
RaiseException(
    _In_ DWORD dwExceptionCode,
    _In_ DWORD dwExceptionFlags,
    _In_ DWORD nNumberOfArguments,
    _In_reads_opt_(nNumberOfArguments) CONST ULONG_PTR* lpArguments
    );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Desktop Family or OneCore Family or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

__callback
WINBASEAPI
LONG
WINAPI
UnhandledExceptionFilter(
    _In_ struct _EXCEPTION_POINTERS* ExceptionInfo
    );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Application Family or OneCore Family or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

WINBASEAPI
LPTOP_LEVEL_EXCEPTION_FILTER
WINAPI
SetUnhandledExceptionFilter(
    _In_opt_ LPTOP_LEVEL_EXCEPTION_FILTER lpTopLevelExceptionFilter
    );

#ifndef _M_CEE_PURE

WINBASEAPI
_Check_return_
_Post_equals_last_error_
DWORD
WINAPI
GetLastError(
    VOID
    );

#endif

WINBASEAPI
VOID
WINAPI
SetLastError(
    _In_ DWORD dwErrCode
    );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Application Family or OneCore Family or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

#if (_WIN32_WINNT >= 0x0600)

WINBASEAPI
UINT
WINAPI
GetErrorMode(
    VOID
    );

#endif // (_WIN32_WINNT >= 0x0600)

WINBASEAPI
UINT
WINAPI
SetErrorMode(
    _In_ UINT uMode
    );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Desktop Family or OneCore Family or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

#if (_WIN32_WINNT >= 0x0501)

WINBASEAPI
_Ret_maybenull_
PVOID
WINAPI
AddVectoredExceptionHandler(
    _In_ ULONG First,
    _In_ PVECTORED_EXCEPTION_HANDLER Handler
    );

WINBASEAPI
ULONG
WINAPI
RemoveVectoredExceptionHandler(
    _In_ PVOID Handle
    );

WINBASEAPI
_Ret_maybenull_
PVOID
WINAPI
AddVectoredContinueHandler(
    _In_ ULONG First,
    _In_ PVECTORED_EXCEPTION_HANDLER Handler
    );

WINBASEAPI
ULONG
WINAPI
RemoveVectoredContinueHandler(
    _In_ PVOID Handle
    );

#endif // (_WIN32_WINNT >= 0x0501)

// RC warns because "WINBASE_DECLARE_RESTORE_LAST_ERROR" is a bit long.
#if !defined(RC_INVOKED)
//#if _WIN32_WINNT >= 0x0501 || defined(WINBASE_DECLARE_RESTORE_LAST_ERROR)
#if defined(WINBASE_DECLARE_RESTORE_LAST_ERROR)

WINBASEAPI
VOID
WINAPI
RestoreLastError(
    _In_ DWORD dwErrCode
    );

typedef VOID (WINAPI* PRESTORE_LAST_ERROR)(DWORD);
#define RESTORE_LAST_ERROR_NAME_A      "RestoreLastError"
#define RESTORE_LAST_ERROR_NAME_W     L"RestoreLastError"
#define RESTORE_LAST_ERROR_NAME   TEXT("RestoreLastError")

#endif
#endif

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Application Family or OneCore Family or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

WINBASEAPI
VOID
WINAPI
RaiseFailFastException(
    _In_opt_ PEXCEPTION_RECORD pExceptionRecord,
    _In_opt_ PCONTEXT pContextRecord,
    _In_ DWORD dwFlags
    );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Desktop Family or OneCore Family or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

WINBASEAPI
VOID
WINAPI
FatalAppExitA(
    _In_ UINT uAction,
    _In_ LPCSTR lpMessageText
    );

WINBASEAPI
VOID
WINAPI
FatalAppExitW(
    _In_ UINT uAction,
    _In_ LPCWSTR lpMessageText
    );

#ifdef UNICODE
#define FatalAppExit  FatalAppExitW
#else
#define FatalAppExit  FatalAppExitA
#endif // !UNICODE

#endif // WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)
#pragma endregion

#pragma region Application Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)

WINBASEAPI
DWORD
WINAPI
GetThreadErrorMode(
    VOID
    );

WINBASEAPI
BOOL
WINAPI
SetThreadErrorMode(
    _In_ DWORD dwNewMode,
    _In_opt_ LPDWORD lpOldMode
    );

#endif // WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)
#pragma endregion

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

WINBASEAPI
VOID
WINAPI
TerminateProcessOnMemoryExhaustion(
    _In_ SIZE_T FailedAllocationSize
    );

#endif // WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)
#pragma endregion

#ifdef __cplusplus
}
#endif

#endif // _ERRHANDLING_H_
