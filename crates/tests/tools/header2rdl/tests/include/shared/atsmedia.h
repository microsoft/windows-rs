//------------------------------------------------------------------------------
// File: ATSMedia.h
//
// Desc: Broadcast Driver Architecture Media Definitions for ATSC
//
// Copyright (c) 1996 - 2001, Microsoft Corporation.  All rights reserved.
//------------------------------------------------------------------------------


#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

#if !defined(_KSMEDIA_)
#error KSMEDIA.H must be included before BDAMEDIA.H
#endif // !defined(_KSMEDIA_)

#if !defined(_BDAMEDIA_)
#error BDAMEDIA.H must be included before ATSCMEDIA.H
#endif // !defined(_KSMEDIA_)

#if !defined(_ATSCMEDIA_)
#define _ATSCMEDIA_


//===========================================================================
//
//  ATSC Network Type
//
//===========================================================================

#define STATIC_BDANETWORKTYPE_ATSC\
    0x71985f51, 0x1ca1, 0x11d3, 0x9c, 0xc8, 0x0, 0xc0, 0x4f, 0x79, 0x71, 0xe0
DEFINE_GUIDSTRUCT("71985F51-1CA1-11d3-9CC8-00C04F7971E0", BDANETWORKTYPE_ATSC);
#define BDANETWORKTYPE_ATSC DEFINE_GUIDNAMED(BDANETWORKTYPE_ATSC)


#endif // _ATSCMEDIA_

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

