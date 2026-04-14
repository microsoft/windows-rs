/*++

Microsoft Windows - File History
Copyright (c) 2010 Microsoft Corporation. All Rights Reserved.

Module Name:

    FhStatus.h

Abstract:

    This module contains the definitions of codes reflecting the
    File History status of a user account.

Environment:
    
    User mode.

--*/

#ifndef _FHSTATUS_
#define _FHSTATUS_

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


#define FH_STATE_NOT_TRACKED                    0x00
#define FH_STATE_OFF                            0x01
#define FH_STATE_DISABLED_BY_GP                 0x02
#define FH_STATE_FATAL_CONFIG_ERROR             0x03
#define FH_STATE_MIGRATING                      0x04
#define FH_STATE_REHYDRATING                    0x05


#define FH_STATE_TARGET_FS_LIMITATION               0x0D
#define FH_STATE_TARGET_ACCESS_DENIED               0x0E
#define FH_STATE_TARGET_VOLUME_DIRTY                0x0F
#define FH_STATE_TARGET_FULL_RETENTION_MAX          0x10
#define FH_STATE_TARGET_FULL                        0x11
#define FH_STATE_STAGING_FULL                       0x12
#define FH_STATE_TARGET_LOW_SPACE_RETENTION_MAX     0x13
#define FH_STATE_TARGET_LOW_SPACE                   0x14
#define FH_STATE_TARGET_ABSENT                      0x15
#define FH_STATE_TOO_MUCH_BEHIND                    0xF0
#define FH_STATE_NO_ERROR                           0xFF

// Indicates that File History is in a depreciated state
// where backup is not supported. This is only applicable
// if the user has an existing backup configured.
#define FH_STATE_BACKUP_NOT_SUPPORTED               0x810

#define FH_STATE_RUNNING                        0x100


#endif // NTDDI_VERSION >= NTDDI_WIN8 

#ifdef __cplusplus
}
#endif

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif // _FHSTATUS_

