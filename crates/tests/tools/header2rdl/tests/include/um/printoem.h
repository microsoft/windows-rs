/*++

Copyright (c) 1996-2004 Microsoft Corporation

Module Name:

    printoem.h

Abstract:

    Declarations for Windows NT printer driver OEM plugins

NOTE:

    This file contains hsplit tags that publish structures and constants to public\internal headers. So
    please take care before making any changes to this file. Please build usermode\test\PublishingTests
    before checking in any changes to this file.

--*/


#ifndef _PRINTOEM_
#define _PRINTOEM_
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


#ifdef __cplusplus
extern "C" {
#endif

// @@BEGIN_INTERNAL_PUBLISHCOMMON

//
// PScript5's private DEVMODE structure to allow its plugins to determine its size.
//
typedef struct _PSCRIPT5_PRIVATE_DEVMODE {

    WORD wReserved[57];
    WORD wSize;          // size of PScript5's private DEVMODE

} PSCRIPT5_PRIVATE_DEVMODE, *PPSCRIPT5_PRIVATE_DEVMODE;

#define GET_PSCRIPT5_PRIVATE_DEVMODE_SIZE(pdm) \
        ( ( (pdm)->dmDriverExtra > (FIELD_OFFSET(PSCRIPT5_PRIVATE_DEVMODE, wSize) + sizeof(WORD)) ) ? \
          ((PPSCRIPT5_PRIVATE_DEVMODE)((PBYTE)(pdm) + (pdm)->dmSize))->wSize : 0 )

//
// Unidrv's private DEVMODE structure to allow its plugins to determine its size.
//
typedef struct _UNIDRV_PRIVATE_DEVMODE {

    WORD wReserved[4];
    WORD wSize;           // size of Unidrv's private DEVMODE

} UNIDRV_PRIVATE_DEVMODE, *PUNIDRV_PRIVATE_DEVMODE;

#define GET_UNIDRV_PRIVATE_DEVMODE_SIZE(pdm) \
        ( ( (pdm)->dmDriverExtra > (FIELD_OFFSET(UNIDRV_PRIVATE_DEVMODE, wSize) + sizeof(WORD)) ) ? \
          ((PUNIDRV_PRIVATE_DEVMODE)((PBYTE)(pdm) + (pdm)->dmSize))->wSize : 0 )

//
// Current OEM plugin interface version number
//

#define PRINTER_OEMINTF_VERSION 0x00010000

#define OEM_MODE_PUBLISHER      0x00000001

typedef struct _PUBLISHERINFO {

    DWORD dwMode;           // flags for publisher
    WORD  wMinoutlinePPEM;  // min size to download as Type1
    WORD  wMaxbitmapPPEM;   // max size to download as Type3

} PUBLISHERINFO, *PPUBLISHERINFO;

#define OEMGI_GETSIGNATURE            1
#define OEMGI_GETINTERFACEVERSION     2
#define OEMGI_GETVERSION              3
#define OEMGI_GETPUBLISHERINFO        4
#if (NTDDI_VERSION >= NTDDI_VISTA)
#define OEMGI_GETREQUESTEDHELPERINTERFACES  5
#endif // (NTDDI_VERSION >= NTDDI_VISTA)

//
// OEMGI_GETREQUESTEDINTERFACES
//
#if (NTDDI_VERSION >= NTDDI_VISTA)
#define OEMPUBLISH_DEFAULT           (0)
#define OEMPUBLISH_IPRINTCOREHELPER  (1<<0)
#endif // (NTDDI_VERSION >= NTDDI_VISTA)

//
// OEMGetInfo
//
__drv_preferredFunction("IPrintOemUI", "Obsolete")
BOOL APIENTRY
OEMGetInfo(
    DWORD                       dwMode,
    _Out_writes_bytes_(cbSize) PVOID  pBuffer,
    DWORD                       cbSize,
    _Out_ PDWORD                pcbNeeded
    );

__drv_preferredFunction("IPrintOemUni", "Obsolete")
BOOL APIENTRY
OEMDriverDMS(
    PVOID                       pDevObj,
    _Out_writes_bytes_(cbSize) PVOID  pBuffer,
    DWORD                       cbSize,
    _Out_ PDWORD                pcbNeeded
    );

//
// OEMDevMode
//

#define OEMDM_SIZE     1
#define OEMDM_DEFAULT  2
#define OEMDM_CONVERT  3
#define OEMDM_MERGE    4

typedef struct _OEMDMPARAM {

    DWORD    cbSize;        // size of OEM_DEVMODEPARAM
    PVOID    pdriverobj;    // reference to driver data structure
    HANDLE   hPrinter;      // PRINTER handle
    HANDLE   hModule;       // OEM module handle
    PDEVMODE pPublicDMIn;   // public devmode in
    PDEVMODE pPublicDMOut;  // public devmode out
    PVOID    pOEMDMIn;      // OEM private devmode in
    PVOID    pOEMDMOut;     // OEM private devmode out
    DWORD    cbBufSize;     // output size of pOEMDMOut buffer

} OEMDMPARAM, *POEMDMPARAM;

typedef struct _OEM_DMEXTRAHEADER {

    DWORD   dwSize;         // size of OEM extra data
    DWORD   dwSignature;    // Unique OEM signature
    DWORD   dwVersion;      // OEM DLL version number

} OEM_DMEXTRAHEADER, *POEM_DMEXTRAHEADER;

//
// USERDATA for OPTITEM.UserData
//

typedef struct _USERDATA {

    DWORD       dwSize;                 // Size of this structure
    ULONG_PTR   dwItemID;               // XXX_ITEM or pointer to FEATURE
    PSTR        pKeyWordName;           // Keyword name
    DWORD       dwReserved[8];
} USERDATA, *PUSERDATA;


__drv_preferredFunction("IPrintOemUI", "Obsolete")
BOOL APIENTRY
OEMDevMode(
    DWORD       dwMode,
    POEMDMPARAM pOemDMParam
    );

//
// Callback function provided by the driver to
// allow OEM plugins access/set/update to driver private settings
//

typedef BOOL (APIENTRY *PFN_DrvGetDriverSetting)(
        PVOID                           pdriverobj,
        PCSTR                           Feature,
        _Out_writes_bytes_opt_(cbSize) PVOID  pOutput,
        DWORD                           cbSize,
        _Out_opt_ PDWORD                pcbNeeded,
        _Out_opt_ PDWORD                pdwOptionsReturned
        );

//
// Callback function provided by the driver to
// allow OEM plugins upgrade private registry settings.
//

typedef BOOL (APIENTRY *PFN_DrvUpgradeRegistrySetting)(
        HANDLE   hPrinter,
        PCSTR    pFeature,
        PCSTR    pOption
        );

//
// Callback function provided by the driver UI to
// allow OEM plugins to update the driver UI settings and
// shows constraint. This function is called only when the UI is present.
//

typedef BOOL (APIENTRY *PFN_DrvUpdateUISetting)(
        PVOID    pdriverobj,
        PVOID    pOptItem,
        DWORD    dwPreviousSelection,
        DWORD    dwMode
        );



// Predefined feature indices used for accessing driver private settings

#define OEMGDS_MIN_DOCSTICKY        1
#define OEMGDS_PSDM_FLAGS           1       // DWORD - misc. flag bits
#define OEMGDS_PSDM_DIALECT         2       // INT - PS output option
#define OEMGDS_PSDM_TTDLFMT         3       // INT - TrueType font downloading option
#define OEMGDS_PSDM_NUP             4       // INT - N-up option
#define OEMGDS_PSDM_PSLEVEL         5       // INT - target language level
#define OEMGDS_PSDM_CUSTOMSIZE      6       // 5*DWORD - custom page size parameters

#define OEMGDS_UNIDM_GPDVER         0x4000  // WORD - GPD Version
#define OEMGDS_UNIDM_FLAGS          0x4001  // DWORD - misc flag bits

// Indices for private devmode fields - start at 0x4000

#define OEMGDS_MIN_PRINTERSTICKY    0x8000
#define OEMGDS_PRINTFLAGS           0x8000  // DWORD - misc. flag bits
#define OEMGDS_FREEMEM              0x8001  // DWORD - amount of VM, ps only
#define OEMGDS_JOBTIMEOUT           0x8002  // DWORD - job timeout, ps only
#define OEMGDS_WAITTIMEOUT          0x8003  // DWORD - wait timeout, ps only
#define OEMGDS_PROTOCOL             0x8004  // WORD - output protocol, ps only
#define OEMGDS_MINOUTLINE           0x8005  // WORD - min outline font size, ps only
#define OEMGDS_MAXBITMAP            0x8006  // WORD - max bitmap font size, ps only

#define OEMGDS_MAX                  0x10000


// dwType  flags for use with     STDMETHOD (DrvGetGPDData)
#define GPD_OEMCUSTOMDATA           1

// @@END_INTERNAL_PUBLISHCOMMON

// @@BEGIN_INTERNAL_PUBLISHDRV

/*******************************************************************************
 *
 * Definitions used by kernel-mode rendering module only:
 *  Make sure the macro KERNEL_MODE is defined and
 *  the header file winddi.h is included before this file.
 */

// @@END_INTERNAL_PUBLISHDRV

#ifdef KERNEL_MODE

// @@BEGIN_INTERNAL_PUBLISHDRV

//
// OEMEnableDriver
//

__drv_preferredFunction("IPrintOemPS or IPrintOemUni", "Obsolete")
BOOL APIENTRY
OEMEnableDriver(
    DWORD                               dwOemIntfVersion,
    DWORD                               cbSize,
    _Out_writes_bytes_(cbSize) PDRVENABLEDATA pded
    );

typedef struct _DEVOBJ *PDEVOBJ;
typedef PVOID PDEVOEM;

typedef DWORD (APIENTRY *PFN_DrvWriteSpoolBuf)(
    PDEVOBJ                    pdevobj,
    _In_reads_bytes_(cbSize) PVOID  pBuffer,
    DWORD                      cbSize
    );

typedef DWORD (APIENTRY *PFN_DrvWriteAbortBuf)(
    PDEVOBJ                    pdevobj,
    _In_reads_bytes_(cbSize) PVOID  pBuffer,
    DWORD                      cbSize,
    DWORD                      dwWait
    );

typedef INT (APIENTRY *PFN_DrvXMoveTo)(
    PDEVOBJ pdevobj,
    INT     x,
    DWORD   dwFlags
    );

typedef INT (APIENTRY *PFN_DrvYMoveTo)(
    PDEVOBJ pdevobj,
    INT     y,
    DWORD   dwFlags
    );

typedef BOOL (APIENTRY *PFN_DrvGetStandardVariable)(
    PDEVOBJ                        pdevobj,
    DWORD                          dwIndex,
    _Out_writes_bytes_opt_(cbSize) PVOID pBuffer,
    DWORD                          cbSize,
    _Out_ PDWORD                   pcbNeeded
    );

typedef enum _STDVARIABLEINDEX{

        SVI_NUMDATABYTES,          // "NumOfDataBytes"
        SVI_WIDTHINBYTES,          // "RasterDataWidthInBytes"
        SVI_HEIGHTINPIXELS,        // "RasterDataHeightInPixels"
        SVI_COPIES,                // "NumOfCopies"
        SVI_PRINTDIRECTION,        // "PrintDirInCCDegrees"
        SVI_DESTX,                 // "DestX"
        SVI_DESTY,                 // "DestY"
        SVI_DESTXREL,              // "DestXRel"
        SVI_DESTYREL,              // "DestYRel"
        SVI_LINEFEEDSPACING,       // "LinefeedSpacing"
        SVI_RECTXSIZE,             // "RectXSize"
        SVI_RECTYSIZE,             // "RectYSize"
        SVI_GRAYPERCENT,           // "GrayPercentage"
        SVI_NEXTFONTID,            // "NextFontID"
        SVI_NEXTGLYPH,             // "NextGlyph"
        SVI_PHYSPAPERLENGTH,       // "PhysPaperLength"
        SVI_PHYSPAPERWIDTH,        // "PhysPaperWidth"
        SVI_FONTHEIGHT,            // "FontHeight"
        SVI_FONTWIDTH,             // "FontWidth"
        SVI_FONTMAXWIDTH,             // "FontMaxWidth"
        SVI_FONTBOLD,              // "FontBold"
        SVI_FONTITALIC,            // "FontItalic"
        SVI_FONTUNDERLINE,         // "FontUnderline"
        SVI_FONTSTRIKETHRU,        // "FontStrikeThru"
        SVI_CURRENTFONTID,         // "CurrentFontID"
        SVI_TEXTYRES,              // "TextYRes"
        SVI_TEXTXRES,              // "TextXRes"

        SVI_GRAPHICSYRES,              // "GraphicsYRes"
        SVI_GRAPHICSXRES,              // "GraphicsXRes"

        SVI_ROP3,                  // "Rop3"
        SVI_REDVALUE,              // "RedValue"
        SVI_GREENVALUE,            // "GreenValue"
        SVI_BLUEVALUE,             // "BlueValue"
        SVI_PALETTEINDEXTOPROGRAM, // "PaletteIndexToProgram"
        SVI_CURRENTPALETTEINDEX,   // "CurrentPaletteIndex"
        SVI_PATTERNBRUSH_TYPE,     // "PatternBrushType"
        SVI_PATTERNBRUSH_ID,       // "PatternBrushID"
        SVI_PATTERNBRUSH_SIZE,     // "PatternBrushSize"
        SVI_CURSORORIGINX,           //  "CursorOriginX"
        SVI_CURSORORIGINY,           //  "CursorOriginY"
                //  this is in MasterUnits and in the coordinates of the currently selected orientation.
                //  this value is defined as ImageableOrigin - CursorOrigin
        SVI_PAGENUMBER,  //  "PageNumber"
                //  this value tracks number of times DrvStartBand has been called since
                //  StartDoc.

        SVI_MAX             //  Just a placeholder do not use.
}STDVARIABLEINDEX;

typedef BOOL (APIENTRY *PFN_DrvUnidriverTextOut)(
    SURFOBJ    *pso,
    STROBJ     *pstro,
    FONTOBJ    *pfo,
    CLIPOBJ    *pco,
    RECTL      *prclExtra,
    RECTL      *prclOpaque,
    BRUSHOBJ   *pboFore,
    BRUSHOBJ   *pboOpaque,
    POINTL     *pptlBrushOrg,
    MIX         mix
    );

//
// bit fields defined for dwFlags
//
// Note:  The following Bit values are reserved for an internal use!
//  0x4000
//  0x8000
//
//
#define MV_UPDATE       0x0001
#define MV_RELATIVE     0x0002
#define MV_GRAPHICS     0x0004
#define MV_PHYSICAL     0x0008
#define MV_SENDXMOVECMD     0x0010
#define MV_SENDYMOVECMD     0x0020

typedef struct _DRVPROCS {

    PFN_DrvWriteSpoolBuf    DrvWriteSpoolBuf;   // common to both pscript and unidrv
    PFN_DrvXMoveTo          DrvXMoveTo;         // unidrv specific
    PFN_DrvYMoveTo          DrvYMoveTo;         // unidrv specific
    PFN_DrvGetDriverSetting DrvGetDriverSetting;// common to both pscript and unidrv
    PFN_DrvGetStandardVariable BGetStandardVariable; // unidrv specific
    PFN_DrvUnidriverTextOut    DrvUnidriverTextOut;  // unidrv specific
    PFN_DrvWriteAbortBuf    DrvWriteAbortBuf;   // unidrv specific

} DRVPROCS, *PDRVPROCS;

typedef struct _DEVOBJ {

    DWORD       dwSize;       // size of DEVOBJ structure
    PDEVOEM     pdevOEM;      // pointer to OEM's device data
    HANDLE      hEngine;      // GDI handle for current printer
    HANDLE      hPrinter;     // spooler handle for current printer
    HANDLE      hOEM;         // handle to OEM dll
    PDEVMODE    pPublicDM;    // public devmode
    PVOID       pOEMDM;       // OEM private devmode
    PDRVPROCS   pDrvProcs;    // pointer to kernel mode helper function table

} DEVOBJ;

//
// OEMDisableDriver
//

__drv_preferredFunction("IPrintOemPS or IPrintOemUni", "Obsolete")
VOID APIENTRY
OEMDisableDriver(
    VOID
    );

//
// OEMEnablePDEV
//

__drv_preferredFunction("IPrintOemPS or IPrintOemUni", "Obsolete")
PDEVOEM APIENTRY
OEMEnablePDEV(
    PDEVOBJ                            pdevobj,
    _In_ PWSTR                         pPrinterName,
    ULONG                              cPatterns,
    _Inout_updates_(cPatterns) HSURF    *phsurfPatterns,
    ULONG                              cjGdiInfo,
    _Inout_updates_bytes_(cjGdiInfo) GDIINFO  *pGdiInfo,
    ULONG                              cjDevInfo,
    _Inout_updates_bytes_(cjDevInfo) DEVINFO  *pDevInfo,
    _In_ DRVENABLEDATA                 *pded
    );

//
// OEMDisablePDEV
//

__drv_preferredFunction("IPrintOemPS or IPrintOemUni", "Obsolete")
VOID APIENTRY
OEMDisablePDEV(
    PDEVOBJ pdevobj
    );

//
// OEMResetPDEV
//

__drv_preferredFunction("IPrintOemPS or IPrintOemUni", "Obsolete")
BOOL APIENTRY
OEMResetPDEV(
    PDEVOBJ pdevobjOld,
    PDEVOBJ pdevobjNew
    );

//
// OEMCommand - PSCRIPT only
//

__drv_preferredFunction("IPrintOemPS", "Obsolete")
DWORD APIENTRY
OEMCommand(
    PDEVOBJ                        pdevobj,
    DWORD                          dwIndex,
    _In_reads_bytes_opt_(cbSize) PVOID  pData,
    DWORD                          cbSize
    );

__drv_preferredFunction("IPrintOemUni", "Obsolete")
INT APIENTRY
OEMCommandCallback(
    PDEVOBJ                         pdevobj,
    DWORD                           dwCallbackID,
    DWORD                           dwCount,
    _In_reads_opt_(dwCount) PDWORD pdwParams
    );

//
// OEMImageProcessing - UNIDRV only
//

typedef struct {
    DWORD dwSize;
    POINT ptOffset;
    PSTR  pHalftoneOption;
    BOOL  bBanding;
    BOOL  bBlankBand;
} IPPARAMS, *PIPPARAMS;

__drv_preferredFunction("IPrintOemUni", "Obsolete")
PBYTE APIENTRY
OEMImageProcessing(
    PDEVOBJ     pdevobj,
    PBYTE       pSrcBitmap,
    PBITMAPINFOHEADER pBitmapInfoHeader,
    PBYTE       pColorTable,
    DWORD       dwCallbackID,
    PIPPARAMS   pIPParams
    );

//
// OEMFilterGraphics - UNIDRV only
//

__drv_preferredFunction("IPrintOemUni", "Obsolete")
BOOL APIENTRY
OEMFilterGraphics(
    PDEVOBJ                    pdevobj,
    _In_reads_bytes_(dwLen) PBYTE   pBuf,
    DWORD                      dwLen
    );

//
// OEMCompression - UNIDRV only
//
__drv_preferredFunction("IPrintOemUni", "Obsolete")
INT APIENTRY
OEMCompression(
    PDEVOBJ                       pdevobj,
    _In_reads_bytes_(dwInLen) PBYTE    pInBuf,
    _Out_writes_bytes_(dwOutLen) PBYTE  pOutBuf,
    DWORD                         dwInLen,
    DWORD                         dwOutLen
    );

//
// OEMHalftone - UNIDRV only
//

__drv_preferredFunction("IPrintOemUni", "Obsolete")
BOOL APIENTRY
OEMHalftonePattern(
    PDEVOBJ                                pdevobj,
    PBYTE                                  pHTPattern,
    DWORD                                  dwHTPatternX,
    DWORD                                  dwHTPatternY,
    DWORD                                  dwHTNumPatterns,
    DWORD                                  dwCallbackID,
    _In_reads_bytes_opt_(dwResourceSize) PBYTE  pResource,
    DWORD                                  dwResourceSize
    );

//
// OEMMemoryUsage - UNIDRV only
//

typedef struct {
    DWORD   dwFixedMemoryUsage;
    DWORD   dwPercentMemoryUsage;
    DWORD   dwMaxBandSize;
} OEMMEMORYUSAGE, *POEMMEMORYUSAGE;

VOID APIENTRY
OEMMemoryUsage(
    PDEVOBJ                 pdevobj,
    _Inout_ POEMMEMORYUSAGE pMemoryUsage
    );

//
// OEMTTYGetInfo - UNIDRV only
//
__drv_preferredFunction("IPrintOemUni", "Obsolete")
INT APIENTRY
OEMTTYGetInfo(
    PDEVOBJ                     pdevobj,
    DWORD                       dwInfoIndex,
    _Out_writes_bytes_(dwSize) PVOID  pOutputBuf,
    DWORD                       dwSize,
    _Out_ DWORD                 *pcbcNeeded
    );

#define OEMTTY_INFO_MARGINS     1
#define OEMTTY_INFO_CODEPAGE    2
#define OEMTTY_INFO_NUM_UFMS    3
#define OEMTTY_INFO_UFM_IDS     4

//
// UNIDRV font callback
//

typedef BOOL (*PFNGETINFO)(struct _UNIFONTOBJ*, DWORD, PVOID, DWORD, PDWORD);

typedef struct _UNIFONTOBJ {
    ULONG       ulFontID;
    DWORD       dwFlags;     // General flags
    IFIMETRICS *pIFIMetrics; // Pointer to IFIMETRICS
    PFNGETINFO  pfnGetInfo;  // Pointer to UNIFONTOBJ_GetInfo callback
} UNIFONTOBJ, *PUNIFONTOBJ;

//
// UNIFONTOBJ.dwFlags
//

#define UFOFLAG_TTFONT               0x00000001
#define UFOFLAG_TTDOWNLOAD_BITMAP    0x00000002
#define UFOFLAG_TTDOWNLOAD_TTOUTLINE 0x00000004

#if (NTDDI_VERSION >= NTDDI_WINXP)

#define UFOFLAG_TTOUTLINE_BOLD_SIM   0x00000008
#define UFOFLAG_TTOUTLINE_ITALIC_SIM 0x00000010
#define UFOFLAG_TTOUTLINE_VERTICAL   0x00000020
#define UFOFLAG_TTSUBSTITUTED        0x00000040

#endif // (NTDDI_VERSION >= NTDDI_WINXP)

//
// UNIFONTOBJ callback ID
//

#define UFO_GETINFO_FONTOBJ     1
#define UFO_GETINFO_GLYPHSTRING 2
#define UFO_GETINFO_GLYPHBITMAP 3
#define UFO_GETINFO_GLYPHWIDTH  4
#define UFO_GETINFO_MEMORY      5
#define UFO_GETINFO_STDVARIABLE 6

//
// UFO_GETINFO_FONTOBJ callback structure
//

typedef struct _GETINFO_FONTOBJ {
    DWORD    dwSize;   // Size of this structure
    FONTOBJ *pFontObj; // Pointer to the FONTOBJ
} GETINFO_FONTOBJ, *PGETINFO_FONTOBJ;

//
// UFO_GETINFO_FONTOBJ callback structure
//

typedef struct _GETINFO_GLYPHSTRING {
    DWORD dwSize;    // Size of this structure
    DWORD dwCount;   // Count of glyphs in pGlyphIn
    DWORD dwTypeIn;  // Glyph type of pGlyphIn, TYPE_GLYPHID/TYPE_HANDLE.
    PVOID pGlyphIn;  // Pointer to the input glyph string
    DWORD dwTypeOut; // Glyph type of pGlyphOut, TYPE_UNICODE/TYPE_TRANSDATA.
    PVOID pGlyphOut; // Pointer to the output glyph string
    DWORD dwGlyphOutSize; // The size of pGlyphOut buffer
} GETINFO_GLYPHSTRING, *PGETINFO_GLYPHSTRING;

//
// UFO_GETINFO_GLYPHBITMAP
//

typedef struct _GETINFO_GLYPHBITMAP {
    DWORD       dwSize;    // Size of this structure
    HGLYPH      hGlyph;    // Glyph hangle passed in OEMDownloadCharGlyph
    GLYPHDATA *pGlyphData; // Pointer to the GLYPHDATA data structure
} GETINFO_GLYPHBITMAP, *PGETINFO_GLYPHBITMAP;

//
// UFO_GETINFO_GLYPHWIDTH
//

typedef struct _GETINFO_GLYPHWIDTH {
    DWORD dwSize;  // Size of this structure
    DWORD dwType;  // Type of glyph stirng in pGlyph, TYPE_GLYPHHANDLE/GLYPHID.
    DWORD dwCount; // Count of glyph in pGlyph
    PVOID pGlyph;  // Pointer to a glyph string
    PLONG plWidth; // Pointer to the buffer of width table.
                   // Minidriver has to prepare this.
} GETINFO_GLYPHWIDTH, *PGETINFO_GLYPHWIDTH;

//
// UFO_GETINFO_MEMORY
//

typedef struct _GETINFO_MEMORY {
    DWORD dwSize;
    DWORD dwRemainingMemory;
} GETINFO_MEMORY, PGETINFO_MEMROY;

//
// UFO_GETINFO_STDVARIABLE
//
// OEM DLL has to prepare all StdVar buffer and set ID in dwStdVarID.
//

typedef struct _GETINFO_STDVAR {
    DWORD dwSize;
    DWORD dwNumOfVariable;
    struct {
        DWORD dwStdVarID;
        LONG  lStdVariable;
    } StdVar[1];
} GETINFO_STDVAR, *PGETINFO_STDVAR;


#define FNT_INFO_PRINTDIRINCCDEGREES  0 // PrintDirInCCDegrees
#define FNT_INFO_GRAYPERCENTAGE       1 // GrayPercentage
#define FNT_INFO_NEXTFONTID           2 // NextfontID
#define FNT_INFO_NEXTGLYPH            3 // NextGlyph
#define FNT_INFO_FONTHEIGHT           4 // FontHeight
#define FNT_INFO_FONTWIDTH            5 // FontWidth
#define FNT_INFO_FONTBOLD             6 // FontBold
#define FNT_INFO_FONTITALIC           7 // FontItalic
#define FNT_INFO_FONTUNDERLINE        8 // FontUnderline
#define FNT_INFO_FONTSTRIKETHRU       9 // FontStrikeThru
#define FNT_INFO_CURRENTFONTID       10 // Current
#define FNT_INFO_TEXTYRES            11 // TextYRes
#define FNT_INFO_TEXTXRES            12 // TextXRes
#define FNT_INFO_FONTMAXWIDTH        13 // FontMaxWidth
#define FNT_INFO_MAX                 14

//
// OEMDownloadFontheader - UNIDRV only
//

__drv_preferredFunction("IPrintOemUni", "Obsolete")
DWORD APIENTRY
OEMDownloadFontHeader(
    PDEVOBJ     pdevobj,
    PUNIFONTOBJ pUFObj
    );

//
// OEMDownloadCharGlyph - UNIDRV only
//

__drv_preferredFunction("IPrintOemUni", "Obsolete")
DWORD APIENTRY
OEMDownloadCharGlyph(
    PDEVOBJ       pdevobj,
    PUNIFONTOBJ   pUFObj,
    HGLYPH        hGlyph,
    _Out_ PDWORD  pdwWidth
    );

//
// OEMTTDownloadMethod - UNIDRV only
//

__drv_preferredFunction("IPrintOemUni", "Obsolete")
DWORD APIENTRY
OEMTTDownloadMethod(
    PDEVOBJ     pdevobj,
    PUNIFONTOBJ pUFObj
    );

#define TTDOWNLOAD_DONTCARE  0
#define TTDOWNLOAD_GRAPHICS  1
#define TTDOWNLOAD_BITMAP    2
#define TTDOWNLOAD_TTOUTLINE 3

//
// OEMOutputCharStr - UNIDRV only
//

__drv_preferredFunction("IPrintOemUni", "Obsolete")
VOID APIENTRY
OEMOutputCharStr(
    PDEVOBJ                     pdevobj,
    PUNIFONTOBJ                 pUFObj,
    DWORD                       dwType,
    DWORD                       dwCount,
    _In_reads_(dwCount) PVOID  pGlyph
    );

#define TYPE_UNICODE      1
#define TYPE_TRANSDATA    2
#define TYPE_GLYPHHANDLE  3
#define TYPE_GLYPHID      4

//
// OEMSendFontCmd - UNIDRV only
//

typedef struct _FINVOCATION {
    DWORD dwCount;    // Size of command
    PBYTE pubCommand; // Pointer to font selection command
} FINVOCATION, *PFINVOCATION;

__drv_preferredFunction("IPrintOemUni", "Obsolete")
VOID APIENTRY
OEMSendFontCmd(
    PDEVOBJ      pdevobj,
    PUNIFONTOBJ  pUFObj,
    PFINVOCATION pFInv
    );

//
// OEMTextOutAsBitmap - UNIDRV only
//
__drv_preferredFunction("IPrintOemUni", "Obsolete")
BOOL APIENTRY
OEMTextOutAsBitmap(
    SURFOBJ    *pso,
    STROBJ     *pstro,
    FONTOBJ    *pfo,
    CLIPOBJ    *pco,
    RECTL      *prclExtra,
    RECTL      *prclOpaque,
    BRUSHOBJ   *pboFore,
    BRUSHOBJ   *pboOpaque,
    POINTL     *pptlOrg,
    MIX         mix
    );

//
// OEMBitBlt
//

BOOL APIENTRY
OEMBitBlt(
    SURFOBJ        *psoTrg,
    SURFOBJ        *psoSrc,
    SURFOBJ        *psoMask,
    CLIPOBJ        *pco,
    XLATEOBJ       *pxlo,
    RECTL          *prclTrg,
    POINTL         *pptlSrc,
    POINTL         *pptlMask,
    BRUSHOBJ       *pbo,
    POINTL         *pptlBrush,
    ROP4            rop4
    );

//
// OEMStretchBlt
//

BOOL APIENTRY
OEMStretchBlt(
    SURFOBJ         *psoDest,
    SURFOBJ         *psoSrc,
    SURFOBJ         *psoMask,
    CLIPOBJ         *pco,
    XLATEOBJ        *pxlo,
    COLORADJUSTMENT *pca,
    POINTL          *pptlHTOrg,
    RECTL           *prclDest,
    RECTL           *prclSrc,
    POINTL          *pptlMask,
    ULONG            iMode
    );

//
// OEMCopyBits
//

BOOL APIENTRY
OEMCopyBits(
    SURFOBJ        *psoDest,
    SURFOBJ        *psoSrc,
    CLIPOBJ        *pco,
    XLATEOBJ       *pxlo,
    RECTL          *prclDest,
    POINTL         *pptlSrc
    );

//
// OEMTextOut
//

BOOL APIENTRY
OEMTextOut(
    SURFOBJ    *pso,
    STROBJ     *pstro,
    FONTOBJ    *pfo,
    CLIPOBJ    *pco,
    RECTL      *prclExtra,
    RECTL      *prclOpaque,
    BRUSHOBJ   *pboFore,
    BRUSHOBJ   *pboOpaque,
    POINTL     *pptlOrg,
    MIX         mix
    );

//
// OEMStrokePath
//

BOOL APIENTRY
OEMStrokePath(
    SURFOBJ    *pso,
    PATHOBJ    *ppo,
    CLIPOBJ    *pco,
    XFORMOBJ   *pxo,
    BRUSHOBJ   *pbo,
    POINTL     *pptlBrushOrg,
    LINEATTRS  *plineattrs,
    MIX         mix
    );

//
// OEMFillPath
//

BOOL APIENTRY
OEMFillPath(
    SURFOBJ    *pso,
    PATHOBJ    *ppo,
    CLIPOBJ    *pco,
    BRUSHOBJ   *pbo,
    POINTL     *pptlBrushOrg,
    MIX         mix,
    FLONG       flOptions
    );

//
// OEMStrokeAndFillPath
//

BOOL APIENTRY
OEMStrokeAndFillPath(
    SURFOBJ    *pso,
    PATHOBJ    *ppo,
    CLIPOBJ    *pco,
    XFORMOBJ   *pxo,
    BRUSHOBJ   *pboStroke,
    LINEATTRS  *plineattrs,
    BRUSHOBJ   *pboFill,
    POINTL     *pptlBrushOrg,
    MIX         mixFill,
    FLONG       flOptions
    );

//
// OEMRealizeBrush
//

BOOL APIENTRY
OEMRealizeBrush(
    BRUSHOBJ   *pbo,
    SURFOBJ    *psoTarget,
    SURFOBJ    *psoPattern,
    SURFOBJ    *psoMask,
    XLATEOBJ   *pxlo,
    ULONG       iHatch
    );

//
// OEMStartPage
//

BOOL APIENTRY
OEMStartPage(
    SURFOBJ    *pso
    );

//
// OEMSendPage
//

BOOL APIENTRY
OEMSendPage(
    SURFOBJ    *pso
    );

//
// OEMEscape
//

ULONG APIENTRY
OEMEscape(
    SURFOBJ                   *pso,
    ULONG                      iEsc,
    ULONG                      cjIn,
    _In_reads_bytes_(cjIn) PVOID    pvIn,
    ULONG                      cjOut,
    _Out_writes_bytes_(cjOut) PVOID  pvOut
    );

//
// OEMStartDoc
//

BOOL APIENTRY
OEMStartDoc(
    SURFOBJ    *pso,
    _In_ PWSTR  pwszDocName,
    DWORD       dwJobId
    );

//
// OEMEndDoc
//

BOOL APIENTRY
OEMEndDoc(
    SURFOBJ    *pso,
    FLONG       fl
    );

//
// OEMQueryFont
//

PIFIMETRICS APIENTRY
OEMQueryFont(
    DHPDEV      dhpdev,
    ULONG_PTR   iFile,
    ULONG       iFace,
    ULONG_PTR   *pid
    );

//
// OEMQueryFontTree
//

PVOID APIENTRY
OEMQueryFontTree(
    DHPDEV      dhpdev,
    ULONG_PTR   iFile,
    ULONG       iFace,
    ULONG       iMode,
    ULONG_PTR   *pid
    );

//
// OEMQueryFontData
//

LONG APIENTRY
OEMQueryFontData(
    DHPDEV                      dhpdev,
    FONTOBJ                    *pfo,
    ULONG                       iMode,
    HGLYPH                      hg,
    GLYPHDATA                  *pgd,
    _Out_writes_bytes_(cjSize) PVOID  pv,
    ULONG                       cjSize
    );

//
// OEMQueryAdvanceWidths
//

BOOL APIENTRY
OEMQueryAdvanceWidths(
    DHPDEV                                       dhpdev,
    FONTOBJ                                     *pfo,
    ULONG                                        iMode,
    _In_reads_(cGlyphs) HGLYPH                 *phg,
    _Out_writes_bytes_(cGlyphs*sizeof(USHORT)) PVOID   pvWidths,
    ULONG                                        cGlyphs
    );

//
// OEMFontManagement
//

ULONG APIENTRY
OEMFontManagement(
    SURFOBJ                   *pso,
    FONTOBJ                   *pfo,
    ULONG                      iMode,
    ULONG                      cjIn,
    _In_reads_bytes_(cjIn) PVOID    pvIn,
    ULONG                      cjOut,
    _Out_writes_bytes_(cjOut) PVOID  pvOut
    );

//
// OEMGetGlyphMode
//

ULONG APIENTRY
OEMGetGlyphMode(
    DHPDEV      dhpdev,
    FONTOBJ    *pfo
    );

BOOL APIENTRY
OEMNextBand(
    SURFOBJ *pso,
    POINTL *pptl
    );

BOOL APIENTRY
OEMStartBanding(
    SURFOBJ *pso,
    POINTL *pptl
    );

ULONG APIENTRY
OEMDitherColor(
    DHPDEV  dhpdev,
    ULONG   iMode,
    ULONG   rgbColor,
    ULONG  *pulDither
    );

BOOL APIENTRY
OEMPaint(
    SURFOBJ         *pso,
    CLIPOBJ         *pco,
    BRUSHOBJ        *pbo,
    POINTL          *pptlBrushOrg,
    MIX             mix
    );

BOOL APIENTRY
OEMLineTo(
    SURFOBJ    *pso,
    CLIPOBJ    *pco,
    BRUSHOBJ   *pbo,
    LONG        x1,
    LONG        y1,
    LONG        x2,
    LONG        y2,
    RECTL      *prclBounds,
    MIX         mix
    );

//
// OEMStretchBltROP
//

BOOL APIENTRY
OEMStretchBltROP(
    SURFOBJ         *psoDest,
    SURFOBJ         *psoSrc,
    SURFOBJ         *psoMask,
    CLIPOBJ         *pco,
    XLATEOBJ        *pxlo,
    COLORADJUSTMENT *pca,
    POINTL          *pptlHTOrg,
    RECTL           *prclDest,
    RECTL           *prclSrc,
    POINTL          *pptlMask,
    ULONG            iMode,
    BRUSHOBJ        *pbo,
    ROP4             rop4
    );

//
// OEMPlgBlt
//

BOOL APIENTRY
OEMPlgBlt(
    SURFOBJ         *psoDst,
    SURFOBJ         *psoSrc,
    SURFOBJ         *psoMask,
    CLIPOBJ         *pco,
    XLATEOBJ        *pxlo,
    COLORADJUSTMENT *pca,
    POINTL          *pptlBrushOrg,
    POINTFIX        *pptfixDest,
    RECTL           *prclSrc,
    POINTL          *pptlMask,
    ULONG           iMode
    );

//
// OEMAlphaBlend
//

BOOL APIENTRY
OEMAlphaBlend(
    SURFOBJ    *psoDest,
    SURFOBJ    *psoSrc,
    CLIPOBJ    *pco,
    XLATEOBJ   *pxlo,
    RECTL      *prclDest,
    RECTL      *prclSrc,
    BLENDOBJ   *pBlendObj
    );

//
// OEMGradientFill
//

BOOL APIENTRY
OEMGradientFill(
    SURFOBJ    *psoDest,
    CLIPOBJ    *pco,
    XLATEOBJ   *pxlo,
    TRIVERTEX  *pVertex,
    ULONG       nVertex,
    PVOID       pMesh,
    ULONG       nMesh,
    RECTL      *prclExtents,
    POINTL     *pptlDitherOrg,
    ULONG       ulMode
    );

//
// OEMIcmCreateTransform
//

HANDLE APIENTRY
OEMIcmCreateColorTransform(
    DHPDEV                                   dhpdev,
    LPLOGCOLORSPACEW                         pLogColorSpace,
    _In_reads_bytes_opt_(cjSourceProfile) PVOID   pvSourceProfile,
    ULONG                                    cjSourceProfile,
    _In_reads_bytes_(cjDestProfile) PVOID         pvDestProfile,
    ULONG                                    cjDestProfile,
    _In_reads_bytes_opt_(cjTargetProfile) PVOID   pvTargetProfile,
    ULONG                                    cjTargetProfile,
    DWORD                                    dwReserved
    );

//
// OEMIcmDeleteTransform
//

BOOL APIENTRY
OEMIcmDeleteColorTransform(
    DHPDEV dhpdev,
    HANDLE hcmXform
    );

//
// OEMQueryDeviceSupport
//

BOOL APIENTRY
OEMQueryDeviceSupport(
    SURFOBJ                   *pso,
    XLATEOBJ                  *pxlo,
    XFORMOBJ                  *pxo,
    ULONG                      iType,
    ULONG                      cjIn,
    _In_reads_bytes_(cjIn) PVOID    pvIn,
    ULONG                      cjOut,
    _Out_writes_bytes_(cjOut) PVOID  pvOut
    );

//
// OEMTransparentBlt
//

BOOL APIENTRY
OEMTransparentBlt(
    SURFOBJ    *psoDst,
    SURFOBJ    *psoSrc,
    CLIPOBJ    *pco,
    XLATEOBJ   *pxlo,
    RECTL      *prclDst,
    RECTL      *prclSrc,
    ULONG      iTransColor,
    ULONG      ulReserved
    );

#if (NTDDI_VERSION >= NTDDI_WINXP)
//
// for the IPrintOemPS2::GetPDEVAdjustment call
//
#define PDEV_ADJUST_PAPER_MARGIN_TYPE 1

typedef struct _PDEV_ADJUST_PAPER_MARGIN {

    RECTL   rcImageableArea;    // contains the imageable area in 0.001 mm units

} PDEV_ADJUST_PAPER_MARGIN;

#define PDEV_HOSTFONT_ENABLED_TYPE 2

typedef struct _PDEV_HOSTFONT_ENABLED {

    BOOL bHostfontEnabled;

} PDEV_HOSTFONT_ENABLED;

#define PDEV_USE_TRUE_COLOR_TYPE 3

typedef struct _PDEV_USE_TRUE_COLOR {

    BOOL bUseTrueColor;

} PDEV_USE_TRUE_COLOR;

#endif // (NTDDI_VERSION >= NTDDI_WINXP)


#if (NTDDI_VERSION >= NTDDI_VISTA)

#define PDEV_ADJUST_GRAPHICS_RESOLUTION_TYPE          (0x1 << 2) //0x4
#define PDEV_ADJUST_IMAGEABLE_ORIGIN_AREA_TYPE        (0x1 << 3) //0x8
#define PDEV_ADJUST_PHYSICAL_PAPER_SIZE_TYPE          (0x1 << 4) //0x10

typedef struct _PDEV_ADJUST_IMAGEABLE_ORIGIN_AREA {

    POINT    ptImageOrigin;
    SIZEL    szlImageableArea;

} PDEV_ADJUST_IMAGEABLE_ORIGIN_AREA, *PPDEV_ADJUST_IMAGEABLE_ORIGIN_AREA;

typedef struct _PDEV_ADJUST_PHYSICAL_PAPER_SIZE {

    SIZEL    szlPhysicalPaperSize;

} PDEV_ADJUST_PHYSICAL_PAPER_SIZE, *PPDEV_ADJUST_PHYSICAL_PAPER_SIZE;

typedef struct _PDEV_ADJUST_GRAPHICS_RESOLUTION {

    POINT    ptGraphicsResolution;

} PDEV_ADJUST_GRAPHICS_RESOLUTION, *PPDEV_ADJUST_GRAPHICS_RESOLUTION;

#endif // (NTDDI_VERSION >= NTDDI_VISTA)

// @@END_INTERNAL_PUBLISHDRV

#endif // KERNEL_MODE

// @@BEGIN_INTERNAL_PUBLISHCFG

/*******************************************************************************
 *
 * Definitions used by user-mode UI module only:
 *  Make sure the macro KERNEL_MODE is NOT defined and
 *  the header file winddiui.h is included before this file.
 *
 */

// @@END_INTERNAL_PUBLISHCFG

#ifndef KERNEL_MODE

// @@BEGIN_INTERNAL_PUBLISHCFG

#if (NTDDI_VERSION >= NTDDI_WINXP)

//
// Reports capability of simulated features
//
typedef struct _SIMULATE_CAPS_1 {
    DWORD     dwLevel;
    DWORD     dwPageOrderFlags;         // Reverse page order
    DWORD     dwNumberOfCopies;         // Max number of copies
    DWORD     dwCollate;                // Collate support
    DWORD     dwNupOptions;             // The (1-base) bit set represents the N-up option available.
                                        // 0x0001 means 1-up
                                        // 0x0002 means 2-up
                                        // 0x0008 means 4-up
                                        // 0x812B means (1,2,4,6,9,16)
} SIMULATE_CAPS_1, *PSIMULATE_CAPS_1;

#endif // (NTDDI_VERSION >= NTDDI_WINXP)

//
// Reference to driver data structure. This is passed to
// most of the OEM UI DLL entrypoints.
//

typedef struct _OEMUIPROCS {

    PFN_DrvGetDriverSetting DrvGetDriverSetting;
    PFN_DrvUpdateUISetting  DrvUpdateUISetting;

} OEMUIPROCS, *POEMUIPROCS;


typedef struct _OEMUIOBJ {

    DWORD           cbSize;             // size of this structure
    POEMUIPROCS     pOemUIProcs;        // pointer to user mode helper function table

} OEMUIOBJ, *POEMUIOBJ;

//
// OEMCommonUIProp
//

typedef struct _OEMCUIPPARAM *POEMCUIPPARAM;
typedef LONG (APIENTRY *OEMCUIPCALLBACK)(PCPSUICBPARAM, POEMCUIPPARAM);

typedef struct _OEMCUIPPARAM {

    DWORD           cbSize;         // size of this structure
    POEMUIOBJ       poemuiobj;      // reference to driver data structure
    HANDLE          hPrinter;       // handle to the current printer
    PWSTR           pPrinterName;   // name of current printer
    HANDLE          hModule;        // instance handle to OEM DLL
    HANDLE          hOEMHeap;       // handle to the OEM memory heap
    PDEVMODE        pPublicDM;      // public devmode
    PVOID           pOEMDM;         // OEM private devmode
    DWORD           dwFlags;        // misc. flag bits
    POPTITEM        pDrvOptItems;   // pointer to driver items
    DWORD           cDrvOptItems;   // number of driver items
    POPTITEM        pOEMOptItems;   // pointer to OEM items
    DWORD           cOEMOptItems;   // number of OEM items
    PVOID           pOEMUserData;   // pointer to OEM private data
    OEMCUIPCALLBACK  OEMCUIPCallback; // address of callback function

} OEMCUIPPARAM;

__drv_preferredFunction("IPrintOemUI", "Obsolete")
BOOL APIENTRY
OEMCommonUIProp(
    DWORD           dwMode,
    POEMCUIPPARAM   pOemCUIPParam
    );

//
// OEMCommonUIProp dwMode parameter value
//
#define OEMCUIP_DOCPROP       1
#define OEMCUIP_PRNPROP       2

//
// OEMDocumentPropertySheets
//

__drv_preferredFunction("IPrintOemUI", "Obsolete")
LRESULT APIENTRY
OEMDocumentPropertySheets(
    _In_ PPROPSHEETUI_INFO pPSUIInfo,
    LPARAM                 lParam
    );

//
// OEMDevicePropertySheets
//

__drv_preferredFunction("IPrintOemUI", "Obsolete")
LRESULT APIENTRY
OEMDevicePropertySheets(
    _In_ PPROPSHEETUI_INFO pPSUIInfo,
    LPARAM                 lParam
    );

//
// pPSUIInfo->lParamInit is a pointer to _OEMUIPSPARAM structure defined below.
//
typedef struct _OEMUIPSPARAM {

    DWORD           cbSize;         // size of this structure
    POEMUIOBJ       poemuiobj;      // reference to driver data structure
    HANDLE          hPrinter;       // handle to the current printer
    PWSTR           pPrinterName;   // name of current printer
    HANDLE          hModule;        // instance handle to OEM DLL
    HANDLE          hOEMHeap;       // handle to the OEM memory heap
    PDEVMODE        pPublicDM;      // public devmode
    PVOID           pOEMDM;         // OEM private devmode
    PVOID           pOEMUserData;   // pointer to OEM private data
    DWORD           dwFlags;        // misc. flag bits
    PVOID           pOemEntry;

} OEMUIPSPARAM, *POEMUIPSPARAM;

#pragma warning(push)
       // Suppress SAL warnings on pre-Windows XP APIs
#pragma warning(disable:28718 28740)

//
// OEMDevQueryPrintEx
//

__drv_preferredFunction("IPrintOemUI", "Obsolete")
BOOL APIENTRY
OEMDevQueryPrintEx(
    POEMUIOBJ           poemuiobj,
    PDEVQUERYPRINT_INFO pDQPInfo,
    PDEVMODE            pPublicDM,
    PVOID               pOEMDM
    );

//
// OEMDeviceCapabilities
//

__drv_preferredFunction("IPrintOemUI", "Obsolete")
DWORD APIENTRY
OEMDeviceCapabilities(
    POEMUIOBJ   poemuiobj,
    HANDLE      hPrinter,
    PWSTR       pDeviceName,
    WORD        wCapability,
    PVOID       pOutput,
    PDEVMODE    pPublicDM,
    PVOID       pOEMDM,
    DWORD       dwLastResult
    );

//
// OEMUpgradePrinter
//

__drv_preferredFunction("IPrintOemUI", "Obsolete")
BOOL APIENTRY
OEMUpgradePrinter(
    DWORD   dwLevel,
    PBYTE   pDriverUpgradeInfo
    );

//
// OEMUpgradeRegistry
//

__drv_preferredFunction("(see documentation)", "Obsolete")
BOOL APIENTRY
OEMUpgradeRegistry(
    DWORD   dwLevel,
    PBYTE   pDriverUpgradeInfo,
    PFN_DrvUpgradeRegistrySetting pfnUpgrade
    );


//
// OEMPrinterEvent
//

__drv_preferredFunction("IPrintOemUI", "Obsolete")
BOOL APIENTRY
OEMPrinterEvent(
    PWSTR   pPrinterName,
    INT     iDriverEvent,
    DWORD   dwFlags,
    LPARAM  lParam
    );

//
// OEMDriverEvent
//

__drv_preferredFunction("IPrintOemUI", "Obsolete")
BOOL APIENTRY
OEMPDriverEvent(
    DWORD   dwDriverEvent,
    DWORD   dwLevel,
    LPBYTE  pDriverInfo,
    LPARAM  lParam
    );


//
// OEMQueryColorProfile
//

__drv_preferredFunction("IPrintOemUI", "Obsolete")
BOOL APIENTRY
OEMQueryColorProfile(
    HANDLE                              hPrinter,
    POEMUIOBJ                           poemuiobj,
    PDEVMODE                            pPublicDM,
    PVOID                               pOEMDM,
    ULONG                               ulQueryMode,
    _Out_writes_bytes_(*pcbProfileData) VOID  *pvProfileData,
    _Out_ ULONG                         *pcbProfileData,
    _Out_ FLONG                         *pflProfileData
    );

//
// Font Installer dialog proc
//

__drv_preferredFunction("IPrintOemUI", "Obsolete")
INT_PTR CALLBACK
OEMFontInstallerDlgProc(
    HWND    hWnd,
    UINT    usMsg,
    WPARAM  wParam,
    LPARAM  lParam
    );


__drv_preferredFunction("IPrintOemUI", "Obsolete")
BOOL CALLBACK
OEMUpdateExternalFonts(
    HANDLE  hPrinter,
    HANDLE  hHeap,
    PWSTR   pwstrCartridges
   );

#pragma warning(pop)

// @@END_INTERNAL_PUBLISHCFG

#endif // !KERNEL_MODE

// @@BEGIN_INTERNAL_PUBLISHCOMMON

#if (NTDDI_VERSION >= NTDDI_WINXP)

//
// Datatypes for attributes plugin can get by calling core driver's
// following helper functions:
//
// GetGlobalAttribute, GetFeatureAttribute, GetOptionAttribute
//

typedef enum _EATTRIBUTE_DATATYPE {

    kADT_UNKNOWN,
    kADT_BOOL,
    kADT_INT,
    kADT_LONG,
    kADT_DWORD,
    kADT_ASCII,              // NULL terminated ASCII string
    kADT_UNICODE,            // NULL terminated Unicode string
    kADT_BINARY,             // binary blob
    kADT_SIZE,
    kADT_RECT,
    kADT_CUSTOMSIZEPARAMS,   // array of CUSTOMSIZEPARAM structures

} EATTRIBUTE_DATATYPE;

//
// Data structure for storing information about PPD's *ParamCustomPageSize entries
//

#define CUSTOMPARAM_WIDTH        0
#define CUSTOMPARAM_HEIGHT       1
#define CUSTOMPARAM_WIDTHOFFSET  2
#define CUSTOMPARAM_HEIGHTOFFSET 3
#define CUSTOMPARAM_ORIENTATION  4
#define CUSTOMPARAM_MAX          5

typedef struct _CUSTOMSIZEPARAM {

    LONG    dwOrder;                // order value
    LONG    lMinVal;                // min value (in microns)
    LONG    lMaxVal;                // max value (in microns)

} CUSTOMSIZEPARAM, *PCUSTOMSIZEPARAM;

//
// constants for SetOptions helper function
//
// SetOptions flag
//

#define SETOPTIONS_FLAG_RESOLVE_CONFLICT       0x00000001
#define SETOPTIONS_FLAG_KEEP_CONFLICT          0x00000002

//
// SetOptions result code
//

#define SETOPTIONS_RESULT_NO_CONFLICT          0
#define SETOPTIONS_RESULT_CONFLICT_RESOLVED    1
#define SETOPTIONS_RESULT_CONFLICT_REMAINED    2

#endif // (NTDDI_VERSION >= NTDDI_WINXP)

// @@END_INTERNAL_PUBLISHCOMMON

#ifdef __cplusplus
}
#endif


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif  // !_PRINTOEM_

