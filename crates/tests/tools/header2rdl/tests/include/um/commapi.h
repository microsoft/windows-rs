/********************************************************************************
*                                                                               *
* commapi.h - ApiSet Contract for api-ms-win-core-comm-l1                       *
*                                                                               *
* Copyright (c) Microsoft Corporation. All rights reserved.                     *
*                                                                               *
********************************************************************************/

#ifdef _MSC_VER
#pragma once
#endif // _MSC_VER

#ifndef _COMM_H_
#define _COMM_H_

#include <apiset.h>
#include <apisetcconv.h>
#include <windows.h>
#include <winbase.h>
#include <winapifamily.h>

#ifdef __cplusplus
extern "C" {
#endif

#pragma region Desktop Family or OneCore or App Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_APP)

WINBASEAPI
BOOL
WINAPI
ClearCommBreak(
    _In_ HANDLE hFile
    );

WINBASEAPI
BOOL
WINAPI
ClearCommError(
    _In_ HANDLE hFile,
    _Out_opt_ LPDWORD lpErrors,
    _Out_opt_ LPCOMSTAT lpStat
    );

WINBASEAPI
BOOL
WINAPI
SetupComm(
    _In_ HANDLE hFile,
    _In_ DWORD dwInQueue,
    _In_ DWORD dwOutQueue
    );

WINBASEAPI
BOOL
WINAPI
EscapeCommFunction(
    _In_ HANDLE hFile,
    _In_ DWORD dwFunc
    );

WINBASEAPI
_Success_(return != FALSE)
BOOL
WINAPI
GetCommConfig(
    _In_ HANDLE hCommDev,
    _Out_writes_bytes_opt_(*lpdwSize) LPCOMMCONFIG lpCC,
    _Inout_ LPDWORD lpdwSize
    );

WINBASEAPI
BOOL
WINAPI
GetCommMask(
    _In_ HANDLE hFile,
    _Out_ LPDWORD lpEvtMask
    );

WINBASEAPI
BOOL
WINAPI
GetCommModemStatus(
    _In_ HANDLE hFile,
    _Out_ LPDWORD lpModemStat
    );

WINBASEAPI
BOOL
WINAPI
GetCommProperties(
    _In_ HANDLE hFile,
    _Inout_ LPCOMMPROP lpCommProp
    );

WINBASEAPI
BOOL
WINAPI
GetCommState(
    _In_ HANDLE hFile,
    _Out_ LPDCB lpDCB
    );

WINBASEAPI
BOOL
WINAPI
GetCommTimeouts(
    _In_ HANDLE hFile,
    _Out_ LPCOMMTIMEOUTS lpCommTimeouts
    );

WINBASEAPI
BOOL
WINAPI
PurgeComm(
    _In_ HANDLE hFile,
    _In_ DWORD dwFlags
    );

WINBASEAPI
BOOL
WINAPI
SetCommBreak(
    _In_ HANDLE hFile
    );

WINBASEAPI
BOOL
WINAPI
SetCommConfig(
    _In_ HANDLE hCommDev,
    _In_reads_bytes_(dwSize) LPCOMMCONFIG lpCC,
    _In_ DWORD dwSize
    );

WINBASEAPI
BOOL
WINAPI
SetCommMask(
    _In_ HANDLE hFile,
    _In_ DWORD dwEvtMask
    );

WINBASEAPI
BOOL
WINAPI
SetCommState(
    _In_ HANDLE hFile,
    _In_ LPDCB lpDCB
    );

WINBASEAPI
BOOL
WINAPI
SetCommTimeouts(
    _In_ HANDLE hFile,
    _In_ LPCOMMTIMEOUTS lpCommTimeouts
    );

WINBASEAPI
BOOL
WINAPI
TransmitCommChar(
    _In_ HANDLE hFile,
    _In_ char cChar
    );

WINBASEAPI
BOOL
WINAPI
WaitCommEvent(
    _In_ HANDLE hFile,
    _Inout_ LPDWORD lpEvtMask,
    _Inout_opt_ LPOVERLAPPED lpOverlapped
    );

WINBASEAPI
HANDLE
WINAPI
OpenCommPort(
    _In_ ULONG uPortNumber,
    _In_ DWORD dwDesiredAccess,
    _In_ DWORD dwFlagsAndAttributes
    );

WINBASEAPI
ULONG
WINAPI
GetCommPorts(
    _Out_writes_(uPortNumbersCount) PULONG lpPortNumbers,
    _In_ ULONG uPortNumbersCount,
    _Out_ PULONG puPortNumbersFound
    );

#endif // WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_APP)
#pragma endregion

#ifdef __cplusplus
}
#endif

#endif // _COMM_H_

