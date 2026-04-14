/*=============================================================================

    Copyright (c) 2003  Microsoft Corporation

    Module Name:

        clfsmgmtw32.h

    Abstract:

        Declares the exported API set for the Common Log Win32
        Management API.

    Author:

        JR Tipton [jrtipton] 24-Aug-2003

    Environment:

        User Mode

    Revision History:


=============================================================================*/
#ifndef __CLFSMGMTW32_H__
#define __CLFSMGMTW32_H__
#include <winapifamily.h>

#pragma region Desktop Family or BootableSku Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_BOOTABLESKU)



#ifdef CLFS_KERNEL_MODE
#    undef CLFS_KERNEL_MODE
#endif /* CLFS_KERNEL_MODE */

#include <clfsmgmt.h>

#ifdef __cplusplus
extern "C"
{
#endif /* __cplusplus */

#if (_WIN32_WINNT >= _WIN32_WINNT_WS03)
typedef
VOID
(*PLOG_TAIL_ADVANCE_CALLBACK) (
    IN HANDLE           hLogFile,
    IN CLFS_LSN         lsnTarget,
    IN PVOID            pvClientContext
    );
#endif /* _WIN32_WINNT */

#if (_WIN32_WINNT >= _WIN32_WINNT_WS03)
typedef
VOID
(*PLOG_FULL_HANDLER_CALLBACK) (
    IN HANDLE           hLogFile,
    IN DWORD            dwError,
    IN BOOL             fLogIsPinned,
    IN PVOID            pvClientContext
    );
#endif /* _WIN32_WINNT */

#if (_WIN32_WINNT >= _WIN32_WINNT_WS03)
typedef
VOID
(*PLOG_UNPINNED_CALLBACK) (
    IN HANDLE           hLogFile,
    IN PVOID            pvClientContext
    );
#endif /* _WIN32_WINNT */

#if (_WIN32_WINNT >= _WIN32_WINNT_WS03)
typedef struct _LOG_MANAGEMENT_CALLBACKS
{
    PVOID   CallbackContext;

    PLOG_TAIL_ADVANCE_CALLBACK AdvanceTailCallback;
    PLOG_FULL_HANDLER_CALLBACK LogFullHandlerCallback;
    PLOG_UNPINNED_CALLBACK     LogUnpinnedCallback;

} LOG_MANAGEMENT_CALLBACKS, *PLOG_MANAGEMENT_CALLBACKS;
#endif /* _WIN32_WINNT */

#if (_WIN32_WINNT >= _WIN32_WINNT_WS03)
//------------------------------------------------------------------------------
// RegisterManageableLogClient
// 
// Registers a client as one that wishes to participate in the CLFS management
// scheme.  The client can receive notifications such as "advance base lsn" if
// pCallbacks is non-null.
//------------------------------------------------------------------------------
CLFSUSER_API BOOL WINAPI RegisterManageableLogClient (
    IN HANDLE                         hLog,
    IN PLOG_MANAGEMENT_CALLBACKS      pCallbacks
    );
#endif /* _WIN32_WINNT */

#if (_WIN32_WINNT >= _WIN32_WINNT_WS03)
//------------------------------------------------------------------------------
// DeregisterManageableLogClient
// 
// When a log client no longer wishes to be registered, either the log handle
// can be closed (deregistering the client implicitly) or this routine can
// be invoked which will deregister the client explicitly.  The client will
// no longer receive notifications related to management.
//------------------------------------------------------------------------------
CLFSUSER_API BOOL WINAPI DeregisterManageableLogClient (
    IN HANDLE hLog
    );
#endif /* _WIN32_WINNT */

#if (_WIN32_WINNT >= _WIN32_WINNT_WS03)
//------------------------------------------------------------------------------
// ReadLogNotification
// 
// Allows a registered log management client to read a management notification.
// Only possible if the client is not receiving callbacks.  Can function in
// synchronous or asynchronous mode.
//------------------------------------------------------------------------------
CLFSUSER_API BOOL WINAPI ReadLogNotification (
    IN  HANDLE                   hLog,
    OUT PCLFS_MGMT_NOTIFICATION  pNotification,
    IN  LPOVERLAPPED             lpOverlapped
    );
#endif /* _WIN32_WINNT */

#if (_WIN32_WINNT >= _WIN32_WINNT_WS03)
//------------------------------------------------------------------------------
// InstallLogPolicy
// 
// Installs a policy on a log.  The possible policies are described by the 
// enumerated type CLFS_MGMT_POLICY_TYPE.
//------------------------------------------------------------------------------
CLFSUSER_API BOOL WINAPI InstallLogPolicy (
    IN HANDLE            hLog,
    IN PCLFS_MGMT_POLICY pPolicy
    );
#endif /* _WIN32_WINNT */

#if (_WIN32_WINNT >= _WIN32_WINNT_WS03)
//------------------------------------------------------------------------------
// InstallLogPolicy
// 
// Removes a policy from a log, reverting back to the default behavior for the
// policy indicated.  The possible policies are described by the enumerated type 
// CLFS_MGMT_POLICY_TYPE.
//------------------------------------------------------------------------------
CLFSUSER_API BOOL WINAPI RemoveLogPolicy (
    IN HANDLE                hLog,
    IN CLFS_MGMT_POLICY_TYPE ePolicyType
    );
#endif /* _WIN32_WINNT */

#if (_WIN32_WINNT >= _WIN32_WINNT_WS03)
//------------------------------------------------------------------------------
// QueryLogPolicy
// 
// Given a policy type, returns the associated policy data.
//------------------------------------------------------------------------------
CLFSUSER_API BOOL WINAPI QueryLogPolicy (
    IN     HANDLE                hLog,
    IN     CLFS_MGMT_POLICY_TYPE ePolicyType,
    OUT    PCLFS_MGMT_POLICY     pPolicyBuffer,
    IN OUT PULONG                pcbPolicyBuffer
    );
#endif /* _WIN32_WINNT */

#if (_WIN32_WINNT >= _WIN32_WINNT_WS03)
//------------------------------------------------------------------------------
// SetLogFileSizeWithPolicy
// 
// Sets the size of a log file, obeying any policies present on the log file
// at the time.
//------------------------------------------------------------------------------
CLFSUSER_API BOOL WINAPI SetLogFileSizeWithPolicy(
    IN  HANDLE     hLog,
    IN  PULONGLONG pDesiredSize,
    OUT PULONGLONG pResultingSize
    );
#endif /* _WIN32_WINNT */

#if (_WIN32_WINNT >= _WIN32_WINNT_WS03)
//------------------------------------------------------------------------------
// HandleLogFull
// 
// Handles a log full condition according to policy (either by growing the log
// or demanding that other streams advance their log bases).
//
// This routine may do the work asynchronously (returning FALSE with last error
// of ERROR_IO_PENDING).
//------------------------------------------------------------------------------
CLFSUSER_API BOOL WINAPI HandleLogFull(
    IN HANDLE hLog
    );
#endif /* _WIN32_WINNT */

#if (_WIN32_WINNT >= _WIN32_WINNT_WS03)
//------------------------------------------------------------------------------
// LogTailAdvanceFailure
// 
// Allows a log client to indicate that it cannot comply with a request from
// log management to advance its tail.
//------------------------------------------------------------------------------
CLFSUSER_API BOOL WINAPI LogTailAdvanceFailure(
    IN HANDLE hLog,
    IN DWORD  dwReason
    );
#endif /* _WIN32_WINNT */

#if (_WIN32_WINNT >= _WIN32_WINNT_WS03)
//------------------------------------------------------------------------------
// RegisterForLogWriteNotification
// 
// Registers a client as one may not wish to participate in the CLFS management
// scheme but does wish to be notified of certain events in the log.
//------------------------------------------------------------------------------
CLFSUSER_API BOOL WINAPI RegisterForLogWriteNotification (
    IN HANDLE   hLog,
    IN ULONG    cbThreshold,
    IN BOOL     fEnable
    );
#endif /* _WIN32_WINNT */

#ifdef __cplusplus
} // extern "C"
#endif /* __cplusplus */


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_BOOTABLESKU) */
#pragma endregion

#endif /* __CLFSMGMTW32_H__ */

//-----------------------------------------------------------------------------
//                                        END OF FILE
//-----------------------------------------------------------------------------
