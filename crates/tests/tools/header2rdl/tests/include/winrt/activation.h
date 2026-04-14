

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

#ifndef __activation_h__
#define __activation_h__

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

#ifndef __IActivationFactory_FWD_DEFINED__
#define __IActivationFactory_FWD_DEFINED__
typedef interface IActivationFactory IActivationFactory;

#endif 	/* __IActivationFactory_FWD_DEFINED__ */


/* header files for imported files */
#include "inspectable.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_activation_0000_0000 */
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


extern RPC_IF_HANDLE __MIDL_itf_activation_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_activation_0000_0000_v0_0_s_ifspec;

#ifndef __IActivationFactory_INTERFACE_DEFINED__
#define __IActivationFactory_INTERFACE_DEFINED__

/* interface IActivationFactory */
/* [unique][uuid][object] */ 

typedef /* [unique] */  __RPC_unique_pointer IActivationFactory *PACTIVATIONFACTORY;


EXTERN_C const IID IID_IActivationFactory;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("00000035-0000-0000-C000-000000000046")
    IActivationFactory : public IInspectable
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE ActivateInstance( 
            /* [out] */ __RPC__deref_out_opt IInspectable **instance) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IActivationFactoryVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IActivationFactory * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IActivationFactory * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IActivationFactory * This);
        
        DECLSPEC_XFGVIRT(IInspectable, GetIids)
        HRESULT ( STDMETHODCALLTYPE *GetIids )( 
            __RPC__in IActivationFactory * This,
            /* [out] */ __RPC__out ULONG *iidCount,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*iidCount) IID **iids);
        
        DECLSPEC_XFGVIRT(IInspectable, GetRuntimeClassName)
        HRESULT ( STDMETHODCALLTYPE *GetRuntimeClassName )( 
            __RPC__in IActivationFactory * This,
            /* [out] */ __RPC__deref_out_opt HSTRING *className);
        
        DECLSPEC_XFGVIRT(IInspectable, GetTrustLevel)
        HRESULT ( STDMETHODCALLTYPE *GetTrustLevel )( 
            __RPC__in IActivationFactory * This,
            /* [out] */ __RPC__out TrustLevel *trustLevel);
        
        DECLSPEC_XFGVIRT(IActivationFactory, ActivateInstance)
        HRESULT ( STDMETHODCALLTYPE *ActivateInstance )( 
            __RPC__in IActivationFactory * This,
            /* [out] */ __RPC__deref_out_opt IInspectable **instance);
        
        END_INTERFACE
    } IActivationFactoryVtbl;

    interface IActivationFactory
    {
        CONST_VTBL struct IActivationFactoryVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IActivationFactory_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IActivationFactory_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IActivationFactory_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IActivationFactory_GetIids(This,iidCount,iids)	\
    ( (This)->lpVtbl -> GetIids(This,iidCount,iids) ) 

#define IActivationFactory_GetRuntimeClassName(This,className)	\
    ( (This)->lpVtbl -> GetRuntimeClassName(This,className) ) 

#define IActivationFactory_GetTrustLevel(This,trustLevel)	\
    ( (This)->lpVtbl -> GetTrustLevel(This,trustLevel) ) 


#define IActivationFactory_ActivateInstance(This,instance)	\
    ( (This)->lpVtbl -> ActivateInstance(This,instance) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IActivationFactory_INTERFACE_DEFINED__ */


/* Additional Prototypes for ALL interfaces */

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


