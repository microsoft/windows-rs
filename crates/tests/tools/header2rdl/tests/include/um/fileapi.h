/********************************************************************************
*                                                                               *
* FileApi.h -- ApiSet Contract for api-ms-win-core-file-l1                      *
*                                                                               *
* Copyright (c) Microsoft Corporation. All rights reserved.                     *
*                                                                               *
********************************************************************************/

#ifdef _MSC_VER
#pragma once
#endif // _MSC_VER

#ifndef _APISETFILE_
#define _APISETFILE_

#include <apiset.h>
#include <apisetcconv.h>
#include <minwindef.h>
#include <minwinbase.h>

#ifdef __cplusplus
extern "C" {
#endif

#pragma region Application Family or OneCore Family or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

//
// Constants
//
#define CREATE_NEW          1
#define CREATE_ALWAYS       2
#define OPEN_EXISTING       3
#define OPEN_ALWAYS         4
#define TRUNCATE_EXISTING   5

#define INVALID_FILE_SIZE ((DWORD)0xFFFFFFFF)
#define INVALID_SET_FILE_POINTER ((DWORD)-1)
#define INVALID_FILE_ATTRIBUTES ((DWORD)-1)

WINBASEAPI
LONG
WINAPI
CompareFileTime(
    _In_ CONST FILETIME* lpFileTime1,
    _In_ CONST FILETIME* lpFileTime2
    );

WINBASEAPI
BOOL
WINAPI
CreateDirectoryA(
    _In_ LPCSTR lpPathName,
    _In_opt_ LPSECURITY_ATTRIBUTES lpSecurityAttributes
    );

WINBASEAPI
BOOL
WINAPI
CreateDirectoryW(
    _In_ LPCWSTR lpPathName,
    _In_opt_ LPSECURITY_ATTRIBUTES lpSecurityAttributes
    );

#ifdef UNICODE
#define CreateDirectory  CreateDirectoryW
#else
#define CreateDirectory  CreateDirectoryA
#endif // !UNICODE

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Desktop Family or OneCore Family or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

WINBASEAPI
HANDLE
WINAPI
CreateFileA(
    _In_ LPCSTR lpFileName,
    _In_ DWORD dwDesiredAccess,
    _In_ DWORD dwShareMode,
    _In_opt_ LPSECURITY_ATTRIBUTES lpSecurityAttributes,
    _In_ DWORD dwCreationDisposition,
    _In_ DWORD dwFlagsAndAttributes,
    _In_opt_ HANDLE hTemplateFile
    );

WINBASEAPI
HANDLE
WINAPI
CreateFileW(
    _In_ LPCWSTR lpFileName,
    _In_ DWORD dwDesiredAccess,
    _In_ DWORD dwShareMode,
    _In_opt_ LPSECURITY_ATTRIBUTES lpSecurityAttributes,
    _In_ DWORD dwCreationDisposition,
    _In_ DWORD dwFlagsAndAttributes,
    _In_opt_ HANDLE hTemplateFile
    );

#ifdef UNICODE
#define CreateFile  CreateFileW
#else
#define CreateFile  CreateFileA
#endif // !UNICODE

#endif // WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)
#pragma endregion

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

WINBASEAPI
BOOL
WINAPI
DefineDosDeviceW(
    _In_ DWORD dwFlags,
    _In_ LPCWSTR lpDeviceName,
    _In_opt_ LPCWSTR lpTargetPath
    );

#ifdef UNICODE
#define DefineDosDevice  DefineDosDeviceW
#endif

#endif // WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)
#pragma endregion

#pragma region Application Family or OneCore Family or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

WINBASEAPI
BOOL
WINAPI
DeleteFileA(
    _In_ LPCSTR lpFileName
    );

WINBASEAPI
BOOL
WINAPI
DeleteFileW(
    _In_ LPCWSTR lpFileName
    );

#ifdef UNICODE
#define DeleteFile  DeleteFileW
#else
#define DeleteFile  DeleteFileA
#endif // !UNICODE

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Application Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)

WINBASEAPI
BOOL
WINAPI
DeleteVolumeMountPointW(
    _In_ LPCWSTR lpszVolumeMountPoint
    );

#ifdef UNICODE
#define DeleteVolumeMountPoint  DeleteVolumeMountPointW
#endif

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#pragma region Application Family or OneCore Family or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

WINBASEAPI
BOOL
WINAPI
FileTimeToLocalFileTime(
    _In_ CONST FILETIME* lpFileTime,
    _Out_ LPFILETIME lpLocalFileTime
    );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Application Family or OneCore Family or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

WINBASEAPI
BOOL
WINAPI
FindClose(
    _Inout_ HANDLE hFindFile
    );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Application Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)

WINBASEAPI
BOOL
WINAPI
FindCloseChangeNotification(
    _In_ HANDLE hChangeHandle
    );

WINBASEAPI
HANDLE
WINAPI
FindFirstChangeNotificationA(
    _In_ LPCSTR lpPathName,
    _In_ BOOL bWatchSubtree,
    _In_ DWORD dwNotifyFilter
    );

WINBASEAPI
HANDLE
WINAPI
FindFirstChangeNotificationW(
    _In_ LPCWSTR lpPathName,
    _In_ BOOL bWatchSubtree,
    _In_ DWORD dwNotifyFilter
    );

#ifdef UNICODE
#define FindFirstChangeNotification  FindFirstChangeNotificationW
#else
#define FindFirstChangeNotification  FindFirstChangeNotificationA
#endif // !UNICODE

#endif // WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)
#pragma endregion

#pragma region Application Family or OneCore Family or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

WINBASEAPI
HANDLE
WINAPI
FindFirstFileA(
    _In_ LPCSTR lpFileName,
    _Out_ LPWIN32_FIND_DATAA lpFindFileData
    );

WINBASEAPI
HANDLE
WINAPI
FindFirstFileW(
    _In_ LPCWSTR lpFileName,
    _Out_ LPWIN32_FIND_DATAW lpFindFileData
    );

#ifdef UNICODE
#define FindFirstFile  FindFirstFileW
#else
#define FindFirstFile  FindFirstFileA
#endif // !UNICODE

#if (_WIN32_WINNT >= 0x0400)

WINBASEAPI
HANDLE
WINAPI
FindFirstFileExA(
    _In_ LPCSTR lpFileName,
    _In_ FINDEX_INFO_LEVELS fInfoLevelId,
    _Out_writes_bytes_(sizeof(WIN32_FIND_DATAA)) LPVOID lpFindFileData,
    _In_ FINDEX_SEARCH_OPS fSearchOp,
    _Reserved_ LPVOID lpSearchFilter,
    _In_ DWORD dwAdditionalFlags
    );

WINBASEAPI
HANDLE
WINAPI
FindFirstFileExW(
    _In_ LPCWSTR lpFileName,
    _In_ FINDEX_INFO_LEVELS fInfoLevelId,
    _Out_writes_bytes_(sizeof(WIN32_FIND_DATAW)) LPVOID lpFindFileData,
    _In_ FINDEX_SEARCH_OPS fSearchOp,
    _Reserved_ LPVOID lpSearchFilter,
    _In_ DWORD dwAdditionalFlags
    );

#ifdef UNICODE
#define FindFirstFileEx  FindFirstFileExW
#else
#define FindFirstFileEx  FindFirstFileExA
#endif // !UNICODE

#endif /* _WIN32_WINNT >= 0x0400 */

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Application Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)

WINBASEAPI
HANDLE
WINAPI
FindFirstVolumeW(
    _Out_writes_(cchBufferLength) LPWSTR lpszVolumeName,
    _In_ DWORD cchBufferLength
    );

#ifdef UNICODE
#define FindFirstVolume FindFirstVolumeW
#endif // !UNICODE

WINBASEAPI
BOOL
WINAPI
FindNextChangeNotification(
    _In_ HANDLE hChangeHandle
    );

#endif // WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)
#pragma endregion

#pragma region Application Family or OneCore Family or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

WINBASEAPI
BOOL
WINAPI
FindNextFileA(
    _In_ HANDLE hFindFile,
    _Out_ LPWIN32_FIND_DATAA lpFindFileData
    );

WINBASEAPI
BOOL
WINAPI
FindNextFileW(
    _In_ HANDLE hFindFile,
    _Out_ LPWIN32_FIND_DATAW lpFindFileData
    );

#ifdef UNICODE
#define FindNextFile  FindNextFileW
#else
#define FindNextFile  FindNextFileA
#endif // !UNICODE

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Application Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)

WINBASEAPI
BOOL
WINAPI
FindNextVolumeW(
    _Inout_ HANDLE hFindVolume,
    _Out_writes_(cchBufferLength) LPWSTR lpszVolumeName,
    _In_ DWORD cchBufferLength
    );

#ifdef UNICODE
#define FindNextVolume FindNextVolumeW
#endif

WINBASEAPI
BOOL
WINAPI
FindVolumeClose(
    _In_ HANDLE hFindVolume
    );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#pragma region Application Family or OneCore Family or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

WINBASEAPI
BOOL
WINAPI
FlushFileBuffers(
    _In_ HANDLE hFile
    );

WINBASEAPI
BOOL
WINAPI
GetDiskFreeSpaceA(
    _In_opt_ LPCSTR lpRootPathName,
    _Out_opt_ LPDWORD lpSectorsPerCluster,
    _Out_opt_ LPDWORD lpBytesPerSector,
    _Out_opt_ LPDWORD lpNumberOfFreeClusters,
    _Out_opt_ LPDWORD lpTotalNumberOfClusters
    );

WINBASEAPI
BOOL
WINAPI
GetDiskFreeSpaceW(
    _In_opt_ LPCWSTR lpRootPathName,
    _Out_opt_ LPDWORD lpSectorsPerCluster,
    _Out_opt_ LPDWORD lpBytesPerSector,
    _Out_opt_ LPDWORD lpNumberOfFreeClusters,
    _Out_opt_ LPDWORD lpTotalNumberOfClusters
    );

#ifdef UNICODE
#define GetDiskFreeSpace  GetDiskFreeSpaceW
#else
#define GetDiskFreeSpace  GetDiskFreeSpaceA
#endif // !UNICODE

WINBASEAPI
BOOL
WINAPI
GetDiskFreeSpaceExA(
    _In_opt_ LPCSTR lpDirectoryName,
    _Out_opt_ PULARGE_INTEGER lpFreeBytesAvailableToCaller,
    _Out_opt_ PULARGE_INTEGER lpTotalNumberOfBytes,
    _Out_opt_ PULARGE_INTEGER lpTotalNumberOfFreeBytes
    );

WINBASEAPI
BOOL
WINAPI
GetDiskFreeSpaceExW(
    _In_opt_ LPCWSTR lpDirectoryName,
    _Out_opt_ PULARGE_INTEGER lpFreeBytesAvailableToCaller,
    _Out_opt_ PULARGE_INTEGER lpTotalNumberOfBytes,
    _Out_opt_ PULARGE_INTEGER lpTotalNumberOfFreeBytes
    );

#ifdef UNICODE
#define GetDiskFreeSpaceEx  GetDiskFreeSpaceExW
#else
#define GetDiskFreeSpaceEx  GetDiskFreeSpaceExA
#endif // !UNICODE

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Application Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)

//
//  The structure definition must be same as the one
//  (FILE_FS_FULL_SIZE_INFORMATION_EX) defined in ntioapi_x.w
//

typedef struct DISK_SPACE_INFORMATION {

    //
    //  AllocationUnits are actually file system clusters.
    //  AllocationUnits * SectorsPerAllocationUnit * BytesPerSector
    //  will get you the sizes in bytes.
    //

    //
    //  The Actual*AllocationUnits are volume sizes without considering Quota
    //  setting.
    //  ActualPoolUnavailableAllocationUnits is the unavailable space for the
    //  volume due to insufficient free pool space (PoolAvailableAllocationUnits).
    //  Be aware AllocationUnits are mesured in clusters, see comments at the beginning.
    //
    //  ActualTotalAllocationUnits = ActualAvailableAllocationUnits +
    //                               ActualPoolUnavailableAllocationUnits +
    //                               UsedAllocationUnits +
    //                               TotalReservedAllocationUnits
    //

    ULONGLONG ActualTotalAllocationUnits;
    ULONGLONG ActualAvailableAllocationUnits;
    ULONGLONG ActualPoolUnavailableAllocationUnits;

    //
    //  The Caller*AllocationUnits are limited by Quota setting.
    //  CallerPoolUnavailableAllocationUnits is the unavailable space for the
    //  volume due to insufficient free pool space (PoolAvailableAllocationUnits).
    //  Be aware AllocationUnits are mesured in clusters, see comments at the beginning.
    //
    //  CallerTotalAllocationUnits = CallerAvailableAllocationUnits +
    //                               CallerPoolUnavailableAllocationUnits +
    //                               UsedAllocationUnits +
    //                               TotalReservedAllocationUnits
    //

    ULONGLONG CallerTotalAllocationUnits;
    ULONGLONG CallerAvailableAllocationUnits;
    ULONGLONG CallerPoolUnavailableAllocationUnits;

    //
    //  The used space (in clusters) of the volume.
    //

    ULONGLONG UsedAllocationUnits;

    //
    //  Total reserved space (in clusters).
    //

    ULONGLONG TotalReservedAllocationUnits;

    //
    //  A special type of reserved space (in clusters) for per-volume storage
    //  reserve and this is included in the above TotalReservedAllocationUnits.
    //

    ULONGLONG VolumeStorageReserveAllocationUnits;

    //
    //  This refers to the space (in clusters) that has been committed by
    //  storage pool but has not been allocated by file system.
    //
    //  s1 = (ActualTotalAllocationUnits - UsedAllocationUnits - TotalReservedAllocationUnits)
    //  s2 = (AvailableCommittedAllocationUnits + PoolAvailableAllocationUnits)
    //  ActualAvailableAllocationUnits = min( s1, s2 )
    //
    //  When s1 >= s2, ActualPoolUnavailableAllocationUnits = 0
    //  When s1 < s2, ActualPoolUnavailableAllocationUnits = s2 - s1.
    //

    ULONGLONG AvailableCommittedAllocationUnits;

    //
    //  Available space (in clusters) in corresponding storage pool. If the volume
    //  is not a spaces volume, the PoolAvailableAllocationUnits is set to zero.
    //

    ULONGLONG PoolAvailableAllocationUnits;

    DWORD SectorsPerAllocationUnit;
    DWORD BytesPerSector;

} DISK_SPACE_INFORMATION;

WINBASEAPI
HRESULT
WINAPI
GetDiskSpaceInformationA(
    _In_opt_ LPCSTR rootPath,
    _Out_ DISK_SPACE_INFORMATION* diskSpaceInfo
    );

WINBASEAPI
HRESULT
WINAPI
GetDiskSpaceInformationW(
    _In_opt_ LPCWSTR rootPath,
    _Out_ DISK_SPACE_INFORMATION* diskSpaceInfo
    );

#ifdef UNICODE
#define GetDiskSpaceInformation  GetDiskSpaceInformationW
#else
#define GetDiskSpaceInformation  GetDiskSpaceInformationA
#endif // !UNICODE

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#pragma region Application Family or OneCore Family or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

WINBASEAPI
UINT
WINAPI
GetDriveTypeA(
    _In_opt_ LPCSTR lpRootPathName
    );

WINBASEAPI
UINT
WINAPI
GetDriveTypeW(
    _In_opt_ LPCWSTR lpRootPathName
    );

#ifdef UNICODE
#define GetDriveType  GetDriveTypeW
#else
#define GetDriveType  GetDriveTypeA
#endif // !UNICODE

typedef struct _WIN32_FILE_ATTRIBUTE_DATA {
    DWORD dwFileAttributes;
    FILETIME ftCreationTime;
    FILETIME ftLastAccessTime;
    FILETIME ftLastWriteTime;
    DWORD nFileSizeHigh;
    DWORD nFileSizeLow;
} WIN32_FILE_ATTRIBUTE_DATA, *LPWIN32_FILE_ATTRIBUTE_DATA;

WINBASEAPI
DWORD
WINAPI
GetFileAttributesA(
    _In_ LPCSTR lpFileName
    );

WINBASEAPI
DWORD
WINAPI
GetFileAttributesW(
    _In_ LPCWSTR lpFileName
    );

#ifdef UNICODE
#define GetFileAttributes  GetFileAttributesW
#else
#define GetFileAttributes  GetFileAttributesA
#endif // !UNICODE

WINBASEAPI
BOOL
WINAPI
GetFileAttributesExA(
    _In_ LPCSTR lpFileName,
    _In_ GET_FILEEX_INFO_LEVELS fInfoLevelId,
    _Out_writes_bytes_(sizeof(WIN32_FILE_ATTRIBUTE_DATA)) LPVOID lpFileInformation
    );

WINBASEAPI
BOOL
WINAPI
GetFileAttributesExW(
    _In_ LPCWSTR lpFileName,
    _In_ GET_FILEEX_INFO_LEVELS fInfoLevelId,
    _Out_writes_bytes_(sizeof(WIN32_FILE_ATTRIBUTE_DATA)) LPVOID lpFileInformation
    );

#ifdef UNICODE
#define GetFileAttributesEx  GetFileAttributesExW
#else
#define GetFileAttributesEx  GetFileAttributesExA
#endif // !UNICODE

typedef struct _BY_HANDLE_FILE_INFORMATION {
    DWORD dwFileAttributes;
    FILETIME ftCreationTime;
    FILETIME ftLastAccessTime;
    FILETIME ftLastWriteTime;
    DWORD dwVolumeSerialNumber;
    DWORD nFileSizeHigh;
    DWORD nFileSizeLow;
    DWORD nNumberOfLinks;
    DWORD nFileIndexHigh;
    DWORD nFileIndexLow;
} BY_HANDLE_FILE_INFORMATION, *PBY_HANDLE_FILE_INFORMATION, *LPBY_HANDLE_FILE_INFORMATION;

WINBASEAPI
BOOL
WINAPI
GetFileInformationByHandle(
    _In_ HANDLE hFile,
    _Out_ LPBY_HANDLE_FILE_INFORMATION lpFileInformation
    );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Desktop Family or OneCore Family or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

WINBASEAPI
DWORD
WINAPI
GetFileSize(
    _In_ HANDLE hFile,
    _Out_opt_ LPDWORD lpFileSizeHigh
    );

#endif // WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)
#pragma endregion

#pragma region Application Family or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

WINBASEAPI
BOOL
WINAPI
GetFileSizeEx(
    _In_ HANDLE hFile,
    _Out_ PLARGE_INTEGER lpFileSize
    );

WINBASEAPI
DWORD
WINAPI
GetFileType(
    _In_ HANDLE hFile
    );

#if (_WIN32_WINNT >= 0x0600)

WINBASEAPI
DWORD
WINAPI
GetFinalPathNameByHandleA(
    _In_ HANDLE hFile,
    _Out_writes_(cchFilePath) LPSTR lpszFilePath,
    _In_ DWORD cchFilePath,
    _In_ DWORD dwFlags
    );

WINBASEAPI
DWORD
WINAPI
GetFinalPathNameByHandleW(
    _In_ HANDLE hFile,
    _Out_writes_(cchFilePath) LPWSTR lpszFilePath,
    _In_ DWORD cchFilePath,
    _In_ DWORD dwFlags
    );

#ifdef UNICODE
#define GetFinalPathNameByHandle  GetFinalPathNameByHandleW
#else
#define GetFinalPathNameByHandle  GetFinalPathNameByHandleA
#endif // !UNICODE

#endif // (_WIN32_WINNT >= 0x0600)

WINBASEAPI
BOOL
WINAPI
GetFileTime(
    _In_ HANDLE hFile,
    _Out_opt_ LPFILETIME lpCreationTime,
    _Out_opt_ LPFILETIME lpLastAccessTime,
    _Out_opt_ LPFILETIME lpLastWriteTime
    );

WINBASEAPI
_Success_(return != 0 && return < nBufferLength)
DWORD
WINAPI
GetFullPathNameW(
    _In_ LPCWSTR lpFileName,
    _In_ DWORD nBufferLength,
    _Out_writes_to_opt_(nBufferLength,return + 1) LPWSTR lpBuffer,
    _Outptr_opt_ LPWSTR* lpFilePart
    );

#ifdef UNICODE
#define GetFullPathName  GetFullPathNameW
#endif

WINBASEAPI
_Success_(return != 0 && return < nBufferLength)
DWORD
WINAPI
GetFullPathNameA(
    _In_ LPCSTR lpFileName,
    _In_ DWORD nBufferLength,
    _Out_writes_to_opt_(nBufferLength,return + 1) LPSTR lpBuffer,
    _Outptr_opt_ LPSTR* lpFilePart
    );

#ifndef UNICODE
#define GetFullPathName GetFullPathNameA
#endif

WINBASEAPI
DWORD
WINAPI
GetLogicalDrives(
    VOID
    );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

WINBASEAPI
DWORD
WINAPI
GetLogicalDriveStringsW(
    _In_ DWORD nBufferLength,
    _Out_writes_to_opt_(nBufferLength,return + 1) LPWSTR lpBuffer
    );

#ifdef UNICODE
#define GetLogicalDriveStrings  GetLogicalDriveStringsW
#endif

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#pragma region Application Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)

WINBASEAPI
_Success_(return != 0 && return < cchBuffer)
DWORD
WINAPI
GetLongPathNameA(
    _In_ LPCSTR lpszShortPath,
    _Out_writes_to_opt_(cchBuffer,return + 1) LPSTR lpszLongPath,
    _In_ DWORD cchBuffer
    );

#ifndef UNICODE
#define GetLongPathName GetLongPathNameA
#endif

WINBASEAPI
_Success_(return != 0 && return < cchBuffer)
DWORD
WINAPI
GetLongPathNameW(
    _In_ LPCWSTR lpszShortPath,
    _Out_writes_to_opt_(cchBuffer,return + 1) LPWSTR lpszLongPath,
    _In_ DWORD cchBuffer
    );

#ifdef UNICODE
#define GetLongPathName GetLongPathNameW
#endif

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

#if (NTDDI_VERSION >= NTDDI_WIN10_FE)

WINBASEAPI
BOOL
WINAPI
AreShortNamesEnabled(
    _In_ HANDLE Handle,
    _Out_ BOOL* Enabled
    );

#endif // (NTDDI_VERSION >= NTDDI_WIN10_FE)

WINBASEAPI
_Success_(return != 0 && return < cchBuffer)
DWORD
WINAPI
GetShortPathNameW(
    _In_ LPCWSTR lpszLongPath,
    _Out_writes_to_opt_(cchBuffer,return + 1) LPWSTR lpszShortPath,
    _In_ DWORD cchBuffer
    );

#ifdef UNICODE
#define GetShortPathName  GetShortPathNameW
#endif

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#pragma region Application Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)

WINBASEAPI
UINT
WINAPI
GetTempFileNameW(
    _In_ LPCWSTR lpPathName,
    _In_ LPCWSTR lpPrefixString,
    _In_ UINT uUnique,
    _Out_writes_(MAX_PATH) LPWSTR lpTempFileName
    );

#ifdef UNICODE
#define GetTempFileName  GetTempFileNameW
#endif

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#pragma region Application Family or OneCore Family or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

#if (_WIN32_WINNT >= 0x0600)

WINBASEAPI
BOOL
WINAPI
GetVolumeInformationByHandleW(
    _In_ HANDLE hFile,
    _Out_writes_opt_(nVolumeNameSize) LPWSTR lpVolumeNameBuffer,
    _In_ DWORD nVolumeNameSize,
    _Out_opt_ LPDWORD lpVolumeSerialNumber,
    _Out_opt_ LPDWORD lpMaximumComponentLength,
    _Out_opt_ LPDWORD lpFileSystemFlags,
    _Out_writes_opt_(nFileSystemNameSize) LPWSTR lpFileSystemNameBuffer,
    _In_ DWORD nFileSystemNameSize
    );

#endif /* _WIN32_WINNT >=  0x0600 */

WINBASEAPI
BOOL
WINAPI
GetVolumeInformationW(
    _In_opt_ LPCWSTR lpRootPathName,
    _Out_writes_opt_(nVolumeNameSize) LPWSTR lpVolumeNameBuffer,
    _In_ DWORD nVolumeNameSize,
    _Out_opt_ LPDWORD lpVolumeSerialNumber,
    _Out_opt_ LPDWORD lpMaximumComponentLength,
    _Out_opt_ LPDWORD lpFileSystemFlags,
    _Out_writes_opt_(nFileSystemNameSize) LPWSTR lpFileSystemNameBuffer,
    _In_ DWORD nFileSystemNameSize
    );

#ifdef UNICODE
#define GetVolumeInformation  GetVolumeInformationW
#endif

WINBASEAPI
BOOL
WINAPI
GetVolumePathNameW(
    _In_ LPCWSTR lpszFileName,
    _Out_writes_(cchBufferLength) LPWSTR lpszVolumePathName,
    _In_ DWORD cchBufferLength
    );

#ifdef UNICODE
#define GetVolumePathName  GetVolumePathNameW
#endif

WINBASEAPI
BOOL
WINAPI
LocalFileTimeToFileTime(
    _In_ CONST FILETIME* lpLocalFileTime,
    _Out_ LPFILETIME lpFileTime
    );

WINBASEAPI
BOOL
WINAPI
LockFile(
    _In_ HANDLE hFile,
    _In_ DWORD dwFileOffsetLow,
    _In_ DWORD dwFileOffsetHigh,
    _In_ DWORD nNumberOfBytesToLockLow,
    _In_ DWORD nNumberOfBytesToLockHigh
    );

WINBASEAPI
BOOL
WINAPI
LockFileEx(
    _In_ HANDLE hFile,
    _In_ DWORD dwFlags,
    _Reserved_ DWORD dwReserved,
    _In_ DWORD nNumberOfBytesToLockLow,
    _In_ DWORD nNumberOfBytesToLockHigh,
    _Inout_ LPOVERLAPPED lpOverlapped
    );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

WINBASEAPI
DWORD
WINAPI
QueryDosDeviceW(
    _In_opt_ LPCWSTR lpDeviceName,
    _Out_writes_to_opt_(ucchMax,return) LPWSTR lpTargetPath,
    _In_ DWORD ucchMax
    );

#ifdef UNICODE
#define QueryDosDevice  QueryDosDeviceW
#endif

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#pragma region Application Family or OneCore Family or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

WINBASEAPI
_Must_inspect_result_
BOOL
WINAPI
ReadFile(
    _In_ HANDLE hFile,
    _Out_writes_bytes_to_opt_(nNumberOfBytesToRead, *lpNumberOfBytesRead) __out_data_source(FILE) LPVOID lpBuffer,
    _In_ DWORD nNumberOfBytesToRead,
    _Out_opt_ LPDWORD lpNumberOfBytesRead,
    _Inout_opt_ LPOVERLAPPED lpOverlapped
    );

WINBASEAPI
_Must_inspect_result_
BOOL
WINAPI
ReadFileEx(
    _In_ HANDLE hFile,
    _Out_writes_bytes_opt_(nNumberOfBytesToRead) __out_data_source(FILE) LPVOID lpBuffer,
    _In_ DWORD nNumberOfBytesToRead,
    _Inout_ LPOVERLAPPED lpOverlapped,
    _In_ LPOVERLAPPED_COMPLETION_ROUTINE lpCompletionRoutine
    );

WINBASEAPI
_Must_inspect_result_
BOOL
WINAPI
ReadFileScatter(
    _In_ HANDLE hFile,
    _In_ FILE_SEGMENT_ELEMENT aSegmentArray[],
    _In_ DWORD nNumberOfBytesToRead,
    _Reserved_ LPDWORD lpReserved,
    _Inout_ LPOVERLAPPED lpOverlapped
    );

WINBASEAPI
BOOL
WINAPI
RemoveDirectoryA(
    _In_ LPCSTR lpPathName
    );

WINBASEAPI
BOOL
WINAPI
RemoveDirectoryW(
    _In_ LPCWSTR lpPathName
    );

#ifdef UNICODE
#define RemoveDirectory  RemoveDirectoryW
#else
#define RemoveDirectory  RemoveDirectoryA
#endif // !UNICODE

WINBASEAPI
BOOL
WINAPI
SetEndOfFile(
    _In_ HANDLE hFile
    );

WINBASEAPI
BOOL
WINAPI
SetFileAttributesA(
    _In_ LPCSTR lpFileName,
    _In_ DWORD dwFileAttributes
    );

WINBASEAPI
BOOL
WINAPI
SetFileAttributesW(
    _In_ LPCWSTR lpFileName,
    _In_ DWORD dwFileAttributes
    );

#ifdef UNICODE
#define SetFileAttributes  SetFileAttributesW
#else
#define SetFileAttributes  SetFileAttributesA
#endif // !UNICODE

#if (_WIN32_WINNT >= 0x0600)

WINBASEAPI
BOOL
WINAPI
SetFileInformationByHandle(
    _In_ HANDLE hFile,
    _In_ FILE_INFO_BY_HANDLE_CLASS FileInformationClass,
    _In_reads_bytes_(dwBufferSize) LPVOID lpFileInformation,
    _In_ DWORD dwBufferSize
    );

#endif

WINBASEAPI
DWORD
WINAPI
SetFilePointer(
    _In_ HANDLE hFile,
    _In_ LONG lDistanceToMove,
    _Inout_opt_ PLONG lpDistanceToMoveHigh,
    _In_ DWORD dwMoveMethod
    );

WINBASEAPI
BOOL
WINAPI
SetFilePointerEx(
    _In_ HANDLE hFile,
    _In_ LARGE_INTEGER liDistanceToMove,
    _Out_opt_ PLARGE_INTEGER lpNewFilePointer,
    _In_ DWORD dwMoveMethod
    );

WINBASEAPI
BOOL
WINAPI
SetFileTime(
    _In_ HANDLE hFile,
    _In_opt_ CONST FILETIME* lpCreationTime,
    _In_opt_ CONST FILETIME* lpLastAccessTime,
    _In_opt_ CONST FILETIME* lpLastWriteTime
    );

#if _WIN32_WINNT >= 0x0501

WINBASEAPI
BOOL
WINAPI
SetFileValidData(
    _In_ HANDLE hFile,
    _In_ LONGLONG ValidDataLength
    );

#endif // (_WIN32_WINNT >= 0x0501)

WINBASEAPI
BOOL
WINAPI
UnlockFile(
    _In_ HANDLE hFile,
    _In_ DWORD dwFileOffsetLow,
    _In_ DWORD dwFileOffsetHigh,
    _In_ DWORD nNumberOfBytesToUnlockLow,
    _In_ DWORD nNumberOfBytesToUnlockHigh
    );

WINBASEAPI
BOOL
WINAPI
UnlockFileEx(
    _In_ HANDLE hFile,
    _Reserved_ DWORD dwReserved,
    _In_ DWORD nNumberOfBytesToUnlockLow,
    _In_ DWORD nNumberOfBytesToUnlockHigh,
    _Inout_ LPOVERLAPPED lpOverlapped
    );

WINBASEAPI
BOOL
WINAPI
WriteFile(
    _In_ HANDLE hFile,
    _In_reads_bytes_opt_(nNumberOfBytesToWrite) LPCVOID lpBuffer,
    _In_ DWORD nNumberOfBytesToWrite,
    _Out_opt_ LPDWORD lpNumberOfBytesWritten,
    _Inout_opt_ LPOVERLAPPED lpOverlapped
    );

WINBASEAPI
BOOL
WINAPI
WriteFileEx(
    _In_ HANDLE hFile,
    _In_reads_bytes_opt_(nNumberOfBytesToWrite) LPCVOID lpBuffer,
    _In_ DWORD nNumberOfBytesToWrite,
    _Inout_ LPOVERLAPPED lpOverlapped,
    _In_ LPOVERLAPPED_COMPLETION_ROUTINE lpCompletionRoutine
    );

WINBASEAPI
BOOL
WINAPI
WriteFileGather(
    _In_ HANDLE hFile,
    _In_ FILE_SEGMENT_ELEMENT aSegmentArray[],
    _In_ DWORD nNumberOfBytesToWrite,
    _Reserved_ LPDWORD lpReserved,
    _Inout_ LPOVERLAPPED lpOverlapped
    );

WINBASEAPI
DWORD
WINAPI
GetTempPathW(
    _In_ DWORD nBufferLength,
    _Out_writes_to_opt_(nBufferLength,return + 1) LPWSTR lpBuffer
    );

#ifdef UNICODE
#define GetTempPath  GetTempPathW
#endif

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Application Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)

WINBASEAPI
BOOL
WINAPI
GetVolumeNameForVolumeMountPointW(
    _In_ LPCWSTR lpszVolumeMountPoint,
    _Out_writes_(cchBufferLength) LPWSTR lpszVolumeName,
    _In_ DWORD cchBufferLength
    );

