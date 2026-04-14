/*++

Copyright (c) 1995-1999 Microsoft Corporation

Module Name:

    ipxsap.h

Abstract:

    This module contains the definitions of the:

    interface management APIs structures
    sap global management APIs structures
    sap MIB management APIs structures

Author:

    Stefan Solomon  06/30/1995

Revision History:


--*/

#ifndef _IPXSAP_
#define _IPXSAP_

#if _MSC_VER > 1000
#pragma once
#endif

#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

#include <ipxconst.h>

typedef struct _SAP_GLOBAL_INFO {
    DWORD       EventLogMask;
} SAP_GLOBAL_INFO, *PSAP_GLOBAL_INFO;

//********************************************************************
//								     *
//		    SAP Configuration Information		     *
//								     *
//********************************************************************

//*** SAP Interface Configuration Information ***

typedef struct _SAP_IF_INFO {

    ULONG	    AdminState;  // The desired state of the interface
    ULONG	    UpdateMode;	// SAP update mechanism used on this interface
    ULONG	    PacketType;  // The SAP packet type used on this interface
    ULONG	    Supply; // Send SAP updates on this interface
    ULONG	    Listen; // Listen to SAP updates on this interface
    ULONG	    GetNearestServerReply; // Reply to GetNearestServer
    ULONG	    PeriodicUpdateInterval; // in seconds - default 60
    ULONG	    AgeIntervalMultiplier; // default 3

    } SAP_IF_INFO, *PSAP_IF_INFO;

// UpdateMode Values:
//
// This parameter controls the SAP	database update on this interface.
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


// IPX Service Filter Info
//

typedef struct _SAP_SERVICE_FILTER_INFO {
    union {
        USHORT	    ServiceType;
        ULONG       ServiceType_align;  // Ensures aligment
        };
    UCHAR	    ServiceName[48];
    } SAP_SERVICE_FILTER_INFO, *PSAP_SERVICE_FILTER_INFO;

// ServiceType - a wildcard (0xFFFF) means any type.

// ServiceName - (1-47)chars service name. A null byte as the first byte
// signifies ANY server of this type

//*** SAP Filters Only Information ***

typedef struct _SAP_IF_FILTERS {

    ULONG		    SupplyFilterAction;
    ULONG		    SupplyFilterCount;
    ULONG		    ListenFilterAction;
    ULONG		    ListenFilterCount;
    SAP_SERVICE_FILTER_INFO ServiceFilter[1];

    } SAP_IF_FILTERS, *PSAP_IF_FILTERS;

// FilterAction -

#define IPX_SERVICE_FILTER_PERMIT	    1
#define IPX_SERVICE_FILTER_DENY 	    2

//
//*** SAP Interface Configuration Information ***
//
// This structure is passed in AddInterface and SetInterface Entry Points
//

typedef struct _SAP_IF_CONFIG {

    SAP_IF_INFO	    SapIfInfo;
    SAP_IF_FILTERS  SapIfFilters;

    } SAP_IF_CONFIG, *PSAP_IF_CONFIG;


// ***********************************************************
// ***							   ***
// ***		SAP MIB Table Identifiers		   ***
// ***							   ***
// ***********************************************************


#define SAP_BASE_ENTRY			    0
#define SAP_INTERFACE_TABLE		    1

//************************************************************
//							     *
//		SAP MIB Basic Structures		     *
//							     *
//************************************************************

//
// SAP MIB Base Entry
//

typedef struct _SAP_MIB_BASE {

    ULONG	    SapOperState;

    } SAP_MIB_BASE, *PSAP_MIB_BASE;


//
// SAP MIB Interface Table Entry
//

typedef struct _SAP_IF_STATS {

    ULONG		    SapIfOperState;   // up, down or sleeping
    ULONG		    SapIfInputPackets;
    ULONG		    SapIfOutputPackets;

    } SAP_IF_STATS, *PSAP_IF_STATS;


typedef struct _SAP_INTERFACE {

    ULONG	    InterfaceIndex;
    SAP_IF_INFO	    SapIfInfo;
    SAP_IF_STATS    SapIfStats;

    } SAP_INTERFACE, *PSAP_INTERFACE;

//***************************************************************
//								*
//	     INPUT DATA For: Get, GetFirst, GetNext		*
//								*
//***************************************************************

typedef struct _SAP_MIB_GET_INPUT_DATA {

    ULONG		TableId;
    ULONG		InterfaceIndex;

    } SAP_MIB_GET_INPUT_DATA, *PSAP_MIB_GET_INPUT_DATA;

//***************************************************************
//								*
//	     INPUT DATA For: Create, Delete, Set		*
//								*
//***************************************************************

typedef struct _SAP_MIB_SET_INPUT_DATA {

    ULONG				TableId;
	SAP_INTERFACE		SapInterface;
    } SAP_MIB_SET_INPUT_DATA, *PSAP_MIB_SET_INPUT_DATA;

//
// ***			SAP Base Entry					***
//

// MIB Functions: Get

// INPUT DATA: SAP_MIB_GET_INPUT_DATA and Index is not used

// OUTPUT DATA: SAP_INTERFACE

//
// ***			SAP Interface Table				    ***
//

// MIB Functions: Get, GetFirst, GetNext, Set

// INPUT DATA: SAP_MIB_GET_INPUT_DATA for Get, GetFirst and GetNext
//	       SAP_MIB_SET_INPUT_DATA for Set

//
// OUTPUT DATA: SAP_INTERFACE
//


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif
