/*
 *  DeviceServices.h 
 *
 *  Contains definitions for the core Device Services platform 
 *
 *  Copyright (c) Microsoft Corporation, All Rights Reserved.
 *
 */


#ifndef _DEVICESERVICES_H_
#define _DEVICESERVICES_H_

#include <BridgeDeviceService.h>


/*****************************************************************************
    Service Info
******************************************************************************/

/*  Service Info Version */
#define  DEVSVC_SERVICEINFO_VERSION      0x00000064

/*  Service Flags */
#define DEVSVCTYPE_DEFAULT              0x00000000
#define DEVSVCTYPE_ABSTRACT             0x00000001


/*****************************************************************************
    Common Service Properties
******************************************************************************/

DEFINE_DEVSVCGUID(NAMESPACE_Services,
     0x14fa7268, 0x0b6c, 0x4214, 0x94, 0x87, 0x43, 0x5b, 0x48, 0x0a, 0x8c, 0x4f );


/*  PKEY_Services_ServiceDisplayName
 *
 *  Type: String
 *  Form: None
 */

DEFINE_DEVSVCPROPKEY(PKEY_Services_ServiceDisplayName,
     0x14fa7268, 0x0b6c, 0x4214, 0x94, 0x87, 0x43, 0x5b, 0x48, 0x0a, 0x8c, 0x4f ,
     2 );

#define NAME_Services_ServiceDisplayName L"ServiceDisplayName"


/*  PKEY_Services_ServiceIcon
 *
 *  Type: AUInt8
 *  Form: ByteArray
 */

DEFINE_DEVSVCPROPKEY(PKEY_Services_ServiceIcon,
     0x14fa7268, 0x0b6c, 0x4214, 0x94, 0x87, 0x43, 0x5b, 0x48, 0x0a, 0x8c, 0x4f ,
     3 );

#define NAME_Services_ServiceIcon L"ServiceIcon"


/*  PKEY_Services_ServiceLocale
 *
 *  Contains the RFC4646 compliant language string for data in this service
 *  
 *  Type: String
 *  Form: None
 */

DEFINE_DEVSVCPROPKEY(PKEY_Services_ServiceLocale,
     0x14fa7268, 0x0b6c, 0x4214, 0x94, 0x87, 0x43, 0x5b, 0x48, 0x0a, 0x8c, 0x4f ,
     4 );

#define NAME_Services_ServiceLocale L"ServiceLocale"


#endif /*_DEVICESERVICES_H_*/
