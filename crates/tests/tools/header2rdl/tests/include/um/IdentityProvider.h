

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

#ifndef __identityprovider_h__
#define __identityprovider_h__

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

#ifndef __IIdentityAdvise_FWD_DEFINED__
#define __IIdentityAdvise_FWD_DEFINED__
typedef interface IIdentityAdvise IIdentityAdvise;

#endif 	/* __IIdentityAdvise_FWD_DEFINED__ */


#ifndef __AsyncIIdentityAdvise_FWD_DEFINED__
#define __AsyncIIdentityAdvise_FWD_DEFINED__
typedef interface AsyncIIdentityAdvise AsyncIIdentityAdvise;

#endif 	/* __AsyncIIdentityAdvise_FWD_DEFINED__ */


#ifndef __IIdentityProvider_FWD_DEFINED__
#define __IIdentityProvider_FWD_DEFINED__
typedef interface IIdentityProvider IIdentityProvider;

#endif 	/* __IIdentityProvider_FWD_DEFINED__ */


#ifndef __AsyncIIdentityProvider_FWD_DEFINED__
#define __AsyncIIdentityProvider_FWD_DEFINED__
typedef interface AsyncIIdentityProvider AsyncIIdentityProvider;

#endif 	/* __AsyncIIdentityProvider_FWD_DEFINED__ */


#ifndef __IAssociatedIdentityProvider_FWD_DEFINED__
#define __IAssociatedIdentityProvider_FWD_DEFINED__
typedef interface IAssociatedIdentityProvider IAssociatedIdentityProvider;

#endif 	/* __IAssociatedIdentityProvider_FWD_DEFINED__ */


#ifndef __AsyncIAssociatedIdentityProvider_FWD_DEFINED__
#define __AsyncIAssociatedIdentityProvider_FWD_DEFINED__
typedef interface AsyncIAssociatedIdentityProvider AsyncIAssociatedIdentityProvider;

#endif 	/* __AsyncIAssociatedIdentityProvider_FWD_DEFINED__ */


#ifndef __IConnectedIdentityProvider_FWD_DEFINED__
#define __IConnectedIdentityProvider_FWD_DEFINED__
typedef interface IConnectedIdentityProvider IConnectedIdentityProvider;

#endif 	/* __IConnectedIdentityProvider_FWD_DEFINED__ */


#ifndef __AsyncIConnectedIdentityProvider_FWD_DEFINED__
#define __AsyncIConnectedIdentityProvider_FWD_DEFINED__
typedef interface AsyncIConnectedIdentityProvider AsyncIConnectedIdentityProvider;

#endif 	/* __AsyncIConnectedIdentityProvider_FWD_DEFINED__ */


#ifndef __IIdentityAuthentication_FWD_DEFINED__
#define __IIdentityAuthentication_FWD_DEFINED__
typedef interface IIdentityAuthentication IIdentityAuthentication;

#endif 	/* __IIdentityAuthentication_FWD_DEFINED__ */


#ifndef __AsyncIIdentityAuthentication_FWD_DEFINED__
#define __AsyncIIdentityAuthentication_FWD_DEFINED__
typedef interface AsyncIIdentityAuthentication AsyncIIdentityAuthentication;

#endif 	/* __AsyncIIdentityAuthentication_FWD_DEFINED__ */


/* header files for imported files */
#include "oaidl.h"
#include "ocidl.h"
#include "propsys.h"
#include "IdentityCommon.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_identityprovider_0000_0000 */
/* [local] */ 

#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
#if 0
#endif // 0

enum tag_IdentityUpdateEvent
    {
        IDENTITY_ASSOCIATED	= 0x1,
        IDENTITY_DISASSOCIATED	= 0x2,
        IDENTITY_CREATED	= 0x4,
        IDENTITY_IMPORTED	= 0x8,
        IDENTITY_DELETED	= 0x10,
        IDENTITY_PROPCHANGED	= 0x20,
        IDENTITY_CONNECTED	= 0x40,
        IDENTITY_DISCONNECTED	= 0x80
    } ;
#define      IDENTITY_KEYWORD_ASSOCIATED L"associated"
#define      IDENTITY_KEYWORD_LOCAL      L"local"
#define      IDENTITY_KEYWORD_HOMEGROUP  L"homegroup"
#define      IDENTITY_KEYWORD_CONNECTED  L"connected"
DEFINE_GUID(OID_OAssociatedIdentityProviderObject, 0x98c5a3dd, 0xdb68, 0x4f1a, 0x8d, 0x2b, 0x90, 0x79, 0xcd, 0xfe, 0xaf, 0x61);


extern RPC_IF_HANDLE __MIDL_itf_identityprovider_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_identityprovider_0000_0000_v0_0_s_ifspec;

#ifndef __IIdentityAdvise_INTERFACE_DEFINED__
#define __IIdentityAdvise_INTERFACE_DEFINED__

/* interface IIdentityAdvise */
/* [unique][async_uuid][uuid][object] */ 


