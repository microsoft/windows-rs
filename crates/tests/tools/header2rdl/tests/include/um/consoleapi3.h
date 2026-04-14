/********************************************************************************
*                                                                               *
* consoleapi3.h -- ApiSet Contract for api-ms-win-core-console-l3                                *
*                                                                               *
* Copyright (c) Microsoft Corporation. All rights reserved.                     *
*                                                                               *
********************************************************************************/

#ifdef _MSC_VER
#pragma once
#endif // _MSC_VER

#ifndef _APISETCONSOLEL3_
#define _APISETCONSOLEL3_

#include <apiset.h>
#include <apisetcconv.h>
#include <minwindef.h>
#include <minwinbase.h>

#include <wincontypes.h>
#include <windef.h>

#ifndef NOGDI
#include <wingdi.h>
#endif

#ifdef __cplusplus
extern "C" {
#endif

#pragma region Application Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)

WINBASEAPI
BOOL
WINAPI
GetNumberOfConsoleMouseButtons(
    _Out_ LPDWORD lpNumberOfMouseButtons
    );

#if (_WIN32_WINNT >= 0x0500)

WINBASEAPI
COORD
WINAPI
GetConsoleFontSize(
    _In_ HANDLE hConsoleOutput,
    _In_ DWORD nFont
    );

WINBASEAPI
BOOL
WINAPI
GetCurrentConsoleFont(
    _In_ HANDLE hConsoleOutput,
    _In_ BOOL bMaximumWindow,
    _Out_ PCONSOLE_FONT_INFO lpConsoleCurrentFont
    );

#ifndef NOGDI

typedef struct _CONSOLE_FONT_INFOEX {
    ULONG cbSize;
    DWORD nFont;
    COORD dwFontSize;
    UINT FontFamily;
    UINT FontWeight;
    WCHAR FaceName[LF_FACESIZE];
} CONSOLE_FONT_INFOEX, *PCONSOLE_FONT_INFOEX;

WINBASEAPI
BOOL
WINAPI
GetCurrentConsoleFontEx(
    _In_ HANDLE hConsoleOutput,
    _In_ BOOL bMaximumWindow,
    _Out_ PCONSOLE_FONT_INFOEX lpConsoleCurrentFontEx
    );

WINBASEAPI
BOOL
WINAPI
SetCurrentConsoleFontEx(
    _In_ HANDLE hConsoleOutput,
    _In_ BOOL bMaximumWindow,
    _In_ PCONSOLE_FONT_INFOEX lpConsoleCurrentFontEx
    );

#endif

//
// Selection flags
//

#define CONSOLE_NO_SELECTION            0x0000
#define CONSOLE_SELECTION_IN_PROGRESS   0x0001   // selection has begun
#define CONSOLE_SELECTION_NOT_EMPTY     0x0002   // non-null select rectangle
#define CONSOLE_MOUSE_SELECTION         0x0004   // selecting with mouse
#define CONSOLE_MOUSE_DOWN              0x0008   // mouse is down

typedef struct _CONSOLE_SELECTION_INFO {
    DWORD dwFlags;
    COORD dwSelectionAnchor;
    SMALL_RECT srSelection;
} CONSOLE_SELECTION_INFO, *PCONSOLE_SELECTION_INFO;

WINBASEAPI
BOOL
WINAPI
GetConsoleSelectionInfo(
    _Out_ PCONSOLE_SELECTION_INFO lpConsoleSelectionInfo
    );

//
// History flags
//

#define HISTORY_NO_DUP_FLAG 0x1

typedef struct _CONSOLE_HISTORY_INFO {
    UINT cbSize;
    UINT HistoryBufferSize;
    UINT NumberOfHistoryBuffers;
    DWORD dwFlags;
} CONSOLE_HISTORY_INFO, *PCONSOLE_HISTORY_INFO;

WINBASEAPI
BOOL
WINAPI
GetConsoleHistoryInfo(
    _Out_ PCONSOLE_HISTORY_INFO lpConsoleHistoryInfo
    );

WINBASEAPI
BOOL
WINAPI
SetConsoleHistoryInfo(
    _In_ PCONSOLE_HISTORY_INFO lpConsoleHistoryInfo
    );

#define CONSOLE_FULLSCREEN 1            // fullscreen console
#define CONSOLE_FULLSCREEN_HARDWARE 2   // console owns the hardware

WINBASEAPI
BOOL
APIENTRY
GetConsoleDisplayMode(
    _Out_ LPDWORD lpModeFlags
    );

#define CONSOLE_FULLSCREEN_MODE 1
#define CONSOLE_WINDOWED_MODE 2

WINBASEAPI
BOOL
APIENTRY
SetConsoleDisplayMode(
    _In_ HANDLE hConsoleOutput,
    _In_ DWORD dwFlags,
    _Out_opt_ PCOORD lpNewScreenBufferDimensions
    );

WINBASEAPI
HWND
APIENTRY
GetConsoleWindow(
    VOID
    );

#endif /* _WIN32_WINNT >= 0x0500 */

#if (_WIN32_WINNT >= 0x0501)

WINBASEAPI
BOOL
APIENTRY
AddConsoleAliasA(
    _In_ LPSTR Source,
    _In_ LPSTR Target,
    _In_ LPSTR ExeName
    );

WINBASEAPI
BOOL
APIENTRY
AddConsoleAliasW(
    _In_ LPWSTR Source,
    _In_ LPWSTR Target,
    _In_ LPWSTR ExeName
    );

#ifdef UNICODE
#define AddConsoleAlias  AddConsoleAliasW
#else
#define AddConsoleAlias  AddConsoleAliasA
#endif // !UNICODE

WINBASEAPI
DWORD
APIENTRY
GetConsoleAliasA(
    _In_ LPSTR Source,
    _Out_writes_(TargetBufferLength) LPSTR TargetBuffer,
    _In_ DWORD TargetBufferLength,
    _In_ LPSTR ExeName
    );

WINBASEAPI
DWORD
APIENTRY
GetConsoleAliasW(
    _In_ LPWSTR Source,
    _Out_writes_(TargetBufferLength) LPWSTR TargetBuffer,
    _In_ DWORD TargetBufferLength,
    _In_ LPWSTR ExeName
    );

#ifdef UNICODE
#define GetConsoleAlias  GetConsoleAliasW
#else
#define GetConsoleAlias  GetConsoleAliasA
#endif // !UNICODE

WINBASEAPI
DWORD
APIENTRY
GetConsoleAliasesLengthA(
    _In_ LPSTR ExeName
    );

WINBASEAPI
DWORD
APIENTRY
GetConsoleAliasesLengthW(
    _In_ LPWSTR ExeName
    );

#ifdef UNICODE
#define GetConsoleAliasesLength  GetConsoleAliasesLengthW
#else
#define GetConsoleAliasesLength  GetConsoleAliasesLengthA
#endif // !UNICODE

WINBASEAPI
DWORD
APIENTRY
GetConsoleAliasExesLengthA(
    VOID
    );

WINBASEAPI
DWORD
APIENTRY
GetConsoleAliasExesLengthW(
    VOID
    );

#ifdef UNICODE
#define GetConsoleAliasExesLength  GetConsoleAliasExesLengthW
#else
#define GetConsoleAliasExesLength  GetConsoleAliasExesLengthA
#endif // !UNICODE

WINBASEAPI
DWORD
APIENTRY
GetConsoleAliasesA(
    _Out_writes_(AliasBufferLength) LPSTR AliasBuffer,
    _In_ DWORD AliasBufferLength,
    _In_ LPSTR ExeName
    );

WINBASEAPI
DWORD
APIENTRY
GetConsoleAliasesW(
    _Out_writes_(AliasBufferLength) LPWSTR AliasBuffer,
    _In_ DWORD AliasBufferLength,
    _In_ LPWSTR ExeName
    );

#ifdef UNICODE
#define GetConsoleAliases  GetConsoleAliasesW
#else
#define GetConsoleAliases  GetConsoleAliasesA
#endif // !UNICODE

WINBASEAPI
DWORD
APIENTRY
GetConsoleAliasExesA(
    _Out_writes_(ExeNameBufferLength) LPSTR ExeNameBuffer,
    _In_ DWORD ExeNameBufferLength
    );

WINBASEAPI
DWORD
APIENTRY
GetConsoleAliasExesW(
    _Out_writes_(ExeNameBufferLength) LPWSTR ExeNameBuffer,
    _In_ DWORD ExeNameBufferLength
    );

#ifdef UNICODE
#define GetConsoleAliasExes  GetConsoleAliasExesW
#else
#define GetConsoleAliasExes  GetConsoleAliasExesA
#endif // !UNICODE

#endif /* _WIN32_WINNT >= 0x0501 */

WINBASEAPI
VOID
APIENTRY
ExpungeConsoleCommandHistoryA(
    _In_ LPSTR ExeName
    );

WINBASEAPI
VOID
APIENTRY
ExpungeConsoleCommandHistoryW(
    _In_ LPWSTR ExeName
    );

#ifdef UNICODE
#define ExpungeConsoleCommandHistory  ExpungeConsoleCommandHistoryW
#else
#define ExpungeConsoleCommandHistory  ExpungeConsoleCommandHistoryA
#endif // !UNICODE

WINBASEAPI
BOOL
APIENTRY
SetConsoleNumberOfCommandsA(
    _In_ DWORD Number,
    _In_ LPSTR ExeName
    );

WINBASEAPI
BOOL
APIENTRY
SetConsoleNumberOfCommandsW(
    _In_ DWORD Number,
    _In_ LPWSTR ExeName
    );

#ifdef UNICODE
#define SetConsoleNumberOfCommands  SetConsoleNumberOfCommandsW
#else
#define SetConsoleNumberOfCommands  SetConsoleNumberOfCommandsA
#endif // !UNICODE

WINBASEAPI
DWORD
APIENTRY
GetConsoleCommandHistoryLengthA(
    _In_ LPSTR ExeName
    );

WINBASEAPI
DWORD
APIENTRY
GetConsoleCommandHistoryLengthW(
    _In_ LPWSTR ExeName
    );

#ifdef UNICODE
#define GetConsoleCommandHistoryLength  GetConsoleCommandHistoryLengthW
#else
#define GetConsoleCommandHistoryLength  GetConsoleCommandHistoryLengthA
#endif // !UNICODE

WINBASEAPI
DWORD
APIENTRY
GetConsoleCommandHistoryA(
    _Out_writes_bytes_(CommandBufferLength) LPSTR Commands,
    _In_ DWORD CommandBufferLength,
    _In_ LPSTR ExeName
    );

WINBASEAPI
DWORD
APIENTRY
GetConsoleCommandHistoryW(
    _Out_writes_bytes_(CommandBufferLength) LPWSTR Commands,
    _In_ DWORD CommandBufferLength,
    _In_ LPWSTR ExeName
    );

#ifdef UNICODE
#define GetConsoleCommandHistory  GetConsoleCommandHistoryW
#else
#define GetConsoleCommandHistory  GetConsoleCommandHistoryA
#endif // !UNICODE

#if (_WIN32_WINNT >= 0x0501)

WINBASEAPI
DWORD
APIENTRY
GetConsoleProcessList(
    _Out_writes_(dwProcessCount) LPDWORD lpdwProcessList,
    _In_ DWORD dwProcessCount
    );

#endif /* _WIN32_WINNT >= 0x0501 */

#endif // WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)
#pragma endregion

#ifdef __cplusplus
}
#endif

#endif // _APISETCONSOLEL3_
