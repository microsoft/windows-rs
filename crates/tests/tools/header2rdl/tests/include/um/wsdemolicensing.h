/*
 * Windows demo licensing API
 *
 * External definitions for Windows demo licensing API
 *
 *  Copyright (c) Microsoft Corporation. All Rights Reserved.
 */

#ifndef _WS_DEMO_LICENSING_H_
#define _WS_DEMO_LICENSING_H_

#if _MSC_VER > 1000
    #pragma once
#endif

#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

#if (NTDDI_VERSION >= NTDDI_WIN8)


STDAPI
AddDemoAppLicense(
    _In_                        UINT                        cbLicenseBlob,
    _In_reads_bytes_(cbLicenseBlob)  CONST BYTE*            pbLicenseBlob
    );

STDAPI
RemoveDemoAppLicense(
    _In_                        LPCWSTR                     pwszPackageFamilyName
    );


#endif

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif
