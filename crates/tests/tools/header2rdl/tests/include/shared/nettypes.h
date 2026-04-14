/*++

Copyright (c) Microsoft Corporation. All rights reserved.

Module Name:

    nettypes.h

Abstract:

    This header file contains type definitions for the NT TDI, NDI,
    DDI, and PDI interfaces which are not specific to a single interface.

Revision History:

--*/

#ifndef _NETTYPES_
#define _NETTYPES_

#pragma once
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


//
// The following basic type is used to provide extensibility in request
// and response packets.  The OFFSET type is used to contain a value which
// is interpreted as a relative address consisting of a number of bytes
// from the beginning of the immediate parent structure.
//

typedef ULONG OFFSET;

//
// The following basic type is used throughout all the layers to pass a
// string through an I/O interface which does not allow embedded pointers.
// To allocate a FLAT_STRING, one must make room for the correct number of
// buffer bytes in the allocation.
//

typedef struct _FLAT_STRING {
    SHORT MaximumLength;            // total size of string buffer.
    SHORT Length;                   // number of bytes represented in string.
    char Buffer [1];                // the buffer itself follows this struct.
} FLAT_STRING, *PFLAT_STRING;

//
// Basic type used to represent a network name, typically as a component of
// a transport address structure through the TDI.  This type is also passed
// through the NDI interface.  This type is declared as a structure so that
// it can be extended easily without modifying applications, even though it
// currently only has one element.
//
//

typedef struct _NETWORK_NAME {
    FLAT_STRING Name;                   // network name in FLAT_STRING format.
} NETWORK_NAME, *PNETWORK_NAME;

//
// Basic type used to represent an address at the hardware level of the
// network.  Hardware addresses are abstract types which are mapped to
// adapter addresses by the physical provider.  See the Physical Driver
// Interface specification for details on how this is accomplished.
//

#define HARDWARE_ADDRESS_LENGTH     6   // number of octets in a hardware address.

typedef struct _HARDWARE_ADDRESS {
    UCHAR Address [HARDWARE_ADDRESS_LENGTH];
} HARDWARE_ADDRESS, *PHARDWARE_ADDRESS;

//
// Network management variable types used by all interface levels.
//

#define NETMAN_VARTYPE_ULONG            0       // type is a ULONG.
#define NETMAN_VARTYPE_HARDWARE_ADDRESS 1       // type is a HARDWARE_ADDRESS.
#define NETMAN_VARTYPE_STRING           2       // type is a FLAT_STRING.


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif // _NETTYPES_
