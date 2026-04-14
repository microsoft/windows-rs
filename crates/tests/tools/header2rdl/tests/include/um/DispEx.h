

/* this ALWAYS GENERATED file contains the definitions for the interfaces */


 /* File created by MIDL compiler version 8.01.0628 */
/* @@MIDL_FILE_HEADING(  ) */



/* verify that the <rpcndr.h> version is high enough to compile this file*/
#ifndef __REQUIRED_RPCNDR_H_VERSION__
#define __REQUIRED_RPCNDR_H_VERSION__ 500
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

#ifndef __dispex_h__
#define __dispex_h__

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

#ifndef __IDispatchEx_FWD_DEFINED__
#define __IDispatchEx_FWD_DEFINED__
typedef interface IDispatchEx IDispatchEx;

#endif 	/* __IDispatchEx_FWD_DEFINED__ */


#ifndef __IDispError_FWD_DEFINED__
#define __IDispError_FWD_DEFINED__
typedef interface IDispError IDispError;

#endif 	/* __IDispError_FWD_DEFINED__ */


#ifndef __IVariantChangeType_FWD_DEFINED__
#define __IVariantChangeType_FWD_DEFINED__
typedef interface IVariantChangeType IVariantChangeType;

#endif 	/* __IVariantChangeType_FWD_DEFINED__ */


#ifndef __IObjectIdentity_FWD_DEFINED__
#define __IObjectIdentity_FWD_DEFINED__
typedef interface IObjectIdentity IObjectIdentity;

#endif 	/* __IObjectIdentity_FWD_DEFINED__ */


#ifndef __ICanHandleException_FWD_DEFINED__
#define __ICanHandleException_FWD_DEFINED__
typedef interface ICanHandleException ICanHandleException;

#endif 	/* __ICanHandleException_FWD_DEFINED__ */


#ifndef __IProvideRuntimeContext_FWD_DEFINED__
#define __IProvideRuntimeContext_FWD_DEFINED__
typedef interface IProvideRuntimeContext IProvideRuntimeContext;

#endif 	/* __IProvideRuntimeContext_FWD_DEFINED__ */


/* header files for imported files */
#include "ocidl.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_dispex_0000_0000 */
/* [local] */ 

//=--------------------------------------------------------------------------=
// DispEx.h
//=--------------------------------------------------------------------------=
// (C) Copyright 1997 Microsoft Corporation.  All Rights Reserved.
//
// THIS CODE AND INFORMATION IS PROVIDED "AS IS" WITHOUT WARRANTY OF
// ANY KIND, EITHER EXPRESSED OR IMPLIED, INCLUDING BUT NOT LIMITED TO
// THE IMPLIED WARRANTIES OF MERCHANTABILITY AND/OR FITNESS FOR A
// PARTICULAR PURPOSE.
//=--------------------------------------------------------------------------=

#pragma comment(lib,"uuid.lib")

//---------------------------------------------------------------------------=
// IDispatchEx Interfaces.
//

#ifndef DISPEX_H_
#define DISPEX_H_

#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)




#include "servprov.h"

#ifndef _NO_DISPATCHEX_GUIDS

// {A6EF9860-C720-11d0-9337-00A0C90DCAA9}
DEFINE_GUID(IID_IDispatchEx, 0xa6ef9860, 0xc720, 0x11d0, 0x93, 0x37, 0x0, 0xa0, 0xc9, 0xd, 0xca, 0xa9);

// {A6EF9861-C720-11d0-9337-00A0C90DCAA9}
DEFINE_GUID(IID_IDispError, 0xa6ef9861, 0xc720, 0x11d0, 0x93, 0x37, 0x0, 0xa0, 0xc9, 0xd, 0xca, 0xa9);

// {A6EF9862-C720-11d0-9337-00A0C90DCAA9}
DEFINE_GUID(IID_IVariantChangeType, 0xa6ef9862, 0xc720, 0x11d0, 0x93, 0x37, 0x0, 0xa0, 0xc9, 0xd, 0xca, 0xa9);

