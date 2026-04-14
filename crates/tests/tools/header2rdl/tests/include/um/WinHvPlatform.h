/*++

Copyright (c) Microsoft Corporation.  All rights reserved.

Module Name:

    WinHvPlatform.ext

Abstract:

    ApiSet contract for the Windows Hypervisor User-Mode Platform APIs.

--*/

#ifndef _WINHVAPI_H_
#define _WINHVAPI_H_

#if defined(_MSC_VER) && (_MSC_VER >= 1200)
#pragma once
#endif

#include <apiset.h>
#include <apisetcconv.h>
#include <minwindef.h>
#include <winapifamily.h>

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

#include <WinHvPlatformDefs.h>

#if defined(_AMD64_) || defined(_ARM64_)

#ifdef __cplusplus
extern "C" {
#endif

//
// Platform capabilities
//

HRESULT
WINAPI
WHvGetCapability(
    _In_ WHV_CAPABILITY_CODE CapabilityCode,
    _Out_writes_bytes_to_(CapabilityBufferSizeInBytes, *WrittenSizeInBytes) VOID* CapabilityBuffer,
    _In_ UINT32 CapabilityBufferSizeInBytes,
    _Out_opt_ UINT32* WrittenSizeInBytes
    );

HRESULT
WINAPI
WHvCreatePartition(
    _Out_ WHV_PARTITION_HANDLE* Partition
    );

HRESULT
WINAPI
WHvSetupPartition(
    _In_ WHV_PARTITION_HANDLE Partition
    );

HRESULT
WINAPI
WHvResetPartition(
    _In_ WHV_PARTITION_HANDLE Partition
    );

HRESULT
WINAPI
WHvDeletePartition(
    _In_ WHV_PARTITION_HANDLE Partition
    );

HRESULT
WINAPI
WHvGetPartitionProperty(
    _In_ WHV_PARTITION_HANDLE Partition,
    _In_ WHV_PARTITION_PROPERTY_CODE PropertyCode,
    _Out_writes_bytes_to_(PropertyBufferSizeInBytes, *WrittenSizeInBytes) VOID* PropertyBuffer,
    _In_ UINT32 PropertyBufferSizeInBytes,
    _Out_opt_ UINT32* WrittenSizeInBytes
    );

HRESULT
WINAPI
WHvSetPartitionProperty(
    _In_ WHV_PARTITION_HANDLE Partition,
    _In_ WHV_PARTITION_PROPERTY_CODE PropertyCode,
    _In_reads_bytes_(PropertyBufferSizeInBytes) const VOID* PropertyBuffer,
    _In_ UINT32 PropertyBufferSizeInBytes
    );

HRESULT
WINAPI
WHvSuspendPartitionTime(
    _In_ WHV_PARTITION_HANDLE Partition
    );

HRESULT
WINAPI
WHvResumePartitionTime(
    _In_ WHV_PARTITION_HANDLE Partition
    );

//
// Memory Management
//

HRESULT
WINAPI
WHvMapGpaRange(
    _In_ WHV_PARTITION_HANDLE Partition,
    _In_ VOID* SourceAddress,
    _In_ WHV_GUEST_PHYSICAL_ADDRESS GuestAddress,
    _In_ UINT64 SizeInBytes,
    _In_ WHV_MAP_GPA_RANGE_FLAGS Flags
    );

HRESULT
WINAPI
WHvMapGpaRange2(
    _In_ WHV_PARTITION_HANDLE Partition,
    _In_ HANDLE Process,
    _In_ VOID* SourceAddress,
    _In_ WHV_GUEST_PHYSICAL_ADDRESS GuestAddress,
    _In_ UINT64 SizeInBytes,
    _In_ WHV_MAP_GPA_RANGE_FLAGS Flags
    );

HRESULT
WINAPI
WHvUnmapGpaRange(
    _In_ WHV_PARTITION_HANDLE Partition,
    _In_ WHV_GUEST_PHYSICAL_ADDRESS GuestAddress,
    _In_ UINT64 SizeInBytes
    );

HRESULT
WINAPI
WHvTranslateGva(
    _In_ WHV_PARTITION_HANDLE Partition,
    _In_ UINT32 VpIndex,
    _In_ WHV_GUEST_VIRTUAL_ADDRESS Gva,
    _In_ WHV_TRANSLATE_GVA_FLAGS TranslateFlags,
    _Out_ WHV_TRANSLATE_GVA_RESULT* TranslationResult,
    _Out_ WHV_GUEST_PHYSICAL_ADDRESS* Gpa
    );

//
// Virtual Processors
//

HRESULT
WINAPI
WHvCreateVirtualProcessor(
    _In_ WHV_PARTITION_HANDLE Partition,
    _In_ UINT32 VpIndex,
    _In_ UINT32 Flags
    );

HRESULT
WINAPI
WHvCreateVirtualProcessor2(
    _In_ WHV_PARTITION_HANDLE Partition,
    _In_ UINT32 VpIndex,
    _In_reads_(PropertyCount) const WHV_VIRTUAL_PROCESSOR_PROPERTY* Properties,
    _In_ UINT32 PropertyCount
    );

HRESULT
WINAPI
WHvDeleteVirtualProcessor(
    _In_ WHV_PARTITION_HANDLE Partition,
    _In_ UINT32 VpIndex
    );

HRESULT
WINAPI
WHvRunVirtualProcessor(
    _In_ WHV_PARTITION_HANDLE Partition,
    _In_ UINT32 VpIndex,
    _Out_writes_bytes_(ExitContextSizeInBytes) VOID* ExitContext,
    _In_ UINT32 ExitContextSizeInBytes
    );

HRESULT
WINAPI
WHvCancelRunVirtualProcessor(
    _In_ WHV_PARTITION_HANDLE Partition,
    _In_ UINT32 VpIndex,
    _In_ UINT32 Flags
    );

HRESULT
WINAPI
WHvGetVirtualProcessorRegisters(
    _In_ WHV_PARTITION_HANDLE Partition,
    _In_ UINT32 VpIndex,
    _In_reads_(RegisterCount) const WHV_REGISTER_NAME* RegisterNames,
    _In_ UINT32 RegisterCount,
    _Out_writes_(RegisterCount) WHV_REGISTER_VALUE* RegisterValues
    );

HRESULT
WINAPI
WHvSetVirtualProcessorRegisters(
    _In_ WHV_PARTITION_HANDLE Partition,
    _In_ UINT32 VpIndex,
    _In_reads_(RegisterCount) const WHV_REGISTER_NAME* RegisterNames,
    _In_ UINT32 RegisterCount,
    _In_reads_(RegisterCount) const WHV_REGISTER_VALUE* RegisterValues
    );

#if defined(_AMD64_)
#if defined(NTDDI_VERSION) && (NTDDI_VERSION >= NTDDI_WIN10_VB)
#pragma deprecated("WHvGetVirtualProcessorInterruptControllerState is deprecated; use WHvGetVirtualProcessorInterruptControllerState2")
#endif
HRESULT
WINAPI
WHvGetVirtualProcessorInterruptControllerState(
    _In_ WHV_PARTITION_HANDLE Partition,
    _In_ UINT32 VpIndex,
    _Out_writes_bytes_to_(StateSize, *WrittenSize) VOID* State,
    _In_ UINT32 StateSize,
    _Out_opt_ UINT32* WrittenSize
    );

#endif

#if defined(_AMD64_)
#if defined(NTDDI_VERSION) && (NTDDI_VERSION >= NTDDI_WIN10_VB)
#pragma deprecated("WHvSetVirtualProcessorInterruptControllerState is deprecated; use WHvSetVirtualProcessorInterruptControllerState2")
#endif
HRESULT
WINAPI
WHvSetVirtualProcessorInterruptControllerState(
    _In_ WHV_PARTITION_HANDLE Partition,
    _In_ UINT32 VpIndex,
    _In_reads_bytes_(StateSize) const VOID* State,
    _In_ UINT32 StateSize
    );

#endif

HRESULT
WINAPI
WHvRequestInterrupt(
    _In_ WHV_PARTITION_HANDLE Partition,
    _In_ const WHV_INTERRUPT_CONTROL* Interrupt,
    _In_ UINT32 InterruptControlSize
    );

#if defined(_AMD64_)
#if defined(NTDDI_VERSION) && (NTDDI_VERSION >= NTDDI_WIN10_FE)
#pragma deprecated("WHvGetVirtualProcessorXsaveState is deprecated; use WHvGetVirtualProcessorState")
#endif
HRESULT
WINAPI
WHvGetVirtualProcessorXsaveState(
    _In_ WHV_PARTITION_HANDLE Partition,
    _In_ UINT32 VpIndex,
    _Out_writes_bytes_to_(BufferSizeInBytes, *BytesWritten) VOID* Buffer,
    _In_ UINT32 BufferSizeInBytes,
    _Out_ UINT32* BytesWritten
    );

#endif

#if defined(_AMD64_)
#if defined(NTDDI_VERSION) && (NTDDI_VERSION >= NTDDI_WIN10_FE)
#pragma deprecated("WHvSetVirtualProcessorXsaveState is deprecated; use WHvSetVirtualProcessorState")
#endif
HRESULT
WINAPI
WHvSetVirtualProcessorXsaveState(
    _In_ WHV_PARTITION_HANDLE Partition,
    _In_ UINT32 VpIndex,
    _In_reads_bytes_(BufferSizeInBytes) const VOID* Buffer,
    _In_ UINT32 BufferSizeInBytes
    );

#endif

HRESULT
WINAPI
WHvQueryGpaRangeDirtyBitmap(
    _In_ WHV_PARTITION_HANDLE Partition,
    _In_ WHV_GUEST_PHYSICAL_ADDRESS GuestAddress,
    _In_ UINT64 RangeSizeInBytes,
    _Out_writes_bytes_to_opt_(BitmapSizeInBytes, RangeSizeInBytes / 4096 / 64) UINT64* Bitmap,
    _In_ UINT32 BitmapSizeInBytes
    );

HRESULT
WINAPI
WHvGetPartitionCounters(
    _In_ WHV_PARTITION_HANDLE Partition,
    _In_ WHV_PARTITION_COUNTER_SET CounterSet,
    _Out_writes_bytes_to_(BufferSizeInBytes, *BytesWritten) VOID* Buffer,
    _In_ UINT32 BufferSizeInBytes,
    _Out_opt_ UINT32* BytesWritten
    );

HRESULT
WINAPI
WHvGetVirtualProcessorCounters(
    _In_ WHV_PARTITION_HANDLE Partition,
    _In_ UINT32 VpIndex,
    _In_ WHV_PROCESSOR_COUNTER_SET CounterSet,
    _Out_writes_bytes_to_(BufferSizeInBytes, *BytesWritten) VOID* Buffer,
    _In_ UINT32 BufferSizeInBytes,
    _Out_opt_ UINT32* BytesWritten
    );

#if defined(_AMD64_)
#if defined(NTDDI_VERSION) && (NTDDI_VERSION >= NTDDI_WIN10_FE)
#pragma deprecated("WHvGetVirtualProcessorInterruptControllerState2 is deprecated; use WHvGetVirtualProcessorState")
#endif
HRESULT
WINAPI
WHvGetVirtualProcessorInterruptControllerState2(
    _In_ WHV_PARTITION_HANDLE Partition,
    _In_ UINT32 VpIndex,
    _Out_writes_bytes_to_(StateSize, *WrittenSize) VOID* State,
    _In_ UINT32 StateSize,
    _Out_opt_ UINT32* WrittenSize
    );

#endif

#if defined(_AMD64_)
#if defined(NTDDI_VERSION) && (NTDDI_VERSION >= NTDDI_WIN10_FE)
#pragma deprecated("WHvSetVirtualProcessorInterruptControllerState2 is deprecated; use WHvSetVirtualProcessorState")
#endif
HRESULT
WINAPI
WHvSetVirtualProcessorInterruptControllerState2(
    _In_ WHV_PARTITION_HANDLE Partition,
    _In_ UINT32 VpIndex,
    _In_reads_bytes_(StateSize) const VOID* State,
    _In_ UINT32 StateSize
    );

#endif

#if defined(NTDDI_VERSION) && (NTDDI_VERSION >= NTDDI_WIN10_FE)
#pragma deprecated("WHvRegisterPartitionDoorbellEvent is deprecated; use WHvCreateNotificationPort")
#endif
HRESULT
WINAPI
WHvRegisterPartitionDoorbellEvent(
    _In_ WHV_PARTITION_HANDLE Partition,
    _In_ const WHV_DOORBELL_MATCH_DATA* MatchData,
    _In_ HANDLE EventHandle
    );

#if defined(NTDDI_VERSION) && (NTDDI_VERSION >= NTDDI_WIN10_FE)
#pragma deprecated("WHvRegisterPartitionDoorbellEvent is deprecated; use WHvDeleteNotificationPort")
#endif
HRESULT
WINAPI
WHvUnregisterPartitionDoorbellEvent(
    _In_ WHV_PARTITION_HANDLE Partition,
    _In_ const WHV_DOORBELL_MATCH_DATA* MatchData
    );

HRESULT
WINAPI
WHvAdviseGpaRange(
    _In_ WHV_PARTITION_HANDLE Partition,
    _In_reads_(GpaRangesCount) const WHV_MEMORY_RANGE_ENTRY* GpaRanges,
    _In_ UINT32 GpaRangesCount,
    _In_ WHV_ADVISE_GPA_RANGE_CODE Advice,
    _In_reads_bytes_(AdviceBufferSizeInBytes) const VOID* AdviceBuffer,
    _In_ UINT32 AdviceBufferSizeInBytes
    );

HRESULT
WINAPI
WHvReadGpaRange(
    _In_ WHV_PARTITION_HANDLE Partition,
    _In_ UINT32 VpIndex,
    _In_ WHV_GUEST_PHYSICAL_ADDRESS GuestAddress,
    _In_ WHV_ACCESS_GPA_CONTROLS Controls,
    _Out_writes_bytes_(DataSizeInBytes) PVOID Data,
    _In_ UINT32 DataSizeInBytes
    );

HRESULT
WINAPI
WHvWriteGpaRange(
    _In_ WHV_PARTITION_HANDLE Partition,
    _In_ UINT32 VpIndex,
    _In_ WHV_GUEST_PHYSICAL_ADDRESS GuestAddress,
    _In_ WHV_ACCESS_GPA_CONTROLS Controls,
    _In_reads_bytes_(DataSizeInBytes) const VOID* Data,
    _In_ UINT32 DataSizeInBytes
    );

HRESULT
WINAPI
WHvSignalVirtualProcessorSynicEvent(
    _In_ WHV_PARTITION_HANDLE Partition,
    _In_ WHV_SYNIC_EVENT_PARAMETERS SynicEvent,
    _Out_opt_ BOOL* NewlySignaled
    );

HRESULT
WINAPI
WHvGetVirtualProcessorState(
    _In_ WHV_PARTITION_HANDLE Partition,
    _In_ UINT32 VpIndex,
    _In_ WHV_VIRTUAL_PROCESSOR_STATE_TYPE StateType,
    _Out_writes_bytes_to_(BufferSizeInBytes, *BytesWritten) VOID* Buffer,
    _In_ UINT32 BufferSizeInBytes,
    _Out_opt_ UINT32* BytesWritten
    );

HRESULT
WINAPI
WHvSetVirtualProcessorState(
    _In_ WHV_PARTITION_HANDLE Partition,
    _In_ UINT32 VpIndex,
    _In_ WHV_VIRTUAL_PROCESSOR_STATE_TYPE StateType,
    _In_reads_bytes_(BufferSizeInBytes) const VOID* Buffer,
    _In_ UINT32 BufferSizeInBytes
    );

//
// Virtual PCI
//

HRESULT
WINAPI
WHvAllocateVpciResource(
    _In_opt_ const GUID* ProviderId,
    _In_ WHV_ALLOCATE_VPCI_RESOURCE_FLAGS Flags,
    _In_reads_opt_(ResourceDescriptorSizeInBytes) const VOID* ResourceDescriptor,
    _In_ UINT32 ResourceDescriptorSizeInBytes,
    _Out_ HANDLE* VpciResource
    );

HRESULT
WINAPI
WHvCreateVpciDevice(
    _In_ WHV_PARTITION_HANDLE Partition,
    _In_ UINT64 LogicalDeviceId,
    _In_ HANDLE VpciResource,
    _In_ WHV_CREATE_VPCI_DEVICE_FLAGS Flags,
    _In_opt_ HANDLE NotificationEventHandle
    );

HRESULT
WINAPI
WHvDeleteVpciDevice(
    _In_ WHV_PARTITION_HANDLE Partition,
    _In_ UINT64 LogicalDeviceId
    );

HRESULT
WINAPI
WHvGetVpciDeviceProperty(
    _In_ WHV_PARTITION_HANDLE Partition,
    _In_ UINT64 LogicalDeviceId,
    _In_ WHV_VPCI_DEVICE_PROPERTY_CODE PropertyCode,
    _Out_writes_bytes_to_(PropertyBufferSizeInBytes, *WrittenSizeInBytes) VOID* PropertyBuffer,
    _In_ UINT32 PropertyBufferSizeInBytes,
    _Out_opt_ UINT32* WrittenSizeInBytes
    );

HRESULT
WINAPI
WHvGetVpciDeviceNotification(
    _In_ WHV_PARTITION_HANDLE Partition,
    _In_ UINT64 LogicalDeviceId,
    _Out_writes_bytes_(NotificationSizeInBytes) WHV_VPCI_DEVICE_NOTIFICATION* Notification,
    _In_ UINT32 NotificationSizeInBytes
    );

HRESULT
WINAPI
WHvMapVpciDeviceMmioRanges(
    _In_ WHV_PARTITION_HANDLE Partition,
    _In_ UINT64 LogicalDeviceId,
    _Out_ UINT32* MappingCount,
    _Outptr_result_buffer_(*MappingCount) WHV_VPCI_MMIO_MAPPING** Mappings
    );

HRESULT
WINAPI
WHvUnmapVpciDeviceMmioRanges(
    _In_ WHV_PARTITION_HANDLE Partition,
    _In_ UINT64 LogicalDeviceId
    );

HRESULT
WINAPI
WHvSetVpciDevicePowerState(
    _In_ WHV_PARTITION_HANDLE Partition,
    _In_ UINT64 LogicalDeviceId,
    _In_ DEVICE_POWER_STATE PowerState
    );

HRESULT
WINAPI
WHvReadVpciDeviceRegister(
    _In_ WHV_PARTITION_HANDLE Partition,
    _In_ UINT64 LogicalDeviceId,
    _In_ const WHV_VPCI_DEVICE_REGISTER* Register,
    _Out_writes_bytes_(Register->SizeInBytes) VOID* Data
    );

HRESULT
WINAPI
WHvWriteVpciDeviceRegister(
    _In_ WHV_PARTITION_HANDLE Partition,
    _In_ UINT64 LogicalDeviceId,
    _In_ const WHV_VPCI_DEVICE_REGISTER* Register,
    _In_reads_bytes_(Register->SizeInBytes) const VOID* Data
    );

HRESULT
WINAPI
WHvMapVpciDeviceInterrupt(
    _In_ WHV_PARTITION_HANDLE Partition,
    _In_ UINT64 LogicalDeviceId,
    _In_ UINT32 Index,
    _In_ UINT32 MessageCount,
    _In_ const WHV_VPCI_INTERRUPT_TARGET* Target,
    _Out_ UINT64* MsiAddress,
    _Out_ UINT32* MsiData
    );

HRESULT
WINAPI
WHvUnmapVpciDeviceInterrupt(
    _In_ WHV_PARTITION_HANDLE Partition,
    _In_ UINT64 LogicalDeviceId,
    _In_ UINT32 Index
    );

HRESULT
WINAPI
WHvRetargetVpciDeviceInterrupt(
    _In_ WHV_PARTITION_HANDLE Partition,
    _In_ UINT64 LogicalDeviceId,
    _In_ UINT64 MsiAddress,
    _In_ UINT32 MsiData,
    _In_ const WHV_VPCI_INTERRUPT_TARGET* Target
    );

HRESULT
WINAPI
WHvRequestVpciDeviceInterrupt(
    _In_ WHV_PARTITION_HANDLE Partition,
    _In_ UINT64 LogicalDeviceId,
    _In_ UINT64 MsiAddress,
    _In_ UINT32 MsiData
    );

HRESULT
WINAPI
WHvGetVpciDeviceInterruptTarget(
    _In_ WHV_PARTITION_HANDLE Partition,
    _In_ UINT64 LogicalDeviceId,
    _In_ UINT32 Index,
    _In_ UINT32 MultiMessageNumber,
    _Out_writes_bytes_to_(TargetSizeInBytes, *BytesWritten) WHV_VPCI_INTERRUPT_TARGET* Target,
    _In_ UINT32 TargetSizeInBytes,
    _Out_opt_ UINT32* BytesWritten
    );

//
// Trigger
//

HRESULT
WINAPI
WHvCreateTrigger(
    _In_ WHV_PARTITION_HANDLE Partition,
    _In_ const WHV_TRIGGER_PARAMETERS* Parameters,
    _Out_ WHV_TRIGGER_HANDLE* TriggerHandle,
    _Out_ HANDLE* EventHandle
    );

HRESULT
WINAPI
WHvUpdateTriggerParameters(
    _In_ WHV_PARTITION_HANDLE Partition,
    _In_ const WHV_TRIGGER_PARAMETERS* Parameters,
    _In_ WHV_TRIGGER_HANDLE TriggerHandle
    );

HRESULT
WINAPI
WHvDeleteTrigger(
    _In_ WHV_PARTITION_HANDLE Partition,
    _In_ WHV_TRIGGER_HANDLE TriggerHandle
    );

//
// Notification ports
//

HRESULT
WINAPI
WHvCreateNotificationPort(
    _In_ WHV_PARTITION_HANDLE Partition,
    _In_ const WHV_NOTIFICATION_PORT_PARAMETERS* Parameters,
    _In_ HANDLE EventHandle,
    _Out_ WHV_NOTIFICATION_PORT_HANDLE* PortHandle
    );

HRESULT
WINAPI
WHvSetNotificationPortProperty(
    _In_ WHV_PARTITION_HANDLE Partition,
    _In_ WHV_NOTIFICATION_PORT_HANDLE PortHandle,
    _In_ WHV_NOTIFICATION_PORT_PROPERTY_CODE PropertyCode,
    _In_ WHV_NOTIFICATION_PORT_PROPERTY PropertyValue
    );

HRESULT
WINAPI
WHvDeleteNotificationPort(
    _In_ WHV_PARTITION_HANDLE Partition,
    _In_ WHV_NOTIFICATION_PORT_HANDLE PortHandle
    );

HRESULT
WINAPI
WHvPostVirtualProcessorSynicMessage(
    _In_ WHV_PARTITION_HANDLE Partition,
    _In_ UINT32 VpIndex,
    _In_ UINT32 SintIndex,
    _In_reads_bytes_(MessageSizeInBytes) const VOID* Message,
    _In_ UINT32 MessageSizeInBytes
    );

#if defined(_AMD64_)
HRESULT
WINAPI
WHvGetVirtualProcessorCpuidOutput(
    _In_ WHV_PARTITION_HANDLE Partition,
    _In_ UINT32 VpIndex,
    _In_ UINT32 Eax,
    _In_ UINT32 Ecx,
    _Out_ WHV_CPUID_OUTPUT* CpuidOutput
    );

#endif

#if defined(_AMD64_)
HRESULT
WINAPI
WHvGetInterruptTargetVpSet(
    _In_ WHV_PARTITION_HANDLE Partition,
    _In_ UINT64 Destination,
    _In_ WHV_INTERRUPT_DESTINATION_MODE DestinationMode,
    _Out_writes_to_(VpCount, *TargetVpCount) UINT32* TargetVps,
    _In_ UINT32 VpCount,
    _Out_ UINT32* TargetVpCount
    );

#endif

HRESULT
WINAPI
WHvStartPartitionMigration(
    _In_ WHV_PARTITION_HANDLE Partition,
    _Out_ HANDLE* MigrationHandle
    );

HRESULT
WHvCancelPartitionMigration(
    _In_ WHV_PARTITION_HANDLE Partition
    );

HRESULT
WHvCompletePartitionMigration(
    _In_ WHV_PARTITION_HANDLE Partition
    );

HRESULT
WINAPI
WHvAcceptPartitionMigration(
    _In_ HANDLE MigrationHandle,
    _Out_ WHV_PARTITION_HANDLE* Partition
    );

#ifdef __cplusplus
}
#endif

