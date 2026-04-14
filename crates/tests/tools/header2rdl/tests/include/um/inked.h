

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


#ifndef __inked_h__
#define __inked_h__

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

#ifndef __IInkEdit_FWD_DEFINED__
#define __IInkEdit_FWD_DEFINED__
typedef interface IInkEdit IInkEdit;

#endif 	/* __IInkEdit_FWD_DEFINED__ */


#ifndef ___IInkEditEvents_FWD_DEFINED__
#define ___IInkEditEvents_FWD_DEFINED__
typedef interface _IInkEditEvents _IInkEditEvents;

#endif 	/* ___IInkEditEvents_FWD_DEFINED__ */


#ifndef __InkEdit_FWD_DEFINED__
#define __InkEdit_FWD_DEFINED__

#ifdef __cplusplus
typedef class InkEdit InkEdit;
#else
typedef struct InkEdit InkEdit;
#endif /* __cplusplus */

#endif 	/* __InkEdit_FWD_DEFINED__ */


/* header files for imported files */
#include "oaidl.h"
#include "ocidl.h"
#include "msinkaut.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_inked_0000_0000 */
/* [local] */ 


///////////////////////////////////////////////////////////////////////////////
//
// InkEdit Win32 API
//
///////////////////////////////////////////////////////////////////////////////

#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
#define INKEDIT_CLASSW  L"INKEDIT"

#ifdef UNICODE
#define INKEDIT_CLASS   INKEDIT_CLASSW
#else
#define INKEDIT_CLASS   "INKEDIT"
#endif

// InkEdit Constants

// InkEdit Structures

struct IEC_STROKEINFO
{
     NMHDR nmhdr; 
     IInkCursor * Cursor;
     IInkStrokeDisp * Stroke;
};

struct IEC_GESTUREINFO
{
     NMHDR nmhdr; 
     IInkCursor * Cursor;
     IInkStrokes * Strokes;
     VARIANT Gestures;
};

struct IEC_RECOGNITIONRESULTINFO
{
     NMHDR nmhdr; 
     IInkRecognitionResult * RecognitionResult;
};

// InkEdit messages

#define IEC__BASE           (WM_USER + 0x0200)
#define EM_GETINKMODE       (IEC__BASE + 1)
#define EM_SETINKMODE       (IEC__BASE + 2)
#define EM_GETINKINSERTMODE (IEC__BASE + 3)
#define EM_SETINKINSERTMODE (IEC__BASE + 4)
#define EM_GETDRAWATTR      (IEC__BASE + 5)
#define EM_SETDRAWATTR      (IEC__BASE + 6)
#define EM_GETRECOTIMEOUT   (IEC__BASE + 7)
#define EM_SETRECOTIMEOUT   (IEC__BASE + 8)
#define EM_GETGESTURESTATUS (IEC__BASE + 9)
#define EM_SETGESTURESTATUS (IEC__BASE + 10)
#define EM_GETRECOGNIZER    (IEC__BASE + 11)
#define EM_SETRECOGNIZER    (IEC__BASE + 12)
#define EM_GETFACTOID       (IEC__BASE + 13)
#define EM_SETFACTOID       (IEC__BASE + 14)
#define EM_GETSELINK        (IEC__BASE + 15)
#define EM_SETSELINK        (IEC__BASE + 16)
#define EM_GETMOUSEICON     (IEC__BASE + 17)
#define EM_SETMOUSEICON     (IEC__BASE + 18)
#define EM_GETMOUSEPOINTER  (IEC__BASE + 19)
#define EM_SETMOUSEPOINTER  (IEC__BASE + 20)
#define EM_GETSTATUS        (IEC__BASE + 21)
#define EM_RECOGNIZE        (IEC__BASE + 22)
#define EM_GETUSEMOUSEFORINPUT   (IEC__BASE + 23)
#define EM_SETUSEMOUSEFORINPUT   (IEC__BASE + 24)
#define EM_SETSELINKDISPLAYMODE  (IEC__BASE + 25)
#define EM_GETSELINKDISPLAYMODE  (IEC__BASE + 26)

///////////////////////////////////////////////////////////////////////////////

// InkEdit notifications

#define IECN__BASE             (0x0800)
#define IECN_STROKE            (IECN__BASE + 1)
#define IECN_GESTURE           (IECN__BASE + 2)
#define IECN_RECOGNITIONRESULT (IECN__BASE + 3)




extern RPC_IF_HANDLE __MIDL_itf_inked_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_inked_0000_0000_v0_0_s_ifspec;


#ifndef __INKEDLib_LIBRARY_DEFINED__
#define __INKEDLib_LIBRARY_DEFINED__

/* library INKEDLib */
/* [helpcontext][helpstring][version][uuid] */ 

typedef /* [helpcontext][helpstring] */ 
enum MouseButton
    {
        NO_BUTTON	= 0,
        LEFT_BUTTON	= 0x1,
        RIGHT_BUTTON	= 0x2,
        MIDDLE_BUTTON	= 0x4
    } 	MouseButton;

typedef /* [helpcontext][helpstring] */ 
enum SelAlignmentConstants
    {
        rtfLeft	= 0,
        rtfRight	= 1,
        rtfCenter	= 2
    } 	SelAlignmentConstants;

typedef /* [hidden] */ 
enum DISPID_InkEdit
    {
        DISPID_Text	= DISPID_VALUE,
        DISPID_TextRTF	= ( DISPID_Text + 1 ) ,
        DISPID_Hwnd	= ( DISPID_TextRTF + 1 ) ,
        DISPID_DisableNoScroll	= ( DISPID_Hwnd + 1 ) ,
        DISPID_Locked	= ( DISPID_DisableNoScroll + 1 ) ,
        DISPID_Enabled	= ( DISPID_Locked + 1 ) ,
        DISPID_MaxLength	= ( DISPID_Enabled + 1 ) ,
        DISPID_MultiLine	= ( DISPID_MaxLength + 1 ) ,
        DISPID_ScrollBars	= ( DISPID_MultiLine + 1 ) ,
        DISPID_RTSelStart	= ( DISPID_ScrollBars + 1 ) ,
        DISPID_RTSelLength	= ( DISPID_RTSelStart + 1 ) ,
        DISPID_RTSelText	= ( DISPID_RTSelLength + 1 ) ,
        DISPID_SelAlignment	= ( DISPID_RTSelText + 1 ) ,
        DISPID_SelBold	= ( DISPID_SelAlignment + 1 ) ,
        DISPID_SelCharOffset	= ( DISPID_SelBold + 1 ) ,
        DISPID_SelColor	= ( DISPID_SelCharOffset + 1 ) ,
        DISPID_SelFontName	= ( DISPID_SelColor + 1 ) ,
        DISPID_SelFontSize	= ( DISPID_SelFontName + 1 ) ,
        DISPID_SelItalic	= ( DISPID_SelFontSize + 1 ) ,
        DISPID_SelRTF	= ( DISPID_SelItalic + 1 ) ,
        DISPID_SelUnderline	= ( DISPID_SelRTF + 1 ) ,
        DISPID_DragIcon	= ( DISPID_SelUnderline + 1 ) ,
        DISPID_Status	= ( DISPID_DragIcon + 1 ) ,
        DISPID_UseMouseForInput	= ( DISPID_Status + 1 ) ,
        DISPID_InkMode	= ( DISPID_UseMouseForInput + 1 ) ,
        DISPID_InkInsertMode	= ( DISPID_InkMode + 1 ) ,
        DISPID_RecoTimeout	= ( DISPID_InkInsertMode + 1 ) ,
        DISPID_DrawAttr	= ( DISPID_RecoTimeout + 1 ) ,
        DISPID_Recognizer	= ( DISPID_DrawAttr + 1 ) ,
        DISPID_Factoid	= ( DISPID_Recognizer + 1 ) ,
        DISPID_SelInk	= ( DISPID_Factoid + 1 ) ,
        DISPID_SelInksDisplayMode	= ( DISPID_SelInk + 1 ) ,
        DISPID_Recognize	= ( DISPID_SelInksDisplayMode + 1 ) ,
        DISPID_GetGestStatus	= ( DISPID_Recognize + 1 ) ,
        DISPID_SetGestStatus	= ( DISPID_GetGestStatus + 1 ) ,
        DISPID_Refresh	= ( DISPID_SetGestStatus + 1 ) 
    } 	DISPID_InkEdit;

typedef /* [hidden] */ 
enum DISPID_InkEditEvents
    {
        DISPID_IeeChange	= 1,
        DISPID_IeeSelChange	= 2,
        DISPID_IeeKeyDown	= 3,
        DISPID_IeeKeyUp	= 4,
        DISPID_IeeMouseUp	= 5,
        DISPID_IeeMouseDown	= 6,
        DISPID_IeeKeyPress	= 7,
        DISPID_IeeDblClick	= 8,
        DISPID_IeeClick	= 9,
        DISPID_IeeMouseMove	= 10,
        DISPID_IeeCursorDown	= 21,
        DISPID_IeeStroke	= 22,
        DISPID_IeeGesture	= 23,
        DISPID_IeeRecognitionResult	= 24
    } 	DISPID_InkEditEvents;


EXTERN_C const IID LIBID_INKEDLib;

#ifndef __IInkEdit_INTERFACE_DEFINED__
#define __IInkEdit_INTERFACE_DEFINED__

/* interface IInkEdit */
/* [unique][helpcontext][helpstring][dual][uuid][object] */ 

typedef /* [helpcontext][helpstring] */ 
enum InkMode
    {
        IEM_Disabled	= 0,
        IEM_Ink	= 1,
        IEM_InkAndGesture	= 2
    } 	InkMode;

typedef /* [helpcontext][helpstring] */ 
enum InkInsertMode
    {
        IEM_InsertText	= 0,
        IEM_InsertInk	= 1
    } 	InkInsertMode;

typedef /* [helpcontext][helpstring] */ 
enum InkEditStatus
    {
        IES_Idle	= 0,
        IES_Collecting	= 1,
        IES_Recognizing	= 2
    } 	InkEditStatus;

typedef /* [helpcontext][helpstring] */ 
enum InkDisplayMode
    {
        IDM_Ink	= 0,
        IDM_Text	= 1
    } 	InkDisplayMode;

typedef /* [helpcontext][helpstring] */ 
enum AppearanceConstants
    {
        rtfFlat	= 0,
        rtfThreeD	= 1
    } 	AppearanceConstants;

typedef /* [helpcontext][helpstring] */ 
enum BorderStyleConstants
    {
        rtfNoBorder	= 0,
        rtfFixedSingle	= 1
    } 	BorderStyleConstants;

typedef /* [helpcontext][helpstring] */ 
enum ScrollBarsConstants
    {
        rtfNone	= 0,
        rtfHorizontal	= 1,
        rtfVertical	= 2,
        rtfBoth	= 3
    } 	ScrollBarsConstants;


EXTERN_C const IID IID_IInkEdit;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("F2127A19-FBFB-4AED-8464-3F36D78CFEFB")
    IInkEdit : public IDispatch
    {
    public:
        virtual /* [helpcontext][helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Status( 
            /* [retval][out] */ __RPC__out InkEditStatus *pStatus) = 0;
        
        virtual /* [helpcontext][helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_UseMouseForInput( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pVal) = 0;
        
        virtual /* [helpcontext][helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_UseMouseForInput( 
            /* [defaultvalue][in] */ VARIANT_BOOL newVal = ( VARIANT_BOOL  )-1) = 0;
        
        virtual /* [helpcontext][helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_InkMode( 
            /* [retval][out] */ __RPC__out InkMode *pVal) = 0;
        
        virtual /* [helpcontext][helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_InkMode( 
            /* [defaultvalue][in] */ InkMode newVal = IEM_InkAndGesture) = 0;
        
        virtual /* [helpcontext][helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_InkInsertMode( 
            /* [retval][out] */ __RPC__out InkInsertMode *pVal) = 0;
        
        virtual /* [helpcontext][helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_InkInsertMode( 
            /* [defaultvalue][in] */ InkInsertMode newVal = IEM_InsertText) = 0;
        
        virtual /* [helpcontext][helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_DrawingAttributes( 
            /* [retval][out] */ __RPC__deref_out_opt IInkDrawingAttributes **pVal) = 0;
        
        virtual /* [helpcontext][helpstring][propputref][id] */ HRESULT STDMETHODCALLTYPE putref_DrawingAttributes( 
            /* [in] */ __RPC__in_opt IInkDrawingAttributes *newVal) = 0;
        
        virtual /* [helpcontext][helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_RecognitionTimeout( 
            /* [retval][out] */ __RPC__out long *pVal) = 0;
        
        virtual /* [helpcontext][helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_RecognitionTimeout( 
            /* [in] */ long newVal) = 0;
        
        virtual /* [helpcontext][helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Recognizer( 
            /* [retval][out] */ __RPC__deref_out_opt IInkRecognizer **pVal) = 0;
        
        virtual /* [helpcontext][helpstring][propputref][id] */ HRESULT STDMETHODCALLTYPE putref_Recognizer( 
            /* [in] */ __RPC__in_opt IInkRecognizer *newVal) = 0;
        
        virtual /* [helpcontext][helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Factoid( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal) = 0;
        
        virtual /* [helpcontext][helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_Factoid( 
            /* [in] */ __RPC__in BSTR newVal) = 0;
        
        virtual /* [helpcontext][helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_SelInks( 
            /* [retval][out] */ __RPC__out VARIANT *pSelInk) = 0;
        
        virtual /* [helpcontext][helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_SelInks( 
            /* [in] */ VARIANT SelInk) = 0;
        
        virtual /* [helpcontext][helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_SelInksDisplayMode( 
            /* [retval][out] */ __RPC__out InkDisplayMode *pInkDisplayMode) = 0;
        
        virtual /* [helpcontext][helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_SelInksDisplayMode( 
            /* [in] */ InkDisplayMode InkDisplayMode) = 0;
        
        virtual /* [helpcontext][helpstring][id] */ HRESULT STDMETHODCALLTYPE Recognize( void) = 0;
        
        virtual /* [helpcontext][helpstring][id] */ HRESULT STDMETHODCALLTYPE GetGestureStatus( 
            /* [in] */ InkApplicationGesture Gesture,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pListen) = 0;
        
        virtual /* [helpcontext][helpstring][id] */ HRESULT STDMETHODCALLTYPE SetGestureStatus( 
            /* [in] */ InkApplicationGesture Gesture,
            /* [in] */ VARIANT_BOOL Listen) = 0;
        
        virtual /* [helpcontext][helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_BackColor( 
            /* [in] */ OLE_COLOR clr) = 0;
        
        virtual /* [helpcontext][helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_BackColor( 
            /* [retval][out] */ __RPC__out OLE_COLOR *pclr) = 0;
        
        virtual /* [helpcontext][helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Appearance( 
            /* [retval][out] */ __RPC__out AppearanceConstants *pAppearance) = 0;
        
        virtual /* [helpcontext][helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_Appearance( 
            /* [in] */ AppearanceConstants pAppearance) = 0;
        
        virtual /* [helpcontext][helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_BorderStyle( 
            /* [retval][out] */ __RPC__out BorderStyleConstants *pBorderStyle) = 0;
        
        virtual /* [helpcontext][helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_BorderStyle( 
            /* [in] */ BorderStyleConstants pBorderStyle) = 0;
        
        virtual /* [helpcontext][helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Hwnd( 
            /* [retval][out] */ __RPC__out OLE_HANDLE *pohHwnd) = 0;
        
        virtual /* [helpcontext][helpstring][bindable][propget][id] */ HRESULT STDMETHODCALLTYPE get_Font( 
            /* [retval][out] */ __RPC__deref_out_opt IFontDisp **ppFont) = 0;
        
        virtual /* [helpcontext][helpstring][bindable][propputref][id] */ HRESULT STDMETHODCALLTYPE putref_Font( 
            /* [in] */ __RPC__in_opt IFontDisp *ppFont) = 0;
        
        virtual /* [helpcontext][helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Text( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrText) = 0;
        
        virtual /* [helpcontext][helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_Text( 
            /* [in] */ __RPC__in BSTR pbstrText) = 0;
        
        virtual /* [helpcontext][helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_MouseIcon( 
            /* [retval][out] */ __RPC__deref_out_opt IPictureDisp **MouseIcon) = 0;
        
        virtual /* [helpcontext][helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_MouseIcon( 
            /* [in] */ __RPC__in_opt IPictureDisp *MouseIcon) = 0;
        
        virtual /* [helpcontext][helpstring][propputref][id] */ HRESULT STDMETHODCALLTYPE putref_MouseIcon( 
            /* [in] */ __RPC__in_opt IPictureDisp *MouseIcon) = 0;
        
        virtual /* [helpcontext][helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_MousePointer( 
            /* [retval][out] */ __RPC__out InkMousePointer *MousePointer) = 0;
        
        virtual /* [helpcontext][helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_MousePointer( 
            /* [in] */ InkMousePointer MousePointer) = 0;
        
        virtual /* [helpcontext][helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Locked( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pVal) = 0;
        
        virtual /* [helpcontext][helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_Locked( 
            /* [in] */ VARIANT_BOOL newVal) = 0;
        
        virtual /* [helpcontext][helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Enabled( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pVal) = 0;
        
        virtual /* [helpcontext][helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_Enabled( 
            /* [in] */ VARIANT_BOOL newVal) = 0;
        
        virtual /* [helpcontext][helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_MaxLength( 
            /* [retval][out] */ __RPC__out long *plMaxLength) = 0;
        
        virtual /* [helpcontext][helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_MaxLength( 
            /* [in] */ long lMaxLength) = 0;
        
        virtual /* [helpcontext][helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_MultiLine( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pVal) = 0;
        
        virtual /* [helpcontext][helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_MultiLine( 
            /* [in] */ VARIANT_BOOL newVal) = 0;
        
        virtual /* [helpcontext][helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_ScrollBars( 
            /* [retval][out] */ __RPC__out ScrollBarsConstants *pVal) = 0;
        
        virtual /* [helpcontext][helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_ScrollBars( 
            /* [in] */ ScrollBarsConstants newVal) = 0;
        
        virtual /* [helpcontext][helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_DisableNoScroll( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pVal) = 0;
        
        virtual /* [helpcontext][helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_DisableNoScroll( 
            /* [in] */ VARIANT_BOOL newVal) = 0;
        
        virtual /* [helpcontext][helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_SelAlignment( 
            /* [retval][out] */ __RPC__out VARIANT *pvarSelAlignment) = 0;
        
        virtual /* [helpcontext][helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_SelAlignment( 
            /* [in] */ VARIANT pvarSelAlignment) = 0;
        
        virtual /* [helpcontext][helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_SelBold( 
            /* [retval][out] */ __RPC__out VARIANT *pvarSelBold) = 0;
        
        virtual /* [helpcontext][helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_SelBold( 
            /* [in] */ VARIANT pvarSelBold) = 0;
        
        virtual /* [helpcontext][helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_SelItalic( 
            /* [retval][out] */ __RPC__out VARIANT *pvarSelItalic) = 0;
        
        virtual /* [helpcontext][helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_SelItalic( 
            /* [in] */ VARIANT pvarSelItalic) = 0;
        
        virtual /* [helpcontext][helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_SelUnderline( 
            /* [retval][out] */ __RPC__out VARIANT *pvarSelUnderline) = 0;
        
        virtual /* [helpcontext][helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_SelUnderline( 
            /* [in] */ VARIANT pvarSelUnderline) = 0;
        
        virtual /* [helpcontext][helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_SelColor( 
            /* [retval][out] */ __RPC__out VARIANT *pvarSelColor) = 0;
        
        virtual /* [helpcontext][helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_SelColor( 
            /* [in] */ VARIANT pvarSelColor) = 0;
        
        virtual /* [helpcontext][helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_SelFontName( 
            /* [retval][out] */ __RPC__out VARIANT *pvarSelFontName) = 0;
        
        virtual /* [helpcontext][helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_SelFontName( 
            /* [in] */ VARIANT pvarSelFontName) = 0;
        
        virtual /* [helpcontext][helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_SelFontSize( 
            /* [retval][out] */ __RPC__out VARIANT *pvarSelFontSize) = 0;
        
        virtual /* [helpcontext][helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_SelFontSize( 
            /* [in] */ VARIANT pvarSelFontSize) = 0;
        
        virtual /* [helpcontext][helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_SelCharOffset( 
            /* [retval][out] */ __RPC__out VARIANT *pvarSelCharOffset) = 0;
        
        virtual /* [helpcontext][helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_SelCharOffset( 
            /* [in] */ VARIANT pvarSelCharOffset) = 0;
        
        virtual /* [helpcontext][helpstring][defaultbind][displaybind][bindable][propget][id] */ HRESULT STDMETHODCALLTYPE get_TextRTF( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrTextRTF) = 0;
        
        virtual /* [helpcontext][helpstring][displaybind][bindable][propput][id] */ HRESULT STDMETHODCALLTYPE put_TextRTF( 
            /* [in] */ __RPC__in BSTR pbstrTextRTF) = 0;
        
        virtual /* [helpcontext][helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_SelStart( 
            /* [retval][out] */ __RPC__out long *plSelStart) = 0;
        
        virtual /* [helpcontext][helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_SelStart( 
            /* [in] */ long plSelStart) = 0;
        
        virtual /* [helpcontext][helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_SelLength( 
            /* [retval][out] */ __RPC__out long *plSelLength) = 0;
        
        virtual /* [helpcontext][helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_SelLength( 
            /* [in] */ long plSelLength) = 0;
        
        virtual /* [helpcontext][helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_SelText( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrSelText) = 0;
        
        virtual /* [helpcontext][helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_SelText( 
            /* [in] */ __RPC__in BSTR pbstrSelText) = 0;
        
        virtual /* [helpcontext][helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_SelRTF( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrSelRTF) = 0;
        
        virtual /* [helpcontext][helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_SelRTF( 
            /* [in] */ __RPC__in BSTR pbstrSelRTF) = 0;
        
        virtual /* [helpcontext][helpstring][id] */ HRESULT STDMETHODCALLTYPE Refresh( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IInkEditVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IInkEdit * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IInkEdit * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IInkEdit * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IInkEdit * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IInkEdit * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IInkEdit * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IInkEdit * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(IInkEdit, get_Status)
        /* [helpcontext][helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Status )( 
            __RPC__in IInkEdit * This,
            /* [retval][out] */ __RPC__out InkEditStatus *pStatus);
        
        DECLSPEC_XFGVIRT(IInkEdit, get_UseMouseForInput)
        /* [helpcontext][helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_UseMouseForInput )( 
            __RPC__in IInkEdit * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pVal);
        
        DECLSPEC_XFGVIRT(IInkEdit, put_UseMouseForInput)
        /* [helpcontext][helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_UseMouseForInput )( 
            __RPC__in IInkEdit * This,
            /* [defaultvalue][in] */ VARIANT_BOOL newVal);
        
        DECLSPEC_XFGVIRT(IInkEdit, get_InkMode)
        /* [helpcontext][helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_InkMode )( 
            __RPC__in IInkEdit * This,
            /* [retval][out] */ __RPC__out InkMode *pVal);
        
        DECLSPEC_XFGVIRT(IInkEdit, put_InkMode)
        /* [helpcontext][helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_InkMode )( 
            __RPC__in IInkEdit * This,
            /* [defaultvalue][in] */ InkMode newVal);
        
        DECLSPEC_XFGVIRT(IInkEdit, get_InkInsertMode)
        /* [helpcontext][helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_InkInsertMode )( 
            __RPC__in IInkEdit * This,
            /* [retval][out] */ __RPC__out InkInsertMode *pVal);
        
        DECLSPEC_XFGVIRT(IInkEdit, put_InkInsertMode)
        /* [helpcontext][helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_InkInsertMode )( 
            __RPC__in IInkEdit * This,
            /* [defaultvalue][in] */ InkInsertMode newVal);
        
        DECLSPEC_XFGVIRT(IInkEdit, get_DrawingAttributes)
        /* [helpcontext][helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_DrawingAttributes )( 
            __RPC__in IInkEdit * This,
            /* [retval][out] */ __RPC__deref_out_opt IInkDrawingAttributes **pVal);
        
        DECLSPEC_XFGVIRT(IInkEdit, putref_DrawingAttributes)
        /* [helpcontext][helpstring][propputref][id] */ HRESULT ( STDMETHODCALLTYPE *putref_DrawingAttributes )( 
            __RPC__in IInkEdit * This,
            /* [in] */ __RPC__in_opt IInkDrawingAttributes *newVal);
        
        DECLSPEC_XFGVIRT(IInkEdit, get_RecognitionTimeout)
        /* [helpcontext][helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_RecognitionTimeout )( 
            __RPC__in IInkEdit * This,
            /* [retval][out] */ __RPC__out long *pVal);
        
        DECLSPEC_XFGVIRT(IInkEdit, put_RecognitionTimeout)
        /* [helpcontext][helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_RecognitionTimeout )( 
            __RPC__in IInkEdit * This,
            /* [in] */ long newVal);
        
        DECLSPEC_XFGVIRT(IInkEdit, get_Recognizer)
        /* [helpcontext][helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Recognizer )( 
            __RPC__in IInkEdit * This,
            /* [retval][out] */ __RPC__deref_out_opt IInkRecognizer **pVal);
        
        DECLSPEC_XFGVIRT(IInkEdit, putref_Recognizer)
        /* [helpcontext][helpstring][propputref][id] */ HRESULT ( STDMETHODCALLTYPE *putref_Recognizer )( 
            __RPC__in IInkEdit * This,
            /* [in] */ __RPC__in_opt IInkRecognizer *newVal);
        
        DECLSPEC_XFGVIRT(IInkEdit, get_Factoid)
        /* [helpcontext][helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Factoid )( 
            __RPC__in IInkEdit * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(IInkEdit, put_Factoid)
        /* [helpcontext][helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Factoid )( 
            __RPC__in IInkEdit * This,
            /* [in] */ __RPC__in BSTR newVal);
        
        DECLSPEC_XFGVIRT(IInkEdit, get_SelInks)
        /* [helpcontext][helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_SelInks )( 
            __RPC__in IInkEdit * This,
            /* [retval][out] */ __RPC__out VARIANT *pSelInk);
        
        DECLSPEC_XFGVIRT(IInkEdit, put_SelInks)
        /* [helpcontext][helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_SelInks )( 
            __RPC__in IInkEdit * This,
            /* [in] */ VARIANT SelInk);
        
        DECLSPEC_XFGVIRT(IInkEdit, get_SelInksDisplayMode)
        /* [helpcontext][helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_SelInksDisplayMode )( 
            __RPC__in IInkEdit * This,
            /* [retval][out] */ __RPC__out InkDisplayMode *pInkDisplayMode);
        
        DECLSPEC_XFGVIRT(IInkEdit, put_SelInksDisplayMode)
        /* [helpcontext][helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_SelInksDisplayMode )( 
            __RPC__in IInkEdit * This,
            /* [in] */ InkDisplayMode InkDisplayMode);
        
        DECLSPEC_XFGVIRT(IInkEdit, Recognize)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Recognize )( 
            __RPC__in IInkEdit * This);
        
        DECLSPEC_XFGVIRT(IInkEdit, GetGestureStatus)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetGestureStatus )( 
            __RPC__in IInkEdit * This,
            /* [in] */ InkApplicationGesture Gesture,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pListen);
        
        DECLSPEC_XFGVIRT(IInkEdit, SetGestureStatus)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *SetGestureStatus )( 
            __RPC__in IInkEdit * This,
            /* [in] */ InkApplicationGesture Gesture,
            /* [in] */ VARIANT_BOOL Listen);
        
        DECLSPEC_XFGVIRT(IInkEdit, put_BackColor)
        /* [helpcontext][helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_BackColor )( 
            __RPC__in IInkEdit * This,
            /* [in] */ OLE_COLOR clr);
        
        DECLSPEC_XFGVIRT(IInkEdit, get_BackColor)
        /* [helpcontext][helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_BackColor )( 
            __RPC__in IInkEdit * This,
            /* [retval][out] */ __RPC__out OLE_COLOR *pclr);
        
        DECLSPEC_XFGVIRT(IInkEdit, get_Appearance)
        /* [helpcontext][helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Appearance )( 
            __RPC__in IInkEdit * This,
            /* [retval][out] */ __RPC__out AppearanceConstants *pAppearance);
        
        DECLSPEC_XFGVIRT(IInkEdit, put_Appearance)
        /* [helpcontext][helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Appearance )( 
            __RPC__in IInkEdit * This,
            /* [in] */ AppearanceConstants pAppearance);
        
        DECLSPEC_XFGVIRT(IInkEdit, get_BorderStyle)
        /* [helpcontext][helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_BorderStyle )( 
            __RPC__in IInkEdit * This,
            /* [retval][out] */ __RPC__out BorderStyleConstants *pBorderStyle);
        
        DECLSPEC_XFGVIRT(IInkEdit, put_BorderStyle)
        /* [helpcontext][helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_BorderStyle )( 
            __RPC__in IInkEdit * This,
            /* [in] */ BorderStyleConstants pBorderStyle);
        
        DECLSPEC_XFGVIRT(IInkEdit, get_Hwnd)
        /* [helpcontext][helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Hwnd )( 
            __RPC__in IInkEdit * This,
            /* [retval][out] */ __RPC__out OLE_HANDLE *pohHwnd);
        
        DECLSPEC_XFGVIRT(IInkEdit, get_Font)
        /* [helpcontext][helpstring][bindable][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Font )( 
            __RPC__in IInkEdit * This,
            /* [retval][out] */ __RPC__deref_out_opt IFontDisp **ppFont);
        
        DECLSPEC_XFGVIRT(IInkEdit, putref_Font)
        /* [helpcontext][helpstring][bindable][propputref][id] */ HRESULT ( STDMETHODCALLTYPE *putref_Font )( 
            __RPC__in IInkEdit * This,
            /* [in] */ __RPC__in_opt IFontDisp *ppFont);
        
        DECLSPEC_XFGVIRT(IInkEdit, get_Text)
        /* [helpcontext][helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Text )( 
            __RPC__in IInkEdit * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrText);
        
        DECLSPEC_XFGVIRT(IInkEdit, put_Text)
        /* [helpcontext][helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Text )( 
            __RPC__in IInkEdit * This,
            /* [in] */ __RPC__in BSTR pbstrText);
        
        DECLSPEC_XFGVIRT(IInkEdit, get_MouseIcon)
        /* [helpcontext][helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_MouseIcon )( 
            __RPC__in IInkEdit * This,
            /* [retval][out] */ __RPC__deref_out_opt IPictureDisp **MouseIcon);
        
        DECLSPEC_XFGVIRT(IInkEdit, put_MouseIcon)
        /* [helpcontext][helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_MouseIcon )( 
            __RPC__in IInkEdit * This,
            /* [in] */ __RPC__in_opt IPictureDisp *MouseIcon);
        
        DECLSPEC_XFGVIRT(IInkEdit, putref_MouseIcon)
        /* [helpcontext][helpstring][propputref][id] */ HRESULT ( STDMETHODCALLTYPE *putref_MouseIcon )( 
            __RPC__in IInkEdit * This,
            /* [in] */ __RPC__in_opt IPictureDisp *MouseIcon);
        
        DECLSPEC_XFGVIRT(IInkEdit, get_MousePointer)
        /* [helpcontext][helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_MousePointer )( 
            __RPC__in IInkEdit * This,
            /* [retval][out] */ __RPC__out InkMousePointer *MousePointer);
        
        DECLSPEC_XFGVIRT(IInkEdit, put_MousePointer)
        /* [helpcontext][helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_MousePointer )( 
            __RPC__in IInkEdit * This,
            /* [in] */ InkMousePointer MousePointer);
        
        DECLSPEC_XFGVIRT(IInkEdit, get_Locked)
        /* [helpcontext][helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Locked )( 
            __RPC__in IInkEdit * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pVal);
        
        DECLSPEC_XFGVIRT(IInkEdit, put_Locked)
        /* [helpcontext][helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Locked )( 
            __RPC__in IInkEdit * This,
            /* [in] */ VARIANT_BOOL newVal);
        
        DECLSPEC_XFGVIRT(IInkEdit, get_Enabled)
        /* [helpcontext][helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Enabled )( 
            __RPC__in IInkEdit * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pVal);
        
        DECLSPEC_XFGVIRT(IInkEdit, put_Enabled)
        /* [helpcontext][helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Enabled )( 
            __RPC__in IInkEdit * This,
            /* [in] */ VARIANT_BOOL newVal);
        
        DECLSPEC_XFGVIRT(IInkEdit, get_MaxLength)
        /* [helpcontext][helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_MaxLength )( 
            __RPC__in IInkEdit * This,
            /* [retval][out] */ __RPC__out long *plMaxLength);
        
        DECLSPEC_XFGVIRT(IInkEdit, put_MaxLength)
        /* [helpcontext][helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_MaxLength )( 
            __RPC__in IInkEdit * This,
            /* [in] */ long lMaxLength);
        
        DECLSPEC_XFGVIRT(IInkEdit, get_MultiLine)
        /* [helpcontext][helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_MultiLine )( 
            __RPC__in IInkEdit * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pVal);
        
        DECLSPEC_XFGVIRT(IInkEdit, put_MultiLine)
        /* [helpcontext][helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_MultiLine )( 
            __RPC__in IInkEdit * This,
            /* [in] */ VARIANT_BOOL newVal);
        
        DECLSPEC_XFGVIRT(IInkEdit, get_ScrollBars)
        /* [helpcontext][helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_ScrollBars )( 
            __RPC__in IInkEdit * This,
            /* [retval][out] */ __RPC__out ScrollBarsConstants *pVal);
        
        DECLSPEC_XFGVIRT(IInkEdit, put_ScrollBars)
        /* [helpcontext][helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_ScrollBars )( 
            __RPC__in IInkEdit * This,
            /* [in] */ ScrollBarsConstants newVal);
        
        DECLSPEC_XFGVIRT(IInkEdit, get_DisableNoScroll)
        /* [helpcontext][helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_DisableNoScroll )( 
            __RPC__in IInkEdit * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pVal);
        
        DECLSPEC_XFGVIRT(IInkEdit, put_DisableNoScroll)
        /* [helpcontext][helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_DisableNoScroll )( 
            __RPC__in IInkEdit * This,
            /* [in] */ VARIANT_BOOL newVal);
        
        DECLSPEC_XFGVIRT(IInkEdit, get_SelAlignment)
        /* [helpcontext][helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_SelAlignment )( 
            __RPC__in IInkEdit * This,
            /* [retval][out] */ __RPC__out VARIANT *pvarSelAlignment);
        
        DECLSPEC_XFGVIRT(IInkEdit, put_SelAlignment)
        /* [helpcontext][helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_SelAlignment )( 
            __RPC__in IInkEdit * This,
            /* [in] */ VARIANT pvarSelAlignment);
        
        DECLSPEC_XFGVIRT(IInkEdit, get_SelBold)
        /* [helpcontext][helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_SelBold )( 
            __RPC__in IInkEdit * This,
            /* [retval][out] */ __RPC__out VARIANT *pvarSelBold);
        
        DECLSPEC_XFGVIRT(IInkEdit, put_SelBold)
        /* [helpcontext][helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_SelBold )( 
            __RPC__in IInkEdit * This,
            /* [in] */ VARIANT pvarSelBold);
        
        DECLSPEC_XFGVIRT(IInkEdit, get_SelItalic)
        /* [helpcontext][helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_SelItalic )( 
            __RPC__in IInkEdit * This,
            /* [retval][out] */ __RPC__out VARIANT *pvarSelItalic);
        
        DECLSPEC_XFGVIRT(IInkEdit, put_SelItalic)
        /* [helpcontext][helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_SelItalic )( 
            __RPC__in IInkEdit * This,
            /* [in] */ VARIANT pvarSelItalic);
        
        DECLSPEC_XFGVIRT(IInkEdit, get_SelUnderline)
        /* [helpcontext][helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_SelUnderline )( 
            __RPC__in IInkEdit * This,
            /* [retval][out] */ __RPC__out VARIANT *pvarSelUnderline);
        
        DECLSPEC_XFGVIRT(IInkEdit, put_SelUnderline)
        /* [helpcontext][helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_SelUnderline )( 
            __RPC__in IInkEdit * This,
            /* [in] */ VARIANT pvarSelUnderline);
        
        DECLSPEC_XFGVIRT(IInkEdit, get_SelColor)
        /* [helpcontext][helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_SelColor )( 
            __RPC__in IInkEdit * This,
            /* [retval][out] */ __RPC__out VARIANT *pvarSelColor);
        
        DECLSPEC_XFGVIRT(IInkEdit, put_SelColor)
        /* [helpcontext][helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_SelColor )( 
            __RPC__in IInkEdit * This,
            /* [in] */ VARIANT pvarSelColor);
        
        DECLSPEC_XFGVIRT(IInkEdit, get_SelFontName)
        /* [helpcontext][helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_SelFontName )( 
            __RPC__in IInkEdit * This,
            /* [retval][out] */ __RPC__out VARIANT *pvarSelFontName);
        
        DECLSPEC_XFGVIRT(IInkEdit, put_SelFontName)
        /* [helpcontext][helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_SelFontName )( 
            __RPC__in IInkEdit * This,
            /* [in] */ VARIANT pvarSelFontName);
        
        DECLSPEC_XFGVIRT(IInkEdit, get_SelFontSize)
        /* [helpcontext][helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_SelFontSize )( 
            __RPC__in IInkEdit * This,
            /* [retval][out] */ __RPC__out VARIANT *pvarSelFontSize);
        
        DECLSPEC_XFGVIRT(IInkEdit, put_SelFontSize)
        /* [helpcontext][helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_SelFontSize )( 
            __RPC__in IInkEdit * This,
            /* [in] */ VARIANT pvarSelFontSize);
        
        DECLSPEC_XFGVIRT(IInkEdit, get_SelCharOffset)
        /* [helpcontext][helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_SelCharOffset )( 
            __RPC__in IInkEdit * This,
            /* [retval][out] */ __RPC__out VARIANT *pvarSelCharOffset);
        
        DECLSPEC_XFGVIRT(IInkEdit, put_SelCharOffset)
        /* [helpcontext][helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_SelCharOffset )( 
            __RPC__in IInkEdit * This,
            /* [in] */ VARIANT pvarSelCharOffset);
        
        DECLSPEC_XFGVIRT(IInkEdit, get_TextRTF)
        /* [helpcontext][helpstring][defaultbind][displaybind][bindable][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_TextRTF )( 
            __RPC__in IInkEdit * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrTextRTF);
        
        DECLSPEC_XFGVIRT(IInkEdit, put_TextRTF)
        /* [helpcontext][helpstring][displaybind][bindable][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_TextRTF )( 
            __RPC__in IInkEdit * This,
            /* [in] */ __RPC__in BSTR pbstrTextRTF);
        
        DECLSPEC_XFGVIRT(IInkEdit, get_SelStart)
        /* [helpcontext][helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_SelStart )( 
            __RPC__in IInkEdit * This,
            /* [retval][out] */ __RPC__out long *plSelStart);
        
        DECLSPEC_XFGVIRT(IInkEdit, put_SelStart)
        /* [helpcontext][helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_SelStart )( 
            __RPC__in IInkEdit * This,
            /* [in] */ long plSelStart);
        
        DECLSPEC_XFGVIRT(IInkEdit, get_SelLength)
        /* [helpcontext][helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_SelLength )( 
            __RPC__in IInkEdit * This,
            /* [retval][out] */ __RPC__out long *plSelLength);
        
        DECLSPEC_XFGVIRT(IInkEdit, put_SelLength)
        /* [helpcontext][helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_SelLength )( 
            __RPC__in IInkEdit * This,
            /* [in] */ long plSelLength);
        
        DECLSPEC_XFGVIRT(IInkEdit, get_SelText)
        /* [helpcontext][helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_SelText )( 
            __RPC__in IInkEdit * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrSelText);
        
        DECLSPEC_XFGVIRT(IInkEdit, put_SelText)
        /* [helpcontext][helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_SelText )( 
            __RPC__in IInkEdit * This,
            /* [in] */ __RPC__in BSTR pbstrSelText);
        
        DECLSPEC_XFGVIRT(IInkEdit, get_SelRTF)
        /* [helpcontext][helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_SelRTF )( 
            __RPC__in IInkEdit * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrSelRTF);
        
        DECLSPEC_XFGVIRT(IInkEdit, put_SelRTF)
        /* [helpcontext][helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_SelRTF )( 
            __RPC__in IInkEdit * This,
            /* [in] */ __RPC__in BSTR pbstrSelRTF);
        
        DECLSPEC_XFGVIRT(IInkEdit, Refresh)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Refresh )( 
            __RPC__in IInkEdit * This);
        
        END_INTERFACE
    } IInkEditVtbl;

    interface IInkEdit
    {
        CONST_VTBL struct IInkEditVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IInkEdit_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IInkEdit_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IInkEdit_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IInkEdit_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IInkEdit_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IInkEdit_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IInkEdit_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IInkEdit_get_Status(This,pStatus)	\
    ( (This)->lpVtbl -> get_Status(This,pStatus) ) 

#define IInkEdit_get_UseMouseForInput(This,pVal)	\
    ( (This)->lpVtbl -> get_UseMouseForInput(This,pVal) ) 

#define IInkEdit_put_UseMouseForInput(This,newVal)	\
    ( (This)->lpVtbl -> put_UseMouseForInput(This,newVal) ) 

#define IInkEdit_get_InkMode(This,pVal)	\
    ( (This)->lpVtbl -> get_InkMode(This,pVal) ) 

#define IInkEdit_put_InkMode(This,newVal)	\
    ( (This)->lpVtbl -> put_InkMode(This,newVal) ) 

#define IInkEdit_get_InkInsertMode(This,pVal)	\
    ( (This)->lpVtbl -> get_InkInsertMode(This,pVal) ) 

#define IInkEdit_put_InkInsertMode(This,newVal)	\
    ( (This)->lpVtbl -> put_InkInsertMode(This,newVal) ) 

#define IInkEdit_get_DrawingAttributes(This,pVal)	\
    ( (This)->lpVtbl -> get_DrawingAttributes(This,pVal) ) 

#define IInkEdit_putref_DrawingAttributes(This,newVal)	\
    ( (This)->lpVtbl -> putref_DrawingAttributes(This,newVal) ) 

#define IInkEdit_get_RecognitionTimeout(This,pVal)	\
    ( (This)->lpVtbl -> get_RecognitionTimeout(This,pVal) ) 

#define IInkEdit_put_RecognitionTimeout(This,newVal)	\
    ( (This)->lpVtbl -> put_RecognitionTimeout(This,newVal) ) 

#define IInkEdit_get_Recognizer(This,pVal)	\
    ( (This)->lpVtbl -> get_Recognizer(This,pVal) ) 

#define IInkEdit_putref_Recognizer(This,newVal)	\
    ( (This)->lpVtbl -> putref_Recognizer(This,newVal) ) 

#define IInkEdit_get_Factoid(This,pVal)	\
    ( (This)->lpVtbl -> get_Factoid(This,pVal) ) 

#define IInkEdit_put_Factoid(This,newVal)	\
    ( (This)->lpVtbl -> put_Factoid(This,newVal) ) 

#define IInkEdit_get_SelInks(This,pSelInk)	\
    ( (This)->lpVtbl -> get_SelInks(This,pSelInk) ) 

#define IInkEdit_put_SelInks(This,SelInk)	\
    ( (This)->lpVtbl -> put_SelInks(This,SelInk) ) 

#define IInkEdit_get_SelInksDisplayMode(This,pInkDisplayMode)	\
    ( (This)->lpVtbl -> get_SelInksDisplayMode(This,pInkDisplayMode) ) 

#define IInkEdit_put_SelInksDisplayMode(This,InkDisplayMode)	\
    ( (This)->lpVtbl -> put_SelInksDisplayMode(This,InkDisplayMode) ) 

#define IInkEdit_Recognize(This)	\
    ( (This)->lpVtbl -> Recognize(This) ) 

#define IInkEdit_GetGestureStatus(This,Gesture,pListen)	\
    ( (This)->lpVtbl -> GetGestureStatus(This,Gesture,pListen) ) 

#define IInkEdit_SetGestureStatus(This,Gesture,Listen)	\
    ( (This)->lpVtbl -> SetGestureStatus(This,Gesture,Listen) ) 

#define IInkEdit_put_BackColor(This,clr)	\
    ( (This)->lpVtbl -> put_BackColor(This,clr) ) 

#define IInkEdit_get_BackColor(This,pclr)	\
    ( (This)->lpVtbl -> get_BackColor(This,pclr) ) 

#define IInkEdit_get_Appearance(This,pAppearance)	\
    ( (This)->lpVtbl -> get_Appearance(This,pAppearance) ) 

#define IInkEdit_put_Appearance(This,pAppearance)	\
    ( (This)->lpVtbl -> put_Appearance(This,pAppearance) ) 

#define IInkEdit_get_BorderStyle(This,pBorderStyle)	\
    ( (This)->lpVtbl -> get_BorderStyle(This,pBorderStyle) ) 

#define IInkEdit_put_BorderStyle(This,pBorderStyle)	\
    ( (This)->lpVtbl -> put_BorderStyle(This,pBorderStyle) ) 

#define IInkEdit_get_Hwnd(This,pohHwnd)	\
    ( (This)->lpVtbl -> get_Hwnd(This,pohHwnd) ) 

#define IInkEdit_get_Font(This,ppFont)	\
    ( (This)->lpVtbl -> get_Font(This,ppFont) ) 

#define IInkEdit_putref_Font(This,ppFont)	\
    ( (This)->lpVtbl -> putref_Font(This,ppFont) ) 

#define IInkEdit_get_Text(This,pbstrText)	\
    ( (This)->lpVtbl -> get_Text(This,pbstrText) ) 

#define IInkEdit_put_Text(This,pbstrText)	\
    ( (This)->lpVtbl -> put_Text(This,pbstrText) ) 

#define IInkEdit_get_MouseIcon(This,MouseIcon)	\
    ( (This)->lpVtbl -> get_MouseIcon(This,MouseIcon) ) 

#define IInkEdit_put_MouseIcon(This,MouseIcon)	\
    ( (This)->lpVtbl -> put_MouseIcon(This,MouseIcon) ) 

#define IInkEdit_putref_MouseIcon(This,MouseIcon)	\
    ( (This)->lpVtbl -> putref_MouseIcon(This,MouseIcon) ) 

#define IInkEdit_get_MousePointer(This,MousePointer)	\
    ( (This)->lpVtbl -> get_MousePointer(This,MousePointer) ) 

#define IInkEdit_put_MousePointer(This,MousePointer)	\
    ( (This)->lpVtbl -> put_MousePointer(This,MousePointer) ) 

#define IInkEdit_get_Locked(This,pVal)	\
    ( (This)->lpVtbl -> get_Locked(This,pVal) ) 

#define IInkEdit_put_Locked(This,newVal)	\
    ( (This)->lpVtbl -> put_Locked(This,newVal) ) 

#define IInkEdit_get_Enabled(This,pVal)	\
    ( (This)->lpVtbl -> get_Enabled(This,pVal) ) 

#define IInkEdit_put_Enabled(This,newVal)	\
    ( (This)->lpVtbl -> put_Enabled(This,newVal) ) 

#define IInkEdit_get_MaxLength(This,plMaxLength)	\
    ( (This)->lpVtbl -> get_MaxLength(This,plMaxLength) ) 

#define IInkEdit_put_MaxLength(This,lMaxLength)	\
    ( (This)->lpVtbl -> put_MaxLength(This,lMaxLength) ) 

#define IInkEdit_get_MultiLine(This,pVal)	\
    ( (This)->lpVtbl -> get_MultiLine(This,pVal) ) 

#define IInkEdit_put_MultiLine(This,newVal)	\
    ( (This)->lpVtbl -> put_MultiLine(This,newVal) ) 

#define IInkEdit_get_ScrollBars(This,pVal)	\
    ( (This)->lpVtbl -> get_ScrollBars(This,pVal) ) 

#define IInkEdit_put_ScrollBars(This,newVal)	\
    ( (This)->lpVtbl -> put_ScrollBars(This,newVal) ) 

#define IInkEdit_get_DisableNoScroll(This,pVal)	\
    ( (This)->lpVtbl -> get_DisableNoScroll(This,pVal) ) 

#define IInkEdit_put_DisableNoScroll(This,newVal)	\
    ( (This)->lpVtbl -> put_DisableNoScroll(This,newVal) ) 

#define IInkEdit_get_SelAlignment(This,pvarSelAlignment)	\
    ( (This)->lpVtbl -> get_SelAlignment(This,pvarSelAlignment) ) 

#define IInkEdit_put_SelAlignment(This,pvarSelAlignment)	\
    ( (This)->lpVtbl -> put_SelAlignment(This,pvarSelAlignment) ) 

#define IInkEdit_get_SelBold(This,pvarSelBold)	\
    ( (This)->lpVtbl -> get_SelBold(This,pvarSelBold) ) 

#define IInkEdit_put_SelBold(This,pvarSelBold)	\
    ( (This)->lpVtbl -> put_SelBold(This,pvarSelBold) ) 

#define IInkEdit_get_SelItalic(This,pvarSelItalic)	\
    ( (This)->lpVtbl -> get_SelItalic(This,pvarSelItalic) ) 

#define IInkEdit_put_SelItalic(This,pvarSelItalic)	\
    ( (This)->lpVtbl -> put_SelItalic(This,pvarSelItalic) ) 

#define IInkEdit_get_SelUnderline(This,pvarSelUnderline)	\
    ( (This)->lpVtbl -> get_SelUnderline(This,pvarSelUnderline) ) 

#define IInkEdit_put_SelUnderline(This,pvarSelUnderline)	\
    ( (This)->lpVtbl -> put_SelUnderline(This,pvarSelUnderline) ) 

#define IInkEdit_get_SelColor(This,pvarSelColor)	\
    ( (This)->lpVtbl -> get_SelColor(This,pvarSelColor) ) 

#define IInkEdit_put_SelColor(This,pvarSelColor)	\
    ( (This)->lpVtbl -> put_SelColor(This,pvarSelColor) ) 

#define IInkEdit_get_SelFontName(This,pvarSelFontName)	\
    ( (This)->lpVtbl -> get_SelFontName(This,pvarSelFontName) ) 

#define IInkEdit_put_SelFontName(This,pvarSelFontName)	\
    ( (This)->lpVtbl -> put_SelFontName(This,pvarSelFontName) ) 

#define IInkEdit_get_SelFontSize(This,pvarSelFontSize)	\
    ( (This)->lpVtbl -> get_SelFontSize(This,pvarSelFontSize) ) 

#define IInkEdit_put_SelFontSize(This,pvarSelFontSize)	\
    ( (This)->lpVtbl -> put_SelFontSize(This,pvarSelFontSize) ) 

#define IInkEdit_get_SelCharOffset(This,pvarSelCharOffset)	\
    ( (This)->lpVtbl -> get_SelCharOffset(This,pvarSelCharOffset) ) 

#define IInkEdit_put_SelCharOffset(This,pvarSelCharOffset)	\
    ( (This)->lpVtbl -> put_SelCharOffset(This,pvarSelCharOffset) ) 

#define IInkEdit_get_TextRTF(This,pbstrTextRTF)	\
    ( (This)->lpVtbl -> get_TextRTF(This,pbstrTextRTF) ) 

#define IInkEdit_put_TextRTF(This,pbstrTextRTF)	\
    ( (This)->lpVtbl -> put_TextRTF(This,pbstrTextRTF) ) 

#define IInkEdit_get_SelStart(This,plSelStart)	\
    ( (This)->lpVtbl -> get_SelStart(This,plSelStart) ) 

#define IInkEdit_put_SelStart(This,plSelStart)	\
    ( (This)->lpVtbl -> put_SelStart(This,plSelStart) ) 

#define IInkEdit_get_SelLength(This,plSelLength)	\
    ( (This)->lpVtbl -> get_SelLength(This,plSelLength) ) 

#define IInkEdit_put_SelLength(This,plSelLength)	\
    ( (This)->lpVtbl -> put_SelLength(This,plSelLength) ) 

#define IInkEdit_get_SelText(This,pbstrSelText)	\
    ( (This)->lpVtbl -> get_SelText(This,pbstrSelText) ) 

#define IInkEdit_put_SelText(This,pbstrSelText)	\
    ( (This)->lpVtbl -> put_SelText(This,pbstrSelText) ) 

#define IInkEdit_get_SelRTF(This,pbstrSelRTF)	\
    ( (This)->lpVtbl -> get_SelRTF(This,pbstrSelRTF) ) 

#define IInkEdit_put_SelRTF(This,pbstrSelRTF)	\
    ( (This)->lpVtbl -> put_SelRTF(This,pbstrSelRTF) ) 

#define IInkEdit_Refresh(This)	\
    ( (This)->lpVtbl -> Refresh(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IInkEdit_INTERFACE_DEFINED__ */


#ifndef ___IInkEditEvents_DISPINTERFACE_DEFINED__
#define ___IInkEditEvents_DISPINTERFACE_DEFINED__

/* dispinterface _IInkEditEvents */
/* [helpcontext][helpstring][uuid] */ 


EXTERN_C const IID DIID__IInkEditEvents;

#if defined(__cplusplus) && !defined(CINTERFACE)

    MIDL_INTERFACE("E3B0B797-A72E-46DB-A0D7-6C9EBA8E9BBC")
    _IInkEditEvents : public IDispatch
    {
    };
    
#else 	/* C style interface */

    typedef struct _IInkEditEventsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in _IInkEditEvents * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in _IInkEditEvents * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in _IInkEditEvents * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in _IInkEditEvents * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in _IInkEditEvents * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in _IInkEditEvents * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            _IInkEditEvents * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        END_INTERFACE
    } _IInkEditEventsVtbl;

    interface _IInkEditEvents
    {
        CONST_VTBL struct _IInkEditEventsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define _IInkEditEvents_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define _IInkEditEvents_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define _IInkEditEvents_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define _IInkEditEvents_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define _IInkEditEvents_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define _IInkEditEvents_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define _IInkEditEvents_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */


#endif 	/* ___IInkEditEvents_DISPINTERFACE_DEFINED__ */


EXTERN_C const CLSID CLSID_InkEdit;

#ifdef __cplusplus

class DECLSPEC_UUID("E5CA59F5-57C4-4DD8-9BD6-1DEEEDD27AF4")
InkEdit;
#endif
#endif /* __INKEDLib_LIBRARY_DEFINED__ */

/* interface __MIDL_itf_inked_0000_0001 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_inked_0000_0001_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_inked_0000_0001_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


