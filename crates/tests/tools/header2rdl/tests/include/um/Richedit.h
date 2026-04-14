/*
 *	RICHEDIT.H
 *	
 *	Purpose:
 *		RICHEDIT v2.0-v6.0 public definitions
 *	
 *	Copyright (c) Microsoft Corporation. All rights reserved.
 */

#ifndef _RICHEDIT_
#define _RICHEDIT_
#pragma once
#include <winapifamily.h>
#include <wtypes.h>
#include <objbase.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


#ifdef _WIN32
#include <pshpack4.h>
#elif !defined(RC_INVOKED)
#pragma pack(4)
#endif

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

// To mimic older RichEdit behavior, set _RICHEDIT_VER to appropriate value
//		Version 1.0 	0x0100	
//		Version 2.0 	0x0200	
//		Version 2.1 	0x0210	
#ifndef _RICHEDIT_VER
#define _RICHEDIT_VER	0x0810
#endif

#define cchTextLimitDefault 32767

#define MSFTEDIT_CLASS		L"RICHEDIT50W"
// NOTE:  MSFTEDIT.DLL only registers MSFTEDIT_CLASS.  If an application wants
// to use the following RichEdit classes, it needs to load riched20.dll.
// Otherwise, CreateWindow with RICHEDIT_CLASS will fail.
// This also applies to any dialog that uses RICHEDIT_CLASS
// RichEdit 2.0 Window Class

// On Windows CE, avoid possible conflicts on Win95
#define CERICHEDIT_CLASSA	"RichEditCEA"
#define CERICHEDIT_CLASSW	L"RichEditCEW"

#define RICHEDIT_CLASSA 	"RichEdit20A"
#define RICHEDIT_CLASS10A	"RICHEDIT"			// Richedit 1.0

#ifndef MACPORT
#define RICHEDIT_CLASSW 	L"RichEdit20W"
#else	//----------------------MACPORT 
#define RICHEDIT_CLASSW 	TEXT("RichEdit20W") // MACPORT change 
#endif // MACPORT  

#if (_RICHEDIT_VER >= 0x0200 )
#ifdef UNICODE
#define RICHEDIT_CLASS		RICHEDIT_CLASSW
#else
#define RICHEDIT_CLASS		RICHEDIT_CLASSA
#endif // UNICODE 
#else
#define RICHEDIT_CLASS		RICHEDIT_CLASS10A
#endif // _RICHEDIT_VER >= 0x0200 

// RichEdit messages 

#ifndef WM_CONTEXTMENU
#define WM_CONTEXTMENU			0x007B
#endif

#ifndef WM_UNICHAR
#define WM_UNICHAR				0x0109
#endif

#ifndef WM_PRINTCLIENT
#define WM_PRINTCLIENT			0x0318
#endif

#ifndef EM_GETLIMITTEXT
#define EM_GETLIMITTEXT 		(WM_USER + 37)
#endif

#ifndef EM_POSFROMCHAR	
#define EM_POSFROMCHAR			(WM_USER + 38)
#define EM_CHARFROMPOS			(WM_USER + 39)
#endif

#ifndef EM_SCROLLCARET
#define EM_SCROLLCARET			(WM_USER + 49)
#endif
#define EM_CANPASTE 			(WM_USER + 50)
#define EM_DISPLAYBAND			(WM_USER + 51)
#define EM_EXGETSEL 			(WM_USER + 52)
#define EM_EXLIMITTEXT			(WM_USER + 53)
#define EM_EXLINEFROMCHAR		(WM_USER + 54)
#define EM_EXSETSEL 			(WM_USER + 55)
#define EM_FINDTEXT 			(WM_USER + 56)
#define EM_FORMATRANGE			(WM_USER + 57)
#define EM_GETCHARFORMAT		(WM_USER + 58)
#define EM_GETEVENTMASK 		(WM_USER + 59)
#define EM_GETOLEINTERFACE		(WM_USER + 60)
#define EM_GETPARAFORMAT		(WM_USER + 61)
#define EM_GETSELTEXT			(WM_USER + 62)
#define EM_HIDESELECTION		(WM_USER + 63)
#define EM_PASTESPECIAL 		(WM_USER + 64)
#define EM_REQUESTRESIZE		(WM_USER + 65)
#define EM_SELECTIONTYPE		(WM_USER + 66)
#define EM_SETBKGNDCOLOR		(WM_USER + 67)
#define EM_SETCHARFORMAT		(WM_USER + 68)
#define EM_SETEVENTMASK 		(WM_USER + 69)
#define EM_SETOLECALLBACK		(WM_USER + 70)
#define EM_SETPARAFORMAT		(WM_USER + 71)
#define EM_SETTARGETDEVICE		(WM_USER + 72)
#define EM_STREAMIN 			(WM_USER + 73)
#define EM_STREAMOUT			(WM_USER + 74)
#define EM_GETTEXTRANGE 		(WM_USER + 75)
#define EM_FINDWORDBREAK		(WM_USER + 76)
#define EM_SETOPTIONS			(WM_USER + 77)
#define EM_GETOPTIONS			(WM_USER + 78)
#define EM_FINDTEXTEX			(WM_USER + 79)
#ifdef _WIN32
#define EM_GETWORDBREAKPROCEX	(WM_USER + 80)
#define EM_SETWORDBREAKPROCEX	(WM_USER + 81)
#endif

// RichEdit 2.0 messages 
#define EM_SETUNDOLIMIT 		(WM_USER + 82)
#define EM_REDO 				(WM_USER + 84)
#define EM_CANREDO				(WM_USER + 85)
#define EM_GETUNDONAME			(WM_USER + 86)
#define EM_GETREDONAME			(WM_USER + 87)
#define EM_STOPGROUPTYPING		(WM_USER + 88)

#define EM_SETTEXTMODE			(WM_USER + 89)
#define EM_GETTEXTMODE			(WM_USER + 90)

// enum for use with EM_GET/SETTEXTMODE 
typedef enum tagTextMode
{
	TM_PLAINTEXT			= 1,
	TM_RICHTEXT 			= 2,	// Default behavior 
	TM_SINGLELEVELUNDO		= 4,
	TM_MULTILEVELUNDO		= 8,	// Default behavior 
	TM_SINGLECODEPAGE		= 16,
	TM_MULTICODEPAGE		= 32	// Default behavior 
} TEXTMODE;

#define EM_AUTOURLDETECT		(WM_USER + 91)

#if (_RICHEDIT_VER >= 0x0800)
#define	AURL_ENABLEURL			1
#define	AURL_ENABLEEMAILADDR	2
#define	AURL_ENABLETELNO		4
#define	AURL_ENABLEEAURLS		8
#define	AURL_ENABLEDRIVELETTERS	16
#define	AURL_DISABLEMIXEDLGC	32	// Disable mixed Latin Greek Cyrillic IDNs
#endif

#define EM_GETAUTOURLDETECT 	(WM_USER + 92)
#define EM_SETPALETTE			(WM_USER + 93)
#define EM_GETTEXTEX			(WM_USER + 94)
#define EM_GETTEXTLENGTHEX		(WM_USER + 95)
#define EM_SHOWSCROLLBAR		(WM_USER + 96)
#define EM_SETTEXTEX			(WM_USER + 97)

// East Asia specific messages 
#define EM_SETPUNCTUATION		(WM_USER + 100)
#define EM_GETPUNCTUATION		(WM_USER + 101)
#define EM_SETWORDWRAPMODE		(WM_USER + 102)
#define EM_GETWORDWRAPMODE		(WM_USER + 103)
#define EM_SETIMECOLOR			(WM_USER + 104)
#define EM_GETIMECOLOR			(WM_USER + 105)
#define EM_SETIMEOPTIONS		(WM_USER + 106)
#define EM_GETIMEOPTIONS		(WM_USER + 107)
#define EM_CONVPOSITION 		(WM_USER + 108)

#define EM_SETLANGOPTIONS		(WM_USER + 120)
#define EM_GETLANGOPTIONS		(WM_USER + 121)
#define EM_GETIMECOMPMODE		(WM_USER + 122)

#define EM_FINDTEXTW			(WM_USER + 123)
#define EM_FINDTEXTEXW			(WM_USER + 124)

// RE3.0 FE messages 
#define EM_RECONVERSION 		(WM_USER + 125)
#define EM_SETIMEMODEBIAS		(WM_USER + 126) 
#define EM_GETIMEMODEBIAS		(WM_USER + 127)

// BiDi specific messages 
#define EM_SETBIDIOPTIONS		(WM_USER + 200)
#define EM_GETBIDIOPTIONS		(WM_USER + 201)

#define EM_SETTYPOGRAPHYOPTIONS (WM_USER + 202)
#define EM_GETTYPOGRAPHYOPTIONS (WM_USER + 203)

// Extended edit style specific messages 
#define EM_SETEDITSTYLE 		(WM_USER + 204)
#define EM_GETEDITSTYLE 		(WM_USER + 205)

