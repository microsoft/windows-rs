

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


#ifndef __wtypes_h__
#define __wtypes_h__

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
#include "wtypesbase.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_wtypes_0000_0000 */
/* [local] */ 

//+-------------------------------------------------------------------------
//
//  Microsoft Windows
//  Copyright (c) Microsoft Corporation. All rights reserved.
//
//--------------------------------------------------------------------------
#if ( _MSC_VER >= 1020 )
#pragma once
#endif
#if _MSC_VER >= 1200
#pragma warning(push)
#pragma warning(disable:4820) /* padding added after data member */
#endif


extern RPC_IF_HANDLE __MIDL_itf_wtypes_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_wtypes_0000_0000_v0_0_s_ifspec;

#ifndef __IWinTypes_INTERFACE_DEFINED__
#define __IWinTypes_INTERFACE_DEFINED__

/* interface IWinTypes */
/* [unique][version][uuid] */ 

typedef struct tagRemHGLOBAL
    {
    LONG fNullHGlobal;
    ULONG cbData;
    /* [size_is] */ byte data[ 1 ];
    } 	RemHGLOBAL;

typedef struct tagRemHMETAFILEPICT
    {
    LONG mm;
    LONG xExt;
    LONG yExt;
    ULONG cbData;
    /* [size_is] */ byte data[ 1 ];
    } 	RemHMETAFILEPICT;

typedef struct tagRemHENHMETAFILE
    {
    ULONG cbData;
    /* [size_is] */ byte data[ 1 ];
    } 	RemHENHMETAFILE;

typedef struct tagRemHBITMAP
    {
    ULONG cbData;
    /* [size_is] */ byte data[ 1 ];
    } 	RemHBITMAP;

typedef struct tagRemHPALETTE
    {
    ULONG cbData;
    /* [size_is] */ byte data[ 1 ];
    } 	RemHPALETTE;

typedef struct tagRemBRUSH
    {
    ULONG cbData;
    /* [size_is] */ byte data[ 1 ];
    } 	RemHBRUSH;

#if !defined(_WIN32) && !defined(_MPPC_)
// The following code is for Win16 only
#ifndef WINAPI          // If not included with 3.1 headers...
#define FAR             _far
#define PASCAL          _pascal
#define CDECL           _cdecl
#define VOID            void
#define WINAPI      FAR PASCAL
#define CALLBACK    FAR PASCAL
#ifndef FALSE
#define FALSE 0
#define TRUE 1
#endif // !FALSE
#ifndef _BYTE_DEFINED
#define _BYTE_DEFINED
typedef byte BYTE;
#endif // !_BYTE_DEFINED
#ifndef _WORD_DEFINED
#define _WORD_DEFINED
typedef unsigned short WORD;
#endif // !_WORD_DEFINED
typedef unsigned int UINT;
typedef int  INT;
typedef long BOOL;
#ifndef _LONG_DEFINED
#define _LONG_DEFINED
typedef long LONG;
#endif // !_LONG_DEFINED
#ifndef _WPARAM_DEFINED
#define _WPARAM_DEFINED
typedef UINT_PTR WPARAM;

#endif // _WPARAM_DEFINED
#ifndef _DWORD_DEFINED
#define _DWORD_DEFINED
typedef unsigned long DWORD;
#endif // !_DWORD_DEFINED
#ifndef _LPARAM_DEFINED
#define _LPARAM_DEFINED
typedef LONG_PTR LPARAM;

#endif // !_LPARAM_DEFINED
#ifndef _LRESULT_DEFINED
#define _LRESULT_DEFINED
typedef LONG_PTR LRESULT;

#endif // !_LRESULT_DEFINED
typedef void * HANDLE;
typedef void *HMODULE;

typedef void *HINSTANCE;

typedef void *HTASK;

typedef void *HKEY;

typedef void *HDESK;

typedef void *HMF;

typedef void *HEMF;

typedef void *HPEN;

typedef void *HRSRC;

typedef void *HSTR;

typedef void *HWINSTA;

typedef void *HKL;

typedef void *HGDIOBJ;

typedef HANDLE HDWP;

#ifndef _HFILE_DEFINED
#define _HFILE_DEFINED
typedef INT HFILE;

#endif // !_HFILE_DEFINED
#ifndef _LPWORD_DEFINED
#define _LPWORD_DEFINED
typedef WORD *LPWORD;
#endif // !_LPWORD_DEFINED
#ifndef _LPDWORD_DEFINED
#define _LPDWORD_DEFINED
typedef DWORD *LPDWORD;
#endif // !_LPDWORD_DEFINED
typedef char CHAR;
typedef CHAR *LPSTR;
typedef const CHAR *LPCSTR;
#ifndef _WCHAR_DEFINED
#define _WCHAR_DEFINED
typedef wchar_t WCHAR;
typedef WCHAR   TCHAR;
#endif // !_WCHAR_DEFINED
typedef WCHAR *LPWSTR;
typedef TCHAR *LPTSTR;
typedef const WCHAR *LPCWSTR;
typedef const TCHAR *LPCTSTR;
#ifndef _COLORREF_DEFINED
#define _COLORREF_DEFINED
typedef DWORD COLORREF;

#endif // !_COLORREF_DEFINED
#ifndef _LPCOLORREF_DEFINED
#define _LPCOLORREF_DEFINED
typedef DWORD *LPCOLORREF;

#endif // !_LPCOLORREF_DEFINED
typedef HANDLE *LPHANDLE;
typedef struct _RECTL
    {
    LONG left;
    LONG top;
    LONG right;
    LONG bottom;
    } 	RECTL;

typedef struct _RECTL *PRECTL;

typedef struct _RECTL *LPRECTL;

typedef struct tagPOINT
    {
    LONG x;
    LONG y;
    } 	POINT;

typedef struct tagPOINT *PPOINT;

typedef struct tagPOINT *LPPOINT;

typedef struct _POINTL
    {
    LONG x;
    LONG y;
    } 	POINTL;

typedef struct _POINTL *PPOINTL;

#ifndef WIN16
typedef struct tagSIZE
    {
    LONG cx;
    LONG cy;
    } 	SIZE;

typedef struct tagSIZE *PSIZE;

typedef struct tagSIZE *LPSIZE;

#else // WIN16
typedef struct tagSIZE
{
    INT cx;
    INT cy;
} SIZE, *PSIZE, *LPSIZE;
#endif // WIN16
typedef struct tagSIZEL
    {
    LONG cx;
    LONG cy;
    } 	SIZEL;

typedef struct tagSIZEL *PSIZEL;

typedef struct tagSIZEL *LPSIZEL;

#endif  //WINAPI
#endif  //!WIN32 && !MPPC
#ifndef _PALETTEENTRY_DEFINED
#define _PALETTEENTRY_DEFINED
typedef struct tagPALETTEENTRY
    {
    BYTE peRed;
    BYTE peGreen;
    BYTE peBlue;
    BYTE peFlags;
    } 	PALETTEENTRY;

typedef struct tagPALETTEENTRY *PPALETTEENTRY;

typedef struct tagPALETTEENTRY *LPPALETTEENTRY;

#endif // !_PALETTEENTRY_DEFINED
#ifndef _LOGPALETTE_DEFINED
#define _LOGPALETTE_DEFINED
typedef struct tagLOGPALETTE
    {
    WORD palVersion;
    WORD palNumEntries;
    /* [size_is] */ PALETTEENTRY palPalEntry[ 1 ];
    } 	LOGPALETTE;

typedef struct tagLOGPALETTE *PLOGPALETTE;

typedef struct tagLOGPALETTE *LPLOGPALETTE;

#endif // !_LOGPALETTE_DEFINED
#ifndef _WINDEF_
typedef const RECTL *LPCRECTL;

typedef struct tagRECT
    {
    LONG left;
    LONG top;
    LONG right;
    LONG bottom;
    } 	RECT;

typedef struct tagRECT *PRECT;

typedef struct tagRECT *LPRECT;

typedef const RECT *LPCRECT;

#endif  //_WINDEF_
#if 0
typedef FMTID *REFFMTID;

#endif // 0
#ifndef _ROTFLAGS_DEFINED
#define _ROTFLAGS_DEFINED
#define ROTFLAGS_REGISTRATIONKEEPSALIVE 0x1
#define ROTFLAGS_ALLOWANYCLIENT 0x2
#endif // !_ROTFLAGS_DEFINED
#ifndef _ROT_COMPARE_MAX_DEFINED
#define _ROT_COMPARE_MAX_DEFINED
#define ROT_COMPARE_MAX 2048
#endif // !_ROT_COMPARE_MAX_DEFINED
typedef 
enum tagDVASPECT
    {
        DVASPECT_CONTENT	= 1,
        DVASPECT_THUMBNAIL	= 2,
        DVASPECT_ICON	= 4,
        DVASPECT_DOCPRINT	= 8
    } 	DVASPECT;

typedef 
enum tagSTGC
    {
        STGC_DEFAULT	= 0,
        STGC_OVERWRITE	= 1,
        STGC_ONLYIFCURRENT	= 2,
        STGC_DANGEROUSLYCOMMITMERELYTODISKCACHE	= 4,
        STGC_CONSOLIDATE	= 8
    } 	STGC;

typedef 
enum tagSTGMOVE
    {
        STGMOVE_MOVE	= 0,
        STGMOVE_COPY	= 1,
        STGMOVE_SHALLOWCOPY	= 2
    } 	STGMOVE;

typedef 
enum tagSTATFLAG
    {
        STATFLAG_DEFAULT	= 0,
        STATFLAG_NONAME	= 1,
        STATFLAG_NOOPEN	= 2
    } 	STATFLAG;

typedef /* [context_handle] */ void *HCONTEXT;

#ifndef _LCID_DEFINED
#define _LCID_DEFINED
typedef DWORD LCID;

#endif // !_LCID_DEFINED
#ifndef _LANGID_DEFINED
#define _LANGID_DEFINED
typedef USHORT LANGID;

#endif // !_LANGID_DEFINED
#define	WDT_INPROC_CALL	( 0x48746457 )

#define	WDT_REMOTE_CALL	( 0x52746457 )

#define	WDT_INPROC64_CALL	( 0x50746457 )

typedef struct _userCLIPFORMAT
    {
    LONG fContext;
    /* [switch_is] */ /* [switch_type] */ union __MIDL_IWinTypes_0001
        {
        /* [case()] */ DWORD dwValue;
        /* [case()][string] */ wchar_t *pwszName;
        } 	u;
    } 	userCLIPFORMAT;

typedef /* [unique] */  __RPC_unique_pointer userCLIPFORMAT *wireCLIPFORMAT;

typedef /* [wire_marshal] */ WORD CLIPFORMAT;

typedef struct _GDI_NONREMOTE
    {
    LONG fContext;
    /* [switch_is] */ /* [switch_type] */ union __MIDL_IWinTypes_0002
        {
        /* [case()] */ LONG hInproc;
        /* [case()] */ DWORD_BLOB *hRemote;
        } 	u;
    } 	GDI_NONREMOTE;

typedef struct _userHGLOBAL
    {
    LONG fContext;
    /* [switch_is] */ /* [switch_type] */ union __MIDL_IWinTypes_0003
        {
        /* [case()] */ LONG hInproc;
        /* [case()] */ FLAGGED_BYTE_BLOB *hRemote;
        /* [case()] */ __int64 hInproc64;
        } 	u;
    } 	userHGLOBAL;

typedef /* [unique] */  __RPC_unique_pointer userHGLOBAL *wireHGLOBAL;

typedef struct _userHMETAFILE
    {
    LONG fContext;
    /* [switch_is] */ /* [switch_type] */ union __MIDL_IWinTypes_0004
        {
        /* [case()] */ LONG hInproc;
        /* [case()] */ BYTE_BLOB *hRemote;
        /* [case()] */ __int64 hInproc64;
        } 	u;
    } 	userHMETAFILE;

typedef struct _remoteMETAFILEPICT
    {
    LONG mm;
    LONG xExt;
    LONG yExt;
    userHMETAFILE *hMF;
    } 	remoteMETAFILEPICT;

typedef struct _userHMETAFILEPICT
    {
    LONG fContext;
    /* [switch_is] */ /* [switch_type] */ union __MIDL_IWinTypes_0005
        {
        /* [case()] */ LONG hInproc;
        /* [case()] */ remoteMETAFILEPICT *hRemote;
        /* [case()] */ __int64 hInproc64;
        } 	u;
    } 	userHMETAFILEPICT;

typedef struct _userHENHMETAFILE
    {
    LONG fContext;
    /* [switch_is] */ /* [switch_type] */ union __MIDL_IWinTypes_0006
        {
        /* [case()] */ LONG hInproc;
        /* [case()] */ BYTE_BLOB *hRemote;
        /* [case()] */ __int64 hInproc64;
        } 	u;
    } 	userHENHMETAFILE;

typedef struct _userBITMAP
    {
    LONG bmType;
    LONG bmWidth;
    LONG bmHeight;
    LONG bmWidthBytes;
    WORD bmPlanes;
    WORD bmBitsPixel;
    ULONG cbSize;
    /* [size_is] */ byte pBuffer[ 1 ];
    } 	userBITMAP;

typedef struct _userHBITMAP
    {
    LONG fContext;
    /* [switch_is] */ /* [switch_type] */ union __MIDL_IWinTypes_0007
        {
        /* [case()] */ LONG hInproc;
        /* [case()] */ userBITMAP *hRemote;
        /* [case()] */ __int64 hInproc64;
        } 	u;
    } 	userHBITMAP;

typedef struct _userHPALETTE
    {
    LONG fContext;
    /* [switch_is] */ /* [switch_type] */ union __MIDL_IWinTypes_0008
        {
        /* [case()] */ LONG hInproc;
        /* [case()] */ LOGPALETTE *hRemote;
        /* [case()] */ __int64 hInproc64;
        } 	u;
    } 	userHPALETTE;

typedef struct _RemotableHandle
    {
    LONG fContext;
    /* [switch_is] */ /* [switch_type] */ union __MIDL_IWinTypes_0009
        {
        /* [case()] */ LONG hInproc;
        /* [case()] */ LONG hRemote;
        } 	u;
    } 	RemotableHandle;

typedef /* [unique] */  __RPC_unique_pointer RemotableHandle *wireHWND;

typedef /* [unique] */  __RPC_unique_pointer RemotableHandle *wireHMENU;

typedef /* [unique] */  __RPC_unique_pointer RemotableHandle *wireHACCEL;

typedef /* [unique] */  __RPC_unique_pointer RemotableHandle *wireHBRUSH;

typedef /* [unique] */  __RPC_unique_pointer RemotableHandle *wireHFONT;

typedef /* [unique] */  __RPC_unique_pointer RemotableHandle *wireHDC;

typedef /* [unique] */  __RPC_unique_pointer RemotableHandle *wireHICON;

typedef /* [unique] */  __RPC_unique_pointer RemotableHandle *wireHRGN;

typedef /* [unique] */  __RPC_unique_pointer RemotableHandle *wireHMONITOR;

#if 0
typedef /* [wire_marshal] */ void *HWND;

typedef /* [wire_marshal] */ void *HMENU;

typedef /* [wire_marshal] */ void *HACCEL;

typedef /* [wire_marshal] */ void *HBRUSH;

typedef /* [wire_marshal] */ void *HFONT;

typedef /* [wire_marshal] */ void *HDC;

typedef /* [wire_marshal] */ void *HICON;

typedef /* [wire_marshal] */ void *HRGN;

typedef /* [wire_marshal] */ void *HMONITOR;

#ifndef _HCURSOR_DEFINED
#define _HCURSOR_DEFINED
typedef HICON HCURSOR;

#endif // !_HCURSOR_DEFINED
#endif //0
#ifndef _TEXTMETRIC_DEFINED
#define _TEXTMETRIC_DEFINED
typedef struct tagTEXTMETRICW
    {
    LONG tmHeight;
    LONG tmAscent;
    LONG tmDescent;
    LONG tmInternalLeading;
    LONG tmExternalLeading;
    LONG tmAveCharWidth;
    LONG tmMaxCharWidth;
    LONG tmWeight;
    LONG tmOverhang;
    LONG tmDigitizedAspectX;
    LONG tmDigitizedAspectY;
    WCHAR tmFirstChar;
    WCHAR tmLastChar;
    WCHAR tmDefaultChar;
    WCHAR tmBreakChar;
    BYTE tmItalic;
    BYTE tmUnderlined;
    BYTE tmStruckOut;
    BYTE tmPitchAndFamily;
    BYTE tmCharSet;
    } 	TEXTMETRICW;

typedef struct tagTEXTMETRICW *PTEXTMETRICW;

typedef struct tagTEXTMETRICW *LPTEXTMETRICW;

#endif // !_TEXTMETRIC_DEFINED
#ifndef _WIN32           // The following code is for Win16 only
#ifndef WINAPI          // If not included with 3.1 headers...
typedef struct tagMSG
    {
    HWND hwnd;
    UINT message;
    WPARAM wParam;
    LPARAM lParam;
    DWORD time;
    POINT pt;
    } 	MSG;

typedef struct tagMSG *PMSG;

