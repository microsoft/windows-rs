#include <winapifamily.h>

/*****************************************************************************\
*                                                                             *
* winver.h -    Version management functions, types, and definitions          *
*                                                                             *
*               Include file for VER.DLL.  This library is                    *
*               designed to allow version stamping of Windows executable files*
*               and of special .VER files for DOS executable files.           *
*                                                                             *
*               Copyright (c) Microsoft Corporation. All rights reserved.     *
*                                                                             *
\*****************************************************************************/

#ifndef VER_H
#define VER_H

#include <SpecStrings.h>
#include <verrsrc.h>


#ifdef __cplusplus
extern "C" {
#endif

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

#ifndef RC_INVOKED              /* RC doesn't need to see the rest of this */


/* ----- Function prototypes ----- */

DWORD
APIENTRY
VerFindFileA(
        _In_                         DWORD uFlags,
        _In_                         LPCSTR szFileName,
        _In_opt_                     LPCSTR szWinDir,
        _In_                         LPCSTR szAppDir,
        _Out_writes_(*puCurDirLen)   LPSTR szCurDir,
        _Inout_                      PUINT puCurDirLen,
        _Out_writes_(*puDestDirLen)  LPSTR szDestDir,
        _Inout_                      PUINT puDestDirLen
        );
DWORD
APIENTRY
VerFindFileW(
        _In_                         DWORD uFlags,
        _In_                         LPCWSTR szFileName,
        _In_opt_                     LPCWSTR szWinDir,
        _In_                         LPCWSTR szAppDir,
        _Out_writes_(*puCurDirLen)   LPWSTR szCurDir,
        _Inout_                      PUINT puCurDirLen,
        _Out_writes_(*puDestDirLen)  LPWSTR szDestDir,
        _Inout_                      PUINT puDestDirLen
        );
#ifdef UNICODE
#define VerFindFile  VerFindFileW
#else
#define VerFindFile  VerFindFileA
#endif // !UNICODE

#endif // RC_INVOKED

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

#ifndef RC_INVOKED

DWORD
APIENTRY
VerInstallFileA(
        _In_                         DWORD uFlags,
        _In_                         LPCSTR szSrcFileName,
        _In_                         LPCSTR szDestFileName,
        _In_                         LPCSTR szSrcDir,
        _In_                         LPCSTR szDestDir,
        _In_                         LPCSTR szCurDir,
        _Out_writes_(*puTmpFileLen)  LPSTR szTmpFile,
        _Inout_                      PUINT puTmpFileLen
        );
DWORD
APIENTRY
VerInstallFileW(
        _In_                         DWORD uFlags,
        _In_                         LPCWSTR szSrcFileName,
        _In_                         LPCWSTR szDestFileName,
        _In_                         LPCWSTR szSrcDir,
        _In_                         LPCWSTR szDestDir,
        _In_                         LPCWSTR szCurDir,
        _Out_writes_(*puTmpFileLen)  LPWSTR szTmpFile,
        _Inout_                      PUINT puTmpFileLen
        );
#ifdef UNICODE
#define VerInstallFile  VerInstallFileW
#else
#define VerInstallFile  VerInstallFileA
#endif // !UNICODE

#endif // RC_INVOKED

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#pragma region Application Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)

#ifndef RC_INVOKED

/* Returns size of version info in bytes */
DWORD
APIENTRY
GetFileVersionInfoSizeA(
        _In_        LPCSTR lptstrFilename, /* Filename of version stamped file */
        _Out_opt_ LPDWORD lpdwHandle       /* Information for use by GetFileVersionInfo */
        );
/* Returns size of version info in bytes */
DWORD
APIENTRY
GetFileVersionInfoSizeW(
        _In_        LPCWSTR lptstrFilename, /* Filename of version stamped file */
        _Out_opt_ LPDWORD lpdwHandle       /* Information for use by GetFileVersionInfo */
        );
#ifdef UNICODE
#define GetFileVersionInfoSize  GetFileVersionInfoSizeW
#else
#define GetFileVersionInfoSize  GetFileVersionInfoSizeA
#endif // !UNICODE

/* Read version info into buffer */
BOOL
APIENTRY
GetFileVersionInfoA(
        _In_                LPCSTR lptstrFilename, /* Filename of version stamped file */
        _Reserved_          DWORD dwHandle,          /* Information from GetFileVersionSize */
        _In_                DWORD dwLen,             /* Length of buffer for info */
        _Out_writes_bytes_(dwLen) LPVOID lpData            /* Buffer to place the data structure */
        );
/* Read version info into buffer */
BOOL
APIENTRY
GetFileVersionInfoW(
        _In_                LPCWSTR lptstrFilename, /* Filename of version stamped file */
        _Reserved_          DWORD dwHandle,          /* Information from GetFileVersionSize */
        _In_                DWORD dwLen,             /* Length of buffer for info */
        _Out_writes_bytes_(dwLen) LPVOID lpData            /* Buffer to place the data structure */
        );
#ifdef UNICODE
#define GetFileVersionInfo  GetFileVersionInfoW
#else
#define GetFileVersionInfo  GetFileVersionInfoA
#endif // !UNICODE

DWORD APIENTRY GetFileVersionInfoSizeExA(_In_ DWORD dwFlags, _In_ LPCSTR lpwstrFilename, _Out_ LPDWORD lpdwHandle);
DWORD APIENTRY GetFileVersionInfoSizeExW(_In_ DWORD dwFlags, _In_ LPCWSTR lpwstrFilename, _Out_ LPDWORD lpdwHandle);
#ifdef UNICODE
#define GetFileVersionInfoSizeEx  GetFileVersionInfoSizeExW
#else
#define GetFileVersionInfoSizeEx  GetFileVersionInfoSizeExA
#endif // !UNICODE

BOOL APIENTRY GetFileVersionInfoExA(_In_ DWORD dwFlags,
                                    _In_ LPCSTR lpwstrFilename,
                                    _Reserved_ DWORD dwHandle,
                                    _In_ DWORD dwLen,
                                    _Out_writes_bytes_(dwLen) LPVOID lpData);
BOOL APIENTRY GetFileVersionInfoExW(_In_ DWORD dwFlags,
                                    _In_ LPCWSTR lpwstrFilename,
                                    _Reserved_ DWORD dwHandle,
                                    _In_ DWORD dwLen,
                                    _Out_writes_bytes_(dwLen) LPVOID lpData);
#ifdef UNICODE
#define GetFileVersionInfoEx  GetFileVersionInfoExW
#else
#define GetFileVersionInfoEx  GetFileVersionInfoExA
#endif // !UNICODE

#endif // RC_INVOKED

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

#ifndef RC_INVOKED


#endif  /* !RC_INVOKED */

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#pragma region Application Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)

#ifndef RC_INVOKED

DWORD
APIENTRY
VerLanguageNameA(
        _In_                  DWORD wLang,
        _Out_writes_(cchLang) LPSTR szLang,
        _In_                  DWORD cchLang
        );
DWORD
APIENTRY
VerLanguageNameW(
        _In_                  DWORD wLang,
        _Out_writes_(cchLang) LPWSTR szLang,
        _In_                  DWORD cchLang
        );
#ifdef UNICODE
#define VerLanguageName  VerLanguageNameW
#else
#define VerLanguageName  VerLanguageNameA
#endif // !UNICODE

BOOL
APIENTRY
VerQueryValueA(
        _In_ LPCVOID pBlock,
        _In_ LPCSTR lpSubBlock,
        _Outptr_result_buffer_(_Inexpressible_("buffer can be PWSTR or DWORD*")) LPVOID * lplpBuffer,
        _Out_ PUINT puLen
        );
BOOL
APIENTRY
VerQueryValueW(
        _In_ LPCVOID pBlock,
        _In_ LPCWSTR lpSubBlock,
        _Outptr_result_buffer_(_Inexpressible_("buffer can be PWSTR or DWORD*")) LPVOID * lplpBuffer,
        _Out_ PUINT puLen
        );
#ifdef UNICODE
#define VerQueryValue  VerQueryValueW
#else
#define VerQueryValue  VerQueryValueA
#endif // !UNICODE


#endif  /* !RC_INVOKED */

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#ifdef __cplusplus
}
#endif


#endif  /* !VER_H */

