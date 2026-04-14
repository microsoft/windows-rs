/*
 *  NotesDeviceService.h 
 *
 *  Contains declarations for the Notes Device Service 
 *
 *  Copyright (c) Microsoft Corporation, All Rights Reserved.
 *
 */


#ifndef _NOTESDEVICESERVICE_H_
#define _NOTESDEVICESERVICE_H_

#include <DeviceServices.h>

#include <MessageDeviceService.h>


/*****************************************************************************
    Notes Service Info
******************************************************************************/

DEFINE_DEVSVCGUID(SERVICE_Notes,
     0x5c017aea, 0xe706, 0x4719, 0x8c, 0xc0, 0xa3, 0x03, 0x83, 0x6f, 0xd3, 0x21 );

#define NAME_NotesSvc L"Notes"
#define TYPE_NotesSvc DEVSVCTYPE_DEFAULT


/*****************************************************************************
    Notes Service Object Formats
******************************************************************************/

/*  FORMAT_AbstractNote
 */

DEFINE_DEVSVCGUID(FORMAT_AbstractNote,
     0xb3d1b688, 0x39f6, 0x4703, 0xb3, 0x39, 0xc6, 0x9b, 0x7d, 0x2a, 0xbb, 0x3f );

#define NAME_AbstractNote L"AbstractNote"



/*****************************************************************************
    Notes Service Object Property Keys
******************************************************************************/

DEFINE_DEVSVCGUID(NAMESPACE_NotesObj,
     0x5FFBFC7B, 0x7483, 0x41AD, 0xAF, 0xB9, 0xDA, 0x3F, 0x4E, 0x59, 0x2B, 0x8D );


#endif /*_NOTESDEVICESERVICE_H_*/
