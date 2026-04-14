

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

#ifndef __unknwn_h__
#define __unknwn_h__

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

#ifndef __IUnknown_FWD_DEFINED__
#define __IUnknown_FWD_DEFINED__
typedef interface IUnknown IUnknown;

#endif 	/* __IUnknown_FWD_DEFINED__ */


#ifndef __AsyncIUnknown_FWD_DEFINED__
#define __AsyncIUnknown_FWD_DEFINED__
typedef interface AsyncIUnknown AsyncIUnknown;

#endif 	/* __AsyncIUnknown_FWD_DEFINED__ */


#ifndef __IClassFactory_FWD_DEFINED__
#define __IClassFactory_FWD_DEFINED__
typedef interface IClassFactory IClassFactory;

#endif 	/* __IClassFactory_FWD_DEFINED__ */


/* header files for imported files */
#include "wtypes.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_unknwn_0000_0000 */
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
#include <winapifamily.h>
#pragma region Application Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)


extern RPC_IF_HANDLE __MIDL_itf_unknwn_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_unknwn_0000_0000_v0_0_s_ifspec;

#ifndef __IUnknown_INTERFACE_DEFINED__
#define __IUnknown_INTERFACE_DEFINED__

/* interface IUnknown */
/* [unique][uuid][object][local] */ 

typedef /* [unique] */ IUnknown *LPUNKNOWN;

//////////////////////////////////////////////////////////////////
// IID_IUnknown and all other system IIDs are provided in UUID.LIB
// Link that library in with your proxies, clients and servers
//////////////////////////////////////////////////////////////////

#if (_MSC_VER >= 1100) && defined(__cplusplus) && !defined(CINTERFACE)
    EXTERN_C const IID IID_IUnknown;
    extern "C++"
    {
        MIDL_INTERFACE("00000000-0000-0000-C000-000000000046")
        IUnknown
        {
        public:
            BEGIN_INTERFACE
            virtual HRESULT STDMETHODCALLTYPE QueryInterface( 
                /* [in] */ REFIID riid,
                /* [iid_is][out] */ _COM_Outptr_ void __RPC_FAR *__RPC_FAR *ppvObject) = 0;

            virtual ULONG STDMETHODCALLTYPE AddRef( void) = 0;

            virtual ULONG STDMETHODCALLTYPE Release( void) = 0;

            template<class Q>
            HRESULT
#ifdef _M_CEE_PURE
            __clrcall
#else
            STDMETHODCALLTYPE
#endif
            QueryInterface(_COM_Outptr_ Q** pp)
            {
                return QueryInterface(__uuidof(Q), (void **)pp);
            }

            END_INTERFACE
        };
    } // extern C++
    HRESULT STDMETHODCALLTYPE IUnknown_QueryInterface_Proxy(
        IUnknown __RPC_FAR * This,
        /* [in] */ REFIID riid,
        /* [iid_is][out] */ __RPC__deref_out void __RPC_FAR *__RPC_FAR *ppvObject);
    
    void __RPC_STUB IUnknown_QueryInterface_Stub(
        IRpcStubBuffer *This,
        IRpcChannelBuffer *_pRpcChannelBuffer,
        PRPC_MESSAGE _pRpcMessage,
        DWORD *_pdwStubPhase);
    
    ULONG STDMETHODCALLTYPE IUnknown_AddRef_Proxy(
        IUnknown __RPC_FAR * This);
    
    void __RPC_STUB IUnknown_AddRef_Stub(
        IRpcStubBuffer *This,
        IRpcChannelBuffer *_pRpcChannelBuffer,
        PRPC_MESSAGE _pRpcMessage,
        DWORD *_pdwStubPhase);
    
    ULONG STDMETHODCALLTYPE IUnknown_Release_Proxy(
        IUnknown __RPC_FAR * This);
    
    void __RPC_STUB IUnknown_Release_Stub(
        IRpcStubBuffer *This,
        IRpcChannelBuffer *_pRpcChannelBuffer,
        PRPC_MESSAGE _pRpcMessage,
        DWORD *_pdwStubPhase);
#else

EXTERN_C const IID IID_IUnknown;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("00000000-0000-0000-C000-000000000046")
    IUnknown
    {
    public:
        BEGIN_INTERFACE
        virtual HRESULT STDMETHODCALLTYPE QueryInterface( 
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject) = 0;
        
        virtual ULONG STDMETHODCALLTYPE AddRef( void) = 0;
        
        virtual ULONG STDMETHODCALLTYPE Release( void) = 0;
        
        END_INTERFACE
    };
    
    
