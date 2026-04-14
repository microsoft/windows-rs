/********************************************************************************
*                                                                               *
* memoryapi.h -- ApiSet Contract for api-ms-win-core-memory-l1-1-0              *
*                                                                               *
* Copyright (c) Microsoft Corporation. All rights reserved.                     *
*                                                                               *
********************************************************************************/

#ifdef _MSC_VER
#pragma once
#endif // _MSC_VER

#ifndef _MEMORYAPI_H_
#define _MEMORYAPI_H_

#include <apiset.h>
#include <apisetcconv.h>
#include <minwindef.h>
#include <minwinbase.h>

#ifdef __cplusplus
extern "C" {
#endif

#if _MSC_VER >= 1200
#pragma warning(push)
#pragma warning(disable:4668) // #if not_defined treated as #if 0
#endif

#pragma region Application Family or OneCore Family or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

#define FILE_MAP_WRITE            SECTION_MAP_WRITE
#define FILE_MAP_READ             SECTION_MAP_READ
#define FILE_MAP_ALL_ACCESS       SECTION_ALL_ACCESS

#define FILE_MAP_EXECUTE          SECTION_MAP_EXECUTE_EXPLICIT  // not included in FILE_MAP_ALL_ACCESS

#define FILE_MAP_COPY             0x00000001

#define FILE_MAP_RESERVE          0x80000000
#define FILE_MAP_TARGETS_INVALID  0x40000000
#define FILE_MAP_LARGE_PAGES      0x20000000

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Desktop Family or OneCore Family or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

WINBASEAPI
_Ret_maybenull_
_Post_writable_byte_size_(dwSize)
LPVOID
WINAPI
VirtualAlloc(
    _In_opt_ LPVOID lpAddress,
    _In_ SIZE_T dwSize,
    _In_ DWORD flAllocationType,
    _In_ DWORD flProtect
    );

WINBASEAPI
_Success_(return != FALSE)
BOOL
WINAPI
VirtualProtect(
    _In_ LPVOID lpAddress,
    _In_ SIZE_T dwSize,
    _In_ DWORD flNewProtect,
    _Out_ PDWORD lpflOldProtect
    );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Application Family or OneCore Family or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

_When_(((dwFreeType&(MEM_RELEASE|MEM_DECOMMIT)))==(MEM_RELEASE|MEM_DECOMMIT),
    __drv_reportError("Passing both MEM_RELEASE and MEM_DECOMMIT to VirtualFree is not allowed. This results in the failure of this call"))

_When_(dwFreeType==0,
    __drv_reportError("Passing zero as the dwFreeType parameter to VirtualFree is not allowed. This results in the failure of this call"))

_When_(((dwFreeType&MEM_RELEASE))!=0 && dwSize!=0,
    __drv_reportError("Passing MEM_RELEASE and a non-zero dwSize parameter to VirtualFree is not allowed. This results in the failure of this call"))
_Success_(return != FALSE)
WINBASEAPI
BOOL
WINAPI
VirtualFree(
    _Pre_notnull_ _When_(dwFreeType == MEM_DECOMMIT,_Post_invalid_) _When_(dwFreeType == MEM_RELEASE,_Post_ptr_invalid_) LPVOID lpAddress,
    _In_ SIZE_T dwSize,
    _In_ DWORD dwFreeType
    );

WINBASEAPI
SIZE_T
WINAPI
VirtualQuery(
    _In_opt_ LPCVOID lpAddress,
    _Out_writes_bytes_to_(dwLength,return) PMEMORY_BASIC_INFORMATION lpBuffer,
    _In_ SIZE_T dwLength
    );

WINBASEAPI
_Ret_maybenull_
_Post_writable_byte_size_(dwSize)
LPVOID
WINAPI
VirtualAllocEx(
    _In_ HANDLE hProcess,
    _In_opt_ LPVOID lpAddress,
    _In_ SIZE_T dwSize,
    _In_ DWORD flAllocationType,
    _In_ DWORD flProtect
    );

WINBASEAPI
_Success_(return != FALSE)
BOOL
WINAPI
VirtualProtectEx(
    _In_ HANDLE hProcess,
    _In_ LPVOID lpAddress,
    _In_ SIZE_T dwSize,
    _In_ DWORD flNewProtect,
    _Out_ PDWORD lpflOldProtect
    );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Desktop Family or OneCore Family or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

WINBASEAPI
SIZE_T
WINAPI
VirtualQueryEx(
    _In_ HANDLE hProcess,
    _In_opt_ LPCVOID lpAddress,
    _Out_writes_bytes_to_(dwLength,return) PMEMORY_BASIC_INFORMATION lpBuffer,
    _In_ SIZE_T dwLength
    );

WINBASEAPI
_Success_(return != FALSE)
BOOL
WINAPI
ReadProcessMemory(
    _In_ HANDLE hProcess,
    _In_ LPCVOID lpBaseAddress,
    _Out_writes_bytes_to_(nSize,*lpNumberOfBytesRead) LPVOID lpBuffer,
    _In_ SIZE_T nSize,
    _Out_opt_ SIZE_T* lpNumberOfBytesRead
    );

WINBASEAPI
_Success_(return != FALSE)
BOOL
WINAPI
WriteProcessMemory(
    _In_ HANDLE hProcess,
    _In_ LPVOID lpBaseAddress,
    _In_reads_bytes_(nSize) LPCVOID lpBuffer,
    _In_ SIZE_T nSize,
    _Out_opt_ SIZE_T* lpNumberOfBytesWritten
    );

WINBASEAPI
_Ret_maybenull_
HANDLE
WINAPI
CreateFileMappingW(
    _In_ HANDLE hFile,
    _In_opt_ LPSECURITY_ATTRIBUTES lpFileMappingAttributes,
    _In_ DWORD flProtect,
    _In_ DWORD dwMaximumSizeHigh,
    _In_ DWORD dwMaximumSizeLow,
    _In_opt_ LPCWSTR lpName
    );

#ifdef UNICODE
#define CreateFileMapping  CreateFileMappingW
#endif

WINBASEAPI
_Ret_maybenull_
HANDLE
WINAPI
OpenFileMappingW(
    _In_ DWORD dwDesiredAccess,
    _In_ BOOL bInheritHandle,
    _In_ LPCWSTR lpName
    );

#ifdef UNICODE
#define OpenFileMapping  OpenFileMappingW
#endif

WINBASEAPI
_Ret_maybenull_  __out_data_source(FILE)
LPVOID
WINAPI
MapViewOfFile(
    _In_ HANDLE hFileMappingObject,
    _In_ DWORD dwDesiredAccess,
    _In_ DWORD dwFileOffsetHigh,
    _In_ DWORD dwFileOffsetLow,
    _In_ SIZE_T dwNumberOfBytesToMap
    );

WINBASEAPI
_Ret_maybenull_  __out_data_source(FILE)
LPVOID
WINAPI
MapViewOfFileEx(
    _In_ HANDLE hFileMappingObject,
    _In_ DWORD dwDesiredAccess,
    _In_ DWORD dwFileOffsetHigh,
    _In_ DWORD dwFileOffsetLow,
    _In_ SIZE_T dwNumberOfBytesToMap,
    _In_opt_ LPVOID lpBaseAddress
    );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Application Family or OneCore Family or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

_When_(((dwFreeType&(MEM_RELEASE|MEM_DECOMMIT)))==(MEM_RELEASE|MEM_DECOMMIT),
    __drv_reportError("Passing both MEM_RELEASE and MEM_DECOMMIT to VirtualFree is not allowed. This results in the failure of this call"))

_When_(dwFreeType==0,
    __drv_reportError("Passing zero as the dwFreeType parameter to VirtualFree is not allowed. This results in the failure of this call"))

_When_(((dwFreeType&MEM_RELEASE))!=0 && dwSize!=0,
    __drv_reportError("Passing MEM_RELEASE and a non-zero dwSize parameter to VirtualFree is not allowed. This results in the failure of this call"))

_When_(((dwFreeType&MEM_DECOMMIT))!=0,
    __drv_reportError("Calling VirtualFreeEx without the MEM_RELEASE flag frees memory but not address descriptors (VADs); results in address space leaks"))
_Success_(return != FALSE)
WINBASEAPI
BOOL
WINAPI
VirtualFreeEx(
    _In_ HANDLE hProcess,
    _Pre_notnull_ _When_(dwFreeType == MEM_DECOMMIT,_Post_invalid_) _When_(dwFreeType == MEM_RELEASE,_Post_ptr_invalid_) LPVOID lpAddress,
    _In_ SIZE_T dwSize,
    _In_ DWORD dwFreeType
    );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Application Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)

