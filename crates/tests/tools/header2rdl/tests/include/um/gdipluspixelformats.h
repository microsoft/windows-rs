/**************************************************************************\
*
* Copyright (c) 1998-2001, Microsoft Corp.  All Rights Reserved.
*
* Module Name:
*
*   Gdiplus Pixel Formats
*
* Abstract:
*
*   GDI+ Pixel Formats
*
\**************************************************************************/

#ifndef _GDIPLUSPIXELFORMATS_H
#define _GDIPLUSPIXELFORMATS_H
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

#if _MSC_VER >= 1200
#pragma warning(push)
#pragma warning(disable:4001) /* nonstandard extension: single line comment */
#pragma warning(disable:4263) /* member function does not override any base class virtual member function */
#pragma warning(disable:4264) /* no override available for virtual member function from base 'class'; function is hidden */
#if _MSC_VER >= 1400
#pragma warning(disable:4365) /* signed/unsigned mismatch */
#endif
#endif

typedef DWORD ARGB;
typedef DWORDLONG ARGB64;

#define ALPHA_SHIFT 24
#define RED_SHIFT   16
#define GREEN_SHIFT 8
#define BLUE_SHIFT  0
#define ALPHA_MASK  ((ARGB) 0xff << ALPHA_SHIFT)

// In-memory pixel data formats:
// bits 0-7 = format index
// bits 8-15 = pixel size (in bits)
// bits 16-23 = flags
// bits 24-31 = reserved

typedef INT PixelFormat;

#define    PixelFormatIndexed      0x00010000 // Indexes into a palette
#define    PixelFormatGDI          0x00020000 // Is a GDI-supported format
#define    PixelFormatAlpha        0x00040000 // Has an alpha component
#define    PixelFormatPAlpha       0x00080000 // Pre-multiplied alpha
#define    PixelFormatExtended     0x00100000 // Extended color 16 bits/channel
#define    PixelFormatCanonical    0x00200000 

#define    PixelFormatUndefined       0
#define    PixelFormatDontCare        0

#define    PixelFormat1bppIndexed     (1 | ( 1 << 8) | PixelFormatIndexed | PixelFormatGDI)
#define    PixelFormat4bppIndexed     (2 | ( 4 << 8) | PixelFormatIndexed | PixelFormatGDI)
#define    PixelFormat8bppIndexed     (3 | ( 8 << 8) | PixelFormatIndexed | PixelFormatGDI)
#define    PixelFormat16bppGrayScale  (4 | (16 << 8) | PixelFormatExtended)
#define    PixelFormat16bppRGB555     (5 | (16 << 8) | PixelFormatGDI)
#define    PixelFormat16bppRGB565     (6 | (16 << 8) | PixelFormatGDI)
#define    PixelFormat16bppARGB1555   (7 | (16 << 8) | PixelFormatAlpha | PixelFormatGDI)
#define    PixelFormat24bppRGB        (8 | (24 << 8) | PixelFormatGDI)
#define    PixelFormat32bppRGB        (9 | (32 << 8) | PixelFormatGDI)
#define    PixelFormat32bppARGB       (10 | (32 << 8) | PixelFormatAlpha | PixelFormatGDI | PixelFormatCanonical)
#define    PixelFormat32bppPARGB      (11 | (32 << 8) | PixelFormatAlpha | PixelFormatPAlpha | PixelFormatGDI)
#define    PixelFormat48bppRGB        (12 | (48 << 8) | PixelFormatExtended)
#define    PixelFormat64bppARGB       (13 | (64 << 8) | PixelFormatAlpha  | PixelFormatCanonical | PixelFormatExtended)
#define    PixelFormat64bppPARGB      (14 | (64 << 8) | PixelFormatAlpha  | PixelFormatPAlpha | PixelFormatExtended)
#define    PixelFormat32bppCMYK       (15 | (32 << 8))
#define    PixelFormatMax             16

