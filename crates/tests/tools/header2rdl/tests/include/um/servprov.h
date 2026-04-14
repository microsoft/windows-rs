

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

#ifndef __servprov_h__
#define __servprov_h__

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

#ifndef __IServiceProvider_FWD_DEFINED__
#define __IServiceProvider_FWD_DEFINED__
typedef interface IServiceProvider IServiceProvider;

#endif 	/* __IServiceProvider_FWD_DEFINED__ */


/* header files for imported files */
#include "objidl.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_servprov_0000_0000 */
/* [local] */ 

//=--------------------------------------------------------------------------=
// ServProv.h
//=--------------------------------------------------------------------------=
// (C) Copyright Microsoft Corporation.  All Rights Reserved.
//
// THIS CODE AND INFORMATION IS PROVIDED "AS IS" WITHOUT WARRANTY OF
// ANY KIND, EITHER EXPRESSED OR IMPLIED, INCLUDING BUT NOT LIMITED TO
// THE IMPLIED WARRANTIES OF MERCHANTABILITY AND/OR FITNESS FOR A
// PARTICULAR PURPOSE.
//=--------------------------------------------------------------------------=

#pragma comment(lib,"uuid.lib")

//---------------------------------------------------------------------------=
// IServiceProvider Interfaces.

#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)



extern RPC_IF_HANDLE __MIDL_itf_servprov_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_servprov_0000_0000_v0_0_s_ifspec;

#ifndef __IServiceProvider_INTERFACE_DEFINED__
#define __IServiceProvider_INTERFACE_DEFINED__

/* interface IServiceProvider */
/* [unique][uuid][object] */ 

typedef /* [unique] */  __RPC_unique_pointer IServiceProvider *LPSERVICEPROVIDER;

#if (_MSC_VER >= 1100) && defined(__cplusplus) && !defined(CINTERFACE)
    EXTERN_C const IID IID_IServiceProvider;
    extern "C++"
    {
        MIDL_INTERFACE("6d5140c1-7436-11ce-8034-00aa006009fa")
        IServiceProvider : public IUnknown
        {
        public:
            virtual /* [local] */ HRESULT STDMETHODCALLTYPE QueryService( 
                /* [in] */ _In_ REFGUID guidService,
                /* [in] */ _In_ REFIID riid,
                /* [out] */ _Outptr_ void __RPC_FAR *__RPC_FAR *ppvObject) = 0;
            
            template <class Q>
            HRESULT STDMETHODCALLTYPE QueryService(_In_ REFGUID guidService, _Outptr_ Q** pp)
            {
                return QueryService(guidService, __uuidof(Q), (void **)pp);
            }
        };
    }

    /* [call_as] */ HRESULT STDMETHODCALLTYPE IServiceProvider_RemoteQueryService_Proxy( 
        _In_ IServiceProvider __RPC_FAR * This,
        /* [in] */ _In_ REFGUID guidService,
        /* [in] */ _In_ REFIID riid,
        /* [iid_is][out] */ _Outptr_ IUnknown __RPC_FAR *__RPC_FAR *ppvObject);

    void __RPC_STUB IServiceProvider_RemoteQueryService_Stub(
        IRpcStubBuffer *This,
        IRpcChannelBuffer *_pRpcChannelBuffer,
        PRPC_MESSAGE _pRpcMessage,
        DWORD *_pdwStubPhase);

#else // VC6 Hack

EXTERN_C const IID IID_IServiceProvider;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("6d5140c1-7436-11ce-8034-00aa006009fa")
    IServiceProvider : public IUnknown
    {
    public:
        virtual /* [local] */ HRESULT STDMETHODCALLTYPE QueryService( 
            /* [annotation][in] */ 
            _In_  REFGUID guidService,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][out] */ 
            _Outptr_  void **ppvObject) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IServiceProviderVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IServiceProvider * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IServiceProvider * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IServiceProvider * This);
        
        DECLSPEC_XFGVIRT(IServiceProvider, QueryService)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *QueryService )( 
            IServiceProvider * This,
            /* [annotation][in] */ 
            _In_  REFGUID guidService,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][out] */ 
            _Outptr_  void **ppvObject);
        
        END_INTERFACE
    } IServiceProviderVtbl;

    interface IServiceProvider
    {
        CONST_VTBL struct IServiceProviderVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IServiceProvider_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IServiceProvider_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IServiceProvider_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IServiceProvider_QueryService(This,guidService,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryService(This,guidService,riid,ppvObject) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */



/* [call_as] */ HRESULT STDMETHODCALLTYPE IServiceProvider_RemoteQueryService_Proxy( 
    __RPC__in IServiceProvider * This,
    /* [in] */ __RPC__in REFGUID guidService,
    /* [in] */ __RPC__in REFIID riid,
    /* [iid_is][out] */ __RPC__deref_out_opt IUnknown **ppvObject);


void __RPC_STUB IServiceProvider_RemoteQueryService_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);



#endif 	/* __IServiceProvider_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_servprov_0000_0001 */
/* [local] */ 

#endif // VC6 Hack
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_servprov_0000_0001_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_servprov_0000_0001_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

/* [local] */ HRESULT STDMETHODCALLTYPE IServiceProvider_QueryService_Proxy( 
    IServiceProvider * This,
    /* [annotation][in] */ 
    _In_  REFGUID guidService,
    /* [annotation][in] */ 
    _In_  REFIID riid,
    /* [annotation][out] */ 
    _Outptr_  void **ppvObject);


/* [call_as] */ HRESULT STDMETHODCALLTYPE IServiceProvider_QueryService_Stub( 
    __RPC__in IServiceProvider * This,
    /* [in] */ __RPC__in REFGUID guidService,
    /* [in] */ __RPC__in REFIID riid,
    /* [iid_is][out] */ __RPC__deref_out_opt IUnknown **ppvObject);



/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


