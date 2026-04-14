/******************************************************************************
*                                                                             *
* imepad.h - - Interface for the Windows IMEPAD, IMEPADAPPLET                 *
*                                                                             *
* Version 15.0                                                                *
*                                                                             *
* Copyright (c) Microsoft Corporation.  All Rights Reserved.                  *
*                                                                             *
******************************************************************************/

#if (NTDDI_VERSION >= NTDDI_WIN8)

#ifndef _IME_PAD_H_
#define _IME_PAD_H_
#include <windows.h>
#include <objbase.h>

#include <winapifamily.h>

#ifdef __cplusplus
extern "C" {
#endif

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

//----------------------------------------------------------------
// ImePadApplet's Category ID
//----------------------------------------------------------------
// {4A0F8E31-C3EE-11d1-AFEF-00805F0C8B6D}
DEFINE_GUID(CATID_MSIME_IImePadApplet_VER7,
0x4a0f8e31, 0xc3ee, 0x11d1, 0xaf, 0xef, 0x0, 0x80, 0x5f, 0xc, 0x8b, 0x6d);

//----------------------------------------------------------------
// ImePadApplet's Category ID version 8.0.0
//----------------------------------------------------------------
// {56F7A792-FEF1-11d3-8463-00C04F7A06E5}
DEFINE_GUID(CATID_MSIME_IImePadApplet_VER80,
0x56f7a792, 0xfef1, 0x11d3, 0x84, 0x63, 0x0, 0xc0, 0x4f, 0x7a, 0x6, 0xe5);

//----------------------------------------------------------------
// ImePadApplet's Category ID version 8.1.0
//----------------------------------------------------------------
// {656520B0-BB88-11d4-84C0-00C04F7A06E5}
DEFINE_GUID(CATID_MSIME_IImePadApplet_VER81, 
0x656520b0, 0xbb88, 0x11d4, 0x84, 0xc0, 0x0, 0xc0, 0x4f, 0x7a, 0x6, 0xe5);

//----------------------------------------------------------------
// ImePadApplet's Category ID version 9.0.0
//----------------------------------------------------------------
// {FAAE51BF-5E5B-4A1D-8DE1-17C1D9E1728D}
DEFINE_GUID(CATID_MSIME_IImePadApplet900, 
0xfaae51bf, 0x5e5b, 0x4a1d, 0x8d, 0xe1, 0x17, 0xc1, 0xd9, 0xe1, 0x72, 0x8d);

//----------------------------------------------------------------
// ImePadApplet's Category ID version 10.0.0
//----------------------------------------------------------------
// {E081E1D6-2389-43cb-B66F-609F823D9F9C}
DEFINE_GUID(CATID_MSIME_IImePadApplet1000, 
0xe081e1d6, 0x2389, 0x43cb, 0xb6, 0x6f, 0x60, 0x9f, 0x82, 0x3d, 0x9f, 0x9c);

//----------------------------------------------------------------
// ImePadApplet's Category ID version 12.0.0
//----------------------------------------------------------------
// {A47FB5FC-7D15-4223-A789-B781BF9AE667}
DEFINE_GUID(CATID_MSIME_IImePadApplet1200, 
0xa47fb5fc, 0x7d15, 0x4223, 0xa7, 0x89, 0xb7, 0x81, 0xbf, 0x9a, 0xe6, 0x67);

//----------------------------------------------------------------
// ImePadApplet's Category ID version 14.0.0
//----------------------------------------------------------------
// {7566CAD1-4EC9-4478-9FE9-8ED766619EDF}
DEFINE_GUID(CATID_MSIME_IImePadApplet, 
0x7566cad1, 0x4ec9, 0x4478, 0x9f, 0xe9, 0x8e, 0xd7, 0x66, 0x61, 0x9e, 0xdf);

//----------------------------------------------------------------
// Interface ID
//----------------------------------------------------------------
// {5D8E643A-C3A9-11d1-AFEF-00805F0C8B6D}
DEFINE_GUID(IID_IImePad, 
0x5d8e643a, 0xc3a9, 0x11d1, 0xaf, 0xef, 0x0, 0x80, 0x5f, 0xc, 0x8b, 0x6d);

// {5D8E643B-C3A9-11d1-AFEF-00805F0C8B6D}
DEFINE_GUID(IID_IImePadApplet,
0x5d8e643b, 0xc3a9, 0x11d1, 0xaf, 0xef, 0x0, 0x80, 0x5f, 0xc, 0x8b, 0x6d);

// {5D8E643C-C3A9-11d1-AFEF-00805F0C8B6D}
DEFINE_GUID(IID_IImeSpecifyApplets,
0x5d8e643c, 0xc3a9, 0x11d1, 0xaf, 0xef, 0x0, 0x80, 0x5f, 0xc, 0x8b, 0x6d);


#pragma pack(8)

typedef struct tagAPPLETIDLIST {
    INT  count;
    IID *pIIDList;
}APPLETIDLIST, *LPAPPLETIDLIST;

//structure for IMEPADREQ_INSERTSTRINGCANDIDATE
typedef struct tagIMESTRINGCANDIDATE {
    UINT    uCount;
    LPWSTR  lpwstr[1];
}IMESTRINGCANDIDATE, *LPIMESTRINGCANDIDATE;

//structure for IMEPADREQ_INSERTITEMCANDIDATE
typedef struct tagIMEITEM {
    INT     cbSize;
    INT     iType;
    LPVOID  lpItemData;
}IMEITEM, *LPIMEITEM;

typedef struct tagIMEITEMCANDIDATE {
    UINT    uCount;
    IMEITEM imeItem[1];
}IMEITEMCANDIDATE, *LPIMEITEMCANDIDATE;

//----------------------------------------------------------------
// Character Id in FarEast
//----------------------------------------------------------------
#define FEID_NONE                   0x00
#define FEID_CHINESE_TRADITIONAL    0x01
#define FEID_CHINESE_SIMPLIFIED     0x02
#define FEID_CHINESE_HONGKONG       0x03
#define FEID_CHINESE_SINGAPORE      0x04
#define FEID_JAPANESE               0x05
#define FEID_KOREAN                 0x06
#define FEID_KOREAN_JOHAB           0x07

//----------------------------------------------------------------
// String with FarEast id
//----------------------------------------------------------------
typedef struct tabIMESTRINGINFO {
    DWORD   dwFarEastId;
    LPWSTR  lpwstr;
}IMESTRINGINFO, *LPIMESTRINGINFO;
typedef const IMESTRINGINFO *LPCIMESTRINGINFO;

#define INFOMASK_NONE           0x00000000
#define INFOMASK_QUERY_CAND     0x00000001
#define INFOMASK_APPLY_CAND     0x00000002
#define INFOMASK_APPLY_CAND_EX  0x00000004
#define INFOMASK_STRING_FIX     0x00010000
#define INFOMASK_HIDE_CAND      0x00020000
#define INFOMASK_BLOCK_CAND     0x00040000

//----------------------------------------------------------------
//FarEast data type
//----------------------------------------------------------------
#define IMEFAREASTINFO_TYPE_DEFAULT     0
#define IMEFAREASTINFO_TYPE_READING     1
#define IMEFAREASTINFO_TYPE_COMMENT     2
#define IMEFAREASTINFO_TYPE_COSTTIME    3


//----------------------------------------------------------------
//FarEast specified data.
//----------------------------------------------------------------
typedef struct tabIMEFAREASTINFO {
    DWORD dwSize;       //total structure size.
    DWORD dwType;       //Data type.
    DWORD dwData[1];    //fareast spec data.
}IMEFAREASTINFO, *LPIMEFAREASTINFO;

//----------------------------------------------------------------
// String candidate info
//----------------------------------------------------------------
typedef struct tagIMESTRINGCANDIDATEINFO {
    DWORD               dwFarEastId;
    LPIMEFAREASTINFO    lpFarEastInfo;
    DWORD               fInfoMask;
    INT                 iSelIndex;
    UINT                uCount;
    LPWSTR              lpwstr[1];
}IMESTRINGCANDIDATEINFO, *LPIMESTRINGCANDIDATEINFO;

//----------------------------------------------------------------
//Composition string's information
//----------------------------------------------------------------
typedef struct tagIMECOMPOSITIONSTRINGINFO {
    INT iCompStrLen;
    INT iCaretPos;
    INT iEditStart;
    INT iEditLen;
    INT iTargetStart;
    INT iTargetLen;
}IMECOMPOSITIONSTRINGINFO, *LPIMECOMPOSITIONSTRINGINFO;

//----------------------------------------------------------------
//Composition string's each character
//----------------------------------------------------------------
typedef struct tagIMECHARINFO {
    WCHAR   wch;
    DWORD   dwCharInfo;
}IMECHARINFO, *LPIMECHARINFO;

//----------------------------------------------------------------
//IMECHARINFO's dwCharInfo bit mask
//----------------------------------------------------------------
#define CHARINFO_APPLETID_MASK  0xFF000000
#define CHARINFO_FEID_MASK      0x00F00000
#define CHARINFO_CHARID_MASK    0x0000FFFF

#define APPLETIDFROMCHARINFO(charInfo)  (((DWORD)(charInfo) & CHARINFO_APPLETID_MASK) >> 24)
#define FEIDFROMCHARINFO(charInfo)      (((DWORD)(charInfo) & CHARINFO_FEID_MASK) >> 20)
#define CHARIDFROMCHARINFO(charInfo)    ((DWORD)(charInfo) & CHARINFO_CHARID_MASK)

//===== IImePadApplet Configuration ===============================
#define MAX_APPLETTITLE     64
#define MAX_FONTFACE        32  
typedef struct tagAPPLETCFG {
    DWORD       dwConfig;                       //set combination of IPACFG_XXXX
    WCHAR       wchTitle[MAX_APPLETTITLE];      //set applet's title name;
    WCHAR       wchTitleFontFace[MAX_FONTFACE]; //set title's font face. 
    DWORD       dwCharSet;                      //set character set.
    INT         iCategory;                      //set IPACID_XXXX
    HICON       hIcon;                          //set Icon Handle for ImePad Appelt's menu.
    LANGID      langID;                         //set Applet langID.
    WORD        dummy;
    LPARAM      lReserved1;
}IMEAPPLETCFG, *LPIMEAPPLETCFG;

//----------------------------------------------------------------
// APPLETCFG dwConfig mask
//
#define IPACFG_NONE                 0x00000000L
#define IPACFG_PROPERTY             0x00000001L     //Applet has property.
#define IPACFG_HELP                 0x00000002L     //Applet has help.
#define IPACFG_TITLE                0x00010000L     //Title is set.
#define IPACFG_TITLEFONTFACE        0x00020000L     //wchFontFace, dwCharSet is valid.
#define IPACFG_CATEGORY             0x00040000L     //category has set.
#define IPACFG_LANG                 0x00000010L     //LangID has set.
//----------------------------------------------------------------
// APPLETCFG iCategory
//
#define IPACID_NONE                 0x0000
#define IPACID_SOFTKEY              0x0001
#define IPACID_HANDWRITING          0x0002
#define IPACID_STROKESEARCH         0x0003
#define IPACID_RADICALSEARCH        0x0004
#define IPACID_SYMBOLSEARCH         0x0005
#define IPACID_VOICE                0x0006
#define IPACID_EPWING               0x0007
#define IPACID_OCR                  0x0008
#define IPACID_CHARLIST             0x0009
#define IPACID_USER                 0x0100


typedef struct tagIMEAPPLETUI {
    HWND    hwnd;                   //Window handle of Applet.
    DWORD   dwStyle;                //set combination of IPAWS_XXX.
    INT     width;                  //set Applet's initial width. 
    INT     height;                 //set Applet's initial height.
    INT     minWidth;               //set min width.  Valid only IPAWS_MINSIZEFIXED style has set.
    INT     minHeight;              //set min height. Valid only IPAWS_MINSIZEFIXED style has set.
    INT     maxWidth;               //set max width.  Valid only IPAWS_MAXSIZEFIXED style has set.
    INT     maxHeight;              //set max height. Valid only IPAWS_MAXSIZEFIXED style has set.
    LPARAM  lReserved1;             //reserved area. 
    LPARAM  lReserved2;             //reserved area. 
}IMEAPPLETUI, *LPIMEAPPLETUI;

#pragma pack()


//Default insert position
#define IPR_DEFAULT_INSERTPOS       ((WORD)0xFFFF)


//==== IImePad Request ID ==========================================
#define IMEPADREQ_FIRST                         0x1000
//----------------------------------------------------------------
// IMEPADREQ_INSERTSTRING
// wParam = (WPARMA)(LPWSTR)lpwstr;     //address of Unicode text string.
// lParam = 0;                          //not used.
//----------------------------------------------------------------
#define IMEPADREQ_INSERTSTRING                  (IMEPADREQ_FIRST + 1)

//----------------------------------------------------------------
// IMEPADREQ_INSERTSTRINGCANDIDATE
// wParam = (WPARAM)(LPIMESTRINGCANDIDATE)lpStrCand;    //address of IMESTRINGCANDIDATE
// lParam = 0;                                          //not used.
//----------------------------------------------------------------
#define IMEPADREQ_INSERTSTRINGCANDIDATE         (IMEPADREQ_FIRST + 2)

//----------------------------------------------------------------
// IMEPADREQ_INSERTITEMCANDIDATE
// Not implemented in version 7.1.0
// wParam = 0;
// lParam = 0;
//----------------------------------------------------------------
#define IMEPADREQ_INSERTITEMCANDIDATE           (IMEPADREQ_FIRST + 3)

//----------------------------------------------------------------
// IMEPADREQ_SENDCONTROL
// wParam = (WPARAM)imePadCtrl; //control code (IMEPADCTRL_XXXX)
// lParam = 0;                  //not used.
//----------------------------------------------------------------
#define IMEPADREQ_SENDCONTROL                   (IMEPADREQ_FIRST + 4)

//----------------------------------------------------------------
// IMEPADREQ_SENDKEYCONTROL
// wParam = MAKEWPARAM(ctlMask, updown);
//          ctlMask is IMEKEYCTRLMASK_XXX combination
//          upDown  is IMEKEYCTRL_UP or DOWN
// lParam = (LPARAM)wvKey;      //Virtual keycode.
//----------------------------------------------------------------
#define IMEPADREQ_SENDKEYCONTROL                (IMEPADREQ_FIRST + 5)

//----------------------------------------------------------------
// IMEPADREQ_GETCOMPOSITIONSTRING
// wParam = (WPARAM)(LPWSTR)lpwstr;     //address of Unicode string buffer.     
// lParam = (LPARAM)(UINT)cchMax;       //buffer max count.
//----------------------------------------------------------------
#define IMEPADREQ_GETCOMPOSITIONSTRING          (IMEPADREQ_FIRST + 6)

//----------------------------------------------------------------
// IMEPADREQ_GETSELECTEDSTRING
// Not implemented in version 6.0.0
// wParam = 0;
// lParam = 0;
//----------------------------------------------------------------
#define IMEPADREQ_GETSELECTEDSTRING             (IMEPADREQ_FIRST + 7)

//----------------------------------------------------------------
// IMEPADREQ_SETAPPLETSIZE
// wParam = MAKEWPARAM(width, height);  // Applet's width & height
// lParam = 0;                          // not used.
//----------------------------------------------------------------
#define IMEPADREQ_SETAPPLETSIZE                 (IMEPADREQ_FIRST + 8)

//----------------------------------------------------------------
// IMEPADREQ_SETAPPLETDATA
// wParam = (WPARAM)(PBYTE)pByte;       //address of applet's data.
// lParam = (LPARAM)(INT)size;          //byte size of pByte. 
//----------------------------------------------------------------
#define IMEPADREQ_SETAPPLETDATA                 (IMEPADREQ_FIRST + 9)

//----------------------------------------------------------------
// IMEPADREQ_GETAPPLETDATA
// wParam = (WPARAM)(PBYTE)pByte;       //address of applet's data.
// lParam = (LPARAM)(INT)size;          //byte size of pByte. 
//----------------------------------------------------------------
#define IMEPADREQ_GETAPPLETDATA                 (IMEPADREQ_FIRST + 10)

//----------------------------------------------------------------
// IMEPADREQ_SETTITLEFONT
// wParam = (WPARAM)(LPWSTR)lpwstrFontFace; //FontFace name
// lParam = (LPARAM)(INT)charSet;           //character set
//----------------------------------------------------------------
#define IMEPADREQ_SETTITLEFONT                  (IMEPADREQ_FIRST + 11)

//----------------------------------------------------------------
// IMEPADREQ_GETCOMPOSITIONSTRINGINFO
// wParam = (WPARAM)(LPIMECOMPOSITIONSTRINGINFO)lpImeCompInfo. 
//              //IMECOMPOSITIONSTRINGINFO struct address.
// lParam = 0;  //no use.
//----------------------------------------------------------------
#define IMEPADREQ_GETCOMPOSITIONSTRINGINFO      (IMEPADREQ_FIRST + 12)

//----------------------------------------------------------------
// IMEPADREQ_GETCOMPOSITIONSTRINGID
// wParam = (WPARAM)(LPIMECHARINFO)lpCharInfo;
// lParam = (LPARAM)(INT)dwMaxLen;
//----------------------------------------------------------------
#define IMEPADREQ_GETCOMPOSITIONSTRINGID        (IMEPADREQ_FIRST + 13)

//----------------------------------------------------------------
// IMEPADREQ_INSERTSTRINGCANDIDATEINFO
// wParam = (WPARAM)(LPIMESTRINGCANDIDATEINFO)lpCandInfo;
// lParam = (LPARAM)(WORD)wStartPos;
//----------------------------------------------------------------
#define IMEPADREQ_INSERTSTRINGCANDIDATEINFO     (IMEPADREQ_FIRST + 14)

//----------------------------------------------------------------
// IMEPADREQ_CHANGESTRINGCANDIDATEINFO
// wParam = (WPARAM)(LPIMESTRINGCANDIDATEINFO)lpCandInfo;
// lParam = MAKELPARAM(startPos, length);
//----------------------------------------------------------------
#define IMEPADREQ_CHANGESTRINGCANDIDATEINFO     (IMEPADREQ_FIRST + 15)

//----------------------------------------------------------------
// IMEPADREQ_DELETESTRING
// wParam = MAKEWPARAM(wStartPos, wLength); 
// lParam = 0;                              //not used.
//----------------------------------------------------------------
#define IMEPADREQ_DELETESTRING                  (IMEPADREQ_FIRST + 16)

//----------------------------------------------------------------
// IMEPADREQ_CHANGESTRING
// wParam = (WPARAM)(LPWSTR)lpwstr; 
// lParam = MAKELPARAM(wStartPos, wLength);
//----------------------------------------------------------------
#define IMEPADREQ_CHANGESTRING                  (IMEPADREQ_FIRST + 17)

//----------------------------------------------------------------
// IMEPADREQ_INSERTSTRINGINFO
// wParam = (WPARAM)(LPIMESTRINGINFO)lpStrInfo;
// lParam = dwStartPos
//----------------------------------------------------------------
#define IMEPADREQ_INSERTSTRINGINFO              (IMEPADREQ_FIRST + 18)

//----------------------------------------------------------------
// IMEPADREQ_CHANGESTRINGINFO
// wParam = (WPARAM)(LPIMESTRINGINFO)lpStrInfo;
// lParam = MAKELPARAM(wStartPos, wLength);
//----------------------------------------------------------------
#define IMEPADREQ_CHANGESTRINGINFO              (IMEPADREQ_FIRST + 19)

//----------------------------------------------------------------
// IMEPADREQ_GETAPPLHWND
// wParam = (WPARAM)(HWND *)lpHwnd;
// lParam = 0; not used.
//----------------------------------------------------------------
#define IMEPADREQ_GETAPPLHWND                   (IMEPADREQ_FIRST + 20)

//----------------------------------------------------------------
// IMEPADREQ_FORCEIMEPADWINDOWSHOW
// wParam = (WPARAM)(BOOL)fShowForce
// lParam = 0; not used.
//----------------------------------------------------------------
#define IMEPADREQ_FORCEIMEPADWINDOWSHOW         (IMEPADREQ_FIRST + 21)

//----------------------------------------------------------------
// IMEPADREQ_POSTMODALNOTIFY
// wParam = (WPARAM)notifycode 
// lParam = (LPARAM)dwData.
//----------------------------------------------------------------
#define IMEPADREQ_POSTMODALNOTIFY               (IMEPADREQ_FIRST + 22)

//----------------------------------------------------------------
// IMEPADREQ_GETDEFAULTUILANGID
// wParam = (WPARAM)(LANGID *)pLangID;
// lParam = 0; not used
//----------------------------------------------------------------
#define IMEPADREQ_GETDEFAULTUILANGID            (IMEPADREQ_FIRST + 23)

//----------------------------------------------------------------
// IMEPADREQ_GETCURRENTUILANGID
// wParam = (WPARAM)(LANGID *)pLangID;
// lParam = 0; not used
//----------------------------------------------------------------
#define IMEPADREQ_GETCURRENTUILANGID            (IMEPADREQ_FIRST + 24)

//----------------------------------------------------------------
// IMEPADREQ_GETAPPLETUISTYLE
// wParam = (WPARAM)(DWORD *)pdwStyle;
// lParam = 0; not used.
//----------------------------------------------------------------
#define IMEPADREQ_GETAPPLETUISTYLE              (IMEPADREQ_FIRST + 25)

//----------------------------------------------------------------
// IMEPADREQ_SETAPPLETUISTYLE
// wParam = (WPARAM)(DWORD)dwStyle;
// lParam = 0; not used.
//----------------------------------------------------------------
#define IMEPADREQ_SETAPPLETUISTYLE              (IMEPADREQ_FIRST + 26)

//----------------------------------------------------------------
// IMEPADREQ_ISAPPLETACTIVE
// wParam = (WPARAM)(BOOL *)pfActive;
// lParam = 0; not used.
//----------------------------------------------------------------
#define IMEPADREQ_ISAPPLETACTIVE                (IMEPADREQ_FIRST + 27)

//----------------------------------------------------------------
// IMEPADREQ_ISIMEPADWINDOWVISIBLE
// wParam = (WPARAM)(BOOL *)pfVisible;
// lParam = 0; not used.
//----------------------------------------------------------------
#define IMEPADREQ_ISIMEPADWINDOWVISIBLE         (IMEPADREQ_FIRST + 28)

//----------------------------------------------------------------
// IMEPADREQ_SETAPPLETMINMAXSIZE
// wParam = MAKEWPARAM(width, height);  // Applet's width & height
// lParam = MAKELPARAM(fMax, fNoMove);  // Set Max size or Min size,
//                                      // And move or not move. 
//----------------------------------------------------------------
#define IMEPADREQ_SETAPPLETMINMAXSIZE           (IMEPADREQ_FIRST + 29)

//----------------------------------------------------------------
// IMEPADREQ_GETCONVERSIONSTATUS
// wParam = (WPARAM)(DWORD *)pdwConversionMode; //conversion mode.
// lParam = (LPARAM)(DWORD *)pdwSentenceMode;   //sentence mode.
//----------------------------------------------------------------
#define IMEPADREQ_GETCONVERSIONSTATUS           (IMEPADREQ_FIRST + 30)

//----------------------------------------------------------------
// IMEPADREQ_GETVERSION
// wParam = (WPARAM)(DWORD *)pdwVerMS;
// lParam = (LPARAM)(DWORD *)pdwVerLS;
//----------------------------------------------------------------
#define IMEPADREQ_GETVERSION                    (IMEPADREQ_FIRST + 31)

//----------------------------------------------------------------
// IMEPADREQ_GETCURRENTIMEINFO
// wParam = (WPARAM)(DWORD *)pdwImeLangID;
// lParam = (LPARAM)(DWORD *)pdwImeInputID;
//----------------------------------------------------------------
#define IMEPADREQ_GETCURRENTIMEINFO             (IMEPADREQ_FIRST + 32)


//===== IMEPADREQ_SENDCONTROL reques parameter ======================
#define IMEPADCTRL_CONVERTALL           1
#define IMEPADCTRL_DETERMINALL          2
#define IMEPADCTRL_DETERMINCHAR         3
#define IMEPADCTRL_CLEARALL             4
#define IMEPADCTRL_CARETSET             5
#define IMEPADCTRL_CARETLEFT            6   
#define IMEPADCTRL_CARETRIGHT           7
#define IMEPADCTRL_CARETTOP             8 
#define IMEPADCTRL_CARETBOTTOM          9 
#define IMEPADCTRL_CARETBACKSPACE       10 
#define IMEPADCTRL_CARETDELETE          11 
#define IMEPADCTRL_PHRASEDELETE         12
#define IMEPADCTRL_INSERTSPACE          13
#define IMEPADCTRL_INSERTFULLSPACE      14
#define IMEPADCTRL_INSERTHALFSPACE      15
#define IMEPADCTRL_ONIME                16
#define IMEPADCTRL_OFFIME               17
#define IMEPADCTRL_ONPRECONVERSION      18
#define IMEPADCTRL_OFFPRECONVERSION     19
#define IMEPADCTRL_PHONETICCANDIDATE    20


//hot key definition
#define IMEKEYCTRLMASK_ALT          0x0001
#define IMEKEYCTRLMASK_CTRL         0x0002
#define IMEKEYCTRLMASK_SHIFT        0x0004

#define IMEKEYCTRL_UP               1
#define IMEKEYCTRL_DOWN             0

//===== IImePadApplet Notify ID =====================================
#define IMEPN_FIRST                 0x0100
#define IMEPN_ACTIVATE              (IMEPN_FIRST + 1)
#define IMEPN_INACTIVATE            (IMEPN_FIRST + 2)
#define IMEPN_SHOW                  (IMEPN_FIRST + 4)
#define IMEPN_HIDE                  (IMEPN_FIRST + 5)
#define IMEPN_SIZECHANGING          (IMEPN_FIRST + 6)
#define IMEPN_SIZECHANGED           (IMEPN_FIRST + 7)
#define IMEPN_CONFIG                (IMEPN_FIRST + 8)
#define IMEPN_HELP                  (IMEPN_FIRST + 9)
#define IMEPN_QUERYCAND             (IMEPN_FIRST +10)
#define IMEPN_APPLYCAND             (IMEPN_FIRST +11)
#define IMEPN_APPLYCANDEX           (IMEPN_FIRST +12)
#define IMEPN_SETTINGCHANGED        (IMEPN_FIRST +13)   

#define IMEPN_USER                  (IMEPN_FIRST + 100)


/*------------------------------------------------------------------------------
    %%Owner: KotaroY
    %%Id: 77a6c36a-c7b1-4165-801e-cf320fec71b4
------------------------------------------------------------------------------*/
typedef struct tagAPPLYCANDEXPARAM
{

    DWORD   dwSize;
    LPWSTR  lpwstrDisplay;
    LPWSTR  lpwstrReading;
    DWORD   dwReserved;

} APPLYCANDEXPARAM, *LPAPPLYCANDEXPARAM;


//===== IImePadApplet window style ================================
#define IPAWS_ENABLED               0x00000001L     //Show Applet as Enabled window.
#define IPAWS_SIZINGNOTIFY          0x00000004L     //send IMEPN_SIZECHANGING(ED) notify to applet.
#define IPAWS_VERTICALFIXED         0x00000100L     //Vertically fixed.
#define IPAWS_HORIZONTALFIXED       0x00000200L     //Horizontally fixed.
#define IPAWS_SIZEFIXED             0x00000300L     //size is fixed.
#define IPAWS_MAXWIDTHFIXED         0x00001000L     //max width  is fixed.
#define IPAWS_MAXHEIGHTFIXED        0x00002000L     //max height is fixed.
#define IPAWS_MAXSIZEFIXED          0x00003000L     //max size is fixed.
#define IPAWS_MINWIDTHFIXED         0x00010000L     //min width  is fixed. 
#define IPAWS_MINHEIGHTFIXED        0x00020000L     //min height is fixed.
#define IPAWS_MINSIZEFIXED          0x00030000L     //min size is fixed.


#ifdef __cplusplus

//======IImePad IImePadApplet Interface definition===============



DECLARE_INTERFACE_(IImeSpecifyApplets, IUnknown)
{
    /*** IUnknown methods ***/
    STDMETHOD(QueryInterface) (THIS_ REFIID riid, LPVOID* ppvObj) PURE;
    STDMETHOD_(ULONG, AddRef) (THIS) PURE;
    STDMETHOD_(ULONG, Release)(THIS) PURE;
    /*** IImeSpecifyApplet methods ***/
    STDMETHOD(GetAppletIIDList)(THIS_
                                REFIID          refiid,
                                LPAPPLETIDLIST  lpIIDList) PURE;
};


DECLARE_INTERFACE_(IImePadApplet, IUnknown)
{
    /*** IUnknown methods ***/
    STDMETHOD(QueryInterface) (THIS_ REFIID riid, LPVOID* ppvObj) PURE;
    STDMETHOD_(ULONG, AddRef) (THIS) PURE;
    STDMETHOD_(ULONG, Release) (THIS) PURE;
    /*** IImePadApplet methods ***/
    STDMETHOD(Initialize)(THIS_ IUnknown *lpIImePad)  PURE;
    STDMETHOD(Terminate) (THIS)                     PURE;
    STDMETHOD(GetAppletConfig)(THIS_ LPIMEAPPLETCFG lpAppletCfg) PURE;
    STDMETHOD(CreateUI)(THIS_ 
                        HWND                hwndParent,
                        LPIMEAPPLETUI       lpImeAppletUI) PURE;
    STDMETHOD(Notify)(THIS_ 
                      IUnknown  *lpImePad,
                      INT       notify,
                      WPARAM    wParam,
                      LPARAM    lParam) PURE;
};

DECLARE_INTERFACE_(IImePad, IUnknown)
{
    /*** IUnknown methods ***/
    STDMETHOD(QueryInterface) (THIS_ REFIID riid, LPVOID* ppvObj) PURE;
    STDMETHOD_(ULONG, AddRef) (THIS) PURE;
    STDMETHOD_(ULONG, Release)(THIS) PURE;
    /*** IImePad methods ***/
    STDMETHOD(Request)(THIS_
                       IImePadApplet *pIImePadApplet,
                       INT reqId,
                       WPARAM wParam,
                       LPARAM lParam) PURE;
};

#endif 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#ifdef __cplusplus
};
#endif

#endif //_IME_PAD_H_

#endif // (NTDDI >= NTDDI_WIN8)
