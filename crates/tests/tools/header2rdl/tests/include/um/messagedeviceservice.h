/*
 *  MessageDeviceService.h 
 *
 *  Contains declarations for the Message Device Service 
 *
 *  Copyright (c) Microsoft Corporation, All Rights Reserved.
 *
 */


#ifndef _MESSAGEDEVICESERVICE_H_
#define _MESSAGEDEVICESERVICE_H_

#include <DeviceServices.h>


/*****************************************************************************
    Message Service Info
******************************************************************************/

DEFINE_DEVSVCGUID(SERVICE_Message,
     0x29b43bd0, 0x6b16, 0x49af, 0xb7, 0x2e, 0x85, 0x77, 0x0a, 0xdf, 0xeb, 0xdd );

#define NAME_MessageSvc L"Message"
#define TYPE_MessageSvc DEVSVCTYPE_DEFAULT


/*****************************************************************************
    Message Service Object Formats
******************************************************************************/

/*  FORMAT_AbstractMessage
 *
 *  MTP Format: Abstract Message (0xBB01)
 *  Suggested MIME Type:
 */

DEFINE_DEVSVCGUID(FORMAT_AbstractMessage,
     0xBB010000, 0xAE6C, 0x4804, 0x98, 0xBA, 0xC5, 0x7B, 0x46, 0x96, 0x5F, 0xE7 );

#define NAME_AbstractMessage L"AbstractMessage"


/*  FORMAT_AbstractMessageFolder
 *
 *  MTP Format: Abstract Message Folder (0xBA07)
 *  Suggested MIME Type:
 */

DEFINE_DEVSVCGUID(FORMAT_AbstractMessageFolder,
     0xBA070000, 0xAE6C, 0x4804, 0x98, 0xBA, 0xC5, 0x7B, 0x46, 0x96, 0x5F, 0xE7 );

#define NAME_AbstractMessageFolder L"AbstractMessageFolder"



/*****************************************************************************
    Message Service Object Property Keys
******************************************************************************/

DEFINE_DEVSVCGUID(NAMESPACE_MessageObj,
     0xB28AE94B, 0x05A4, 0x4E8E, 0xBE, 0x01, 0x72, 0xCC, 0x7E, 0x09, 0x9D, 0x8F );


/*  MessageObj.Subject
 *
 *  MTP Property: Subject (0xDCE2)
 *  Type: String/AUInt16
 *  Form: None/LongString
 */

DEFINE_DEVSVCPROPKEY(PKEY_MessageObj_Subject,
     0xB28AE94B, 0x05A4, 0x4E8E, 0xBE, 0x01, 0x72, 0xCC, 0x7E, 0x09, 0x9D, 0x8F ,
     2 );

#define NAME_MessageObj_Subject L"Subject"


/*  MessageObj.Body
 *
 *  MTP Property: Body Text (0xDCE1)
 *  Type: AUInt16
 *  Form: LongString
 */

DEFINE_DEVSVCPROPKEY(PKEY_MessageObj_Body,
     0xB28AE94B, 0x05A4, 0x4E8E, 0xBE, 0x01, 0x72, 0xCC, 0x7E, 0x09, 0x9D, 0x8F ,
     3 );

#define NAME_MessageObj_Body L"Body"


/*  MessageObj.Priority
 *
 *  Type: UInt16
 *  Form: Enum
 */

DEFINE_DEVSVCPROPKEY(PKEY_MessageObj_Priority,
     0xB28AE94B, 0x05A4, 0x4E8E, 0xBE, 0x01, 0x72, 0xCC, 0x7E, 0x09, 0x9D, 0x8F ,
     4 );

#define NAME_MessageObj_Priority L"Priority"

#define ENUM_MessageObj_PriorityHighest 2 
#define ENUM_MessageObj_PriorityNormal 1 
#define ENUM_MessageObj_PriorityLowest 0 


/*  MessageObj.Category
 *
 *  Type: String/AUInt16
 *  Form: None/LongString
 */

DEFINE_DEVSVCPROPKEY(PKEY_MessageObj_Category,
     0xB28AE94B, 0x05A4, 0x4E8E, 0xBE, 0x01, 0x72, 0xCC, 0x7E, 0x09, 0x9D, 0x8F ,
     10 );

#define NAME_MessageObj_Category L"Category"


/*  MessageObj.Sender
 *
 *  MTP Property: Message Sender (0xDD45)
 *  Type: String/AUInt16
 *  Form: None/LongString
 */

DEFINE_DEVSVCPROPKEY(PKEY_MessageObj_Sender,
     0xB28AE94B, 0x05A4, 0x4E8E, 0xBE, 0x01, 0x72, 0xCC, 0x7E, 0x09, 0x9D, 0x8F ,
     18 );

#define NAME_MessageObj_Sender L"Sender"


/*  MessageObj.To
 *
 *  MTP Property: Message To (0xDD40)
 *  Type: AUInt16
 *  Form: LongString
 */

DEFINE_DEVSVCPROPKEY(PKEY_MessageObj_To,
     0xB28AE94B, 0x05A4, 0x4E8E, 0xBE, 0x01, 0x72, 0xCC, 0x7E, 0x09, 0x9D, 0x8F ,
     20 );

