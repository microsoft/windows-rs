/*	@doc EXTERNAL
 *
 *	@module TEXTSRV.H  Text Service Interface |
 *	
 *	Define interfaces between the Text Services component and the host
 *
 *	Original Author: <nl>
 *		Christian Fortini
 *
 *	History: <nl>
 *		8/1/95	ricksa	Revised interface definition
 *
 *	Copyright (c) 1995-2011, Microsoft Corporation. All rights reserved.
 *
 */

#ifndef _TEXTSERV_H
#define _TEXTSERV_H

#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

EXTERN_C const IID IID_ITextServices;
EXTERN_C const IID IID_ITextHost;
EXTERN_C const IID IID_IRicheditWindowlessAccessibility;
EXTERN_C const IID IID_IRichEditUiaInformation;
EXTERN_C const IID IID_IRicheditUiaOverrides;

// Note: error code is first outside of range reserved for OLE.
#define S_MSG_KEY_IGNORED \
	MAKE_HRESULT(SEVERITY_SUCCESS, FACILITY_ITF, 0x201)

// Enums used by property methods

/*
 *	TXTBACKSTYLE
 *
 *	@enum	Defines different background styles control
 */
enum TXTBACKSTYLE {
	TXTBACK_TRANSPARENT = 0,		//@emem	background should show through
	TXTBACK_OPAQUE,					//@emem	erase background
};


/*
 *	TXTHITRESULT
 *
 *	@enum	Defines different hitresults
 */
enum TXTHITRESULT {
	TXTHITRESULT_NOHIT			= 0,	//@emem	no hit
	TXTHITRESULT_TRANSPARENT	= 1,	//@emem point is within the text's rectangle, but 
										//in a transparent region
	TXTHITRESULT_CLOSE			= 2,	//@emem	point is close to the text
	TXTHITRESULT_HIT			= 3		//@emem dead-on hit
};

/*
 *	TXTNATURALSIZE
 *
 *	@enum	useful values for TxGetNaturalSize.
 *
  *	@xref <mf CTxtEdit::TxGetNaturalSize>
 */
enum TXTNATURALSIZE {
	TXTNS_FITTOCONTENT2	  = 0,			//@emem Get size that fits indented content
	TXTNS_FITTOCONTENT	  = 1,			//@emem Get size that fits content
	TXTNS_ROUNDTOLINE	  = 2,			//@emem Round to nearest whole line
	TXTNS_FITTOCONTENT3	  = 3,			//@emem Get size that fits indented
										//		content + trailing whitespace
	TXTNS_FITTOCONTENTWSP = 4,			//@emem Get size that fits unindented
										//		content + trailing whitespace
	TXTNS_INCLUDELASTLINE = 0x40000000,	//@emem In plain-text ctrl, inc height
										//		of a final CR
	TXTNS_EMU			  = 0x80000000	//@emem EMUs, not pixels are used as
										//		measurement units (both ways)
};

/*
 *	TXTVIEW
 *
 *	@enum	useful values for TxDraw lViewId parameter
 *
  *	@xref <mf CTxtEdit::TxDraw>
 */
enum TXTVIEW { 
	TXTVIEW_ACTIVE = 0,
	TXTVIEW_INACTIVE = -1
};


/*
 *	CHANGETYPE
 *
 *	@enum	used for CHANGENOTIFY.dwChangeType; indicates what happened 
 *			for a particular change.
 */
enum CHANGETYPE
{
	CN_GENERIC		= 0,				//@emem Nothing special happened
	CN_TEXTCHANGED	= 1,				//@emem the text changed
	CN_NEWUNDO		= 2,				//@emem	A new undo action was added
	CN_NEWREDO		= 4					//@emem A new redo action was added
};

/* 
 *	@struct CHANGENOTIFY  |
 *
 *	passed during an EN_CHANGE notification; contains information about
 *	what actually happened for a change.
 */