EXTERN_C const IID IID_IIdentityAdvise;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("4e982fed-d14b-440c-b8d6-bb386453d386")
    IIdentityAdvise : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE IdentityUpdated( 
            /* [in] */ DWORD dwIdentityUpdateEvents,
            /* [in] */ __RPC__in LPCWSTR lpszUniqueID) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IIdentityAdviseVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IIdentityAdvise * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IIdentityAdvise * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IIdentityAdvise * This);
        
        DECLSPEC_XFGVIRT(IIdentityAdvise, IdentityUpdated)
        HRESULT ( STDMETHODCALLTYPE *IdentityUpdated )( 
            __RPC__in IIdentityAdvise * This,
            /* [in] */ DWORD dwIdentityUpdateEvents,
            /* [in] */ __RPC__in LPCWSTR lpszUniqueID);
        
        END_INTERFACE
    } IIdentityAdviseVtbl;

    interface IIdentityAdvise
    {
        CONST_VTBL struct IIdentityAdviseVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IIdentityAdvise_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IIdentityAdvise_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IIdentityAdvise_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IIdentityAdvise_IdentityUpdated(This,dwIdentityUpdateEvents,lpszUniqueID)	\
    ( (This)->lpVtbl -> IdentityUpdated(This,dwIdentityUpdateEvents,lpszUniqueID) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IIdentityAdvise_INTERFACE_DEFINED__ */


#ifndef __AsyncIIdentityAdvise_INTERFACE_DEFINED__
#define __AsyncIIdentityAdvise_INTERFACE_DEFINED__

/* interface AsyncIIdentityAdvise */
/* [uuid][unique][object] */ 


EXTERN_C const IID IID_AsyncIIdentityAdvise;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("3ab4c8da-d038-4830-8dd9-3253c55a127f")
    AsyncIIdentityAdvise : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Begin_IdentityUpdated( 
            /* [in] */ DWORD dwIdentityUpdateEvents,
            /* [in] */ __RPC__in LPCWSTR lpszUniqueID) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Finish_IdentityUpdated( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct AsyncIIdentityAdviseVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in AsyncIIdentityAdvise * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in AsyncIIdentityAdvise * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in AsyncIIdentityAdvise * This);
        
        DECLSPEC_XFGVIRT(AsyncIIdentityAdvise, Begin_IdentityUpdated)
        HRESULT ( STDMETHODCALLTYPE *Begin_IdentityUpdated )( 
            __RPC__in AsyncIIdentityAdvise * This,
            /* [in] */ DWORD dwIdentityUpdateEvents,
            /* [in] */ __RPC__in LPCWSTR lpszUniqueID);
        
        DECLSPEC_XFGVIRT(AsyncIIdentityAdvise, Finish_IdentityUpdated)
        HRESULT ( STDMETHODCALLTYPE *Finish_IdentityUpdated )( 
            __RPC__in AsyncIIdentityAdvise * This);
        
        END_INTERFACE
    } AsyncIIdentityAdviseVtbl;

    interface AsyncIIdentityAdvise
    {
        CONST_VTBL struct AsyncIIdentityAdviseVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define AsyncIIdentityAdvise_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define AsyncIIdentityAdvise_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define AsyncIIdentityAdvise_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define AsyncIIdentityAdvise_Begin_IdentityUpdated(This,dwIdentityUpdateEvents,lpszUniqueID)	\
    ( (This)->lpVtbl -> Begin_IdentityUpdated(This,dwIdentityUpdateEvents,lpszUniqueID) ) 

#define AsyncIIdentityAdvise_Finish_IdentityUpdated(This)	\
    ( (This)->lpVtbl -> Finish_IdentityUpdated(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __AsyncIIdentityAdvise_INTERFACE_DEFINED__ */


#ifndef __IIdentityProvider_INTERFACE_DEFINED__
#define __IIdentityProvider_INTERFACE_DEFINED__

/* interface IIdentityProvider */
/* [unique][async_uuid][uuid][object] */ 


EXTERN_C const IID IID_IIdentityProvider;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("0d1b9e0c-e8ba-4f55-a81b-bce934b948f5")
    IIdentityProvider : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetIdentityEnum( 
            /* [in] */ const IDENTITY_TYPE eIdentityType,
            /* [unique][in] */ __RPC__in_opt const PROPERTYKEY *pFilterkey,
            /* [unique][in] */ __RPC__in_opt const PROPVARIANT *pFilterPropVarValue,
            /* [out] */ __RPC__deref_out_opt IEnumUnknown **ppIdentityEnum) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Create( 
            /* [in] */ __RPC__in LPCWSTR lpszUserName,
            /* [out] */ __RPC__deref_out_opt IPropertyStore **ppPropertyStore,
            /* [in] */ __RPC__in const PROPVARIANT *pKeywordsToAdd) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Import( 
            /* [in] */ __RPC__in_opt IPropertyStore *pPropertyStore) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Delete( 
            /* [in] */ __RPC__in LPCWSTR lpszUniqueID,
            /* [in] */ __RPC__in const PROPVARIANT *pKeywordsToDelete) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE FindByUniqueID( 
            /* [in] */ __RPC__in LPCWSTR lpszUniqueID,
            /* [out] */ __RPC__deref_out_opt IPropertyStore **ppPropertyStore) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetProviderPropertyStore( 
            /* [out] */ __RPC__deref_out_opt IPropertyStore **ppPropertyStore) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Advise( 
            /* [in] */ __RPC__in_opt IIdentityAdvise *pIdentityAdvise,
            /* [in] */ DWORD dwIdentityUpdateEvents,
            /* [out] */ __RPC__out DWORD *pdwCookie) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE UnAdvise( 
            /* [in] */ const DWORD dwCookie) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IIdentityProviderVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IIdentityProvider * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IIdentityProvider * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IIdentityProvider * This);
        
        DECLSPEC_XFGVIRT(IIdentityProvider, GetIdentityEnum)
        HRESULT ( STDMETHODCALLTYPE *GetIdentityEnum )( 
            __RPC__in IIdentityProvider * This,
            /* [in] */ const IDENTITY_TYPE eIdentityType,
            /* [unique][in] */ __RPC__in_opt const PROPERTYKEY *pFilterkey,
            /* [unique][in] */ __RPC__in_opt const PROPVARIANT *pFilterPropVarValue,
            /* [out] */ __RPC__deref_out_opt IEnumUnknown **ppIdentityEnum);
        
        DECLSPEC_XFGVIRT(IIdentityProvider, Create)
        HRESULT ( STDMETHODCALLTYPE *Create )( 
            __RPC__in IIdentityProvider * This,
            /* [in] */ __RPC__in LPCWSTR lpszUserName,
            /* [out] */ __RPC__deref_out_opt IPropertyStore **ppPropertyStore,
            /* [in] */ __RPC__in const PROPVARIANT *pKeywordsToAdd);
        
        DECLSPEC_XFGVIRT(IIdentityProvider, Import)
        HRESULT ( STDMETHODCALLTYPE *Import )( 
            __RPC__in IIdentityProvider * This,
            /* [in] */ __RPC__in_opt IPropertyStore *pPropertyStore);
        
        DECLSPEC_XFGVIRT(IIdentityProvider, Delete)
        HRESULT ( STDMETHODCALLTYPE *Delete )( 
            __RPC__in IIdentityProvider * This,
            /* [in] */ __RPC__in LPCWSTR lpszUniqueID,
            /* [in] */ __RPC__in const PROPVARIANT *pKeywordsToDelete);
        
        DECLSPEC_XFGVIRT(IIdentityProvider, FindByUniqueID)
        HRESULT ( STDMETHODCALLTYPE *FindByUniqueID )( 
            __RPC__in IIdentityProvider * This,
            /* [in] */ __RPC__in LPCWSTR lpszUniqueID,
            /* [out] */ __RPC__deref_out_opt IPropertyStore **ppPropertyStore);
        
        DECLSPEC_XFGVIRT(IIdentityProvider, GetProviderPropertyStore)
        HRESULT ( STDMETHODCALLTYPE *GetProviderPropertyStore )( 
            __RPC__in IIdentityProvider * This,
            /* [out] */ __RPC__deref_out_opt IPropertyStore **ppPropertyStore);
        
        DECLSPEC_XFGVIRT(IIdentityProvider, Advise)
        HRESULT ( STDMETHODCALLTYPE *Advise )( 
            __RPC__in IIdentityProvider * This,
            /* [in] */ __RPC__in_opt IIdentityAdvise *pIdentityAdvise,
            /* [in] */ DWORD dwIdentityUpdateEvents,
            /* [out] */ __RPC__out DWORD *pdwCookie);
        
        DECLSPEC_XFGVIRT(IIdentityProvider, UnAdvise)
        HRESULT ( STDMETHODCALLTYPE *UnAdvise )( 
            __RPC__in IIdentityProvider * This,
            /* [in] */ const DWORD dwCookie);
        
        END_INTERFACE
    } IIdentityProviderVtbl;

    interface IIdentityProvider
    {
        CONST_VTBL struct IIdentityProviderVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IIdentityProvider_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IIdentityProvider_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IIdentityProvider_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IIdentityProvider_GetIdentityEnum(This,eIdentityType,pFilterkey,pFilterPropVarValue,ppIdentityEnum)	\
    ( (This)->lpVtbl -> GetIdentityEnum(This,eIdentityType,pFilterkey,pFilterPropVarValue,ppIdentityEnum) ) 

#define IIdentityProvider_Create(This,lpszUserName,ppPropertyStore,pKeywordsToAdd)	\
    ( (This)->lpVtbl -> Create(This,lpszUserName,ppPropertyStore,pKeywordsToAdd) ) 

#define IIdentityProvider_Import(This,pPropertyStore)	\
    ( (This)->lpVtbl -> Import(This,pPropertyStore) ) 

#define IIdentityProvider_Delete(This,lpszUniqueID,pKeywordsToDelete)	\
    ( (This)->lpVtbl -> Delete(This,lpszUniqueID,pKeywordsToDelete) ) 

#define IIdentityProvider_FindByUniqueID(This,lpszUniqueID,ppPropertyStore)	\
    ( (This)->lpVtbl -> FindByUniqueID(This,lpszUniqueID,ppPropertyStore) ) 

#define IIdentityProvider_GetProviderPropertyStore(This,ppPropertyStore)	\
    ( (This)->lpVtbl -> GetProviderPropertyStore(This,ppPropertyStore) ) 

#define IIdentityProvider_Advise(This,pIdentityAdvise,dwIdentityUpdateEvents,pdwCookie)	\
    ( (This)->lpVtbl -> Advise(This,pIdentityAdvise,dwIdentityUpdateEvents,pdwCookie) ) 

#define IIdentityProvider_UnAdvise(This,dwCookie)	\
    ( (This)->lpVtbl -> UnAdvise(This,dwCookie) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IIdentityProvider_INTERFACE_DEFINED__ */


#ifndef __AsyncIIdentityProvider_INTERFACE_DEFINED__
#define __AsyncIIdentityProvider_INTERFACE_DEFINED__

/* interface AsyncIIdentityProvider */
/* [uuid][unique][object] */ 


EXTERN_C const IID IID_AsyncIIdentityProvider;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("c6fc9901-c433-4646-8f48-4e4687aae2a0")
    AsyncIIdentityProvider : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Begin_GetIdentityEnum( 
            /* [in] */ const IDENTITY_TYPE eIdentityType,
            /* [unique][in] */ __RPC__in_opt const PROPERTYKEY *pFilterkey,
            /* [unique][in] */ __RPC__in_opt const PROPVARIANT *pFilterPropVarValue) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Finish_GetIdentityEnum( 
            /* [out] */ __RPC__deref_out_opt IEnumUnknown **ppIdentityEnum) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Begin_Create( 
            /* [in] */ __RPC__in LPCWSTR lpszUserName,
            /* [in] */ __RPC__in const PROPVARIANT *pKeywordsToAdd) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Finish_Create( 
            /* [out] */ __RPC__deref_out_opt IPropertyStore **ppPropertyStore) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Begin_Import( 
            /* [in] */ __RPC__in_opt IPropertyStore *pPropertyStore) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Finish_Import( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Begin_Delete( 
            /* [in] */ __RPC__in LPCWSTR lpszUniqueID,
            /* [in] */ __RPC__in const PROPVARIANT *pKeywordsToDelete) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Finish_Delete( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Begin_FindByUniqueID( 
            /* [in] */ __RPC__in LPCWSTR lpszUniqueID) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Finish_FindByUniqueID( 
            /* [out] */ __RPC__deref_out_opt IPropertyStore **ppPropertyStore) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Begin_GetProviderPropertyStore( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Finish_GetProviderPropertyStore( 
            /* [out] */ __RPC__deref_out_opt IPropertyStore **ppPropertyStore) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Begin_Advise( 
            /* [in] */ __RPC__in_opt IIdentityAdvise *pIdentityAdvise,
            /* [in] */ DWORD dwIdentityUpdateEvents) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Finish_Advise( 
            /* [out] */ __RPC__out DWORD *pdwCookie) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Begin_UnAdvise( 
            /* [in] */ const DWORD dwCookie) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Finish_UnAdvise( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct AsyncIIdentityProviderVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in AsyncIIdentityProvider * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in AsyncIIdentityProvider * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in AsyncIIdentityProvider * This);
        
        DECLSPEC_XFGVIRT(AsyncIIdentityProvider, Begin_GetIdentityEnum)
        HRESULT ( STDMETHODCALLTYPE *Begin_GetIdentityEnum )( 
            __RPC__in AsyncIIdentityProvider * This,
            /* [in] */ const IDENTITY_TYPE eIdentityType,
            /* [unique][in] */ __RPC__in_opt const PROPERTYKEY *pFilterkey,
            /* [unique][in] */ __RPC__in_opt const PROPVARIANT *pFilterPropVarValue);
        
        DECLSPEC_XFGVIRT(AsyncIIdentityProvider, Finish_GetIdentityEnum)
        HRESULT ( STDMETHODCALLTYPE *Finish_GetIdentityEnum )( 
            __RPC__in AsyncIIdentityProvider * This,
            /* [out] */ __RPC__deref_out_opt IEnumUnknown **ppIdentityEnum);
        
        DECLSPEC_XFGVIRT(AsyncIIdentityProvider, Begin_Create)
        HRESULT ( STDMETHODCALLTYPE *Begin_Create )( 
            __RPC__in AsyncIIdentityProvider * This,
            /* [in] */ __RPC__in LPCWSTR lpszUserName,
            /* [in] */ __RPC__in const PROPVARIANT *pKeywordsToAdd);
        
        DECLSPEC_XFGVIRT(AsyncIIdentityProvider, Finish_Create)
        HRESULT ( STDMETHODCALLTYPE *Finish_Create )( 
            __RPC__in AsyncIIdentityProvider * This,
            /* [out] */ __RPC__deref_out_opt IPropertyStore **ppPropertyStore);
        
        DECLSPEC_XFGVIRT(AsyncIIdentityProvider, Begin_Import)
        HRESULT ( STDMETHODCALLTYPE *Begin_Import )( 
            __RPC__in AsyncIIdentityProvider * This,
            /* [in] */ __RPC__in_opt IPropertyStore *pPropertyStore);
        
        DECLSPEC_XFGVIRT(AsyncIIdentityProvider, Finish_Import)
        HRESULT ( STDMETHODCALLTYPE *Finish_Import )( 
            __RPC__in AsyncIIdentityProvider * This);
        
        DECLSPEC_XFGVIRT(AsyncIIdentityProvider, Begin_Delete)
        HRESULT ( STDMETHODCALLTYPE *Begin_Delete )( 
            __RPC__in AsyncIIdentityProvider * This,
            /* [in] */ __RPC__in LPCWSTR lpszUniqueID,
            /* [in] */ __RPC__in const PROPVARIANT *pKeywordsToDelete);
        
        DECLSPEC_XFGVIRT(AsyncIIdentityProvider, Finish_Delete)
        HRESULT ( STDMETHODCALLTYPE *Finish_Delete )( 
            __RPC__in AsyncIIdentityProvider * This);
        
        DECLSPEC_XFGVIRT(AsyncIIdentityProvider, Begin_FindByUniqueID)
        HRESULT ( STDMETHODCALLTYPE *Begin_FindByUniqueID )( 
            __RPC__in AsyncIIdentityProvider * This,
            /* [in] */ __RPC__in LPCWSTR lpszUniqueID);
        
        DECLSPEC_XFGVIRT(AsyncIIdentityProvider, Finish_FindByUniqueID)
        HRESULT ( STDMETHODCALLTYPE *Finish_FindByUniqueID )( 
            __RPC__in AsyncIIdentityProvider * This,
            /* [out] */ __RPC__deref_out_opt IPropertyStore **ppPropertyStore);
        
        DECLSPEC_XFGVIRT(AsyncIIdentityProvider, Begin_GetProviderPropertyStore)
        HRESULT ( STDMETHODCALLTYPE *Begin_GetProviderPropertyStore )( 
            __RPC__in AsyncIIdentityProvider * This);
        
        DECLSPEC_XFGVIRT(AsyncIIdentityProvider, Finish_GetProviderPropertyStore)
        HRESULT ( STDMETHODCALLTYPE *Finish_GetProviderPropertyStore )( 
            __RPC__in AsyncIIdentityProvider * This,
            /* [out] */ __RPC__deref_out_opt IPropertyStore **ppPropertyStore);
        
        DECLSPEC_XFGVIRT(AsyncIIdentityProvider, Begin_Advise)
        HRESULT ( STDMETHODCALLTYPE *Begin_Advise )( 
            __RPC__in AsyncIIdentityProvider * This,
            /* [in] */ __RPC__in_opt IIdentityAdvise *pIdentityAdvise,
            /* [in] */ DWORD dwIdentityUpdateEvents);
        
        DECLSPEC_XFGVIRT(AsyncIIdentityProvider, Finish_Advise)
        HRESULT ( STDMETHODCALLTYPE *Finish_Advise )( 
            __RPC__in AsyncIIdentityProvider * This,
            /* [out] */ __RPC__out DWORD *pdwCookie);
        
        DECLSPEC_XFGVIRT(AsyncIIdentityProvider, Begin_UnAdvise)
        HRESULT ( STDMETHODCALLTYPE *Begin_UnAdvise )( 
            __RPC__in AsyncIIdentityProvider * This,
            /* [in] */ const DWORD dwCookie);
        
        DECLSPEC_XFGVIRT(AsyncIIdentityProvider, Finish_UnAdvise)
        HRESULT ( STDMETHODCALLTYPE *Finish_UnAdvise )( 
            __RPC__in AsyncIIdentityProvider * This);
        
        END_INTERFACE
    } AsyncIIdentityProviderVtbl;

    interface AsyncIIdentityProvider
    {
        CONST_VTBL struct AsyncIIdentityProviderVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define AsyncIIdentityProvider_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define AsyncIIdentityProvider_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define AsyncIIdentityProvider_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define AsyncIIdentityProvider_Begin_GetIdentityEnum(This,eIdentityType,pFilterkey,pFilterPropVarValue)	\
    ( (This)->lpVtbl -> Begin_GetIdentityEnum(This,eIdentityType,pFilterkey,pFilterPropVarValue) ) 

#define AsyncIIdentityProvider_Finish_GetIdentityEnum(This,ppIdentityEnum)	\
    ( (This)->lpVtbl -> Finish_GetIdentityEnum(This,ppIdentityEnum) ) 

#define AsyncIIdentityProvider_Begin_Create(This,lpszUserName,pKeywordsToAdd)	\
    ( (This)->lpVtbl -> Begin_Create(This,lpszUserName,pKeywordsToAdd) ) 

#define AsyncIIdentityProvider_Finish_Create(This,ppPropertyStore)	\
    ( (This)->lpVtbl -> Finish_Create(This,ppPropertyStore) ) 

#define AsyncIIdentityProvider_Begin_Import(This,pPropertyStore)	\
    ( (This)->lpVtbl -> Begin_Import(This,pPropertyStore) ) 

#define AsyncIIdentityProvider_Finish_Import(This)	\
    ( (This)->lpVtbl -> Finish_Import(This) ) 

#define AsyncIIdentityProvider_Begin_Delete(This,lpszUniqueID,pKeywordsToDelete)	\
    ( (This)->lpVtbl -> Begin_Delete(This,lpszUniqueID,pKeywordsToDelete) ) 

#define AsyncIIdentityProvider_Finish_Delete(This)	\
    ( (This)->lpVtbl -> Finish_Delete(This) ) 

#define AsyncIIdentityProvider_Begin_FindByUniqueID(This,lpszUniqueID)	\
    ( (This)->lpVtbl -> Begin_FindByUniqueID(This,lpszUniqueID) ) 

#define AsyncIIdentityProvider_Finish_FindByUniqueID(This,ppPropertyStore)	\
    ( (This)->lpVtbl -> Finish_FindByUniqueID(This,ppPropertyStore) ) 

#define AsyncIIdentityProvider_Begin_GetProviderPropertyStore(This)	\
    ( (This)->lpVtbl -> Begin_GetProviderPropertyStore(This) ) 

#define AsyncIIdentityProvider_Finish_GetProviderPropertyStore(This,ppPropertyStore)	\
    ( (This)->lpVtbl -> Finish_GetProviderPropertyStore(This,ppPropertyStore) ) 

#define AsyncIIdentityProvider_Begin_Advise(This,pIdentityAdvise,dwIdentityUpdateEvents)	\
    ( (This)->lpVtbl -> Begin_Advise(This,pIdentityAdvise,dwIdentityUpdateEvents) ) 

#define AsyncIIdentityProvider_Finish_Advise(This,pdwCookie)	\
    ( (This)->lpVtbl -> Finish_Advise(This,pdwCookie) ) 

#define AsyncIIdentityProvider_Begin_UnAdvise(This,dwCookie)	\
    ( (This)->lpVtbl -> Begin_UnAdvise(This,dwCookie) ) 

#define AsyncIIdentityProvider_Finish_UnAdvise(This)	\
    ( (This)->lpVtbl -> Finish_UnAdvise(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __AsyncIIdentityProvider_INTERFACE_DEFINED__ */


#ifndef __IAssociatedIdentityProvider_INTERFACE_DEFINED__
#define __IAssociatedIdentityProvider_INTERFACE_DEFINED__

/* interface IAssociatedIdentityProvider */
/* [unique][async_uuid][uuid][object] */ 


EXTERN_C const IID IID_IAssociatedIdentityProvider;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("2af066b3-4cbb-4cba-a798-204b6af68cc0")
    IAssociatedIdentityProvider : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE AssociateIdentity( 
            /* [in] */ __RPC__in HWND hwndParent,
            /* [out] */ __RPC__deref_out_opt IPropertyStore **ppPropertyStore) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE DisassociateIdentity( 
            /* [in] */ __RPC__in HWND hwndParent,
            /* [in] */ __RPC__in LPCWSTR lpszUniqueID) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ChangeCredential( 
            /* [in] */ __RPC__in HWND hwndParent,
            /* [in] */ __RPC__in LPCWSTR lpszUniqueID) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAssociatedIdentityProviderVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IAssociatedIdentityProvider * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IAssociatedIdentityProvider * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IAssociatedIdentityProvider * This);
        
        DECLSPEC_XFGVIRT(IAssociatedIdentityProvider, AssociateIdentity)
        HRESULT ( STDMETHODCALLTYPE *AssociateIdentity )( 
            __RPC__in IAssociatedIdentityProvider * This,
            /* [in] */ __RPC__in HWND hwndParent,
            /* [out] */ __RPC__deref_out_opt IPropertyStore **ppPropertyStore);
        
        DECLSPEC_XFGVIRT(IAssociatedIdentityProvider, DisassociateIdentity)
        HRESULT ( STDMETHODCALLTYPE *DisassociateIdentity )( 
            __RPC__in IAssociatedIdentityProvider * This,
            /* [in] */ __RPC__in HWND hwndParent,
            /* [in] */ __RPC__in LPCWSTR lpszUniqueID);
        
        DECLSPEC_XFGVIRT(IAssociatedIdentityProvider, ChangeCredential)
        HRESULT ( STDMETHODCALLTYPE *ChangeCredential )( 
            __RPC__in IAssociatedIdentityProvider * This,
            /* [in] */ __RPC__in HWND hwndParent,
            /* [in] */ __RPC__in LPCWSTR lpszUniqueID);
        
        END_INTERFACE
    } IAssociatedIdentityProviderVtbl;

    interface IAssociatedIdentityProvider
    {
        CONST_VTBL struct IAssociatedIdentityProviderVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAssociatedIdentityProvider_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAssociatedIdentityProvider_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAssociatedIdentityProvider_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAssociatedIdentityProvider_AssociateIdentity(This,hwndParent,ppPropertyStore)	\
    ( (This)->lpVtbl -> AssociateIdentity(This,hwndParent,ppPropertyStore) ) 

#define IAssociatedIdentityProvider_DisassociateIdentity(This,hwndParent,lpszUniqueID)	\
    ( (This)->lpVtbl -> DisassociateIdentity(This,hwndParent,lpszUniqueID) ) 

#define IAssociatedIdentityProvider_ChangeCredential(This,hwndParent,lpszUniqueID)	\
    ( (This)->lpVtbl -> ChangeCredential(This,hwndParent,lpszUniqueID) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAssociatedIdentityProvider_INTERFACE_DEFINED__ */


#ifndef __AsyncIAssociatedIdentityProvider_INTERFACE_DEFINED__
#define __AsyncIAssociatedIdentityProvider_INTERFACE_DEFINED__

/* interface AsyncIAssociatedIdentityProvider */
/* [uuid][unique][object] */ 


EXTERN_C const IID IID_AsyncIAssociatedIdentityProvider;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("2834d6ed-297e-4e72-8a51-961e86f05152")
    AsyncIAssociatedIdentityProvider : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Begin_AssociateIdentity( 
            /* [in] */ __RPC__in HWND hwndParent) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Finish_AssociateIdentity( 
            /* [out] */ __RPC__deref_out_opt IPropertyStore **ppPropertyStore) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Begin_DisassociateIdentity( 
            /* [in] */ __RPC__in HWND hwndParent,
            /* [in] */ __RPC__in LPCWSTR lpszUniqueID) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Finish_DisassociateIdentity( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Begin_ChangeCredential( 
            /* [in] */ __RPC__in HWND hwndParent,
            /* [in] */ __RPC__in LPCWSTR lpszUniqueID) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Finish_ChangeCredential( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct AsyncIAssociatedIdentityProviderVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in AsyncIAssociatedIdentityProvider * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in AsyncIAssociatedIdentityProvider * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in AsyncIAssociatedIdentityProvider * This);
        
        DECLSPEC_XFGVIRT(AsyncIAssociatedIdentityProvider, Begin_AssociateIdentity)
        HRESULT ( STDMETHODCALLTYPE *Begin_AssociateIdentity )( 
            __RPC__in AsyncIAssociatedIdentityProvider * This,
            /* [in] */ __RPC__in HWND hwndParent);
        
        DECLSPEC_XFGVIRT(AsyncIAssociatedIdentityProvider, Finish_AssociateIdentity)
        HRESULT ( STDMETHODCALLTYPE *Finish_AssociateIdentity )( 
            __RPC__in AsyncIAssociatedIdentityProvider * This,
            /* [out] */ __RPC__deref_out_opt IPropertyStore **ppPropertyStore);
        
        DECLSPEC_XFGVIRT(AsyncIAssociatedIdentityProvider, Begin_DisassociateIdentity)
        HRESULT ( STDMETHODCALLTYPE *Begin_DisassociateIdentity )( 
            __RPC__in AsyncIAssociatedIdentityProvider * This,
            /* [in] */ __RPC__in HWND hwndParent,
            /* [in] */ __RPC__in LPCWSTR lpszUniqueID);
        
        DECLSPEC_XFGVIRT(AsyncIAssociatedIdentityProvider, Finish_DisassociateIdentity)
        HRESULT ( STDMETHODCALLTYPE *Finish_DisassociateIdentity )( 
            __RPC__in AsyncIAssociatedIdentityProvider * This);
        
        DECLSPEC_XFGVIRT(AsyncIAssociatedIdentityProvider, Begin_ChangeCredential)
        HRESULT ( STDMETHODCALLTYPE *Begin_ChangeCredential )( 
            __RPC__in AsyncIAssociatedIdentityProvider * This,
            /* [in] */ __RPC__in HWND hwndParent,
            /* [in] */ __RPC__in LPCWSTR lpszUniqueID);
        
        DECLSPEC_XFGVIRT(AsyncIAssociatedIdentityProvider, Finish_ChangeCredential)
        HRESULT ( STDMETHODCALLTYPE *Finish_ChangeCredential )( 
            __RPC__in AsyncIAssociatedIdentityProvider * This);
        
        END_INTERFACE
    } AsyncIAssociatedIdentityProviderVtbl;

    interface AsyncIAssociatedIdentityProvider
    {
        CONST_VTBL struct AsyncIAssociatedIdentityProviderVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define AsyncIAssociatedIdentityProvider_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define AsyncIAssociatedIdentityProvider_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define AsyncIAssociatedIdentityProvider_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define AsyncIAssociatedIdentityProvider_Begin_AssociateIdentity(This,hwndParent)	\
    ( (This)->lpVtbl -> Begin_AssociateIdentity(This,hwndParent) ) 

#define AsyncIAssociatedIdentityProvider_Finish_AssociateIdentity(This,ppPropertyStore)	\
    ( (This)->lpVtbl -> Finish_AssociateIdentity(This,ppPropertyStore) ) 

#define AsyncIAssociatedIdentityProvider_Begin_DisassociateIdentity(This,hwndParent,lpszUniqueID)	\
    ( (This)->lpVtbl -> Begin_DisassociateIdentity(This,hwndParent,lpszUniqueID) ) 

#define AsyncIAssociatedIdentityProvider_Finish_DisassociateIdentity(This)	\
    ( (This)->lpVtbl -> Finish_DisassociateIdentity(This) ) 

#define AsyncIAssociatedIdentityProvider_Begin_ChangeCredential(This,hwndParent,lpszUniqueID)	\
    ( (This)->lpVtbl -> Begin_ChangeCredential(This,hwndParent,lpszUniqueID) ) 

#define AsyncIAssociatedIdentityProvider_Finish_ChangeCredential(This)	\
    ( (This)->lpVtbl -> Finish_ChangeCredential(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __AsyncIAssociatedIdentityProvider_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_identityprovider_0000_0003 */
/* [local] */ 

typedef /* [public][public][public][v1_enum] */ 
enum __MIDL___MIDL_itf_identityprovider_0000_0003_0001
    {
        IDENTITY_URL_CREATE_ACCOUNT_WIZARD	= 0,
        IDENTITY_URL_SIGN_IN_WIZARD	= 1,
        IDENTITY_URL_CHANGE_PASSWORD_WIZARD	= 2,
        IDENTITY_URL_IFEXISTS_WIZARD	= 3,
        IDENTITY_URL_ACCOUNT_SETTINGS	= 4,
        IDENTITY_URL_RESTORE_WIZARD	= 5,
        IDENTITY_URL_CONNECT_WIZARD	= 6
    } 	IDENTITY_URL;

#define STR_OUT_OF_BOX_EXPERIENCE L"OutOfBoxExperience"
#define STR_MODERN_SETTINGS_ADD_USER L"ModernSettingsAddUser"
#define STR_OUT_OF_BOX_UPGRADE_EXPERIENCE L"OutOfBoxUpgradeExperience"
#define STR_COMPLETE_ACCOUNT L"CompleteAccount"
#define STR_NTH_USER_FIRST_AUTH L"NthUserFirstAuth"
#define STR_USER_NAME L"Username"
#define STR_PROPERTY_STORE L"PropertyStore"
typedef /* [public][public][public][v1_enum] */ 
enum __MIDL___MIDL_itf_identityprovider_0000_0003_0002
    {
        NOT_CONNECTED	= 0,
        CONNECTING	= 1,
        CONNECT_COMPLETED	= 2
    } 	ACCOUNT_STATE;



extern RPC_IF_HANDLE __MIDL_itf_identityprovider_0000_0003_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_identityprovider_0000_0003_v0_0_s_ifspec;

#ifndef __IConnectedIdentityProvider_INTERFACE_DEFINED__
#define __IConnectedIdentityProvider_INTERFACE_DEFINED__

/* interface IConnectedIdentityProvider */
/* [unique][helpstring][async_uuid][uuid][object] */ 


EXTERN_C const IID IID_IConnectedIdentityProvider;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("B7417B54-E08C-429b-96C8-678D1369ECB1")
    IConnectedIdentityProvider : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE ConnectIdentity( 
            /* [ref][size_is][in] */ __RPC__in_ecount_full(AuthBufferSize) BYTE *AuthBuffer,
            /* [in] */ ULONG AuthBufferSize) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE DisconnectIdentity( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE IsConnected( 
            /* [ref][out] */ __RPC__out BOOL *Connected) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetUrl( 
            /* [in] */ IDENTITY_URL Identifier,
            /* [unique][in] */ __RPC__in_opt IBindCtx *Context,
            /* [out] */ __RPC__out VARIANT *PostData,
            /* [string][out] */ __RPC__deref_out_opt_string LPWSTR *Url) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetAccountState( 
            /* [out] */ __RPC__out ACCOUNT_STATE *pState) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IConnectedIdentityProviderVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IConnectedIdentityProvider * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IConnectedIdentityProvider * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IConnectedIdentityProvider * This);
        
        DECLSPEC_XFGVIRT(IConnectedIdentityProvider, ConnectIdentity)
        HRESULT ( STDMETHODCALLTYPE *ConnectIdentity )( 
            __RPC__in IConnectedIdentityProvider * This,
            /* [ref][size_is][in] */ __RPC__in_ecount_full(AuthBufferSize) BYTE *AuthBuffer,
            /* [in] */ ULONG AuthBufferSize);
        
        DECLSPEC_XFGVIRT(IConnectedIdentityProvider, DisconnectIdentity)
        HRESULT ( STDMETHODCALLTYPE *DisconnectIdentity )( 
            __RPC__in IConnectedIdentityProvider * This);
        
        DECLSPEC_XFGVIRT(IConnectedIdentityProvider, IsConnected)
        HRESULT ( STDMETHODCALLTYPE *IsConnected )( 
            __RPC__in IConnectedIdentityProvider * This,
            /* [ref][out] */ __RPC__out BOOL *Connected);
        
        DECLSPEC_XFGVIRT(IConnectedIdentityProvider, GetUrl)
        HRESULT ( STDMETHODCALLTYPE *GetUrl )( 
            __RPC__in IConnectedIdentityProvider * This,
            /* [in] */ IDENTITY_URL Identifier,
            /* [unique][in] */ __RPC__in_opt IBindCtx *Context,
            /* [out] */ __RPC__out VARIANT *PostData,
            /* [string][out] */ __RPC__deref_out_opt_string LPWSTR *Url);
        
        DECLSPEC_XFGVIRT(IConnectedIdentityProvider, GetAccountState)
        HRESULT ( STDMETHODCALLTYPE *GetAccountState )( 
            __RPC__in IConnectedIdentityProvider * This,
            /* [out] */ __RPC__out ACCOUNT_STATE *pState);
        
        END_INTERFACE
    } IConnectedIdentityProviderVtbl;

    interface IConnectedIdentityProvider
    {
        CONST_VTBL struct IConnectedIdentityProviderVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IConnectedIdentityProvider_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IConnectedIdentityProvider_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IConnectedIdentityProvider_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IConnectedIdentityProvider_ConnectIdentity(This,AuthBuffer,AuthBufferSize)	\
    ( (This)->lpVtbl -> ConnectIdentity(This,AuthBuffer,AuthBufferSize) ) 

#define IConnectedIdentityProvider_DisconnectIdentity(This)	\
    ( (This)->lpVtbl -> DisconnectIdentity(This) ) 

#define IConnectedIdentityProvider_IsConnected(This,Connected)	\
    ( (This)->lpVtbl -> IsConnected(This,Connected) ) 

#define IConnectedIdentityProvider_GetUrl(This,Identifier,Context,PostData,Url)	\
    ( (This)->lpVtbl -> GetUrl(This,Identifier,Context,PostData,Url) ) 

#define IConnectedIdentityProvider_GetAccountState(This,pState)	\
    ( (This)->lpVtbl -> GetAccountState(This,pState) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IConnectedIdentityProvider_INTERFACE_DEFINED__ */


#ifndef __AsyncIConnectedIdentityProvider_INTERFACE_DEFINED__
#define __AsyncIConnectedIdentityProvider_INTERFACE_DEFINED__

/* interface AsyncIConnectedIdentityProvider */
/* [uuid][unique][helpstring][object] */ 


EXTERN_C const IID IID_AsyncIConnectedIdentityProvider;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("9CE55141-BCE9-4E15-824D-43D79F512F93")
    AsyncIConnectedIdentityProvider : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Begin_ConnectIdentity( 
            /* [ref][size_is][in] */ __RPC__in_xcount_full(AuthBufferSize) BYTE *AuthBuffer,
            /* [in] */ ULONG AuthBufferSize) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Finish_ConnectIdentity( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Begin_DisconnectIdentity( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Finish_DisconnectIdentity( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Begin_IsConnected( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Finish_IsConnected( 
            /* [ref][out] */ __RPC__out BOOL *Connected) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Begin_GetUrl( 
            /* [in] */ IDENTITY_URL Identifier,
            /* [unique][in] */ __RPC__in_opt IBindCtx *Context) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Finish_GetUrl( 
            /* [out] */ __RPC__out VARIANT *PostData,
            /* [string][out] */ __RPC__deref_out_opt_string LPWSTR *Url) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Begin_GetAccountState( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Finish_GetAccountState( 
            /* [out] */ __RPC__out ACCOUNT_STATE *pState) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct AsyncIConnectedIdentityProviderVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in AsyncIConnectedIdentityProvider * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in AsyncIConnectedIdentityProvider * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in AsyncIConnectedIdentityProvider * This);
        
        DECLSPEC_XFGVIRT(AsyncIConnectedIdentityProvider, Begin_ConnectIdentity)
        HRESULT ( STDMETHODCALLTYPE *Begin_ConnectIdentity )( 
            __RPC__in AsyncIConnectedIdentityProvider * This,
            /* [ref][size_is][in] */ __RPC__in_xcount_full(AuthBufferSize) BYTE *AuthBuffer,
            /* [in] */ ULONG AuthBufferSize);
        
        DECLSPEC_XFGVIRT(AsyncIConnectedIdentityProvider, Finish_ConnectIdentity)
        HRESULT ( STDMETHODCALLTYPE *Finish_ConnectIdentity )( 
            __RPC__in AsyncIConnectedIdentityProvider * This);
        
        DECLSPEC_XFGVIRT(AsyncIConnectedIdentityProvider, Begin_DisconnectIdentity)
        HRESULT ( STDMETHODCALLTYPE *Begin_DisconnectIdentity )( 
            __RPC__in AsyncIConnectedIdentityProvider * This);
        
        DECLSPEC_XFGVIRT(AsyncIConnectedIdentityProvider, Finish_DisconnectIdentity)
        HRESULT ( STDMETHODCALLTYPE *Finish_DisconnectIdentity )( 
            __RPC__in AsyncIConnectedIdentityProvider * This);
        
        DECLSPEC_XFGVIRT(AsyncIConnectedIdentityProvider, Begin_IsConnected)
        HRESULT ( STDMETHODCALLTYPE *Begin_IsConnected )( 
            __RPC__in AsyncIConnectedIdentityProvider * This);
        
        DECLSPEC_XFGVIRT(AsyncIConnectedIdentityProvider, Finish_IsConnected)
        HRESULT ( STDMETHODCALLTYPE *Finish_IsConnected )( 
            __RPC__in AsyncIConnectedIdentityProvider * This,
            /* [ref][out] */ __RPC__out BOOL *Connected);
        
        DECLSPEC_XFGVIRT(AsyncIConnectedIdentityProvider, Begin_GetUrl)
        HRESULT ( STDMETHODCALLTYPE *Begin_GetUrl )( 
            __RPC__in AsyncIConnectedIdentityProvider * This,
            /* [in] */ IDENTITY_URL Identifier,
            /* [unique][in] */ __RPC__in_opt IBindCtx *Context);
        
        DECLSPEC_XFGVIRT(AsyncIConnectedIdentityProvider, Finish_GetUrl)
        HRESULT ( STDMETHODCALLTYPE *Finish_GetUrl )( 
            __RPC__in AsyncIConnectedIdentityProvider * This,
            /* [out] */ __RPC__out VARIANT *PostData,
            /* [string][out] */ __RPC__deref_out_opt_string LPWSTR *Url);
        
        DECLSPEC_XFGVIRT(AsyncIConnectedIdentityProvider, Begin_GetAccountState)
        HRESULT ( STDMETHODCALLTYPE *Begin_GetAccountState )( 
            __RPC__in AsyncIConnectedIdentityProvider * This);
        
        DECLSPEC_XFGVIRT(AsyncIConnectedIdentityProvider, Finish_GetAccountState)
        HRESULT ( STDMETHODCALLTYPE *Finish_GetAccountState )( 
            __RPC__in AsyncIConnectedIdentityProvider * This,
            /* [out] */ __RPC__out ACCOUNT_STATE *pState);
        
        END_INTERFACE
    } AsyncIConnectedIdentityProviderVtbl;

    interface AsyncIConnectedIdentityProvider
    {
        CONST_VTBL struct AsyncIConnectedIdentityProviderVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define AsyncIConnectedIdentityProvider_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define AsyncIConnectedIdentityProvider_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define AsyncIConnectedIdentityProvider_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define AsyncIConnectedIdentityProvider_Begin_ConnectIdentity(This,AuthBuffer,AuthBufferSize)	\
    ( (This)->lpVtbl -> Begin_ConnectIdentity(This,AuthBuffer,AuthBufferSize) ) 

#define AsyncIConnectedIdentityProvider_Finish_ConnectIdentity(This)	\
    ( (This)->lpVtbl -> Finish_ConnectIdentity(This) ) 

#define AsyncIConnectedIdentityProvider_Begin_DisconnectIdentity(This)	\
    ( (This)->lpVtbl -> Begin_DisconnectIdentity(This) ) 

#define AsyncIConnectedIdentityProvider_Finish_DisconnectIdentity(This)	\
    ( (This)->lpVtbl -> Finish_DisconnectIdentity(This) ) 

#define AsyncIConnectedIdentityProvider_Begin_IsConnected(This)	\
    ( (This)->lpVtbl -> Begin_IsConnected(This) ) 

#define AsyncIConnectedIdentityProvider_Finish_IsConnected(This,Connected)	\
    ( (This)->lpVtbl -> Finish_IsConnected(This,Connected) ) 

#define AsyncIConnectedIdentityProvider_Begin_GetUrl(This,Identifier,Context)	\
    ( (This)->lpVtbl -> Begin_GetUrl(This,Identifier,Context) ) 

#define AsyncIConnectedIdentityProvider_Finish_GetUrl(This,PostData,Url)	\
    ( (This)->lpVtbl -> Finish_GetUrl(This,PostData,Url) ) 

#define AsyncIConnectedIdentityProvider_Begin_GetAccountState(This)	\
    ( (This)->lpVtbl -> Begin_GetAccountState(This) ) 

#define AsyncIConnectedIdentityProvider_Finish_GetAccountState(This,pState)	\
    ( (This)->lpVtbl -> Finish_GetAccountState(This,pState) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __AsyncIConnectedIdentityProvider_INTERFACE_DEFINED__ */


#ifndef __IIdentityAuthentication_INTERFACE_DEFINED__
#define __IIdentityAuthentication_INTERFACE_DEFINED__

/* interface IIdentityAuthentication */
/* [unique][helpstring][async_uuid][uuid][object] */ 


EXTERN_C const IID IID_IIdentityAuthentication;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("5E7EF254-979F-43b5-B74E-06E4EB7DF0F9")
    IIdentityAuthentication : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SetIdentityCredential( 
            /* [unique][size_is][in] */ __RPC__in_ecount_full_opt(CredBufferLength) BYTE *CredBuffer,
            /* [in] */ ULONG CredBufferLength) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ValidateIdentityCredential( 
            /* [ref][size_is][in] */ __RPC__in_ecount_full(CredBufferLength) BYTE *CredBuffer,
            /* [in] */ ULONG CredBufferLength,
            /* [unique][out][in] */ __RPC__deref_opt_inout_opt IPropertyStore **ppIdentityProperties) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IIdentityAuthenticationVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IIdentityAuthentication * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IIdentityAuthentication * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IIdentityAuthentication * This);
        
        DECLSPEC_XFGVIRT(IIdentityAuthentication, SetIdentityCredential)
        HRESULT ( STDMETHODCALLTYPE *SetIdentityCredential )( 
            __RPC__in IIdentityAuthentication * This,
            /* [unique][size_is][in] */ __RPC__in_ecount_full_opt(CredBufferLength) BYTE *CredBuffer,
            /* [in] */ ULONG CredBufferLength);
        
        DECLSPEC_XFGVIRT(IIdentityAuthentication, ValidateIdentityCredential)
        HRESULT ( STDMETHODCALLTYPE *ValidateIdentityCredential )( 
            __RPC__in IIdentityAuthentication * This,
            /* [ref][size_is][in] */ __RPC__in_ecount_full(CredBufferLength) BYTE *CredBuffer,
            /* [in] */ ULONG CredBufferLength,
            /* [unique][out][in] */ __RPC__deref_opt_inout_opt IPropertyStore **ppIdentityProperties);
        
        END_INTERFACE
    } IIdentityAuthenticationVtbl;

    interface IIdentityAuthentication
    {
        CONST_VTBL struct IIdentityAuthenticationVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IIdentityAuthentication_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IIdentityAuthentication_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IIdentityAuthentication_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IIdentityAuthentication_SetIdentityCredential(This,CredBuffer,CredBufferLength)	\
    ( (This)->lpVtbl -> SetIdentityCredential(This,CredBuffer,CredBufferLength) ) 

#define IIdentityAuthentication_ValidateIdentityCredential(This,CredBuffer,CredBufferLength,ppIdentityProperties)	\
    ( (This)->lpVtbl -> ValidateIdentityCredential(This,CredBuffer,CredBufferLength,ppIdentityProperties) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IIdentityAuthentication_INTERFACE_DEFINED__ */


#ifndef __AsyncIIdentityAuthentication_INTERFACE_DEFINED__
#define __AsyncIIdentityAuthentication_INTERFACE_DEFINED__

/* interface AsyncIIdentityAuthentication */
/* [uuid][unique][helpstring][object] */ 


EXTERN_C const IID IID_AsyncIIdentityAuthentication;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("F9A2F918-FECA-4e9c-9633-61CBF13ED34D")
    AsyncIIdentityAuthentication : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Begin_SetIdentityCredential( 
            /* [unique][size_is][in] */ __RPC__in_xcount_full_opt(CredBufferLength) BYTE *CredBuffer,
            /* [in] */ ULONG CredBufferLength) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Finish_SetIdentityCredential( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Begin_ValidateIdentityCredential( 
            /* [ref][size_is][in] */ __RPC__in_xcount_full(CredBufferLength) BYTE *CredBuffer,
            /* [in] */ ULONG CredBufferLength,
            /* [unique][out][in] */ __RPC__deref_opt_inout_opt IPropertyStore **ppIdentityProperties) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Finish_ValidateIdentityCredential( 
            /* [unique][out][in] */ __RPC__deref_opt_inout_opt IPropertyStore **ppIdentityProperties) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct AsyncIIdentityAuthenticationVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in AsyncIIdentityAuthentication * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in AsyncIIdentityAuthentication * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in AsyncIIdentityAuthentication * This);
        
        DECLSPEC_XFGVIRT(AsyncIIdentityAuthentication, Begin_SetIdentityCredential)
        HRESULT ( STDMETHODCALLTYPE *Begin_SetIdentityCredential )( 
            __RPC__in AsyncIIdentityAuthentication * This,
            /* [unique][size_is][in] */ __RPC__in_xcount_full_opt(CredBufferLength) BYTE *CredBuffer,
            /* [in] */ ULONG CredBufferLength);
        
        DECLSPEC_XFGVIRT(AsyncIIdentityAuthentication, Finish_SetIdentityCredential)
        HRESULT ( STDMETHODCALLTYPE *Finish_SetIdentityCredential )( 
            __RPC__in AsyncIIdentityAuthentication * This);
        
        DECLSPEC_XFGVIRT(AsyncIIdentityAuthentication, Begin_ValidateIdentityCredential)
        HRESULT ( STDMETHODCALLTYPE *Begin_ValidateIdentityCredential )( 
            __RPC__in AsyncIIdentityAuthentication * This,
            /* [ref][size_is][in] */ __RPC__in_xcount_full(CredBufferLength) BYTE *CredBuffer,
            /* [in] */ ULONG CredBufferLength,
            /* [unique][out][in] */ __RPC__deref_opt_inout_opt IPropertyStore **ppIdentityProperties);
        
        DECLSPEC_XFGVIRT(AsyncIIdentityAuthentication, Finish_ValidateIdentityCredential)
        HRESULT ( STDMETHODCALLTYPE *Finish_ValidateIdentityCredential )( 
            __RPC__in AsyncIIdentityAuthentication * This,
            /* [unique][out][in] */ __RPC__deref_opt_inout_opt IPropertyStore **ppIdentityProperties);
        
        END_INTERFACE
    } AsyncIIdentityAuthenticationVtbl;

    interface AsyncIIdentityAuthentication
    {
        CONST_VTBL struct AsyncIIdentityAuthenticationVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define AsyncIIdentityAuthentication_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define AsyncIIdentityAuthentication_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define AsyncIIdentityAuthentication_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define AsyncIIdentityAuthentication_Begin_SetIdentityCredential(This,CredBuffer,CredBufferLength)	\
    ( (This)->lpVtbl -> Begin_SetIdentityCredential(This,CredBuffer,CredBufferLength) ) 

#define AsyncIIdentityAuthentication_Finish_SetIdentityCredential(This)	\
    ( (This)->lpVtbl -> Finish_SetIdentityCredential(This) ) 

#define AsyncIIdentityAuthentication_Begin_ValidateIdentityCredential(This,CredBuffer,CredBufferLength,ppIdentityProperties)	\
    ( (This)->lpVtbl -> Begin_ValidateIdentityCredential(This,CredBuffer,CredBufferLength,ppIdentityProperties) ) 

#define AsyncIIdentityAuthentication_Finish_ValidateIdentityCredential(This,ppIdentityProperties)	\
    ( (This)->lpVtbl -> Finish_ValidateIdentityCredential(This,ppIdentityProperties) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __AsyncIIdentityAuthentication_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_identityprovider_0000_0005 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_identityprovider_0000_0005_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_identityprovider_0000_0005_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

unsigned long             __RPC_USER  BSTR_UserSize(     __RPC__in unsigned long *, unsigned long            , __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserMarshal(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out BSTR * ); 
void                      __RPC_USER  BSTR_UserFree(     __RPC__in unsigned long *, __RPC__in BSTR * ); 

unsigned long             __RPC_USER  HWND_UserSize(     __RPC__in unsigned long *, unsigned long            , __RPC__in HWND * ); 
unsigned char * __RPC_USER  HWND_UserMarshal(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in HWND * ); 
unsigned char * __RPC_USER  HWND_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out HWND * ); 
void                      __RPC_USER  HWND_UserFree(     __RPC__in unsigned long *, __RPC__in HWND * ); 

unsigned long             __RPC_USER  LPSAFEARRAY_UserSize(     __RPC__in unsigned long *, unsigned long            , __RPC__in LPSAFEARRAY * ); 
unsigned char * __RPC_USER  LPSAFEARRAY_UserMarshal(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in LPSAFEARRAY * ); 
unsigned char * __RPC_USER  LPSAFEARRAY_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out LPSAFEARRAY * ); 
void                      __RPC_USER  LPSAFEARRAY_UserFree(     __RPC__in unsigned long *, __RPC__in LPSAFEARRAY * ); 

unsigned long             __RPC_USER  VARIANT_UserSize(     __RPC__in unsigned long *, unsigned long            , __RPC__in VARIANT * ); 
unsigned char * __RPC_USER  VARIANT_UserMarshal(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in VARIANT * ); 
unsigned char * __RPC_USER  VARIANT_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out VARIANT * ); 
void                      __RPC_USER  VARIANT_UserFree(     __RPC__in unsigned long *, __RPC__in VARIANT * ); 

unsigned long             __RPC_USER  BSTR_UserSize64(     __RPC__in unsigned long *, unsigned long            , __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserMarshal64(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out BSTR * ); 
void                      __RPC_USER  BSTR_UserFree64(     __RPC__in unsigned long *, __RPC__in BSTR * ); 

unsigned long             __RPC_USER  HWND_UserSize64(     __RPC__in unsigned long *, unsigned long            , __RPC__in HWND * ); 
unsigned char * __RPC_USER  HWND_UserMarshal64(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in HWND * ); 
unsigned char * __RPC_USER  HWND_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out HWND * ); 
void                      __RPC_USER  HWND_UserFree64(     __RPC__in unsigned long *, __RPC__in HWND * ); 

unsigned long             __RPC_USER  LPSAFEARRAY_UserSize64(     __RPC__in unsigned long *, unsigned long            , __RPC__in LPSAFEARRAY * ); 
unsigned char * __RPC_USER  LPSAFEARRAY_UserMarshal64(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in LPSAFEARRAY * ); 
unsigned char * __RPC_USER  LPSAFEARRAY_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out LPSAFEARRAY * ); 
void                      __RPC_USER  LPSAFEARRAY_UserFree64(     __RPC__in unsigned long *, __RPC__in LPSAFEARRAY * ); 

unsigned long             __RPC_USER  VARIANT_UserSize64(     __RPC__in unsigned long *, unsigned long            , __RPC__in VARIANT * ); 
unsigned char * __RPC_USER  VARIANT_UserMarshal64(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in VARIANT * ); 
unsigned char * __RPC_USER  VARIANT_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out VARIANT * ); 
void                      __RPC_USER  VARIANT_UserFree64(     __RPC__in unsigned long *, __RPC__in VARIANT * ); 

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


