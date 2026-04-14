/********************************************************************************
*                                                                               *
* HeapApi.h -- ApiSet Contract for api-ms-win-core-heap-l1                      *
*                                                                               *
* Copyright (c) Microsoft Corporation. All rights reserved.                     *
*                                                                               *
********************************************************************************/

#ifdef _MSC_VER
#pragma once
#endif // _MSC_VER

#ifndef _HEAPAPI_H_
#define _HEAPAPI_H_

#include <apiset.h>
#include <apisetcconv.h>
#include <minwindef.h>
#include <minwinbase.h>

#if _MSC_VER < 1900
#define DECLSPEC_ALLOCATOR
#else
#define DECLSPEC_ALLOCATOR __declspec(allocator)
#endif

#ifdef __cplusplus
extern "C" {
#endif

#if _MSC_VER >= 1200
#pragma warning(push)
#pragma warning(disable:4820) // padding added after data member
#endif

#pragma region Application Family or OneCore Family or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

//
// typdefs
//

typedef struct _HEAP_SUMMARY {
    DWORD cb;
    SIZE_T cbAllocated;
    SIZE_T cbCommitted;
    SIZE_T cbReserved;
    SIZE_T cbMaxReserve;
} HEAP_SUMMARY, *PHEAP_SUMMARY;
typedef PHEAP_SUMMARY LPHEAP_SUMMARY;

//
// Prototypes
//

WINBASEAPI
_Ret_maybenull_
HANDLE
WINAPI
HeapCreate(
    _In_ DWORD flOptions,
    _In_ SIZE_T dwInitialSize,
    _In_ SIZE_T dwMaximumSize
    );

WINBASEAPI
BOOL
WINAPI
HeapDestroy(
    _In_ HANDLE hHeap
    );

WINBASEAPI
_Ret_maybenull_
_Post_writable_byte_size_(dwBytes)
DECLSPEC_ALLOCATOR
LPVOID
WINAPI
HeapAlloc(
    _In_ HANDLE hHeap,
    _In_ DWORD dwFlags,
    _In_ SIZE_T dwBytes
    );

WINBASEAPI
_Success_(return != 0)
_Ret_maybenull_
_Post_writable_byte_size_(dwBytes)
DECLSPEC_ALLOCATOR
LPVOID
WINAPI
HeapReAlloc(
    _Inout_ HANDLE hHeap,
    _In_ DWORD dwFlags,
    _Frees_ptr_opt_ LPVOID lpMem,
    _In_ SIZE_T dwBytes
    );

WINBASEAPI
_Success_(return != FALSE)
BOOL
WINAPI
HeapFree(
    _Inout_ HANDLE hHeap,
    _In_ DWORD dwFlags,
    __drv_freesMem(Mem) _Frees_ptr_opt_ LPVOID lpMem
    );

WINBASEAPI
SIZE_T
WINAPI
HeapSize(
    _In_ HANDLE hHeap,
    _In_ DWORD dwFlags,
    _In_ LPCVOID lpMem
    );

WINBASEAPI
HANDLE
WINAPI
GetProcessHeap(
    VOID
    );

WINBASEAPI
SIZE_T
WINAPI
HeapCompact(
    _In_ HANDLE hHeap,
    _In_ DWORD dwFlags
    );

WINBASEAPI
BOOL
WINAPI
HeapSetInformation(
    _In_opt_ HANDLE HeapHandle,
    _In_ HEAP_INFORMATION_CLASS HeapInformationClass,
    _In_reads_bytes_opt_(HeapInformationLength) PVOID HeapInformation,
    _In_ SIZE_T HeapInformationLength
    );

#endif // WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)
#pragma endregion

#pragma region Desktop Family or OneCore Family or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

WINBASEAPI
BOOL
WINAPI
HeapValidate(
    _In_ HANDLE hHeap,
    _In_ DWORD dwFlags,
    _In_opt_ LPCVOID lpMem
    );

#endif // WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)
#pragma endregion

#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP)

BOOL
WINAPI
HeapSummary(
    _In_ HANDLE hHeap,
    _In_ DWORD dwFlags,
    _Out_ LPHEAP_SUMMARY lpSummary
    );

#endif // WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP)
#pragma endregion

#pragma region Application Family or OneCore Family or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

WINBASEAPI
DWORD
WINAPI
GetProcessHeaps(
    _In_ DWORD NumberOfHeaps,
    _Out_writes_to_(NumberOfHeaps,return) PHANDLE ProcessHeaps
    );

WINBASEAPI
BOOL
WINAPI
HeapLock(
    _In_ HANDLE hHeap
    );

WINBASEAPI
BOOL
WINAPI
HeapUnlock(
    _In_ HANDLE hHeap
    );

#endif // WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)
#pragma endregion

#pragma region Desktop Family or OneCore Family or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

WINBASEAPI
BOOL
WINAPI
HeapWalk(
    _In_ HANDLE hHeap,
    _Inout_ LPPROCESS_HEAP_ENTRY lpEntry
    );

WINBASEAPI
BOOL
WINAPI
HeapQueryInformation(
    _In_opt_ HANDLE HeapHandle,
    _In_ HEAP_INFORMATION_CLASS HeapInformationClass,
    _Out_writes_bytes_to_opt_(HeapInformationLength,*ReturnLength) PVOID HeapInformation,
    _In_ SIZE_T HeapInformationLength,
    _Out_opt_ PSIZE_T ReturnLength
    );

#endif // WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)
#pragma endregion

//
// HeapSummary() is in minwinbase.w within ;beg_internal tags. Has to stay there for downlevel reasons.
//

#if _MSC_VER >= 1200
#pragma warning(pop)
#endif

#ifdef __cplusplus
}
#endif

#endif // _HEAPAPI_H_
