/*++

Copyright (c) Microsoft Corporation.  All rights reserved.

Module Name:

    cscsearchapiif.hxx

Abstract:

    Public Interface definition for Offline Files Search Assistance.

--*/

#ifndef _INC_CSCSEARCHAPIIF_HXX
#define _INC_CSCSEARCHAPIIF_HXX

#pragma once
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)



class CCscSearchApiInterface
{
public:

    //
    // Memory Management
    //

    virtual
    LONG
    AddRef(
        ) = 0;

    virtual
    LONG
    DelRef(
        ) = 0;

    //
    // CSC Search API Methods
    //

    virtual
    DWORD
    OfflineFilesOpenIndexingHandle(
        _Out_ PHANDLE Handle,
        _In_ PCWSTR FileName,
        _In_ DWORD DesiredAccess,
        _In_ DWORD ShareMode,
        _In_opt_ PBOOL IsDirectoryOrFile
        ) = 0;

};


#define _CSC_SEARCHAPI_INTERFACE_VERSION 0x1
#define _CSC_SEARCHAPI_INTERFACE_COOKIE  0x8fb92809

#define CSC_SEARCHAPI_INTERFACE_VERSION ( ((_CSC_SEARCHAPI_INTERFACE_VERSION) << 16) | (sizeof(CCscSearchApiInterface)) )
#define CSC_SEARCHAPI_INTERFACE_COOKIE (_CSC_SEARCHAPI_INTERFACE_COOKIE)

typedef
NTSTATUS
(*PFUNC_CSC_SEARCHAPI_GET_INTERFACE)(
    _In_ ULONG Version,
    _In_ ULONG Cookie,
    _Out_ CCscSearchApiInterface** Interface
    );

extern "C"
NTSTATUS
CscSearchApiGetInterface(
    _In_ ULONG Version,
    _In_ ULONG Cookie,
    _Out_ CCscSearchApiInterface** Interface
    );


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif // _INC_CSCSEARCHAPIIF_HXX
