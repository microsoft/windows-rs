/*++

Copyright (c) 1991-1999  Microsoft Corporation

Module Name:

    ntdskreg.h

Abstract:

    This file contains disk registry data structures.


--*/

#ifndef _NTDSKREG_
#define _NTDSKREG_

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
// Component types for FT members.  If a disk partition
// is not a part of an FT component then it is stamped
// as "NotAnFtMember".
//

typedef enum {
    Mirror,
    Stripe,
    StripeWithParity,
    VolumeSet,
    NotAnFtMember,
    WholeDisk
} FT_TYPE;

//
// FT_PARTITION_STATE is an enumerated type that describes the state of an
// NTFT member.
// Up to one member can be 'orphaned', that is missing or out of sync with
// the other members.  The orphaned member will remain orphaned until recovery
// action is commenced in DISKMAN.EXE.
// When READs are directed at an orphaned member, the data is returned from
// the redundant copy.  WRITEs go to the secondary members only.
//
// Note that SyncRedundantCopy should NEVER be seen by anything other
// than ft itself.
//

typedef enum _FT_PARTITION_STATE {
    Healthy,
    Orphaned,
    Regenerating,
    Initializing,
    SyncRedundantCopy
} FT_PARTITION_STATE, *PFT_PARTITION_STATE;

//
// The description of a partition on a disk.
//

#include "pshpack4.h"
typedef struct _DISK_PARTITION {

    FT_TYPE            FtType;
    FT_PARTITION_STATE FtState;
    LARGE_INTEGER      StartingOffset;
    LARGE_INTEGER      Length;
    LARGE_INTEGER      FtLength;
    ULONG              ReservedTwoLongs[2];
    UCHAR              DriveLetter;
    BOOLEAN            AssignDriveLetter;
    USHORT             LogicalNumber;
    USHORT             FtGroup;
    USHORT             FtMember;
    BOOLEAN            Modified;
    UCHAR              ReservedChars[3];

} DISK_PARTITION, *PDISK_PARTITION;

//
// The description of a disk.
//

typedef struct _DISK_DESCRIPTION {

    USHORT NumberOfPartitions;
    USHORT ReservedShort;
    ULONG  Signature;

    //
    // An array for NumberOfPartitions
    //

    DISK_PARTITION Partitions[1];

} DISK_DESCRIPTION, *PDISK_DESCRIPTION;

//
// Header for all disk descriptions.
//

typedef struct _DISK_REGISTRY {

    USHORT NumberOfDisks;
    USHORT ReservedShort;

    //
    // An array for NumberOfDisks.
    //

    DISK_DESCRIPTION Disks[1];

} DISK_REGISTRY, *PDISK_REGISTRY;
#include "poppack.h"

//
// Function prototypes.
//


#if (NTDDI_VERSION < NTDDI_VISTA)

//
// Set the current registry information.
//

NTSTATUS
DiskRegistrySet(
    _In_ PDISK_REGISTRY Buffer
    );


//
// Add a single disk to the registry.  This disk is not a part of
// a Fault Tolerance volume.
//

NTSTATUS
DiskRegistryAddNewDisk(
    _In_ PDISK_DESCRIPTION Buffer
    );

//
// Get the current registry information.
// A Buffer address == NULL indicates that the caller only wants.
// to know the size of the registry information.
//

NTSTATUS
DiskRegistryGet(
    _Out_ PDISK_REGISTRY Buffer,
    _Out_ PULONG         LengthReturned
    );

//
// Determine if FT is installed and currently active in the system.
//

BOOLEAN
FtInstalled(
    );

//
// Cause an FT set to be initialized at the earliest possible moment.
//

VOID
DiskRegistryInitializeSet(
    _In_ USHORT  FtType,
    _In_ USHORT  FtGroup
    );

//
// Cause an FT set member to be regenerated at the earliest possible moment.
//

VOID
DiskRegistryRegenerateSet(
    _In_ USHORT  FtType,
    _In_ USHORT  FtGroup,
    _In_ USHORT  FtMember
    );

//
// Enable the FT driver.
//

BOOLEAN
DiskRegistryEnableFt();

//
// Disable the FT driver.
//

VOID
DiskRegistryDisableFt();

//
// Determine if the existing FT registry information requires the FT driver.
//

BOOLEAN
DiskRegistryRequiresFt();

//
// Update drive letter assignment.
//

BOOLEAN
DiskRegistryAssignDriveLetter(
    _In_ ULONG         Signature,
    _In_ LARGE_INTEGER StartingOffset,
    _In_ LARGE_INTEGER Length,
    _In_ UCHAR         DriveLetter
    );

//
// Determine if DoubleSpace support is set to automount double space
// volumes on removable media.
//

BOOLEAN
DiskRegistryAutomountCurrentState(
    );

//
// Set the automount state for double space volumes on removable media.
//

NTSTATUS
DiskRegistryDblSpaceRemovable(
    _In_ BOOLEAN Automount
    );

//
// Set the drive letter for space mount information in the registry.
//

NTSTATUS
DiskRegistryAssignDblSpaceLetter(
    _In_ PWSTR CvfName,
    _In_ WCHAR DriveLetter
    );

//
// Set up the registry information for CdRom drive letters.
//

NTSTATUS
DiskRegistryAssignCdRomLetter(
    _In_ PWSTR CdromName,
    _In_ WCHAR DriveLetter
    );

#endif // < NTDDI_VISTA

#ifdef __cplusplus
}
#endif


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#endif // _NTDSKREG_
