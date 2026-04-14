// Copyright (C) Microsoft Corporation.  All rights reserved.

#ifndef __BTHSDPDEF_H__
#define __BTHSDPDEF_H__
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

#if (NTDDI_VERSION >= NTDDI_WINXPSP2)

#ifdef __cplusplus
extern "C" {
#endif

typedef struct SDP_LARGE_INTEGER_16
{
    ULONGLONG LowPart;
    LONGLONG HighPart;
} SDP_LARGE_INTEGER_16, *PSDP_LARGE_INTEGER_16, *LPSDP_LARGE_INTEGER_16;
 
typedef struct SDP_ULARGE_INTEGER_16
{
    ULONGLONG LowPart;
    ULONGLONG HighPart;
} SDP_ULARGE_INTEGER_16, *PSDP_ULARGE_INTEGER_16, *LPSDP_ULARGE_INTEGER_16;

typedef enum NodeContainerType
{
    NodeContainerTypeSequence,
    NodeContainerTypeAlternative
} NodeContainerType;

typedef USHORT SDP_ERROR, *PSDP_ERROR;

// 9 - 31 are reserved
typedef enum SDP_TYPE
{
    SDP_TYPE_NIL =  0x00,
    SDP_TYPE_UINT = 0x01,
    SDP_TYPE_INT = 0x02,
    SDP_TYPE_UUID = 0x03,
    SDP_TYPE_STRING = 0x04,
    SDP_TYPE_BOOLEAN = 0x05,
    SDP_TYPE_SEQUENCE = 0x06,
    SDP_TYPE_ALTERNATIVE = 0x07,
    SDP_TYPE_URL = 0x08,
    SDP_TYPE_CONTAINER = 0x20
} SDP_TYPE;

// allow for a little easier type checking / sizing for integers and UUIDs
// ((SDP_ST_XXX & 0xF0) >> 4) == SDP_TYPE_XXX
// size of the data (in bytes) is encoded as ((SDP_ST_XXX & 0xF0) >> 8)
typedef enum SDP_SPECIFICTYPE
{
    SDP_ST_NONE = 0x0000,

    SDP_ST_UINT8 = 0x0010,
    SDP_ST_UINT16 = 0x0110,
    SDP_ST_UINT32 = 0x0210,
    SDP_ST_UINT64 = 0x0310,
    SDP_ST_UINT128 = 0x0410,
    
    SDP_ST_INT8 = 0x0020,
    SDP_ST_INT16 = 0x0120,
    SDP_ST_INT32 = 0x0220,
    SDP_ST_INT64 = 0x0320,
    SDP_ST_INT128 = 0x0420,
    
    SDP_ST_UUID16 = 0x0130,
    SDP_ST_UUID32 = 0x0220,
    SDP_ST_UUID128 = 0x0430
} SDP_SPECIFICTYPE;

typedef struct SdpAttributeRange
{
    USHORT minAttribute;
    USHORT maxAttribute;
} SdpAttributeRange;

typedef
#ifdef MIDL_PASS 
      [switch_type(unsigned short)]
#endif
union SdpQueryUuidUnion {
#ifdef MIDL_PASS 
    [case(SDP_ST_UUID128)]
#endif
       GUID uuid128;

#ifdef MIDL_PASS 
    [case(SDP_ST_UUID32)] 
#endif // _NTDDK_
       ULONG uuid32;

#ifdef MIDL_PASS 
    [case(SDP_ST_UUID16)]
#endif // _NTDDK_
        USHORT uuid16;
} SdpQueryUuidUnion;

typedef struct _SdpQueryUuid {
#ifdef MIDL_PASS 
    [switch_is(uuidType)]
#endif
       SdpQueryUuidUnion u;

    USHORT uuidType;
} SdpQueryUuid;


#ifdef __cplusplus
};
#endif

#endif // (NTDDI_VERSION >= NTDDI_WINXPSP2)

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif // __BTHSDPDEF_H__
