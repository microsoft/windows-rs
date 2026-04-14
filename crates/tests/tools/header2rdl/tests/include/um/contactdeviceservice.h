/*
 *  ContactDeviceService.h 
 *
 *  Contains declarations for the Contact Device Service 
 *
 *  Copyright (c) Microsoft Corporation, All Rights Reserved.
 *
 */


#ifndef _CONTACTDEVICESERVICE_H_
#define _CONTACTDEVICESERVICE_H_

#include <DeviceServices.h>

#include <SyncDeviceService.h>


/*****************************************************************************
    Contacts Service Info
******************************************************************************/

DEFINE_DEVSVCGUID(SERVICE_Contacts,
     0xDD04D5FC, 0x9D6E, 0x4F76, 0x9D, 0xCF, 0xEC, 0xA6, 0x33, 0x9B, 0x73, 0x89 );

#define NAME_ContactsSvc L"Contacts"
#define TYPE_ContactsSvc DEVSVCTYPE_DEFAULT


/*****************************************************************************
    Contacts Service Properties
******************************************************************************/


/*  PKEY_ContactSvc_SyncWithPhoneOnly
 *
 *  Type: UInt8
 *  Form: None
 */

#define PKEY_ContactSvc_SyncWithPhoneOnly PKEY_SyncSvc_FilterType
#define NAME_ContactSvc_SyncWithPhoneOnly NAME_SyncSvc_FilterType

/*****************************************************************************
    Contacts Service Object Formats
******************************************************************************/

/*  FORMAT_AbstractContact
 */

DEFINE_DEVSVCGUID(FORMAT_AbstractContact,
     0xBB810000, 0xAE6C, 0x4804, 0x98, 0xBA, 0xC5, 0x7B, 0x46, 0x96, 0x5F, 0xE7 );

#define NAME_AbstractContact L"AbstractContact"


/*  FORMAT_VCard2Contact
 */

DEFINE_DEVSVCGUID(FORMAT_VCard2Contact,
     0xBB820000, 0xAE6C, 0x4804, 0x98, 0xBA, 0xC5, 0x7B, 0x46, 0x96, 0x5F, 0xE7 );

#define NAME_VCard2Contact L"VCard2Contact"


/*  FORMAT_VCard3Contact
 */

DEFINE_DEVSVCGUID(FORMAT_VCard3Contact,
     0xBB830000, 0xAE6C, 0x4804, 0x98, 0xBA, 0xC5, 0x7B, 0x46, 0x96, 0x5F, 0xE7 );

#define NAME_VCard3Contact L"VCard3Contact"


/*  FORMAT_AbstractContactGroup
 */

DEFINE_DEVSVCGUID(FORMAT_AbstractContactGroup,
     0xBA060000, 0xAE6C, 0x4804, 0x98, 0xBA, 0xC5, 0x7B, 0x46, 0x96, 0x5F, 0xE7 );

#define NAME_AbstractContactGroup L"AbstractContactGroup"



/*****************************************************************************
    Contacts Service Object Property Keys
******************************************************************************/

DEFINE_DEVSVCGUID(NAMESPACE_ContactObj,
     0xFBD4FDAB, 0x987D, 0x4777, 0xB3, 0xF9, 0x72, 0x61, 0x85, 0xA9, 0x31, 0x2B );


/*  ContactObj.GivenName
 *
 *  MTP Property: Given Name (0xDD00)
 *  Type: String/AUInt16
 *  Form: None/LongString
 */

DEFINE_DEVSVCPROPKEY(PKEY_ContactObj_GivenName,
     0xFBD4FDAB, 0x987D, 0x4777, 0xB3, 0xF9, 0x72, 0x61, 0x85, 0xA9, 0x31, 0x2B ,
     3 );

#define NAME_ContactObj_GivenName L"GivenName"


/*  ContactObj.MiddleNames
 *
 *  MTP Property: Middle Names (0xDD01)
 *  Type: String/AUInt16
 *  Form: None/LongString
 */

DEFINE_DEVSVCPROPKEY(PKEY_ContactObj_MiddleNames,
     0xFBD4FDAB, 0x987D, 0x4777, 0xB3, 0xF9, 0x72, 0x61, 0x85, 0xA9, 0x31, 0x2B ,
     4 );

#define NAME_ContactObj_MiddleNames L"MiddleNames"


/*  ContactObj.FamilyName
 *
 *  MTP Property: Family Name (0xDD02)
 *  Type: String/AUInt16
 *  Form: None/LongString
 */

DEFINE_DEVSVCPROPKEY(PKEY_ContactObj_FamilyName,
     0xFBD4FDAB, 0x987D, 0x4777, 0xB3, 0xF9, 0x72, 0x61, 0x85, 0xA9, 0x31, 0x2B ,
     5 );

#define NAME_ContactObj_FamilyName L"FamilyName"


/*  ContactObj.Title
 *
 *  MTP Property: Prefix (0xDD03)
 *  Type: String/AUInt16
 *  Form: None/LongString
 */

DEFINE_DEVSVCPROPKEY(PKEY_ContactObj_Title,
     0xFBD4FDAB, 0x987D, 0x4777, 0xB3, 0xF9, 0x72, 0x61, 0x85, 0xA9, 0x31, 0x2B ,
     6 );

#define NAME_ContactObj_Title L"Title"


/*  ContactObj.Suffix
 *
 *  MTP Property: Suffix (0xDD04)
 *  Type: String/AUInt16
 *  Form: None/LongString
 */

DEFINE_DEVSVCPROPKEY(PKEY_ContactObj_Suffix,
     0xFBD4FDAB, 0x987D, 0x4777, 0xB3, 0xF9, 0x72, 0x61, 0x85, 0xA9, 0x31, 0x2B ,
     7 );

#define NAME_ContactObj_Suffix L"Suffix"


/*  ContactObj.PhoneticGivenName
 *
 *  MTP Property: Phonetic Given Name (0xDD05)
 *  Type: String/AUInt16
 *  Form: None/LongString
 */

DEFINE_DEVSVCPROPKEY(PKEY_ContactObj_PhoneticGivenName,
     0xFBD4FDAB, 0x987D, 0x4777, 0xB3, 0xF9, 0x72, 0x61, 0x85, 0xA9, 0x31, 0x2B ,
     8 );

#define NAME_ContactObj_PhoneticGivenName L"PhoneticGivenName"


/*  ContactObj.PhoneticFamilyName
 *
 *  MTP Property: Phonetic Family Name (0xDD06)
 *  Type: String/AUInt16
 *  Form: None/LongString
 */

DEFINE_DEVSVCPROPKEY(PKEY_ContactObj_PhoneticFamilyName,
     0xFBD4FDAB, 0x987D, 0x4777, 0xB3, 0xF9, 0x72, 0x61, 0x85, 0xA9, 0x31, 0x2B ,
     9 );

#define NAME_ContactObj_PhoneticFamilyName L"PhoneticFamilyName"


/*  ContactObj.PersonalAddressFull
 *
 *  MTP Property: Postal Address Personal Full (0xDD1F)
 *  Type: String/AUInt16
 *  Form: None/RegEx/LongString
 */

DEFINE_DEVSVCPROPKEY(PKEY_ContactObj_PersonalAddressFull,
     0xFBD4FDAB, 0x987D, 0x4777, 0xB3, 0xF9, 0x72, 0x61, 0x85, 0xA9, 0x31, 0x2B ,
     10 );

#define NAME_ContactObj_PersonalAddressFull L"PersonalAddressFull"


/*  ContactObj.PersonalAddressStreet
 *
 *  MTP Property: Postal Address Line 1 (0xDD20)
 *  Type: String/AUInt16
 *  Form: None/RegEx/LongString
 */

DEFINE_DEVSVCPROPKEY(PKEY_ContactObj_PersonalAddressStreet,
     0xFBD4FDAB, 0x987D, 0x4777, 0xB3, 0xF9, 0x72, 0x61, 0x85, 0xA9, 0x31, 0x2B ,
     11 );

#define NAME_ContactObj_PersonalAddressStreet L"PersonalAddressStreet"


/*  ContactObj.PersonalAddressLine2
 *
 *  MTP Property: Postal Address Line 2 (0xDD21)
 *  Type: String/AUInt16
 *  Form: None/RegEx/LongString
 */

DEFINE_DEVSVCPROPKEY(PKEY_ContactObj_PersonalAddressLine2,
     0xFBD4FDAB, 0x987D, 0x4777, 0xB3, 0xF9, 0x72, 0x61, 0x85, 0xA9, 0x31, 0x2B ,
     12 );

#define NAME_ContactObj_PersonalAddressLine2 L"PersonalAddressLine2"


/*  ContactObj.PersonalAddressCity
 *
 *  MTP Property: Postal Address Personal City (0xDD22)
 *  Type: String/AUInt16
 *  Form: None/RegEx/LongString
 */

DEFINE_DEVSVCPROPKEY(PKEY_ContactObj_PersonalAddressCity,
     0xFBD4FDAB, 0x987D, 0x4777, 0xB3, 0xF9, 0x72, 0x61, 0x85, 0xA9, 0x31, 0x2B ,
     13 );

#define NAME_ContactObj_PersonalAddressCity L"PersonalAddressCity"


/*  ContactObj.PersonalAddressRegion
 *
 *  MTP Property: Postal Address Personal Region (0xDD23)
 *  Type: String/AUInt16
 *  Form: None/RegEx/LongString
 */

DEFINE_DEVSVCPROPKEY(PKEY_ContactObj_PersonalAddressRegion,
     0xFBD4FDAB, 0x987D, 0x4777, 0xB3, 0xF9, 0x72, 0x61, 0x85, 0xA9, 0x31, 0x2B ,
     14 );

#define NAME_ContactObj_PersonalAddressRegion L"PersonalAddressRegion"


/*  ContactObj.PersonalAddressPostalCode
 *
 *  MTP Property: Postal Address Personal Postal Code (0xDD24)
 *  Type: String/AUInt16
 *  Form: None/RegEx/LongString
 */

DEFINE_DEVSVCPROPKEY(PKEY_ContactObj_PersonalAddressPostalCode,
     0xFBD4FDAB, 0x987D, 0x4777, 0xB3, 0xF9, 0x72, 0x61, 0x85, 0xA9, 0x31, 0x2B ,
     15 );

#define NAME_ContactObj_PersonalAddressPostalCode L"PersonalAddressPostalCode"


/*  ContactObj.PersonalAddressCountry
 *
 *  MTP Property: Postal Address Personal Country (0xDD25)
 *  Type: String/AUInt16
 *  Form: None/RegEx/LongString
 */

DEFINE_DEVSVCPROPKEY(PKEY_ContactObj_PersonalAddressCountry,
     0xFBD4FDAB, 0x987D, 0x4777, 0xB3, 0xF9, 0x72, 0x61, 0x85, 0xA9, 0x31, 0x2B ,
     16 );

#define NAME_ContactObj_PersonalAddressCountry L"PersonalAddressCountry"


/*  ContactObj.BusinessAddressFull
 *
 *  MTP Property: Postal Address Business Full (0xDD26)
 *  Type: String/AUInt16
 *  Form: None/RegEx/LongString
 */

DEFINE_DEVSVCPROPKEY(PKEY_ContactObj_BusinessAddressFull,
     0xFBD4FDAB, 0x987D, 0x4777, 0xB3, 0xF9, 0x72, 0x61, 0x85, 0xA9, 0x31, 0x2B ,
     17 );

#define NAME_ContactObj_BusinessAddressFull L"BusinessAddressFull"


/*  ContactObj.BusinessAddressStreet
 *
 *  MTP Property: Postal Address Business Line 1 (0xDD27)
 *  Type: String/AUInt16
 *  Form: None/RegEx/LongString
 */

DEFINE_DEVSVCPROPKEY(PKEY_ContactObj_BusinessAddressStreet,
     0xFBD4FDAB, 0x987D, 0x4777, 0xB3, 0xF9, 0x72, 0x61, 0x85, 0xA9, 0x31, 0x2B ,
     18 );

#define NAME_ContactObj_BusinessAddressStreet L"BusinessAddressStreet"


/*  ContactObj.BusinessAddressLine2
 *
 *  MTP Property: Postal Address Business Line 2 (0xDD28)
 *  Type: String/AUInt16
 *  Form: None/RegEx/LongString
 */

DEFINE_DEVSVCPROPKEY(PKEY_ContactObj_BusinessAddressLine2,
     0xFBD4FDAB, 0x987D, 0x4777, 0xB3, 0xF9, 0x72, 0x61, 0x85, 0xA9, 0x31, 0x2B ,
     19 );

#define NAME_ContactObj_BusinessAddressLine2 L"BusinessAddressLine2"


/*  ContactObj.BusinessAddressCity
 *
 *  MTP Property: Postal Address Business City (0xDD29)
 *  Type: String/AUInt16
 *  Form: None/RegEx/LongString
 */

DEFINE_DEVSVCPROPKEY(PKEY_ContactObj_BusinessAddressCity,
     0xFBD4FDAB, 0x987D, 0x4777, 0xB3, 0xF9, 0x72, 0x61, 0x85, 0xA9, 0x31, 0x2B ,
     20 );

#define NAME_ContactObj_BusinessAddressCity L"BusinessAddressCity"


/*  ContactObj.BusinessAddressRegion
 *
 *  MTP Property: Postal Address Business Region (0xDD2A)
 *  Type: String/AUInt16
 *  Form: None/RegEx/LongString
 */

DEFINE_DEVSVCPROPKEY(PKEY_ContactObj_BusinessAddressRegion,
     0xFBD4FDAB, 0x987D, 0x4777, 0xB3, 0xF9, 0x72, 0x61, 0x85, 0xA9, 0x31, 0x2B ,
     21 );