WINBASEAPI
BOOL
WINAPI
FlushViewOfFile(
    _In_ LPCVOID lpBaseAddress,
    _In_ SIZE_T dwNumberOfBytesToFlush
    );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#pragma region Application Family or OneCore Family or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

WINBASEAPI
BOOL
WINAPI
UnmapViewOfFile(
    _In_ LPCVOID lpBaseAddress
    );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Application Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)

WINBASEAPI
SIZE_T
WINAPI
GetLargePageMinimum(
    VOID
    );

WINBASEAPI
BOOL
WINAPI
GetProcessWorkingSetSize(
    _In_ HANDLE hProcess,
    _Out_ PSIZE_T lpMinimumWorkingSetSize,
    _Out_ PSIZE_T lpMaximumWorkingSetSize
    );

WINBASEAPI
_Success_(return != FALSE)
BOOL
WINAPI
GetProcessWorkingSetSizeEx(
    _In_ HANDLE hProcess,
    _Out_ PSIZE_T lpMinimumWorkingSetSize,
    _Out_ PSIZE_T lpMaximumWorkingSetSize,
    _Out_ PDWORD Flags
    );

WINBASEAPI
BOOL
WINAPI
SetProcessWorkingSetSize(
    _In_ HANDLE hProcess,
    _In_ SIZE_T dwMinimumWorkingSetSize,
    _In_ SIZE_T dwMaximumWorkingSetSize
    );

WINBASEAPI
BOOL
WINAPI
SetProcessWorkingSetSizeEx(
    _In_ HANDLE hProcess,
    _In_ SIZE_T dwMinimumWorkingSetSize,
    _In_ SIZE_T dwMaximumWorkingSetSize,
    _In_ DWORD Flags
    );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

WINBASEAPI
BOOL
WINAPI
VirtualLock(
    _In_ LPVOID lpAddress,
    _In_ SIZE_T dwSize
    );

WINBASEAPI
BOOL
WINAPI
VirtualUnlock(
    _In_ LPVOID lpAddress,
    _In_ SIZE_T dwSize
    );

#endif // WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)
#pragma endregion

#pragma region Application Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)

WINBASEAPI
_Success_(return == 0)
UINT
WINAPI
GetWriteWatch(
    _In_ DWORD dwFlags,
    _In_ PVOID lpBaseAddress,
    _In_ SIZE_T dwRegionSize,
    _Out_writes_to_opt_(*lpdwCount,*lpdwCount) PVOID* lpAddresses,
    _Inout_opt_ ULONG_PTR* lpdwCount,
    _Out_opt_ LPDWORD lpdwGranularity
    );

WINBASEAPI
UINT
WINAPI
ResetWriteWatch(
    _In_ LPVOID lpBaseAddress,
    _In_ SIZE_T dwRegionSize
    );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

#if (_WIN32_WINNT >= _WIN32_WINNT_WINXP)

typedef enum _MEMORY_RESOURCE_NOTIFICATION_TYPE {
    LowMemoryResourceNotification,
    HighMemoryResourceNotification
} MEMORY_RESOURCE_NOTIFICATION_TYPE;

WINBASEAPI
_Ret_maybenull_
HANDLE
WINAPI
CreateMemoryResourceNotification(
    _In_ MEMORY_RESOURCE_NOTIFICATION_TYPE NotificationType
    );

WINBASEAPI
_Success_(return != FALSE)
BOOL
WINAPI
QueryMemoryResourceNotification(
    _In_ HANDLE ResourceNotificationHandle,
    _Out_ PBOOL ResourceState
    );

#endif // (_WIN32_WINNT >= _WIN32_WINNT_WINXP)

#endif // WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)
#pragma endregion

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

#if (_WIN32_WINNT >= _WIN32_WINNT_WS03)

#define FILE_CACHE_FLAGS_DEFINED
#define FILE_CACHE_MAX_HARD_ENABLE      0x00000001
#define FILE_CACHE_MAX_HARD_DISABLE     0x00000002
#define FILE_CACHE_MIN_HARD_ENABLE      0x00000004
#define FILE_CACHE_MIN_HARD_DISABLE     0x00000008

WINBASEAPI
_Success_(return != FALSE)
BOOL
WINAPI
GetSystemFileCacheSize(
    _Out_ PSIZE_T lpMinimumFileCacheSize,
    _Out_ PSIZE_T lpMaximumFileCacheSize,
    _Out_ PDWORD lpFlags
    );

WINBASEAPI
BOOL
WINAPI
SetSystemFileCacheSize(
    _In_ SIZE_T MinimumFileCacheSize,
    _In_ SIZE_T MaximumFileCacheSize,
    _In_ DWORD Flags
    );

#endif // (_WIN32_WINNT >= _WIN32_WINNT_WS03)

#endif // WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)
#pragma endregion

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

#if (_WIN32_WINNT >= _WIN32_WINNT_VISTA)

WINBASEAPI
_Ret_maybenull_
HANDLE
WINAPI
CreateFileMappingNumaW(
    _In_ HANDLE hFile,
    _In_opt_ LPSECURITY_ATTRIBUTES lpFileMappingAttributes,
    _In_ DWORD flProtect,
    _In_ DWORD dwMaximumSizeHigh,
    _In_ DWORD dwMaximumSizeLow,
    _In_opt_ LPCWSTR lpName,
    _In_ DWORD nndPreferred
    );

#ifdef UNICODE
#define CreateFileMappingNuma CreateFileMappingNumaW
#endif

#endif // (_WIN32_WINNT >= _WIN32_WINNT_VISTA)

#if (_WIN32_WINNT >= _WIN32_WINNT_WIN8)

typedef struct _WIN32_MEMORY_RANGE_ENTRY {
    PVOID VirtualAddress;
    SIZE_T NumberOfBytes;
} WIN32_MEMORY_RANGE_ENTRY, *PWIN32_MEMORY_RANGE_ENTRY;

