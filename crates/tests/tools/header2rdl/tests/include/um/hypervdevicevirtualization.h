// HyperVDeviceVirtualization.h: ApiSet Contract for ext-ms-win-hyperv-devicevirtualization-l1-1
// Copyright (c) Microsoft Corporation. All rights reserved.

#ifndef _HYPERV_DEVICE_VIRTUALIZATION_H_
#define _HYPERV_DEVICE_VIRTUALIZATION_H_

#if defined(_MSC_VER) && (_MSC_VER >= 1200)
#pragma once
#endif

#include <apiset.h>
#include <apisetcconv.h>
#include <minwindef.h>
#include <winapifamily.h>

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

#include <ComputeDefs.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef void* HDV_HOST;
typedef void* HDV_DEVICE;

typedef enum HDV_DEVICE_TYPE
{
    HdvDeviceTypeUndefined = 0,
    HdvDeviceTypePCI

} HDV_DEVICE_TYPE;

typedef enum HDV_DEVICE_HOST_FLAGS
{
    HdvDeviceHostFlagNone = 0,
    HdvDeviceHostFlagInitializeComSecurity = 1,
} HDV_DEVICE_HOST_FLAGS;
DEFINE_ENUM_FLAG_OPERATORS(HDV_DEVICE_HOST_FLAGS);

#ifndef _HDV_COMMON_DEFINITIONS_
#define _HDV_COMMON_DEFINITIONS_

typedef struct HDV_PCI_PNP_ID
{
    UINT16 VendorID;
    UINT16 DeviceID;
    UINT8 RevisionID;
    UINT8 ProgIf;
    UINT8 SubClass;
    UINT8 BaseClass;
    UINT16 SubVendorID;
    UINT16 SubSystemID;

} HDV_PCI_PNP_ID, *PHDV_PCI_PNP_ID;

typedef enum HDV_PCI_BAR_SELECTOR
{
    HDV_PCI_BAR0 = 0,
    HDV_PCI_BAR1,
    HDV_PCI_BAR2,
    HDV_PCI_BAR3,
    HDV_PCI_BAR4,
    HDV_PCI_BAR5

} HDV_PCI_BAR_SELECTOR;

typedef enum HDV_DOORBELL_FLAGS
{
    HDV_DOORBELL_FLAG_TRIGGER_SIZE_ANY   = 0,
    HDV_DOORBELL_FLAG_TRIGGER_SIZE_BYTE  = 1,
    HDV_DOORBELL_FLAG_TRIGGER_SIZE_WORD  = 2,
    HDV_DOORBELL_FLAG_TRIGGER_SIZE_DWORD = 3,
    HDV_DOORBELL_FLAG_TRIGGER_SIZE_QWORD = 4,
    HDV_DOORBELL_FLAG_TRIGGER_ANY_VALUE  = 0x80000000
} HDV_DOORBELL_FLAGS;

typedef enum HDV_MMIO_MAPPING_FLAGS
{
    HdvMmioMappingFlagNone = 0x00000000,
    HdvMmioMappingFlagWriteable = 0x00000001,
    HdvMmioMappingFlagExecutable = 0x00000002

} HDV_MMIO_MAPPING_FLAGS;

DEFINE_ENUM_FLAG_OPERATORS(HDV_MMIO_MAPPING_FLAGS);

#endif // _HDV_COMMON_DEFINITIONS_

#define HDV_PCI_BAR_COUNT 6

HRESULT
WINAPI
HdvInitializeDeviceHost(
    _In_ HCS_SYSTEM computeSystem,
    _Out_ HDV_HOST* deviceHostHandle
    );

HRESULT
WINAPI
HdvInitializeDeviceHostEx(
    _In_ HCS_SYSTEM computeSystem,
    _In_ HDV_DEVICE_HOST_FLAGS flags,
    _Out_ HDV_HOST* deviceHostHandle
    );

HRESULT
WINAPI
HdvTeardownDeviceHost(
    _In_ HDV_HOST deviceHostHandle
    );

HRESULT
WINAPI
HdvCreateDeviceInstance(
    _In_ HDV_HOST deviceHostHandle,
    _In_ HDV_DEVICE_TYPE deviceType,
    _In_ const GUID* deviceClassId,
    _In_ const GUID* deviceInstanceId,
    _In_ const VOID* deviceInterface,
    _In_opt_ VOID* deviceContext,
    _Out_ HDV_DEVICE* deviceHandle
    );

