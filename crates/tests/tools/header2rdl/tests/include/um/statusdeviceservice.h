/*
 *  StatusDeviceService.h 
 *
 *  Contains definitions of the Status Device Service 
 *
 *  Copyright (c) Microsoft Corporation, All Rights Reserved.
 *
 */


#ifndef _STATUSDEVICESERVICE_H_
#define _STATUSDEVICESERVICE_H_

#include <DeviceServices.h>


/*****************************************************************************
    Status Service Info
******************************************************************************/

DEFINE_DEVSVCGUID(SERVICE_Status,
     0x0B9F1048, 0xB94B, 0xDC9A, 0x4e, 0xd7, 0xfe, 0x4f, 0xed, 0x3a, 0x0d, 0xeb );

#define NAME_StatusSvc L"Status"
#define TYPE_StatusSvc DEVSVCTYPE_DEFAULT


/*****************************************************************************
    Status Service Properties
******************************************************************************/

DEFINE_DEVSVCGUID(NAMESPACE_StatusSvc,
     0x49cd1f76, 0x5626, 0x4b17, 0xa4, 0xe8, 0x18, 0xb4, 0xaa, 0x1a, 0x22, 0x13 );


/*  PKEY_StatusSvc_SignalStrength
 *
 *  Signal strength in "bars" from 0 (no signal) to 4 (excellent signal)
 *  
 *  Type: UInt8
 *  Form: Range
 */

DEFINE_DEVSVCPROPKEY(PKEY_StatusSvc_SignalStrength,
     0x49cd1f76, 0x5626, 0x4b17, 0xa4, 0xe8, 0x18, 0xb4, 0xaa, 0x1a, 0x22, 0x13 ,
     2 );

#define NAME_StatusSvc_SignalStrength L"SignalStrength"

#define RANGEMIN_StatusSvc_SignalStrength 0 
#define RANGEMAX_StatusSvc_SignalStrength 4 
#define RANGESTEP_StatusSvc_SignalStrength 1 


/*  PKEY_StatusSvc_TextMessages
 *
 *  Number of unread text messages (255 max)
 *  
 *  Type: UInt8
 *  Form: None
 */

DEFINE_DEVSVCPROPKEY(PKEY_StatusSvc_TextMessages,
     0x49cd1f76, 0x5626, 0x4b17, 0xa4, 0xe8, 0x18, 0xb4, 0xaa, 0x1a, 0x22, 0x13 ,
     3 );

#define NAME_StatusSvc_TextMessages L"TextMessages"

#define RANGEMAX_StatusSvc_TextMessages 255 


/*  PKEY_StatusSvc_NewPictures
 *
 *  Number of "new" pictures on the device (65535 max)
 *  
 *  Type: UInt16
 *  Form: None
 */

DEFINE_DEVSVCPROPKEY(PKEY_StatusSvc_NewPictures,
     0x49cd1f76, 0x5626, 0x4b17, 0xa4, 0xe8, 0x18, 0xb4, 0xaa, 0x1a, 0x22, 0x13 ,
     4 );

#define NAME_StatusSvc_NewPictures L"NewPictures"

#define RANGEMAX_StatusSvc_NewPictures 65535 


/*  PKEY_StatusSvc_MissedCalls
 *
 *  Number of missed calls on the device (255 max)
 *  
 *  Type: UInt8
 *  Form: None
 */

DEFINE_DEVSVCPROPKEY(PKEY_StatusSvc_MissedCalls,
     0x49cd1f76, 0x5626, 0x4b17, 0xa4, 0xe8, 0x18, 0xb4, 0xaa, 0x1a, 0x22, 0x13 ,
     5 );

#define NAME_StatusSvc_MissedCalls L"MissedCalls"

#define RANGEMAX_StatusSvc_MissedCalls 255 


/*  PKEY_StatusSvc_VoiceMail
 *
 *  Number of "available" voice mail messages (255 max)
 *  
 *  Type: UInt8
 *  Form: None
 */

DEFINE_DEVSVCPROPKEY(PKEY_StatusSvc_VoiceMail,
     0x49cd1f76, 0x5626, 0x4b17, 0xa4, 0xe8, 0x18, 0xb4, 0xaa, 0x1a, 0x22, 0x13 ,
     6 );

#define NAME_StatusSvc_VoiceMail L"VoiceMail"

#define RANGEMAX_StatusSvc_VoiceMail 255 


/*  PKEY_StatusSvc_NetworkName
 *
 *  Network provider network name
 *  
 *  Type: String
 *  Form: None
 */

DEFINE_DEVSVCPROPKEY(PKEY_StatusSvc_NetworkName,
     0x49cd1f76, 0x5626, 0x4b17, 0xa4, 0xe8, 0x18, 0xb4, 0xaa, 0x1a, 0x22, 0x13 ,
     7 );

#define NAME_StatusSvc_NetworkName L"NetworkName"


/*  PKEY_StatusSvc_NetworkType
 *
 *  Network "type" (e.g. GPRS, EDGE, UMTS, 1xRTT, EVDO, or operator branded)
 *  
 *  Type: String
 *  Form: None
 */

DEFINE_DEVSVCPROPKEY(PKEY_StatusSvc_NetworkType,
     0x49cd1f76, 0x5626, 0x4b17, 0xa4, 0xe8, 0x18, 0xb4, 0xaa, 0x1a, 0x22, 0x13 ,
     8 );

#define NAME_StatusSvc_NetworkType L"NetworkType"


/*  PKEY_StatusSvc_Roaming
 *
 *  Current network roaming state
 *  
 *  Type: UInt8
 *  Form: Enum
 */

DEFINE_DEVSVCPROPKEY(PKEY_StatusSvc_Roaming,
     0x49cd1f76, 0x5626, 0x4b17, 0xa4, 0xe8, 0x18, 0xb4, 0xaa, 0x1a, 0x22, 0x13 ,
     9 );

#define NAME_StatusSvc_Roaming L"Roaming"

#define ENUM_StatusSvc_RoamingInactive 0x00 
#define ENUM_StatusSvc_RoamingActive 0x01 
#define ENUM_StatusSvc_RoamingUnknown 0x02 


/*  PKEY_StatusSvc_BatteryLife
 *
 *  Remaining battery life on the device as a percentage between 100 and 0.
 *  
 *  Type: UInt8
 *  Form: Range
 */

DEFINE_DEVSVCPROPKEY(PKEY_StatusSvc_BatteryLife,
     0x49cd1f76, 0x5626, 0x4b17, 0xa4, 0xe8, 0x18, 0xb4, 0xaa, 0x1a, 0x22, 0x13 ,
     10 );

#define NAME_StatusSvc_BatteryLife L"BatteryLife"

#define RANGEMIN_StatusSvc_BatteryLife 0 
#define RANGEMAX_StatusSvc_BatteryLife 100 
#define RANGESTEP_StatusSvc_BatteryLife 1 


/*  PKEY_StatusSvc_ChargingState
 *
 *  Current charging state of the device
 *  
 *  Type: UInt8
 *  Form: Enum
 */

DEFINE_DEVSVCPROPKEY(PKEY_StatusSvc_ChargingState,
     0x49cd1f76, 0x5626, 0x4b17, 0xa4, 0xe8, 0x18, 0xb4, 0xaa, 0x1a, 0x22, 0x13 ,
     11 );

#define NAME_StatusSvc_ChargingState L"ChargingState"

#define ENUM_StatusSvc_ChargingInactive 0x00 
#define ENUM_StatusSvc_ChargingActive 0x01 
#define ENUM_StatusSvc_ChargingUnknown 0x02 


/*  PKEY_StatusSvc_StorageCapacity
 *
 *  Total storage capacity on the device (across all storages)
 *  
 *  Type: UInt64
 *  Form: None
 */

DEFINE_DEVSVCPROPKEY(PKEY_StatusSvc_StorageCapacity,
     0x49cd1f76, 0x5626, 0x4b17, 0xa4, 0xe8, 0x18, 0xb4, 0xaa, 0x1a, 0x22, 0x13 ,
     12 );

#define NAME_StatusSvc_StorageCapacity L"StorageCapacity"


/*  PKEY_StatusSvc_StorageFreeSpace
 *
 *  Total free storage capacity on the device (across all storages)
 *  
 *  Type: UInt64
 *  Form: None
 */

DEFINE_DEVSVCPROPKEY(PKEY_StatusSvc_StorageFreeSpace,
     0x49cd1f76, 0x5626, 0x4b17, 0xa4, 0xe8, 0x18, 0xb4, 0xaa, 0x1a, 0x22, 0x13 ,
     13 );

#define NAME_StatusSvc_StorageFreeSpace L"StorageFreeSpace"


#endif /*_STATUSDEVICESERVICE_H_*/
