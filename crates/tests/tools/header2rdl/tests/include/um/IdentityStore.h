

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

#ifndef __identitystore_h__
#define __identitystore_h__

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

#ifndef __IIdentityStore_FWD_DEFINED__
#define __IIdentityStore_FWD_DEFINED__
typedef interface IIdentityStore IIdentityStore;

#endif 	/* __IIdentityStore_FWD_DEFINED__ */


#ifndef __AsyncIIdentityStore_FWD_DEFINED__
#define __AsyncIIdentityStore_FWD_DEFINED__
typedef interface AsyncIIdentityStore AsyncIIdentityStore;

#endif 	/* __AsyncIIdentityStore_FWD_DEFINED__ */


#ifndef __IIdentityStoreEx_FWD_DEFINED__
#define __IIdentityStoreEx_FWD_DEFINED__
typedef interface IIdentityStoreEx IIdentityStoreEx;

#endif 	/* __IIdentityStoreEx_FWD_DEFINED__ */


#ifndef __AsyncIIdentityStoreEx_FWD_DEFINED__
#define __AsyncIIdentityStoreEx_FWD_DEFINED__
typedef interface AsyncIIdentityStoreEx AsyncIIdentityStoreEx;

#endif 	/* __AsyncIIdentityStoreEx_FWD_DEFINED__ */


#ifndef __CoClassIdentityStore_FWD_DEFINED__
#define __CoClassIdentityStore_FWD_DEFINED__

#ifdef __cplusplus
typedef class CoClassIdentityStore CoClassIdentityStore;
#else
typedef struct CoClassIdentityStore CoClassIdentityStore;
#endif /* __cplusplus */

#endif 	/* __CoClassIdentityStore_FWD_DEFINED__ */


#ifndef __CIdentityProfileHandler_FWD_DEFINED__
#define __CIdentityProfileHandler_FWD_DEFINED__

#ifdef __cplusplus
typedef class CIdentityProfileHandler CIdentityProfileHandler;
#else
typedef struct CIdentityProfileHandler CIdentityProfileHandler;
#endif /* __cplusplus */

#endif 	/* __CIdentityProfileHandler_FWD_DEFINED__ */


/* header files for imported files */
#include "oaidl.h"
#include "ocidl.h"
#include "propsys.h"
#include "IdentityCommon.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_identitystore_0000_0000 */
/* [local] */ 

#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


extern RPC_IF_HANDLE __MIDL_itf_identitystore_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_identitystore_0000_0000_v0_0_s_ifspec;

#ifndef __IIdentityStore_INTERFACE_DEFINED__
#define __IIdentityStore_INTERFACE_DEFINED__

/* interface IIdentityStore */
/* [unique][helpstring][async_uuid][uuid][object] */ 


EXTERN_C const IID IID_IIdentityStore;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("df586fa5-6f35-44f1-b209-b38e169772eb")
    IIdentityStore : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetCount( 
            /* [out] */ __RPC__out DWORD *pdwProviders) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetAt( 
            /* [in] */ const DWORD dwProvider,
            /* [unique][out][in] */ __RPC__inout_opt GUID *pProvGuid,
            /* [out] */ __RPC__deref_out_opt IUnknown **ppIdentityProvider) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE AddToCache( 
            /* [in] */ __RPC__in LPCWSTR lpszUniqueID,
            /* [in] */ __RPC__in REFGUID ProviderGUID) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ConvertToSid( 
            /* [in] */ __RPC__in LPCWSTR lpszUniqueID,
            /* [in] */ __RPC__in REFGUID ProviderGUID,
            /* [in] */ USHORT cbSid,
            /* [size_is][unique][out][in] */ __RPC__inout_ecount_full_opt(cbSid) BYTE *pSid,
            /* [out] */ __RPC__out USHORT *pcbRequiredSid) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EnumerateIdentities( 
            /* [in] */ const IDENTITY_TYPE eIdentityType,
            /* [unique][in] */ __RPC__in_opt const PROPERTYKEY *pFilterkey,
            /* [unique][in] */ __RPC__in_opt const PROPVARIANT *pFilterPropVarValue,
            /* [out] */ __RPC__deref_out_opt IEnumUnknown **ppIdentityEnum) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Reset( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IIdentityStoreVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IIdentityStore * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IIdentityStore * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IIdentityStore * This);
        
        DECLSPEC_XFGVIRT(IIdentityStore, GetCount)
        HRESULT ( STDMETHODCALLTYPE *GetCount )( 
            __RPC__in IIdentityStore * This,
            /* [out] */ __RPC__out DWORD *pdwProviders);
        
        DECLSPEC_XFGVIRT(IIdentityStore, GetAt)
        HRESULT ( STDMETHODCALLTYPE *GetAt )( 
            __RPC__in IIdentityStore * This,
            /* [in] */ const DWORD dwProvider,
            /* [unique][out][in] */ __RPC__inout_opt GUID *pProvGuid,
            /* [out] */ __RPC__deref_out_opt IUnknown **ppIdentityProvider);
        
        DECLSPEC_XFGVIRT(IIdentityStore, AddToCache)
        HRESULT ( STDMETHODCALLTYPE *AddToCache )( 
            __RPC__in IIdentityStore * This,
            /* [in] */ __RPC__in LPCWSTR lpszUniqueID,
            /* [in] */ __RPC__in REFGUID ProviderGUID);
        
        DECLSPEC_XFGVIRT(IIdentityStore, ConvertToSid)
        HRESULT ( STDMETHODCALLTYPE *ConvertToSid )( 
            __RPC__in IIdentityStore * This,
            /* [in] */ __RPC__in LPCWSTR lpszUniqueID,
            /* [in] */ __RPC__in REFGUID ProviderGUID,
            /* [in] */ USHORT cbSid,
            /* [size_is][unique][out][in] */ __RPC__inout_ecount_full_opt(cbSid) BYTE *pSid,
            /* [out] */ __RPC__out USHORT *pcbRequiredSid);
        
        DECLSPEC_XFGVIRT(IIdentityStore, EnumerateIdentities)
        HRESULT ( STDMETHODCALLTYPE *EnumerateIdentities )( 
            __RPC__in IIdentityStore * This,
            /* [in] */ const IDENTITY_TYPE eIdentityType,
            /* [unique][in] */ __RPC__in_opt const PROPERTYKEY *pFilterkey,
            /* [unique][in] */ __RPC__in_opt const PROPVARIANT *pFilterPropVarValue,
            /* [out] */ __RPC__deref_out_opt IEnumUnknown **ppIdentityEnum);
        
        DECLSPEC_XFGVIRT(IIdentityStore, Reset)
        HRESULT ( STDMETHODCALLTYPE *Reset )( 
            __RPC__in IIdentityStore * This);
        
        END_INTERFACE
    } IIdentityStoreVtbl;

    interface IIdentityStore
    {
        CONST_VTBL struct IIdentityStoreVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IIdentityStore_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IIdentityStore_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IIdentityStore_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IIdentityStore_GetCount(This,pdwProviders)	\
    ( (This)->lpVtbl -> GetCount(This,pdwProviders) ) 

#define IIdentityStore_GetAt(This,dwProvider,pProvGuid,ppIdentityProvider)	\
    ( (This)->lpVtbl -> GetAt(This,dwProvider,pProvGuid,ppIdentityProvider) ) 

#define IIdentityStore_AddToCache(This,lpszUniqueID,ProviderGUID)	\
    ( (This)->lpVtbl -> AddToCache(This,lpszUniqueID,ProviderGUID) ) 

#define IIdentityStore_ConvertToSid(This,lpszUniqueID,ProviderGUID,cbSid,pSid,pcbRequiredSid)	\
    ( (This)->lpVtbl -> ConvertToSid(This,lpszUniqueID,ProviderGUID,cbSid,pSid,pcbRequiredSid) ) 

#define IIdentityStore_EnumerateIdentities(This,eIdentityType,pFilterkey,pFilterPropVarValue,ppIdentityEnum)	\
    ( (This)->lpVtbl -> EnumerateIdentities(This,eIdentityType,pFilterkey,pFilterPropVarValue,ppIdentityEnum) ) 

#define IIdentityStore_Reset(This)	\
    ( (This)->lpVtbl -> Reset(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IIdentityStore_INTERFACE_DEFINED__ */


#ifndef __AsyncIIdentityStore_INTERFACE_DEFINED__
#define __AsyncIIdentityStore_INTERFACE_DEFINED__

/* interface AsyncIIdentityStore */
/* [uuid][unique][helpstring][object] */ 


EXTERN_C const IID IID_AsyncIIdentityStore;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("eefa1616-48de-4872-aa64-6e6206535a51")
    AsyncIIdentityStore : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Begin_GetCount( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Finish_GetCount( 
            /* [out] */ __RPC__out DWORD *pdwProviders) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Begin_GetAt( 
            /* [in] */ const DWORD dwProvider,
            /* [unique][out][in] */ __RPC__inout_opt GUID *pProvGuid) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Finish_GetAt( 
            /* [unique][out][in] */ __RPC__inout_opt GUID *pProvGuid,
            /* [out] */ __RPC__deref_out_opt IUnknown **ppIdentityProvider) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Begin_AddToCache( 
            /* [in] */ __RPC__in LPCWSTR lpszUniqueID,
            /* [in] */ __RPC__in REFGUID ProviderGUID) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Finish_AddToCache( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Begin_ConvertToSid( 
            /* [in] */ __RPC__in LPCWSTR lpszUniqueID,
            /* [in] */ __RPC__in REFGUID ProviderGUID,
            /* [in] */ USHORT cbSid,
            /* [size_is][unique][out][in] */ __RPC__inout_xcount_full_opt(cbSid) BYTE *pSid) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Finish_ConvertToSid( 
            /* [size_is][unique][out][in] */ __RPC__inout_xcount_full_opt(cbSid) BYTE *pSid,
            /* [out] */ __RPC__out USHORT *pcbRequiredSid) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Begin_EnumerateIdentities( 
            /* [in] */ const IDENTITY_TYPE eIdentityType,
            /* [unique][in] */ __RPC__in_opt const PROPERTYKEY *pFilterkey,
            /* [unique][in] */ __RPC__in_opt const PROPVARIANT *pFilterPropVarValue) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Finish_EnumerateIdentities( 
            /* [out] */ __RPC__deref_out_opt IEnumUnknown **ppIdentityEnum) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Begin_Reset( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Finish_Reset( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct AsyncIIdentityStoreVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in AsyncIIdentityStore * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in AsyncIIdentityStore * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in AsyncIIdentityStore * This);
        
        DECLSPEC_XFGVIRT(AsyncIIdentityStore, Begin_GetCount)
        HRESULT ( STDMETHODCALLTYPE *Begin_GetCount )( 
            __RPC__in AsyncIIdentityStore * This);
        
        DECLSPEC_XFGVIRT(AsyncIIdentityStore, Finish_GetCount)
        HRESULT ( STDMETHODCALLTYPE *Finish_GetCount )( 
            __RPC__in AsyncIIdentityStore * This,
            /* [out] */ __RPC__out DWORD *pdwProviders);
        
        DECLSPEC_XFGVIRT(AsyncIIdentityStore, Begin_GetAt)
        HRESULT ( STDMETHODCALLTYPE *Begin_GetAt )( 
            __RPC__in AsyncIIdentityStore * This,
            /* [in] */ const DWORD dwProvider,
            /* [unique][out][in] */ __RPC__inout_opt GUID *pProvGuid);
        
        DECLSPEC_XFGVIRT(AsyncIIdentityStore, Finish_GetAt)
        HRESULT ( STDMETHODCALLTYPE *Finish_GetAt )( 
            __RPC__in AsyncIIdentityStore * This,
            /* [unique][out][in] */ __RPC__inout_opt GUID *pProvGuid,
            /* [out] */ __RPC__deref_out_opt IUnknown **ppIdentityProvider);
        
        DECLSPEC_XFGVIRT(AsyncIIdentityStore, Begin_AddToCache)
        HRESULT ( STDMETHODCALLTYPE *Begin_AddToCache )( 
            __RPC__in AsyncIIdentityStore * This,
            /* [in] */ __RPC__in LPCWSTR lpszUniqueID,
            /* [in] */ __RPC__in REFGUID ProviderGUID);
        
        DECLSPEC_XFGVIRT(AsyncIIdentityStore, Finish_AddToCache)
        HRESULT ( STDMETHODCALLTYPE *Finish_AddToCache )( 
            __RPC__in AsyncIIdentityStore * This);
        
        DECLSPEC_XFGVIRT(AsyncIIdentityStore, Begin_ConvertToSid)
        HRESULT ( STDMETHODCALLTYPE *Begin_ConvertToSid )( 
            __RPC__in AsyncIIdentityStore * This,
            /* [in] */ __RPC__in LPCWSTR lpszUniqueID,
            /* [in] */ __RPC__in REFGUID ProviderGUID,
            /* [in] */ USHORT cbSid,
            /* [size_is][unique][out][in] */ __RPC__inout_xcount_full_opt(cbSid) BYTE *pSid);
        
        DECLSPEC_XFGVIRT(AsyncIIdentityStore, Finish_ConvertToSid)
        HRESULT ( STDMETHODCALLTYPE *Finish_ConvertToSid )( 
            __RPC__in AsyncIIdentityStore * This,
            /* [size_is][unique][out][in] */ __RPC__inout_xcount_full_opt(cbSid) BYTE *pSid,
            /* [out] */ __RPC__out USHORT *pcbRequiredSid);
        
        DECLSPEC_XFGVIRT(AsyncIIdentityStore, Begin_EnumerateIdentities)
        HRESULT ( STDMETHODCALLTYPE *Begin_EnumerateIdentities )( 
            __RPC__in AsyncIIdentityStore * This,
            /* [in] */ const IDENTITY_TYPE eIdentityType,
            /* [unique][in] */ __RPC__in_opt const PROPERTYKEY *pFilterkey,
            /* [unique][in] */ __RPC__in_opt const PROPVARIANT *pFilterPropVarValue);
        
        DECLSPEC_XFGVIRT(AsyncIIdentityStore, Finish_EnumerateIdentities)
        HRESULT ( STDMETHODCALLTYPE *Finish_EnumerateIdentities )( 
            __RPC__in AsyncIIdentityStore * This,
            /* [out] */ __RPC__deref_out_opt IEnumUnknown **ppIdentityEnum);
        
        DECLSPEC_XFGVIRT(AsyncIIdentityStore, Begin_Reset)
        HRESULT ( STDMETHODCALLTYPE *Begin_Reset )( 
            __RPC__in AsyncIIdentityStore * This);
        
        DECLSPEC_XFGVIRT(AsyncIIdentityStore, Finish_Reset)
        HRESULT ( STDMETHODCALLTYPE *Finish_Reset )( 
            __RPC__in AsyncIIdentityStore * This);
        
        END_INTERFACE
    } AsyncIIdentityStoreVtbl;

    interface AsyncIIdentityStore
    {
        CONST_VTBL struct AsyncIIdentityStoreVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define AsyncIIdentityStore_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define AsyncIIdentityStore_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define AsyncIIdentityStore_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define AsyncIIdentityStore_Begin_GetCount(This)	\
    ( (This)->lpVtbl -> Begin_GetCount(This) ) 

#define AsyncIIdentityStore_Finish_GetCount(This,pdwProviders)	\
    ( (This)->lpVtbl -> Finish_GetCount(This,pdwProviders) ) 

#define AsyncIIdentityStore_Begin_GetAt(This,dwProvider,pProvGuid)	\
    ( (This)->lpVtbl -> Begin_GetAt(This,dwProvider,pProvGuid) ) 

#define AsyncIIdentityStore_Finish_GetAt(This,pProvGuid,ppIdentityProvider)	\
    ( (This)->lpVtbl -> Finish_GetAt(This,pProvGuid,ppIdentityProvider) ) 

#define AsyncIIdentityStore_Begin_AddToCache(This,lpszUniqueID,ProviderGUID)	\
    ( (This)->lpVtbl -> Begin_AddToCache(This,lpszUniqueID,ProviderGUID) ) 

#define AsyncIIdentityStore_Finish_AddToCache(This)	\
    ( (This)->lpVtbl -> Finish_AddToCache(This) ) 

#define AsyncIIdentityStore_Begin_ConvertToSid(This,lpszUniqueID,ProviderGUID,cbSid,pSid)	\
    ( (This)->lpVtbl -> Begin_ConvertToSid(This,lpszUniqueID,ProviderGUID,cbSid,pSid) ) 

#define AsyncIIdentityStore_Finish_ConvertToSid(This,pSid,pcbRequiredSid)	\
    ( (This)->lpVtbl -> Finish_ConvertToSid(This,pSid,pcbRequiredSid) ) 

#define AsyncIIdentityStore_Begin_EnumerateIdentities(This,eIdentityType,pFilterkey,pFilterPropVarValue)	\
    ( (This)->lpVtbl -> Begin_EnumerateIdentities(This,eIdentityType,pFilterkey,pFilterPropVarValue) ) 

#define AsyncIIdentityStore_Finish_EnumerateIdentities(This,ppIdentityEnum)	\
    ( (This)->lpVtbl -> Finish_EnumerateIdentities(This,ppIdentityEnum) ) 

#define AsyncIIdentityStore_Begin_Reset(This)	\
    ( (This)->lpVtbl -> Begin_Reset(This) ) 

#define AsyncIIdentityStore_Finish_Reset(This)	\
    ( (This)->lpVtbl -> Finish_Reset(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __AsyncIIdentityStore_INTERFACE_DEFINED__ */


#ifndef __IIdentityStoreEx_INTERFACE_DEFINED__
#define __IIdentityStoreEx_INTERFACE_DEFINED__

/* interface IIdentityStoreEx */
/* [unique][helpstring][async_uuid][uuid][object] */ 


EXTERN_C const IID IID_IIdentityStoreEx;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("f9f9eb98-8f7f-4e38-9577-6980114ce32b")
    IIdentityStoreEx : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE CreateConnectedIdentity( 
            /* [unique][in] */ __RPC__in_opt LPCWSTR LocalName,
            /* [in] */ __RPC__in LPCWSTR ConnectedName,
            /* [in] */ __RPC__in REFGUID ProviderGUID) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE DeleteConnectedIdentity( 
            /* [in] */ __RPC__in LPCWSTR ConnectedName,
            /* [in] */ __RPC__in REFGUID ProviderGUID) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IIdentityStoreExVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IIdentityStoreEx * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IIdentityStoreEx * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IIdentityStoreEx * This);
        
        DECLSPEC_XFGVIRT(IIdentityStoreEx, CreateConnectedIdentity)
        HRESULT ( STDMETHODCALLTYPE *CreateConnectedIdentity )( 
            __RPC__in IIdentityStoreEx * This,
            /* [unique][in] */ __RPC__in_opt LPCWSTR LocalName,
            /* [in] */ __RPC__in LPCWSTR ConnectedName,
            /* [in] */ __RPC__in REFGUID ProviderGUID);
        
        DECLSPEC_XFGVIRT(IIdentityStoreEx, DeleteConnectedIdentity)
        HRESULT ( STDMETHODCALLTYPE *DeleteConnectedIdentity )( 
            __RPC__in IIdentityStoreEx * This,
            /* [in] */ __RPC__in LPCWSTR ConnectedName,
            /* [in] */ __RPC__in REFGUID ProviderGUID);
        
        END_INTERFACE
    } IIdentityStoreExVtbl;

    interface IIdentityStoreEx
    {
        CONST_VTBL struct IIdentityStoreExVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IIdentityStoreEx_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IIdentityStoreEx_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IIdentityStoreEx_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IIdentityStoreEx_CreateConnectedIdentity(This,LocalName,ConnectedName,ProviderGUID)	\
    ( (This)->lpVtbl -> CreateConnectedIdentity(This,LocalName,ConnectedName,ProviderGUID) ) 

#define IIdentityStoreEx_DeleteConnectedIdentity(This,ConnectedName,ProviderGUID)	\
    ( (This)->lpVtbl -> DeleteConnectedIdentity(This,ConnectedName,ProviderGUID) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IIdentityStoreEx_INTERFACE_DEFINED__ */


#ifndef __AsyncIIdentityStoreEx_INTERFACE_DEFINED__
#define __AsyncIIdentityStoreEx_INTERFACE_DEFINED__

/* interface AsyncIIdentityStoreEx */
/* [uuid][unique][helpstring][object] */ 


EXTERN_C const IID IID_AsyncIIdentityStoreEx;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("fca3af9a-8a07-4eae-8632-ec3de658a36a")
    AsyncIIdentityStoreEx : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Begin_CreateConnectedIdentity( 
            /* [unique][in] */ __RPC__in_opt LPCWSTR LocalName,
            /* [in] */ __RPC__in LPCWSTR ConnectedName,
            /* [in] */ __RPC__in REFGUID ProviderGUID) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Finish_CreateConnectedIdentity( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Begin_DeleteConnectedIdentity( 
            /* [in] */ __RPC__in LPCWSTR ConnectedName,
            /* [in] */ __RPC__in REFGUID ProviderGUID) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Finish_DeleteConnectedIdentity( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct AsyncIIdentityStoreExVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in AsyncIIdentityStoreEx * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in AsyncIIdentityStoreEx * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in AsyncIIdentityStoreEx * This);
        
        DECLSPEC_XFGVIRT(AsyncIIdentityStoreEx, Begin_CreateConnectedIdentity)
        HRESULT ( STDMETHODCALLTYPE *Begin_CreateConnectedIdentity )( 
            __RPC__in AsyncIIdentityStoreEx * This,
            /* [unique][in] */ __RPC__in_opt LPCWSTR LocalName,
            /* [in] */ __RPC__in LPCWSTR ConnectedName,
            /* [in] */ __RPC__in REFGUID ProviderGUID);
        
        DECLSPEC_XFGVIRT(AsyncIIdentityStoreEx, Finish_CreateConnectedIdentity)
        HRESULT ( STDMETHODCALLTYPE *Finish_CreateConnectedIdentity )( 
            __RPC__in AsyncIIdentityStoreEx * This);
        
        DECLSPEC_XFGVIRT(AsyncIIdentityStoreEx, Begin_DeleteConnectedIdentity)
        HRESULT ( STDMETHODCALLTYPE *Begin_DeleteConnectedIdentity )( 
            __RPC__in AsyncIIdentityStoreEx * This,
            /* [in] */ __RPC__in LPCWSTR ConnectedName,
            /* [in] */ __RPC__in REFGUID ProviderGUID);
        
        DECLSPEC_XFGVIRT(AsyncIIdentityStoreEx, Finish_DeleteConnectedIdentity)
        HRESULT ( STDMETHODCALLTYPE *Finish_DeleteConnectedIdentity )( 
            __RPC__in AsyncIIdentityStoreEx * This);
        
        END_INTERFACE
    } AsyncIIdentityStoreExVtbl;

    interface AsyncIIdentityStoreEx
    {
        CONST_VTBL struct AsyncIIdentityStoreExVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define AsyncIIdentityStoreEx_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define AsyncIIdentityStoreEx_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define AsyncIIdentityStoreEx_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define AsyncIIdentityStoreEx_Begin_CreateConnectedIdentity(This,LocalName,ConnectedName,ProviderGUID)	\
    ( (This)->lpVtbl -> Begin_CreateConnectedIdentity(This,LocalName,ConnectedName,ProviderGUID) ) 

#define AsyncIIdentityStoreEx_Finish_CreateConnectedIdentity(This)	\
    ( (This)->lpVtbl -> Finish_CreateConnectedIdentity(This) ) 

#define AsyncIIdentityStoreEx_Begin_DeleteConnectedIdentity(This,ConnectedName,ProviderGUID)	\
    ( (This)->lpVtbl -> Begin_DeleteConnectedIdentity(This,ConnectedName,ProviderGUID) ) 

#define AsyncIIdentityStoreEx_Finish_DeleteConnectedIdentity(This)	\
    ( (This)->lpVtbl -> Finish_DeleteConnectedIdentity(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __AsyncIIdentityStoreEx_INTERFACE_DEFINED__ */



#ifndef __IdentityStoreLib_LIBRARY_DEFINED__
#define __IdentityStoreLib_LIBRARY_DEFINED__

/* library IdentityStoreLib */
/* [helpstring][version][uuid] */ 


EXTERN_C const IID LIBID_IdentityStoreLib;

EXTERN_C const CLSID CLSID_CoClassIdentityStore;

#ifdef __cplusplus

class DECLSPEC_UUID("30d49246-d217-465f-b00b-ac9ddd652eb7")
CoClassIdentityStore;
#endif

EXTERN_C const CLSID CLSID_CIdentityProfileHandler;

#ifdef __cplusplus

class DECLSPEC_UUID("ecf5bf46-e3b6-449a-b56b-43f58f867814")
CIdentityProfileHandler;
#endif
#endif /* __IdentityStoreLib_LIBRARY_DEFINED__ */

/* interface __MIDL_itf_identitystore_0000_0003 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_identitystore_0000_0003_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_identitystore_0000_0003_v0_0_s_ifspec;

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


