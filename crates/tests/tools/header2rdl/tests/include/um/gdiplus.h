/**************************************************************************\
*
* Copyright (c) 1998-2001, Microsoft Corp.  All Rights Reserved.
*
* Module Name:
*
*   Gdiplus.h
*
* Abstract:
*
*   GDI+ public header file
*
\**************************************************************************/

#ifndef _GDIPLUS_H
#define _GDIPLUS_H
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


struct IDirectDrawSurface7;

typedef signed   short   INT16;
typedef unsigned short  UINT16;


// Define the Current GDIPlus Version
#ifndef GDIPVER
#define GDIPVER 0x0100
#endif

#include <pshpack8.h>   // set structure packing to 8

namespace Gdiplus
{
    namespace DllExports
    {
        #include "GdiplusMem.h"
    };

    #include "GdiplusBase.h"

    #include "GdiplusEnums.h"
    #include "GdiplusTypes.h"
    #include "GdiplusInit.h"
    #include "GdiplusPixelFormats.h"
    #include "GdiplusColor.h"
    #include "GdiplusMetaHeader.h"
    #include "GdiplusImaging.h"
    #include "GdiplusColorMatrix.h"
#if (GDIPVER >= 0x0110)    
    #include "GdiplusEffects.h"
#endif
    #include "GdiplusGpStubs.h"
    #include "GdiplusHeaders.h"

    namespace DllExports
    {
        #include "GdiplusFlat.h"
    };


    #include "GdiplusImageAttributes.h"
    #include "GdiplusMatrix.h"
    #include "GdiplusBrush.h"
    #include "GdiplusPen.h"
    #include "GdiplusStringFormat.h"
    #include "GdiplusPath.h"
    #include "GdiplusLineCaps.h"
    #include "GdiplusGraphics.h"
    #include "GdiplusMetafile.h"
    #include "GdiplusCachedBitmap.h"
    #include "GdiplusRegion.h"
    #include "GdiplusFontCollection.h"
    #include "GdiplusFontFamily.h"
    #include "GdiplusFont.h"
    #include "GdiplusBitmap.h"
    #include "GdiplusImageCodec.h"

}; // namespace Gdiplus

#include <poppack.h>    // pop structure packing back to previous state


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif // !_GDIPLUS_HPP
