

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

#ifndef COM_NO_WINDOWS_H
#include "windows.h"
#include "ole2.h"
#endif /*COM_NO_WINDOWS_H*/

#ifndef __micaut_h__
#define __micaut_h__

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

#ifndef __IMathInputControl_FWD_DEFINED__
#define __IMathInputControl_FWD_DEFINED__
typedef interface IMathInputControl IMathInputControl;

#endif 	/* __IMathInputControl_FWD_DEFINED__ */


#ifndef ___IMathInputControlEvents_FWD_DEFINED__
#define ___IMathInputControlEvents_FWD_DEFINED__
typedef interface _IMathInputControlEvents _IMathInputControlEvents;

#endif 	/* ___IMathInputControlEvents_FWD_DEFINED__ */


#ifndef __MathInputControl_FWD_DEFINED__
#define __MathInputControl_FWD_DEFINED__

#ifdef __cplusplus
typedef class MathInputControl MathInputControl;
#else
typedef struct MathInputControl MathInputControl;
#endif /* __cplusplus */

#endif 	/* __MathInputControl_FWD_DEFINED__ */


/* header files for imported files */
#include "oaidl.h"
#include "ocidl.h"
#include "msinkaut.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_micaut_0000_0000 */
/* [local] */ 

#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
typedef /* [public][uuid] */  DECLSPEC_UUID("D7803AF6-B018-4a42-AE48-B2D2319BCB05") 
enum __MIDL___MIDL_itf_micaut_0000_0000_0001
    {
        MICUIELEMENT_BUTTON_WRITE	= ( 1 << 0 ) ,
        MICUIELEMENT_BUTTON_ERASE	= ( 1 << 1 ) ,
        MICUIELEMENT_BUTTON_CORRECT	= ( 1 << 2 ) ,
        MICUIELEMENT_BUTTON_CLEAR	= ( 1 << 3 ) ,
        MICUIELEMENT_BUTTON_UNDO	= ( 1 << 4 ) ,
        MICUIELEMENT_BUTTON_REDO	= ( 1 << 5 ) ,
        MICUIELEMENT_BUTTON_INSERT	= ( 1 << 6 ) ,
        MICUIELEMENT_BUTTON_CANCEL	= ( 1 << 7 ) ,
        MICUIELEMENT_INKPANEL_BACKGROUND	= ( 1 << 8 ) ,
        MICUIELEMENT_RESULTPANEL_BACKGROUND	= ( 1 << 9 ) 
    } 	MICUIELEMENT;

typedef /* [public][uuid] */  DECLSPEC_UUID("117F1516-9539-4e26-9CCF-CD7511AE9BF1") 
enum __MIDL___MIDL_itf_micaut_0000_0000_0002
    {
        MICUIELEMENTSTATE_NORMAL	= 1,
        MICUIELEMENTSTATE_HOT	= ( MICUIELEMENTSTATE_NORMAL + 1 ) ,
        MICUIELEMENTSTATE_PRESSED	= ( MICUIELEMENTSTATE_HOT + 1 ) ,
        MICUIELEMENTSTATE_DISABLED	= ( MICUIELEMENTSTATE_PRESSED + 1 ) 
    } 	MICUIELEMENTSTATE;

typedef /* [hidden] */ 
enum DISPID_MathInputControlEvents
    {
        DISPID_MICInsert	= 0,
        DISPID_MICClose	= ( DISPID_MICInsert + 1 ) ,
        DISPID_MICPaint	= ( DISPID_MICClose + 1 ) ,
        DISPID_MICClear	= ( DISPID_MICPaint + 1 ) 
    } 	DISPID_MathInputControlEvents;



extern RPC_IF_HANDLE __MIDL_itf_micaut_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_micaut_0000_0000_v0_0_s_ifspec;

#ifndef __IMathInputControl_INTERFACE_DEFINED__
#define __IMathInputControl_INTERFACE_DEFINED__

/* interface IMathInputControl */
/* [unique][helpstring][nonextensible][dual][uuid][object] */ 


EXTERN_C const IID IID_IMathInputControl;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("EBA615AA-FAC6-4738-BA5F-FF09E9FE473E")
    IMathInputControl : public IDispatch
    {
    public:
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Show( void) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Hide( void) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE IsVisible( 
            /* [out] */ __RPC__out VARIANT_BOOL *pvbShown) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetPosition( 
            /* [out] */ __RPC__out LONG *Left,
            /* [out] */ __RPC__out LONG *Top,
            /* [out] */ __RPC__out LONG *Right,
            /* [out] */ __RPC__out LONG *Bottom) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE SetPosition( 
            /* [in] */ LONG Left,
            /* [in] */ LONG Top,
            /* [in] */ LONG Right,
            /* [in] */ LONG Bottom) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Clear( void) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE SetCustomPaint( 
            /* [in] */ LONG Element,
            /* [in] */ VARIANT_BOOL Paint) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE SetCaptionText( 
            /* [in] */ __RPC__in BSTR CaptionText) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE LoadInk( 
            /* [in] */ __RPC__in_opt IInkDisp *Ink) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE SetOwnerWindow( 
            /* [in] */ LONG_PTR OwnerWindow) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE EnableExtendedButtons( 
            /* [in] */ VARIANT_BOOL Extended) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetPreviewHeight( 
            /* [out] */ __RPC__out LONG *Height) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE SetPreviewHeight( 
            /* [in] */ LONG Height) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE EnableAutoGrow( 
            /* [in] */ VARIANT_BOOL AutoGrow) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE AddFunctionName( 
            /* [in] */ __RPC__in BSTR FunctionName) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE RemoveFunctionName( 
            /* [in] */ __RPC__in BSTR FunctionName) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetHoverIcon( 
            /* [out] */ __RPC__deref_out_opt IPictureDisp **HoverImage) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMathInputControlVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMathInputControl * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMathInputControl * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMathInputControl * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IMathInputControl * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IMathInputControl * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IMathInputControl * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IMathInputControl * This,
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
        
        DECLSPEC_XFGVIRT(IMathInputControl, Show)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Show )( 
            __RPC__in IMathInputControl * This);
        
        DECLSPEC_XFGVIRT(IMathInputControl, Hide)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Hide )( 
            __RPC__in IMathInputControl * This);
        
        DECLSPEC_XFGVIRT(IMathInputControl, IsVisible)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *IsVisible )( 
            __RPC__in IMathInputControl * This,
            /* [out] */ __RPC__out VARIANT_BOOL *pvbShown);
        
        DECLSPEC_XFGVIRT(IMathInputControl, GetPosition)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetPosition )( 
            __RPC__in IMathInputControl * This,
            /* [out] */ __RPC__out LONG *Left,
            /* [out] */ __RPC__out LONG *Top,
            /* [out] */ __RPC__out LONG *Right,
            /* [out] */ __RPC__out LONG *Bottom);
        
        DECLSPEC_XFGVIRT(IMathInputControl, SetPosition)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *SetPosition )( 
            __RPC__in IMathInputControl * This,
            /* [in] */ LONG Left,
            /* [in] */ LONG Top,
            /* [in] */ LONG Right,
            /* [in] */ LONG Bottom);
        
        DECLSPEC_XFGVIRT(IMathInputControl, Clear)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Clear )( 
            __RPC__in IMathInputControl * This);
        
        DECLSPEC_XFGVIRT(IMathInputControl, SetCustomPaint)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *SetCustomPaint )( 
            __RPC__in IMathInputControl * This,
            /* [in] */ LONG Element,
            /* [in] */ VARIANT_BOOL Paint);
        
        DECLSPEC_XFGVIRT(IMathInputControl, SetCaptionText)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *SetCaptionText )( 
            __RPC__in IMathInputControl * This,
            /* [in] */ __RPC__in BSTR CaptionText);
        
        DECLSPEC_XFGVIRT(IMathInputControl, LoadInk)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *LoadInk )( 
            __RPC__in IMathInputControl * This,
            /* [in] */ __RPC__in_opt IInkDisp *Ink);
        
        DECLSPEC_XFGVIRT(IMathInputControl, SetOwnerWindow)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *SetOwnerWindow )( 
            __RPC__in IMathInputControl * This,
            /* [in] */ LONG_PTR OwnerWindow);
        
        DECLSPEC_XFGVIRT(IMathInputControl, EnableExtendedButtons)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *EnableExtendedButtons )( 
            __RPC__in IMathInputControl * This,
            /* [in] */ VARIANT_BOOL Extended);
        
        DECLSPEC_XFGVIRT(IMathInputControl, GetPreviewHeight)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetPreviewHeight )( 
            __RPC__in IMathInputControl * This,
            /* [out] */ __RPC__out LONG *Height);
        
        DECLSPEC_XFGVIRT(IMathInputControl, SetPreviewHeight)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *SetPreviewHeight )( 
            __RPC__in IMathInputControl * This,
            /* [in] */ LONG Height);
        
        DECLSPEC_XFGVIRT(IMathInputControl, EnableAutoGrow)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *EnableAutoGrow )( 
            __RPC__in IMathInputControl * This,
            /* [in] */ VARIANT_BOOL AutoGrow);
        
        DECLSPEC_XFGVIRT(IMathInputControl, AddFunctionName)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *AddFunctionName )( 
            __RPC__in IMathInputControl * This,
            /* [in] */ __RPC__in BSTR FunctionName);
        
        DECLSPEC_XFGVIRT(IMathInputControl, RemoveFunctionName)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *RemoveFunctionName )( 
            __RPC__in IMathInputControl * This,
            /* [in] */ __RPC__in BSTR FunctionName);
        
        DECLSPEC_XFGVIRT(IMathInputControl, GetHoverIcon)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetHoverIcon )( 
            __RPC__in IMathInputControl * This,
            /* [out] */ __RPC__deref_out_opt IPictureDisp **HoverImage);
        
        END_INTERFACE
    } IMathInputControlVtbl;

    interface IMathInputControl
    {
        CONST_VTBL struct IMathInputControlVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMathInputControl_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMathInputControl_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMathInputControl_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMathInputControl_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IMathInputControl_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IMathInputControl_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IMathInputControl_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IMathInputControl_Show(This)	\
    ( (This)->lpVtbl -> Show(This) ) 

#define IMathInputControl_Hide(This)	\
    ( (This)->lpVtbl -> Hide(This) ) 

#define IMathInputControl_IsVisible(This,pvbShown)	\
    ( (This)->lpVtbl -> IsVisible(This,pvbShown) ) 

#define IMathInputControl_GetPosition(This,Left,Top,Right,Bottom)	\
    ( (This)->lpVtbl -> GetPosition(This,Left,Top,Right,Bottom) ) 

#define IMathInputControl_SetPosition(This,Left,Top,Right,Bottom)	\
    ( (This)->lpVtbl -> SetPosition(This,Left,Top,Right,Bottom) ) 

#define IMathInputControl_Clear(This)	\
    ( (This)->lpVtbl -> Clear(This) ) 

#define IMathInputControl_SetCustomPaint(This,Element,Paint)	\
    ( (This)->lpVtbl -> SetCustomPaint(This,Element,Paint) ) 

#define IMathInputControl_SetCaptionText(This,CaptionText)	\
    ( (This)->lpVtbl -> SetCaptionText(This,CaptionText) ) 

#define IMathInputControl_LoadInk(This,Ink)	\
    ( (This)->lpVtbl -> LoadInk(This,Ink) ) 

#define IMathInputControl_SetOwnerWindow(This,OwnerWindow)	\
    ( (This)->lpVtbl -> SetOwnerWindow(This,OwnerWindow) ) 

#define IMathInputControl_EnableExtendedButtons(This,Extended)	\
    ( (This)->lpVtbl -> EnableExtendedButtons(This,Extended) ) 

#define IMathInputControl_GetPreviewHeight(This,Height)	\
    ( (This)->lpVtbl -> GetPreviewHeight(This,Height) ) 

#define IMathInputControl_SetPreviewHeight(This,Height)	\
    ( (This)->lpVtbl -> SetPreviewHeight(This,Height) ) 

#define IMathInputControl_EnableAutoGrow(This,AutoGrow)	\
    ( (This)->lpVtbl -> EnableAutoGrow(This,AutoGrow) ) 

#define IMathInputControl_AddFunctionName(This,FunctionName)	\
    ( (This)->lpVtbl -> AddFunctionName(This,FunctionName) ) 

#define IMathInputControl_RemoveFunctionName(This,FunctionName)	\
    ( (This)->lpVtbl -> RemoveFunctionName(This,FunctionName) ) 

#define IMathInputControl_GetHoverIcon(This,HoverImage)	\
    ( (This)->lpVtbl -> GetHoverIcon(This,HoverImage) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMathInputControl_INTERFACE_DEFINED__ */



#ifndef __micautLib_LIBRARY_DEFINED__
#define __micautLib_LIBRARY_DEFINED__

/* library micautLib */
/* [helpstring][version][uuid] */ 


EXTERN_C const IID LIBID_micautLib;

#ifndef ___IMathInputControlEvents_DISPINTERFACE_DEFINED__
#define ___IMathInputControlEvents_DISPINTERFACE_DEFINED__

/* dispinterface _IMathInputControlEvents */
/* [helpstring][uuid] */ 


EXTERN_C const IID DIID__IMathInputControlEvents;

#if defined(__cplusplus) && !defined(CINTERFACE)

    MIDL_INTERFACE("683336B5-A47D-4358-96F9-875A472AE70A")
    _IMathInputControlEvents : public IDispatch
    {
    };
    
#else 	/* C style interface */

    typedef struct _IMathInputControlEventsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in _IMathInputControlEvents * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in _IMathInputControlEvents * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in _IMathInputControlEvents * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in _IMathInputControlEvents * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in _IMathInputControlEvents * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in _IMathInputControlEvents * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            _IMathInputControlEvents * This,
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
    } _IMathInputControlEventsVtbl;

    interface _IMathInputControlEvents
    {
        CONST_VTBL struct _IMathInputControlEventsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define _IMathInputControlEvents_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define _IMathInputControlEvents_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define _IMathInputControlEvents_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define _IMathInputControlEvents_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define _IMathInputControlEvents_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define _IMathInputControlEvents_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define _IMathInputControlEvents_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */


#endif 	/* ___IMathInputControlEvents_DISPINTERFACE_DEFINED__ */


EXTERN_C const CLSID CLSID_MathInputControl;

#ifdef __cplusplus

class DECLSPEC_UUID("C561816C-14D8-4090-830C-98D994B21C7B")
MathInputControl;
#endif
#endif /* __micautLib_LIBRARY_DEFINED__ */

/* interface __MIDL_itf_micaut_0000_0002 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_micaut_0000_0002_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_micaut_0000_0002_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

unsigned long             __RPC_USER  BSTR_UserSize(     __RPC__in unsigned long *, unsigned long            , __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserMarshal(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out BSTR * ); 
void                      __RPC_USER  BSTR_UserFree(     __RPC__in unsigned long *, __RPC__in BSTR * ); 

unsigned long             __RPC_USER  BSTR_UserSize64(     __RPC__in unsigned long *, unsigned long            , __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserMarshal64(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out BSTR * ); 
void                      __RPC_USER  BSTR_UserFree64(     __RPC__in unsigned long *, __RPC__in BSTR * ); 

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