// Extended edit style masks 
#define SES_EMULATESYSEDIT		1
#define SES_BEEPONMAXTEXT		2
#define SES_EXTENDBACKCOLOR 	4
#define SES_MAPCPS				8		// Obsolete (never used)
#if (_RICHEDIT_VER >= 0x0500)
#define SES_HYPERLINKTOOLTIPS	8
#endif
#define SES_EMULATE10			16		// Obsolete (never used)
#if (_RICHEDIT_VER >= 0x0700)
#define SES_DEFAULTLATINLIGA	16
#endif
#define SES_USECRLF 			32		// Obsolete (never used)
#if (_RICHEDIT_VER >= 0x0700)
#define SES_NOFOCUSLINKNOTIFY	32
#endif
#define SES_USEAIMM 			64
#define SES_NOIME				128

#define SES_ALLOWBEEPS			256
#define SES_UPPERCASE			512
#define SES_LOWERCASE			1024
#define SES_NOINPUTSEQUENCECHK	2048
#define SES_BIDI				4096
#define SES_SCROLLONKILLFOCUS	8192
#define SES_XLTCRCRLFTOCR		16384
#define SES_DRAFTMODE			32768

#define SES_USECTF					0x00010000
#define SES_HIDEGRIDLINES			0x00020000
#define SES_USEATFONT				0x00040000
#define SES_CUSTOMLOOK				0x00080000
#define SES_LBSCROLLNOTIFY			0x00100000
#define SES_CTFALLOWEMBED			0x00200000
#define SES_CTFALLOWSMARTTAG		0x00400000
#define SES_CTFALLOWPROOFING		0x00800000
#if (_RICHEDIT_VER >= 0x0500)
#define SES_LOGICALCARET			0x01000000
#define SES_WORDDRAGDROP			0x02000000
#define SES_SMARTDRAGDROP			0x04000000
#define SES_MULTISELECT				0x08000000
#define SES_CTFNOLOCK				0x10000000
#define SES_NOEALINEHEIGHTADJUST	0x20000000
#define SES_MAX						0x20000000
#endif

// Options for EM_SETLANGOPTIONS and EM_GETLANGOPTIONS 
#define IMF_AUTOKEYBOARD		0x0001
#define IMF_AUTOFONT			0x0002
#define IMF_IMECANCELCOMPLETE	0x0004	// High completes comp string when aborting, low cancels
#define IMF_IMEALWAYSSENDNOTIFY 0x0008
#define IMF_AUTOFONTSIZEADJUST	0x0010
#define IMF_UIFONTS 			0x0020
#if (_RICHEDIT_VER >= 0x0800)
#define IMF_NOIMPLICITLANG		0x0040
#endif
#define IMF_DUALFONT			0x0080
#if (_RICHEDIT_VER >= 0x0800)
#define IMF_NOKBDLIDFIXUP		0x0200
#endif
#define IMF_NORTFFONTSUBSTITUTE 0x0400
#if (_RICHEDIT_VER >= 0x0800)
#define IMF_SPELLCHECKING		0x0800
#define IMF_TKBPREDICTION		0x1000
#endif
#if (_RICHEDIT_VER >= 0x0810)
#define IMF_IMEUIINTEGRATION    0x2000
#endif

// Values for EM_GETIMECOMPMODE 
#define ICM_NOTOPEN 			0x0000
#define ICM_LEVEL3				0x0001
#define ICM_LEVEL2				0x0002
#define ICM_LEVEL2_5			0x0003
#define ICM_LEVEL2_SUI			0x0004
#define ICM_CTF 				0x0005

// Options for EM_SETTYPOGRAPHYOPTIONS 
#define TO_ADVANCEDTYPOGRAPHY	0x0001
#define TO_SIMPLELINEBREAK		0x0002
#define TO_DISABLECUSTOMTEXTOUT 0x0004
#define TO_ADVANCEDLAYOUT		0x0008

// Pegasus outline mode messages (RE 3.0) 

// Outline mode message
#define EM_OUTLINE				(WM_USER + 220)
// Message for getting and restoring scroll pos
#define EM_GETSCROLLPOS 		(WM_USER + 221)
#define EM_SETSCROLLPOS 		(WM_USER + 222)
// Change fontsize in current selection by wParam
#define EM_SETFONTSIZE			(WM_USER + 223)
#define EM_GETZOOM				(WM_USER + 224)
#define EM_SETZOOM				(WM_USER + 225)
#define EM_GETVIEWKIND			(WM_USER + 226)
#define EM_SETVIEWKIND			(WM_USER + 227)

// RichEdit 4.0 messages
#define EM_GETPAGE				(WM_USER + 228)
#define EM_SETPAGE				(WM_USER + 229)
#define EM_GETHYPHENATEINFO 	(WM_USER + 230)
#define EM_SETHYPHENATEINFO 	(WM_USER + 231)

#define EM_GETPAGEROTATE		(WM_USER + 235)
#define EM_SETPAGEROTATE		(WM_USER + 236)
#define EM_GETCTFMODEBIAS		(WM_USER + 237)
#define EM_SETCTFMODEBIAS		(WM_USER + 238)
#define EM_GETCTFOPENSTATUS 	(WM_USER + 240)
#define EM_SETCTFOPENSTATUS 	(WM_USER + 241)
#define EM_GETIMECOMPTEXT		(WM_USER + 242)
#define EM_ISIME				(WM_USER + 243)
#define EM_GETIMEPROPERTY		(WM_USER + 244)

// These messages control what rich edit does when it comes accross
// OLE objects during RTF stream in.  Normally rich edit queries the client
// application only after OleLoad has been called.  With these messages it is possible to
// set the rich edit control to a mode where it will query the client application before
// OleLoad is called
#define EM_GETQUERYRTFOBJ		(WM_USER + 269)
#define EM_SETQUERYRTFOBJ		(WM_USER + 270)

#define EM_SETQUERYCONVERTOLELINKCALLBACK	(WM_USER + 403)
#define EM_SETDISABLEOLELINKCONVERSION		(WM_USER + 404)

// EM_SETPAGEROTATE wparam values
#define EPR_0					0		// Text flows left to right and top to bottom
#define EPR_270 				1		// Text flows top to bottom and right to left
#define EPR_180 				2		// Text flows right to left and bottom to top
#define EPR_90					3		// Text flows bottom to top and left to right
#if (_RICHEDIT_VER >= 0x0800)
#define EPR_SE					5		// Text flows top to bottom and left to right (Mongolian text layout)
#endif


// EM_SETCTFMODEBIAS wparam values
#define CTFMODEBIAS_DEFAULT 				0x0000
#define CTFMODEBIAS_FILENAME				0x0001
#define CTFMODEBIAS_NAME					0x0002
#define CTFMODEBIAS_READING 				0x0003
#define CTFMODEBIAS_DATETIME				0x0004
#define CTFMODEBIAS_CONVERSATION			0x0005
#define CTFMODEBIAS_NUMERIC 				0x0006
#define CTFMODEBIAS_HIRAGANA				0x0007
#define CTFMODEBIAS_KATAKANA				0x0008
#define CTFMODEBIAS_HANGUL					0x0009
#define CTFMODEBIAS_HALFWIDTHKATAKANA		0x000A
#define CTFMODEBIAS_FULLWIDTHALPHANUMERIC	0x000B
#define CTFMODEBIAS_HALFWIDTHALPHANUMERIC	0x000C

// EM_SETIMEMODEBIAS lparam values
#define IMF_SMODE_PLAURALCLAUSE 0x0001
#define IMF_SMODE_NONE			0x0002

// EM_GETIMECOMPTEXT wparam structure
typedef struct _imecomptext {
	LONG	cb; 		// count of bytes in the output buffer.
	DWORD	flags;		// value specifying the composition string type.
						//	Currently only support ICT_RESULTREADSTR
} IMECOMPTEXT;
#define ICT_RESULTREADSTR		1

// Outline mode wparam values
#define EMO_EXIT				0		// Enter normal mode,  lparam ignored
#define EMO_ENTER				1		// Enter outline mode, lparam ignored
#define EMO_PROMOTE 			2		// LOWORD(lparam) == 0 ==>
										//	promote  to body-text
										// LOWORD(lparam) != 0 ==>
										//	promote/demote current selection
										//	by indicated number of levels
#define EMO_EXPAND				3		// HIWORD(lparam) = EMO_EXPANDSELECTION
										//	-> expands selection to level
										//	indicated in LOWORD(lparam)
										//	LOWORD(lparam) = -1/+1 corresponds
										//	to collapse/expand button presses
										//	in winword (other values are
										//	equivalent to having pressed these
										//	buttons more than once)
										//	HIWORD(lparam) = EMO_EXPANDDOCUMENT
										//	-> expands whole document to
										//	indicated level
#define EMO_MOVESELECTION		4		// LOWORD(lparam) != 0 -> move current
										//	selection up/down by indicated amount
#define EMO_GETVIEWMODE 		5		// Returns VM_NORMAL or VM_OUTLINE

// EMO_EXPAND options
#define EMO_EXPANDSELECTION 	0
#define EMO_EXPANDDOCUMENT		1

#define VM_NORMAL				4		// Agrees with RTF \viewkindN
#define VM_OUTLINE				2
#define VM_PAGE 				9		// Screen page view (not print layout)

/// New messages as of Win8
#if (_RICHEDIT_VER >= 0x0800)

#define EM_INSERTTABLE			(WM_USER + 232)

#pragma warning(push)
#pragma warning(disable:4214) // Microsoft extension -- Bitfield not int

// Data type defining table rows for EM_INSERTTABLE
typedef struct _tableRowParms
{							// EM_INSERTTABLE wparam is a (TABLEROWPARMS *)
	BYTE	cbRow;			// Count of bytes in this structure
	BYTE	cbCell; 		// Count of bytes in TABLECELLPARMS
	BYTE	cCell;			// Count of cells
	BYTE	cRow;			// Count of rows
	LONG	dxCellMargin;	// Cell left/right margin (\trgaph)
	LONG	dxIndent;		// Row left (right if fRTL indent (similar to \trleft)
	LONG	dyHeight;		// Row height (\trrh)
	DWORD	nAlignment:3;	// Row alignment (like PARAFORMAT::bAlignment, \trql, trqr, \trqc)
	DWORD	fRTL:1; 		// Display cells in RTL order (\rtlrow)
	DWORD	fKeep:1;		// Keep row together (\trkeep}
	DWORD	fKeepFollow:1;	// Keep row on same page as following row (\trkeepfollow)
	DWORD	fWrap:1;		// Wrap text to right/left (depending on bAlignment)
	// (see \tdfrmtxtLeftN, \tdfrmtxtRightN)
	DWORD	fIdentCells:1;	// lparam points at single struct valid for all cells
	LONG	cpStartRow;		// cp where to insert table (-1 for selection cp)
	//	(can be used for either TRD by EM_GETTABLEPARMS)
	BYTE	bTableLevel;	// Table nesting level (EM_GETTABLEPARMS only)
	BYTE	iCell;			// Index of cell to insert/delete (EM_SETTABLEPARMS only)
} TABLEROWPARMS;

// Data type defining table cells for EM_INSERTTABLE
typedef struct _tableCellParms
{							// EM_INSERTTABLE lparam is a (TABLECELLPARMS *)
	LONG	dxWidth;		// Cell width (\cellx)
	WORD	nVertAlign:2;	// Vertical alignment (0/1/2 = top/center/bottom
	//	\clvertalt (def), \clvertalc, \clvertalb)
	WORD	fMergeTop:1;	// Top cell for vertical merge (\clvmgf)
	WORD	fMergePrev:1;	// Merge with cell above (\clvmrg)
	WORD	fVertical:1;	// Display text top to bottom, right to left (\cltxtbrlv)
	WORD	fMergeStart:1;	// Start set of horizontally merged cells (\clmgf)
	WORD	fMergeCont:1;	// Merge with previous cell (\clmrg)
	WORD	wShading;		// Shading in .01%		(\clshdng) e.g., 10000 flips fore/back

	SHORT	dxBrdrLeft; 	// Left border width	(\clbrdrl\brdrwN) (in twips)
	SHORT	dyBrdrTop;		// Top border width 	(\clbrdrt\brdrwN)
	SHORT	dxBrdrRight;	// Right border width	(\clbrdrr\brdrwN)
	SHORT	dyBrdrBottom;	// Bottom border width	(\clbrdrb\brdrwN)
	COLORREF crBrdrLeft;	// Left border color	(\clbrdrl\brdrcf)
	COLORREF crBrdrTop; 	// Top border color 	(\clbrdrt\brdrcf)
	COLORREF crBrdrRight;	// Right border color	(\clbrdrr\brdrcf)
	COLORREF crBrdrBottom;	// Bottom border color	(\clbrdrb\brdrcf)
	COLORREF crBackPat; 	// Background color 	(\clcbpat)
	COLORREF crForePat; 	// Foreground color 	(\clcfpat)
} TABLECELLPARMS;

#pragma warning(pop)

#define EM_GETAUTOCORRECTPROC	(WM_USER + 233)
#define EM_SETAUTOCORRECTPROC	(WM_USER + 234)
#define EM_CALLAUTOCORRECTPROC	(WM_USER + 255)

// AutoCorrect callback
typedef int (WINAPI *AutoCorrectProc)(LANGID langid, const WCHAR *pszBefore, WCHAR *pszAfter, LONG cchAfter, LONG *pcchReplaced);

#define ATP_NOCHANGE			0
#define ATP_CHANGE				1
#define ATP_NODELIMITER			2
#define ATP_REPLACEALLTEXT		4

#define EM_GETTABLEPARMS			(WM_USER + 265)

#define EM_SETEDITSTYLEEX			(WM_USER + 275)
#define EM_GETEDITSTYLEEX			(WM_USER + 276)
// wparam values for EM_SETEDITSTYLEEX/EM_GETEDITSTYLEEX
// All unused bits are reserved.
#define SES_EX_NOTABLE				0x00000004
#define SES_EX_NOMATH				0x00000040
#define SES_EX_HANDLEFRIENDLYURL	0x00000100
#define SES_EX_NOTHEMING			0x00080000
#define SES_EX_NOACETATESELECTION	0x00100000
#define SES_EX_USESINGLELINE		0x00200000
#define SES_EX_MULTITOUCH			0x08000000	// Only works under Win8+
#define SES_EX_HIDETEMPFORMAT		0x10000000
#define SES_EX_USEMOUSEWPARAM		0x20000000	// Use wParam when handling WM_MOUSEMOVE message and do not call GetAsyncKeyState

#define EM_GETSTORYTYPE				(WM_USER + 290)
#define EM_SETSTORYTYPE				(WM_USER + 291)

#define EM_GETELLIPSISMODE			(WM_USER + 305)
#define EM_SETELLIPSISMODE			(WM_USER + 306)

// DWORD: *lparam for EM_GETELLIPSISMODE, lparam for EM_SETELLIPSISMODE
#define ELLIPSIS_MASK				0x00000003	// all meaningful bits
#define ELLIPSIS_NONE				0x00000000	// ellipsis disabled
#define ELLIPSIS_END				0x00000001	// ellipsis at the end (forced break)
#define ELLIPSIS_WORD				0x00000003	// ellipsis at the end (word break)

#define EM_SETTABLEPARMS			(WM_USER + 307)

#define EM_GETTOUCHOPTIONS			(WM_USER + 310)
#define EM_SETTOUCHOPTIONS			(WM_USER + 311)
#define EM_INSERTIMAGE 				(WM_USER + 314)
#define EM_SETUIANAME				(WM_USER + 320)
#define EM_GETELLIPSISSTATE			(WM_USER + 322)


// Values for EM_SETTOUCHOPTIONS/EM_GETTOUCHOPTIONS
#define RTO_SHOWHANDLES				1
#define RTO_DISABLEHANDLES			2
#define RTO_READINGMODE				3

// lparam for EM_INSERTIMAGE
typedef struct tagRICHEDIT_IMAGE_PARAMETERS
{
	LONG			xWidth;					// Units are HIMETRIC
	LONG			yHeight;				// Units are HIMETRIC
	LONG			Ascent;					// Units are HIMETRIC
	LONG			Type;					// Valid values are TA_TOP, TA_BOTTOM and TA_BASELINE
	LPCWSTR			pwszAlternateText;
	IStream *		pIStream;
} RICHEDIT_IMAGE_PARAMETERS;

#endif

// New notifications 
#define EN_MSGFILTER			0x0700
#define EN_REQUESTRESIZE		0x0701
#define EN_SELCHANGE			0x0702
#define EN_DROPFILES			0x0703
#define EN_PROTECTED			0x0704
#define EN_CORRECTTEXT			0x0705			// PenWin specific 
#define EN_STOPNOUNDO			0x0706
#define EN_IMECHANGE			0x0707			// East Asia specific 
#define EN_SAVECLIPBOARD		0x0708
#define EN_OLEOPFAILED			0x0709
#define EN_OBJECTPOSITIONS		0x070a
#define EN_LINK 				0x070b
#define EN_DRAGDROPDONE 		0x070c
#define EN_PARAGRAPHEXPANDED	0x070d
#define EN_PAGECHANGE			0x070e
#define EN_LOWFIRTF 			0x070f
#define EN_ALIGNLTR 			0x0710			// BiDi specific notification
#define EN_ALIGNRTL 			0x0711			// BiDi specific notification
#if (_RICHEDIT_VER >= 0x0800)
#define EN_CLIPFORMAT			0x0712
#define EN_STARTCOMPOSITION		0x0713
#define EN_ENDCOMPOSITION		0x0714

// Notification structure for EN_ENDCOMPOSITION
typedef struct _endcomposition
{
	NMHDR	nmhdr;
	DWORD	dwCode;
} ENDCOMPOSITIONNOTIFY;

// Constants for ENDCOMPOSITIONNOTIFY dwCode
#define ECN_ENDCOMPOSITION	0x0001
#define ECN_NEWTEXT			0x0002
#endif

// Event notification masks 
#define ENM_NONE				0x00000000
#define ENM_CHANGE				0x00000001
#define ENM_UPDATE				0x00000002
#define ENM_SCROLL				0x00000004
#define ENM_SCROLLEVENTS		0x00000008
#define ENM_DRAGDROPDONE		0x00000010
#define ENM_PARAGRAPHEXPANDED	0x00000020
#define ENM_PAGECHANGE			0x00000040
#if (_RICHEDIT_VER >= 0x0800)
#define ENM_CLIPFORMAT			0x00000080
#endif
#define ENM_KEYEVENTS			0x00010000
#define ENM_MOUSEEVENTS 		0x00020000
#define ENM_REQUESTRESIZE		0x00040000
#define ENM_SELCHANGE			0x00080000
#define ENM_DROPFILES			0x00100000
#define ENM_PROTECTED			0x00200000
#define ENM_CORRECTTEXT 		0x00400000		// PenWin specific 
#define ENM_IMECHANGE			0x00800000		// Used by RE1.0 compatibility
#define ENM_LANGCHANGE			0x01000000
#define ENM_OBJECTPOSITIONS 	0x02000000
#define ENM_LINK				0x04000000
#define ENM_LOWFIRTF			0x08000000
#if (_RICHEDIT_VER >= 0x0800)
#define ENM_STARTCOMPOSITION	0x10000000
#define ENM_ENDCOMPOSITION		0x20000000
#define ENM_GROUPTYPINGCHANGE	0x40000000
#define ENM_HIDELINKTOOLTIP		0x80000000
#endif

// New edit control styles 
#define ES_SAVESEL				0x00008000
#define ES_SUNKEN				0x00004000
#define ES_DISABLENOSCROLL		0x00002000
// Same as WS_MAXIMIZE, but that doesn't make sense so we re-use the value 
#define ES_SELECTIONBAR 		0x01000000
// Same as ES_UPPERCASE, but re-used to completely disable OLE drag'n'drop 
#define ES_NOOLEDRAGDROP		0x00000008

// Obsolete Edit Style
#define ES_EX_NOCALLOLEINIT 	0x00000000		// Not supported in RE 2.0/3.0 

// These flags are used in FE Windows 
#define ES_VERTICAL 			0x00400000		// Not supported in RE 2.0/3.0 
#define ES_NOIME				0x00080000
#define ES_SELFIME				0x00040000

// Edit control options 
#define ECO_AUTOWORDSELECTION	0x00000001
#define ECO_AUTOVSCROLL 		0x00000040
#define ECO_AUTOHSCROLL 		0x00000080
#define ECO_NOHIDESEL			0x00000100
#define ECO_READONLY			0x00000800
#define ECO_WANTRETURN			0x00001000
#define ECO_SAVESEL 			0x00008000
#define ECO_SELECTIONBAR		0x01000000
#define ECO_VERTICAL			0x00400000		// FE specific 


// ECO operations 
#define ECOOP_SET				0x0001
#define ECOOP_OR				0x0002
#define ECOOP_AND				0x0003
#define ECOOP_XOR				0x0004

// New word break function actions 
#define WB_CLASSIFY 		3
#define WB_MOVEWORDLEFT 	4
#define WB_MOVEWORDRIGHT	5
#define WB_LEFTBREAK		6
#define WB_RIGHTBREAK		7

// East Asia specific flags 
#define WB_MOVEWORDPREV 	4
#define WB_MOVEWORDNEXT 	5
#define WB_PREVBREAK		6
#define WB_NEXTBREAK		7

#define PC_FOLLOWING		1
#define PC_LEADING			2
#define PC_OVERFLOW 		3
#define PC_DELIMITER		4
#define WBF_WORDWRAP		0x010
#define WBF_WORDBREAK		0x020
#define WBF_OVERFLOW		0x040	
#define WBF_LEVEL1			0x080
#define WBF_LEVEL2			0x100
#define WBF_CUSTOM			0x200

// East Asia specific flags 
#define IMF_FORCENONE			0x0001
#define IMF_FORCEENABLE 		0x0002
#define IMF_FORCEDISABLE		0x0004
#define IMF_CLOSESTATUSWINDOW	0x0008
#define IMF_VERTICAL			0x0020
#define IMF_FORCEACTIVE 		0x0040
#define IMF_FORCEINACTIVE		0x0080
#define IMF_FORCEREMEMBER		0x0100
#define IMF_MULTIPLEEDIT		0x0400

// Word break flags (used with WB_CLASSIFY) 
#define WBF_CLASS			((BYTE) 0x0F)
#define WBF_ISWHITE 		((BYTE) 0x10)
#define WBF_BREAKLINE		((BYTE) 0x20)
#define WBF_BREAKAFTER		((BYTE) 0x40)

// Data types 

#ifdef _WIN32
// Extended edit word break proc (character set aware) 
typedef LONG (*EDITWORDBREAKPROCEX)(char *pchText, LONG cchText, BYTE bCharSet, INT action);
#endif

// All character format measurements are in twips 
typedef struct _charformat
{
	UINT		cbSize;
	DWORD		dwMask;
	DWORD		dwEffects;
	LONG		yHeight;
	LONG		yOffset;
	COLORREF	crTextColor;
	BYTE		bCharSet;
	BYTE		bPitchAndFamily;
	char		szFaceName[LF_FACESIZE];
} CHARFORMATA;

typedef struct _charformatw
{
	UINT		cbSize;
	DWORD		dwMask;
	DWORD		dwEffects;
	LONG		yHeight;
	LONG		yOffset;
	COLORREF	crTextColor;
	BYTE		bCharSet;
	BYTE		bPitchAndFamily;
	WCHAR		szFaceName[LF_FACESIZE];
} CHARFORMATW;

#if (_RICHEDIT_VER >= 0x0200)
#ifdef UNICODE
#define CHARFORMAT CHARFORMATW
#else
#define CHARFORMAT CHARFORMATA
#endif // UNICODE 
#else
#define CHARFORMAT CHARFORMATA
#endif // _RICHEDIT_VER >= 0x0200 

// CHARFORMAT2 structure 

#if defined(__cplusplus)

struct CHARFORMAT2W : _charformatw
{
	WORD		wWeight;			// Font weight (LOGFONT value)
	SHORT		sSpacing;			// Amount to space between letters
	COLORREF	crBackColor;		// Background color
	LCID		lcid;				// Locale ID
#if (_RICHEDIT_VER >= 0x0500)
	union
	{
	  DWORD		dwReserved;			// Name up to 5.0
	  DWORD		dwCookie;	 		// Client cookie opaque to RichEdit
	};
#else
	  DWORD		dwReserved;			// Name up to 5.0
#endif
	SHORT		sStyle; 			// Style handle
	WORD		wKerning;			// Twip size above which to kern char pair
	BYTE		bUnderlineType; 	// Underline type
	BYTE		bAnimation; 		// Animated text like marching ants
	BYTE		bRevAuthor; 		// Revision author index
#if (_RICHEDIT_VER >= 0x0800)
	BYTE		bUnderlineColor;	// Underline color
#endif
};

struct CHARFORMAT2A : _charformat
{
	WORD		wWeight;			// Font weight (LOGFONT value)
	SHORT		sSpacing;			// Amount to space between letters
	COLORREF	crBackColor;		// Background color
	LCID		lcid;				// Locale ID
#if (_RICHEDIT_VER >= 0x0500)
	union
	{
		DWORD		dwReserved;			// Name up to 5.0
		DWORD		dwCookie;	 		// Client cookie opaque to RichEdit
	};
#else
	DWORD		dwReserved;			// Name up to 5.0
#endif
	SHORT		sStyle; 			// Style handle
	WORD		wKerning;			// Twip size above which to kern char pair
	BYTE		bUnderlineType; 	// Underline type
	BYTE		bAnimation; 		// Animated text like marching ants
	BYTE		bRevAuthor; 		// Revision author index
#if (_RICHEDIT_VER >= 0x0800)
	BYTE		bUnderlineColor;	// Underline color
#endif
};

#else	// regular C-style	

typedef struct _charformat2w
{
	UINT		cbSize;
	DWORD		dwMask;
	DWORD		dwEffects;
	LONG		yHeight;
	LONG		yOffset;			// > 0 for superscript, < 0 for subscript 
	COLORREF	crTextColor;
	BYTE		bCharSet;
	BYTE		bPitchAndFamily;
	WCHAR		szFaceName[LF_FACESIZE];
	WORD		wWeight;			// Font weight (LOGFONT value)		
	SHORT		sSpacing;			// Amount to space between letters	
	COLORREF	crBackColor;		// Background color 				
	LCID		lcid;				// Locale ID						
#if (_RICHEDIT_VER >= 0x0500)
	union
	{
		DWORD		dwReserved;			// Name up to 5.0
		DWORD		dwCookie;	 		// Client cookie opaque to RichEdit
	};
#else
	DWORD		dwReserved;			// Name up to 5.0
#endif
	SHORT		sStyle; 			// Style handle 					
	WORD		wKerning;			// Twip size above which to kern char pair
	BYTE		bUnderlineType; 	// Underline type					
	BYTE		bAnimation; 		// Animated text like marching ants 
	BYTE		bRevAuthor; 		// Revision author index			
#if (_RICHEDIT_VER >= 0x0800)
	BYTE		bUnderlineColor;	// Underline color
#endif
} CHARFORMAT2W;

typedef struct _charformat2a
{
	UINT		cbSize;
	DWORD		dwMask;
	DWORD		dwEffects;
	LONG		yHeight;
	LONG		yOffset;			// > 0 for superscript, < 0 for subscript 
	COLORREF	crTextColor;
	BYTE		bCharSet;
	BYTE		bPitchAndFamily;
	char		szFaceName[LF_FACESIZE];
	WORD		wWeight;			// Font weight (LOGFONT value)		
	SHORT		sSpacing;			// Amount to space between letters	
	COLORREF	crBackColor;		// Background color 				
	LCID		lcid;				// Locale ID						
#if (_RICHEDIT_VER >= 0x0500)
	union
	{
		DWORD		dwReserved;			// Name up to 5.0
		DWORD		dwCookie;	 		// Client cookie opaque to RichEdit
	};
#else
	DWORD		dwReserved;			// Name up to 5.0
#endif
	SHORT		sStyle; 			// Style handle 					
	WORD		wKerning;			// Twip size above which to kern char pair
	BYTE		bUnderlineType; 	// Underline type					
	BYTE		bAnimation; 		// Animated text like marching ants 
	BYTE		bRevAuthor; 		// Revision author index			
#if (_RICHEDIT_VER >= 0x0800)
	BYTE		bUnderlineColor;	// Underline color
#endif
} CHARFORMAT2A;

#endif // C++ 

#ifdef UNICODE
#define CHARFORMAT2 CHARFORMAT2W
#else
#define CHARFORMAT2 CHARFORMAT2A
#endif

#define CHARFORMATDELTA 	(sizeof(CHARFORMAT2) - sizeof(CHARFORMAT))


// CFM_COLOR mirrors CFE_AUTOCOLOR, a little hack to easily deal with autocolor

// CHARFORMAT masks 
#define CFM_BOLD		0x00000001
#define CFM_ITALIC		0x00000002
#define CFM_UNDERLINE	0x00000004
#define CFM_STRIKEOUT	0x00000008
#define CFM_PROTECTED	0x00000010
#define CFM_LINK		0x00000020			// Exchange hyperlink extension 
#define CFM_SIZE		0x80000000
#define CFM_COLOR		0x40000000
#define CFM_FACE		0x20000000
#define CFM_OFFSET		0x10000000
#define CFM_CHARSET 	0x08000000

// CHARFORMAT effects 
#define CFE_BOLD			0x00000001
#define CFE_ITALIC			0x00000002
#define CFE_UNDERLINE		0x00000004
#define CFE_STRIKEOUT		0x00000008
#define CFE_PROTECTED		0x00000010
#define CFE_LINK			0x00000020
#define CFE_AUTOCOLOR		0x40000000		// NOTE: this corresponds to 
											// CFM_COLOR, which controls it 
// Masks and effects defined for CHARFORMAT2 -- an (*) indicates
// that the data is stored by RichEdit 2.0/3.0, but not displayed
#define CFM_SMALLCAPS		0x00000040		// (*)	
#define CFM_ALLCAPS 		0x00000080		// Displayed by 3.0 
#define CFM_HIDDEN			0x00000100		// Hidden by 3.0 
#define CFM_OUTLINE 		0x00000200		// (*)	
#define CFM_SHADOW			0x00000400		// (*)	
#define CFM_EMBOSS			0x00000800		// (*)	
#define CFM_IMPRINT 		0x00001000		// (*)	
#define CFM_DISABLED		0x00002000
#define CFM_REVISED 		0x00004000

#define CFM_REVAUTHOR		0x00008000
#define CFE_SUBSCRIPT		0x00010000		// Superscript and subscript are 
#define CFE_SUPERSCRIPT 	0x00020000		//	mutually exclusive			 
#define CFM_ANIMATION		0x00040000		// (*)	
#define CFM_STYLE			0x00080000		// (*)	
#define CFM_KERNING 		0x00100000
#define CFM_SPACING 		0x00200000		// Displayed by 3.0 
#define CFM_WEIGHT			0x00400000
#define CFM_UNDERLINETYPE	0x00800000		// Many displayed by 3.0 
#if (_RICHEDIT_VER >= 0x0600)
#define CFM_COOKIE			0x01000000		// RE 6.0
#endif
#define CFM_LCID			0x02000000
#define CFM_BACKCOLOR		0x04000000		// Higher mask bits defined above

#define CFM_SUBSCRIPT		(CFE_SUBSCRIPT | CFE_SUPERSCRIPT)
#define CFM_SUPERSCRIPT 	CFM_SUBSCRIPT

// CHARFORMAT "ALL" masks
#define CFM_EFFECTS (CFM_BOLD | CFM_ITALIC | CFM_UNDERLINE | CFM_COLOR | \
					 CFM_STRIKEOUT | CFE_PROTECTED | CFM_LINK)
#define CFM_ALL (CFM_EFFECTS | CFM_SIZE | CFM_FACE | CFM_OFFSET | CFM_CHARSET)

#define CFM_EFFECTS2 (CFM_EFFECTS | CFM_DISABLED | CFM_SMALLCAPS | CFM_ALLCAPS \
					| CFM_HIDDEN  | CFM_OUTLINE | CFM_SHADOW | CFM_EMBOSS \
					| CFM_IMPRINT | CFM_REVISED \
					| CFM_SUBSCRIPT | CFM_SUPERSCRIPT | CFM_BACKCOLOR)

#if (_RICHEDIT_VER >= 0x0600)
#define CFM_ALL2	 (CFM_ALL | CFM_EFFECTS2 | CFM_BACKCOLOR | CFM_LCID \
					| CFM_UNDERLINETYPE | CFM_WEIGHT | CFM_REVAUTHOR \
					| CFM_SPACING | CFM_KERNING | CFM_STYLE | CFM_ANIMATION \
					| CFM_COOKIE)
#else
#define CFM_ALL2	 (CFM_ALL | CFM_EFFECTS2 | CFM_BACKCOLOR | CFM_LCID \
					| CFM_UNDERLINETYPE | CFM_WEIGHT | CFM_REVAUTHOR \
					| CFM_SPACING | CFM_KERNING | CFM_STYLE | CFM_ANIMATION)
#endif

#define CFE_SMALLCAPS		CFM_SMALLCAPS
#define CFE_ALLCAPS 		CFM_ALLCAPS
#define CFE_HIDDEN			CFM_HIDDEN
#define CFE_OUTLINE 		CFM_OUTLINE
#define CFE_SHADOW			CFM_SHADOW
#define CFE_EMBOSS			CFM_EMBOSS
#define CFE_IMPRINT 		CFM_IMPRINT
#define CFE_DISABLED		CFM_DISABLED
#define CFE_REVISED 		CFM_REVISED

// CFE_AUTOCOLOR and CFE_AUTOBACKCOLOR correspond to CFM_COLOR and
// CFM_BACKCOLOR, respectively, which control them
#define CFE_AUTOBACKCOLOR	CFM_BACKCOLOR

#define CFM_FONTBOUND				0x00100000
#define	CFM_LINKPROTECTED			0x00800000	// Word hyperlink field
#define CFM_EXTENDED				0x02000000
#define CFM_MATHNOBUILDUP			0x08000000
#define CFM_MATH					0x10000000
#define CFM_MATHORDINARY			0x20000000

#define	CFM_ALLEFFECTS				(CFM_EFFECTS2 | CFM_FONTBOUND | CFM_EXTENDED | CFM_MATHNOBUILDUP | CFM_MATH | CFM_MATHORDINARY)

#define CFE_FONTBOUND				0x00100000	// Font chosen by binder, not user
#define	CFE_LINKPROTECTED			0x00800000
#define CFE_EXTENDED				0x02000000
#define CFE_MATHNOBUILDUP			0x08000000
#define CFE_MATH					0x10000000
#define CFE_MATHORDINARY			0x20000000

// Underline types. RE 1.0 displays only CFU_UNDERLINE
#define CFU_CF1UNDERLINE	0xFF	// Map charformat's bit underline to CF2
#define CFU_INVERT			0xFE	// For IME composition fake a selection
#define CFU_UNDERLINETHICKLONGDASH		18	// (*) display as dash
#define CFU_UNDERLINETHICKDOTTED		17	// (*) display as dot
#define CFU_UNDERLINETHICKDASHDOTDOT	16	// (*) display as dash dot dot
#define CFU_UNDERLINETHICKDASHDOT		15	// (*) display as dash dot
#define CFU_UNDERLINETHICKDASH			14	// (*) display as dash
#define CFU_UNDERLINELONGDASH			13	// (*) display as dash
#define CFU_UNDERLINEHEAVYWAVE			12	// (*) display as wave
#define CFU_UNDERLINEDOUBLEWAVE 		11	// (*) display as wave
#define CFU_UNDERLINEHAIRLINE			10	// (*) display as single	
#define CFU_UNDERLINETHICK				9
#define CFU_UNDERLINEWAVE				8
#define CFU_UNDERLINEDASHDOTDOT 		7
#define CFU_UNDERLINEDASHDOT			6
#define CFU_UNDERLINEDASH				5
#define CFU_UNDERLINEDOTTED 			4
#define CFU_UNDERLINEDOUBLE 			3	// (*) display as single
#define CFU_UNDERLINEWORD				2	// (*) display as single	
#define CFU_UNDERLINE					1
#define CFU_UNDERLINENONE				0

#define yHeightCharPtsMost 1638

// EM_SETCHARFORMAT wParam masks 
#define SCF_SELECTION		0x0001
#define SCF_WORD			0x0002
#define SCF_DEFAULT 		0x0000	// Set default charformat or paraformat
#define SCF_ALL 			0x0004	// Not valid with SCF_SELECTION or SCF_WORD
#define SCF_USEUIRULES		0x0008	// Modifier for SCF_SELECTION; says that
									//	format came from a toolbar, etc., and
									//	hence UI formatting rules should be
									//	used instead of literal formatting
#define SCF_ASSOCIATEFONT	0x0010	// Associate fontname with bCharSet (one
									//	possible for each of Western, ME, FE,
									//	Thai)
#define SCF_NOKBUPDATE		0x0020	// Do not update KB layout for this change
									//	even if autokeyboard is on
#define SCF_ASSOCIATEFONT2	0x0040	// Associate plane-2 (surrogate) font
#if (_RICHEDIT_VER >= 0x0500)
#define SCF_SMARTFONT		0x0080	// Apply font only if it can handle script (5.0)
#define SCF_CHARREPFROMLCID	0x0100	// Get character repertoire from lcid (5.0)

#define SPF_DONTSETDEFAULT	0x0002	// Suppress setting default on empty control
#define SPF_SETDEFAULT		0x0004	// Set the default paraformat
#endif

typedef struct _charrange
{
	LONG	cpMin;
	LONG	cpMax;
} CHARRANGE;

typedef struct _textrange
{
	CHARRANGE chrg;
	LPSTR lpstrText;	// Allocated by caller, zero terminated by RichEdit 
} TEXTRANGEA;

typedef struct _textrangew
{
	CHARRANGE chrg;
	LPWSTR lpstrText;	// Allocated by caller, zero terminated by RichEdit 
} TEXTRANGEW;

#if (_RICHEDIT_VER >= 0x0200)
#ifdef UNICODE
#define TEXTRANGE	TEXTRANGEW
#else
#define TEXTRANGE	TEXTRANGEA
#endif // UNICODE 
#else
#define TEXTRANGE	TEXTRANGEA
#endif // _RICHEDIT_VER >= 0x0200 

typedef DWORD (CALLBACK *EDITSTREAMCALLBACK)(DWORD_PTR dwCookie, LPBYTE pbBuff, LONG cb, LONG *pcb);

typedef struct _editstream
{
	DWORD_PTR dwCookie; 	// User value passed to callback as first parameter 
	DWORD	  dwError;		// Last error 
	EDITSTREAMCALLBACK pfnCallback;
} EDITSTREAM;

// Stream formats. Flags are all in low word, since high word
// gives possible codepage choice. 
#define SF_TEXT 		0x0001
#define SF_RTF			0x0002
#define SF_RTFNOOBJS	0x0003		// Write only 
#define SF_TEXTIZED 	0x0004		// Write only

#define SF_UNICODE		0x0010		// Unicode file (UCS2 little endian) 
#define SF_USECODEPAGE	0x0020		// CodePage given by high word 
#define SF_NCRFORNONASCII 0x40		// Output \uN for nonASCII 
#define SFF_WRITEXTRAPAR  0x80		// Output \par at end

// Flag telling stream operations to operate on selection only 
// EM_STREAMIN	replaces current selection 
// EM_STREAMOUT streams out current selection 
#define SFF_SELECTION	0x8000

// Flag telling stream operations to ignore some FE control words 
// having to do with FE word breaking and horiz vs vertical text. 
// Not used in RichEdit 2.0 and later	
#define SFF_PLAINRTF	0x4000

// Flag telling file stream output (SFF_SELECTION flag not set) to persist 
// \viewscaleN control word. 
#define SFF_PERSISTVIEWSCALE 0x2000

// Flag telling file stream input with SFF_SELECTION flag not set not to 
// close the document 
#define SFF_KEEPDOCINFO 0x1000

// Flag telling stream operations to output in Pocket Word format 
#define SFF_PWD 		0x0800

// 3-bit field specifying the value of N - 1 to use for \rtfN or \pwdN 
#define SF_RTFVAL		0x0700

typedef struct _findtext
{
	CHARRANGE chrg;
	LPCSTR lpstrText;
} FINDTEXTA;

typedef struct _findtextw
{
	CHARRANGE chrg;
	LPCWSTR lpstrText;
} FINDTEXTW;

#if (_RICHEDIT_VER >= 0x0200)
#ifdef UNICODE
#define FINDTEXT	FINDTEXTW
#else
#define FINDTEXT	FINDTEXTA
#endif	// UNICODE 
#else
#define FINDTEXT	FINDTEXTA
#endif // _RICHEDIT_VER >= 0x0200 

typedef struct _findtextexa
{
	CHARRANGE chrg;
	LPCSTR	  lpstrText;
	CHARRANGE chrgText;
} FINDTEXTEXA;

typedef struct _findtextexw
{
	CHARRANGE chrg;
	LPCWSTR   lpstrText;
	CHARRANGE chrgText;
} FINDTEXTEXW;

#if (_RICHEDIT_VER >= 0x0200)
#ifdef UNICODE
#define FINDTEXTEX	FINDTEXTEXW
#else
#define FINDTEXTEX	FINDTEXTEXA
#endif // UNICODE 
#else
#define FINDTEXTEX	FINDTEXTEXA
#endif // _RICHEDIT_VER >= 0x0200 


typedef struct _formatrange
{
	HDC hdc;
	HDC hdcTarget;
	RECT rc;
	RECT rcPage;
	CHARRANGE chrg;
} FORMATRANGE;

// All paragraph measurements are in twips 

#define MAX_TAB_STOPS 32
#define lDefaultTab 720
#define MAX_TABLE_CELLS 63

typedef struct _paraformat
{
	UINT	cbSize;
	DWORD	dwMask;
	WORD	wNumbering;
	union
	{
		WORD	wReserved;
		WORD	wEffects;
	};
	LONG	dxStartIndent;
	LONG	dxRightIndent;
	LONG	dxOffset;
	WORD	wAlignment;
	SHORT	cTabCount;
	LONG	rgxTabs[MAX_TAB_STOPS];
} PARAFORMAT;

#ifdef __cplusplus
struct PARAFORMAT2 : _paraformat
{
	LONG	dySpaceBefore;			// Vertical spacing before para
	LONG	dySpaceAfter;			// Vertical spacing after para
	LONG	dyLineSpacing;			// Line spacing depending on Rule
	SHORT	sStyle; 				// Style handle
	BYTE	bLineSpacingRule;		// Rule for line spacing (see tom.doc)
	BYTE	bOutlineLevel;			// Outline level
	WORD	wShadingWeight; 		// Shading in hundredths of a per cent
	WORD	wShadingStyle;			// Nibble 0: style, 1: cfpat, 2: cbpat
	WORD	wNumberingStart;		// Starting value for numbering
	WORD	wNumberingStyle;		// Alignment, roman/arabic, (), ), ., etc.
	WORD	wNumberingTab;			// Space bet FirstIndent & 1st-line text
	WORD	wBorderSpace;			// Border-text spaces (nbl/bdr in pts)
	WORD	wBorderWidth;			// Pen widths (nbl/bdr in half pts)
	WORD	wBorders;				// Border styles (nibble/border)
};

#else	// Regular C-style	

typedef struct _paraformat2
{
	UINT	cbSize;
	DWORD	dwMask;
	WORD	wNumbering;
	union
	{
		WORD	wReserved;
		WORD	wEffects;
	};
	LONG	dxStartIndent;
	LONG	dxRightIndent;
	LONG	dxOffset;
	WORD	wAlignment;
	SHORT	cTabCount;
	LONG	rgxTabs[MAX_TAB_STOPS];
	LONG	dySpaceBefore;			// Vertical spacing before para 		
	LONG	dySpaceAfter;			// Vertical spacing after para			
	LONG	dyLineSpacing;			// Line spacing depending on Rule		
	SHORT	sStyle; 				// Style handle 						
	BYTE	bLineSpacingRule;		// Rule for line spacing (see tom.doc)	
	BYTE	bOutlineLevel;			// Outline Level						
	WORD	wShadingWeight; 		// Shading in hundredths of a per cent	
	WORD	wShadingStyle;			// Byte 0: style, nib 2: cfpat, 3: cbpat
	WORD	wNumberingStart;		// Starting value for numbering 			
	WORD	wNumberingStyle;		// Alignment, Roman/Arabic, (), ), ., etc.
	WORD	wNumberingTab;			// Space bet 1st indent and 1st-line text
	WORD	wBorderSpace;			// Border-text spaces (nbl/bdr in pts)	
	WORD	wBorderWidth;			// Pen widths (nbl/bdr in half twips)	
	WORD	wBorders;				// Border styles (nibble/border)		
} PARAFORMAT2;

#endif // C++	

// PARAFORMAT mask values 
#define PFM_STARTINDENT 		0x00000001
#define PFM_RIGHTINDENT 		0x00000002
#define PFM_OFFSET				0x00000004
#define PFM_ALIGNMENT			0x00000008
#define PFM_TABSTOPS			0x00000010
#define PFM_NUMBERING			0x00000020
#define PFM_OFFSETINDENT		0x80000000

// PARAFORMAT 2.0 masks and effects 
#define PFM_SPACEBEFORE 		0x00000040
#define PFM_SPACEAFTER			0x00000080
#define PFM_LINESPACING 		0x00000100
#define PFM_STYLE				0x00000400
#define PFM_BORDER				0x00000800	// (*)	
#define PFM_SHADING 			0x00001000	// (*)	
#define PFM_NUMBERINGSTYLE		0x00002000	// RE 3.0	
#define PFM_NUMBERINGTAB		0x00004000	// RE 3.0	
#define PFM_NUMBERINGSTART		0x00008000	// RE 3.0	

#define PFM_RTLPARA 			0x00010000
#define PFM_KEEP				0x00020000	// (*)	
#define PFM_KEEPNEXT			0x00040000	// (*)	
#define PFM_PAGEBREAKBEFORE 		0x00080000	// (*)	
#define PFM_NOLINENUMBER		0x00100000	// (*)	
#define PFM_NOWIDOWCONTROL		0x00200000	// (*)	
#define PFM_DONOTHYPHEN 		0x00400000	// (*)	
#define PFM_SIDEBYSIDE			0x00800000	// (*)	

// The following two paragraph-format properties are read only
#define PFM_COLLAPSED			0x01000000	// RE 3.0 
#define PFM_OUTLINELEVEL		0x02000000	// RE 3.0 
#define PFM_BOX 				0x04000000	// RE 3.0 
#define PFM_RESERVED2			0x08000000	// RE 4.0 
#define PFM_TABLEROWDELIMITER	0x10000000	// RE 4.0 
#define PFM_TEXTWRAPPINGBREAK	0x20000000	// RE 3.0 
#define PFM_TABLE				0x40000000	// RE 3.0 

// PARAFORMAT "ALL" masks
#define PFM_ALL (PFM_STARTINDENT | PFM_RIGHTINDENT | PFM_OFFSET | \
				 PFM_ALIGNMENT	 | PFM_TABSTOPS    | PFM_NUMBERING | \
				 PFM_OFFSETINDENT| PFM_RTLPARA)

// Note: PARAFORMAT has no effects (BiDi RichEdit 1.0 does have PFE_RTLPARA)
#define PFM_EFFECTS (PFM_RTLPARA | PFM_KEEP | PFM_KEEPNEXT | PFM_TABLE \
					| PFM_PAGEBREAKBEFORE | PFM_NOLINENUMBER  \
					| PFM_NOWIDOWCONTROL | PFM_DONOTHYPHEN | PFM_SIDEBYSIDE \
					| PFM_TABLE | PFM_TABLEROWDELIMITER)

#define PFM_ALL2	(PFM_ALL | PFM_EFFECTS | PFM_SPACEBEFORE | PFM_SPACEAFTER \
					| PFM_LINESPACING | PFM_STYLE | PFM_SHADING | PFM_BORDER \
					| PFM_NUMBERINGTAB | PFM_NUMBERINGSTART | PFM_NUMBERINGSTYLE)

#define PFE_RTLPARA 			(PFM_RTLPARA		 >> 16)
#define PFE_KEEP				(PFM_KEEP			 >> 16) // (*)	
#define PFE_KEEPNEXT			(PFM_KEEPNEXT		 >> 16) // (*)	
#define PFE_PAGEBREAKBEFORE 	(PFM_PAGEBREAKBEFORE >> 16) // (*)	
#define PFE_NOLINENUMBER		(PFM_NOLINENUMBER	 >> 16) // (*)	
#define PFE_NOWIDOWCONTROL		(PFM_NOWIDOWCONTROL  >> 16) // (*)	
#define PFE_DONOTHYPHEN 		(PFM_DONOTHYPHEN	 >> 16) // (*)	
#define PFE_SIDEBYSIDE			(PFM_SIDEBYSIDE 	 >> 16) // (*)	
#define PFE_TEXTWRAPPINGBREAK	(PFM_TEXTWRAPPINGBREAK>>16) // (*)	

// The following four effects are read only
#define PFE_COLLAPSED			(PFM_COLLAPSED		 >> 16) // (+)	
#define PFE_BOX 				(PFM_BOX			 >> 16) // (+)	
#define PFE_TABLE				(PFM_TABLE			 >> 16) // Inside table row. RE 3.0 
#define PFE_TABLEROWDELIMITER	(PFM_TABLEROWDELIMITER>>16) // Table row start. RE 4.0 

// PARAFORMAT numbering options 
#define PFN_BULLET		1		// tomListBullet

// PARAFORMAT2 wNumbering options 
#define PFN_ARABIC		2		// tomListNumberAsArabic:	0, 1, 2,	...
#define PFN_LCLETTER	3		// tomListNumberAsLCLetter: a, b, c,	...
#define PFN_UCLETTER	4		// tomListNumberAsUCLetter: A, B, C,	...
#define PFN_LCROMAN 	5		// tomListNumberAsLCRoman:	i, ii, iii, ...
#define PFN_UCROMAN 	6		// tomListNumberAsUCRoman:	I, II, III, ...

// PARAFORMAT2 wNumberingStyle options 
#define PFNS_PAREN		0x000	// default, e.g.,				  1)	
#define PFNS_PARENS 	0x100	// tomListParentheses/256, e.g., (1)	
#define PFNS_PERIOD 	0x200	// tomListPeriod/256, e.g., 	  1.	
#define PFNS_PLAIN		0x300	// tomListPlain/256, e.g.,		  1 	
#define PFNS_NONUMBER	0x400	// Used for continuation w/o number

#define PFNS_NEWNUMBER	0x8000	// Start new number with wNumberingStart		
								// (can be combined with other PFNS_xxx)
// PARAFORMAT alignment options 
#define PFA_LEFT			 1
#define PFA_RIGHT			 2
#define PFA_CENTER			 3

// PARAFORMAT2 alignment options 
#define PFA_JUSTIFY 		 4	// New paragraph-alignment option 2.0 (*) 
#define PFA_FULL_INTERWORD	 4	// These are supported in 3.0 with advanced

// Notification structures 
#ifndef WM_NOTIFY
#define WM_NOTIFY		0x004E

typedef struct _nmhdr
{
	HWND	hwndFrom;
	UINT	idFrom;
	UINT	code;
} NMHDR;
#endif	// !WM_NOTIFY 

typedef struct _msgfilter
{
	NMHDR	nmhdr;
	UINT	msg;
	WPARAM	wParam;
	LPARAM	lParam;
} MSGFILTER;

typedef struct _reqresize
{
	NMHDR nmhdr;
	RECT rc;
} REQRESIZE;

typedef struct _selchange
{
	NMHDR nmhdr;
	CHARRANGE chrg;
	WORD seltyp;
} SELCHANGE;

#if (_RICHEDIT_VER >= 0x0800)
typedef struct _grouptypingchange
{
	NMHDR nmhdr;
	BOOL fGroupTyping;
} GROUPTYPINGCHANGE;
#endif

typedef struct _clipboardformat
{
	NMHDR nmhdr;
	CLIPFORMAT cf;
} CLIPBOARDFORMAT;

#define SEL_EMPTY		0x0000
#define SEL_TEXT		0x0001
#define SEL_OBJECT		0x0002
#define SEL_MULTICHAR	0x0004
#define SEL_MULTIOBJECT 0x0008

// Used with IRichEditOleCallback::GetContextMenu, this flag will be
// passed as a "selection type".  It indicates that a context menu for
// a right-mouse drag drop should be generated.  The IOleObject parameter
// will really be the IDataObject for the drop
#define GCM_RIGHTMOUSEDROP	0x8000

#if (_RICHEDIT_VER >= 0x0800)
typedef struct _getcontextmenuex
{
	CHARRANGE chrg;
	DWORD dwFlags;
	POINT pt;
	void* pvReserved;
} GETCONTEXTMENUEX;

// bits for GETCONTEXTMENUEX::dwFlags
#define GCMF_GRIPPER			0x00000001
#define GCMF_SPELLING			0x00000002	// pSpellingSuggestions is valid
											// and points to the list of spelling suggestions
#define GCMF_TOUCHMENU			0x00004000
#define GCMF_MOUSEMENU			0x00002000
#endif

typedef struct _endropfiles
{
	NMHDR nmhdr;
	HANDLE hDrop;
	LONG cp;
	BOOL fProtected;
} ENDROPFILES;

typedef struct _enprotected
{
	NMHDR nmhdr;
	UINT msg;
	WPARAM wParam;
	LPARAM lParam;
	CHARRANGE chrg;
} ENPROTECTED;

typedef struct _ensaveclipboard
{
	NMHDR nmhdr;
	LONG cObjectCount;
	LONG cch;
} ENSAVECLIPBOARD;

#ifndef MACPORT
typedef struct _enoleopfailed
{
	NMHDR nmhdr;
	LONG iob;
	LONG lOper;
	HRESULT hr;
} ENOLEOPFAILED;
#endif

#define OLEOP_DOVERB	1

typedef struct _objectpositions
{
	NMHDR nmhdr;
	LONG cObjectCount;
	LONG *pcpPositions;
} OBJECTPOSITIONS;

typedef struct _enlink
{
	NMHDR nmhdr;
	UINT msg;
	WPARAM wParam;
	LPARAM lParam;
	CHARRANGE chrg;
} ENLINK;

typedef struct _enlowfirtf
{
	NMHDR nmhdr;
	char *szControl;
} ENLOWFIRTF;

// PenWin specific 
typedef struct _encorrecttext
{
	NMHDR nmhdr;
	CHARRANGE chrg;
	WORD seltyp;
} ENCORRECTTEXT;

// East Asia specific 
typedef struct _punctuation
{
	UINT	iSize;
	LPSTR	szPunctuation;
} PUNCTUATION;

// East Asia specific 
typedef struct _compcolor
{
	COLORREF crText;
	COLORREF crBackground;
	DWORD dwEffects;
}COMPCOLOR;


// Clipboard formats - use as parameter to RegisterClipboardFormat() 
#define CF_RTF			TEXT("Rich Text Format")
#define CF_RTFNOOBJS	TEXT("Rich Text Format Without Objects")
#define CF_RETEXTOBJ	TEXT("RichEdit Text and Objects")

// Paste Special 
typedef struct _repastespecial
{
	DWORD		dwAspect;
	DWORD_PTR	dwParam;
} REPASTESPECIAL;

//	UndoName info 
typedef enum _undonameid
{
	UID_UNKNOWN 	= 0,
	UID_TYPING		= 1,
	UID_DELETE		= 2,
	UID_DRAGDROP	= 3,
	UID_CUT 		= 4,
	UID_PASTE		= 5,
	UID_AUTOTABLE	= 6
} UNDONAMEID;

// Flags for the SETEXTEX data structure 
#define ST_DEFAULT		0
#define ST_KEEPUNDO 	1
#define ST_SELECTION	2
#define ST_NEWCHARS 	4
#if (_RICHEDIT_VER >= 0x0500)
#define ST_UNICODE	 	8
#endif

// EM_SETTEXTEX info; this struct is passed in the wparam of the message 
typedef struct _settextex
{
	DWORD	flags;			// Flags (see the ST_XXX defines)
	UINT	codepage;		// Code page for translation (CP_ACP for sys default,
							//	1200 for Unicode, -1 for control default)
} SETTEXTEX;

// Flags for the GETEXTEX data structure 
#define GT_DEFAULT		0
#define GT_USECRLF		1
#define GT_SELECTION	2
#define GT_RAWTEXT		4
#define GT_NOHIDDENTEXT 8

// EM_GETTEXTEX info; this struct is passed in the wparam of the message 
typedef struct _gettextex
{
	DWORD	cb; 			// Count of bytes in the string
	DWORD	flags;			// Flags (see the GT_XXX defines
	UINT	codepage;		// Code page for translation (CP_ACP for sys default,
							//	1200 for Unicode, -1 for control default)
	LPCSTR	lpDefaultChar;	// Replacement for unmappable chars
	LPBOOL	lpUsedDefChar;	// Pointer to flag set when def char used
} GETTEXTEX;

// Flags for the GETTEXTLENGTHEX data structure
#define GTL_DEFAULT 	0	// Do default (return # of chars)
#define GTL_USECRLF 	1	// Compute answer using CRLFs for paragraphs
#define GTL_PRECISE 	2	// Compute a precise answer
#define GTL_CLOSE		4	// Fast computation of a "close" answer
#define GTL_NUMCHARS	8	// Return number of characters
#define GTL_NUMBYTES	16	// Return number of _bytes_

// EM_GETTEXTLENGTHEX info; this struct is passed in the wparam of the msg 
typedef struct _gettextlengthex
{
	DWORD	flags;			// Flags (see GTL_XXX defines)
	UINT	codepage;		// Code page for translation (CP_ACP for default,
							//	1200 for Unicode)
} GETTEXTLENGTHEX;
	
// BiDi specific features 
typedef struct _bidioptions
{
	UINT	cbSize;
	WORD	wMask;
	WORD	wEffects; 
} BIDIOPTIONS;

// BIDIOPTIONS masks 
#if (_RICHEDIT_VER == 0x0100)
#define BOM_DEFPARADIR			0x0001	// Default paragraph direction (implies alignment) (obsolete) 
#define BOM_PLAINTEXT			0x0002	// Use plain text layout (obsolete) 
#endif // _RICHEDIT_VER == 0x0100 
#define BOM_NEUTRALOVERRIDE 	0x0004	// Override neutral layout (obsolete) 
#define BOM_CONTEXTREADING		0x0008	// Context reading order 
#define BOM_CONTEXTALIGNMENT	0x0010	// Context alignment 
#define BOM_LEGACYBIDICLASS		0x0040	// Legacy Bidi classification (obsolete)
#if (_RICHEDIT_VER >= 0x0600)
#define BOM_UNICODEBIDI			0x0080	// Use Unicode BiDi algorithm
#endif

// BIDIOPTIONS effects 
#if (_RICHEDIT_VER == 0x0100)
#define BOE_RTLDIR				0x0001	// Default paragraph direction (implies alignment) (obsolete) 
#define BOE_PLAINTEXT			0x0002	// Use plain text layout (obsolete) 
#endif // _RICHEDIT_VER == 0x0100 
#define BOE_NEUTRALOVERRIDE 	0x0004	// Override neutral layout (obsolete) 
#define BOE_CONTEXTREADING		0x0008	// Context reading order 
#define BOE_CONTEXTALIGNMENT	0x0010	// Context alignment 
#if (_RICHEDIT_VER >= 0x0800)
#define BOE_FORCERECALC			0x0020	// Force recalc and redraw 
#endif
#define BOE_LEGACYBIDICLASS		0x0040	// Legacy Bidi classification (obsolete) 
#if (_RICHEDIT_VER >= 0x0600)
#define BOE_UNICODEBIDI			0x0080	// Use Unicode BiDi algorithm
#endif

// Additional EM_FINDTEXT[EX] flags 
#define FR_MATCHDIAC					0x20000000
#define FR_MATCHKASHIDA 				0x40000000
#define FR_MATCHALEFHAMZA				0x80000000
	
// UNICODE embedding character 
#ifndef WCH_EMBEDDING
#define WCH_EMBEDDING (WCHAR)0xFFFC
#endif // WCH_EMBEDDING 

// khyph - Kind of hyphenation
typedef enum tagKHYPH
{
	khyphNil,				// No Hyphenation
	khyphNormal,			// Normal Hyphenation
	khyphAddBefore, 		// Add letter before hyphen
	khyphChangeBefore,		// Change letter before hyphen
	khyphDeleteBefore,		// Delete letter before hyphen
	khyphChangeAfter,		// Change letter after hyphen
	khyphDelAndChange		// Delete letter before hyphen and change
							//	letter preceding hyphen
} KHYPH;

typedef struct hyphresult
{
	KHYPH khyph;			// Kind of hyphenation
	long  ichHyph;			// Character which was hyphenated
	WCHAR chHyph;			// Depending on hyphenation type, character added, changed, etc.
} HYPHRESULT;

void WINAPI HyphenateProc(_In_ WCHAR *pszWord, LANGID langid, long ichExceed, HYPHRESULT *phyphresult);
typedef struct tagHyphenateInfo
{
	SHORT cbSize;			// Size of HYPHENATEINFO structure
	SHORT dxHyphenateZone;	// If a space character is closer to the margin
							//	than this value, don't hyphenate (in TWIPs)
	void (WINAPI* pfnHyphenate)(WCHAR*, LANGID, long, HYPHRESULT*);
} HYPHENATEINFO;

#ifdef _WIN32
#include <poppack.h>
#elif !defined(RC_INVOKED)
#pragma pack()
#endif

// Additional class for Richedit 6.0
#if (_RICHEDIT_VER >= 0x0600 )
#ifndef RICHEDIT60_CLASS
#define RICHEDIT60_CLASS	L"RICHEDIT60W"
#endif
#endif

// The following identifiers are deprecated
// They were in previous versions of richedit.h
#define PFA_FULL_NEWSPAPER	 5	// Not implemented
#define PFA_FULL_INTERLETTER 6	// Not implemented
#define PFA_FULL_SCALED 	 7	// Not implemented
#define PFA_FULL_GLYPHS 	 8	// Not implemented

// This is deprecated identifier.
// This constant is used for a discontinued lparam value in EM_AUTOURLDETECT handling
#define	AURL_ENABLEEA			1	// Deprecated: use AURL_ENABLEEAURLS

#define GCM_TOUCHMENU		0x4000	// Deprecated: Use GCMF_TOUCHMENU
#define GCM_MOUSEMENU		0x2000	// Deprecated: Use GCMF_MOUSEMENU

#ifdef __cplusplus
}
#endif	// __cplusplus 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif // !_RICHEDIT_ 
