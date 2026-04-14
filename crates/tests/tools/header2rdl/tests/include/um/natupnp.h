

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

#ifndef __natupnp_h__
#define __natupnp_h__

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

#ifndef __IUPnPNAT_FWD_DEFINED__
#define __IUPnPNAT_FWD_DEFINED__
typedef interface IUPnPNAT IUPnPNAT;

#endif 	/* __IUPnPNAT_FWD_DEFINED__ */


#ifndef __INATEventManager_FWD_DEFINED__
#define __INATEventManager_FWD_DEFINED__
typedef interface INATEventManager INATEventManager;

#endif 	/* __INATEventManager_FWD_DEFINED__ */


#ifndef __INATExternalIPAddressCallback_FWD_DEFINED__
#define __INATExternalIPAddressCallback_FWD_DEFINED__
typedef interface INATExternalIPAddressCallback INATExternalIPAddressCallback;

#endif 	/* __INATExternalIPAddressCallback_FWD_DEFINED__ */


#ifndef __INATNumberOfEntriesCallback_FWD_DEFINED__
#define __INATNumberOfEntriesCallback_FWD_DEFINED__
typedef interface INATNumberOfEntriesCallback INATNumberOfEntriesCallback;

#endif 	/* __INATNumberOfEntriesCallback_FWD_DEFINED__ */


#ifndef __IDynamicPortMappingCollection_FWD_DEFINED__
#define __IDynamicPortMappingCollection_FWD_DEFINED__
typedef interface IDynamicPortMappingCollection IDynamicPortMappingCollection;

#endif 	/* __IDynamicPortMappingCollection_FWD_DEFINED__ */


#ifndef __IDynamicPortMapping_FWD_DEFINED__
#define __IDynamicPortMapping_FWD_DEFINED__
typedef interface IDynamicPortMapping IDynamicPortMapping;

#endif 	/* __IDynamicPortMapping_FWD_DEFINED__ */


#ifndef __IStaticPortMappingCollection_FWD_DEFINED__
#define __IStaticPortMappingCollection_FWD_DEFINED__
typedef interface IStaticPortMappingCollection IStaticPortMappingCollection;

#endif 	/* __IStaticPortMappingCollection_FWD_DEFINED__ */


#ifndef __IStaticPortMapping_FWD_DEFINED__
#define __IStaticPortMapping_FWD_DEFINED__
typedef interface IStaticPortMapping IStaticPortMapping;

#endif 	/* __IStaticPortMapping_FWD_DEFINED__ */


#ifndef __UPnPNAT_FWD_DEFINED__
#define __UPnPNAT_FWD_DEFINED__

#ifdef __cplusplus
typedef class UPnPNAT UPnPNAT;
#else
typedef struct UPnPNAT UPnPNAT;
#endif /* __cplusplus */

#endif 	/* __UPnPNAT_FWD_DEFINED__ */


/* header files for imported files */
#include "oaidl.h"
#include "ocidl.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_natupnp_0000_0000 */
/* [local] */ 

//+-------------------------------------------------------------------------
//
//  Microsoft Windows
//  Copyright (C) Microsoft Corporation, 1992-2001.
//
//--------------------------------------------------------------------------
//  MODULE: natupnp.h
//
#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)








extern RPC_IF_HANDLE __MIDL_itf_natupnp_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_natupnp_0000_0000_v0_0_s_ifspec;

#ifndef __IUPnPNAT_INTERFACE_DEFINED__
#define __IUPnPNAT_INTERFACE_DEFINED__

