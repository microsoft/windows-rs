/*
 *  RingtoneDeviceService.h 
 *
 *  Contains declarations for the Ringtone Device Service 
 *
 *  Copyright (c) Microsoft Corporation, All Rights Reserved.
 *
 */


#ifndef _RINGTONEDEVICESERVICE_H_
#define _RINGTONEDEVICESERVICE_H_

#include <DeviceServices.h>

#include <MessageDeviceService.h>


/*****************************************************************************
    Ringtone Service Info
******************************************************************************/

DEFINE_DEVSVCGUID(SERVICE_Ringtones,
     0xd0eace0e, 0x707d, 0x4106, 0x8d, 0x38, 0x4f, 0x56, 0x0e, 0x6a, 0x9f, 0x8e );

#define NAME_RingtonesSvc L"Ringtones"
#define TYPE_RingtonesSvc DEVSVCTYPE_DEFAULT


/*****************************************************************************
    Ringtone Service Properties
******************************************************************************/

DEFINE_DEVSVCGUID(NAMESPACE_RingtonesSvc,
     0x7d05d925, 0x32e6, 0x4790, 0x92, 0x05, 0x54, 0x76, 0x4b, 0xb3, 0xcb, 0x74 );


/*  PKEY_RingtonesSvc_DefaultRingtone
 *
 *  Indicates the objectID of the default ringtone for incoming calls
 *  
 *  Type: UInt32
 *  Form: ObjectID
 */

DEFINE_DEVSVCPROPKEY(PKEY_RingtonesSvc_DefaultRingtone,
     0x7d05d925, 0x32e6, 0x4790, 0x92, 0x05, 0x54, 0x76, 0x4b, 0xb3, 0xcb, 0x74 ,
     2 );

#define NAME_RingtonesSvc_DefaultRingtone L"DefaultRingtone"


/*****************************************************************************
    Ringtone Service Object Property Keys
******************************************************************************/

DEFINE_DEVSVCGUID(NAMESPACE_RingtonesObj,
     0x8d943c78, 0x2c7d, 0x4c74, 0x94, 0x5a, 0x42, 0xd8, 0x3c, 0xb5, 0x8b, 0x5a );


#endif /*_RINGTONEDEVICESERVICE_H_*/