struct CHANGENOTIFY {
	DWORD	dwChangeType;				//@field TEXT changed, etc
	void *	pvCookieData; 				//@field cookie for the undo action 
										// associated with the change.
};

// The TxGetPropertyBits and OnTxPropertyBitsChange methods can pass the following bits:

// NB!!! Do NOT rely on the ordering of these bits.

#define TXTBIT_RICHTEXT			1		// rich-text control
#define TXTBIT_MULTILINE		2		// single vs multi-line control
#define TXTBIT_READONLY			4		// read only text
#define TXTBIT_SHOWACCELERATOR	8		// underline accelerator character
#define TXTBIT_USEPASSWORD		0x10	// use password char to display text
#define TXTBIT_HIDESELECTION	0x20	// show selection when inactive
#define TXTBIT_SAVESELECTION	0x40	// remember selection when inactive
#define TXTBIT_AUTOWORDSEL		0x80	// auto-word selection 
#define TXTBIT_VERTICAL			0x100	// vertical 
#define TXTBIT_SELBARCHANGE 	0x200	// notification that the selection bar width 
										// has changed.
										// FUTURE: move this bit to the end to
										// maintain the division between 
										// properties and notifications.
#define TXTBIT_WORDWRAP  		0x400	// if set, then multi-line controls
										// should wrap words to fit the available
										// display
#define	TXTBIT_ALLOWBEEP		0x800	// enable/disable beeping
#define TXTBIT_DISABLEDRAG      0x1000  // disable/enable dragging
#define TXTBIT_VIEWINSETCHANGE	0x2000	// the inset changed
#define TXTBIT_BACKSTYLECHANGE	0x4000 
#define TXTBIT_MAXLENGTHCHANGE	0x8000
#define TXTBIT_SCROLLBARCHANGE	0x10000
#define TXTBIT_CHARFORMATCHANGE 0x20000
#define TXTBIT_PARAFORMATCHANGE	0x40000
#define TXTBIT_EXTENTCHANGE		0x80000
#define TXTBIT_CLIENTRECTCHANGE	0x100000	// the client rectangle changed
#define TXTBIT_USECURRENTBKG	0x200000	// tells the renderer to use the current background
											// color rather than the system default for an entire line
#define TXTBIT_NOTHREADREFCOUNT	0x400000	// don't reference TLS data on behalf of this instance
#define TXTBIT_SHOWPASSWORD		0x800000	// Show password string
#define TXTBIT_D2DDWRITE			0x1000000	// Use D2D/DWrite for this instance (and not GDI/Uniscribe).
#define TXTBIT_D2DSIMPLETYPOGRAPHY	0x2000000	// Don't glyph all. Only valid if D2DDWRITE is set.
#define TXTBIT_D2DPIXELSNAPPED		0x4000000	// Don't do subpixel. Only valid if D2DDWRITE is set.
#define TXTBIT_D2DSUBPIXELLINES		0x8000000	// Don't pixel-snap text lines and underline, strikethrough
												//  in the secondary text flow direction (usually vertical)
												//  Only valid if D2DDWRITE is set and D2DPIXELSNAPPED is not set.
#define TXTBIT_FLASHLASTPASSWORDCHAR 0x10000000	// Show last password char momentarily
#define TXTBIT_ADVANCEDINPUT 	0x20000000	// Use advanced input features.

/*
 *	ITextServices
 *	
 * 	@class	An interface extending Microsoft's Text Object Model to provide
 *			extra functionality for windowless operation.  In conjunction
 *			with ITextHost, ITextServices provides the means by which the
 *			the RichEdit control can be used *without* creating a window.
 *
 *	@base	public | IUnknown
 */
class ITextServices : public IUnknown
{
public:

	//@cmember Generic Send Message interface
	virtual HRESULT 	TxSendMessage(
							UINT msg, 
							WPARAM wparam, 
							LPARAM lparam,
							LRESULT *plresult) = 0;
	
	//@cmember Rendering
	virtual HRESULT		TxDraw(	
							DWORD dwDrawAspect,		
							LONG  lindex,			
							void * pvAspect,		 
							DVTARGETDEVICE * ptd,									
							HDC hdcDraw,			
							HDC hicTargetDev,		 
							LPCRECTL lprcBounds,	
							LPCRECTL lprcWBounds,	
							LPRECT lprcUpdate,		
							BOOL (CALLBACK * pfnContinue) (DWORD), 
							DWORD dwContinue,
							LONG lViewId) = 0;	

	//@cmember Horizontal scrollbar support
	virtual HRESULT		TxGetHScroll(
							LONG *plMin, 
							LONG *plMax, 
							LONG *plPos, 
							LONG *plPage,
							BOOL * pfEnabled ) = 0;

	//@cmember Horizontal scrollbar support
	virtual HRESULT		TxGetVScroll(
							LONG *plMin, 
							LONG *plMax, 
							LONG *plPos, 
							LONG *plPage, 
							BOOL * pfEnabled ) = 0;

	//@cmember Setcursor
	virtual HRESULT 	OnTxSetCursor(
							DWORD dwDrawAspect,		
							LONG  lindex,			
							void * pvAspect,		 
							DVTARGETDEVICE * ptd,									
							HDC hdcDraw,			
							HDC hicTargetDev,		 
							LPCRECT lprcClient, 
							INT x, 
							INT y) = 0;

	//@cmember Hit-test
	virtual HRESULT 	TxQueryHitPoint(
							DWORD dwDrawAspect,		
							LONG  lindex,			
							void * pvAspect,		 
							DVTARGETDEVICE * ptd,									
							HDC hdcDraw,			
							HDC hicTargetDev,		 
							LPCRECT lprcClient, 
							INT x, 
							INT y, 
							DWORD * pHitResult) = 0;

	//@cmember Inplace activate notification
	virtual HRESULT		OnTxInPlaceActivate(LPCRECT prcClient) = 0;

	//@cmember Inplace deactivate notification
	virtual HRESULT		OnTxInPlaceDeactivate() = 0;

	//@cmember UI activate notification
	virtual HRESULT		OnTxUIActivate() = 0;

	//@cmember UI deactivate notification
	virtual HRESULT		OnTxUIDeactivate() = 0;

	//@cmember Get text in control
	virtual HRESULT		TxGetText(BSTR *pbstrText) = 0;

	//@cmember Set text in control
	virtual HRESULT		TxSetText(LPCWSTR pszText) = 0;
	
	//@cmember Get x position of 
	virtual HRESULT		TxGetCurTargetX(LONG *) = 0;
	//@cmember Get baseline position
	virtual HRESULT		TxGetBaseLinePos(LONG *) = 0;

	//@cmember Get Size to fit / Natural size
	virtual HRESULT		TxGetNaturalSize(
							DWORD dwAspect,
							HDC hdcDraw,
							HDC hicTargetDev,
							DVTARGETDEVICE *ptd,
							DWORD dwMode, 	
							const SIZEL *psizelExtent,
							LONG *pwidth, 
							LONG *pheight) = 0;

	//@cmember Drag & drop
	virtual HRESULT		TxGetDropTarget( IDropTarget **ppDropTarget ) = 0;

	//@cmember Bulk bit property change notifications
	virtual HRESULT		OnTxPropertyBitsChange(DWORD dwMask, DWORD dwBits) = 0;

	//@cmember Fetch the cached drawing size (logical not physical)
	virtual	HRESULT		TxGetCachedSize(DWORD *pdwWidth, DWORD *pdwHeight)=0;
};

// These definition are used with SES_LOGOCALCARET to delegate caret creation to a host through TxCreateCaret
// With SES_LOGICALCARET, the bitmap passed to the host is actually a set of flags
// The host can use these definitions to convert the bitmap handle passed to TxCreateCaret into caret information.
// Only logical carets are provided to D2D hosts
enum CARET_FLAGS
{
	CARET_NONE		= 0,	// Normal Western caret (blinking vertical bar)
	CARET_CUSTOM	= 1,	// Adorned caret, only set currently for RTL
	CARET_RTL		= 2,
	CARET_ITALIC	= 32,
	CARET_NULL		= 64,	// Nondegenerate selection: use empty bitmap
	CARET_ROTATE90	= 128	// Rotate 90 degrees clockwise
};

union CARET_INFO
{
	HBITMAP hbitmap;
	CARET_FLAGS caretFlags;
	CARET_INFO(HBITMAP hbmp) : hbitmap(hbmp) {};
	CARET_INFO(CARET_FLAGS flags) : caretFlags(flags) {};
	operator const HBITMAP () const { return hbitmap; };
	operator const CARET_FLAGS () const { return caretFlags; };
};

/*
 *	ITextHost
 *	
 * 	@class	Interface to be used by text services to obtain text host services
 *
 *	@base	public | IUnknown 
 */
class ITextHost : public IUnknown
{
public:

	//@cmember Get the DC for the host
	virtual HDC			TxGetDC() = 0;

	//@cmember Release the DC gotten from the host
	virtual INT			TxReleaseDC(HDC hdc) = 0;
	
	//@cmember Show the scroll bar
	virtual BOOL		TxShowScrollBar(INT fnBar, BOOL fShow) = 0;

	//@cmember Enable the scroll bar
	virtual BOOL		TxEnableScrollBar (INT fuSBFlags, INT fuArrowflags) = 0;

	//@cmember Set the scroll range
	virtual BOOL		TxSetScrollRange(
							INT fnBar, 
							LONG nMinPos, 
							INT nMaxPos, 
							BOOL fRedraw) = 0;

	//@cmember Set the scroll position
	virtual BOOL		TxSetScrollPos (INT fnBar, INT nPos, BOOL fRedraw) = 0;

	//@cmember InvalidateRect
	virtual void		TxInvalidateRect(LPCRECT prc, BOOL fMode) = 0;

	//@cmember Send a WM_PAINT to the window
	virtual void		TxViewChange(BOOL fUpdate) = 0;
	
	//@cmember Create the caret
	virtual BOOL		TxCreateCaret(HBITMAP hbmp, INT xWidth, INT yHeight) = 0;

	//@cmember Show the caret
	virtual BOOL		TxShowCaret(BOOL fShow) = 0;

	//@cmember Set the caret position
	virtual BOOL		TxSetCaretPos(INT x, INT y) = 0;

	//@cmember Create a timer with the specified timeout
	virtual BOOL		TxSetTimer(UINT idTimer, UINT uTimeout) = 0;

	//@cmember Destroy a timer
	virtual void		TxKillTimer(UINT idTimer) = 0;

	//@cmember Scroll the content of the specified window's client area
	virtual void		TxScrollWindowEx (
							INT dx, 
							INT dy, 
							LPCRECT lprcScroll, 
							LPCRECT lprcClip,
							HRGN hrgnUpdate, 
							LPRECT lprcUpdate, 
							UINT fuScroll) = 0;
	
	//@cmember Get mouse capture
	virtual void		TxSetCapture(BOOL fCapture) = 0;

	//@cmember Set the focus to the text window
	virtual void		TxSetFocus() = 0;

	//@cmember Establish a new cursor shape
	virtual void		TxSetCursor(HCURSOR hcur, BOOL fText) = 0;

	//@cmember Converts screen coordinates of a specified point to the client coordinates 
	virtual BOOL		TxScreenToClient (LPPOINT lppt) = 0;

	//@cmember Converts the client coordinates of a specified point to screen coordinates
	virtual BOOL		TxClientToScreen (LPPOINT lppt) = 0;

	//@cmember Request host to activate text services
	virtual HRESULT		TxActivate( LONG * plOldState ) = 0;

	//@cmember Request host to deactivate text services
	virtual HRESULT		TxDeactivate( LONG lNewState ) = 0;

	//@cmember Retrieves the coordinates of a window's client area
	virtual HRESULT		TxGetClientRect(LPRECT prc) = 0;

	//@cmember Get the view rectangle relative to the inset
	virtual HRESULT		TxGetViewInset(LPRECT prc) = 0;

	//@cmember Get the default character format for the text
	virtual HRESULT		TxGetCharFormat(const CHARFORMATW **ppCF ) = 0;

	//@cmember Get the default paragraph format for the text
	virtual HRESULT		TxGetParaFormat(const PARAFORMAT **ppPF) = 0;

	//@cmember Get the background color for the window
	virtual COLORREF	TxGetSysColor(int nIndex) = 0;

	//@cmember Get the background (either opaque or transparent)
	virtual HRESULT		TxGetBackStyle(TXTBACKSTYLE *pstyle) = 0;

	//@cmember Get the maximum length for the text
	virtual HRESULT		TxGetMaxLength(DWORD *plength) = 0;

	//@cmember Get the bits representing requested scroll bars for the window
	virtual HRESULT		TxGetScrollBars(DWORD *pdwScrollBar) = 0;

	//@cmember Get the character to display for password input
	virtual HRESULT		TxGetPasswordChar(_Out_ TCHAR *pch) = 0;

	//@cmember Get the accelerator character
	virtual HRESULT		TxGetAcceleratorPos(LONG *pcp) = 0;

	//@cmember Get the native size
	virtual HRESULT		TxGetExtent(LPSIZEL lpExtent) = 0;

	//@cmember Notify host that default character format has changed
	virtual HRESULT		OnTxCharFormatChange (const CHARFORMATW * pCF) = 0;

	//@cmember Notify host that default paragraph format has changed
	virtual HRESULT		OnTxParaFormatChange (const PARAFORMAT * pPF) = 0;

	//@cmember Bulk access to bit properties
	virtual HRESULT		TxGetPropertyBits(DWORD dwMask, DWORD *pdwBits) = 0;

	//@cmember Notify host of events
	virtual HRESULT		TxNotify(DWORD iNotify, void *pv) = 0;

	// East Asia Methods for getting the Input Context
	virtual HIMC		TxImmGetContext() = 0;
	virtual void		TxImmReleaseContext( HIMC himc ) = 0;

	//@cmember Returns HIMETRIC size of the control bar.
	virtual HRESULT		TxGetSelectionBarWidth (LONG *lSelBarWidth) = 0;
};


// Forward declaration to decrease the includes
interface IRawElementProviderWindowlessSite;
interface IRawElementProviderSimple;

/*
 *	IRicheditWindowlessAccessibility
 *	
 * 	@class	Interface to be used by hosts to get the UIA object for Text Services 
 *
 *	@base	public | IUnknown 
 */
interface DECLSPEC_UUID("983E572D-20CD-460B-9104-83111592DD10") IRicheditWindowlessAccessibility : public IUnknown
{
	virtual HRESULT STDMETHODCALLTYPE CreateProvider(
		IRawElementProviderWindowlessSite *pSite, 
		IRawElementProviderSimple **ppProvider) = 0;
};

struct UiaRect;

/*
 *	IRichEditUiaInformation
 *	
 * 	@class	Interface to be used by blobs to retrieve UIA informatoin
 *
 *	@base	public | IUnknown 
 */
interface DECLSPEC_UUID("23969A9D-8546-4032-A1BB-73750CBF3333") IRichEditUiaInformation : public IUnknown
{
	virtual HRESULT STDMETHODCALLTYPE GetBoundaryRectangle(UiaRect *pUiaRect) = 0;

	virtual HRESULT STDMETHODCALLTYPE IsVisible() = 0;
};

typedef int PROPERTYID;

/*
 *	IRicheditUiaOverrides
 *
 * 	@class	Interface to be used by containers of windowless RichEdit controls to allow
 *			the container to override the RichEdit's UIA properties.
 *
 *	@base	public | IUnknown 
 */
interface IRicheditUiaOverrides : public IUnknown
{
	virtual HRESULT STDMETHODCALLTYPE GetPropertyOverrideValue(
		PROPERTYID propertyId,
		VARIANT *pRetValue) = 0;
};

//+-----------------------------------------------------------------------
// 	Factories
//------------------------------------------------------------------------

// Text Services factory
STDAPI CreateTextServices(
	IUnknown *punkOuter,
	ITextHost *pITextHost,
	IUnknown **ppUnk);

typedef HRESULT (STDAPICALLTYPE * PCreateTextServices)(
	IUnknown *punkOuter,
	ITextHost *pITextHost,
	IUnknown **ppUnk);

STDAPI ShutdownTextServices(IUnknown *pTextServices);

typedef HRESULT (STDAPICALLTYPE * PShutdownTextServices)(
	IUnknown *pTextServices);

//+-----------------------------------------------------------------------
// 	Extended Windowless Interfaces
//------------------------------------------------------------------------

EXTERN_C const IID IID_ITextServices2;
EXTERN_C const IID IID_ITextHost2;

interface ID2D1RenderTarget;

/*
 *	class ITextHost2
 *
 *	@class	An optional extension to ITextHost which provides functionality
 *			necessary to allow TextServices to embed OLE objects
 *
 *	@base	public | ITextHost 
 */
class ITextHost2 : public ITextHost
{
public:					//@cmember Is a double click in the message queue?
	virtual BOOL		TxIsDoubleClickPending() = 0; 

						//@cmember Get the overall window for this control	 
	virtual HRESULT		TxGetWindow(HWND *phwnd) = 0;

						//@cmember Set control window to foreground
	virtual HRESULT		TxSetForegroundWindow() = 0;

						//@cmember Set control window to foreground
	virtual HPALETTE	TxGetPalette() = 0;

						//@cmember Get East Asian flags
	virtual HRESULT		TxGetEastAsianFlags(LONG *pFlags) = 0;

						//@cmember Routes the cursor change to the winhost
	virtual HCURSOR		TxSetCursor2(HCURSOR hcur, BOOL bText) = 0;

						//@cmember Notification that text services is freed
	virtual void		TxFreeTextServicesNotification() = 0;

						//@cmember Get Edit Style flags
	virtual HRESULT		TxGetEditStyle(DWORD dwItem, DWORD *pdwData) = 0;

						//@cmember Get Window Style bits
	virtual HRESULT		TxGetWindowStyles(DWORD *pdwStyle, DWORD *pdwExStyle) = 0;

						//@cmember Show / hide drop caret (D2D-only)
	virtual HRESULT		TxShowDropCaret(BOOL fShow, HDC hdc, LPCRECT prc) = 0;

						//@cmember Destroy caret (D2D-only)
	virtual HRESULT 	TxDestroyCaret() = 0;

						//@cmember Get Horizontal scroll extent
	virtual HRESULT		TxGetHorzExtent(LONG *plHorzExtent) = 0;
};

// Various flags for TxGetEditStyle data
#define TXES_ISDIALOG		1

/*
 *	ITextServices2
 *	
 * 	@class	Extends TxGetNaturalSize to return ascent of single-line controls 
 *
 *	@base	public | ITextServices
 */
class ITextServices2 : public ITextServices
{
public:
	//@cmember Get Size to fit / Natural size and single-line ascent
	virtual HRESULT		TxGetNaturalSize2(
							DWORD dwAspect,
							HDC hdcDraw,
							HDC hicTargetDev,
							DVTARGETDEVICE *ptd,
							DWORD dwMode, 	
							const SIZEL *psizelExtent,
							LONG *pwidth, 
							LONG *pheight, 
							LONG *pascent) = 0;

	virtual HRESULT		TxDrawD2D(
							ID2D1RenderTarget* pRenderTarget,
							LPCRECTL lprcBounds,
							LPRECT lprcUpdate,
							LONG lViewId) = 0;
};

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif // _TEXTSERV_H