#define NAME_MessageObj_To L"To"


/*  MessageObj.CC
 *
 *  MTP Property: Message CC (0xDD41)
 *  Type: AUInt16
 *  Form: LongString
 */

DEFINE_DEVSVCPROPKEY(PKEY_MessageObj_CC,
     0xB28AE94B, 0x05A4, 0x4E8E, 0xBE, 0x01, 0x72, 0xCC, 0x7E, 0x09, 0x9D, 0x8F ,
     21 );

#define NAME_MessageObj_CC L"CC"


/*  MessageObj.BCC
 *
 *  MTP Property: Message BCC (0xDD42)
 *  Type: AUInt16
 *  Form: LongString
 */

DEFINE_DEVSVCPROPKEY(PKEY_MessageObj_BCC,
     0xB28AE94B, 0x05A4, 0x4E8E, 0xBE, 0x01, 0x72, 0xCC, 0x7E, 0x09, 0x9D, 0x8F ,
     22 );

#define NAME_MessageObj_BCC L"BCC"


/*  MessageObj.Read
 *
 *  MTP Property: Message Read (0xDD43)
 *  Type: UInt16
 *  Form: Enum
 */

DEFINE_DEVSVCPROPKEY(PKEY_MessageObj_Read,
     0xB28AE94B, 0x05A4, 0x4E8E, 0xBE, 0x01, 0x72, 0xCC, 0x7E, 0x09, 0x9D, 0x8F ,
     23 );

#define NAME_MessageObj_Read L"Read"

#define ENUM_MessageObj_ReadFalse 0x00 
#define ENUM_MessageObj_ReadTrue 0xff 


/*  MessageObj.ReceivedTime
 *
 *  MTP Property: Message Received Time (0xDD44)
 *  Type: String
 *  Form: DateTime
 */

DEFINE_DEVSVCPROPKEY(PKEY_MessageObj_ReceivedTime,
     0xB28AE94B, 0x05A4, 0x4E8E, 0xBE, 0x01, 0x72, 0xCC, 0x7E, 0x09, 0x9D, 0x8F ,
     24 );

#define NAME_MessageObj_ReceivedTime L"ReceivedTime"


/*  MessageObj.PatternOriginalDateTime
 *
 *  Contains the original UTC time that a recurring item was to take place.
 *  
 *  Type: String
 *  Form: DateTime
 */

DEFINE_DEVSVCPROPKEY(PKEY_MessageObj_PatternOriginalDateTime,
     0xB28AE94B, 0x05A4, 0x4E8E, 0xBE, 0x01, 0x72, 0xCC, 0x7E, 0x09, 0x9D, 0x8F ,
     26 );

#define NAME_MessageObj_PatternOriginalDateTime L"PatternOriginalDateTime"


/*  MessageObj.PatternType
 *
 *  Contains the pattern type of the recurring item.
 *  
 *  Type: UInt32
 *  Form: Enum
 */

DEFINE_DEVSVCPROPKEY(PKEY_MessageObj_PatternType,
     0xB28AE94B, 0x05A4, 0x4E8E, 0xBE, 0x01, 0x72, 0xCC, 0x7E, 0x09, 0x9D, 0x8F ,
     27 );

#define NAME_MessageObj_PatternType L"PatternType"

#define ENUM_MessageObj_PatternTypeDaily 0x00000001 
#define ENUM_MessageObj_PatternTypeWeekly 0x00000002 
#define ENUM_MessageObj_PatternTypeMonthly 0x00000003 
#define ENUM_MessageObj_PatternTypeYearly 0x00000004 


/*  MessageObj.PatternValidStartDate
 *
 *  Contains the first date on which the recurrence pattern is valid. The
 *  first instance of the recurring item is on or after this date. The date
 *  is in the time reference of the recurring item.
 *  
 *  Type: String
 *  Form: ISO 8601 Date
 */

DEFINE_DEVSVCPROPKEY(PKEY_MessageObj_PatternValidStartDate,
     0xB28AE94B, 0x05A4, 0x4E8E, 0xBE, 0x01, 0x72, 0xCC, 0x7E, 0x09, 0x9D, 0x8F ,
     28 );

#define NAME_MessageObj_PatternValidStartDate L"PatternValidStartDate"


/*  MessageObj.PatternValidEndDate
 *
 *  Contains the last date on which the recurrence pattern is valid. The
 *  last instance of the recurring item is on or before this date. The date
 *  is in the time reference of the recurring item.
 *  
 *  Type: String
 *  Form: ISO 8601 Date
 */

DEFINE_DEVSVCPROPKEY(PKEY_MessageObj_PatternValidEndDate,
     0xB28AE94B, 0x05A4, 0x4E8E, 0xBE, 0x01, 0x72, 0xCC, 0x7E, 0x09, 0x9D, 0x8F ,
     29 );

#define NAME_MessageObj_PatternValidEndDate L"PatternValidEndDate"


/*  MessageObj.PatternPeriod
 *
 *  Contains the period of the repeating recurrence pattern. The units of this
 *  property are determined by the value of MessageObj.PatternType.
 *  
 *  Type: UInt32
 *  Form: None
 */

DEFINE_DEVSVCPROPKEY(PKEY_MessageObj_PatternPeriod,
     0xB28AE94B, 0x05A4, 0x4E8E, 0xBE, 0x01, 0x72, 0xCC, 0x7E, 0x09, 0x9D, 0x8F ,
     31 );

#define NAME_MessageObj_PatternPeriod L"PatternPeriod"


/*  MessageObj.PatternDayOfWeek
 *
 *  Contains the day of week mask for weekly, monthly, and yearly recurrence
 *  patterns.
 *  
 *  Type: UInt16
 *  Form: None
 */

DEFINE_DEVSVCPROPKEY(PKEY_MessageObj_PatternDayOfWeek,
     0xB28AE94B, 0x05A4, 0x4E8E, 0xBE, 0x01, 0x72, 0xCC, 0x7E, 0x09, 0x9D, 0x8F ,
     32 );

#define NAME_MessageObj_PatternDayOfWeek L"PatternDayOfWeek"

#define FLAG_MessageObj_DayOfWeekNone 0x0000 
#define FLAG_MessageObj_DayOfWeekSunday 0x0001 
#define FLAG_MessageObj_DayOfWeekMonday 0x0002 
#define FLAG_MessageObj_DayOfWeekTuesday 0x0004 
#define FLAG_MessageObj_DayOfWeekWednesday 0x0008 
#define FLAG_MessageObj_DayOfWeekThursday 0x0010 
#define FLAG_MessageObj_DayOfWeekFriday 0x0020 
#define FLAG_MessageObj_DayOfWeekSaturday 0x0040 


/*  MessageObj.PatternDayOfMonth
 *
 *  Contains the day of the month for monthly and yearly recurrence patterns.
 *  If day specified is greater than the number of days in the month the
 *  value is interpreted as the last valid day of the month.
 *  
 *  Type: UInt8
 *  Form: Range
 */

DEFINE_DEVSVCPROPKEY(PKEY_MessageObj_PatternDayOfMonth,
     0xB28AE94B, 0x05A4, 0x4E8E, 0xBE, 0x01, 0x72, 0xCC, 0x7E, 0x09, 0x9D, 0x8F ,
     33 );

#define NAME_MessageObj_PatternDayOfMonth L"PatternDayOfMonth"

#define RANGEMIN_MessageObj_PatternDayOfMonth 1 
#define RANGEMAX_MessageObj_PatternDayOfMonth 31 
#define RANGESTEP_MessageObj_PatternDayOfMonth 1 


/*  MessageObj.PatternMonthOfYear
 *
 *  Contains the month of the year for yearly recurrence patterns.
 *  
 *  Type: UInt8
 *  Form: Range
 */

DEFINE_DEVSVCPROPKEY(PKEY_MessageObj_PatternMonthOfYear,
     0xB28AE94B, 0x05A4, 0x4E8E, 0xBE, 0x01, 0x72, 0xCC, 0x7E, 0x09, 0x9D, 0x8F ,
     34 );

#define NAME_MessageObj_PatternMonthOfYear L"PatternMonthOfYear"

#define RANGEMIN_MessageObj_PatternMonthOfYear 1 
#define RANGEMAX_MessageObj_PatternMonthOfYear 12 
#define RANGESTEP_MessageObj_PatternMonthOfYear 1 


/*  MessageObj.PatternInstance
 *
 *  Contains the instance of the recurring pattern that is to be matched.
 *  
 *  Type: UInt8
 *  Form: Enum
 */

DEFINE_DEVSVCPROPKEY(PKEY_MessageObj_PatternInstance,
     0xB28AE94B, 0x05A4, 0x4E8E, 0xBE, 0x01, 0x72, 0xCC, 0x7E, 0x09, 0x9D, 0x8F ,
     35 );

#define NAME_MessageObj_PatternInstance L"PatternInstance"

#define ENUM_MessageObj_PatternInstanceNone 0x00 
#define ENUM_MessageObj_PatternInstanceFirst 0x01 
#define ENUM_MessageObj_PatternInstanceSecond 0x02 
#define ENUM_MessageObj_PatternInstanceThird 0x03 
#define ENUM_MessageObj_PatternInstanceFourth 0x04 
#define ENUM_MessageObj_PatternInstanceLast 0x05 


/*  MessageObj.PatternDeleteDates
 *
 *  Contains a semi-colon separated list of the the dates on which instances
 *  of the recurring item have been deleted. Does not contain the original
 *  dates for items which have been moved. The date specified is in the
 *  time reference of the recurrence.
 *  
 *  Type: AUInt16
 *  Form: LongString
 */

DEFINE_DEVSVCPROPKEY(PKEY_MessageObj_PatternDeleteDates,
     0xB28AE94B, 0x05A4, 0x4E8E, 0xBE, 0x01, 0x72, 0xCC, 0x7E, 0x09, 0x9D, 0x8F ,
     36 );

#define NAME_MessageObj_PatternDeleteDates L"PatternDeleteDates"


#endif /*_MESSAGEDEVICESERVICE_H_*/
