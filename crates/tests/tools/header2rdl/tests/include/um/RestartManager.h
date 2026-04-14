////////////////////////////////////////////////////////////////////////
//
//  RestartManager.h --  Header for RestartManager API
//
//  Copyright (c) Microsoft Corp.  All rights reserverd.
//
////////////////////////////////////////////////////////////////////////

#pragma once
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


#ifdef __cplusplus
extern "C" {
#endif // __cplusplus


// Macros and Types

// RM_SESSION_KEY_LEN - size in bytes of binary session key
#define RM_SESSION_KEY_LEN  sizeof(GUID)
// CCH_RM_SESSION_KEY - character count of text-encoded session key
#define CCH_RM_SESSION_KEY  RM_SESSION_KEY_LEN*2
// CCH_RM_MAX_APP_NAME - maximum character count of application friendly name
#define CCH_RM_MAX_APP_NAME 255
// CCH_RM_MAX_SVC_NAME - maximum character count of service short name
#define CCH_RM_MAX_SVC_NAME 63
// Uninitialized value for TS Session ID
#define RM_INVALID_TS_SESSION -1
// Uninitialized value for Process ID
#define RM_INVALID_PROCESS -1

typedef enum _RM_APP_TYPE {
    RmUnknownApp = 0,   // Application type cannot be classified in
                        // known categories
    RmMainWindow = 1,   // Application is a windows application that
                        // displays a top-level window
    RmOtherWindow = 2,  // Application is a windows app but does not
                        // display a top-level window
    RmService = 3,      // Application is an NT service
    RmExplorer = 4,     // Application is Explorer
    RmConsole = 5,      // Application is Console application
    RmCritical = 1000   // Application is critical system process
                        // where a reboot is required to restart
} RM_APP_TYPE;

typedef enum _RM_SHUTDOWN_TYPE {
    RmForceShutdown = 0x1,          // Force app shutdown
    RmShutdownOnlyRegistered = 0x10 // Only shudown apps if all apps
                                    // registered for restart
} RM_SHUTDOWN_TYPE;

typedef enum _RM_APP_STATUS {
    RmStatusUnknown = 0x0,          // Application in unknown state 
                                    // or state not important
    RmStatusRunning = 0x1,          // Application is currently running
    RmStatusStopped = 0x2,          // Application stopped by Restart Manager
    RmStatusStoppedOther = 0x4,     // Application detected stopped 
                                    // by outside action
    RmStatusRestarted = 0x8,        // Application restarted by Restart Manager
    RmStatusErrorOnStop = 0x10,     // An error occurred when stopping
                                    // this application
    RmStatusErrorOnRestart = 0x20,  // An error occurred when restarting
                                    // this application
    RmStatusShutdownMasked = 0x40,  // Shutdown action masked by filer
    RmStatusRestartMasked = 0x80    // Restart action masked by filter
} RM_APP_STATUS;

typedef enum _RM_REBOOT_REASON {
    RmRebootReasonNone = 0x0,               // Reboot not required
    RmRebootReasonPermissionDenied = 0x1,   // Current user does not have
                                            // permission to shut down
                                            // one or more detected processes
    RmRebootReasonSessionMismatch = 0x2,    // One or more processes are
                                            // running in another TS session.
    RmRebootReasonCriticalProcess = 0x4,    // A critical process has been
                                            // detected
    RmRebootReasonCriticalService = 0x8,    // A critical service has been
                                            // detected
    RmRebootReasonDetectedSelf = 0x10       // The current process has been
                                            // detected
} RM_REBOOT_REASON;

typedef struct _RM_UNIQUE_PROCESS {
    DWORD dwProcessId;              // PID
    FILETIME ProcessStartTime;      // Process creation time
} RM_UNIQUE_PROCESS, *PRM_UNIQUE_PROCESS;

typedef struct _RM_PROCESS_INFO{
    RM_UNIQUE_PROCESS Process;      // Unique process identification
    WCHAR strAppName[CCH_RM_MAX_APP_NAME+1];    // Application friendly name
    WCHAR strServiceShortName[CCH_RM_MAX_SVC_NAME+1];   // Service short name,
                                                        // if applicable
    RM_APP_TYPE ApplicationType;    // Application type
    ULONG AppStatus;                // Bit mask of application status
    DWORD TSSessionId;              // Terminal Service session ID of 
                                    // process (-1 if n/a)
    BOOL bRestartable;              // Is application restartable?
} RM_PROCESS_INFO, *PRM_PROCESS_INFO;

typedef enum _RM_FILTER_TRIGGER
{
    RmFilterTriggerInvalid = 0,         // Invalid value
    RmFilterTriggerFile,                // Filter is triggered by EXE name
    RmFilterTriggerProcess,             // Filter is triggered by PID/create time
    RmFilterTriggerService              // Filter is triggered by service short name
} RM_FILTER_TRIGGER;

typedef enum _RM_FILTER_ACTION {
    RmInvalidFilterAction = 0,          // Invalid value
    RmNoRestart = 1,                    // Prevent restart of application
    RmNoShutdown = 2                    // Prevent shutdown and restart of
                                        // application
} RM_FILTER_ACTION;

typedef struct _RM_FILTER_INFO
{
    RM_FILTER_ACTION FilterAction;      // Filter action
    RM_FILTER_TRIGGER FilterTrigger;    // Filter trigger
    DWORD cbNextOffset;                 // Bytes to next structure
    union
    {
        LPWSTR strFilename;             // When RmFilterTrigger == RmFilterTriggerFile,
                                        // this contains the filename this filter applies to
        RM_UNIQUE_PROCESS Process;      // When RmFilterTrigger == RmFilterTriggerProcess,
                                        // this contains the process this filter applies to
        LPWSTR strServiceShortName;     // When RmFilterTrigger == RmFilterTriggerService,
                                        // this contians the service name this filter applies to
    };
} RM_FILTER_INFO, *PRM_FILTER_INFO;

typedef void (*RM_WRITE_STATUS_CALLBACK)( _In_ UINT nPercentComplete );

// Functions

// Function: RmStartSession
// Purpose: Begins a Restart Manager session as the Conductor
// Error Codes:
//        ERROR_SEM_TIMEOUT
//            Failed to obtain semaphore lock in allotted time.
//        ERROR_BAD_ARGUMENTS
//            An invalid argument was supplied to the function.
//        ERROR_WRITE_FAULT
//            Read/write operation failed.
//        ERROR_OUTOFMEMORY
//            Could not allocate memory or instantiate class object.
DWORD WINAPI
RmStartSession(
    _Out_ DWORD *pSessionHandle,
    _Reserved_ DWORD dwSessionFlags,
    _Out_writes_(CCH_RM_SESSION_KEY+1) WCHAR strSessionKey[]);

// Function: RmJoinSession
// Purpose: Joins an existing Restart Manager session as a Subordinate
// Error Codes:
//        ERROR_SEM_TIMEOUT
//            Failed to obtain semaphore lock in allotted time.
//        ERROR_BAD_ARGUMENTS
//            An invalid argument was supplied to the function.
//        ERROR_WRITE_FAULT
//            Read/write operation failed.
//        ERROR_OUTOFMEMORY
//            Could not allocate memory or instantiate class object.
DWORD WINAPI
RmJoinSession(
    _Out_ DWORD *pSessionHandle,
    _In_reads_(CCH_RM_SESSION_KEY+1) const WCHAR strSessionKey[]);

// Function: RmEndSession
// Purpose: Terminates and cleans up an existing session.
// Error Codes:
//        ERROR_SEM_TIMEOUT
//            Failed to obtain semaphore lock in allotted time.
//        ERROR_BAD_ARGUMENTS
//            An invalid argument was supplied to the function.
//        ERROR_WRITE_FAULT
//            Read/write operation failed.
//        ERROR_OUTOFMEMORY
//            Could not allocate memory or instantiate class object.
//        ERROR_INVALID_HANDLE
//            No session exists for the supplied handle.
DWORD WINAPI
RmEndSession( _In_ DWORD dwSessionHandle );

// Function: RmRegisterResources
// Purpose: Called if error is encountered. Installer
//          must pass in all files to be updated 
//          that may prevent the install from completing. Processes
//          passed in array are forced to stop/start. Services short
//          names passed in array are stopped (along with dependencies
//          and restarted.
// Error Codes:
//        ERROR_SEM_TIMEOUT
//            Failed to obtain semaphore lock in allotted time.
//        ERROR_BAD_ARGUMENTS
//            An invalid argument was supplied to the function.
//        ERROR_WRITE_FAULT
//            Read/write operation failed.
//        ERROR_OUTOFMEMORY
//            Could not allocate memory or instantiate class object.
//        ERROR_INVALID_HANDLE
//            No session exists for the supplied handle.
DWORD WINAPI
RmRegisterResources(
    _In_ DWORD dwSessionHandle,
    _In_ UINT nFiles,
    _In_reads_opt_(nFiles) LPCWSTR rgsFileNames[],
    _In_ UINT nApplications,
    _In_reads_opt_(nApplications) RM_UNIQUE_PROCESS rgApplications[],
    _In_ UINT nServices,
    _In_reads_opt_(nServices) LPCWSTR rgsServiceNames[]);

// Function: RmGetList
// Purpose: Gets the list of applications affected by registered
//          resources and their current status.
// Returns: An array of RM_PROCESS_INFO structs containing a list of
//          all applications affected by registered items, the number
//          of structs needed to retreive all application information,
//          the number of structs that were populated, and a series
//          of flags describing why a reboot will be necessary.
// Error Codes:
//        ERROR_MORE_DATA
//            Buffer is not large enough for all information.
//        ERROR_CANCELLED
//            Operation was cancelled by user.
//        ERROR_SEM_TIMEOUT
//            Failed to obtain semaphore lock in allotted time.
//        ERROR_BAD_ARGUMENTS
//            An invalid argument was supplied to the function.
//        ERROR_WRITE_FAULT
//            Read/write operation failed.
//        ERROR_OUTOFMEMORY
//            Could not allocate memory or instantiate class object.
//        ERROR_INVALID_HANDLE
//            No session exists for the supplied handle.
DWORD WINAPI
RmGetList(
    _In_ DWORD dwSessionHandle,
    _Out_ UINT *pnProcInfoNeeded,
    _Inout_ UINT *pnProcInfo,
    _Inout_updates_opt_(*pnProcInfo) RM_PROCESS_INFO rgAffectedApps[],
    _Out_ LPDWORD lpdwRebootReasons );

// Function: RmShutdown
// Purpose: Initiates application shutdown sequence.
// Error Codes:
//        ERROR_FAIL_NOACTION_REBOOT
//            One or more of the affected applications requires a reboot
//            to be shut down.
//        ERROR_FAIL_SHUTDOWN
//            There was an error shuting down some application(s).
//        ERROR_CANCELLED
//            Operation was cancelled by user.
//        ERROR_SEM_TIMEOUT
//            Failed to obtain semaphore lock in allotted time.
//        ERROR_BAD_ARGUMENTS
//            An invalid argument was supplied to the function.
//        ERROR_WRITE_FAULT
//            Read/write operation failed.
//        ERROR_OUTOFMEMORY
//            Could not allocate memory or instantiate class object.
//        ERROR_INVALID_HANDLE
//            No session exists for the supplied handle.
DWORD WINAPI
RmShutdown(
    _In_ DWORD dwSessionHandle,
    _In_ ULONG lActionFlags,
    _In_opt_ RM_WRITE_STATUS_CALLBACK fnStatus);

// Function: RmRestart
// Purpose: Applications automatically shutdown by RmShutdown() 
//          and are registered for restart or are services are restarted.
// Error Codes:
//        ERROR_FAIL_NOACTION_REBOOT
//            One or more of the affected applications requires a reboot
//            to be shut down.
//        ERROR_FAIL_SHUTDOWN
//            There was an error shuting down some application(s).
//        ERROR_CANCELLED
//            Operation was cancelled by user.
//        ERROR_SEM_TIMEOUT
//            Failed to obtain semaphore lock in allotted time.
//        ERROR_BAD_ARGUMENTS
//            An invalid argument was supplied to the function.
//        ERROR_WRITE_FAULT
//            Read/write operation failed.
//        ERROR_OUTOFMEMORY
//            Could not allocate memory or instantiate class object.
//        ERROR_INVALID_HANDLE
//            No session exists for the supplied handle.
DWORD WINAPI
RmRestart(
        _In_ DWORD dwSessionHandle,
        _Reserved_ DWORD dwRestartFlags,
        _In_opt_ RM_WRITE_STATUS_CALLBACK fnStatus);

// Function: RmCancelCurrentTask()
// Purpose: Notifies Restart Manager to cancel the task it is performing
//          as soon as possible.
// Error Codes:
//        ERROR_BAD_ARGUMENTS
//            An invalid argument was supplied to the function.
//        ERROR_OUTOFMEMORY
//            Could not allocate memory or instantiate class object.
//        ERROR_INVALID_HANDLE
//            No session exists for the supplied handle.
DWORD WINAPI
RmCancelCurrentTask(
        _In_ DWORD dwSessionHandle);

// Function: RmAddFilter()
// Purpose: Adds a filter for process module (ie. executable name), process ID,
//          or service short name to prevent Restart Manager from performing
//          the specified action on the process(es) or service noted. 
// Error Codes:
//        ERROR_BAD_ARGUMENTS
//            An invalid argument was supplied to the function.
//        ERROR_INVALID_HANDLE
//            No session exists for the supplied handle.
DWORD WINAPI
RmAddFilter(
        _In_ DWORD dwSessionHandle,
        _In_opt_ LPCWSTR strModuleName,
        _In_opt_ RM_UNIQUE_PROCESS *pProcess,
        _In_opt_ LPCWSTR strServiceShortName,
        _In_ RM_FILTER_ACTION FilterAction );

// Function: RmRemoveFilter()
// Purpose: Removes a previously added filter for process module
//          (ie. executable name), process ID, or service short name
//          enabling all Restart Manager actions on the process(es)
//          or service noted. 
// Error Codes:
//        ERROR_BAD_ARGUMENTS
//            An invalid argument was supplied to the function.
//        ERROR_INVALID_HANDLE
//            No session exists for the supplied handle.
DWORD WINAPI
RmRemoveFilter(
        _In_ DWORD dwSessionHandle,
        _In_opt_ LPCWSTR strModuleName,
        _In_opt_ RM_UNIQUE_PROCESS *pProcess,
        _In_opt_ LPCWSTR strServiceShortName );

// Function: RmGetFilterList()
// Purpose: Writes a set of currenly applied filters
//          to a caller-allocated buffer.
// Error Codes:
//        ERROR_BAD_ARGUMENTS
//            An invalid argument was supplied to the function.
//        ERROR_INVALID_HANDLE
//            No session exists for the supplied handle.
DWORD WINAPI
RmGetFilterList(
        _In_ DWORD dwSessionHandle,
        _Out_writes_bytes_opt_(cbFilterBuf) PBYTE pbFilterBuf,
        _In_ DWORD cbFilterBuf,
        _Out_ LPDWORD cbFilterBufNeeded );

#ifdef __cplusplus
}
#endif // __cplusplus

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

