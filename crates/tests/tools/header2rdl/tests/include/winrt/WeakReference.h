

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

#ifndef __weakreference_h__
#define __weakreference_h__

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

#ifndef __IWeakReference_FWD_DEFINED__
#define __IWeakReference_FWD_DEFINED__
typedef interface IWeakReference IWeakReference;

#endif 	/* __IWeakReference_FWD_DEFINED__ */


#ifndef __IWeakReferenceSource_FWD_DEFINED__
#define __IWeakReferenceSource_FWD_DEFINED__
typedef interface IWeakReferenceSource IWeakReferenceSource;

#endif 	/* __IWeakReferenceSource_FWD_DEFINED__ */


/* header files for imported files */
#include "inspectable.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_weakreference_0000_0000 */
/* [local] */ 

//  Microsoft Windows
//  Copyright (c) Microsoft Corporation. All rights reserved.
#pragma once


extern RPC_IF_HANDLE __MIDL_itf_weakreference_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_weakreference_0000_0000_v0_0_s_ifspec;

#ifndef __IWeakReference_INTERFACE_DEFINED__
#define __IWeakReference_INTERFACE_DEFINED__

/* interface IWeakReference */
/* [unique][uuid][object] */ 

#if (_MSC_VER >= 1600) && defined(__cplusplus) && !defined(CINTERFACE)
    EXTERN_C const IID IID_IWeakReference;
    extern "C++"
    {
        MIDL_INTERFACE("00000037-0000-0000-C000-000000000046")
        IWeakReference : public IUnknown
        {
        public:
            virtual HRESULT STDMETHODCALLTYPE Resolve( 
                /* [in] */ __RPC__in REFIID riid,
                /* [iid_is][out] */ __RPC__deref_out IInspectable **objectReference) = 0;

            template <typename T>
            _At_(*objectReference, _When_(FAILED(return), _Null_))
            _At_(*objectReference, _When_(SUCCEEDED(return), _Maybenull_))
            HRESULT Resolve(_Out_ T** objectReference)
            {
                static_assert(__is_base_of(IInspectable, T), "Only Windows Runtime interfaces can be resolved by weak reference");
                return Resolve(__uuidof(T), (IInspectable**)objectReference);
            }
        };
    } // extern C++
#else

EXTERN_C const IID IID_IWeakReference;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("00000037-0000-0000-C000-000000000046")
    IWeakReference : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Resolve( 
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][retval][out] */ __RPC__deref_out_opt IInspectable **objectReference) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWeakReferenceVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IWeakReference * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IWeakReference * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IWeakReference * This);
        
        DECLSPEC_XFGVIRT(IWeakReference, Resolve)
        HRESULT ( STDMETHODCALLTYPE *Resolve )( 
            __RPC__in IWeakReference * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][retval][out] */ __RPC__deref_out_opt IInspectable **objectReference);
        
        END_INTERFACE
    } IWeakReferenceVtbl;

    interface IWeakReference
    {
        CONST_VTBL struct IWeakReferenceVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWeakReference_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWeakReference_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWeakReference_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWeakReference_Resolve(This,riid,objectReference)	\
    ( (This)->lpVtbl -> Resolve(This,riid,objectReference) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWeakReference_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_weakreference_0000_0001 */
/* [local] */ 

#endif


extern RPC_IF_HANDLE __MIDL_itf_weakreference_0000_0001_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_weakreference_0000_0001_v0_0_s_ifspec;

#ifndef __IWeakReferenceSource_INTERFACE_DEFINED__
#define __IWeakReferenceSource_INTERFACE_DEFINED__

/* interface IWeakReferenceSource */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IWeakReferenceSource;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("00000038-0000-0000-C000-000000000046")
    IWeakReferenceSource : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetWeakReference( 
            /* [retval][out] */ __RPC__deref_out_opt IWeakReference **weakReference) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWeakReferenceSourceVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IWeakReferenceSource * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IWeakReferenceSource * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IWeakReferenceSource * This);
        
        DECLSPEC_XFGVIRT(IWeakReferenceSource, GetWeakReference)
        HRESULT ( STDMETHODCALLTYPE *GetWeakReference )( 
            __RPC__in IWeakReferenceSource * This,
            /* [retval][out] */ __RPC__deref_out_opt IWeakReference **weakReference);
        
        END_INTERFACE
    } IWeakReferenceSourceVtbl;

    interface IWeakReferenceSource
    {
        CONST_VTBL struct IWeakReferenceSourceVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWeakReferenceSource_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWeakReferenceSource_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWeakReferenceSource_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWeakReferenceSource_GetWeakReference(This,weakReference)	\
    ( (This)->lpVtbl -> GetWeakReference(This,weakReference) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWeakReferenceSource_INTERFACE_DEFINED__ */


/* Additional Prototypes for ALL interfaces */

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