// {1F101481-BCCD-11d0-9336-00A0C90DCAA9}
DEFINE_GUID(SID_VariantConversion, 0x1f101481, 0xbccd, 0x11d0, 0x93, 0x36, 0x0, 0xa0, 0xc9, 0xd, 0xca, 0xa9);

// {4717CC40-BCB9-11d0-9336-00A0C90DCAA9}
DEFINE_GUID(SID_GetCaller, 0x4717cc40, 0xbcb9, 0x11d0, 0x93, 0x36, 0x0, 0xa0, 0xc9, 0xd, 0xca, 0xa9);

// {74A5040C-DD0C-48f0-AC85-194C3259180A}
DEFINE_GUID(SID_ProvideRuntimeContext, 0x74a5040c, 0xdd0c, 0x48f0, 0xac, 0x85, 0x19, 0x4c, 0x32, 0x59, 0x18, 0xa);

// {10E2414A-EC59-49d2-BC51-5ADD2C36FEBC}
DEFINE_GUID(IID_IProvideRuntimeContext, 0x10e2414a, 0xec59, 0x49d2, 0xbc, 0x51, 0x5a, 0xdd, 0x2c, 0x36, 0xfe, 0xbc);

// {CA04B7E6-0D21-11d1-8CC5-00C04FC2B085}
DEFINE_GUID(IID_IObjectIdentity, 0xca04b7e6, 0xd21, 0x11d1, 0x8c, 0xc5, 0x0, 0xc0, 0x4f, 0xc2, 0xb0, 0x85);

// {c5598e60-b307-11d1-b27d-006008c3fbfb}
DEFINE_GUID(IID_ICanHandleException, 0xc5598e60, 0xb307, 0x11d1, 0xb2, 0x7d, 0x0, 0x60, 0x08, 0xc3, 0xfb, 0xfb);

#define SID_GetScriptSite IID_IActiveScriptSite

#endif // _NO_DISPATCHEX_GUIDS


#ifndef _NO_DISPATCHEX_CONSTS

// Input flags for GetDispID
#define fdexNameCaseSensitive       0x00000001L
#define fdexNameEnsure              0x00000002L
#define fdexNameImplicit            0x00000004L
#define fdexNameCaseInsensitive     0x00000008L
#define fdexNameInternal            0x00000010L
#define fdexNameNoDynamicProperties 0x00000020L

// Output flags for GetMemberProperties
#define fdexPropCanGet              0x00000001L
#define fdexPropCannotGet           0x00000002L
#define fdexPropCanPut              0x00000004L
#define fdexPropCannotPut           0x00000008L
#define fdexPropCanPutRef           0x00000010L
#define fdexPropCannotPutRef        0x00000020L
#define fdexPropNoSideEffects       0x00000040L
#define fdexPropDynamicType         0x00000080L
#define fdexPropCanCall             0x00000100L
#define fdexPropCannotCall          0x00000200L
#define fdexPropCanConstruct        0x00000400L
#define fdexPropCannotConstruct     0x00000800L
#define fdexPropCanSourceEvents     0x00001000L
#define fdexPropCannotSourceEvents  0x00002000L

#define grfdexPropCanAll \
       (fdexPropCanGet | fdexPropCanPut | fdexPropCanPutRef | \
        fdexPropCanCall | fdexPropCanConstruct | fdexPropCanSourceEvents)
#define grfdexPropCannotAll \
       (fdexPropCannotGet | fdexPropCannotPut | fdexPropCannotPutRef | \
        fdexPropCannotCall | fdexPropCannotConstruct | fdexPropCannotSourceEvents)
#define grfdexPropExtraAll \
       (fdexPropNoSideEffects | fdexPropDynamicType)
#define grfdexPropAll \
       (grfdexPropCanAll | grfdexPropCannotAll | grfdexPropExtraAll)

// Input flags for GetNextDispID
#define fdexEnumDefault             0x00000001L
#define fdexEnumAll                 0x00000002L

// Additional flags for Invoke - when object member is
// used as a constructor.
#define DISPATCH_CONSTRUCT 0x4000

// Standard DISPIDs
#define DISPID_THIS (-613)
#define DISPID_STARTENUM DISPID_UNKNOWN

#endif //_NO_DISPATCHEX_CONSTS



extern RPC_IF_HANDLE __MIDL_itf_dispex_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_dispex_0000_0000_v0_0_s_ifspec;

#ifndef __IDispatchEx_INTERFACE_DEFINED__
#define __IDispatchEx_INTERFACE_DEFINED__

/* interface IDispatchEx */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IDispatchEx;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("A6EF9860-C720-11d0-9337-00A0C90DCAA9")
    IDispatchEx : public IDispatch
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetDispID( 
            /* [in] */ __RPC__in BSTR bstrName,
            /* [in] */ DWORD grfdex,
            /* [out] */ __RPC__out DISPID *pid) = 0;
        
        virtual /* [local] */ HRESULT STDMETHODCALLTYPE InvokeEx( 
            /* [annotation][in] */ 
            _In_  DISPID id,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][in] */ 
            _In_  DISPPARAMS *pdp,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pvarRes,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pei,
            /* [annotation][unique][in] */ 
            _In_opt_  IServiceProvider *pspCaller) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE DeleteMemberByName( 
            /* [in] */ __RPC__in BSTR bstrName,
            /* [in] */ DWORD grfdex) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE DeleteMemberByDispID( 
            /* [in] */ DISPID id) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetMemberProperties( 
            /* [in] */ DISPID id,
            /* [in] */ DWORD grfdexFetch,
            /* [out] */ __RPC__out DWORD *pgrfdex) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetMemberName( 
            /* [in] */ DISPID id,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrName) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetNextDispID( 
            /* [in] */ DWORD grfdex,
            /* [in] */ DISPID id,
            /* [out] */ __RPC__out DISPID *pid) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetNameSpaceParent( 
            /* [out] */ __RPC__deref_out_opt IUnknown **ppunk) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDispatchExVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IDispatchEx * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IDispatchEx * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IDispatchEx * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IDispatchEx * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IDispatchEx * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IDispatchEx * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IDispatchEx * This,
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
        
        DECLSPEC_XFGVIRT(IDispatchEx, GetDispID)
        HRESULT ( STDMETHODCALLTYPE *GetDispID )( 
            __RPC__in IDispatchEx * This,
            /* [in] */ __RPC__in BSTR bstrName,
            /* [in] */ DWORD grfdex,
            /* [out] */ __RPC__out DISPID *pid);
        
        DECLSPEC_XFGVIRT(IDispatchEx, InvokeEx)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *InvokeEx )( 
            IDispatchEx * This,
            /* [annotation][in] */ 
            _In_  DISPID id,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][in] */ 
            _In_  DISPPARAMS *pdp,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pvarRes,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pei,
            /* [annotation][unique][in] */ 
            _In_opt_  IServiceProvider *pspCaller);
        
        DECLSPEC_XFGVIRT(IDispatchEx, DeleteMemberByName)
        HRESULT ( STDMETHODCALLTYPE *DeleteMemberByName )( 
            __RPC__in IDispatchEx * This,
            /* [in] */ __RPC__in BSTR bstrName,
            /* [in] */ DWORD grfdex);
        
        DECLSPEC_XFGVIRT(IDispatchEx, DeleteMemberByDispID)
        HRESULT ( STDMETHODCALLTYPE *DeleteMemberByDispID )( 
            __RPC__in IDispatchEx * This,
            /* [in] */ DISPID id);
        
        DECLSPEC_XFGVIRT(IDispatchEx, GetMemberProperties)
        HRESULT ( STDMETHODCALLTYPE *GetMemberProperties )( 
            __RPC__in IDispatchEx * This,
            /* [in] */ DISPID id,
            /* [in] */ DWORD grfdexFetch,
            /* [out] */ __RPC__out DWORD *pgrfdex);
        
        DECLSPEC_XFGVIRT(IDispatchEx, GetMemberName)
        HRESULT ( STDMETHODCALLTYPE *GetMemberName )( 
            __RPC__in IDispatchEx * This,
            /* [in] */ DISPID id,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrName);
        
        DECLSPEC_XFGVIRT(IDispatchEx, GetNextDispID)
        HRESULT ( STDMETHODCALLTYPE *GetNextDispID )( 
            __RPC__in IDispatchEx * This,
            /* [in] */ DWORD grfdex,
            /* [in] */ DISPID id,
            /* [out] */ __RPC__out DISPID *pid);
        
        DECLSPEC_XFGVIRT(IDispatchEx, GetNameSpaceParent)
        HRESULT ( STDMETHODCALLTYPE *GetNameSpaceParent )( 
            __RPC__in IDispatchEx * This,
            /* [out] */ __RPC__deref_out_opt IUnknown **ppunk);
        
        END_INTERFACE
    } IDispatchExVtbl;

    interface IDispatchEx
    {
        CONST_VTBL struct IDispatchExVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDispatchEx_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDispatchEx_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDispatchEx_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDispatchEx_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IDispatchEx_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IDispatchEx_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IDispatchEx_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IDispatchEx_GetDispID(This,bstrName,grfdex,pid)	\
    ( (This)->lpVtbl -> GetDispID(This,bstrName,grfdex,pid) ) 

#define IDispatchEx_InvokeEx(This,id,lcid,wFlags,pdp,pvarRes,pei,pspCaller)	\
    ( (This)->lpVtbl -> InvokeEx(This,id,lcid,wFlags,pdp,pvarRes,pei,pspCaller) ) 

#define IDispatchEx_DeleteMemberByName(This,bstrName,grfdex)	\
    ( (This)->lpVtbl -> DeleteMemberByName(This,bstrName,grfdex) ) 

#define IDispatchEx_DeleteMemberByDispID(This,id)	\
    ( (This)->lpVtbl -> DeleteMemberByDispID(This,id) ) 

#define IDispatchEx_GetMemberProperties(This,id,grfdexFetch,pgrfdex)	\
    ( (This)->lpVtbl -> GetMemberProperties(This,id,grfdexFetch,pgrfdex) ) 

#define IDispatchEx_GetMemberName(This,id,pbstrName)	\
    ( (This)->lpVtbl -> GetMemberName(This,id,pbstrName) ) 

#define IDispatchEx_GetNextDispID(This,grfdex,id,pid)	\
    ( (This)->lpVtbl -> GetNextDispID(This,grfdex,id,pid) ) 

#define IDispatchEx_GetNameSpaceParent(This,ppunk)	\
    ( (This)->lpVtbl -> GetNameSpaceParent(This,ppunk) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */



/* [call_as] */ HRESULT STDMETHODCALLTYPE IDispatchEx_RemoteInvokeEx_Proxy( 
    __RPC__in IDispatchEx * This,
    /* [in] */ DISPID id,
    /* [in] */ LCID lcid,
    /* [in] */ DWORD dwFlags,
    /* [in] */ __RPC__in DISPPARAMS *pdp,
    /* [out] */ __RPC__out VARIANT *pvarRes,
    /* [out] */ __RPC__out EXCEPINFO *pei,
    /* [unique][in] */ __RPC__in_opt IServiceProvider *pspCaller,
    /* [in] */ UINT cvarRefArg,
    /* [size_is][in] */ __RPC__in_ecount_full(cvarRefArg) UINT *rgiRefArg,
    /* [size_is][out][in] */ __RPC__inout_ecount_full(cvarRefArg) VARIANT *rgvarRefArg);


void __RPC_STUB IDispatchEx_RemoteInvokeEx_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);



#endif 	/* __IDispatchEx_INTERFACE_DEFINED__ */


#ifndef __IDispError_INTERFACE_DEFINED__
#define __IDispError_INTERFACE_DEFINED__

/* interface IDispError */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IDispError;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("A6EF9861-C720-11d0-9337-00A0C90DCAA9")
    IDispError : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE QueryErrorInfo( 
            /* [in] */ GUID guidErrorType,
            /* [out] */ __RPC__deref_out_opt IDispError **ppde) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetNext( 
            /* [out] */ __RPC__deref_out_opt IDispError **ppde) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetHresult( 
            /* [out] */ __RPC__out HRESULT *phr) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetSource( 
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrSource) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetHelpInfo( 
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrFileName,
            /* [out] */ __RPC__out DWORD *pdwContext) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetDescription( 
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrDescription) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDispErrorVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IDispError * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IDispError * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IDispError * This);
        
        DECLSPEC_XFGVIRT(IDispError, QueryErrorInfo)
        HRESULT ( STDMETHODCALLTYPE *QueryErrorInfo )( 
            __RPC__in IDispError * This,
            /* [in] */ GUID guidErrorType,
            /* [out] */ __RPC__deref_out_opt IDispError **ppde);
        
        DECLSPEC_XFGVIRT(IDispError, GetNext)
        HRESULT ( STDMETHODCALLTYPE *GetNext )( 
            __RPC__in IDispError * This,
            /* [out] */ __RPC__deref_out_opt IDispError **ppde);
        
        DECLSPEC_XFGVIRT(IDispError, GetHresult)
        HRESULT ( STDMETHODCALLTYPE *GetHresult )( 
            __RPC__in IDispError * This,
            /* [out] */ __RPC__out HRESULT *phr);
        
        DECLSPEC_XFGVIRT(IDispError, GetSource)
        HRESULT ( STDMETHODCALLTYPE *GetSource )( 
            __RPC__in IDispError * This,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrSource);
        
        DECLSPEC_XFGVIRT(IDispError, GetHelpInfo)
        HRESULT ( STDMETHODCALLTYPE *GetHelpInfo )( 
            __RPC__in IDispError * This,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrFileName,
            /* [out] */ __RPC__out DWORD *pdwContext);
        
        DECLSPEC_XFGVIRT(IDispError, GetDescription)
        HRESULT ( STDMETHODCALLTYPE *GetDescription )( 
            __RPC__in IDispError * This,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrDescription);
        
        END_INTERFACE
    } IDispErrorVtbl;

    interface IDispError
    {
        CONST_VTBL struct IDispErrorVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDispError_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDispError_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDispError_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDispError_QueryErrorInfo(This,guidErrorType,ppde)	\
    ( (This)->lpVtbl -> QueryErrorInfo(This,guidErrorType,ppde) ) 

#define IDispError_GetNext(This,ppde)	\
    ( (This)->lpVtbl -> GetNext(This,ppde) ) 

#define IDispError_GetHresult(This,phr)	\
    ( (This)->lpVtbl -> GetHresult(This,phr) ) 

#define IDispError_GetSource(This,pbstrSource)	\
    ( (This)->lpVtbl -> GetSource(This,pbstrSource) ) 

#define IDispError_GetHelpInfo(This,pbstrFileName,pdwContext)	\
    ( (This)->lpVtbl -> GetHelpInfo(This,pbstrFileName,pdwContext) ) 

#define IDispError_GetDescription(This,pbstrDescription)	\
    ( (This)->lpVtbl -> GetDescription(This,pbstrDescription) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDispError_INTERFACE_DEFINED__ */


#ifndef __IVariantChangeType_INTERFACE_DEFINED__
#define __IVariantChangeType_INTERFACE_DEFINED__

/* interface IVariantChangeType */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IVariantChangeType;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("A6EF9862-C720-11d0-9337-00A0C90DCAA9")
    IVariantChangeType : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE ChangeType( 
            /* [unique][out][in] */ __RPC__inout_opt VARIANT *pvarDst,
            /* [unique][in] */ __RPC__in_opt VARIANT *pvarSrc,
            /* [in] */ LCID lcid,
            /* [in] */ VARTYPE vtNew) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IVariantChangeTypeVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IVariantChangeType * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IVariantChangeType * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IVariantChangeType * This);
        
        DECLSPEC_XFGVIRT(IVariantChangeType, ChangeType)
        HRESULT ( STDMETHODCALLTYPE *ChangeType )( 
            __RPC__in IVariantChangeType * This,
            /* [unique][out][in] */ __RPC__inout_opt VARIANT *pvarDst,
            /* [unique][in] */ __RPC__in_opt VARIANT *pvarSrc,
            /* [in] */ LCID lcid,
            /* [in] */ VARTYPE vtNew);
        
        END_INTERFACE
    } IVariantChangeTypeVtbl;

    interface IVariantChangeType
    {
        CONST_VTBL struct IVariantChangeTypeVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IVariantChangeType_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IVariantChangeType_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IVariantChangeType_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IVariantChangeType_ChangeType(This,pvarDst,pvarSrc,lcid,vtNew)	\
    ( (This)->lpVtbl -> ChangeType(This,pvarDst,pvarSrc,lcid,vtNew) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IVariantChangeType_INTERFACE_DEFINED__ */


#ifndef __IObjectIdentity_INTERFACE_DEFINED__
#define __IObjectIdentity_INTERFACE_DEFINED__

/* interface IObjectIdentity */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IObjectIdentity;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("CA04B7E6-0D21-11d1-8CC5-00C04FC2B085")
    IObjectIdentity : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE IsEqualObject( 
            /* [in] */ __RPC__in_opt IUnknown *punk) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IObjectIdentityVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IObjectIdentity * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IObjectIdentity * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IObjectIdentity * This);
        
        DECLSPEC_XFGVIRT(IObjectIdentity, IsEqualObject)
        HRESULT ( STDMETHODCALLTYPE *IsEqualObject )( 
            __RPC__in IObjectIdentity * This,
            /* [in] */ __RPC__in_opt IUnknown *punk);
        
        END_INTERFACE
    } IObjectIdentityVtbl;

    interface IObjectIdentity
    {
        CONST_VTBL struct IObjectIdentityVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IObjectIdentity_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IObjectIdentity_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IObjectIdentity_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IObjectIdentity_IsEqualObject(This,punk)	\
    ( (This)->lpVtbl -> IsEqualObject(This,punk) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IObjectIdentity_INTERFACE_DEFINED__ */


#ifndef __ICanHandleException_INTERFACE_DEFINED__
#define __ICanHandleException_INTERFACE_DEFINED__

/* interface ICanHandleException */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_ICanHandleException;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("c5598e60-b307-11d1-b27d-006008c3fbfb")
    ICanHandleException : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE CanHandleException( 
            /* [in] */ __RPC__in EXCEPINFO *pExcepInfo,
            /* [in] */ __RPC__in VARIANT *pvar) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ICanHandleExceptionVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ICanHandleException * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ICanHandleException * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ICanHandleException * This);
        
        DECLSPEC_XFGVIRT(ICanHandleException, CanHandleException)
        HRESULT ( STDMETHODCALLTYPE *CanHandleException )( 
            __RPC__in ICanHandleException * This,
            /* [in] */ __RPC__in EXCEPINFO *pExcepInfo,
            /* [in] */ __RPC__in VARIANT *pvar);
        
        END_INTERFACE
    } ICanHandleExceptionVtbl;

    interface ICanHandleException
    {
        CONST_VTBL struct ICanHandleExceptionVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ICanHandleException_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ICanHandleException_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ICanHandleException_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ICanHandleException_CanHandleException(This,pExcepInfo,pvar)	\
    ( (This)->lpVtbl -> CanHandleException(This,pExcepInfo,pvar) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ICanHandleException_INTERFACE_DEFINED__ */


#ifndef __IProvideRuntimeContext_INTERFACE_DEFINED__
#define __IProvideRuntimeContext_INTERFACE_DEFINED__

/* interface IProvideRuntimeContext */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IProvideRuntimeContext;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("10E2414A-EC59-49d2-BC51-5ADD2C36FEBC")
    IProvideRuntimeContext : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetCurrentSourceContext( 
            /* [out] */ __RPC__out DWORD_PTR *pdwContext,
            /* [out] */ __RPC__out VARIANT_BOOL *pfExecutingGlobalCode) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IProvideRuntimeContextVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IProvideRuntimeContext * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IProvideRuntimeContext * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IProvideRuntimeContext * This);
        
        DECLSPEC_XFGVIRT(IProvideRuntimeContext, GetCurrentSourceContext)
        HRESULT ( STDMETHODCALLTYPE *GetCurrentSourceContext )( 
            __RPC__in IProvideRuntimeContext * This,
            /* [out] */ __RPC__out DWORD_PTR *pdwContext,
            /* [out] */ __RPC__out VARIANT_BOOL *pfExecutingGlobalCode);
        
        END_INTERFACE
    } IProvideRuntimeContextVtbl;

    interface IProvideRuntimeContext
    {
        CONST_VTBL struct IProvideRuntimeContextVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IProvideRuntimeContext_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IProvideRuntimeContext_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IProvideRuntimeContext_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IProvideRuntimeContext_GetCurrentSourceContext(This,pdwContext,pfExecutingGlobalCode)	\
    ( (This)->lpVtbl -> GetCurrentSourceContext(This,pdwContext,pfExecutingGlobalCode) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IProvideRuntimeContext_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_dispex_0000_0006 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion
#endif //DISPEX_H_


extern RPC_IF_HANDLE __MIDL_itf_dispex_0000_0006_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_dispex_0000_0006_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

unsigned long             __RPC_USER  BSTR_UserSize(     __RPC__in unsigned long *, unsigned long            , __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserMarshal(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out BSTR * ); 
void                      __RPC_USER  BSTR_UserFree(     __RPC__in unsigned long *, __RPC__in BSTR * ); 

unsigned long             __RPC_USER  VARIANT_UserSize(     __RPC__in unsigned long *, unsigned long            , __RPC__in VARIANT * ); 
unsigned char * __RPC_USER  VARIANT_UserMarshal(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in VARIANT * ); 
unsigned char * __RPC_USER  VARIANT_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out VARIANT * ); 
void                      __RPC_USER  VARIANT_UserFree(     __RPC__in unsigned long *, __RPC__in VARIANT * ); 

unsigned long             __RPC_USER  BSTR_UserSize64(     __RPC__in unsigned long *, unsigned long            , __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserMarshal64(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out BSTR * ); 
void                      __RPC_USER  BSTR_UserFree64(     __RPC__in unsigned long *, __RPC__in BSTR * ); 

unsigned long             __RPC_USER  VARIANT_UserSize64(     __RPC__in unsigned long *, unsigned long            , __RPC__in VARIANT * ); 
unsigned char * __RPC_USER  VARIANT_UserMarshal64(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in VARIANT * ); 
unsigned char * __RPC_USER  VARIANT_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out VARIANT * ); 
void                      __RPC_USER  VARIANT_UserFree64(     __RPC__in unsigned long *, __RPC__in VARIANT * ); 

/* [local] */ HRESULT STDMETHODCALLTYPE IDispatchEx_InvokeEx_Proxy( 
    IDispatchEx * This,
    /* [annotation][in] */ 
    _In_  DISPID id,
    /* [annotation][in] */ 
    _In_  LCID lcid,
    /* [annotation][in] */ 
    _In_  WORD wFlags,
    /* [annotation][in] */ 
    _In_  DISPPARAMS *pdp,
    /* [annotation][out] */ 
    _Out_opt_  VARIANT *pvarRes,
    /* [annotation][out] */ 
    _Out_opt_  EXCEPINFO *pei,
    /* [annotation][unique][in] */ 
    _In_opt_  IServiceProvider *pspCaller);


/* [call_as] */ HRESULT STDMETHODCALLTYPE IDispatchEx_InvokeEx_Stub( 
    __RPC__in IDispatchEx * This,
    /* [in] */ DISPID id,
    /* [in] */ LCID lcid,
    /* [in] */ DWORD dwFlags,
    /* [in] */ __RPC__in DISPPARAMS *pdp,
    /* [out] */ __RPC__out VARIANT *pvarRes,
    /* [out] */ __RPC__out EXCEPINFO *pei,
    /* [unique][in] */ __RPC__in_opt IServiceProvider *pspCaller,
    /* [in] */ UINT cvarRefArg,
    /* [size_is][in] */ __RPC__in_ecount_full(cvarRefArg) UINT *rgiRefArg,
    /* [size_is][out][in] */ __RPC__inout_ecount_full(cvarRefArg) VARIANT *rgvarRefArg);



/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


