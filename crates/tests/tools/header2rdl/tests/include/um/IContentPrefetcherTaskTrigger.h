

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

#ifndef __icontentprefetchertasktrigger_h__
#define __icontentprefetchertasktrigger_h__

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

#ifndef __IContentPrefetcherTaskTrigger_FWD_DEFINED__
#define __IContentPrefetcherTaskTrigger_FWD_DEFINED__
typedef interface IContentPrefetcherTaskTrigger IContentPrefetcherTaskTrigger;

#endif 	/* __IContentPrefetcherTaskTrigger_FWD_DEFINED__ */


/* header files for imported files */
#include "oaidl.h"
#include "inspectable.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_icontentprefetchertasktrigger_0000_0000 */
/* [local] */ 

#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


extern RPC_IF_HANDLE __MIDL_itf_icontentprefetchertasktrigger_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_icontentprefetchertasktrigger_0000_0000_v0_0_s_ifspec;

#ifndef __IContentPrefetcherTaskTrigger_INTERFACE_DEFINED__
#define __IContentPrefetcherTaskTrigger_INTERFACE_DEFINED__

/* interface IContentPrefetcherTaskTrigger */
/* [object][uuid] */ 


EXTERN_C const IID IID_IContentPrefetcherTaskTrigger;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("1b35a14a-6094-4799-a60e-e474e15d4dc9")
    IContentPrefetcherTaskTrigger : public IInspectable
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE TriggerContentPrefetcherTask( 
            /* [in] */ __RPC__in LPCWSTR packageFullName) = 0;
        
        virtual /* [annotation][local] */ 
        _On_failure_(_Post_satisfies_(*isRegistered == false))
        HRESULT STDMETHODCALLTYPE IsRegisteredForContentPrefetch( 
            /* [in] */ LPCWSTR packageFullName,
            /* [out] */ boolean *isRegistered) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IContentPrefetcherTaskTriggerVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IContentPrefetcherTaskTrigger * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IContentPrefetcherTaskTrigger * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IContentPrefetcherTaskTrigger * This);
        
        DECLSPEC_XFGVIRT(IInspectable, GetIids)
        HRESULT ( STDMETHODCALLTYPE *GetIids )( 
            __RPC__in IContentPrefetcherTaskTrigger * This,
            /* [out] */ __RPC__out ULONG *iidCount,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*iidCount) IID **iids);
        
        DECLSPEC_XFGVIRT(IInspectable, GetRuntimeClassName)
        HRESULT ( STDMETHODCALLTYPE *GetRuntimeClassName )( 
            __RPC__in IContentPrefetcherTaskTrigger * This,
            /* [out] */ __RPC__deref_out_opt HSTRING *className);
        
        DECLSPEC_XFGVIRT(IInspectable, GetTrustLevel)
        HRESULT ( STDMETHODCALLTYPE *GetTrustLevel )( 
            __RPC__in IContentPrefetcherTaskTrigger * This,
            /* [out] */ __RPC__out TrustLevel *trustLevel);
        
        DECLSPEC_XFGVIRT(IContentPrefetcherTaskTrigger, TriggerContentPrefetcherTask)
        HRESULT ( STDMETHODCALLTYPE *TriggerContentPrefetcherTask )( 
            __RPC__in IContentPrefetcherTaskTrigger * This,
            /* [in] */ __RPC__in LPCWSTR packageFullName);
        
        DECLSPEC_XFGVIRT(IContentPrefetcherTaskTrigger, IsRegisteredForContentPrefetch)
        /* [annotation][local] */ 
        _On_failure_(_Post_satisfies_(*isRegistered == false))
        HRESULT ( STDMETHODCALLTYPE *IsRegisteredForContentPrefetch )( 
            IContentPrefetcherTaskTrigger * This,
            /* [in] */ LPCWSTR packageFullName,
            /* [out] */ boolean *isRegistered);
        
        END_INTERFACE
    } IContentPrefetcherTaskTriggerVtbl;

    interface IContentPrefetcherTaskTrigger
    {
        CONST_VTBL struct IContentPrefetcherTaskTriggerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IContentPrefetcherTaskTrigger_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IContentPrefetcherTaskTrigger_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IContentPrefetcherTaskTrigger_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IContentPrefetcherTaskTrigger_GetIids(This,iidCount,iids)	\
    ( (This)->lpVtbl -> GetIids(This,iidCount,iids) ) 

#define IContentPrefetcherTaskTrigger_GetRuntimeClassName(This,className)	\
    ( (This)->lpVtbl -> GetRuntimeClassName(This,className) ) 

#define IContentPrefetcherTaskTrigger_GetTrustLevel(This,trustLevel)	\
    ( (This)->lpVtbl -> GetTrustLevel(This,trustLevel) ) 


#define IContentPrefetcherTaskTrigger_TriggerContentPrefetcherTask(This,packageFullName)	\
    ( (This)->lpVtbl -> TriggerContentPrefetcherTask(This,packageFullName) ) 

#define IContentPrefetcherTaskTrigger_IsRegisteredForContentPrefetch(This,packageFullName,isRegistered)	\
    ( (This)->lpVtbl -> IsRegisteredForContentPrefetch(This,packageFullName,isRegistered) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IContentPrefetcherTaskTrigger_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_icontentprefetchertasktrigger_0000_0001 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_icontentprefetchertasktrigger_0000_0001_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_icontentprefetchertasktrigger_0000_0001_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