#ifdef UNICODE
#define GetVolumeNameForVolumeMountPoint  GetVolumeNameForVolumeMountPointW
#endif

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

#if (_WIN32_WINNT >= 0x0501)

WINBASEAPI
BOOL
WINAPI
GetVolumePathNamesForVolumeNameW(
    _In_ LPCWSTR lpszVolumeName,
    _Out_writes_to_opt_(cchBufferLength,*lpcchReturnLength) _Post_ _NullNull_terminated_ LPWCH lpszVolumePathNames,
    _In_ DWORD cchBufferLength,
    _Out_ PDWORD lpcchReturnLength
    );

#ifdef UNICODE
#define GetVolumePathNamesForVolumeName  GetVolumePathNamesForVolumeNameW
#endif

#endif // _WIN32_WINNT >= 0x0501

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#pragma region Application Family or OneCore Family or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

#if (_WIN32_WINNT >= 0x0602)

typedef struct _CREATEFILE2_EXTENDED_PARAMETERS {
    DWORD dwSize;
    DWORD dwFileAttributes;
    DWORD dwFileFlags;
    DWORD dwSecurityQosFlags;
    LPSECURITY_ATTRIBUTES lpSecurityAttributes;
    HANDLE hTemplateFile;
} CREATEFILE2_EXTENDED_PARAMETERS, *PCREATEFILE2_EXTENDED_PARAMETERS, *LPCREATEFILE2_EXTENDED_PARAMETERS;

WINBASEAPI
HANDLE
WINAPI
CreateFile2(
    _In_ LPCWSTR lpFileName,
    _In_ DWORD dwDesiredAccess,
    _In_ DWORD dwShareMode,
    _In_ DWORD dwCreationDisposition,
    _In_opt_ LPCREATEFILE2_EXTENDED_PARAMETERS pCreateExParams
    );

#endif // _WIN32_WINNT >= 0x0602

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

#if (_WIN32_WINNT >= 0x0600)

WINBASEAPI
BOOL
WINAPI
SetFileIoOverlappedRange(
    _In_ HANDLE FileHandle,
    _In_ PUCHAR OverlappedRangeStart,
    _In_ ULONG Length
    );

#endif // _WIN32_WINNT >= 0x0600

#if _WIN32_WINNT >= 0x0501

WINBASEAPI
DWORD
WINAPI
GetCompressedFileSizeA(
    _In_ LPCSTR lpFileName,
    _Out_opt_ LPDWORD lpFileSizeHigh
    );

WINBASEAPI
DWORD
WINAPI
GetCompressedFileSizeW(
    _In_ LPCWSTR lpFileName,
    _Out_opt_ LPDWORD lpFileSizeHigh
    );

#ifdef UNICODE
#define GetCompressedFileSize  GetCompressedFileSizeW
#else
#define GetCompressedFileSize  GetCompressedFileSizeA
#endif // !UNICODE

#endif // _WIN32_WINNT >= 0x0501

#if (_WIN32_WINNT >= 0x0501)

typedef enum _STREAM_INFO_LEVELS {

    FindStreamInfoStandard,
    FindStreamInfoMaxInfoLevel

} STREAM_INFO_LEVELS;

typedef struct _WIN32_FIND_STREAM_DATA {

    LARGE_INTEGER StreamSize;
    WCHAR cStreamName[ MAX_PATH + 36 ];

} WIN32_FIND_STREAM_DATA, *PWIN32_FIND_STREAM_DATA;

WINBASEAPI
HANDLE
WINAPI
FindFirstStreamW(
    _In_ LPCWSTR lpFileName,
    _In_ STREAM_INFO_LEVELS InfoLevel,
    _Out_writes_bytes_(sizeof(WIN32_FIND_STREAM_DATA)) LPVOID lpFindStreamData,
    _Reserved_ DWORD dwFlags
    );

WINBASEAPI
BOOL
APIENTRY
FindNextStreamW(
    _In_ HANDLE hFindStream,
    _Out_writes_bytes_(sizeof(WIN32_FIND_STREAM_DATA)) LPVOID lpFindStreamData
    );

#endif // (_WIN32_WINNT >= 0x0501)

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#pragma region Desktop Family or OneCore Family or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

WINBASEAPI
BOOL
WINAPI
AreFileApisANSI(
    VOID
    );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Application Family or OneCore Family or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

WINBASEAPI
DWORD
WINAPI
GetTempPathA(
    _In_ DWORD nBufferLength,
    _Out_writes_to_opt_(nBufferLength,return + 1) LPSTR lpBuffer
    );

#ifndef UNICODE
#define GetTempPath  GetTempPathA
#endif

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

#if _WIN32_WINNT >= 0x0600

WINBASEAPI
HANDLE
WINAPI
FindFirstFileNameW(
    _In_ LPCWSTR lpFileName,
    _In_ DWORD dwFlags,
    _Inout_ LPDWORD StringLength,
    _Out_writes_(*StringLength) PWSTR LinkName
    );

WINBASEAPI
BOOL
APIENTRY
FindNextFileNameW(
    _In_ HANDLE hFindStream,
    _Inout_ LPDWORD StringLength,
    _Out_writes_(*StringLength) PWSTR LinkName
    );

#endif // (_WIN32_WINNT >= 0x0600)

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#pragma region Application Family or OneCore Family or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

WINBASEAPI
BOOL
WINAPI
GetVolumeInformationA(
    _In_opt_ LPCSTR lpRootPathName,
    _Out_writes_opt_(nVolumeNameSize) LPSTR lpVolumeNameBuffer,
    _In_ DWORD nVolumeNameSize,
    _Out_opt_ LPDWORD lpVolumeSerialNumber,
    _Out_opt_ LPDWORD lpMaximumComponentLength,
    _Out_opt_ LPDWORD lpFileSystemFlags,
    _Out_writes_opt_(nFileSystemNameSize) LPSTR lpFileSystemNameBuffer,
    _In_ DWORD nFileSystemNameSize
    );

#ifndef UNICODE
#define GetVolumeInformation  GetVolumeInformationA
#endif

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Application Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)

WINBASEAPI
UINT
WINAPI
GetTempFileNameA(
    _In_ LPCSTR lpPathName,
    _In_ LPCSTR lpPrefixString,
    _In_ UINT uUnique,
    _Out_writes_(MAX_PATH) LPSTR lpTempFileName
    );

#ifndef UNICODE
#define GetTempFileName  GetTempFileNameA
#endif
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

WINBASEAPI
VOID
WINAPI
SetFileApisToOEM(
    VOID
    );

WINBASEAPI
VOID
WINAPI
SetFileApisToANSI(
    VOID
    );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#pragma region Application Family or OneCore Family or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

#if (NTDDI_VERSION >= NTDDI_WIN10_RS5)
WINBASEAPI
_Success_(return > 0 && return < BufferLength)
DWORD
WINAPI
GetTempPath2W(
    _In_ DWORD BufferLength,
    _Out_writes_to_opt_(BufferLength,return + 1) LPWSTR Buffer
    );

#ifdef UNICODE
#define GetTempPath2  GetTempPath2W
#endif

WINBASEAPI
_Success_(return > 0 && return < BufferLength)
DWORD
WINAPI
GetTempPath2A(
    _In_ DWORD BufferLength,
    _Out_writes_to_opt_(BufferLength,return + 1) LPSTR Buffer
    );

#ifndef UNICODE
#define GetTempPath2  GetTempPath2A
#endif

#endif // (NTDDI_VERSION >= NTDDI_WIN10_RS5)

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

typedef struct _CREATEFILE3_EXTENDED_PARAMETERS {
    DWORD dwSize;
    DWORD dwFileAttributes;
    DWORD dwFileFlags;
    DWORD dwSecurityQosFlags;
    LPSECURITY_ATTRIBUTES lpSecurityAttributes;
    HANDLE hTemplateFile;
} CREATEFILE3_EXTENDED_PARAMETERS, *PCREATEFILE3_EXTENDED_PARAMETERS, *LPCREATEFILE3_EXTENDED_PARAMETERS;

typedef enum DIRECTORY_FLAGS
{
    DIRECTORY_FLAGS_NONE = 0,
    DIRECTORY_FLAGS_DISALLOW_PATH_REDIRECTS = 0x000000001,
} DIRECTORY_FLAGS;
DEFINE_ENUM_FLAG_OPERATORS(DIRECTORY_FLAGS)

#pragma region Application Family or OneCore Family or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

#if defined(NTDDI_WIN11_GE) && (NTDDI_VERSION >= NTDDI_WIN11_GE)

WINBASEAPI
HANDLE
WINAPI
CreateFile3(
    _In_z_ LPCWSTR lpFileName,
    _In_ DWORD dwDesiredAccess,
    _In_ DWORD dwShareMode,
    _In_ DWORD dwCreationDisposition,
    _In_opt_ LPCREATEFILE3_EXTENDED_PARAMETERS pCreateExParams
    );

WINBASEAPI
HANDLE
WINAPI
CreateDirectory2A(
    _In_z_ LPCSTR lpPathName,
    _In_ DWORD dwDesiredAccess,
    _In_ DWORD dwShareMode,
    _In_ DIRECTORY_FLAGS DirectoryFlags,
    _In_opt_ LPSECURITY_ATTRIBUTES lpSecurityAttributes
    );

WINBASEAPI
HANDLE
WINAPI
CreateDirectory2W(
    _In_z_ LPCWSTR lpPathName,
    _In_ DWORD dwDesiredAccess,
    _In_ DWORD dwShareMode,
    _In_ DIRECTORY_FLAGS DirectoryFlags,
    _In_opt_ LPSECURITY_ATTRIBUTES lpSecurityAttributes
    );

#ifdef UNICODE
#define CreateDirectory2  CreateDirectory2W
#else
#define CreateDirectory2  CreateDirectory2A
#endif // !UNICODE

WINBASEAPI
BOOL
WINAPI
RemoveDirectory2A(
    _In_z_ LPCSTR lpPathName,
    _In_ DIRECTORY_FLAGS DirectoryFlags
    );

WINBASEAPI
BOOL
WINAPI
RemoveDirectory2W(
    _In_z_ LPCWSTR lpPathName,
    _In_ DIRECTORY_FLAGS DirectoryFlags
    );

#ifdef UNICODE
#define RemoveDirectory2  RemoveDirectory2W
#else
#define RemoveDirectory2  RemoveDirectory2A
#endif // !UNICODE

WINBASEAPI
BOOL
WINAPI
DeleteFile2A(
    _In_z_ LPCSTR lpFileName,
    _In_ DWORD Flags
    );

WINBASEAPI
BOOL
WINAPI
DeleteFile2W(
    _In_z_ LPCWSTR lpFileName,
    _In_ DWORD Flags
    );

#ifdef UNICODE
#define DeleteFile2  DeleteFile2W
#else
#define DeleteFile2  DeleteFile2A
#endif // !UNICODE

#endif // (NTDDI_VERSION >= NTDDI_WIN11_GE)

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

#ifdef __cplusplus
}
#endif

#endif // _APISETFILE_
