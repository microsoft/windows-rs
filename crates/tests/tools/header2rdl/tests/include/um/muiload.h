/*++

Copyright (c) Microsoft Corporation

Module Name:

    MUILoad.h

Abstract:

    Public include file containing API declarations with Language Neutral
    resource module loading capabilities and downlevel compatibility

--*/

#ifndef _MUILOAD_H_INCLUDED_
#define _MUILOAD_H_INCLUDED_

#pragma once
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)



#include <windows.h>

// Language convention flags
#define MUI_LANGUAGE_ID          0x4      // Use traditional language ID convention
#define MUI_LANGUAGE_NAME        0x8      // Use ISO language (culture) name convention
#define MUI_LANGUAGE_EXACT      0x10      // Don't fall back if language module is not found

#ifdef  __cplusplus
extern "C" {
#endif

/*++
LoadMUILibrary

Routine Description:

    This function is meant as a replacement for existing calls to
    LoadLibrary/LoadLibraryEx for components that need to provide
    support for Language neutral scenarios on Vista, Windows Server 2008 as well
    as older versions of the OS.
    It adds a wrapper around the actual call to LoadLibraryEx
    to search for MUI versions of the module in different
    locations depending on the language and the OS.
    The semantics are the same as for a call to LoadLibraryEx,
    including the return value and the error codes.


Arguments:

    pszFullModuleName - file name of the module (with or without path)
        originally containing the resources, as called by LoadLibraryEx().
        This filename is used to build the appropriate Language Neutral
        filename depending on the OS version.

    dwLangConvention - used to let the API know which installation
    convention was used to setup the files.
        Possible values:
            MUI_LANGUAGE_NAME  - (default) use ISO language name convention
            MUI_LANGUAGE_ID    - use traditional LCID language name convention

        Options:
            MUI_LANGUAGE_EXACT - When wLangID is present and the associated library is not found,
                                 return NULL rather than falling back to the module at pwszModuleName

    LangID - if nonzero, the caller is looking for an arbitrary Lang ID path
               specified by this parameter

Return Value:

    Handle to the mapped resource module if found.
    Otherwise, returns NULL.

--*/

// ANSI version
HINSTANCE __stdcall LoadMUILibraryA(
    _In_ PCSTR pszFullModuleName,
    _In_ DWORD dwLangConvention,
    _In_ LANGID LangID
    );

// Wide char version
HINSTANCE __stdcall LoadMUILibraryW(
    _In_ PCWSTR pszFullModuleName,
    _In_ DWORD dwLangConvention,
    _In_ LANGID LangID
    );

#ifdef UNICODE
#define LoadMUILibrary LoadMUILibraryW
#else
#define LoadMUILibrary LoadMUILibraryA
#endif

/*++
FreeMUILibrary

Routine Description:

    This function should be used the same way as FreeLibrary() to
    free the handle returned by LoadMUILibrary().
    It is provided in order to reproduce the LoadLibrary/FreeLibrary
    mechanism; any call to LoadMUILibrary should be
    paired with a call to FreeMUILibrary.


Arguments:

    hResModule - handle to the module loaded with LoadMUILibrary.


Return Value:

    If the function succeeds, the return value is nonzero.
    If the function fails, the return value is zero. To get extended error
    information, call GetLastError.

--*/
BOOL __stdcall FreeMUILibrary(
    _In_ HMODULE hResModule
    );

/*++
GetUILanguageFallbackList

Routine Description:

    This function returns the UI language fallback list in the provided
    buffer 'pFallbackList'.

    To find the length of the buffer needed to contain the list, call with
    'pFallbackList' set to NULL and 'cchFallbackList' set to 0, and the
    required length will be supplied via 'pcchFallbackOut'.

Arguments:

    pFallbackList   - On successful return, contains a NULL-delimited list
                      of UI Language names e.g. 'tk-TM'.  This list is double-null
                      terminated.

    cchFallbackList - The size in characters of the provided buffer,
                      'pFallbackList'

    pcchFallbackOut - If 'pFallbackList' is large enough to hold the list,
                      this will be the number of characters copied into
                      the buffer.  Otherwise, this will return the number of
                      size in characters necessary to hold the list.

Return Value:

    TRUE on success
    FALSE on failure

    To get extended error information, call GetLastError.

--*/
BOOL __stdcall GetUILanguageFallbackList(
    _Out_writes_opt_(cchFallbackList)   PWSTR       pFallbackList,
    _In_                                ULONG       cchFallbackList,
    _Out_opt_                           PULONG      pcchFallbackOut
    );

#ifdef  __cplusplus
}
#endif


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif

