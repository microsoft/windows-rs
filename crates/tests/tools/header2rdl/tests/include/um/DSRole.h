/*++ BUILD Version: 0001    // Increment this if a change has global effects

Copyright (c) 1997-1999 Microsoft Corporation

Module Name:

    dsrole.h

Abstract:

    This module contains the public interfaces to query the network roles of 
    workstations, servers, and DCs

--*/

#ifndef __DSROLE_H__
#define __DSROLE_H__

#if _MSC_VER > 1000
#pragma once
#endif
#include <winapifamily.h>

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)



#ifdef __cplusplus
extern "C" {
#endif

//
// Domain information
//
typedef enum _DSROLE_MACHINE_ROLE {

    DsRole_RoleStandaloneWorkstation,
    DsRole_RoleMemberWorkstation,
    DsRole_RoleStandaloneServer,
    DsRole_RoleMemberServer,
    DsRole_RoleBackupDomainController,
    DsRole_RolePrimaryDomainController

} DSROLE_MACHINE_ROLE;

//
// Previous server state
//
typedef enum _DSROLE_SERVER_STATE {

    DsRoleServerUnknown = 0,
    DsRoleServerPrimary,
    DsRoleServerBackup

} DSROLE_SERVER_STATE, *PDSROLE_SERVER_STATE;

typedef enum _DSROLE_PRIMARY_DOMAIN_INFO_LEVEL {

    DsRolePrimaryDomainInfoBasic = 1,
    DsRoleUpgradeStatus,
    DsRoleOperationState

} DSROLE_PRIMARY_DOMAIN_INFO_LEVEL;

//
// Flags to be used with the PRIMARY_DOMAIN_INFO_LEVEL structures below
//
#define DSROLE_PRIMARY_DS_RUNNING           0x00000001
#define DSROLE_PRIMARY_DS_MIXED_MODE        0x00000002
#define DSROLE_UPGRADE_IN_PROGRESS          0x00000004
#define DSROLE_PRIMARY_DS_READONLY          0x00000008
#define DSROLE_PRIMARY_DOMAIN_GUID_PRESENT  0x01000000

//
// Structure that correspond to the DSROLE_PRIMARY_DOMAIN_INFO_LEVEL
//
typedef struct _DSROLE_PRIMARY_DOMAIN_INFO_BASIC {

    DSROLE_MACHINE_ROLE MachineRole;
    ULONG Flags;
    LPWSTR DomainNameFlat;
    LPWSTR DomainNameDns;
    LPWSTR DomainForestName;
    GUID DomainGuid;

} DSROLE_PRIMARY_DOMAIN_INFO_BASIC, *PDSROLE_PRIMARY_DOMAIN_INFO_BASIC;

typedef struct _DSROLE_UPGRADE_STATUS_INFO {

    ULONG OperationState;
    DSROLE_SERVER_STATE PreviousServerState;

} DSROLE_UPGRADE_STATUS_INFO, *PDSROLE_UPGRADE_STATUS_INFO;

typedef enum _DSROLE_OPERATION_STATE {

    DsRoleOperationIdle = 0,
    DsRoleOperationActive,
    DsRoleOperationNeedReboot

} DSROLE_OPERATION_STATE;

typedef struct _DSROLE_OPERATION_STATE_INFO {

    DSROLE_OPERATION_STATE OperationState;

} DSROLE_OPERATION_STATE_INFO, *PDSROLE_OPERATION_STATE_INFO;

DWORD
WINAPI
DsRoleGetPrimaryDomainInformation(
    IN  LPCWSTR lpServer OPTIONAL,
    IN  DSROLE_PRIMARY_DOMAIN_INFO_LEVEL InfoLevel,
    OUT PBYTE *Buffer 
    );

VOID
WINAPI
DsRoleFreeMemory(
    IN PVOID    Buffer
    );


#ifdef __cplusplus
}
#endif


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#endif // __DSROLE_H__


