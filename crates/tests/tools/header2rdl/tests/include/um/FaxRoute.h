/*++

Copyright (c) Microsoft Corporation.  All rights reserved.

THIS CODE AND INFORMATION IS PROVIDED "AS IS" WITHOUT WARRANTY OF ANY KIND, 
EITHER EXPRESSED OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE IMPLIED 
WARRANTIES OF MERCHANTABILITY AND/OR FITNESS FOR A PARTICULAR PURPOSE.

Module Name:

    faxroute.h

Abstract:

    This file contains the prototypes for the
    FAX routing extension API.


--*/

#include <commctrl.h>

#ifndef _FAXROUTE_
#define _FAXROUTE_

#if _MSC_VER > 1000
#pragma once
#endif
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


#ifdef __cplusplus
extern "C" {
#endif


//
// microsoft routing guids
//

#define MS_FAXROUTE_PRINTING_GUID               TEXT("{aec1b37c-9af2-11d0-abf7-00c04fd91a4e}")
#define MS_FAXROUTE_FOLDER_GUID                 TEXT("{92041a90-9af2-11d0-abf7-00c04fd91a4e}")
#define MS_FAXROUTE_EMAIL_GUID                  TEXT("{6bbf7bfe-9af2-11d0-abf7-00c04fd91a4e}")

//
// callback routines
//

typedef LONG (WINAPI *PFAXROUTEADDFILE)(DWORD JobId, LPCWSTR FileName, GUID *Guid);
typedef LONG (WINAPI *PFAXROUTEDELETEFILE)(DWORD JobId, LPCWSTR FileName);
typedef BOOL (WINAPI *PFAXROUTEGETFILE)(DWORD JobId, DWORD Index, _Out_writes_bytes_opt_(*RequiredSize) LPWSTR FileNameBuffer, LPDWORD RequiredSize);
typedef BOOL (WINAPI *PFAXROUTEENUMFILE)(DWORD JobId, GUID *GuidOwner, GUID *GuidCaller, LPCWSTR FileName, PVOID Context);
typedef BOOL (WINAPI *PFAXROUTEENUMFILES)(DWORD JobId, GUID *Guid,PFAXROUTEENUMFILE FileEnumerator, PVOID Context);
typedef BOOL (WINAPI *PFAXROUTEMODIFYROUTINGDATA)(DWORD JobId, LPCWSTR RoutingGuid, LPBYTE RoutingData, DWORD RoutingDataSize );


typedef struct _FAX_ROUTE_CALLBACKROUTINES {
    DWORD                       SizeOfStruct;                // size of the struct set by the fax service
    PFAXROUTEADDFILE            FaxRouteAddFile;
    PFAXROUTEDELETEFILE         FaxRouteDeleteFile;
    PFAXROUTEGETFILE            FaxRouteGetFile;
    PFAXROUTEENUMFILES          FaxRouteEnumFiles;
    PFAXROUTEMODIFYROUTINGDATA  FaxRouteModifyRoutingData;
} FAX_ROUTE_CALLBACKROUTINES, *PFAX_ROUTE_CALLBACKROUTINES;


//
// routing data structure
//

typedef struct _FAX_ROUTE {
    DWORD           SizeOfStruct;       // size of the struct set by the fax service
    DWORD           JobId;              // Fax job identifier
    DWORDLONG       ElapsedTime;        // Elapsed time for whole fax job in Coordinated Universal Time (UTC)
    DWORDLONG       ReceiveTime;        // Starting time for fax job in Coordinated Universal Time (UTC)
    DWORD           PageCount;          // Number of pages received
    LPCWSTR         Csid;               // Called station identifier
    LPCWSTR         Tsid;               // Transmitting station identifier
    LPCWSTR         CallerId;           // Caller id
    LPCWSTR         RoutingInfo;        // Routing info such as DID, T.30-subaddress, etc.
    LPCWSTR         ReceiverName;       // Receiver's name
    LPCWSTR         ReceiverNumber;     // Receiver's fax number
    LPCWSTR         DeviceName;         // Device name for the line that received the fax
    DWORD           DeviceId;           // Permenant line identifier for the receiving device
    LPBYTE          RoutingInfoData;    // Routing infor data to override configured info
    DWORD           RoutingInfoDataSize;// Size of routing info data
} FAX_ROUTE, *PFAX_ROUTE;

enum FAXROUTE_ENABLE {
    QUERY_STATUS = -1,
    STATUS_DISABLE = 0,
    STATUS_ENABLE = 1
};


//
// prototypes
//

BOOL WINAPI
FaxRouteInitialize(
    IN HANDLE HeapHandle,
    IN PFAX_ROUTE_CALLBACKROUTINES FaxRouteCallbackRoutines
    );

BOOL WINAPI
FaxRouteDeviceEnable(
    IN  LPCWSTR RoutingGuid,
    IN  DWORD DeviceId,
    IN  LONG Enabled
    );

BOOL WINAPI
FaxRouteDeviceChangeNotification(
    IN  DWORD DeviceId,
    IN  BOOL  NewDevice
    );

BOOL WINAPI
FaxRouteGetRoutingInfo(
    IN  LPCWSTR RoutingGuid,
    IN  DWORD DeviceId,
    IN  LPBYTE RoutingInfo,
    OUT LPDWORD RoutingInfoSize
    );

BOOL WINAPI
FaxRouteSetRoutingInfo(
    IN  LPCWSTR RoutingGuid,
    IN  DWORD DeviceId,
    IN  const BYTE *RoutingInfo,
    IN  DWORD RoutingInfoSize
    );


typedef BOOL (WINAPI *PFAXROUTEINITIALIZE)               (HANDLE,PFAX_ROUTE_CALLBACKROUTINES);
typedef BOOL (WINAPI *PFAXROUTEMETHOD)                   (const FAX_ROUTE*,PVOID*,LPDWORD);
typedef BOOL (WINAPI *PFAXROUTEDEVICEENABLE)             (LPCWSTR,DWORD,LONG);
typedef BOOL (WINAPI *PFAXROUTEDEVICECHANGENOTIFICATION) (DWORD,BOOL);
typedef BOOL (WINAPI *PFAXROUTEGETROUTINGINFO)           (LPCWSTR,DWORD,LPBYTE,LPDWORD);
typedef BOOL (WINAPI *PFAXROUTESETROUTINGINFO)           (LPCWSTR,DWORD,const BYTE*,DWORD);

#ifdef __cplusplus
}
#endif


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif
