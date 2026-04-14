//------------------------------------------------------------------------------
// File: AMParse.h
//
// Desc: Interface to the parser to get current time.  This is useful for
//       multifile playback.
//
// Copyright (c) 1996 - 2001, Microsoft Corporation.  All rights reserved.
//------------------------------------------------------------------------------


#ifndef __AMPARSE__
#define __AMPARSE__
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


#ifdef __cplusplus
extern "C" {
#endif // __cplusplus


DEFINE_GUID(IID_IAMParse,
0xc47a3420, 0x005c, 0x11d2, 0x90, 0x38, 0x00, 0xa0, 0xc9, 0x69, 0x72, 0x98);

//
//  Parser interface - supported by MPEG-2 splitter filter
//
DECLARE_INTERFACE_(IAMParse, IUnknown) {
    STDMETHOD(GetParseTime) (THIS_
                             _Out_ REFERENCE_TIME *prtCurrent
                            ) PURE;
    STDMETHOD(SetParseTime) (THIS_
                             REFERENCE_TIME rtCurrent
                            ) PURE;
    STDMETHOD(Flush) (THIS) PURE;
};

#ifdef __cplusplus
}
#endif // __cplusplus

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif // __AMPARSE__
