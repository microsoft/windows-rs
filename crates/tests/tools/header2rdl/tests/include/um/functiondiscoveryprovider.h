

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

#ifndef __functiondiscoveryprovider_h__
#define __functiondiscoveryprovider_h__

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

#ifndef __IFunctionDiscoveryProvider_FWD_DEFINED__
#define __IFunctionDiscoveryProvider_FWD_DEFINED__
typedef interface IFunctionDiscoveryProvider IFunctionDiscoveryProvider;

#endif 	/* __IFunctionDiscoveryProvider_FWD_DEFINED__ */


#ifndef __IProviderProperties_FWD_DEFINED__
#define __IProviderProperties_FWD_DEFINED__
typedef interface IProviderProperties IProviderProperties;

#endif 	/* __IProviderProperties_FWD_DEFINED__ */


#ifndef __IProviderPublishing_FWD_DEFINED__
#define __IProviderPublishing_FWD_DEFINED__
typedef interface IProviderPublishing IProviderPublishing;

#endif 	/* __IProviderPublishing_FWD_DEFINED__ */


#ifndef __IFunctionDiscoveryProviderFactory_FWD_DEFINED__
#define __IFunctionDiscoveryProviderFactory_FWD_DEFINED__
typedef interface IFunctionDiscoveryProviderFactory IFunctionDiscoveryProviderFactory;

#endif 	/* __IFunctionDiscoveryProviderFactory_FWD_DEFINED__ */


#ifndef __IFunctionDiscoveryProviderQuery_FWD_DEFINED__
#define __IFunctionDiscoveryProviderQuery_FWD_DEFINED__
typedef interface IFunctionDiscoveryProviderQuery IFunctionDiscoveryProviderQuery;

#endif 	/* __IFunctionDiscoveryProviderQuery_FWD_DEFINED__ */


#ifndef __IProviderQueryConstraintCollection_FWD_DEFINED__
#define __IProviderQueryConstraintCollection_FWD_DEFINED__
typedef interface IProviderQueryConstraintCollection IProviderQueryConstraintCollection;

#endif 	/* __IProviderQueryConstraintCollection_FWD_DEFINED__ */


#ifndef __IProviderPropertyConstraintCollection_FWD_DEFINED__
#define __IProviderPropertyConstraintCollection_FWD_DEFINED__
typedef interface IProviderPropertyConstraintCollection IProviderPropertyConstraintCollection;

#endif 	/* __IProviderPropertyConstraintCollection_FWD_DEFINED__ */


#ifndef __IFunctionDiscoveryServiceProvider_FWD_DEFINED__
#define __IFunctionDiscoveryServiceProvider_FWD_DEFINED__
typedef interface IFunctionDiscoveryServiceProvider IFunctionDiscoveryServiceProvider;

#endif 	/* __IFunctionDiscoveryServiceProvider_FWD_DEFINED__ */


/* header files for imported files */
#include "oaidl.h"
#include "ocidl.h"
#include "propsys.h"
#include "FunctionDiscoveryAPI.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_functiondiscoveryprovider_0000_0000 */
/* [local] */ 

#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)






extern RPC_IF_HANDLE __MIDL_itf_functiondiscoveryprovider_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_functiondiscoveryprovider_0000_0000_v0_0_s_ifspec;

#ifndef __IFunctionDiscoveryProvider_INTERFACE_DEFINED__
#define __IFunctionDiscoveryProvider_INTERFACE_DEFINED__

/* interface IFunctionDiscoveryProvider */
/* [restricted][unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IFunctionDiscoveryProvider;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("dcde394f-1478-4813-a402-f6fb10657222")
    IFunctionDiscoveryProvider : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Initialize( 
            /* [in] */ __RPC__in_opt IFunctionDiscoveryProviderFactory *pIFunctionDiscoveryProviderFactory,
            /* [in] */ __RPC__in_opt IFunctionDiscoveryNotification *pIFunctionDiscoveryNotification,
            /* [in] */ LCID lcidUserDefault,
            /* [out] */ __RPC__out DWORD *pdwStgAccessCapabilities) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Query( 
            /* [in] */ __RPC__in_opt IFunctionDiscoveryProviderQuery *pIFunctionDiscoveryProviderQuery,
            /* [out] */ __RPC__deref_out_opt IFunctionInstanceCollection **ppIFunctionInstanceCollection) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EndQuery( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE InstancePropertyStoreValidateAccess( 
            /* [in] */ __RPC__in_opt IFunctionInstance *pIFunctionInstance,
            /* [in] */ INT_PTR iProviderInstanceContext,
            /* [in] */ const DWORD dwStgAccess) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE InstancePropertyStoreOpen( 
            /* [in] */ __RPC__in_opt IFunctionInstance *pIFunctionInstance,
            /* [in] */ INT_PTR iProviderInstanceContext,
            /* [in] */ const DWORD dwStgAccess,
            /* [out] */ __RPC__deref_out_opt IPropertyStore **ppIPropertyStore) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE InstancePropertyStoreFlush( 
            /* [in] */ __RPC__in_opt IFunctionInstance *pIFunctionInstance,
            /* [in] */ INT_PTR iProviderInstanceContext) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE InstanceQueryService( 
            /* [in] */ __RPC__in_opt IFunctionInstance *pIFunctionInstance,
            /* [in] */ INT_PTR iProviderInstanceContext,
            /* [in] */ __RPC__in REFGUID guidService,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][out] */ __RPC__deref_out_opt IUnknown **ppIUnknown) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE InstanceReleased( 
            /* [in] */ __RPC__in_opt IFunctionInstance *pIFunctionInstance,
            /* [in] */ INT_PTR iProviderInstanceContext) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IFunctionDiscoveryProviderVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IFunctionDiscoveryProvider * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IFunctionDiscoveryProvider * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IFunctionDiscoveryProvider * This);
        
        DECLSPEC_XFGVIRT(IFunctionDiscoveryProvider, Initialize)
        HRESULT ( STDMETHODCALLTYPE *Initialize )( 
            __RPC__in IFunctionDiscoveryProvider * This,
            /* [in] */ __RPC__in_opt IFunctionDiscoveryProviderFactory *pIFunctionDiscoveryProviderFactory,
            /* [in] */ __RPC__in_opt IFunctionDiscoveryNotification *pIFunctionDiscoveryNotification,
            /* [in] */ LCID lcidUserDefault,
            /* [out] */ __RPC__out DWORD *pdwStgAccessCapabilities);
        
        DECLSPEC_XFGVIRT(IFunctionDiscoveryProvider, Query)
        HRESULT ( STDMETHODCALLTYPE *Query )( 
            __RPC__in IFunctionDiscoveryProvider * This,
            /* [in] */ __RPC__in_opt IFunctionDiscoveryProviderQuery *pIFunctionDiscoveryProviderQuery,
            /* [out] */ __RPC__deref_out_opt IFunctionInstanceCollection **ppIFunctionInstanceCollection);
        
        DECLSPEC_XFGVIRT(IFunctionDiscoveryProvider, EndQuery)
        HRESULT ( STDMETHODCALLTYPE *EndQuery )( 
            __RPC__in IFunctionDiscoveryProvider * This);
        
        DECLSPEC_XFGVIRT(IFunctionDiscoveryProvider, InstancePropertyStoreValidateAccess)
        HRESULT ( STDMETHODCALLTYPE *InstancePropertyStoreValidateAccess )( 
            __RPC__in IFunctionDiscoveryProvider * This,
            /* [in] */ __RPC__in_opt IFunctionInstance *pIFunctionInstance,
            /* [in] */ INT_PTR iProviderInstanceContext,
            /* [in] */ const DWORD dwStgAccess);
        
        DECLSPEC_XFGVIRT(IFunctionDiscoveryProvider, InstancePropertyStoreOpen)
        HRESULT ( STDMETHODCALLTYPE *InstancePropertyStoreOpen )( 
            __RPC__in IFunctionDiscoveryProvider * This,
            /* [in] */ __RPC__in_opt IFunctionInstance *pIFunctionInstance,
            /* [in] */ INT_PTR iProviderInstanceContext,
            /* [in] */ const DWORD dwStgAccess,
            /* [out] */ __RPC__deref_out_opt IPropertyStore **ppIPropertyStore);
        
        DECLSPEC_XFGVIRT(IFunctionDiscoveryProvider, InstancePropertyStoreFlush)
        HRESULT ( STDMETHODCALLTYPE *InstancePropertyStoreFlush )( 
            __RPC__in IFunctionDiscoveryProvider * This,
            /* [in] */ __RPC__in_opt IFunctionInstance *pIFunctionInstance,
            /* [in] */ INT_PTR iProviderInstanceContext);
        
        DECLSPEC_XFGVIRT(IFunctionDiscoveryProvider, InstanceQueryService)
        HRESULT ( STDMETHODCALLTYPE *InstanceQueryService )( 
            __RPC__in IFunctionDiscoveryProvider * This,
            /* [in] */ __RPC__in_opt IFunctionInstance *pIFunctionInstance,
            /* [in] */ INT_PTR iProviderInstanceContext,
            /* [in] */ __RPC__in REFGUID guidService,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][out] */ __RPC__deref_out_opt IUnknown **ppIUnknown);
        
        DECLSPEC_XFGVIRT(IFunctionDiscoveryProvider, InstanceReleased)
        HRESULT ( STDMETHODCALLTYPE *InstanceReleased )( 
            __RPC__in IFunctionDiscoveryProvider * This,
            /* [in] */ __RPC__in_opt IFunctionInstance *pIFunctionInstance,
            /* [in] */ INT_PTR iProviderInstanceContext);
        
        END_INTERFACE
    } IFunctionDiscoveryProviderVtbl;

    interface IFunctionDiscoveryProvider
    {
        CONST_VTBL struct IFunctionDiscoveryProviderVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IFunctionDiscoveryProvider_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IFunctionDiscoveryProvider_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IFunctionDiscoveryProvider_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IFunctionDiscoveryProvider_Initialize(This,pIFunctionDiscoveryProviderFactory,pIFunctionDiscoveryNotification,lcidUserDefault,pdwStgAccessCapabilities)	\
    ( (This)->lpVtbl -> Initialize(This,pIFunctionDiscoveryProviderFactory,pIFunctionDiscoveryNotification,lcidUserDefault,pdwStgAccessCapabilities) ) 

#define IFunctionDiscoveryProvider_Query(This,pIFunctionDiscoveryProviderQuery,ppIFunctionInstanceCollection)	\
    ( (This)->lpVtbl -> Query(This,pIFunctionDiscoveryProviderQuery,ppIFunctionInstanceCollection) ) 

#define IFunctionDiscoveryProvider_EndQuery(This)	\
    ( (This)->lpVtbl -> EndQuery(This) ) 

#define IFunctionDiscoveryProvider_InstancePropertyStoreValidateAccess(This,pIFunctionInstance,iProviderInstanceContext,dwStgAccess)	\
    ( (This)->lpVtbl -> InstancePropertyStoreValidateAccess(This,pIFunctionInstance,iProviderInstanceContext,dwStgAccess) ) 

#define IFunctionDiscoveryProvider_InstancePropertyStoreOpen(This,pIFunctionInstance,iProviderInstanceContext,dwStgAccess,ppIPropertyStore)	\
    ( (This)->lpVtbl -> InstancePropertyStoreOpen(This,pIFunctionInstance,iProviderInstanceContext,dwStgAccess,ppIPropertyStore) ) 

#define IFunctionDiscoveryProvider_InstancePropertyStoreFlush(This,pIFunctionInstance,iProviderInstanceContext)	\
    ( (This)->lpVtbl -> InstancePropertyStoreFlush(This,pIFunctionInstance,iProviderInstanceContext) ) 

#define IFunctionDiscoveryProvider_InstanceQueryService(This,pIFunctionInstance,iProviderInstanceContext,guidService,riid,ppIUnknown)	\
    ( (This)->lpVtbl -> InstanceQueryService(This,pIFunctionInstance,iProviderInstanceContext,guidService,riid,ppIUnknown) ) 

#define IFunctionDiscoveryProvider_InstanceReleased(This,pIFunctionInstance,iProviderInstanceContext)	\
    ( (This)->lpVtbl -> InstanceReleased(This,pIFunctionInstance,iProviderInstanceContext) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IFunctionDiscoveryProvider_INTERFACE_DEFINED__ */


#ifndef __IProviderProperties_INTERFACE_DEFINED__
#define __IProviderProperties_INTERFACE_DEFINED__

/* interface IProviderProperties */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IProviderProperties;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("cf986ea6-3b5f-4c5f-b88a-2f8b20ceef17")
    IProviderProperties : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetCount( 
            /* [in] */ __RPC__in_opt IFunctionInstance *pIFunctionInstance,
            /* [in] */ INT_PTR iProviderInstanceContext,
            /* [out] */ __RPC__out DWORD *pdwCount) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetAt( 
            /* [in] */ __RPC__in_opt IFunctionInstance *pIFunctionInstance,
            /* [in] */ INT_PTR iProviderInstanceContext,
            /* [in] */ DWORD dwIndex,
            /* [out] */ __RPC__out PROPERTYKEY *pKey) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetValue( 
            /* [in] */ __RPC__in_opt IFunctionInstance *pIFunctionInstance,
            /* [in] */ INT_PTR iProviderInstanceContext,
            /* [in] */ __RPC__in REFPROPERTYKEY Key,
            /* [out] */ __RPC__out PROPVARIANT *ppropVar) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetValue( 
            /* [in] */ __RPC__in_opt IFunctionInstance *pIFunctionInstance,
            /* [in] */ INT_PTR iProviderInstanceContext,
            /* [in] */ __RPC__in REFPROPERTYKEY Key,
            /* [in] */ __RPC__in const PROPVARIANT *ppropVar) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IProviderPropertiesVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IProviderProperties * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IProviderProperties * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IProviderProperties * This);
        
        DECLSPEC_XFGVIRT(IProviderProperties, GetCount)
        HRESULT ( STDMETHODCALLTYPE *GetCount )( 
            __RPC__in IProviderProperties * This,
            /* [in] */ __RPC__in_opt IFunctionInstance *pIFunctionInstance,
            /* [in] */ INT_PTR iProviderInstanceContext,
            /* [out] */ __RPC__out DWORD *pdwCount);
        
        DECLSPEC_XFGVIRT(IProviderProperties, GetAt)
        HRESULT ( STDMETHODCALLTYPE *GetAt )( 
            __RPC__in IProviderProperties * This,
            /* [in] */ __RPC__in_opt IFunctionInstance *pIFunctionInstance,
            /* [in] */ INT_PTR iProviderInstanceContext,
            /* [in] */ DWORD dwIndex,
            /* [out] */ __RPC__out PROPERTYKEY *pKey);
        
        DECLSPEC_XFGVIRT(IProviderProperties, GetValue)
        HRESULT ( STDMETHODCALLTYPE *GetValue )( 
            __RPC__in IProviderProperties * This,
            /* [in] */ __RPC__in_opt IFunctionInstance *pIFunctionInstance,
            /* [in] */ INT_PTR iProviderInstanceContext,
            /* [in] */ __RPC__in REFPROPERTYKEY Key,
            /* [out] */ __RPC__out PROPVARIANT *ppropVar);
        
        DECLSPEC_XFGVIRT(IProviderProperties, SetValue)
        HRESULT ( STDMETHODCALLTYPE *SetValue )( 
            __RPC__in IProviderProperties * This,
            /* [in] */ __RPC__in_opt IFunctionInstance *pIFunctionInstance,
            /* [in] */ INT_PTR iProviderInstanceContext,
            /* [in] */ __RPC__in REFPROPERTYKEY Key,
            /* [in] */ __RPC__in const PROPVARIANT *ppropVar);
        
        END_INTERFACE
    } IProviderPropertiesVtbl;

    interface IProviderProperties
    {
        CONST_VTBL struct IProviderPropertiesVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IProviderProperties_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IProviderProperties_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IProviderProperties_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IProviderProperties_GetCount(This,pIFunctionInstance,iProviderInstanceContext,pdwCount)	\
    ( (This)->lpVtbl -> GetCount(This,pIFunctionInstance,iProviderInstanceContext,pdwCount) ) 

#define IProviderProperties_GetAt(This,pIFunctionInstance,iProviderInstanceContext,dwIndex,pKey)	\
    ( (This)->lpVtbl -> GetAt(This,pIFunctionInstance,iProviderInstanceContext,dwIndex,pKey) ) 

#define IProviderProperties_GetValue(This,pIFunctionInstance,iProviderInstanceContext,Key,ppropVar)	\
    ( (This)->lpVtbl -> GetValue(This,pIFunctionInstance,iProviderInstanceContext,Key,ppropVar) ) 

#define IProviderProperties_SetValue(This,pIFunctionInstance,iProviderInstanceContext,Key,ppropVar)	\
    ( (This)->lpVtbl -> SetValue(This,pIFunctionInstance,iProviderInstanceContext,Key,ppropVar) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IProviderProperties_INTERFACE_DEFINED__ */


#ifndef __IProviderPublishing_INTERFACE_DEFINED__
#define __IProviderPublishing_INTERFACE_DEFINED__

/* interface IProviderPublishing */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IProviderPublishing;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("CD1B9A04-206C-4a05-A0C8-1635A21A2B7C")
    IProviderPublishing : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE CreateInstance( 
            /* [in] */ SystemVisibilityFlags enumVisibilityFlags,
            /* [unique][string][in] */ __RPC__in_opt_string const WCHAR *pszSubCategory,
            /* [string][in] */ __RPC__in_string const WCHAR *pszProviderInstanceIdentity,
            /* [out] */ __RPC__deref_out_opt IFunctionInstance **ppIFunctionInstance) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RemoveInstance( 
            /* [in] */ SystemVisibilityFlags enumVisibilityFlags,
            /* [unique][string][in] */ __RPC__in_opt_string const WCHAR *pszSubCategory,
            /* [string][in] */ __RPC__in_string const WCHAR *pszProviderInstanceIdentity) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IProviderPublishingVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IProviderPublishing * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IProviderPublishing * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IProviderPublishing * This);
        
        DECLSPEC_XFGVIRT(IProviderPublishing, CreateInstance)
        HRESULT ( STDMETHODCALLTYPE *CreateInstance )( 
            __RPC__in IProviderPublishing * This,
            /* [in] */ SystemVisibilityFlags enumVisibilityFlags,
            /* [unique][string][in] */ __RPC__in_opt_string const WCHAR *pszSubCategory,
            /* [string][in] */ __RPC__in_string const WCHAR *pszProviderInstanceIdentity,
            /* [out] */ __RPC__deref_out_opt IFunctionInstance **ppIFunctionInstance);
        
        DECLSPEC_XFGVIRT(IProviderPublishing, RemoveInstance)
        HRESULT ( STDMETHODCALLTYPE *RemoveInstance )( 
            __RPC__in IProviderPublishing * This,
            /* [in] */ SystemVisibilityFlags enumVisibilityFlags,
            /* [unique][string][in] */ __RPC__in_opt_string const WCHAR *pszSubCategory,
            /* [string][in] */ __RPC__in_string const WCHAR *pszProviderInstanceIdentity);
        
        END_INTERFACE
    } IProviderPublishingVtbl;

    interface IProviderPublishing
    {
        CONST_VTBL struct IProviderPublishingVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IProviderPublishing_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IProviderPublishing_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IProviderPublishing_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IProviderPublishing_CreateInstance(This,enumVisibilityFlags,pszSubCategory,pszProviderInstanceIdentity,ppIFunctionInstance)	\
    ( (This)->lpVtbl -> CreateInstance(This,enumVisibilityFlags,pszSubCategory,pszProviderInstanceIdentity,ppIFunctionInstance) ) 

#define IProviderPublishing_RemoveInstance(This,enumVisibilityFlags,pszSubCategory,pszProviderInstanceIdentity)	\
    ( (This)->lpVtbl -> RemoveInstance(This,enumVisibilityFlags,pszSubCategory,pszProviderInstanceIdentity) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IProviderPublishing_INTERFACE_DEFINED__ */


#ifndef __IFunctionDiscoveryProviderFactory_INTERFACE_DEFINED__
#define __IFunctionDiscoveryProviderFactory_INTERFACE_DEFINED__

/* interface IFunctionDiscoveryProviderFactory */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IFunctionDiscoveryProviderFactory;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("86443ff0-1ad5-4e68-a45a-40c2c329de3b")
    IFunctionDiscoveryProviderFactory : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE CreatePropertyStore( 
            /* [out] */ __RPC__deref_out_opt IPropertyStore **ppIPropertyStore) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreateInstance( 
            /* [unique][string][in] */ __RPC__in_opt_string const WCHAR *pszSubCategory,
            /* [string][in] */ __RPC__in_string const WCHAR *pszProviderInstanceIdentity,
            /* [in] */ INT_PTR iProviderInstanceContext,
            /* [in] */ __RPC__in_opt IPropertyStore *pIPropertyStore,
            /* [in] */ __RPC__in_opt IFunctionDiscoveryProvider *pIFunctionDiscoveryProvider,
            /* [out] */ __RPC__deref_out_opt IFunctionInstance **ppIFunctionInstance) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreateFunctionInstanceCollection( 
            /* [out] */ __RPC__deref_out_opt IFunctionInstanceCollection **ppIFunctionInstanceCollection) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IFunctionDiscoveryProviderFactoryVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IFunctionDiscoveryProviderFactory * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IFunctionDiscoveryProviderFactory * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IFunctionDiscoveryProviderFactory * This);
        
        DECLSPEC_XFGVIRT(IFunctionDiscoveryProviderFactory, CreatePropertyStore)
        HRESULT ( STDMETHODCALLTYPE *CreatePropertyStore )( 
            __RPC__in IFunctionDiscoveryProviderFactory * This,
            /* [out] */ __RPC__deref_out_opt IPropertyStore **ppIPropertyStore);
        
        DECLSPEC_XFGVIRT(IFunctionDiscoveryProviderFactory, CreateInstance)
        HRESULT ( STDMETHODCALLTYPE *CreateInstance )( 
            __RPC__in IFunctionDiscoveryProviderFactory * This,
            /* [unique][string][in] */ __RPC__in_opt_string const WCHAR *pszSubCategory,
            /* [string][in] */ __RPC__in_string const WCHAR *pszProviderInstanceIdentity,
            /* [in] */ INT_PTR iProviderInstanceContext,
            /* [in] */ __RPC__in_opt IPropertyStore *pIPropertyStore,
            /* [in] */ __RPC__in_opt IFunctionDiscoveryProvider *pIFunctionDiscoveryProvider,
            /* [out] */ __RPC__deref_out_opt IFunctionInstance **ppIFunctionInstance);
        
        DECLSPEC_XFGVIRT(IFunctionDiscoveryProviderFactory, CreateFunctionInstanceCollection)
        HRESULT ( STDMETHODCALLTYPE *CreateFunctionInstanceCollection )( 
            __RPC__in IFunctionDiscoveryProviderFactory * This,
            /* [out] */ __RPC__deref_out_opt IFunctionInstanceCollection **ppIFunctionInstanceCollection);
        
        END_INTERFACE
    } IFunctionDiscoveryProviderFactoryVtbl;

    interface IFunctionDiscoveryProviderFactory
    {
        CONST_VTBL struct IFunctionDiscoveryProviderFactoryVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IFunctionDiscoveryProviderFactory_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IFunctionDiscoveryProviderFactory_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IFunctionDiscoveryProviderFactory_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IFunctionDiscoveryProviderFactory_CreatePropertyStore(This,ppIPropertyStore)	\
    ( (This)->lpVtbl -> CreatePropertyStore(This,ppIPropertyStore) ) 

#define IFunctionDiscoveryProviderFactory_CreateInstance(This,pszSubCategory,pszProviderInstanceIdentity,iProviderInstanceContext,pIPropertyStore,pIFunctionDiscoveryProvider,ppIFunctionInstance)	\
    ( (This)->lpVtbl -> CreateInstance(This,pszSubCategory,pszProviderInstanceIdentity,iProviderInstanceContext,pIPropertyStore,pIFunctionDiscoveryProvider,ppIFunctionInstance) ) 

#define IFunctionDiscoveryProviderFactory_CreateFunctionInstanceCollection(This,ppIFunctionInstanceCollection)	\
    ( (This)->lpVtbl -> CreateFunctionInstanceCollection(This,ppIFunctionInstanceCollection) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IFunctionDiscoveryProviderFactory_INTERFACE_DEFINED__ */


#ifndef __IFunctionDiscoveryProviderQuery_INTERFACE_DEFINED__
#define __IFunctionDiscoveryProviderQuery_INTERFACE_DEFINED__

/* interface IFunctionDiscoveryProviderQuery */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IFunctionDiscoveryProviderQuery;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("6876ea98-baec-46db-bc20-75a76e267a3a")
    IFunctionDiscoveryProviderQuery : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE IsInstanceQuery( 
            /* [out] */ __RPC__out BOOL *pisInstanceQuery,
            /* [string][out] */ __RPC__deref_out_opt_string WCHAR **ppszConstraintValue) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE IsSubcategoryQuery( 
            /* [out] */ __RPC__out BOOL *pisSubcategoryQuery,
            /* [string][out] */ __RPC__deref_out_opt_string WCHAR **ppszConstraintValue) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetQueryConstraints( 
            /* [out] */ __RPC__deref_out_opt IProviderQueryConstraintCollection **ppIProviderQueryConstraints) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetPropertyConstraints( 
            /* [out] */ __RPC__deref_out_opt IProviderPropertyConstraintCollection **ppIProviderPropertyConstraints) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IFunctionDiscoveryProviderQueryVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IFunctionDiscoveryProviderQuery * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IFunctionDiscoveryProviderQuery * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IFunctionDiscoveryProviderQuery * This);
        
        DECLSPEC_XFGVIRT(IFunctionDiscoveryProviderQuery, IsInstanceQuery)
        HRESULT ( STDMETHODCALLTYPE *IsInstanceQuery )( 
            __RPC__in IFunctionDiscoveryProviderQuery * This,
            /* [out] */ __RPC__out BOOL *pisInstanceQuery,
            /* [string][out] */ __RPC__deref_out_opt_string WCHAR **ppszConstraintValue);
        
        DECLSPEC_XFGVIRT(IFunctionDiscoveryProviderQuery, IsSubcategoryQuery)
        HRESULT ( STDMETHODCALLTYPE *IsSubcategoryQuery )( 
            __RPC__in IFunctionDiscoveryProviderQuery * This,
            /* [out] */ __RPC__out BOOL *pisSubcategoryQuery,
            /* [string][out] */ __RPC__deref_out_opt_string WCHAR **ppszConstraintValue);
        
        DECLSPEC_XFGVIRT(IFunctionDiscoveryProviderQuery, GetQueryConstraints)
        HRESULT ( STDMETHODCALLTYPE *GetQueryConstraints )( 
            __RPC__in IFunctionDiscoveryProviderQuery * This,
            /* [out] */ __RPC__deref_out_opt IProviderQueryConstraintCollection **ppIProviderQueryConstraints);
        
        DECLSPEC_XFGVIRT(IFunctionDiscoveryProviderQuery, GetPropertyConstraints)
        HRESULT ( STDMETHODCALLTYPE *GetPropertyConstraints )( 
            __RPC__in IFunctionDiscoveryProviderQuery * This,
            /* [out] */ __RPC__deref_out_opt IProviderPropertyConstraintCollection **ppIProviderPropertyConstraints);
        
        END_INTERFACE
    } IFunctionDiscoveryProviderQueryVtbl;

    interface IFunctionDiscoveryProviderQuery
    {
        CONST_VTBL struct IFunctionDiscoveryProviderQueryVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IFunctionDiscoveryProviderQuery_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IFunctionDiscoveryProviderQuery_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IFunctionDiscoveryProviderQuery_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IFunctionDiscoveryProviderQuery_IsInstanceQuery(This,pisInstanceQuery,ppszConstraintValue)	\
    ( (This)->lpVtbl -> IsInstanceQuery(This,pisInstanceQuery,ppszConstraintValue) ) 

#define IFunctionDiscoveryProviderQuery_IsSubcategoryQuery(This,pisSubcategoryQuery,ppszConstraintValue)	\
    ( (This)->lpVtbl -> IsSubcategoryQuery(This,pisSubcategoryQuery,ppszConstraintValue) ) 

#define IFunctionDiscoveryProviderQuery_GetQueryConstraints(This,ppIProviderQueryConstraints)	\
    ( (This)->lpVtbl -> GetQueryConstraints(This,ppIProviderQueryConstraints) ) 

#define IFunctionDiscoveryProviderQuery_GetPropertyConstraints(This,ppIProviderPropertyConstraints)	\
    ( (This)->lpVtbl -> GetPropertyConstraints(This,ppIProviderPropertyConstraints) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IFunctionDiscoveryProviderQuery_INTERFACE_DEFINED__ */


#ifndef __IProviderQueryConstraintCollection_INTERFACE_DEFINED__
#define __IProviderQueryConstraintCollection_INTERFACE_DEFINED__

/* interface IProviderQueryConstraintCollection */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IProviderQueryConstraintCollection;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("9c243e11-3261-4bcd-b922-84a873d460ae")
    IProviderQueryConstraintCollection : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetCount( 
            /* [out] */ __RPC__out DWORD *pdwCount) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Get( 
            /* [string][in] */ __RPC__in_string const WCHAR *pszConstraintName,
            /* [string][out] */ __RPC__deref_out_opt_string WCHAR **ppszConstraintValue) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Item( 
            /* [in] */ DWORD dwIndex,
            /* [string][out] */ __RPC__deref_out_opt_string WCHAR **ppszConstraintName,
            /* [string][out] */ __RPC__deref_out_opt_string WCHAR **ppszConstraintValue) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Next( 
            /* [string][out] */ __RPC__deref_out_opt_string WCHAR **ppszConstraintName,
            /* [string][out] */ __RPC__deref_out_opt_string WCHAR **ppszConstraintValue) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Skip( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Reset( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IProviderQueryConstraintCollectionVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IProviderQueryConstraintCollection * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IProviderQueryConstraintCollection * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IProviderQueryConstraintCollection * This);
        
        DECLSPEC_XFGVIRT(IProviderQueryConstraintCollection, GetCount)
        HRESULT ( STDMETHODCALLTYPE *GetCount )( 
            __RPC__in IProviderQueryConstraintCollection * This,
            /* [out] */ __RPC__out DWORD *pdwCount);
        
        DECLSPEC_XFGVIRT(IProviderQueryConstraintCollection, Get)
        HRESULT ( STDMETHODCALLTYPE *Get )( 
            __RPC__in IProviderQueryConstraintCollection * This,
            /* [string][in] */ __RPC__in_string const WCHAR *pszConstraintName,
            /* [string][out] */ __RPC__deref_out_opt_string WCHAR **ppszConstraintValue);
        
        DECLSPEC_XFGVIRT(IProviderQueryConstraintCollection, Item)
        HRESULT ( STDMETHODCALLTYPE *Item )( 
            __RPC__in IProviderQueryConstraintCollection * This,
            /* [in] */ DWORD dwIndex,
            /* [string][out] */ __RPC__deref_out_opt_string WCHAR **ppszConstraintName,
            /* [string][out] */ __RPC__deref_out_opt_string WCHAR **ppszConstraintValue);
        
        DECLSPEC_XFGVIRT(IProviderQueryConstraintCollection, Next)
        HRESULT ( STDMETHODCALLTYPE *Next )( 
            __RPC__in IProviderQueryConstraintCollection * This,
            /* [string][out] */ __RPC__deref_out_opt_string WCHAR **ppszConstraintName,
            /* [string][out] */ __RPC__deref_out_opt_string WCHAR **ppszConstraintValue);
        
        DECLSPEC_XFGVIRT(IProviderQueryConstraintCollection, Skip)
        HRESULT ( STDMETHODCALLTYPE *Skip )( 
            __RPC__in IProviderQueryConstraintCollection * This);
        
        DECLSPEC_XFGVIRT(IProviderQueryConstraintCollection, Reset)
        HRESULT ( STDMETHODCALLTYPE *Reset )( 
            __RPC__in IProviderQueryConstraintCollection * This);
        
        END_INTERFACE
    } IProviderQueryConstraintCollectionVtbl;

    interface IProviderQueryConstraintCollection
    {
        CONST_VTBL struct IProviderQueryConstraintCollectionVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IProviderQueryConstraintCollection_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IProviderQueryConstraintCollection_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IProviderQueryConstraintCollection_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IProviderQueryConstraintCollection_GetCount(This,pdwCount)	\
    ( (This)->lpVtbl -> GetCount(This,pdwCount) ) 

#define IProviderQueryConstraintCollection_Get(This,pszConstraintName,ppszConstraintValue)	\
    ( (This)->lpVtbl -> Get(This,pszConstraintName,ppszConstraintValue) ) 

#define IProviderQueryConstraintCollection_Item(This,dwIndex,ppszConstraintName,ppszConstraintValue)	\
    ( (This)->lpVtbl -> Item(This,dwIndex,ppszConstraintName,ppszConstraintValue) ) 

#define IProviderQueryConstraintCollection_Next(This,ppszConstraintName,ppszConstraintValue)	\
    ( (This)->lpVtbl -> Next(This,ppszConstraintName,ppszConstraintValue) ) 

#define IProviderQueryConstraintCollection_Skip(This)	\
    ( (This)->lpVtbl -> Skip(This) ) 

#define IProviderQueryConstraintCollection_Reset(This)	\
    ( (This)->lpVtbl -> Reset(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IProviderQueryConstraintCollection_INTERFACE_DEFINED__ */


#ifndef __IProviderPropertyConstraintCollection_INTERFACE_DEFINED__
#define __IProviderPropertyConstraintCollection_INTERFACE_DEFINED__

/* interface IProviderPropertyConstraintCollection */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IProviderPropertyConstraintCollection;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("f4fae42f-5778-4a13-8540-b5fd8c1398dd")
    IProviderPropertyConstraintCollection : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetCount( 
            /* [out] */ __RPC__out DWORD *pdwCount) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Get( 
            /* [in] */ __RPC__in REFPROPERTYKEY Key,
            /* [out] */ __RPC__out PROPVARIANT *pPropVar,
            /* [out] */ __RPC__out DWORD *pdwPropertyConstraint) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Item( 
            /* [in] */ DWORD dwIndex,
            /* [out] */ __RPC__out PROPERTYKEY *pKey,
            /* [out] */ __RPC__out PROPVARIANT *pPropVar,
            /* [out] */ __RPC__out DWORD *pdwPropertyConstraint) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Next( 
            /* [out] */ __RPC__out PROPERTYKEY *pKey,
            /* [out] */ __RPC__out PROPVARIANT *pPropVar,
            /* [out] */ __RPC__out DWORD *pdwPropertyConstraint) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Skip( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Reset( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IProviderPropertyConstraintCollectionVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IProviderPropertyConstraintCollection * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IProviderPropertyConstraintCollection * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IProviderPropertyConstraintCollection * This);
        
        DECLSPEC_XFGVIRT(IProviderPropertyConstraintCollection, GetCount)
        HRESULT ( STDMETHODCALLTYPE *GetCount )( 
            __RPC__in IProviderPropertyConstraintCollection * This,
            /* [out] */ __RPC__out DWORD *pdwCount);
        
        DECLSPEC_XFGVIRT(IProviderPropertyConstraintCollection, Get)
        HRESULT ( STDMETHODCALLTYPE *Get )( 
            __RPC__in IProviderPropertyConstraintCollection * This,
            /* [in] */ __RPC__in REFPROPERTYKEY Key,
            /* [out] */ __RPC__out PROPVARIANT *pPropVar,
            /* [out] */ __RPC__out DWORD *pdwPropertyConstraint);
        
        DECLSPEC_XFGVIRT(IProviderPropertyConstraintCollection, Item)
        HRESULT ( STDMETHODCALLTYPE *Item )( 
            __RPC__in IProviderPropertyConstraintCollection * This,
            /* [in] */ DWORD dwIndex,
            /* [out] */ __RPC__out PROPERTYKEY *pKey,
            /* [out] */ __RPC__out PROPVARIANT *pPropVar,
            /* [out] */ __RPC__out DWORD *pdwPropertyConstraint);
        
        DECLSPEC_XFGVIRT(IProviderPropertyConstraintCollection, Next)
        HRESULT ( STDMETHODCALLTYPE *Next )( 
            __RPC__in IProviderPropertyConstraintCollection * This,
            /* [out] */ __RPC__out PROPERTYKEY *pKey,
            /* [out] */ __RPC__out PROPVARIANT *pPropVar,
            /* [out] */ __RPC__out DWORD *pdwPropertyConstraint);
        
        DECLSPEC_XFGVIRT(IProviderPropertyConstraintCollection, Skip)
        HRESULT ( STDMETHODCALLTYPE *Skip )( 
            __RPC__in IProviderPropertyConstraintCollection * This);
        
        DECLSPEC_XFGVIRT(IProviderPropertyConstraintCollection, Reset)
        HRESULT ( STDMETHODCALLTYPE *Reset )( 
            __RPC__in IProviderPropertyConstraintCollection * This);
        
        END_INTERFACE
    } IProviderPropertyConstraintCollectionVtbl;

    interface IProviderPropertyConstraintCollection
    {
        CONST_VTBL struct IProviderPropertyConstraintCollectionVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IProviderPropertyConstraintCollection_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IProviderPropertyConstraintCollection_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IProviderPropertyConstraintCollection_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IProviderPropertyConstraintCollection_GetCount(This,pdwCount)	\
    ( (This)->lpVtbl -> GetCount(This,pdwCount) ) 

#define IProviderPropertyConstraintCollection_Get(This,Key,pPropVar,pdwPropertyConstraint)	\
    ( (This)->lpVtbl -> Get(This,Key,pPropVar,pdwPropertyConstraint) ) 

#define IProviderPropertyConstraintCollection_Item(This,dwIndex,pKey,pPropVar,pdwPropertyConstraint)	\
    ( (This)->lpVtbl -> Item(This,dwIndex,pKey,pPropVar,pdwPropertyConstraint) ) 

#define IProviderPropertyConstraintCollection_Next(This,pKey,pPropVar,pdwPropertyConstraint)	\
    ( (This)->lpVtbl -> Next(This,pKey,pPropVar,pdwPropertyConstraint) ) 

#define IProviderPropertyConstraintCollection_Skip(This)	\
    ( (This)->lpVtbl -> Skip(This) ) 

#define IProviderPropertyConstraintCollection_Reset(This)	\
    ( (This)->lpVtbl -> Reset(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IProviderPropertyConstraintCollection_INTERFACE_DEFINED__ */


#ifndef __IFunctionDiscoveryServiceProvider_INTERFACE_DEFINED__
#define __IFunctionDiscoveryServiceProvider_INTERFACE_DEFINED__

/* interface IFunctionDiscoveryServiceProvider */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IFunctionDiscoveryServiceProvider;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("4c81ed02-1b04-43f2-a451-69966cbcd1c2")
    IFunctionDiscoveryServiceProvider : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Initialize( 
            /* [in] */ __RPC__in_opt IFunctionInstance *pIFunctionInstance,
            /* [in] */ __RPC__in REFIID riid,
            /* [retval][iid_is][out] */ __RPC__deref_out_opt void **ppv) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IFunctionDiscoveryServiceProviderVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IFunctionDiscoveryServiceProvider * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IFunctionDiscoveryServiceProvider * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IFunctionDiscoveryServiceProvider * This);
        
        DECLSPEC_XFGVIRT(IFunctionDiscoveryServiceProvider, Initialize)
        HRESULT ( STDMETHODCALLTYPE *Initialize )( 
            __RPC__in IFunctionDiscoveryServiceProvider * This,
            /* [in] */ __RPC__in_opt IFunctionInstance *pIFunctionInstance,
            /* [in] */ __RPC__in REFIID riid,
            /* [retval][iid_is][out] */ __RPC__deref_out_opt void **ppv);
        
        END_INTERFACE
    } IFunctionDiscoveryServiceProviderVtbl;

    interface IFunctionDiscoveryServiceProvider
    {
        CONST_VTBL struct IFunctionDiscoveryServiceProviderVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IFunctionDiscoveryServiceProvider_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IFunctionDiscoveryServiceProvider_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IFunctionDiscoveryServiceProvider_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IFunctionDiscoveryServiceProvider_Initialize(This,pIFunctionInstance,riid,ppv)	\
    ( (This)->lpVtbl -> Initialize(This,pIFunctionInstance,riid,ppv) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IFunctionDiscoveryServiceProvider_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_functiondiscoveryprovider_0000_0008 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_functiondiscoveryprovider_0000_0008_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_functiondiscoveryprovider_0000_0008_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

unsigned long             __RPC_USER  BSTR_UserSize(     __RPC__in unsigned long *, unsigned long            , __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserMarshal(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out BSTR * ); 
void                      __RPC_USER  BSTR_UserFree(     __RPC__in unsigned long *, __RPC__in BSTR * ); 

unsigned long             __RPC_USER  LPSAFEARRAY_UserSize(     __RPC__in unsigned long *, unsigned long            , __RPC__in LPSAFEARRAY * ); 
unsigned char * __RPC_USER  LPSAFEARRAY_UserMarshal(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in LPSAFEARRAY * ); 
unsigned char * __RPC_USER  LPSAFEARRAY_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out LPSAFEARRAY * ); 
void                      __RPC_USER  LPSAFEARRAY_UserFree(     __RPC__in unsigned long *, __RPC__in LPSAFEARRAY * ); 

unsigned long             __RPC_USER  BSTR_UserSize64(     __RPC__in unsigned long *, unsigned long            , __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserMarshal64(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out BSTR * ); 
void                      __RPC_USER  BSTR_UserFree64(     __RPC__in unsigned long *, __RPC__in BSTR * ); 

unsigned long             __RPC_USER  LPSAFEARRAY_UserSize64(     __RPC__in unsigned long *, unsigned long            , __RPC__in LPSAFEARRAY * ); 
unsigned char * __RPC_USER  LPSAFEARRAY_UserMarshal64(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in LPSAFEARRAY * ); 
unsigned char * __RPC_USER  LPSAFEARRAY_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out LPSAFEARRAY * ); 
void                      __RPC_USER  LPSAFEARRAY_UserFree64(     __RPC__in unsigned long *, __RPC__in LPSAFEARRAY * ); 

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


