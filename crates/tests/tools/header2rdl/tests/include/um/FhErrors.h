/*++

Microsoft Windows - File History
Copyright (c) 2010 Microsoft Corporation. All Rights Reserved.

Module Name:

    FhErrors.h

Abstract:

    This module contains the definitions of the error codes
    returned by File History APIs and components.

Environment:
    
    User mode.

--*/

#ifndef _FHERRORS_
#define _FHERRORS_

#if _MSC_VER > 1000
#pragma once
#endif

#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

#ifdef __cplusplus
extern "C" {
#endif

#if NTDDI_VERSION >= NTDDI_WIN8 


///////////////////////////////////////////////////////////////////////
//
// Error codes for the Configuration Manager. (0x0300 - 0x03ff)
//
///////////////////////////////////////////////////////////////////////

//
// The Config File is corrupted and hence invalid
//

#define FHCFG_E_CORRUPT_CONFIG_FILE         MAKE_HRESULT(SEVERITY_ERROR, FACILITY_ITF, 0x0300)

//
// The Config File not found
//

#define FHCFG_E_CONFIG_FILE_NOT_FOUND       MAKE_HRESULT(SEVERITY_ERROR, FACILITY_ITF, 0x0301)

//
// The Config File already exists
//

#define FHCFG_E_CONFIG_ALREADY_EXISTS       MAKE_HRESULT(SEVERITY_ERROR, FACILITY_ITF, 0x0302)

//
// A valid configuration has not been loaded
//

#define FHCFG_E_NO_VALID_CONFIGURATION_LOADED       MAKE_HRESULT(SEVERITY_ERROR, FACILITY_ITF, 0x0303)

//
// Target is not connected
//

#define FHCFG_E_TARGET_NOT_CONNECTED        MAKE_HRESULT(SEVERITY_ERROR, FACILITY_ITF, 0x0304)

//
// Configuration has been loaded previously
//

#define FHCFG_E_CONFIGURATION_PREVIOUSLY_LOADED     MAKE_HRESULT(SEVERITY_ERROR, FACILITY_ITF, 0x0305)

//
// Default target verification failed
//

#define FHCFG_E_TARGET_VERIFICATION_FAILED  MAKE_HRESULT(SEVERITY_ERROR, FACILITY_ITF, 0x0306)

//
// Target is not configured
//

#define FHCFG_E_TARGET_NOT_CONFIGURED       MAKE_HRESULT(SEVERITY_ERROR, FACILITY_ITF, 0x0307)

//
// Target doesn't have enough free space
//

#define FHCFG_E_TARGET_NOT_ENOUGH_FREE_SPACE        MAKE_HRESULT(SEVERITY_ERROR, FACILITY_ITF, 0x0308)

//
// Target cannot be used for File History
//

#define FHCFG_E_TARGET_CANNOT_BE_USED       MAKE_HRESULT(SEVERITY_ERROR, FACILITY_ITF, 0x0309)

//
// Rehydration can't be performed in the current configuration state
//

#define FHCFG_E_INVALID_REHYDRATION_STATE               MAKE_HRESULT(SEVERITY_ERROR, FACILITY_ITF, 0x30A)

//
// Changing target recommendation is not allowed
//

#define FHCFG_E_RECOMMENDATION_CHANGE_NOT_ALLOWED       MAKE_HRESULT(SEVERITY_ERROR, FACILITY_ITF, 0x0310)

//
// The target was rehydrated on another PC.
//

#define FHCFG_E_TARGET_REHYDRATED_ELSEWHERE             MAKE_HRESULT(SEVERITY_ERROR, FACILITY_ITF, 0X0311)

//
// The legacy backup target was not supported by FileHistory
//

#define FHCFG_E_LEGACY_TARGET_UNSUPPORTED               MAKE_HRESULT(SEVERITY_ERROR, FACILITY_ITF, 0X0312)

//
// The validation result of the legacy backup target was not supported
//

#define FHCFG_E_LEGACY_TARGET_VALIDATION_UNSUPPORTED    MAKE_HRESULT(SEVERITY_ERROR, FACILITY_ITF, 0X0313)

//
// The legacy backup user was fully excluded from backups
//

#define FHCFG_E_LEGACY_BACKUP_USER_EXCLUDED             MAKE_HRESULT(SEVERITY_ERROR, FACILITY_ITF, 0X0314)

//
// The legacy backup was not found
//

#define FHCFG_E_LEGACY_BACKUP_NOT_FOUND                 MAKE_HRESULT(SEVERITY_ERROR, FACILITY_ITF, 0X0315)


///////////////////////////////////////////////////////////////////////
//
// Error codes for the File History Service. (0x0600 - 0x06ff)
//
///////////////////////////////////////////////////////////////////////

//
// Backups are blocked for the given configuration
//

#define FHSVC_E_BACKUP_BLOCKED      MAKE_HRESULT(SEVERITY_ERROR, FACILITY_ITF, 0x0600)

//
// File History is not configured for the user
//

#define FHSVC_E_NOT_CONFIGURED      MAKE_HRESULT(SEVERITY_ERROR, FACILITY_ITF, 0x0601)

//
// The specified configuration is disabled by the user
//

#define FHSVC_E_CONFIG_DISABLED     MAKE_HRESULT(SEVERITY_ERROR, FACILITY_ITF, 0x0602)

//
// The specified configuration is disabled via Group Policy
//

#define FHSVC_E_CONFIG_DISABLED_GP  MAKE_HRESULT(SEVERITY_ERROR, FACILITY_ITF, 0x0603)

//
// There is a fatal error with the specified configuration, backup cannot be even started
//

#define FHSVC_E_FATAL_CONFIG_ERROR  MAKE_HRESULT(SEVERITY_ERROR, FACILITY_ITF, 0x0604)

//
// The specified configuration is undergoing rehydration
//

#define FHSVC_E_CONFIG_REHYDRATING  MAKE_HRESULT(SEVERITY_ERROR, FACILITY_ITF, 0x0605)

#endif // NTDDI_VERSION >= NTDDI_WIN8 

#ifdef __cplusplus
}
#endif

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif // _FHERRORS_

