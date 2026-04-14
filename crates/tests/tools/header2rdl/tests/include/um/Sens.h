/*++

Copyright (c) Microsoft Corporation. All rights reserved.

Module Name:

    sens.h

Abstract:

    This file is the master header file for Event System events published
    and subscribed by the System Event Notification service (SENS).

Author:

    Gopal Parupudi    <GopalP>

[Notes:]

    optional-notes

Revision History:

    GopalP          12/8/1997         Start.

--*/


#ifndef __SENS_H__
#define __SENS_H__

#if _MSC_VER > 1000
#pragma once
#endif

#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

//
// Constants
//

#define CONNECTION_LAN   0x00000001
#define CONNECTION_WAN   0x00000002
#define CONNECTION_AOL   0x00000004




//
// SENS Guids related to Event System
//


DEFINE_GUID(
    SENSGUID_PUBLISHER,             /* 5fee1bd6-5b9b-11d1-8dd2-00aa004abd5e */
    0x5fee1bd6,
    0x5b9b,
    0x11d1,
    0x8d, 0xd2, 0x00, 0xaa, 0x00, 0x4a, 0xbd, 0x5e
);

DEFINE_GUID(
    SENSGUID_SUBSCRIBER_LCE,        /* d3938ab0-5b9d-11d1-8dd2-00aa004abd5e */
    0xd3938ab0,
    0x5b9d,
    0x11d1,
    0x8d, 0xd2, 0x00, 0xaa, 0x00, 0x4a, 0xbd, 0x5e
);

DEFINE_GUID(
    SENSGUID_SUBSCRIBER_WININET,    /* d3938ab5-5b9d-11d1-8dd2-00aa004abd5e */
    0xd3938ab5,
    0x5b9d,
    0x11d1,
    0x8d, 0xd2, 0x00, 0xaa, 0x00, 0x4a, 0xbd, 0x5e
);





//
// Classes of Events published by SENS
//

DEFINE_GUID(
    SENSGUID_EVENTCLASS_NETWORK,        /* d5978620-5b9f-11d1-8dd2-00aa004abd5e */
    0xd5978620,
    0x5b9f,
    0x11d1,
    0x8d, 0xd2, 0x00, 0xaa, 0x00, 0x4a, 0xbd, 0x5e
);

DEFINE_GUID(
    SENSGUID_EVENTCLASS_LOGON,          /* d5978630-5b9f-11d1-8dd2-00aa004abd5e */
    0xd5978630,
    0x5b9f,
    0x11d1,
    0x8d, 0xd2, 0x00, 0xaa, 0x00, 0x4a, 0xbd, 0x5e
);

DEFINE_GUID(
    SENSGUID_EVENTCLASS_ONNOW,          /* d5978640-5b9f-11d1-8dd2-00aa004abd5e */
    0xd5978640,
    0x5b9f,
    0x11d1,
    0x8d, 0xd2, 0x00, 0xaa, 0x00, 0x4a, 0xbd, 0x5e
);

DEFINE_GUID(
    SENSGUID_EVENTCLASS_LOGON2,         /* d5978650-5b9f-11d1-8dd2-00aa004abd5e */
    0xd5978650,
    0x5b9f,
    0x11d1,
    0x8d, 0xd2, 0x00, 0xaa, 0x00, 0x4a, 0xbd, 0x5e
);


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif // __SENS_H__
