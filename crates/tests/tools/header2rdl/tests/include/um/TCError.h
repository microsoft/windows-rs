/* Copyright (c) 1997-1999, Microsoft Corporation, all rights reserved
**
** tcerror.h
** Traffic Control external API
** TC specific error codes
*/

#ifndef _TCERROR_H_
#define _TCERROR_H_

#if _MSC_VER > 1000
#pragma once
#endif

#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

#define TCBASE 7500


//
// Incompatible TC version number
//
#define ERROR_INCOMPATIBLE_TCI_VERSION			(TCBASE+1)

//
// Unspecified or bad intserv service type
//
#define ERROR_INVALID_SERVICE_TYPE			(TCBASE+2)

//
// Unspecified or bad TokenRate
//
#define ERROR_INVALID_TOKEN_RATE			(TCBASE+3)
	
//
// Bad PeakBandwidth
//
#define ERROR_INVALID_PEAK_RATE				(TCBASE+4)
	
//
// Invalid ShapeDiscardMode
//
#define ERROR_INVALID_SD_MODE				(TCBASE+5)
	
//
// Invalid priority value
//
#define ERROR_INVALID_QOS_PRIORITY			(TCBASE+6)
	
//
// Invalid traffic class value
//
#define ERROR_INVALID_TRAFFIC_CLASS			(TCBASE+7)
	
//
// Invalid address type
//
#define ERROR_INVALID_ADDRESS_TYPE			(TCBASE+8)
	
//
// Attempt to install identical filter on same flow
//
#define ERROR_DUPLICATE_FILTER				(TCBASE+9)
	
//
// Attempt to install conflicting filter
//
#define ERROR_FILTER_CONFLICT				(TCBASE+10)
	
//
// This address type is not supported
//
#define ERROR_ADDRESS_TYPE_NOT_SUPPORTED	(TCBASE+11)

//
// This object can not be deleted since its suporting opened objects
//
#define ERROR_TC_SUPPORTED_OBJECTS_EXIST	(TCBASE+12)

//
// Incompatable QoS parameters
//
#define ERROR_INCOMPATABLE_QOS				(TCBASE+13)

//
// Traffic Control is not supported in the system
//
#define ERROR_TC_NOT_SUPPORTED				(TCBASE+14)

//
// TcObjectsLength is inconsistent with CfInfoSize
//
#define ERROR_TC_OBJECT_LENGTH_INVALID      (TCBASE+15)

//
// Adding an Intserv flow in Diffserv mode or vice versa
//
#define ERROR_INVALID_FLOW_MODE             (TCBASE+16)

//
// Invalid Diffserv flow
//
#define ERROR_INVALID_DIFFSERV_FLOW         (TCBASE+17)

//
// DS codepoint already exists
//
#define ERROR_DS_MAPPING_EXISTS             (TCBASE+18)

//
// Invalid Shape Rate specified
//
#define ERROR_INVALID_SHAPE_RATE            (TCBASE+19)

// 
// Invalid DCLASS
// 
#define ERROR_INVALID_DS_CLASS              (TCBASE+20)

// 
// Too many GPC clients
// 
#define ERROR_TOO_MANY_CLIENTS              (TCBASE+21)


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif // _TCERROR_H_
