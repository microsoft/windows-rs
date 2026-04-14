//-----------------------------------------------------------------------
// <copyright file="WindowsSideShow.h" company="Microsoft">
//      Copyright (c) 2004-2005 Microsoft Corporation.  All rights
//  reserved.
// </copyright>
//
// Module:       
//        WindowsSideShow.h
//
// Description:
//        This file defines supporting structures and values used in
//        the Windows SideShow platform.
//
//-----------------------------------------------------------------------

#pragma once
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

#if (_WIN32_WINNT >= 0x0600) // Windows Vista and later 
#include "propkeydef.h"
/****************************************************************************
 * This section defines platform constants
 ****************************************************************************/

const   CONTENT_ID      CONTENT_ID_GLANCE = 0;

// Event IDs for ApplicationEvents fired by the device when the user enters/exits
// an application on the device.
const   DWORD           SIDESHOW_EVENTID_APPLICATION_ENTER  = 0xFFFF0000;
const   DWORD           SIDESHOW_EVENTID_APPLICATION_EXIT   = 0xFFFF0001;


/****************************************************************************
 * This section defines well-known device endpoints
 ****************************************************************************/

// {A9A5353F-2D4B-47ce-93EE-759F3A7DDA4F}
DEFINE_GUID(SIDESHOW_ENDPOINT_SIMPLE_CONTENT_FORMAT,    0xa9a5353f, 0x2d4b, 0x47ce, 0x93, 0xee, 0x75, 0x9f, 0x3a, 0x7d, 0xda, 0x4f);

// {4DFF36B5-9DDE-4F76-9A2A-96435047063D}
DEFINE_GUID(SIDESHOW_ENDPOINT_ICAL,                     0x4dff36b5, 0x9dde, 0x4f76, 0x9a, 0x2a, 0x96, 0x43, 0x50, 0x47, 0x06, 0x3d);


/****************************************************************************
 * This section defines well-known device capabilities
 ****************************************************************************/

// {8ABC88A8-857B-4ad7-A35A-B5942F492B99}
DEFINE_GUID(SIDESHOW_CAPABILITY_DEVICE_PROPERTIES,          0x8abc88a8, 0x857b, 0x4ad7, 0xa3, 0x5a, 0xb5, 0x94, 0x2f, 0x49, 0x2b, 0x99);

DEFINE_PROPERTYKEY(SIDESHOW_CAPABILITY_DEVICE_ID,           0x8abc88a8, 0x857b, 0x4ad7, 0xa3, 0x5a, 0xb5, 0x94, 0x2f, 0x49, 0x2b, 0x99, 1); // [ VT_LPWSTR ]
DEFINE_PROPERTYKEY(SIDESHOW_CAPABILITY_SCREEN_TYPE,         0x8abc88a8, 0x857b, 0x4ad7, 0xa3, 0x5a, 0xb5, 0x94, 0x2f, 0x49, 0x2b, 0x99, 2); // [ VT_I4 ]
DEFINE_PROPERTYKEY(SIDESHOW_CAPABILITY_SCREEN_WIDTH,        0x8abc88a8, 0x857b, 0x4ad7, 0xa3, 0x5a, 0xb5, 0x94, 0x2f, 0x49, 0x2b, 0x99, 3); // [ VT_UI2 ]
DEFINE_PROPERTYKEY(SIDESHOW_CAPABILITY_SCREEN_HEIGHT,       0x8abc88a8, 0x857b, 0x4ad7, 0xa3, 0x5a, 0xb5, 0x94, 0x2f, 0x49, 0x2b, 0x99, 4); // [ VT_UI2 ]
DEFINE_PROPERTYKEY(SIDESHOW_CAPABILITY_COLOR_DEPTH,         0x8abc88a8, 0x857b, 0x4ad7, 0xa3, 0x5a, 0xb5, 0x94, 0x2f, 0x49, 0x2b, 0x99, 5); // [ VT_UI2 ]
DEFINE_PROPERTYKEY(SIDESHOW_CAPABILITY_COLOR_TYPE,          0x8abc88a8, 0x857b, 0x4ad7, 0xa3, 0x5a, 0xb5, 0x94, 0x2f, 0x49, 0x2b, 0x99, 6); // [ VT_I4 ]
DEFINE_PROPERTYKEY(SIDESHOW_CAPABILITY_DATA_CACHE,          0x8abc88a8, 0x857b, 0x4ad7, 0xa3, 0x5a, 0xb5, 0x94, 0x2f, 0x49, 0x2b, 0x99, 7); // [ VT_BOOL ]
DEFINE_PROPERTYKEY(SIDESHOW_CAPABILITY_SUPPORTED_LANGUAGES, 0x8abc88a8, 0x857b, 0x4ad7, 0xa3, 0x5a, 0xb5, 0x94, 0x2f, 0x49, 0x2b, 0x99, 8); // [ VT_LPWSTR ]
DEFINE_PROPERTYKEY(SIDESHOW_CAPABILITY_CURRENT_LANGUAGE,    0x8abc88a8, 0x857b, 0x4ad7, 0xa3, 0x5a, 0xb5, 0x94, 0x2f, 0x49, 0x2b, 0x99, 9); // [ VT_LPWSTR ]
DEFINE_PROPERTYKEY(SIDESHOW_CAPABILITY_SUPPORTED_THEMES,    0x8abc88a8, 0x857b, 0x4ad7, 0xa3, 0x5a, 0xb5, 0x94, 0x2f, 0x49, 0x2b, 0x99, 10);// [ VT_LPWSTR ]
DEFINE_PROPERTYKEY(SIDESHOW_CAPABILITY_SUPPORTED_IMAGE_FORMATS, 0x8abc88a8, 0x857b, 0x4ad7, 0xa3, 0x5a, 0xb5, 0x94, 0x2f, 0x49, 0x2b, 0x99, 14);// [ VT_LPWSTR ]
DEFINE_PROPERTYKEY(SIDESHOW_CAPABILITY_CLIENT_AREA_WIDTH,   0x8abc88a8, 0x857b, 0x4ad7, 0xa3, 0x5a, 0xb5, 0x94, 0x2f, 0x49, 0x2b, 0x99, 15);// [ VT_UI2 ]
DEFINE_PROPERTYKEY(SIDESHOW_CAPABILITY_CLIENT_AREA_HEIGHT,  0x8abc88a8, 0x857b, 0x4ad7, 0xa3, 0x5a, 0xb5, 0x94, 0x2f, 0x49, 0x2b, 0x99, 16);// [ VT_UI2 ]

/****************************************************************************
 * This section defines enumerations used by the device capabilities
 ****************************************************************************/

// Used with SIDESHOW_CAPABILITY_SCREEN_TYPE
typedef enum tagSIDESHOW_SCREEN_TYPE
{
    SIDESHOW_SCREEN_TYPE_BITMAP  =   0,
    SIDESHOW_SCREEN_TYPE_TEXT    =   1,
} SIDESHOW_SCREEN_TYPE;

// Used with SIDESHOW_CAPABILITY_COLOR_TYPE
typedef enum tagSIDESHOW_COLOR_TYPE
{
    SIDESHOW_COLOR_TYPE_COLOR            = 0,
    SIDESHOW_COLOR_TYPE_GREYSCALE        = 1,
    SIDESHOW_COLOR_TYPE_BLACK_AND_WHITE  = 2,
} SIDESHOW_COLOR_TYPE;


/****************************************************************************
 * This section defines constants, structures and enumerations relating to
 * the Simple Content Format
 ****************************************************************************/

// The content id of the home page for a Simple Content Format application
const CONTENT_ID    CONTENT_ID_HOME     = 1;

// The ApplicationEvent event ids from the Simple Content Format device application
typedef enum tagSCF_EVENT_IDS
{
    SCF_EVENT_NAVIGATION        = 1,
    SCF_EVENT_MENUACTION        = 2,
    SCF_EVENT_CONTEXTMENU       = 3,
} SCF_EVENT_IDS;

// The button ids used in the Simple Content Format events
typedef enum tagSCF_BUTTON_IDS
{
    SCF_BUTTON_MENU     = 1,
    SCF_BUTTON_SELECT   = 2,   
    SCF_BUTTON_UP       = 3,
    SCF_BUTTON_DOWN     = 4,
    SCF_BUTTON_LEFT     = 5,
    SCF_BUTTON_RIGHT    = 6,
    SCF_BUTTON_PLAY     = 7,
    SCF_BUTTON_PAUSE    = 8,
    SCF_BUTTON_FASTFORWARD = 9,
    SCF_BUTTON_REWIND   = 10,
    SCF_BUTTON_STOP     = 11,
    SCF_BUTTON_BACK     = 65280, // 0xFF00
} SCF_BUTTON_IDS;

// A header structure that is common amongst all Simple Content Format event structures
typedef struct tagSCF_EVENT_HEADER
{
    CONTENT_ID  PreviousPage;
    CONTENT_ID  TargetPage;
} SCF_EVENT_HEADER, *PSCF_EVENT_HEADER;

// The data passed with an SCF_EVENT_NAVIGATION ApplicationEvent
typedef struct tagSCF_NAVIGATION_EVENT
{
    CONTENT_ID  PreviousPage;
    CONTENT_ID  TargetPage;
    UINT32      Button;
} SCF_NAVIGATION_EVENT, *PSCF_NAVIGATION_EVENT;

// The data passed with an SCF_EVENT_MENUACTION ApplicationEvent
typedef struct tagSCF_MENUACTION_EVENT
{
    CONTENT_ID  PreviousPage;
    CONTENT_ID  TargetPage;
    UINT32      Button;
    UINT32      ItemId;
} SCF_MENUACTION_EVENT, *PSCF_MENUACTION_EVENT;

// The data passed with an SCF_EVENT_CONTEXTMENU ApplicationEvent
typedef struct tagSCF_CONTEXTMENU_EVENT
{
    CONTENT_ID  PreviousPage;
    CONTENT_ID  TargetPage;
    UINT32      PreviousItemId;
    CONTENT_ID  MenuPage;
    UINT32      MenuItemId;
} SCF_CONTEXTMENU_EVENT, *PSCF_CONTEXTMENU_EVENT;

#endif // (_WIN32_WINNT >= 0x0600) 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

