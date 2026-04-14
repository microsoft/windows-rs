
/*++

Copyright (c) 1996-1997  Microsoft Corporation

Module Name:

    prntfont.h

Abstract:

    Declarations for Windows NT printer driver font metrics and glyphset data
    *.UFF, *.UFM and *.GTT file data structure definition

--*/

#ifndef _PRNTFONT_H_
#define _PRNTFONT_H_
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)




//
//
// F O N T  M E T R I C S  ( U F M )
//
//

//
// NOTE: To include this header file, it is necessary to include 
//       winddi.h, that has a definition of IFIMETRICS, FD_KERNINGPAIR
//
//


//
// UNIFM
//
// Universal printer driver (UNIDRV) font file header.
//

#define UNIFM_VERSION_1_0 0x00010000

typedef struct _UNIFM_HDR
{
    DWORD      dwSize;             // a total size of this font file
    DWORD      dwVersion;          // a version number of this font file
    ULONG      ulDefaultCodepage;  // this font's default codepage
    LONG       lGlyphSetDataRCID;  // a resource ID of GLYPHDATA
    DWORD      loUnidrvInfo;       // offset to UNIDRVINFO
    DWORD      loIFIMetrics;       // offset to IFIMETRICS
    DWORD      loExtTextMetric;    // offset to EXTTEXTMETRIC
    DWORD      loWidthTable;       // offset to WIDTHTABLE
    DWORD      loKernPair;         // offset to KERNPAIR
    DWORD      dwReserved[2];
} UNIFM_HDR, *PUNIFM_HDR;

#define GET_UNIDRVINFO(pUFM)    \
        ((PUNIDRVINFO)((PBYTE)(pUFM) + (pUFM)->loUnidrvInfo))
//
// Unidrv UFM files always contain a PRINTIFI32 struct, even on 64-bit windows.
//
#define GET_PRINTIFI32(pUFM)    \
        ((PPRINTIFI32)((PBYTE)(pUFM) + (pUFM)->loIFIMetrics))
#define GET_EXTTEXTMETRIC(pUFM) \
        ((EXTTEXTMETRIC*)((PBYTE)(pUFM) + (pUFM)->loExtTextMetric))
#define GET_WIDTHTABLE(pUFM)    \
        ((PWIDTHTABLE)((PBYTE)(pUFM) + (pUFM)->loWidthTable))
#define GET_KERNDATA(pUFM)      \
        ((PKERNDATA)((PBYTE)(pUFM) + (pUFM)->loKernPair))

//
// Deprecated: Use GET_UNIDRV_IFI instead
//
#define GET_IFIMETRICS(pUFM)    \
        ((IFIMETRICS*)((PBYTE)(pUFM) + (pUFM)->loIFIMetrics))


//
// UNIDRVINFO
//
// UNIDRVINFO is used to define printer specific information.
//

typedef struct _INVOC {
        DWORD  dwCount;     // the number of bytes in the invocation string
        DWORD  loOffset;    // byte-offset to the beginning of the array
} INVOC, *PINVOC;

typedef struct _UNIDRVINFO
{
    DWORD   dwSize;
    DWORD   flGenFlags;
    WORD    wType;
    WORD    fCaps;
    WORD    wXRes;
    WORD    wYRes;
    short   sYAdjust;
    short   sYMoved;
    WORD    wPrivateData; 
    short   sShift; 
    INVOC   SelectFont;
    INVOC   UnSelectFont;
    WORD    wReserved[4];
}  UNIDRVINFO, *PUNIDRVINFO;

#define GET_SELECT_CMD(pUni)    \
        ((PCHAR)(pUni) + (pUni)->SelectFont.loOffset)
#define GET_UNSELECT_CMD(pUni)  \
        ((PCHAR)(pUni) + (pUni)->UnSelectFont.loOffset)

//
// _PRINTIFI32 and _PRINTIFI64 are fixed-size versions of the IFIMETRICS structures 
// that are gauranteed not to change accross architectures or OS versions, and can
// be used for binary file layouts.  _PRINTIFI32 is the format used for Unidrv UFM 
// files on all platformts.  PScript NTF files use the platform-specific version of
// the code.
//
typedef struct _PRINTIFI32 {
    ULONG    cjThis;           // includes attached information
    ULONG    cjIfiExtra;       // sizeof IFIEXTRA if any, formerly ulVersion
    PTRDIFF  dpwszFamilyName;
    PTRDIFF  dpwszStyleName;
    PTRDIFF  dpwszFaceName;
    PTRDIFF  dpwszUniqueName;
    PTRDIFF  dpFontSim;
    LONG     lEmbedId;
    LONG     lItalicAngle;
    LONG     lCharBias;
    PTRDIFF  dpCharSets;            // only used if > 1 charset supported
    BYTE     jWinCharSet;           // as in LOGFONT::lfCharSet
    BYTE     jWinPitchAndFamily;    // as in LOGFONT::lfPitchAndFamily
    USHORT   usWinWeight;           // as in LOGFONT::lfWeight
    ULONG    flInfo;                // see above
    USHORT   fsSelection;           // see above
    USHORT   fsType;                // see above
    FWORD    fwdUnitsPerEm;         // em height
    FWORD    fwdLowestPPEm;         // readable limit
    FWORD    fwdWinAscender;
    FWORD    fwdWinDescender;
    FWORD    fwdMacAscender;
    FWORD    fwdMacDescender;
    FWORD    fwdMacLineGap;
    FWORD    fwdTypoAscender;
    FWORD    fwdTypoDescender;
    FWORD    fwdTypoLineGap;
    FWORD    fwdAveCharWidth;
    FWORD    fwdMaxCharInc;
    FWORD    fwdCapHeight;
    FWORD    fwdXHeight;
    FWORD    fwdSubscriptXSize;
    FWORD    fwdSubscriptYSize;
    FWORD    fwdSubscriptXOffset;
    FWORD    fwdSubscriptYOffset;
    FWORD    fwdSuperscriptXSize;
    FWORD    fwdSuperscriptYSize;
    FWORD    fwdSuperscriptXOffset;
    FWORD    fwdSuperscriptYOffset;
    FWORD    fwdUnderscoreSize;
    FWORD    fwdUnderscorePosition;
    FWORD    fwdStrikeoutSize;
    FWORD    fwdStrikeoutPosition;
    BYTE     chFirstChar;           // for win 3.1 compatibility
    BYTE     chLastChar;            // for win 3.1 compatibility
    BYTE     chDefaultChar;         // for win 3.1 compatibility
    BYTE     chBreakChar;           // for win 3.1 compatibility
    WCHAR    wcFirstChar;           // lowest supported code in Unicode set
    WCHAR    wcLastChar;            // highest supported code in Unicode set
    WCHAR    wcDefaultChar;
    WCHAR    wcBreakChar;
    POINTL   ptlBaseline;           //
    POINTL   ptlAspect;             // designed aspect ratio (bitmaps)
    POINTL   ptlCaret;              // points along caret
    RECTL    rclFontBox;            // bounding box for all glyphs (font space)
    BYTE     achVendId[4];          // as per TrueType
    ULONG    cKerningPairs;
    ULONG    ulPanoseCulture;
    PANOSE   panose;
} PRINTIFI32, *PPRINTIFI32;


//
// flGenFlags
//

#define UFM_SOFT        0x00000001 // Softfont, thus needs downloading 
#define UFM_CART        0x00000002 // This is a cartridge font
#define UFM_SCALABLE    0x00000004 // Font is scalable

//
// wType
//

#define DF_TYPE_HPINTELLIFONT         0     // HP's Intellifont
#define DF_TYPE_TRUETYPE              1     // HP's PCLETTO fonts on LJ4
#define DF_TYPE_PST1                  2     // Lexmark PPDS scalable fonts
#define DF_TYPE_CAPSL                 3     // Canon CAPSL scalable fonts
#define DF_TYPE_OEM1                  4     // OEM scalable font type 1
#define DF_TYPE_OEM2                  5     // OEM scalable font type 2

//
// fCaps
//

#define DF_NOITALIC             0x0001  // Cannot italicize via FONTSIMULATION
#define DF_NOUNDER              0x0002  // Cannot underline via FONTSIMULATION
#define DF_XM_CR                0x0004  // send CR after using this font
#define DF_NO_BOLD              0x0008  // Cannot bold via FONTSIMULATION
#define DF_NO_DOUBLE_UNDERLINE  0x0010  // Cannot double underline via 
                                        // FONTSIMU ATION
#define DF_NO_STRIKETHRU        0x0020  // Cannot strikethru via FONTSIMULATION
#define DF_BKSP_OK              0x0040  // Can use backspace char, see spec 
                                        // for details

//
// EXTTEXTMETRIC
//
// The EXTTEXTMETRIC structure provides extended-metric information for a font. 
// All the measurements are given in the specified units, 
// regardless of the current mapping mode of the display context.
//

#ifndef _EXTTEXTMETRIC_
#define _EXTTEXTMETRIC_

typedef struct _EXTTEXTMETRIC
    {
    short   emSize;
    short   emPointSize;
    short   emOrientation;
    short   emMasterHeight;
    short   emMinScale;
    short   emMaxScale;
    short   emMasterUnits;
    short   emCapHeight;
    short   emXHeight;
    short   emLowerCaseAscent;
    short   emLowerCaseDescent;
    short   emSlant;
    short   emSuperScript;
    short   emSubScript;
    short   emSuperScriptSize;
    short   emSubScriptSize;
    short   emUnderlineOffset;
    short   emUnderlineWidth;
    short   emDoubleUpperUnderlineOffset;
    short   emDoubleLowerUnderlineOffset;
    short   emDoubleUpperUnderlineWidth;
    short   emDoubleLowerUnderlineWidth;
    short   emStrikeOutOffset;
    short   emStrikeOutWidth;
    WORD    emKernPairs;
    WORD    emKernTracks;
} EXTTEXTMETRIC, *PEXTTEXTMETRIC;

#endif // _EXTTEXTMETRIC_


//
// WIDTHTABLE
//
// This data structure represents the character width table. 
// This width table is a continuous GLYPHHANDLE base, 
// not Unicode nor codepage/character code base. 
// GLYPHANDLE information is in the GLYPHDATA.
//

typedef struct _WIDTHRUN
{
    WORD    wStartGlyph;       // index of the first glyph handle
    WORD    wGlyphCount;       // number of glyphs covered
    DWORD   loCharWidthOffset; // glyph width table
} WIDTHRUN, *PWIDTHRUN;

typedef struct _WIDTHTABLE
{
    DWORD   dwSize;        // the size of this structure including every run
    DWORD   dwRunNum;      // the number of widthrun
    WIDTHRUN WidthRun[1];  // width run array
} WIDTHTABLE, *PWIDTHTABLE;

//
// The array has wGlyphCount elements and each element is the char width 
// for a single glyph. The first width corresponds to glyph index wStartGlyph 
// and so on. The byte offset is relative to the beginning of WIDTHTABLE
// structure and must be WORD-aligned.
// In case of Western device font, proportional font has all varibal pitch
// characters. This means that dwRunNum is set to 1 and loCharWidthOffset
// would be an offset from the top of WIDTHTABLE to a width vector of all 
// characters.
// In case of East Asian device font, basically IFIMETRICS.fwdAveCharWidth and
// IFIMETRICS.fwdMaxCharWidth are used for single byte and double byte character
// width. If a font is proportional, a UFM has a WIDTHTABLE which represents
// only the proportional pitch characters. Other characters use fdwAveCharWidth
// and fwdMaxCharInc for single and double byte characters.
//

//
// KERNDATA
// This data structure represents kerning pair information.
// This kerning pair table is a Unicode base.
//

typedef struct _KERNDATA
{
    DWORD dwSize;               // the size of this structure including array
    DWORD dwKernPairNum;        // the number of kerning pair
    FD_KERNINGPAIR KernPair[1]; // FD_KERNINGPAIR array
} KERNDATA, *PKERNDATA;



//
//
// G L Y P H  S E T D A T A  ( G T T )
//
//

//
// UNI_GLYPHSETDATA
//
// GLYPHSETDATA data structure represents a character encoding information 
// of printer device font. 
//

typedef struct _UNI_GLYPHSETDATA {
        DWORD   dwSize;
        DWORD   dwVersion;
        DWORD   dwFlags;
        LONG    lPredefinedID;
        DWORD   dwGlyphCount;
        DWORD   dwRunCount;
        DWORD   loRunOffset;
        DWORD   dwCodePageCount;
        DWORD   loCodePageOffset;
        DWORD   loMapTableOffset;
        DWORD   dwReserved[2];
} UNI_GLYPHSETDATA, *PUNI_GLYPHSETDATA;

#define UNI_GLYPHSETDATA_VERSION_1_0    0x00010000

#define GET_GLYPHRUN(pGTT)     \
    ((PGLYPHRUN) ((PBYTE)(pGTT) + ((PUNI_GLYPHSETDATA)pGTT)->loRunOffset))
#define GET_CODEPAGEINFO(pGTT) \
    ((PUNI_CODEPAGEINFO) ((PBYTE)(pGTT) + ((PUNI_GLYPHSETDATA)pGTT)->loCodePageOffset))
#define GET_MAPTABLE(pGTT) \
    ((PMAPTABLE) ((PBYTE)(pGTT) + ((PUNI_GLYPHSETDATA)pGTT)->loMapTableOffset))

//
// UNI_CODEPAGEINFO
//
// This UNI_CODEPAGEINFO dats structure has a list of Codepage values 
// which are supported by this UNI_GLYPHSETDATA.
//

typedef struct _UNI_CODEPAGEINFO {
    DWORD dwCodePage;
    INVOC SelectSymbolSet;
    INVOC UnSelectSymbolSet;
} UNI_CODEPAGEINFO, *PUNI_CODEPAGEINFO;

//
// GLYPHRUN
//
// GLYPHRUN dats structure represents the conversion table from Unicode to 
// UNI_GLYPHSETDATA specific glyph handle. Glyph handle is continuous number 
// starting from zero.
//

typedef struct _GLYPHRUN {
    WCHAR   wcLow;
    WORD    wGlyphCount;
} GLYPHRUN, *PGLYPHRUN;


//
// MAPTABLE and TRANSDATA
//
// This MAPTABLE data structure represents a conversion table fron glyph handle
// to codepage/character code.
//

typedef struct _TRANSDATA {
    BYTE  ubCodePageID; // Codepage index to CODEPAGENFO data structure array
    BYTE  ubType;       // a type of TRANSDATA
    union
    {
        SHORT   sCode;
        BYTE    ubCode;
        BYTE    ubPairs[2];
    } uCode;
} TRANSDATA, *PTRANSDATA;

typedef struct _MAPTABLE {
    DWORD     dwSize;     // the size of MAPTABLE including TRANSDATA array
    DWORD     dwGlyphNum; // the number of glyphs supported in MAPTABLE
    TRANSDATA Trans[1];   // an array of TRANSDATA
} MAPTABLE, *PMAPTABLE;

//
// ubType flags
//
// One of following three can be specified for the type of uCode.
//

#define MTYPE_FORMAT_MASK 0x07
#define MTYPE_COMPOSE   0x01 // wCode is an array of 16-bit offsets from the
                             // beginning of the MAPTABLE pointing to the
                             // strings to use for translation.
                             // bData representes thelength of the translated
                             // string.
#define MTYPE_DIRECT    0x02 // wCode is a byte data of one-to-one translation
#define MTYPE_PAIRED    0x04 // wCode contains a word data to emit.

//
// One of following tow can be specified for East Asia multibyte character.
//

#define MTYPE_DOUBLEBYTECHAR_MASK   0x18
#define MTYPE_SINGLE    0x08 // wCode contains a single byte character code in
                             // multi byte character string.
#define MTYPE_DOUBLE    0x10 // wCode contains a double byte character code in
                             // multi byte character string.
//
// One of following three can be specified for replace/add/disable system 
// predefined GTT.
//

#define MTYPE_PREDEFIN_MASK   0xe0
#define MTYPE_REPLACE   0x20 // wCode contains a data to replace predefined one.
#define MTYPE_ADD       0x40 // wCode contains a data to add to predefiend one.
#define MTYPE_DISABLE   0x80 // wCode contains a data to remove from predefined.


//
// System predefined character conversion
//
// UNIDRV is going to support  following system predefined character conversion.
// By speciffying these number in UNIFM.dwGlyphSetDataRCID;
//

#define CC_NOPRECNV 0x0000FFFF // Not use predefined

//
// ANSI
//
#define CC_DEFAULT  0 // Default Character Conversion
#define CC_CP437   -1 // Unicode to IBM Codepage 437
#define CC_CP850   -2 // Unicode to IBM Codepage 850
#define CC_CP863   -3 // Unicode to IBM Codepage 863

//
// East Asia
//

#define CC_BIG5     -10 // Unicode to Chinese Big 5. Codepage 950.
#define CC_ISC      -11 // Unicode to Korean Industrial Standard. Codepage 949.
#define CC_JIS      -12 // Unicode to JIS X0208. Codepage 932.
#define CC_JIS_ANK  -13 // Unicode to JIS X0208 except ANK. Codepage 932.
#define CC_NS86     -14 // Big-5 to National Standstand conversion. Codepage 950
#define CC_TCA      -15 // Big-5 to Taipei Computer Association. Codepage 950.
#define CC_GB2312   -16 // Unicode to GB2312. Codepage 936
#define CC_SJIS     -17 // Unicode to Shift-JIS. Codepage 932.
#define CC_WANSUNG  -18 // Unicode to Extented Wansung. Codepage 949.


//
//
// U N I V E R S A L  F O N T  F O R M A T  ( U F F )
//
//

//
// Font file header
//

typedef struct _UFF_FILEHEADER {
    DWORD       dwSignature;            // File magic number
    DWORD       dwVersion;              // UFF file format version number
    DWORD       dwSize;                 // Size of this structure

    DWORD       nFonts;                 // Count of fonts in directory
    DWORD       nGlyphSets;             // Count of glyph set data
    DWORD       nVarData;               // Count of variable data

    DWORD       offFontDir;             // Offset of font directory
    DWORD       dwFlags;                // Miscellaneous flags
    DWORD       dwReserved[4];          // Reserved, set to zero
} UFF_FILEHEADER, *PUFF_FILEHEADER;

//
// Values used in the file header
//

#define UFF_FILE_MAGIC      '1FFU'
#define UFF_VERSION_NUMBER  0x00010001

#define FONT_DIR_SORTED     0x00000001

//
// Font directory structure
//

typedef struct _UFF_FONTDIRECTORY {
    DWORD       dwSignature;            // Signature of font metrics record
    WORD        wSize;                  // Size of this structure
    WORD        wFontID;                // Unique font ID
    SHORT       sGlyphID;               // Associated glyph ID. 0 is default.
                                        // -ve values are predefined IDs
    WORD        wFlags;                 // Miscellaneous flags
    DWORD       dwInstallerSig;         // Signature of installer that installed this font
    DWORD       offFontName;            // Offset to name of font
    DWORD       offCartridgeName;       // Offset to name of font cartridge
    DWORD       offFontData;            // Offset to font data record
    DWORD       offGlyphData;           // Offset to glyph set data
    DWORD       offVarData;             // Offset to softfont data
} UFF_FONTDIRECTORY, *PUFF_FONTDIRECTORY;

#define FONT_REC_SIG            'CERF'  // font metrics record signature

#define WINNT_INSTALLER_SIG     'IFTN'  // NT font installer

//
// Flags used in font directory
//

#define FONT_FL_UFM             0x0001
#define FONT_FL_IFI             0x0002
#define FONT_FL_SOFTFONT        0x0004
#define FONT_FL_PERMANENT_SF    0x0008
#define FONT_FL_DEVICEFONT      0x0010
#define FONT_FL_GLYPHSET_GTT    0x0020
#define FONT_FL_GLYPHSET_RLE    0x0040
#define FONT_FL_RESERVED        0x8000

//
// Data header
//

typedef struct _DATA_HEADER {
    DWORD       dwSignature;            // Signature of data type
    WORD        wSize;                  // Size of this structure
    WORD        wDataID;                // Identifier number for data
    DWORD       dwDataSize;             // Size of data excluding structure
    DWORD       dwReserved;             // Reserved, set to zero
} DATA_HEADER, *PDATA_HEADER;

//
// Data signatures
//

#define DATA_UFM_SIG        'MFUD'
#define DATA_IFI_SIG        'IFID'
#define DATA_GTT_SIG        'TTGD'
#define DATA_CTT_SIG        'TTCD'
#define DATA_VAR_SIG        'RAVD'

//
// Structure passed to font installer dialog proc through LPARAM
//

typedef struct _OEMFONTINSTPARAM {
    DWORD   cbSize;
    HANDLE  hPrinter;
    HANDLE  hModule;
    HANDLE  hHeap;
    DWORD   dwFlags;
    PWSTR   pFontInstallerName;
} OEMFONTINSTPARAM, *POEMFONTINSTPARAM;

#define FG_CANCHANGE        0x00080    // Have access to change data

#define WM_FI_FILENAME      900        // To get the font installer name.




#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif //_PRNTFONT_H_
