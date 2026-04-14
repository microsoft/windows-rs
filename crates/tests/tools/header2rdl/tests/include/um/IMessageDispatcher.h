

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

#ifndef __imessagedispatcher_h__
#define __imessagedispatcher_h__

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

#ifndef __IMessageDispatcher_FWD_DEFINED__
#define __IMessageDispatcher_FWD_DEFINED__
typedef interface IMessageDispatcher IMessageDispatcher;

#endif 	/* __IMessageDispatcher_FWD_DEFINED__ */


/* header files for imported files */
#include "Inspectable.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_imessagedispatcher_0000_0000 */
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
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
#pragma warning(push)
#pragma warning(disable:4668) 
#pragma warning(disable:4001) 
#pragma once
#pragma warning(pop)


extern RPC_IF_HANDLE __MIDL_itf_imessagedispatcher_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_imessagedispatcher_0000_0000_v0_0_s_ifspec;

#ifndef __IMessageDispatcher_INTERFACE_DEFINED__
#define __IMessageDispatcher_INTERFACE_DEFINED__

/* interface IMessageDispatcher */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_IMessageDispatcher;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("F5F84C8F-CFD0-4CD6-B66B-C5D26FF1689D")
    IMessageDispatcher : public IInspectable
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE PumpMessages( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMessageDispatcherVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMessageDispatcher * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMessageDispatcher * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMessageDispatcher * This);
        
        DECLSPEC_XFGVIRT(IInspectable, GetIids)
        HRESULT ( STDMETHODCALLTYPE *GetIids )( 
            IMessageDispatcher * This,
            /* [out] */ ULONG *iidCount,
            /* [size_is][size_is][out] */ IID **iids);
        
        DECLSPEC_XFGVIRT(IInspectable, GetRuntimeClassName)
        HRESULT ( STDMETHODCALLTYPE *GetRuntimeClassName )( 
            IMessageDispatcher * This,
            /* [out] */ HSTRING *className);
        
        DECLSPEC_XFGVIRT(IInspectable, GetTrustLevel)
        HRESULT ( STDMETHODCALLTYPE *GetTrustLevel )( 
            IMessageDispatcher * This,
            /* [out] */ TrustLevel *trustLevel);
        
        DECLSPEC_XFGVIRT(IMessageDispatcher, PumpMessages)
        HRESULT ( STDMETHODCALLTYPE *PumpMessages )( 
            IMessageDispatcher * This);
        
        END_INTERFACE
    } IMessageDispatcherVtbl;

    interface IMessageDispatcher
    {
        CONST_VTBL struct IMessageDispatcherVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMessageDispatcher_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMessageDispatcher_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMessageDispatcher_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMessageDispatcher_GetIids(This,iidCount,iids)	\
    ( (This)->lpVtbl -> GetIids(This,iidCount,iids) ) 

#define IMessageDispatcher_GetRuntimeClassName(This,className)	\
    ( (This)->lpVtbl -> GetRuntimeClassName(This,className) ) 

#define IMessageDispatcher_GetTrustLevel(This,trustLevel)	\
    ( (This)->lpVtbl -> GetTrustLevel(This,trustLevel) ) 


#define IMessageDispatcher_PumpMessages(This)	\
    ( (This)->lpVtbl -> PumpMessages(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMessageDispatcher_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_imessagedispatcher_0000_0001 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_imessagedispatcher_0000_0001_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_imessagedispatcher_0000_0001_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


