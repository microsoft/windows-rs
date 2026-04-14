//-----------------------------------------------------------------------
// <copyright file="WindowsSideShowDriverEvents.h" company="Microsoft">
//      Copyright (c) 2005 Microsoft Corporation.  All rights
//  reserved.
// </copyright>
//
// Module:
//      WindowsSideShowDriverEvents.h
//
// Description:
//      This header contains structures and values related to events
//      that can be sent by Windows SideShow drivers.
//
// Comments:
//      They are currently intended to be used by the UMDF PnP eventing
//      mechanism.
//
//-----------------------------------------------------------------------

#pragma once
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

#if (_WIN32_WINNT >= 0x0600) // Windows Vista and later


// Version number used in new Windows 7 structs
const DWORD VERSION_1_WINDOWS_7 = 0;


//////////////////////////////////////////////////////////////////////////////
//
// This section defines the device functional interface GUIDs for
// Windows SideShow-compatible devices.
//
//////////////////////////////////////////////////////////////////////////////

// {152E5811-FEB9-4B00-90F4-D32947AE1681}
DEFINE_GUID(GUID_DEVINTERFACE_SIDESHOW,         0x152e5811, 0xfeb9, 0x4b00, 0x90, 0xf4, 0xd3, 0x29, 0x47, 0xae, 0x16, 0x81);

//////////////////////////////////////////////////////////////////////////////
//
// This section defines the GUIDs associated with the various event types.
//
//////////////////////////////////////////////////////////////////////////////

// {5007FBA8-D313-439f-BEA2-A50201D3E9A8}
DEFINE_GUID(SIDESHOW_CONTENT_MISSING_EVENT,     0x5007fba8, 0xd313, 0x439f, 0xbe, 0xa2, 0xa5, 0x02, 0x01, 0xd3, 0xe9, 0xa8);

// {4CB572FA-1D3B-49b3-A17A-2E6BFF052854}
DEFINE_GUID(SIDESHOW_APPLICATION_EVENT,         0x4cb572fa, 0x1d3b, 0x49b3, 0xa1, 0x7a, 0x2e, 0x6b, 0xff, 0x05, 0x28, 0x54);

// {5009673c-3f7d-4c7e-9971-eaa2e91f1575}
DEFINE_GUID(SIDESHOW_USER_CHANGE_REQUEST_EVENT, 0x5009673c, 0x3f7d, 0x4c7e, 0x99, 0x71, 0xea, 0xa2, 0xe9, 0x1f, 0x15, 0x75);

// {57813854-2FC1-411C-A59F-F24927608804}
DEFINE_GUID(SIDESHOW_NEW_EVENT_DATA_AVAILABLE,  0x57813854, 0x2FC1, 0x411C, 0xA5, 0x9F, 0xF2, 0x49, 0x27, 0x60, 0x88, 0x04);


//////////////////////////////////////////////////////////////////////////////
//
// This section defines the data associated with each event.
//
//////////////////////////////////////////////////////////////////////////////

#pragma pack(push, WindowsSideShowEvents, 1)

//////////////////////////////////////////////////////////////////////////////
//
// This event is posted in response to Content Missing Event on the device.  The
// struct contains the parameters that identify which device the event came
// from, which application/endpoint it is for, and the content id of the
// requested content.
//
////////////////////////////////////////////////////////////////////////////////
typedef struct _CONTENT_MISSING_EVENT_DATA
{
    DWORD           cbContentMissingEventData;
    APPLICATION_ID  ApplicationId;
    ENDPOINT_ID     EndpointId;
    CONTENT_ID      ContentId;
} CONTENT_MISSING_EVENT_DATA, *PCONTENT_MISSING_EVENT_DATA;

//////////////////////////////////////////////////////////////////////////////
//
// This struct contains an event sent by an application on the device.
// The data is determined by the application and the event id.  Since
// the data can be variable size, we store the size of the data and then
// the first byte of it.  This struct should always be allocated as
// offsetof(APPLICATION_EVENT_DATA, bEventData) + <event data size>, and this
// size should be set in cbApplicationEventData.  The cbEventData
// member should contain just <event data size>.
//
//////////////////////////////////////////////////////////////////////////////
typedef struct _APPLICATION_EVENT_DATA
{
    DWORD           cbApplicationEventData;
    APPLICATION_ID  ApplicationId;
    ENDPOINT_ID     EndpointId;
    DWORD           dwEventId;
    DWORD           cbEventData;
    BYTE            bEventData[1];
} APPLICATION_EVENT_DATA, *PAPPLICATION_EVENT_DATA;


//////////////////////////////////////////////////////////////////////////////
//
// This struct contains an event sent by the driver to the platform.  Shared
// devices can use this event in their driver to signal to the platform that a
// a request for a new user to take ownership of the device has been made.  The
// platform will set the current user on the device in response to the message,
// and upon success, will notify gadgets as if a device add had taken place.
// When posting this event to request that the active user of the device be
// changed, drivers should set wszUser to the SID of the user that is the new
// requested owner of the device.
//
//////////////////////////////////////////////////////////////////////////////
typedef struct _DEVICE_USER_CHANGE_EVENT_DATA
{
    DWORD                       cbDeviceUserChangeEventData;
    WCHAR                       wszUser;    // First character of user name (sid) string.
} DEVICE_USER_CHANGE_EVENT_DATA, *PDEVICE_USER_CHANGE_EVENT_DATA;


//////////////////////////////////////////////////////////////////////////////
//
// This struct contains an event sent by the driver to the platform. It tells the
// API that the device has new event data available to be retrieved
// It does not describe the type of events, nor the quantity of events available
//
//////////////////////////////////////////////////////////////////////////////
typedef struct _NEW_EVENT_DATA_AVAILABLE
{
    DWORD          cbNewEventDataAvailable; // Size of this structure
    DWORD          dwVersion; // Always set to VERSION_1_WINDOWS_7 in Windows 7
} NEW_EVENT_DATA_AVAILABLE, *PNEW_EVENT_DATA_AVAILABLE;


//////////////////////////////////////////////////////////////////////////////
//
// This is the header for all event data sent from a driver to the client API
//      via Authorized Eventing
// This will usually be followed by another struct that is specific to the guidEventType.
//      For Application Event GUID, the subsequent struct will be of type APPLICATION_EVENT_DATA
//      For Content Missing GUID, the subsequent struct will be of type CONTENT_MISSING_EVENT_DATA
// Immediately following this struct is a buffer of cbEventDataSid bytes (can
//      be 0). This buffer contains a SID (used for All User devices).
//
//////////////////////////////////////////////////////////////////////////////
typedef struct _EVENT_DATA_HEADER
{
    DWORD           cbEventDataHeader; // Size of this structure
    GUID            guidEventType; // The GUID of the event type. This could be {"Application Event", "Content Missing", "Device Added", "Device Removed"}
    DWORD           dwVersion; // Always set to VERSION_1_WINDOWS_7 in Windows 7
    DWORD           cbEventDataSid;
} EVENT_DATA_HEADER, *PEVENT_DATA_HEADER;


#pragma pack(pop, WindowsSideShowEvents)

#endif // (_WIN32_WINNT >= 0x0600)

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

