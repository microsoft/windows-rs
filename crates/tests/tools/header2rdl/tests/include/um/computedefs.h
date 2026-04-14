/*++

Copyright (c) Microsoft Corporation. All rights reserved.

Module Name:

    ComputeDefs.h

Abstract:

    Contains the public types and definitions used by the Host Compute APIs.

Environment:

    User mode.

--*/
#ifndef _HYPERV_COMPUTEDEFS_H_
#define _HYPERV_COMPUTEDEFS_H_

#if defined(_MSC_VER) && (_MSC_VER >= 1200)
#pragma once
#endif

// Handle to a compute system
DECLARE_HANDLE(HCS_SYSTEM);

// Handle to a process running in a compute system
DECLARE_HANDLE(HCS_PROCESS);

// Handle to an operation on a compute system
DECLARE_HANDLE(HCS_OPERATION);

// Handle to a callback registered on a compute system or process handle.
DECLARE_HANDLE(HCS_CALLBACK);

// Type of an operation. These correspond to the functions that invoke the operation.
typedef enum HCS_OPERATION_TYPE
{
    HcsOperationTypeNone = -1,
    HcsOperationTypeEnumerate = 0,
    HcsOperationTypeCreate = 1,
    HcsOperationTypeStart = 2,
    HcsOperationTypeShutdown = 3,
    HcsOperationTypePause = 4,
    HcsOperationTypeResume = 5,
    HcsOperationTypeSave = 6,
    HcsOperationTypeTerminate = 7,
    HcsOperationTypeModify = 8,
    HcsOperationTypeGetProperties = 9,
    HcsOperationTypeCreateProcess = 10,
    HcsOperationTypeSignalProcess = 11,
    HcsOperationTypeGetProcessInfo = 12,
    HcsOperationTypeGetProcessProperties = 13,
    HcsOperationTypeModifyProcess = 14,
    HcsOperationTypeCrash = 15,
    HcsOperationTypeLiveMigration = 19,
    HcsOperationTypeReserved1 = 16,
    HcsOperationTypeReserved2 = 17,
    HcsOperationTypeReserved3 = 18,
} HCS_OPERATION_TYPE;

#define HCS_INVALID_OPERATION_ID (UINT64)(-1)

// Function type for the completion callback of an operation.
typedef void (CALLBACK *HCS_OPERATION_COMPLETION)(
    _In_ HCS_OPERATION operation,
    _In_opt_ void* context
    );

// Events indicated to callbacks registered by HcsRegisterComputeSystemCallback or
// HcsRegisterProcessCallback (since Windows 1809).
typedef enum HCS_EVENT_TYPE
{
    HcsEventInvalid = 0x00000000,

    /// Events for HCS_SYSTEM handles
    HcsEventSystemExited = 0x00000001,
    HcsEventSystemCrashInitiated = 0x00000002,
    HcsEventSystemCrashReport = 0x00000003,
    HcsEventSystemRdpEnhancedModeStateChanged = 0x00000004,
    HcsEventSystemSiloJobCreated = 0x00000005,
    HcsEventSystemGuestConnectionClosed = 0x00000006,

    /// Events for HCS_PROCESS handles
    HcsEventProcessExited = 0x00010000,

    /// Common Events
    HcsEventOperationCallback = 0x01000000,
    HcsEventServiceDisconnect = 0x02000000,

    // Event groups, enabled by HCS_EVENT_OPTIONS set by clients
    HcsEventGroupVmLifecycle = 0x80000002,

    // A group of live migration events
    HcsEventGroupLiveMigration = 0x80000003,

    // Events for HCS_OPERATION
    HcsEventGroupOperationInfo = 0xC0000001,

} HCS_EVENT_TYPE;

// Provides information about an event that occurred on a compute system or process.
typedef struct HCS_EVENT
{
    // Type of Event (see HCS_EVENT_TYPE)
    HCS_EVENT_TYPE Type;

    // Provides additional data for the event.
    PCWSTR EventData;

    // Handle to a completed operation (if Type is HcsEventOperationCallback).
    HCS_OPERATION Operation;

} HCS_EVENT;

// Options for an event callback registration
typedef enum HCS_EVENT_OPTIONS
{
    HcsEventOptionNone = 0x00000000,
    HcsEventOptionEnableOperationCallbacks = 0x00000001,
    HcsEventOptionEnableVmLifecycle = 0x00000002,
    HcsEventOptionEnableLiveMigrationEvents = 0x00000004,
} HCS_EVENT_OPTIONS;

DEFINE_ENUM_FLAG_OPERATORS(HCS_EVENT_OPTIONS);

// Filter for an operation's events
typedef enum HCS_OPERATION_OPTIONS
{
    HcsOperationOptionNone = 0x00000000,
    HcsOperationOptionProgressUpdate = 0x00000001,
    HcsOperationOptionReserved1 = 0x00000002,
} HCS_OPERATION_OPTIONS;

DEFINE_ENUM_FLAG_OPERATORS(HCS_OPERATION_OPTIONS);

// Function type for compute system event callbacks
typedef void (CALLBACK *HCS_EVENT_CALLBACK)(
    _In_ HCS_EVENT* event,
    _In_opt_ void* context
    );

// Resource type for HCS_OPERATION resource support.
typedef enum HCS_RESOURCE_TYPE
{
    HcsResourceTypeNone = 0,
    HcsResourceTypeFile = 1,
    HcsResourceTypeJob = 2,
    HcsResourceTypeComObject = 3,
    HcsResourceTypeSocket = 4
} HCS_RESOURCE_TYPE;

// Flags applicable to HCS_NOTIFICATIONS
typedef enum HCS_NOTIFICATION_FLAGS
{
    HcsNotificationFlagSuccess = 0x00000000,
    HcsNotificationFlagFailure = 0x80000000
} HCS_NOTIFICATION_FLAGS;

// Notifications indicated to callbacks registered by HcsRegisterComputeSystemCallback or
// HcsRegisterProcessCallback (until Windows 1803).
typedef enum HCS_NOTIFICATIONS
{
    HcsNotificationInvalid = 0x00000000,

    // Notifications for HCS_SYSTEM handles
    HcsNotificationSystemExited = 0x00000001,
    HcsNotificationSystemCreateCompleted = 0x00000002,
    HcsNotificationSystemStartCompleted = 0x00000003,
    HcsNotificationSystemPauseCompleted = 0x00000004,
    HcsNotificationSystemResumeCompleted = 0x00000005,
    HcsNotificationSystemCrashReport = 0x00000006,
    HcsNotificationSystemSiloJobCreated = 0x00000007,
    HcsNotificationSystemSaveCompleted = 0x00000008,
    HcsNotificationSystemRdpEnhancedModeStateChanged = 0x00000009,
    HcsNotificationSystemShutdownFailed = 0x0000000A,
    HcsNotificationSystemShutdownCompleted = 0x0000000A,
    HcsNotificationSystemGetPropertiesCompleted = 0x0000000B,
    HcsNotificationSystemModifyCompleted = 0x0000000C,
    HcsNotificationSystemCrashInitiated =  0x0000000D,
    HcsNotificationSystemGuestConnectionClosed = 0x0000000E,
    HcsNotificationSystemOperationCompletion = 0x0000000F,

    HcsNotificationSystemPassThru = 0x00000010,

    HcsNotificationOperationProgressUpdate = 0x00000100,

    // Notifications for HCS_PROCESS handles
    HcsNotificationProcessExited = 0x00010000,

    // Common notifications
    HcsNotificationServiceDisconnect = 0x01000000,

    // The upper 4 bits are reserved.
    HcsNotificationFlagsReserved = 0xF0000000
} HCS_NOTIFICATIONS;

// Function type for compute system notification callbacks
typedef void (CALLBACK *HCS_NOTIFICATION_CALLBACK)(
    _In_ DWORD notificationType,
    _In_opt_ void*  context,
    _In_ HRESULT notificationStatus,
    _In_opt_ PCWSTR notificationData
    );

// Struct containing information about a process created by HcsStartProcessInComputeSystem
typedef struct
{
    DWORD ProcessId;  // Identifier of the created process
    DWORD Reserved;

    HANDLE StdInput;  // If created, standard input handle of the process
    HANDLE StdOutput; // If created, standard output handle of the process
    HANDLE StdError;  // If created, standard error handle of the process
} HCS_PROCESS_INFORMATION;

// Versions available for HCS_CREATE_OPTIONS used by HcsCreateComputeSystemInNamespace.
typedef enum HCS_CREATE_OPTIONS
{
    HcsCreateOptions_1 = 0x00010000 // HCS_CREATE_OPTIONS_1
}HCS_CREATE_OPTIONS;

// Struct containing different options when creating a compute system with HcsCreateComputeSystemInNamespace.
// Header.Version should be initialized to HcsCreateOptions_1.
typedef struct
{
    HCS_CREATE_OPTIONS      Version;    // HcsCreateOptions_1
    HANDLE                  UserToken;  // Optional user token to run the compute system as
    SECURITY_DESCRIPTOR*    SecurityDescriptor;
    HCS_EVENT_OPTIONS       CallbackOptions;
    void*                   CallbackContext;
    HCS_EVENT_CALLBACK      Callback;
} HCS_CREATE_OPTIONS_1;

#endif
