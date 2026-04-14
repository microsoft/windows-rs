/**************************************************************************\
*
* Copyright (c) 1998-2001, Microsoft Corp.  All Rights Reserved.
*
* Module Name:
*
*   GdiplusMem.h
*
* Abstract:
*
*   GDI+ Private Memory Management APIs
*
\**************************************************************************/

#ifndef _GDIPLUSMEM_H
#define _GDIPLUSMEM_H
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


#ifdef __cplusplus
extern "C" {
#endif

#define WINGDIPAPI __stdcall

//----------------------------------------------------------------------------
// Memory Allocation APIs
//----------------------------------------------------------------------------

void* WINGDIPAPI
GdipAlloc(size_t size);

void WINGDIPAPI
GdipFree(void* ptr);

#ifdef __cplusplus
}
#endif


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif // !_GDIPLUSMEM_H
