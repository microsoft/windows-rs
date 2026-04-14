

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

#ifndef __ctxtcall_h__
#define __ctxtcall_h__

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

#ifndef __IContextCallback_FWD_DEFINED__
#define __IContextCallback_FWD_DEFINED__
typedef interface IContextCallback IContextCallback;

#endif 	/* __IContextCallback_FWD_DEFINED__ */


/* header files for imported files */
#include "wtypes.h"
#include "objidl.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_ctxtcall_0000_0000 */
/* [local] */ 

//+-----------------------------------------------------------------
//
//  Microsoft Windows
//  Copyright (c) Microsoft Corporation. All rights reserved.
//
//------------------------------------------------------------------
typedef struct tagComCallData
    {
    DWORD dwDispid;
    DWORD dwReserved;
    void *pUserDefined;
    } 	ComCallData;



extern RPC_IF_HANDLE __MIDL_itf_ctxtcall_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_ctxtcall_0000_0000_v0_0_s_ifspec;

#ifndef __IContextCallback_INTERFACE_DEFINED__
#define __IContextCallback_INTERFACE_DEFINED__

/* interface IContextCallback */
/* [unique][uuid][object][local] */ 

typedef /* [ref] */ HRESULT ( __stdcall *PFNCONTEXTCALL )( 
    ComCallData *pParam);


EXTERN_C const IID IID_IContextCallback;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("000001da-0000-0000-C000-000000000046")
    IContextCallback : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE ContextCallback( 
            /* [in] */ PFNCONTEXTCALL pfnCallback,
            /* [in] */ ComCallData *pParam,
            /* [in] */ REFIID riid,
            /* [in] */ int iMethod,
            /* [in] */ IUnknown *pUnk) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IContextCallbackVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IContextCallback * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IContextCallback * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IContextCallback * This);
        
        DECLSPEC_XFGVIRT(IContextCallback, ContextCallback)
        HRESULT ( STDMETHODCALLTYPE *ContextCallback )( 
            IContextCallback * This,
            /* [in] */ PFNCONTEXTCALL pfnCallback,
            /* [in] */ ComCallData *pParam,
            /* [in] */ REFIID riid,
            /* [in] */ int iMethod,
            /* [in] */ IUnknown *pUnk);
        
        END_INTERFACE
    } IContextCallbackVtbl;

    interface IContextCallback
    {
        CONST_VTBL struct IContextCallbackVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IContextCallback_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IContextCallback_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IContextCallback_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IContextCallback_ContextCallback(This,pfnCallback,pParam,riid,iMethod,pUnk)	\
    ( (This)->lpVtbl -> ContextCallback(This,pfnCallback,pParam,riid,iMethod,pUnk) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IContextCallback_INTERFACE_DEFINED__ */


/* Additional Prototypes for ALL interfaces */

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


