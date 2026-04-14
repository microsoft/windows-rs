/*++

Copyright (c) Microsoft Corporation

Abstract:

    This module contains the public Software Device API

*/

#ifndef _SWDEVICE_H_
#define _SWDEVICE_H_

#if _MSC_VER > 1000
#pragma once
#endif

#include <winapifamily.h>

#include <devpropdef.h>
#include <swdevicedef.h>

#if (NTDDI_VERSION >= NTDDI_WIN8)

#if defined(__cplusplus)
extern "C" {
#endif // defined(__cplusplus)

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

DECLARE_HANDLE(HSWDEVICE);
typedef HSWDEVICE *PHSWDEVICE;

typedef VOID (WINAPI *SW_DEVICE_CREATE_CALLBACK)(
    _In_ HSWDEVICE hSwDevice,
    _In_ HRESULT CreateResult,
    _In_opt_ PVOID pContext,
    _In_opt_ PCWSTR pszDeviceInstanceId);

_Check_return_ HRESULT WINAPI
SwDeviceCreate(
    _In_ PCWSTR pszEnumeratorName,
    _In_ PCWSTR pszParentDeviceInstance,
    _In_ const SW_DEVICE_CREATE_INFO *pCreateInfo,
    _In_ ULONG cPropertyCount,
    _In_reads_opt_(cPropertyCount) const DEVPROPERTY *pProperties,
    _In_ SW_DEVICE_CREATE_CALLBACK pCallback,
    _In_opt_ PVOID pContext,
    _Out_ PHSWDEVICE phSwDevice);

VOID WINAPI
SwDeviceClose(
    _In_ HSWDEVICE hSwDevice);

#if (NTDDI_VERSION >= NTDDI_WINBLUE)

_Check_return_ HRESULT WINAPI
SwDeviceSetLifetime(
    _In_ HSWDEVICE hSwDevice,
    _In_ SW_DEVICE_LIFETIME Lifetime);

_Check_return_ HRESULT WINAPI
SwDeviceGetLifetime(
    _In_ HSWDEVICE hSwDevice,
    _Out_ PSW_DEVICE_LIFETIME pLifetime);

#endif

_Check_return_ HRESULT WINAPI
SwDevicePropertySet(
    _In_ HSWDEVICE hSwDevice,
    _In_ ULONG cPropertyCount,
    _In_reads_(cPropertyCount) const DEVPROPERTY *pProperties);

_Check_return_ HRESULT WINAPI
SwDeviceInterfaceRegister(
    _In_ HSWDEVICE hSwDevice,
    _In_ const GUID *pInterfaceClassGuid,
    _In_opt_ PCWSTR pszReferenceString,
    _In_ ULONG cPropertyCount,
    _In_reads_opt_(cPropertyCount) const DEVPROPERTY *pProperties,
    _In_ BOOL fEnabled,
    _Outptr_result_maybenull_ PWSTR* ppszDeviceInterfaceId);

VOID WINAPI
SwMemFree(
    _In_ PVOID pMem);

_Check_return_ HRESULT WINAPI
SwDeviceInterfaceSetState(
    _In_ HSWDEVICE hSwDevice,
    _In_ PCWSTR pszDeviceInterfaceId,
    _In_ BOOL fEnabled);

_Check_return_ HRESULT WINAPI
SwDeviceInterfacePropertySet(
    _In_ HSWDEVICE hSwDevice,
    _In_ PCWSTR pszDeviceInterfaceId,
    _In_ ULONG cPropertyCount,
    _In_reads_(cPropertyCount) const DEVPROPERTY *pProperties);

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#if defined(__cplusplus)
}
#endif // defined(__cplusplus)

#endif // NTDDI_VERSION
#endif // _SWDEVICE_H_