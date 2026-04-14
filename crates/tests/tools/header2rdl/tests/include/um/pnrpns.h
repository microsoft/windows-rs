/*++

Copyright (c) Microsoft Corporation.  All rights reserved.

Module Name:

    pnrpns.h

Abstract:
    The PNRP interface participates as a WinSock 2.0 Name Space Provider (NSP)
    The actual functions are defined in winsock2.h of the platform SDK

    This header file defines the specific structures and data used for PNRP.

--*/

#ifndef _PNRPNS_H_
#define _PNRPNS_H_
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


#include "pnrpdef.h"

// Namespaces
#ifndef NS_PNRPNAME
#define NS_PNRPNAME     (38) 
#endif

#ifndef NS_PNRPCLOUD
#define NS_PNRPCLOUD    (39)
#endif

//
// Bit values used for dwFlags in PNRPINFO
//

#define PNRPINFO_HINT           0x00000001      // set if Hint valid

//
// PNRP specific data that is referenced by the lpBlob pointer of WSAQUERYSET
//


#pragma warning(push)
#pragma warning(disable:4201) // nameless struct/union


typedef struct _PNRPINFO_V1
{
    DWORD                       dwSize;                 // size of this struct
    LPWSTR                      lpwszIdentity;          // identity name string
    DWORD                       nMaxResolve;            // number of desired resolutions
    DWORD                       dwTimeout;              // time in seconds to wait for responses
    DWORD                       dwLifetime;             // time in seconds for validity
    PNRP_RESOLVE_CRITERIA       enResolveCriteria;      // type of matching for resolves
    DWORD                       dwFlags;                // set of flags
    SOCKET_ADDRESS              saHint;                 // service location part of an ID
    PNRP_REGISTERED_ID_STATE    enNameState;            // state of registered name

} PNRPINFO_V1, * PPNRPINFO_V1;



typedef struct _PNRPINFO_V2
{
    DWORD                       dwSize;                 // size of this struct
    LPWSTR                      lpwszIdentity;          // identity name string
    DWORD                       nMaxResolve;            // number of desired resolutions
    DWORD                       dwTimeout;              // time in seconds to wait for responses
    DWORD                       dwLifetime;             // time in seconds for validity
    PNRP_RESOLVE_CRITERIA       enResolveCriteria;      // type of matching for resolves
    DWORD                       dwFlags;                // set of flags
    SOCKET_ADDRESS              saHint;                 // service location part of an ID
    PNRP_REGISTERED_ID_STATE    enNameState;            // state of registered name
    PNRP_EXTENDED_PAYLOAD_TYPE  enExtendedPayloadType;
    union {
        BLOB                    blobPayload;
        PWSTR                   pwszPayload;
    };
   
} PNRPINFO_V2, * PPNRPINFO_V2;



#if defined(PNRP_USE_V1_API)

typedef  PNRPINFO_V1 PNRPINFO;
typedef  PPNRPINFO_V1 PPNRPINFO;


#else
typedef  PNRPINFO_V2 PNRPINFO;
typedef  PPNRPINFO_V2 PPNRPINFO;

#endif
//
// Cloud specific data that is referenced by the lpBlob pointer of WSAQUERYSET
//

typedef struct _PNRPCLOUDINFO
{
    DWORD                       dwSize;                 // size of this struct
    PNRP_CLOUD_ID               Cloud;                  // network cloud information
    PNRP_CLOUD_STATE            enCloudState;           // state of cloud
    PNRP_CLOUD_FLAGS            enCloudFlags;           // flags for cloud
                                                        //   PNRP_CLOUD_NAME_LOCAL - not a network name
} PNRPCLOUDINFO, * PPNRPCLOUDINFO;


#pragma warning(pop)


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif // _PNRPNS_H_



/////////////////////////////////////////////////////////////////////////////
//
// These GUIDs are outside conditional includes so you can
//   #include <pnrpns.h>   in precompiled header
// then
//   #include <initguid.h>  in a single source file
//   #include <pnrpns.h>   in that source file a second time to instantiate the GUIDs

#ifdef DEFINE_GUID

// NSP Provider GUID
DEFINE_GUID(NS_PROVIDER_PNRPNAME,   0x03fe89cd, 0x766d, 0x4976, 0xb9, 0xc1, 0xbb, 0x9b, 0xc4, 0x2c, 0x7b, 0x4d);
DEFINE_GUID(NS_PROVIDER_PNRPCLOUD,  0x03fe89ce, 0x766d, 0x4976, 0xb9, 0xc1, 0xbb, 0x9b, 0xc4, 0x2c, 0x7b, 0x4d);

// Service Class GUID (specific to PNRP)
DEFINE_GUID(SVCID_PNRPCLOUD,    0xc2239ce6, 0x00c0, 0x4fbf, 0xba, 0xd6, 0x18, 0x13, 0x93, 0x85, 0xa4, 0x9a);

DEFINE_GUID(SVCID_PNRPNAME_V1,	0xc2239ce5, 0x00c0, 0x4fbf, 0xba, 0xd6, 0x18, 0x13, 0x93, 0x85, 0xa4, 0x9a);
DEFINE_GUID(SVCID_PNRPNAME_V2,	0xc2239ce7, 0x00c0, 0x4fbf, 0xba, 0xd6, 0x18, 0x13, 0x93, 0x85, 0xa4, 0x9a);


#if defined(PNRP_USE_V2_API)

#define SVCID_PNRPNAME SVCID_PNRPNAME_V2

#else

#define SVCID_PNRPNAME SVCID_PNRPNAME_V1

#endif

#endif // DEFINE_GUID
