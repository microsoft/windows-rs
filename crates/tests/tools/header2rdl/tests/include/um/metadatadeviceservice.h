/*
 *  MetadataDeviceService.h 
 *
 *  Contains definitions of the Device Metadata Service 
 *
 *  Copyright (c) Microsoft Corporation, All Rights Reserved.
 *
 */


#ifndef _METADATADEVICESERVICE_H_
#define _METADATADEVICESERVICE_H_

#include <DeviceServices.h>


/*****************************************************************************
    Device Metadata Service Info
******************************************************************************/

DEFINE_DEVSVCGUID(SERVICE_DeviceMetadata,
     0x332ffe6a, 0xaf65, 0x41e1, 0xa0, 0xaf, 0xd3, 0xe2, 0x62, 0x7b, 0xdf, 0x54 );

#define NAME_DeviceMetadataSvc L"Metadata"
#define TYPE_DeviceMetadataSvc DEVSVCTYPE_DEFAULT


/*****************************************************************************
    Device Metadata Service Object Formats
******************************************************************************/

/*  FORMAT_DeviceMetadataCAB
 *  
 *  CAB object format
 *  
 */

DEFINE_DEVSVCGUID(FORMAT_DeviceMetadataCAB,
     0xe1809599, 0x4303, 0x4e3b, 0x92, 0x44, 0x99, 0xc6, 0x2c, 0x25, 0x45, 0x51 );

#define NAME_DeviceMetadataCAB L"DeviceMetadataCAB"



/*****************************************************************************
    Device Metadata Service Object Property Keys
******************************************************************************/

DEFINE_DEVSVCGUID(NAMESPACE_DeviceMetadataObj,
     0x68bb7eeb, 0x9eef, 0x45bd, 0x8d, 0xe6, 0x3b, 0x92, 0xa5, 0x7c, 0xae, 0x1e );


/*  DeviceMetadataObj.ContentID
 *
 *  Contains the GUID that uniquely identifies the object cab contents.
 *  
 *  Type: UInt128
 *  Form: None
 */

DEFINE_DEVSVCPROPKEY(PKEY_DeviceMetadataObj_ContentID,
     0x68bb7eeb, 0x9eef, 0x45bd, 0x8d, 0xe6, 0x3b, 0x92, 0xa5, 0x7c, 0xae, 0x1e ,
     3 );

#define NAME_DeviceMetadataObj_ContentID L"ContentID"


/*  DeviceMetadataObj.DefaultCAB
 *
 *  Indicates whether this object is the default cab. Each service shall have
 *  only one object marked as default.
 *  
 *  Type: UInt8 Boolean
 *  Form: Enum
 */

DEFINE_DEVSVCPROPKEY(PKEY_DeviceMetadataObj_DefaultCAB,
     0x68bb7eeb, 0x9eef, 0x45bd, 0x8d, 0xe6, 0x3b, 0x92, 0xa5, 0x7c, 0xae, 0x1e ,
     4 );

#define NAME_DeviceMetadataObj_DefaultCAB L"DefaultCAB"

#define ENUM_DeviceMetadataObj_DefaultCABFalse 0 
#define ENUM_DeviceMetadataObj_DefaultCABTrue 1 


#endif /*_METADATADEVICESERVICE_H_*/
