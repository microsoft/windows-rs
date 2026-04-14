/********************************************************************************
*                                                                               *
* psmapp.h - ApiSet Contract for api-ms-win-core-psm-app-l1                     *
*                                                                               *
* Copyright (c) Microsoft Corporation. All rights reserved.                     *
*                                                                               *
********************************************************************************/

#ifndef _APISET_PSMAPPNOTIFY_H_
#define _APISET_PSMAPPNOTIFY_H_

#include <apiset.h>
#include <apisetcconv.h>

#ifdef _CONTRACT_GEN        // Header(s) needed for contract generation only.
#include <windows.h>

#define PSM_APP_API_HOST
#endif

#ifdef __cplusplus
extern "C" {
#endif

#undef APICONTRACT
#ifndef PSM_APP_API_HOST
#define APICONTRACT         DECLSPEC_IMPORT
#else
#define APICONTRACT
#endif

#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_GAMES)

typedef
VOID
(*PAPPSTATE_CHANGE_ROUTINE) (
    _In_ BOOLEAN Quiesced,
    _In_ PVOID Context
    );

typedef struct _APPSTATE_REGISTRATION *PAPPSTATE_REGISTRATION;

APICONTRACT
ULONG
NTAPI
RegisterAppStateChangeNotification(
    _In_ PAPPSTATE_CHANGE_ROUTINE Routine,
    _In_opt_ PVOID Context,
    _Out_ PAPPSTATE_REGISTRATION* Registration
    );

APICONTRACT
VOID
NTAPI
UnregisterAppStateChangeNotification(
    _Inout_ PAPPSTATE_REGISTRATION Registration
    );

typedef
VOID
(*PAPPCONSTRAIN_CHANGE_ROUTINE) (
    _In_ BOOLEAN Constrained,
    _In_ PVOID Context
);

typedef struct _APPCONSTRAIN_REGISTRATION *PAPPCONSTRAIN_REGISTRATION;

APICONTRACT
ULONG
NTAPI
RegisterAppConstrainedChangeNotification(
    _In_ PAPPCONSTRAIN_CHANGE_ROUTINE Routine,
    _In_opt_ PVOID Context,
    _Out_ PAPPCONSTRAIN_REGISTRATION* Registration
    );

APICONTRACT
VOID
NTAPI
UnregisterAppConstrainedChangeNotification(
    _Inout_ PAPPCONSTRAIN_REGISTRATION Registration
    );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_GAMES) */
#pragma endregion

#ifdef __cplusplus
}
#endif

#endif // _APISET_PSMAPPNOTIFY_H_
