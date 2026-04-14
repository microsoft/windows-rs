//------------------------------------------------------------------------------
// File: DMORt.h
//
// Desc: Miscellaneous runtime support for DirectShow Media Objects
//
// Copyright (c) 1999 - 2001, Microsoft Corporation.  All rights reserved.
//------------------------------------------------------------------------------


#ifndef __DMORT_H__
#define __DMORT_H__
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


//
// Mediatype helpers.  MoInitMediaType() goes with MoFreeMediaType(), and
// MoCreateMediaType() goes with MoDeleteMediaType().  Don't mix them!
//



//
// Takes a pointer to an already allocated DMO_MEDIA_TYPE structure, allocates
// a format block of cbFormat bytes, and sets appropriate members of
// DMO_MEDIA_TYPE to point to the newly allocated format block.  Also
// initializes the IUnknown pointer inside DMO_MEDIA_TYPE to NULL.
//
// The format block allocated by MoInitMediaType must be freed by calling
// MoFreeMediaType().
//
STDAPI MoInitMediaType(DMO_MEDIA_TYPE *pmt, DWORD cbFormat);

//
// Frees the format block and releases any IUnknown, but does not free the
// DMO_MEDIA_TYPE structure itself.  Input parameter must point to an
// DMO_MEDIA_TYPE structure previously initialized by MoInitMediaType().
//
STDAPI MoFreeMediaType(DMO_MEDIA_TYPE *pmt);

//
// Copies the DMO_MEDIA_TYPE members.  Also duplicates the format block and
// the IUnknown pointer.  Both parameters must point to valid DMO_MEDIA_TYPE
// structures.  Target structure must be later freed using MoFreeMediaType().
//
STDAPI MoCopyMediaType(DMO_MEDIA_TYPE *pmtDest, const DMO_MEDIA_TYPE *pmtSrc);



//
// Allocates a new DMO_MEDIA_TYPE structure and initializes it just like
// MoInitMediaType.  I.e., this function allocates both the format block
// and the DMO_MEDIA_TYPE structure itself.  Pointer to DMO_MEDIA_TYPE is
// returned as *ppmt.
//
// DMO_MEDIA_TYPE structures allocated by MoCreateMediaType() must be freed
// by calling MoDeleteMediaType().
//
STDAPI MoCreateMediaType(DMO_MEDIA_TYPE **ppmt, DWORD cbFormat);

//
// Frees any format block, releases any IUnknown, and deletes the
// DMO_MEDIA_TYPE structure itself.  The input parameter must point to an
// DMO_MEDIA_TYPE structure previously allocated by MoCreateMediaType().
//
STDAPI MoDeleteMediaType(DMO_MEDIA_TYPE *pmt);

//
// Allocates a new DMO_MEDIA_TYPE structure and copies pmtSrc into it like
// MoCopyMediaType.  I.e., this function allocates a new DMO_MEDIA_TYPE struct
// as well as a new format block for the target mediatype.  Trager mediatype
// must later be freed using MoDeleteMediaType().
//
STDAPI MoDuplicateMediaType(DMO_MEDIA_TYPE **ppmtDest, const DMO_MEDIA_TYPE *pmtSrc);




#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif //__DMORT_H__