WINBASEAPI
BOOL
WINAPI
PrefetchVirtualMemory(
    _In_ HANDLE hProcess,
    _In_ ULONG_PTR NumberOfEntries,
    _In_reads_(NumberOfEntries) PWIN32_MEMORY_RANGE_ENTRY VirtualAddresses,
    _In_ ULONG Flags
    );

#endif // (_WIN32_WINNT >= _WIN32_WINNT_WIN8)

#endif // WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)
#pragma endregion

#pragma region Application Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)

#if (_WIN32_WINNT >= _WIN32_WINNT_WIN8)

WINBASEAPI
_Ret_maybenull_
HANDLE
WINAPI
CreateFileMappingFromApp(
    _In_ HANDLE hFile,
    _In_opt_ PSECURITY_ATTRIBUTES SecurityAttributes,
    _In_ ULONG PageProtection,
    _In_ ULONG64 MaximumSize,
    _In_opt_ PCWSTR Name
    );

WINBASEAPI
_Ret_maybenull_  __out_data_source(FILE)
PVOID
WINAPI
MapViewOfFileFromApp(
    _In_ HANDLE hFileMappingObject,
    _In_ ULONG DesiredAccess,
    _In_ ULONG64 FileOffset,
    _In_ SIZE_T NumberOfBytesToMap
    );

WINBASEAPI
BOOL
WINAPI
UnmapViewOfFileEx(
    _In_ PVOID BaseAddress,
    _In_ ULONG UnmapFlags
    );

#endif // (_WIN32_WINNT >= _WIN32_WINNT_WIN8)

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

#if (_WIN32_WINNT >= _WIN32_WINNT_WINXP)

WINBASEAPI
_Success_(return != FALSE)
BOOL
WINAPI
AllocateUserPhysicalPages(
    _In_ HANDLE hProcess,
    _Inout_ PULONG_PTR NumberOfPages,
    _Out_writes_to_(*NumberOfPages,*NumberOfPages) PULONG_PTR PageArray
    );

WINBASEAPI
_Success_(return != FALSE)
BOOL
WINAPI
FreeUserPhysicalPages(
    _In_ HANDLE hProcess,
    _Inout_ PULONG_PTR NumberOfPages,
    _In_reads_(*NumberOfPages) PULONG_PTR PageArray
    );

WINBASEAPI
_Success_(return != FALSE)
BOOL
WINAPI
MapUserPhysicalPages(
    _In_ PVOID VirtualAddress,
    _In_ ULONG_PTR NumberOfPages,
    _In_reads_opt_(NumberOfPages) PULONG_PTR PageArray
    );

#endif // (_WIN32_WINNT >= _WIN32_WINNT_WINXP)

#if (_WIN32_WINNT >= _WIN32_WINNT_VISTA)

WINBASEAPI
_Success_(return != FALSE)
BOOL
WINAPI
AllocateUserPhysicalPagesNuma(
    _In_ HANDLE hProcess,
    _Inout_ PULONG_PTR NumberOfPages,
    _Out_writes_to_(*NumberOfPages,*NumberOfPages) PULONG_PTR PageArray,
    _In_ DWORD nndPreferred
    );

WINBASEAPI
_Post_writable_byte_size_(dwSize)
LPVOID
WINAPI
VirtualAllocExNuma(
    _In_ HANDLE hProcess,
    _In_opt_ LPVOID lpAddress,
    _In_ SIZE_T dwSize,
    _In_ DWORD flAllocationType,
    _In_ DWORD flProtect,
    _In_ DWORD nndPreferred
    );

#endif // (_WIN32_WINNT >= _WIN32_WINNT_VISTA)

#if (_WIN32_WINNT >= _WIN32_WINNT_WIN8)

#define MEHC_PATROL_SCRUBBER_PRESENT  0x1

WINBASEAPI
_Success_(return != FALSE)
BOOL
WINAPI
GetMemoryErrorHandlingCapabilities(
    _Out_ PULONG Capabilities
    );

_Function_class_(BAD_MEMORY_CALLBACK_ROUTINE)
typedef
VOID
WINAPI
BAD_MEMORY_CALLBACK_ROUTINE(
    VOID
    );

typedef BAD_MEMORY_CALLBACK_ROUTINE *PBAD_MEMORY_CALLBACK_ROUTINE;

WINBASEAPI
_Success_(return != NULL)
PVOID
WINAPI
RegisterBadMemoryNotification(
    _In_ PBAD_MEMORY_CALLBACK_ROUTINE Callback
    );

WINBASEAPI
_Success_(return != FALSE)
BOOL
WINAPI
UnregisterBadMemoryNotification(
    _In_ PVOID RegistrationHandle
    );

#endif // (_WIN32_WINNT >= _WIN32_WINNT_WIN8)

#endif // WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)
#pragma endregion

#pragma region Application Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)

// This API is not actually available in all blue builds since it is part
// of the S14 GDR release, however because there is no new version for GDR
// this is the most accurate version available.  To safely use this API on
// BLUE builds callers will need to use LoadLibrary and GetProcAddress to
// check for the existance of the API's before calling them.

#if (_WIN32_WINNT >= _WIN32_WINNT_WINBLUE)

#if ((NTDDI_VERSION > NTDDI_WINBLUE) || (NTDDI_VERSION == NTDDI_WINBLUE && defined(WINBLUE_KBSPRING14)))

typedef enum OFFER_PRIORITY {
    VmOfferPriorityVeryLow = 1,
    VmOfferPriorityLow,
    VmOfferPriorityBelowNormal,
    VmOfferPriorityNormal
} OFFER_PRIORITY;

DWORD
WINAPI
OfferVirtualMemory(
    _Inout_updates_(Size) PVOID VirtualAddress,
    _In_ SIZE_T Size,
    _In_ OFFER_PRIORITY Priority
    );

DWORD
WINAPI
ReclaimVirtualMemory(
    _In_reads_(Size) void const* VirtualAddress,
    _In_ SIZE_T Size
    );

DWORD
WINAPI
DiscardVirtualMemory(
    _Inout_updates_(Size) PVOID VirtualAddress,
    _In_ SIZE_T Size
    );

#endif /* NTDDI_VERSION > NTDDI_WINBLUE || (NTDDI_VERSION == NTDDI_WINBLUE && defined(WINBLUE_KBSPRING14))) */

#endif /* (_WIN32_WINNT >= _WIN32_WINNT_WINBLUE) */

#if (_WIN32_WINNT >= _WIN32_WINNT_WIN10)

WINBASEAPI
BOOL
WINAPI
SetProcessValidCallTargets(
    _In_ HANDLE hProcess,
    _In_ PVOID VirtualAddress,
    _In_ SIZE_T RegionSize,
    _In_ ULONG NumberOfOffsets,
    _Inout_updates_(NumberOfOffsets) PCFG_CALL_TARGET_INFO OffsetInformation
    );

WINBASEAPI
BOOL
WINAPI
SetProcessValidCallTargetsForMappedView(
    _In_ HANDLE Process,
    _In_ PVOID VirtualAddress,
    _In_ SIZE_T RegionSize,
    _In_ ULONG NumberOfOffsets,
    _Inout_updates_(NumberOfOffsets) PCFG_CALL_TARGET_INFO OffsetInformation,
    _In_ HANDLE Section,
    _In_ ULONG64 ExpectedFileOffset
    );

WINBASEAPI
_Ret_maybenull_
_Post_writable_byte_size_(Size)
PVOID
WINAPI
VirtualAllocFromApp(
    _In_opt_ PVOID BaseAddress,
    _In_ SIZE_T Size,
    _In_ ULONG AllocationType,
    _In_ ULONG Protection
    );

WINBASEAPI
_Success_(return != FALSE)
BOOL
WINAPI
VirtualProtectFromApp(
    _In_ PVOID Address,
    _In_ SIZE_T Size,
    _In_ ULONG NewProtection,
    _Out_ PULONG OldProtection
    );

WINBASEAPI
_Ret_maybenull_
HANDLE
WINAPI
OpenFileMappingFromApp(
    _In_ ULONG DesiredAccess,
    _In_ BOOL InheritHandle,
    _In_ PCWSTR Name
    );

#endif /* (_WIN32_WINNT >= _WIN32_WINNT_WIN10) */

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#pragma region Application Family
#if WINAPI_PARTITION_APP && !(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

#if (_WIN32_WINNT >= _WIN32_WINNT_WIN8)

#define CreateFileMapping  CreateFileMappingW

FORCEINLINE
_Ret_maybenull_
HANDLE
WINAPI
CreateFileMappingW(
    _In_     HANDLE hFile,
    _In_opt_ LPSECURITY_ATTRIBUTES lpFileMappingAttributes,
    _In_     DWORD flProtect,
    _In_     DWORD dwMaximumSizeHigh,
    _In_     DWORD dwMaximumSizeLow,
    _In_opt_ LPCWSTR lpName
    )
{
    return CreateFileMappingFromApp (hFile,
                                     lpFileMappingAttributes,
                                     flProtect,
                                     (((ULONG64) dwMaximumSizeHigh) << 32) | dwMaximumSizeLow,
                                     lpName);
}

