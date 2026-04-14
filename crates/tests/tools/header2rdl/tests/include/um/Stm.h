/*++

Copyright (c) 1995-1999 Microsoft Corporation

Module Name:

    stm.h

Abstract:

    This module contains the definitions of the IPX Service Table Manger APIs

Author:


Revision History:


--*/

#ifndef __ROUTING_STM_H__
#define __ROUTING_STM_H__

#if _MSC_VER > 1000
#pragma once
#endif

#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

#if _MSC_VER >= 1200
#pragma warning(push)
#endif
#pragma warning(disable:4201)

//////////////////////////////////////////////////////////////////////////////
//                                                                          //
// Supported functionality flags                                            //
//                                                                          //
// ROUTING                  Imports Routing Table Manager APIs              //
// SERVICES                 Exports Service Table Manager APIs              //
// DEMAND_UPDATE_ROUTES     IP and IPX RIP support for Autostatic           //
// DEMAND_UPDATE_SERVICES   IPX SAP, NLSP support for Autostatic            //
//                                                                          //
//////////////////////////////////////////////////////////////////////////////

#define SERVICES                0x00000002
#define DEMAND_UPDATE_SERVICES  0x00000008

//
// Server Entry
//

typedef struct _IPX_SERVER_ENTRY
{
    USHORT	Type;
    UCHAR	Name[48];
    UCHAR	Network[4];
    UCHAR	Node[6];
    UCHAR	Socket[2];
    USHORT	HopCount;
} IPX_SERVER_ENTRY, *PIPX_SERVER_ENTRY;

typedef struct _IPX_SERVICE
{
    ULONG		        InterfaceIndex;
    ULONG	            Protocol;	// protocol from which knowledge of the service was obatined
    IPX_SERVER_ENTRY	Server;
} IPX_SERVICE, *PIPX_SERVICE;

// Function which returns TRUE if the service exists

typedef
BOOL
(WINAPI * PIS_SERVICE)(
      IN USHORT 	Type,
      IN PUCHAR 	Name,
      OUT PIPX_SERVICE	Service OPTIONAL
      );

// Exclusion flags.  Limit enumeration to only servers that
// have same values of the specified by flags parameter(s) as those of
// criterea service.

#define STM_ONLY_THIS_INTERFACE     0x00000001
#define STM_ONLY_THIS_PROTOCOL	    0x00000002
#define STM_ONLY_THIS_TYPE	        0x00000004
#define STM_ONLY_THIS_NAME	        0x00000008

// Ordering methods. Specify the order in which services should be
// retreived (methods are mutually exclusive).

#define STM_ORDER_BY_TYPE_AND_NAME		    0
#define STM_ORDER_BY_INTERFACE_TYPE_NAME	1


// Create handle to start enumeration of the services in the STM table.
// Returns handle to be used for enumerations or NULL if operation failed
//	GetLastError () returns the follwing error codes in case of failure:
//		ERROR_CAN_NOT_COMPLETE
//		ERROR_NOT_ENOUGH_MEMORY

typedef
HANDLE
(WINAPI * PCREATE_SERVICE_ENUMERATION_HANDLE)(
    IN  DWORD           ExclusionFlags, // Flags to limit enumeration to certain
                                        // types of servers
    IN	PIPX_SERVICE  CriteriaService	// Criteria for exclusion flags
    );

// Get next service in the enumeration started by CreateServiceEnumerationHandle
// Returns NO_ERROR if next service was placed in provided buffer or
// ERROR_NO_MORE_ITEMS when there are no more services to be
// returned in the enumeration; ERROR_CAN_NOT_COMPLETE will be
// returned if operation failed.

typedef
DWORD
(WINAPI * PENUMERATE_GET_NEXT_SERVICE)(
    IN  HANDLE          EnumerationHandle, // Handle that identifies this
                                           // enumeration
    OUT PIPX_SERVICE  Service		    // buffer to place parameters of next service entry
										// to be returned by enumeration
    );

// Frees resources associated with enumeration.
// Returns NO_ERROR if operation succeded, ERROR_CAN_NOT_COMPLETE
// otherwise

typedef
DWORD
(WINAPI * PCLOSE_SERVICE_ENUMERATION_HANDLE)(
    IN	HANDLE	       EnumerationHandle
    );

// Get total number of known services

typedef
ULONG
(WINAPI * PGET_SERVICE_COUNT)(
	VOID
	);

//	Add service of IPX_PROTOCOL_STATIC to the table

typedef
DWORD
(WINAPI * PCREATE_STATIC_SERVICE)(IN ULONG		InterfaceIndex,
		       IN PIPX_SERVER_ENTRY		ServerEntry);

//	Delete service of IPX_PROTOCOL_STATIC from the table

typedef
DWORD
(WINAPI * PDELETE_STATIC_SERVICE)(IN ULONG		InterfaceIndex,
		       IN PIPX_SERVER_ENTRY		ServerEntry);


//	Converts protocol of all services associated with given interface to
//	IPX_PROTOCOL_STATIC