#else 	/* C style interface */

    typedef struct IUnknownVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IUnknown * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IUnknown * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IUnknown * This);
        
        END_INTERFACE
    } IUnknownVtbl;

    interface IUnknown
    {
        CONST_VTBL struct IUnknownVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IUnknown_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IUnknown_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IUnknown_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */



HRESULT STDMETHODCALLTYPE IUnknown_QueryInterface_Proxy( 
    IUnknown * This,
    /* [in] */ REFIID riid,
    /* [annotation][iid_is][out] */ 
    _COM_Outptr_  void **ppvObject);


void __RPC_STUB IUnknown_QueryInterface_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


ULONG STDMETHODCALLTYPE IUnknown_AddRef_Proxy( 
    IUnknown * This);


void __RPC_STUB IUnknown_AddRef_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


ULONG STDMETHODCALLTYPE IUnknown_Release_Proxy( 
    IUnknown * This);


void __RPC_STUB IUnknown_Release_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);



#endif 	/* __IUnknown_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_unknwn_0000_0001 */
/* [local] */ 

#endif
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion
#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)


extern RPC_IF_HANDLE __MIDL_itf_unknwn_0000_0001_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_unknwn_0000_0001_v0_0_s_ifspec;

#ifndef __AsyncIUnknown_INTERFACE_DEFINED__
#define __AsyncIUnknown_INTERFACE_DEFINED__

