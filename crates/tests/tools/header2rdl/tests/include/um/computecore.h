// ComputeCore.h: ApiSet Contract for ext-ms-win-hyperv-compute-l1
// Copyright (c) Microsoft Corporation. All rights reserved.

#pragma once

#ifndef _HYPERV_COMPUTECORE_H_
#define _HYPERV_COMPUTECORE_H_

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

// Enumerates existing compute systems.
HRESULT
WINAPI
HcsEnumerateComputeSystems(
    _In_opt_ PCWSTR query,
    _In_ HCS_OPERATION operation
    );

// Enumerates existing compute systems in a given namespace.
HRESULT
WINAPI
HcsEnumerateComputeSystemsInNamespace(
    _In_ PCWSTR idNamespace,
    _In_opt_ PCWSTR query,
    _In_ HCS_OPERATION operation
    );

// Creates a new operation.
HCS_OPERATION
WINAPI
HcsCreateOperation(
    _In_opt_ const void* context,
    _In_opt_ HCS_OPERATION_COMPLETION callback
    );

// Creates a new operation that allows registration for event notifications
HCS_OPERATION
WINAPI
HcsCreateOperationWithNotifications(
    _In_ HCS_OPERATION_OPTIONS eventTypes,
    _In_opt_ const void* context,
    _In_ HCS_EVENT_CALLBACK callback
    );

// Closes an operation.
void
WINAPI
HcsCloseOperation(
    _In_ HCS_OPERATION operation
    );

// Returns the context pointer of an operation.
void*
WINAPI
HcsGetOperationContext(
    _In_ HCS_OPERATION operation
    );

// Sets the context pointer for an operation.
HRESULT
WINAPI
HcsSetOperationContext(
    _In_ HCS_OPERATION operation,
    _In_opt_ const void* context
    );

// Returns the handle to compute system associated with an operation.
HCS_SYSTEM
WINAPI
HcsGetComputeSystemFromOperation(
    _In_ HCS_OPERATION operation
    );

// Returns the handle to the process associated with an operation
HCS_PROCESS
WINAPI
HcsGetProcessFromOperation(
    _In_ HCS_OPERATION operation
    );

// Returns the type of an operation.
HCS_OPERATION_TYPE
WINAPI
HcsGetOperationType(
    _In_ HCS_OPERATION operation
    );

// Returns the ID that uniquely identifies an operation.
UINT64
WINAPI
HcsGetOperationId(
    _In_ HCS_OPERATION operation
    );

// Returns the result of an operation.
HRESULT
WINAPI
HcsGetOperationResult(
    _In_ HCS_OPERATION operation,
    _Outptr_opt_ PWSTR* resultDocument
    );

// Returns the result of an operation, including the process information for HcsCreateProcess
// and HcsGetProcessInfo.
HRESULT
WINAPI
HcsGetOperationResultAndProcessInfo(
    _In_ HCS_OPERATION operation,
    _Out_opt_ HCS_PROCESS_INFORMATION* processInformation,
    _Outptr_opt_ PWSTR* resultDocument
    );

// Adds a resource to an HCS_OPERATION.
HRESULT
WINAPI
HcsAddResourceToOperation(
    _In_ HCS_OPERATION operation,
    HCS_RESOURCE_TYPE type,
    _In_ PCWSTR uri,
    HANDLE handle
    );

// Returns processor compatibility fields in JSON string format
HRESULT
WINAPI
HcsGetProcessorCompatibilityFromSavedState(
    PCWSTR RuntimeFileName,
    _Outptr_opt_ PCWSTR* ProcessorFeaturesString
    );

// Waits for the completion of an operation and returns the result.
HRESULT
WINAPI
HcsWaitForOperationResult(
    _In_ HCS_OPERATION operation,
    _In_ DWORD timeoutMs,
    _Outptr_opt_ PWSTR* resultDocument
    );

// Waits for the completion of an operation and returns the result, including the process information
// for HcsCreateProcess and HcsGetProcessInfo.
HRESULT
WINAPI
HcsWaitForOperationResultAndProcessInfo(
    _In_ HCS_OPERATION operation,
    _In_ DWORD timeoutMs,
    _Out_opt_ HCS_PROCESS_INFORMATION* processInformation,
    _Outptr_opt_ PWSTR* resultDocument
    );