#endif // defined(_AMD64_) || defined(_ARM64_)

#endif // WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)
#pragma endregion

#endif // _WINHVAPI_H_

#ifndef ext_ms_win_hyperv_hvplatform_l1_1_5_query_routines
#define ext_ms_win_hyperv_hvplatform_l1_1_5_query_routines

//
//Private Extension API Query Routines
//

#ifdef __cplusplus
extern "C" {
#endif

#if defined(_AMD64_) || defined(_ARM64_)

BOOLEAN
__stdcall
IsWHvGetCapabilityPresent(
    VOID
    );

BOOLEAN
__stdcall
IsWHvCreatePartitionPresent(
    VOID
    );

BOOLEAN
__stdcall
IsWHvSetupPartitionPresent(
    VOID
    );

BOOLEAN
__stdcall
IsWHvResetPartitionPresent(
    VOID
    );

BOOLEAN
__stdcall
IsWHvDeletePartitionPresent(
    VOID
    );

BOOLEAN
__stdcall
IsWHvGetPartitionPropertyPresent(
    VOID
    );

BOOLEAN
__stdcall
IsWHvSetPartitionPropertyPresent(
    VOID
    );

BOOLEAN
__stdcall
IsWHvSuspendPartitionTimePresent(
    VOID
    );

BOOLEAN
__stdcall
IsWHvResumePartitionTimePresent(
    VOID
    );

BOOLEAN
__stdcall
IsWHvMapGpaRangePresent(
    VOID
    );

BOOLEAN
__stdcall
IsWHvMapGpaRange2Present(
    VOID
    );

BOOLEAN
__stdcall
IsWHvUnmapGpaRangePresent(
    VOID
    );

BOOLEAN
__stdcall
IsWHvTranslateGvaPresent(
    VOID
    );

BOOLEAN
__stdcall
IsWHvCreateVirtualProcessorPresent(
    VOID
    );

BOOLEAN
__stdcall
IsWHvCreateVirtualProcessor2Present(
    VOID
    );

BOOLEAN
__stdcall
IsWHvDeleteVirtualProcessorPresent(
    VOID
    );

BOOLEAN
__stdcall
IsWHvRunVirtualProcessorPresent(
    VOID
    );

BOOLEAN
__stdcall
IsWHvCancelRunVirtualProcessorPresent(
    VOID
    );

BOOLEAN
__stdcall
IsWHvGetVirtualProcessorRegistersPresent(
    VOID
    );

BOOLEAN
__stdcall
IsWHvSetVirtualProcessorRegistersPresent(
    VOID
    );

#if defined(_AMD64_)

#if defined(NTDDI_VERSION) && (NTDDI_VERSION >= NTDDI_WIN10_VB)

#endif

BOOLEAN
__stdcall
IsWHvGetVirtualProcessorInterruptControllerStatePresent(
    VOID
    );

#endif

#if defined(_AMD64_)

#if defined(NTDDI_VERSION) && (NTDDI_VERSION >= NTDDI_WIN10_VB)

#endif

BOOLEAN
__stdcall
IsWHvSetVirtualProcessorInterruptControllerStatePresent(
    VOID
    );

#endif

BOOLEAN
__stdcall
IsWHvRequestInterruptPresent(
    VOID
    );

#if defined(_AMD64_)

#if defined(NTDDI_VERSION) && (NTDDI_VERSION >= NTDDI_WIN10_FE)

#endif

BOOLEAN
__stdcall
IsWHvGetVirtualProcessorXsaveStatePresent(
    VOID
    );

#endif

#if defined(_AMD64_)

#if defined(NTDDI_VERSION) && (NTDDI_VERSION >= NTDDI_WIN10_FE)

#endif

BOOLEAN
__stdcall
IsWHvSetVirtualProcessorXsaveStatePresent(
    VOID
    );

#endif

BOOLEAN
__stdcall
IsWHvQueryGpaRangeDirtyBitmapPresent(
    VOID
    );

BOOLEAN
__stdcall
IsWHvGetPartitionCountersPresent(
    VOID
    );

BOOLEAN
__stdcall
IsWHvGetVirtualProcessorCountersPresent(
    VOID
    );

#if defined(_AMD64_)

#if defined(NTDDI_VERSION) && (NTDDI_VERSION >= NTDDI_WIN10_FE)

#endif

BOOLEAN
__stdcall
IsWHvGetVirtualProcessorInterruptControllerState2Present(
    VOID
    );

#endif

#if defined(_AMD64_)

#if defined(NTDDI_VERSION) && (NTDDI_VERSION >= NTDDI_WIN10_FE)

#endif

BOOLEAN
__stdcall
IsWHvSetVirtualProcessorInterruptControllerState2Present(
    VOID
    );

#endif

#if defined(NTDDI_VERSION) && (NTDDI_VERSION >= NTDDI_WIN10_FE)

#endif

BOOLEAN
__stdcall
IsWHvRegisterPartitionDoorbellEventPresent(
    VOID
    );

#if defined(NTDDI_VERSION) && (NTDDI_VERSION >= NTDDI_WIN10_FE)

#endif

BOOLEAN
__stdcall
IsWHvUnregisterPartitionDoorbellEventPresent(
    VOID
    );

BOOLEAN
__stdcall
IsWHvAdviseGpaRangePresent(
    VOID
    );

BOOLEAN
__stdcall
IsWHvReadGpaRangePresent(
    VOID
    );

BOOLEAN
__stdcall
IsWHvWriteGpaRangePresent(
    VOID
    );

BOOLEAN
__stdcall
IsWHvSignalVirtualProcessorSynicEventPresent(
    VOID
    );

BOOLEAN
__stdcall
IsWHvGetVirtualProcessorStatePresent(
    VOID
    );

BOOLEAN
__stdcall
IsWHvSetVirtualProcessorStatePresent(
    VOID
    );

BOOLEAN
__stdcall
IsWHvAllocateVpciResourcePresent(
    VOID
    );

BOOLEAN
__stdcall
IsWHvCreateVpciDevicePresent(
    VOID
    );

BOOLEAN
__stdcall
IsWHvDeleteVpciDevicePresent(
    VOID
    );

BOOLEAN
__stdcall
IsWHvGetVpciDevicePropertyPresent(
    VOID
    );

BOOLEAN
__stdcall
IsWHvGetVpciDeviceNotificationPresent(
    VOID
    );

BOOLEAN
__stdcall
IsWHvMapVpciDeviceMmioRangesPresent(
    VOID
    );

BOOLEAN
__stdcall
IsWHvUnmapVpciDeviceMmioRangesPresent(
    VOID
    );

BOOLEAN
__stdcall
IsWHvSetVpciDevicePowerStatePresent(
    VOID
    );

BOOLEAN
__stdcall
IsWHvReadVpciDeviceRegisterPresent(
    VOID
    );

BOOLEAN
__stdcall
IsWHvWriteVpciDeviceRegisterPresent(
    VOID
    );

BOOLEAN
__stdcall
IsWHvMapVpciDeviceInterruptPresent(
    VOID
    );

BOOLEAN
__stdcall
IsWHvUnmapVpciDeviceInterruptPresent(
    VOID
    );

BOOLEAN
__stdcall
IsWHvRetargetVpciDeviceInterruptPresent(
    VOID
    );

BOOLEAN
__stdcall
IsWHvRequestVpciDeviceInterruptPresent(
    VOID
    );

BOOLEAN
__stdcall
IsWHvGetVpciDeviceInterruptTargetPresent(
    VOID
    );

BOOLEAN
__stdcall
IsWHvCreateTriggerPresent(
    VOID
    );

BOOLEAN
__stdcall
IsWHvUpdateTriggerParametersPresent(
    VOID
    );

BOOLEAN
__stdcall
IsWHvDeleteTriggerPresent(
    VOID
    );

BOOLEAN
__stdcall
IsWHvCreateNotificationPortPresent(
    VOID
    );

BOOLEAN
__stdcall
IsWHvSetNotificationPortPropertyPresent(
    VOID
    );

BOOLEAN
__stdcall
IsWHvDeleteNotificationPortPresent(
    VOID
    );

BOOLEAN
__stdcall
IsWHvPostVirtualProcessorSynicMessagePresent(
    VOID
    );

#if defined(_AMD64_)

BOOLEAN
__stdcall
IsWHvGetVirtualProcessorCpuidOutputPresent(
    VOID
    );

#endif

#if defined(_AMD64_)

BOOLEAN
__stdcall
IsWHvGetInterruptTargetVpSetPresent(
    VOID
    );

#endif

BOOLEAN
__stdcall
IsWHvStartPartitionMigrationPresent(
    VOID
    );

BOOLEAN
__stdcall
IsWHvCancelPartitionMigrationPresent(
    VOID
    );

BOOLEAN
__stdcall
IsWHvCompletePartitionMigrationPresent(
    VOID
    );

BOOLEAN
__stdcall
IsWHvAcceptPartitionMigrationPresent(
    VOID
    );

#endif // defined(_AMD64_) || defined(_ARM64_)

#ifdef __cplusplus
}
#endif

#endif // endof guard

