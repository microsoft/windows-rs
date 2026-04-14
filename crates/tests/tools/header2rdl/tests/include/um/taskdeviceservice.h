/*
 *  TaskDeviceService.h 
 *
 *  Contains declarations for the Task Device Service 
 *
 *  Copyright (c) Microsoft Corporation, All Rights Reserved.
 *
 */


#ifndef _TASKDEVICESERVICE_H_
#define _TASKDEVICESERVICE_H_

#include <DeviceServices.h>

#include <MessageDeviceService.h>

#include <SyncDeviceService.h>


/*****************************************************************************
    Task Service Info
******************************************************************************/

DEFINE_DEVSVCGUID(SERVICE_Tasks,
     0xBB340C54, 0xB5C6, 0x491D, 0x88, 0x27, 0x28, 0xD0, 0xE7, 0x63, 0x19, 0x03 );

#define NAME_TasksSvc L"Tasks"
#define TYPE_TasksSvc DEVSVCTYPE_DEFAULT


/*****************************************************************************
    Task Service Properties
******************************************************************************/


/*  PKEY_TasksSvc_SyncActiveOnly
 *
 *  Type: UInt8
 *  Form: None
 */

#define PKEY_TasksSvc_SyncActiveOnly PKEY_SyncSvc_FilterType
#define NAME_TasksSvc_SyncActiveOnly NAME_SyncSvc_FilterType

/*****************************************************************************
    Task Service Object Formats
******************************************************************************/

/*  FORMAT_AbstractTask
 */

DEFINE_DEVSVCGUID(FORMAT_AbstractTask,
     0x522979c0, 0x74cf, 0x44ab, 0x97, 0x54, 0x55, 0xbc, 0x59, 0x6a, 0x67, 0xdf );

#define NAME_AbstractTask L"AbstractTask"



/*****************************************************************************
    Task Service Object Property Keys
******************************************************************************/

DEFINE_DEVSVCGUID(NAMESPACE_TaskObj,
     0xE354E95E, 0xD8A0, 0x4637, 0xA0, 0x3A, 0x0C, 0xB2, 0x68, 0x38, 0xDB, 0xC7 );


/*  TaskObj.ReminderDateTime
 *
 *  Type: String
 *  Form: DateTime
 */

DEFINE_DEVSVCPROPKEY(PKEY_TaskObj_ReminderDateTime,
     0xE354E95E, 0xD8A0, 0x4637, 0xA0, 0x3A, 0x0C, 0xB2, 0x68, 0x38, 0xDB, 0xC7 ,
     13 );

#define NAME_TaskObj_ReminderDateTime L"ReminderDateTime"


/*  TaskObj.Complete
 *
 *  Type: UInt8
 *  Form: Enum
 */

DEFINE_DEVSVCPROPKEY(PKEY_TaskObj_Complete,
     0xE354E95E, 0xD8A0, 0x4637, 0xA0, 0x3A, 0x0C, 0xB2, 0x68, 0x38, 0xDB, 0xC7 ,
     14 );

#define NAME_TaskObj_Complete L"Complete"

#define ENUM_TaskObj_CompleteFalse 0x00 
#define ENUM_TaskObj_CompleteTrue 0xff 


/*  TaskObj.BeginDate
 *
 *  Contains the date that the task should start- the date is assumed to
 *  be relative to the current device time zone
 *  
 *  Type: String
 *  Form: ISO 8601 Date
 */

DEFINE_DEVSVCPROPKEY(PKEY_TaskObj_BeginDate,
     0xE354E95E, 0xD8A0, 0x4637, 0xA0, 0x3A, 0x0C, 0xB2, 0x68, 0x38, 0xDB, 0xC7 ,
     15 );

#define NAME_TaskObj_BeginDate L"BeginDate"


/*  TaskObj.EndDate
 *
 *  Contains the date that the task should end- the date is assumed to be
 *  relative to the current device time zone
 *  
 *  Type: String
 *  Form: ISO 8601 Date
 */

DEFINE_DEVSVCPROPKEY(PKEY_TaskObj_EndDate,
     0xE354E95E, 0xD8A0, 0x4637, 0xA0, 0x3A, 0x0C, 0xB2, 0x68, 0x38, 0xDB, 0xC7 ,
     16 );

#define NAME_TaskObj_EndDate L"EndDate"


#endif /*_TASKDEVICESERVICE_H_*/