#define NAME_ContactObj_BusinessAddressRegion L"BusinessAddressRegion"


/*  ContactObj.BusinessAddressPostalCode
 *
 *  MTP Property: Postal Address Business Postal Code (0xDD2B)
 *  Type: String/AUInt16
 *  Form: None/RegEx/LongString
 */

DEFINE_DEVSVCPROPKEY(PKEY_ContactObj_BusinessAddressPostalCode,
     0xFBD4FDAB, 0x987D, 0x4777, 0xB3, 0xF9, 0x72, 0x61, 0x85, 0xA9, 0x31, 0x2B ,
     22 );

#define NAME_ContactObj_BusinessAddressPostalCode L"BusinessAddressPostalCode"


/*  ContactObj.BusinessAddressCountry
 *
 *  MTP Property: Postal Address Business Country (0xDD2C)
 *  Type: String/AUInt16
 *  Form: None/RegEx/LongString
 */

DEFINE_DEVSVCPROPKEY(PKEY_ContactObj_BusinessAddressCountry,
     0xFBD4FDAB, 0x987D, 0x4777, 0xB3, 0xF9, 0x72, 0x61, 0x85, 0xA9, 0x31, 0x2B ,
     23 );

#define NAME_ContactObj_BusinessAddressCountry L"BusinessAddressCountry"


/*  ContactObj.OtherAddressFull
 *
 *  MTP Property: Postal Address Other Full (0xDD2D)
 *  Type: String/AUInt16
 *  Form: None/RegEx/LongString
 */

DEFINE_DEVSVCPROPKEY(PKEY_ContactObj_OtherAddressFull,
     0xFBD4FDAB, 0x987D, 0x4777, 0xB3, 0xF9, 0x72, 0x61, 0x85, 0xA9, 0x31, 0x2B ,
     24 );

#define NAME_ContactObj_OtherAddressFull L"OtherAddressFull"


/*  ContactObj.OtherAddressStreet
 *
 *  MTP Property: Postal Address Other Line 1 (0xDD2E)
 *  Type: String/AUInt16
 *  Form: None/RegEx/LongString
 */

DEFINE_DEVSVCPROPKEY(PKEY_ContactObj_OtherAddressStreet,
     0xFBD4FDAB, 0x987D, 0x4777, 0xB3, 0xF9, 0x72, 0x61, 0x85, 0xA9, 0x31, 0x2B ,
     25 );

#define NAME_ContactObj_OtherAddressStreet L"OtherAddressStreet"


/*  ContactObj.OtherAddressLine2
 *
 *  MTP Property: Postal Address Other Line 2 (0xDD2F)
 *  Type: String/AUInt16
 *  Form: None/RegEx/LongString
 */

DEFINE_DEVSVCPROPKEY(PKEY_ContactObj_OtherAddressLine2,
     0xFBD4FDAB, 0x987D, 0x4777, 0xB3, 0xF9, 0x72, 0x61, 0x85, 0xA9, 0x31, 0x2B ,
     26 );

#define NAME_ContactObj_OtherAddressLine2 L"OtherAddressLine2"


/*  ContactObj.OtherAddressCity
 *
 *  MTP Property: Postal Address Other City (0xDD30)
 *  Type: String/AUInt16
 *  Form: None/RegEx/LongString
 */

DEFINE_DEVSVCPROPKEY(PKEY_ContactObj_OtherAddressCity,
     0xFBD4FDAB, 0x987D, 0x4777, 0xB3, 0xF9, 0x72, 0x61, 0x85, 0xA9, 0x31, 0x2B ,
     27 );

#define NAME_ContactObj_OtherAddressCity L"OtherAddressCity"


/*  ContactObj.OtherAddressRegion
 *
 *  MTP Property: Postal Address Other Region (0xDD31)
 *  Type: String/AUInt16
 *  Form: None/RegEx/LongString
 */

DEFINE_DEVSVCPROPKEY(PKEY_ContactObj_OtherAddressRegion,
     0xFBD4FDAB, 0x987D, 0x4777, 0xB3, 0xF9, 0x72, 0x61, 0x85, 0xA9, 0x31, 0x2B ,
     28 );

#define NAME_ContactObj_OtherAddressRegion L"OtherAddressRegion"


/*  ContactObj.OtherAddressPostalCode
 *
 *  MTP Property: Postal Address Other Postal Code (0xDD32)
 *  Type: String/AUInt16
 *  Form: None/RegEx/LongString
 */

DEFINE_DEVSVCPROPKEY(PKEY_ContactObj_OtherAddressPostalCode,
     0xFBD4FDAB, 0x987D, 0x4777, 0xB3, 0xF9, 0x72, 0x61, 0x85, 0xA9, 0x31, 0x2B ,
     29 );

#define NAME_ContactObj_OtherAddressPostalCode L"OtherAddressPostalCode"


/*  ContactObj.OtherAddressCountry
 *
 *  MTP Property: Postal Address Other Country (0xDD33)
 *  Type: String/AUInt16
 *  Form: None/RegEx/LongString
 */

DEFINE_DEVSVCPROPKEY(PKEY_ContactObj_OtherAddressCountry,
     0xFBD4FDAB, 0x987D, 0x4777, 0xB3, 0xF9, 0x72, 0x61, 0x85, 0xA9, 0x31, 0x2B ,
     30 );

#define NAME_ContactObj_OtherAddressCountry L"OtherAddressCountry"


/*  ContactObj.Email
 *
 *  MTP Property: Email Primary (0xDD07)
 *  Type: String/AUInt16
 *  Form: None/RegEx/LongString
 */

DEFINE_DEVSVCPROPKEY(PKEY_ContactObj_Email,
     0xFBD4FDAB, 0x987D, 0x4777, 0xB3, 0xF9, 0x72, 0x61, 0x85, 0xA9, 0x31, 0x2B ,
     31 );

#define NAME_ContactObj_Email L"Email"


/*  ContactObj.PersonalEmail
 *
 *  MTP Property: Email Personal 1 (0xDD08)
 *  Type: String/AUInt16
 *  Form: None/RegEx/LongString
 */

DEFINE_DEVSVCPROPKEY(PKEY_ContactObj_PersonalEmail,
     0xFBD4FDAB, 0x987D, 0x4777, 0xB3, 0xF9, 0x72, 0x61, 0x85, 0xA9, 0x31, 0x2B ,
     32 );

#define NAME_ContactObj_PersonalEmail L"PersonalEmail"


/*  ContactObj.PersonalEmail2
 *
 *  MTP Property: Email Personal 2 (0xDD09)
 *  Type: String/AUInt16
 *  Form: None/RegEx/LongString
 */

DEFINE_DEVSVCPROPKEY(PKEY_ContactObj_PersonalEmail2,
     0xFBD4FDAB, 0x987D, 0x4777, 0xB3, 0xF9, 0x72, 0x61, 0x85, 0xA9, 0x31, 0x2B ,
     33 );

#define NAME_ContactObj_PersonalEmail2 L"PersonalEmail2"


/*  ContactObj.BusinessEmail
 *
 *  MTP Property: Email Business 1 (0xDD0A)
 *  Type: String/AUInt16
 *  Form: None/RegEx/LongString
 */

DEFINE_DEVSVCPROPKEY(PKEY_ContactObj_BusinessEmail,
     0xFBD4FDAB, 0x987D, 0x4777, 0xB3, 0xF9, 0x72, 0x61, 0x85, 0xA9, 0x31, 0x2B ,
     34 );

#define NAME_ContactObj_BusinessEmail L"BusinessEmail"


/*  ContactObj.BusinessEmail2
 *
 *  MTP Property: Email Business 2 (0xDD0B)
 *  Type: String/AUInt16
 *  Form: None/RegEx/LongString
 */

DEFINE_DEVSVCPROPKEY(PKEY_ContactObj_BusinessEmail2,
     0xFBD4FDAB, 0x987D, 0x4777, 0xB3, 0xF9, 0x72, 0x61, 0x85, 0xA9, 0x31, 0x2B ,
     35 );

#define NAME_ContactObj_BusinessEmail2 L"BusinessEmail2"


/*  ContactObj.OtherEmail
 *
 *  MTP Property: Email Others (0xDD0C)
 *  Type: AUInt16
 *  Form: LongString
 */

DEFINE_DEVSVCPROPKEY(PKEY_ContactObj_OtherEmail,
     0xFBD4FDAB, 0x987D, 0x4777, 0xB3, 0xF9, 0x72, 0x61, 0x85, 0xA9, 0x31, 0x2B ,
     36 );

#define NAME_ContactObj_OtherEmail L"OtherEmail"


/*  ContactObj.Phone
 *
 *  MTP Property: Phone Primary (0xDD0D)
 *  Type: String/AUInt16
 *  Form: None/RegEx/LongString
 */

DEFINE_DEVSVCPROPKEY(PKEY_ContactObj_Phone,
     0xFBD4FDAB, 0x987D, 0x4777, 0xB3, 0xF9, 0x72, 0x61, 0x85, 0xA9, 0x31, 0x2B ,
     37 );