FORCEINLINE
_Ret_maybenull_  __out_data_source(FILE)
LPVOID
WINAPI
MapViewOfFile(
    _In_ HANDLE hFileMappingObject,
    _In_ DWORD dwDesiredAccess,
    _In_ DWORD dwFileOffsetHigh,
    _In_ DWORD dwFileOffsetLow,
    _In_ SIZE_T dwNumberOfBytesToMap
    )
{
    return MapViewOfFileFromApp (hFileMappingObject,
                                 dwDesiredAccess,
                                 (((ULONG64) dwFileOffsetHigh) << 32) | dwFileOffsetLow,
                                 dwNumberOfBytesToMap);
}

#endif

#if (_WIN32_WINNT >= _WIN32_WINNT_WIN10)

FORCEINLINE
_Ret_maybenull_ _Post_writable_byte_size_(dwSize)
LPVOID
WINAPI
VirtualAlloc(
    _In_opt_ LPVOID lpAddress,
    _In_     SIZE_T dwSize,
    _In_     DWORD flAllocationType,
    _In_     DWORD flProtect
    )
{
    return VirtualAllocFromApp (lpAddress, dwSize, flAllocationType, flProtect);
}

FORCEINLINE
_Success_(return != FALSE)
BOOL
WINAPI
VirtualProtect(
    _In_  LPVOID lpAddress,
    _In_  SIZE_T dwSize,
    _In_  DWORD flNewProtect,
    _Out_ PDWORD lpflOldProtect
    )
{
    return VirtualProtectFromApp (lpAddress, dwSize, flNewProtect, lpflOldProtect);
}

#define OpenFileMapping  OpenFileMappingW

FORCEINLINE
_Ret_maybenull_
HANDLE
WINAPI
OpenFileMappingW(
    _In_ DWORD dwDesiredAccess,
    _In_ BOOL bInheritHandle,
    _In_ LPCWSTR lpName
    )
{
    return OpenFileMappingFromApp (dwDesiredAccess, bInheritHandle, lpName);
}

#endif

#endif  /* WINAPI_PARTITION_APP && !(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

#if (NTDDI_VERSION >= NTDDI_WIN10_RS1)

typedef enum WIN32_MEMORY_INFORMATION_CLASS {
    MemoryRegionInfo
} WIN32_MEMORY_INFORMATION_CLASS;

#if _MSC_VER >= 1200
#pragma warning(push)
#pragma warning(disable:4201)   // unnamed struct
#pragma warning(disable:4214)   // bit fields other than int
#endif

typedef struct WIN32_MEMORY_REGION_INFORMATION {
    PVOID AllocationBase;
    ULONG AllocationProtect;

    union {
        ULONG Flags;

        struct {
            ULONG Private : 1;
            ULONG MappedDataFile : 1;
            ULONG MappedImage : 1;
            ULONG MappedPageFile : 1;
            ULONG MappedPhysical : 1;
            ULONG DirectMapped : 1;
            ULONG Reserved : 26;
        } DUMMYSTRUCTNAME;
    } DUMMYUNIONNAME;

    SIZE_T RegionSize;
    SIZE_T CommitSize;
} WIN32_MEMORY_REGION_INFORMATION;

#if _MSC_VER >= 1200
#pragma warning(pop)
#endif

WINBASEAPI
_Success_(return != FALSE)
BOOL
WINAPI
QueryVirtualMemoryInformation(
    _In_ HANDLE Process,
    _In_ const VOID* VirtualAddress,
    _In_ WIN32_MEMORY_INFORMATION_CLASS MemoryInformationClass,
    _Out_writes_bytes_(MemoryInformationSize) PVOID MemoryInformation,
    _In_ SIZE_T MemoryInformationSize,
    _Out_opt_ PSIZE_T ReturnSize
    );

#endif // (NTDDI_VERSION >= NTDDI_WIN10_RS1)

#if (NTDDI_VERSION >= NTDDI_WIN10_RS2)

WINBASEAPI
_Ret_maybenull_  __out_data_source(FILE)
PVOID
WINAPI
MapViewOfFileNuma2(
    _In_ HANDLE FileMappingHandle,
    _In_ HANDLE ProcessHandle,
    _In_ ULONG64 Offset,
    _In_opt_ PVOID BaseAddress,
    _In_ SIZE_T ViewSize,
    _In_ ULONG AllocationType,
    _In_ ULONG PageProtection,
    _In_ ULONG PreferredNode
    );

#if !defined(MIDL_PASS)

FORCEINLINE
_Ret_maybenull_  __out_data_source(FILE)
PVOID
MapViewOfFile2(
    _In_ HANDLE FileMappingHandle,
    _In_ HANDLE ProcessHandle,
    _In_ ULONG64 Offset,
    _In_opt_ PVOID BaseAddress,
    _In_ SIZE_T ViewSize,
    _In_ ULONG AllocationType,
    _In_ ULONG PageProtection
    )
{
    return MapViewOfFileNuma2(FileMappingHandle,
                              ProcessHandle,
                              Offset,
                              BaseAddress,
                              ViewSize,
                              AllocationType,
                              PageProtection,
                              NUMA_NO_PREFERRED_NODE);
}

