/*++

Copyright (c) 1995-1999 Microsoft Corporation

Module Name:

    ipxrip.h

Abstract:

    This module contains the definitions of the:

    interface management APIs structures
    rip MIB management APIs structures

Author:

    Stefan Solomon  06/30/1995

Revision History:


--*/

#ifndef _IPXRIP_
#define _IPXRIP_

#if _MSC_VER > 1000
#pragma once
#endif

#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

#include <ipxconst.h>

typedef struct _RIP_GLOBAL_INFO {
    DWORD       EventLogMask;
} RIP_GLOBAL_INFO, *PRIP_GLOBAL_INFO;

//********************************************************************
//								     *
//		    RIP Configuration Information		     *
//								     *
//********************************************************************


//*** RIP Interface Only Information ***

typedef struct _RIP_IF_INFO {

    ULONG	    AdminState;  // The desired state of the interface
    ULONG	    UpdateMode;	// RIP update mechanism used on this interface
    ULONG	    PacketType;  // The RIP packet type used on this interface
    ULONG	    Supply; // Send RIP updates on this interface
    ULONG	    Listen; // Listen to RIP updates on this interface
    ULONG	    PeriodicUpdateInterval; // in seconds - default 60
    ULONG	    AgeIntervalMultiplier; // default - 3

    } RIP_IF_INFO, *PRIP_IF_INFO;

// UpdateMode Values:
//
// This parameter controls the RIP	database update on this interface.
//
// If this is a LAN interface, use IPX_PERIODIC_UPDATE as default.
// If this is a WAN router interface with static routes, use IPX_NO_UPDATE as default.
// If you want to trigger an update on this interface and to keep the data
// as static data, use IPX_AUTO_STATIC value.
// IPX_AUTO_STATIC update can be set only on a WAN interface
//
// PacketType Default Values:
//
// If UpdateMode is set to IPX_NO_UPDATE, this field is meaningless.
// If this is a LAN interface, use IPX_STANDARD_PACKET_TYPE as default.
// If you want reliable delivery of the update data in a triggered update,
// use IPX_RELIABLE_DELIVERY_PACKET_TYPE (this can be set only in combination with
// IPX_AUTO_STATIC_UPDATE in the UpdateMode).

// RIP Route Filter Info
//
// These filters apply to routes accepted or advertised by RIP on each interface.

typedef struct _RIP_ROUTE_FILTER_INFO {

    UCHAR	    Network[4];
    UCHAR	    Mask[4];

    } RIP_ROUTE_FILTER_INFO, *PRIP_ROUTE_FILTER_INFO;

//*** RIP Filters Only Information ***
//
//  This header is followed by RIP_ROUTE_FILTER_STRUCTURES in order:
//  First Supply filters
//  Next Listen filters

typedef struct _RIP_IF_FILTERS {

    ULONG		    SupplyFilterAction;
    ULONG		    SupplyFilterCount;
    ULONG		    ListenFilterAction;
    ULONG		    ListenFilterCount;
    RIP_ROUTE_FILTER_INFO   RouteFilter[1];

    } RIP_IF_FILTERS, *PRIP_IF_FILTERS;

// FilterAction -

#define IPX_ROUTE_FILTER_PERMIT	    1
#define IPX_ROUTE_FILTER_DENY	    2

//
//*** RIP Interface Configuration Information ***
//
// This structure is passed in AddInterface and SetInterface Entry Points
//

typedef struct _RIP_IF_CONFIG {

    RIP_IF_INFO     RipIfInfo;
    RIP_IF_FILTERS  RipIfFilters;

    } RIP_IF_CONFIG, *PRIP_IF_CONFIG;

// ***********************************************************
// ***							   ***
// ***		RIP MIB Table Identifiers		   ***
// ***							   ***
// ***********************************************************


#define RIP_BASE_ENTRY			    0
#define RIP_INTERFACE_TABLE		    1

//************************************************************
//							     *
//		RIP MIB Basic Structures		     *
//							     *
//************************************************************

//
// RIP MIB Base Entry
//

typedef struct _RIPMIB_BASE {

    ULONG	    RIPOperState;

    } RIPMIB_BASE, *PRIPMIB_BASE;


//
// RIP MIB Interface Table Entry
//

typedef struct _RIP_IF_STATS {

    ULONG		    RipIfOperState;   // up, down or sleeping
    ULONG		    RipIfInputPackets;
    ULONG		    RipIfOutputPackets;

    } RIP_IF_STATS, *PRIP_IF_STATS;



typedef struct _RIP_INTERFACE {

    ULONG	    InterfaceIndex;
    RIP_IF_INFO	    RipIfInfo;
    RIP_IF_STATS    RipIfStats;

    } RIP_INTERFACE, *PRIP_INTERFACE;

//***************************************************************
//								*
//	     INPUT DATA For: Get, GetFirst, GetNext		*
//								*
//***************************************************************

typedef struct _RIP_MIB_GET_INPUT_DATA {

    ULONG		TableId;
    ULONG		InterfaceIndex;

    } RIP_MIB_GET_INPUT_DATA, *PRIP_MIB_GET_INPUT_DATA;

//***************************************************************
//								*
//	     INPUT DATA For: Create, Delete, Set		*
//								*
//***************************************************************

typedef struct _RIP_MIB_SET_INPUT_DATA {

    ULONG		 TableId;
    RIP_INTERFACE	 RipInterface;

    } RIP_MIB_SET_INPUT_DATA, *PRIP_MIB_SET_INPUT_DATA;

//
// ***			RIP Base Entry					***
//

// MIB Functions: Get

// INPUT DATA: RIP_MIB_GET_INPUT_DATA and Index is not used

// OUTPUT DATA: RIP_INTERFACE

//
// ***			RIP Interface Table				    ***
//

// MIB Functions: Get, GetFirst, GetNext, Set

// INPUT DATA: RIP_MIB_GET_INPUT_DATA for Get, GetFirst and GetNext
//	       RIP_MIB_SET_INPUT_DATA for Set

//
// OUTPUT DATA: RIP_INTERFACE
//

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif
