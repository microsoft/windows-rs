#include <winapifamily.h>

/**************************************************************************
*                                                                         *
* wingdi.h -- GDI procedure declarations, constant definitions and macros *
*                                                                         *
* Copyright (c) Microsoft Corp. All rights reserved.                      *
*                                                                         *
**************************************************************************/

#ifndef _WINGDI_
#define _WINGDI_


#pragma once

#if _MSC_VER >= 1200
#pragma warning(push)
#pragma warning(disable: 4201)      // nonstandard extension used : nameless struct/union
#ifndef _MSC_EXTENSIONS
#pragma warning(disable: 4309)      // truncation of constant value
#endif
#pragma warning(disable: 4820)      // padding added after data member
#endif

#ifdef _MAC
#include <macwin32.h>
#endif


//
// Define API decoration for direct importing of DLL references.
//

#if !defined(_GDI32_)
#define WINGDIAPI DECLSPEC_IMPORT
#else
#define WINGDIAPI
#endif

//
// Define API decoration for direct importing of DLL references.
//

#if !defined(_SPOOL32_)
#define WINSPOOLAPI DECLSPEC_IMPORT
#else
#define WINSPOOLAPI
#endif

#ifdef __cplusplus
extern "C" {
#endif

#ifndef WINVER
#define WINVER 0x0500   // version 5.0
#endif /* WINVER */

#ifndef NOGDI

#ifndef NORASTEROPS

/* Binary raster ops */
#define R2_BLACK            1   /*  0       */
#define R2_NOTMERGEPEN      2   /* DPon     */
#define R2_MASKNOTPEN       3   /* DPna     */
#define R2_NOTCOPYPEN       4   /* PN       */
#define R2_MASKPENNOT       5   /* PDna     */
#define R2_NOT              6   /* Dn       */
#define R2_XORPEN           7   /* DPx      */
#define R2_NOTMASKPEN       8   /* DPan     */
#define R2_MASKPEN          9   /* DPa      */
#define R2_NOTXORPEN        10  /* DPxn     */
#define R2_NOP              11  /* D        */
#define R2_MERGENOTPEN      12  /* DPno     */
#define R2_COPYPEN          13  /* P        */
#define R2_MERGEPENNOT      14  /* PDno     */
#define R2_MERGEPEN         15  /* DPo      */
#define R2_WHITE            16  /*  1       */
#define R2_LAST             16

/* Ternary raster operations */
#define SRCCOPY             (DWORD)0x00CC0020 /* dest = source                   */
#define SRCPAINT            (DWORD)0x00EE0086 /* dest = source OR dest           */
#define SRCAND              (DWORD)0x008800C6 /* dest = source AND dest          */
#define SRCINVERT           (DWORD)0x00660046 /* dest = source XOR dest          */
#define SRCERASE            (DWORD)0x00440328 /* dest = source AND (NOT dest )   */
#define NOTSRCCOPY          (DWORD)0x00330008 /* dest = (NOT source)             */
#define NOTSRCERASE         (DWORD)0x001100A6 /* dest = (NOT src) AND (NOT dest) */
#define MERGECOPY           (DWORD)0x00C000CA /* dest = (source AND pattern)     */
#define MERGEPAINT          (DWORD)0x00BB0226 /* dest = (NOT source) OR dest     */
#define PATCOPY             (DWORD)0x00F00021 /* dest = pattern                  */
#define PATPAINT            (DWORD)0x00FB0A09 /* dest = DPSnoo                   */
#define PATINVERT           (DWORD)0x005A0049 /* dest = pattern XOR dest         */
#define DSTINVERT           (DWORD)0x00550009 /* dest = (NOT dest)               */
#define BLACKNESS           (DWORD)0x00000042 /* dest = BLACK                    */
#define WHITENESS           (DWORD)0x00FF0062 /* dest = WHITE                    */
#if(WINVER >= 0x0500)

#define NOMIRRORBITMAP               (DWORD)0x80000000 /* Do not Mirror the bitmap in this call */
#define CAPTUREBLT                   (DWORD)0x40000000 /* Include layered windows */
#endif /* WINVER >= 0x0500 */


/* Quaternary raster codes */
#define MAKEROP4(fore,back) (DWORD)((((back) << 8) & 0xFF000000) | (fore))

#endif /* NORASTEROPS */

#define GDI_ERROR (0xFFFFFFFFL)
#if (_WIN32_WINNT >= _WIN32_WINNT_WINXP)
#define HGDI_ERROR (LongToHandle(0xFFFFFFFFL))
#else
#define HGDI_ERROR ((HANDLE)-1)
#endif // (_WIN32_WINNT >= _WIN32_WINNT_WINXP)

/* Region Flags */
#define ERROR               0
#define NULLREGION          1
#define SIMPLEREGION        2
#define COMPLEXREGION       3
#define RGN_ERROR ERROR

/* CombineRgn() Styles */
#define RGN_AND             1
#define RGN_OR              2
#define RGN_XOR             3
#define RGN_DIFF            4
#define RGN_COPY            5
#define RGN_MIN             RGN_AND
#define RGN_MAX             RGN_COPY

/* StretchBlt() Modes */
#define BLACKONWHITE                 1
#define WHITEONBLACK                 2
#define COLORONCOLOR                 3
#define HALFTONE                     4
#define MAXSTRETCHBLTMODE            4

#if(WINVER >= 0x0400)
/* New StretchBlt() Modes */
#define STRETCH_ANDSCANS    BLACKONWHITE
#define STRETCH_ORSCANS     WHITEONBLACK
#define STRETCH_DELETESCANS COLORONCOLOR
#define STRETCH_HALFTONE    HALFTONE
#endif /* WINVER >= 0x0400 */

/* PolyFill() Modes */
#define ALTERNATE                    1
#define WINDING                      2
#define POLYFILL_LAST                2

/* Layout Orientation Options */
#if(WINVER >= 0x0500)
#define LAYOUT_RTL                         0x00000001 // Right to left
#define LAYOUT_BTT                         0x00000002 // Bottom to top
#define LAYOUT_VBH                         0x00000004 // Vertical before horizontal
#define LAYOUT_ORIENTATIONMASK             (LAYOUT_RTL | LAYOUT_BTT | LAYOUT_VBH)
#define LAYOUT_BITMAPORIENTATIONPRESERVED  0x00000008
#endif /* WINVER >= 0x0500 */

/* Text Alignment Options */
#define TA_NOUPDATECP                0
#define TA_UPDATECP                  1

#define TA_LEFT                      0
#define TA_RIGHT                     2
#define TA_CENTER                    6

#define TA_TOP                       0
#define TA_BOTTOM                    8
#define TA_BASELINE                  24
#if (WINVER >= 0x0400)
#define TA_RTLREADING                256
#define TA_MASK       (TA_BASELINE+TA_CENTER+TA_UPDATECP+TA_RTLREADING)
#else
#define TA_MASK       (TA_BASELINE+TA_CENTER+TA_UPDATECP)
#endif

#define VTA_BASELINE TA_BASELINE
#define VTA_LEFT     TA_BOTTOM
#define VTA_RIGHT    TA_TOP
#define VTA_CENTER   TA_CENTER
#define VTA_BOTTOM   TA_RIGHT
#define VTA_TOP      TA_LEFT

#define ETO_OPAQUE                   0x0002
#define ETO_CLIPPED                  0x0004
#if(WINVER >= 0x0400)
#define ETO_GLYPH_INDEX              0x0010
#define ETO_RTLREADING               0x0080
#define ETO_NUMERICSLOCAL            0x0400
#define ETO_NUMERICSLATIN            0x0800
#define ETO_IGNORELANGUAGE           0x1000
#endif /* WINVER >= 0x0400 */
#if (_WIN32_WINNT >= _WIN32_WINNT_WIN2K)
#define ETO_PDY                      0x2000
#endif // (_WIN32_WINNT >= _WIN32_WINNT_WIN2K)
#if (_WIN32_WINNT >= _WIN32_WINNT_LONGHORN)
#define ETO_REVERSE_INDEX_MAP        0x10000
#endif

#define ASPECT_FILTERING             0x0001

/* Bounds Accumulation APIs */

#define DCB_RESET       0x0001
#define DCB_ACCUMULATE  0x0002
#define DCB_DIRTY       DCB_ACCUMULATE
#define DCB_SET         (DCB_RESET | DCB_ACCUMULATE)
#define DCB_ENABLE      0x0004
#define DCB_DISABLE     0x0008

#ifndef NOMETAFILE

/* Metafile Functions */
#define META_SETBKCOLOR              0x0201
#define META_SETBKMODE               0x0102
#define META_SETMAPMODE              0x0103
#define META_SETROP2                 0x0104
#define META_SETRELABS               0x0105
#define META_SETPOLYFILLMODE         0x0106
#define META_SETSTRETCHBLTMODE       0x0107
#define META_SETTEXTCHAREXTRA        0x0108
#define META_SETTEXTCOLOR            0x0209
#define META_SETTEXTJUSTIFICATION    0x020A
#define META_SETWINDOWORG            0x020B
#define META_SETWINDOWEXT            0x020C
#define META_SETVIEWPORTORG          0x020D
#define META_SETVIEWPORTEXT          0x020E
#define META_OFFSETWINDOWORG         0x020F
#define META_SCALEWINDOWEXT          0x0410
#define META_OFFSETVIEWPORTORG       0x0211
#define META_SCALEVIEWPORTEXT        0x0412
#define META_LINETO                  0x0213
#define META_MOVETO                  0x0214
#define META_EXCLUDECLIPRECT         0x0415
#define META_INTERSECTCLIPRECT       0x0416
#define META_ARC                     0x0817
#define META_ELLIPSE                 0x0418
#define META_FLOODFILL               0x0419
#define META_PIE                     0x081A
#define META_RECTANGLE               0x041B
#define META_ROUNDRECT               0x061C
#define META_PATBLT                  0x061D
#define META_SAVEDC                  0x001E
#define META_SETPIXEL                0x041F
#define META_OFFSETCLIPRGN           0x0220
#define META_TEXTOUT                 0x0521
#define META_BITBLT                  0x0922
#define META_STRETCHBLT              0x0B23
#define META_POLYGON                 0x0324
#define META_POLYLINE                0x0325
#define META_ESCAPE                  0x0626
#define META_RESTOREDC               0x0127
#define META_FILLREGION              0x0228
#define META_FRAMEREGION             0x0429
#define META_INVERTREGION            0x012A
#define META_PAINTREGION             0x012B
#define META_SELECTCLIPREGION        0x012C
#define META_SELECTOBJECT            0x012D
#define META_SETTEXTALIGN            0x012E
#define META_CHORD                   0x0830
#define META_SETMAPPERFLAGS          0x0231
#define META_EXTTEXTOUT              0x0a32
#define META_SETDIBTODEV             0x0d33
#define META_SELECTPALETTE           0x0234
#define META_REALIZEPALETTE          0x0035
#define META_ANIMATEPALETTE          0x0436
#define META_SETPALENTRIES           0x0037
#define META_POLYPOLYGON             0x0538
#define META_RESIZEPALETTE           0x0139
#define META_DIBBITBLT               0x0940
#define META_DIBSTRETCHBLT           0x0b41
#define META_DIBCREATEPATTERNBRUSH   0x0142
#define META_STRETCHDIB              0x0f43
#define META_EXTFLOODFILL            0x0548
#if(WINVER >= 0x0500)
#define META_SETLAYOUT               0x0149
#endif /* WINVER >= 0x0500 */
#define META_DELETEOBJECT            0x01f0
#define META_CREATEPALETTE           0x00f7
#define META_CREATEPATTERNBRUSH      0x01F9
#define META_CREATEPENINDIRECT       0x02FA
#define META_CREATEFONTINDIRECT      0x02FB
#define META_CREATEBRUSHINDIRECT     0x02FC
#define META_CREATEREGION            0x06FF

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

#if(WINVER >= 0x0400)
typedef struct _DRAWPATRECT {
        POINT ptPosition;
        POINT ptSize;
        WORD wStyle;
        WORD wPattern;
} DRAWPATRECT, *PDRAWPATRECT;
#endif /* WINVER >= 0x0400 */

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif /* NOMETAFILE */

/* GDI Escapes */
#define NEWFRAME                     1
#define ABORTDOC                     2
#define NEXTBAND                     3
#define SETCOLORTABLE                4
#define GETCOLORTABLE                5
#define FLUSHOUTPUT                  6
#define DRAFTMODE                    7
#define QUERYESCSUPPORT              8
#define SETABORTPROC                 9
#define STARTDOC                     10
#define ENDDOC                       11
#define GETPHYSPAGESIZE              12
#define GETPRINTINGOFFSET            13
#define GETSCALINGFACTOR             14
#define MFCOMMENT                    15
#define GETPENWIDTH                  16
#define SETCOPYCOUNT                 17
#define SELECTPAPERSOURCE            18
#define DEVICEDATA                   19
#define PASSTHROUGH                  19
#define GETTECHNOLGY                 20
#define GETTECHNOLOGY                20
#define SETLINECAP                   21
#define SETLINEJOIN                  22
#define SETMITERLIMIT                23
#define BANDINFO                     24
#define DRAWPATTERNRECT              25
#define GETVECTORPENSIZE             26
#define GETVECTORBRUSHSIZE           27
#define ENABLEDUPLEX                 28
#define GETSETPAPERBINS              29
#define GETSETPRINTORIENT            30
#define ENUMPAPERBINS                31
#define SETDIBSCALING                32
#define EPSPRINTING                  33
#define ENUMPAPERMETRICS             34
#define GETSETPAPERMETRICS           35
#define POSTSCRIPT_DATA              37
#define POSTSCRIPT_IGNORE            38
#define MOUSETRAILS                  39
#define GETDEVICEUNITS               42

#define GETEXTENDEDTEXTMETRICS       256
#define GETEXTENTTABLE               257
#define GETPAIRKERNTABLE             258
#define GETTRACKKERNTABLE            259
#define EXTTEXTOUT                   512
#define GETFACENAME                  513
#define DOWNLOADFACE                 514
#define ENABLERELATIVEWIDTHS         768
#define ENABLEPAIRKERNING            769
#define SETKERNTRACK                 770
#define SETALLJUSTVALUES             771
#define SETCHARSET                   772

#define STRETCHBLT                   2048
#define METAFILE_DRIVER              2049
#define GETSETSCREENPARAMS           3072
#define QUERYDIBSUPPORT              3073
#define BEGIN_PATH                   4096
#define CLIP_TO_PATH                 4097
#define END_PATH                     4098
#define EXT_DEVICE_CAPS              4099
#define RESTORE_CTM                  4100
#define SAVE_CTM                     4101
#define SET_ARC_DIRECTION            4102
#define SET_BACKGROUND_COLOR         4103
#define SET_POLY_MODE                4104
#define SET_SCREEN_ANGLE             4105
#define SET_SPREAD                   4106
#define TRANSFORM_CTM                4107
#define SET_CLIP_BOX                 4108
#define SET_BOUNDS                   4109
#define SET_MIRROR_MODE              4110
#define OPENCHANNEL                  4110
#define DOWNLOADHEADER               4111
#define CLOSECHANNEL                 4112
#define POSTSCRIPT_PASSTHROUGH       4115
#define ENCAPSULATED_POSTSCRIPT      4116

#define POSTSCRIPT_IDENTIFY          4117   /* new escape for NT5 pscript driver */
#define POSTSCRIPT_INJECTION         4118   /* new escape for NT5 pscript driver */

#define CHECKJPEGFORMAT              4119
#define CHECKPNGFORMAT               4120

#define GET_PS_FEATURESETTING        4121   /* new escape for NT5 pscript driver */

#define GDIPLUS_TS_QUERYVER          4122   /* private escape */
#define GDIPLUS_TS_RECORD            4123   /* private escape */


/*
 * Return Values for MILCORE_TS_QUERYVER
 */

#if (_WIN32_WINNT >= _WIN32_WINNT_LONGHORN)
#define MILCORE_TS_QUERYVER_RESULT_FALSE        0x0
#define MILCORE_TS_QUERYVER_RESULT_TRUE  0x7FFFFFFF
#endif // (_WIN32_WINNT >= _WIN32_WINNT_LONGHORN)


#define SPCLPASSTHROUGH2             4568   /* new escape for NT5 pscript driver */

/*
 * Parameters for POSTSCRIPT_IDENTIFY escape
 */

#define PSIDENT_GDICENTRIC    0
#define PSIDENT_PSCENTRIC     1

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

/*
 * Header structure for the input buffer to POSTSCRIPT_INJECTION escape
 */

typedef struct _PSINJECTDATA {

    DWORD   DataBytes;      /* number of raw data bytes (NOT including this header) */
    WORD    InjectionPoint; /* injection point */
    WORD    PageNumber;     /* page number to apply the injection */

    /* Followed by raw data to be injected */

} PSINJECTDATA, *PPSINJECTDATA;

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

/*
 * Constants for PSINJECTDATA.InjectionPoint field
 */

#define PSINJECT_BEGINSTREAM                1
#define PSINJECT_PSADOBE                    2
#define PSINJECT_PAGESATEND                 3
#define PSINJECT_PAGES                      4

#define PSINJECT_DOCNEEDEDRES               5
#define PSINJECT_DOCSUPPLIEDRES             6
#define PSINJECT_PAGEORDER                  7
#define PSINJECT_ORIENTATION                8
#define PSINJECT_BOUNDINGBOX                9
#define PSINJECT_DOCUMENTPROCESSCOLORS      10

#define PSINJECT_COMMENTS                   11
#define PSINJECT_BEGINDEFAULTS              12
#define PSINJECT_ENDDEFAULTS                13
#define PSINJECT_BEGINPROLOG                14
#define PSINJECT_ENDPROLOG                  15
#define PSINJECT_BEGINSETUP                 16
#define PSINJECT_ENDSETUP                   17
#define PSINJECT_TRAILER                    18
#define PSINJECT_EOF                        19
#define PSINJECT_ENDSTREAM                  20
#define PSINJECT_DOCUMENTPROCESSCOLORSATEND 21

#define PSINJECT_PAGENUMBER                 100
#define PSINJECT_BEGINPAGESETUP             101
#define PSINJECT_ENDPAGESETUP               102
#define PSINJECT_PAGETRAILER                103
#define PSINJECT_PLATECOLOR                 104

#define PSINJECT_SHOWPAGE                   105
#define PSINJECT_PAGEBBOX                   106
#define PSINJECT_ENDPAGECOMMENTS            107

#define PSINJECT_VMSAVE                     200
#define PSINJECT_VMRESTORE                  201

/*
 * InjectionPoint for publisher mode PScript5 OEM plugin to
 * generate DSC comment for included font resource
 */
#define PSINJECT_DLFONT                     0xdddddddd

/*
 * Parameter for GET_PS_FEATURESETTING escape
 */

#define FEATURESETTING_NUP                  0
#define FEATURESETTING_OUTPUT               1
#define FEATURESETTING_PSLEVEL              2
#define FEATURESETTING_CUSTPAPER            3
#define FEATURESETTING_MIRROR               4
#define FEATURESETTING_NEGATIVE             5
#define FEATURESETTING_PROTOCOL             6

#if (_WIN32_WINNT >= _WIN32_WINNT_WINXP)
//
// The range of selectors between FEATURESETTING_PRIVATE_BEGIN and
// FEATURESETTING_PRIVATE_END is reserved by Microsoft for private use
//
#define FEATURESETTING_PRIVATE_BEGIN 0x1000
#define FEATURESETTING_PRIVATE_END   0x1FFF
#endif // (_WIN32_WINNT >= _WIN32_WINNT_WINXP)

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

/*
 * Information about output options
 */

typedef struct _PSFEATURE_OUTPUT {

    BOOL bPageIndependent;
    BOOL bSetPageDevice;

} PSFEATURE_OUTPUT, *PPSFEATURE_OUTPUT;

/*
 * Information about custom paper size
 */

typedef struct _PSFEATURE_CUSTPAPER {

    LONG lOrientation;
    LONG lWidth;
    LONG lHeight;
    LONG lWidthOffset;
    LONG lHeightOffset;

} PSFEATURE_CUSTPAPER, *PPSFEATURE_CUSTPAPER;

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

/* Value returned for FEATURESETTING_PROTOCOL */
#define PSPROTOCOL_ASCII             0
#define PSPROTOCOL_BCP               1
#define PSPROTOCOL_TBCP              2
#define PSPROTOCOL_BINARY            3

/* Flag returned from QUERYDIBSUPPORT */
#define QDI_SETDIBITS                1
#define QDI_GETDIBITS                2
#define QDI_DIBTOSCREEN              4
#define QDI_STRETCHDIB               8

/* Spooler Error Codes */
#define SP_NOTREPORTED               0x4000
#define SP_ERROR                     (-1)
#define SP_APPABORT                  (-2)
#define SP_USERABORT                 (-3)
#define SP_OUTOFDISK                 (-4)
#define SP_OUTOFMEMORY               (-5)

#define PR_JOBSTATUS                 0x0000

/* Object Definitions for EnumObjects() */
#define OBJ_PEN             1
#define OBJ_BRUSH           2
#define OBJ_DC              3
#define OBJ_METADC          4
#define OBJ_PAL             5
#define OBJ_FONT            6
#define OBJ_BITMAP          7
#define OBJ_REGION          8
#define OBJ_METAFILE        9
#define OBJ_MEMDC           10
#define OBJ_EXTPEN          11
#define OBJ_ENHMETADC       12
#define OBJ_ENHMETAFILE     13
#define OBJ_COLORSPACE      14

#define GDI_OBJ_LAST        OBJ_COLORSPACE

#define GDI_MIN_OBJ_TYPE    OBJ_PEN
#define GDI_MAX_OBJ_TYPE    GDI_OBJ_LAST

/* xform stuff */
#define MWT_IDENTITY        1
#define MWT_LEFTMULTIPLY    2
#define MWT_RIGHTMULTIPLY   3

#define MWT_MIN             MWT_IDENTITY
#define MWT_MAX             MWT_RIGHTMULTIPLY

#define _XFORM_

#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)

typedef struct  tagXFORM
  {
    FLOAT   eM11;
    FLOAT   eM12;
    FLOAT   eM21;
    FLOAT   eM22;
    FLOAT   eDx;
    FLOAT   eDy;
  } XFORM, *PXFORM, FAR *LPXFORM;

/* Bitmap Header Definition */
typedef struct tagBITMAP
  {
    LONG        bmType;
    LONG        bmWidth;
    LONG        bmHeight;
    LONG        bmWidthBytes;
    WORD        bmPlanes;
    WORD        bmBitsPixel;
    LPVOID      bmBits;
  } BITMAP, *PBITMAP, NEAR *NPBITMAP, FAR *LPBITMAP;

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#include <pshpack1.h>

#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)

typedef struct tagRGBTRIPLE {
        BYTE    rgbtBlue;
        BYTE    rgbtGreen;
        BYTE    rgbtRed;
} RGBTRIPLE, *PRGBTRIPLE, NEAR *NPRGBTRIPLE, FAR *LPRGBTRIPLE;

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#include <poppack.h>

#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)

typedef struct tagRGBQUAD {
        BYTE    rgbBlue;
        BYTE    rgbGreen;
        BYTE    rgbRed;
        BYTE    rgbReserved;
} RGBQUAD;

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

typedef RGBQUAD FAR* LPRGBQUAD;

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#if(WINVER >= 0x0400)

/* Image Color Matching color definitions */

#define CS_ENABLE                       0x00000001L
#define CS_DISABLE                      0x00000002L
#define CS_DELETE_TRANSFORM             0x00000003L

/* Logcolorspace signature */

#define LCS_SIGNATURE           'PSOC'

/* Logcolorspace lcsType values */

#define LCS_sRGB                'sRGB'
#define LCS_WINDOWS_COLOR_SPACE 'Win '  // Windows default color space

#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)

typedef LONG   LCSCSTYPE;

#define LCS_CALIBRATED_RGB              0x00000000L

typedef LONG    LCSGAMUTMATCH;

#define LCS_GM_BUSINESS                 0x00000001L
#define LCS_GM_GRAPHICS                 0x00000002L
#define LCS_GM_IMAGES                   0x00000004L
#define LCS_GM_ABS_COLORIMETRIC         0x00000008L

/* ICM Defines for results from CheckColorInGamut() */
#define CM_OUT_OF_GAMUT                 255
#define CM_IN_GAMUT                     0

/* UpdateICMRegKey Constants               */
#define ICM_ADDPROFILE                  1
#define ICM_DELETEPROFILE               2
#define ICM_QUERYPROFILE                3
#define ICM_SETDEFAULTPROFILE           4
#define ICM_REGISTERICMATCHER           5
#define ICM_UNREGISTERICMATCHER         6
#define ICM_QUERYMATCH                  7

/* Macros to retrieve CMYK values from a COLORREF */
#define GetKValue(cmyk)      ((BYTE)(cmyk))
#define GetYValue(cmyk)      ((BYTE)((cmyk)>> 8))
#define GetMValue(cmyk)      ((BYTE)((cmyk)>>16))
#define GetCValue(cmyk)      ((BYTE)((cmyk)>>24))

#define CMYK(c,m,y,k)       ((COLORREF)((((BYTE)(k)|((WORD)((BYTE)(y))<<8))|(((DWORD)(BYTE)(m))<<16))|(((DWORD)(BYTE)(c))<<24)))

typedef long            FXPT16DOT16, FAR *LPFXPT16DOT16;
typedef long            FXPT2DOT30, FAR *LPFXPT2DOT30;

/* ICM Color Definitions */
// The following two structures are used for defining RGB's in terms of CIEXYZ.

typedef struct tagCIEXYZ
{
        FXPT2DOT30 ciexyzX;
        FXPT2DOT30 ciexyzY;
        FXPT2DOT30 ciexyzZ;
} CIEXYZ;

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

typedef CIEXYZ  FAR *LPCIEXYZ;

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)

typedef struct tagICEXYZTRIPLE
{
        CIEXYZ  ciexyzRed;
        CIEXYZ  ciexyzGreen;
        CIEXYZ  ciexyzBlue;
} CIEXYZTRIPLE;

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

typedef CIEXYZTRIPLE    FAR *LPCIEXYZTRIPLE;

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)

// The next structures the logical color space. Unlike pens and brushes,
// but like palettes, there is only one way to create a LogColorSpace.
// A pointer to it must be passed, its elements can't be pushed as
// arguments.

typedef struct tagLOGCOLORSPACEA {
    DWORD lcsSignature;
    DWORD lcsVersion;
    DWORD lcsSize;
    LCSCSTYPE lcsCSType;
    LCSGAMUTMATCH lcsIntent;
    CIEXYZTRIPLE lcsEndpoints;
    DWORD lcsGammaRed;
    DWORD lcsGammaGreen;
    DWORD lcsGammaBlue;
    CHAR   lcsFilename[MAX_PATH];
} LOGCOLORSPACEA, *LPLOGCOLORSPACEA;
typedef struct tagLOGCOLORSPACEW {
    DWORD lcsSignature;
    DWORD lcsVersion;
    DWORD lcsSize;
    LCSCSTYPE lcsCSType;
    LCSGAMUTMATCH lcsIntent;
    CIEXYZTRIPLE lcsEndpoints;
    DWORD lcsGammaRed;
    DWORD lcsGammaGreen;
    DWORD lcsGammaBlue;
    WCHAR  lcsFilename[MAX_PATH];
} LOGCOLORSPACEW, *LPLOGCOLORSPACEW;
#ifdef UNICODE
typedef LOGCOLORSPACEW LOGCOLORSPACE;
typedef LPLOGCOLORSPACEW LPLOGCOLORSPACE;
#else
typedef LOGCOLORSPACEA LOGCOLORSPACE;
typedef LPLOGCOLORSPACEA LPLOGCOLORSPACE;
#endif // UNICODE

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#endif /* WINVER >= 0x0400 */

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

/* structures for defining DIBs */
typedef struct tagBITMAPCOREHEADER {
        DWORD   bcSize;                 /* used to get to color table */
        WORD    bcWidth;
        WORD    bcHeight;
        WORD    bcPlanes;
        WORD    bcBitCount;
} BITMAPCOREHEADER, FAR *LPBITMAPCOREHEADER, *PBITMAPCOREHEADER;

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#pragma region Application Family or OneCore Family or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

typedef struct tagBITMAPINFOHEADER{
        DWORD      biSize;
        LONG       biWidth;
        LONG       biHeight;
        WORD       biPlanes;
        WORD       biBitCount;
        DWORD      biCompression;
        DWORD      biSizeImage;
        LONG       biXPelsPerMeter;
        LONG       biYPelsPerMeter;
        DWORD      biClrUsed;
        DWORD      biClrImportant;
} BITMAPINFOHEADER, FAR *LPBITMAPINFOHEADER, *PBITMAPINFOHEADER;

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

#if(WINVER >= 0x0400)

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

typedef struct {
        DWORD        bV4Size;
        LONG         bV4Width;
        LONG         bV4Height;
        WORD         bV4Planes;
        WORD         bV4BitCount;
        DWORD        bV4V4Compression;
        DWORD        bV4SizeImage;
        LONG         bV4XPelsPerMeter;
        LONG         bV4YPelsPerMeter;
        DWORD        bV4ClrUsed;
        DWORD        bV4ClrImportant;
        DWORD        bV4RedMask;
        DWORD        bV4GreenMask;
        DWORD        bV4BlueMask;
        DWORD        bV4AlphaMask;
        DWORD        bV4CSType;
        CIEXYZTRIPLE bV4Endpoints;
        DWORD        bV4GammaRed;
        DWORD        bV4GammaGreen;
        DWORD        bV4GammaBlue;
} BITMAPV4HEADER, FAR *LPBITMAPV4HEADER, *PBITMAPV4HEADER;

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion
#endif /* WINVER >= 0x0400 */

#if (WINVER >= 0x0500)

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
typedef struct {
        DWORD        bV5Size;
        LONG         bV5Width;
        LONG         bV5Height;
        WORD         bV5Planes;
        WORD         bV5BitCount;
        DWORD        bV5Compression;
        DWORD        bV5SizeImage;
        LONG         bV5XPelsPerMeter;
        LONG         bV5YPelsPerMeter;
        DWORD        bV5ClrUsed;
        DWORD        bV5ClrImportant;
        DWORD        bV5RedMask;
        DWORD        bV5GreenMask;
        DWORD        bV5BlueMask;
        DWORD        bV5AlphaMask;
        DWORD        bV5CSType;
        CIEXYZTRIPLE bV5Endpoints;
        DWORD        bV5GammaRed;
        DWORD        bV5GammaGreen;
        DWORD        bV5GammaBlue;
        DWORD        bV5Intent;
        DWORD        bV5ProfileData;
        DWORD        bV5ProfileSize;
        DWORD        bV5Reserved;
} BITMAPV5HEADER, FAR *LPBITMAPV5HEADER, *PBITMAPV5HEADER;

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

// Values for bV5CSType
#define PROFILE_LINKED          'LINK'
#define PROFILE_EMBEDDED        'MBED'
#endif

/* constants for the biCompression field */
#define BI_RGB        0L
#define BI_RLE8       1L
#define BI_RLE4       2L
#define BI_BITFIELDS  3L
#define BI_JPEG       4L
#define BI_PNG        5L
#if (_WIN32_WINNT >= _WIN32_WINNT_NT4)
#endif

#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)

typedef struct tagBITMAPINFO {
    BITMAPINFOHEADER    bmiHeader;
    RGBQUAD             bmiColors[1];
} BITMAPINFO, FAR *LPBITMAPINFO, *PBITMAPINFO;

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

typedef struct tagBITMAPCOREINFO {
    BITMAPCOREHEADER    bmciHeader;
    RGBTRIPLE           bmciColors[1];
} BITMAPCOREINFO, FAR *LPBITMAPCOREINFO, *PBITMAPCOREINFO;

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#include <pshpack2.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

typedef struct tagBITMAPFILEHEADER {
        WORD    bfType;
        DWORD   bfSize;
        WORD    bfReserved1;
        WORD    bfReserved2;
        DWORD   bfOffBits;
} BITMAPFILEHEADER, FAR *LPBITMAPFILEHEADER, *PBITMAPFILEHEADER;

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#include <poppack.h>

#define MAKEPOINTS(l)       (*((POINTS FAR *)&(l)))

#if(WINVER >= 0x0400)
#ifndef NOFONTSIG

#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)

typedef struct tagFONTSIGNATURE
{
    DWORD fsUsb[4];
    DWORD fsCsb[2];
} FONTSIGNATURE, *PFONTSIGNATURE,FAR *LPFONTSIGNATURE;

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

typedef struct tagCHARSETINFO
{
    UINT ciCharset;
    UINT ciACP;
    FONTSIGNATURE fs;
} CHARSETINFO, *PCHARSETINFO, NEAR *NPCHARSETINFO, FAR *LPCHARSETINFO;

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#define TCI_SRCCHARSET  1
#define TCI_SRCCODEPAGE 2
#define TCI_SRCFONTSIG  3
#if (_WIN32_WINNT >= _WIN32_WINNT_WINXP)
#define TCI_SRCLOCALE   0x1000
#endif // (_WIN32_WINNT >= _WIN32_WINNT_WINXP)

#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)

typedef struct tagLOCALESIGNATURE
{
    DWORD lsUsb[4];
    DWORD lsCsbDefault[2];
    DWORD lsCsbSupported[2];
} LOCALESIGNATURE, *PLOCALESIGNATURE,FAR *LPLOCALESIGNATURE;

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#endif
#endif /* WINVER >= 0x0400 */

#ifndef NOMETAFILE

#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)

/* Clipboard Metafile Picture Structure */

typedef struct tagHANDLETABLE
  {
    HGDIOBJ     objectHandle[1];
  } HANDLETABLE, *PHANDLETABLE, FAR *LPHANDLETABLE;

typedef struct tagMETARECORD
  {
    DWORD       rdSize;
    WORD        rdFunction;
    WORD        rdParm[1];
  } METARECORD;

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

typedef struct tagMETARECORD UNALIGNED *PMETARECORD;

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)

typedef struct tagMETARECORD UNALIGNED FAR *LPMETARECORD;

typedef struct tagMETAFILEPICT
  {
    LONG        mm;
    LONG        xExt;
    LONG        yExt;
    HMETAFILE   hMF;
  } METAFILEPICT, FAR *LPMETAFILEPICT;

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#include <pshpack2.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

typedef struct tagMETAHEADER
{
    WORD        mtType;
    WORD        mtHeaderSize;
    WORD        mtVersion;
    DWORD       mtSize;
    WORD        mtNoObjects;
    DWORD       mtMaxRecord;
    WORD        mtNoParameters;
} METAHEADER;
typedef struct tagMETAHEADER UNALIGNED *PMETAHEADER;
typedef struct tagMETAHEADER UNALIGNED FAR *LPMETAHEADER;

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#include <poppack.h>

#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)

/* Enhanced Metafile structures */
typedef struct tagENHMETARECORD
{
    DWORD   iType;              // Record type EMR_XXX
    DWORD   nSize;              // Record size in bytes
    DWORD   dParm[1];           // Parameters
} ENHMETARECORD, *PENHMETARECORD, *LPENHMETARECORD;

typedef struct tagENHMETAHEADER
{
    DWORD   iType;              // Record typeEMR_HEADER
    DWORD   nSize;              // Record size in bytes.  This may be greater
                                // than the sizeof(ENHMETAHEADER).
    RECTL   rclBounds;          // Inclusive-inclusive bounds in device units
    RECTL   rclFrame;           // Inclusive-inclusive Picture Frame of metafile in .01 mm units
    DWORD   dSignature;         // Signature.  Must be ENHMETA_SIGNATURE.
    DWORD   nVersion;           // Version number
    DWORD   nBytes;             // Size of the metafile in bytes
    DWORD   nRecords;           // Number of records in the metafile
    WORD    nHandles;           // Number of handles in the handle table
                                // Handle index zero is reserved.
    WORD    sReserved;          // Reserved.  Must be zero.
    DWORD   nDescription;       // Number of chars in the unicode description string
                                // This is 0 if there is no description string
    DWORD   offDescription;     // Offset to the metafile description record.
                                // This is 0 if there is no description string
    DWORD   nPalEntries;        // Number of entries in the metafile palette.
    SIZEL   szlDevice;          // Size of the reference device in pels
    SIZEL   szlMillimeters;     // Size of the reference device in millimeters
#if(WINVER >= 0x0400)
    DWORD   cbPixelFormat;      // Size of PIXELFORMATDESCRIPTOR information
                                // This is 0 if no pixel format is set
    DWORD   offPixelFormat;     // Offset to PIXELFORMATDESCRIPTOR
                                // This is 0 if no pixel format is set
    DWORD   bOpenGL;            // TRUE if OpenGL commands are present in
                                // the metafile, otherwise FALSE
#endif /* WINVER >= 0x0400 */
#if(WINVER >= 0x0500)
    SIZEL   szlMicrometers;     // Size of the reference device in micrometers
#endif /* WINVER >= 0x0500 */

} ENHMETAHEADER, *PENHMETAHEADER, *LPENHMETAHEADER;

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#endif /* NOMETAFILE */

#ifndef NOTEXTMETRIC

/* tmPitchAndFamily flags */
#define TMPF_FIXED_PITCH    0x01
#define TMPF_VECTOR             0x02
#define TMPF_DEVICE             0x08
#define TMPF_TRUETYPE       0x04

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

//
// BCHAR definition for APPs
//
#ifdef UNICODE
    typedef WCHAR BCHAR;
#else
    typedef BYTE BCHAR;
#endif

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#ifndef _TEXTMETRIC_DEFINED
#define _TEXTMETRIC_DEFINED
#include <pshpack4.h>

#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)

typedef struct tagTEXTMETRICA
{
    LONG        tmHeight;
    LONG        tmAscent;
    LONG        tmDescent;
    LONG        tmInternalLeading;
    LONG        tmExternalLeading;
    LONG        tmAveCharWidth;
    LONG        tmMaxCharWidth;
    LONG        tmWeight;
    LONG        tmOverhang;
    LONG        tmDigitizedAspectX;
    LONG        tmDigitizedAspectY;
    BYTE        tmFirstChar;
    BYTE        tmLastChar;
    BYTE        tmDefaultChar;
    BYTE        tmBreakChar;
    BYTE        tmItalic;
    BYTE        tmUnderlined;
    BYTE        tmStruckOut;
    BYTE        tmPitchAndFamily;
    BYTE        tmCharSet;
} TEXTMETRICA, *PTEXTMETRICA, NEAR *NPTEXTMETRICA, FAR *LPTEXTMETRICA;
typedef struct tagTEXTMETRICW
{
    LONG        tmHeight;
    LONG        tmAscent;
    LONG        tmDescent;
    LONG        tmInternalLeading;
    LONG        tmExternalLeading;
    LONG        tmAveCharWidth;
    LONG        tmMaxCharWidth;
    LONG        tmWeight;
    LONG        tmOverhang;
    LONG        tmDigitizedAspectX;
    LONG        tmDigitizedAspectY;
    WCHAR       tmFirstChar;
    WCHAR       tmLastChar;
    WCHAR       tmDefaultChar;
    WCHAR       tmBreakChar;
    BYTE        tmItalic;
    BYTE        tmUnderlined;
    BYTE        tmStruckOut;
    BYTE        tmPitchAndFamily;
    BYTE        tmCharSet;
} TEXTMETRICW, *PTEXTMETRICW, NEAR *NPTEXTMETRICW, FAR *LPTEXTMETRICW;
#ifdef UNICODE
typedef TEXTMETRICW TEXTMETRIC;
typedef PTEXTMETRICW PTEXTMETRIC;
typedef NPTEXTMETRICW NPTEXTMETRIC;
typedef LPTEXTMETRICW LPTEXTMETRIC;
#else
typedef TEXTMETRICA TEXTMETRIC;
typedef PTEXTMETRICA PTEXTMETRIC;
typedef NPTEXTMETRICA NPTEXTMETRIC;
typedef LPTEXTMETRICA LPTEXTMETRIC;
#endif // UNICODE

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#include <poppack.h>
#endif // !_TEXTMETRIC_DEFINED

/* ntmFlags field flags */
#define NTM_REGULAR     0x00000040L
#define NTM_BOLD        0x00000020L
#define NTM_ITALIC      0x00000001L

/* new in NT 5.0 */

#define NTM_NONNEGATIVE_AC  0x00010000
#define NTM_PS_OPENTYPE     0x00020000
#define NTM_TT_OPENTYPE     0x00040000
#define NTM_MULTIPLEMASTER  0x00080000
#define NTM_TYPE1           0x00100000
#define NTM_DSIG            0x00200000

#include <pshpack4.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

typedef struct tagNEWTEXTMETRICA
{
    LONG        tmHeight;
    LONG        tmAscent;
    LONG        tmDescent;
    LONG        tmInternalLeading;
    LONG        tmExternalLeading;
    LONG        tmAveCharWidth;
    LONG        tmMaxCharWidth;
    LONG        tmWeight;
    LONG        tmOverhang;
    LONG        tmDigitizedAspectX;
    LONG        tmDigitizedAspectY;
    BYTE        tmFirstChar;
    BYTE        tmLastChar;
    BYTE        tmDefaultChar;
    BYTE        tmBreakChar;
    BYTE        tmItalic;
    BYTE        tmUnderlined;
    BYTE        tmStruckOut;
    BYTE        tmPitchAndFamily;
    BYTE        tmCharSet;
    DWORD   ntmFlags;
    UINT    ntmSizeEM;
    UINT    ntmCellHeight;
    UINT    ntmAvgWidth;
} NEWTEXTMETRICA, *PNEWTEXTMETRICA, NEAR *NPNEWTEXTMETRICA, FAR *LPNEWTEXTMETRICA;
typedef struct tagNEWTEXTMETRICW
{
    LONG        tmHeight;
    LONG        tmAscent;
    LONG        tmDescent;
    LONG        tmInternalLeading;
    LONG        tmExternalLeading;
    LONG        tmAveCharWidth;
    LONG        tmMaxCharWidth;
    LONG        tmWeight;
    LONG        tmOverhang;
    LONG        tmDigitizedAspectX;
    LONG        tmDigitizedAspectY;
    WCHAR       tmFirstChar;
    WCHAR       tmLastChar;
    WCHAR       tmDefaultChar;
    WCHAR       tmBreakChar;
    BYTE        tmItalic;
    BYTE        tmUnderlined;
    BYTE        tmStruckOut;
    BYTE        tmPitchAndFamily;
    BYTE        tmCharSet;
    DWORD   ntmFlags;
    UINT    ntmSizeEM;
    UINT    ntmCellHeight;
    UINT    ntmAvgWidth;
} NEWTEXTMETRICW, *PNEWTEXTMETRICW, NEAR *NPNEWTEXTMETRICW, FAR *LPNEWTEXTMETRICW;
#ifdef UNICODE
typedef NEWTEXTMETRICW NEWTEXTMETRIC;
typedef PNEWTEXTMETRICW PNEWTEXTMETRIC;
typedef NPNEWTEXTMETRICW NPNEWTEXTMETRIC;
typedef LPNEWTEXTMETRICW LPNEWTEXTMETRIC;
#else
typedef NEWTEXTMETRICA NEWTEXTMETRIC;
typedef PNEWTEXTMETRICA PNEWTEXTMETRIC;
typedef NPNEWTEXTMETRICA NPNEWTEXTMETRIC;
typedef LPNEWTEXTMETRICA LPNEWTEXTMETRIC;
#endif // UNICODE

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#include <poppack.h>

#if(WINVER >= 0x0400)

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

typedef struct tagNEWTEXTMETRICEXA
{
    NEWTEXTMETRICA  ntmTm;
    FONTSIGNATURE   ntmFontSig;
}NEWTEXTMETRICEXA;
typedef struct tagNEWTEXTMETRICEXW
{
    NEWTEXTMETRICW  ntmTm;
    FONTSIGNATURE   ntmFontSig;
}NEWTEXTMETRICEXW;
#ifdef UNICODE
typedef NEWTEXTMETRICEXW NEWTEXTMETRICEX;
#else
typedef NEWTEXTMETRICEXA NEWTEXTMETRICEX;
#endif // UNICODE

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion
#endif /* WINVER >= 0x0400 */

#endif /* NOTEXTMETRIC */
/* GDI Logical Objects: */

/* Pel Array */
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

typedef struct tagPELARRAY
  {
    LONG        paXCount;
    LONG        paYCount;
    LONG        paXExt;
    LONG        paYExt;
    BYTE        paRGBs;
  } PELARRAY, *PPELARRAY, NEAR *NPPELARRAY, FAR *LPPELARRAY;

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)

/* Logical Brush (or Pattern) */
typedef struct tagLOGBRUSH
  {
    UINT        lbStyle;
    COLORREF    lbColor;
    ULONG_PTR   lbHatch;
  } LOGBRUSH, *PLOGBRUSH, NEAR *NPLOGBRUSH, FAR *LPLOGBRUSH;

typedef struct tagLOGBRUSH32
  {
    UINT        lbStyle;
    COLORREF    lbColor;
    ULONG       lbHatch;
  } LOGBRUSH32, *PLOGBRUSH32, NEAR *NPLOGBRUSH32, FAR *LPLOGBRUSH32;

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

typedef LOGBRUSH            PATTERN;
typedef PATTERN             *PPATTERN;
typedef PATTERN NEAR        *NPPATTERN;
typedef PATTERN FAR         *LPPATTERN;

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)

/* Logical Pen */
typedef struct tagLOGPEN
  {
    UINT        lopnStyle;
    POINT       lopnWidth;
    COLORREF    lopnColor;
  } LOGPEN, *PLOGPEN, NEAR *NPLOGPEN, FAR *LPLOGPEN;

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

typedef struct tagEXTLOGPEN {
    DWORD       elpPenStyle;
    DWORD       elpWidth;
    UINT        elpBrushStyle;
    COLORREF    elpColor;
    ULONG_PTR   elpHatch;
    DWORD       elpNumEntries;
    DWORD       elpStyleEntry[1];
} EXTLOGPEN, *PEXTLOGPEN, NEAR *NPEXTLOGPEN, FAR *LPEXTLOGPEN;

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)

typedef struct tagEXTLOGPEN32 {
    DWORD       elpPenStyle;
    DWORD       elpWidth;
    UINT        elpBrushStyle;
    COLORREF    elpColor;
    ULONG       elpHatch;
    DWORD       elpNumEntries;
    DWORD       elpStyleEntry[1];
} EXTLOGPEN32, *PEXTLOGPEN32, NEAR *NPEXTLOGPEN32, FAR *LPEXTLOGPEN32;

#ifndef _PALETTEENTRY_DEFINED
#define _PALETTEENTRY_DEFINED
typedef struct tagPALETTEENTRY {
    BYTE        peRed;
    BYTE        peGreen;
    BYTE        peBlue;
    BYTE        peFlags;
} PALETTEENTRY, *PPALETTEENTRY, FAR *LPPALETTEENTRY;
#endif // !_PALETTEENTRY_DEFINED

#ifndef _LOGPALETTE_DEFINED
#define _LOGPALETTE_DEFINED
/* Logical Palette */
typedef struct tagLOGPALETTE {
    WORD        palVersion;
    WORD        palNumEntries;
    _Field_size_opt_(palNumEntries) PALETTEENTRY        palPalEntry[1];
} LOGPALETTE, *PLOGPALETTE, NEAR *NPLOGPALETTE, FAR *LPLOGPALETTE;
#endif // !_LOGPALETTE_DEFINED


/* Logical Font */
#define LF_FACESIZE         32

typedef struct tagLOGFONTA
{
    LONG      lfHeight;
    LONG      lfWidth;
    LONG      lfEscapement;
    LONG      lfOrientation;
    LONG      lfWeight;
    BYTE      lfItalic;
    BYTE      lfUnderline;
    BYTE      lfStrikeOut;
    BYTE      lfCharSet;
    BYTE      lfOutPrecision;
    BYTE      lfClipPrecision;
    BYTE      lfQuality;
    BYTE      lfPitchAndFamily;
    CHAR      lfFaceName[LF_FACESIZE];
} LOGFONTA, *PLOGFONTA, NEAR *NPLOGFONTA, FAR *LPLOGFONTA;
typedef struct tagLOGFONTW
{
    LONG      lfHeight;
    LONG      lfWidth;
    LONG      lfEscapement;
    LONG      lfOrientation;
    LONG      lfWeight;
    BYTE      lfItalic;
    BYTE      lfUnderline;
    BYTE      lfStrikeOut;
    BYTE      lfCharSet;
    BYTE      lfOutPrecision;
    BYTE      lfClipPrecision;
    BYTE      lfQuality;
    BYTE      lfPitchAndFamily;
    WCHAR     lfFaceName[LF_FACESIZE];
} LOGFONTW, *PLOGFONTW, NEAR *NPLOGFONTW, FAR *LPLOGFONTW;
#ifdef UNICODE
typedef LOGFONTW LOGFONT;
typedef PLOGFONTW PLOGFONT;
typedef NPLOGFONTW NPLOGFONT;
typedef LPLOGFONTW LPLOGFONT;
#else
typedef LOGFONTA LOGFONT;
typedef PLOGFONTA PLOGFONT;
typedef NPLOGFONTA NPLOGFONT;
typedef LPLOGFONTA LPLOGFONT;
#endif // UNICODE

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#define LF_FULLFACESIZE     64

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

/* Structure passed to FONTENUMPROC */
typedef struct tagENUMLOGFONTA
{
    LOGFONTA elfLogFont;
    BYTE     elfFullName[LF_FULLFACESIZE];
    BYTE     elfStyle[LF_FACESIZE];
} ENUMLOGFONTA, FAR* LPENUMLOGFONTA;
/* Structure passed to FONTENUMPROC */
typedef struct tagENUMLOGFONTW
{
    LOGFONTW elfLogFont;
    WCHAR    elfFullName[LF_FULLFACESIZE];
    WCHAR    elfStyle[LF_FACESIZE];
} ENUMLOGFONTW, FAR* LPENUMLOGFONTW;
#ifdef UNICODE
typedef ENUMLOGFONTW ENUMLOGFONT;
typedef LPENUMLOGFONTW LPENUMLOGFONT;
#else
typedef ENUMLOGFONTA ENUMLOGFONT;
typedef LPENUMLOGFONTA LPENUMLOGFONT;
#endif // UNICODE

#if(WINVER >= 0x0400)
typedef struct tagENUMLOGFONTEXA
{
    LOGFONTA    elfLogFont;
    BYTE        elfFullName[LF_FULLFACESIZE];
    BYTE        elfStyle[LF_FACESIZE];
    BYTE        elfScript[LF_FACESIZE];
} ENUMLOGFONTEXA, FAR *LPENUMLOGFONTEXA;
typedef struct tagENUMLOGFONTEXW
{
    LOGFONTW    elfLogFont;
    WCHAR       elfFullName[LF_FULLFACESIZE];
    WCHAR       elfStyle[LF_FACESIZE];
    WCHAR       elfScript[LF_FACESIZE];
} ENUMLOGFONTEXW, FAR *LPENUMLOGFONTEXW;
#ifdef UNICODE
typedef ENUMLOGFONTEXW ENUMLOGFONTEX;
typedef LPENUMLOGFONTEXW LPENUMLOGFONTEX;
#else
typedef ENUMLOGFONTEXA ENUMLOGFONTEX;
typedef LPENUMLOGFONTEXA LPENUMLOGFONTEX;
#endif // UNICODE
#endif /* WINVER >= 0x0400 */

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#define OUT_DEFAULT_PRECIS          0
#define OUT_STRING_PRECIS           1
#define OUT_CHARACTER_PRECIS        2
#define OUT_STROKE_PRECIS           3
#define OUT_TT_PRECIS               4
#define OUT_DEVICE_PRECIS           5
#define OUT_RASTER_PRECIS           6
#define OUT_TT_ONLY_PRECIS          7
#define OUT_OUTLINE_PRECIS          8
#define OUT_SCREEN_OUTLINE_PRECIS   9
#define OUT_PS_ONLY_PRECIS          10

#define CLIP_DEFAULT_PRECIS     0
#define CLIP_CHARACTER_PRECIS   1
#define CLIP_STROKE_PRECIS      2
#define CLIP_MASK               0xf
#define CLIP_LH_ANGLES          (1<<4)
#define CLIP_TT_ALWAYS          (2<<4)
#if (_WIN32_WINNT >= _WIN32_WINNT_LONGHORN)
#define CLIP_DFA_DISABLE        (4<<4)
#endif // (_WIN32_WINNT >= _WIN32_WINNT_LONGHORN)
#define CLIP_EMBEDDED           (8<<4)

#define DEFAULT_QUALITY         0
#define DRAFT_QUALITY           1
#define PROOF_QUALITY           2
#if(WINVER >= 0x0400)
#define NONANTIALIASED_QUALITY  3
#define ANTIALIASED_QUALITY     4
#endif /* WINVER >= 0x0400 */

#if (_WIN32_WINNT >= _WIN32_WINNT_WINXP)
#define CLEARTYPE_QUALITY       5
#define CLEARTYPE_NATURAL_QUALITY       6
#endif

#define DEFAULT_PITCH           0
#define FIXED_PITCH             1
#define VARIABLE_PITCH          2
#if(WINVER >= 0x0400)
#define MONO_FONT               8
#endif /* WINVER >= 0x0400 */

#define ANSI_CHARSET            0
#define DEFAULT_CHARSET         1
#define SYMBOL_CHARSET          2
#define SHIFTJIS_CHARSET        128
#define HANGEUL_CHARSET         129
#define HANGUL_CHARSET          129
#define GB2312_CHARSET          134
#define CHINESEBIG5_CHARSET     136
#define OEM_CHARSET             255
#if(WINVER >= 0x0400)
#define JOHAB_CHARSET           130
#define HEBREW_CHARSET          177
#define ARABIC_CHARSET          178
#define GREEK_CHARSET           161
#define TURKISH_CHARSET         162
#define VIETNAMESE_CHARSET      163
#define THAI_CHARSET            222
#define EASTEUROPE_CHARSET      238
#define RUSSIAN_CHARSET         204

#define MAC_CHARSET             77
#define BALTIC_CHARSET          186

#define FS_LATIN1               0x00000001L
#define FS_LATIN2               0x00000002L
#define FS_CYRILLIC             0x00000004L
#define FS_GREEK                0x00000008L
#define FS_TURKISH              0x00000010L
#define FS_HEBREW               0x00000020L
#define FS_ARABIC               0x00000040L
#define FS_BALTIC               0x00000080L
#define FS_VIETNAMESE           0x00000100L
#define FS_THAI                 0x00010000L
#define FS_JISJAPAN             0x00020000L
#define FS_CHINESESIMP          0x00040000L
#define FS_WANSUNG              0x00080000L
#define FS_CHINESETRAD          0x00100000L
#define FS_JOHAB                0x00200000L
#define FS_SYMBOL               0x80000000L
#endif /* WINVER >= 0x0400 */

/* Font Families */
#define FF_DONTCARE         0x00    /* Don't care or don't know. */
#define FF_ROMAN            0x10    /* Variable stroke width, serifed. */
                                    /* Times Roman, Century Schoolbook, etc. */
#define FF_SWISS            0x20    /* Variable stroke width, sans-serifed. */
                                    /* Helvetica, Swiss, etc. */
#define FF_MODERN           0x30    /* Constant stroke width, serifed or sans-serifed. */
                                    /* Pica, Elite, Courier, etc. */
#define FF_SCRIPT           0x40    /* Cursive, etc. */
#define FF_DECORATIVE       0x50    /* Old English, etc. */

/* Font Weights */
#define FW_DONTCARE         0
#define FW_THIN             100
#define FW_EXTRALIGHT       200
#define FW_LIGHT            300
#define FW_NORMAL           400
#define FW_MEDIUM           500
#define FW_SEMIBOLD         600
#define FW_BOLD             700
#define FW_EXTRABOLD        800
#define FW_HEAVY            900

#define FW_ULTRALIGHT       FW_EXTRALIGHT
#define FW_REGULAR          FW_NORMAL
#define FW_DEMIBOLD         FW_SEMIBOLD
#define FW_ULTRABOLD        FW_EXTRABOLD
#define FW_BLACK            FW_HEAVY

#define PANOSE_COUNT               10
#define PAN_FAMILYTYPE_INDEX        0
#define PAN_SERIFSTYLE_INDEX        1
#define PAN_WEIGHT_INDEX            2
#define PAN_PROPORTION_INDEX        3
#define PAN_CONTRAST_INDEX          4
#define PAN_STROKEVARIATION_INDEX   5
#define PAN_ARMSTYLE_INDEX          6
#define PAN_LETTERFORM_INDEX        7
#define PAN_MIDLINE_INDEX           8
#define PAN_XHEIGHT_INDEX           9

#define PAN_CULTURE_LATIN           0

#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)

typedef struct tagPANOSE
{
    BYTE    bFamilyType;
    BYTE    bSerifStyle;
    BYTE    bWeight;
    BYTE    bProportion;
    BYTE    bContrast;
    BYTE    bStrokeVariation;
    BYTE    bArmStyle;
    BYTE    bLetterform;
    BYTE    bMidline;
    BYTE    bXHeight;
} PANOSE, * LPPANOSE;

#define PAN_ANY                         0 /* Any                            */
#define PAN_NO_FIT                      1 /* No Fit                         */

#define PAN_FAMILY_TEXT_DISPLAY         2 /* Text and Display               */
#define PAN_FAMILY_SCRIPT               3 /* Script                         */
#define PAN_FAMILY_DECORATIVE           4 /* Decorative                     */
#define PAN_FAMILY_PICTORIAL            5 /* Pictorial                      */

#define PAN_SERIF_COVE                  2 /* Cove                           */
#define PAN_SERIF_OBTUSE_COVE           3 /* Obtuse Cove                    */
#define PAN_SERIF_SQUARE_COVE           4 /* Square Cove                    */
#define PAN_SERIF_OBTUSE_SQUARE_COVE    5 /* Obtuse Square Cove             */
#define PAN_SERIF_SQUARE                6 /* Square                         */
#define PAN_SERIF_THIN                  7 /* Thin                           */
#define PAN_SERIF_BONE                  8 /* Bone                           */
#define PAN_SERIF_EXAGGERATED           9 /* Exaggerated                    */
#define PAN_SERIF_TRIANGLE             10 /* Triangle                       */
#define PAN_SERIF_NORMAL_SANS          11 /* Normal Sans                    */
#define PAN_SERIF_OBTUSE_SANS          12 /* Obtuse Sans                    */
#define PAN_SERIF_PERP_SANS            13 /* Prep Sans                      */
#define PAN_SERIF_FLARED               14 /* Flared                         */
#define PAN_SERIF_ROUNDED              15 /* Rounded                        */

#define PAN_WEIGHT_VERY_LIGHT           2 /* Very Light                     */
#define PAN_WEIGHT_LIGHT                3 /* Light                          */
#define PAN_WEIGHT_THIN                 4 /* Thin                           */
#define PAN_WEIGHT_BOOK                 5 /* Book                           */
#define PAN_WEIGHT_MEDIUM               6 /* Medium                         */
#define PAN_WEIGHT_DEMI                 7 /* Demi                           */
#define PAN_WEIGHT_BOLD                 8 /* Bold                           */
#define PAN_WEIGHT_HEAVY                9 /* Heavy                          */
#define PAN_WEIGHT_BLACK               10 /* Black                          */
#define PAN_WEIGHT_NORD                11 /* Nord                           */

#define PAN_PROP_OLD_STYLE              2 /* Old Style                      */
#define PAN_PROP_MODERN                 3 /* Modern                         */
#define PAN_PROP_EVEN_WIDTH             4 /* Even Width                     */
#define PAN_PROP_EXPANDED               5 /* Expanded                       */
#define PAN_PROP_CONDENSED              6 /* Condensed                      */
#define PAN_PROP_VERY_EXPANDED          7 /* Very Expanded                  */
#define PAN_PROP_VERY_CONDENSED         8 /* Very Condensed                 */
#define PAN_PROP_MONOSPACED             9 /* Monospaced                     */

#define PAN_CONTRAST_NONE               2 /* None                           */
#define PAN_CONTRAST_VERY_LOW           3 /* Very Low                       */
#define PAN_CONTRAST_LOW                4 /* Low                            */
#define PAN_CONTRAST_MEDIUM_LOW         5 /* Medium Low                     */
#define PAN_CONTRAST_MEDIUM             6 /* Medium                         */
#define PAN_CONTRAST_MEDIUM_HIGH        7 /* Mediim High                    */
#define PAN_CONTRAST_HIGH               8 /* High                           */
#define PAN_CONTRAST_VERY_HIGH          9 /* Very High                      */

#define PAN_STROKE_GRADUAL_DIAG         2 /* Gradual/Diagonal               */
#define PAN_STROKE_GRADUAL_TRAN         3 /* Gradual/Transitional           */
#define PAN_STROKE_GRADUAL_VERT         4 /* Gradual/Vertical               */
#define PAN_STROKE_GRADUAL_HORZ         5 /* Gradual/Horizontal             */
#define PAN_STROKE_RAPID_VERT           6 /* Rapid/Vertical                 */
#define PAN_STROKE_RAPID_HORZ           7 /* Rapid/Horizontal               */
#define PAN_STROKE_INSTANT_VERT         8 /* Instant/Vertical               */

#define PAN_STRAIGHT_ARMS_HORZ          2 /* Straight Arms/Horizontal       */
#define PAN_STRAIGHT_ARMS_WEDGE         3 /* Straight Arms/Wedge            */
#define PAN_STRAIGHT_ARMS_VERT          4 /* Straight Arms/Vertical         */
#define PAN_STRAIGHT_ARMS_SINGLE_SERIF  5 /* Straight Arms/Single-Serif     */
#define PAN_STRAIGHT_ARMS_DOUBLE_SERIF  6 /* Straight Arms/Double-Serif     */
#define PAN_BENT_ARMS_HORZ              7 /* Non-Straight Arms/Horizontal   */
#define PAN_BENT_ARMS_WEDGE             8 /* Non-Straight Arms/Wedge        */
#define PAN_BENT_ARMS_VERT              9 /* Non-Straight Arms/Vertical     */
#define PAN_BENT_ARMS_SINGLE_SERIF     10 /* Non-Straight Arms/Single-Serif */
#define PAN_BENT_ARMS_DOUBLE_SERIF     11 /* Non-Straight Arms/Double-Serif */

#define PAN_LETT_NORMAL_CONTACT         2 /* Normal/Contact                 */
#define PAN_LETT_NORMAL_WEIGHTED        3 /* Normal/Weighted                */
#define PAN_LETT_NORMAL_BOXED           4 /* Normal/Boxed                   */
#define PAN_LETT_NORMAL_FLATTENED       5 /* Normal/Flattened               */
#define PAN_LETT_NORMAL_ROUNDED         6 /* Normal/Rounded                 */
#define PAN_LETT_NORMAL_OFF_CENTER      7 /* Normal/Off Center              */
#define PAN_LETT_NORMAL_SQUARE          8 /* Normal/Square                  */
#define PAN_LETT_OBLIQUE_CONTACT        9 /* Oblique/Contact                */
#define PAN_LETT_OBLIQUE_WEIGHTED      10 /* Oblique/Weighted               */
#define PAN_LETT_OBLIQUE_BOXED         11 /* Oblique/Boxed                  */
#define PAN_LETT_OBLIQUE_FLATTENED     12 /* Oblique/Flattened              */
#define PAN_LETT_OBLIQUE_ROUNDED       13 /* Oblique/Rounded                */
#define PAN_LETT_OBLIQUE_OFF_CENTER    14 /* Oblique/Off Center             */
#define PAN_LETT_OBLIQUE_SQUARE        15 /* Oblique/Square                 */

#define PAN_MIDLINE_STANDARD_TRIMMED    2 /* Standard/Trimmed               */
#define PAN_MIDLINE_STANDARD_POINTED    3 /* Standard/Pointed               */
#define PAN_MIDLINE_STANDARD_SERIFED    4 /* Standard/Serifed               */
#define PAN_MIDLINE_HIGH_TRIMMED        5 /* High/Trimmed                   */
#define PAN_MIDLINE_HIGH_POINTED        6 /* High/Pointed                   */
#define PAN_MIDLINE_HIGH_SERIFED        7 /* High/Serifed                   */
#define PAN_MIDLINE_CONSTANT_TRIMMED    8 /* Constant/Trimmed               */
#define PAN_MIDLINE_CONSTANT_POINTED    9 /* Constant/Pointed               */
#define PAN_MIDLINE_CONSTANT_SERIFED   10 /* Constant/Serifed               */
#define PAN_MIDLINE_LOW_TRIMMED        11 /* Low/Trimmed                    */
#define PAN_MIDLINE_LOW_POINTED        12 /* Low/Pointed                    */
#define PAN_MIDLINE_LOW_SERIFED        13 /* Low/Serifed                    */

#define PAN_XHEIGHT_CONSTANT_SMALL      2 /* Constant/Small                 */
#define PAN_XHEIGHT_CONSTANT_STD        3 /* Constant/Standard              */
#define PAN_XHEIGHT_CONSTANT_LARGE      4 /* Constant/Large                 */
#define PAN_XHEIGHT_DUCKING_SMALL       5 /* Ducking/Small                  */
#define PAN_XHEIGHT_DUCKING_STD         6 /* Ducking/Standard               */
#define PAN_XHEIGHT_DUCKING_LARGE       7 /* Ducking/Large                  */


#define ELF_VENDOR_SIZE     4

/* The extended logical font       */
/* An extension of the ENUMLOGFONT */

typedef struct tagEXTLOGFONTA {
    LOGFONTA    elfLogFont;
    BYTE        elfFullName[LF_FULLFACESIZE];
    BYTE        elfStyle[LF_FACESIZE];
    DWORD       elfVersion;     /* 0 for the first release of NT */
    DWORD       elfStyleSize;
    DWORD       elfMatch;
    DWORD       elfReserved;
    BYTE        elfVendorId[ELF_VENDOR_SIZE];
    DWORD       elfCulture;     /* 0 for Latin                   */
    PANOSE      elfPanose;
} EXTLOGFONTA, *PEXTLOGFONTA, NEAR *NPEXTLOGFONTA, FAR *LPEXTLOGFONTA;
typedef struct tagEXTLOGFONTW {
    LOGFONTW    elfLogFont;
    WCHAR       elfFullName[LF_FULLFACESIZE];
    WCHAR       elfStyle[LF_FACESIZE];
    DWORD       elfVersion;     /* 0 for the first release of NT */
    DWORD       elfStyleSize;
    DWORD       elfMatch;
    DWORD       elfReserved;
    BYTE        elfVendorId[ELF_VENDOR_SIZE];
    DWORD       elfCulture;     /* 0 for Latin                   */
    PANOSE      elfPanose;
} EXTLOGFONTW, *PEXTLOGFONTW, NEAR *NPEXTLOGFONTW, FAR *LPEXTLOGFONTW;
#ifdef UNICODE
typedef EXTLOGFONTW EXTLOGFONT;
typedef PEXTLOGFONTW PEXTLOGFONT;
typedef NPEXTLOGFONTW NPEXTLOGFONT;
typedef LPEXTLOGFONTW LPEXTLOGFONT;
#else
typedef EXTLOGFONTA EXTLOGFONT;
typedef PEXTLOGFONTA PEXTLOGFONT;
typedef NPEXTLOGFONTA NPEXTLOGFONT;
typedef LPEXTLOGFONTA LPEXTLOGFONT;
#endif // UNICODE

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#define ELF_VERSION         0
#define ELF_CULTURE_LATIN   0

/* EnumFonts Masks */
#define RASTER_FONTTYPE     0x0001
#define DEVICE_FONTTYPE     0x0002
#define TRUETYPE_FONTTYPE   0x0004

#define RGB(r,g,b)          ((COLORREF)(((BYTE)(r)|((WORD)((BYTE)(g))<<8))|(((DWORD)(BYTE)(b))<<16)))
#define PALETTERGB(r,g,b)   (0x02000000 | RGB(r,g,b))
#define PALETTEINDEX(i)     ((COLORREF)(0x01000000 | (DWORD)(WORD)(i)))

/* palette entry flags */

#define PC_RESERVED     0x01    /* palette index used for animation */
#define PC_EXPLICIT     0x02    /* palette index is explicit to device */
#define PC_NOCOLLAPSE   0x04    /* do not match color to system palette */

#define GetRValue(rgb)      (LOBYTE(rgb))
#define GetGValue(rgb)      (LOBYTE(((WORD)(rgb)) >> 8))
#define GetBValue(rgb)      (LOBYTE((rgb)>>16))

/* Background Modes */
#define TRANSPARENT         1
#define OPAQUE              2
#define BKMODE_LAST         2

/* Graphics Modes */

#define GM_COMPATIBLE       1
#define GM_ADVANCED         2
#define GM_LAST             2

/* PolyDraw and GetPath point types */
#define PT_CLOSEFIGURE      0x01
#define PT_LINETO           0x02
#define PT_BEZIERTO         0x04
#define PT_MOVETO           0x06

/* Mapping Modes */
#define MM_TEXT             1
#define MM_LOMETRIC         2
#define MM_HIMETRIC         3
#define MM_LOENGLISH        4
#define MM_HIENGLISH        5
#define MM_TWIPS            6
#define MM_ISOTROPIC        7
#define MM_ANISOTROPIC      8

/* Min and Max Mapping Mode values */
#define MM_MIN              MM_TEXT
#define MM_MAX              MM_ANISOTROPIC
#define MM_MAX_FIXEDSCALE   MM_TWIPS

/* Coordinate Modes */
#define ABSOLUTE            1
#define RELATIVE            2

/* Stock Logical Objects */
#define WHITE_BRUSH         0
#define LTGRAY_BRUSH        1
#define GRAY_BRUSH          2
#define DKGRAY_BRUSH        3
#define BLACK_BRUSH         4
#define NULL_BRUSH          5
#define HOLLOW_BRUSH        NULL_BRUSH
#define WHITE_PEN           6
#define BLACK_PEN           7
#define NULL_PEN            8
#define OEM_FIXED_FONT      10
#define ANSI_FIXED_FONT     11
#define ANSI_VAR_FONT       12
#define SYSTEM_FONT         13
#define DEVICE_DEFAULT_FONT 14
#define DEFAULT_PALETTE     15
#define SYSTEM_FIXED_FONT   16

#if(WINVER >= 0x0400)
#define DEFAULT_GUI_FONT    17
#endif /* WINVER >= 0x0400 */

#if (_WIN32_WINNT >= _WIN32_WINNT_WIN2K)
#define DC_BRUSH            18
#define DC_PEN              19
#endif

#if (_WIN32_WINNT >= _WIN32_WINNT_WIN2K)
#define STOCK_LAST          19
#elif (WINVER >= 0x0400)
#define STOCK_LAST          17
#else
#define STOCK_LAST          16
#endif

#define CLR_INVALID     0xFFFFFFFF

/* Brush Styles */
#define BS_SOLID            0
#define BS_NULL             1
#define BS_HOLLOW           BS_NULL
#define BS_HATCHED          2
#define BS_PATTERN          3
#define BS_INDEXED          4
#define BS_DIBPATTERN       5
#define BS_DIBPATTERNPT     6
#define BS_PATTERN8X8       7
#define BS_DIBPATTERN8X8    8
#define BS_MONOPATTERN      9

/* Hatch Styles */
#define HS_HORIZONTAL       0       /* ----- */
#define HS_VERTICAL         1       /* ||||| */
#define HS_FDIAGONAL        2       /* \\\\\ */
#define HS_BDIAGONAL        3       /* ///// */
#define HS_CROSS            4       /* +++++ */
#define HS_DIAGCROSS        5       /* xxxxx */
#define HS_API_MAX          12

/* Pen Styles */
#define PS_SOLID            0
#define PS_DASH             1       /* -------  */
#define PS_DOT              2       /* .......  */
#define PS_DASHDOT          3       /* _._._._  */
#define PS_DASHDOTDOT       4       /* _.._.._  */
#define PS_NULL             5
#define PS_INSIDEFRAME      6
#define PS_USERSTYLE        7
#define PS_ALTERNATE        8
#define PS_STYLE_MASK       0x0000000F

#define PS_ENDCAP_ROUND     0x00000000
#define PS_ENDCAP_SQUARE    0x00000100
#define PS_ENDCAP_FLAT      0x00000200
#define PS_ENDCAP_MASK      0x00000F00

#define PS_JOIN_ROUND       0x00000000
#define PS_JOIN_BEVEL       0x00001000
#define PS_JOIN_MITER       0x00002000
#define PS_JOIN_MASK        0x0000F000

#define PS_COSMETIC         0x00000000
#define PS_GEOMETRIC        0x00010000
#define PS_TYPE_MASK        0x000F0000

#define AD_COUNTERCLOCKWISE 1
#define AD_CLOCKWISE        2

/* Device Parameters for GetDeviceCaps() */
#define DRIVERVERSION 0     /* Device driver version                    */
#define TECHNOLOGY    2     /* Device classification                    */
#define HORZSIZE      4     /* Horizontal size in millimeters           */
#define VERTSIZE      6     /* Vertical size in millimeters             */
#define HORZRES       8     /* Horizontal width in pixels               */
#define VERTRES       10    /* Vertical height in pixels                */
#define BITSPIXEL     12    /* Number of bits per pixel                 */
#define PLANES        14    /* Number of planes                         */
#define NUMBRUSHES    16    /* Number of brushes the device has         */
#define NUMPENS       18    /* Number of pens the device has            */
#define NUMMARKERS    20    /* Number of markers the device has         */
#define NUMFONTS      22    /* Number of fonts the device has           */
#define NUMCOLORS     24    /* Number of colors the device supports     */
#define PDEVICESIZE   26    /* Size required for device descriptor      */
#define CURVECAPS     28    /* Curve capabilities                       */
#define LINECAPS      30    /* Line capabilities                        */
#define POLYGONALCAPS 32    /* Polygonal capabilities                   */
#define TEXTCAPS      34    /* Text capabilities                        */
#define CLIPCAPS      36    /* Clipping capabilities                    */
#define RASTERCAPS    38    /* Bitblt capabilities                      */
#define ASPECTX       40    /* Length of the X leg                      */
#define ASPECTY       42    /* Length of the Y leg                      */
#define ASPECTXY      44    /* Length of the hypotenuse                 */

#define LOGPIXELSX    88    /* Logical pixels/inch in X                 */
#define LOGPIXELSY    90    /* Logical pixels/inch in Y                 */

#define SIZEPALETTE  104    /* Number of entries in physical palette    */
#define NUMRESERVED  106    /* Number of reserved entries in palette    */
#define COLORRES     108    /* Actual color resolution                  */

// Printing related DeviceCaps. These replace the appropriate Escapes

#define PHYSICALWIDTH   110 /* Physical Width in device units           */
#define PHYSICALHEIGHT  111 /* Physical Height in device units          */
#define PHYSICALOFFSETX 112 /* Physical Printable Area x margin         */
#define PHYSICALOFFSETY 113 /* Physical Printable Area y margin         */
#define SCALINGFACTORX  114 /* Scaling factor x                         */
#define SCALINGFACTORY  115 /* Scaling factor y                         */

// Display driver specific

#define VREFRESH        116  /* Current vertical refresh rate of the    */
                             /* display device (for displays only) in Hz*/
#define DESKTOPVERTRES  117  /* Horizontal width of entire desktop in   */
                             /* pixels                                  */
#define DESKTOPHORZRES  118  /* Vertical height of entire desktop in    */
                             /* pixels                                  */
#define BLTALIGNMENT    119  /* Preferred blt alignment                 */

#if(WINVER >= 0x0500)
#define SHADEBLENDCAPS  120  /* Shading and blending caps               */
#define COLORMGMTCAPS   121  /* Color Management caps                   */
#endif /* WINVER >= 0x0500 */

#ifndef NOGDICAPMASKS

/* Device Capability Masks: */

/* Device Technologies */
#define DT_PLOTTER          0   /* Vector plotter                   */
#define DT_RASDISPLAY       1   /* Raster display                   */
#define DT_RASPRINTER       2   /* Raster printer                   */
#define DT_RASCAMERA        3   /* Raster camera                    */
#define DT_CHARSTREAM       4   /* Character-stream, PLP            */
#define DT_METAFILE         5   /* Metafile, VDM                    */
#define DT_DISPFILE         6   /* Display-file                     */

/* Curve Capabilities */
#define CC_NONE             0   /* Curves not supported             */
#define CC_CIRCLES          1   /* Can do circles                   */
#define CC_PIE              2   /* Can do pie wedges                */
#define CC_CHORD            4   /* Can do chord arcs                */
#define CC_ELLIPSES         8   /* Can do ellipese                  */
#define CC_WIDE             16  /* Can do wide lines                */
#define CC_STYLED           32  /* Can do styled lines              */
#define CC_WIDESTYLED       64  /* Can do wide styled lines         */
#define CC_INTERIORS        128 /* Can do interiors                 */
#define CC_ROUNDRECT        256 /*                                  */

/* Line Capabilities */
#define LC_NONE             0   /* Lines not supported              */
#define LC_POLYLINE         2   /* Can do polylines                 */
#define LC_MARKER           4   /* Can do markers                   */
#define LC_POLYMARKER       8   /* Can do polymarkers               */
#define LC_WIDE             16  /* Can do wide lines                */
#define LC_STYLED           32  /* Can do styled lines              */
#define LC_WIDESTYLED       64  /* Can do wide styled lines         */
#define LC_INTERIORS        128 /* Can do interiors                 */

/* Polygonal Capabilities */
#define PC_NONE             0   /* Polygonals not supported         */
#define PC_POLYGON          1   /* Can do polygons                  */
#define PC_RECTANGLE        2   /* Can do rectangles                */
#define PC_WINDPOLYGON      4   /* Can do winding polygons          */
#define PC_TRAPEZOID        4   /* Can do trapezoids                */
#define PC_SCANLINE         8   /* Can do scanlines                 */
#define PC_WIDE             16  /* Can do wide borders              */
#define PC_STYLED           32  /* Can do styled borders            */
#define PC_WIDESTYLED       64  /* Can do wide styled borders       */
#define PC_INTERIORS        128 /* Can do interiors                 */
#define PC_POLYPOLYGON      256 /* Can do polypolygons              */
#define PC_PATHS            512 /* Can do paths                     */

/* Clipping Capabilities */
#define CP_NONE             0   /* No clipping of output            */
#define CP_RECTANGLE        1   /* Output clipped to rects          */
#define CP_REGION           2   /* obsolete                         */

/* Text Capabilities */
#define TC_OP_CHARACTER     0x00000001  /* Can do OutputPrecision   CHARACTER      */
#define TC_OP_STROKE        0x00000002  /* Can do OutputPrecision   STROKE         */
#define TC_CP_STROKE        0x00000004  /* Can do ClipPrecision     STROKE         */
#define TC_CR_90            0x00000008  /* Can do CharRotAbility    90             */
#define TC_CR_ANY           0x00000010  /* Can do CharRotAbility    ANY            */
#define TC_SF_X_YINDEP      0x00000020  /* Can do ScaleFreedom      X_YINDEPENDENT */
#define TC_SA_DOUBLE        0x00000040  /* Can do ScaleAbility      DOUBLE         */
#define TC_SA_INTEGER       0x00000080  /* Can do ScaleAbility      INTEGER        */
#define TC_SA_CONTIN        0x00000100  /* Can do ScaleAbility      CONTINUOUS     */
#define TC_EA_DOUBLE        0x00000200  /* Can do EmboldenAbility   DOUBLE         */
#define TC_IA_ABLE          0x00000400  /* Can do ItalisizeAbility  ABLE           */
#define TC_UA_ABLE          0x00000800  /* Can do UnderlineAbility  ABLE           */
#define TC_SO_ABLE          0x00001000  /* Can do StrikeOutAbility  ABLE           */
#define TC_RA_ABLE          0x00002000  /* Can do RasterFontAble    ABLE           */
#define TC_VA_ABLE          0x00004000  /* Can do VectorFontAble    ABLE           */
#define TC_RESERVED         0x00008000
#define TC_SCROLLBLT        0x00010000  /* Don't do text scroll with blt           */

#endif /* NOGDICAPMASKS */

/* Raster Capabilities */
#define RC_NONE
#define RC_BITBLT           1       /* Can do standard BLT.             */
#define RC_BANDING          2       /* Device requires banding support  */
#define RC_SCALING          4       /* Device requires scaling support  */
#define RC_BITMAP64         8       /* Device can support >64K bitmap   */
#define RC_GDI20_OUTPUT     0x0010      /* has 2.0 output calls         */
#define RC_GDI20_STATE      0x0020
#define RC_SAVEBITMAP       0x0040
#define RC_DI_BITMAP        0x0080      /* supports DIB to memory       */
#define RC_PALETTE          0x0100      /* supports a palette           */
#define RC_DIBTODEV         0x0200      /* supports DIBitsToDevice      */
#define RC_BIGFONT          0x0400      /* supports >64K fonts          */
#define RC_STRETCHBLT       0x0800      /* supports StretchBlt          */
#define RC_FLOODFILL        0x1000      /* supports FloodFill           */
#define RC_STRETCHDIB       0x2000      /* supports StretchDIBits       */
#define RC_OP_DX_OUTPUT     0x4000
#define RC_DEVBITS          0x8000

#if(WINVER >= 0x0500)

/* Shading and blending caps */
#define SB_NONE             0x00000000
#define SB_CONST_ALPHA      0x00000001
#define SB_PIXEL_ALPHA      0x00000002
#define SB_PREMULT_ALPHA    0x00000004

#define SB_GRAD_RECT        0x00000010
#define SB_GRAD_TRI         0x00000020

/* Color Management caps */
#define CM_NONE             0x00000000
#define CM_DEVICE_ICM       0x00000001
#define CM_GAMMA_RAMP       0x00000002
#define CM_CMYK_COLOR       0x00000004

#endif /* WINVER >= 0x0500 */


/* DIB color table identifiers */

#define DIB_RGB_COLORS      0 /* color table in RGBs */
#define DIB_PAL_COLORS      1 /* color table in palette indices */

/* constants for Get/SetSystemPaletteUse() */

#define SYSPAL_ERROR    0
#define SYSPAL_STATIC   1
#define SYSPAL_NOSTATIC 2
#define SYSPAL_NOSTATIC256 3

/* constants for CreateDIBitmap */
#define CBM_INIT        0x04L   /* initialize bitmap */


/* ExtFloodFill style flags */
#define  FLOODFILLBORDER   0
#define  FLOODFILLSURFACE  1

/* size of a device name string */
#define CCHDEVICENAME 32

/* size of a form name string */
#define CCHFORMNAME 32

#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)

#if (_WIN32_WINNT >= ((OSVER(NTDDI_WINXPSP2)) >> 16))
typedef struct _devicemodeA {
    BYTE   dmDeviceName[CCHDEVICENAME];
    WORD dmSpecVersion;
    WORD dmDriverVersion;
    WORD dmSize;
    WORD dmDriverExtra;
    DWORD dmFields;
    union {
      /* printer only fields */
      struct {
        short dmOrientation;
        short dmPaperSize;
        short dmPaperLength;
        short dmPaperWidth;
        short dmScale;
        short dmCopies;
        short dmDefaultSource;
        short dmPrintQuality;
      } DUMMYSTRUCTNAME;
      /* display only fields */
      struct {
        POINTL dmPosition;
        DWORD  dmDisplayOrientation;
        DWORD  dmDisplayFixedOutput;
      } DUMMYSTRUCTNAME2;
    } DUMMYUNIONNAME;
    short dmColor;
    short dmDuplex;
    short dmYResolution;
    short dmTTOption;
    short dmCollate;
    BYTE   dmFormName[CCHFORMNAME];
    WORD   dmLogPixels;
    DWORD  dmBitsPerPel;
    DWORD  dmPelsWidth;
    DWORD  dmPelsHeight;
    union {
        DWORD  dmDisplayFlags;
        DWORD  dmNup;
    } DUMMYUNIONNAME2;
    DWORD  dmDisplayFrequency;
#if(WINVER >= 0x0400)
    DWORD  dmICMMethod;
    DWORD  dmICMIntent;
    DWORD  dmMediaType;
    DWORD  dmDitherType;
    DWORD  dmReserved1;
    DWORD  dmReserved2;
#if (WINVER >= 0x0500) || (_WIN32_WINNT >= _WIN32_WINNT_NT4)
    DWORD  dmPanningWidth;
    DWORD  dmPanningHeight;
#endif
#endif /* WINVER >= 0x0400 */
} DEVMODEA, *PDEVMODEA, *NPDEVMODEA, *LPDEVMODEA;
typedef struct _devicemodeW {
    WCHAR  dmDeviceName[CCHDEVICENAME];
    WORD dmSpecVersion;
    WORD dmDriverVersion;
    WORD dmSize;
    WORD dmDriverExtra;
    DWORD dmFields;
    union {
      /* printer only fields */
      struct {
        short dmOrientation;
        short dmPaperSize;
        short dmPaperLength;
        short dmPaperWidth;
        short dmScale;
        short dmCopies;
        short dmDefaultSource;
        short dmPrintQuality;
      } DUMMYSTRUCTNAME;
      /* display only fields */
      struct {
        POINTL dmPosition;
        DWORD  dmDisplayOrientation;
        DWORD  dmDisplayFixedOutput;
      } DUMMYSTRUCTNAME2;
    } DUMMYUNIONNAME;
    short dmColor;
    short dmDuplex;
    short dmYResolution;
    short dmTTOption;
    short dmCollate;
    WCHAR  dmFormName[CCHFORMNAME];
    WORD   dmLogPixels;
    DWORD  dmBitsPerPel;
    DWORD  dmPelsWidth;
    DWORD  dmPelsHeight;
    union {
        DWORD  dmDisplayFlags;
        DWORD  dmNup;
    } DUMMYUNIONNAME2;
    DWORD  dmDisplayFrequency;
#if(WINVER >= 0x0400)
    DWORD  dmICMMethod;
    DWORD  dmICMIntent;
    DWORD  dmMediaType;
    DWORD  dmDitherType;
    DWORD  dmReserved1;
    DWORD  dmReserved2;
#if (WINVER >= 0x0500) || (_WIN32_WINNT >= _WIN32_WINNT_NT4)
    DWORD  dmPanningWidth;
    DWORD  dmPanningHeight;
#endif
#endif /* WINVER >= 0x0400 */
} DEVMODEW, *PDEVMODEW, *NPDEVMODEW, *LPDEVMODEW;
#ifdef UNICODE
typedef DEVMODEW DEVMODE;
typedef PDEVMODEW PDEVMODE;
typedef NPDEVMODEW NPDEVMODE;
typedef LPDEVMODEW LPDEVMODE;
#else
typedef DEVMODEA DEVMODE;
typedef PDEVMODEA PDEVMODE;
typedef NPDEVMODEA NPDEVMODE;
typedef LPDEVMODEA LPDEVMODE;
#endif // UNICODE
#else
typedef struct _devicemodeA {
    BYTE   dmDeviceName[CCHDEVICENAME];
    WORD dmSpecVersion;
    WORD dmDriverVersion;
    WORD dmSize;
    WORD dmDriverExtra;
    DWORD dmFields;
    union {
      struct {
        short dmOrientation;
        short dmPaperSize;
        short dmPaperLength;
        short dmPaperWidth;
      } DUMMYSTRUCTNAME;
      POINTL dmPosition;
    } DUMMYUNIONNAME;
    short dmScale;
    short dmCopies;
    short dmDefaultSource;
    short dmPrintQuality;
    short dmColor;
    short dmDuplex;
    short dmYResolution;
    short dmTTOption;
    short dmCollate;
    BYTE   dmFormName[CCHFORMNAME];
    WORD   dmLogPixels;
    DWORD  dmBitsPerPel;
    DWORD  dmPelsWidth;
    DWORD  dmPelsHeight;
    union {
        DWORD  dmDisplayFlags;
        DWORD  dmNup;
    } DUMMYUNIONNAME2;
    DWORD  dmDisplayFrequency;
#if(WINVER >= 0x0400)
    DWORD  dmICMMethod;
    DWORD  dmICMIntent;
    DWORD  dmMediaType;
    DWORD  dmDitherType;
    DWORD  dmReserved1;
    DWORD  dmReserved2;
#if (WINVER >= 0x0500) || (_WIN32_WINNT >= _WIN32_WINNT_NT4)
    DWORD  dmPanningWidth;
    DWORD  dmPanningHeight;
#endif
#endif /* WINVER >= 0x0400 */
} DEVMODEA, *PDEVMODEA, *NPDEVMODEA, *LPDEVMODEA;
typedef struct _devicemodeW {
    WCHAR  dmDeviceName[CCHDEVICENAME];
    WORD dmSpecVersion;
    WORD dmDriverVersion;
    WORD dmSize;
    WORD dmDriverExtra;
    DWORD dmFields;
    union {
      struct {
        short dmOrientation;
        short dmPaperSize;
        short dmPaperLength;
        short dmPaperWidth;
      } DUMMYSTRUCTNAME;
      POINTL dmPosition;
    } DUMMYUNIONNAME;
    short dmScale;
    short dmCopies;
    short dmDefaultSource;
    short dmPrintQuality;
    short dmColor;
    short dmDuplex;
    short dmYResolution;
    short dmTTOption;
    short dmCollate;
    WCHAR  dmFormName[CCHFORMNAME];
    WORD   dmLogPixels;
    DWORD  dmBitsPerPel;
    DWORD  dmPelsWidth;
    DWORD  dmPelsHeight;
    union {
        DWORD  dmDisplayFlags;
        DWORD  dmNup;
    } DUMMYUNIONNAME2;
    DWORD  dmDisplayFrequency;
#if(WINVER >= 0x0400)
    DWORD  dmICMMethod;
    DWORD  dmICMIntent;
    DWORD  dmMediaType;
    DWORD  dmDitherType;
    DWORD  dmReserved1;
    DWORD  dmReserved2;
#if (WINVER >= 0x0500) || (_WIN32_WINNT >= _WIN32_WINNT_NT4)
    DWORD  dmPanningWidth;
    DWORD  dmPanningHeight;
#endif
#endif /* WINVER >= 0x0400 */
} DEVMODEW, *PDEVMODEW, *NPDEVMODEW, *LPDEVMODEW;
#ifdef UNICODE
typedef DEVMODEW DEVMODE;
typedef PDEVMODEW PDEVMODE;
typedef NPDEVMODEW NPDEVMODE;
typedef LPDEVMODEW LPDEVMODE;
#else
typedef DEVMODEA DEVMODE;
typedef PDEVMODEA PDEVMODE;
typedef NPDEVMODEA NPDEVMODE;
typedef LPDEVMODEA LPDEVMODE;
#endif // UNICODE
#endif // (_WIN32_WINNT >= _WIN32_WINNT_WINXP)

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

/* current version of specification */
#if (WINVER >= 0x0500) || (_WIN32_WINNT >= _WIN32_WINNT_NT4)
#define DM_SPECVERSION 0x0401
#elif (WINVER >= 0x0400)
#define DM_SPECVERSION 0x0400
#else
#define DM_SPECVERSION 0x0320
#endif /* WINVER */

/* field selection bits */
#define DM_ORIENTATION          0x00000001L
#define DM_PAPERSIZE            0x00000002L
#define DM_PAPERLENGTH          0x00000004L
#define DM_PAPERWIDTH           0x00000008L
#define DM_SCALE                0x00000010L
#if(WINVER >= 0x0500)
#define DM_POSITION             0x00000020L
#define DM_NUP                  0x00000040L
#endif /* WINVER >= 0x0500 */
#if(WINVER >= 0x0501)
#define DM_DISPLAYORIENTATION   0x00000080L
#endif /* WINVER >= 0x0501 */
#define DM_COPIES               0x00000100L
#define DM_DEFAULTSOURCE        0x00000200L
#define DM_PRINTQUALITY         0x00000400L
#define DM_COLOR                0x00000800L
#define DM_DUPLEX               0x00001000L
#define DM_YRESOLUTION          0x00002000L
#define DM_TTOPTION             0x00004000L
#define DM_COLLATE              0x00008000L
#define DM_FORMNAME             0x00010000L
#define DM_LOGPIXELS            0x00020000L
#define DM_BITSPERPEL           0x00040000L
#define DM_PELSWIDTH            0x00080000L
#define DM_PELSHEIGHT           0x00100000L
#define DM_DISPLAYFLAGS         0x00200000L
#define DM_DISPLAYFREQUENCY     0x00400000L
#if(WINVER >= 0x0400)
#define DM_ICMMETHOD            0x00800000L
#define DM_ICMINTENT            0x01000000L
#define DM_MEDIATYPE            0x02000000L
#define DM_DITHERTYPE           0x04000000L
#define DM_PANNINGWIDTH         0x08000000L
#define DM_PANNINGHEIGHT        0x10000000L
#endif /* WINVER >= 0x0400 */
#if(WINVER >= 0x0501)
#define DM_DISPLAYFIXEDOUTPUT   0x20000000L
#endif /* WINVER >= 0x0501 */


/* orientation selections */
#define DMORIENT_PORTRAIT   1
#define DMORIENT_LANDSCAPE  2

/* paper selections */
#define DMPAPER_FIRST                DMPAPER_LETTER
#define DMPAPER_LETTER               1  /* Letter 8 1/2 x 11 in               */
#define DMPAPER_LETTERSMALL          2  /* Letter Small 8 1/2 x 11 in         */
#define DMPAPER_TABLOID              3  /* Tabloid 11 x 17 in                 */
#define DMPAPER_LEDGER               4  /* Ledger 17 x 11 in                  */
#define DMPAPER_LEGAL                5  /* Legal 8 1/2 x 14 in                */
#define DMPAPER_STATEMENT            6  /* Statement 5 1/2 x 8 1/2 in         */
#define DMPAPER_EXECUTIVE            7  /* Executive 7 1/4 x 10 1/2 in        */
#define DMPAPER_A3                   8  /* A3 297 x 420 mm                    */
#define DMPAPER_A4                   9  /* A4 210 x 297 mm                    */
#define DMPAPER_A4SMALL             10  /* A4 Small 210 x 297 mm              */
#define DMPAPER_A5                  11  /* A5 148 x 210 mm                    */
#define DMPAPER_B4                  12  /* B4 (JIS) 250 x 354                 */
#define DMPAPER_B5                  13  /* B5 (JIS) 182 x 257 mm              */
#define DMPAPER_FOLIO               14  /* Folio 8 1/2 x 13 in                */
#define DMPAPER_QUARTO              15  /* Quarto 215 x 275 mm                */
#define DMPAPER_10X14               16  /* 10x14 in                           */
#define DMPAPER_11X17               17  /* 11x17 in                           */
#define DMPAPER_NOTE                18  /* Note 8 1/2 x 11 in                 */
#define DMPAPER_ENV_9               19  /* Envelope #9 3 7/8 x 8 7/8          */
#define DMPAPER_ENV_10              20  /* Envelope #10 4 1/8 x 9 1/2         */
#define DMPAPER_ENV_11              21  /* Envelope #11 4 1/2 x 10 3/8        */
#define DMPAPER_ENV_12              22  /* Envelope #12 4 \276 x 11           */
#define DMPAPER_ENV_14              23  /* Envelope #14 5 x 11 1/2            */
#define DMPAPER_CSHEET              24  /* C size sheet                       */
#define DMPAPER_DSHEET              25  /* D size sheet                       */
#define DMPAPER_ESHEET              26  /* E size sheet                       */
#define DMPAPER_ENV_DL              27  /* Envelope DL 110 x 220mm            */
#define DMPAPER_ENV_C5              28  /* Envelope C5 162 x 229 mm           */
#define DMPAPER_ENV_C3              29  /* Envelope C3  324 x 458 mm          */
#define DMPAPER_ENV_C4              30  /* Envelope C4  229 x 324 mm          */
#define DMPAPER_ENV_C6              31  /* Envelope C6  114 x 162 mm          */
#define DMPAPER_ENV_C65             32  /* Envelope C65 114 x 229 mm          */
#define DMPAPER_ENV_B4              33  /* Envelope B4  250 x 353 mm          */
#define DMPAPER_ENV_B5              34  /* Envelope B5  176 x 250 mm          */
#define DMPAPER_ENV_B6              35  /* Envelope B6  176 x 125 mm          */
#define DMPAPER_ENV_ITALY           36  /* Envelope 110 x 230 mm              */
#define DMPAPER_ENV_MONARCH         37  /* Envelope Monarch 3.875 x 7.5 in    */
#define DMPAPER_ENV_PERSONAL        38  /* 6 3/4 Envelope 3 5/8 x 6 1/2 in    */
#define DMPAPER_FANFOLD_US          39  /* US Std Fanfold 14 7/8 x 11 in      */
#define DMPAPER_FANFOLD_STD_GERMAN  40  /* German Std Fanfold 8 1/2 x 12 in   */
#define DMPAPER_FANFOLD_LGL_GERMAN  41  /* German Legal Fanfold 8 1/2 x 13 in */
#if(WINVER >= 0x0400)
#define DMPAPER_ISO_B4              42  /* B4 (ISO) 250 x 353 mm              */
#define DMPAPER_JAPANESE_POSTCARD   43  /* Japanese Postcard 100 x 148 mm     */
#define DMPAPER_9X11                44  /* 9 x 11 in                          */
#define DMPAPER_10X11               45  /* 10 x 11 in                         */
#define DMPAPER_15X11               46  /* 15 x 11 in                         */
#define DMPAPER_ENV_INVITE          47  /* Envelope Invite 220 x 220 mm       */
#define DMPAPER_RESERVED_48         48  /* RESERVED--DO NOT USE               */
#define DMPAPER_RESERVED_49         49  /* RESERVED--DO NOT USE               */
#define DMPAPER_LETTER_EXTRA        50  /* Letter Extra 9 \275 x 12 in        */
#define DMPAPER_LEGAL_EXTRA         51  /* Legal Extra 9 \275 x 15 in         */
#define DMPAPER_TABLOID_EXTRA       52  /* Tabloid Extra 11.69 x 18 in        */
#define DMPAPER_A4_EXTRA            53  /* A4 Extra 9.27 x 12.69 in           */
#define DMPAPER_LETTER_TRANSVERSE   54  /* Letter Transverse 8 \275 x 11 in   */
#define DMPAPER_A4_TRANSVERSE       55  /* A4 Transverse 210 x 297 mm         */
#define DMPAPER_LETTER_EXTRA_TRANSVERSE 56 /* Letter Extra Transverse 9\275 x 12 in */
#define DMPAPER_A_PLUS              57  /* SuperA/SuperA/A4 227 x 356 mm      */
#define DMPAPER_B_PLUS              58  /* SuperB/SuperB/A3 305 x 487 mm      */
#define DMPAPER_LETTER_PLUS         59  /* Letter Plus 8.5 x 12.69 in         */
#define DMPAPER_A4_PLUS             60  /* A4 Plus 210 x 330 mm               */
#define DMPAPER_A5_TRANSVERSE       61  /* A5 Transverse 148 x 210 mm         */
#define DMPAPER_B5_TRANSVERSE       62  /* B5 (JIS) Transverse 182 x 257 mm   */
#define DMPAPER_A3_EXTRA            63  /* A3 Extra 322 x 445 mm              */
#define DMPAPER_A5_EXTRA            64  /* A5 Extra 174 x 235 mm              */
#define DMPAPER_B5_EXTRA            65  /* B5 (ISO) Extra 201 x 276 mm        */
#define DMPAPER_A2                  66  /* A2 420 x 594 mm                    */
#define DMPAPER_A3_TRANSVERSE       67  /* A3 Transverse 297 x 420 mm         */
#define DMPAPER_A3_EXTRA_TRANSVERSE 68  /* A3 Extra Transverse 322 x 445 mm   */
#endif /* WINVER >= 0x0400 */

#if(WINVER >= 0x0500)
#define DMPAPER_DBL_JAPANESE_POSTCARD 69 /* Japanese Double Postcard 200 x 148 mm */
#define DMPAPER_A6                  70  /* A6 105 x 148 mm                 */
#define DMPAPER_JENV_KAKU2          71  /* Japanese Envelope Kaku #2       */
#define DMPAPER_JENV_KAKU3          72  /* Japanese Envelope Kaku #3       */
#define DMPAPER_JENV_CHOU3          73  /* Japanese Envelope Chou #3       */
#define DMPAPER_JENV_CHOU4          74  /* Japanese Envelope Chou #4       */
#define DMPAPER_LETTER_ROTATED      75  /* Letter Rotated 11 x 8 1/2 11 in */
#define DMPAPER_A3_ROTATED          76  /* A3 Rotated 420 x 297 mm         */
#define DMPAPER_A4_ROTATED          77  /* A4 Rotated 297 x 210 mm         */
#define DMPAPER_A5_ROTATED          78  /* A5 Rotated 210 x 148 mm         */
#define DMPAPER_B4_JIS_ROTATED      79  /* B4 (JIS) Rotated 364 x 257 mm   */
#define DMPAPER_B5_JIS_ROTATED      80  /* B5 (JIS) Rotated 257 x 182 mm   */
#define DMPAPER_JAPANESE_POSTCARD_ROTATED 81 /* Japanese Postcard Rotated 148 x 100 mm */
#define DMPAPER_DBL_JAPANESE_POSTCARD_ROTATED 82 /* Double Japanese Postcard Rotated 148 x 200 mm */
#define DMPAPER_A6_ROTATED          83  /* A6 Rotated 148 x 105 mm         */
#define DMPAPER_JENV_KAKU2_ROTATED  84  /* Japanese Envelope Kaku #2 Rotated */
#define DMPAPER_JENV_KAKU3_ROTATED  85  /* Japanese Envelope Kaku #3 Rotated */
#define DMPAPER_JENV_CHOU3_ROTATED  86  /* Japanese Envelope Chou #3 Rotated */
#define DMPAPER_JENV_CHOU4_ROTATED  87  /* Japanese Envelope Chou #4 Rotated */
#define DMPAPER_B6_JIS              88  /* B6 (JIS) 128 x 182 mm           */
#define DMPAPER_B6_JIS_ROTATED      89  /* B6 (JIS) Rotated 182 x 128 mm   */
#define DMPAPER_12X11               90  /* 12 x 11 in                      */
#define DMPAPER_JENV_YOU4           91  /* Japanese Envelope You #4        */
#define DMPAPER_JENV_YOU4_ROTATED   92  /* Japanese Envelope You #4 Rotated*/
#define DMPAPER_P16K                93  /* PRC 16K 146 x 215 mm            */
#define DMPAPER_P32K                94  /* PRC 32K 97 x 151 mm             */
#define DMPAPER_P32KBIG             95  /* PRC 32K(Big) 97 x 151 mm        */
#define DMPAPER_PENV_1              96  /* PRC Envelope #1 102 x 165 mm    */
#define DMPAPER_PENV_2              97  /* PRC Envelope #2 102 x 176 mm    */
#define DMPAPER_PENV_3              98  /* PRC Envelope #3 125 x 176 mm    */
#define DMPAPER_PENV_4              99  /* PRC Envelope #4 110 x 208 mm    */
#define DMPAPER_PENV_5              100 /* PRC Envelope #5 110 x 220 mm    */
#define DMPAPER_PENV_6              101 /* PRC Envelope #6 120 x 230 mm    */
#define DMPAPER_PENV_7              102 /* PRC Envelope #7 160 x 230 mm    */
#define DMPAPER_PENV_8              103 /* PRC Envelope #8 120 x 309 mm    */
#define DMPAPER_PENV_9              104 /* PRC Envelope #9 229 x 324 mm    */
#define DMPAPER_PENV_10             105 /* PRC Envelope #10 324 x 458 mm   */
#define DMPAPER_P16K_ROTATED        106 /* PRC 16K Rotated                 */
#define DMPAPER_P32K_ROTATED        107 /* PRC 32K Rotated                 */
#define DMPAPER_P32KBIG_ROTATED     108 /* PRC 32K(Big) Rotated            */
#define DMPAPER_PENV_1_ROTATED      109 /* PRC Envelope #1 Rotated 165 x 102 mm */
#define DMPAPER_PENV_2_ROTATED      110 /* PRC Envelope #2 Rotated 176 x 102 mm */
#define DMPAPER_PENV_3_ROTATED      111 /* PRC Envelope #3 Rotated 176 x 125 mm */
#define DMPAPER_PENV_4_ROTATED      112 /* PRC Envelope #4 Rotated 208 x 110 mm */
#define DMPAPER_PENV_5_ROTATED      113 /* PRC Envelope #5 Rotated 220 x 110 mm */
#define DMPAPER_PENV_6_ROTATED      114 /* PRC Envelope #6 Rotated 230 x 120 mm */
#define DMPAPER_PENV_7_ROTATED      115 /* PRC Envelope #7 Rotated 230 x 160 mm */
#define DMPAPER_PENV_8_ROTATED      116 /* PRC Envelope #8 Rotated 309 x 120 mm */
#define DMPAPER_PENV_9_ROTATED      117 /* PRC Envelope #9 Rotated 324 x 229 mm */
#define DMPAPER_PENV_10_ROTATED     118 /* PRC Envelope #10 Rotated 458 x 324 mm */
#endif /* WINVER >= 0x0500 */

#if (WINVER >= 0x0500)
#define DMPAPER_LAST                DMPAPER_PENV_10_ROTATED
#elif (WINVER >= 0x0400)
#define DMPAPER_LAST                DMPAPER_A3_EXTRA_TRANSVERSE
#else
#define DMPAPER_LAST                DMPAPER_FANFOLD_LGL_GERMAN
#endif

#define DMPAPER_USER                256

/* bin selections */
#define DMBIN_FIRST         DMBIN_UPPER
#define DMBIN_UPPER         1
#define DMBIN_ONLYONE       1
#define DMBIN_LOWER         2
#define DMBIN_MIDDLE        3
#define DMBIN_MANUAL        4
#define DMBIN_ENVELOPE      5
#define DMBIN_ENVMANUAL     6
#define DMBIN_AUTO          7
#define DMBIN_TRACTOR       8
#define DMBIN_SMALLFMT      9
#define DMBIN_LARGEFMT      10
#define DMBIN_LARGECAPACITY 11
#define DMBIN_CASSETTE      14
#define DMBIN_FORMSOURCE    15
#define DMBIN_LAST          DMBIN_FORMSOURCE

#define DMBIN_USER          256     /* device specific bins start here */

/* print qualities */
#define DMRES_DRAFT         (-1)
#define DMRES_LOW           (-2)
#define DMRES_MEDIUM        (-3)
#define DMRES_HIGH          (-4)

/* color enable/disable for color printers */
#define DMCOLOR_MONOCHROME  1
#define DMCOLOR_COLOR       2

/* duplex enable */
#define DMDUP_SIMPLEX    1
#define DMDUP_VERTICAL   2
#define DMDUP_HORIZONTAL 3

/* TrueType options */
#define DMTT_BITMAP     1       /* print TT fonts as graphics */
#define DMTT_DOWNLOAD   2       /* download TT fonts as soft fonts */
#define DMTT_SUBDEV     3       /* substitute device fonts for TT fonts */
#if(WINVER >= 0x0400)
#define DMTT_DOWNLOAD_OUTLINE 4 /* download TT fonts as outline soft fonts */
#endif /* WINVER >= 0x0400 */

/* Collation selections */
#define DMCOLLATE_FALSE  0
#define DMCOLLATE_TRUE   1

#if(WINVER >= 0x0501)
/* DEVMODE dmDisplayOrientation specifiations */
#define DMDO_DEFAULT    0
#define DMDO_90         1
#define DMDO_180        2
#define DMDO_270        3

/* DEVMODE dmDisplayFixedOutput specifiations */
#define DMDFO_DEFAULT   0
#define DMDFO_STRETCH   1
#define DMDFO_CENTER    2
#endif /* WINVER >= 0x0501 */

/* DEVMODE dmDisplayFlags flags */

// #define DM_GRAYSCALE            0x00000001 /* This flag is no longer valid */
#define DM_INTERLACED           0x00000002
#define DMDISPLAYFLAGS_TEXTMODE 0x00000004

/* dmNup , multiple logical page per physical page options */
#define DMNUP_SYSTEM        1
#define DMNUP_ONEUP         2

#if(WINVER >= 0x0400)
/* ICM methods */
#define DMICMMETHOD_NONE    1   /* ICM disabled */
#define DMICMMETHOD_SYSTEM  2   /* ICM handled by system */
#define DMICMMETHOD_DRIVER  3   /* ICM handled by driver */
#define DMICMMETHOD_DEVICE  4   /* ICM handled by device */

#define DMICMMETHOD_USER  256   /* Device-specific methods start here */

/* ICM Intents */
#define DMICM_SATURATE          1   /* Maximize color saturation */
#define DMICM_CONTRAST          2   /* Maximize color contrast */
#define DMICM_COLORIMETRIC       3   /* Use specific color metric */
#define DMICM_ABS_COLORIMETRIC   4   /* Use specific color metric */

#define DMICM_USER        256   /* Device-specific intents start here */

/* Media types */

#define DMMEDIA_STANDARD      1   /* Standard paper */
#define DMMEDIA_TRANSPARENCY  2   /* Transparency */
#define DMMEDIA_GLOSSY        3   /* Glossy paper */

#define DMMEDIA_USER        256   /* Device-specific media start here */

/* Dither types */
#define DMDITHER_NONE       1      /* No dithering */
#define DMDITHER_COARSE     2      /* Dither with a coarse brush */
#define DMDITHER_FINE       3      /* Dither with a fine brush */
#define DMDITHER_LINEART    4      /* LineArt dithering */
#define DMDITHER_ERRORDIFFUSION 5  /* LineArt dithering */
#define DMDITHER_RESERVED6      6      /* LineArt dithering */
#define DMDITHER_RESERVED7      7      /* LineArt dithering */
#define DMDITHER_RESERVED8      8      /* LineArt dithering */
#define DMDITHER_RESERVED9      9      /* LineArt dithering */
#define DMDITHER_GRAYSCALE  10     /* Device does grayscaling */

#define DMDITHER_USER     256   /* Device-specific dithers start here */
#endif /* WINVER >= 0x0400 */

#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)

typedef struct _DISPLAY_DEVICEA {
    DWORD  cb;
    CHAR   DeviceName[32];
    CHAR   DeviceString[128];
    DWORD  StateFlags;
    CHAR   DeviceID[128];
    CHAR   DeviceKey[128];
} DISPLAY_DEVICEA, *PDISPLAY_DEVICEA, *LPDISPLAY_DEVICEA;
typedef struct _DISPLAY_DEVICEW {
    DWORD  cb;
    WCHAR  DeviceName[32];
    WCHAR  DeviceString[128];
    DWORD  StateFlags;
    WCHAR  DeviceID[128];
    WCHAR  DeviceKey[128];
} DISPLAY_DEVICEW, *PDISPLAY_DEVICEW, *LPDISPLAY_DEVICEW;
#ifdef UNICODE
typedef DISPLAY_DEVICEW DISPLAY_DEVICE;
typedef PDISPLAY_DEVICEW PDISPLAY_DEVICE;
typedef LPDISPLAY_DEVICEW LPDISPLAY_DEVICE;
#else
typedef DISPLAY_DEVICEA DISPLAY_DEVICE;
typedef PDISPLAY_DEVICEA PDISPLAY_DEVICE;
typedef LPDISPLAY_DEVICEA LPDISPLAY_DEVICE;
#endif // UNICODE

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#define DISPLAY_DEVICE_ATTACHED_TO_DESKTOP      0x00000001
#define DISPLAY_DEVICE_MULTI_DRIVER             0x00000002
#define DISPLAY_DEVICE_PRIMARY_DEVICE           0x00000004
#define DISPLAY_DEVICE_MIRRORING_DRIVER         0x00000008
#define DISPLAY_DEVICE_VGA_COMPATIBLE           0x00000010
#if (_WIN32_WINNT >= _WIN32_WINNT_WIN2K)
#define DISPLAY_DEVICE_REMOVABLE                0x00000020
#endif // (_WIN32_WINNT >= _WIN32_WINNT_WIN2K)
#if (_WIN32_WINNT >= _WIN32_WINNT_WIN8)
#define DISPLAY_DEVICE_ACC_DRIVER               0x00000040
#endif
#define DISPLAY_DEVICE_MODESPRUNED              0x08000000
#if (_WIN32_WINNT >= _WIN32_WINNT_WIN2K)
#define DISPLAY_DEVICE_RDPUDD                   0x01000000
#define DISPLAY_DEVICE_REMOTE                   0x04000000
#define DISPLAY_DEVICE_DISCONNECT               0x02000000
#endif
#define DISPLAY_DEVICE_TS_COMPATIBLE            0x00200000
#if (_WIN32_WINNT >= _WIN32_WINNT_LONGHORN)
#define DISPLAY_DEVICE_UNSAFE_MODES_ON          0x00080000
#endif

/* Child device state */
#if (_WIN32_WINNT >= _WIN32_WINNT_WIN2K)
#define DISPLAY_DEVICE_ACTIVE              0x00000001
#define DISPLAY_DEVICE_ATTACHED            0x00000002
#endif // (_WIN32_WINNT >= _WIN32_WINNT_WIN2K)

#if(WINVER >= 0x0601)

#define DISPLAYCONFIG_MAXPATH 1024        // Maximum display path in system.
                                          // Max adapter (16) * Max source (16) *
                                          // Max clone pre source (4)

#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)

typedef struct DISPLAYCONFIG_RATIONAL
{
    UINT32    Numerator;
    UINT32    Denominator;
} DISPLAYCONFIG_RATIONAL;

typedef enum
{
    DISPLAYCONFIG_OUTPUT_TECHNOLOGY_OTHER                   = -1,
    DISPLAYCONFIG_OUTPUT_TECHNOLOGY_HD15                    =  0,
    DISPLAYCONFIG_OUTPUT_TECHNOLOGY_SVIDEO                  =  1,
    DISPLAYCONFIG_OUTPUT_TECHNOLOGY_COMPOSITE_VIDEO         =  2,
    DISPLAYCONFIG_OUTPUT_TECHNOLOGY_COMPONENT_VIDEO         =  3,
    DISPLAYCONFIG_OUTPUT_TECHNOLOGY_DVI                     =  4,
    DISPLAYCONFIG_OUTPUT_TECHNOLOGY_HDMI                    =  5,
    DISPLAYCONFIG_OUTPUT_TECHNOLOGY_LVDS                    =  6,
    DISPLAYCONFIG_OUTPUT_TECHNOLOGY_D_JPN                   =  8,
    DISPLAYCONFIG_OUTPUT_TECHNOLOGY_SDI                     =  9,
    DISPLAYCONFIG_OUTPUT_TECHNOLOGY_DISPLAYPORT_EXTERNAL    = 10,
    DISPLAYCONFIG_OUTPUT_TECHNOLOGY_DISPLAYPORT_EMBEDDED    = 11,
    DISPLAYCONFIG_OUTPUT_TECHNOLOGY_UDI_EXTERNAL            = 12,
    DISPLAYCONFIG_OUTPUT_TECHNOLOGY_UDI_EMBEDDED            = 13,
    DISPLAYCONFIG_OUTPUT_TECHNOLOGY_SDTVDONGLE              = 14,
    DISPLAYCONFIG_OUTPUT_TECHNOLOGY_MIRACAST                = 15,
    DISPLAYCONFIG_OUTPUT_TECHNOLOGY_INDIRECT_WIRED          = 16,
    DISPLAYCONFIG_OUTPUT_TECHNOLOGY_INDIRECT_VIRTUAL        = 17,
    DISPLAYCONFIG_OUTPUT_TECHNOLOGY_DISPLAYPORT_USB_TUNNEL  = 18,
    DISPLAYCONFIG_OUTPUT_TECHNOLOGY_INTERNAL                = 0x80000000,
    DISPLAYCONFIG_OUTPUT_TECHNOLOGY_FORCE_UINT32            = 0xFFFFFFFF
} DISPLAYCONFIG_VIDEO_OUTPUT_TECHNOLOGY;

typedef enum
{
    DISPLAYCONFIG_SCANLINE_ORDERING_UNSPECIFIED                 = 0,
    DISPLAYCONFIG_SCANLINE_ORDERING_PROGRESSIVE                 = 1,
    DISPLAYCONFIG_SCANLINE_ORDERING_INTERLACED                  = 2,
    DISPLAYCONFIG_SCANLINE_ORDERING_INTERLACED_UPPERFIELDFIRST  = DISPLAYCONFIG_SCANLINE_ORDERING_INTERLACED,
    DISPLAYCONFIG_SCANLINE_ORDERING_INTERLACED_LOWERFIELDFIRST  = 3,
    DISPLAYCONFIG_SCANLINE_ORDERING_FORCE_UINT32                = 0xFFFFFFFF
} DISPLAYCONFIG_SCANLINE_ORDERING;

typedef struct DISPLAYCONFIG_2DREGION
{
    UINT32 cx;
    UINT32 cy;
} DISPLAYCONFIG_2DREGION;

typedef struct DISPLAYCONFIG_VIDEO_SIGNAL_INFO
{
    UINT64                          pixelRate;
    DISPLAYCONFIG_RATIONAL          hSyncFreq;
    DISPLAYCONFIG_RATIONAL          vSyncFreq;
    DISPLAYCONFIG_2DREGION          activeSize;
    DISPLAYCONFIG_2DREGION          totalSize;

    union
    {
        struct
        {
            UINT32 videoStandard : 16;

            // Vertical refresh frequency divider
            UINT32 vSyncFreqDivider : 6;

            UINT32 reserved : 10;
        } AdditionalSignalInfo;

        UINT32 videoStandard;
    } DUMMYUNIONNAME;

    // Scan line ordering (e.g. progressive, interlaced).
    DISPLAYCONFIG_SCANLINE_ORDERING scanLineOrdering;
} DISPLAYCONFIG_VIDEO_SIGNAL_INFO;

typedef enum
{
    DISPLAYCONFIG_SCALING_IDENTITY                  = 1,
    DISPLAYCONFIG_SCALING_CENTERED                  = 2,
    DISPLAYCONFIG_SCALING_STRETCHED                 = 3,
    DISPLAYCONFIG_SCALING_ASPECTRATIOCENTEREDMAX    = 4,
    DISPLAYCONFIG_SCALING_CUSTOM                    = 5,
    DISPLAYCONFIG_SCALING_PREFERRED                 = 128,
    DISPLAYCONFIG_SCALING_FORCE_UINT32              = 0xFFFFFFFF
} DISPLAYCONFIG_SCALING;

typedef enum
{
    DISPLAYCONFIG_ROTATION_IDENTITY     = 1,
    DISPLAYCONFIG_ROTATION_ROTATE90     = 2,
    DISPLAYCONFIG_ROTATION_ROTATE180    = 3,
    DISPLAYCONFIG_ROTATION_ROTATE270    = 4,
    DISPLAYCONFIG_ROTATION_FORCE_UINT32 = 0xFFFFFFFF
} DISPLAYCONFIG_ROTATION;

typedef enum
{
    DISPLAYCONFIG_MODE_INFO_TYPE_SOURCE        = 1,
    DISPLAYCONFIG_MODE_INFO_TYPE_TARGET        = 2,
    DISPLAYCONFIG_MODE_INFO_TYPE_DESKTOP_IMAGE = 3,
    DISPLAYCONFIG_MODE_INFO_TYPE_FORCE_UINT32 = 0xFFFFFFFF
} DISPLAYCONFIG_MODE_INFO_TYPE;

typedef enum
{
    DISPLAYCONFIG_PIXELFORMAT_8BPP          = 1,
    DISPLAYCONFIG_PIXELFORMAT_16BPP         = 2,
    DISPLAYCONFIG_PIXELFORMAT_24BPP         = 3,
    DISPLAYCONFIG_PIXELFORMAT_32BPP         = 4,
    DISPLAYCONFIG_PIXELFORMAT_NONGDI        = 5,
    DISPLAYCONFIG_PIXELFORMAT_FORCE_UINT32  = 0xffffffff
} DISPLAYCONFIG_PIXELFORMAT;

typedef struct DISPLAYCONFIG_SOURCE_MODE
{
    UINT32                      width;
    UINT32                      height;
    DISPLAYCONFIG_PIXELFORMAT   pixelFormat;
    POINTL                      position;
} DISPLAYCONFIG_SOURCE_MODE;

typedef struct DISPLAYCONFIG_TARGET_MODE
{
    DISPLAYCONFIG_VIDEO_SIGNAL_INFO   targetVideoSignalInfo;
} DISPLAYCONFIG_TARGET_MODE;

typedef struct DISPLAYCONFIG_DESKTOP_IMAGE_INFO
{
    POINTL PathSourceSize;
    RECTL DesktopImageRegion;
    RECTL DesktopImageClip;
} DISPLAYCONFIG_DESKTOP_IMAGE_INFO;

typedef struct DISPLAYCONFIG_MODE_INFO
{
    DISPLAYCONFIG_MODE_INFO_TYPE    infoType;
    UINT32                          id;
    LUID                            adapterId;
    union
    {
        DISPLAYCONFIG_TARGET_MODE   targetMode;
        DISPLAYCONFIG_SOURCE_MODE   sourceMode;
        DISPLAYCONFIG_DESKTOP_IMAGE_INFO    desktopImageInfo;
    } DUMMYUNIONNAME;
} DISPLAYCONFIG_MODE_INFO;

#define DISPLAYCONFIG_PATH_MODE_IDX_INVALID             0xffffffff
#define DISPLAYCONFIG_PATH_TARGET_MODE_IDX_INVALID      0xffff
#define DISPLAYCONFIG_PATH_DESKTOP_IMAGE_IDX_INVALID    0xffff
#define DISPLAYCONFIG_PATH_SOURCE_MODE_IDX_INVALID      0xffff
#define DISPLAYCONFIG_PATH_CLONE_GROUP_INVALID          0xffff

typedef struct DISPLAYCONFIG_PATH_SOURCE_INFO
{
    LUID    adapterId;
    UINT32  id;
    union
    {
        UINT32 modeInfoIdx;
        struct
        {
            UINT32 cloneGroupId       : 16;
            UINT32 sourceModeInfoIdx  : 16;
        } DUMMYSTRUCTNAME;
    } DUMMYUNIONNAME;

    UINT32  statusFlags;
} DISPLAYCONFIG_PATH_SOURCE_INFO;

//
// Flags for source info structure (from OS to application through QDC)
//

#define DISPLAYCONFIG_SOURCE_IN_USE     0x00000001

typedef struct DISPLAYCONFIG_PATH_TARGET_INFO
{
    LUID                                    adapterId;
    UINT32                                  id;
    union
    {
        UINT32                                  modeInfoIdx;
        struct
        {
            UINT32 desktopModeInfoIdx : 16;
            UINT32 targetModeInfoIdx  : 16;
        } DUMMYSTRUCTNAME;
    } DUMMYUNIONNAME;
    DISPLAYCONFIG_VIDEO_OUTPUT_TECHNOLOGY   outputTechnology;
    DISPLAYCONFIG_ROTATION                  rotation;
    DISPLAYCONFIG_SCALING                   scaling;
    DISPLAYCONFIG_RATIONAL                  refreshRate;
    DISPLAYCONFIG_SCANLINE_ORDERING         scanLineOrdering;
    BOOL                                    targetAvailable;
    UINT32                                  statusFlags;
} DISPLAYCONFIG_PATH_TARGET_INFO;

//
// Status flags for target info structure (from OS to application through QDC)
//
#define DISPLAYCONFIG_TARGET_IN_USE                         0x00000001
#define DISPLAYCONFIG_TARGET_FORCIBLE                       0x00000002
#define DISPLAYCONFIG_TARGET_FORCED_AVAILABILITY_BOOT       0x00000004
#define DISPLAYCONFIG_TARGET_FORCED_AVAILABILITY_PATH       0x00000008
#define DISPLAYCONFIG_TARGET_FORCED_AVAILABILITY_SYSTEM     0x00000010
#define DISPLAYCONFIG_TARGET_IS_HMD                         0x00000020

typedef struct DISPLAYCONFIG_PATH_INFO
{
    DISPLAYCONFIG_PATH_SOURCE_INFO  sourceInfo;
    DISPLAYCONFIG_PATH_TARGET_INFO  targetInfo;
    UINT32                          flags;
} DISPLAYCONFIG_PATH_INFO;

//
// Flags for path info structure (from OS to application through QDC)
//

#define DISPLAYCONFIG_PATH_ACTIVE               0x00000001
#define DISPLAYCONFIG_PATH_PREFERRED_UNSCALED   0x00000004 // Not implemented
#define DISPLAYCONFIG_PATH_SUPPORT_VIRTUAL_MODE 0x00000008
#define DISPLAYCONFIG_PATH_BOOST_REFRESH_RATE   0x00000010
#define DISPLAYCONFIG_PATH_VALID_FLAGS          0x0000001D

typedef enum DISPLAYCONFIG_TOPOLOGY_ID
{
    DISPLAYCONFIG_TOPOLOGY_INTERNAL       = 0x00000001,
    DISPLAYCONFIG_TOPOLOGY_CLONE          = 0x00000002,
    DISPLAYCONFIG_TOPOLOGY_EXTEND         = 0x00000004,
    DISPLAYCONFIG_TOPOLOGY_EXTERNAL       = 0x00000008,
    DISPLAYCONFIG_TOPOLOGY_FORCE_UINT32   = 0xFFFFFFFF
} DISPLAYCONFIG_TOPOLOGY_ID;


typedef enum
{
    DISPLAYCONFIG_DEVICE_INFO_GET_SOURCE_NAME                 = 1,
    DISPLAYCONFIG_DEVICE_INFO_GET_TARGET_NAME                 = 2,
    DISPLAYCONFIG_DEVICE_INFO_GET_TARGET_PREFERRED_MODE       = 3,
    DISPLAYCONFIG_DEVICE_INFO_GET_ADAPTER_NAME                = 4,
    DISPLAYCONFIG_DEVICE_INFO_SET_TARGET_PERSISTENCE          = 5,
    DISPLAYCONFIG_DEVICE_INFO_GET_TARGET_BASE_TYPE            = 6,
    DISPLAYCONFIG_DEVICE_INFO_GET_SUPPORT_VIRTUAL_RESOLUTION  = 7,
    DISPLAYCONFIG_DEVICE_INFO_SET_SUPPORT_VIRTUAL_RESOLUTION  = 8,
    DISPLAYCONFIG_DEVICE_INFO_GET_ADVANCED_COLOR_INFO         = 9,
    DISPLAYCONFIG_DEVICE_INFO_SET_ADVANCED_COLOR_STATE        = 10,
    DISPLAYCONFIG_DEVICE_INFO_GET_SDR_WHITE_LEVEL             = 11,
    DISPLAYCONFIG_DEVICE_INFO_GET_MONITOR_SPECIALIZATION      = 12,
    DISPLAYCONFIG_DEVICE_INFO_SET_MONITOR_SPECIALIZATION      = 13,
    DISPLAYCONFIG_DEVICE_INFO_SET_RESERVED1                   = 14,
    DISPLAYCONFIG_DEVICE_INFO_GET_ADVANCED_COLOR_INFO_2       = 15,
    DISPLAYCONFIG_DEVICE_INFO_SET_HDR_STATE                   = 16,
    DISPLAYCONFIG_DEVICE_INFO_SET_WCG_STATE                   = 17,

      DISPLAYCONFIG_DEVICE_INFO_FORCE_UINT32                = 0xFFFFFFFF
} DISPLAYCONFIG_DEVICE_INFO_TYPE;

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion


#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)

typedef struct DISPLAYCONFIG_DEVICE_INFO_HEADER
{
    DISPLAYCONFIG_DEVICE_INFO_TYPE  type;
    UINT32                          size;
    LUID                            adapterId;
    UINT32                          id;
} DISPLAYCONFIG_DEVICE_INFO_HEADER;

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

typedef struct DISPLAYCONFIG_SOURCE_DEVICE_NAME
{
    DISPLAYCONFIG_DEVICE_INFO_HEADER            header;
    WCHAR                                       viewGdiDeviceName[CCHDEVICENAME];
} DISPLAYCONFIG_SOURCE_DEVICE_NAME;

typedef struct DISPLAYCONFIG_TARGET_DEVICE_NAME_FLAGS
{
    union
    {
        struct
        {
            UINT32  friendlyNameFromEdid : 1;
            UINT32  friendlyNameForced : 1;
            UINT32  edidIdsValid : 1;
            UINT32  reserved : 29;
        } DUMMYSTRUCTNAME;
        UINT32  value;
    } DUMMYUNIONNAME;
} DISPLAYCONFIG_TARGET_DEVICE_NAME_FLAGS;

typedef struct DISPLAYCONFIG_TARGET_DEVICE_NAME
{
    DISPLAYCONFIG_DEVICE_INFO_HEADER            header;
    DISPLAYCONFIG_TARGET_DEVICE_NAME_FLAGS      flags;
    DISPLAYCONFIG_VIDEO_OUTPUT_TECHNOLOGY       outputTechnology;
    UINT16                                      edidManufactureId;
    UINT16                                      edidProductCodeId;
    UINT32                                      connectorInstance;
    WCHAR                                       monitorFriendlyDeviceName[64];
    WCHAR                                       monitorDevicePath[128];
} DISPLAYCONFIG_TARGET_DEVICE_NAME;

typedef struct DISPLAYCONFIG_TARGET_PREFERRED_MODE
{
    DISPLAYCONFIG_DEVICE_INFO_HEADER    header;
    UINT32                              width;
    UINT32                              height;
    DISPLAYCONFIG_TARGET_MODE           targetMode;
} DISPLAYCONFIG_TARGET_PREFERRED_MODE;

typedef struct DISPLAYCONFIG_ADAPTER_NAME
{
    DISPLAYCONFIG_DEVICE_INFO_HEADER    header;
    WCHAR                               adapterDevicePath[128];
} DISPLAYCONFIG_ADAPTER_NAME;

typedef struct DISPLAYCONFIG_TARGET_BASE_TYPE {
    DISPLAYCONFIG_DEVICE_INFO_HEADER      header;
    DISPLAYCONFIG_VIDEO_OUTPUT_TECHNOLOGY baseOutputTechnology;
} DISPLAYCONFIG_TARGET_BASE_TYPE;


typedef struct DISPLAYCONFIG_SET_TARGET_PERSISTENCE
{
    DISPLAYCONFIG_DEVICE_INFO_HEADER    header;
    union
    {
        struct
        {
            UINT32 bootPersistenceOn    : 1;
            UINT32 reserved             : 31;
        } DUMMYSTRUCTNAME;
        UINT32 value;
    } DUMMYUNIONNAME;
} DISPLAYCONFIG_SET_TARGET_PERSISTENCE;

typedef struct DISPLAYCONFIG_SUPPORT_VIRTUAL_RESOLUTION
{
    DISPLAYCONFIG_DEVICE_INFO_HEADER    header;
    union
    {
        struct
        {
            UINT32 disableMonitorVirtualResolution  : 1;
            UINT32 reserved                         : 31;
        } DUMMYSTRUCTNAME;
        UINT32 value;
    } DUMMYSTRUCTNAME;
} DISPLAYCONFIG_SUPPORT_VIRTUAL_RESOLUTION;

typedef enum _DISPLAYCONFIG_COLOR_ENCODING
{
    DISPLAYCONFIG_COLOR_ENCODING_RGB           = 0,
    DISPLAYCONFIG_COLOR_ENCODING_YCBCR444      = 1,
    DISPLAYCONFIG_COLOR_ENCODING_YCBCR422      = 2,
    DISPLAYCONFIG_COLOR_ENCODING_YCBCR420      = 3,
    DISPLAYCONFIG_COLOR_ENCODING_INTENSITY     = 4,
    DISPLAYCONFIG_COLOR_ENCODING_FORCE_UINT32  = 0xFFFFFFFF
} DISPLAYCONFIG_COLOR_ENCODING;

typedef struct _DISPLAYCONFIG_GET_ADVANCED_COLOR_INFO
{
    DISPLAYCONFIG_DEVICE_INFO_HEADER header;
    union
    {
        struct
        {
          UINT32 advancedColorSupported        :1;    // A type of advanced color is supported
          UINT32 advancedColorEnabled          :1;    // A type of advanced color is enabled
          UINT32 wideColorEnforced             :1;    // Wide color gamut is enabled
          UINT32 advancedColorForceDisabled    :1;    // Advanced color is force disabled due to system/OS policy
          UINT32 reserved                      :28;
        } DUMMYSTRUCTNAME;

        UINT32 value;
    } DUMMYUNIONNAME;

    DISPLAYCONFIG_COLOR_ENCODING colorEncoding;
    UINT32 bitsPerColorChannel;
} DISPLAYCONFIG_GET_ADVANCED_COLOR_INFO;

typedef struct _DISPLAYCONFIG_SET_ADVANCED_COLOR_STATE
{
    DISPLAYCONFIG_DEVICE_INFO_HEADER header;
    union
    {
        struct
        {
          UINT32 enableAdvancedColor  :1;
          UINT32 reserved  :31;
        } DUMMYSTRUCTNAME;

        UINT32 value;
    }DUMMYUNIONNAME;
} DISPLAYCONFIG_SET_ADVANCED_COLOR_STATE;

#if (NTDDI_VERSION >= NTDDI_WIN11_GA)

typedef enum _DISPLAYCONFIG_ADVANCED_COLOR_MODE
{
    DISPLAYCONFIG_ADVANCED_COLOR_MODE_SDR,          // RGB888 composition, display-referred color, display-referred luminance
    DISPLAYCONFIG_ADVANCED_COLOR_MODE_WCG,          // Advanced color (FP16 scRGB composition), scene-referred color, display-referred luminance
    DISPLAYCONFIG_ADVANCED_COLOR_MODE_HDR,          // Advanced color (FP16 scRGB composition), scene-referred color, scene-referred luminance
} DISPLAYCONFIG_ADVANCED_COLOR_MODE;

typedef struct _DISPLAYCONFIG_GET_ADVANCED_COLOR_INFO_2
{
    DISPLAYCONFIG_DEVICE_INFO_HEADER header;
    union
    {
        struct
        {
          UINT32 advancedColorSupported        :1;    // A type of advanced color is supported
          UINT32 advancedColorActive           :1;    // A type of advanced color is active (see currentColorMode for the specific advanced color mode)
          UINT32 reserved1                     :1;
          UINT32 advancedColorLimitedByPolicy  :1;    // System/OS policy is limiting advanced color options (see currentColorMode for the current mode)

          UINT32 highDynamicRangeSupported     :1;    // HDR is supported
          UINT32 highDynamicRangeUserEnabled   :1;    // HDR is enabled by the user (but may not be active)

          UINT32 wideColorSupported            :1;    // Wide color gamut is supported
          UINT32 wideColorUserEnabled          :1;    // Wide color gamut is enabled by the user (but may not be active)

          UINT32 reserved                      :24;
        } DUMMYSTRUCTNAME;

        UINT32 value;
    } DUMMYUNIONNAME;

    DISPLAYCONFIG_COLOR_ENCODING colorEncoding;
    UINT32 bitsPerColorChannel;

    DISPLAYCONFIG_ADVANCED_COLOR_MODE activeColorMode; // The active color mode for this monitor

} DISPLAYCONFIG_GET_ADVANCED_COLOR_INFO_2;

typedef struct _DISPLAYCONFIG_SET_HDR_STATE
{
    DISPLAYCONFIG_DEVICE_INFO_HEADER header;
    union
    {
        struct
        {
          UINT32 enableHdr  :1;
          UINT32 reserved  :31;
        } DUMMYSTRUCTNAME;

        UINT32 value;
    }DUMMYUNIONNAME;
} DISPLAYCONFIG_SET_HDR_STATE;

typedef struct _DISPLAYCONFIG_SET_WCG_STATE
{
    DISPLAYCONFIG_DEVICE_INFO_HEADER header;
    union
    {
        struct
        {
          UINT32 enableWcg  :1;
          UINT32 reserved  :31;
        } DUMMYSTRUCTNAME;

        UINT32 value;
    }DUMMYUNIONNAME;
} DISPLAYCONFIG_SET_WCG_STATE;

#endif // (NTDDI_VERSION >= NTDDI_WIN11_GA)


typedef struct _DISPLAYCONFIG_SDR_WHITE_LEVEL
{
    DISPLAYCONFIG_DEVICE_INFO_HEADER header;

    // SDRWhiteLevel represents a multiplier for standard SDR white
    // peak value i.e. 80 nits represented as fixed point.
    // To get value in nits use the following conversion
    // SDRWhiteLevel in nits = (SDRWhiteLevel / 1000 ) * 80
    ULONG SDRWhiteLevel;
} DISPLAYCONFIG_SDR_WHITE_LEVEL;

typedef struct _DISPLAYCONFIG_GET_MONITOR_SPECIALIZATION
{
    DISPLAYCONFIG_DEVICE_INFO_HEADER    header;
    union
    {
        struct
        {
            UINT32 isSpecializationEnabled : 1;
            UINT32 isSpecializationAvailableForMonitor : 1;
            UINT32 isSpecializationAvailableForSystem : 1;
            UINT32 reserved                : 29;
        } DUMMYSTRUCTNAME;
        UINT32 value;
    } DUMMYUNIONNAME;
} DISPLAYCONFIG_GET_MONITOR_SPECIALIZATION;

typedef struct _DISPLAYCONFIG_SET_MONITOR_SPECIALIZATION
{
    DISPLAYCONFIG_DEVICE_INFO_HEADER    header;
    union
    {
        struct
        {
            UINT32 isSpecializationEnabled : 1;
            UINT32 reserved             : 31;
        } DUMMYSTRUCTNAME;
        UINT32 value;
    } DUMMYUNIONNAME;

    GUID specializationType;
    GUID specializationSubType;

    WCHAR specializationApplicationName[128];
} DISPLAYCONFIG_SET_MONITOR_SPECIALIZATION;

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


//
// Definitions to be used by GetDisplayConfigBufferSizes and QueryDisplayConfig.
//

#define QDC_ALL_PATHS                                 0x00000001
#define QDC_ONLY_ACTIVE_PATHS                         0x00000002
#define QDC_DATABASE_CURRENT                          0x00000004
#define QDC_VIRTUAL_MODE_AWARE                        0x00000010
#define QDC_INCLUDE_HMD                               0x00000020
#define QDC_VIRTUAL_REFRESH_RATE_AWARE                0x00000040

//
// Definitions used by SetDisplayConfig.
//

#define SDC_TOPOLOGY_INTERNAL            0x00000001
#define SDC_TOPOLOGY_CLONE               0x00000002
#define SDC_TOPOLOGY_EXTEND              0x00000004
#define SDC_TOPOLOGY_EXTERNAL            0x00000008
#define SDC_TOPOLOGY_SUPPLIED            0x00000010
#define SDC_USE_DATABASE_CURRENT         (SDC_TOPOLOGY_INTERNAL | SDC_TOPOLOGY_CLONE | SDC_TOPOLOGY_EXTEND | SDC_TOPOLOGY_EXTERNAL)

#define SDC_USE_SUPPLIED_DISPLAY_CONFIG  0x00000020
#define SDC_VALIDATE                     0x00000040
#define SDC_APPLY                        0x00000080
#define SDC_NO_OPTIMIZATION              0x00000100
#define SDC_SAVE_TO_DATABASE             0x00000200
#define SDC_ALLOW_CHANGES                0x00000400
#define SDC_PATH_PERSIST_IF_REQUIRED     0x00000800
#define SDC_FORCE_MODE_ENUMERATION       0x00001000
#define SDC_ALLOW_PATH_ORDER_CHANGES     0x00002000
#define SDC_VIRTUAL_MODE_AWARE           0x00008000


#define SDC_VIRTUAL_REFRESH_RATE_AWARE   0x00020000


#endif /* WINVER >= 0x0601 */

/* GetRegionData/ExtCreateRegion */

#define RDH_RECTANGLES  1

#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)

typedef struct _RGNDATAHEADER {
    DWORD   dwSize;
    DWORD   iType;
    DWORD   nCount;
    DWORD   nRgnSize;
    RECT    rcBound;
} RGNDATAHEADER, *PRGNDATAHEADER;

typedef struct _RGNDATA {
    RGNDATAHEADER   rdh;
    char            Buffer[1];
} RGNDATA, *PRGNDATA, NEAR *NPRGNDATA, FAR *LPRGNDATA;

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion


/* for GetRandomRgn */
#define SYSRGN  4


#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

typedef struct _ABC {
    int     abcA;
    UINT    abcB;
    int     abcC;
} ABC, *PABC, NEAR *NPABC, FAR *LPABC;

typedef struct _ABCFLOAT {
    FLOAT   abcfA;
    FLOAT   abcfB;
    FLOAT   abcfC;
} ABCFLOAT, *PABCFLOAT, NEAR *NPABCFLOAT, FAR *LPABCFLOAT;

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#ifndef NOTEXTMETRIC

#ifdef _MAC
#include "pshpack4.h"
#endif

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

typedef struct _OUTLINETEXTMETRICA {
    UINT    otmSize;
    TEXTMETRICA otmTextMetrics;
    BYTE    otmFiller;
    PANOSE  otmPanoseNumber;
    UINT    otmfsSelection;
    UINT    otmfsType;
     int    otmsCharSlopeRise;
     int    otmsCharSlopeRun;
     int    otmItalicAngle;
    UINT    otmEMSquare;
     int    otmAscent;
     int    otmDescent;
    UINT    otmLineGap;
    UINT    otmsCapEmHeight;
    UINT    otmsXHeight;
    RECT    otmrcFontBox;
     int    otmMacAscent;
     int    otmMacDescent;
    UINT    otmMacLineGap;
    UINT    otmusMinimumPPEM;
    POINT   otmptSubscriptSize;
    POINT   otmptSubscriptOffset;
    POINT   otmptSuperscriptSize;
    POINT   otmptSuperscriptOffset;
    UINT    otmsStrikeoutSize;
     int    otmsStrikeoutPosition;
     int    otmsUnderscoreSize;
     int    otmsUnderscorePosition;
    PSTR    otmpFamilyName;
    PSTR    otmpFaceName;
    PSTR    otmpStyleName;
    PSTR    otmpFullName;
} OUTLINETEXTMETRICA, *POUTLINETEXTMETRICA, NEAR *NPOUTLINETEXTMETRICA, FAR *LPOUTLINETEXTMETRICA;
typedef struct _OUTLINETEXTMETRICW {
    UINT    otmSize;
    TEXTMETRICW otmTextMetrics;
    BYTE    otmFiller;
    PANOSE  otmPanoseNumber;
    UINT    otmfsSelection;
    UINT    otmfsType;
     int    otmsCharSlopeRise;
     int    otmsCharSlopeRun;
     int    otmItalicAngle;
    UINT    otmEMSquare;
     int    otmAscent;
     int    otmDescent;
    UINT    otmLineGap;
    UINT    otmsCapEmHeight;
    UINT    otmsXHeight;
    RECT    otmrcFontBox;
     int    otmMacAscent;
     int    otmMacDescent;
    UINT    otmMacLineGap;
    UINT    otmusMinimumPPEM;
    POINT   otmptSubscriptSize;
    POINT   otmptSubscriptOffset;
    POINT   otmptSuperscriptSize;
    POINT   otmptSuperscriptOffset;
    UINT    otmsStrikeoutSize;
     int    otmsStrikeoutPosition;
     int    otmsUnderscoreSize;
     int    otmsUnderscorePosition;
    PSTR    otmpFamilyName;
    PSTR    otmpFaceName;
    PSTR    otmpStyleName;
    PSTR    otmpFullName;
} OUTLINETEXTMETRICW, *POUTLINETEXTMETRICW, NEAR *NPOUTLINETEXTMETRICW, FAR *LPOUTLINETEXTMETRICW;
#ifdef UNICODE
typedef OUTLINETEXTMETRICW OUTLINETEXTMETRIC;
typedef POUTLINETEXTMETRICW POUTLINETEXTMETRIC;
typedef NPOUTLINETEXTMETRICW NPOUTLINETEXTMETRIC;
typedef LPOUTLINETEXTMETRICW LPOUTLINETEXTMETRIC;
#else
typedef OUTLINETEXTMETRICA OUTLINETEXTMETRIC;
typedef POUTLINETEXTMETRICA POUTLINETEXTMETRIC;
typedef NPOUTLINETEXTMETRICA NPOUTLINETEXTMETRIC;
typedef LPOUTLINETEXTMETRICA LPOUTLINETEXTMETRIC;
#endif // UNICODE

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#ifdef _MAC
#include "poppack.h"
#endif

#endif /* NOTEXTMETRIC */

#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)

typedef struct tagPOLYTEXTA
{
    int       x;
    int       y;
    UINT      n;
    LPCSTR    lpstr;
    UINT      uiFlags;
    RECT      rcl;
    int      *pdx;
} POLYTEXTA, *PPOLYTEXTA, NEAR *NPPOLYTEXTA, FAR *LPPOLYTEXTA;
typedef struct tagPOLYTEXTW
{
    int       x;
    int       y;
    UINT      n;
    LPCWSTR   lpstr;
    UINT      uiFlags;
    RECT      rcl;
    int      *pdx;
} POLYTEXTW, *PPOLYTEXTW, NEAR *NPPOLYTEXTW, FAR *LPPOLYTEXTW;
#ifdef UNICODE
typedef POLYTEXTW POLYTEXT;
typedef PPOLYTEXTW PPOLYTEXT;
typedef NPPOLYTEXTW NPPOLYTEXT;
typedef LPPOLYTEXTW LPPOLYTEXT;
#else
typedef POLYTEXTA POLYTEXT;
typedef PPOLYTEXTA PPOLYTEXT;
typedef NPPOLYTEXTA NPPOLYTEXT;
typedef LPPOLYTEXTA LPPOLYTEXT;
#endif // UNICODE

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

typedef struct _FIXED {
#ifndef _MAC
    WORD    fract;
    short   value;
#else
    short   value;
    WORD    fract;
#endif
} FIXED;


typedef struct _MAT2 {
     FIXED  eM11;
     FIXED  eM12;
     FIXED  eM21;
     FIXED  eM22;
} MAT2, FAR *LPMAT2;



typedef struct _GLYPHMETRICS {
    UINT    gmBlackBoxX;
    UINT    gmBlackBoxY;
    POINT   gmptGlyphOrigin;
    short   gmCellIncX;
    short   gmCellIncY;
} GLYPHMETRICS, FAR *LPGLYPHMETRICS;

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

//  GetGlyphOutline constants

#define GGO_METRICS        0
#define GGO_BITMAP         1
#define GGO_NATIVE         2
#define GGO_BEZIER         3

#if(WINVER >= 0x0400)
#define  GGO_GRAY2_BITMAP   4
#define  GGO_GRAY4_BITMAP   5
#define  GGO_GRAY8_BITMAP   6
#define  GGO_GLYPH_INDEX    0x0080
#endif /* WINVER >= 0x0400 */

#if (_WIN32_WINNT >= _WIN32_WINNT_WIN2K)
#define  GGO_UNHINTED       0x0100
#endif // (_WIN32_WINNT >= _WIN32_WINNT_WIN2K)

#define TT_POLYGON_TYPE   24

#define TT_PRIM_LINE       1
#define TT_PRIM_QSPLINE    2
#define TT_PRIM_CSPLINE    3

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

typedef struct tagPOINTFX
{
    FIXED x;
    FIXED y;
} POINTFX, FAR* LPPOINTFX;

typedef struct tagTTPOLYCURVE
{
    WORD    wType;
    WORD    cpfx;
    POINTFX apfx[1];
} TTPOLYCURVE, FAR* LPTTPOLYCURVE;

typedef struct tagTTPOLYGONHEADER
{
    DWORD   cb;
    DWORD   dwType;
    POINTFX pfxStart;
} TTPOLYGONHEADER, FAR* LPTTPOLYGONHEADER;

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


#if(WINVER >= 0x0400)
#define GCP_DBCS           0x0001
#define GCP_REORDER        0x0002
#define GCP_USEKERNING     0x0008
#define GCP_GLYPHSHAPE     0x0010
#define GCP_LIGATE         0x0020
////#define GCP_GLYPHINDEXING  0x0080
#define GCP_DIACRITIC      0x0100
#define GCP_KASHIDA        0x0400
#define GCP_ERROR          0x8000
#define FLI_MASK           0x103B

#define GCP_JUSTIFY        0x00010000L
////#define GCP_NODIACRITICS   0x00020000L
#define FLI_GLYPHS         0x00040000L
#define GCP_CLASSIN        0x00080000L
#define GCP_MAXEXTENT      0x00100000L
#define GCP_JUSTIFYIN      0x00200000L
#define GCP_DISPLAYZWG      0x00400000L
#define GCP_SYMSWAPOFF      0x00800000L
#define GCP_NUMERICOVERRIDE 0x01000000L
#define GCP_NEUTRALOVERRIDE 0x02000000L
#define GCP_NUMERICSLATIN   0x04000000L
#define GCP_NUMERICSLOCAL   0x08000000L

#define GCPCLASS_LATIN                  1
#define GCPCLASS_HEBREW                 2
#define GCPCLASS_ARABIC                 2
#define GCPCLASS_NEUTRAL                3
#define GCPCLASS_LOCALNUMBER            4
#define GCPCLASS_LATINNUMBER            5
#define GCPCLASS_LATINNUMERICTERMINATOR 6
#define GCPCLASS_LATINNUMERICSEPARATOR  7
#define GCPCLASS_NUMERICSEPARATOR       8
#define GCPCLASS_PREBOUNDLTR         0x80
#define GCPCLASS_PREBOUNDRTL         0x40
#define GCPCLASS_POSTBOUNDLTR        0x20
#define GCPCLASS_POSTBOUNDRTL        0x10

#define GCPGLYPH_LINKBEFORE          0x8000
#define GCPGLYPH_LINKAFTER           0x4000


#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

typedef struct tagGCP_RESULTSA
    {
    DWORD   lStructSize;
    LPSTR     lpOutString;
    UINT FAR *lpOrder;
    int FAR  *lpDx;
    int FAR  *lpCaretPos;
    LPSTR   lpClass;
    LPWSTR  lpGlyphs;
    UINT    nGlyphs;
    int     nMaxFit;
    } GCP_RESULTSA, FAR* LPGCP_RESULTSA;
typedef struct tagGCP_RESULTSW
    {
    DWORD   lStructSize;
    LPWSTR    lpOutString;
    UINT FAR *lpOrder;
    int FAR  *lpDx;
    int FAR  *lpCaretPos;
    LPSTR   lpClass;
    LPWSTR  lpGlyphs;
    UINT    nGlyphs;
    int     nMaxFit;
    } GCP_RESULTSW, FAR* LPGCP_RESULTSW;
#ifdef UNICODE
typedef GCP_RESULTSW GCP_RESULTS;
typedef LPGCP_RESULTSW LPGCP_RESULTS;
#else
typedef GCP_RESULTSA GCP_RESULTS;
typedef LPGCP_RESULTSA LPGCP_RESULTS;
#endif // UNICODE

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion
#endif /* WINVER >= 0x0400 */

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

typedef struct _RASTERIZER_STATUS {
    short   nSize;
    short   wFlags;
    short   nLanguageID;
} RASTERIZER_STATUS, FAR *LPRASTERIZER_STATUS;

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

/* bits defined in wFlags of RASTERIZER_STATUS */
#define TT_AVAILABLE    0x0001
#define TT_ENABLED      0x0002

#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)

/* Pixel format descriptor */
typedef struct tagPIXELFORMATDESCRIPTOR
{
    WORD  nSize;
    WORD  nVersion;
    DWORD dwFlags;
    BYTE  iPixelType;
    BYTE  cColorBits;
    BYTE  cRedBits;
    BYTE  cRedShift;
    BYTE  cGreenBits;
    BYTE  cGreenShift;
    BYTE  cBlueBits;
    BYTE  cBlueShift;
    BYTE  cAlphaBits;
    BYTE  cAlphaShift;
    BYTE  cAccumBits;
    BYTE  cAccumRedBits;
    BYTE  cAccumGreenBits;
    BYTE  cAccumBlueBits;
    BYTE  cAccumAlphaBits;
    BYTE  cDepthBits;
    BYTE  cStencilBits;
    BYTE  cAuxBuffers;
    BYTE  iLayerType;
    BYTE  bReserved;
    DWORD dwLayerMask;
    DWORD dwVisibleMask;
    DWORD dwDamageMask;
} PIXELFORMATDESCRIPTOR, *PPIXELFORMATDESCRIPTOR, FAR *LPPIXELFORMATDESCRIPTOR;

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

/* pixel types */
#define PFD_TYPE_RGBA        0
#define PFD_TYPE_COLORINDEX  1

/* layer types */
#define PFD_MAIN_PLANE       0
#define PFD_OVERLAY_PLANE    1
#define PFD_UNDERLAY_PLANE   (-1)

/* PIXELFORMATDESCRIPTOR flags */
#define PFD_DOUBLEBUFFER            0x00000001
#define PFD_STEREO                  0x00000002
#define PFD_DRAW_TO_WINDOW          0x00000004
#define PFD_DRAW_TO_BITMAP          0x00000008
#define PFD_SUPPORT_GDI             0x00000010
#define PFD_SUPPORT_OPENGL          0x00000020
#define PFD_GENERIC_FORMAT          0x00000040
#define PFD_NEED_PALETTE            0x00000080
#define PFD_NEED_SYSTEM_PALETTE     0x00000100
#define PFD_SWAP_EXCHANGE           0x00000200
#define PFD_SWAP_COPY               0x00000400
#define PFD_SWAP_LAYER_BUFFERS      0x00000800
#define PFD_GENERIC_ACCELERATED     0x00001000
#define PFD_SUPPORT_DIRECTDRAW      0x00002000
#define PFD_DIRECT3D_ACCELERATED    0x00004000
#define PFD_SUPPORT_COMPOSITION     0x00008000

/* PIXELFORMATDESCRIPTOR flags for use in ChoosePixelFormat only */
#define PFD_DEPTH_DONTCARE          0x20000000
#define PFD_DOUBLEBUFFER_DONTCARE   0x40000000
#define PFD_STEREO_DONTCARE         0x80000000


#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

#ifdef STRICT

#if !defined(NOTEXTMETRIC)
typedef int (CALLBACK* OLDFONTENUMPROCA)(CONST LOGFONTA *, CONST TEXTMETRICA *, DWORD, LPARAM);
typedef int (CALLBACK* OLDFONTENUMPROCW)(CONST LOGFONTW *, CONST TEXTMETRICW *, DWORD, LPARAM);
#ifdef UNICODE
#define OLDFONTENUMPROC  OLDFONTENUMPROCW
#else
#define OLDFONTENUMPROC  OLDFONTENUMPROCA
#endif // !UNICODE
#else
typedef int (CALLBACK* OLDFONTENUMPROCA)(CONST LOGFONTA *, CONST VOID *, DWORD, LPARAM);
typedef int (CALLBACK* OLDFONTENUMPROCW)(CONST LOGFONTW *, CONST VOID *, DWORD, LPARAM);
#ifdef UNICODE
#define OLDFONTENUMPROC  OLDFONTENUMPROCW
#else
#define OLDFONTENUMPROC  OLDFONTENUMPROCA
#endif // !UNICODE
#endif

typedef OLDFONTENUMPROCA    FONTENUMPROCA;
typedef OLDFONTENUMPROCW    FONTENUMPROCW;
#ifdef UNICODE
typedef FONTENUMPROCW FONTENUMPROC;
#else
typedef FONTENUMPROCA FONTENUMPROC;
#endif // UNICODE

typedef int (CALLBACK* GOBJENUMPROC)(LPVOID, LPARAM);
typedef VOID (CALLBACK* LINEDDAPROC)(int, int, LPARAM);
#else
typedef FARPROC OLDFONTENUMPROC;
typedef FARPROC FONTENUMPROCA;
typedef FARPROC FONTENUMPROCW;
#ifdef UNICODE
typedef FONTENUMPROCW FONTENUMPROC;
#else
typedef FONTENUMPROCA FONTENUMPROC;
#endif // UNICODE
typedef FARPROC GOBJENUMPROC;
typedef FARPROC LINEDDAPROC;
#endif

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion



#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

WINGDIAPI int WINAPI AddFontResourceA(_In_ LPCSTR);
WINGDIAPI int WINAPI AddFontResourceW(_In_ LPCWSTR);
#ifdef UNICODE
#define AddFontResource  AddFontResourceW
#else
#define AddFontResource  AddFontResourceA
#endif // !UNICODE

 WINGDIAPI BOOL  WINAPI AnimatePalette( _In_ HPALETTE hPal, _In_ UINT iStartIndex, _In_  UINT cEntries, _In_reads_(cEntries) CONST PALETTEENTRY * ppe);
 WINGDIAPI BOOL  WINAPI Arc( _In_ HDC hdc, _In_ int x1, _In_ int y1, _In_ int x2, _In_ int y2, _In_ int x3, _In_ int y3, _In_ int x4, _In_ int y4);
 WINGDIAPI BOOL  WINAPI BitBlt( _In_ HDC hdc, _In_ int x, _In_ int y, _In_ int cx, _In_ int cy, _In_opt_ HDC hdcSrc, _In_ int x1, _In_ int y1, _In_ DWORD rop);
WINGDIAPI BOOL  WINAPI CancelDC( _In_ HDC hdc);
 WINGDIAPI BOOL  WINAPI Chord( _In_ HDC hdc, _In_ int x1, _In_ int y1, _In_ int x2, _In_ int y2, _In_ int x3, _In_ int y3, _In_ int x4, _In_ int y4);
WINGDIAPI int   WINAPI ChoosePixelFormat( _In_ HDC hdc, _In_ CONST PIXELFORMATDESCRIPTOR *ppfd);
WINGDIAPI HMETAFILE  WINAPI CloseMetaFile( _In_ HDC hdc);
WINGDIAPI int     WINAPI CombineRgn( _In_opt_ HRGN hrgnDst, _In_opt_ HRGN hrgnSrc1, _In_opt_ HRGN hrgnSrc2, _In_ int iMode);
WINGDIAPI HMETAFILE WINAPI CopyMetaFileA( _In_ HMETAFILE, _In_opt_ LPCSTR);
WINGDIAPI HMETAFILE WINAPI CopyMetaFileW( _In_ HMETAFILE, _In_opt_ LPCWSTR);
#ifdef UNICODE
#define CopyMetaFile  CopyMetaFileW
#else
#define CopyMetaFile  CopyMetaFileA
#endif // !UNICODE
 WINGDIAPI HBITMAP WINAPI CreateBitmap( _In_ int nWidth, _In_ int nHeight, _In_ UINT nPlanes, _In_ UINT nBitCount, _In_opt_ CONST VOID *lpBits);
 WINGDIAPI HBITMAP WINAPI CreateBitmapIndirect( _In_ CONST BITMAP *pbm);
 WINGDIAPI HBRUSH  WINAPI CreateBrushIndirect( _In_ CONST LOGBRUSH *plbrush);
WINGDIAPI HBITMAP WINAPI CreateCompatibleBitmap( _In_ HDC hdc, _In_ int cx, _In_ int cy);
WINGDIAPI HBITMAP WINAPI CreateDiscardableBitmap( _In_ HDC hdc, _In_ int cx, _In_ int cy);
WINGDIAPI HDC     WINAPI CreateCompatibleDC( _In_opt_ HDC hdc);
WINGDIAPI HDC     WINAPI CreateDCA( _In_opt_ LPCSTR pwszDriver, _In_opt_ LPCSTR pwszDevice, _In_opt_ LPCSTR pszPort, _In_opt_ CONST DEVMODEA * pdm);
WINGDIAPI HDC     WINAPI CreateDCW( _In_opt_ LPCWSTR pwszDriver, _In_opt_ LPCWSTR pwszDevice, _In_opt_ LPCWSTR pszPort, _In_opt_ CONST DEVMODEW * pdm);
#ifdef UNICODE
#define CreateDC  CreateDCW
#else
#define CreateDC  CreateDCA
#endif // !UNICODE
WINGDIAPI HBITMAP WINAPI CreateDIBitmap( _In_ HDC hdc, _In_opt_ CONST BITMAPINFOHEADER *pbmih, _In_ DWORD flInit, _In_opt_ CONST VOID *pjBits, _In_opt_ CONST BITMAPINFO *pbmi, _In_ UINT iUsage);
WINGDIAPI HBRUSH  WINAPI CreateDIBPatternBrush( _In_ HGLOBAL h, _In_ UINT iUsage);
 WINGDIAPI HBRUSH  WINAPI CreateDIBPatternBrushPt( _In_ CONST VOID *lpPackedDIB, _In_ UINT iUsage);
WINGDIAPI HRGN    WINAPI CreateEllipticRgn( _In_ int x1, _In_ int y1, _In_ int x2, _In_ int y2);
WINGDIAPI HRGN    WINAPI CreateEllipticRgnIndirect( _In_ CONST RECT *lprect);
 WINGDIAPI HFONT   WINAPI CreateFontIndirectA( _In_ CONST LOGFONTA *lplf);
 WINGDIAPI HFONT   WINAPI CreateFontIndirectW( _In_ CONST LOGFONTW *lplf);
#ifdef UNICODE
#define CreateFontIndirect  CreateFontIndirectW
#else
#define CreateFontIndirect  CreateFontIndirectA
#endif // !UNICODE
WINGDIAPI HFONT   WINAPI CreateFontA( _In_ int cHeight, _In_ int cWidth, _In_ int cEscapement, _In_ int cOrientation, _In_ int cWeight, _In_ DWORD bItalic,
                             _In_ DWORD bUnderline, _In_ DWORD bStrikeOut, _In_ DWORD iCharSet, _In_ DWORD iOutPrecision, _In_ DWORD iClipPrecision,
                             _In_ DWORD iQuality, _In_ DWORD iPitchAndFamily, _In_opt_ LPCSTR pszFaceName);
WINGDIAPI HFONT   WINAPI CreateFontW( _In_ int cHeight, _In_ int cWidth, _In_ int cEscapement, _In_ int cOrientation, _In_ int cWeight, _In_ DWORD bItalic,
                             _In_ DWORD bUnderline, _In_ DWORD bStrikeOut, _In_ DWORD iCharSet, _In_ DWORD iOutPrecision, _In_ DWORD iClipPrecision,
                             _In_ DWORD iQuality, _In_ DWORD iPitchAndFamily, _In_opt_ LPCWSTR pszFaceName);
#ifdef UNICODE
#define CreateFont  CreateFontW
#else
#define CreateFont  CreateFontA
#endif // !UNICODE

WINGDIAPI HBRUSH  WINAPI CreateHatchBrush( _In_ int iHatch, _In_ COLORREF color);
WINGDIAPI HDC     WINAPI CreateICA( _In_opt_ LPCSTR pszDriver, _In_opt_ LPCSTR pszDevice, _In_opt_ LPCSTR pszPort, _In_opt_ CONST DEVMODEA * pdm);
WINGDIAPI HDC     WINAPI CreateICW( _In_opt_ LPCWSTR pszDriver, _In_opt_ LPCWSTR pszDevice, _In_opt_ LPCWSTR pszPort, _In_opt_ CONST DEVMODEW * pdm);
#ifdef UNICODE
#define CreateIC  CreateICW
#else
#define CreateIC  CreateICA
#endif // !UNICODE
WINGDIAPI HDC     WINAPI CreateMetaFileA( _In_opt_ LPCSTR pszFile);
WINGDIAPI HDC     WINAPI CreateMetaFileW( _In_opt_ LPCWSTR pszFile);
#ifdef UNICODE
#define CreateMetaFile  CreateMetaFileW
#else
#define CreateMetaFile  CreateMetaFileA
#endif // !UNICODE
 WINGDIAPI HPALETTE WINAPI CreatePalette( _In_reads_(_Inexpressible_(2*sizeof(WORD) + plpal->palNumEntries * sizeof(PALETTEENTRY))) CONST LOGPALETTE * plpal);
WINGDIAPI HPEN    WINAPI CreatePen( _In_ int iStyle, _In_ int cWidth, _In_ COLORREF color);
 WINGDIAPI HPEN    WINAPI CreatePenIndirect( _In_ CONST LOGPEN *plpen);
WINGDIAPI HRGN    WINAPI CreatePolyPolygonRgn(  _In_ CONST POINT *pptl,
                                                _In_reads_(cPoly) CONST INT  *pc,
                                                _In_ int cPoly,
                                                _In_ int iMode);
 WINGDIAPI HBRUSH  WINAPI CreatePatternBrush( _In_ HBITMAP hbm);
WINGDIAPI HRGN    WINAPI CreateRectRgn( _In_ int x1, _In_ int y1, _In_ int x2, _In_ int y2);
WINGDIAPI HRGN    WINAPI CreateRectRgnIndirect( _In_ CONST RECT *lprect);
WINGDIAPI HRGN    WINAPI CreateRoundRectRgn( _In_ int x1, _In_ int y1, _In_ int x2, _In_ int y2, _In_ int w, _In_ int h);
WINGDIAPI BOOL    WINAPI CreateScalableFontResourceA( _In_ DWORD fdwHidden, _In_ LPCSTR lpszFont, _In_ LPCSTR lpszFile, _In_opt_ LPCSTR lpszPath);
WINGDIAPI BOOL    WINAPI CreateScalableFontResourceW( _In_ DWORD fdwHidden, _In_ LPCWSTR lpszFont, _In_ LPCWSTR lpszFile, _In_opt_ LPCWSTR lpszPath);
#ifdef UNICODE
#define CreateScalableFontResource  CreateScalableFontResourceW
#else
#define CreateScalableFontResource  CreateScalableFontResourceA
#endif // !UNICODE
WINGDIAPI HBRUSH  WINAPI CreateSolidBrush( _In_ COLORREF color);

WINGDIAPI BOOL WINAPI DeleteDC( _In_ HDC hdc);
WINGDIAPI BOOL WINAPI DeleteMetaFile( _In_ HMETAFILE hmf);
 WINGDIAPI BOOL WINAPI DeleteObject( _In_ HGDIOBJ ho);
WINGDIAPI int  WINAPI DescribePixelFormat(  _In_ HDC hdc,
                                            _In_ int iPixelFormat,
                                            _In_ UINT nBytes,
                                            _Out_writes_bytes_opt_(nBytes) LPPIXELFORMATDESCRIPTOR ppfd);

/* define types of pointers to ExtDeviceMode() and DeviceCapabilities()
 * functions for Win 3.1 compatibility
 */

typedef UINT   (CALLBACK* LPFNDEVMODE)(HWND, HMODULE, LPDEVMODE, LPSTR, LPSTR, LPDEVMODE, LPSTR, UINT);

typedef DWORD  (CALLBACK* LPFNDEVCAPS)(LPSTR, LPSTR, UINT, LPSTR, LPDEVMODE);

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

/* mode selections for the device mode function */
#define DM_UPDATE           1
#define DM_COPY             2
#define DM_PROMPT           4
#define DM_MODIFY           8

#define DM_IN_BUFFER        DM_MODIFY
#define DM_IN_PROMPT        DM_PROMPT
#define DM_OUT_BUFFER       DM_COPY
#define DM_OUT_DEFAULT      DM_UPDATE

/* device capabilities indices */
#define DC_FIELDS           1
#define DC_PAPERS           2
#define DC_PAPERSIZE        3
#define DC_MINEXTENT        4
#define DC_MAXEXTENT        5
#define DC_BINS             6
#define DC_DUPLEX           7
#define DC_SIZE             8
#define DC_EXTRA            9
#define DC_VERSION          10
#define DC_DRIVER           11
#define DC_BINNAMES         12
#define DC_ENUMRESOLUTIONS  13
#define DC_FILEDEPENDENCIES 14
#define DC_TRUETYPE         15
#define DC_PAPERNAMES       16
#define DC_ORIENTATION      17
#define DC_COPIES           18
#if(WINVER >= 0x0400)
#define DC_BINADJUST            19
#define DC_EMF_COMPLIANT        20
#define DC_DATATYPE_PRODUCED    21
#define DC_COLLATE              22
#define DC_MANUFACTURER         23
#define DC_MODEL                24
#endif /* WINVER >= 0x0400 */

#if(WINVER >= 0x0500)
#define DC_PERSONALITY          25
#define DC_PRINTRATE            26
#define DC_PRINTRATEUNIT        27
#define   PRINTRATEUNIT_PPM     1
#define   PRINTRATEUNIT_CPS     2
#define   PRINTRATEUNIT_LPM     3
#define   PRINTRATEUNIT_IPM     4
#define DC_PRINTERMEM           28
#define DC_MEDIAREADY           29
#define DC_STAPLE               30
#define DC_PRINTRATEPPM         31
#define DC_COLORDEVICE          32
#define DC_NUP                  33
#if (_WIN32_WINNT >= _WIN32_WINNT_WIN2K)
#define DC_MEDIATYPENAMES       34
#define DC_MEDIATYPES           35
#endif // (_WIN32_WINNT >= _WIN32_WINNT_WIN2K)
#endif /* WINVER >= 0x0500 */

/* bit fields of the return value (DWORD) for DC_TRUETYPE */
#define DCTT_BITMAP             0x0000001L
#define DCTT_DOWNLOAD           0x0000002L
#define DCTT_SUBDEV             0x0000004L
#if(WINVER >= 0x0400)
#define DCTT_DOWNLOAD_OUTLINE   0x0000008L

/* return values for DC_BINADJUST */
#define DCBA_FACEUPNONE       0x0000
#define DCBA_FACEUPCENTER     0x0001
#define DCBA_FACEUPLEFT       0x0002
#define DCBA_FACEUPRIGHT      0x0003
#define DCBA_FACEDOWNNONE     0x0100
#define DCBA_FACEDOWNCENTER   0x0101
#define DCBA_FACEDOWNLEFT     0x0102
#define DCBA_FACEDOWNRIGHT    0x0103
#endif /* WINVER >= 0x0400 */

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

WINSPOOLAPI
int
WINAPI
DeviceCapabilitiesA(
    _In_                LPCSTR         pDevice,
    _In_opt_            LPCSTR         pPort,
    _In_                WORD             fwCapability,
    _Out_writes_opt_(_Inexpressible_(1)) LPSTR          pOutput,
    _In_opt_            CONST DEVMODEA   *pDevMode
    );
WINSPOOLAPI
int
WINAPI
DeviceCapabilitiesW(
    _In_                LPCWSTR         pDevice,
    _In_opt_            LPCWSTR         pPort,
    _In_                WORD             fwCapability,
    _Out_writes_opt_(_Inexpressible_(1)) LPWSTR          pOutput,
    _In_opt_            CONST DEVMODEW   *pDevMode
    );
#ifdef UNICODE
#define DeviceCapabilities  DeviceCapabilitiesW
#else
#define DeviceCapabilities  DeviceCapabilitiesA
#endif // !UNICODE

WINGDIAPI int  WINAPI DrawEscape(   _In_ HDC    hdc,
                                    _In_ int    iEscape,
                                    _In_ int    cjIn,
                                    _In_reads_bytes_opt_(cjIn) LPCSTR lpIn);

 WINGDIAPI BOOL WINAPI Ellipse( _In_ HDC hdc, _In_ int left, _In_ int top,  _In_ int right, _In_ int bottom);

#if(WINVER >= 0x0400)
WINGDIAPI int  WINAPI EnumFontFamiliesExA( _In_ HDC hdc, _In_ LPLOGFONTA lpLogfont, _In_ FONTENUMPROCA lpProc, _In_ LPARAM lParam, _In_ DWORD dwFlags);
WINGDIAPI int  WINAPI EnumFontFamiliesExW( _In_ HDC hdc, _In_ LPLOGFONTW lpLogfont, _In_ FONTENUMPROCW lpProc, _In_ LPARAM lParam, _In_ DWORD dwFlags);
#ifdef UNICODE
#define EnumFontFamiliesEx  EnumFontFamiliesExW
#else
#define EnumFontFamiliesEx  EnumFontFamiliesExA
#endif // !UNICODE
#endif /* WINVER >= 0x0400 */

WINGDIAPI int  WINAPI EnumFontFamiliesA( _In_ HDC hdc, _In_opt_ LPCSTR lpLogfont, _In_ FONTENUMPROCA lpProc, _In_ LPARAM lParam);
WINGDIAPI int  WINAPI EnumFontFamiliesW( _In_ HDC hdc, _In_opt_ LPCWSTR lpLogfont, _In_ FONTENUMPROCW lpProc, _In_ LPARAM lParam);
#ifdef UNICODE
#define EnumFontFamilies  EnumFontFamiliesW
#else
#define EnumFontFamilies  EnumFontFamiliesA
#endif // !UNICODE
WINGDIAPI int  WINAPI EnumFontsA( _In_ HDC hdc, _In_opt_ LPCSTR lpLogfont,  _In_ FONTENUMPROCA lpProc, _In_ LPARAM lParam);
WINGDIAPI int  WINAPI EnumFontsW( _In_ HDC hdc, _In_opt_ LPCWSTR lpLogfont,  _In_ FONTENUMPROCW lpProc, _In_ LPARAM lParam);
#ifdef UNICODE
#define EnumFonts  EnumFontsW
#else
#define EnumFonts  EnumFontsA
#endif // !UNICODE

#ifdef STRICT
WINGDIAPI int  WINAPI EnumObjects( _In_ HDC hdc, _In_ int nType, _In_ GOBJENUMPROC lpFunc, _In_ LPARAM lParam);
#else
WINGDIAPI int  WINAPI EnumObjects( _In_ HDC hdc, _In_ int nType, _In_ GOBJENUMPROC lpFunc, _In_ LPVOID lParam);
#endif


WINGDIAPI BOOL WINAPI EqualRgn( _In_ HRGN hrgn1, _In_ HRGN hrgn2);
 WINGDIAPI int  WINAPI Escape(   _In_ HDC hdc,
                                _In_ int iEscape,
                                _In_ int cjIn,
                                _In_reads_bytes_opt_(cjIn) LPCSTR pvIn,
                                _Out_opt_ LPVOID pvOut);
WINGDIAPI int  WINAPI ExtEscape(    _In_ HDC hdc,
                                    _In_ int iEscape,
                                    _In_ int cjInput,
                                    _In_reads_bytes_opt_(cjInput) LPCSTR lpInData,
                                    _In_ int cjOutput,
                                    _Out_writes_bytes_opt_(cjOutput) LPSTR lpOutData);
 WINGDIAPI int  WINAPI ExcludeClipRect( _In_ HDC hdc, _In_ int left, _In_ int top, _In_ int right, _In_ int bottom);
 WINGDIAPI HRGN WINAPI ExtCreateRegion( _In_opt_ CONST XFORM * lpx, _In_ DWORD nCount, _In_reads_bytes_(nCount) CONST RGNDATA * lpData);
 WINGDIAPI BOOL WINAPI ExtFloodFill( _In_ HDC hdc, _In_ int x, _In_ int y, _In_ COLORREF color, _In_ UINT type);
 WINGDIAPI BOOL WINAPI FillRgn( _In_ HDC hdc, _In_ HRGN hrgn, _In_ HBRUSH hbr);
 WINGDIAPI BOOL WINAPI FloodFill( _In_ HDC hdc, _In_ int x, _In_ int y, _In_ COLORREF color);
 WINGDIAPI BOOL WINAPI FrameRgn( _In_ HDC hdc, _In_ HRGN hrgn, _In_ HBRUSH hbr, _In_ int w, _In_ int h);
WINGDIAPI int  WINAPI GetROP2( _In_ HDC hdc);
WINGDIAPI BOOL WINAPI GetAspectRatioFilterEx( _In_ HDC hdc, _Out_ LPSIZE lpsize);
WINGDIAPI COLORREF WINAPI GetBkColor( _In_ HDC hdc);

#if (_WIN32_WINNT >= _WIN32_WINNT_WIN2K)
WINGDIAPI COLORREF WINAPI GetDCBrushColor( _In_ HDC hdc);
WINGDIAPI COLORREF WINAPI GetDCPenColor( _In_ HDC hdc);
#endif

WINGDIAPI
int
WINAPI
GetBkMode(
    _In_ HDC hdc
    );

WINGDIAPI
LONG
WINAPI
GetBitmapBits(
    _In_ HBITMAP hbit,
    _In_ LONG cb,
    _Out_writes_bytes_(cb) LPVOID lpvBits
    );

WINGDIAPI BOOL  WINAPI GetBitmapDimensionEx( _In_ HBITMAP hbit, _Out_ LPSIZE lpsize);
WINGDIAPI UINT  WINAPI GetBoundsRect( _In_ HDC hdc, _Out_ LPRECT lprect, _In_ UINT flags);

WINGDIAPI BOOL  WINAPI GetBrushOrgEx( _In_ HDC hdc, _Out_ LPPOINT lppt);

WINGDIAPI BOOL  WINAPI GetCharWidthA( _In_ HDC hdc, _In_ UINT iFirst, _In_ UINT iLast, _Out_writes_(iLast + 1 - iFirst) LPINT lpBuffer);
WINGDIAPI BOOL  WINAPI GetCharWidthW( _In_ HDC hdc, _In_ UINT iFirst, _In_ UINT iLast, _Out_writes_(iLast + 1 - iFirst) LPINT lpBuffer);
#ifdef UNICODE
#define GetCharWidth  GetCharWidthW
#else
#define GetCharWidth  GetCharWidthA
#endif // !UNICODE
WINGDIAPI BOOL  WINAPI GetCharWidth32A( _In_ HDC hdc, _In_ UINT iFirst, _In_ UINT iLast,  _Out_writes_(iLast + 1 - iFirst) LPINT lpBuffer);
WINGDIAPI BOOL  WINAPI GetCharWidth32W( _In_ HDC hdc, _In_ UINT iFirst, _In_ UINT iLast,  _Out_writes_(iLast + 1 - iFirst) LPINT lpBuffer);
#ifdef UNICODE
#define GetCharWidth32  GetCharWidth32W
#else
#define GetCharWidth32  GetCharWidth32A
#endif // !UNICODE
WINGDIAPI BOOL  APIENTRY GetCharWidthFloatA( _In_ HDC hdc, _In_ UINT iFirst, _In_ UINT iLast, _Out_writes_(iLast + 1 - iFirst) PFLOAT lpBuffer);
WINGDIAPI BOOL  APIENTRY GetCharWidthFloatW( _In_ HDC hdc, _In_ UINT iFirst, _In_ UINT iLast, _Out_writes_(iLast + 1 - iFirst) PFLOAT lpBuffer);
#ifdef UNICODE
#define GetCharWidthFloat  GetCharWidthFloatW
#else
#define GetCharWidthFloat  GetCharWidthFloatA
#endif // !UNICODE

WINGDIAPI BOOL  APIENTRY GetCharABCWidthsA( _In_ HDC hdc,
                                            _In_ UINT wFirst,
                                            _In_ UINT wLast,
                                            _Out_writes_(wLast - wFirst + 1) LPABC lpABC);
WINGDIAPI BOOL  APIENTRY GetCharABCWidthsW( _In_ HDC hdc,
                                            _In_ UINT wFirst,
                                            _In_ UINT wLast,
                                            _Out_writes_(wLast - wFirst + 1) LPABC lpABC);
#ifdef UNICODE
#define GetCharABCWidths  GetCharABCWidthsW
#else
#define GetCharABCWidths  GetCharABCWidthsA
#endif // !UNICODE

WINGDIAPI BOOL  APIENTRY GetCharABCWidthsFloatA( _In_ HDC hdc, _In_ UINT iFirst, _In_ UINT iLast, _Out_writes_(iLast + 1 - iFirst) LPABCFLOAT lpABC);
WINGDIAPI BOOL  APIENTRY GetCharABCWidthsFloatW( _In_ HDC hdc, _In_ UINT iFirst, _In_ UINT iLast, _Out_writes_(iLast + 1 - iFirst) LPABCFLOAT lpABC);
#ifdef UNICODE
#define GetCharABCWidthsFloat  GetCharABCWidthsFloatW
#else
#define GetCharABCWidthsFloat  GetCharABCWidthsFloatA
#endif // !UNICODE
WINGDIAPI int   WINAPI GetClipBox( _In_ HDC hdc,  _Out_ LPRECT lprect);
WINGDIAPI int   WINAPI GetClipRgn( _In_ HDC hdc, _In_ HRGN hrgn);
WINGDIAPI int   WINAPI GetMetaRgn( _In_ HDC hdc, _In_ HRGN hrgn);
WINGDIAPI HGDIOBJ WINAPI GetCurrentObject( _In_ HDC hdc, _In_ UINT type);
WINGDIAPI BOOL  WINAPI GetCurrentPositionEx( _In_ HDC hdc,  _Out_ LPPOINT lppt);
WINGDIAPI int   WINAPI GetDeviceCaps( _In_opt_ HDC hdc, _In_ int index);
WINGDIAPI int   WINAPI GetDIBits( _In_ HDC hdc, _In_ HBITMAP hbm, _In_ UINT start, _In_ UINT cLines,
    _Out_opt_ LPVOID lpvBits, _At_((LPBITMAPINFOHEADER)lpbmi, _Inout_) LPBITMAPINFO lpbmi, _In_ UINT usage);  // SAL actual size of lpbmi is computed from structure elements

_Success_(return != GDI_ERROR)
WINGDIAPI DWORD WINAPI GetFontData (    _In_ HDC     hdc,
                                        _In_ DWORD   dwTable,
                                        _In_ DWORD   dwOffset,
                                        _Out_writes_bytes_to_opt_(cjBuffer, return) PVOID pvBuffer,
                                        _In_ DWORD   cjBuffer
                                        );

WINGDIAPI DWORD WINAPI GetGlyphOutlineA(    _In_ HDC hdc,
                                            _In_ UINT uChar,
                                            _In_ UINT fuFormat,
                                            _Out_ LPGLYPHMETRICS lpgm,
                                            _In_ DWORD cjBuffer,
                                            _Out_writes_bytes_opt_(cjBuffer) LPVOID pvBuffer,
                                            _In_ CONST MAT2 *lpmat2
                                        );
WINGDIAPI DWORD WINAPI GetGlyphOutlineW(    _In_ HDC hdc,
                                            _In_ UINT uChar,
                                            _In_ UINT fuFormat,
                                            _Out_ LPGLYPHMETRICS lpgm,
                                            _In_ DWORD cjBuffer,
                                            _Out_writes_bytes_opt_(cjBuffer) LPVOID pvBuffer,
                                            _In_ CONST MAT2 *lpmat2
                                        );
#ifdef UNICODE
#define GetGlyphOutline  GetGlyphOutlineW
#else
#define GetGlyphOutline  GetGlyphOutlineA
#endif // !UNICODE

WINGDIAPI int   WINAPI GetGraphicsMode( _In_ HDC hdc);
WINGDIAPI int   WINAPI GetMapMode( _In_ HDC hdc);
WINGDIAPI UINT  WINAPI GetMetaFileBitsEx(_In_ HMETAFILE hMF, _In_ UINT cbBuffer, _Out_writes_bytes_opt_(cbBuffer) LPVOID lpData);
WINGDIAPI HMETAFILE   WINAPI GetMetaFileA( _In_ LPCSTR lpName);
WINGDIAPI HMETAFILE   WINAPI GetMetaFileW( _In_ LPCWSTR lpName);
#ifdef UNICODE
#define GetMetaFile  GetMetaFileW
#else
#define GetMetaFile  GetMetaFileA
#endif // !UNICODE
WINGDIAPI COLORREF WINAPI GetNearestColor( _In_ HDC hdc, _In_ COLORREF color);
WINGDIAPI UINT  WINAPI GetNearestPaletteIndex( _In_ HPALETTE h, _In_ COLORREF color);
_Post_satisfies_((return == 0) || (return >= GDI_MIN_OBJ_TYPE && return <= GDI_MAX_OBJ_TYPE))
WINGDIAPI DWORD WINAPI GetObjectType( _In_ HGDIOBJ h);

#ifndef NOTEXTMETRIC

WINGDIAPI UINT APIENTRY GetOutlineTextMetricsA( _In_ HDC hdc,
                                                _In_ UINT cjCopy,
                                                _Out_writes_bytes_opt_(cjCopy) LPOUTLINETEXTMETRICA potm);
WINGDIAPI UINT APIENTRY GetOutlineTextMetricsW( _In_ HDC hdc,
                                                _In_ UINT cjCopy,
                                                _Out_writes_bytes_opt_(cjCopy) LPOUTLINETEXTMETRICW potm);
#ifdef UNICODE
#define GetOutlineTextMetrics  GetOutlineTextMetricsW
#else
#define GetOutlineTextMetrics  GetOutlineTextMetricsA
#endif // !UNICODE

#endif /* NOTEXTMETRIC */

_Ret_range_(0,cEntries)
WINGDIAPI UINT  WINAPI GetPaletteEntries(   _In_ HPALETTE hpal,
                                            _In_ UINT iStart,
                                            _In_ UINT cEntries,
                                            _Out_writes_to_opt_(cEntries,return) LPPALETTEENTRY pPalEntries);
WINGDIAPI COLORREF WINAPI GetPixel( _In_ HDC hdc, _In_ int x, _In_ int y);
WINGDIAPI int   WINAPI GetPixelFormat( _In_ HDC hdc);
WINGDIAPI int   WINAPI GetPolyFillMode( _In_ HDC hdc);
WINGDIAPI BOOL  WINAPI GetRasterizerCaps(   _Out_writes_bytes_(cjBytes) LPRASTERIZER_STATUS lpraststat,
                                            _In_ UINT cjBytes);

WINGDIAPI int   WINAPI GetRandomRgn (_In_ HDC hdc, _In_ HRGN hrgn, _In_ INT i);
WINGDIAPI DWORD WINAPI GetRegionData(   _In_ HRGN hrgn,
                                        _In_ DWORD nCount,
                                        _Out_writes_bytes_to_opt_(nCount, return) LPRGNDATA lpRgnData);
WINGDIAPI int   WINAPI GetRgnBox( _In_ HRGN hrgn,  _Out_ LPRECT lprc);
WINGDIAPI HGDIOBJ WINAPI GetStockObject( _In_ int i);
WINGDIAPI int   WINAPI GetStretchBltMode(_In_ HDC hdc);
WINGDIAPI
UINT
WINAPI
GetSystemPaletteEntries(
    _In_ HDC  hdc,
    _In_ UINT iStart,
    _In_ UINT cEntries,
    _Out_writes_opt_(cEntries) LPPALETTEENTRY pPalEntries
    );

WINGDIAPI UINT  WINAPI GetSystemPaletteUse(_In_ HDC hdc);
WINGDIAPI int   WINAPI GetTextCharacterExtra(_In_ HDC hdc);
WINGDIAPI UINT  WINAPI GetTextAlign(_In_ HDC hdc);
WINGDIAPI COLORREF WINAPI GetTextColor(_In_ HDC hdc);

WINGDIAPI
BOOL
APIENTRY
GetTextExtentPointA(
    _In_ HDC hdc,
    _In_reads_(c) LPCSTR lpString,
    _In_ int c,
    _Out_ LPSIZE lpsz
    );
WINGDIAPI
BOOL
APIENTRY
GetTextExtentPointW(
    _In_ HDC hdc,
    _In_reads_(c) LPCWSTR lpString,
    _In_ int c,
    _Out_ LPSIZE lpsz
    );
#ifdef UNICODE
#define GetTextExtentPoint  GetTextExtentPointW
#else
#define GetTextExtentPoint  GetTextExtentPointA
#endif // !UNICODE

WINGDIAPI
BOOL
APIENTRY
GetTextExtentPoint32A(
    _In_ HDC hdc,
    _In_reads_(c) LPCSTR lpString,
    _In_ int c,
    _Out_ LPSIZE psizl
    );
WINGDIAPI
BOOL
APIENTRY
GetTextExtentPoint32W(
    _In_ HDC hdc,
    _In_reads_(c) LPCWSTR lpString,
    _In_ int c,
    _Out_ LPSIZE psizl
    );
#ifdef UNICODE
#define GetTextExtentPoint32  GetTextExtentPoint32W
#else
#define GetTextExtentPoint32  GetTextExtentPoint32A
#endif // !UNICODE

WINGDIAPI
BOOL
APIENTRY
GetTextExtentExPointA(
    _In_ HDC hdc,
    _In_reads_(cchString) LPCSTR lpszString,
    _In_ int cchString,
    _In_ int nMaxExtent,
    _Out_opt_ LPINT lpnFit,
    _Out_writes_to_opt_ (cchString, *lpnFit) LPINT lpnDx,
    _Out_ LPSIZE lpSize
    );
WINGDIAPI
BOOL
APIENTRY
GetTextExtentExPointW(
    _In_ HDC hdc,
    _In_reads_(cchString) LPCWSTR lpszString,
    _In_ int cchString,
    _In_ int nMaxExtent,
    _Out_opt_ LPINT lpnFit,
    _Out_writes_to_opt_ (cchString, *lpnFit) LPINT lpnDx,
    _Out_ LPSIZE lpSize
    );
#ifdef UNICODE
#define GetTextExtentExPoint  GetTextExtentExPointW
#else
#define GetTextExtentExPoint  GetTextExtentExPointA
#endif // !UNICODE

#if(WINVER >= 0x0400)
WINGDIAPI int WINAPI GetTextCharset( _In_ HDC hdc);
WINGDIAPI int WINAPI GetTextCharsetInfo( _In_ HDC hdc, _Out_opt_ LPFONTSIGNATURE lpSig, _In_ DWORD dwFlags);
WINGDIAPI BOOL WINAPI TranslateCharsetInfo( _Inout_ DWORD FAR *lpSrc,  _Out_ LPCHARSETINFO lpCs, _In_ DWORD dwFlags);
WINGDIAPI DWORD WINAPI GetFontLanguageInfo( _In_ HDC hdc);
WINGDIAPI DWORD WINAPI GetCharacterPlacementA(  _In_ HDC hdc, _In_reads_(nCount) LPCSTR lpString, _In_ int nCount, _In_ int nMexExtent, _Inout_ LPGCP_RESULTSA lpResults, _In_ DWORD dwFlags);
WINGDIAPI DWORD WINAPI GetCharacterPlacementW(  _In_ HDC hdc, _In_reads_(nCount) LPCWSTR lpString, _In_ int nCount, _In_ int nMexExtent, _Inout_ LPGCP_RESULTSW lpResults, _In_ DWORD dwFlags);
#ifdef UNICODE
#define GetCharacterPlacement  GetCharacterPlacementW
#else
#define GetCharacterPlacement  GetCharacterPlacementA
#endif // !UNICODE
#endif /* WINVER >= 0x0400 */

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#if (_WIN32_WINNT >= _WIN32_WINNT_WIN2K)

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

typedef struct tagWCRANGE
{
    WCHAR  wcLow;
    USHORT cGlyphs;
} WCRANGE, *PWCRANGE,FAR *LPWCRANGE;


typedef struct tagGLYPHSET
{
    DWORD    cbThis;
    DWORD    flAccel;
    DWORD    cGlyphsSupported;
    DWORD    cRanges;
    WCRANGE  ranges[1];
} GLYPHSET, *PGLYPHSET, FAR *LPGLYPHSET;

/* flAccel flags for the GLYPHSET structure above */

#define GS_8BIT_INDICES     0x00000001

/* flags for GetGlyphIndices */

#define GGI_MARK_NONEXISTING_GLYPHS  0X0001

WINGDIAPI DWORD WINAPI GetFontUnicodeRanges( _In_ HDC hdc, _Out_opt_ LPGLYPHSET lpgs);
WINGDIAPI DWORD WINAPI GetGlyphIndicesA( _In_ HDC hdc, _In_reads_(c) LPCSTR lpstr, _In_ int c, _Out_writes_(c) LPWORD pgi, _In_ DWORD fl);
WINGDIAPI DWORD WINAPI GetGlyphIndicesW( _In_ HDC hdc, _In_reads_(c) LPCWSTR lpstr, _In_ int c, _Out_writes_(c) LPWORD pgi, _In_ DWORD fl);
#ifdef UNICODE
#define GetGlyphIndices  GetGlyphIndicesW
#else
#define GetGlyphIndices  GetGlyphIndicesA
#endif // !UNICODE
WINGDIAPI BOOL  WINAPI GetTextExtentPointI(_In_ HDC hdc, _In_reads_(cgi) LPWORD pgiIn, _In_ int cgi, _Out_ LPSIZE psize);
WINGDIAPI BOOL  WINAPI GetTextExtentExPointI (  _In_ HDC hdc,
                                                _In_reads_(cwchString) LPWORD lpwszString,
                                                _In_ int cwchString,
                                                _In_ int nMaxExtent,
                                                _Out_opt_ LPINT lpnFit,
                                                _Out_writes_to_opt_(cwchString, *lpnFit) LPINT lpnDx,
                                                _Out_ LPSIZE lpSize
                                                );

WINGDIAPI BOOL  WINAPI GetCharWidthI(   _In_ HDC hdc,
                                        _In_ UINT giFirst,
                                        _In_ UINT cgi,
                                        _In_reads_opt_(cgi) LPWORD pgi,
                                        _Out_writes_(cgi) LPINT piWidths
                                        );

WINGDIAPI BOOL  WINAPI GetCharABCWidthsI(   _In_ HDC    hdc,
                                            _In_ UINT   giFirst,
                                            _In_ UINT   cgi,
                                            _In_reads_opt_(cgi) LPWORD pgi,
                                            _Out_writes_(cgi) LPABC  pabc
                                        );


#define STAMP_DESIGNVECTOR  (0x8000000 + 'd' + ('v' << 8))
#define STAMP_AXESLIST      (0x8000000 + 'a' + ('l' << 8))
#define STAMP_TRUETYPE_VARIATION (0x8000000 + 't' + ('v' << 8))
#define STAMP_CFF2          (0x8000000 + 'c' + ('v' << 8))
#define MM_MAX_NUMAXES      16



typedef struct tagDESIGNVECTOR
{
    DWORD  dvReserved;
    DWORD  dvNumAxes;
    LONG   dvValues[MM_MAX_NUMAXES];
} DESIGNVECTOR, *PDESIGNVECTOR, FAR *LPDESIGNVECTOR;

WINGDIAPI int  WINAPI AddFontResourceExA( _In_ LPCSTR name, _In_ DWORD fl, _Reserved_ PVOID res);
WINGDIAPI int  WINAPI AddFontResourceExW( _In_ LPCWSTR name, _In_ DWORD fl, _Reserved_ PVOID res);
#ifdef UNICODE
#define AddFontResourceEx  AddFontResourceExW
#else
#define AddFontResourceEx  AddFontResourceExA
#endif // !UNICODE
WINGDIAPI BOOL WINAPI RemoveFontResourceExA( _In_ LPCSTR name, _In_ DWORD fl, _Reserved_ PVOID pdv);
WINGDIAPI BOOL WINAPI RemoveFontResourceExW( _In_ LPCWSTR name, _In_ DWORD fl, _Reserved_ PVOID pdv);
#ifdef UNICODE
#define RemoveFontResourceEx  RemoveFontResourceExW
#else
#define RemoveFontResourceEx  RemoveFontResourceExA
#endif // !UNICODE
WINGDIAPI HANDLE WINAPI AddFontMemResourceEx(   _In_reads_bytes_(cjSize) PVOID pFileView,
                                                _In_ DWORD cjSize,
                                                _Reserved_ PVOID pvResrved,
                                                _In_ DWORD* pNumFonts);

WINGDIAPI BOOL WINAPI RemoveFontMemResourceEx( _In_ HANDLE h);
#define FR_PRIVATE     0x10
#define FR_NOT_ENUM    0x20

// The actual size of the DESIGNVECTOR and ENUMLOGFONTEXDV structures
// is determined by dvNumAxes,
// MM_MAX_NUMAXES only detemines the maximal size allowed

#define MM_MAX_AXES_NAMELEN 16

typedef struct tagAXISINFOA
{
    LONG   axMinValue;
    LONG   axMaxValue;
    BYTE   axAxisName[MM_MAX_AXES_NAMELEN];
} AXISINFOA, *PAXISINFOA, FAR *LPAXISINFOA;
typedef struct tagAXISINFOW
{
    LONG   axMinValue;
    LONG   axMaxValue;
    WCHAR  axAxisName[MM_MAX_AXES_NAMELEN];
} AXISINFOW, *PAXISINFOW, FAR *LPAXISINFOW;
#ifdef UNICODE
typedef AXISINFOW AXISINFO;
typedef PAXISINFOW PAXISINFO;
typedef LPAXISINFOW LPAXISINFO;
#else
typedef AXISINFOA AXISINFO;
typedef PAXISINFOA PAXISINFO;
typedef LPAXISINFOA LPAXISINFO;
#endif // UNICODE

typedef struct tagAXESLISTA
{
    DWORD     axlReserved;
    DWORD     axlNumAxes;
    AXISINFOA axlAxisInfo[MM_MAX_NUMAXES];
} AXESLISTA, *PAXESLISTA, FAR *LPAXESLISTA;
typedef struct tagAXESLISTW
{
    DWORD     axlReserved;
    DWORD     axlNumAxes;
    AXISINFOW axlAxisInfo[MM_MAX_NUMAXES];
} AXESLISTW, *PAXESLISTW, FAR *LPAXESLISTW;
#ifdef UNICODE
typedef AXESLISTW AXESLIST;
typedef PAXESLISTW PAXESLIST;
typedef LPAXESLISTW LPAXESLIST;
#else
typedef AXESLISTA AXESLIST;
typedef PAXESLISTA PAXESLIST;
typedef LPAXESLISTA LPAXESLIST;
#endif // UNICODE

// The actual size of the AXESLIST and ENUMTEXTMETRIC structure is
// determined by axlNumAxes,
// MM_MAX_NUMAXES only detemines the maximal size allowed

typedef struct tagENUMLOGFONTEXDVA
{
    ENUMLOGFONTEXA elfEnumLogfontEx;
    DESIGNVECTOR   elfDesignVector;
} ENUMLOGFONTEXDVA, *PENUMLOGFONTEXDVA, FAR *LPENUMLOGFONTEXDVA;
typedef struct tagENUMLOGFONTEXDVW
{
    ENUMLOGFONTEXW elfEnumLogfontEx;
    DESIGNVECTOR   elfDesignVector;
} ENUMLOGFONTEXDVW, *PENUMLOGFONTEXDVW, FAR *LPENUMLOGFONTEXDVW;
#ifdef UNICODE
typedef ENUMLOGFONTEXDVW ENUMLOGFONTEXDV;
typedef PENUMLOGFONTEXDVW PENUMLOGFONTEXDV;
typedef LPENUMLOGFONTEXDVW LPENUMLOGFONTEXDV;
#else
typedef ENUMLOGFONTEXDVA ENUMLOGFONTEXDV;
typedef PENUMLOGFONTEXDVA PENUMLOGFONTEXDV;
typedef LPENUMLOGFONTEXDVA LPENUMLOGFONTEXDV;
#endif // UNICODE

WINGDIAPI HFONT  WINAPI CreateFontIndirectExA( _In_ CONST ENUMLOGFONTEXDVA *);
WINGDIAPI HFONT  WINAPI CreateFontIndirectExW( _In_ CONST ENUMLOGFONTEXDVW *);
#ifdef UNICODE
#define CreateFontIndirectEx  CreateFontIndirectExW
#else
#define CreateFontIndirectEx  CreateFontIndirectExA
#endif // !UNICODE

#ifndef NOTEXTMETRIC
typedef struct tagENUMTEXTMETRICA
{
    NEWTEXTMETRICEXA etmNewTextMetricEx;
    AXESLISTA        etmAxesList;
} ENUMTEXTMETRICA, *PENUMTEXTMETRICA, FAR *LPENUMTEXTMETRICA;
typedef struct tagENUMTEXTMETRICW
{
    NEWTEXTMETRICEXW etmNewTextMetricEx;
    AXESLISTW        etmAxesList;
} ENUMTEXTMETRICW, *PENUMTEXTMETRICW, FAR *LPENUMTEXTMETRICW;
#ifdef UNICODE
typedef ENUMTEXTMETRICW ENUMTEXTMETRIC;
typedef PENUMTEXTMETRICW PENUMTEXTMETRIC;
typedef LPENUMTEXTMETRICW LPENUMTEXTMETRIC;
#else
typedef ENUMTEXTMETRICA ENUMTEXTMETRIC;
typedef PENUMTEXTMETRICA PENUMTEXTMETRIC;
typedef LPENUMTEXTMETRICA LPENUMTEXTMETRIC;
#endif // UNICODE
#endif /* NOTEXTMETRIC */

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif // (_WIN32_WINNT >= _WIN32_WINNT_WIN2K)

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

WINGDIAPI BOOL  WINAPI GetViewportExtEx( _In_ HDC hdc, _Out_ LPSIZE lpsize);
WINGDIAPI BOOL  WINAPI GetViewportOrgEx( _In_ HDC hdc, _Out_ LPPOINT lppoint);
WINGDIAPI BOOL  WINAPI GetWindowExtEx( _In_ HDC hdc, _Out_ LPSIZE lpsize);
WINGDIAPI BOOL  WINAPI GetWindowOrgEx( _In_ HDC hdc, _Out_ LPPOINT lppoint);

 WINGDIAPI int  WINAPI IntersectClipRect( _In_ HDC hdc, _In_ int left, _In_ int top, _In_ int right, _In_ int bottom);
 WINGDIAPI BOOL WINAPI InvertRgn( _In_ HDC hdc, _In_ HRGN hrgn);
WINGDIAPI BOOL WINAPI LineDDA( _In_ int xStart, _In_ int yStart, _In_ int xEnd, _In_ int yEnd, _In_ LINEDDAPROC lpProc, _In_opt_ LPARAM data);
 WINGDIAPI BOOL WINAPI LineTo( _In_ HDC hdc, _In_ int x, _In_ int y);
WINGDIAPI BOOL WINAPI MaskBlt( _In_ HDC hdcDest, _In_ int xDest, _In_ int yDest, _In_ int width, _In_ int height,
              _In_ HDC hdcSrc, _In_ int xSrc, _In_ int ySrc, _In_ HBITMAP hbmMask, _In_ int xMask, _In_ int yMask, _In_ DWORD rop);
WINGDIAPI BOOL WINAPI PlgBlt( _In_ HDC hdcDest, _In_reads_(3) CONST POINT * lpPoint, _In_ HDC hdcSrc, _In_ int xSrc, _In_ int ySrc, _In_ int width,
                     _In_ int height, _In_opt_ HBITMAP hbmMask, _In_ int xMask, _In_ int yMask);

 WINGDIAPI int  WINAPI OffsetClipRgn(_In_ HDC hdc, _In_ int x, _In_ int y);
WINGDIAPI int  WINAPI OffsetRgn(_In_ HRGN hrgn, _In_ int x, _In_ int y);
 WINGDIAPI BOOL WINAPI PatBlt(_In_ HDC hdc, _In_ int x, _In_ int y, _In_ int w, _In_ int h, _In_ DWORD rop);
 WINGDIAPI BOOL WINAPI Pie(_In_ HDC hdc, _In_ int left, _In_ int top, _In_ int right, _In_ int bottom, _In_ int xr1, _In_ int yr1, _In_ int xr2, _In_ int yr2);
WINGDIAPI BOOL WINAPI PlayMetaFile(_In_ HDC hdc, _In_ HMETAFILE hmf);
 WINGDIAPI BOOL WINAPI PaintRgn(_In_ HDC hdc, _In_ HRGN hrgn);
 WINGDIAPI BOOL WINAPI PolyPolygon(_In_ HDC hdc,  _In_ CONST POINT *apt,  _In_reads_(csz) CONST INT *asz,  _In_ int csz);
WINGDIAPI BOOL WINAPI PtInRegion(_In_ HRGN hrgn, _In_ int x, _In_ int y);
WINGDIAPI BOOL WINAPI PtVisible(_In_ HDC hdc, _In_ int x, _In_ int y);
WINGDIAPI BOOL WINAPI RectInRegion(_In_ HRGN hrgn, _In_ CONST RECT * lprect);
WINGDIAPI BOOL WINAPI RectVisible(_In_ HDC hdc, _In_ CONST RECT * lprect);
 WINGDIAPI BOOL WINAPI Rectangle(_In_ HDC hdc, _In_ int left, _In_ int top, _In_ int right, _In_ int bottom);
 WINGDIAPI BOOL WINAPI RestoreDC(_In_ HDC hdc, _In_ int nSavedDC);
 WINGDIAPI HDC  WINAPI ResetDCA(_In_ HDC hdc, _In_ CONST DEVMODEA * lpdm);
 WINGDIAPI HDC  WINAPI ResetDCW(_In_ HDC hdc, _In_ CONST DEVMODEW * lpdm);
#ifdef UNICODE
#define ResetDC  ResetDCW
#else
#define ResetDC  ResetDCA
#endif // !UNICODE
 WINGDIAPI UINT WINAPI RealizePalette(_In_ HDC hdc);
WINGDIAPI BOOL WINAPI RemoveFontResourceA(_In_ LPCSTR lpFileName);
WINGDIAPI BOOL WINAPI RemoveFontResourceW(_In_ LPCWSTR lpFileName);
#ifdef UNICODE
#define RemoveFontResource  RemoveFontResourceW
#else
#define RemoveFontResource  RemoveFontResourceA
#endif // !UNICODE
 WINGDIAPI BOOL  WINAPI RoundRect(_In_ HDC hdc, _In_ int left, _In_ int top, _In_ int right, _In_ int bottom, _In_ int width, _In_ int height);
 WINGDIAPI BOOL WINAPI ResizePalette(_In_ HPALETTE hpal, _In_ UINT n);

 WINGDIAPI int  WINAPI SaveDC(_In_ HDC hdc);
 WINGDIAPI int  WINAPI SelectClipRgn(_In_ HDC hdc, _In_opt_ HRGN hrgn);
WINGDIAPI int  WINAPI ExtSelectClipRgn(_In_ HDC hdc, _In_opt_ HRGN hrgn, _In_ int mode);
WINGDIAPI int  WINAPI SetMetaRgn(_In_ HDC hdc);
 WINGDIAPI HGDIOBJ WINAPI SelectObject(_In_ HDC hdc, _In_ HGDIOBJ h);
 WINGDIAPI HPALETTE WINAPI SelectPalette(_In_ HDC hdc, _In_ HPALETTE hPal, _In_ BOOL bForceBkgd);
 WINGDIAPI COLORREF WINAPI SetBkColor(_In_ HDC hdc, _In_ COLORREF color);

#if (_WIN32_WINNT >= _WIN32_WINNT_WIN2K)
WINGDIAPI COLORREF WINAPI SetDCBrushColor(_In_ HDC hdc, _In_ COLORREF color);
WINGDIAPI COLORREF WINAPI SetDCPenColor(_In_ HDC hdc, _In_ COLORREF color);
#endif

 WINGDIAPI int   WINAPI SetBkMode(_In_ HDC hdc, _In_ int mode);

WINGDIAPI
LONG WINAPI
SetBitmapBits(
    _In_ HBITMAP hbm,
    _In_ DWORD cb,
    _In_reads_bytes_(cb) CONST VOID *pvBits);

WINGDIAPI UINT  WINAPI SetBoundsRect(_In_ HDC hdc, _In_opt_ CONST RECT * lprect, _In_ UINT flags);
WINGDIAPI int   WINAPI SetDIBits(_In_opt_ HDC hdc, _In_ HBITMAP hbm, _In_ UINT start, _In_ UINT cLines, _In_ CONST VOID *lpBits, _In_ CONST BITMAPINFO * lpbmi, _In_ UINT ColorUse);
 WINGDIAPI int   WINAPI SetDIBitsToDevice(_In_ HDC hdc, _In_ int xDest, _In_ int yDest, _In_ DWORD w, _In_ DWORD h, _In_ int xSrc,
        _In_ int ySrc, _In_ UINT StartScan, _In_ UINT cLines, _In_ CONST VOID * lpvBits, _In_ CONST BITMAPINFO * lpbmi, _In_ UINT ColorUse);
 WINGDIAPI DWORD WINAPI SetMapperFlags(_In_ HDC hdc, _In_ DWORD flags);
WINGDIAPI int   WINAPI SetGraphicsMode(_In_ HDC hdc, _In_ int iMode);
 WINGDIAPI int   WINAPI SetMapMode(_In_ HDC hdc, _In_ int iMode);

#if(WINVER >= 0x0500)
 WINGDIAPI DWORD WINAPI SetLayout(_In_ HDC hdc, _In_ DWORD l);
WINGDIAPI DWORD WINAPI GetLayout(_In_ HDC hdc);
#endif /* WINVER >= 0x0500 */

WINGDIAPI HMETAFILE   WINAPI SetMetaFileBitsEx(_In_ UINT cbBuffer, _In_reads_bytes_(cbBuffer) CONST BYTE *lpData);
 WINGDIAPI UINT  WINAPI SetPaletteEntries(   _In_ HPALETTE hpal,
                                            _In_ UINT iStart,
                                            _In_ UINT cEntries,
                                            _In_reads_(cEntries) CONST PALETTEENTRY *pPalEntries);
 WINGDIAPI COLORREF WINAPI SetPixel(_In_ HDC hdc, _In_ int x, _In_ int y, _In_ COLORREF color);
WINGDIAPI BOOL   WINAPI SetPixelV(_In_ HDC hdc, _In_ int x, _In_ int y, _In_ COLORREF color);
WINGDIAPI BOOL  WINAPI SetPixelFormat(_In_ HDC hdc, _In_ int format, _In_ CONST PIXELFORMATDESCRIPTOR * ppfd);
 WINGDIAPI int   WINAPI SetPolyFillMode(_In_ HDC hdc, _In_ int mode);
 WINGDIAPI BOOL  WINAPI StretchBlt(_In_ HDC hdcDest, _In_ int xDest, _In_ int yDest, _In_ int wDest, _In_ int hDest, _In_opt_ HDC hdcSrc, _In_ int xSrc, _In_ int ySrc, _In_ int wSrc, _In_ int hSrc, _In_ DWORD rop);
WINGDIAPI BOOL   WINAPI SetRectRgn(_In_ HRGN hrgn, _In_ int left, _In_ int top, _In_ int right, _In_ int bottom);
 WINGDIAPI int   WINAPI StretchDIBits(_In_ HDC hdc, _In_ int xDest, _In_ int yDest, _In_ int DestWidth, _In_ int DestHeight, _In_ int xSrc, _In_ int ySrc, _In_ int SrcWidth, _In_ int SrcHeight,
        _In_opt_ CONST VOID * lpBits, _In_ CONST BITMAPINFO * lpbmi, _In_ UINT iUsage, _In_ DWORD rop);
 WINGDIAPI int   WINAPI SetROP2(_In_ HDC hdc, _In_ int rop2);
 WINGDIAPI int   WINAPI SetStretchBltMode(_In_ HDC hdc, _In_ int mode);
WINGDIAPI UINT  WINAPI SetSystemPaletteUse(_In_ HDC hdc, _In_ UINT use);
 WINGDIAPI int   WINAPI SetTextCharacterExtra(_In_ HDC hdc, _In_ int extra);
 WINGDIAPI COLORREF WINAPI SetTextColor(_In_ HDC hdc, _In_ COLORREF color);
 WINGDIAPI UINT  WINAPI SetTextAlign(_In_ HDC hdc, _In_ UINT align);
 WINGDIAPI BOOL  WINAPI SetTextJustification(_In_ HDC hdc, _In_ int extra, _In_ int count);
WINGDIAPI BOOL  WINAPI UpdateColors(_In_ HDC hdc);

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


#ifdef  COMBOX_SANDBOX

#if (_WIN32_WINNT >= 0x0600)

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

typedef PVOID   (WINAPI *GDIMARSHALLOC)(DWORD dwSize, _In_ LPVOID pGdiRef);

typedef HRESULT (WINAPI *DDRAWMARSHCALLBACKMARSHAL)(_In_ HGDIOBJ hGdiObj, _In_ LPVOID pGdiRef, _Out_ LPVOID *ppDDrawRef);
typedef HRESULT (WINAPI *DDRAWMARSHCALLBACKUNMARSHAL)(_In_ LPVOID pData, _Out_ HDC *phdc, _Out_ LPVOID *ppDDrawRef);
typedef HRESULT (WINAPI *DDRAWMARSHCALLBACKRELEASE)(_In_ LPVOID pDDrawRef);

#define GDIREGISTERDDRAWPACKETVERSION   0x1

typedef struct {
    DWORD                       dwSize;
    DWORD                       dwVersion;
    DDRAWMARSHCALLBACKMARSHAL   pfnDdMarshal;
    DDRAWMARSHCALLBACKUNMARSHAL pfnDdUnmarshal;
    DDRAWMARSHCALLBACKRELEASE   pfnDdRelease;
} GDIREGISTERDDRAWPACKET, *PGDIREGISTERDDRAWPACKET;

WINGDIAPI BOOL    WINAPI   GdiRegisterDdraw(_In_ PGDIREGISTERDDRAWPACKET pPacket, _Out_ GDIMARSHALLOC *ppfnGdiAlloc);

WINGDIAPI ULONG   WINAPI   GdiMarshalSize(VOID);
WINGDIAPI VOID    WINAPI   GdiMarshal(DWORD dwProcessIdTo, _In_ HGDIOBJ hGdiObj, _Inout_ PVOID pData, ULONG ulFlags);
WINGDIAPI HGDIOBJ WINAPI   GdiUnmarshal(_In_ PVOID pData, ULONG ulFlags);

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif // (_WIN32_WINNT >= 0x0600)

#endif  // COMBOX_SANDBOX

#if (WINVER >= 0x0400)

//
// image blt
//

#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)

typedef USHORT COLOR16;

typedef struct _TRIVERTEX
{
    LONG    x;
    LONG    y;
    COLOR16 Red;
    COLOR16 Green;
    COLOR16 Blue;
    COLOR16 Alpha;
}TRIVERTEX,*PTRIVERTEX,*LPTRIVERTEX;

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

typedef struct _GRADIENT_TRIANGLE
{
    ULONG Vertex1;
    ULONG Vertex2;
    ULONG Vertex3;
} GRADIENT_TRIANGLE,*PGRADIENT_TRIANGLE,*LPGRADIENT_TRIANGLE;

typedef struct _GRADIENT_RECT
{
    ULONG UpperLeft;
    ULONG LowerRight;
}GRADIENT_RECT,*PGRADIENT_RECT,*LPGRADIENT_RECT;

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)

typedef struct _BLENDFUNCTION
{
    BYTE   BlendOp;
    BYTE   BlendFlags;
    BYTE   SourceConstantAlpha;
    BYTE   AlphaFormat;
}BLENDFUNCTION,*PBLENDFUNCTION;

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


//
// currentlly defined blend function
//

#define AC_SRC_OVER                 0x00

//
// alpha format flags
//

#define AC_SRC_ALPHA                0x01

WINGDIAPI BOOL WINAPI AlphaBlend(
    _In_ HDC hdcDest,
    _In_ int xoriginDest,
    _In_ int yoriginDest,
    _In_ int wDest,
    _In_ int hDest,
    _In_ HDC hdcSrc,
    _In_ int xoriginSrc,
    _In_ int yoriginSrc,
    _In_ int wSrc,
    _In_ int hSrc,
    _In_ BLENDFUNCTION ftn);

WINGDIAPI BOOL WINAPI TransparentBlt(
    _In_ HDC hdcDest,
    _In_ int xoriginDest,
    _In_ int yoriginDest,
    _In_ int wDest,
    _In_ int hDest,
    _In_ HDC hdcSrc,
    _In_ int xoriginSrc,
    _In_ int yoriginSrc,
    _In_ int wSrc,
    _In_ int hSrc,
    _In_ UINT crTransparent);


//
// gradient drawing modes
//

#define GRADIENT_FILL_RECT_H    0x00000000
#define GRADIENT_FILL_RECT_V    0x00000001
#define GRADIENT_FILL_TRIANGLE  0x00000002
#define GRADIENT_FILL_OP_FLAG   0x000000ff

WINGDIAPI
BOOL
WINAPI
GradientFill(
    _In_ HDC hdc,
    _In_reads_(nVertex) PTRIVERTEX pVertex,
    _In_ ULONG nVertex,
    _In_ PVOID pMesh,
    _In_ ULONG nMesh,
    _In_ ULONG ulMode
    );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif // (WINVER >= 0x0400)

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

#if (_WIN32_WINNT >= _WIN32_WINNT_WIN2K)


WINGDIAPI BOOL  WINAPI GdiAlphaBlend(_In_ HDC hdcDest, _In_ int xoriginDest, _In_ int yoriginDest, _In_ int wDest, _In_ int hDest, _In_ HDC hdcSrc, _In_ int xoriginSrc, _In_ int yoriginSrc, _In_ int wSrc, _In_ int hSrc, _In_ BLENDFUNCTION ftn);

WINGDIAPI BOOL  WINAPI GdiTransparentBlt(_In_ HDC hdcDest,_In_ int xoriginDest, _In_ int yoriginDest, _In_ int wDest, _In_ int hDest, _In_ HDC hdcSrc,
                                           _In_ int xoriginSrc, _In_ int yoriginSrc, _In_ int wSrc, _In_ int hSrc, _In_ UINT crTransparent);

WINGDIAPI BOOL  WINAPI GdiGradientFill( _In_ HDC hdc,
                                        _In_reads_(nVertex) PTRIVERTEX pVertex,
                                        _In_ ULONG nVertex,
                                        _In_ PVOID pMesh,
                                        _In_ ULONG nCount,
                                        _In_ ULONG ulMode);

#endif



#ifndef NOMETAFILE

WINGDIAPI BOOL  WINAPI PlayMetaFileRecord(  _In_ HDC hdc,
                                            _In_reads_(noObjs) LPHANDLETABLE lpHandleTable,
                                            _In_ LPMETARECORD lpMR,
                                            _In_ UINT noObjs);

typedef int (CALLBACK* MFENUMPROC)( _In_ HDC hdc, _In_reads_(nObj) HANDLETABLE FAR* lpht, _In_ METARECORD FAR* lpMR, _In_ int nObj, _In_opt_ LPARAM param);
WINGDIAPI BOOL  WINAPI EnumMetaFile( _In_ HDC hdc, _In_ HMETAFILE hmf, _In_ MFENUMPROC proc, _In_opt_ LPARAM param);

typedef int (CALLBACK* ENHMFENUMPROC)(_In_ HDC hdc, _In_reads_(nHandles) HANDLETABLE FAR* lpht, _In_ CONST ENHMETARECORD * lpmr, _In_ int nHandles, _In_opt_ LPARAM data);

// Enhanced Metafile Function Declarations

WINGDIAPI HENHMETAFILE WINAPI CloseEnhMetaFile( _In_ HDC hdc);
WINGDIAPI HENHMETAFILE WINAPI CopyEnhMetaFileA( _In_ HENHMETAFILE hEnh, _In_opt_ LPCSTR lpFileName);
WINGDIAPI HENHMETAFILE WINAPI CopyEnhMetaFileW( _In_ HENHMETAFILE hEnh, _In_opt_ LPCWSTR lpFileName);
#ifdef UNICODE
#define CopyEnhMetaFile  CopyEnhMetaFileW
#else
#define CopyEnhMetaFile  CopyEnhMetaFileA
#endif // !UNICODE
WINGDIAPI HDC   WINAPI CreateEnhMetaFileA( _In_opt_ HDC hdc, _In_opt_ LPCSTR lpFilename, _In_opt_ CONST RECT *lprc, _In_opt_ LPCSTR lpDesc);
WINGDIAPI HDC   WINAPI CreateEnhMetaFileW( _In_opt_ HDC hdc, _In_opt_ LPCWSTR lpFilename, _In_opt_ CONST RECT *lprc, _In_opt_ LPCWSTR lpDesc);
#ifdef UNICODE
#define CreateEnhMetaFile  CreateEnhMetaFileW
#else
#define CreateEnhMetaFile  CreateEnhMetaFileA
#endif // !UNICODE
WINGDIAPI BOOL  WINAPI DeleteEnhMetaFile( _In_opt_ HENHMETAFILE hmf);
WINGDIAPI BOOL  WINAPI EnumEnhMetaFile( _In_opt_ HDC hdc, _In_ HENHMETAFILE hmf, _In_ ENHMFENUMPROC proc,
                                        _In_opt_ LPVOID param, _In_opt_ CONST RECT * lpRect);
WINGDIAPI HENHMETAFILE  WINAPI GetEnhMetaFileA( _In_ LPCSTR lpName);
WINGDIAPI HENHMETAFILE  WINAPI GetEnhMetaFileW( _In_ LPCWSTR lpName);
#ifdef UNICODE
#define GetEnhMetaFile  GetEnhMetaFileW
#else
#define GetEnhMetaFile  GetEnhMetaFileA
#endif // !UNICODE
WINGDIAPI UINT  WINAPI GetEnhMetaFileBits(  _In_ HENHMETAFILE hEMF,
                                            _In_ UINT nSize,
                                            _Out_writes_bytes_opt_(nSize) LPBYTE lpData);
WINGDIAPI UINT  WINAPI GetEnhMetaFileDescriptionA(  _In_ HENHMETAFILE hemf,
                                                    _In_ UINT cchBuffer,
                                                    _Out_writes_opt_(cchBuffer) LPSTR lpDescription);
