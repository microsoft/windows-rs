

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

#ifndef __tpwaitablehandle_h__
#define __tpwaitablehandle_h__

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

#ifndef __ISignalableNotifier_FWD_DEFINED__
#define __ISignalableNotifier_FWD_DEFINED__
typedef interface ISignalableNotifier ISignalableNotifier;

#endif 	/* __ISignalableNotifier_FWD_DEFINED__ */


#ifndef __SignalableNotifier_FWD_DEFINED__
#define __SignalableNotifier_FWD_DEFINED__

#ifdef __cplusplus
typedef class SignalableNotifier SignalableNotifier;
#else
typedef struct SignalableNotifier SignalableNotifier;
#endif /* __cplusplus */

#endif 	/* __SignalableNotifier_FWD_DEFINED__ */


/* header files for imported files */
#include "oaidl.h"
#include "ocidl.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_tpwaitablehandle_0000_0000 */
/* [local] */ 

#if (NTDDI_VERSION >= NTDDI_WIN8)


extern RPC_IF_HANDLE __MIDL_itf_tpwaitablehandle_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_tpwaitablehandle_0000_0000_v0_0_s_ifspec;

#ifndef __ISignalableNotifier_INTERFACE_DEFINED__
#define __ISignalableNotifier_INTERFACE_DEFINED__

/* interface ISignalableNotifier */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_ISignalableNotifier;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("46305c32-09c6-45b2-b032-c12e467d7c7e")
    ISignalableNotifier : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE AttachToWaitHandle( 
            /* [in] */ HANDLE_PTR waitableHandle,
            /* [in] */ __RPC__in_opt IUnknown *handler,
            /* [in] */ __RPC__in_opt IUnknown *pUnkOuter,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][out] */ __RPC__deref_out_opt void **SignalNotifier) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE AttachToWaitHandleWithTimeout( 
            /* [in] */ HANDLE_PTR waitableHandle,
            /* [in] */ __RPC__in_opt IUnknown *handler,
            /* [in] */ LARGE_INTEGER timeout,
            /* [in] */ __RPC__in_opt IUnknown *pUnkOuter,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][out] */ __RPC__deref_out_opt void **SignalNotifier) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISignalableNotifierVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ISignalableNotifier * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ISignalableNotifier * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ISignalableNotifier * This);
        
        DECLSPEC_XFGVIRT(ISignalableNotifier, AttachToWaitHandle)
        HRESULT ( STDMETHODCALLTYPE *AttachToWaitHandle )( 
            __RPC__in ISignalableNotifier * This,
            /* [in] */ HANDLE_PTR waitableHandle,
            /* [in] */ __RPC__in_opt IUnknown *handler,
            /* [in] */ __RPC__in_opt IUnknown *pUnkOuter,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][out] */ __RPC__deref_out_opt void **SignalNotifier);
        
        DECLSPEC_XFGVIRT(ISignalableNotifier, AttachToWaitHandleWithTimeout)
        HRESULT ( STDMETHODCALLTYPE *AttachToWaitHandleWithTimeout )( 
            __RPC__in ISignalableNotifier * This,
            /* [in] */ HANDLE_PTR waitableHandle,
            /* [in] */ __RPC__in_opt IUnknown *handler,
            /* [in] */ LARGE_INTEGER timeout,
            /* [in] */ __RPC__in_opt IUnknown *pUnkOuter,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][out] */ __RPC__deref_out_opt void **SignalNotifier);
        
        END_INTERFACE
    } ISignalableNotifierVtbl;

    interface ISignalableNotifier
    {
        CONST_VTBL struct ISignalableNotifierVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISignalableNotifier_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISignalableNotifier_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISignalableNotifier_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISignalableNotifier_AttachToWaitHandle(This,waitableHandle,handler,pUnkOuter,riid,SignalNotifier)	\
    ( (This)->lpVtbl -> AttachToWaitHandle(This,waitableHandle,handler,pUnkOuter,riid,SignalNotifier) ) 

#define ISignalableNotifier_AttachToWaitHandleWithTimeout(This,waitableHandle,handler,timeout,pUnkOuter,riid,SignalNotifier)	\
    ( (This)->lpVtbl -> AttachToWaitHandleWithTimeout(This,waitableHandle,handler,timeout,pUnkOuter,riid,SignalNotifier) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISignalableNotifier_INTERFACE_DEFINED__ */



#ifndef __SignalableNotifierProviders_LIBRARY_DEFINED__
#define __SignalableNotifierProviders_LIBRARY_DEFINED__

/* library SignalableNotifierProviders */
/* [uuid] */ 


EXTERN_C const IID LIBID_SignalableNotifierProviders;

EXTERN_C const CLSID CLSID_SignalableNotifier;

#ifdef __cplusplus

class DECLSPEC_UUID("96c7a5ef-0e2c-46d7-9bc1-6445c2444d7a")
SignalableNotifier;
#endif
#endif /* __SignalableNotifierProviders_LIBRARY_DEFINED__ */

/* interface __MIDL_itf_tpwaitablehandle_0000_0002 */
/* [local] */ 

#endif // (NTDDI >= NTDDI_WIN8)


extern RPC_IF_HANDLE __MIDL_itf_tpwaitablehandle_0000_0002_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_tpwaitablehandle_0000_0002_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


