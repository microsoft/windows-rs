/********************************************************************************
*                                                                               *
* consoleapi.h -- ApiSet Contract for api-ms-win-core-namespace-l1              *
*                                                                               *
* Copyright (c) Microsoft Corporation. All rights reserved.                     *
*                                                                               *
********************************************************************************/

#ifdef _MSC_VER
#pragma once
#endif // _MSC_VER

#ifndef _APISETNAMESPACE_
#define _APISETNAMESPACE_

#include <apiset.h>
#include <apisetcconv.h>
#include <minwindef.h>
#include <minwinbase.h>

#ifdef __cplusplus
extern "C" {
#endif

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)

#define PRIVATE_NAMESPACE_FLAG_DESTROY      0x00000001

WINBASEAPI
HANDLE
WINAPI
CreatePrivateNamespaceW(
    _In_opt_ LPSECURITY_ATTRIBUTES lpPrivateNamespaceAttributes,
    _In_ LPVOID lpBoundaryDescriptor,
    _In_ LPCWSTR lpAliasPrefix
    );

WINBASEAPI
HANDLE
WINAPI
OpenPrivateNamespaceW(
    _In_ LPVOID lpBoundaryDescriptor,
    _In_ LPCWSTR lpAliasPrefix
    );

WINBASEAPI
BOOLEAN
WINAPI
ClosePrivateNamespace(
    _In_ HANDLE Handle,
    _In_ ULONG Flags
    );

WINBASEAPI
HANDLE
WINAPI
CreateBoundaryDescriptorW(
    _In_ LPCWSTR Name,
    _In_ ULONG Flags
    );

WINBASEAPI
BOOL
WINAPI
AddSIDToBoundaryDescriptor(
    _Inout_ HANDLE* BoundaryDescriptor,
    _In_ PSID RequiredSid
    );

WINBASEAPI
VOID
WINAPI
DeleteBoundaryDescriptor(
    _In_ HANDLE BoundaryDescriptor
    );

#endif // WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)
#pragma endregion

#ifdef __cplusplus
}
#endif

#endif // _APISETNAMESPACE_
