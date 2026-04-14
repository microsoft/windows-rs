/************************************************************************
*                                                                       *
*   WOFAPI.H                                                            *
*                                                                       *
*   Contains structures and routines for manipulating WOF-backed        *
*   files and file overlays                                             *
*                                                                       *
*   Copyright (c) Microsoft Corp. All rights reserved.                  *
*                                                                       *
************************************************************************/

#ifndef WOFAPI_H
#define WOFAPI_H

#if _MSC_VER > 1000
#pragma once
#endif

#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

#if (_WIN32_WINNT >= _WIN32_WINNT_WIN7)

#ifdef __cplusplus
extern "C" {            /* Assume C declarations for C++ */
#endif  /* __cplusplus */

BOOL
_Success_(return)
WINAPI
WofShouldCompressBinaries (
    __in LPCWSTR Volume,
    __out PULONG Algorithm
    );

HRESULT
WINAPI
WofGetDriverVersion (
    __in HANDLE FileOrVolumeHandle,
    __in ULONG Provider,
    __out PULONG WofVersion
    );

//
//  Type of ExternalFileInfo is provider-specific.  For
//  WOF_PROVIDER_WIM, will be PWIM_EXTERNAL_FILE_INFO; for
//  WOF_PROVIDER_FILE, will be PWOF_FILE_COMPRESSION_INFO.
//

HRESULT
WINAPI
WofSetFileDataLocation (
    __in HANDLE FileHandle,
    __in ULONG Provider,
    __in PVOID ExternalFileInfo,
    __in ULONG Length
    );

//
//  Type of ExternalFileInfo is provider-specific.  For
//  WOF_PROVIDER_WIM, will be PWIM_EXTERNAL_FILE_INFO; for
//  WOF_PROVIDER_FILE, will be PWOF_FILE_COMPRESSION_INFO.
//

HRESULT
WINAPI
WofIsExternalFile (
    __in LPCWSTR FilePath,
    __out_opt PBOOL IsExternalFile,
    __out_opt PULONG Provider,
    __out_opt PVOID ExternalFileInfo,
    __inout_opt PULONG BufferLength
    );

//
//  Type of EntryInfo is provider-specific.  For WOF_PROVIDER_WIM,
//  will be PWIM_ENTRY_INFO.
//

typedef 
BOOL (CALLBACK * WofEnumEntryProc) (
    __in const PVOID EntryInfo,
    __in_opt PVOID UserData
    );

//
//  Type of ExternalFileInfo is provider-specific.  For
//  WOF_PROVIDER_WIM, will be PWIM_EXTERNAL_FILE_INFO.
//

typedef
BOOL (CALLBACK * WofEnumFilesProc) (
    __in PCWSTR FilePath,
    __in PVOID ExternalFileInfo,
    __in_opt PVOID UserData
    );

HRESULT
WINAPI
WofEnumEntries (
    __in PCWSTR VolumeName,
    __in ULONG Provider,
    __in WofEnumEntryProc EnumProc,
    __in_opt PVOID UserData
    );

//
//  List of providers supported by WOF
//

#define WOF_PROVIDER_WIM    (0x00000001)
#define WOF_PROVIDER_FILE   (0x00000002)

//  *************************************************************************
//
//     WOF Provider specific APIs and structures
//
//  *************************************************************************

#define WIM_PROVIDER_HASH_SIZE 20

//
//  WimType specifies a set of flags corresponding to the type of WIM.
//
//  WIM_BOOT_OS_WIM means the wim contains Windows system files.
//  WIM_BOOT_NOT_OS_WIM means the wim contains 3rd party files or data files.
//
//  Do not use WIM_BOOT_OS_WIM for data that should be preserved on
//  system upgrade.
//

#define WIM_BOOT_OS_WIM       (0x00000001)
#define WIM_BOOT_NOT_OS_WIM   (0x00000000)

//
//  Flags for WIM_ENTRY_INFO and WIM_EXTERNAL_FILE_INFO correspond
//  to the state of the WIM.
//

#define WIM_ENTRY_FLAG_NOT_ACTIVE    (0x00000001)
#define WIM_ENTRY_FLAG_SUSPENDED     (0x00000002)

#define WIM_EXTERNAL_FILE_INFO_FLAG_NOT_ACTIVE    WIM_ENTRY_FLAG_NOT_ACTIVE
#define WIM_EXTERNAL_FILE_INFO_FLAG_SUSPENDED     WIM_ENTRY_FLAG_SUSPENDED

typedef struct _WIM_ENTRY_INFO {
    DWORD WimEntryInfoSize;
    DWORD WimType;
    LARGE_INTEGER DataSourceId;
    GUID WimGuid;
    PCWSTR WimPath;
    DWORD WimIndex;
    DWORD Flags;
} WIM_ENTRY_INFO, *PWIM_ENTRY_INFO;

typedef struct _WIM_EXTERNAL_FILE_INFO {
    LARGE_INTEGER DataSourceId;
    UCHAR ResourceHash[WIM_PROVIDER_HASH_SIZE];
    ULONG Flags;
} WIM_EXTERNAL_FILE_INFO, *PWIM_EXTERNAL_FILE_INFO;

HRESULT
WINAPI
WofWimAddEntry (
    __in PCWSTR VolumeName,
    __in PCWSTR WimPath,
    __in DWORD WimType,
    __in DWORD WimIndex,
    __out PLARGE_INTEGER DataSourceId
    );

HRESULT
WINAPI
WofWimEnumFiles (
    __in PCWSTR VolumeName,
    __in LARGE_INTEGER DataSourceId,
    __in WofEnumFilesProc EnumProc,
    __in_opt PVOID UserData
    );

HRESULT
WINAPI
WofWimSuspendEntry (
    __in PCWSTR VolumeName,
    __in LARGE_INTEGER DataSourceId
    );

HRESULT
WINAPI
WofWimRemoveEntry (
    __in PCWSTR VolumeName,
    __in LARGE_INTEGER DataSourceId
    );

HRESULT
WINAPI
WofWimUpdateEntry (
    __in PCWSTR VolumeName,
    __in LARGE_INTEGER DataSourceId,
    __in PCWSTR NewWimPath
    );

#define FILE_PROVIDER_COMPRESSION_XPRESS4K   (0x00000000)
#define FILE_PROVIDER_COMPRESSION_LZX        (0x00000001)
#define FILE_PROVIDER_COMPRESSION_XPRESS8K   (0x00000002)
#define FILE_PROVIDER_COMPRESSION_XPRESS16K  (0x00000003)

typedef struct _WOF_FILE_COMPRESSION_INFO_V0 {
    ULONG Algorithm;
} WOF_FILE_COMPRESSION_INFO_V0, *PWOF_FILE_COMPRESSION_INFO_V0;

typedef struct _WOF_FILE_COMPRESSION_INFO_V1 {
    ULONG Algorithm;
    ULONG Flags;
} WOF_FILE_COMPRESSION_INFO_V1, *PWOF_FILE_COMPRESSION_INFO_V1;

typedef WOF_FILE_COMPRESSION_INFO_V1 WOF_FILE_COMPRESSION_INFO;
typedef PWOF_FILE_COMPRESSION_INFO_V1 PWOF_FILE_COMPRESSION_INFO;

HRESULT
WINAPI
WofFileEnumFiles (
    __in PCWSTR VolumeName,
    __in ULONG Algorithm,
    __in WofEnumFilesProc EnumProc,
    __in_opt PVOID UserData
    );

#ifdef __cplusplus
}
#endif  /* __cplusplus */

#endif /* _WIN32_WINNT >= _WIN32_WINNT_WIN7 */
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#endif /* WOFAPI_H */

