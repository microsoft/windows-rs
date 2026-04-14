/*++

Copyright (c) Microsoft Corporation. All rights reserved.

Module Name:

    clfsmgmt.h

Abstract:

    This module exposes the API for managing CLFS logs for kernel
    mode clients.  "Managing" in this sense means manipulating
    log file policies and performing operations (such as log growth)
    which honor the policies.

Author:

    J.R. Tipton (jrtipton) 15-Aug-2003

Revision History:

--*/

#ifndef _CLFSMGMT_H_
#define _CLFSMGMT_H_
#pragma once
#include <winapifamily.h>

#pragma region Desktop Family or BootableSku Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_BOOTABLESKU)


// begin_wdm

#ifdef __cplusplus
extern "C" {
#endif

#if (NTDDI_VERSION >= NTDDI_WS03SP1) || (_WIN32_WINNT >= _WIN32_WINNT_WS03)
//
// We start with the information that is shared
// between user and kernel mode.
//

typedef enum _CLFS_MGMT_POLICY_TYPE {

    ClfsMgmtPolicyMaximumSize = 0x0,
    ClfsMgmtPolicyMinimumSize,
    ClfsMgmtPolicyNewContainerSize,
    ClfsMgmtPolicyGrowthRate,
    ClfsMgmtPolicyLogTail,
    ClfsMgmtPolicyAutoShrink,
    ClfsMgmtPolicyAutoGrow,
    ClfsMgmtPolicyNewContainerPrefix,
    ClfsMgmtPolicyNewContainerSuffix,
    ClfsMgmtPolicyNewContainerExtension,

    ClfsMgmtPolicyInvalid

} CLFS_MGMT_POLICY_TYPE, *PCLFS_MGMT_POLICY_TYPE;
#endif /* NTDDI_VERSION || _WIN32_WINNT */

#if (NTDDI_VERSION >= NTDDI_WS03SP1) || (_WIN32_WINNT >= _WIN32_WINNT_WS03)
#define CLFS_MGMT_NUM_POLICIES ((ULONG)ClfsMgmtPolicyInvalid)
#endif /* NTDDI_VERSION || _WIN32_WINNT */


#if (NTDDI_VERSION >= NTDDI_WS03SP1) || (_WIN32_WINNT >= _WIN32_WINNT_WS03)
//
// Relative sizes used when explicitly setting log size.
//
#define CLFS_LOG_SIZE_MINIMUM ((ULONGLONG)(0))
#define CLFS_LOG_SIZE_MAXIMUM ((ULONGLONG)(-1))
#endif /* NTDDI_VERSION || _WIN32_WINNT */


#if (NTDDI_VERSION >= NTDDI_WS03SP1) || (_WIN32_WINNT >= _WIN32_WINNT_WS03)
//
// The version of a given policy structure.  See CLFS_MGMT_POLICY.
//
#define CLFS_MGMT_POLICY_VERSION (0x01)
#endif /* NTDDI_VERSION || _WIN32_WINNT */


#if (NTDDI_VERSION >= NTDDI_WS03SP1) || (_WIN32_WINNT >= _WIN32_WINNT_WS03)
//
// Log policy flags.
//
// LOG_POLICY_OVERWRITE: If set when adding a log policy, the previous
//                       policy of given type will be replaced.
//
// LOG_POLICY_PERSIST:   If set when adding a log policy, the policy
//                       will be persisted with the log metadata.
//
#define LOG_POLICY_OVERWRITE   (0x01)
#define LOG_POLICY_PERSIST     (0x02)
#endif /* NTDDI_VERSION || _WIN32_WINNT */


#if (NTDDI_VERSION >= NTDDI_WS03SP1) || (_WIN32_WINNT >= _WIN32_WINNT_WS03)
//
// CLFS_MGMT_POLICY
//
// This structure describes one particular policy that
// may be present on a log file.  These are installed
// via InstallLogPolicy (Win32) or ClfsMgmtInstallPolicy (kernel).
//
typedef struct _CLFS_MGMT_POLICY {

    //
    // Version of the structure.  Should be CLFS_MGMT_POLICY_VERSION.
    //
    ULONG                   Version;

    //
    // The entire length of the structure.
    //
    ULONG                   LengthInBytes;

    //
    // Flags which apply to all policies, such as LOG_POLICY_OVERWRITE
    // and LOG_POLICY_PERSIST.
    //
    ULONG                   PolicyFlags;

    //
    // Determines how PolicyParameters union is interpreted.
    //
    CLFS_MGMT_POLICY_TYPE   PolicyType;

    //
    // The way to interpret the PolicyParameters union is
    // determined by the value of PolicyType -- if it is
    // ClfsMgmtPolicyMaximumSize, for instance, then the
    // MaximumSize structure is the relevant one.
    //

    union {

        struct {
            ULONG       Containers;
        } MaximumSize;

        struct {
            ULONG       Containers;
        } MinimumSize;

        struct {
            ULONG       SizeInBytes;
        } NewContainerSize;

        struct {
            ULONG       AbsoluteGrowthInContainers;
            ULONG       RelativeGrowthPercentage;
        } GrowthRate;

        struct {
            ULONG       MinimumAvailablePercentage;
            ULONG       MinimumAvailableContainers;
        } LogTail;

        struct {
            ULONG       Percentage;
        } AutoShrink;

        struct {
            ULONG       Enabled;
        } AutoGrow;

        struct {
            USHORT      PrefixLengthInBytes;
            WCHAR       PrefixString[1]; // dynamic in length depending on PrefixLength
        } NewContainerPrefix;

        struct {
            ULONGLONG   NextContainerSuffix;
        } NewContainerSuffix;

        struct {
            USHORT      ExtensionLengthInBytes;
            WCHAR       ExtensionString[1]; // dynamic in length depending on ExtensionLengthInBytes
        } NewContainerExtension;

    } PolicyParameters;

    //
    // Nothing will be added down here since the structure above
    // can be of dynamic length.
    //

} CLFS_MGMT_POLICY, *PCLFS_MGMT_POLICY;
#endif /* NTDDI_VERSION || _WIN32_WINNT */


#if (NTDDI_VERSION >= NTDDI_WS03SP1) || (_WIN32_WINNT >= _WIN32_WINNT_WS03)
//
// CLFS_MGMT_NOTIFICATION_TYPE
// 
// The types of notifications given to either the callback proxy
// or to readers of notifications.
//

typedef enum _CLFS_MGMT_NOTIFICATION_TYPE
{

    //
    // Notification to advance base LSN.
    //

    ClfsMgmtAdvanceTailNotification = 0,

    //
    // Notification that a request to handle log full condition
    // has completed.
    //

    ClfsMgmtLogFullHandlerNotification,

    //
    // Notification that a previously pinned log is now considered
    // unpinned.
    //

    ClfsMgmtLogUnpinnedNotification,

    //
    // Notification that a non-zero number of bytes has been written
    // to the log.
    //

    ClfsMgmtLogWriteNotification

} CLFS_MGMT_NOTIFICATION_TYPE, *PCLFS_MGMT_NOTIFICATION_TYPE;
#endif /* NTDDI_VERSION || _WIN32_WINNT */


#if (NTDDI_VERSION >= NTDDI_WS03SP1) || (_WIN32_WINNT >= _WIN32_WINNT_WS03)
//
// CLFS_MGMT_NOTIFICATION
//
// A notification and associated parameters.
//

typedef struct _CLFS_MGMT_NOTIFICATION
{

    //
    // Nature of the notification.
    //

    CLFS_MGMT_NOTIFICATION_TYPE     Notification;
    
    //
    // Target LSN for base LSN advancement if the
    // notification type is ClfsMgmtAdvanceTailNotification.
    //

    CLFS_LSN                        Lsn;

    //
    // TRUE if the log is pinned, FALSE otherwise.
    // Especially meaningful when receiving an error
    // status for ClfsMgmtLogFullHandlerNotification.
    //

    USHORT                          LogIsPinned;

} CLFS_MGMT_NOTIFICATION, *PCLFS_MGMT_NOTIFICATION;
#endif /* NTDDI_VERSION || _WIN32_WINNT */


//
// Kernel interface described below.
//

// end_wdm

#ifdef CLFS_KERNEL_MODE

// begin_wdm

#if (NTDDI_VERSION >= NTDDI_WS03SP1) || (_WIN32_WINNT >= _WIN32_WINNT_WS03)
//
// The advance tail callback is required when log clients 
// register for management.  It is invoked whenever the 
// management library decides that this client needs to 
// advance the tail of its log. Only minimal processing is 
// allowed.
//
typedef
NTSTATUS
(*PCLFS_CLIENT_ADVANCE_TAIL_CALLBACK) (
    _In_ PLOG_FILE_OBJECT LogFile,
    _In_ PCLFS_LSN TargetLsn,
    _In_ PVOID ClientData
    );
#endif /* NTDDI_VERSION || _WIN32_WINNT */

#if (NTDDI_VERSION >= NTDDI_WS03SP1) || (_WIN32_WINNT >= _WIN32_WINNT_WS03)
//
// The log file full handler complete callback is invoked upon
// completion of a log growth request (that is, via a call 
// to ClfsMgmtHandleLogFileFull).
//
typedef
VOID
(*PCLFS_CLIENT_LFF_HANDLER_COMPLETE_CALLBACK) (
    _In_ PLOG_FILE_OBJECT LogFile,
    _In_ NTSTATUS OperationStatus,
    _In_ BOOLEAN LogIsPinned,
    _In_ PVOID ClientData
    );
#endif /* NTDDI_VERSION || _WIN32_WINNT */

#if (NTDDI_VERSION >= NTDDI_WS03SP1) || (_WIN32_WINNT >= _WIN32_WINNT_WS03)
//
// The log pinned callback is invoked when log space is freed up
// after a log file full handler completion callback indicates an 
// NT_ERROR status code and LogIsPinned = TRUE.
//

typedef
VOID
(*PCLFS_CLIENT_LOG_UNPINNED_CALLBACK) (
    _In_ PLOG_FILE_OBJECT LogFile,
    _In_ PVOID ClientData
    );
#endif /* NTDDI_VERSION || _WIN32_WINNT */


#if (NTDDI_VERSION >= NTDDI_WS03SP1) || (_WIN32_WINNT >= _WIN32_WINNT_WS03)
//
// The log size complete callback is invoked whenever
// ClfsMgmtSetLogFileSize operation which returned
// STATUS_PENDING is completed.
//

typedef
VOID
(*PCLFS_SET_LOG_SIZE_COMPLETE_CALLBACK) (
    _In_ PLOG_FILE_OBJECT LogFile,
    _In_ NTSTATUS OperationStatus,
    _In_ PVOID ClientData
    );
#endif /* NTDDI_VERSION || _WIN32_WINNT */


#if (NTDDI_VERSION >= NTDDI_WS03SP1) || (_WIN32_WINNT >= _WIN32_WINNT_WS03)
//
// CLFS_MGMT_CLIENT_REGISTRATION
//
// This structure is given to the CLFS management infrastructure
// by clients who wish to be managed (via ClfsMgmtRegisterManagedClient).
// The CLFS_MGMT_CLIENT_REGISTRATION_VERSION value must be stored
// in the 'Version' field of the structure.
//

#define CLFS_MGMT_CLIENT_REGISTRATION_VERSION (0x1)

typedef struct _CLFS_MGMT_CLIENT_REGISTRATION {

    //
    // Initialize Version to CLFS_MGMT_CLIENT_REGISTRATION_VERSION.
    //

    ULONG   Version;  

    PCLFS_CLIENT_ADVANCE_TAIL_CALLBACK AdvanceTailCallback;
    PVOID                              AdvanceTailCallbackData;

    PCLFS_CLIENT_LFF_HANDLER_COMPLETE_CALLBACK LogGrowthCompleteCallback;
    PVOID                                      LogGrowthCompleteCallbackData;

    PCLFS_CLIENT_LOG_UNPINNED_CALLBACK LogUnpinnedCallback;
    PVOID                              LogUnpinnedCallbackData;

} CLFS_MGMT_CLIENT_REGISTRATION, *PCLFS_MGMT_CLIENT_REGISTRATION;
#endif /* NTDDI_VERSION || _WIN32_WINNT */


#if (NTDDI_VERSION >= NTDDI_WS03SP1) || (_WIN32_WINNT >= _WIN32_WINNT_WS03)
//
// CLFS_MGMT_CLIENT
//
// This is the cookie that clients are given when registering and
// must give back to the management infrastructure whenever 
// performing an operation.
//
typedef PVOID CLFS_MGMT_CLIENT, *PCLFS_MGMT_CLIENT;
#endif /* NTDDI_VERSION || _WIN32_WINNT */


#if (NTDDI_VERSION >= NTDDI_WS03SP1) || (_WIN32_WINNT >= _WIN32_WINNT_WS03)
CLFSUSER_API
NTSTATUS
ClfsMgmtRegisterManagedClient(
    _In_ PLOG_FILE_OBJECT LogFile,
    _In_ PCLFS_MGMT_CLIENT_REGISTRATION RegistrationData,
    _Out_ PCLFS_MGMT_CLIENT ClientCookie
    );
#endif /* NTDDI_VERSION || _WIN32_WINNT */

#if (NTDDI_VERSION >= NTDDI_WS03SP1) || (_WIN32_WINNT >= _WIN32_WINNT_WS03)
CLFSUSER_API
NTSTATUS
ClfsMgmtDeregisterManagedClient(
    _In_ CLFS_MGMT_CLIENT ClientCookie
    );
#endif /* NTDDI_VERSION || _WIN32_WINNT */

#if (NTDDI_VERSION >= NTDDI_WS03SP1) || (_WIN32_WINNT >= _WIN32_WINNT_WS03)
CLFSUSER_API
NTSTATUS
ClfsMgmtTailAdvanceFailure(
    _In_ CLFS_MGMT_CLIENT Client,
    _In_ NTSTATUS Reason
    );
#endif /* NTDDI_VERSION || _WIN32_WINNT */

#if (NTDDI_VERSION >= NTDDI_WS03SP1) || (_WIN32_WINNT >= _WIN32_WINNT_WS03)
CLFSUSER_API
NTSTATUS
ClfsMgmtHandleLogFileFull(
    _In_ CLFS_MGMT_CLIENT Client
    );
#endif /* NTDDI_VERSION || _WIN32_WINNT */

#if (NTDDI_VERSION >= NTDDI_WS03SP1) || (_WIN32_WINNT >= _WIN32_WINNT_WS03)
CLFSUSER_API
NTSTATUS
ClfsMgmtInstallPolicy(
    _In_ PLOG_FILE_OBJECT  LogFile,
    _In_reads_bytes_(PolicyLength) PCLFS_MGMT_POLICY Policy,
    _In_ ULONG PolicyLength
    );
#endif /* NTDDI_VERSION || _WIN32_WINNT */

#if (NTDDI_VERSION >= NTDDI_WS03SP1) || (_WIN32_WINNT >= _WIN32_WINNT_WS03)
CLFSUSER_API
NTSTATUS
ClfsMgmtQueryPolicy(
    _In_ PLOG_FILE_OBJECT LogFile,
    _In_ CLFS_MGMT_POLICY_TYPE PolicyType,
    _Out_writes_bytes_(*PolicyLength) PCLFS_MGMT_POLICY Policy,
    _Out_ PULONG PolicyLength
    );
#endif /* NTDDI_VERSION || _WIN32_WINNT */

#if (NTDDI_VERSION >= NTDDI_WS03SP1) || (_WIN32_WINNT >= _WIN32_WINNT_WS03)
CLFSUSER_API
NTSTATUS
ClfsMgmtRemovePolicy(
    _In_ PLOG_FILE_OBJECT LogFile,
    _In_ CLFS_MGMT_POLICY_TYPE PolicyType
    );
#endif /* NTDDI_VERSION || _WIN32_WINNT */

#if (NTDDI_VERSION >= NTDDI_WS03SP1) || (_WIN32_WINNT >= _WIN32_WINNT_WS03)
CLFSUSER_API
NTSTATUS
ClfsMgmtSetLogFileSize(
    _In_ PLOG_FILE_OBJECT LogFile,
    _In_ PULONGLONG NewSizeInContainers,
    _Out_opt_ PULONGLONG ResultingSizeInContainers,
    _In_opt_ PCLFS_SET_LOG_SIZE_COMPLETE_CALLBACK CompletionRoutine,
    _In_opt_ PVOID CompletionRoutineData
    );
#endif /* NTDDI_VERSION || _WIN32_WINNT */


#if (NTDDI_VERSION >= NTDDI_VISTA) || (_WIN32_WINNT >= _WIN32_WINNT_LONGHORN)
CLFSUSER_API
NTSTATUS
ClfsMgmtSetLogFileSizeAsClient(
    _In_ PLOG_FILE_OBJECT LogFile,
    _In_opt_ CLFS_MGMT_CLIENT ClientCookie,
    _In_ PULONGLONG NewSizeInContainers,
    _Out_opt_ PULONGLONG ResultingSizeInContainers,
    _In_opt_ PCLFS_SET_LOG_SIZE_COMPLETE_CALLBACK CompletionRoutine,
    _In_opt_ PVOID CompletionRoutineData
    );
#endif /* NTDDI_VERSION || _WIN32_WINNT */

// end_wdm

#endif // CLFS_KERNEL_MODE

// begin_wdm

#ifdef __cplusplus
} // extern "C"
#endif

// end_wdm


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_BOOTABLESKU) */
#pragma endregion

#endif // _CLFSMGMT_H_
