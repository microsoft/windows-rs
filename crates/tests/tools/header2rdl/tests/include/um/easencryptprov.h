/*++ 

Copyright (c) Microsoft Corporation

Module Name:

    EasEncryptProv.h

Abstract:

    This file contains structures, function signatures for 3rd Party 
    Encryption providers that intend to support EAS

Environment:

    User Mode - Win32

Notes:

--*/

//
// User Class
//

#if NTDDI_VERSION >= NTDDI_WINBLUE 

#ifndef _EAS_ENCRYPT_PROV_
#define _EAS_ENCRYPT_PROV_

#ifdef _MSC_VER
#pragma once
#endif

#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

#ifdef __cplusplus
extern "C" {
#endif

typedef enum _EasEncryptProvProtectionStatus_
{
    EasEncryptProvDeviceStatusUnknown,
    EasEncryptProvDeviceNotProtected,
    EasEncryptProvDeviceProtected
} EasEncryptProvProtectionStatus;

typedef
HRESULT
(WINAPI *PFEnCrypQueryProtectionStatus) (
    _Out_ EasEncryptProvProtectionStatus *pDeviceStatus,
    _Out_ HRESULT *phrExtendedStatus
    );

typedef
HRESULT
(WINAPI *PFEncryptProvIsDeviceLockable) ();

typedef
HRESULT
(WINAPI *PFEncryptProvLockDevice) ();

typedef
HRESULT
(WINAPI *PFEncryptProvValidateDeviceLockoutState) ();

typedef
HRESULT
(WINAPI *pFEncryptProvDisableDeviceLockoutState) ();

typedef
HRESULT
(WINAPI *PFEncryptProvGetDeviceLockoutData) (
    _Out_writes_bytes_opt_(*pPerUserSize) PBYTE pPerUserData,
    _Inout_ ULONG *pPerUserSize
    );

typedef
HRESULT
(WINAPI *PFEncryptProvUpdateDeviceLockoutState) (
    _In_reads_bytes_(PerUserSize) PBYTE PerUserData,
    _In_ ULONG PerUserSize
    );

HRESULT
WINAPI
EasEncryptProvQueryProtectionStatus (
    _Out_ EasEncryptProvProtectionStatus *pDeviceStatus,
    _Out_ HRESULT *phrExtendedStatus
    );

HRESULT
WINAPI
EasEncryptProvIsDeviceLockable (
    );

HRESULT
WINAPI
EasEncryptProvLockDevice (
    );

HRESULT
WINAPI
EasEncryptProvGetDeviceLockoutData (
    _Out_writes_bytes_opt_(*pPerUserSize) PBYTE pPerUserData,
    _Inout_ ULONG *pPerUserSize
    );

HRESULT
WINAPI
EasEncryptProvValidateDeviceLockoutState (
    );

HRESULT
WINAPI
EasEncryptProvUpdateDeviceLockoutState (
    _In_reads_bytes_(PerUserSize) PBYTE PerUserData,
    _In_ ULONG PerUserSize
    );

HRESULT
WINAPI
EasEncryptProvDisableDeviceLockoutState (
    );

#ifdef __cplusplus
}
#endif

#endif // WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
#pragma endregion

#endif // _EAS_ENCRYPT_PROV_

#endif // NTDDI_VERSION >= NTDDI_WINBLUE 