inline UINT
GetPixelFormatSize(PixelFormat pixfmt)
{
    return (pixfmt >> 8) & 0xff;
}

inline BOOL
IsIndexedPixelFormat(PixelFormat pixfmt)
{
    return (pixfmt & PixelFormatIndexed) != 0;
}

inline BOOL
IsAlphaPixelFormat(PixelFormat pixfmt)
{
   return (pixfmt & PixelFormatAlpha) != 0;
}

inline BOOL
IsExtendedPixelFormat(PixelFormat pixfmt)
{
   return (pixfmt & PixelFormatExtended) != 0;
}

//--------------------------------------------------------------------------
// Determine if the Pixel Format is Canonical format:
//   PixelFormat32bppARGB
//   PixelFormat32bppPARGB
//   PixelFormat64bppARGB
//   PixelFormat64bppPARGB
//--------------------------------------------------------------------------

inline BOOL
IsCanonicalPixelFormat(PixelFormat pixfmt)
{
   return (pixfmt & PixelFormatCanonical) != 0;
}

#if (GDIPVER >= 0x0110)
//----------------------------------------------------------------------------
// Color format conversion parameters
//----------------------------------------------------------------------------

enum PaletteType
{
    // Arbitrary custom palette provided by caller.
    
    PaletteTypeCustom           = 0,
    
    // Optimal palette generated using a median-cut algorithm.
    
    PaletteTypeOptimal        = 1,
    
    // Black and white palette.
    
    PaletteTypeFixedBW          = 2,
    
    // Symmetric halftone palettes.
    // Each of these halftone palettes will be a superset of the system palette.
    // E.g. Halftone8 will have it's 8-color on-off primaries and the 16 system
    // colors added. With duplicates removed, that leaves 16 colors.
    
    PaletteTypeFixedHalftone8   = 3, // 8-color, on-off primaries
    PaletteTypeFixedHalftone27  = 4, // 3 intensity levels of each color
    PaletteTypeFixedHalftone64  = 5, // 4 intensity levels of each color
    PaletteTypeFixedHalftone125 = 6, // 5 intensity levels of each color
    PaletteTypeFixedHalftone216 = 7, // 6 intensity levels of each color

    // Assymetric halftone palettes.
    // These are somewhat less useful than the symmetric ones, but are 
    // included for completeness. These do not include all of the system
    // colors.
    
    PaletteTypeFixedHalftone252 = 8, // 6-red, 7-green, 6-blue intensities
    PaletteTypeFixedHalftone256 = 9, // 8-red, 8-green, 4-blue intensities
};

enum DitherType
{
    DitherTypeNone          = 0,
    
    // Solid color - picks the nearest matching color with no attempt to 
    // halftone or dither. May be used on an arbitrary palette.
    
    DitherTypeSolid         = 1,
    
    // Ordered dithers and spiral dithers must be used with a fixed palette.
    
    // NOTE: DitherOrdered4x4 is unique in that it may apply to 16bpp 
    // conversions also.
    
    DitherTypeOrdered4x4    = 2,
    
    DitherTypeOrdered8x8    = 3,
    DitherTypeOrdered16x16  = 4,
    DitherTypeSpiral4x4     = 5,
    DitherTypeSpiral8x8     = 6,
    DitherTypeDualSpiral4x4 = 7,
    DitherTypeDualSpiral8x8 = 8,
    
    // Error diffusion. May be used with any palette.
    
    DitherTypeErrorDiffusion   = 9,

    DitherTypeMax              = 10
};
#endif //(GDIPVER >= 0x0110)

enum PaletteFlags
{
    PaletteFlagsHasAlpha    = 0x0001,
    PaletteFlagsGrayScale   = 0x0002,
    PaletteFlagsHalftone    = 0x0004
};

struct ColorPalette
{

public:

    UINT Flags;             // Palette flags
    UINT Count;             // Number of color entries
    ARGB Entries[1];        // Palette color entries
};

#if _MSC_VER >= 1200
#pragma warning(pop)
#endif

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif
