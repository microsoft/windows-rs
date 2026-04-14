

/* this ALWAYS GENERATED file contains the definitions for the interfaces */


 /* File created by MIDL compiler version 8.01.0628 */
/* @@MIDL_FILE_HEADING(  ) */



/* verify that the <rpcndr.h> version is high enough to compile this file*/
#ifndef __REQUIRED_RPCNDR_H_VERSION__
#define __REQUIRED_RPCNDR_H_VERSION__ 501
#endif

/* verify that the <rpcsal.h> version is high enough to compile this file*/
#ifndef __REQUIRED_RPCSAL_H_VERSION__
#define __REQUIRED_RPCSAL_H_VERSION__ 100
#endif

#include "rpc.h"
#include "rpcndr.h"

#ifndef __RPCNDR_H_VERSION__
#error this stub requires an updated version of <rpcndr.h>
#endif /* __RPCNDR_H_VERSION__ */


#ifndef __rectypes_h__
#define __rectypes_h__

#if defined(_MSC_VER) && (_MSC_VER >= 1020)
#pragma once
#endif

#ifndef DECLSPEC_XFGVIRT
#if defined(_CONTROL_FLOW_GUARD_XFG)
#define DECLSPEC_XFGVIRT(base, func) __declspec(xfg_virtual(base, func))
#else
#define DECLSPEC_XFGVIRT(base, func)
#endif
#endif

/* Forward Declarations */ 

/* header files for imported files */
#include "wtypes.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_rectypes_0000_0000 */
/* [local] */ 

#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
#include "RecDefs.h"
#define SAFE_PARTIAL     1
#define BEST_COMPLETE    2
#define MAX_VENDORNAME   32
#define MAX_FRIENDLYNAME 64
#define MAX_LANGUAGES    64
#define CAC_FULL     0
#define CAC_PREFIX   1
#define CAC_RANDOM   2
#define ASYNC_RECO_INTERRUPTED               0x1     //when the process is interrupted
#define ASYNC_RECO_PROCESS_FAILED            0x2
#define ASYNC_RECO_ADDSTROKE_FAILED          0x4
#define ASYNC_RECO_SETCACMODE_FAILED         0x8
#define ASYNC_RECO_RESETCONTEXT_FAILED       0x10
#define ASYNC_RECO_SETGUIDE_FAILED           0x20
#define ASYNC_RECO_SETFLAGS_FAILED           0x40
#define ASYNC_RECO_SETFACTOID_FAILED         0x80
#define ASYNC_RECO_SETTEXTCONTEXT_FAILED     0x100
#define ASYNC_RECO_SETWORDLIST_FAILED        0x200
#define RF_DONTCARE              1L      // overrides all other ones if set
#define RF_OBJECT                2L      // if not set, this is a text recognizer
#define RF_FREE_INPUT            4L      // supports free input
#define RF_LINED_INPUT           8L      // supports simple guide structure with lines only
#define RF_BOXED_INPUT           16L     // supports boxed (guided) input
#define RF_CAC_INPUT             32L     // supports boxed Character Auto Completion
#define RF_RIGHT_AND_DOWN        64L     // used in western and FE languages
#define RF_LEFT_AND_DOWN         128L    // used in Hebrew and Arabic
#define RF_DOWN_AND_LEFT         256L    // used in most FE languages
#define RF_DOWN_AND_RIGHT        512L    // used in Chinese only
#define RF_ARBITRARY_ANGLE       1024L   // can read text written at arbitrary angles (mimio)
#define RF_LATTICE               2048L   // can return lattice in results
#define RF_ADVISEINKCHANGE       4096L   // advise ink change can interrupt process
#define RF_STROKEREORDER         8192L   // indicates that stroke order - spatial and temporal is handled
#define RF_PERSONALIZABLE        16384L  // Supports personalization
//                               32768L is reserved.
#define RF_PERFORMSLINEBREAKING  65536L  // Recognizer prefers to do the line breaking
#define RF_REQUIRESSEGMENTATIONBREAKING 131072L // Recognizer wants only segments of ink
#ifndef __RECOTYPES__
#define __RECOTYPES__
typedef struct tagRECO_GUIDE
    {
    int xOrigin;
    int yOrigin;
    int cxBox;
    int cyBox;
    int cxBase;
    int cyBase;
    int cHorzBox;
    int cVertBox;
    int cyMid;
    } 	RECO_GUIDE;

typedef struct tagRECO_ATTRS
    {
    DWORD dwRecoCapabilityFlags;
    WCHAR awcVendorName[ 32 ];
    WCHAR awcFriendlyName[ 64 ];
    WORD awLanguageId[ 64 ];
    } 	RECO_ATTRS;

typedef struct tagRECO_RANGE
    {
    ULONG iwcBegin;
    ULONG cCount;
    } 	RECO_RANGE;

typedef struct tagLINE_SEGMENT
    {
    POINT PtA;
    POINT PtB;
    } 	LINE_SEGMENT;

typedef struct tagLATTICE_METRICS
    {
    LINE_SEGMENT lsBaseline;
    short iMidlineOffset;
    } 	LATTICE_METRICS;

typedef 
enum enumLINE_METRICS
    {
        LM_BASELINE	= 0,
        LM_MIDLINE	= 1,
        LM_ASCENDER	= 2,
        LM_DESCENDER	= 3
    } 	LINE_METRICS;

typedef 
enum enumCONFIDENCE_LEVEL
    {
        CFL_STRONG	= 0,
        CFL_INTERMEDIATE	= 1,
        CFL_POOR	= 2
    } 	CONFIDENCE_LEVEL;

typedef 
enum enumALT_BREAKS
    {
        ALT_BREAKS_SAME	= 0,
        ALT_BREAKS_UNIQUE	= 1,
        ALT_BREAKS_FULL	= 2
    } 	ALT_BREAKS;

typedef 
enum enumRECO_TYPE
    {
        RECO_TYPE_WSTRING	= 0,
        RECO_TYPE_WCHAR	= 1
    } 	RECO_TYPE;

typedef struct tagRECO_LATTICE_PROPERTY
    {
    GUID guidProperty;
    USHORT cbPropertyValue;
    /* [size_is][unique] */ BYTE *pPropertyValue;
    } 	RECO_LATTICE_PROPERTY;

typedef struct tagRECO_LATTICE_PROPERTIES
    {
    ULONG cProperties;
    /* [size_is][size_is][unique] */ RECO_LATTICE_PROPERTY **apProps;
    } 	RECO_LATTICE_PROPERTIES;

typedef int RECO_SCORE;

typedef struct tagRECO_LATTICE_ELEMENT
    {
    RECO_SCORE score;
    WORD type;
    BYTE *pData;
    ULONG ulNextColumn;
    ULONG ulStrokeNumber;
    RECO_LATTICE_PROPERTIES epProp;
    } 	RECO_LATTICE_ELEMENT;

typedef struct tagRECO_LATTICE_COLUMN
    {
    ULONG key;
    RECO_LATTICE_PROPERTIES cpProp;
    ULONG cStrokes;
    ULONG *pStrokes;
    ULONG cLatticeElements;
    RECO_LATTICE_ELEMENT *pLatticeElements;
    } 	RECO_LATTICE_COLUMN;

typedef struct tagRECO_LATTICE
    {
    ULONG ulColumnCount;
    RECO_LATTICE_COLUMN *pLatticeColumns;
    ULONG ulPropertyCount;
    GUID *pGuidProperties;
    ULONG ulBestResultColumnCount;
    ULONG *pulBestResultColumns;
    ULONG *pulBestResultIndexes;
    } 	RECO_LATTICE;

typedef struct tagCHARACTER_RANGE
    {
    WCHAR wcLow;
    USHORT cChars;
    } 	CHARACTER_RANGE;

typedef struct tagCHARACTER_RANGE *PCHARACTER_RANGE;

#endif
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion
#pragma region APP Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP)
#include "RecDefs.h"
#define SAFE_PARTIAL     1
#define BEST_COMPLETE    2
#define MAX_VENDORNAME   32
#define MAX_FRIENDLYNAME 64
#define MAX_LANGUAGES    64
#define CAC_FULL     0
#define CAC_PREFIX   1
#define CAC_RANDOM   2
#define ASYNC_RECO_INTERRUPTED               0x1     //when the process is interrupted
#define ASYNC_RECO_PROCESS_FAILED            0x2
#define ASYNC_RECO_ADDSTROKE_FAILED          0x4
#define ASYNC_RECO_SETCACMODE_FAILED         0x8
#define ASYNC_RECO_RESETCONTEXT_FAILED       0x10
#define ASYNC_RECO_SETGUIDE_FAILED           0x20
#define ASYNC_RECO_SETFLAGS_FAILED           0x40
#define ASYNC_RECO_SETFACTOID_FAILED         0x80
#define ASYNC_RECO_SETTEXTCONTEXT_FAILED     0x100
#define ASYNC_RECO_SETWORDLIST_FAILED        0x200
#define RF_DONTCARE              1L      // overrides all other ones if set
#define RF_OBJECT                2L      // if not set, this is a text recognizer
#define RF_FREE_INPUT            4L      // supports free input
#define RF_LINED_INPUT           8L      // supports simple guide structure with lines only
#define RF_BOXED_INPUT           16L     // supports boxed (guided) input
#define RF_CAC_INPUT             32L     // supports boxed Character Auto Completion
#define RF_RIGHT_AND_DOWN        64L     // used in western and FE languages
#define RF_LEFT_AND_DOWN         128L    // used in Hebrew and Arabic
#define RF_DOWN_AND_LEFT         256L    // used in most FE languages
#define RF_DOWN_AND_RIGHT        512L    // used in Chinese only
#define RF_ARBITRARY_ANGLE       1024L   // can read text written at arbitrary angles (mimio)
#define RF_LATTICE               2048L   // can return lattice in results
#define RF_ADVISEINKCHANGE       4096L   // advise ink change can interrupt process
#define RF_STROKEREORDER         8192L   // indicates that stroke order - spatial and temporal is handled
#define RF_PERSONALIZABLE        16384L  // Supports personalization
//                               32768L is reserved.
#define RF_PERFORMSLINEBREAKING  65536L  // Recognizer prefers to do the line breaking
#define RF_REQUIRESSEGMENTATIONBREAKING 131072L // Recognizer wants only segments of ink
#ifndef __RECOTYPES__
#define __RECOTYPES__
typedef struct tagRECO_GUIDE         // 1.0 guide structure
{
   int xOrigin;                 // left edge of first box (screen coord))
   int yOrigin;                 // ditto top edge
   int cxBox;                   // width of a single box
   int cyBox;                   // ditto height
   int cxBase;                  // in-box x-margin to guideline
   int cyBase;                  // in-box y offset from top to baseline
   int cHorzBox;                // count of boxed columns
   int cVertBox;                // ditto rows
   int cyMid;                   // 0 or distance from baseline to midline
}  RECO_GUIDE;
typedef struct tagRECO_ATTRS
{
    DWORD    dwRecoCapabilityFlags;     // text vs. object, etc
    WCHAR    awcVendorName[32];      
    WCHAR    awcFriendlyName[64];  
    WORD    awLanguageId[64];      // supported languages
} RECO_ATTRS; 
typedef struct    tagRECO_RANGE 
{
    ULONG iwcBegin; 
    ULONG cCount; 
} RECO_RANGE; 
typedef struct tagLINE_SEGMENT
{
    POINT    PtA;     // one of the end points of the line segment
    POINT    PtB;     // the other end point of the line segment
} LINE_SEGMENT;
// This type is used to persist the metrics
// in a more compact format in the lattice
typedef struct tagLATTICE_METRICS
{
    LINE_SEGMENT lsBaseline;
    short iMidlineOffset;
} LATTICE_METRICS;
typedef enum enumLINE_METRICS
{
    LM_BASELINE    = 0,     // get the position of the baseline
    LM_MIDLINE     = 1,     // get the position of the midline
    LM_ASCENDER    = 2,     // get the position of the ascender
    LM_DESCENDER   = 3,     // get the position of the descender
} LINE_METRICS;
typedef enum enumCONFIDENCE_LEVEL
{
        CFL_STRONG = 0,
        CFL_INTERMEDIATE = 1,
        CFL_POOR = 2,
} CONFIDENCE_LEVEL;
typedef enum enumALT_BREAKS
{
        ALT_BREAKS_SAME = 0,
        ALT_BREAKS_UNIQUE = 1,
        ALT_BREAKS_FULL = 2,
} ALT_BREAKS;
//
// Definition of the lattice structure
//
typedef enum enumRECO_TYPE
{
    RECO_TYPE_WSTRING = 0,
    RECO_TYPE_WCHAR = 1,
} RECO_TYPE;
typedef struct tagRECO_LATTICE_PROPERTY
{
    GUID     guidProperty;    // guid that identifies the property
    USHORT  cbPropertyValue; // number of bytes of the property value
    [unique, size_is(cbPropertyValue)] BYTE     *pPropertyValue;    // points to data, depending on guid, it may begin with size 
} RECO_LATTICE_PROPERTY;
typedef struct tagRECO_LATTICE_PROPERTIES
{
    ULONG                 cProperties;    // count of properties
    [unique, size_is(,cProperties)] RECO_LATTICE_PROPERTY    **apProps;        // array of property structures
} RECO_LATTICE_PROPERTIES;
typedef int RECO_SCORE;
typedef struct tagRECO_LATTICE_ELEMENT
{
    RECO_SCORE    score;            // Shape probability of this element
    WORD         type;            //  what kind of data (char, string, COM object)
    BYTE        *pData;            // string (or whatever) recognized
    ULONG        ulNextColumn;        // following column key
    ULONG        ulStrokeNumber;    // number of strokes used by this alternate
    RECO_LATTICE_PROPERTIES epProp;    // property of the lattice element
} RECO_LATTICE_ELEMENT;
typedef struct tagRECO_LATTICE_COLUMN
{
    ULONG                 key;        // key of this column
    RECO_LATTICE_PROPERTIES    cpProp;    // Column property
    ULONG                 cStrokes;    // number of strokes used in this column
    ULONG                 *pStrokes;    // array of stroke numbers used in this column
    ULONG                 cLatticeElements;    // number of lattice elements in this column
    RECO_LATTICE_ELEMENT  *pLatticeElements;    // array of lattice elements    
} RECO_LATTICE_COLUMN;
typedef struct tagRECO_LATTICE
{
    ULONG             ulColumnCount;    // number of columns used in the lattice
    RECO_LATTICE_COLUMN    *pLatticeColumns; // array of lattice columns
    ULONG            ulPropertyCount; // number of properties that can be queried
    GUID            *pGuidProperties; // array of all properties
    ULONG            ulBestResultColumnCount;
    ULONG            *pulBestResultColumns;
    ULONG            *pulBestResultIndexes;
} RECO_LATTICE;
typedef struct tagCHARACTER_RANGE
{
    WCHAR        wcLow;
    USHORT       cChars;
} CHARACTER_RANGE, *PCHARACTER_RANGE;
#endif
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_rectypes_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_rectypes_0000_0000_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


