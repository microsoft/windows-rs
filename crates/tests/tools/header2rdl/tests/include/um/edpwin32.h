//+-----------------------------------------------------------------------
//
// Microsoft Windows
//
// Copyright (c) Microsoft Corporation
//
// Description: Enterprise Data Protection Win32 APIs
//
//------------------------------------------------------------------------

#ifdef _MSC_VER
#pragma once
#endif

#ifndef _EDPWIN32_H_
#define _EDPWIN32_H_

#ifndef NT_INCLUDED
#include <winnt.h>
#endif /* NT_INCLUDED */

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_PC_APP)

#if NTDDI_VERSION >= NTDDI_WIN10_TH2

STDAPI ProtectFileToEnterpriseIdentity(
    _In_ PCWSTR fileOrFolderPath,
    _In_ PCWSTR identity
    );

#endif

#if NTDDI_VERSION >= NTDDI_WIN10_RS2

typedef struct
{
    bool audit;
}FILE_UNPROTECT_OPTIONS;

STDAPI UnprotectFile(
    _In_ PCWSTR fileOrFolderPath,
    _In_opt_ const FILE_UNPROTECT_OPTIONS* options
    );

#endif

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_PC_APP) */
#pragma endregion

#endif // _EDPWIN32_H_