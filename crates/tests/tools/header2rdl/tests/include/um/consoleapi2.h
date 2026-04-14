/********************************************************************************
*                                                                               *
* consoleapi2.h -- ApiSet Contract for api-ms-win-core-console-l2                *
*                                                                               *
* Copyright (c) Microsoft Corporation. All rights reserved.                     *
*                                                                               *
********************************************************************************/

#ifdef _MSC_VER
#pragma once
#endif // _MSC_VER

#ifndef _APISETCONSOLEL2_
#define _APISETCONSOLEL2_

#include <apiset.h>
#include <apisetcconv.h>
#include <minwindef.h>
#include <minwinbase.h>

#include <wincontypes.h>
#include <windef.h>

#ifdef __cplusplus
extern "C" {
#endif

#pragma region Application Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)

//
// Attributes flags:
//

#define FOREGROUND_BLUE      0x0001 // text color contains blue.
#define FOREGROUND_GREEN     0x0002 // text color contains green.
#define FOREGROUND_RED       0x0004 // text color contains red.
#define FOREGROUND_INTENSITY 0x0008 // text color is intensified.
#define BACKGROUND_BLUE      0x0010 // background color contains blue.
#define BACKGROUND_GREEN     0x0020 // background color contains green.
#define BACKGROUND_RED       0x0040 // background color contains red.
#define BACKGROUND_INTENSITY 0x0080 // background color is intensified.
#define COMMON_LVB_LEADING_BYTE    0x0100 // Leading Byte of DBCS
#define COMMON_LVB_TRAILING_BYTE   0x0200 // Trailing Byte of DBCS
#define COMMON_LVB_GRID_HORIZONTAL 0x0400 // DBCS: Grid attribute: top horizontal.
#define COMMON_LVB_GRID_LVERTICAL  0x0800 // DBCS: Grid attribute: left vertical.
#define COMMON_LVB_GRID_RVERTICAL  0x1000 // DBCS: Grid attribute: right vertical.
#define COMMON_LVB_REVERSE_VIDEO   0x4000 // DBCS: Reverse fore/back ground attribute.
#define COMMON_LVB_UNDERSCORE      0x8000 // DBCS: Underscore.

#define COMMON_LVB_SBCSDBCS        0x0300 // SBCS or DBCS flag.

WINBASEAPI
BOOL
WINAPI
FillConsoleOutputCharacterA(
    _In_ HANDLE hConsoleOutput,
    _In_ CHAR cCharacter,
    _In_ DWORD nLength,
    _In_ COORD dwWriteCoord,
    _Out_ LPDWORD lpNumberOfCharsWritten
    );

WINBASEAPI
BOOL
WINAPI
FillConsoleOutputCharacterW(
    _In_ HANDLE hConsoleOutput,
    _In_ WCHAR cCharacter,
    _In_ DWORD nLength,
    _In_ COORD dwWriteCoord,
    _Out_ LPDWORD lpNumberOfCharsWritten
    );

#ifdef UNICODE
#define FillConsoleOutputCharacter  FillConsoleOutputCharacterW
#else
#define FillConsoleOutputCharacter  FillConsoleOutputCharacterA
#endif // !UNICODE

WINBASEAPI
BOOL
WINAPI
FillConsoleOutputAttribute(
    _In_ HANDLE hConsoleOutput,
    _In_ WORD wAttribute,
    _In_ DWORD nLength,
    _In_ COORD dwWriteCoord,
    _Out_ LPDWORD lpNumberOfAttrsWritten
    );

WINBASEAPI
BOOL
WINAPI
GenerateConsoleCtrlEvent(
    _In_ DWORD dwCtrlEvent,
    _In_ DWORD dwProcessGroupId
    );

WINBASEAPI
HANDLE
WINAPI
CreateConsoleScreenBuffer(
    _In_ DWORD dwDesiredAccess,
    _In_ DWORD dwShareMode,
    _In_opt_ CONST SECURITY_ATTRIBUTES* lpSecurityAttributes,
    _In_ DWORD dwFlags,
    _Reserved_ LPVOID lpScreenBufferData
    );

WINBASEAPI
BOOL
WINAPI
SetConsoleActiveScreenBuffer(
    _In_ HANDLE hConsoleOutput
    );

WINBASEAPI
BOOL
WINAPI
FlushConsoleInputBuffer(
    _In_ HANDLE hConsoleInput
    );

WINBASEAPI
BOOL
WINAPI
SetConsoleCP(
    _In_ UINT wCodePageID
    );

WINBASEAPI
BOOL
WINAPI
SetConsoleOutputCP(
    _In_ UINT wCodePageID
    );

typedef struct _CONSOLE_CURSOR_INFO {
    DWORD  dwSize;
    BOOL   bVisible;
} CONSOLE_CURSOR_INFO, *PCONSOLE_CURSOR_INFO;

WINBASEAPI
BOOL
WINAPI
GetConsoleCursorInfo(
    _In_ HANDLE hConsoleOutput,
    _Out_ PCONSOLE_CURSOR_INFO lpConsoleCursorInfo
    );

WINBASEAPI
BOOL
WINAPI
SetConsoleCursorInfo(
    _In_ HANDLE hConsoleOutput,
    _In_ CONST CONSOLE_CURSOR_INFO* lpConsoleCursorInfo
    );

typedef struct _CONSOLE_SCREEN_BUFFER_INFO {
    COORD dwSize;
    COORD dwCursorPosition;
    WORD  wAttributes;
    SMALL_RECT srWindow;
    COORD dwMaximumWindowSize;
} CONSOLE_SCREEN_BUFFER_INFO, *PCONSOLE_SCREEN_BUFFER_INFO;

WINBASEAPI
BOOL
WINAPI
GetConsoleScreenBufferInfo(
    _In_ HANDLE hConsoleOutput,
    _Out_ PCONSOLE_SCREEN_BUFFER_INFO lpConsoleScreenBufferInfo
    );

typedef struct _CONSOLE_SCREEN_BUFFER_INFOEX {
    ULONG cbSize;
    COORD dwSize;
    COORD dwCursorPosition;
    WORD wAttributes;
    SMALL_RECT srWindow;
    COORD dwMaximumWindowSize;
    WORD wPopupAttributes;
    BOOL bFullscreenSupported;
    COLORREF ColorTable[16];
} CONSOLE_SCREEN_BUFFER_INFOEX, *PCONSOLE_SCREEN_BUFFER_INFOEX;

WINBASEAPI
BOOL
WINAPI
GetConsoleScreenBufferInfoEx(
    _In_ HANDLE hConsoleOutput,
    _Inout_ PCONSOLE_SCREEN_BUFFER_INFOEX lpConsoleScreenBufferInfoEx
    );

WINBASEAPI
BOOL
WINAPI
SetConsoleScreenBufferInfoEx(
    _In_ HANDLE hConsoleOutput,
    _In_ PCONSOLE_SCREEN_BUFFER_INFOEX lpConsoleScreenBufferInfoEx
    );

WINBASEAPI
BOOL
WINAPI
SetConsoleScreenBufferSize(
    _In_ HANDLE hConsoleOutput,
    _In_ COORD dwSize
    );

WINBASEAPI
BOOL
WINAPI
SetConsoleCursorPosition(
    _In_ HANDLE hConsoleOutput,
    _In_ COORD dwCursorPosition
    );

WINBASEAPI
COORD
WINAPI
GetLargestConsoleWindowSize(
    _In_ HANDLE hConsoleOutput
    );

WINBASEAPI
BOOL
WINAPI
SetConsoleTextAttribute(
    _In_ HANDLE hConsoleOutput,
    _In_ WORD wAttributes
    );

WINBASEAPI
BOOL
WINAPI
SetConsoleWindowInfo(
    _In_ HANDLE hConsoleOutput,
    _In_ BOOL bAbsolute,
    _In_ CONST SMALL_RECT* lpConsoleWindow
    );

WINBASEAPI
BOOL
WINAPI
WriteConsoleOutputCharacterA(
    _In_ HANDLE hConsoleOutput,
    _In_reads_(nLength) LPCSTR lpCharacter,
    _In_ DWORD nLength,
    _In_ COORD dwWriteCoord,
    _Out_ LPDWORD lpNumberOfCharsWritten
    );

WINBASEAPI
BOOL
WINAPI
WriteConsoleOutputCharacterW(
    _In_ HANDLE hConsoleOutput,
    _In_reads_(nLength) LPCWSTR lpCharacter,
    _In_ DWORD nLength,
    _In_ COORD dwWriteCoord,
    _Out_ LPDWORD lpNumberOfCharsWritten
    );

#ifdef UNICODE
#define WriteConsoleOutputCharacter  WriteConsoleOutputCharacterW
#else
#define WriteConsoleOutputCharacter  WriteConsoleOutputCharacterA
#endif // !UNICODE

WINBASEAPI
BOOL
WINAPI
WriteConsoleOutputAttribute(
    _In_ HANDLE hConsoleOutput,
    _In_reads_(nLength) CONST WORD* lpAttribute,
    _In_ DWORD nLength,
    _In_ COORD dwWriteCoord,
    _Out_ LPDWORD lpNumberOfAttrsWritten
    );

WINBASEAPI
BOOL
WINAPI
ReadConsoleOutputCharacterA(
    _In_ HANDLE hConsoleOutput,
    _Out_writes_(nLength) LPSTR lpCharacter,
    _In_ DWORD nLength,
    _In_ COORD dwReadCoord,
    _Out_ LPDWORD lpNumberOfCharsRead
    );

WINBASEAPI
BOOL
WINAPI
ReadConsoleOutputCharacterW(
    _In_ HANDLE hConsoleOutput,
    _Out_writes_(nLength) LPWSTR lpCharacter,
    _In_ DWORD nLength,
    _In_ COORD dwReadCoord,
    _Out_ LPDWORD lpNumberOfCharsRead
    );

#ifdef UNICODE
#define ReadConsoleOutputCharacter  ReadConsoleOutputCharacterW
#else
#define ReadConsoleOutputCharacter  ReadConsoleOutputCharacterA
#endif // !UNICODE

WINBASEAPI
BOOL
WINAPI
ReadConsoleOutputAttribute(
    _In_ HANDLE hConsoleOutput,
    _Out_writes_(nLength) LPWORD lpAttribute,
    _In_ DWORD nLength,
    _In_ COORD dwReadCoord,
    _Out_ LPDWORD lpNumberOfAttrsRead
    );

WINBASEAPI
BOOL
WINAPI
WriteConsoleInputA(
    _In_ HANDLE hConsoleInput,
    _In_reads_(nLength) CONST INPUT_RECORD* lpBuffer,
    _In_ DWORD nLength,
    _Out_ LPDWORD lpNumberOfEventsWritten
    );

WINBASEAPI
BOOL
WINAPI
WriteConsoleInputW(
    _In_ HANDLE hConsoleInput,
    _In_reads_(nLength) CONST INPUT_RECORD* lpBuffer,
    _In_ DWORD nLength,
    _Out_ LPDWORD lpNumberOfEventsWritten
    );

#ifdef UNICODE
#define WriteConsoleInput  WriteConsoleInputW
#else
#define WriteConsoleInput  WriteConsoleInputA
#endif // !UNICODE

WINBASEAPI
BOOL
WINAPI
ScrollConsoleScreenBufferA(
    _In_ HANDLE hConsoleOutput,
    _In_ CONST SMALL_RECT* lpScrollRectangle,
    _In_opt_ CONST SMALL_RECT* lpClipRectangle,
    _In_ COORD dwDestinationOrigin,
    _In_ CONST CHAR_INFO* lpFill
    );

WINBASEAPI
BOOL
WINAPI
ScrollConsoleScreenBufferW(
    _In_ HANDLE hConsoleOutput,
    _In_ CONST SMALL_RECT* lpScrollRectangle,
    _In_opt_ CONST SMALL_RECT* lpClipRectangle,
    _In_ COORD dwDestinationOrigin,
    _In_ CONST CHAR_INFO* lpFill
    );

#ifdef UNICODE
#define ScrollConsoleScreenBuffer  ScrollConsoleScreenBufferW
#else
#define ScrollConsoleScreenBuffer  ScrollConsoleScreenBufferA
#endif // !UNICODE

WINBASEAPI
BOOL
WINAPI
WriteConsoleOutputA(
    _In_ HANDLE hConsoleOutput,
    _In_reads_(dwBufferSize.X * dwBufferSize.Y) CONST CHAR_INFO* lpBuffer,
    _In_ COORD dwBufferSize,
    _In_ COORD dwBufferCoord,
    _Inout_ PSMALL_RECT lpWriteRegion
    );

WINBASEAPI
BOOL
WINAPI
WriteConsoleOutputW(
    _In_ HANDLE hConsoleOutput,
    _In_reads_(dwBufferSize.X * dwBufferSize.Y) CONST CHAR_INFO* lpBuffer,
    _In_ COORD dwBufferSize,
    _In_ COORD dwBufferCoord,
    _Inout_ PSMALL_RECT lpWriteRegion
    );

#ifdef UNICODE
#define WriteConsoleOutput  WriteConsoleOutputW
#else
#define WriteConsoleOutput  WriteConsoleOutputA
#endif // !UNICODE

WINBASEAPI
BOOL
WINAPI
ReadConsoleOutputA(
    _In_ HANDLE hConsoleOutput,
    _Out_writes_(dwBufferSize.X * dwBufferSize.Y) PCHAR_INFO lpBuffer,
    _In_ COORD dwBufferSize,
    _In_ COORD dwBufferCoord,
    _Inout_ PSMALL_RECT lpReadRegion
    );

WINBASEAPI
BOOL
WINAPI
ReadConsoleOutputW(
    _In_ HANDLE hConsoleOutput,
    _Out_writes_(dwBufferSize.X * dwBufferSize.Y) PCHAR_INFO lpBuffer,
    _In_ COORD dwBufferSize,
    _In_ COORD dwBufferCoord,
    _Inout_ PSMALL_RECT lpReadRegion
    );

#ifdef UNICODE
#define ReadConsoleOutput  ReadConsoleOutputW
#else
#define ReadConsoleOutput  ReadConsoleOutputA
#endif // !UNICODE

WINBASEAPI
DWORD
WINAPI
GetConsoleTitleA(
    _Out_writes_(nSize) LPSTR lpConsoleTitle,
    _In_ DWORD nSize
    );

WINBASEAPI
DWORD
WINAPI
GetConsoleTitleW(
    _Out_writes_(nSize) LPWSTR lpConsoleTitle,
    _In_ DWORD nSize
    );

#ifdef UNICODE
#define GetConsoleTitle  GetConsoleTitleW
#else
#define GetConsoleTitle  GetConsoleTitleA
#endif // !UNICODE

#if (_WIN32_WINNT >= 0x0600)

WINBASEAPI
DWORD
WINAPI
GetConsoleOriginalTitleA(
    _Out_writes_(nSize) LPSTR lpConsoleTitle,
    _In_ DWORD nSize
    );

WINBASEAPI
DWORD
WINAPI
GetConsoleOriginalTitleW(
    _Out_writes_(nSize) LPWSTR lpConsoleTitle,
    _In_ DWORD nSize
    );

#ifdef UNICODE
#define GetConsoleOriginalTitle  GetConsoleOriginalTitleW
#else
#define GetConsoleOriginalTitle  GetConsoleOriginalTitleA
#endif // !UNICODE

#endif /* _WIN32_WINNT >= 0x0600 */

WINBASEAPI
BOOL
WINAPI
SetConsoleTitleA(
    _In_ LPCSTR lpConsoleTitle
    );

WINBASEAPI
BOOL
WINAPI
SetConsoleTitleW(
    _In_ LPCWSTR lpConsoleTitle
    );

#ifdef UNICODE
#define SetConsoleTitle  SetConsoleTitleW
#else
#define SetConsoleTitle  SetConsoleTitleA
#endif // !UNICODE

#endif // WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)
#pragma endregion

#ifdef __cplusplus
}
#endif

#endif // _APISETCONSOLEL2_
