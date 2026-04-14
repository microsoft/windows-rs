/*++

Copyright (c) Microsoft Corporation.  All rights reserved.

Module Name:

    cscapi.h

Abstract:

    Public Win32 API functions for Offline Files (Client Side Caching - CSC).

    These functions complement the COM-based interfaces and methods declared
    in cscobj.h.  

--*/
#ifndef _INC_CSCAPI_H
#define _INC_CSCAPI_H

#if _MSC_VER > 1000
#pragma once
#endif
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)



#include <specstrings.h>

//
// Enable or disable the Offline Files feature. 
// If *pbRebootRequired returns TRUE, a system restart is necessary to 
// complete the operation.
//
STDAPI_(DWORD)
OfflineFilesEnable(
    _In_ BOOL bEnable, 
    _Out_ BOOL *pbRebootRequired
    );

#if (NTDDI_VERSION >= NTDDI_WIN8)
//
// Start Offline Files feature if it's available. 
// If the feature is not available then a ERROR_NOT_SUPPORTED error is returned.
//
STDAPI_(DWORD)
OfflineFilesStart(
    );
#endif

//
// Query the active/inactive and enabled/disabled state of the Offline Files feature.
//
//  - active/inactive describes the current running state.
//
//  - enabled/disabled describes the configured state as last successfully set by 
//    OfflineFilesEnable().
//
STDAPI_(DWORD)
OfflineFilesQueryStatus(
    _Out_opt_ BOOL *pbActive,
    _Out_opt_ BOOL *pbEnabled
    );

#if (NTDDI_VERSION >= NTDDI_WIN8)
//
// Query the active/inactive, enabled/disabled and available/unavailable state of the 
// Offline Files feature.
//
//  - active/inactive describes the current running state.
//
//  - enabled/disabled describes the configured state as last successfully set by 
//    OfflineFilesEnable().
//
//  - available/unavailable describes the Offline Files service state. 
//    If the status is inactive/enabled/available then the OfflineFilesStart function
//    can be used to transition the Offline Files feature to active state (no reboot required). 
//
STDAPI_(DWORD)
OfflineFilesQueryStatusEx(
    _Out_opt_ BOOL *pbActive,
    _Out_opt_ BOOL *pbEnabled,
    _Out_opt_ BOOL *pbAvailable
    );
#endif


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif // _INC_CSCAPI_H
