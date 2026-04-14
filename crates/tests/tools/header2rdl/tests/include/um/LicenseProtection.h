/****************************************************************************************
*                                                                                       *
* licenseprotection.h - ApiSet Contract for api-ms-win-security-licenseprotection-l1    *
*                                                                                       *
* Copyright (c) Microsoft Corporation. All rights reserved.                             *
*                                                                                       *
*****************************************************************************************/

#pragma once

#include <apiset.h>
#include <windows.h>

#pragma region Desktop Family or SecureStartup Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

#if (NTDDI_VERSION >= NTDDI_WIN10_MN)

typedef enum _LicenseProtectionStatus
{
    Success,
    LicenseKeyNotFound,
    LicenseKeyUnprotected,
    LicenseKeyCorrupted,
    LicenseKeyAlreadyExists,
} LicenseProtectionStatus;

STDAPI
RegisterLicenseKeyWithExpiration(
    _In_ PCWSTR licenseKey,
    _In_ UINT32 validityInDays,
    _Out_ LicenseProtectionStatus* status
    );

STDAPI
ValidateLicenseKeyProtection(
    _In_ PCWSTR licenseKey,
    _Out_ PFILETIME notValidBefore,
    _Out_ PFILETIME notValidAfter,
    _Out_ LicenseProtectionStatus* status
    );

#endif // (NTDDI_VERSION >= NTDDI_WIN10_MN)

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion
