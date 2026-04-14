/*++

Copyright (c) 1995-1999 Microsoft Corporation

Module Name:

    ipxrtdef.h

Abstract:

    This module contains the definitions of the:

    interface management APIs structures
    ipx global router management APIs structures
    ipx router MIB management APIs structures

Author:

    Stefan Solomon  03/03/1995

Revision History:


--*/

#ifndef _IPXRTDEF_
#define _IPXRTDEF_

#if _MSC_VER > 1000
#pragma once
#endif

#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

#include <ipxconst.h>
#include <ipxsap.h>
#include <ipxrip.h>
#include <stm.h>
#include <ipxtfflt.h>

//****************************************************************
//***							       ***
//***			Global Definitions		       ***
//***							       ***
//****************************************************************

//
//  Version of this router
//

#define IPX_ROUTER_VERSION_1		    RTR_INFO_BLOCK_VERSION

//
//  IPX Protocols
//

#define IPX_PROTOCOL_LOCAL		    1
#define IPX_PROTOCOL_STATIC		    2

//
// Interface Type Definitions for MIB reporting - these are mapped from the
// DDM interface type
//

#define IF_TYPE_OTHER				1
#define IF_TYPE_LAN				2
#define IF_TYPE_WAN_ROUTER			3
#define IF_TYPE_WAN_WORKSTATION			4  // remote workstation dialing in
#define IF_TYPE_INTERNAL			5  // the internal (virtual) interface
#define IF_TYPE_PERSONAL_WAN_ROUTER		6
#define IF_TYPE_ROUTER_WORKSTATION_DIALOUT	7  // local workstation dialing out
#define IF_TYPE_STANDALONE_WORKSTATION_DIALOUT	8

//
// Definitions for Table of Contents Entries Info Types
//

#define IPX_INTERFACE_INFO_TYPE			1
#define IPX_STATIC_ROUTE_INFO_TYPE		2
#define IPX_STATIC_SERVICE_INFO_TYPE		3
#define IPX_SERVICE_FILTER_INFO_TYPE		4
#define IPX_ROUTE_FILTER_INFO_TYPE		5
#define IPX_IN_TRAFFIC_FILTER_INFO_TYPE		6
#define IPX_ADAPTER_INFO_TYPE			7
#define IPXWAN_INTERFACE_INFO_TYPE		8
#define IPX_GLOBAL_INFO_TYPE			9
#define IPX_STATIC_NETBIOS_NAME_INFO_TYPE	10
#define IPX_IN_TRAFFIC_FILTER_GLOBAL_INFO_TYPE	11
#define IPX_OUT_TRAFFIC_FILTER_INFO_TYPE		12
#define IPX_OUT_TRAFFIC_FILTER_GLOBAL_INFO_TYPE	13

//****************************************************************
//***							       ***
//***	Specific Info Structures In The Information Block      ***
//***							       ***
//****************************************************************

//*** IPX Router Manager Global Info ***

#define     IPX_SMALL_ROUTING_TABLE_HASH_SIZE	    31
#define     IPX_MEDIUM_ROUTING_TABLE_HASH_SIZE	    257
#define     IPX_LARGE_ROUTING_TABLE_HASH_SIZE	    2047

typedef struct _IPX_GLOBAL_INFO {

    ULONG	    RoutingTableHashSize;
    ULONG       EventLogMask;
    } IPX_GLOBAL_INFO, *PIPX_GLOBAL_INFO;


//*** IPX Interface Info ***

typedef struct _IPX_IF_INFO {

    ULONG	    AdminState;        // The desired state of the interface
    ULONG	    NetbiosAccept;     // Accept Netbios broadcast packets
    ULONG	    NetbiosDeliver;    // Deliver Netbios broadcast packets

    } IPX_IF_INFO, *PIPX_IF_INFO;


// Interface Device Type Definitions

#define IPX_DEDICATED_LINK	    1	// includes LAN, leased lines, frame-relay
#define IPX_DIALED_LINK 	    2	// dial on demand links

// Default values:
//
// AdminState: enabled - disabling it disables also RIP, SAP and anything else
// NetbiosAccept: enabled
// NetbiosDeliver: enabled on LAN interface, disabled on WAN interface
// IpxWanNegotiation: disabled.

// The interface name corresponds to the local adapter name in the case the
// interface is the local LAN. If the adapter is multiplexed with different
// packet types, there is a unique net number assigned to each pseudo-adapter.
// In this case the NetNumber is used to differentiate which pseudo-adapter gets
// this interface info.

//*** IPXWAN Interface Info ***

typedef struct _IPXWAN_IF_INFO {

    ULONG	    AdminState;  // Enable/Disable IPXWAN negotiation

    } IPXWAN_IF_INFO, *PIPXWAN_IF_INFO;

//*** Static Route Entry ***

typedef struct _IPX_STATIC_ROUTE_INFO {

    union {

    ULONG	DwordAlign;
    UCHAR	Network[4];  };
    USHORT	TickCount;
    USHORT	HopCount;
    UCHAR	NextHopMacAddress[6];

    } IPX_STATIC_ROUTE_INFO, *PIPX_STATIC_ROUTE_INFO;


//*** Static Service Entry ***

typedef IPX_SERVER_ENTRY IPX_STATIC_SERVICE_INFO, *PIPX_STATIC_SERVICE_INFO;


//*** Static Netbios Name Entry ***

typedef struct	_IPX_STATIC_NETBIOS_NAME_INFO {

    union {

    ULONG	DwordAlign;
    UCHAR	Name[16];     };

    } IPX_STATIC_NETBIOS_NAME_INFO, *PIPX_STATIC_NETBIOS_NAME_INFO;


//
// *** IPX LAN ADAPTER INFO ***
//

#define MAX_ADAPTER_NAME_LEN		    48

typedef struct _IPX_ADAPTER_INFO {

    ULONG	PacketType;
    WCHAR	AdapterName[MAX_ADAPTER_NAME_LEN];

    } IPX_ADAPTER_INFO, *PIPX_ADAPTER_INFO;

#define AUTO_DETECT_PACKET_TYPE 	    0xFFFFFFFF



// ***********************************************************
// ***							   ***
// ***		IPX MIB Table Identifiers		   ***
// ***							   ***
// ***********************************************************


#define IPX_BASE_ENTRY			    0
#define IPX_INTERFACE_TABLE		    1
#define IPX_DEST_TABLE			    2	 // IPX Best Routes Table
#define IPX_STATIC_ROUTE_TABLE		    3	 // IPX Static Routes Table
#define IPX_SERV_TABLE			    4	 // IPX Services Table
#define IPX_STATIC_SERV_TABLE		    5	 // IPX Static Services Table

#define MAX_IPX_MIB_TABLES		    6


//
// Some Global MIB Constants
//

// max size of the interface readable name

#define IPX_INTERFACE_ANSI_NAME_LEN	    48

// ***************************************************************************
//
// ***	     IPX MIB APIs Input/Output Structures For Each Table	   ***
//
//****************************************************************************

// Global definition of the MIB Identifier (locates the table and the row index)

typedef struct	_IF_TABLE_INDEX {

    ULONG	InterfaceIndex;

    } IF_TABLE_INDEX, *PIF_TABLE_INDEX;

typedef struct _ROUTING_TABLE_INDEX {

    UCHAR	Network[4];

    } ROUTING_TABLE_INDEX, *PROUTING_TABLE_INDEX;

typedef struct _STATIC_ROUTES_TABLE_INDEX {

    ULONG	InterfaceIndex;
    UCHAR	Network[4];

    } STATIC_ROUTES_TABLE_INDEX, *PSTATIC_ROUTES_TABLE_INDEX;

typedef struct _SERVICES_TABLE_INDEX {

    USHORT	ServiceType;
    UCHAR	ServiceName[48];

    } SERVICES_TABLE_INDEX, *PSERVICES_TABLE_INDEX;

typedef struct _STATIC_SERVICES_TABLE_INDEX {

    ULONG	InterfaceIndex;
    USHORT	ServiceType;
    UCHAR	ServiceName[48];

    } STATIC_SERVICES_TABLE_INDEX, *PSTATIC_SERVICES_TABLE_INDEX;

typedef union _IPX_MIB_INDEX {

    IF_TABLE_INDEX		InterfaceTableIndex;
    ROUTING_TABLE_INDEX		RoutingTableIndex;
    STATIC_ROUTES_TABLE_INDEX	StaticRoutesTableIndex;
    SERVICES_TABLE_INDEX	ServicesTableIndex;
    STATIC_SERVICES_TABLE_INDEX StaticServicesTableIndex;

    } IPX_MIB_INDEX, *PIPX_MIB_INDEX;

//**********************************************************************
//								       *
//	     INPUT DATA For: Get, GetFirst, GetNext		       *
//								       *
//**********************************************************************

typedef struct _IPX_MIB_GET_INPUT_DATA {

    ULONG		TableId;
    IPX_MIB_INDEX	MibIndex;

    } IPX_MIB_GET_INPUT_DATA, *PIPX_MIB_GET_INPUT_DATA;

typedef struct _IPXMIB_BASE {

    ULONG	    OperState;
    UCHAR	    PrimaryNetNumber[4];
    UCHAR	    Node[6];
    UCHAR	    SysName[IPX_INTERFACE_ANSI_NAME_LEN];
    ULONG	    MaxPathSplits;
    ULONG	    IfCount;
    ULONG	    DestCount;
    ULONG	    ServCount;

    } IPXMIB_BASE, *PIPXMIB_BASE;

// structure used to gather interface statistics

typedef struct _IPX_IF_STATS {

	ULONG		IfOperState;
	ULONG		MaxPacketSize;
	ULONG		InHdrErrors;
	ULONG		InFiltered;
	ULONG		InNoRoutes;
	ULONG		InDiscards;
	ULONG		InDelivers;
	ULONG		OutFiltered;
	ULONG		OutDiscards;
	ULONG		OutDelivers;
	ULONG		NetbiosReceived;
	ULONG		NetbiosSent;

	} IPX_IF_STATS, *PIPX_IF_STATS;

// structure describing the MIB Row for the IPX_INTERFACE_TABLE in the
// IPX_INTERFACE_GROUP

typedef struct _IPX_INTERFACE {

    ULONG	    InterfaceIndex;
    ULONG	    AdminState;
    ULONG	    AdapterIndex;
    UCHAR	    InterfaceName[IPX_INTERFACE_ANSI_NAME_LEN];
    ULONG	    InterfaceType;
    ULONG	    MediaType;
    UCHAR	    NetNumber[4];
    UCHAR	    MacAddress[6];
    ULONG	    Delay;
    ULONG	    Throughput;
    ULONG	    NetbiosAccept;
    ULONG	    NetbiosDeliver;
    ULONG	    EnableIpxWanNegotiation;
    IPX_IF_STATS    IfStats;

    } IPX_INTERFACE, *PIPX_INTERFACE;


typedef struct _IPX_ROUTE {

    ULONG	InterfaceIndex; // see ipxconst.h for specific indices definitions
    ULONG	Protocol;
    UCHAR	Network[4];
    USHORT	TickCount;
    USHORT	HopCount;
    UCHAR	NextHopMacAddress[6];
    ULONG	Flags;

    }  IPX_ROUTE, *PIPX_ROUTE;

//
// INPUT DATA For: Create, Delete, Set
//

typedef	union _IPX_MIB_ROW {

	IPX_INTERFACE	 Interface;
	IPX_ROUTE	 Route;
	IPX_SERVICE	 Service;

	} IPX_MIB_ROW, *PIPX_MIB_ROW;


typedef struct _IPX_MIB_SET_INPUT_DATA {

    ULONG		 TableId;
    IPX_MIB_ROW 	 MibRow;

    } IPX_MIB_SET_INPUT_DATA, *PIPX_MIB_SET_INPUT_DATA;

//
// ***			IPX Base Entry					***
//

// MIB Functions: Get

// INPUT DATA: IPX_MIB_GET_INPUT_DATA and Index is not used

// OUTPUT DATA:

//
// ***			IPX Interface Table				    ***
//

// MIB Functions: Get, GetFirst, GetNext, Set

// INPUT DATA: IPX_MIB_GET_INPUT_DATA and Index is IF_TABLE_INDEX for Get, GetFirst and GetNext
//	       IPX_MIB_SET_INPUT_DATA and MibRow is IPX_INTERFACE for Set

//
// OUTPUT DATA: described by the IPX_INTERFACE structure below
//

//
// ***			IPX Routes Table				   ***
//

// MIB Functions:  Get, GetFirst, GetNext

// INPUT DATA: IPX_MIB_INPUT_DATA with Index ROUTES_TABLE_INDEX

// OUTPUT DATA: IPX_ROUTE
//


//
// ***			IPX Static Routes Table				   ***
//

// MIB Functions:  Create, Delete, Get, GetFirst, GetNext, Set

// INPUT DATA: IPX_MIB_GET_INPUT_DATA with Index STATIC_ROUTES_TABLE_INDEX for Get, GetFirst, GetNext
//	       IPX_MIB_SET_INPUT_DATA and MibRow is IPX_ROUTE for Create, Delete, Set

// OUTPUT DATA: IPX_ROUTE
//

//
// ***			 IPX Services Table				   ***
//

// MIB Functions: Get, GetFirst, GetNext

// INPUT DATA: IPX_MIB_INPUT_DATA with Index SERVICES_TABLE_INDEX

// OUTPUT DATA: The output data is the structure IPX_SERVICE

//
// ***			 IPX Static Services Table			   ***
//

// MIB Functions: Create, Delete, Get, GetFirst, GetNext

// INPUT DATA: IPX_MIB_GET_INPUT_DATA with Index STATIC_SERVICES_TABLE_INDEX for Get, GetFirst, GetNext
//	       IPX_MIB_SET_INPUT_DATA and MibRow is IPX_SERVICE for Create, Delete and Set.

// OUTPUT DATA: IPX_SERVICE


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif	 // _IPXRTDEF_
