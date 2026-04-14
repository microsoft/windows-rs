/*
 *  CalendarDeviceService.h 
 *
 *  Contains declarations for the Calendar Device Service 
 *
 *  Copyright (c) Microsoft Corporation, All Rights Reserved.
 *
 */


#ifndef _CALENDARDEVICESERVICE_H_
#define _CALENDARDEVICESERVICE_H_

#include <DeviceServices.h>

#include <MessageDeviceService.h>

#include <SyncDeviceService.h>


/*****************************************************************************
    Calendar Service Info
******************************************************************************/

DEFINE_DEVSVCGUID(SERVICE_Calendar,
     0xE4DFDBD3, 0x7F04, 0x45E9, 0x9F, 0xA1, 0x5C, 0xA0, 0xEA, 0xEB, 0x0A, 0xE3 );

#define NAME_CalendarSvc L"Calendar"
#define TYPE_CalendarSvc DEVSVCTYPE_DEFAULT


/*****************************************************************************
    Calendar Service Properties
******************************************************************************/

DEFINE_DEVSVCGUID(NAMESPACE_CalendarSvc,
     0x63816297, 0x61E5, 0x4306, 0xB1, 0xA3, 0xCE, 0xDF, 0x48, 0x1B, 0x86, 0x29 );


/*  PKEY_CalendarSvc_SyncInWindowOnly
 *
 *  Type: UInt8
 *  Form: None
 */

#define PKEY_CalendarSvc_SyncInWindowOnly PKEY_SyncSvc_FilterType
#define NAME_CalendarSvc_SyncInWindowOnly NAME_SyncSvc_FilterType

/*  PKEY_CalendarSvc_SyncWindowStart
 *
 *  Indicates the number of minutes before TODAY that the sync window starts
 *  
 *  Type: UInt32
 *  Form: None
 */

DEFINE_DEVSVCPROPKEY(PKEY_CalendarSvc_SyncWindowStart,
     0x63816297, 0x61E5, 0x4306, 0xB1, 0xA3, 0xCE, 0xDF, 0x48, 0x1B, 0x86, 0x29 ,
     2 );

#define NAME_CalendarSvc_SyncWindowStart L"SyncWindowStart"


/*  PKEY_CalendarSvc_SyncWindowEnd
 *
 *  Indicates the number of minutes after TODAY that the sync window ends
 *  
 *  Type: UInt32
 *  Form: None
 */

DEFINE_DEVSVCPROPKEY(PKEY_CalendarSvc_SyncWindowEnd,
     0x63816297, 0x61E5, 0x4306, 0xB1, 0xA3, 0xCE, 0xDF, 0x48, 0x1B, 0x86, 0x29 ,
     3 );

#define NAME_CalendarSvc_SyncWindowEnd L"SyncWindowEnd"


/*****************************************************************************
    Calendar Service Object Formats
******************************************************************************/

/*  FORMAT_AbstractActivity
 */

DEFINE_DEVSVCGUID(FORMAT_AbstractActivity,
     0xbf70e114, 0x3901, 0x4449, 0xbe, 0xe7, 0xd9, 0xea, 0x14, 0x93, 0xc3, 0x09 );

#define NAME_AbstractActivity L"AbstractActivity"


/*  FORMAT_AbstractActivityOccurrence
 */

DEFINE_DEVSVCGUID(FORMAT_AbstractActivityOccurrence,
     0xE87A7008, 0x32D1, 0x42C5, 0x84, 0x88, 0x4C, 0x23, 0x58, 0x66, 0xAF, 0x32 );

#define NAME_AbstractActivityOccurrence L"AbstractActivityOccurrence"


/*  FORMAT_VCalendar1Activity
 */

DEFINE_DEVSVCGUID(FORMAT_VCalendar1Activity,
     0x23F7A5A5, 0xF7D3, 0x4585, 0xA1, 0xFF, 0x76, 0xE2, 0xD4, 0x5C, 0x91, 0x21 );

#define NAME_VCalendar1Activity L"VCalendar1"


/*  FORMAT_ICalendarActivity
 *  
 *  iCalendar file format (vCalendar Version 2)
 *  
 */

DEFINE_DEVSVCGUID(FORMAT_ICalendarActivity,
     0xCC4538CB, 0x7890, 0x41B7, 0xA3, 0xF1, 0xB6, 0xE6, 0x0B, 0xDD, 0x2A, 0x61 );

#define NAME_ICalendarActivity L"ICalendar"



/*****************************************************************************
    Calendar Service Object Property Keys
******************************************************************************/

DEFINE_DEVSVCGUID(NAMESPACE_CalendarObj,
     0xF99EFD03, 0x431D, 0x40D8, 0xA1, 0xC9, 0x4E, 0x22, 0x0D, 0x9C, 0x88, 0xD3 );


/*  CalendarObj.Location
 *
 *  MTP Property: Activity Location (0xDD52)
 *  Type: String/AUInt16
 *  Form: None/LongString
 */

DEFINE_DEVSVCPROPKEY(PKEY_CalendarObj_Location,
     0xF99EFD03, 0x431D, 0x40D8, 0xA1, 0xC9, 0x4E, 0x22, 0x0D, 0x9C, 0x88, 0xD3 ,
     3 );

#define NAME_CalendarObj_Location L"Location"


/*  CalendarObj.Accepted
 *
 *  MTP Property: Activity Accepted (0xDD57)
 *  Type: AUInt16
 *  Form: LongString
 */

DEFINE_DEVSVCPROPKEY(PKEY_CalendarObj_Accepted,
     0xF99EFD03, 0x431D, 0x40D8, 0xA1, 0xC9, 0x4E, 0x22, 0x0D, 0x9C, 0x88, 0xD3 ,
     10 );

#define NAME_CalendarObj_Accepted L"Accepted"


/*  CalendarObj.Tentative
 *
 *  MTP Property: Activity Tentative (0xDD58)
 *  Type: AUInt16
 *  Form: LongString
 */

DEFINE_DEVSVCPROPKEY(PKEY_CalendarObj_Tentative,
     0xF99EFD03, 0x431D, 0x40D8, 0xA1, 0xC9, 0x4E, 0x22, 0x0D, 0x9C, 0x88, 0xD3 ,
     12 );

#define NAME_CalendarObj_Tentative L"Tentative"


/*  CalendarObj.Declined
 *
 *  MTP Property: Activity Declined (0xDD59)
 *  Type: AUInt16
 *  Form: LongString
 */

DEFINE_DEVSVCPROPKEY(PKEY_CalendarObj_Declined,
     0xF99EFD03, 0x431D, 0x40D8, 0xA1, 0xC9, 0x4E, 0x22, 0x0D, 0x9C, 0x88, 0xD3 ,
     13 );

#define NAME_CalendarObj_Declined L"Declined"


/*  CalendarObj.TimeZone
 *
 *  Contains the TZ Database name for the time zone in which the appointment
 *  was created.
 *  
 *  Type: String
 *  Form: None
 */

DEFINE_DEVSVCPROPKEY(PKEY_CalendarObj_TimeZone,
     0xF99EFD03, 0x431D, 0x40D8, 0xA1, 0xC9, 0x4E, 0x22, 0x0D, 0x9C, 0x88, 0xD3 ,
     14 );

#define NAME_CalendarObj_TimeZone L"TimeZone"


/*  CalendarObj.ReminderOffset
 *
 *  Contains the offset in minutes from the start of the appointment that
 *  a reminder is to be fired.
 *  
 *  Type: UInt32
 *  Form: None
 */

DEFINE_DEVSVCPROPKEY(PKEY_CalendarObj_ReminderOffset,
     0xF99EFD03, 0x431D, 0x40D8, 0xA1, 0xC9, 0x4E, 0x22, 0x0D, 0x9C, 0x88, 0xD3 ,
     15 );

#define NAME_CalendarObj_ReminderOffset L"ReminderOffset"


/*  CalendarObj.BusyStatus
 *
 *  Contains the free/busy status for the specified appointment.
 *  
 *  Type: UInt16
 *  Form: Enum
 */

DEFINE_DEVSVCPROPKEY(PKEY_CalendarObj_BusyStatus,
     0xF99EFD03, 0x431D, 0x40D8, 0xA1, 0xC9, 0x4E, 0x22, 0x0D, 0x9C, 0x88, 0xD3 ,
     16 );

#define NAME_CalendarObj_BusyStatus L"BusyStatus"

#define ENUM_CalendarObj_BusyStatusFree 0x0000 
#define ENUM_CalendarObj_BusyStatusBusy 0x0001 
#define ENUM_CalendarObj_BusyStatusOutOfOffice 0x0002 
#define ENUM_CalendarObj_BusyStatusTentative 0x0003 


/*  CalendarObj.PatternStartTime
 *
 *  Contains the time of day at which a recurring item is to start. The
 *  format is the time portion of an ISO 8601 DateTime value- e.g. HHMMSS.S
 *  
 *  Type: String
 *  Form: ISO 8601 Time
 */

DEFINE_DEVSVCPROPKEY(PKEY_CalendarObj_PatternStartTime,
     0xF99EFD03, 0x431D, 0x40D8, 0xA1, 0xC9, 0x4E, 0x22, 0x0D, 0x9C, 0x88, 0xD3 ,
     17 );

#define NAME_CalendarObj_PatternStartTime L"PatternStartTime"


/*  CalendarObj.PatternDuration
 *
 *  Contains the duration of the recurring item in minutes.
 *  
 *  Type: UInt32
 *  Form: None
 */

DEFINE_DEVSVCPROPKEY(PKEY_CalendarObj_PatternDuration,
     0xF99EFD03, 0x431D, 0x40D8, 0xA1, 0xC9, 0x4E, 0x22, 0x0D, 0x9C, 0x88, 0xD3 ,
     18 );

#define NAME_CalendarObj_PatternDuration L"PatternDuration"


/*  CalendarObj.BeginDateTime
 *
 *  Contains the UTC date and time that the calendar item begins
 *  
 *  Type: String
 *  Form: DateTime
 */

DEFINE_DEVSVCPROPKEY(PKEY_CalendarObj_BeginDateTime,
     0xF99EFD03, 0x431D, 0x40D8, 0xA1, 0xC9, 0x4E, 0x22, 0x0D, 0x9C, 0x88, 0xD3 ,
     19 );

#define NAME_CalendarObj_BeginDateTime L"BeginDateTime"


/*  CalendarObj.EndDateTime
 *
 *  Contains the UTC date and time that the calendar item ends
 *  
 *  Type: String
 *  Form: DateTime
 */

DEFINE_DEVSVCPROPKEY(PKEY_CalendarObj_EndDateTime,
     0xF99EFD03, 0x431D, 0x40D8, 0xA1, 0xC9, 0x4E, 0x22, 0x0D, 0x9C, 0x88, 0xD3 ,
     20 );

#define NAME_CalendarObj_EndDateTime L"EndDateTime"


#endif /*_CALENDARDEVICESERVICE_H_*/