#define NAME_ContactObj_Phone L"Phone"


/*  ContactObj.PersonalPhone
 *
 *  MTP Property: Phone Number Personal 1 (0xDD0E)
 *  Type: String/AUInt16
 *  Form: None/RegEx/LongString
 */

DEFINE_DEVSVCPROPKEY(PKEY_ContactObj_PersonalPhone,
     0xFBD4FDAB, 0x987D, 0x4777, 0xB3, 0xF9, 0x72, 0x61, 0x85, 0xA9, 0x31, 0x2B ,
     38 );

#define NAME_ContactObj_PersonalPhone L"PersonalPhone"


/*  ContactObj.PersonalPhone2
 *
 *  MTP Property: Phone Number Personal 2 (0xDD0F)
 *  Type: String/AUInt16
 *  Form: None/RegEx/LongString
 */

DEFINE_DEVSVCPROPKEY(PKEY_ContactObj_PersonalPhone2,
     0xFBD4FDAB, 0x987D, 0x4777, 0xB3, 0xF9, 0x72, 0x61, 0x85, 0xA9, 0x31, 0x2B ,
     39 );

#define NAME_ContactObj_PersonalPhone2 L"PersonalPhone2"


/*  ContactObj.BusinessPhone
 *
 *  MTP Property: Phone Number Business 1 (0xDD10)
 *  Type: String/AUInt16
 *  Form: None/RegEx/LongString
 */

DEFINE_DEVSVCPROPKEY(PKEY_ContactObj_BusinessPhone,
     0xFBD4FDAB, 0x987D, 0x4777, 0xB3, 0xF9, 0x72, 0x61, 0x85, 0xA9, 0x31, 0x2B ,
     40 );

#define NAME_ContactObj_BusinessPhone L"BusinessPhone"


/*  ContactObj.BusinessPhone2
 *
 *  MTP Property: Phone Number Business 2 (0xDD11)
 *  Type: String/AUInt16
 *  Form: None/RegEx/LongString
 */

DEFINE_DEVSVCPROPKEY(PKEY_ContactObj_BusinessPhone2,
     0xFBD4FDAB, 0x987D, 0x4777, 0xB3, 0xF9, 0x72, 0x61, 0x85, 0xA9, 0x31, 0x2B ,
     41 );

#define NAME_ContactObj_BusinessPhone2 L"BusinessPhone2"


/*  ContactObj.MobilePhone
 *
 *  MTP Property: Phone Number Mobile 1 (0xDD12)
 *  Type: String/AUInt16
 *  Form: None/RegEx/LongString
 */

DEFINE_DEVSVCPROPKEY(PKEY_ContactObj_MobilePhone,
     0xFBD4FDAB, 0x987D, 0x4777, 0xB3, 0xF9, 0x72, 0x61, 0x85, 0xA9, 0x31, 0x2B ,
     42 );

#define NAME_ContactObj_MobilePhone L"MobilePhone"


/*  ContactObj.MobilePhone2
 *
 *  MTP Property: Phone Number Mobile 2 (0xDD13)
 *  Type: String/AUInt16
 *  Form: None/RegEx/LongString
 */

DEFINE_DEVSVCPROPKEY(PKEY_ContactObj_MobilePhone2,
     0xFBD4FDAB, 0x987D, 0x4777, 0xB3, 0xF9, 0x72, 0x61, 0x85, 0xA9, 0x31, 0x2B ,
     43 );

#define NAME_ContactObj_MobilePhone2 L"MobilePhone2"


/*  ContactObj.PersonalFax
 *
 *  MTP Property: Fax Number Personal (0xDD15)
 *  Type: String/AUInt16
 *  Form: None/RegEx/LongString
 */

DEFINE_DEVSVCPROPKEY(PKEY_ContactObj_PersonalFax,
     0xFBD4FDAB, 0x987D, 0x4777, 0xB3, 0xF9, 0x72, 0x61, 0x85, 0xA9, 0x31, 0x2B ,
     44 );

#define NAME_ContactObj_PersonalFax L"PersonalFax"


/*  ContactObj.BusinessFax
 *
 *  MTP Property: Fax Number Business (0xDD16)
 *  Type: String/AUInt16
 *  Form: None/RegEx/LongString
 */

DEFINE_DEVSVCPROPKEY(PKEY_ContactObj_BusinessFax,
     0xFBD4FDAB, 0x987D, 0x4777, 0xB3, 0xF9, 0x72, 0x61, 0x85, 0xA9, 0x31, 0x2B ,
     45 );

#define NAME_ContactObj_BusinessFax L"BusinessFax"


/*  ContactObj.Pager
 *
 *  MTP Property: Pager Number (0xDD17)
 *  Type: String/AUInt16
 *  Form: None/RegEx/LongString
 */

DEFINE_DEVSVCPROPKEY(PKEY_ContactObj_Pager,
     0xFBD4FDAB, 0x987D, 0x4777, 0xB3, 0xF9, 0x72, 0x61, 0x85, 0xA9, 0x31, 0x2B ,
     46 );

#define NAME_ContactObj_Pager L"Pager"


/*  ContactObj.OtherPhone
 *
 *  MTP Property: Phone Number Others (0xDD18)
 *  Type: AUInt16
 *  Form: LongString
 */

DEFINE_DEVSVCPROPKEY(PKEY_ContactObj_OtherPhone,
     0xFBD4FDAB, 0x987D, 0x4777, 0xB3, 0xF9, 0x72, 0x61, 0x85, 0xA9, 0x31, 0x2B ,
     47 );

#define NAME_ContactObj_OtherPhone L"OtherPhone"


/*  ContactObj.WebAddress
 *
 *  MTP Property: Primary Web Address (0xDD19)
 *  Type: String/AUInt16
 *  Form: None/RegEx/LongString
 */

DEFINE_DEVSVCPROPKEY(PKEY_ContactObj_WebAddress,
     0xFBD4FDAB, 0x987D, 0x4777, 0xB3, 0xF9, 0x72, 0x61, 0x85, 0xA9, 0x31, 0x2B ,
     48 );

#define NAME_ContactObj_WebAddress L"WebAddress"


/*  ContactObj.PersonalWebAddress
 *
 *  MTP Property: Personal Web Address (0xDD1A)
 *  Type: String/AUInt16
 *  Form: None/RegEx/LongString
 */

DEFINE_DEVSVCPROPKEY(PKEY_ContactObj_PersonalWebAddress,
     0xFBD4FDAB, 0x987D, 0x4777, 0xB3, 0xF9, 0x72, 0x61, 0x85, 0xA9, 0x31, 0x2B ,
     49 );

#define NAME_ContactObj_PersonalWebAddress L"PersonalWebAddress"


/*  ContactObj.BusinessWebAddress
 *
 *  MTP Property: Business Web Address (0xDD1B)
 *  Type: String/AUInt16
 *  Form: None/RegEx/LongString
 */

DEFINE_DEVSVCPROPKEY(PKEY_ContactObj_BusinessWebAddress,
     0xFBD4FDAB, 0x987D, 0x4777, 0xB3, 0xF9, 0x72, 0x61, 0x85, 0xA9, 0x31, 0x2B ,
     50 );

#define NAME_ContactObj_BusinessWebAddress L"BusinessWebAddress"


/*  ContactObj.IMAddress
 *
 *  MTP Property: Instant Messanger Address (0xDD1C)
 *  Type: String/AUInt16
 *  Form: None/RegEx/LongString
 */

DEFINE_DEVSVCPROPKEY(PKEY_ContactObj_IMAddress,
     0xFBD4FDAB, 0x987D, 0x4777, 0xB3, 0xF9, 0x72, 0x61, 0x85, 0xA9, 0x31, 0x2B ,
     51 );

#define NAME_ContactObj_IMAddress L"IMAddress"


/*  ContactObj.IMAddress2
 *
 *  MTP Property: Instant Messanger Address 2 (0xDD1D)
 *  Type: String/AUInt16
 *  Form: None/RegEx/LongString
 */

DEFINE_DEVSVCPROPKEY(PKEY_ContactObj_IMAddress2,
     0xFBD4FDAB, 0x987D, 0x4777, 0xB3, 0xF9, 0x72, 0x61, 0x85, 0xA9, 0x31, 0x2B ,
     52 );

#define NAME_ContactObj_IMAddress2 L"IMAddress2"


/*  ContactObj.IMAddress3
 *
 *  MTP Property: Instant Messanger Address 3 (0xDD1E)
 *  Type: String/AUInt16
 *  Form: None/RegEx/LongString
 */

DEFINE_DEVSVCPROPKEY(PKEY_ContactObj_IMAddress3,
     0xFBD4FDAB, 0x987D, 0x4777, 0xB3, 0xF9, 0x72, 0x61, 0x85, 0xA9, 0x31, 0x2B ,
     53 );

#define NAME_ContactObj_IMAddress3 L"IMAddress3"


/*  ContactObj.Organization
 *
 *  MTP Property: Organization Name (0xDD34)
 *  Type: String/AUInt16
 *  Form: None/LongString
 */

DEFINE_DEVSVCPROPKEY(PKEY_ContactObj_Organization,
     0xFBD4FDAB, 0x987D, 0x4777, 0xB3, 0xF9, 0x72, 0x61, 0x85, 0xA9, 0x31, 0x2B ,
     54 );

#define NAME_ContactObj_Organization L"Organization"


/*  ContactObj.PhoneticOrganization
 *
 *  MTP Property: Phonetic Organization Name (0xDD35)
 *  Type: String/AUInt16
 *  Form: None/LongString
 */

DEFINE_DEVSVCPROPKEY(PKEY_ContactObj_PhoneticOrganization,
     0xFBD4FDAB, 0x987D, 0x4777, 0xB3, 0xF9, 0x72, 0x61, 0x85, 0xA9, 0x31, 0x2B ,
     55 );

#define NAME_ContactObj_PhoneticOrganization L"PhoneticOrganization"


/*  ContactObj.Role
 *
 *  MTP Property: Role (0xDD36)
 *  Type: String/AUInt16
 *  Form: None/LongString
 */

DEFINE_DEVSVCPROPKEY(PKEY_ContactObj_Role,
     0xFBD4FDAB, 0x987D, 0x4777, 0xB3, 0xF9, 0x72, 0x61, 0x85, 0xA9, 0x31, 0x2B ,
     56 );

#define NAME_ContactObj_Role L"Role"


/*  ContactObj.Fax
 *
 *  MTP Property: Fax Number Primary (0xDD14)
 *  Type: String/AUInt16
 *  Form: None/RegEx/LongString
 */

DEFINE_DEVSVCPROPKEY(PKEY_ContactObj_Fax,
     0xFBD4FDAB, 0x987D, 0x4777, 0xB3, 0xF9, 0x72, 0x61, 0x85, 0xA9, 0x31, 0x2B ,
     58 );

#define NAME_ContactObj_Fax L"Fax"


/*  ContactObj.Spouse
 *
 *  Type: String/AUInt16
 *  Form: None/RegEx/LongString
 */

DEFINE_DEVSVCPROPKEY(PKEY_ContactObj_Spouse,
     0xFBD4FDAB, 0x987D, 0x4777, 0xB3, 0xF9, 0x72, 0x61, 0x85, 0xA9, 0x31, 0x2B ,
     59 );

#define NAME_ContactObj_Spouse L"Spouse"


/*  ContactObj.Children
 *
 *  Type: String/AUInt16
 *  Form: None/RegEx/LongString
 */

DEFINE_DEVSVCPROPKEY(PKEY_ContactObj_Children,
     0xFBD4FDAB, 0x987D, 0x4777, 0xB3, 0xF9, 0x72, 0x61, 0x85, 0xA9, 0x31, 0x2B ,
     60 );

#define NAME_ContactObj_Children L"Children"


/*  ContactObj.Assistant
 *
 *  Type: String/AUInt16
 *  Form: None/RegEx/LongString
 */

DEFINE_DEVSVCPROPKEY(PKEY_ContactObj_Assistant,
     0xFBD4FDAB, 0x987D, 0x4777, 0xB3, 0xF9, 0x72, 0x61, 0x85, 0xA9, 0x31, 0x2B ,
     61 );

#define NAME_ContactObj_Assistant L"Assistant"


/*  ContactObj.Ringtone
 *
 *  Type: UInt32
 *  Form: ObjectID
 */

DEFINE_DEVSVCPROPKEY(PKEY_ContactObj_Ringtone,
     0xFBD4FDAB, 0x987D, 0x4777, 0xB3, 0xF9, 0x72, 0x61, 0x85, 0xA9, 0x31, 0x2B ,
     63 );

#define NAME_ContactObj_Ringtone L"Ringtone"


/*  ContactObj.Birthdate
 *
 *  MTP Property: Birthdate (0xDD37)
 *  Type: String
 *  Form: DateTime
 */

DEFINE_DEVSVCPROPKEY(PKEY_ContactObj_Birthdate,
     0xFBD4FDAB, 0x987D, 0x4777, 0xB3, 0xF9, 0x72, 0x61, 0x85, 0xA9, 0x31, 0x2B ,
     65 );

#define NAME_ContactObj_Birthdate L"Birthdate"


/*  ContactObj.AnniversaryDate
 *
 *  Type: String
 *  Form: DateTime
 */

DEFINE_DEVSVCPROPKEY(PKEY_ContactObj_AnniversaryDate,
     0xFBD4FDAB, 0x987D, 0x4777, 0xB3, 0xF9, 0x72, 0x61, 0x85, 0xA9, 0x31, 0x2B ,
     66 );

#define NAME_ContactObj_AnniversaryDate L"AnniversaryDate"


#endif /*_CONTACTDEVICESERVICE_H_*/