typedef struct tagMSG *NPMSG;

typedef struct tagMSG *LPMSG;

#endif // _WIN32
#endif // WINAPI
typedef /* [unique] */  __RPC_unique_pointer userHBITMAP *wireHBITMAP;

typedef /* [unique] */  __RPC_unique_pointer userHPALETTE *wireHPALETTE;

typedef /* [unique] */  __RPC_unique_pointer userHENHMETAFILE *wireHENHMETAFILE;

typedef /* [unique] */  __RPC_unique_pointer userHMETAFILE *wireHMETAFILE;

typedef /* [unique] */  __RPC_unique_pointer userHMETAFILEPICT *wireHMETAFILEPICT;

#if 0
typedef /* [wire_marshal] */ void *HGLOBAL;

typedef HGLOBAL HLOCAL;

typedef /* [wire_marshal] */ void *HBITMAP;

typedef /* [wire_marshal] */ void *HPALETTE;

typedef /* [wire_marshal] */ void *HENHMETAFILE;

typedef /* [wire_marshal] */ void *HMETAFILE;

#endif //0
typedef /* [wire_marshal] */ void *HMETAFILEPICT;



extern RPC_IF_HANDLE IWinTypes_v0_1_c_ifspec;
extern RPC_IF_HANDLE IWinTypes_v0_1_s_ifspec;
#endif /* __IWinTypes_INTERFACE_DEFINED__ */

/* interface __MIDL_itf_wtypes_0000_0001 */
/* [local] */ 

#if ( _MSC_VER >= 800 )
#if _MSC_VER >= 1200
#pragma warning(push)
#endif
#pragma warning(disable:4201)
#endif
typedef double DATE;

#ifndef _tagCY_DEFINED
#define _tagCY_DEFINED
#define _CY_DEFINED
#if 0
/* the following isn't the real definition of CY, but it is */
/* what RPC knows how to remote */
typedef struct tagCY
    {
    LONGLONG int64;
    } 	CY;

#else /* 0 */
/* real definition that makes the C++ compiler happy */
typedef union tagCY {
    struct {
        ULONG Lo;
        LONG      Hi;
    } DUMMYSTRUCTNAME;
    LONGLONG int64;
} CY;
#endif /* 0 */
#endif /* _tagCY_DEFINED */
typedef CY *LPCY;

#if 0 /* _tagDEC_DEFINED */
/* The following isn't the real definition of Decimal type, */
/* but it is what RPC knows how to remote */
typedef struct tagDEC
    {
    USHORT wReserved;
    BYTE scale;
    BYTE sign;
    ULONG Hi32;
    ULONGLONG Lo64;
    } 	DECIMAL;

#else /* _tagDEC_DEFINED */
/* real definition that makes the C++ compiler happy */
typedef struct tagDEC {
    USHORT wReserved;
    union {
        struct {
            BYTE scale;
            BYTE sign;
        } DUMMYSTRUCTNAME;
        USHORT signscale;
    } DUMMYUNIONNAME;
    ULONG Hi32;
    union {
        struct {
            ULONG Lo32;
            ULONG Mid32;
        } DUMMYSTRUCTNAME2;
        ULONGLONG Lo64;
    } DUMMYUNIONNAME2;
} DECIMAL;
#define DECIMAL_NEG ((BYTE)0x80)
#define DECIMAL_SETZERO(dec) \
        {(dec).Lo64 = 0; (dec).Hi32 = 0; (dec).signscale = 0;}
#endif /* _tagDEC_DEFINED */
typedef DECIMAL *LPDECIMAL;

#if ( _MSC_VER >= 800 )
#if _MSC_VER >= 1200
#pragma warning(pop)
#else
#pragma warning(default:4201)
#endif
#endif
typedef /* [unique] */  __RPC_unique_pointer FLAGGED_WORD_BLOB *wireBSTR;

#ifndef _PREFAST_
typedef /* [wire_marshal] */ OLECHAR *BSTR;

#else // _PREFAST_
typedef _Null_terminated_ OLECHAR *BSTR;
#endif
typedef BSTR *LPBSTR;

/* 0 == FALSE, -1 == TRUE */
typedef short VARIANT_BOOL;

/* The BSTRBLOB structure is used by some implementations */
/* of the IPropertyStorage interface when marshaling BSTRs */
/* on systems which don't support BSTR marshaling. */
#ifndef _tagBSTRBLOB_DEFINED
#define _tagBSTRBLOB_DEFINED
typedef struct tagBSTRBLOB
    {
    ULONG cbSize;
    /* [size_is] */ BYTE *pData;
    } 	BSTRBLOB;

typedef struct tagBSTRBLOB *LPBSTRBLOB;

#endif
#define VARIANT_TRUE ((VARIANT_BOOL)-1)
#define VARIANT_FALSE ((VARIANT_BOOL)0)
typedef struct tagCLIPDATA
    {
    ULONG cbSize;
    LONG ulClipFmt;
    /* [size_is] */ BYTE *pClipData;
    } 	CLIPDATA;

// Macro to calculate the size of the above pClipData
#define CBPCLIPDATA(clipdata)    ( (clipdata).cbSize - sizeof((clipdata).ulClipFmt) )
typedef unsigned short VARTYPE;

/*
 * VARENUM usage key,
 *
 * * [V] - may appear in a VARIANT
 * * [T] - may appear in a TYPEDESC
 * * [P] - may appear in an OLE property set
 * * [S] - may appear in a Safe Array
 *
 *
 *  VT_EMPTY            [V]   [P]     nothing
 *  VT_NULL             [V]   [P]     SQL style Null
 *  VT_I2               [V][T][P][S]  2 byte signed int
 *  VT_I4               [V][T][P][S]  4 byte signed int
 *  VT_R4               [V][T][P][S]  4 byte real
 *  VT_R8               [V][T][P][S]  8 byte real
 *  VT_CY               [V][T][P][S]  currency
 *  VT_DATE             [V][T][P][S]  date
 *  VT_BSTR             [V][T][P][S]  OLE Automation string
 *  VT_DISPATCH         [V][T]   [S]  IDispatch *
 *  VT_ERROR            [V][T][P][S]  SCODE
 *  VT_BOOL             [V][T][P][S]  True=-1, False=0
 *  VT_VARIANT          [V][T][P][S]  VARIANT *
 *  VT_UNKNOWN          [V][T]   [S]  IUnknown *
 *  VT_DECIMAL          [V][T]   [S]  16 byte fixed point
 *  VT_RECORD           [V]   [P][S]  user defined type
 *  VT_I1               [V][T][P][s]  signed char
 *  VT_UI1              [V][T][P][S]  unsigned char
 *  VT_UI2              [V][T][P][S]  unsigned short
 *  VT_UI4              [V][T][P][S]  ULONG
 *  VT_I8                  [T][P]     signed 64-bit int
 *  VT_UI8                 [T][P]     unsigned 64-bit int
 *  VT_INT              [V][T][P][S]  signed machine int
 *  VT_UINT             [V][T]   [S]  unsigned machine int
 *  VT_INT_PTR             [T]        signed machine register size width
 *  VT_UINT_PTR            [T]        unsigned machine register size width
 *  VT_VOID                [T]        C style void
 *  VT_HRESULT             [T]        Standard return type
 *  VT_PTR                 [T]        pointer type
 *  VT_SAFEARRAY           [T]        (use VT_ARRAY in VARIANT)
 *  VT_CARRAY              [T]        C style array
 *  VT_USERDEFINED         [T]        user defined type
 *  VT_LPSTR               [T][P]     null terminated string
 *  VT_LPWSTR              [T][P]     wide null terminated string
 *  VT_FILETIME               [P]     FILETIME
 *  VT_BLOB                   [P]     Length prefixed bytes
 *  VT_STREAM                 [P]     Name of the stream follows
 *  VT_STORAGE                [P]     Name of the storage follows
 *  VT_STREAMED_OBJECT        [P]     Stream contains an object
 *  VT_STORED_OBJECT          [P]     Storage contains an object
 *  VT_VERSIONED_STREAM       [P]     Stream with a GUID version
 *  VT_BLOB_OBJECT            [P]     Blob contains an object 
 *  VT_CF                     [P]     Clipboard format
 *  VT_CLSID                  [P]     A Class ID
 *  VT_VECTOR                 [P]     simple counted array
 *  VT_ARRAY            [V]           SAFEARRAY*
 *  VT_BYREF            [V]           void* for local use
 *  VT_BSTR_BLOB                      Reserved for system use
 */

enum VARENUM
    {
        VT_EMPTY	= 0,
        VT_NULL	= 1,
        VT_I2	= 2,
        VT_I4	= 3,
        VT_R4	= 4,
        VT_R8	= 5,
        VT_CY	= 6,
        VT_DATE	= 7,
        VT_BSTR	= 8,
        VT_DISPATCH	= 9,
        VT_ERROR	= 10,
        VT_BOOL	= 11,
        VT_VARIANT	= 12,
        VT_UNKNOWN	= 13,
        VT_DECIMAL	= 14,
        VT_I1	= 16,
        VT_UI1	= 17,
        VT_UI2	= 18,
        VT_UI4	= 19,
        VT_I8	= 20,
        VT_UI8	= 21,
        VT_INT	= 22,
        VT_UINT	= 23,
        VT_VOID	= 24,
        VT_HRESULT	= 25,
        VT_PTR	= 26,
        VT_SAFEARRAY	= 27,
        VT_CARRAY	= 28,
        VT_USERDEFINED	= 29,
        VT_LPSTR	= 30,
        VT_LPWSTR	= 31,
        VT_RECORD	= 36,
        VT_INT_PTR	= 37,
        VT_UINT_PTR	= 38,
        VT_FILETIME	= 64,
        VT_BLOB	= 65,
        VT_STREAM	= 66,
        VT_STORAGE	= 67,
        VT_STREAMED_OBJECT	= 68,
        VT_STORED_OBJECT	= 69,
        VT_BLOB_OBJECT	= 70,
        VT_CF	= 71,
        VT_CLSID	= 72,
        VT_VERSIONED_STREAM	= 73,
        VT_BSTR_BLOB	= 0xfff,
        VT_VECTOR	= 0x1000,
        VT_ARRAY	= 0x2000,
        VT_BYREF	= 0x4000,
        VT_RESERVED	= 0x8000,
        VT_ILLEGAL	= 0xffff,
        VT_ILLEGALMASKED	= 0xfff,
        VT_TYPEMASK	= 0xfff
    } ;
typedef ULONG PROPID;

#ifndef PROPERTYKEY_DEFINED
#define PROPERTYKEY_DEFINED
typedef struct _tagpropertykey
    {
    GUID fmtid;
    DWORD pid;
    } 	PROPERTYKEY;

#endif
typedef struct tagCSPLATFORM
    {
    DWORD dwPlatformId;
    DWORD dwVersionHi;
    DWORD dwVersionLo;
    DWORD dwProcessorArch;
    } 	CSPLATFORM;

typedef struct tagQUERYCONTEXT
    {
    DWORD dwContext;
    CSPLATFORM Platform;
    LCID Locale;
    DWORD dwVersionHi;
    DWORD dwVersionLo;
    } 	QUERYCONTEXT;

typedef /* [v1_enum] */ 
enum tagTYSPEC
    {
        TYSPEC_CLSID	= 0,
        TYSPEC_FILEEXT	= ( TYSPEC_CLSID + 1 ) ,
        TYSPEC_MIMETYPE	= ( TYSPEC_FILEEXT + 1 ) ,
        TYSPEC_FILENAME	= ( TYSPEC_MIMETYPE + 1 ) ,
        TYSPEC_PROGID	= ( TYSPEC_FILENAME + 1 ) ,
        TYSPEC_PACKAGENAME	= ( TYSPEC_PROGID + 1 ) ,
        TYSPEC_OBJECTID	= ( TYSPEC_PACKAGENAME + 1 ) 
    } 	TYSPEC;

typedef /* [public] */ struct __MIDL___MIDL_itf_wtypes_0000_0001_0001
    {
    DWORD tyspec;
    /* [switch_is] */ /* [switch_type] */ union __MIDL___MIDL_itf_wtypes_0000_0001_0005
        {
        /* [case()] */ CLSID clsid;
        /* [case()] */ LPOLESTR pFileExt;
        /* [case()] */ LPOLESTR pMimeType;
        /* [case()] */ LPOLESTR pProgId;
        /* [case()] */ LPOLESTR pFileName;
        /* [case()] */ struct 
            {
            LPOLESTR pPackageName;
            GUID PolicyId;
            } 	ByName;
        /* [case()] */ struct 
            {
            GUID ObjectId;
            GUID PolicyId;
            } 	ByObjectId;
        } 	tagged_union;
    } 	uCLSSPEC;

#if _MSC_VER >= 1200
#pragma warning(pop)
#endif


extern RPC_IF_HANDLE __MIDL_itf_wtypes_0000_0001_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_wtypes_0000_0001_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