// Sets a callback that is invoked on completion of an operation.
HRESULT
WINAPI
HcsSetOperationCallback(
    _In_ HCS_OPERATION operation,
    _In_opt_ const void* context,
    _In_ HCS_OPERATION_COMPLETION callback
    );

// Cancels an operation
HRESULT
WINAPI
HcsCancelOperation(
    _In_ HCS_OPERATION operation
    );

// Query for information or properties about a specific operation
HRESULT
WINAPI
HcsGetOperationProperties(
    _In_ HCS_OPERATION operation,
    _In_ PCWSTR options,
    _Outptr_ PWSTR* resultDocument
    );

/// Creates a new compute system.
HRESULT
WINAPI
HcsCreateComputeSystem(
    _In_ PCWSTR id,
    _In_ PCWSTR configuration,
    _In_ HCS_OPERATION operation,
    _In_opt_ const SECURITY_DESCRIPTOR* securityDescriptor,
    _Out_ HCS_SYSTEM* computeSystem
    );

/// Creates a new compute system in a given namespace.
HRESULT
WINAPI
HcsCreateComputeSystemInNamespace(
    _In_ PCWSTR idNamespace,
    _In_ PCWSTR id,
    _In_ PCWSTR configuration,
    _In_ HCS_OPERATION operation,
    _In_opt_ const HCS_CREATE_OPTIONS* options,
    _Out_ HCS_SYSTEM* computeSystem
    );

/// Opens a handle to an existing compute system.
HRESULT
WINAPI
HcsOpenComputeSystem(
    _In_ PCWSTR id,
    _In_ DWORD requestedAccess,
    _Out_ HCS_SYSTEM* computeSystem
    );

/// Opens a handle to an existing compute system in a given namespace.
HRESULT
WINAPI
HcsOpenComputeSystemInNamespace(
    _In_ PCWSTR idNamespace,
    _In_ PCWSTR id,
    _In_ DWORD requestedAccess,
    _Out_ HCS_SYSTEM* computeSystem
    );

/// Closes a handle to a compute system.
void
WINAPI
HcsCloseComputeSystem(
    _In_ _Post_invalid_ HCS_SYSTEM computeSystem
    );

/// Starts a compute system.
HRESULT
WINAPI
HcsStartComputeSystem(
    _In_ HCS_SYSTEM computeSystem,
    _In_ HCS_OPERATION operation,
    _In_opt_ PCWSTR options
    );

/// Cleanly shuts down a compute system.
HRESULT
WINAPI
HcsShutDownComputeSystem(
    _In_ HCS_SYSTEM computeSystem,
    _In_ HCS_OPERATION operation,
    _In_opt_ PCWSTR options
    );

/// Forcefully terminates a compute system.
HRESULT
WINAPI
HcsTerminateComputeSystem(
    _In_ HCS_SYSTEM computeSystem,
    _In_ HCS_OPERATION operation,
    _In_opt_ PCWSTR options
    );

/// Forcefully terminates a compute system.
HRESULT
WINAPI
HcsCrashComputeSystem(
    _In_ HCS_SYSTEM computeSystem,
    _In_ HCS_OPERATION operation,
    _In_opt_ PCWSTR options
    );

/// Pauses the execution of a compute system.
HRESULT
WINAPI
HcsPauseComputeSystem(
    _In_ HCS_SYSTEM computeSystem,
    _In_ HCS_OPERATION operation,
    _In_opt_ PCWSTR options
    );

/// Resumes the execution of a compute system.
HRESULT
WINAPI
HcsResumeComputeSystem(
    _In_ HCS_SYSTEM computeSystem,
    _In_ HCS_OPERATION operation,
    _In_opt_ PCWSTR options
    );

/// Saves the state of a compute system.
HRESULT
WINAPI
HcsSaveComputeSystem(
    _In_ HCS_SYSTEM computeSystem,
    _In_ HCS_OPERATION operation,
    _In_opt_ PCWSTR options
    );

/// Returns properties of a compute system.
HRESULT
WINAPI
HcsGetComputeSystemProperties(
    _In_ HCS_SYSTEM computeSystem,
    _In_ HCS_OPERATION operation,
    _In_opt_ PCWSTR propertyQuery
    );

/// Modifies settings of a compute system.
HRESULT
WINAPI
HcsModifyComputeSystem(
    _In_ HCS_SYSTEM computeSystem,
    _In_ HCS_OPERATION operation,
    _In_ PCWSTR configuration,
    _In_opt_ HANDLE identity
    );

// Waits for a compute system to exit.
HRESULT
WINAPI
HcsWaitForComputeSystemExit(
    _In_ HCS_SYSTEM computeSystem,
    _In_ DWORD timeoutMs,
    _Outptr_opt_ PWSTR* result
    );

/// Registers a callback function to receive notifications for the compute system.
HRESULT
WINAPI
HcsSetComputeSystemCallback(
    _In_ HCS_SYSTEM computeSystem,
    _In_ HCS_EVENT_OPTIONS callbackOptions,
    _In_opt_ const void* context,
    _In_ HCS_EVENT_CALLBACK callback
    );

/// Initialize live migration on source
HRESULT
WINAPI
HcsInitializeLiveMigrationOnSource(
    _In_ HCS_SYSTEM computeSystem,
    _In_ HCS_OPERATION operation,
    _In_opt_ PCWSTR options
    );

/// Start live migration on source
HRESULT
WINAPI
HcsStartLiveMigrationOnSource(
    _In_ HCS_SYSTEM computeSystem,
    _In_ HCS_OPERATION operation,
    _In_opt_ PCWSTR options
    );

/// Start live migration transfer
HRESULT
WINAPI
HcsStartLiveMigrationTransfer(
    _In_ HCS_SYSTEM computeSystem,
    _In_ HCS_OPERATION operation,
    _In_opt_ PCWSTR options
    );

/// Live migration transitions to final state
HRESULT
WINAPI
HcsFinalizeLiveMigration(
    _In_ HCS_SYSTEM computeSystem,
    _In_ HCS_OPERATION operation,
    _In_opt_ PCWSTR options
    );

/// Starts a process in a compute system.
HRESULT
WINAPI
HcsCreateProcess(
    _In_ HCS_SYSTEM computeSystem,
    _In_ PCWSTR processParameters,
    _In_ HCS_OPERATION operation,
    _In_opt_ const SECURITY_DESCRIPTOR* securityDescriptor,
    _Out_ HCS_PROCESS* process
    );

/// Opens an existing process in a compute system
HRESULT
WINAPI
HcsOpenProcess(
    _In_ HCS_SYSTEM computeSystem,
    _In_ DWORD processId,
    _In_ DWORD requestedAccess,
    _Out_ HCS_PROCESS* process
    );

/// Closes the handle to a process in a compute system
void
WINAPI
HcsCloseProcess(
    _In_ HCS_PROCESS process
    );

/// Terminates a process in a compute system
HRESULT
WINAPI
HcsTerminateProcess(
    _In_ HCS_PROCESS process,
    _In_ HCS_OPERATION operation,
    _In_opt_ PCWSTR options
    );

/// Sends a signal to a process in a compute system
HRESULT
WINAPI
HcsSignalProcess(
    _In_ HCS_PROCESS process,
    _In_ HCS_OPERATION operation,
    _In_opt_ PCWSTR options
    );

/// Returns the initial startup info of a process in a compute system
HRESULT
WINAPI
HcsGetProcessInfo(
    _In_ HCS_PROCESS process,
    _In_ HCS_OPERATION operation
    );

/// Returns the properties of a process in a compute system
HRESULT
WINAPI
HcsGetProcessProperties(
    _In_ HCS_PROCESS process,
    _In_ HCS_OPERATION operation,
    _In_opt_ PCWSTR propertyQuery
    );

/// Modifies the parameters of a process in a compute system
HRESULT
WINAPI
HcsModifyProcess(
    _In_ HCS_PROCESS process,
    _In_ HCS_OPERATION operation,
    _In_ PCWSTR settings
    );

/// Registers a callback function to receive notifications for a process in a compute system
HRESULT
WINAPI
HcsSetProcessCallback(
    _In_ HCS_PROCESS process,
    _In_ HCS_EVENT_OPTIONS callbackOptions,
    _In_ void* context,
    _In_ HCS_EVENT_CALLBACK callback
    );

// Waits for a process in a compute system to exit.
HRESULT
WINAPI
HcsWaitForProcessExit(
    _In_ HCS_PROCESS computeSystem,
    _In_ DWORD timeoutMs,
    _Outptr_opt_ PWSTR* result
    );

/// Returns properties of the Host Compute Service
HRESULT
WINAPI
HcsGetServiceProperties(
    _In_opt_ PCWSTR propertyQuery,
    _Outptr_ PWSTR* result
    );

/// Modifies settings of the Host Compute Service
HRESULT
WINAPI
HcsModifyServiceSettings(
    _In_ PCWSTR settings,
    _Outptr_opt_ PWSTR* result
    );

/// Submits a WER report
HRESULT
WINAPI
HcsSubmitWerReport(
    _In_ PCWSTR settings
    );

// Creates an empty guest-state file (.vmgs) for a VM.
HRESULT
WINAPI
HcsCreateEmptyGuestStateFile(
    _In_ PCWSTR guestStateFilePath
    );

// Creates an empty runtime-state file (.vmrs) for a VM.
HRESULT
WINAPI
HcsCreateEmptyRuntimeStateFile(
    _In_ PCWSTR runtimeStateFilePath
    );

// Adds an entry to a file's ACL that grants access for a VM.
HRESULT
WINAPI
HcsGrantVmAccess(
    _In_ PCWSTR vmId,
    _In_ PCWSTR filePath
    );

// Removes an entry to a file's ACL that granted access for a VM.
HRESULT
WINAPI
HcsRevokeVmAccess(
    _In_ PCWSTR vmId,
    _In_ PCWSTR filePath
    );

// Grants VM group access (R/O) to the specified file.
HRESULT
WINAPI
HcsGrantVmGroupAccess(
    _In_ PCWSTR filePath
    );

// Removes VM group access for the specified file.
HRESULT
WINAPI
HcsRevokeVmGroupAccess(
    _In_ PCWSTR filePath
    );

#ifdef __cplusplus
}
#endif

#endif // WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)
#pragma endregion

#endif // _HYPERV_COMPUTECORE_H_

#ifndef ext_ms_win_hyperv_compute_l1_2_5_query_routines
#define ext_ms_win_hyperv_compute_l1_2_5_query_routines

//
//Private Extension API Query Routines
//

#ifdef __cplusplus
extern "C" {
#endif

BOOLEAN
__stdcall
IsHcsEnumerateComputeSystemsPresent(
    VOID
    );

BOOLEAN
__stdcall
IsHcsEnumerateComputeSystemsInNamespacePresent(
    VOID
    );

BOOLEAN
__stdcall
IsHcsCreateOperationPresent(
    VOID
    );

BOOLEAN
__stdcall
IsHcsCreateOperationWithNotificationsPresent(
    VOID
    );

BOOLEAN
__stdcall
IsHcsCloseOperationPresent(
    VOID
    );

BOOLEAN
__stdcall
IsHcsGetOperationContextPresent(
    VOID
    );

BOOLEAN
__stdcall
IsHcsSetOperationContextPresent(
    VOID
    );

BOOLEAN
__stdcall
IsHcsGetComputeSystemFromOperationPresent(
    VOID
    );

BOOLEAN
__stdcall
IsHcsGetProcessFromOperationPresent(
    VOID
    );

BOOLEAN
__stdcall
IsHcsGetOperationTypePresent(
    VOID
    );

BOOLEAN
__stdcall
IsHcsGetOperationIdPresent(
    VOID
    );

BOOLEAN
__stdcall
IsHcsGetOperationResultPresent(
    VOID
    );

BOOLEAN
__stdcall
IsHcsGetOperationResultAndProcessInfoPresent(
    VOID
    );

BOOLEAN
__stdcall
IsHcsAddResourceToOperationPresent(
    VOID
    );

BOOLEAN
__stdcall
IsHcsGetProcessorCompatibilityFromSavedStatePresent(
    VOID
    );

BOOLEAN
__stdcall
IsHcsWaitForOperationResultPresent(
    VOID
    );

BOOLEAN
__stdcall
IsHcsWaitForOperationResultAndProcessInfoPresent(
    VOID
    );

BOOLEAN
__stdcall
IsHcsSetOperationCallbackPresent(
    VOID
    );

BOOLEAN
__stdcall
IsHcsCancelOperationPresent(
    VOID
    );

BOOLEAN
__stdcall
IsHcsGetOperationPropertiesPresent(
    VOID
    );

BOOLEAN
__stdcall
IsHcsCreateComputeSystemPresent(
    VOID
    );

BOOLEAN
__stdcall
IsHcsCreateComputeSystemInNamespacePresent(
    VOID
    );

BOOLEAN
__stdcall
IsHcsOpenComputeSystemPresent(
    VOID
    );

BOOLEAN
__stdcall
IsHcsOpenComputeSystemInNamespacePresent(
    VOID
    );

BOOLEAN
__stdcall
IsHcsCloseComputeSystemPresent(
    VOID
    );

BOOLEAN
__stdcall
IsHcsStartComputeSystemPresent(
    VOID
    );

BOOLEAN
__stdcall
IsHcsShutDownComputeSystemPresent(
    VOID
    );

BOOLEAN
__stdcall
IsHcsTerminateComputeSystemPresent(
    VOID
    );

BOOLEAN
__stdcall
IsHcsCrashComputeSystemPresent(
    VOID
    );

BOOLEAN
__stdcall
IsHcsPauseComputeSystemPresent(
    VOID
    );

BOOLEAN
__stdcall
IsHcsResumeComputeSystemPresent(
    VOID
    );

BOOLEAN
__stdcall
IsHcsSaveComputeSystemPresent(
    VOID
    );

BOOLEAN
__stdcall
IsHcsGetComputeSystemPropertiesPresent(
    VOID
    );

BOOLEAN
__stdcall
IsHcsModifyComputeSystemPresent(
    VOID
    );

BOOLEAN
__stdcall
IsHcsWaitForComputeSystemExitPresent(
    VOID
    );

BOOLEAN
__stdcall
IsHcsSetComputeSystemCallbackPresent(
    VOID
    );

BOOLEAN
__stdcall
IsHcsInitializeLiveMigrationOnSourcePresent(
    VOID
    );

BOOLEAN
__stdcall
IsHcsStartLiveMigrationOnSourcePresent(
    VOID
    );

BOOLEAN
__stdcall
IsHcsStartLiveMigrationTransferPresent(
    VOID
    );

BOOLEAN
__stdcall
IsHcsFinalizeLiveMigrationPresent(
    VOID
    );

BOOLEAN
__stdcall
IsHcsCreateProcessPresent(
    VOID
    );

BOOLEAN
__stdcall
IsHcsOpenProcessPresent(
    VOID
    );

BOOLEAN
__stdcall
IsHcsCloseProcessPresent(
    VOID
    );

BOOLEAN
__stdcall
IsHcsTerminateProcessPresent(
    VOID
    );

BOOLEAN
__stdcall
IsHcsSignalProcessPresent(
    VOID
    );

BOOLEAN
__stdcall
IsHcsGetProcessInfoPresent(
    VOID
    );

BOOLEAN
__stdcall
IsHcsGetProcessPropertiesPresent(
    VOID
    );

BOOLEAN
__stdcall
IsHcsModifyProcessPresent(
    VOID
    );

BOOLEAN
__stdcall
IsHcsSetProcessCallbackPresent(
    VOID
    );

BOOLEAN
__stdcall
IsHcsWaitForProcessExitPresent(
    VOID
    );

BOOLEAN
__stdcall
IsHcsGetServicePropertiesPresent(
    VOID
    );

BOOLEAN
__stdcall
IsHcsModifyServiceSettingsPresent(
    VOID
    );

BOOLEAN
__stdcall
IsHcsSubmitWerReportPresent(
    VOID
    );

BOOLEAN
__stdcall
IsHcsCreateEmptyGuestStateFilePresent(
    VOID
    );

BOOLEAN
__stdcall
IsHcsCreateEmptyRuntimeStateFilePresent(
    VOID
    );

BOOLEAN
__stdcall
IsHcsGrantVmAccessPresent(
    VOID
    );

BOOLEAN
__stdcall
IsHcsRevokeVmAccessPresent(
    VOID
    );

BOOLEAN
__stdcall
IsHcsGrantVmGroupAccessPresent(
    VOID
    );

BOOLEAN
__stdcall
IsHcsRevokeVmGroupAccessPresent(
    VOID
    );

#ifdef __cplusplus
}
#endif

#endif // endof guard

