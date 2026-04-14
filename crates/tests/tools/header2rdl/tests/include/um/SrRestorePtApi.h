/******************************************************************************

Copyright (c) 2000 Microsoft Corporation

Module Name:
    SRRestorePtAPI.h

Abstract:
    This file contains the declarations for the SRRESTOREPT_API

******************************************************************************/

#if !defined( _SRRESTOREPTAPI_H )
#define _SRRESTOREPTAPI_H

#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

//
// Type of Event
//

#define MIN_EVENT                        100
#define BEGIN_SYSTEM_CHANGE              100
#define END_SYSTEM_CHANGE                101
#define BEGIN_NESTED_SYSTEM_CHANGE       102    // for Whistler only - use this to prevent nested restore pts
#define END_NESTED_SYSTEM_CHANGE         103    // for Whistler only - use this to prevent nested restore pts
#define BEGIN_NESTED_SYSTEM_CHANGE_NORP  104
#define END_NESTED_SYSTEM_CHANGE_NORP    END_NESTED_SYSTEM_CHANGE
#define MAX_EVENT                        104

//
// Type of Restore Points
//

#define MIN_RPT                 0
#define APPLICATION_INSTALL     0
#define APPLICATION_UNINSTALL   1
#define DESKTOP_SETTING         2    /* Not implemented */
#define ACCESSIBILITY_SETTING   3    /* Not implemented */
#define OE_SETTING              4    /* Not implemented */
#define APPLICATION_RUN         5    /* Not implemented */
#define RESTORE                 6
#define CHECKPOINT              7
#define WINDOWS_SHUTDOWN        8    /* Not implemented */
#define WINDOWS_BOOT            9    /* Not implemented */
#define DEVICE_DRIVER_INSTALL   10
#define FIRSTRUN                11
#define MODIFY_SETTINGS         12
#define CANCELLED_OPERATION     13   /* Only valid for END_SYSTEM_CHANGE */
#define BACKUP_RECOVERY         14
#define BACKUP                  15
#define MANUAL_CHECKPOINT       16
#define WINDOWS_UPDATE          17
#define CRITICAL_UPDATE         18
#define MAX_RPT                 18


#define MAX_DESC                64
#define MAX_DESC_W              256   // longer for Whistler

//
// for Millennium compatibility
//

#pragma pack(push, srrestoreptapi_include)
#pragma pack(1)

//
// Restore point information
//

typedef struct _RESTOREPTINFOA {
    DWORD   dwEventType;                // Type of Event - Begin or End
    DWORD   dwRestorePtType;            // Type of Restore Point - App install/uninstall
    INT64   llSequenceNumber;           // Sequence Number - 0 for begin
    CHAR    szDescription[MAX_DESC];    // Description - Name of Application / Operation
} RESTOREPOINTINFOA, *PRESTOREPOINTINFOA;

typedef struct _RESTOREPTINFOW {
    DWORD   dwEventType;
    DWORD   dwRestorePtType;
    INT64   llSequenceNumber;
    WCHAR   szDescription[MAX_DESC_W];
} RESTOREPOINTINFOW, *PRESTOREPOINTINFOW;

typedef struct _RESTOREPTINFOEX {
    FILETIME ftCreation;
    DWORD    dwEventType;
    DWORD    dwRestorePtType;
    DWORD    dwRPNum;
    WCHAR    szDescription[MAX_DESC_W];
} RESTOREPOINTINFOEX, *PRESTOREPOINTINFOEX;


//
// Status returned by System Restore
//

typedef struct _SMGRSTATUS {
    DWORD   nStatus;            // Status returned by State Manager Process
    INT64   llSequenceNumber;   // Sequence Number for the restore point
} STATEMGRSTATUS, *PSTATEMGRSTATUS;

#pragma pack(pop, srrestoreptapi_include)


#ifdef __cplusplus
extern "C" {
#endif

//
// RPC call to set a restore point
//
// Return value  TRUE if the call was a success
//               FALSE if the call failed
//
// If pSmgrStatus nStatus field is set as follows
//
// ERROR_SUCCESS              If the call succeeded (return value will be TRUE)
//
// ERROR_TIMEOUT              If the call timed out due to a wait on a mutex for
//                            for setting restore points.
//
// ERROR_INVALID_DATA         If the cancel restore point is called with an invalid
//                            sequence number
//
// ERROR_INTERNAL_ERROR       If there are internal failures.
//
// ERROR_BAD_ENVIRONMENT      If the API is called in SafeMode
//
// ERROR_SERVICE_DISABLED     If SystemRestore is Disabled.
//
// ERROR_DISK_FULL            If System Restore is frozen (Windows Whistler only)
//
// ERROR_ALREADY_EXISTS       If this is a nested restore point

BOOL __stdcall
SRSetRestorePointA(
                  _In_ PRESTOREPOINTINFOA  pRestorePtSpec,// [in] Restore Point specification
                  _Out_ PSTATEMGRSTATUS     pSMgrStatus   // [out] Status returned
                  );


BOOL __stdcall
SRSetRestorePointW(
                  _In_ PRESTOREPOINTINFOW pRestorePtSpec,
                  _Out_ PSTATEMGRSTATUS   pSMgrStatus
                  );

BOOL __stdcall
SRSetRestorePointInternal(
                  _In_ PRESTOREPOINTINFOW pRestorePtSpec,
                  _Out_ PSTATEMGRSTATUS   pSMgrStatus,
                  _In_ BOOL           fForceSurrogate
                  );

DWORD __stdcall
SRRemoveRestorePoint(DWORD dwRPNum);

#ifdef __cplusplus
}
#endif


#ifdef UNICODE
#define RESTOREPOINTINFO        RESTOREPOINTINFOW
#define PRESTOREPOINTINFO       PRESTOREPOINTINFOW
#define SRSetRestorePoint       SRSetRestorePointW
#else
#define RESTOREPOINTINFO        RESTOREPOINTINFOA
#define PRESTOREPOINTINFO       PRESTOREPOINTINFOA
#define SRSetRestorePoint       SRSetRestorePointA
#endif


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


#endif // !defined( _RESTOREPTAPI_H )