#endif // !defined(MIDL_PASS)

#endif // (NTDDI_VERSION >= NTDDI_WIN10_RS2)

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#pragma region Application Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)

WINBASEAPI
BOOL
WINAPI
UnmapViewOfFile2(
    _In_ HANDLE Process,
    _In_ PVOID BaseAddress,
    _In_ ULONG UnmapFlags
    );

WINBASEAPI
BOOL
WINAPI
VirtualUnlockEx(
    _In_opt_ HANDLE Process,
    _In_ LPVOID Address,
    _In_ SIZE_T Size
    );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#if (NTDDI_VERSION >= NTDDI_WIN10_RS4)

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

WINBASEAPI
_Ret_maybenull_
_Post_writable_byte_size_(Size)
PVOID
WINAPI
VirtualAlloc2(
    _In_opt_ HANDLE Process,
    _In_opt_ PVOID BaseAddress,
    _In_ SIZE_T Size,
    _In_ ULONG AllocationType,
    _In_ ULONG PageProtection,
    _Inout_updates_opt_(ParameterCount) MEM_EXTENDED_PARAMETER* ExtendedParameters,
    _In_ ULONG ParameterCount
    );

WINBASEAPI
_Ret_maybenull_  __out_data_source(FILE)
PVOID
WINAPI
MapViewOfFile3(
    _In_ HANDLE FileMapping,
    _In_opt_ HANDLE Process,
    _In_opt_ PVOID BaseAddress,
    _In_ ULONG64 Offset,
    _In_ SIZE_T ViewSize,
    _In_ ULONG AllocationType,
    _In_ ULONG PageProtection,
    _Inout_updates_opt_(ParameterCount) MEM_EXTENDED_PARAMETER* ExtendedParameters,
    _In_ ULONG ParameterCount
    );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#pragma region Application Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)

WINBASEAPI
_Ret_maybenull_
_Post_writable_byte_size_(Size)
PVOID
WINAPI
VirtualAlloc2FromApp(
    _In_opt_ HANDLE Process,
    _In_opt_ PVOID BaseAddress,
    _In_ SIZE_T Size,
    _In_ ULONG AllocationType,
    _In_ ULONG PageProtection,
    _Inout_updates_opt_(ParameterCount) MEM_EXTENDED_PARAMETER* ExtendedParameters,
    _In_ ULONG ParameterCount
    );

WINBASEAPI
_Ret_maybenull_  __out_data_source(FILE)
PVOID
WINAPI
MapViewOfFile3FromApp(
    _In_ HANDLE FileMapping,
    _In_opt_ HANDLE Process,
    _In_opt_ PVOID BaseAddress,
    _In_ ULONG64 Offset,
    _In_ SIZE_T ViewSize,
    _In_ ULONG AllocationType,
    _In_ ULONG PageProtection,
    _Inout_updates_opt_(ParameterCount) MEM_EXTENDED_PARAMETER* ExtendedParameters,
    _In_ ULONG ParameterCount
    );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#endif // (NTDDI_VERSION >= NTDDI_WIN10_RS4)

#if (NTDDI_VERSION >= NTDDI_WIN10_RS5)

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

WINBASEAPI
_Ret_maybenull_
HANDLE
WINAPI
CreateFileMapping2(
    _In_ HANDLE File,
    _In_opt_ SECURITY_ATTRIBUTES* SecurityAttributes,
    _In_ ULONG DesiredAccess,
    _In_ ULONG PageProtection,
    _In_ ULONG AllocationAttributes,
    _In_ ULONG64 MaximumSize,
    _In_opt_ PCWSTR Name,
    _Inout_updates_opt_(ParameterCount) MEM_EXTENDED_PARAMETER* ExtendedParameters,
    _In_ ULONG ParameterCount
    );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#endif // (NTDDI_VERSION >= NTDDI_WIN10_RS5)

#if (NTDDI_VERSION >= NTDDI_WIN10_FE)

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

WINBASEAPI
_Success_(return != FALSE)
BOOL
WINAPI
AllocateUserPhysicalPages2(
    _In_ HANDLE ObjectHandle,
    _Inout_ PULONG_PTR NumberOfPages,
    _Out_writes_(*NumberOfPages) PULONG_PTR PageArray,
    _Inout_updates_opt_(ExtendedParameterCount) PMEM_EXTENDED_PARAMETER ExtendedParameters,
    _In_ ULONG ExtendedParameterCount
    );

typedef enum WIN32_MEMORY_PARTITION_INFORMATION_CLASS {
    MemoryPartitionInfo,
    MemoryPartitionDedicatedMemoryInfo
} WIN32_MEMORY_PARTITION_INFORMATION_CLASS;

