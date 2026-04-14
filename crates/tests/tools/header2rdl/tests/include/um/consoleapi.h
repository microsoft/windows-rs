// begin_consoleapi_h
/********************************************************************************
*                                                                               *
* consoleapi.h -- ApiSet Contract for api-ms-win-core-console-l1                *
*                                                                               *
* Copyright (c) Microsoft Corporation. All rights reserved.                     *
*                                                                               *
********************************************************************************/

#ifdef _MSC_VER
#pragma once
#endif // _MSC_VER

#ifndef _APISETCONSOLE_
#define _APISETCONSOLE_

#include <apiset.h>
#include <apisetcconv.h>
#include <minwindef.h>
#include <minwinbase.h>

#include <wincontypes.h>

#ifdef __cplusplus
extern "C" {
#endif

#pragma region Application Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)

WINBASEAPI
BOOL
WINAPI
AllocConsole(
    VOID
    );

#if (NTDDI_VERSION >= NTDDI_WIN11_GE)

typedef enum ALLOC_CONSOLE_MODE {
    ALLOC_CONSOLE_MODE_DEFAULT    = 0,
    ALLOC_CONSOLE_MODE_NEW_WINDOW = 1,
    ALLOC_CONSOLE_MODE_NO_WINDOW  = 2
} ALLOC_CONSOLE_MODE;

typedef struct ALLOC_CONSOLE_OPTIONS {
    ALLOC_CONSOLE_MODE mode;
    BOOL  useShowWindow;
    WORD  showWindow;
} ALLOC_CONSOLE_OPTIONS, *PALLOC_CONSOLE_OPTIONS;

typedef enum ALLOC_CONSOLE_RESULT {
    ALLOC_CONSOLE_RESULT_NO_CONSOLE       = 0,
    ALLOC_CONSOLE_RESULT_NEW_CONSOLE      = 1,
    ALLOC_CONSOLE_RESULT_EXISTING_CONSOLE = 2
} ALLOC_CONSOLE_RESULT, *PALLOC_CONSOLE_RESULT;

WINBASEAPI
HRESULT
WINAPI
AllocConsoleWithOptions(
    _In_opt_ PALLOC_CONSOLE_OPTIONS options,
    _Out_opt_ PALLOC_CONSOLE_RESULT result
    );

#endif

WINBASEAPI
BOOL
WINAPI
FreeConsole(
    VOID
    );

#if (_WIN32_WINNT >= 0x0500)

WINBASEAPI
BOOL
WINAPI
AttachConsole(
    _In_ DWORD dwProcessId
    );

#define ATTACH_PARENT_PROCESS ((DWORD)-1)

#endif /* _WIN32_WINNT >= 0x0500 */

WINBASEAPI
UINT
WINAPI
GetConsoleCP(
    VOID
    );

WINBASEAPI
UINT
WINAPI
GetConsoleOutputCP(
    VOID
    );

//
// Input Mode flags:
//

#define ENABLE_PROCESSED_INPUT              0x0001
#define ENABLE_LINE_INPUT                   0x0002
#define ENABLE_ECHO_INPUT                   0x0004
#define ENABLE_WINDOW_INPUT                 0x0008
#define ENABLE_MOUSE_INPUT                  0x0010
#define ENABLE_INSERT_MODE                  0x0020
#define ENABLE_QUICK_EDIT_MODE              0x0040
#define ENABLE_EXTENDED_FLAGS               0x0080
#define ENABLE_AUTO_POSITION                0x0100
#define ENABLE_VIRTUAL_TERMINAL_INPUT       0x0200

//
// Output Mode flags:
//

#define ENABLE_PROCESSED_OUTPUT             0x0001
#define ENABLE_WRAP_AT_EOL_OUTPUT           0x0002
#define ENABLE_VIRTUAL_TERMINAL_PROCESSING  0x0004
#define DISABLE_NEWLINE_AUTO_RETURN         0x0008
#define ENABLE_LVB_GRID_WORLDWIDE           0x0010

WINBASEAPI
BOOL
WINAPI
GetConsoleMode(
    _In_ HANDLE hConsoleHandle,
    _Out_ LPDWORD lpMode
    );

WINBASEAPI
BOOL
WINAPI
SetConsoleMode(
    _In_ HANDLE hConsoleHandle,
    _In_ DWORD dwMode
    );

WINBASEAPI
BOOL
WINAPI
GetNumberOfConsoleInputEvents(
    _In_ HANDLE hConsoleInput,
    _Out_ LPDWORD lpNumberOfEvents
    );

WINBASEAPI
_Success_(return != FALSE)
BOOL
WINAPI
ReadConsoleInputA(
    _In_ HANDLE hConsoleInput,
    _Out_writes_to_(nLength, *lpNumberOfEventsRead) PINPUT_RECORD lpBuffer,
    _In_ DWORD nLength,
    _Out_ _Deref_out_range_(<=, nLength) LPDWORD lpNumberOfEventsRead
    );

WINBASEAPI
_Success_(return != FALSE)
BOOL
WINAPI
ReadConsoleInputW(
    _In_ HANDLE hConsoleInput,
    _Out_writes_to_(nLength, *lpNumberOfEventsRead) PINPUT_RECORD lpBuffer,
    _In_ DWORD nLength,
    _Out_ _Deref_out_range_(<=, nLength) LPDWORD lpNumberOfEventsRead
    );

#ifdef UNICODE
#define ReadConsoleInput  ReadConsoleInputW
#else
#define ReadConsoleInput  ReadConsoleInputA
#endif // !UNICODE

// end_consoleapi_h

#ifndef UNICODE
#define PeekConsoleInput  PeekConsoleInputA
#endif

// begin_consoleapi_h

WINBASEAPI
BOOL
WINAPI
PeekConsoleInputA(
    _In_ HANDLE hConsoleInput,
    _Out_writes_(nLength) PINPUT_RECORD lpBuffer,
    _In_ DWORD nLength,
    _Out_ LPDWORD lpNumberOfEventsRead
    );

WINBASEAPI
BOOL
WINAPI
PeekConsoleInputW(
    _In_ HANDLE hConsoleInput,
    _Out_writes_(nLength) PINPUT_RECORD lpBuffer,
    _In_ DWORD nLength,
    _Out_ LPDWORD lpNumberOfEventsRead
    );

#ifdef UNICODE
#define PeekConsoleInput  PeekConsoleInputW
#else
#define PeekConsoleInput  PeekConsoleInputA
#endif // !UNICODE

typedef struct _CONSOLE_READCONSOLE_CONTROL {
    ULONG nLength;
    ULONG nInitialChars;
    ULONG dwCtrlWakeupMask;
    ULONG dwControlKeyState;
} CONSOLE_READCONSOLE_CONTROL, *PCONSOLE_READCONSOLE_CONTROL;

WINBASEAPI
_Success_(return != FALSE)
BOOL
WINAPI
ReadConsoleA(
    _In_ HANDLE hConsoleInput,
    _Out_writes_bytes_to_(nNumberOfCharsToRead * sizeof(CHAR), *lpNumberOfCharsRead * sizeof(TCHAR%)) LPVOID lpBuffer,
    _In_ DWORD nNumberOfCharsToRead,
    _Out_ _Deref_out_range_(<=, nNumberOfCharsToRead) LPDWORD lpNumberOfCharsRead,
    _In_opt_ PCONSOLE_READCONSOLE_CONTROL pInputControl
    );

WINBASEAPI
_Success_(return != FALSE)
BOOL
WINAPI
ReadConsoleW(
    _In_ HANDLE hConsoleInput,
    _Out_writes_bytes_to_(nNumberOfCharsToRead * sizeof(WCHAR), *lpNumberOfCharsRead * sizeof(TCHAR%)) LPVOID lpBuffer,
    _In_ DWORD nNumberOfCharsToRead,
    _Out_ _Deref_out_range_(<=, nNumberOfCharsToRead) LPDWORD lpNumberOfCharsRead,
    _In_opt_ PCONSOLE_READCONSOLE_CONTROL pInputControl
    );

#ifdef UNICODE
#define ReadConsole  ReadConsoleW
#else
#define ReadConsole  ReadConsoleA
#endif // !UNICODE

WINBASEAPI
BOOL
WINAPI
WriteConsoleA(
    _In_ HANDLE hConsoleOutput,
    _In_reads_(nNumberOfCharsToWrite) CONST VOID* lpBuffer,
    _In_ DWORD nNumberOfCharsToWrite,
    _Out_opt_ LPDWORD lpNumberOfCharsWritten,
    _Reserved_ LPVOID lpReserved
    );

WINBASEAPI
BOOL
WINAPI
WriteConsoleW(
    _In_ HANDLE hConsoleOutput,
    _In_reads_(nNumberOfCharsToWrite) CONST VOID* lpBuffer,
    _In_ DWORD nNumberOfCharsToWrite,
    _Out_opt_ LPDWORD lpNumberOfCharsWritten,
    _Reserved_ LPVOID lpReserved
    );

#ifdef UNICODE
#define WriteConsole  WriteConsoleW
#else
#define WriteConsole  WriteConsoleA
#endif // !UNICODE

//
// Ctrl Event flags
//

#define CTRL_C_EVENT        0
#define CTRL_BREAK_EVENT    1
#define CTRL_CLOSE_EVENT    2
// 3 is reserved!
// 4 is reserved!
#define CTRL_LOGOFF_EVENT   5
#define CTRL_SHUTDOWN_EVENT 6

//
// typedef for ctrl-c handler routines
//

typedef
BOOL
(WINAPI *PHANDLER_ROUTINE)(
    _In_ DWORD CtrlType
    );

WINBASEAPI
BOOL
WINAPI
SetConsoleCtrlHandler(
    _In_opt_ PHANDLER_ROUTINE HandlerRoutine,
    _In_ BOOL Add
    );

#endif // WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)
#pragma endregion

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

#if (NTDDI_VERSION >= NTDDI_WIN10_RS5)

// CreatePseudoConsole Flags
#define PSEUDOCONSOLE_INHERIT_CURSOR (0x1)

WINBASEAPI
HRESULT
WINAPI
CreatePseudoConsole(
    _In_ COORD size,
    _In_ HANDLE hInput,
    _In_ HANDLE hOutput,
    _In_ DWORD dwFlags,
    _Out_ HPCON* phPC
    );

WINBASEAPI
HRESULT
WINAPI
ResizePseudoConsole(
    _In_ HPCON hPC,
    _In_ COORD size
    );

WINBASEAPI
VOID
WINAPI
ClosePseudoConsole(
    _In_ HPCON hPC
    );

#endif

#if (NTDDI_VERSION >= NTDDI_WIN11_GE)

WINBASEAPI
HRESULT
WINAPI
ReleasePseudoConsole(
    _In_ HPCON hPC
    );

#endif

#endif // WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)
#pragma endregion

#ifdef __cplusplus
}
#endif

#endif // _APISETCONSOLE_
// end_consoleapi_h
