/*++

Microsoft Windows - File History
Copyright (c) Microsoft Corporation. All Rights Reserved.

Module Name:

    FhSvcCtl.h

Abstract:

    This module contains the definitions of the API provided
    by the File History Service Control Library.

Environment:
    
    User mode.

--*/

#ifndef _FHSVCCTL_
#define _FHSVCCTL_

#if _MSC_VER > 1000
#pragma once
#endif

#include <winapifamily.h>

#ifdef __cplusplus
extern "C" {
#endif

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

#if NTDDI_VERSION >= NTDDI_WIN8 


// Backup stop reason
// This will be used for telemetry purposes
typedef enum {
    BackupInvalidStopReason = 0,
    BackupLimitUserBusyMachineOnAC = 1,   // Backup quota reached for user busy and machine on AC condition (100 MB)
    BackupLimitUserIdleMachineOnDC = 2,   // Backup quota reached for user idle and machine on DC condition (75 MB)
    BackupLimitUserBusyMachineOnDC = 3,   // Backup quota reached for user busy and machine on DC condition (50 MB)
    BackupCancelled = 4                   // Backup cancelled by the user initiated action
} FhBackupStopReason;

DECLARE_HANDLE (FH_SERVICE_PIPE_HANDLE);

__declspec(deprecated("FhServiceOpenPipe is deprecated and might not work on all platforms. For more info, see MSDN."))
HRESULT
WINAPI
FhServiceOpenPipe(
    _In_ BOOL StartServiceIfStopped,
    _Out_ FH_SERVICE_PIPE_HANDLE *Pipe
    );

__declspec(deprecated("FhServiceClosePipe is deprecated and might not work on all platforms. For more info, see MSDN."))
HRESULT
WINAPI
FhServiceClosePipe(
    _In_ FH_SERVICE_PIPE_HANDLE Pipe
    );

__declspec(deprecated("FhServiceStartBackup is deprecated and might not work on all platforms. For more info, see MSDN."))
HRESULT
WINAPI
FhServiceStartBackup(
    _In_ FH_SERVICE_PIPE_HANDLE Pipe,
    _In_ BOOL LowPriorityIo
    );

__declspec(deprecated("FhServiceStopBackup is deprecated and might not work on all platforms. For more info, see MSDN."))
HRESULT
WINAPI
FhServiceStopBackup(
    _In_ FH_SERVICE_PIPE_HANDLE Pipe,
    _In_ BOOL StopTracking
    );

__declspec(deprecated("FhServiceReloadConfiguration is deprecated and might not work on all platforms. For more info, see MSDN."))
HRESULT
WINAPI
FhServiceReloadConfiguration(
    _In_ FH_SERVICE_PIPE_HANDLE Pipe
    );

__declspec(deprecated("FhServiceBlockBackup is deprecated and might not work on all platforms. For more info, see MSDN."))
HRESULT
WINAPI
FhServiceBlockBackup(
    _In_ FH_SERVICE_PIPE_HANDLE Pipe
    );

__declspec(deprecated("FhServiceUnblockBackup is deprecated and might not work on all platforms. For more info, see MSDN."))
HRESULT
WINAPI
FhServiceUnblockBackup(
    _In_ FH_SERVICE_PIPE_HANDLE Pipe
    );


#endif // NTDDI_VERSION >= NTDDI_WIN8 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#ifdef __cplusplus
}
#endif

#endif // _FHSVCCTL_