typedef
DWORD
(WINAPI * PBLOCK_CONVERT_SERVICES_TO_STATIC) (
	IN ULONG		InterfaceIndex
	);

//	Delete all services of IPX_PROTOCOL_STATIC
//	associated with  given interface from the table

typedef
DWORD
(WINAPI * PBLOCK_DELETE_STATIC_SERVICES)(
	IN ULONG		InterfaceIndex
	);


// Find and return first service in the order specified by the ordering method.
// Search is limited only to ceratin types of services as specified by the
// exclusion flags end corresponding fields in Service parameter.
// Returns ERROR_NO_MORE_ITEMS if there are no services in the
// table that meet specified criteria.

typedef
DWORD
(WINAPI * PGET_FIRST_ORDERED_SERVICE)(
    IN  DWORD           OrderingMethod,     // What ordering to use
    IN  DWORD           ExclusionFlags,     // Flags to limit search to ceratin
                                            // types of servers
    IN OUT PIPX_SERVICE Service 	    // On input: criteria for exclusion
                                            //          flags
                                            // On output: first service entry
                                            //          in the specified order
    );

// Find and return next service in the order specified by the ordering method.
// Search starts from specified service and is limited only to ceratin types
// of services as specified by the exclusion flags and corresponding fields
// in Service parameter.
// Returns ERROR_NO_MORE_ITEMS if there are no services in table
// table that meet specified criteria.

typedef
DWORD
(WINAPI * PGET_NEXT_ORDERED_SERVICE)(
    IN  DWORD           OrderingMethod,     // What ordering to use
    IN  DWORD           ExclusionFlags,     // Flags to limit search to ceratin
                                            // types of servers
    IN OUT PIPX_SERVICE Service 	    // On input: service to start the
                                            //          search from and
                                            //          criteria for exclusion
                                            //          flags
                                            // On output: next service entry
                                            //          in the specified order
    );

typedef
DWORD
(WINAPI * PDO_UPDATE_SERVICES) (
    IN ULONG    InterfaceIndex
    );

typedef
BOOL
(WINAPI * PGET_SERVICE_ID)(
      IN USHORT 	Type,
      IN PUCHAR 	Name,
      OUT PULONG	ServiceID
      );

typedef
BOOL
(WINAPI * PGET_SERVICE_FROM_ID)(
      IN ULONG	        ServiceID,
      OUT PIPX_SERVICE  Service
      );

typedef
DWORD
(WINAPI * PGET_NEXT_SERVICE_FROM_ID)(
      IN ULONG	        ServiceID,
      OUT PIPX_SERVICE  NextService,
      OUT PULONG        NextServiceID
      );

typedef struct _MPR40_SERVICE_CHARACTERISTICS
{
    DWORD                               dwVersion;
    DWORD                               dwProtocolId;
    DWORD                               fSupportedFunctionality;
    PIS_SERVICE                         pfnIsService;
    PDO_UPDATE_SERVICES                 pfnUpdateServices;
    PCREATE_SERVICE_ENUMERATION_HANDLE  pfnCreateServiceEnumerationHandle;
    PENUMERATE_GET_NEXT_SERVICE         pfnEnumerateGetNextService;
    PCLOSE_SERVICE_ENUMERATION_HANDLE   pfnCloseServiceEnumerationHandle;
    PGET_SERVICE_COUNT                  pfnGetServiceCount;
    PCREATE_STATIC_SERVICE              pfnCreateStaticService;
    PDELETE_STATIC_SERVICE              pfnDeleteStaticService;
    PBLOCK_CONVERT_SERVICES_TO_STATIC   pfnBlockConvertServicesToStatic;
    PBLOCK_DELETE_STATIC_SERVICES       pfnBlockDeleteStaticServices;
    PGET_FIRST_ORDERED_SERVICE          pfnGetFirstOrderedService;
    PGET_NEXT_ORDERED_SERVICE           pfnGetNextOrderedService;
}MPR40_SERVICE_CHARACTERISTICS;

typedef struct _MPR50_SERVICE_CHARACTERISTICS
{

#ifdef __cplusplus
    MPR40_SERVICE_CHARACTERISTICS       mscMpr40ServiceChars;
#else
    MPR40_SERVICE_CHARACTERISTICS;
#endif

}MPR50_SERVICE_CHARACTERISTICS;

#if MPR60
    typedef MPR50_SERVICE_CHARACTERISTICS MPR_SERVICE_CHARACTERISTICS;
#elif MPR50
    typedef MPR50_SERVICE_CHARACTERISTICS MPR_SERVICE_CHARACTERISTICS;
#elif MPR40
    typedef MPR40_SERVICE_CHARACTERISTICS MPR_SERVICE_CHARACTERISTICS;
#endif

typedef MPR_SERVICE_CHARACTERISTICS *PMPR_SERVICE_CHARACTERISTICS;

#if _MSC_VER >= 1200
#pragma warning(pop)
#else
#pragma warning(default:4201)
#endif

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif

