/**************************************************************************
*
* Copyright (c) 2000 Microsoft Corporation
*
* Module Name:
*
*   CachedBitmap class definition
*
* Abstract:
*
*   GDI+ CachedBitmap is a representation of an accelerated drawing
*   that has restrictions on what operations are allowed in order
*   to accelerate the drawing to the destination.
*
*   Look for class definition in GdiplusHeaders.h
*
**************************************************************************/

#ifndef _GDIPLUSCACHEDBITMAP_H
#define _GDIPLUSCACHEDBITMAP_H
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


inline 
CachedBitmap::CachedBitmap(
    IN Bitmap *bitmap, 
    IN Graphics *graphics)
{
    nativeCachedBitmap = NULL;    

    lastResult = DllExports::GdipCreateCachedBitmap(
        (GpBitmap *)bitmap->nativeImage,
        graphics->nativeGraphics,
        &nativeCachedBitmap
    );
}

inline 
CachedBitmap::~CachedBitmap()
{
    DllExports::GdipDeleteCachedBitmap(nativeCachedBitmap);
}

inline Status 
CachedBitmap::GetLastStatus() const 
{
    Status lastStatus = lastResult;
    lastResult = Ok;    
    return (lastStatus);
}


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif

