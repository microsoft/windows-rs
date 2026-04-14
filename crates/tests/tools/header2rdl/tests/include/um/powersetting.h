/********************************************************************************
*                                                                               *
* powersetting.h -- ApiSet Contract for api-ms-win-power-setting-l1-1-0         *
*                                                                               *
* Copyright (c) Microsoft Corporation. All rights reserved.                     *
*                                                                               *
********************************************************************************/

#ifdef _MSC_VER
#if (_MSC_VER > 1000)
#pragma once
#endif // _MSC_VER > 1000
#endif // _MSC_VER

#ifndef _POWERSETTING_H_
#define _POWERSETTING_H_

#include <apiset.h>
#include <apisetcconv.h>

#ifdef _CONTRACT_GEN
#include <nt.h>
#include <ntrtl.h>
#include <nturtl.h>
#include <minwindef.h>
#endif

#ifdef __cplusplus
extern "C" {
#endif

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

#ifndef _HPOWERNOTIFY_DEF_
#define _HPOWERNOTIFY_DEF_

typedef PVOID HPOWERNOTIFY, *PHPOWERNOTIFY;

#endif // _HPOWERNOTIFY_DEF_

#if (NTDDI_VERSION >= NTDDI_VISTA)
DWORD
WINAPI
PowerReadACValue(
    _In_opt_ HKEY RootPowerKey,
    _In_opt_ CONST GUID* SchemeGuid,
    _In_opt_ CONST GUID* SubGroupOfPowerSettingsGuid,
    _In_opt_ CONST GUID* PowerSettingGuid,
    _Out_opt_ PULONG Type,
    _Out_writes_bytes_opt_(*BufferSize) LPBYTE Buffer,
    _Inout_opt_ LPDWORD BufferSize
    );

#endif

#if (NTDDI_VERSION >= NTDDI_VISTA)
DWORD
WINAPI
PowerReadDCValue(
    _In_opt_ HKEY RootPowerKey,
    _In_opt_ CONST GUID* SchemeGuid,
    _In_opt_ CONST GUID* SubGroupOfPowerSettingsGuid,
    _In_opt_ CONST GUID* PowerSettingGuid,
    _Out_opt_ PULONG Type,
    _Out_writes_bytes_opt_(*BufferSize) PUCHAR Buffer,
    _Inout_ LPDWORD BufferSize
    );

#endif

#if (NTDDI_VERSION >= NTDDI_VISTA)
DWORD
WINAPI
PowerWriteACValueIndex(
    _In_opt_ HKEY RootPowerKey,
    _In_ CONST GUID* SchemeGuid,
    _In_opt_ CONST GUID* SubGroupOfPowerSettingsGuid,
    _In_opt_ CONST GUID* PowerSettingGuid,
    _In_ DWORD AcValueIndex
    );

#endif

#if (NTDDI_VERSION >= NTDDI_VISTA)
DWORD
WINAPI
PowerWriteDCValueIndex(
    _In_opt_ HKEY RootPowerKey,
    _In_ CONST GUID* SchemeGuid,
    _In_opt_ CONST GUID* SubGroupOfPowerSettingsGuid,
    _In_opt_ CONST GUID* PowerSettingGuid,
    _In_ DWORD DcValueIndex
    );

#endif

#if (NTDDI_VERSION >= NTDDI_VISTA)
DWORD
WINAPI
PowerGetActiveScheme(
    _In_opt_ HKEY UserRootPowerKey,
    _Outptr_ GUID** ActivePolicyGuid
    );

#endif

#if (NTDDI_VERSION >= NTDDI_VISTA)
DWORD
WINAPI
PowerSetActiveScheme(
    _In_opt_ HKEY UserRootPowerKey,
    _In_opt_ CONST GUID* SchemeGuid
    );

#endif

#if (NTDDI_VERSION >= NTDDI_WIN7)
DWORD
WINAPI
PowerSettingRegisterNotification(
    _In_ LPCGUID SettingGuid,
    _In_ DWORD Flags,
    _In_ HANDLE Recipient,
    _Out_ PHPOWERNOTIFY RegistrationHandle
    );

#endif

#if (NTDDI_VERSION >= NTDDI_WIN7)
DWORD
WINAPI
PowerSettingUnregisterNotification(
    _Inout_ HPOWERNOTIFY RegistrationHandle
    );

#endif

#if NTDDI_VERSION >= NTDDI_WIN10_RS5

typedef enum EFFECTIVE_POWER_MODE {
    EffectivePowerModeBatterySaver,
    EffectivePowerModeEnergySaverHighSavings = EffectivePowerModeBatterySaver,
    EffectivePowerModeBetterBattery,
    EffectivePowerModeEnergySaverStandard = EffectivePowerModeBetterBattery,
    EffectivePowerModeBalanced,
    EffectivePowerModeHighPerformance,
    EffectivePowerModeMaxPerformance,   // v1 last supported
    EffectivePowerModeGameMode,
    EffectivePowerModeMixedReality      // v2 last supported
} EFFECTIVE_POWER_MODE;

#define EFFECTIVE_POWER_MODE_V1 (0x00000001)
#define EFFECTIVE_POWER_MODE_V2 (0x00000002)

typedef _Function_class_(EFFECTIVE_POWER_MODE_CALLBACK)
VOID
WINAPI
EFFECTIVE_POWER_MODE_CALLBACK (
    _In_ EFFECTIVE_POWER_MODE Mode,
    _In_opt_ VOID* Context
    );
#endif

#if NTDDI_VERSION >= NTDDI_WIN10_RS5
_Must_inspect_result_
HRESULT
WINAPI
PowerRegisterForEffectivePowerModeNotifications(
    _In_ ULONG Version,
    _In_ EFFECTIVE_POWER_MODE_CALLBACK* Callback,
    _In_opt_ PVOID Context,
    _Outptr_ PVOID* RegistrationHandle
    );

#endif

#if NTDDI_VERSION >= NTDDI_WIN10_RS5
HRESULT
WINAPI
PowerUnregisterFromEffectivePowerModeNotifications(
    _In_ PVOID RegistrationHandle
    );

#endif

#endif // WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)
#pragma endregion

#ifdef __cplusplus
}
#endif

#endif // _POWERSETTING_H_