/* interface AsyncIUnknown */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_AsyncIUnknown;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("000e0000-0000-0000-C000-000000000046")
    AsyncIUnknown : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Begin_QueryInterface( 
            /* [in] */ REFIID riid) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Finish_QueryInterface( 
            /* [annotation][out] */ 
            __RPC__deref_out  void **ppvObject) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Begin_AddRef( void) = 0;
        
        virtual ULONG STDMETHODCALLTYPE Finish_AddRef( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Begin_Release( void) = 0;
        
        virtual ULONG STDMETHODCALLTYPE Finish_Release( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct AsyncIUnknownVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            AsyncIUnknown * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            AsyncIUnknown * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            AsyncIUnknown * This);
        
        DECLSPEC_XFGVIRT(AsyncIUnknown, Begin_QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *Begin_QueryInterface )( 
            AsyncIUnknown * This,
            /* [in] */ REFIID riid);
        
        DECLSPEC_XFGVIRT(AsyncIUnknown, Finish_QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *Finish_QueryInterface )( 
            AsyncIUnknown * This,
            /* [annotation][out] */ 
            __RPC__deref_out  void **ppvObject);
        
        DECLSPEC_XFGVIRT(AsyncIUnknown, Begin_AddRef)
        HRESULT ( STDMETHODCALLTYPE *Begin_AddRef )( 
            AsyncIUnknown * This);
        
        DECLSPEC_XFGVIRT(AsyncIUnknown, Finish_AddRef)
        ULONG ( STDMETHODCALLTYPE *Finish_AddRef )( 
            AsyncIUnknown * This);
        
        DECLSPEC_XFGVIRT(AsyncIUnknown, Begin_Release)
        HRESULT ( STDMETHODCALLTYPE *Begin_Release )( 
            AsyncIUnknown * This);
        
        DECLSPEC_XFGVIRT(AsyncIUnknown, Finish_Release)
        ULONG ( STDMETHODCALLTYPE *Finish_Release )( 
            AsyncIUnknown * This);
        
        END_INTERFACE
    } AsyncIUnknownVtbl;

    interface AsyncIUnknown
    {
        CONST_VTBL struct AsyncIUnknownVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define AsyncIUnknown_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define AsyncIUnknown_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define AsyncIUnknown_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define AsyncIUnknown_Begin_QueryInterface(This,riid)	\
    ( (This)->lpVtbl -> Begin_QueryInterface(This,riid) ) 

#define AsyncIUnknown_Finish_QueryInterface(This,ppvObject)	\
    ( (This)->lpVtbl -> Finish_QueryInterface(This,ppvObject) ) 

#define AsyncIUnknown_Begin_AddRef(This)	\
    ( (This)->lpVtbl -> Begin_AddRef(This) ) 

#define AsyncIUnknown_Finish_AddRef(This)	\
    ( (This)->lpVtbl -> Finish_AddRef(This) ) 

#define AsyncIUnknown_Begin_Release(This)	\
    ( (This)->lpVtbl -> Begin_Release(This) ) 

#define AsyncIUnknown_Finish_Release(This)	\
    ( (This)->lpVtbl -> Finish_Release(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __AsyncIUnknown_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_unknwn_0000_0002 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion
#pragma region Application Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)


extern RPC_IF_HANDLE __MIDL_itf_unknwn_0000_0002_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_unknwn_0000_0002_v0_0_s_ifspec;

#ifndef __IClassFactory_INTERFACE_DEFINED__
#define __IClassFactory_INTERFACE_DEFINED__

/* interface IClassFactory */
/* [unique][uuid][object] */ 

typedef /* [unique] */  __RPC_unique_pointer IClassFactory *LPCLASSFACTORY;


EXTERN_C const IID IID_IClassFactory;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("00000001-0000-0000-C000-000000000046")
    IClassFactory : public IUnknown
    {
    public:
        virtual /* [local] */ HRESULT STDMETHODCALLTYPE CreateInstance( 
            /* [annotation][unique][in] */ 
            _In_opt_  IUnknown *pUnkOuter,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject) = 0;
        
        virtual /* [local] */ HRESULT STDMETHODCALLTYPE LockServer( 
            /* [in] */ BOOL fLock) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IClassFactoryVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IClassFactory * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IClassFactory * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IClassFactory * This);
        
        DECLSPEC_XFGVIRT(IClassFactory, CreateInstance)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *CreateInstance )( 
            IClassFactory * This,
            /* [annotation][unique][in] */ 
            _In_opt_  IUnknown *pUnkOuter,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IClassFactory, LockServer)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *LockServer )( 
            IClassFactory * This,
            /* [in] */ BOOL fLock);
        
        END_INTERFACE
    } IClassFactoryVtbl;

    interface IClassFactory
    {
        CONST_VTBL struct IClassFactoryVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IClassFactory_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IClassFactory_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IClassFactory_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IClassFactory_CreateInstance(This,pUnkOuter,riid,ppvObject)	\
    ( (This)->lpVtbl -> CreateInstance(This,pUnkOuter,riid,ppvObject) ) 

#define IClassFactory_LockServer(This,fLock)	\
    ( (This)->lpVtbl -> LockServer(This,fLock) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */



/* [call_as] */ HRESULT STDMETHODCALLTYPE IClassFactory_RemoteCreateInstance_Proxy( 
    __RPC__in IClassFactory * This,
    /* [in] */ __RPC__in REFIID riid,
    /* [iid_is][out] */ __RPC__deref_out_opt IUnknown **ppvObject);


void __RPC_STUB IClassFactory_RemoteCreateInstance_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


/* [call_as] */ HRESULT __stdcall IClassFactory_RemoteLockServer_Proxy( 
    __RPC__in IClassFactory * This,
    /* [in] */ BOOL fLock);


void __RPC_STUB IClassFactory_RemoteLockServer_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);



#endif 	/* __IClassFactory_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_unknwn_0000_0003 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_unknwn_0000_0003_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_unknwn_0000_0003_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

/* [local] */ HRESULT STDMETHODCALLTYPE IClassFactory_CreateInstance_Proxy( 
    IClassFactory * This,
    /* [annotation][unique][in] */ 
    _In_opt_  IUnknown *pUnkOuter,
    /* [annotation][in] */ 
    _In_  REFIID riid,
    /* [annotation][iid_is][out] */ 
    _COM_Outptr_  void **ppvObject);


/* [call_as] */ HRESULT STDMETHODCALLTYPE IClassFactory_CreateInstance_Stub( 
    __RPC__in IClassFactory * This,
    /* [in] */ __RPC__in REFIID riid,
    /* [iid_is][out] */ __RPC__deref_out_opt IUnknown **ppvObject);

/* [local] */ HRESULT STDMETHODCALLTYPE IClassFactory_LockServer_Proxy( 
    IClassFactory * This,
    /* [in] */ BOOL fLock);


/* [call_as] */ HRESULT __stdcall IClassFactory_LockServer_Stub( 
    __RPC__in IClassFactory * This,
    /* [in] */ BOOL fLock);



/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