HRESULT
WINAPI
HdvReadGuestMemory(
    _In_ HDV_DEVICE requestor,
    _In_ UINT64 guestPhysicalAddress,
    _In_ UINT32 byteCount,
    _Out_writes_(byteCount) BYTE* buffer
    );

HRESULT
WINAPI
HdvWriteGuestMemory(
    _In_ HDV_DEVICE requestor,
    _In_ UINT64 guestPhysicalAddress,
    _In_ UINT32 byteCount,
    _In_reads_(byteCount) const BYTE* buffer
    );

HRESULT
WINAPI
HdvCreateGuestMemoryAperture(
    _In_ HDV_DEVICE requestor,
    _In_ UINT64 guestPhysicalAddress,
    _In_ UINT32 byteCount,
    _In_ BOOL writeProtected,
    _Out_ PVOID* mappedAddress
    );

HRESULT
WINAPI
HdvDestroyGuestMemoryAperture(
    _In_ HDV_DEVICE requestor,
    _In_ PVOID mappedAddress
    );

HRESULT
WINAPI
HdvDeliverGuestInterrupt(
    _In_ HDV_DEVICE requestor,
    _In_ UINT64 msiAddress,
    _In_ UINT32 msiData
    );

HRESULT
WINAPI
HdvRegisterDoorbell(
    _In_ HDV_DEVICE requestor,
    _In_ HDV_PCI_BAR_SELECTOR BarIndex,
    _In_ UINT64 BarOffset,
    _In_ UINT64 TriggerValue,
    _In_ UINT64 Flags,
    _In_ HANDLE DoorbellEvent
    );

HRESULT
WINAPI
HdvUnregisterDoorbell(
    _In_ HDV_DEVICE requestor,
    _In_ HDV_PCI_BAR_SELECTOR BarIndex,
    _In_ UINT64 BarOffset,
    _In_ UINT64 TriggerValue,
    _In_ UINT64 Flags
    );

HRESULT
WINAPI
HdvCreateSectionBackedMmioRange(
    _In_ HDV_DEVICE requestor,
    _In_ HDV_PCI_BAR_SELECTOR barIndex,
    _In_ UINT64 offsetInPages,
    _In_ UINT64 lengthInPages,
    _In_ HDV_MMIO_MAPPING_FLAGS MappingFlags,
    _In_ HANDLE sectionHandle,
    _In_ UINT64 sectionOffsetInPages
    );

HRESULT
WINAPI
HdvDestroySectionBackedMmioRange(
    _In_ HDV_DEVICE requestor,
    _In_ HDV_PCI_BAR_SELECTOR barIndex,
    _In_ UINT64 offsetInPages
    );

//
// PCI device interface.
//

typedef HRESULT (CALLBACK *HDV_PCI_DEVICE_INITIALIZE)(
    _In_opt_ PVOID deviceContext
    );

typedef void (CALLBACK *HDV_PCI_DEVICE_TEARDOWN)(
    _In_opt_ PVOID deviceContext
    );

typedef HRESULT (CALLBACK *HDV_PCI_DEVICE_SET_CONFIGURATION)(
    _In_opt_ PVOID deviceContext,
    _In_ UINT32 configurationValueCount,
    _In_reads_(configurationValueCount) const LPCWSTR* configurationValues
    );

typedef HRESULT (CALLBACK *HDV_PCI_DEVICE_GET_DETAILS)(
    _In_opt_ PVOID deviceContext,
    _Out_ PHDV_PCI_PNP_ID pnpId,
    _In_ UINT32 probedBarsCount,
    _Out_writes_(probedBarsCount) UINT32* probedBars
    );

typedef HRESULT (CALLBACK *HDV_PCI_DEVICE_START)(
    _In_opt_ PVOID deviceContext
    );

typedef void (CALLBACK *HDV_PCI_DEVICE_STOP)(
    _In_opt_ PVOID deviceContext
    );

typedef HRESULT (CALLBACK *HDV_PCI_READ_CONFIG_SPACE)(
    _In_opt_ PVOID deviceContext,
    _In_ UINT32 offset,
    _Out_ UINT32* value
    );

