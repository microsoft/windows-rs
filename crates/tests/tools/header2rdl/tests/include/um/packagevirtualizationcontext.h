/*******************************************************************************************************
*                                                                                                      *
* PackageVirtualizationContext.h -- ApiSet Contract for ext-ms-win-packagevirtualizationcontext-l1-1-0 *
*                                                                                                      *
* Copyright (c) Microsoft Corporation. All rights reserved.                                            *
*                                                                                                      *
*******************************************************************************************************/

#pragma once

#include <apiset.h>
#include <apisetcconv.h>

#ifdef _CONTRACT_GEN
#include <nt.h>
#include <ntrtl.h>
#include <nturtl.h>
#endif

#include <minwindef.h>
#include <minwinbase.h>

#pragma region Desktop Family

#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

DECLARE_HANDLE(PACKAGE_VIRTUALIZATION_CONTEXT_HANDLE);

#if NTDDI_VERSION >= NTDDI_WIN10_CO

STDAPI
CreatePackageVirtualizationContext(
    _In_opt_ PCWSTR packageFamilyName,
    _Out_ PACKAGE_VIRTUALIZATION_CONTEXT_HANDLE* context
    );

STDAPI
ActivatePackageVirtualizationContext(
    _In_ PACKAGE_VIRTUALIZATION_CONTEXT_HANDLE context,
    _Out_ ULONG_PTR* cookie
    );

STDAPI_(VOID)
ReleasePackageVirtualizationContext(
    _In_ PACKAGE_VIRTUALIZATION_CONTEXT_HANDLE context
    );

STDAPI_(VOID)
DeactivatePackageVirtualizationContext(
    _In_ ULONG_PTR cookie
    );

STDAPI
DuplicatePackageVirtualizationContext(
    _In_ PACKAGE_VIRTUALIZATION_CONTEXT_HANDLE sourceContext,
    _Out_ PACKAGE_VIRTUALIZATION_CONTEXT_HANDLE* destContext
    );

STDAPI_(PACKAGE_VIRTUALIZATION_CONTEXT_HANDLE)
GetCurrentPackageVirtualizationContext(
    );

STDAPI
GetProcessesInVirtualizationContext(
    _In_ PCWSTR packageFamilyName,
    _Out_ UINT* count,
    _Out_ HANDLE** processes
    );

#endif

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */

#pragma endregion

#ifndef ext_ms_win_packagevirtualizationcontext_l1_1_0_query_routines
#define ext_ms_win_packagevirtualizationcontext_l1_1_0_query_routines

//
//Private Extension API Query Routines
//

#ifdef __cplusplus
extern "C" {
#endif

#if NTDDI_VERSION >= NTDDI_WIN10_CO

BOOLEAN
__stdcall
IsCreatePackageVirtualizationContextPresent(
    VOID
    );

BOOLEAN
__stdcall
IsActivatePackageVirtualizationContextPresent(
    VOID
    );

BOOLEAN
__stdcall
IsReleasePackageVirtualizationContextPresent(
    VOID
    );

BOOLEAN
__stdcall
IsDeactivatePackageVirtualizationContextPresent(
    VOID
    );

BOOLEAN
__stdcall
IsDuplicatePackageVirtualizationContextPresent(
    VOID
    );

BOOLEAN
__stdcall
IsGetCurrentPackageVirtualizationContextPresent(
    VOID
    );

BOOLEAN
__stdcall
IsGetProcessesInVirtualizationContextPresent(
    VOID
    );

#endif

#ifdef __cplusplus
}
#endif

#endif // endof guard

