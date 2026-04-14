/*
 * Developer licensing API
 *
 * External definitions for developer licensing API
 *
 *  Copyright (c) Microsoft Corporation. All Rights Reserved.
 */

#ifndef _WS_DEV_LICENSING_H_
#define _WS_DEV_LICENSING_H_

#if _MSC_VER > 1000
    #pragma once
#endif

#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

#if (NTDDI_VERSION >= NTDDI_WIN8)

STDAPI
CheckDeveloperLicense(
    _Out_    FILETIME                                               *pExpiration
    );

STDAPI
AcquireDeveloperLicense(
    _In_opt_    HWND                                                hwndParent,
    _Out_       FILETIME                                            *pExpiration
    );

STDAPI
RemoveDeveloperLicense(
    _In_opt_ HWND                                                   hwndParent
    );

#endif

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif
