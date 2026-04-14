/********************************************************
*                                                       *
*   Copyright (C) Microsoft. All rights reserved.       *
*                                                       *
********************************************************/

//-----------------------------------------------------------------------------
// File:			IDLMULTI.H
//
// Contents: 		preprocessor trickery to make our .idl/.tdl files compile
//					with MIDL or APBU Mktyplib.
//
// Comments: 		
//
//-----------------------------------------------------------------------------

#ifndef __IDLMULTI_H__
#define __IDLMULTI_H__
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


#define ENDCOCLASS  };


#ifndef __MKTYPLIB__
#define TYPEDEF(guid)   \
typedef

#define LOCAL_INTERFACE(guid)       \
[                                   \
    local,                          \
    object,                         \
    uuid(guid),                     \
    pointer_default(unique)         \
]


#define REMOTED_INTERFACE(guid)     \
[                                   \
    object,                         \
    uuid(guid),                     \
    pointer_default(unique)         \
]


#define BEGINEVENTSET(es)   \
eventset es  \
{            \

#define ENDEVENTSET  };


#define COCLASS(name, dispint, events)  \
cotype name                                                 \
{                                                           \
    dispinterface dispint;                                  \
    eventset events;



#else // __MKTYPLIB__

#define TYPEDEF(guid) typedef [uuid(guid)]

#define cpp_quote(string)

#define const

#define LOCAL_INTERFACE(guid)    \
[                           \
    uuid(guid),             \
    odl                     \
]

#define REMOTED_INTERFACE(guid)    \
[                           \
    uuid(guid),             \
    odl                     \
]

#define BEGINEVENTSET(es)  \
dispinterface es           \
{                          \
properties:                \
                           \
methods:                   \

#define ENDEVENTSET  };


#define COCLASS(name, dispint, events)                 \
coclass name                                                \
{                                                           \
    dispinterface dispint;                                  \
    [source,] dispinterface events;                         \



#endif  /// MKTYPLIB


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif  // __IDLMULTI_H__

