/**************************************************************************\
*
* Copyright (c) 1998-2001, Microsoft Corp.  All Rights Reserved.
*
* Module Name:
*
*   GdiplusColorMatrix.h
*
* Abstract:
*
*  GDI+ Color Matrix object, used with Graphics.DrawImage
*
\**************************************************************************/

#ifndef _GDIPLUSCOLORMATRIX_H
#define _GDIPLUSCOLORMATRIX_H
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


#if (GDIPVER >= 0x0110)
//----------------------------------------------------------------------------
// Color channel look up table (LUT)
//----------------------------------------------------------------------------

typedef BYTE ColorChannelLUT[256];

//----------------------------------------------------------------------------
// Per-channel Histogram for 8bpp images.
//----------------------------------------------------------------------------

enum HistogramFormat
{
    HistogramFormatARGB,
    HistogramFormatPARGB,
    HistogramFormatRGB,
    HistogramFormatGray,
    HistogramFormatB,
    HistogramFormatG,
    HistogramFormatR,
    HistogramFormatA    
};
#endif //(GDIPVER >= 0x0110)

//----------------------------------------------------------------------------
// Color matrix
//----------------------------------------------------------------------------

struct ColorMatrix
{
    REAL m[5][5];
};

//----------------------------------------------------------------------------
// Color Matrix flags
//----------------------------------------------------------------------------

enum ColorMatrixFlags
{
    ColorMatrixFlagsDefault   = 0,
    ColorMatrixFlagsSkipGrays = 1,
    ColorMatrixFlagsAltGray   = 2
};

//----------------------------------------------------------------------------
// Color Adjust Type
//----------------------------------------------------------------------------

enum ColorAdjustType
{
    ColorAdjustTypeDefault,
    ColorAdjustTypeBitmap,
    ColorAdjustTypeBrush,
    ColorAdjustTypePen,
    ColorAdjustTypeText,
    ColorAdjustTypeCount,
    ColorAdjustTypeAny      // Reserved
};

//----------------------------------------------------------------------------
// Color Map
//----------------------------------------------------------------------------

struct ColorMap
{
    Color oldColor;
    Color newColor;
};


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif
