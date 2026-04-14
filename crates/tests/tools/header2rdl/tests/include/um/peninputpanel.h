

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


#ifndef __peninputpanel_h__
#define __peninputpanel_h__

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

#ifndef __IPenInputPanel_FWD_DEFINED__
#define __IPenInputPanel_FWD_DEFINED__
typedef interface IPenInputPanel IPenInputPanel;

#endif 	/* __IPenInputPanel_FWD_DEFINED__ */


#ifndef ___IPenInputPanelEvents_FWD_DEFINED__
#define ___IPenInputPanelEvents_FWD_DEFINED__
typedef interface _IPenInputPanelEvents _IPenInputPanelEvents;

#endif 	/* ___IPenInputPanelEvents_FWD_DEFINED__ */


#ifndef __IHandwrittenTextInsertion_FWD_DEFINED__
#define __IHandwrittenTextInsertion_FWD_DEFINED__
typedef interface IHandwrittenTextInsertion IHandwrittenTextInsertion;

#endif 	/* __IHandwrittenTextInsertion_FWD_DEFINED__ */


#ifndef __HandwrittenTextInsertion_FWD_DEFINED__
#define __HandwrittenTextInsertion_FWD_DEFINED__

#ifdef __cplusplus
typedef class HandwrittenTextInsertion HandwrittenTextInsertion;
#else
typedef struct HandwrittenTextInsertion HandwrittenTextInsertion;
#endif /* __cplusplus */

#endif 	/* __HandwrittenTextInsertion_FWD_DEFINED__ */


#ifndef __PenInputPanel_FWD_DEFINED__
#define __PenInputPanel_FWD_DEFINED__

#ifdef __cplusplus
typedef class PenInputPanel PenInputPanel;
#else
typedef struct PenInputPanel PenInputPanel;
#endif /* __cplusplus */

#endif 	/* __PenInputPanel_FWD_DEFINED__ */


#ifndef __ITextInputPanelEventSink_FWD_DEFINED__
#define __ITextInputPanelEventSink_FWD_DEFINED__
typedef interface ITextInputPanelEventSink ITextInputPanelEventSink;

#endif 	/* __ITextInputPanelEventSink_FWD_DEFINED__ */


#ifndef __ITextInputPanel_FWD_DEFINED__
#define __ITextInputPanel_FWD_DEFINED__
typedef interface ITextInputPanel ITextInputPanel;

#endif 	/* __ITextInputPanel_FWD_DEFINED__ */


#ifndef __IInputPanelWindowHandle_FWD_DEFINED__
#define __IInputPanelWindowHandle_FWD_DEFINED__
typedef interface IInputPanelWindowHandle IInputPanelWindowHandle;

#endif 	/* __IInputPanelWindowHandle_FWD_DEFINED__ */


#ifndef __ITextInputPanelRunInfo_FWD_DEFINED__
#define __ITextInputPanelRunInfo_FWD_DEFINED__
typedef interface ITextInputPanelRunInfo ITextInputPanelRunInfo;

#endif 	/* __ITextInputPanelRunInfo_FWD_DEFINED__ */


#ifndef __TextInputPanel_FWD_DEFINED__
#define __TextInputPanel_FWD_DEFINED__

#ifdef __cplusplus
typedef class TextInputPanel TextInputPanel;
#else
typedef struct TextInputPanel TextInputPanel;
#endif /* __cplusplus */

#endif 	/* __TextInputPanel_FWD_DEFINED__ */


#ifndef __PenInputPanel_Internal_FWD_DEFINED__
#define __PenInputPanel_Internal_FWD_DEFINED__

#ifdef __cplusplus
typedef class PenInputPanel_Internal PenInputPanel_Internal;
#else
typedef struct PenInputPanel_Internal PenInputPanel_Internal;
#endif /* __cplusplus */

#endif 	/* __PenInputPanel_Internal_FWD_DEFINED__ */


/* header files for imported files */
#include "oaidl.h"
#include "ocidl.h"
#include "msinkaut.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_peninputpanel_0000_0000 */
/* [local] */ 

#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


extern RPC_IF_HANDLE __MIDL_itf_peninputpanel_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_peninputpanel_0000_0000_v0_0_s_ifspec;


#ifndef __PenInputPanelLib_LIBRARY_DEFINED__
#define __PenInputPanelLib_LIBRARY_DEFINED__

/* library PenInputPanelLib */
/* [helpcontext][helpstring][version][uuid] */ 

#define MICROSOFT_PENINPUT_PANEL_PROPERTY_T	L"Microsoft PenInputPanel 1.5"
typedef /* [hidden] */ 
enum DISPID_PenInputPanel
    {
        DISPID_PIPAttachedEditWindow	= DISPID_VALUE,
        DISPID_PIPFactoid	= ( DISPID_PIPAttachedEditWindow + 1 ) ,
        DISPID_PIPCurrentPanel	= ( DISPID_PIPFactoid + 1 ) ,
        DISPID_PIPDefaultPanel	= ( DISPID_PIPCurrentPanel + 1 ) ,
        DISPID_PIPVisible	= ( DISPID_PIPDefaultPanel + 1 ) ,
        DISPID_PIPTop	= ( DISPID_PIPVisible + 1 ) ,
        DISPID_PIPLeft	= ( DISPID_PIPTop + 1 ) ,
        DISPID_PIPWidth	= ( DISPID_PIPLeft + 1 ) ,
        DISPID_PIPHeight	= ( DISPID_PIPWidth + 1 ) ,
        DISPID_PIPMoveTo	= ( DISPID_PIPHeight + 1 ) ,
        DISPID_PIPCommitPendingInput	= ( DISPID_PIPMoveTo + 1 ) ,
        DISPID_PIPRefresh	= ( DISPID_PIPCommitPendingInput + 1 ) ,
        DISPID_PIPBusy	= ( DISPID_PIPRefresh + 1 ) ,
        DISPID_PIPVerticalOffset	= ( DISPID_PIPBusy + 1 ) ,
        DISPID_PIPHorizontalOffset	= ( DISPID_PIPVerticalOffset + 1 ) ,
        DISPID_PIPEnableTsf	= ( DISPID_PIPHorizontalOffset + 1 ) ,
        DISPID_PIPAutoShow	= ( DISPID_PIPEnableTsf + 1 ) 
    } 	DISPID_PenInputPanel;

typedef /* [hidden] */ 
enum DISPID_PenInputPanelEvents
    {
        DISPID_PIPEVisibleChanged	= 0,
        DISPID_PIPEPanelChanged	= ( DISPID_PIPEVisibleChanged + 1 ) ,
        DISPID_PIPEInputFailed	= ( DISPID_PIPEPanelChanged + 1 ) ,
        DISPID_PIPEPanelMoving	= ( DISPID_PIPEInputFailed + 1 ) 
    } 	DISPID_PenInputPanelEvents;

typedef /* [uuid] */  DECLSPEC_UUID("7fd1134a-b2ba-4673-8d64-e28be4168e5d") 
enum VisualState
    {
        InPlace	= 0,
        Floating	= 1,
        DockedTop	= 2,
        DockedBottom	= 3,
        Closed	= 4
    } 	VisualState;

typedef /* [public][public][uuid] */  DECLSPEC_UUID("500f9c5a-6739-449b-9cfa-5fc2f2e9ddce") 
enum __MIDL___MIDL_itf_peninputpanel_0000_0000_0001
    {
        InteractionMode_InPlace	= 0,
        InteractionMode_Floating	= 1,
        InteractionMode_DockedTop	= 2,
        InteractionMode_DockedBottom	= 3
    } 	InteractionMode;

typedef /* [public][public][public][public][public][public][public][public][uuid] */  DECLSPEC_UUID("aa9bda6b-fc6a-49a3-9d7c-26b233690583") 
enum __MIDL___MIDL_itf_peninputpanel_0000_0000_0002
    {
        InPlaceState_Auto	= 0,
        InPlaceState_HoverTarget	= 1,
        InPlaceState_Expanded	= 2
    } 	InPlaceState;

typedef /* [public][public][public][public][public][public][public][public][uuid] */  DECLSPEC_UUID("8b4f78bf-4253-4467-a006-670419caa993") 
enum __MIDL___MIDL_itf_peninputpanel_0000_0000_0003
    {
        PanelInputArea_Auto	= 0,
        PanelInputArea_Keyboard	= 1,
        PanelInputArea_WritingPad	= 2,
        PanelInputArea_CharacterPad	= 3
    } 	PanelInputArea;

typedef /* [public][public][public][public][public][public][uuid] */  DECLSPEC_UUID("d708f745-981e-4e9b-afa0-98082aadb421") 
enum __MIDL___MIDL_itf_peninputpanel_0000_0000_0004
    {
        CorrectionMode_NotVisible	= 0,
        CorrectionMode_PreInsertion	= 1,
        CorrectionMode_PostInsertionCollapsed	= 2,
        CorrectionMode_PostInsertionExpanded	= 3
    } 	CorrectionMode;

typedef /* [public][public][uuid] */  DECLSPEC_UUID("84ccefd0-9212-44e4-94e6-91562a94016e") 
enum __MIDL___MIDL_itf_peninputpanel_0000_0000_0005
    {
        CorrectionPosition_Auto	= 0,
        CorrectionPosition_Bottom	= 1,
        CorrectionPosition_Top	= 2
    } 	CorrectionPosition;

typedef /* [public][public][public][uuid] */  DECLSPEC_UUID("619eab37-412f-44ca-996f-0f415fb8bc12") 
enum __MIDL___MIDL_itf_peninputpanel_0000_0000_0006
    {
        InPlaceDirection_Auto	= 0,
        InPlaceDirection_Bottom	= 1,
        InPlaceDirection_Top	= 2
    } 	InPlaceDirection;

typedef /* [public][uuid] */  DECLSPEC_UUID("1ad3e1f7-4dd0-48c3-a89b-dfccba13d6f7") 
enum __MIDL___MIDL_itf_peninputpanel_0000_0000_0007
    {
        EventMask_InPlaceStateChanging	= 1,
        EventMask_InPlaceStateChanged	= 2,
        EventMask_InPlaceSizeChanging	= 4,
        EventMask_InPlaceSizeChanged	= 8,
        EventMask_InputAreaChanging	= 16,
        EventMask_InputAreaChanged	= 32,
        EventMask_CorrectionModeChanging	= 64,
        EventMask_CorrectionModeChanged	= 128,
        EventMask_InPlaceVisibilityChanging	= 256,
        EventMask_InPlaceVisibilityChanged	= 512,
        EventMask_TextInserting	= 1024,
        EventMask_TextInserted	= 2048,
        EventMask_All	= ( ( ( ( ( ( ( ( ( ( ( EventMask_InPlaceStateChanging + EventMask_InPlaceStateChanged )  + EventMask_InPlaceSizeChanging )  + EventMask_InPlaceSizeChanged )  + EventMask_InputAreaChanging )  + EventMask_InputAreaChanged )  + EventMask_CorrectionModeChanging )  + EventMask_CorrectionModeChanged )  + EventMask_InPlaceVisibilityChanging )  + EventMask_InPlaceVisibilityChanged )  + EventMask_TextInserting )  + EventMask_TextInserted ) 
    } 	EventMask;

const WCHAR MICROSOFT_URL_EXPERIENCE_PROPERTY[]       = L"Microsoft TIP URL Experience";
const WCHAR MICROSOFT_TIP_NO_INSERT_BUTTON_PROPERTY[] = L"Microsoft TIP No Insert Option";
const WCHAR MICROSOFT_TIP_COMBOBOXLIST_PROPERTY[]     = L"Microsoft TIP ComboBox List Window Identifier";
const WCHAR MICROSOFT_TIP_OPENING_MSG[]               = L"TabletInputPanelOpening";

EXTERN_C const IID LIBID_PenInputPanelLib;

#ifndef __IPenInputPanel_INTERFACE_DEFINED__
#define __IPenInputPanel_INTERFACE_DEFINED__

/* interface IPenInputPanel */
/* [helpcontext][helpstring][unique][dual][uuid][object] */ 

typedef /* [helpcontext][helpstring] */ 
enum PanelType
    {
        PT_Default	= 0,
        PT_Inactive	= ( PT_Default + 1 ) ,
        PT_Handwriting	= ( PT_Inactive + 1 ) ,
        PT_Keyboard	= ( PT_Handwriting + 1 ) 
    } 	PanelType;


EXTERN_C const IID IID_IPenInputPanel;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("fa7a4083-5747-4040-a182-0b0e9fd4fac7")
    IPenInputPanel : public IDispatch
    {
    public:
        virtual /* [helpcontext][helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Busy( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *Busy) = 0;
        
        virtual /* [helpcontext][helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Factoid( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *Factoid) = 0;
        
        virtual /* [helpcontext][helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_Factoid( 
            /* [in] */ __RPC__in BSTR Factoid) = 0;
        
        virtual /* [helpcontext][helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_AttachedEditWindow( 
            /* [retval][out] */ __RPC__out LONG32 *AttachedEditWindow) = 0;
        
        virtual /* [helpcontext][helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_AttachedEditWindow( 
            /* [in] */ LONG32 AttachedEditWindow) = 0;
        
        virtual /* [helpcontext][helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_CurrentPanel( 
            /* [retval][out] */ __RPC__out PanelType *CurrentPanel) = 0;
        
        virtual /* [helpcontext][helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_CurrentPanel( 
            /* [in] */ PanelType CurrentPanel) = 0;
        
        virtual /* [helpcontext][helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_DefaultPanel( 
            /* [retval][out] */ __RPC__out PanelType *pDefaultPanel) = 0;
        
        virtual /* [helpcontext][helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_DefaultPanel( 
            /* [in] */ PanelType DefaultPanel) = 0;
        
        virtual /* [helpcontext][helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Visible( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *Visible) = 0;
        
        virtual /* [helpcontext][helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_Visible( 
            /* [in] */ VARIANT_BOOL Visible) = 0;
        
        virtual /* [helpcontext][helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Top( 
            /* [retval][out] */ __RPC__out long *Top) = 0;
        
        virtual /* [helpcontext][helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Left( 
            /* [retval][out] */ __RPC__out long *Left) = 0;
        
        virtual /* [helpcontext][helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Width( 
            /* [retval][out] */ __RPC__out long *Width) = 0;
        
        virtual /* [helpcontext][helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Height( 
            /* [retval][out] */ __RPC__out long *Height) = 0;
        
        virtual /* [helpcontext][helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_VerticalOffset( 
            /* [retval][out] */ __RPC__out long *VerticalOffset) = 0;
        
        virtual /* [helpcontext][helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_VerticalOffset( 
            /* [in] */ long VerticalOffset) = 0;
        
        virtual /* [helpcontext][helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_HorizontalOffset( 
            /* [retval][out] */ __RPC__out long *HorizontalOffset) = 0;
        
        virtual /* [helpcontext][helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_HorizontalOffset( 
            /* [in] */ long HorizontalOffset) = 0;
        
        virtual /* [helpcontext][helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_AutoShow( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pAutoShow) = 0;
        
        virtual /* [helpcontext][helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_AutoShow( 
            /* [in] */ VARIANT_BOOL AutoShow) = 0;
        
        virtual /* [helpcontext][helpstring][id] */ HRESULT STDMETHODCALLTYPE MoveTo( 
            /* [in] */ long Left,
            /* [in] */ long Top) = 0;
        
        virtual /* [helpcontext][helpstring][id] */ HRESULT STDMETHODCALLTYPE CommitPendingInput( void) = 0;
        
        virtual /* [helpcontext][helpstring][id] */ HRESULT STDMETHODCALLTYPE Refresh( void) = 0;
        
        virtual /* [helpcontext][helpstring][id] */ HRESULT STDMETHODCALLTYPE EnableTsf( 
            /* [in] */ VARIANT_BOOL Enable) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IPenInputPanelVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IPenInputPanel * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IPenInputPanel * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IPenInputPanel * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IPenInputPanel * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IPenInputPanel * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IPenInputPanel * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IPenInputPanel * This,
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
        
        DECLSPEC_XFGVIRT(IPenInputPanel, get_Busy)
        /* [helpcontext][helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Busy )( 
            __RPC__in IPenInputPanel * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *Busy);
        
        DECLSPEC_XFGVIRT(IPenInputPanel, get_Factoid)
        /* [helpcontext][helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Factoid )( 
            __RPC__in IPenInputPanel * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *Factoid);
        
        DECLSPEC_XFGVIRT(IPenInputPanel, put_Factoid)
        /* [helpcontext][helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Factoid )( 
            __RPC__in IPenInputPanel * This,
            /* [in] */ __RPC__in BSTR Factoid);
        
        DECLSPEC_XFGVIRT(IPenInputPanel, get_AttachedEditWindow)
        /* [helpcontext][helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_AttachedEditWindow )( 
            __RPC__in IPenInputPanel * This,
            /* [retval][out] */ __RPC__out LONG32 *AttachedEditWindow);
        
        DECLSPEC_XFGVIRT(IPenInputPanel, put_AttachedEditWindow)
        /* [helpcontext][helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_AttachedEditWindow )( 
            __RPC__in IPenInputPanel * This,
            /* [in] */ LONG32 AttachedEditWindow);
        
        DECLSPEC_XFGVIRT(IPenInputPanel, get_CurrentPanel)
        /* [helpcontext][helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_CurrentPanel )( 
            __RPC__in IPenInputPanel * This,
            /* [retval][out] */ __RPC__out PanelType *CurrentPanel);
        
        DECLSPEC_XFGVIRT(IPenInputPanel, put_CurrentPanel)
        /* [helpcontext][helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_CurrentPanel )( 
            __RPC__in IPenInputPanel * This,
            /* [in] */ PanelType CurrentPanel);
        
        DECLSPEC_XFGVIRT(IPenInputPanel, get_DefaultPanel)
        /* [helpcontext][helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_DefaultPanel )( 
            __RPC__in IPenInputPanel * This,
            /* [retval][out] */ __RPC__out PanelType *pDefaultPanel);
        
        DECLSPEC_XFGVIRT(IPenInputPanel, put_DefaultPanel)
        /* [helpcontext][helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_DefaultPanel )( 
            __RPC__in IPenInputPanel * This,
            /* [in] */ PanelType DefaultPanel);
        
        DECLSPEC_XFGVIRT(IPenInputPanel, get_Visible)
        /* [helpcontext][helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Visible )( 
            __RPC__in IPenInputPanel * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *Visible);
        
        DECLSPEC_XFGVIRT(IPenInputPanel, put_Visible)
        /* [helpcontext][helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Visible )( 
            __RPC__in IPenInputPanel * This,
            /* [in] */ VARIANT_BOOL Visible);
        
        DECLSPEC_XFGVIRT(IPenInputPanel, get_Top)
        /* [helpcontext][helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Top )( 
            __RPC__in IPenInputPanel * This,
            /* [retval][out] */ __RPC__out long *Top);
        
        DECLSPEC_XFGVIRT(IPenInputPanel, get_Left)
        /* [helpcontext][helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Left )( 
            __RPC__in IPenInputPanel * This,
            /* [retval][out] */ __RPC__out long *Left);
        
        DECLSPEC_XFGVIRT(IPenInputPanel, get_Width)
        /* [helpcontext][helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Width )( 
            __RPC__in IPenInputPanel * This,
            /* [retval][out] */ __RPC__out long *Width);
        
        DECLSPEC_XFGVIRT(IPenInputPanel, get_Height)
        /* [helpcontext][helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Height )( 
            __RPC__in IPenInputPanel * This,
            /* [retval][out] */ __RPC__out long *Height);
        
        DECLSPEC_XFGVIRT(IPenInputPanel, get_VerticalOffset)
        /* [helpcontext][helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_VerticalOffset )( 
            __RPC__in IPenInputPanel * This,
            /* [retval][out] */ __RPC__out long *VerticalOffset);
        
        DECLSPEC_XFGVIRT(IPenInputPanel, put_VerticalOffset)
        /* [helpcontext][helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_VerticalOffset )( 
            __RPC__in IPenInputPanel * This,
            /* [in] */ long VerticalOffset);
        
        DECLSPEC_XFGVIRT(IPenInputPanel, get_HorizontalOffset)
        /* [helpcontext][helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_HorizontalOffset )( 
            __RPC__in IPenInputPanel * This,
            /* [retval][out] */ __RPC__out long *HorizontalOffset);
        
        DECLSPEC_XFGVIRT(IPenInputPanel, put_HorizontalOffset)
        /* [helpcontext][helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_HorizontalOffset )( 
            __RPC__in IPenInputPanel * This,
            /* [in] */ long HorizontalOffset);
        
        DECLSPEC_XFGVIRT(IPenInputPanel, get_AutoShow)
        /* [helpcontext][helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_AutoShow )( 
            __RPC__in IPenInputPanel * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pAutoShow);
        
        DECLSPEC_XFGVIRT(IPenInputPanel, put_AutoShow)
        /* [helpcontext][helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_AutoShow )( 
            __RPC__in IPenInputPanel * This,
            /* [in] */ VARIANT_BOOL AutoShow);
        
        DECLSPEC_XFGVIRT(IPenInputPanel, MoveTo)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *MoveTo )( 
            __RPC__in IPenInputPanel * This,
            /* [in] */ long Left,
            /* [in] */ long Top);
        
        DECLSPEC_XFGVIRT(IPenInputPanel, CommitPendingInput)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *CommitPendingInput )( 
            __RPC__in IPenInputPanel * This);
        
        DECLSPEC_XFGVIRT(IPenInputPanel, Refresh)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Refresh )( 
            __RPC__in IPenInputPanel * This);
        
        DECLSPEC_XFGVIRT(IPenInputPanel, EnableTsf)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *EnableTsf )( 
            __RPC__in IPenInputPanel * This,
            /* [in] */ VARIANT_BOOL Enable);
        
        END_INTERFACE
    } IPenInputPanelVtbl;

    interface IPenInputPanel
    {
        CONST_VTBL struct IPenInputPanelVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IPenInputPanel_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IPenInputPanel_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IPenInputPanel_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IPenInputPanel_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IPenInputPanel_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IPenInputPanel_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IPenInputPanel_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IPenInputPanel_get_Busy(This,Busy)	\
    ( (This)->lpVtbl -> get_Busy(This,Busy) ) 

#define IPenInputPanel_get_Factoid(This,Factoid)	\
    ( (This)->lpVtbl -> get_Factoid(This,Factoid) ) 

#define IPenInputPanel_put_Factoid(This,Factoid)	\
    ( (This)->lpVtbl -> put_Factoid(This,Factoid) ) 

#define IPenInputPanel_get_AttachedEditWindow(This,AttachedEditWindow)	\
    ( (This)->lpVtbl -> get_AttachedEditWindow(This,AttachedEditWindow) ) 

#define IPenInputPanel_put_AttachedEditWindow(This,AttachedEditWindow)	\
    ( (This)->lpVtbl -> put_AttachedEditWindow(This,AttachedEditWindow) ) 

#define IPenInputPanel_get_CurrentPanel(This,CurrentPanel)	\
    ( (This)->lpVtbl -> get_CurrentPanel(This,CurrentPanel) ) 

#define IPenInputPanel_put_CurrentPanel(This,CurrentPanel)	\
    ( (This)->lpVtbl -> put_CurrentPanel(This,CurrentPanel) ) 

#define IPenInputPanel_get_DefaultPanel(This,pDefaultPanel)	\
    ( (This)->lpVtbl -> get_DefaultPanel(This,pDefaultPanel) ) 

#define IPenInputPanel_put_DefaultPanel(This,DefaultPanel)	\
    ( (This)->lpVtbl -> put_DefaultPanel(This,DefaultPanel) ) 

#define IPenInputPanel_get_Visible(This,Visible)	\
    ( (This)->lpVtbl -> get_Visible(This,Visible) ) 

#define IPenInputPanel_put_Visible(This,Visible)	\
    ( (This)->lpVtbl -> put_Visible(This,Visible) ) 

#define IPenInputPanel_get_Top(This,Top)	\
    ( (This)->lpVtbl -> get_Top(This,Top) ) 

#define IPenInputPanel_get_Left(This,Left)	\
    ( (This)->lpVtbl -> get_Left(This,Left) ) 

#define IPenInputPanel_get_Width(This,Width)	\
    ( (This)->lpVtbl -> get_Width(This,Width) ) 

#define IPenInputPanel_get_Height(This,Height)	\
    ( (This)->lpVtbl -> get_Height(This,Height) ) 

#define IPenInputPanel_get_VerticalOffset(This,VerticalOffset)	\
    ( (This)->lpVtbl -> get_VerticalOffset(This,VerticalOffset) ) 

#define IPenInputPanel_put_VerticalOffset(This,VerticalOffset)	\
    ( (This)->lpVtbl -> put_VerticalOffset(This,VerticalOffset) ) 

#define IPenInputPanel_get_HorizontalOffset(This,HorizontalOffset)	\
    ( (This)->lpVtbl -> get_HorizontalOffset(This,HorizontalOffset) ) 

#define IPenInputPanel_put_HorizontalOffset(This,HorizontalOffset)	\
    ( (This)->lpVtbl -> put_HorizontalOffset(This,HorizontalOffset) ) 

#define IPenInputPanel_get_AutoShow(This,pAutoShow)	\
    ( (This)->lpVtbl -> get_AutoShow(This,pAutoShow) ) 

#define IPenInputPanel_put_AutoShow(This,AutoShow)	\
    ( (This)->lpVtbl -> put_AutoShow(This,AutoShow) ) 

#define IPenInputPanel_MoveTo(This,Left,Top)	\
    ( (This)->lpVtbl -> MoveTo(This,Left,Top) ) 

#define IPenInputPanel_CommitPendingInput(This)	\
    ( (This)->lpVtbl -> CommitPendingInput(This) ) 

#define IPenInputPanel_Refresh(This)	\
    ( (This)->lpVtbl -> Refresh(This) ) 

#define IPenInputPanel_EnableTsf(This,Enable)	\
    ( (This)->lpVtbl -> EnableTsf(This,Enable) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IPenInputPanel_INTERFACE_DEFINED__ */


#ifndef ___IPenInputPanelEvents_DISPINTERFACE_DEFINED__
#define ___IPenInputPanelEvents_DISPINTERFACE_DEFINED__

/* dispinterface _IPenInputPanelEvents */
/* [helpcontext][helpstring][uuid] */ 


EXTERN_C const IID DIID__IPenInputPanelEvents;

#if defined(__cplusplus) && !defined(CINTERFACE)

    MIDL_INTERFACE("b7e489da-3719-439f-848f-e7acbd820f17")
    _IPenInputPanelEvents : public IDispatch
    {
    };
    
#else 	/* C style interface */

    typedef struct _IPenInputPanelEventsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in _IPenInputPanelEvents * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in _IPenInputPanelEvents * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in _IPenInputPanelEvents * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in _IPenInputPanelEvents * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in _IPenInputPanelEvents * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in _IPenInputPanelEvents * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            _IPenInputPanelEvents * This,
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
    } _IPenInputPanelEventsVtbl;

    interface _IPenInputPanelEvents
    {
        CONST_VTBL struct _IPenInputPanelEventsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define _IPenInputPanelEvents_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define _IPenInputPanelEvents_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define _IPenInputPanelEvents_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define _IPenInputPanelEvents_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define _IPenInputPanelEvents_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define _IPenInputPanelEvents_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define _IPenInputPanelEvents_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */


#endif 	/* ___IPenInputPanelEvents_DISPINTERFACE_DEFINED__ */


#ifndef __IHandwrittenTextInsertion_INTERFACE_DEFINED__
#define __IHandwrittenTextInsertion_INTERFACE_DEFINED__

/* interface IHandwrittenTextInsertion */
/* [helpstring][unique][uuid][object] */ 


EXTERN_C const IID IID_IHandwrittenTextInsertion;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("56FDEA97-ECD6-43e7-AA3A-816BE7785860")
    IHandwrittenTextInsertion : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE InsertRecognitionResultsArray( 
            /* [in] */ __RPC__in SAFEARRAY * psaAlternates,
            /* [in] */ LCID locale,
            /* [in] */ BOOL fAlternateContainsAutoSpacingInformation) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE InsertInkRecognitionResult( 
            /* [in] */ __RPC__in_opt IInkRecognitionResult *pIInkRecoResult,
            /* [in] */ LCID locale,
            /* [in] */ BOOL fAlternateContainsAutoSpacingInformation) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IHandwrittenTextInsertionVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IHandwrittenTextInsertion * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IHandwrittenTextInsertion * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IHandwrittenTextInsertion * This);
        
        DECLSPEC_XFGVIRT(IHandwrittenTextInsertion, InsertRecognitionResultsArray)
        HRESULT ( STDMETHODCALLTYPE *InsertRecognitionResultsArray )( 
            __RPC__in IHandwrittenTextInsertion * This,
            /* [in] */ __RPC__in SAFEARRAY * psaAlternates,
            /* [in] */ LCID locale,
            /* [in] */ BOOL fAlternateContainsAutoSpacingInformation);
        
        DECLSPEC_XFGVIRT(IHandwrittenTextInsertion, InsertInkRecognitionResult)
        HRESULT ( STDMETHODCALLTYPE *InsertInkRecognitionResult )( 
            __RPC__in IHandwrittenTextInsertion * This,
            /* [in] */ __RPC__in_opt IInkRecognitionResult *pIInkRecoResult,
            /* [in] */ LCID locale,
            /* [in] */ BOOL fAlternateContainsAutoSpacingInformation);
        
        END_INTERFACE
    } IHandwrittenTextInsertionVtbl;

    interface IHandwrittenTextInsertion
    {
        CONST_VTBL struct IHandwrittenTextInsertionVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IHandwrittenTextInsertion_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IHandwrittenTextInsertion_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IHandwrittenTextInsertion_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IHandwrittenTextInsertion_InsertRecognitionResultsArray(This,psaAlternates,locale,fAlternateContainsAutoSpacingInformation)	\
    ( (This)->lpVtbl -> InsertRecognitionResultsArray(This,psaAlternates,locale,fAlternateContainsAutoSpacingInformation) ) 

#define IHandwrittenTextInsertion_InsertInkRecognitionResult(This,pIInkRecoResult,locale,fAlternateContainsAutoSpacingInformation)	\
    ( (This)->lpVtbl -> InsertInkRecognitionResult(This,pIInkRecoResult,locale,fAlternateContainsAutoSpacingInformation) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IHandwrittenTextInsertion_INTERFACE_DEFINED__ */


EXTERN_C const CLSID CLSID_HandwrittenTextInsertion;

#ifdef __cplusplus

class DECLSPEC_UUID("9F074EE2-E6E9-4d8a-A047-EB5B5C3C55DA")
HandwrittenTextInsertion;
#endif

EXTERN_C const CLSID CLSID_PenInputPanel;

#ifdef __cplusplus

class DECLSPEC_UUID("f744e496-1b5a-489e-81dc-fbd7ac6298a8")
PenInputPanel;
#endif

#ifndef __ITextInputPanelEventSink_INTERFACE_DEFINED__
#define __ITextInputPanelEventSink_INTERFACE_DEFINED__

/* interface ITextInputPanelEventSink */
/* [oleautomation][unique][uuid][object] */ 


EXTERN_C const IID IID_ITextInputPanelEventSink;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("27560408-8e64-4fe1-804e-421201584b31")
    ITextInputPanelEventSink : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE InPlaceStateChanging( 
            /* [in] */ InPlaceState oldInPlaceState,
            /* [in] */ InPlaceState newInPlaceState) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE InPlaceStateChanged( 
            /* [in] */ InPlaceState oldInPlaceState,
            /* [in] */ InPlaceState newInPlaceState) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE InPlaceSizeChanging( 
            /* [in] */ RECT oldBoundingRectangle,
            /* [in] */ RECT newBoundingRectangle) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE InPlaceSizeChanged( 
            /* [in] */ RECT oldBoundingRectangle,
            /* [in] */ RECT newBoundingRectangle) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE InputAreaChanging( 
            /* [in] */ PanelInputArea oldInputArea,
            /* [in] */ PanelInputArea newInputArea) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE InputAreaChanged( 
            /* [in] */ PanelInputArea oldInputArea,
            /* [in] */ PanelInputArea newInputArea) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CorrectionModeChanging( 
            /* [in] */ CorrectionMode oldCorrectionMode,
            /* [in] */ CorrectionMode newCorrectionMode) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CorrectionModeChanged( 
            /* [in] */ CorrectionMode oldCorrectionMode,
            /* [in] */ CorrectionMode newCorrectionMode) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE InPlaceVisibilityChanging( 
            /* [in] */ BOOL oldVisible,
            /* [in] */ BOOL newVisible) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE InPlaceVisibilityChanged( 
            /* [in] */ BOOL oldVisible,
            /* [in] */ BOOL newVisible) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE TextInserting( 
            /* [in] */ __RPC__in SAFEARRAY * Ink) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE TextInserted( 
            /* [in] */ __RPC__in SAFEARRAY * Ink) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITextInputPanelEventSinkVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITextInputPanelEventSink * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITextInputPanelEventSink * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITextInputPanelEventSink * This);
        
        DECLSPEC_XFGVIRT(ITextInputPanelEventSink, InPlaceStateChanging)
        HRESULT ( STDMETHODCALLTYPE *InPlaceStateChanging )( 
            __RPC__in ITextInputPanelEventSink * This,
            /* [in] */ InPlaceState oldInPlaceState,
            /* [in] */ InPlaceState newInPlaceState);
        
        DECLSPEC_XFGVIRT(ITextInputPanelEventSink, InPlaceStateChanged)
        HRESULT ( STDMETHODCALLTYPE *InPlaceStateChanged )( 
            __RPC__in ITextInputPanelEventSink * This,
            /* [in] */ InPlaceState oldInPlaceState,
            /* [in] */ InPlaceState newInPlaceState);
        
        DECLSPEC_XFGVIRT(ITextInputPanelEventSink, InPlaceSizeChanging)
        HRESULT ( STDMETHODCALLTYPE *InPlaceSizeChanging )( 
            __RPC__in ITextInputPanelEventSink * This,
            /* [in] */ RECT oldBoundingRectangle,
            /* [in] */ RECT newBoundingRectangle);
        
        DECLSPEC_XFGVIRT(ITextInputPanelEventSink, InPlaceSizeChanged)
        HRESULT ( STDMETHODCALLTYPE *InPlaceSizeChanged )( 
            __RPC__in ITextInputPanelEventSink * This,
            /* [in] */ RECT oldBoundingRectangle,
            /* [in] */ RECT newBoundingRectangle);
        
        DECLSPEC_XFGVIRT(ITextInputPanelEventSink, InputAreaChanging)
        HRESULT ( STDMETHODCALLTYPE *InputAreaChanging )( 
            __RPC__in ITextInputPanelEventSink * This,
            /* [in] */ PanelInputArea oldInputArea,
            /* [in] */ PanelInputArea newInputArea);
        
        DECLSPEC_XFGVIRT(ITextInputPanelEventSink, InputAreaChanged)
        HRESULT ( STDMETHODCALLTYPE *InputAreaChanged )( 
            __RPC__in ITextInputPanelEventSink * This,
            /* [in] */ PanelInputArea oldInputArea,
            /* [in] */ PanelInputArea newInputArea);
        
        DECLSPEC_XFGVIRT(ITextInputPanelEventSink, CorrectionModeChanging)
        HRESULT ( STDMETHODCALLTYPE *CorrectionModeChanging )( 
            __RPC__in ITextInputPanelEventSink * This,
            /* [in] */ CorrectionMode oldCorrectionMode,
            /* [in] */ CorrectionMode newCorrectionMode);
        
        DECLSPEC_XFGVIRT(ITextInputPanelEventSink, CorrectionModeChanged)
        HRESULT ( STDMETHODCALLTYPE *CorrectionModeChanged )( 
            __RPC__in ITextInputPanelEventSink * This,
            /* [in] */ CorrectionMode oldCorrectionMode,
            /* [in] */ CorrectionMode newCorrectionMode);
        
        DECLSPEC_XFGVIRT(ITextInputPanelEventSink, InPlaceVisibilityChanging)
        HRESULT ( STDMETHODCALLTYPE *InPlaceVisibilityChanging )( 
            __RPC__in ITextInputPanelEventSink * This,
            /* [in] */ BOOL oldVisible,
            /* [in] */ BOOL newVisible);
        
        DECLSPEC_XFGVIRT(ITextInputPanelEventSink, InPlaceVisibilityChanged)
        HRESULT ( STDMETHODCALLTYPE *InPlaceVisibilityChanged )( 
            __RPC__in ITextInputPanelEventSink * This,
            /* [in] */ BOOL oldVisible,
            /* [in] */ BOOL newVisible);
        
        DECLSPEC_XFGVIRT(ITextInputPanelEventSink, TextInserting)
        HRESULT ( STDMETHODCALLTYPE *TextInserting )( 
            __RPC__in ITextInputPanelEventSink * This,
            /* [in] */ __RPC__in SAFEARRAY * Ink);
        
        DECLSPEC_XFGVIRT(ITextInputPanelEventSink, TextInserted)
        HRESULT ( STDMETHODCALLTYPE *TextInserted )( 
            __RPC__in ITextInputPanelEventSink * This,
            /* [in] */ __RPC__in SAFEARRAY * Ink);
        
        END_INTERFACE
    } ITextInputPanelEventSinkVtbl;

    interface ITextInputPanelEventSink
    {
        CONST_VTBL struct ITextInputPanelEventSinkVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITextInputPanelEventSink_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITextInputPanelEventSink_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITextInputPanelEventSink_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITextInputPanelEventSink_InPlaceStateChanging(This,oldInPlaceState,newInPlaceState)	\
    ( (This)->lpVtbl -> InPlaceStateChanging(This,oldInPlaceState,newInPlaceState) ) 

#define ITextInputPanelEventSink_InPlaceStateChanged(This,oldInPlaceState,newInPlaceState)	\
    ( (This)->lpVtbl -> InPlaceStateChanged(This,oldInPlaceState,newInPlaceState) ) 

#define ITextInputPanelEventSink_InPlaceSizeChanging(This,oldBoundingRectangle,newBoundingRectangle)	\
    ( (This)->lpVtbl -> InPlaceSizeChanging(This,oldBoundingRectangle,newBoundingRectangle) ) 

#define ITextInputPanelEventSink_InPlaceSizeChanged(This,oldBoundingRectangle,newBoundingRectangle)	\
    ( (This)->lpVtbl -> InPlaceSizeChanged(This,oldBoundingRectangle,newBoundingRectangle) ) 

#define ITextInputPanelEventSink_InputAreaChanging(This,oldInputArea,newInputArea)	\
    ( (This)->lpVtbl -> InputAreaChanging(This,oldInputArea,newInputArea) ) 

#define ITextInputPanelEventSink_InputAreaChanged(This,oldInputArea,newInputArea)	\
    ( (This)->lpVtbl -> InputAreaChanged(This,oldInputArea,newInputArea) ) 

#define ITextInputPanelEventSink_CorrectionModeChanging(This,oldCorrectionMode,newCorrectionMode)	\
    ( (This)->lpVtbl -> CorrectionModeChanging(This,oldCorrectionMode,newCorrectionMode) ) 

#define ITextInputPanelEventSink_CorrectionModeChanged(This,oldCorrectionMode,newCorrectionMode)	\
    ( (This)->lpVtbl -> CorrectionModeChanged(This,oldCorrectionMode,newCorrectionMode) ) 

#define ITextInputPanelEventSink_InPlaceVisibilityChanging(This,oldVisible,newVisible)	\
    ( (This)->lpVtbl -> InPlaceVisibilityChanging(This,oldVisible,newVisible) ) 

#define ITextInputPanelEventSink_InPlaceVisibilityChanged(This,oldVisible,newVisible)	\
    ( (This)->lpVtbl -> InPlaceVisibilityChanged(This,oldVisible,newVisible) ) 

#define ITextInputPanelEventSink_TextInserting(This,Ink)	\
    ( (This)->lpVtbl -> TextInserting(This,Ink) ) 

#define ITextInputPanelEventSink_TextInserted(This,Ink)	\
    ( (This)->lpVtbl -> TextInserted(This,Ink) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITextInputPanelEventSink_INTERFACE_DEFINED__ */


#ifndef __ITextInputPanel_INTERFACE_DEFINED__
#define __ITextInputPanel_INTERFACE_DEFINED__

/* interface ITextInputPanel */
/* [oleautomation][unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_ITextInputPanel;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("6b6a65a5-6af3-46c2-b6ea-56cd1f80df71")
    ITextInputPanel : public IUnknown
    {
    public:
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_AttachedEditWindow( 
            /* [retval][out] */ __RPC__deref_out_opt HWND *AttachedEditWindow) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_AttachedEditWindow( 
            /* [in] */ __RPC__in HWND AttachedEditWindow) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_CurrentInteractionMode( 
            /* [retval][out] */ __RPC__out InteractionMode *CurrentInteractionMode) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_DefaultInPlaceState( 
            /* [retval][out] */ __RPC__out InPlaceState *State) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_DefaultInPlaceState( 
            /* [in] */ InPlaceState State) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_CurrentInPlaceState( 
            /* [retval][out] */ __RPC__out InPlaceState *State) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_DefaultInputArea( 
            /* [retval][out] */ __RPC__out PanelInputArea *Area) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_DefaultInputArea( 
            /* [in] */ PanelInputArea Area) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_CurrentInputArea( 
            /* [retval][out] */ __RPC__out PanelInputArea *Area) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_CurrentCorrectionMode( 
            /* [retval][out] */ __RPC__out CorrectionMode *Mode) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_PreferredInPlaceDirection( 
            /* [retval][out] */ __RPC__out InPlaceDirection *Direction) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_PreferredInPlaceDirection( 
            /* [in] */ InPlaceDirection Direction) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_ExpandPostInsertionCorrection( 
            /* [retval][out] */ __RPC__out BOOL *Expand) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_ExpandPostInsertionCorrection( 
            /* [in] */ BOOL Expand) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_InPlaceVisibleOnFocus( 
            /* [retval][out] */ __RPC__out BOOL *Visible) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_InPlaceVisibleOnFocus( 
            /* [in] */ BOOL Visible) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_InPlaceBoundingRectangle( 
            /* [retval][out] */ __RPC__out RECT *BoundingRectangle) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_PopUpCorrectionHeight( 
            /* [retval][out] */ __RPC__out int *Height) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_PopDownCorrectionHeight( 
            /* [retval][out] */ __RPC__out int *Height) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CommitPendingInput( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetInPlaceVisibility( 
            BOOL Visible) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetInPlacePosition( 
            int xPosition,
            int yPosition,
            CorrectionPosition position) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetInPlaceHoverTargetPosition( 
            int xPosition,
            int yPosition) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Advise( 
            __RPC__in_opt ITextInputPanelEventSink *EventSink,
            DWORD EventMask) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Unadvise( 
            __RPC__in_opt ITextInputPanelEventSink *EventSink) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITextInputPanelVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITextInputPanel * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITextInputPanel * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITextInputPanel * This);
        
        DECLSPEC_XFGVIRT(ITextInputPanel, get_AttachedEditWindow)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_AttachedEditWindow )( 
            __RPC__in ITextInputPanel * This,
            /* [retval][out] */ __RPC__deref_out_opt HWND *AttachedEditWindow);
        
        DECLSPEC_XFGVIRT(ITextInputPanel, put_AttachedEditWindow)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_AttachedEditWindow )( 
            __RPC__in ITextInputPanel * This,
            /* [in] */ __RPC__in HWND AttachedEditWindow);
        
        DECLSPEC_XFGVIRT(ITextInputPanel, get_CurrentInteractionMode)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_CurrentInteractionMode )( 
            __RPC__in ITextInputPanel * This,
            /* [retval][out] */ __RPC__out InteractionMode *CurrentInteractionMode);
        
        DECLSPEC_XFGVIRT(ITextInputPanel, get_DefaultInPlaceState)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_DefaultInPlaceState )( 
            __RPC__in ITextInputPanel * This,
            /* [retval][out] */ __RPC__out InPlaceState *State);
        
        DECLSPEC_XFGVIRT(ITextInputPanel, put_DefaultInPlaceState)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_DefaultInPlaceState )( 
            __RPC__in ITextInputPanel * This,
            /* [in] */ InPlaceState State);
        
        DECLSPEC_XFGVIRT(ITextInputPanel, get_CurrentInPlaceState)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_CurrentInPlaceState )( 
            __RPC__in ITextInputPanel * This,
            /* [retval][out] */ __RPC__out InPlaceState *State);
        
        DECLSPEC_XFGVIRT(ITextInputPanel, get_DefaultInputArea)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_DefaultInputArea )( 
            __RPC__in ITextInputPanel * This,
            /* [retval][out] */ __RPC__out PanelInputArea *Area);
        
        DECLSPEC_XFGVIRT(ITextInputPanel, put_DefaultInputArea)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_DefaultInputArea )( 
            __RPC__in ITextInputPanel * This,
            /* [in] */ PanelInputArea Area);
        
        DECLSPEC_XFGVIRT(ITextInputPanel, get_CurrentInputArea)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_CurrentInputArea )( 
            __RPC__in ITextInputPanel * This,
            /* [retval][out] */ __RPC__out PanelInputArea *Area);
        
        DECLSPEC_XFGVIRT(ITextInputPanel, get_CurrentCorrectionMode)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_CurrentCorrectionMode )( 
            __RPC__in ITextInputPanel * This,
            /* [retval][out] */ __RPC__out CorrectionMode *Mode);
        
        DECLSPEC_XFGVIRT(ITextInputPanel, get_PreferredInPlaceDirection)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_PreferredInPlaceDirection )( 
            __RPC__in ITextInputPanel * This,
            /* [retval][out] */ __RPC__out InPlaceDirection *Direction);
        
        DECLSPEC_XFGVIRT(ITextInputPanel, put_PreferredInPlaceDirection)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_PreferredInPlaceDirection )( 
            __RPC__in ITextInputPanel * This,
            /* [in] */ InPlaceDirection Direction);
        
        DECLSPEC_XFGVIRT(ITextInputPanel, get_ExpandPostInsertionCorrection)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_ExpandPostInsertionCorrection )( 
            __RPC__in ITextInputPanel * This,
            /* [retval][out] */ __RPC__out BOOL *Expand);
        
        DECLSPEC_XFGVIRT(ITextInputPanel, put_ExpandPostInsertionCorrection)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_ExpandPostInsertionCorrection )( 
            __RPC__in ITextInputPanel * This,
            /* [in] */ BOOL Expand);
        
        DECLSPEC_XFGVIRT(ITextInputPanel, get_InPlaceVisibleOnFocus)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_InPlaceVisibleOnFocus )( 
            __RPC__in ITextInputPanel * This,
            /* [retval][out] */ __RPC__out BOOL *Visible);
        
        DECLSPEC_XFGVIRT(ITextInputPanel, put_InPlaceVisibleOnFocus)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_InPlaceVisibleOnFocus )( 
            __RPC__in ITextInputPanel * This,
            /* [in] */ BOOL Visible);
        
        DECLSPEC_XFGVIRT(ITextInputPanel, get_InPlaceBoundingRectangle)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_InPlaceBoundingRectangle )( 
            __RPC__in ITextInputPanel * This,
            /* [retval][out] */ __RPC__out RECT *BoundingRectangle);
        
        DECLSPEC_XFGVIRT(ITextInputPanel, get_PopUpCorrectionHeight)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_PopUpCorrectionHeight )( 
            __RPC__in ITextInputPanel * This,
            /* [retval][out] */ __RPC__out int *Height);
        
        DECLSPEC_XFGVIRT(ITextInputPanel, get_PopDownCorrectionHeight)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_PopDownCorrectionHeight )( 
            __RPC__in ITextInputPanel * This,
            /* [retval][out] */ __RPC__out int *Height);
        
        DECLSPEC_XFGVIRT(ITextInputPanel, CommitPendingInput)
        HRESULT ( STDMETHODCALLTYPE *CommitPendingInput )( 
            __RPC__in ITextInputPanel * This);
        
        DECLSPEC_XFGVIRT(ITextInputPanel, SetInPlaceVisibility)
        HRESULT ( STDMETHODCALLTYPE *SetInPlaceVisibility )( 
            __RPC__in ITextInputPanel * This,
            BOOL Visible);
        
        DECLSPEC_XFGVIRT(ITextInputPanel, SetInPlacePosition)
        HRESULT ( STDMETHODCALLTYPE *SetInPlacePosition )( 
            __RPC__in ITextInputPanel * This,
            int xPosition,
            int yPosition,
            CorrectionPosition position);
        
        DECLSPEC_XFGVIRT(ITextInputPanel, SetInPlaceHoverTargetPosition)
        HRESULT ( STDMETHODCALLTYPE *SetInPlaceHoverTargetPosition )( 
            __RPC__in ITextInputPanel * This,
            int xPosition,
            int yPosition);
        
        DECLSPEC_XFGVIRT(ITextInputPanel, Advise)
        HRESULT ( STDMETHODCALLTYPE *Advise )( 
            __RPC__in ITextInputPanel * This,
            __RPC__in_opt ITextInputPanelEventSink *EventSink,
            DWORD EventMask);
        
        DECLSPEC_XFGVIRT(ITextInputPanel, Unadvise)
        HRESULT ( STDMETHODCALLTYPE *Unadvise )( 
            __RPC__in ITextInputPanel * This,
            __RPC__in_opt ITextInputPanelEventSink *EventSink);
        
        END_INTERFACE
    } ITextInputPanelVtbl;

    interface ITextInputPanel
    {
        CONST_VTBL struct ITextInputPanelVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITextInputPanel_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITextInputPanel_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITextInputPanel_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITextInputPanel_get_AttachedEditWindow(This,AttachedEditWindow)	\
    ( (This)->lpVtbl -> get_AttachedEditWindow(This,AttachedEditWindow) ) 

#define ITextInputPanel_put_AttachedEditWindow(This,AttachedEditWindow)	\
    ( (This)->lpVtbl -> put_AttachedEditWindow(This,AttachedEditWindow) ) 

#define ITextInputPanel_get_CurrentInteractionMode(This,CurrentInteractionMode)	\
    ( (This)->lpVtbl -> get_CurrentInteractionMode(This,CurrentInteractionMode) ) 

#define ITextInputPanel_get_DefaultInPlaceState(This,State)	\
    ( (This)->lpVtbl -> get_DefaultInPlaceState(This,State) ) 

#define ITextInputPanel_put_DefaultInPlaceState(This,State)	\
    ( (This)->lpVtbl -> put_DefaultInPlaceState(This,State) ) 

#define ITextInputPanel_get_CurrentInPlaceState(This,State)	\
    ( (This)->lpVtbl -> get_CurrentInPlaceState(This,State) ) 

#define ITextInputPanel_get_DefaultInputArea(This,Area)	\
    ( (This)->lpVtbl -> get_DefaultInputArea(This,Area) ) 

#define ITextInputPanel_put_DefaultInputArea(This,Area)	\
    ( (This)->lpVtbl -> put_DefaultInputArea(This,Area) ) 

#define ITextInputPanel_get_CurrentInputArea(This,Area)	\
    ( (This)->lpVtbl -> get_CurrentInputArea(This,Area) ) 

#define ITextInputPanel_get_CurrentCorrectionMode(This,Mode)	\
    ( (This)->lpVtbl -> get_CurrentCorrectionMode(This,Mode) ) 

#define ITextInputPanel_get_PreferredInPlaceDirection(This,Direction)	\
    ( (This)->lpVtbl -> get_PreferredInPlaceDirection(This,Direction) ) 

#define ITextInputPanel_put_PreferredInPlaceDirection(This,Direction)	\
    ( (This)->lpVtbl -> put_PreferredInPlaceDirection(This,Direction) ) 

#define ITextInputPanel_get_ExpandPostInsertionCorrection(This,Expand)	\
    ( (This)->lpVtbl -> get_ExpandPostInsertionCorrection(This,Expand) ) 

#define ITextInputPanel_put_ExpandPostInsertionCorrection(This,Expand)	\
    ( (This)->lpVtbl -> put_ExpandPostInsertionCorrection(This,Expand) ) 

#define ITextInputPanel_get_InPlaceVisibleOnFocus(This,Visible)	\
    ( (This)->lpVtbl -> get_InPlaceVisibleOnFocus(This,Visible) ) 

#define ITextInputPanel_put_InPlaceVisibleOnFocus(This,Visible)	\
    ( (This)->lpVtbl -> put_InPlaceVisibleOnFocus(This,Visible) ) 

#define ITextInputPanel_get_InPlaceBoundingRectangle(This,BoundingRectangle)	\
    ( (This)->lpVtbl -> get_InPlaceBoundingRectangle(This,BoundingRectangle) ) 

#define ITextInputPanel_get_PopUpCorrectionHeight(This,Height)	\
    ( (This)->lpVtbl -> get_PopUpCorrectionHeight(This,Height) ) 

#define ITextInputPanel_get_PopDownCorrectionHeight(This,Height)	\
    ( (This)->lpVtbl -> get_PopDownCorrectionHeight(This,Height) ) 

#define ITextInputPanel_CommitPendingInput(This)	\
    ( (This)->lpVtbl -> CommitPendingInput(This) ) 

#define ITextInputPanel_SetInPlaceVisibility(This,Visible)	\
    ( (This)->lpVtbl -> SetInPlaceVisibility(This,Visible) ) 

#define ITextInputPanel_SetInPlacePosition(This,xPosition,yPosition,position)	\
    ( (This)->lpVtbl -> SetInPlacePosition(This,xPosition,yPosition,position) ) 

#define ITextInputPanel_SetInPlaceHoverTargetPosition(This,xPosition,yPosition)	\
    ( (This)->lpVtbl -> SetInPlaceHoverTargetPosition(This,xPosition,yPosition) ) 

#define ITextInputPanel_Advise(This,EventSink,EventMask)	\
    ( (This)->lpVtbl -> Advise(This,EventSink,EventMask) ) 

#define ITextInputPanel_Unadvise(This,EventSink)	\
    ( (This)->lpVtbl -> Unadvise(This,EventSink) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITextInputPanel_INTERFACE_DEFINED__ */


#ifndef __IInputPanelWindowHandle_INTERFACE_DEFINED__
#define __IInputPanelWindowHandle_INTERFACE_DEFINED__

/* interface IInputPanelWindowHandle */
/* [oleautomation][unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IInputPanelWindowHandle;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("4AF81847-FDC4-4fc3-AD0B-422479C1B935")
    IInputPanelWindowHandle : public IUnknown
    {
    public:
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_AttachedEditWindow32( 
            /* [retval][out] */ __RPC__out LONG32 *AttachedEditWindow) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_AttachedEditWindow32( 
            /* [in] */ LONG32 AttachedEditWindow) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_AttachedEditWindow64( 
            /* [retval][out] */ __RPC__out LONG64 *AttachedEditWindow) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_AttachedEditWindow64( 
            /* [in] */ LONG64 AttachedEditWindow) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IInputPanelWindowHandleVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IInputPanelWindowHandle * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IInputPanelWindowHandle * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IInputPanelWindowHandle * This);
        
        DECLSPEC_XFGVIRT(IInputPanelWindowHandle, get_AttachedEditWindow32)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_AttachedEditWindow32 )( 
            __RPC__in IInputPanelWindowHandle * This,
            /* [retval][out] */ __RPC__out LONG32 *AttachedEditWindow);
        
        DECLSPEC_XFGVIRT(IInputPanelWindowHandle, put_AttachedEditWindow32)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_AttachedEditWindow32 )( 
            __RPC__in IInputPanelWindowHandle * This,
            /* [in] */ LONG32 AttachedEditWindow);
        
        DECLSPEC_XFGVIRT(IInputPanelWindowHandle, get_AttachedEditWindow64)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_AttachedEditWindow64 )( 
            __RPC__in IInputPanelWindowHandle * This,
            /* [retval][out] */ __RPC__out LONG64 *AttachedEditWindow);
        
        DECLSPEC_XFGVIRT(IInputPanelWindowHandle, put_AttachedEditWindow64)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_AttachedEditWindow64 )( 
            __RPC__in IInputPanelWindowHandle * This,
            /* [in] */ LONG64 AttachedEditWindow);
        
        END_INTERFACE
    } IInputPanelWindowHandleVtbl;

    interface IInputPanelWindowHandle
    {
        CONST_VTBL struct IInputPanelWindowHandleVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IInputPanelWindowHandle_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IInputPanelWindowHandle_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IInputPanelWindowHandle_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IInputPanelWindowHandle_get_AttachedEditWindow32(This,AttachedEditWindow)	\
    ( (This)->lpVtbl -> get_AttachedEditWindow32(This,AttachedEditWindow) ) 

#define IInputPanelWindowHandle_put_AttachedEditWindow32(This,AttachedEditWindow)	\
    ( (This)->lpVtbl -> put_AttachedEditWindow32(This,AttachedEditWindow) ) 

#define IInputPanelWindowHandle_get_AttachedEditWindow64(This,AttachedEditWindow)	\
    ( (This)->lpVtbl -> get_AttachedEditWindow64(This,AttachedEditWindow) ) 

#define IInputPanelWindowHandle_put_AttachedEditWindow64(This,AttachedEditWindow)	\
    ( (This)->lpVtbl -> put_AttachedEditWindow64(This,AttachedEditWindow) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IInputPanelWindowHandle_INTERFACE_DEFINED__ */


#ifndef __ITextInputPanelRunInfo_INTERFACE_DEFINED__
#define __ITextInputPanelRunInfo_INTERFACE_DEFINED__

/* interface ITextInputPanelRunInfo */
/* [oleautomation][unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_ITextInputPanelRunInfo;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("9F424568-1920-48cc-9811-A993CBF5ADBA")
    ITextInputPanelRunInfo : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE IsTipRunning( 
            /* [out] */ __RPC__out BOOL *pfRunning) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITextInputPanelRunInfoVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITextInputPanelRunInfo * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITextInputPanelRunInfo * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITextInputPanelRunInfo * This);
        
        DECLSPEC_XFGVIRT(ITextInputPanelRunInfo, IsTipRunning)
        HRESULT ( STDMETHODCALLTYPE *IsTipRunning )( 
            __RPC__in ITextInputPanelRunInfo * This,
            /* [out] */ __RPC__out BOOL *pfRunning);
        
        END_INTERFACE
    } ITextInputPanelRunInfoVtbl;

    interface ITextInputPanelRunInfo
    {
        CONST_VTBL struct ITextInputPanelRunInfoVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITextInputPanelRunInfo_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITextInputPanelRunInfo_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITextInputPanelRunInfo_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITextInputPanelRunInfo_IsTipRunning(This,pfRunning)	\
    ( (This)->lpVtbl -> IsTipRunning(This,pfRunning) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITextInputPanelRunInfo_INTERFACE_DEFINED__ */


EXTERN_C const CLSID CLSID_TextInputPanel;

#ifdef __cplusplus

class DECLSPEC_UUID("f9b189d7-228b-4f2b-8650-b97f59e02c8c")
TextInputPanel;
#endif

EXTERN_C const CLSID CLSID_PenInputPanel_Internal;

#ifdef __cplusplus

class DECLSPEC_UUID("802B1FB9-056B-4720-B0CC-80D23B71171E")
PenInputPanel_Internal;
#endif
#endif /* __PenInputPanelLib_LIBRARY_DEFINED__ */

/* interface __MIDL_itf_peninputpanel_0000_0001 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_peninputpanel_0000_0001_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_peninputpanel_0000_0001_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


