/////////////////////////////////////////////////////////////////////////////
//
// Copyright (c) Microsoft Corporation.  All rights reserved.
//
// Module Name:
//
//      Mpeg2Bits.h
//
// Abstract:
//
//      This file defines the MPEG-2 section header bitfields. These are
//      defined here instead of in mpegstructs.idl because of MIDL
//      compiler conflicts with bitfield definitions.
//
/////////////////////////////////////////////////////////////////////////////

#pragma once
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


#pragma pack(push)
#pragma pack(1)


//
// PID structure
//

#ifdef __midl

typedef struct
{
    WORD Bits;
} PID_BITS_MIDL;

#else

typedef struct
{
    WORD Reserved  :  3;
    WORD ProgramId : 13;
} PID_BITS, *PPID_BITS;

#endif



//
// Generic MPEG packet header structure
//

#ifdef __midl

typedef struct
{
    WORD Bits;
} MPEG_HEADER_BITS_MIDL;

#else

typedef struct
{
    WORD SectionLength          : 12;
    WORD Reserved               :  2;
    WORD PrivateIndicator       :  1;
    WORD SectionSyntaxIndicator :  1;
} MPEG_HEADER_BITS, *PMPEG_HEADER_BITS;

#endif



//
// Long MPEG packet header structure
//

#ifdef __midl

typedef struct
{
    BYTE Bits;
} MPEG_HEADER_VERSION_BITS_MIDL;

#else

typedef struct
{
    BYTE CurrentNextIndicator : 1;
    BYTE VersionNumber        : 5;
    BYTE Reserved             : 2;
} MPEG_HEADER_VERSION_BITS, *PMPEG_HEADER_VERSION_BITS;

#endif



#pragma pack(pop)

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