typedef HRESULT (CALLBACK *HDV_PCI_WRITE_CONFIG_SPACE)(
    _In_opt_ PVOID deviceContext,
    _In_ UINT32 offset,
    _In_ UINT32 value
    );

typedef HRESULT (CALLBACK *HDV_PCI_READ_INTERCEPTED_MEMORY)(
    _In_opt_ PVOID deviceContext,
    _In_ HDV_PCI_BAR_SELECTOR barIndex,
    _In_ UINT64 offset,
    _In_ UINT64 length,
    _Out_writes_(length) BYTE* value
    );

typedef HRESULT (CALLBACK *HDV_PCI_WRITE_INTERCEPTED_MEMORY)(
    _In_opt_ PVOID deviceContext,
    _In_ HDV_PCI_BAR_SELECTOR barIndex,
    _In_ UINT64 offset,
    _In_ UINT64 length,
    _In_reads_(length) const BYTE* value
    );

typedef enum HDV_PCI_INTERFACE_VERSION
{
    HdvPciDeviceInterfaceVersionInvalid = 0,
    HdvPciDeviceInterfaceVersion1 = 1

} HDV_PCI_INTERFACE_VERSION;

typedef struct HDV_PCI_DEVICE_INTERFACE
{
    HDV_PCI_INTERFACE_VERSION Version;
    HDV_PCI_DEVICE_INITIALIZE Initialize;
    HDV_PCI_DEVICE_TEARDOWN Teardown;
    HDV_PCI_DEVICE_SET_CONFIGURATION SetConfiguration;
    HDV_PCI_DEVICE_GET_DETAILS GetDetails;
    HDV_PCI_DEVICE_START Start;
    HDV_PCI_DEVICE_STOP Stop;
    HDV_PCI_READ_CONFIG_SPACE ReadConfigSpace;
    HDV_PCI_WRITE_CONFIG_SPACE WriteConfigSpace;
    HDV_PCI_READ_INTERCEPTED_MEMORY ReadInterceptedMemory;
    HDV_PCI_WRITE_INTERCEPTED_MEMORY WriteInterceptedMemory;

} HDV_PCI_DEVICE_INTERFACE, *PHDV_PCI_DEVICE_INTERFACE;

#ifdef __cplusplus
}
#endif

#endif // WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)
#pragma endregion

#endif // _HYPERV_DEVICE_VIRTUALIZATION_H_

#ifndef ext_ms_win_hyperv_devicevirtualization_l1_2_2_query_routines
#define ext_ms_win_hyperv_devicevirtualization_l1_2_2_query_routines

//
//Private Extension API Query Routines
//

#ifdef __cplusplus
extern "C" {
#endif

BOOLEAN
__stdcall
IsHdvInitializeDeviceHostPresent(
    VOID
    );

BOOLEAN
__stdcall
IsHdvInitializeDeviceHostExPresent(
    VOID
    );

BOOLEAN
__stdcall
IsHdvTeardownDeviceHostPresent(
    VOID
    );

BOOLEAN
__stdcall
IsHdvCreateDeviceInstancePresent(
    VOID
    );

BOOLEAN
__stdcall
IsHdvReadGuestMemoryPresent(
    VOID
    );

BOOLEAN
__stdcall
IsHdvWriteGuestMemoryPresent(
    VOID
    );

BOOLEAN
__stdcall
IsHdvCreateGuestMemoryAperturePresent(
    VOID
    );

BOOLEAN
__stdcall
IsHdvDestroyGuestMemoryAperturePresent(
    VOID
    );

BOOLEAN
__stdcall
IsHdvDeliverGuestInterruptPresent(
    VOID
    );

BOOLEAN
__stdcall
IsHdvRegisterDoorbellPresent(
    VOID
    );

BOOLEAN
__stdcall
IsHdvUnregisterDoorbellPresent(
    VOID
    );

BOOLEAN
__stdcall
IsHdvCreateSectionBackedMmioRangePresent(
    VOID
    );

BOOLEAN
__stdcall
IsHdvDestroySectionBackedMmioRangePresent(
    VOID
    );

#ifdef __cplusplus
}
#endif

#endif // endof guard