typedef struct DECLSPEC_ALIGN(8) WIN32_MEMORY_PARTITION_INFORMATION {
    ULONG Flags;
    ULONG NumaNode;             // -1 indicates sum all nodes in the partition
    ULONG Channel;              // -1 indicates sum all channels in node
    ULONG NumberOfNumaNodes;            // 0 unless all nodes specified above
    ULONG64 ResidentAvailablePages;     // 0 unless all nodes specified above
    ULONG64 CommittedPages;             // 0 unless all nodes specified above
    ULONG64 CommitLimit;                // 0 unless all nodes specified above
    ULONG64 PeakCommitment;             // 0 unless all nodes specified above
    ULONG64 TotalNumberOfPages;
    ULONG64 AvailablePages;
    ULONG64 ZeroPages;
    ULONG64 FreePages;
    ULONG64 StandbyPages;
    ULONG64 Reserved[16];
    ULONG64 MaximumCommitLimit;         // 0 unless all nodes specified above
    ULONG64 Reserved2;

    ULONG PartitionId;

} WIN32_MEMORY_PARTITION_INFORMATION;

WINBASEAPI
HANDLE
WINAPI
OpenDedicatedMemoryPartition(
    _In_ HANDLE Partition,
    _In_ ULONG64 DedicatedMemoryTypeId,
    _In_ ACCESS_MASK DesiredAccess,
    _In_ BOOL InheritHandle
    );

WINBASEAPI
_Success_(return != FALSE)
BOOL
WINAPI
QueryPartitionInformation(
    _In_ HANDLE Partition,
    _In_ WIN32_MEMORY_PARTITION_INFORMATION_CLASS PartitionInformationClass,
    _Inout_updates_bytes_(PartitionInformationLength) PVOID PartitionInformation,
    _In_ ULONG PartitionInformationLength
    );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#endif // (NTDDI_VERSION >= NTDDI_WIN10_FE)

#if (NTDDI_VERSION >= NTDDI_WIN11_GA)

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

WINBASEAPI
_Success_(return != FALSE)
BOOL
WINAPI
GetMemoryNumaClosestInitiatorNode(
    _In_ ULONG TargetNodeNumber,
    _Out_ ULONG* InitiatorNodeNumber
    );

#define WIN32_MEMORY_NUMA_PERFORMANCE_ALL_TARGET_NODE   0xffffffff

#define WIN32_MEMORY_NUMA_PERFORMANCE_READ_LATENCY      0x1
#define WIN32_MEMORY_NUMA_PERFORMANCE_READ_BANDWIDTH    0x2
#define WIN32_MEMORY_NUMA_PERFORMANCE_WRITE_LATENCY     0x4
#define WIN32_MEMORY_NUMA_PERFORMANCE_WRITE_BANDWIDTH   0x8

typedef struct _WIN32_MEMORY_NUMA_PERFORMANCE_ENTRY {

    //
    // The initiator processor node. The performance value is the measurement of operations from the
    // initiator processor node to the target memory node.
    // This value is 0-based.
    //
    ULONG InitiatorNodeNumber;

    //
    // The target memory node. This value is 0-based.
    //
    ULONG TargetNodeNumber;

    //
    // DataType should be one of the WIN32_MEMROY_NUMA_PERFORMANCE_* types
    //
    UCHAR DataType;

    struct {

        //
        // The performance value is achieved with a minimum transfer size.
        //
        UCHAR MinTransferSizeToAchieveValues : 1;

        //
        // The performance value is achieved with non sequential transfers.
        //
        UCHAR NonSequentialTransfers : 1;

        UCHAR Reserved : 6;
    } Flags;

    //
    // Minimum transfer size, only valid when MinTransferSizeToAchieveValues is 1.
    // 0 means byte-aligned (any alignment).
    //
    ULONGLONG MinTransferSizeInBytes;

    //
    // EntryValue is latency or bandwidth depends on the DataType.
    // Latency unit is picoseconds. Bandwidth unit is MB/s.
    //
    ULONGLONG EntryValue;
} WIN32_MEMORY_NUMA_PERFORMANCE_ENTRY;

typedef struct _WIN32_MEMORY_NUMA_PERFORMANCE_INFORMATION_OUTPUT {
    ULONG EntryCount;

    WIN32_MEMORY_NUMA_PERFORMANCE_ENTRY PerformanceEntries[ANYSIZE_ARRAY];
} WIN32_MEMORY_NUMA_PERFORMANCE_INFORMATION_OUTPUT;

BOOL
WINAPI
GetMemoryNumaPerformanceInformation(
    _In_ ULONG NodeNumber,
    _In_ UCHAR DataType,
    _Outptr_ WIN32_MEMORY_NUMA_PERFORMANCE_INFORMATION_OUTPUT** PerfInfo
    );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#endif // (NTDDI_VERSION >= NTDDI_WIN11_GA)

#if _MSC_VER >= 1200
#pragma warning(pop)
#endif

#ifdef __cplusplus
}
#endif

#endif // _MEMORYAPI_H_
