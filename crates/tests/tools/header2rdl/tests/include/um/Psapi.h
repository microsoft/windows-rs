/*++ BUILD Version: 0001    // Increment this if a change has global effects

Copyright (c) 1994-1999  Microsoft Corporation

Module Name:

    psapi.h

Abstract:

    Include file for APIs provided by PSAPI.DLL

Author:

    Richard Shupak   [richards]  06-Jan-1994

Revision History:

--*/

#ifndef _PSAPI_H_
#define _PSAPI_H_

#if _MSC_VER > 1000
#pragma once
#endif
#include <winapifamily.h>

#ifdef __cplusplus
extern "C" {
#endif

#pragma region Desktop Family or OneCore Family or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

#if _MSC_VER >= 1200
#pragma warning(push)
#pragma warning(disable:4820) /* padding added after data member */
#endif

#define LIST_MODULES_DEFAULT 0x0  // This is the default one app would get without any flag.
#define LIST_MODULES_32BIT   0x01  // list 32bit modules in the target process.
#define LIST_MODULES_64BIT   0x02  // list all 64bit modules. 32bit exe will be stripped off.

// list all the modules
#define LIST_MODULES_ALL   (LIST_MODULES_32BIT | LIST_MODULES_64BIT)

//
// Give teams a choice of using a downlevel version of psapi.h for an OS versions.
// Teams can set C_DEFINES=$(C_DEFINES) -DPSAPI_VERSION=1 for downlevel psapi
// on windows 7 and higher.  We found that test code needs this capability.
//
#ifndef PSAPI_VERSION
#if (NTDDI_VERSION >= NTDDI_WIN7)
#define PSAPI_VERSION 2
#else
#define PSAPI_VERSION 1
#endif
#endif

#if (PSAPI_VERSION > 1)
#define EnumProcessModules          K32EnumProcessModules
#define EnumProcessModulesEx        K32EnumProcessModulesEx
#define InitializeProcessForWsWatch K32InitializeProcessForWsWatch
#define GetWsChanges                K32GetWsChanges
#define GetWsChangesEx              K32GetWsChangesEx
#define GetMappedFileNameW          K32GetMappedFileNameW
#define GetMappedFileNameA          K32GetMappedFileNameA
#define EnumDeviceDrivers           K32EnumDeviceDrivers
#define GetDeviceDriverBaseNameA    K32GetDeviceDriverBaseNameA
#define GetDeviceDriverBaseNameW    K32GetDeviceDriverBaseNameW
#define GetDeviceDriverFileNameA    K32GetDeviceDriverFileNameA
#define GetDeviceDriverFileNameW    K32GetDeviceDriverFileNameW
#define GetPerformanceInfo          K32GetPerformanceInfo
#define GetProcessImageFileNameA    K32GetProcessImageFileNameA
#define GetProcessImageFileNameW    K32GetProcessImageFileNameW
#endif

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

#ifndef PSAPI_VERSION
#if (NTDDI_VERSION >= NTDDI_WIN7)
#define PSAPI_VERSION 2
#else
#define PSAPI_VERSION 1
#endif
#endif

#if (PSAPI_VERSION > 1)
#define GetModuleBaseNameA          K32GetModuleBaseNameA
#define GetModuleBaseNameW          K32GetModuleBaseNameW
#define GetModuleFileNameExA        K32GetModuleFileNameExA
#define GetModuleFileNameExW        K32GetModuleFileNameExW
#define EmptyWorkingSet             K32EmptyWorkingSet
#define EnumPageFilesW              K32EnumPageFilesW
#define EnumPageFilesA              K32EnumPageFilesA
#endif

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#pragma region Application Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

#ifndef PSAPI_VERSION
#if (NTDDI_VERSION >= NTDDI_WIN7)
#define PSAPI_VERSION 2
#else
#define PSAPI_VERSION 1
#endif
#endif

#if (PSAPI_VERSION > 1)

#define EnumProcesses               K32EnumProcesses
#define GetProcessMemoryInfo        K32GetProcessMemoryInfo
#define GetModuleInformation        K32GetModuleInformation
#define GetModuleBaseNameA          K32GetModuleBaseNameA
#define GetModuleBaseNameW          K32GetModuleBaseNameW
#define GetModuleFileNameExA        K32GetModuleFileNameExA
#define GetModuleFileNameExW        K32GetModuleFileNameExW
#define QueryWorkingSet             K32QueryWorkingSet
#define QueryWorkingSetEx           K32QueryWorkingSetEx

#endif

BOOL
WINAPI
EnumProcesses(
    _Out_writes_bytes_(cb) DWORD* lpidProcess,
    _In_ DWORD cb,
    _Out_ LPDWORD lpcbNeeded
    );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Desktop Family or OneCore Family or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

BOOL
WINAPI
EnumProcessModules(
    _In_ HANDLE hProcess,
    _Out_writes_bytes_(cb) HMODULE* lphModule,
    _In_ DWORD cb,
    _Out_ LPDWORD lpcbNeeded
    );

BOOL
WINAPI
EnumProcessModulesEx(
    _In_ HANDLE hProcess,
    _Out_writes_bytes_(cb) HMODULE* lphModule,
    _In_ DWORD cb,
    _Out_ LPDWORD lpcbNeeded,
    _In_ DWORD dwFilterFlag
    );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Application Family or OneCore Family or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

DWORD
WINAPI
GetModuleBaseNameA(
    _In_ HANDLE hProcess,
    _In_opt_ HMODULE hModule,
    _Out_writes_(nSize) LPSTR lpBaseName,
    _In_ DWORD nSize
    );

DWORD
WINAPI
GetModuleBaseNameW(
    _In_ HANDLE hProcess,
    _In_opt_ HMODULE hModule,
    _Out_writes_(nSize) LPWSTR lpBaseName,
    _In_ DWORD nSize
    );

#ifdef UNICODE
#define GetModuleBaseName  GetModuleBaseNameW
#else
#define GetModuleBaseName  GetModuleBaseNameA
#endif // !UNICODE

_Success_(return != 0)
_Ret_range_(1, nSize)
DWORD
WINAPI
GetModuleFileNameExA(
    _In_opt_ HANDLE hProcess,
    _In_opt_ HMODULE hModule,
    _When_(return < nSize, _Out_writes_to_(nSize, return + 1))
    _When_(return == nSize, _Out_writes_all_(nSize))
         LPSTR lpFilename,
    _In_ DWORD nSize
    );

_Success_(return != 0)
_Ret_range_(1, nSize)
DWORD
WINAPI
GetModuleFileNameExW(
    _In_opt_ HANDLE hProcess,
    _In_opt_ HMODULE hModule,
    _When_(return < nSize, _Out_writes_to_(nSize, return + 1))
    _When_(return == nSize, _Out_writes_all_(nSize))
         LPWSTR lpFilename,
    _In_ DWORD nSize
    );

#ifdef UNICODE
#define GetModuleFileNameEx  GetModuleFileNameExW
#else
#define GetModuleFileNameEx  GetModuleFileNameExA
#endif // !UNICODE

typedef struct _MODULEINFO {
    LPVOID lpBaseOfDll;
    DWORD SizeOfImage;
    LPVOID EntryPoint;
} MODULEINFO, *LPMODULEINFO;

BOOL
WINAPI
GetModuleInformation(
    _In_ HANDLE hProcess,
    _In_ HMODULE hModule,
    _Out_ LPMODULEINFO lpmodinfo,
    _In_ DWORD cb
    );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

BOOL
WINAPI
EmptyWorkingSet(
    _In_ HANDLE hProcess
    );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#pragma region Desktop Family or OneCore Family or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

BOOL
WINAPI
InitializeProcessForWsWatch(
    _In_ HANDLE hProcess
    );

typedef struct _PSAPI_WS_WATCH_INFORMATION {
    LPVOID FaultingPc;
    LPVOID FaultingVa;
} PSAPI_WS_WATCH_INFORMATION, *PPSAPI_WS_WATCH_INFORMATION;

typedef struct _PSAPI_WS_WATCH_INFORMATION_EX {
    PSAPI_WS_WATCH_INFORMATION BasicInfo;
    ULONG_PTR FaultingThreadId;
    ULONG_PTR Flags;    // Reserved
} PSAPI_WS_WATCH_INFORMATION_EX, *PPSAPI_WS_WATCH_INFORMATION_EX;

BOOL
WINAPI
GetWsChanges(
    _In_ HANDLE hProcess,
    _Out_writes_bytes_(cb) PPSAPI_WS_WATCH_INFORMATION lpWatchInfo,
    _In_ DWORD cb
    );

BOOL
WINAPI
GetWsChangesEx(
    _In_ HANDLE hProcess,
    _Out_writes_bytes_to_(*cb, *cb) PPSAPI_WS_WATCH_INFORMATION_EX lpWatchInfoEx,
    _Inout_ PDWORD cb
    );

DWORD
WINAPI
GetMappedFileNameW (
    _In_ HANDLE hProcess,
    _In_ LPVOID lpv,
    _Out_writes_(nSize) LPWSTR lpFilename,
    _In_ DWORD nSize
    );

DWORD
WINAPI
GetMappedFileNameA (
    _In_ HANDLE hProcess,
    _In_ LPVOID lpv,
    _Out_writes_(nSize) LPSTR lpFilename,
    _In_ DWORD nSize
    );

#ifdef UNICODE
#define GetMappedFileName  GetMappedFileNameW
#else
#define GetMappedFileName  GetMappedFileNameA
#endif // !UNICODE

BOOL
WINAPI
EnumDeviceDrivers (
    _Out_writes_bytes_(cb) LPVOID *lpImageBase,
    _In_ DWORD cb,
    _Out_ LPDWORD lpcbNeeded
    );

DWORD
WINAPI
GetDeviceDriverBaseNameA (
    _In_ LPVOID ImageBase,
    _Out_writes_(nSize) LPSTR lpFilename,
    _In_ DWORD nSize
    );

DWORD
WINAPI
GetDeviceDriverBaseNameW (
    _In_ LPVOID ImageBase,
    _Out_writes_(nSize) LPWSTR lpBaseName,
    _In_ DWORD nSize
    );

#ifdef UNICODE
#define GetDeviceDriverBaseName  GetDeviceDriverBaseNameW
#else
#define GetDeviceDriverBaseName  GetDeviceDriverBaseNameA
#endif // !UNICODE

DWORD
WINAPI
GetDeviceDriverFileNameA (
    _In_ LPVOID ImageBase,
    _Out_writes_(nSize) LPSTR lpFilename,
    _In_ DWORD nSize
    );

DWORD
WINAPI
GetDeviceDriverFileNameW (
    _In_ LPVOID ImageBase,
    _Out_writes_(nSize) LPWSTR lpFilename,
    _In_ DWORD nSize
    );

#ifdef UNICODE
#define GetDeviceDriverFileName  GetDeviceDriverFileNameW
#else
#define GetDeviceDriverFileName  GetDeviceDriverFileNameA
#endif // !UNICODE

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Application Family or OneCore Family or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

//
// Working set information structures. All non-specified bits are reserved.
//

#if _MSC_VER >= 1200
#pragma warning(push)
#endif
#pragma warning(disable:4201)   // unnamed struct
#pragma warning(disable:4214)   // bit fields other than int

typedef union _PSAPI_WORKING_SET_BLOCK {
    ULONG_PTR Flags;
    struct {
        ULONG_PTR Protection : 5;
        ULONG_PTR ShareCount : 3;
        ULONG_PTR Shared : 1;
        ULONG_PTR Reserved : 3;
#if defined(_WIN64)
        ULONG_PTR VirtualPage : 52;
#else
        ULONG_PTR VirtualPage : 20;
#endif
    };
} PSAPI_WORKING_SET_BLOCK, *PPSAPI_WORKING_SET_BLOCK;

typedef struct _PSAPI_WORKING_SET_INFORMATION {
    ULONG_PTR NumberOfEntries;
    PSAPI_WORKING_SET_BLOCK WorkingSetInfo[1];
} PSAPI_WORKING_SET_INFORMATION, *PPSAPI_WORKING_SET_INFORMATION;

typedef union _PSAPI_WORKING_SET_EX_BLOCK {
    ULONG_PTR Flags;
    union {
        struct {
            ULONG_PTR Valid : 1;
            ULONG_PTR ShareCount : 3;
            ULONG_PTR Win32Protection : 11;
            ULONG_PTR Shared : 1;
            ULONG_PTR Node : 6;
            ULONG_PTR Locked : 1;
            ULONG_PTR LargePage : 1;
            ULONG_PTR Reserved : 7;
            ULONG_PTR Bad : 1;

#if defined(_WIN64)
            ULONG_PTR ReservedUlong : 32;
#endif
        };
        struct {
            ULONG_PTR Valid : 1;            // Valid = 0 in this format.
            ULONG_PTR Reserved0 : 14;
            ULONG_PTR Shared : 1;
            ULONG_PTR Reserved1 : 15;
            ULONG_PTR Bad : 1;

#if defined(_WIN64)
            ULONG_PTR ReservedUlong : 32;
#endif
        } Invalid;
    };
} PSAPI_WORKING_SET_EX_BLOCK, *PPSAPI_WORKING_SET_EX_BLOCK;

typedef struct _PSAPI_WORKING_SET_EX_INFORMATION {
    PVOID VirtualAddress;
    PSAPI_WORKING_SET_EX_BLOCK VirtualAttributes;
} PSAPI_WORKING_SET_EX_INFORMATION, *PPSAPI_WORKING_SET_EX_INFORMATION;

#if _MSC_VER >= 1200
#pragma warning(pop)
#else
#pragma warning(default:4214)
#pragma warning(default:4201)
#endif

BOOL
WINAPI
QueryWorkingSet(
    _In_ HANDLE hProcess,
    _Out_writes_bytes_(cb) PVOID pv,
    _In_ DWORD cb
    );

BOOL
WINAPI
QueryWorkingSetEx(
    _In_ HANDLE hProcess,
    _Out_writes_bytes_(cb) PVOID pv,
    _In_ DWORD cb
    );

// Structure for GetProcessMemoryInfo()

typedef struct _PROCESS_MEMORY_COUNTERS {
    DWORD cb;
    DWORD PageFaultCount;
    SIZE_T PeakWorkingSetSize;
    SIZE_T WorkingSetSize;
    SIZE_T QuotaPeakPagedPoolUsage;
    SIZE_T QuotaPagedPoolUsage;
    SIZE_T QuotaPeakNonPagedPoolUsage;
    SIZE_T QuotaNonPagedPoolUsage;
    SIZE_T PagefileUsage;
    SIZE_T PeakPagefileUsage;
} PROCESS_MEMORY_COUNTERS;
typedef PROCESS_MEMORY_COUNTERS *PPROCESS_MEMORY_COUNTERS;

#if (_WIN32_WINNT >= 0x0501)

typedef struct _PROCESS_MEMORY_COUNTERS_EX {
    DWORD cb;
    DWORD PageFaultCount;
    SIZE_T PeakWorkingSetSize;
    SIZE_T WorkingSetSize;
    SIZE_T QuotaPeakPagedPoolUsage;
    SIZE_T QuotaPagedPoolUsage;
    SIZE_T QuotaPeakNonPagedPoolUsage;
    SIZE_T QuotaNonPagedPoolUsage;
    SIZE_T PagefileUsage;
    SIZE_T PeakPagefileUsage;
    SIZE_T PrivateUsage;
} PROCESS_MEMORY_COUNTERS_EX;
typedef PROCESS_MEMORY_COUNTERS_EX *PPROCESS_MEMORY_COUNTERS_EX;

#endif

#if (NTDDI_VERSION >= NTDDI_WIN10_CU)

typedef struct _PROCESS_MEMORY_COUNTERS_EX2 {
    DWORD cb;
    DWORD PageFaultCount;
    SIZE_T PeakWorkingSetSize;
    SIZE_T WorkingSetSize;
    SIZE_T QuotaPeakPagedPoolUsage;
    SIZE_T QuotaPagedPoolUsage;
    SIZE_T QuotaPeakNonPagedPoolUsage;
    SIZE_T QuotaNonPagedPoolUsage;
    SIZE_T PagefileUsage;
    SIZE_T PeakPagefileUsage;
    SIZE_T PrivateUsage;
    SIZE_T PrivateWorkingSetSize;
    ULONG64 SharedCommitUsage;
} PROCESS_MEMORY_COUNTERS_EX2;
typedef PROCESS_MEMORY_COUNTERS_EX2 *PPROCESS_MEMORY_COUNTERS_EX2;

#endif

BOOL
WINAPI
GetProcessMemoryInfo (
    _In_ HANDLE Process,
    _Out_writes_bytes_(cb) PPROCESS_MEMORY_COUNTERS ppsmemCounters,
    _In_ DWORD cb
    );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Desktop Family or OneCore Family or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

typedef struct _PERFORMANCE_INFORMATION {
    DWORD cb;
    SIZE_T CommitTotal;
    SIZE_T CommitLimit;
    SIZE_T CommitPeak;
    SIZE_T PhysicalTotal;
    SIZE_T PhysicalAvailable;
    SIZE_T SystemCache;
    SIZE_T KernelTotal;
    SIZE_T KernelPaged;
    SIZE_T KernelNonpaged;
    SIZE_T PageSize;
    DWORD HandleCount;
    DWORD ProcessCount;
    DWORD ThreadCount;
} PERFORMANCE_INFORMATION, *PPERFORMANCE_INFORMATION, PERFORMACE_INFORMATION, *PPERFORMACE_INFORMATION;

BOOL
WINAPI
GetPerformanceInfo (
    PPERFORMANCE_INFORMATION pPerformanceInformation,
    DWORD cb
    );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

typedef struct _ENUM_PAGE_FILE_INFORMATION {
    DWORD cb;
    DWORD Reserved;
    SIZE_T TotalSize;
    SIZE_T TotalInUse;
    SIZE_T PeakUsage;
} ENUM_PAGE_FILE_INFORMATION, *PENUM_PAGE_FILE_INFORMATION;

typedef BOOL (__stdcall *PENUM_PAGE_FILE_CALLBACKW) (LPVOID pContext, PENUM_PAGE_FILE_INFORMATION pPageFileInfo, LPCWSTR lpFilename);

typedef BOOL (__stdcall *PENUM_PAGE_FILE_CALLBACKA) (LPVOID pContext, PENUM_PAGE_FILE_INFORMATION pPageFileInfo, LPCSTR lpFilename);

BOOL
WINAPI
EnumPageFilesW (
    PENUM_PAGE_FILE_CALLBACKW pCallBackRoutine,
    LPVOID pContext
    );

BOOL
WINAPI
EnumPageFilesA (
    PENUM_PAGE_FILE_CALLBACKA pCallBackRoutine,
    LPVOID pContext
    );

#ifdef UNICODE
#define PENUM_PAGE_FILE_CALLBACK PENUM_PAGE_FILE_CALLBACKW
#define EnumPageFiles EnumPageFilesW
#else
#define PENUM_PAGE_FILE_CALLBACK PENUM_PAGE_FILE_CALLBACKA
#define EnumPageFiles EnumPageFilesA
#endif // !UNICODE

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#pragma region Desktop Family or OneCore Family or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

DWORD
WINAPI
GetProcessImageFileNameA (
    _In_ HANDLE hProcess,
    _Out_writes_(nSize) LPSTR lpImageFileName,
    _In_ DWORD nSize
    );

DWORD
WINAPI
GetProcessImageFileNameW (
    _In_ HANDLE hProcess,
    _Out_writes_(nSize) LPWSTR lpImageFileName,
    _In_ DWORD nSize
    );

#ifdef UNICODE
#define GetProcessImageFileName  GetProcessImageFileNameW
#else
#define GetProcessImageFileName  GetProcessImageFileNameA
#endif // !UNICODE

#if _MSC_VER >= 1200
#pragma warning(pop)
#endif

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

#ifdef __cplusplus
}
#endif

#endif /* _PSAPI_H_ */
