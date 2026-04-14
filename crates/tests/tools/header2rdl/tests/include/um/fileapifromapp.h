/*********************************************************************************
*                                                                                *
* fileapifromapp.h - ApiSet Contract for api-ms-win-core-file-fromapp-l1         *
*                                                                                *
* Copyright (c) Microsoft Corporation. All rights reserved.                      *
*                                                                                *
*********************************************************************************/

#ifndef _FILEAPIFROMAPP_H_
#define _FILEAPIFROMAPP_H_

#ifdef _MSC_VER
#pragma once
#endif // _MSC_VER

#include <apiset.h>
#include <apisetcconv.h>
#include <minwindef.h>
#include <minwinbase.h>
#include <fileapi.h>

#ifdef __cplusplus
extern "C" {
#endif

#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP)
#if ((_WIN32_WINNT >= _WIN32_WINNT_WIN10) && (NTDDI_VERSION >= NTDDI_WIN10_RS3))

WINSTORAGEAPI
BOOL
WINAPI
CopyFileFromAppW(
    _In_ LPCWSTR lpExistingFileName,
    _In_ LPCWSTR lpNewFileName,
    _In_ BOOL bFailIfExists
    )
#if defined(__cplusplus) && _MSC_VER >= 1900
noexcept
#endif
;

WINSTORAGEAPI
BOOL
WINAPI
CreateDirectoryFromAppW(
    _In_ LPCWSTR lpPathName,
    _In_opt_ LPSECURITY_ATTRIBUTES lpSecurityAttributes
    )
#if defined(__cplusplus) && _MSC_VER >= 1900
noexcept
#endif
;

WINSTORAGEAPI
HANDLE
WINAPI
CreateFileFromAppW(
    _In_ LPCWSTR lpFileName,
    _In_ DWORD dwDesiredAccess,
    _In_ DWORD dwShareMode,
    _In_opt_ LPSECURITY_ATTRIBUTES lpSecurityAttributes,
    _In_ DWORD dwCreationDisposition,
    _In_ DWORD dwFlagsAndAttributes,
    _In_opt_ HANDLE hTemplateFile
    )
#if defined(__cplusplus) && _MSC_VER >= 1900
noexcept
#endif
;

WINSTORAGEAPI
HANDLE
WINAPI
CreateFile2FromAppW(
    _In_ LPCWSTR lpFileName,
    _In_ DWORD dwDesiredAccess,
    _In_ DWORD dwShareMode,
    _In_ DWORD dwCreationDisposition,
    _In_opt_ LPCREATEFILE2_EXTENDED_PARAMETERS pCreateExParams
    )
#if defined(__cplusplus) && _MSC_VER >= 1900
noexcept
#endif
;

WINSTORAGEAPI
BOOL
WINAPI
DeleteFileFromAppW(
    _In_ LPCWSTR lpFileName
    )
#if defined(__cplusplus) && _MSC_VER >= 1900
noexcept
#endif
;

WINSTORAGEAPI
HANDLE
WINAPI
FindFirstFileExFromAppW(
    _In_ LPCWSTR lpFileName,
    _In_ FINDEX_INFO_LEVELS fInfoLevelId,
    _Out_writes_bytes_(sizeof(WIN32_FIND_DATAW)) LPVOID lpFindFileData,
    _In_ FINDEX_SEARCH_OPS fSearchOp,
    _Reserved_ LPVOID lpSearchFilter,
    _In_ DWORD dwAdditionalFlags
    )
#if defined(__cplusplus) && _MSC_VER >= 1900
noexcept
#endif
;

WINSTORAGEAPI
BOOL
WINAPI
GetFileAttributesExFromAppW(
    _In_ LPCWSTR lpFileName,
    _In_ GET_FILEEX_INFO_LEVELS fInfoLevelId,
    _Out_writes_bytes_(sizeof(WIN32_FILE_ATTRIBUTE_DATA)) LPVOID lpFileInformation
    )
#if defined(__cplusplus) && _MSC_VER >= 1900
noexcept
#endif
;

WINSTORAGEAPI
BOOL
WINAPI
MoveFileFromAppW(
    _In_ LPCWSTR lpExistingFileName,
    _In_ LPCWSTR lpNewFileName
    )
#if defined(__cplusplus) && _MSC_VER >= 1900
noexcept
#endif
;

WINSTORAGEAPI
BOOL
WINAPI
RemoveDirectoryFromAppW(
    _In_ LPCWSTR lpPathName
    )
#if defined(__cplusplus) && _MSC_VER >= 1900
noexcept
#endif
;

WINSTORAGEAPI
BOOL
WINAPI
ReplaceFileFromAppW(
    _In_ LPCWSTR lpReplacedFileName,
    _In_ LPCWSTR lpReplacementFileName,
    _In_opt_ LPCWSTR lpBackupFileName,
    _In_ DWORD dwReplaceFlags,
    _Reserved_ LPVOID lpExclude,
    _Reserved_ LPVOID lpReserved
    )
#if defined(__cplusplus) && _MSC_VER >= 1900
noexcept
#endif
;

WINSTORAGEAPI
BOOL
WINAPI
SetFileAttributesFromAppW(
    _In_ LPCWSTR lpFileName,
    _In_ DWORD dwFileAttributes
    )
#if defined(__cplusplus) && _MSC_VER >= 1900
noexcept
#endif
;

#endif // Version check
#endif/* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP) */
#pragma endregion

#ifdef __cplusplus
} // extern "C"
#endif

#endif // _FILEAPIFROMAPP_H_
