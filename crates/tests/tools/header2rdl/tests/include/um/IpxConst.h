/*++

Copyright (c) 1995-1999 Microsoft Corporation

Module Name:

    ipxconst.h

Abstract:

    This module contains the common constants and macros used
    by the IPX Routing Protocols

Author:

    Stefan Solomon  07/10/1995

Revision History:


--*/

#ifndef _IPXCONST_
#define _IPXCONST_

#if _MSC_VER > 1000
#pragma once
#endif

#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

//
// Administrative States Definitions
//
// Note: these states correspond to the MIB admin states of ENABLED and DISABLED

#define ADMIN_STATE_DISABLED		    1
#define ADMIN_STATE_ENABLED		    2

// additional admin states used for netbios delivery.
//
// Note: these states correspond to the MIB states of:
//
// ENABLED_ONLY_FOR_NETBIOS_STATIC_ROUTING and
// ENABLED_ONLY_FOR_OPER_STATE_UP

#define ADMIN_STATE_ENABLED_ONLY_FOR_NETBIOS_STATIC_ROUTING	3
#define ADMIN_STATE_ENABLED_ONLY_FOR_OPER_STATE_UP		4

//
// Interface Operational States Definitions
//
// Note 1: applies to the IPX, RIP and SAP operational states
// Note 2: these states correspond to the MIB operational states of
//	   UP, DOWN and SLEEPING
//

#define OPER_STATE_DOWN			   1 // not operational
#define OPER_STATE_UP			   2 // operational & can pass packets
#define OPER_STATE_SLEEPING		   3 // operational but has to connect to pass packets

//
// Additional operational states in starting/stopping the router
//

#define OPER_STATE_STARTING		    4
#define OPER_STATE_STOPPING		    5

//
// Definitions and default values for the RIP and SAP Interface Info
//

// UpdateMode definitions
//

#define IPX_STANDARD_UPDATE		1 // Periodic update, every UpdateInterval
#define IPX_NO_UPDATE			2 // No update, used for static routes config
#define IPX_AUTO_STATIC_UPDATE		3 // AutoStatic triggered update

// PacketType definitions
//

#define IPX_STANDARD_PACKET_TYPE		1
#define IPX_RELIABLE_DELIVERY_PACKET_TYPE	2

// Pace definitions

#define IPX_PACE_DEFVAL 	18 // This corresponds to a 55 ms interpacketgap

// UpdateInterval definitions

#define IPX_UPDATE_INTERVAL_DEFVAL  60

//*********************************************************
//							  *
//		 IPX Route Entry Definitions		  *
//							  *
//*********************************************************

//
// IPX route entry defs for RTM mapping
//

#define   R_Interface		       RR_InterfaceID
#define   R_Protocol		       RR_RoutingProtocol

#define   R_Network		       RR_Network.N_NetNumber
#define   R_TickCount		       RR_FamilySpecificData.FSD_TickCount
#define   R_HopCount		       RR_FamilySpecificData.FSD_HopCount
#define   R_NextHopMacAddress	       RR_NextHopAddress.NHA_Mac

#define   R_Flags		       RR_FamilySpecificData.FSD_Flags

//
// Some particular interface indices values
//

#define MAX_INTERFACE_INDEX		0xFFFFFFFE
#define GLOBAL_INTERFACE_INDEX		0xFFFFFFFF

//
// Flags definitions
//

#define GLOBAL_WAN_ROUTE		0x00000001
#define DO_NOT_ADVERTISE_ROUTE		0x00000002


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif
