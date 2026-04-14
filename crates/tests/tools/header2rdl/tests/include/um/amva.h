//------------------------------------------------------------------------------
// File: AMVA.h
//
// Desc: DirectShowMotionComp include file.
//
// Copyright (c) 1997 - 2001, Microsoft Corporation.  All rights reserved.
//------------------------------------------------------------------------------


#ifndef __AMVA_INCLUDED__
#define __AMVA_INCLUDED__
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


#ifdef __cplusplus
extern "C" {
#endif


#define AMVA_TYPEINDEX_OUTPUTFRAME 0xFFFFFFFF

//  Flags for QueryRenderStatus
#define AMVA_QUERYRENDERSTATUSF_READ     0x00000001  // Query for read
                                                     // set this bit to 0
                                                     // if query for update

typedef struct _tag_AMVAUncompBufferInfo
{
 DWORD                   dwMinNumSurfaces;           // IN   min number of surfaces to be allocated
 DWORD                   dwMaxNumSurfaces;           // IN   max number of surfaces to be allocated
 DDPIXELFORMAT           ddUncompPixelFormat;        // IN   pixel format of surfaces to be allocated
} AMVAUncompBufferInfo, *LPAMVAUncompBufferInfo;

typedef struct _tag_AMVAUncompDataInfo
{
    DWORD                   dwUncompWidth;              // [in]     width of uncompressed data
    DWORD                   dwUncompHeight;             // [in]     height of uncompressed data
    DDPIXELFORMAT           ddUncompPixelFormat;        // [in]     pixel-format of uncompressed data
} AMVAUncompDataInfo, *LPAMVAUncompDataInfo;

typedef struct _tag_AMVAInternalMemInfo
{
    DWORD                   dwScratchMemAlloc;          // [out]    amount of scratch memory will the hal allocate for its private use
} AMVAInternalMemInfo, *LPAMVAInternalMemInfo;


typedef struct _tag_AMVACompBufferInfo
{
    DWORD                   dwNumCompBuffers;           // [out]    number of buffers reqd for compressed data
    DWORD                   dwWidthToCreate;            // [out]    Width of surface to create
    DWORD                   dwHeightToCreate;           // [out]    Height of surface to create
    DWORD                   dwBytesToAllocate;          // [out]    Total number of bytes used by each surface
    DDSCAPS2                ddCompCaps;                 // [out]    caps to create surfaces to store compressed data
    DDPIXELFORMAT           ddPixelFormat;              // [out]    fourcc to create surfaces to store compressed data
} AMVACompBufferInfo, *LPAMVACompBufferInfo;


// Note that you are NOT allowed to store any pointer in pMiscData
typedef struct _tag_AMVABeginFrameInfo
{
    DWORD                dwDestSurfaceIndex;         // IN  destination buffer in which to decoding this frame
    LPVOID               pInputData;                 // IN  pointer to misc data
    DWORD                dwSizeInputData;            // IN  size of other misc data to begin frame
    LPVOID               pOutputData;                // OUT pointer to data which the VGA is going to fill
    DWORD                dwSizeOutputData;           // IN  size of data which the VGA is going to fill
} AMVABeginFrameInfo, *LPAMVABeginFrameInfo;

// Note that you are NOT allowed to store any pointer in pMiscData
typedef struct _tag_AMVAEndFrameInfo
{
    DWORD                   dwSizeMiscData;             // [in]     size of other misc data to begin frame
    LPVOID                  pMiscData;                  // [in]     pointer to misc data
} AMVAEndFrameInfo, *LPAMVAEndFrameInfo;

typedef struct _tag_AMVABUFFERINFO
{
    DWORD                   dwTypeIndex;                // [in]    Type of buffer
    DWORD                   dwBufferIndex;              // [in]    Buffer index
    DWORD                   dwDataOffset;               // [in]    offset of relevant data from the beginning of buffer
    DWORD                   dwDataSize;                 // [in]    size of relevant data
} AMVABUFFERINFO, *LPAMVABUFFERINFO;

#ifdef __cplusplus
};
#endif


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif // _AMVA_INCLUDED
