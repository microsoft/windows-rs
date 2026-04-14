/*++
Copyright (c) Microsoft Corporation.  All rights reserved.

Module Name:
    loadperf.h

Abstract:
    Header file for the Performance Monitor counter string installation
    and removal functions.

Revision History
    16-Nov-95   Created (a-robw)
--*/

#ifndef _LOADPERF_H_
#define _LOADPERF_H_

#if _MSC_VER > 1000
#pragma once
#endif
#include <winapifamily.h>

#pragma region Desktop Family or WinMgmt Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_WINMGMT)


// function prototypes for perf counter name string load & unload functions
// provided in LOADPERF.DLL

#ifdef __cplusplus
extern "C" {
#endif

#ifdef __LOADPERF__
#define LOADPERF_FUNCTION   DWORD __stdcall
#else
#define LOADPERF_FUNCTION   __declspec(dllimport) DWORD __stdcall
#endif

// Values for dwFlags Argument of InstallPerfDll
#define LOADPERF_FLAGS_DELETE_MOF_ON_EXIT   ((ULONG_PTR) 1) // Has no effect.
#define LOADPERF_FLAGS_LOAD_REGISTRY_ONLY   ((ULONG_PTR) 2) // Has no effect.
#define LOADPERF_FLAGS_CREATE_MOF_ONLY      ((ULONG_PTR) 4) // Has no effect.
#define LOADPERF_FLAGS_DISPLAY_USER_MSGS    ((ULONG_PTR) 8) // Logs to stdout.

LOADPERF_FUNCTION
InstallPerfDllW(
    _In_opt_ LPCWSTR   szComputerName,
    _In_     LPCWSTR   lpIniFile,
    _In_     ULONG_PTR dwFlags         
);

LOADPERF_FUNCTION
InstallPerfDllA(
    _In_opt_ LPCSTR    szComputerName,
    _In_     LPCSTR    lpIniFile,
    _In_     ULONG_PTR dwFlags         
);

LOADPERF_FUNCTION
LoadPerfCounterTextStringsA(
    _In_ LPSTR     lpCommandLine,
    _In_ BOOL      bQuietModeArg
);

LOADPERF_FUNCTION
LoadPerfCounterTextStringsW(
    _In_ LPWSTR    lpCommandLine,
    _In_ BOOL      bQuietModeArg
);

LOADPERF_FUNCTION
UnloadPerfCounterTextStringsW(
    _In_ LPWSTR    lpCommandLine,
    _In_ BOOL      bQuietModeArg
);

LOADPERF_FUNCTION
UnloadPerfCounterTextStringsA(
    _In_ LPSTR     lpCommandLine,
    _In_ BOOL      bQuietModeArg
);

LOADPERF_FUNCTION
UpdatePerfNameFilesA(
    _In_     LPCSTR    szNewCtrFilePath,
    _In_opt_ LPCSTR    szNewHlpFilePath,
    _In_     LPSTR     szLanguageID,
    _In_     ULONG_PTR dwModes
);

LOADPERF_FUNCTION
UpdatePerfNameFilesW(
    _In_     LPCWSTR   szNewCtrFilePath,
    _In_opt_ LPCWSTR   szNewHlpFilePath,
    _In_     LPWSTR    szLanguageID,
    _In_     ULONG_PTR dwModes
);

LOADPERF_FUNCTION
SetServiceAsTrustedA(
    _In_opt_ LPCSTR    szReserved,
    _In_     LPCSTR    szServiceName
);

LOADPERF_FUNCTION
SetServiceAsTrustedW(
    _In_opt_ LPCWSTR   szReserved,
    _In_     LPCWSTR   szServiceName
);

LOADPERF_FUNCTION
BackupPerfRegistryToFileW(
    _In_     LPCWSTR   szFileName,
    _In_opt_ LPCWSTR   szCommentString
);

LOADPERF_FUNCTION
RestorePerfRegistryFromFileW(
    _In_opt_ LPCWSTR   szFileName,
    _In_opt_ LPCWSTR   szLangId
);

#ifdef UNICODE
#define InstallPerfDll                  InstallPerfDllW
#define LoadPerfCounterTextStrings      LoadPerfCounterTextStringsW
#define UnloadPerfCounterTextStrings    UnloadPerfCounterTextStringsW
#define UpdatePerfNameFiles             UpdatePerfNameFilesW 
#define SetServiceAsTrusted             SetServiceAsTrustedW
#else
#define InstallPerfDll                  InstallPerfDllA
#define LoadPerfCounterTextStrings      LoadPerfCounterTextStringsA
#define UnloadPerfCounterTextStrings    UnloadPerfCounterTextStringsA
#define UpdatePerfNameFiles             UpdatePerfNameFilesA
#define SetServiceAsTrusted             SetServiceAsTrustedA
#endif

#ifdef __cplusplus
}
#endif



#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_WINMGMT) */
#pragma endregion

#endif // _LOADPERF_H_