WINGDIAPI UINT  WINAPI GetEnhMetaFileDescriptionW(  _In_ HENHMETAFILE hemf,
                                                    _In_ UINT cchBuffer,
                                                    _Out_writes_opt_(cchBuffer) LPWSTR lpDescription);
#ifdef UNICODE
#define GetEnhMetaFileDescription  GetEnhMetaFileDescriptionW
#else
#define GetEnhMetaFileDescription  GetEnhMetaFileDescriptionA
#endif // !UNICODE
WINGDIAPI UINT  WINAPI GetEnhMetaFileHeader(    _In_ HENHMETAFILE hemf,
                                                _In_ UINT nSize,
                                                _Out_writes_bytes_opt_(nSize) LPENHMETAHEADER lpEnhMetaHeader);
WINGDIAPI UINT  WINAPI GetEnhMetaFilePaletteEntries(_In_ HENHMETAFILE hemf,
                                                    _In_ UINT nNumEntries,
                                                    _Out_writes_opt_(nNumEntries) LPPALETTEENTRY lpPaletteEntries);

WINGDIAPI UINT  WINAPI GetEnhMetaFilePixelFormat(   _In_ HENHMETAFILE hemf,
                                                    _In_ UINT cbBuffer,
                                                    _Out_writes_bytes_opt_(cbBuffer) PIXELFORMATDESCRIPTOR *ppfd);
WINGDIAPI UINT  WINAPI GetWinMetaFileBits(  _In_ HENHMETAFILE hemf,
                                            _In_ UINT cbData16,
                                            _Out_writes_bytes_opt_(cbData16) LPBYTE pData16,
                                            _In_ INT iMapMode,
                                            _In_ HDC hdcRef);
WINGDIAPI BOOL  WINAPI PlayEnhMetaFile( _In_ HDC hdc, _In_ HENHMETAFILE hmf, _In_ CONST RECT * lprect);
WINGDIAPI BOOL  WINAPI PlayEnhMetaFileRecord(   _In_ HDC hdc,
                                                _In_reads_(cht) LPHANDLETABLE pht,
                                                _In_ CONST ENHMETARECORD *pmr,
                                                _In_ UINT cht);

WINGDIAPI HENHMETAFILE  WINAPI SetEnhMetaFileBits(  _In_ UINT nSize,
                                                    _In_reads_bytes_(nSize) CONST BYTE * pb);

WINGDIAPI HENHMETAFILE  WINAPI SetWinMetaFileBits(  _In_ UINT nSize,
                                                    _In_reads_bytes_(nSize) CONST BYTE *lpMeta16Data,
                                                    _In_opt_ HDC hdcRef,
                                                    _In_opt_ CONST METAFILEPICT *lpMFP);
WINGDIAPI BOOL  WINAPI GdiComment(_In_ HDC hdc, _In_ UINT nSize, _In_reads_bytes_(nSize) CONST BYTE *lpData);

#endif  /* NOMETAFILE */

#ifndef NOTEXTMETRIC

WINGDIAPI BOOL WINAPI GetTextMetricsA( _In_ HDC hdc, _Out_ LPTEXTMETRICA lptm);
WINGDIAPI BOOL WINAPI GetTextMetricsW( _In_ HDC hdc, _Out_ LPTEXTMETRICW lptm);
#ifdef UNICODE
#define GetTextMetrics  GetTextMetricsW
#else
#define GetTextMetrics  GetTextMetricsA
#endif // !UNICODE

#if defined(_M_CEE)
#undef GetTextMetrics
__inline
BOOL
GetTextMetrics(
    HDC hdc,
    LPTEXTMETRIC lptm
    )
{
#ifdef UNICODE
    return GetTextMetricsW(
#else
    return GetTextMetricsA(
#endif
        hdc,
        lptm
        );
}
#endif  /* _M_CEE */

#endif

/* new GDI */

typedef struct tagDIBSECTION {
    BITMAP       dsBm;
    BITMAPINFOHEADER    dsBmih;
    DWORD               dsBitfields[3];
    HANDLE              dshSection;
    DWORD               dsOffset;
} DIBSECTION, FAR *LPDIBSECTION, *PDIBSECTION;


WINGDIAPI BOOL WINAPI AngleArc( _In_ HDC hdc, _In_ int x, _In_ int y, _In_ DWORD r, _In_ FLOAT StartAngle, _In_ FLOAT SweepAngle);
WINGDIAPI BOOL WINAPI PolyPolyline(_In_ HDC hdc, _In_ CONST POINT *apt, _In_reads_(csz) CONST DWORD *asz, _In_ DWORD csz);
WINGDIAPI BOOL WINAPI GetWorldTransform( _In_ HDC hdc, _Out_ LPXFORM lpxf);
WINGDIAPI BOOL WINAPI SetWorldTransform( _In_ HDC hdc, _In_ CONST XFORM * lpxf);
WINGDIAPI BOOL WINAPI ModifyWorldTransform( _In_ HDC hdc, _In_opt_ CONST XFORM * lpxf, _In_ DWORD mode);
WINGDIAPI BOOL WINAPI CombineTransform( _Out_ LPXFORM lpxfOut, _In_ CONST XFORM *lpxf1, _In_ CONST XFORM *lpxf2);

#define GDI_WIDTHBYTES(bits) ((DWORD)(((bits)+31) & (~31)) / 8)
#define GDI_DIBWIDTHBYTES(bi) (DWORD)GDI_WIDTHBYTES((DWORD)(bi).biWidth * (DWORD)(bi).biBitCount)
#define GDI__DIBSIZE(bi) (GDI_DIBWIDTHBYTES(bi) * (DWORD)(bi).biHeight)
#define GDI_DIBSIZE(bi) ((bi).biHeight < 0 ? (-1)*(GDI__DIBSIZE(bi)) : GDI__DIBSIZE(bi))

WINGDIAPI _Success_(return != NULL) HBITMAP WINAPI CreateDIBSection(
    _In_opt_        HDC               hdc,
    _In_            CONST BITMAPINFO *pbmi,
    _In_            UINT              usage,
    _When_((pbmi->bmiHeader.biBitCount != 0), _Outptr_result_bytebuffer_(_Inexpressible_(GDI_DIBSIZE((pbmi->bmiHeader)))))
    _When_((pbmi->bmiHeader.biBitCount == 0), _Outptr_result_bytebuffer_((pbmi->bmiHeader).biSizeImage))
                    VOID            **ppvBits,
    _In_opt_        HANDLE            hSection,
    _In_            DWORD             offset);


_Ret_range_(0,cEntries)
WINGDIAPI UINT WINAPI GetDIBColorTable( _In_ HDC  hdc,
                                        _In_ UINT iStart,
                                        _In_ UINT cEntries,
                                        _Out_writes_to_(cEntries,return) RGBQUAD *prgbq);
WINGDIAPI UINT WINAPI SetDIBColorTable( _In_ HDC  hdc,
                                        _In_ UINT iStart,
                                        _In_ UINT cEntries,
                                        _In_reads_(cEntries) CONST RGBQUAD *prgbq);

/* Flags value for COLORADJUSTMENT */
#define CA_NEGATIVE                 0x0001
#define CA_LOG_FILTER               0x0002

/* IlluminantIndex values */
#define ILLUMINANT_DEVICE_DEFAULT   0
#define ILLUMINANT_A                1
#define ILLUMINANT_B                2
#define ILLUMINANT_C                3
#define ILLUMINANT_D50              4
#define ILLUMINANT_D55              5
#define ILLUMINANT_D65              6
#define ILLUMINANT_D75              7
#define ILLUMINANT_F2               8
#define ILLUMINANT_MAX_INDEX        ILLUMINANT_F2

#define ILLUMINANT_TUNGSTEN         ILLUMINANT_A
#define ILLUMINANT_DAYLIGHT         ILLUMINANT_C
#define ILLUMINANT_FLUORESCENT      ILLUMINANT_F2
#define ILLUMINANT_NTSC             ILLUMINANT_C

/* Min and max for RedGamma, GreenGamma, BlueGamma */
#define RGB_GAMMA_MIN               (WORD)02500
#define RGB_GAMMA_MAX               (WORD)65000

/* Min and max for ReferenceBlack and ReferenceWhite */
#define REFERENCE_WHITE_MIN         (WORD)6000
#define REFERENCE_WHITE_MAX         (WORD)10000
#define REFERENCE_BLACK_MIN         (WORD)0
#define REFERENCE_BLACK_MAX         (WORD)4000

/* Min and max for Contrast, Brightness, Colorfulness, RedGreenTint */
#define COLOR_ADJ_MIN               (SHORT)-100
#define COLOR_ADJ_MAX               (SHORT)100

typedef struct  tagCOLORADJUSTMENT {
    WORD   caSize;
    WORD   caFlags;
    WORD   caIlluminantIndex;
    WORD   caRedGamma;
    WORD   caGreenGamma;
    WORD   caBlueGamma;
    WORD   caReferenceBlack;
    WORD   caReferenceWhite;
    SHORT  caContrast;
    SHORT  caBrightness;
    SHORT  caColorfulness;
    SHORT  caRedGreenTint;
} COLORADJUSTMENT, *PCOLORADJUSTMENT, FAR *LPCOLORADJUSTMENT;

WINGDIAPI BOOL WINAPI SetColorAdjustment( _In_ HDC hdc, _In_ CONST COLORADJUSTMENT *lpca);
WINGDIAPI BOOL WINAPI GetColorAdjustment( _In_ HDC hdc, _Out_ LPCOLORADJUSTMENT lpca);
WINGDIAPI HPALETTE WINAPI CreateHalftonePalette( _In_opt_ HDC hdc);

#ifdef STRICT
typedef BOOL (CALLBACK* ABORTPROC)( _In_ HDC, _In_ int);
#else
typedef FARPROC ABORTPROC;
#endif

typedef struct _DOCINFOA {
    int     cbSize;
    LPCSTR   lpszDocName;
    LPCSTR   lpszOutput;
#if (WINVER >= 0x0400)
    LPCSTR   lpszDatatype;
    DWORD    fwType;
#endif /* WINVER */
} DOCINFOA, *LPDOCINFOA;
typedef struct _DOCINFOW {
    int     cbSize;
    LPCWSTR  lpszDocName;
    LPCWSTR  lpszOutput;
#if (WINVER >= 0x0400)
    LPCWSTR  lpszDatatype;
    DWORD    fwType;
#endif /* WINVER */
} DOCINFOW, *LPDOCINFOW;
#ifdef UNICODE
typedef DOCINFOW DOCINFO;
typedef LPDOCINFOW LPDOCINFO;
#else
typedef DOCINFOA DOCINFO;
typedef LPDOCINFOA LPDOCINFO;
#endif // UNICODE

#if(WINVER >= 0x0400)
#define DI_APPBANDING               0x00000001
#define DI_ROPS_READ_DESTINATION    0x00000002
#endif /* WINVER >= 0x0400 */

 WINGDIAPI int WINAPI StartDocA(_In_ HDC hdc, _In_ CONST DOCINFOA *lpdi);
 WINGDIAPI int WINAPI StartDocW(_In_ HDC hdc, _In_ CONST DOCINFOW *lpdi);
#ifdef UNICODE
#define StartDoc  StartDocW
#else
#define StartDoc  StartDocA
#endif // !UNICODE
 WINGDIAPI int WINAPI EndDoc(_In_ HDC hdc);
 WINGDIAPI int WINAPI StartPage(_In_ HDC hdc);
 WINGDIAPI int WINAPI EndPage(_In_ HDC hdc);
 WINGDIAPI int WINAPI AbortDoc(_In_ HDC hdc);
WINGDIAPI int WINAPI SetAbortProc(_In_ HDC hdc, _In_ ABORTPROC proc);

WINGDIAPI BOOL WINAPI AbortPath(_In_ HDC hdc);
WINGDIAPI BOOL WINAPI ArcTo(_In_ HDC hdc, _In_ int left, _In_ int top, _In_ int right, _In_ int bottom, _In_ int xr1, _In_ int yr1, _In_ int xr2, _In_ int yr2);
WINGDIAPI BOOL WINAPI BeginPath(_In_ HDC hdc);
WINGDIAPI BOOL WINAPI CloseFigure(_In_ HDC hdc);
WINGDIAPI BOOL WINAPI EndPath(_In_ HDC hdc);
WINGDIAPI BOOL WINAPI FillPath(_In_ HDC hdc);
WINGDIAPI BOOL WINAPI FlattenPath(_In_ HDC hdc);
WINGDIAPI int  WINAPI GetPath(_In_ HDC hdc, _Out_writes_opt_(cpt) LPPOINT apt, _Out_writes_opt_(cpt) LPBYTE aj, int cpt);
WINGDIAPI HRGN WINAPI PathToRegion(_In_ HDC hdc);
WINGDIAPI BOOL WINAPI PolyDraw(_In_ HDC hdc, _In_reads_(cpt) CONST POINT * apt, _In_reads_(cpt) CONST BYTE * aj, _In_ int cpt);
WINGDIAPI BOOL WINAPI SelectClipPath(_In_ HDC hdc, _In_ int mode);
WINGDIAPI int  WINAPI SetArcDirection(_In_ HDC hdc, _In_ int dir);
WINGDIAPI BOOL WINAPI SetMiterLimit(_In_ HDC hdc, _In_ FLOAT limit, _Out_opt_ PFLOAT old);
WINGDIAPI BOOL WINAPI StrokeAndFillPath(_In_ HDC hdc);
WINGDIAPI BOOL WINAPI StrokePath(_In_ HDC hdc);
WINGDIAPI BOOL WINAPI WidenPath(_In_ HDC hdc);
WINGDIAPI HPEN WINAPI ExtCreatePen( _In_ DWORD iPenStyle,
                                    _In_ DWORD cWidth,
                                    _In_ CONST LOGBRUSH *plbrush,
                                    _In_ DWORD cStyle,
                                    _In_reads_opt_(cStyle) CONST DWORD *pstyle);
WINGDIAPI BOOL WINAPI GetMiterLimit(_In_ HDC hdc, _Out_ PFLOAT plimit);
WINGDIAPI int  WINAPI GetArcDirection(_In_ HDC hdc);

WINGDIAPI int   WINAPI GetObjectA(_In_ HANDLE h, _In_ int c, _Out_writes_bytes_opt_(c) LPVOID pv);
WINGDIAPI int   WINAPI GetObjectW(_In_ HANDLE h, _In_ int c, _Out_writes_bytes_opt_(c) LPVOID pv);
#ifdef UNICODE
#define GetObject  GetObjectW
#else
#define GetObject  GetObjectA
#endif // !UNICODE
#if defined(_M_CEE)
#undef GetObject
__inline
int
GetObject(
    HANDLE h,
    int c,
    LPVOID pv
    )
{
#ifdef UNICODE
    return GetObjectW(
#else
    return GetObjectA(
#endif
        h,
        c,
        pv
        );
}
#endif  /* _M_CEE */


 WINGDIAPI BOOL  WINAPI MoveToEx( _In_ HDC hdc, _In_ int x, _In_ int y, _Out_opt_ LPPOINT lppt);
 WINGDIAPI BOOL  WINAPI TextOutA( _In_ HDC hdc, _In_ int x, _In_ int y, _In_reads_(c) LPCSTR lpString, _In_ int c);
 WINGDIAPI BOOL  WINAPI TextOutW( _In_ HDC hdc, _In_ int x, _In_ int y, _In_reads_(c) LPCWSTR lpString, _In_ int c);
#ifdef UNICODE
#define TextOut  TextOutW
#else
#define TextOut  TextOutA
#endif // !UNICODE
 WINGDIAPI BOOL  WINAPI ExtTextOutA( _In_ HDC hdc, _In_ int x, _In_ int y, _In_ UINT options, _In_opt_ CONST RECT * lprect, _In_reads_opt_(c) LPCSTR lpString, _In_ UINT c, _In_reads_opt_(c) CONST INT * lpDx);
 WINGDIAPI BOOL  WINAPI ExtTextOutW( _In_ HDC hdc, _In_ int x, _In_ int y, _In_ UINT options, _In_opt_ CONST RECT * lprect, _In_reads_opt_(c) LPCWSTR lpString, _In_ UINT c, _In_reads_opt_(c) CONST INT * lpDx);
#ifdef UNICODE
#define ExtTextOut  ExtTextOutW
#else
#define ExtTextOut  ExtTextOutA
#endif // !UNICODE
WINGDIAPI BOOL  WINAPI PolyTextOutA(_In_ HDC hdc, _In_reads_(nstrings) CONST POLYTEXTA * ppt, _In_ int nstrings);
WINGDIAPI BOOL  WINAPI PolyTextOutW(_In_ HDC hdc, _In_reads_(nstrings) CONST POLYTEXTW * ppt, _In_ int nstrings);
#ifdef UNICODE
#define PolyTextOut  PolyTextOutW
#else
#define PolyTextOut  PolyTextOutA
#endif // !UNICODE

WINGDIAPI HRGN  WINAPI CreatePolygonRgn(    _In_reads_(cPoint) CONST POINT *pptl,
                                            _In_ int cPoint,
                                            _In_ int iMode);
WINGDIAPI BOOL  WINAPI DPtoLP( _In_ HDC hdc, _Inout_updates_(c) LPPOINT lppt, _In_ int c);
WINGDIAPI BOOL  WINAPI LPtoDP( _In_ HDC hdc, _Inout_updates_(c) LPPOINT lppt, _In_ int c);
 WINGDIAPI BOOL  WINAPI Polygon(_In_ HDC hdc, _In_reads_(cpt) CONST POINT *apt, _In_ int cpt);
 WINGDIAPI BOOL  WINAPI Polyline(_In_ HDC hdc, _In_reads_(cpt) CONST POINT *apt, _In_ int cpt);

WINGDIAPI BOOL  WINAPI PolyBezier(_In_ HDC hdc, _In_reads_(cpt) CONST POINT * apt, _In_ DWORD cpt);
WINGDIAPI BOOL  WINAPI PolyBezierTo(_In_ HDC hdc, _In_reads_(cpt) CONST POINT * apt, _In_ DWORD cpt);
WINGDIAPI BOOL  WINAPI PolylineTo(_In_ HDC hdc, _In_reads_(cpt) CONST POINT * apt, _In_ DWORD cpt);

 WINGDIAPI BOOL  WINAPI SetViewportExtEx( _In_ HDC hdc, _In_ int x, _In_ int y, _Out_opt_ LPSIZE lpsz);
 WINGDIAPI BOOL  WINAPI SetViewportOrgEx( _In_ HDC hdc, _In_ int x, _In_ int y, _Out_opt_ LPPOINT lppt);
 WINGDIAPI BOOL  WINAPI SetWindowExtEx( _In_ HDC hdc, _In_ int x, _In_ int y, _Out_opt_ LPSIZE lpsz);
 WINGDIAPI BOOL  WINAPI SetWindowOrgEx( _In_ HDC hdc, _In_ int x, _In_ int y, _Out_opt_ LPPOINT lppt);

 WINGDIAPI BOOL  WINAPI OffsetViewportOrgEx( _In_ HDC hdc, _In_ int x, _In_ int y, _Out_opt_ LPPOINT lppt);
 WINGDIAPI BOOL  WINAPI OffsetWindowOrgEx( _In_ HDC hdc, _In_ int x, _In_ int y, _Out_opt_ LPPOINT lppt);
 WINGDIAPI BOOL  WINAPI ScaleViewportExtEx( _In_ HDC hdc, _In_ int xn, _In_ int dx, _In_ int yn, _In_ int yd, _Out_opt_ LPSIZE lpsz);
 WINGDIAPI BOOL  WINAPI ScaleWindowExtEx( _In_ HDC hdc, _In_ int xn, _In_ int xd, _In_ int yn, _In_ int yd, _Out_opt_ LPSIZE lpsz);
WINGDIAPI BOOL  WINAPI SetBitmapDimensionEx( _In_ HBITMAP hbm, _In_ int w, _In_ int h, _Out_opt_ LPSIZE lpsz);
WINGDIAPI BOOL  WINAPI SetBrushOrgEx( _In_ HDC hdc, _In_ int x, _In_ int y, _Out_opt_ LPPOINT lppt);

WINGDIAPI int   WINAPI GetTextFaceA( _In_ HDC hdc, _In_ int c, _Out_writes_to_opt_(c, return)  LPSTR lpName);
WINGDIAPI int   WINAPI GetTextFaceW( _In_ HDC hdc, _In_ int c, _Out_writes_to_opt_(c, return)  LPWSTR lpName);
#ifdef UNICODE
#define GetTextFace  GetTextFaceW
#else
#define GetTextFace  GetTextFaceA
#endif // !UNICODE

#define FONTMAPPER_MAX 10

typedef struct tagKERNINGPAIR {
   WORD wFirst;
   WORD wSecond;
   int  iKernAmount;
} KERNINGPAIR, *LPKERNINGPAIR;

WINGDIAPI DWORD WINAPI GetKerningPairsA(    _In_ HDC hdc,
                                            _In_ DWORD nPairs,
                                            _Out_writes_to_opt_(nPairs, return) LPKERNINGPAIR   lpKernPair);
WINGDIAPI DWORD WINAPI GetKerningPairsW(    _In_ HDC hdc,
                                            _In_ DWORD nPairs,
                                            _Out_writes_to_opt_(nPairs, return) LPKERNINGPAIR   lpKernPair);
#ifdef UNICODE
#define GetKerningPairs  GetKerningPairsW
#else
#define GetKerningPairs  GetKerningPairsA
#endif // !UNICODE


WINGDIAPI BOOL  WINAPI GetDCOrgEx( _In_ HDC hdc, _Out_ LPPOINT lppt);
WINGDIAPI BOOL  WINAPI FixBrushOrgEx( _In_ HDC hdc, _In_ int x, _In_ int y,  _In_opt_ LPPOINT ptl);
WINGDIAPI BOOL  WINAPI UnrealizeObject( _In_ HGDIOBJ h);

WINGDIAPI BOOL  WINAPI GdiFlush(void);
WINGDIAPI DWORD WINAPI GdiSetBatchLimit( _In_ DWORD dw);
WINGDIAPI DWORD WINAPI GdiGetBatchLimit(void);

#if(WINVER >= 0x0400)

#define ICM_OFF               1
#define ICM_ON                2
#define ICM_QUERY             3
#define ICM_DONE_OUTSIDEDC    4

typedef int (CALLBACK* ICMENUMPROCA)(LPSTR, LPARAM);
typedef int (CALLBACK* ICMENUMPROCW)(LPWSTR, LPARAM);
#ifdef UNICODE
#define ICMENUMPROC  ICMENUMPROCW
#else
#define ICMENUMPROC  ICMENUMPROCA
#endif // !UNICODE

WINGDIAPI int         WINAPI SetICMMode( _In_ HDC hdc, _In_ int mode);
WINGDIAPI BOOL        WINAPI CheckColorsInGamut(    _In_ HDC hdc,
                                                    _In_reads_(nCount) LPRGBTRIPLE lpRGBTriple,
                                                    _Out_writes_bytes_(nCount) LPVOID dlpBuffer,
                                                    _In_ DWORD nCount);

WINGDIAPI HCOLORSPACE WINAPI GetColorSpace( _In_ HDC hdc);
WINGDIAPI BOOL        WINAPI GetLogColorSpaceA( _In_ HCOLORSPACE hColorSpace,
                                                _Out_writes_bytes_(nSize) LPLOGCOLORSPACEA lpBuffer,
                                                _In_ DWORD nSize);
WINGDIAPI BOOL        WINAPI GetLogColorSpaceW( _In_ HCOLORSPACE hColorSpace,
                                                _Out_writes_bytes_(nSize) LPLOGCOLORSPACEW lpBuffer,
                                                _In_ DWORD nSize);
#ifdef UNICODE
#define GetLogColorSpace  GetLogColorSpaceW
#else
#define GetLogColorSpace  GetLogColorSpaceA
#endif // !UNICODE

WINGDIAPI HCOLORSPACE WINAPI CreateColorSpaceA( _In_ LPLOGCOLORSPACEA lplcs);
WINGDIAPI HCOLORSPACE WINAPI CreateColorSpaceW( _In_ LPLOGCOLORSPACEW lplcs);
#ifdef UNICODE
#define CreateColorSpace  CreateColorSpaceW
#else
#define CreateColorSpace  CreateColorSpaceA
#endif // !UNICODE
WINGDIAPI HCOLORSPACE WINAPI SetColorSpace( _In_ HDC hdc, _In_ HCOLORSPACE hcs);
WINGDIAPI BOOL        WINAPI DeleteColorSpace( _In_ HCOLORSPACE hcs);
WINGDIAPI BOOL        WINAPI GetICMProfileA(    _In_ HDC hdc,
                                                _Inout_ LPDWORD pBufSize,
                                                _Out_writes_opt_(*pBufSize) LPSTR pszFilename);
WINGDIAPI BOOL        WINAPI GetICMProfileW(    _In_ HDC hdc,
                                                _Inout_ LPDWORD pBufSize,
                                                _Out_writes_opt_(*pBufSize) LPWSTR pszFilename);
#ifdef UNICODE
#define GetICMProfile  GetICMProfileW
#else
#define GetICMProfile  GetICMProfileA
#endif // !UNICODE

WINGDIAPI BOOL        WINAPI SetICMProfileA( _In_ HDC hdc, _In_ LPSTR lpFileName);
WINGDIAPI BOOL        WINAPI SetICMProfileW( _In_ HDC hdc, _In_ LPWSTR lpFileName);
#ifdef UNICODE
#define SetICMProfile  SetICMProfileW
#else
#define SetICMProfile  SetICMProfileA
#endif // !UNICODE
WINGDIAPI BOOL        WINAPI GetDeviceGammaRamp( _In_ HDC hdc, _Out_writes_bytes_(3*256*2) LPVOID lpRamp);
WINGDIAPI BOOL        WINAPI SetDeviceGammaRamp( _In_ HDC hdc, _In_reads_bytes_(3*256*2)  LPVOID lpRamp);
WINGDIAPI BOOL        WINAPI ColorMatchToTarget( _In_ HDC hdc, _In_ HDC hdcTarget, _In_ DWORD action);
WINGDIAPI int         WINAPI EnumICMProfilesA( _In_ HDC hdc, _In_ ICMENUMPROCA proc, _In_opt_ LPARAM param);
WINGDIAPI int         WINAPI EnumICMProfilesW( _In_ HDC hdc, _In_ ICMENUMPROCW proc, _In_opt_ LPARAM param);
#ifdef UNICODE
#define EnumICMProfiles  EnumICMProfilesW
#else
#define EnumICMProfiles  EnumICMProfilesA
#endif // !UNICODE
// The Win95 update API UpdateICMRegKeyA is deprecated to set last error to ERROR_NOT_SUPPORTED and return FALSE
WINGDIAPI BOOL        WINAPI UpdateICMRegKeyA( _Reserved_ DWORD reserved, _In_ LPSTR lpszCMID, _In_ LPSTR lpszFileName, _In_ UINT command);
// The Win95 update API UpdateICMRegKeyW is deprecated to set last error to ERROR_NOT_SUPPORTED and return FALSE
WINGDIAPI BOOL        WINAPI UpdateICMRegKeyW( _Reserved_ DWORD reserved, _In_ LPWSTR lpszCMID, _In_ LPWSTR lpszFileName, _In_ UINT command);
#ifdef UNICODE
#define UpdateICMRegKey  UpdateICMRegKeyW
#else
#define UpdateICMRegKey  UpdateICMRegKeyA
#endif // !UNICODE

#ifndef _CONTRACT_GEN
#pragma deprecated (UpdateICMRegKeyW)
#pragma deprecated (UpdateICMRegKeyA)
#endif // _CONTRACT_GEN

#endif /* WINVER >= 0x0400 */

#if (WINVER >= 0x0500)
WINGDIAPI BOOL        WINAPI ColorCorrectPalette( _In_ HDC hdc, _In_ HPALETTE hPal, _In_ DWORD deFirst, _In_ DWORD num);
#endif

#ifndef NOMETAFILE

// Enhanced metafile constants.

#ifndef _MAC
#define ENHMETA_SIGNATURE       0x464D4520
#else
#define ENHMETA_SIGNATURE       0x20454D46
#endif

// Stock object flag used in the object handle index in the enhanced
// metafile records.
// E.g. The object handle index (META_STOCK_OBJECT | BLACK_BRUSH)
// represents the stock object BLACK_BRUSH.

#define ENHMETA_STOCK_OBJECT    0x80000000

// Enhanced metafile record types.

#define EMR_HEADER                      1
#define EMR_POLYBEZIER                  2
#define EMR_POLYGON                     3
#define EMR_POLYLINE                    4
#define EMR_POLYBEZIERTO                5
#define EMR_POLYLINETO                  6
#define EMR_POLYPOLYLINE                7
#define EMR_POLYPOLYGON                 8
#define EMR_SETWINDOWEXTEX              9
#define EMR_SETWINDOWORGEX              10
#define EMR_SETVIEWPORTEXTEX            11
#define EMR_SETVIEWPORTORGEX            12
#define EMR_SETBRUSHORGEX               13
#define EMR_EOF                         14
#define EMR_SETPIXELV                   15
#define EMR_SETMAPPERFLAGS              16
#define EMR_SETMAPMODE                  17
#define EMR_SETBKMODE                   18
#define EMR_SETPOLYFILLMODE             19
#define EMR_SETROP2                     20
#define EMR_SETSTRETCHBLTMODE           21
#define EMR_SETTEXTALIGN                22
#define EMR_SETCOLORADJUSTMENT          23
#define EMR_SETTEXTCOLOR                24
#define EMR_SETBKCOLOR                  25
#define EMR_OFFSETCLIPRGN               26
#define EMR_MOVETOEX                    27
#define EMR_SETMETARGN                  28
#define EMR_EXCLUDECLIPRECT             29
#define EMR_INTERSECTCLIPRECT           30
#define EMR_SCALEVIEWPORTEXTEX          31
#define EMR_SCALEWINDOWEXTEX            32
#define EMR_SAVEDC                      33
#define EMR_RESTOREDC                   34
#define EMR_SETWORLDTRANSFORM           35
#define EMR_MODIFYWORLDTRANSFORM        36
#define EMR_SELECTOBJECT                37
#define EMR_CREATEPEN                   38
#define EMR_CREATEBRUSHINDIRECT         39
#define EMR_DELETEOBJECT                40
#define EMR_ANGLEARC                    41
#define EMR_ELLIPSE                     42
#define EMR_RECTANGLE                   43
#define EMR_ROUNDRECT                   44
#define EMR_ARC                         45
#define EMR_CHORD                       46
#define EMR_PIE                         47
#define EMR_SELECTPALETTE               48
#define EMR_CREATEPALETTE               49
#define EMR_SETPALETTEENTRIES           50
#define EMR_RESIZEPALETTE               51
#define EMR_REALIZEPALETTE              52
#define EMR_EXTFLOODFILL                53
#define EMR_LINETO                      54
#define EMR_ARCTO                       55
#define EMR_POLYDRAW                    56
#define EMR_SETARCDIRECTION             57
#define EMR_SETMITERLIMIT               58
#define EMR_BEGINPATH                   59
#define EMR_ENDPATH                     60
#define EMR_CLOSEFIGURE                 61
#define EMR_FILLPATH                    62
#define EMR_STROKEANDFILLPATH           63
#define EMR_STROKEPATH                  64
#define EMR_FLATTENPATH                 65
#define EMR_WIDENPATH                   66
#define EMR_SELECTCLIPPATH              67
#define EMR_ABORTPATH                   68

#define EMR_GDICOMMENT                  70
#define EMR_FILLRGN                     71
#define EMR_FRAMERGN                    72
#define EMR_INVERTRGN                   73
#define EMR_PAINTRGN                    74
#define EMR_EXTSELECTCLIPRGN            75
#define EMR_BITBLT                      76
#define EMR_STRETCHBLT                  77
#define EMR_MASKBLT                     78
#define EMR_PLGBLT                      79
#define EMR_SETDIBITSTODEVICE           80
#define EMR_STRETCHDIBITS               81
#define EMR_EXTCREATEFONTINDIRECTW      82
#define EMR_EXTTEXTOUTA                 83
#define EMR_EXTTEXTOUTW                 84
#define EMR_POLYBEZIER16                85
#define EMR_POLYGON16                   86
#define EMR_POLYLINE16                  87
#define EMR_POLYBEZIERTO16              88
#define EMR_POLYLINETO16                89
#define EMR_POLYPOLYLINE16              90
#define EMR_POLYPOLYGON16               91
#define EMR_POLYDRAW16                  92
#define EMR_CREATEMONOBRUSH             93
#define EMR_CREATEDIBPATTERNBRUSHPT     94
#define EMR_EXTCREATEPEN                95
#define EMR_POLYTEXTOUTA                96
#define EMR_POLYTEXTOUTW                97

#if(WINVER >= 0x0400)
#define EMR_SETICMMODE                  98
#define EMR_CREATECOLORSPACE            99
#define EMR_SETCOLORSPACE              100
#define EMR_DELETECOLORSPACE           101
#define EMR_GLSRECORD                  102
#define EMR_GLSBOUNDEDRECORD           103
#define EMR_PIXELFORMAT                104
#endif /* WINVER >= 0x0400 */

#if(WINVER >= 0x0500)
#define EMR_RESERVED_105               105
#define EMR_RESERVED_106               106
#define EMR_RESERVED_107               107
#define EMR_RESERVED_108               108
#define EMR_RESERVED_109               109
#define EMR_RESERVED_110               110
#define EMR_COLORCORRECTPALETTE        111
#define EMR_SETICMPROFILEA             112
#define EMR_SETICMPROFILEW             113
#define EMR_ALPHABLEND                 114
#define EMR_SETLAYOUT                  115
#define EMR_TRANSPARENTBLT             116
#if (_WIN32_WINNT >= _WIN32_WINNT_WIN2K)
#define EMR_RESERVED_117               117
#endif // (_WIN32_WINNT >= _WIN32_WINNT_WIN2K)
#define EMR_GRADIENTFILL               118
#define EMR_RESERVED_119               119
#define EMR_RESERVED_120               120
#define EMR_COLORMATCHTOTARGETW        121
#define EMR_CREATECOLORSPACEW          122
#endif /* WINVER >= 0x0500 */

#define EMR_MIN                          1

#if (WINVER >= 0x0500)
#define EMR_MAX                        122
#elif (WINVER >= 0x0400)
#define EMR_MAX                        104
#else
#define EMR_MAX                         97
#endif

// Base record type for the enhanced metafile.

typedef struct tagEMR
{
    DWORD   iType;              // Enhanced metafile record type
    DWORD   nSize;              // Length of the record in bytes.
                                // This must be a multiple of 4.
} EMR, *PEMR;

// Base text record type for the enhanced metafile.

typedef struct tagEMRTEXT
{
    POINTL  ptlReference;
    DWORD   nChars;
    DWORD   offString;          // Offset to the string
    DWORD   fOptions;
    RECTL   rcl;
    DWORD   offDx;              // Offset to the inter-character spacing array.
                                // This is always given.
} EMRTEXT, *PEMRTEXT;

// Record structures for the enhanced metafile.

typedef struct tagABORTPATH
{
    EMR     emr;
} EMRABORTPATH,      *PEMRABORTPATH,
  EMRBEGINPATH,      *PEMRBEGINPATH,
  EMRENDPATH,        *PEMRENDPATH,
  EMRCLOSEFIGURE,    *PEMRCLOSEFIGURE,
  EMRFLATTENPATH,    *PEMRFLATTENPATH,
  EMRWIDENPATH,      *PEMRWIDENPATH,
  EMRSETMETARGN,     *PEMRSETMETARGN,
  EMRSAVEDC,         *PEMRSAVEDC,
  EMRREALIZEPALETTE, *PEMRREALIZEPALETTE;

typedef struct tagEMRSELECTCLIPPATH
{
    EMR     emr;
    DWORD   iMode;
} EMRSELECTCLIPPATH,    *PEMRSELECTCLIPPATH,
  EMRSETBKMODE,         *PEMRSETBKMODE,
  EMRSETMAPMODE,        *PEMRSETMAPMODE,
#if(WINVER >= 0x0500)
  EMRSETLAYOUT,         *PEMRSETLAYOUT,
#endif /* WINVER >= 0x0500 */
  EMRSETPOLYFILLMODE,   *PEMRSETPOLYFILLMODE,
  EMRSETROP2,           *PEMRSETROP2,
  EMRSETSTRETCHBLTMODE, *PEMRSETSTRETCHBLTMODE,
  EMRSETICMMODE,        *PEMRSETICMMODE,
  EMRSETTEXTALIGN,      *PEMRSETTEXTALIGN;

typedef struct tagEMRSETMITERLIMIT
{
    EMR     emr;
    FLOAT   eMiterLimit;
} EMRSETMITERLIMIT, *PEMRSETMITERLIMIT;

typedef struct tagEMRRESTOREDC
{
    EMR     emr;
    LONG    iRelative;          // Specifies a relative instance
} EMRRESTOREDC, *PEMRRESTOREDC;

typedef struct tagEMRSETARCDIRECTION
{
    EMR     emr;
    DWORD   iArcDirection;      // Specifies the arc direction in the
                                // advanced graphics mode.
} EMRSETARCDIRECTION, *PEMRSETARCDIRECTION;

typedef struct tagEMRSETMAPPERFLAGS
{
    EMR     emr;
    DWORD   dwFlags;
} EMRSETMAPPERFLAGS, *PEMRSETMAPPERFLAGS;

typedef struct tagEMRSETTEXTCOLOR
{
    EMR     emr;
    COLORREF crColor;
} EMRSETBKCOLOR,   *PEMRSETBKCOLOR,
  EMRSETTEXTCOLOR, *PEMRSETTEXTCOLOR;

typedef struct tagEMRSELECTOBJECT
{
    EMR     emr;
    DWORD   ihObject;           // Object handle index
} EMRSELECTOBJECT, *PEMRSELECTOBJECT,
  EMRDELETEOBJECT, *PEMRDELETEOBJECT;

typedef struct tagEMRSELECTPALETTE
{
    EMR     emr;
    DWORD   ihPal;              // Palette handle index, background mode only
} EMRSELECTPALETTE, *PEMRSELECTPALETTE;

typedef struct tagEMRRESIZEPALETTE
{
    EMR     emr;
    DWORD   ihPal;              // Palette handle index
    DWORD   cEntries;
} EMRRESIZEPALETTE, *PEMRRESIZEPALETTE;

typedef struct tagEMRSETPALETTEENTRIES
{
    EMR     emr;
    DWORD   ihPal;              // Palette handle index
    DWORD   iStart;
    DWORD   cEntries;
    PALETTEENTRY aPalEntries[1];// The peFlags fields do not contain any flags
} EMRSETPALETTEENTRIES, *PEMRSETPALETTEENTRIES;

typedef struct tagEMRSETCOLORADJUSTMENT
{
    EMR     emr;
    COLORADJUSTMENT ColorAdjustment;
} EMRSETCOLORADJUSTMENT, *PEMRSETCOLORADJUSTMENT;

typedef struct tagEMRGDICOMMENT
{
    EMR     emr;
    DWORD   cbData;             // Size of data in bytes
    BYTE    Data[1];
} EMRGDICOMMENT, *PEMRGDICOMMENT;

typedef struct tagEMREOF
{
    EMR     emr;
    DWORD   nPalEntries;        // Number of palette entries
    DWORD   offPalEntries;      // Offset to the palette entries
    DWORD   nSizeLast;          // Same as nSize and must be the last DWORD
                                // of the record.  The palette entries,
                                // if exist, precede this field.
} EMREOF, *PEMREOF;

typedef struct tagEMRLINETO
{
    EMR     emr;
    POINTL  ptl;
} EMRLINETO,   *PEMRLINETO,
  EMRMOVETOEX, *PEMRMOVETOEX;

typedef struct tagEMROFFSETCLIPRGN
{
    EMR     emr;
    POINTL  ptlOffset;
} EMROFFSETCLIPRGN, *PEMROFFSETCLIPRGN;

typedef struct tagEMRFILLPATH
{
    EMR     emr;
    RECTL   rclBounds;          // Inclusive-inclusive bounds in device units
} EMRFILLPATH,          *PEMRFILLPATH,
  EMRSTROKEANDFILLPATH, *PEMRSTROKEANDFILLPATH,
  EMRSTROKEPATH,        *PEMRSTROKEPATH;

typedef struct tagEMREXCLUDECLIPRECT
{
    EMR     emr;
    RECTL   rclClip;
} EMREXCLUDECLIPRECT,   *PEMREXCLUDECLIPRECT,
  EMRINTERSECTCLIPRECT, *PEMRINTERSECTCLIPRECT;

typedef struct tagEMRSETVIEWPORTORGEX
{
    EMR     emr;
    POINTL  ptlOrigin;
} EMRSETVIEWPORTORGEX, *PEMRSETVIEWPORTORGEX,
  EMRSETWINDOWORGEX,   *PEMRSETWINDOWORGEX,
  EMRSETBRUSHORGEX,    *PEMRSETBRUSHORGEX;

typedef struct tagEMRSETVIEWPORTEXTEX
{
    EMR     emr;
    SIZEL   szlExtent;
} EMRSETVIEWPORTEXTEX, *PEMRSETVIEWPORTEXTEX,
  EMRSETWINDOWEXTEX,   *PEMRSETWINDOWEXTEX;

typedef struct tagEMRSCALEVIEWPORTEXTEX
{
    EMR     emr;
    LONG    xNum;
    LONG    xDenom;
    LONG    yNum;
    LONG    yDenom;
} EMRSCALEVIEWPORTEXTEX, *PEMRSCALEVIEWPORTEXTEX,
  EMRSCALEWINDOWEXTEX,   *PEMRSCALEWINDOWEXTEX;

typedef struct tagEMRSETWORLDTRANSFORM
{
    EMR     emr;
    XFORM   xform;
} EMRSETWORLDTRANSFORM, *PEMRSETWORLDTRANSFORM;

typedef struct tagEMRMODIFYWORLDTRANSFORM
{
    EMR     emr;
    XFORM   xform;
    DWORD   iMode;
} EMRMODIFYWORLDTRANSFORM, *PEMRMODIFYWORLDTRANSFORM;

typedef struct tagEMRSETPIXELV
{
    EMR     emr;
    POINTL  ptlPixel;
    COLORREF crColor;
} EMRSETPIXELV, *PEMRSETPIXELV;

typedef struct tagEMREXTFLOODFILL
{
    EMR     emr;
    POINTL  ptlStart;
    COLORREF crColor;
    DWORD   iMode;
} EMREXTFLOODFILL, *PEMREXTFLOODFILL;

typedef struct tagEMRELLIPSE
{
    EMR     emr;
    RECTL   rclBox;             // Inclusive-inclusive bounding rectangle
} EMRELLIPSE,  *PEMRELLIPSE,
  EMRRECTANGLE, *PEMRRECTANGLE;


typedef struct tagEMRROUNDRECT
{
    EMR     emr;
    RECTL   rclBox;             // Inclusive-inclusive bounding rectangle
    SIZEL   szlCorner;
} EMRROUNDRECT, *PEMRROUNDRECT;

typedef struct tagEMRARC
{
    EMR     emr;
    RECTL   rclBox;             // Inclusive-inclusive bounding rectangle
    POINTL  ptlStart;
    POINTL  ptlEnd;
} EMRARC,   *PEMRARC,
  EMRARCTO, *PEMRARCTO,
  EMRCHORD, *PEMRCHORD,
  EMRPIE,   *PEMRPIE;

typedef struct tagEMRANGLEARC
{
    EMR     emr;
    POINTL  ptlCenter;
    DWORD   nRadius;
    FLOAT   eStartAngle;
    FLOAT   eSweepAngle;
} EMRANGLEARC, *PEMRANGLEARC;

typedef struct tagEMRPOLYLINE
{
    EMR     emr;
    RECTL   rclBounds;          // Inclusive-inclusive bounds in device units
    DWORD   cptl;
    POINTL  aptl[1];
} EMRPOLYLINE,     *PEMRPOLYLINE,
  EMRPOLYBEZIER,   *PEMRPOLYBEZIER,
  EMRPOLYGON,      *PEMRPOLYGON,
  EMRPOLYBEZIERTO, *PEMRPOLYBEZIERTO,
  EMRPOLYLINETO,   *PEMRPOLYLINETO;

typedef struct tagEMRPOLYLINE16
{
    EMR     emr;
    RECTL   rclBounds;          // Inclusive-inclusive bounds in device units
    DWORD   cpts;
    POINTS  apts[1];
} EMRPOLYLINE16,     *PEMRPOLYLINE16,
  EMRPOLYBEZIER16,   *PEMRPOLYBEZIER16,
  EMRPOLYGON16,      *PEMRPOLYGON16,
  EMRPOLYBEZIERTO16, *PEMRPOLYBEZIERTO16,
  EMRPOLYLINETO16,   *PEMRPOLYLINETO16;

typedef struct tagEMRPOLYDRAW
{
    EMR     emr;
    RECTL   rclBounds;          // Inclusive-inclusive bounds in device units
    DWORD   cptl;               // Number of points
    POINTL  aptl[1];            // Array of points
    BYTE    abTypes[1];         // Array of point types
} EMRPOLYDRAW, *PEMRPOLYDRAW;

typedef struct tagEMRPOLYDRAW16
{
    EMR     emr;
    RECTL   rclBounds;          // Inclusive-inclusive bounds in device units
    DWORD   cpts;               // Number of points
    POINTS  apts[1];            // Array of points
    BYTE    abTypes[1];         // Array of point types
} EMRPOLYDRAW16, *PEMRPOLYDRAW16;

typedef struct tagEMRPOLYPOLYLINE
{
    EMR     emr;
    RECTL   rclBounds;          // Inclusive-inclusive bounds in device units
    DWORD   nPolys;             // Number of polys
    DWORD   cptl;               // Total number of points in all polys
    DWORD   aPolyCounts[1];     // Array of point counts for each poly
    POINTL  aptl[1];            // Array of points
} EMRPOLYPOLYLINE, *PEMRPOLYPOLYLINE,
  EMRPOLYPOLYGON,  *PEMRPOLYPOLYGON;

typedef struct tagEMRPOLYPOLYLINE16
{
    EMR     emr;
    RECTL   rclBounds;          // Inclusive-inclusive bounds in device units
    DWORD   nPolys;             // Number of polys
    DWORD   cpts;               // Total number of points in all polys
    DWORD   aPolyCounts[1];     // Array of point counts for each poly
    POINTS  apts[1];            // Array of points
} EMRPOLYPOLYLINE16, *PEMRPOLYPOLYLINE16,
  EMRPOLYPOLYGON16,  *PEMRPOLYPOLYGON16;

typedef struct tagEMRINVERTRGN
{
    EMR     emr;
    RECTL   rclBounds;          // Inclusive-inclusive bounds in device units
    DWORD   cbRgnData;          // Size of region data in bytes
    BYTE    RgnData[1];
} EMRINVERTRGN, *PEMRINVERTRGN,
  EMRPAINTRGN,  *PEMRPAINTRGN;

typedef struct tagEMRFILLRGN
{
    EMR     emr;
    RECTL   rclBounds;          // Inclusive-inclusive bounds in device units
    DWORD   cbRgnData;          // Size of region data in bytes
    DWORD   ihBrush;            // Brush handle index
    BYTE    RgnData[1];
} EMRFILLRGN, *PEMRFILLRGN;

typedef struct tagEMRFRAMERGN
{
    EMR     emr;
    RECTL   rclBounds;          // Inclusive-inclusive bounds in device units
    DWORD   cbRgnData;          // Size of region data in bytes
    DWORD   ihBrush;            // Brush handle index
    SIZEL   szlStroke;
    BYTE    RgnData[1];
} EMRFRAMERGN, *PEMRFRAMERGN;

typedef struct tagEMREXTSELECTCLIPRGN
{
    EMR     emr;
    DWORD   cbRgnData;          // Size of region data in bytes
    DWORD   iMode;
    BYTE    RgnData[1];
} EMREXTSELECTCLIPRGN, *PEMREXTSELECTCLIPRGN;

typedef struct tagEMREXTTEXTOUTA
{
    EMR     emr;
    RECTL   rclBounds;          // Inclusive-inclusive bounds in device units
    DWORD   iGraphicsMode;      // Current graphics mode
    FLOAT   exScale;            // X and Y scales from Page units to .01mm units
    FLOAT   eyScale;            //   if graphics mode is GM_COMPATIBLE.
    EMRTEXT emrtext;            // This is followed by the string and spacing
                                // array
} EMREXTTEXTOUTA, *PEMREXTTEXTOUTA,
  EMREXTTEXTOUTW, *PEMREXTTEXTOUTW;

typedef struct tagEMRPOLYTEXTOUTA
{
    EMR     emr;
    RECTL   rclBounds;          // Inclusive-inclusive bounds in device units
    DWORD   iGraphicsMode;      // Current graphics mode
    FLOAT   exScale;            // X and Y scales from Page units to .01mm units
    FLOAT   eyScale;            //   if graphics mode is GM_COMPATIBLE.
    LONG    cStrings;
    EMRTEXT aemrtext[1];        // Array of EMRTEXT structures.  This is
                                // followed by the strings and spacing arrays.
} EMRPOLYTEXTOUTA, *PEMRPOLYTEXTOUTA,
  EMRPOLYTEXTOUTW, *PEMRPOLYTEXTOUTW;

typedef struct tagEMRBITBLT
{
    EMR     emr;
    RECTL   rclBounds;          // Inclusive-inclusive bounds in device units
    LONG    xDest;
    LONG    yDest;
    LONG    cxDest;
    LONG    cyDest;
    DWORD   dwRop;
    LONG    xSrc;
    LONG    ySrc;
    XFORM   xformSrc;           // Source DC transform
    COLORREF crBkColorSrc;      // Source DC BkColor in RGB
    DWORD   iUsageSrc;          // Source bitmap info color table usage
                                // (DIB_RGB_COLORS)
    DWORD   offBmiSrc;          // Offset to the source BITMAPINFO structure
    DWORD   cbBmiSrc;           // Size of the source BITMAPINFO structure
    DWORD   offBitsSrc;         // Offset to the source bitmap bits
    DWORD   cbBitsSrc;          // Size of the source bitmap bits
} EMRBITBLT, *PEMRBITBLT;

typedef struct tagEMRSTRETCHBLT
{
    EMR     emr;
    RECTL   rclBounds;          // Inclusive-inclusive bounds in device units
    LONG    xDest;
    LONG    yDest;
    LONG    cxDest;
    LONG    cyDest;
    DWORD   dwRop;
    LONG    xSrc;
    LONG    ySrc;
    XFORM   xformSrc;           // Source DC transform
    COLORREF crBkColorSrc;      // Source DC BkColor in RGB
    DWORD   iUsageSrc;          // Source bitmap info color table usage
                                // (DIB_RGB_COLORS)
    DWORD   offBmiSrc;          // Offset to the source BITMAPINFO structure
    DWORD   cbBmiSrc;           // Size of the source BITMAPINFO structure
    DWORD   offBitsSrc;         // Offset to the source bitmap bits
    DWORD   cbBitsSrc;          // Size of the source bitmap bits
    LONG    cxSrc;
    LONG    cySrc;
} EMRSTRETCHBLT, *PEMRSTRETCHBLT;

typedef struct tagEMRMASKBLT
{
    EMR     emr;
    RECTL   rclBounds;          // Inclusive-inclusive bounds in device units
    LONG    xDest;
    LONG    yDest;
    LONG    cxDest;
    LONG    cyDest;
    DWORD   dwRop;
    LONG    xSrc;
    LONG    ySrc;
    XFORM   xformSrc;           // Source DC transform
    COLORREF crBkColorSrc;      // Source DC BkColor in RGB
    DWORD   iUsageSrc;          // Source bitmap info color table usage
                                // (DIB_RGB_COLORS)
    DWORD   offBmiSrc;          // Offset to the source BITMAPINFO structure
    DWORD   cbBmiSrc;           // Size of the source BITMAPINFO structure
    DWORD   offBitsSrc;         // Offset to the source bitmap bits
    DWORD   cbBitsSrc;          // Size of the source bitmap bits
    LONG    xMask;
    LONG    yMask;
    DWORD   iUsageMask;         // Mask bitmap info color table usage
    DWORD   offBmiMask;         // Offset to the mask BITMAPINFO structure if any
    DWORD   cbBmiMask;          // Size of the mask BITMAPINFO structure if any
    DWORD   offBitsMask;        // Offset to the mask bitmap bits if any
    DWORD   cbBitsMask;         // Size of the mask bitmap bits if any
} EMRMASKBLT, *PEMRMASKBLT;

typedef struct tagEMRPLGBLT
{
    EMR     emr;
    RECTL   rclBounds;          // Inclusive-inclusive bounds in device units
    POINTL  aptlDest[3];
    LONG    xSrc;
    LONG    ySrc;
    LONG    cxSrc;
    LONG    cySrc;
    XFORM   xformSrc;           // Source DC transform
    COLORREF crBkColorSrc;      // Source DC BkColor in RGB
    DWORD   iUsageSrc;          // Source bitmap info color table usage
                                // (DIB_RGB_COLORS)
    DWORD   offBmiSrc;          // Offset to the source BITMAPINFO structure
    DWORD   cbBmiSrc;           // Size of the source BITMAPINFO structure
    DWORD   offBitsSrc;         // Offset to the source bitmap bits
    DWORD   cbBitsSrc;          // Size of the source bitmap bits
    LONG    xMask;
    LONG    yMask;
    DWORD   iUsageMask;         // Mask bitmap info color table usage
    DWORD   offBmiMask;         // Offset to the mask BITMAPINFO structure if any
    DWORD   cbBmiMask;          // Size of the mask BITMAPINFO structure if any
    DWORD   offBitsMask;        // Offset to the mask bitmap bits if any
    DWORD   cbBitsMask;         // Size of the mask bitmap bits if any
} EMRPLGBLT, *PEMRPLGBLT;

typedef struct tagEMRSETDIBITSTODEVICE
{
    EMR     emr;
    RECTL   rclBounds;          // Inclusive-inclusive bounds in device units
    LONG    xDest;
    LONG    yDest;
    LONG    xSrc;
    LONG    ySrc;
    LONG    cxSrc;
    LONG    cySrc;
    DWORD   offBmiSrc;          // Offset to the source BITMAPINFO structure
    DWORD   cbBmiSrc;           // Size of the source BITMAPINFO structure
    DWORD   offBitsSrc;         // Offset to the source bitmap bits
    DWORD   cbBitsSrc;          // Size of the source bitmap bits
    DWORD   iUsageSrc;          // Source bitmap info color table usage
    DWORD   iStartScan;
    DWORD   cScans;
} EMRSETDIBITSTODEVICE, *PEMRSETDIBITSTODEVICE;

typedef struct tagEMRSTRETCHDIBITS
{
    EMR     emr;
    RECTL   rclBounds;          // Inclusive-inclusive bounds in device units
    LONG    xDest;
    LONG    yDest;
    LONG    xSrc;
    LONG    ySrc;
    LONG    cxSrc;
    LONG    cySrc;
    DWORD   offBmiSrc;          // Offset to the source BITMAPINFO structure
    DWORD   cbBmiSrc;           // Size of the source BITMAPINFO structure
    DWORD   offBitsSrc;         // Offset to the source bitmap bits
    DWORD   cbBitsSrc;          // Size of the source bitmap bits
    DWORD   iUsageSrc;          // Source bitmap info color table usage
    DWORD   dwRop;
    LONG    cxDest;
    LONG    cyDest;
} EMRSTRETCHDIBITS, *PEMRSTRETCHDIBITS;

typedef struct tagEMREXTCREATEFONTINDIRECTW
{
    EMR     emr;
    DWORD   ihFont;             // Font handle index
    EXTLOGFONTW elfw;
} EMREXTCREATEFONTINDIRECTW, *PEMREXTCREATEFONTINDIRECTW;

typedef struct tagEMRCREATEPALETTE
{
    EMR     emr;
    DWORD   ihPal;              // Palette handle index
    LOGPALETTE lgpl;            // The peFlags fields in the palette entries
                                // do not contain any flags
} EMRCREATEPALETTE, *PEMRCREATEPALETTE;

typedef struct tagEMRCREATEPEN
{
    EMR     emr;
    DWORD   ihPen;              // Pen handle index
    LOGPEN  lopn;
} EMRCREATEPEN, *PEMRCREATEPEN;

typedef struct tagEMREXTCREATEPEN
{
    EMR     emr;
    DWORD   ihPen;              // Pen handle index
    DWORD   offBmi;             // Offset to the BITMAPINFO structure if any
    DWORD   cbBmi;              // Size of the BITMAPINFO structure if any
                                // The bitmap info is followed by the bitmap
                                // bits to form a packed DIB.
    DWORD   offBits;            // Offset to the brush bitmap bits if any
    DWORD   cbBits;             // Size of the brush bitmap bits if any
    EXTLOGPEN32 elp;            // The extended pen with the style array.
} EMREXTCREATEPEN, *PEMREXTCREATEPEN;

typedef struct tagEMRCREATEBRUSHINDIRECT
{
    EMR        emr;
    DWORD      ihBrush;          // Brush handle index
    LOGBRUSH32 lb;               // The style must be BS_SOLID, BS_HOLLOW,
                                 // BS_NULL or BS_HATCHED.
} EMRCREATEBRUSHINDIRECT, *PEMRCREATEBRUSHINDIRECT;

typedef struct tagEMRCREATEMONOBRUSH
{
    EMR     emr;
    DWORD   ihBrush;            // Brush handle index
    DWORD   iUsage;             // Bitmap info color table usage
    DWORD   offBmi;             // Offset to the BITMAPINFO structure
    DWORD   cbBmi;              // Size of the BITMAPINFO structure
    DWORD   offBits;            // Offset to the bitmap bits
    DWORD   cbBits;             // Size of the bitmap bits
} EMRCREATEMONOBRUSH, *PEMRCREATEMONOBRUSH;

typedef struct tagEMRCREATEDIBPATTERNBRUSHPT
{
    EMR     emr;
    DWORD   ihBrush;            // Brush handle index
    DWORD   iUsage;             // Bitmap info color table usage
    DWORD   offBmi;             // Offset to the BITMAPINFO structure
    DWORD   cbBmi;              // Size of the BITMAPINFO structure
                                // The bitmap info is followed by the bitmap
                                // bits to form a packed DIB.
    DWORD   offBits;            // Offset to the bitmap bits
    DWORD   cbBits;             // Size of the bitmap bits
} EMRCREATEDIBPATTERNBRUSHPT, *PEMRCREATEDIBPATTERNBRUSHPT;

typedef struct tagEMRFORMAT
{
    DWORD   dSignature;         // Format signature, e.g. ENHMETA_SIGNATURE.
    DWORD   nVersion;           // Format version number.
    DWORD   cbData;             // Size of data in bytes.
    DWORD   offData;            // Offset to data from GDICOMMENT_IDENTIFIER.
                                // It must begin at a DWORD offset.
} EMRFORMAT, *PEMRFORMAT;

#if(WINVER >= 0x0400)

typedef struct tagEMRGLSRECORD
{
    EMR     emr;
    DWORD   cbData;             // Size of data in bytes
    BYTE    Data[1];
} EMRGLSRECORD, *PEMRGLSRECORD;

typedef struct tagEMRGLSBOUNDEDRECORD
{
    EMR     emr;
    RECTL   rclBounds;          // Bounds in recording coordinates
    DWORD   cbData;             // Size of data in bytes
    BYTE    Data[1];
} EMRGLSBOUNDEDRECORD, *PEMRGLSBOUNDEDRECORD;

typedef struct tagEMRPIXELFORMAT
{
    EMR     emr;
    PIXELFORMATDESCRIPTOR pfd;
} EMRPIXELFORMAT, *PEMRPIXELFORMAT;

typedef struct tagEMRCREATECOLORSPACE
{
    EMR             emr;
    DWORD           ihCS;       // ColorSpace handle index
    LOGCOLORSPACEA  lcs;        // Ansi version of LOGCOLORSPACE
} EMRCREATECOLORSPACE, *PEMRCREATECOLORSPACE;

typedef struct tagEMRSETCOLORSPACE
{
    EMR     emr;
    DWORD   ihCS;               // ColorSpace handle index
} EMRSETCOLORSPACE,    *PEMRSETCOLORSPACE,
  EMRSELECTCOLORSPACE, *PEMRSELECTCOLORSPACE,
  EMRDELETECOLORSPACE, *PEMRDELETECOLORSPACE;

#endif /* WINVER >= 0x0400 */

#if(WINVER >= 0x0500)

typedef struct tagEMREXTESCAPE
{
    EMR     emr;
    INT     iEscape;            // Escape code
    INT     cbEscData;          // Size of escape data
    BYTE    EscData[1];         // Escape data
} EMREXTESCAPE,  *PEMREXTESCAPE,
  EMRDRAWESCAPE, *PEMRDRAWESCAPE;

typedef struct tagEMRNAMEDESCAPE
{
    EMR     emr;
    INT     iEscape;            // Escape code
    INT     cbDriver;           // Size of driver name
    INT     cbEscData;          // Size of escape data
    BYTE    EscData[1];         // Driver name and Escape data
} EMRNAMEDESCAPE, *PEMRNAMEDESCAPE;

#define SETICMPROFILE_EMBEDED           0x00000001

typedef struct tagEMRSETICMPROFILE
{
    EMR     emr;
    DWORD   dwFlags;            // flags
    DWORD   cbName;             // Size of desired profile name
    DWORD   cbData;             // Size of raw profile data if attached
    BYTE    Data[1];            // Array size is cbName + cbData
} EMRSETICMPROFILE,  *PEMRSETICMPROFILE,
  EMRSETICMPROFILEA, *PEMRSETICMPROFILEA,
  EMRSETICMPROFILEW, *PEMRSETICMPROFILEW;

#define CREATECOLORSPACE_EMBEDED        0x00000001

typedef struct tagEMRCREATECOLORSPACEW
{
    EMR             emr;
    DWORD           ihCS;       // ColorSpace handle index
    LOGCOLORSPACEW  lcs;        // Unicode version of logical color space structure
    DWORD           dwFlags;    // flags
    DWORD           cbData;     // size of raw source profile data if attached
    BYTE            Data[1];    // Array size is cbData
} EMRCREATECOLORSPACEW, *PEMRCREATECOLORSPACEW;

#define COLORMATCHTOTARGET_EMBEDED      0x00000001

typedef struct tagCOLORMATCHTOTARGET
{
    EMR     emr;
    DWORD   dwAction;           // CS_ENABLE, CS_DISABLE or CS_DELETE_TRANSFORM
    DWORD   dwFlags;            // flags
    DWORD   cbName;             // Size of desired target profile name
    DWORD   cbData;             // Size of raw target profile data if attached
    BYTE    Data[1];            // Array size is cbName + cbData
} EMRCOLORMATCHTOTARGET, *PEMRCOLORMATCHTOTARGET;

typedef struct tagCOLORCORRECTPALETTE
{
    EMR     emr;
    DWORD   ihPalette;          // Palette handle index
    DWORD   nFirstEntry;        // Index of first entry to correct
    DWORD   nPalEntries;        // Number of palette entries to correct
    DWORD   nReserved;          // Reserved
} EMRCOLORCORRECTPALETTE, *PEMRCOLORCORRECTPALETTE;

typedef struct tagEMRALPHABLEND
{
    EMR     emr;
    RECTL   rclBounds;          // Inclusive-inclusive bounds in device units
    LONG    xDest;
    LONG    yDest;
    LONG    cxDest;
    LONG    cyDest;
    DWORD   dwRop;
    LONG    xSrc;
    LONG    ySrc;
    XFORM   xformSrc;           // Source DC transform
    COLORREF crBkColorSrc;      // Source DC BkColor in RGB
    DWORD   iUsageSrc;          // Source bitmap info color table usage
                                // (DIB_RGB_COLORS)
    DWORD   offBmiSrc;          // Offset to the source BITMAPINFO structure
    DWORD   cbBmiSrc;           // Size of the source BITMAPINFO structure
    DWORD   offBitsSrc;         // Offset to the source bitmap bits
    DWORD   cbBitsSrc;          // Size of the source bitmap bits
    LONG    cxSrc;
    LONG    cySrc;
} EMRALPHABLEND, *PEMRALPHABLEND;

typedef struct tagEMRGRADIENTFILL
{
    EMR       emr;
    RECTL     rclBounds;          // Inclusive-inclusive bounds in device units
    DWORD     nVer;
    DWORD     nTri;
    ULONG     ulMode;
    TRIVERTEX Ver[1];
}EMRGRADIENTFILL,*PEMRGRADIENTFILL;

typedef struct tagEMRTRANSPARENTBLT
{
    EMR     emr;
    RECTL   rclBounds;          // Inclusive-inclusive bounds in device units
    LONG    xDest;
    LONG    yDest;
    LONG    cxDest;
    LONG    cyDest;
    DWORD   dwRop;
    LONG    xSrc;
    LONG    ySrc;
    XFORM   xformSrc;           // Source DC transform
    COLORREF crBkColorSrc;      // Source DC BkColor in RGB
    DWORD   iUsageSrc;          // Source bitmap info color table usage
                                // (DIB_RGB_COLORS)
    DWORD   offBmiSrc;          // Offset to the source BITMAPINFO structure
    DWORD   cbBmiSrc;           // Size of the source BITMAPINFO structure
    DWORD   offBitsSrc;         // Offset to the source bitmap bits
    DWORD   cbBitsSrc;          // Size of the source bitmap bits
    LONG    cxSrc;
    LONG    cySrc;
} EMRTRANSPARENTBLT, *PEMRTRANSPARENTBLT;


#endif /* WINVER >= 0x0500 */

#define GDICOMMENT_IDENTIFIER           0x43494447
#define GDICOMMENT_WINDOWS_METAFILE     0x80000001
#define GDICOMMENT_BEGINGROUP           0x00000002
#define GDICOMMENT_ENDGROUP             0x00000003
#define GDICOMMENT_MULTIFORMATS         0x40000004
#define EPS_SIGNATURE                   0x46535045
#define GDICOMMENT_UNICODE_STRING       0x00000040
#define GDICOMMENT_UNICODE_END          0x00000080

#endif  /* NOMETAFILE */


// OpenGL wgl prototypes

WINGDIAPI BOOL  WINAPI wglCopyContext(HGLRC, HGLRC, UINT);
WINGDIAPI HGLRC WINAPI wglCreateContext(HDC);
WINGDIAPI HGLRC WINAPI wglCreateLayerContext(HDC, int);
WINGDIAPI BOOL  WINAPI wglDeleteContext(HGLRC);
WINGDIAPI HGLRC WINAPI wglGetCurrentContext(VOID);
WINGDIAPI HDC   WINAPI wglGetCurrentDC(VOID);
WINGDIAPI PROC  WINAPI wglGetProcAddress(LPCSTR);
WINGDIAPI BOOL  WINAPI wglMakeCurrent(HDC, HGLRC);
WINGDIAPI BOOL  WINAPI wglShareLists(HGLRC, HGLRC);
WINGDIAPI BOOL  WINAPI wglUseFontBitmapsA(HDC, DWORD, DWORD, DWORD);
WINGDIAPI BOOL  WINAPI wglUseFontBitmapsW(HDC, DWORD, DWORD, DWORD);
#ifdef UNICODE
#define wglUseFontBitmaps  wglUseFontBitmapsW
#else
#define wglUseFontBitmaps  wglUseFontBitmapsA
#endif // !UNICODE
WINGDIAPI BOOL  WINAPI SwapBuffers(HDC);

typedef struct _POINTFLOAT {
    FLOAT   x;
    FLOAT   y;
} POINTFLOAT, *PPOINTFLOAT;

typedef struct _GLYPHMETRICSFLOAT {
    FLOAT       gmfBlackBoxX;
    FLOAT       gmfBlackBoxY;
    POINTFLOAT  gmfptGlyphOrigin;
    FLOAT       gmfCellIncX;
    FLOAT       gmfCellIncY;
} GLYPHMETRICSFLOAT, *PGLYPHMETRICSFLOAT, FAR *LPGLYPHMETRICSFLOAT;

#define WGL_FONT_LINES      0
#define WGL_FONT_POLYGONS   1
WINGDIAPI BOOL  WINAPI wglUseFontOutlinesA(HDC, DWORD, DWORD, DWORD, FLOAT,
                                           FLOAT, int, LPGLYPHMETRICSFLOAT);
WINGDIAPI BOOL  WINAPI wglUseFontOutlinesW(HDC, DWORD, DWORD, DWORD, FLOAT,
                                           FLOAT, int, LPGLYPHMETRICSFLOAT);
#ifdef UNICODE
#define wglUseFontOutlines  wglUseFontOutlinesW
#else
#define wglUseFontOutlines  wglUseFontOutlinesA
#endif // !UNICODE

/* Layer plane descriptor */
typedef struct tagLAYERPLANEDESCRIPTOR { // lpd
    WORD  nSize;
    WORD  nVersion;
    DWORD dwFlags;
    BYTE  iPixelType;
    BYTE  cColorBits;
    BYTE  cRedBits;
    BYTE  cRedShift;
    BYTE  cGreenBits;
    BYTE  cGreenShift;
    BYTE  cBlueBits;
    BYTE  cBlueShift;
    BYTE  cAlphaBits;
    BYTE  cAlphaShift;
    BYTE  cAccumBits;
    BYTE  cAccumRedBits;
    BYTE  cAccumGreenBits;
    BYTE  cAccumBlueBits;
    BYTE  cAccumAlphaBits;
    BYTE  cDepthBits;
    BYTE  cStencilBits;
    BYTE  cAuxBuffers;
    BYTE  iLayerPlane;
    BYTE  bReserved;
    COLORREF crTransparent;
} LAYERPLANEDESCRIPTOR, *PLAYERPLANEDESCRIPTOR, FAR *LPLAYERPLANEDESCRIPTOR;

/* LAYERPLANEDESCRIPTOR flags */
#define LPD_DOUBLEBUFFER        0x00000001
#define LPD_STEREO              0x00000002
#define LPD_SUPPORT_GDI         0x00000010
#define LPD_SUPPORT_OPENGL      0x00000020
#define LPD_SHARE_DEPTH         0x00000040
#define LPD_SHARE_STENCIL       0x00000080
#define LPD_SHARE_ACCUM         0x00000100
#define LPD_SWAP_EXCHANGE       0x00000200
#define LPD_SWAP_COPY           0x00000400
#define LPD_TRANSPARENT         0x00001000

#define LPD_TYPE_RGBA        0
#define LPD_TYPE_COLORINDEX  1

/* wglSwapLayerBuffers flags */
#define WGL_SWAP_MAIN_PLANE     0x00000001
#define WGL_SWAP_OVERLAY1       0x00000002
#define WGL_SWAP_OVERLAY2       0x00000004
#define WGL_SWAP_OVERLAY3       0x00000008
#define WGL_SWAP_OVERLAY4       0x00000010
#define WGL_SWAP_OVERLAY5       0x00000020
#define WGL_SWAP_OVERLAY6       0x00000040
#define WGL_SWAP_OVERLAY7       0x00000080
#define WGL_SWAP_OVERLAY8       0x00000100
#define WGL_SWAP_OVERLAY9       0x00000200
#define WGL_SWAP_OVERLAY10      0x00000400
#define WGL_SWAP_OVERLAY11      0x00000800
#define WGL_SWAP_OVERLAY12      0x00001000
#define WGL_SWAP_OVERLAY13      0x00002000
#define WGL_SWAP_OVERLAY14      0x00004000
#define WGL_SWAP_OVERLAY15      0x00008000
#define WGL_SWAP_UNDERLAY1      0x00010000
#define WGL_SWAP_UNDERLAY2      0x00020000
#define WGL_SWAP_UNDERLAY3      0x00040000
#define WGL_SWAP_UNDERLAY4      0x00080000
#define WGL_SWAP_UNDERLAY5      0x00100000
#define WGL_SWAP_UNDERLAY6      0x00200000
#define WGL_SWAP_UNDERLAY7      0x00400000
#define WGL_SWAP_UNDERLAY8      0x00800000
#define WGL_SWAP_UNDERLAY9      0x01000000
#define WGL_SWAP_UNDERLAY10     0x02000000
#define WGL_SWAP_UNDERLAY11     0x04000000
#define WGL_SWAP_UNDERLAY12     0x08000000
#define WGL_SWAP_UNDERLAY13     0x10000000
#define WGL_SWAP_UNDERLAY14     0x20000000
#define WGL_SWAP_UNDERLAY15     0x40000000

WINGDIAPI BOOL  WINAPI wglDescribeLayerPlane(HDC, int, int, UINT,
                                             LPLAYERPLANEDESCRIPTOR);
WINGDIAPI int   WINAPI wglSetLayerPaletteEntries(HDC, int, int, int,
                                                 CONST COLORREF *);
WINGDIAPI int   WINAPI wglGetLayerPaletteEntries(HDC, int, int, int,
                                                 COLORREF *);
WINGDIAPI BOOL  WINAPI wglRealizeLayerPalette(HDC, int, BOOL);
WINGDIAPI BOOL  WINAPI wglSwapLayerBuffers(HDC, UINT);

#if (WINVER >= 0x0500)

typedef struct _WGLSWAP
{
    HDC hdc;
    UINT uiFlags;
} WGLSWAP, *PWGLSWAP, FAR *LPWGLSWAP;

#define WGL_SWAPMULTIPLE_MAX 16

WINGDIAPI DWORD WINAPI wglSwapMultipleBuffers(UINT, CONST WGLSWAP *);

#endif // (WINVER >= 0x0500)

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif /* NOGDI */

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#ifdef __cplusplus
}
#endif

#if _MSC_VER >= 1200
#pragma warning(pop)
#endif

#endif /* _WINGDI_ */

