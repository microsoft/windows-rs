

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


#ifndef __msinkaut15_h__
#define __msinkaut15_h__

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

#ifndef __IInkDivider_FWD_DEFINED__
#define __IInkDivider_FWD_DEFINED__
typedef interface IInkDivider IInkDivider;

#endif 	/* __IInkDivider_FWD_DEFINED__ */


#ifndef __IInkDivisionResult_FWD_DEFINED__
#define __IInkDivisionResult_FWD_DEFINED__
typedef interface IInkDivisionResult IInkDivisionResult;

#endif 	/* __IInkDivisionResult_FWD_DEFINED__ */


#ifndef __IInkDivisionUnit_FWD_DEFINED__
#define __IInkDivisionUnit_FWD_DEFINED__
typedef interface IInkDivisionUnit IInkDivisionUnit;

#endif 	/* __IInkDivisionUnit_FWD_DEFINED__ */


#ifndef __IInkDivisionUnits_FWD_DEFINED__
#define __IInkDivisionUnits_FWD_DEFINED__
typedef interface IInkDivisionUnits IInkDivisionUnits;

#endif 	/* __IInkDivisionUnits_FWD_DEFINED__ */


#ifndef __InkDivider_FWD_DEFINED__
#define __InkDivider_FWD_DEFINED__

#ifdef __cplusplus
typedef class InkDivider InkDivider;
#else
typedef struct InkDivider InkDivider;
#endif /* __cplusplus */

#endif 	/* __InkDivider_FWD_DEFINED__ */


/* header files for imported files */
#include "oaidl.h"
#include "ocidl.h"
#include "msinkaut.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_msinkaut15_0000_0000 */
/* [local] */ 

#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
#include <TPCError.h> // for Tablet PC Error codes




extern RPC_IF_HANDLE __MIDL_itf_msinkaut15_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_msinkaut15_0000_0000_v0_0_s_ifspec;


#ifndef __MSINKDIVLib_LIBRARY_DEFINED__
#define __MSINKDIVLib_LIBRARY_DEFINED__

/* library MSINKDIVLib */
/* [helpcontext][helpstring][version][uuid] */ 

typedef /* [helpcontext][helpstring] */ 
enum InkDivisionType
    {
        IDT_Segment	= 0,
        IDT_Line	= 1,
        IDT_Paragraph	= 2,
        IDT_Drawing	= 3
    } 	InkDivisionType;

typedef /* [hidden] */ 
enum DISPID_InkDivider
    {
        DISPID_IInkDivider_Strokes	= 1,
        DISPID_IInkDivider_RecognizerContext	= ( DISPID_IInkDivider_Strokes + 1 ) ,
        DISPID_IInkDivider_LineHeight	= ( DISPID_IInkDivider_RecognizerContext + 1 ) ,
        DISPID_IInkDivider_Divide	= ( DISPID_IInkDivider_LineHeight + 1 ) 
    } 	DISPID_InkDivider;

typedef /* [hidden] */ 
enum DISPID_InkDivisionResult
    {
        DISPID_IInkDivisionResult_Strokes	= 1,
        DISPID_IInkDivisionResult_ResultByType	= ( DISPID_IInkDivisionResult_Strokes + 1 ) 
    } 	DISPID_InkDivisionResult;

typedef /* [hidden] */ 
enum DISPID_InkDivisionUnit
    {
        DISPID_IInkDivisionUnit_Strokes	= 1,
        DISPID_IInkDivisionUnit_DivisionType	= ( DISPID_IInkDivisionUnit_Strokes + 1 ) ,
        DISPID_IInkDivisionUnit_RecognizedString	= ( DISPID_IInkDivisionUnit_DivisionType + 1 ) ,
        DISPID_IInkDivisionUnit_RotationTransform	= ( DISPID_IInkDivisionUnit_RecognizedString + 1 ) 
    } 	DISPID_InkDivisionUnit;

typedef /* [hidden] */ 
enum DISPID_InkDivisionUnits
    {
        DISPID_IInkDivisionUnits_NewEnum	= DISPID_NEWENUM,
        DISPID_IInkDivisionUnits_Item	= DISPID_VALUE,
        DISPID_IInkDivisionUnits_Count	= 1
    } 	DISPID_InkDivisionUnits;


EXTERN_C const IID LIBID_MSINKDIVLib;

#ifndef __IInkDivider_INTERFACE_DEFINED__
#define __IInkDivider_INTERFACE_DEFINED__

/* interface IInkDivider */
/* [unique][helpcontext][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IInkDivider;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("5DE00405-F9A4-4651-B0C5-C317DEFD58B9")
    IInkDivider : public IDispatch
    {
    public:
        virtual /* [helpcontext][helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Strokes( 
            /* [retval][out] */ __RPC__deref_out_opt IInkStrokes **Strokes) = 0;
        
        virtual /* [helpcontext][helpstring][id][propputref] */ HRESULT STDMETHODCALLTYPE putref_Strokes( 
            /* [in] */ __RPC__in_opt IInkStrokes *Strokes) = 0;
        
        virtual /* [helpcontext][helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_RecognizerContext( 
            /* [retval][out] */ __RPC__deref_out_opt IInkRecognizerContext **RecognizerContext) = 0;
        
        virtual /* [helpcontext][helpstring][id][propputref] */ HRESULT STDMETHODCALLTYPE putref_RecognizerContext( 
            /* [in] */ __RPC__in_opt IInkRecognizerContext *RecognizerContext) = 0;
        
        virtual /* [helpcontext][helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_LineHeight( 
            /* [retval][out] */ __RPC__out LONG *LineHeight) = 0;
        
        virtual /* [helpcontext][helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_LineHeight( 
            /* [in] */ LONG LineHeight) = 0;
        
        virtual /* [helpcontext][helpstring][id] */ HRESULT STDMETHODCALLTYPE Divide( 
            /* [retval][out] */ __RPC__deref_out_opt IInkDivisionResult **InkDivisionResult) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IInkDividerVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IInkDivider * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IInkDivider * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IInkDivider * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IInkDivider * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IInkDivider * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IInkDivider * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IInkDivider * This,
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
        
        DECLSPEC_XFGVIRT(IInkDivider, get_Strokes)
        /* [helpcontext][helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Strokes )( 
            __RPC__in IInkDivider * This,
            /* [retval][out] */ __RPC__deref_out_opt IInkStrokes **Strokes);
        
        DECLSPEC_XFGVIRT(IInkDivider, putref_Strokes)
        /* [helpcontext][helpstring][id][propputref] */ HRESULT ( STDMETHODCALLTYPE *putref_Strokes )( 
            __RPC__in IInkDivider * This,
            /* [in] */ __RPC__in_opt IInkStrokes *Strokes);
        
        DECLSPEC_XFGVIRT(IInkDivider, get_RecognizerContext)
        /* [helpcontext][helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_RecognizerContext )( 
            __RPC__in IInkDivider * This,
            /* [retval][out] */ __RPC__deref_out_opt IInkRecognizerContext **RecognizerContext);
        
        DECLSPEC_XFGVIRT(IInkDivider, putref_RecognizerContext)
        /* [helpcontext][helpstring][id][propputref] */ HRESULT ( STDMETHODCALLTYPE *putref_RecognizerContext )( 
            __RPC__in IInkDivider * This,
            /* [in] */ __RPC__in_opt IInkRecognizerContext *RecognizerContext);
        
        DECLSPEC_XFGVIRT(IInkDivider, get_LineHeight)
        /* [helpcontext][helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_LineHeight )( 
            __RPC__in IInkDivider * This,
            /* [retval][out] */ __RPC__out LONG *LineHeight);
        
        DECLSPEC_XFGVIRT(IInkDivider, put_LineHeight)
        /* [helpcontext][helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_LineHeight )( 
            __RPC__in IInkDivider * This,
            /* [in] */ LONG LineHeight);
        
        DECLSPEC_XFGVIRT(IInkDivider, Divide)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Divide )( 
            __RPC__in IInkDivider * This,
            /* [retval][out] */ __RPC__deref_out_opt IInkDivisionResult **InkDivisionResult);
        
        END_INTERFACE
    } IInkDividerVtbl;

    interface IInkDivider
    {
        CONST_VTBL struct IInkDividerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IInkDivider_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IInkDivider_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IInkDivider_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IInkDivider_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IInkDivider_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IInkDivider_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IInkDivider_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IInkDivider_get_Strokes(This,Strokes)	\
    ( (This)->lpVtbl -> get_Strokes(This,Strokes) ) 

#define IInkDivider_putref_Strokes(This,Strokes)	\
    ( (This)->lpVtbl -> putref_Strokes(This,Strokes) ) 

#define IInkDivider_get_RecognizerContext(This,RecognizerContext)	\
    ( (This)->lpVtbl -> get_RecognizerContext(This,RecognizerContext) ) 

#define IInkDivider_putref_RecognizerContext(This,RecognizerContext)	\
    ( (This)->lpVtbl -> putref_RecognizerContext(This,RecognizerContext) ) 

#define IInkDivider_get_LineHeight(This,LineHeight)	\
    ( (This)->lpVtbl -> get_LineHeight(This,LineHeight) ) 

#define IInkDivider_put_LineHeight(This,LineHeight)	\
    ( (This)->lpVtbl -> put_LineHeight(This,LineHeight) ) 

#define IInkDivider_Divide(This,InkDivisionResult)	\
    ( (This)->lpVtbl -> Divide(This,InkDivisionResult) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IInkDivider_INTERFACE_DEFINED__ */


#ifndef __IInkDivisionResult_INTERFACE_DEFINED__
#define __IInkDivisionResult_INTERFACE_DEFINED__

/* interface IInkDivisionResult */
/* [unique][helpcontext][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IInkDivisionResult;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("2DBEC0A7-74C7-4B38-81EB-AA8EF0C24900")
    IInkDivisionResult : public IDispatch
    {
    public:
        virtual /* [helpcontext][helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Strokes( 
            /* [retval][out] */ __RPC__deref_out_opt IInkStrokes **Strokes) = 0;
        
        virtual /* [helpcontext][helpstring][id] */ HRESULT STDMETHODCALLTYPE ResultByType( 
            /* [in] */ InkDivisionType divisionType,
            /* [retval][out] */ __RPC__deref_out_opt IInkDivisionUnits **InkDivisionUnits) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IInkDivisionResultVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IInkDivisionResult * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IInkDivisionResult * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IInkDivisionResult * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IInkDivisionResult * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IInkDivisionResult * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IInkDivisionResult * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IInkDivisionResult * This,
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
        
        DECLSPEC_XFGVIRT(IInkDivisionResult, get_Strokes)
        /* [helpcontext][helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Strokes )( 
            __RPC__in IInkDivisionResult * This,
            /* [retval][out] */ __RPC__deref_out_opt IInkStrokes **Strokes);
        
        DECLSPEC_XFGVIRT(IInkDivisionResult, ResultByType)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *ResultByType )( 
            __RPC__in IInkDivisionResult * This,
            /* [in] */ InkDivisionType divisionType,
            /* [retval][out] */ __RPC__deref_out_opt IInkDivisionUnits **InkDivisionUnits);
        
        END_INTERFACE
    } IInkDivisionResultVtbl;

    interface IInkDivisionResult
    {
        CONST_VTBL struct IInkDivisionResultVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IInkDivisionResult_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IInkDivisionResult_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IInkDivisionResult_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IInkDivisionResult_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IInkDivisionResult_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IInkDivisionResult_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IInkDivisionResult_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IInkDivisionResult_get_Strokes(This,Strokes)	\
    ( (This)->lpVtbl -> get_Strokes(This,Strokes) ) 

#define IInkDivisionResult_ResultByType(This,divisionType,InkDivisionUnits)	\
    ( (This)->lpVtbl -> ResultByType(This,divisionType,InkDivisionUnits) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IInkDivisionResult_INTERFACE_DEFINED__ */


#ifndef __IInkDivisionUnit_INTERFACE_DEFINED__
#define __IInkDivisionUnit_INTERFACE_DEFINED__

/* interface IInkDivisionUnit */
/* [unique][helpcontext][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IInkDivisionUnit;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("85AEE342-48B0-4244-9DD5-1ED435410FAB")
    IInkDivisionUnit : public IDispatch
    {
    public:
        virtual /* [helpcontext][helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Strokes( 
            /* [retval][out] */ __RPC__deref_out_opt IInkStrokes **Strokes) = 0;
        
        virtual /* [helpcontext][helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_DivisionType( 
            /* [retval][out] */ __RPC__out InkDivisionType *divisionType) = 0;
        
        virtual /* [helpcontext][helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_RecognizedString( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *RecoString) = 0;
        
        virtual /* [helpcontext][helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_RotationTransform( 
            /* [retval][out] */ __RPC__deref_out_opt IInkTransform **RotationTransform) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IInkDivisionUnitVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IInkDivisionUnit * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IInkDivisionUnit * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IInkDivisionUnit * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IInkDivisionUnit * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IInkDivisionUnit * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IInkDivisionUnit * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IInkDivisionUnit * This,
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
        
        DECLSPEC_XFGVIRT(IInkDivisionUnit, get_Strokes)
        /* [helpcontext][helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Strokes )( 
            __RPC__in IInkDivisionUnit * This,
            /* [retval][out] */ __RPC__deref_out_opt IInkStrokes **Strokes);
        
        DECLSPEC_XFGVIRT(IInkDivisionUnit, get_DivisionType)
        /* [helpcontext][helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_DivisionType )( 
            __RPC__in IInkDivisionUnit * This,
            /* [retval][out] */ __RPC__out InkDivisionType *divisionType);
        
        DECLSPEC_XFGVIRT(IInkDivisionUnit, get_RecognizedString)
        /* [helpcontext][helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_RecognizedString )( 
            __RPC__in IInkDivisionUnit * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *RecoString);
        
        DECLSPEC_XFGVIRT(IInkDivisionUnit, get_RotationTransform)
        /* [helpcontext][helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_RotationTransform )( 
            __RPC__in IInkDivisionUnit * This,
            /* [retval][out] */ __RPC__deref_out_opt IInkTransform **RotationTransform);
        
        END_INTERFACE
    } IInkDivisionUnitVtbl;

    interface IInkDivisionUnit
    {
        CONST_VTBL struct IInkDivisionUnitVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IInkDivisionUnit_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IInkDivisionUnit_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IInkDivisionUnit_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IInkDivisionUnit_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IInkDivisionUnit_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IInkDivisionUnit_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IInkDivisionUnit_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IInkDivisionUnit_get_Strokes(This,Strokes)	\
    ( (This)->lpVtbl -> get_Strokes(This,Strokes) ) 

#define IInkDivisionUnit_get_DivisionType(This,divisionType)	\
    ( (This)->lpVtbl -> get_DivisionType(This,divisionType) ) 

#define IInkDivisionUnit_get_RecognizedString(This,RecoString)	\
    ( (This)->lpVtbl -> get_RecognizedString(This,RecoString) ) 

#define IInkDivisionUnit_get_RotationTransform(This,RotationTransform)	\
    ( (This)->lpVtbl -> get_RotationTransform(This,RotationTransform) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IInkDivisionUnit_INTERFACE_DEFINED__ */


#ifndef __IInkDivisionUnits_INTERFACE_DEFINED__
#define __IInkDivisionUnits_INTERFACE_DEFINED__

/* interface IInkDivisionUnits */
/* [unique][helpcontext][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IInkDivisionUnits;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("1BB5DDC2-31CC-4135-AB82-2C66C9F00C41")
    IInkDivisionUnits : public IDispatch
    {
    public:
        virtual /* [helpcontext][helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Count( 
            /* [retval][out] */ __RPC__out long *Count) = 0;
        
        virtual /* [helpcontext][helpstring][restricted][id][propget] */ HRESULT STDMETHODCALLTYPE get__NewEnum( 
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **_NewEnum) = 0;
        
        virtual /* [helpcontext][helpstring][id] */ HRESULT STDMETHODCALLTYPE Item( 
            /* [in] */ long Index,
            /* [retval][out] */ __RPC__deref_out_opt IInkDivisionUnit **InkDivisionUnit) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IInkDivisionUnitsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IInkDivisionUnits * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IInkDivisionUnits * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IInkDivisionUnits * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IInkDivisionUnits * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IInkDivisionUnits * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IInkDivisionUnits * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IInkDivisionUnits * This,
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
        
        DECLSPEC_XFGVIRT(IInkDivisionUnits, get_Count)
        /* [helpcontext][helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Count )( 
            __RPC__in IInkDivisionUnits * This,
            /* [retval][out] */ __RPC__out long *Count);
        
        DECLSPEC_XFGVIRT(IInkDivisionUnits, get__NewEnum)
        /* [helpcontext][helpstring][restricted][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get__NewEnum )( 
            __RPC__in IInkDivisionUnits * This,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **_NewEnum);
        
        DECLSPEC_XFGVIRT(IInkDivisionUnits, Item)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Item )( 
            __RPC__in IInkDivisionUnits * This,
            /* [in] */ long Index,
            /* [retval][out] */ __RPC__deref_out_opt IInkDivisionUnit **InkDivisionUnit);
        
        END_INTERFACE
    } IInkDivisionUnitsVtbl;

    interface IInkDivisionUnits
    {
        CONST_VTBL struct IInkDivisionUnitsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IInkDivisionUnits_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IInkDivisionUnits_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IInkDivisionUnits_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IInkDivisionUnits_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IInkDivisionUnits_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IInkDivisionUnits_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IInkDivisionUnits_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IInkDivisionUnits_get_Count(This,Count)	\
    ( (This)->lpVtbl -> get_Count(This,Count) ) 

#define IInkDivisionUnits_get__NewEnum(This,_NewEnum)	\
    ( (This)->lpVtbl -> get__NewEnum(This,_NewEnum) ) 

#define IInkDivisionUnits_Item(This,Index,InkDivisionUnit)	\
    ( (This)->lpVtbl -> Item(This,Index,InkDivisionUnit) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IInkDivisionUnits_INTERFACE_DEFINED__ */


EXTERN_C const CLSID CLSID_InkDivider;

#ifdef __cplusplus

class DECLSPEC_UUID("8854F6A0-4683-4AE7-9191-752FE64612C3")
InkDivider;
#endif
#endif /* __MSINKDIVLib_LIBRARY_DEFINED__ */

/* interface __MIDL_itf_msinkaut15_0000_0001 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_msinkaut15_0000_0001_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_msinkaut15_0000_0001_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


