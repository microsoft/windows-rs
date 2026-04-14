/*++

Copyright (c) 1996-1999 Microsoft Corporation

Module Name:

    fttypes.h

Abstract:

    This header file defines types and structures for the FT API
    and the IOCTLs used to implement the FT API.

Notes:

Revision History:

--*/

#if !defined( _FT_TYPES_DEFINITION_ )

#define _FT_TYPES_DEFINITION_ 1

#if _MSC_VER > 1000
#pragma once
#endif
#include <winapifamily.h>

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)


//
// The logical disk id type is used to uniquely identify a logical disk.  It
// needs to be unique in the universe.
//

typedef LONGLONG FT_LOGICAL_DISK_ID, *PFT_LOGICAL_DISK_ID;

#if (NTDDI_VERSION < NTDDI_VISTA)

//
// This enumerated type defines all of the different types of logical
// disks that can be constructed with this API.
//

typedef enum _FT_LOGICAL_DISK_TYPE {
    FtPartition,
    FtVolumeSet,
    FtStripeSet,
    FtMirrorSet,
    FtStripeSetWithParity,
    FtRedistribution
} FT_LOGICAL_DISK_TYPE, *PFT_LOGICAL_DISK_TYPE;

//
// This enumerated type defines all of the possible states that members
// of an FT set can have.
//

typedef enum _FT_MEMBER_STATE {
    FtMemberHealthy,
    FtMemberRegenerating,
    FtMemberOrphaned
} FT_MEMBER_STATE, *PFT_MEMBER_STATE;

//
// This is the structure needed for the configuration information of
// a logical disk type of 'FtPartition'.
//

typedef struct _FT_PARTITION_CONFIGURATION_INFORMATION {
    ULONG       Signature;
    ULONG       DiskNumber;
    LONGLONG    ByteOffset;
} FT_PARTITION_CONFIGURATION_INFORMATION, *PFT_PARTITION_CONFIGURATION_INFORMATION;

//
// This is the structure needed for the configuration information of
// a logical disk type of 'FtStripeSet'.
//

typedef struct _FT_STRIPE_SET_CONFIGURATION_INFORMATION {
    ULONG   StripeSize;
} FT_STRIPE_SET_CONFIGURATION_INFORMATION, *PFT_STRIPE_SET_CONFIGURATION_INFORMATION;

//
// This is the structure needed for the configuration information of
// a logical disk type of 'FtMirrorSet'.
//

typedef struct _FT_MIRROR_SET_CONFIGURATION_INFORMATION {
    LONGLONG    MemberSize;
} FT_MIRROR_SET_CONFIGURATION_INFORMATION, *PFT_MIRROR_SET_CONFIGURATION_INFORMATION;

//
// This is the structure needed for the configuration information of
// a logical disk type of 'FtStripeSetWithParity'.
//

typedef struct _FT_STRIPE_SET_WITH_PARITY_CONFIGURATION_INFORMATION {
    LONGLONG    MemberSize;
    ULONG       StripeSize;
} FT_STRIPE_SET_WITH_PARITY_CONFIGURATION_INFORMATION, *PFT_STRIPE_SET_WITH_PARITY_CONFIGURATION_INFORMATION;

//
// This is the structure needed for the configuration information of
// a logical disk type of 'FtRedistribution'.
//

typedef struct _FT_REDISTRIBUTION_CONFIGURATION_INFORMATION {
    ULONG   StripeSize;
    USHORT  FirstMemberWidth;
    USHORT  SecondMemberWidth;
} FT_REDISTRIBUTION_CONFIGURATION_INFORMATION, *PFT_REDISTRIBUTION_CONFIGURATION_INFORMATION;

//
// This is the structure needed for the state of a logical disk type of
// 'FtStripeSetWithParity' or 'FtMirrorSet'.
//

typedef struct _FT_MIRROR_AND_SWP_STATE_INFORMATION {
    BOOLEAN         IsDirty;
    BOOLEAN         IsInitializing;
    USHORT          UnhealthyMemberNumber;
    FT_MEMBER_STATE UnhealthyMemberState;
} FT_MIRROR_AND_SWP_STATE_INFORMATION, *PFT_MIRROR_AND_SWP_STATE_INFORMATION;

//
// This is the structure needed for the state of a logical disk type of
// 'FtRedistribution'.
//

typedef struct _FT_REDISTRIBUTION_STATE_INFORMATION {
    LONGLONG    BytesRedistributed;
} FT_REDISTRIBUTION_STATE_INFORMATION, *PFT_REDISTRIBUTION_STATE_INFORMATION;

#endif // < NTDDI_VISTA


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#endif