/* interface IUPnPNAT */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IUPnPNAT;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("B171C812-CC76-485A-94D8-B6B3A2794E99")
    IUPnPNAT : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_StaticPortMappingCollection( 
            /* [retval][out] */ __RPC__deref_out_opt IStaticPortMappingCollection **ppSPMs) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_DynamicPortMappingCollection( 
            /* [retval][out] */ __RPC__deref_out_opt IDynamicPortMappingCollection **ppDPMs) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_NATEventManager( 
            /* [retval][out] */ __RPC__deref_out_opt INATEventManager **ppNEM) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IUPnPNATVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IUPnPNAT * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IUPnPNAT * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IUPnPNAT * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IUPnPNAT * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IUPnPNAT * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IUPnPNAT * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IUPnPNAT * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(IUPnPNAT, get_StaticPortMappingCollection)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_StaticPortMappingCollection )( 
            __RPC__in IUPnPNAT * This,
            /* [retval][out] */ __RPC__deref_out_opt IStaticPortMappingCollection **ppSPMs);
        
        DECLSPEC_XFGVIRT(IUPnPNAT, get_DynamicPortMappingCollection)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_DynamicPortMappingCollection )( 
            __RPC__in IUPnPNAT * This,
            /* [retval][out] */ __RPC__deref_out_opt IDynamicPortMappingCollection **ppDPMs);
        
        DECLSPEC_XFGVIRT(IUPnPNAT, get_NATEventManager)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_NATEventManager )( 
            __RPC__in IUPnPNAT * This,
            /* [retval][out] */ __RPC__deref_out_opt INATEventManager **ppNEM);
        
        END_INTERFACE
    } IUPnPNATVtbl;

    interface IUPnPNAT
    {
        CONST_VTBL struct IUPnPNATVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IUPnPNAT_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IUPnPNAT_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IUPnPNAT_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IUPnPNAT_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IUPnPNAT_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IUPnPNAT_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IUPnPNAT_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IUPnPNAT_get_StaticPortMappingCollection(This,ppSPMs)	\
    ( (This)->lpVtbl -> get_StaticPortMappingCollection(This,ppSPMs) ) 

#define IUPnPNAT_get_DynamicPortMappingCollection(This,ppDPMs)	\
    ( (This)->lpVtbl -> get_DynamicPortMappingCollection(This,ppDPMs) ) 

#define IUPnPNAT_get_NATEventManager(This,ppNEM)	\
    ( (This)->lpVtbl -> get_NATEventManager(This,ppNEM) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IUPnPNAT_INTERFACE_DEFINED__ */


#ifndef __INATEventManager_INTERFACE_DEFINED__
#define __INATEventManager_INTERFACE_DEFINED__

/* interface INATEventManager */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_INATEventManager;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("624BD588-9060-4109-B0B0-1ADBBCAC32DF")
    INATEventManager : public IDispatch
    {
    public:
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_ExternalIPAddressCallback( 
            /* [in] */ __RPC__in_opt IUnknown *pUnk) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_NumberOfEntriesCallback( 
            /* [in] */ __RPC__in_opt IUnknown *pUnk) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct INATEventManagerVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in INATEventManager * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in INATEventManager * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in INATEventManager * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in INATEventManager * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in INATEventManager * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in INATEventManager * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            INATEventManager * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(INATEventManager, put_ExternalIPAddressCallback)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_ExternalIPAddressCallback )( 
            __RPC__in INATEventManager * This,
            /* [in] */ __RPC__in_opt IUnknown *pUnk);
        
        DECLSPEC_XFGVIRT(INATEventManager, put_NumberOfEntriesCallback)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_NumberOfEntriesCallback )( 
            __RPC__in INATEventManager * This,
            /* [in] */ __RPC__in_opt IUnknown *pUnk);
        
        END_INTERFACE
    } INATEventManagerVtbl;

    interface INATEventManager
    {
        CONST_VTBL struct INATEventManagerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define INATEventManager_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define INATEventManager_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define INATEventManager_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define INATEventManager_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define INATEventManager_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define INATEventManager_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define INATEventManager_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define INATEventManager_put_ExternalIPAddressCallback(This,pUnk)	\
    ( (This)->lpVtbl -> put_ExternalIPAddressCallback(This,pUnk) ) 

#define INATEventManager_put_NumberOfEntriesCallback(This,pUnk)	\
    ( (This)->lpVtbl -> put_NumberOfEntriesCallback(This,pUnk) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __INATEventManager_INTERFACE_DEFINED__ */


#ifndef __INATExternalIPAddressCallback_INTERFACE_DEFINED__
#define __INATExternalIPAddressCallback_INTERFACE_DEFINED__

/* interface INATExternalIPAddressCallback */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_INATExternalIPAddressCallback;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("9C416740-A34E-446F-BA06-ABD04C3149AE")
    INATExternalIPAddressCallback : public IUnknown
    {
    public:
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE NewExternalIPAddress( 
            /* [in] */ __RPC__in BSTR bstrNewExternalIPAddress) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct INATExternalIPAddressCallbackVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in INATExternalIPAddressCallback * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in INATExternalIPAddressCallback * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in INATExternalIPAddressCallback * This);
        
        DECLSPEC_XFGVIRT(INATExternalIPAddressCallback, NewExternalIPAddress)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *NewExternalIPAddress )( 
            __RPC__in INATExternalIPAddressCallback * This,
            /* [in] */ __RPC__in BSTR bstrNewExternalIPAddress);
        
        END_INTERFACE
    } INATExternalIPAddressCallbackVtbl;

    interface INATExternalIPAddressCallback
    {
        CONST_VTBL struct INATExternalIPAddressCallbackVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define INATExternalIPAddressCallback_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define INATExternalIPAddressCallback_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define INATExternalIPAddressCallback_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define INATExternalIPAddressCallback_NewExternalIPAddress(This,bstrNewExternalIPAddress)	\
    ( (This)->lpVtbl -> NewExternalIPAddress(This,bstrNewExternalIPAddress) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __INATExternalIPAddressCallback_INTERFACE_DEFINED__ */


#ifndef __INATNumberOfEntriesCallback_INTERFACE_DEFINED__
#define __INATNumberOfEntriesCallback_INTERFACE_DEFINED__

/* interface INATNumberOfEntriesCallback */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_INATNumberOfEntriesCallback;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("C83A0A74-91EE-41B6-B67A-67E0F00BBD78")
    INATNumberOfEntriesCallback : public IUnknown
    {
    public:
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE NewNumberOfEntries( 
            /* [in] */ long lNewNumberOfEntries) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct INATNumberOfEntriesCallbackVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in INATNumberOfEntriesCallback * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in INATNumberOfEntriesCallback * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in INATNumberOfEntriesCallback * This);
        
        DECLSPEC_XFGVIRT(INATNumberOfEntriesCallback, NewNumberOfEntries)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *NewNumberOfEntries )( 
            __RPC__in INATNumberOfEntriesCallback * This,
            /* [in] */ long lNewNumberOfEntries);
        
        END_INTERFACE
    } INATNumberOfEntriesCallbackVtbl;

    interface INATNumberOfEntriesCallback
    {
        CONST_VTBL struct INATNumberOfEntriesCallbackVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define INATNumberOfEntriesCallback_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define INATNumberOfEntriesCallback_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define INATNumberOfEntriesCallback_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define INATNumberOfEntriesCallback_NewNumberOfEntries(This,lNewNumberOfEntries)	\
    ( (This)->lpVtbl -> NewNumberOfEntries(This,lNewNumberOfEntries) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __INATNumberOfEntriesCallback_INTERFACE_DEFINED__ */


#ifndef __IDynamicPortMappingCollection_INTERFACE_DEFINED__
#define __IDynamicPortMappingCollection_INTERFACE_DEFINED__

/* interface IDynamicPortMappingCollection */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IDynamicPortMappingCollection;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("B60DE00F-156E-4E8D-9EC1-3A2342C10899")
    IDynamicPortMappingCollection : public IDispatch
    {
    public:
        virtual /* [restricted][hidden][helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get__NewEnum( 
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Item( 
            /* [in] */ __RPC__in BSTR bstrRemoteHost,
            /* [in] */ long lExternalPort,
            /* [in] */ __RPC__in BSTR bstrProtocol,
            /* [retval][out] */ __RPC__deref_out_opt IDynamicPortMapping **ppDPM) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Count( 
            /* [retval][out] */ __RPC__out long *pVal) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Remove( 
            /* [in] */ __RPC__in BSTR bstrRemoteHost,
            /* [in] */ long lExternalPort,
            /* [in] */ __RPC__in BSTR bstrProtocol) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Add( 
            /* [in] */ __RPC__in BSTR bstrRemoteHost,
            /* [in] */ long lExternalPort,
            /* [in] */ __RPC__in BSTR bstrProtocol,
            /* [in] */ long lInternalPort,
            /* [in] */ __RPC__in BSTR bstrInternalClient,
            /* [in] */ VARIANT_BOOL bEnabled,
            /* [in] */ __RPC__in BSTR bstrDescription,
            /* [in] */ long lLeaseDuration,
            /* [retval][out] */ __RPC__deref_out_opt IDynamicPortMapping **ppDPM) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDynamicPortMappingCollectionVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IDynamicPortMappingCollection * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IDynamicPortMappingCollection * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IDynamicPortMappingCollection * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IDynamicPortMappingCollection * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IDynamicPortMappingCollection * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IDynamicPortMappingCollection * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IDynamicPortMappingCollection * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(IDynamicPortMappingCollection, get__NewEnum)
        /* [restricted][hidden][helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get__NewEnum )( 
            __RPC__in IDynamicPortMappingCollection * This,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **pVal);
        
        DECLSPEC_XFGVIRT(IDynamicPortMappingCollection, get_Item)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Item )( 
            __RPC__in IDynamicPortMappingCollection * This,
            /* [in] */ __RPC__in BSTR bstrRemoteHost,
            /* [in] */ long lExternalPort,
            /* [in] */ __RPC__in BSTR bstrProtocol,
            /* [retval][out] */ __RPC__deref_out_opt IDynamicPortMapping **ppDPM);
        
        DECLSPEC_XFGVIRT(IDynamicPortMappingCollection, get_Count)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Count )( 
            __RPC__in IDynamicPortMappingCollection * This,
            /* [retval][out] */ __RPC__out long *pVal);
        
        DECLSPEC_XFGVIRT(IDynamicPortMappingCollection, Remove)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Remove )( 
            __RPC__in IDynamicPortMappingCollection * This,
            /* [in] */ __RPC__in BSTR bstrRemoteHost,
            /* [in] */ long lExternalPort,
            /* [in] */ __RPC__in BSTR bstrProtocol);
        
        DECLSPEC_XFGVIRT(IDynamicPortMappingCollection, Add)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Add )( 
            __RPC__in IDynamicPortMappingCollection * This,
            /* [in] */ __RPC__in BSTR bstrRemoteHost,
            /* [in] */ long lExternalPort,
            /* [in] */ __RPC__in BSTR bstrProtocol,
            /* [in] */ long lInternalPort,
            /* [in] */ __RPC__in BSTR bstrInternalClient,
            /* [in] */ VARIANT_BOOL bEnabled,
            /* [in] */ __RPC__in BSTR bstrDescription,
            /* [in] */ long lLeaseDuration,
            /* [retval][out] */ __RPC__deref_out_opt IDynamicPortMapping **ppDPM);
        
        END_INTERFACE
    } IDynamicPortMappingCollectionVtbl;

    interface IDynamicPortMappingCollection
    {
        CONST_VTBL struct IDynamicPortMappingCollectionVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDynamicPortMappingCollection_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDynamicPortMappingCollection_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDynamicPortMappingCollection_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDynamicPortMappingCollection_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IDynamicPortMappingCollection_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IDynamicPortMappingCollection_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IDynamicPortMappingCollection_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IDynamicPortMappingCollection_get__NewEnum(This,pVal)	\
    ( (This)->lpVtbl -> get__NewEnum(This,pVal) ) 

#define IDynamicPortMappingCollection_get_Item(This,bstrRemoteHost,lExternalPort,bstrProtocol,ppDPM)	\
    ( (This)->lpVtbl -> get_Item(This,bstrRemoteHost,lExternalPort,bstrProtocol,ppDPM) ) 

#define IDynamicPortMappingCollection_get_Count(This,pVal)	\
    ( (This)->lpVtbl -> get_Count(This,pVal) ) 

#define IDynamicPortMappingCollection_Remove(This,bstrRemoteHost,lExternalPort,bstrProtocol)	\
    ( (This)->lpVtbl -> Remove(This,bstrRemoteHost,lExternalPort,bstrProtocol) ) 

#define IDynamicPortMappingCollection_Add(This,bstrRemoteHost,lExternalPort,bstrProtocol,lInternalPort,bstrInternalClient,bEnabled,bstrDescription,lLeaseDuration,ppDPM)	\
    ( (This)->lpVtbl -> Add(This,bstrRemoteHost,lExternalPort,bstrProtocol,lInternalPort,bstrInternalClient,bEnabled,bstrDescription,lLeaseDuration,ppDPM) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDynamicPortMappingCollection_INTERFACE_DEFINED__ */


#ifndef __IDynamicPortMapping_INTERFACE_DEFINED__
#define __IDynamicPortMapping_INTERFACE_DEFINED__

/* interface IDynamicPortMapping */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IDynamicPortMapping;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("4FC80282-23B6-4378-9A27-CD8F17C9400C")
    IDynamicPortMapping : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_ExternalIPAddress( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_RemoteHost( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_ExternalPort( 
            /* [retval][out] */ __RPC__out long *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Protocol( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_InternalPort( 
            /* [retval][out] */ __RPC__out long *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_InternalClient( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Enabled( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Description( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_LeaseDuration( 
            /* [retval][out] */ __RPC__out long *pVal) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE RenewLease( 
            /* [in] */ long lLeaseDurationDesired,
            /* [retval][out] */ __RPC__out long *pLeaseDurationReturned) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE EditInternalClient( 
            /* [in] */ __RPC__in BSTR bstrInternalClient) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Enable( 
            /* [in] */ VARIANT_BOOL vb) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE EditDescription( 
            /* [in] */ __RPC__in BSTR bstrDescription) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE EditInternalPort( 
            /* [in] */ long lInternalPort) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDynamicPortMappingVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IDynamicPortMapping * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IDynamicPortMapping * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IDynamicPortMapping * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IDynamicPortMapping * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IDynamicPortMapping * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IDynamicPortMapping * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IDynamicPortMapping * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(IDynamicPortMapping, get_ExternalIPAddress)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ExternalIPAddress )( 
            __RPC__in IDynamicPortMapping * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(IDynamicPortMapping, get_RemoteHost)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_RemoteHost )( 
            __RPC__in IDynamicPortMapping * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(IDynamicPortMapping, get_ExternalPort)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ExternalPort )( 
            __RPC__in IDynamicPortMapping * This,
            /* [retval][out] */ __RPC__out long *pVal);
        
        DECLSPEC_XFGVIRT(IDynamicPortMapping, get_Protocol)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Protocol )( 
            __RPC__in IDynamicPortMapping * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(IDynamicPortMapping, get_InternalPort)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_InternalPort )( 
            __RPC__in IDynamicPortMapping * This,
            /* [retval][out] */ __RPC__out long *pVal);
        
        DECLSPEC_XFGVIRT(IDynamicPortMapping, get_InternalClient)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_InternalClient )( 
            __RPC__in IDynamicPortMapping * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(IDynamicPortMapping, get_Enabled)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Enabled )( 
            __RPC__in IDynamicPortMapping * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pVal);
        
        DECLSPEC_XFGVIRT(IDynamicPortMapping, get_Description)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Description )( 
            __RPC__in IDynamicPortMapping * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(IDynamicPortMapping, get_LeaseDuration)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_LeaseDuration )( 
            __RPC__in IDynamicPortMapping * This,
            /* [retval][out] */ __RPC__out long *pVal);
        
        DECLSPEC_XFGVIRT(IDynamicPortMapping, RenewLease)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *RenewLease )( 
            __RPC__in IDynamicPortMapping * This,
            /* [in] */ long lLeaseDurationDesired,
            /* [retval][out] */ __RPC__out long *pLeaseDurationReturned);
        
        DECLSPEC_XFGVIRT(IDynamicPortMapping, EditInternalClient)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *EditInternalClient )( 
            __RPC__in IDynamicPortMapping * This,
            /* [in] */ __RPC__in BSTR bstrInternalClient);
        
        DECLSPEC_XFGVIRT(IDynamicPortMapping, Enable)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Enable )( 
            __RPC__in IDynamicPortMapping * This,
            /* [in] */ VARIANT_BOOL vb);
        
        DECLSPEC_XFGVIRT(IDynamicPortMapping, EditDescription)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *EditDescription )( 
            __RPC__in IDynamicPortMapping * This,
            /* [in] */ __RPC__in BSTR bstrDescription);
        
        DECLSPEC_XFGVIRT(IDynamicPortMapping, EditInternalPort)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *EditInternalPort )( 
            __RPC__in IDynamicPortMapping * This,
            /* [in] */ long lInternalPort);
        
        END_INTERFACE
    } IDynamicPortMappingVtbl;

    interface IDynamicPortMapping
    {
        CONST_VTBL struct IDynamicPortMappingVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDynamicPortMapping_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDynamicPortMapping_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDynamicPortMapping_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDynamicPortMapping_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IDynamicPortMapping_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IDynamicPortMapping_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IDynamicPortMapping_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IDynamicPortMapping_get_ExternalIPAddress(This,pVal)	\
    ( (This)->lpVtbl -> get_ExternalIPAddress(This,pVal) ) 

#define IDynamicPortMapping_get_RemoteHost(This,pVal)	\
    ( (This)->lpVtbl -> get_RemoteHost(This,pVal) ) 

#define IDynamicPortMapping_get_ExternalPort(This,pVal)	\
    ( (This)->lpVtbl -> get_ExternalPort(This,pVal) ) 

#define IDynamicPortMapping_get_Protocol(This,pVal)	\
    ( (This)->lpVtbl -> get_Protocol(This,pVal) ) 

#define IDynamicPortMapping_get_InternalPort(This,pVal)	\
    ( (This)->lpVtbl -> get_InternalPort(This,pVal) ) 

#define IDynamicPortMapping_get_InternalClient(This,pVal)	\
    ( (This)->lpVtbl -> get_InternalClient(This,pVal) ) 

#define IDynamicPortMapping_get_Enabled(This,pVal)	\
    ( (This)->lpVtbl -> get_Enabled(This,pVal) ) 

#define IDynamicPortMapping_get_Description(This,pVal)	\
    ( (This)->lpVtbl -> get_Description(This,pVal) ) 

#define IDynamicPortMapping_get_LeaseDuration(This,pVal)	\
    ( (This)->lpVtbl -> get_LeaseDuration(This,pVal) ) 

#define IDynamicPortMapping_RenewLease(This,lLeaseDurationDesired,pLeaseDurationReturned)	\
    ( (This)->lpVtbl -> RenewLease(This,lLeaseDurationDesired,pLeaseDurationReturned) ) 

#define IDynamicPortMapping_EditInternalClient(This,bstrInternalClient)	\
    ( (This)->lpVtbl -> EditInternalClient(This,bstrInternalClient) ) 

#define IDynamicPortMapping_Enable(This,vb)	\
    ( (This)->lpVtbl -> Enable(This,vb) ) 

#define IDynamicPortMapping_EditDescription(This,bstrDescription)	\
    ( (This)->lpVtbl -> EditDescription(This,bstrDescription) ) 

#define IDynamicPortMapping_EditInternalPort(This,lInternalPort)	\
    ( (This)->lpVtbl -> EditInternalPort(This,lInternalPort) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDynamicPortMapping_INTERFACE_DEFINED__ */


#ifndef __IStaticPortMappingCollection_INTERFACE_DEFINED__
#define __IStaticPortMappingCollection_INTERFACE_DEFINED__

/* interface IStaticPortMappingCollection */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IStaticPortMappingCollection;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("CD1F3E77-66D6-4664-82C7-36DBB641D0F1")
    IStaticPortMappingCollection : public IDispatch
    {
    public:
        virtual /* [restricted][hidden][helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get__NewEnum( 
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Item( 
            /* [in] */ long lExternalPort,
            /* [in] */ __RPC__in BSTR bstrProtocol,
            /* [retval][out] */ __RPC__deref_out_opt IStaticPortMapping **ppSPM) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Count( 
            /* [retval][out] */ __RPC__out long *pVal) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Remove( 
            /* [in] */ long lExternalPort,
            /* [in] */ __RPC__in BSTR bstrProtocol) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Add( 
            /* [in] */ long lExternalPort,
            /* [in] */ __RPC__in BSTR bstrProtocol,
            /* [in] */ long lInternalPort,
            /* [in] */ __RPC__in BSTR bstrInternalClient,
            /* [in] */ VARIANT_BOOL bEnabled,
            /* [in] */ __RPC__in BSTR bstrDescription,
            /* [retval][out] */ __RPC__deref_out_opt IStaticPortMapping **ppSPM) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IStaticPortMappingCollectionVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IStaticPortMappingCollection * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IStaticPortMappingCollection * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IStaticPortMappingCollection * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IStaticPortMappingCollection * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IStaticPortMappingCollection * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IStaticPortMappingCollection * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IStaticPortMappingCollection * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(IStaticPortMappingCollection, get__NewEnum)
        /* [restricted][hidden][helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get__NewEnum )( 
            __RPC__in IStaticPortMappingCollection * This,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **pVal);
        
        DECLSPEC_XFGVIRT(IStaticPortMappingCollection, get_Item)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Item )( 
            __RPC__in IStaticPortMappingCollection * This,
            /* [in] */ long lExternalPort,
            /* [in] */ __RPC__in BSTR bstrProtocol,
            /* [retval][out] */ __RPC__deref_out_opt IStaticPortMapping **ppSPM);
        
        DECLSPEC_XFGVIRT(IStaticPortMappingCollection, get_Count)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Count )( 
            __RPC__in IStaticPortMappingCollection * This,
            /* [retval][out] */ __RPC__out long *pVal);
        
        DECLSPEC_XFGVIRT(IStaticPortMappingCollection, Remove)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Remove )( 
            __RPC__in IStaticPortMappingCollection * This,
            /* [in] */ long lExternalPort,
            /* [in] */ __RPC__in BSTR bstrProtocol);
        
        DECLSPEC_XFGVIRT(IStaticPortMappingCollection, Add)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Add )( 
            __RPC__in IStaticPortMappingCollection * This,
            /* [in] */ long lExternalPort,
            /* [in] */ __RPC__in BSTR bstrProtocol,
            /* [in] */ long lInternalPort,
            /* [in] */ __RPC__in BSTR bstrInternalClient,
            /* [in] */ VARIANT_BOOL bEnabled,
            /* [in] */ __RPC__in BSTR bstrDescription,
            /* [retval][out] */ __RPC__deref_out_opt IStaticPortMapping **ppSPM);
        
        END_INTERFACE
    } IStaticPortMappingCollectionVtbl;

    interface IStaticPortMappingCollection
    {
        CONST_VTBL struct IStaticPortMappingCollectionVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IStaticPortMappingCollection_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IStaticPortMappingCollection_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IStaticPortMappingCollection_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IStaticPortMappingCollection_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IStaticPortMappingCollection_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IStaticPortMappingCollection_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IStaticPortMappingCollection_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IStaticPortMappingCollection_get__NewEnum(This,pVal)	\
    ( (This)->lpVtbl -> get__NewEnum(This,pVal) ) 

#define IStaticPortMappingCollection_get_Item(This,lExternalPort,bstrProtocol,ppSPM)	\
    ( (This)->lpVtbl -> get_Item(This,lExternalPort,bstrProtocol,ppSPM) ) 

#define IStaticPortMappingCollection_get_Count(This,pVal)	\
    ( (This)->lpVtbl -> get_Count(This,pVal) ) 

#define IStaticPortMappingCollection_Remove(This,lExternalPort,bstrProtocol)	\
    ( (This)->lpVtbl -> Remove(This,lExternalPort,bstrProtocol) ) 

#define IStaticPortMappingCollection_Add(This,lExternalPort,bstrProtocol,lInternalPort,bstrInternalClient,bEnabled,bstrDescription,ppSPM)	\
    ( (This)->lpVtbl -> Add(This,lExternalPort,bstrProtocol,lInternalPort,bstrInternalClient,bEnabled,bstrDescription,ppSPM) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IStaticPortMappingCollection_INTERFACE_DEFINED__ */


#ifndef __IStaticPortMapping_INTERFACE_DEFINED__
#define __IStaticPortMapping_INTERFACE_DEFINED__

/* interface IStaticPortMapping */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IStaticPortMapping;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("6F10711F-729B-41E5-93B8-F21D0F818DF1")
    IStaticPortMapping : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_ExternalIPAddress( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_ExternalPort( 
            /* [retval][out] */ __RPC__out long *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_InternalPort( 
            /* [retval][out] */ __RPC__out long *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Protocol( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_InternalClient( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Enabled( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Description( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE EditInternalClient( 
            /* [in] */ __RPC__in BSTR bstrInternalClient) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Enable( 
            /* [in] */ VARIANT_BOOL vb) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE EditDescription( 
            /* [in] */ __RPC__in BSTR bstrDescription) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE EditInternalPort( 
            /* [in] */ long lInternalPort) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IStaticPortMappingVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IStaticPortMapping * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IStaticPortMapping * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IStaticPortMapping * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IStaticPortMapping * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IStaticPortMapping * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IStaticPortMapping * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IStaticPortMapping * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(IStaticPortMapping, get_ExternalIPAddress)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ExternalIPAddress )( 
            __RPC__in IStaticPortMapping * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(IStaticPortMapping, get_ExternalPort)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ExternalPort )( 
            __RPC__in IStaticPortMapping * This,
            /* [retval][out] */ __RPC__out long *pVal);
        
        DECLSPEC_XFGVIRT(IStaticPortMapping, get_InternalPort)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_InternalPort )( 
            __RPC__in IStaticPortMapping * This,
            /* [retval][out] */ __RPC__out long *pVal);
        
        DECLSPEC_XFGVIRT(IStaticPortMapping, get_Protocol)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Protocol )( 
            __RPC__in IStaticPortMapping * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(IStaticPortMapping, get_InternalClient)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_InternalClient )( 
            __RPC__in IStaticPortMapping * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(IStaticPortMapping, get_Enabled)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Enabled )( 
            __RPC__in IStaticPortMapping * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pVal);
        
        DECLSPEC_XFGVIRT(IStaticPortMapping, get_Description)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Description )( 
            __RPC__in IStaticPortMapping * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(IStaticPortMapping, EditInternalClient)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *EditInternalClient )( 
            __RPC__in IStaticPortMapping * This,
            /* [in] */ __RPC__in BSTR bstrInternalClient);
        
        DECLSPEC_XFGVIRT(IStaticPortMapping, Enable)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Enable )( 
            __RPC__in IStaticPortMapping * This,
            /* [in] */ VARIANT_BOOL vb);
        
        DECLSPEC_XFGVIRT(IStaticPortMapping, EditDescription)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *EditDescription )( 
            __RPC__in IStaticPortMapping * This,
            /* [in] */ __RPC__in BSTR bstrDescription);
        
        DECLSPEC_XFGVIRT(IStaticPortMapping, EditInternalPort)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *EditInternalPort )( 
            __RPC__in IStaticPortMapping * This,
            /* [in] */ long lInternalPort);
        
        END_INTERFACE
    } IStaticPortMappingVtbl;

    interface IStaticPortMapping
    {
        CONST_VTBL struct IStaticPortMappingVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IStaticPortMapping_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IStaticPortMapping_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IStaticPortMapping_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IStaticPortMapping_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IStaticPortMapping_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IStaticPortMapping_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IStaticPortMapping_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IStaticPortMapping_get_ExternalIPAddress(This,pVal)	\
    ( (This)->lpVtbl -> get_ExternalIPAddress(This,pVal) ) 

#define IStaticPortMapping_get_ExternalPort(This,pVal)	\
    ( (This)->lpVtbl -> get_ExternalPort(This,pVal) ) 

#define IStaticPortMapping_get_InternalPort(This,pVal)	\
    ( (This)->lpVtbl -> get_InternalPort(This,pVal) ) 

#define IStaticPortMapping_get_Protocol(This,pVal)	\
    ( (This)->lpVtbl -> get_Protocol(This,pVal) ) 

#define IStaticPortMapping_get_InternalClient(This,pVal)	\
    ( (This)->lpVtbl -> get_InternalClient(This,pVal) ) 

#define IStaticPortMapping_get_Enabled(This,pVal)	\
    ( (This)->lpVtbl -> get_Enabled(This,pVal) ) 

#define IStaticPortMapping_get_Description(This,pVal)	\
    ( (This)->lpVtbl -> get_Description(This,pVal) ) 

#define IStaticPortMapping_EditInternalClient(This,bstrInternalClient)	\
    ( (This)->lpVtbl -> EditInternalClient(This,bstrInternalClient) ) 

#define IStaticPortMapping_Enable(This,vb)	\
    ( (This)->lpVtbl -> Enable(This,vb) ) 

#define IStaticPortMapping_EditDescription(This,bstrDescription)	\
    ( (This)->lpVtbl -> EditDescription(This,bstrDescription) ) 

#define IStaticPortMapping_EditInternalPort(This,lInternalPort)	\
    ( (This)->lpVtbl -> EditInternalPort(This,lInternalPort) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IStaticPortMapping_INTERFACE_DEFINED__ */



#ifndef __NATUPNPLib_LIBRARY_DEFINED__
#define __NATUPNPLib_LIBRARY_DEFINED__

/* library NATUPNPLib */
/* [helpstring][version][uuid] */ 


EXTERN_C const IID LIBID_NATUPNPLib;

EXTERN_C const CLSID CLSID_UPnPNAT;

#ifdef __cplusplus

class DECLSPEC_UUID("AE1E00AA-3FD5-403C-8A27-2BBDC30CD0E1")
UPnPNAT;
#endif
#endif /* __NATUPNPLib_LIBRARY_DEFINED__ */

/* interface __MIDL_itf_natupnp_0000_0009 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_natupnp_0000_0009_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_natupnp_0000_0009_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

unsigned long             __RPC_USER  BSTR_UserSize(     __RPC__in unsigned long *, unsigned long            , __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserMarshal(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out BSTR * ); 
void                      __RPC_USER  BSTR_UserFree(     __RPC__in unsigned long *, __RPC__in BSTR * ); 

unsigned long             __RPC_USER  BSTR_UserSize64(     __RPC__in unsigned long *, unsigned long            , __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserMarshal64(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out BSTR * ); 
void                      __RPC_USER  BSTR_UserFree64(     __RPC__in unsigned long *, __RPC__in BSTR * ); 

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


