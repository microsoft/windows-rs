

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

#ifndef __mdhcp_h__
#define __mdhcp_h__

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

#ifndef __IMcastScope_FWD_DEFINED__
#define __IMcastScope_FWD_DEFINED__
typedef interface IMcastScope IMcastScope;

#endif 	/* __IMcastScope_FWD_DEFINED__ */


#ifndef __IMcastLeaseInfo_FWD_DEFINED__
#define __IMcastLeaseInfo_FWD_DEFINED__
typedef interface IMcastLeaseInfo IMcastLeaseInfo;

#endif 	/* __IMcastLeaseInfo_FWD_DEFINED__ */


#ifndef __IEnumMcastScope_FWD_DEFINED__
#define __IEnumMcastScope_FWD_DEFINED__
typedef interface IEnumMcastScope IEnumMcastScope;

#endif 	/* __IEnumMcastScope_FWD_DEFINED__ */


#ifndef __IMcastAddressAllocation_FWD_DEFINED__
#define __IMcastAddressAllocation_FWD_DEFINED__
typedef interface IMcastAddressAllocation IMcastAddressAllocation;

#endif 	/* __IMcastAddressAllocation_FWD_DEFINED__ */


#ifndef __IMcastScope_FWD_DEFINED__
#define __IMcastScope_FWD_DEFINED__
typedef interface IMcastScope IMcastScope;

#endif 	/* __IMcastScope_FWD_DEFINED__ */


#ifndef __IMcastLeaseInfo_FWD_DEFINED__
#define __IMcastLeaseInfo_FWD_DEFINED__
typedef interface IMcastLeaseInfo IMcastLeaseInfo;

#endif 	/* __IMcastLeaseInfo_FWD_DEFINED__ */


#ifndef __IEnumMcastScope_FWD_DEFINED__
#define __IEnumMcastScope_FWD_DEFINED__
typedef interface IEnumMcastScope IEnumMcastScope;

#endif 	/* __IEnumMcastScope_FWD_DEFINED__ */


#ifndef __IMcastAddressAllocation_FWD_DEFINED__
#define __IMcastAddressAllocation_FWD_DEFINED__
typedef interface IMcastAddressAllocation IMcastAddressAllocation;

#endif 	/* __IMcastAddressAllocation_FWD_DEFINED__ */


#ifndef __McastAddressAllocation_FWD_DEFINED__
#define __McastAddressAllocation_FWD_DEFINED__

#ifdef __cplusplus
typedef class McastAddressAllocation McastAddressAllocation;
#else
typedef struct McastAddressAllocation McastAddressAllocation;
#endif /* __cplusplus */

#endif 	/* __McastAddressAllocation_FWD_DEFINED__ */


/* header files for imported files */
#include "oaidl.h"
#include "ocidl.h"
#include "tapi3if.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_mdhcp_0000_0000 */
/* [local] */ 

/* Copyright (c) Microsoft Corporation. All rights reserved.*/
#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


extern RPC_IF_HANDLE __MIDL_itf_mdhcp_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mdhcp_0000_0000_v0_0_s_ifspec;

#ifndef __IMcastScope_INTERFACE_DEFINED__
#define __IMcastScope_INTERFACE_DEFINED__

/* interface IMcastScope */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IMcastScope;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("DF0DAEF4-A289-11D1-8697-006008B0E5D2")
    IMcastScope : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_ScopeID( 
            /* [retval][out] */ __RPC__out long *pID) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_ServerID( 
            /* [retval][out] */ __RPC__out long *pID) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_InterfaceID( 
            /* [retval][out] */ __RPC__out long *pID) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_ScopeDescription( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *ppDescription) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_TTL( 
            /* [retval][out] */ __RPC__out long *pTTL) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMcastScopeVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMcastScope * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMcastScope * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMcastScope * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IMcastScope * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IMcastScope * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IMcastScope * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IMcastScope * This,
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
        
        DECLSPEC_XFGVIRT(IMcastScope, get_ScopeID)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ScopeID )( 
            __RPC__in IMcastScope * This,
            /* [retval][out] */ __RPC__out long *pID);
        
        DECLSPEC_XFGVIRT(IMcastScope, get_ServerID)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ServerID )( 
            __RPC__in IMcastScope * This,
            /* [retval][out] */ __RPC__out long *pID);
        
        DECLSPEC_XFGVIRT(IMcastScope, get_InterfaceID)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_InterfaceID )( 
            __RPC__in IMcastScope * This,
            /* [retval][out] */ __RPC__out long *pID);
        
        DECLSPEC_XFGVIRT(IMcastScope, get_ScopeDescription)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ScopeDescription )( 
            __RPC__in IMcastScope * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *ppDescription);
        
        DECLSPEC_XFGVIRT(IMcastScope, get_TTL)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_TTL )( 
            __RPC__in IMcastScope * This,
            /* [retval][out] */ __RPC__out long *pTTL);
        
        END_INTERFACE
    } IMcastScopeVtbl;

    interface IMcastScope
    {
        CONST_VTBL struct IMcastScopeVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMcastScope_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMcastScope_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMcastScope_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMcastScope_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IMcastScope_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IMcastScope_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IMcastScope_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IMcastScope_get_ScopeID(This,pID)	\
    ( (This)->lpVtbl -> get_ScopeID(This,pID) ) 

#define IMcastScope_get_ServerID(This,pID)	\
    ( (This)->lpVtbl -> get_ServerID(This,pID) ) 

#define IMcastScope_get_InterfaceID(This,pID)	\
    ( (This)->lpVtbl -> get_InterfaceID(This,pID) ) 

#define IMcastScope_get_ScopeDescription(This,ppDescription)	\
    ( (This)->lpVtbl -> get_ScopeDescription(This,ppDescription) ) 

#define IMcastScope_get_TTL(This,pTTL)	\
    ( (This)->lpVtbl -> get_TTL(This,pTTL) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMcastScope_INTERFACE_DEFINED__ */


#ifndef __IMcastLeaseInfo_INTERFACE_DEFINED__
#define __IMcastLeaseInfo_INTERFACE_DEFINED__

/* interface IMcastLeaseInfo */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IMcastLeaseInfo;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("DF0DAEFD-A289-11D1-8697-006008B0E5D2")
    IMcastLeaseInfo : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_RequestID( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *ppRequestID) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_LeaseStartTime( 
            /* [retval][out] */ __RPC__out DATE *pTime) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_LeaseStartTime( 
            /* [in] */ DATE time) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_LeaseStopTime( 
            /* [retval][out] */ __RPC__out DATE *pTime) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_LeaseStopTime( 
            /* [in] */ DATE time) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_AddressCount( 
            /* [retval][out] */ __RPC__out long *pCount) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_ServerAddress( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *ppAddress) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_TTL( 
            /* [retval][out] */ __RPC__out long *pTTL) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Addresses( 
            /* [retval][out] */ __RPC__out VARIANT *pVariant) = 0;
        
        virtual /* [helpstring][hidden][id] */ HRESULT STDMETHODCALLTYPE EnumerateAddresses( 
            /* [retval][out] */ __RPC__deref_out_opt IEnumBstr **ppEnumAddresses) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMcastLeaseInfoVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMcastLeaseInfo * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMcastLeaseInfo * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMcastLeaseInfo * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IMcastLeaseInfo * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IMcastLeaseInfo * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IMcastLeaseInfo * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IMcastLeaseInfo * This,
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
        
        DECLSPEC_XFGVIRT(IMcastLeaseInfo, get_RequestID)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_RequestID )( 
            __RPC__in IMcastLeaseInfo * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *ppRequestID);
        
        DECLSPEC_XFGVIRT(IMcastLeaseInfo, get_LeaseStartTime)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_LeaseStartTime )( 
            __RPC__in IMcastLeaseInfo * This,
            /* [retval][out] */ __RPC__out DATE *pTime);
        
        DECLSPEC_XFGVIRT(IMcastLeaseInfo, put_LeaseStartTime)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_LeaseStartTime )( 
            __RPC__in IMcastLeaseInfo * This,
            /* [in] */ DATE time);
        
        DECLSPEC_XFGVIRT(IMcastLeaseInfo, get_LeaseStopTime)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_LeaseStopTime )( 
            __RPC__in IMcastLeaseInfo * This,
            /* [retval][out] */ __RPC__out DATE *pTime);
        
        DECLSPEC_XFGVIRT(IMcastLeaseInfo, put_LeaseStopTime)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_LeaseStopTime )( 
            __RPC__in IMcastLeaseInfo * This,
            /* [in] */ DATE time);
        
        DECLSPEC_XFGVIRT(IMcastLeaseInfo, get_AddressCount)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_AddressCount )( 
            __RPC__in IMcastLeaseInfo * This,
            /* [retval][out] */ __RPC__out long *pCount);
        
        DECLSPEC_XFGVIRT(IMcastLeaseInfo, get_ServerAddress)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ServerAddress )( 
            __RPC__in IMcastLeaseInfo * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *ppAddress);
        
        DECLSPEC_XFGVIRT(IMcastLeaseInfo, get_TTL)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_TTL )( 
            __RPC__in IMcastLeaseInfo * This,
            /* [retval][out] */ __RPC__out long *pTTL);
        
        DECLSPEC_XFGVIRT(IMcastLeaseInfo, get_Addresses)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Addresses )( 
            __RPC__in IMcastLeaseInfo * This,
            /* [retval][out] */ __RPC__out VARIANT *pVariant);
        
        DECLSPEC_XFGVIRT(IMcastLeaseInfo, EnumerateAddresses)
        /* [helpstring][hidden][id] */ HRESULT ( STDMETHODCALLTYPE *EnumerateAddresses )( 
            __RPC__in IMcastLeaseInfo * This,
            /* [retval][out] */ __RPC__deref_out_opt IEnumBstr **ppEnumAddresses);
        
        END_INTERFACE
    } IMcastLeaseInfoVtbl;

    interface IMcastLeaseInfo
    {
        CONST_VTBL struct IMcastLeaseInfoVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMcastLeaseInfo_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMcastLeaseInfo_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMcastLeaseInfo_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMcastLeaseInfo_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IMcastLeaseInfo_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IMcastLeaseInfo_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IMcastLeaseInfo_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IMcastLeaseInfo_get_RequestID(This,ppRequestID)	\
    ( (This)->lpVtbl -> get_RequestID(This,ppRequestID) ) 

#define IMcastLeaseInfo_get_LeaseStartTime(This,pTime)	\
    ( (This)->lpVtbl -> get_LeaseStartTime(This,pTime) ) 

#define IMcastLeaseInfo_put_LeaseStartTime(This,time)	\
    ( (This)->lpVtbl -> put_LeaseStartTime(This,time) ) 

#define IMcastLeaseInfo_get_LeaseStopTime(This,pTime)	\
    ( (This)->lpVtbl -> get_LeaseStopTime(This,pTime) ) 

#define IMcastLeaseInfo_put_LeaseStopTime(This,time)	\
    ( (This)->lpVtbl -> put_LeaseStopTime(This,time) ) 

#define IMcastLeaseInfo_get_AddressCount(This,pCount)	\
    ( (This)->lpVtbl -> get_AddressCount(This,pCount) ) 

#define IMcastLeaseInfo_get_ServerAddress(This,ppAddress)	\
    ( (This)->lpVtbl -> get_ServerAddress(This,ppAddress) ) 

#define IMcastLeaseInfo_get_TTL(This,pTTL)	\
    ( (This)->lpVtbl -> get_TTL(This,pTTL) ) 

#define IMcastLeaseInfo_get_Addresses(This,pVariant)	\
    ( (This)->lpVtbl -> get_Addresses(This,pVariant) ) 

#define IMcastLeaseInfo_EnumerateAddresses(This,ppEnumAddresses)	\
    ( (This)->lpVtbl -> EnumerateAddresses(This,ppEnumAddresses) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMcastLeaseInfo_INTERFACE_DEFINED__ */


#ifndef __IEnumMcastScope_INTERFACE_DEFINED__
#define __IEnumMcastScope_INTERFACE_DEFINED__

/* interface IEnumMcastScope */
/* [unique][helpstring][hidden][uuid][object] */ 


EXTERN_C const IID IID_IEnumMcastScope;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("DF0DAF09-A289-11D1-8697-006008B0E5D2")
    IEnumMcastScope : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Next( 
            /* [in] */ ULONG celt,
            /* [out] */ __RPC__deref_out_opt IMcastScope **ppScopes,
            /* [full][out][in] */ __RPC__inout_opt ULONG *pceltFetched) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Reset( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Skip( 
            /* [in] */ ULONG celt) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Clone( 
            /* [retval][out] */ __RPC__deref_out_opt IEnumMcastScope **ppEnum) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IEnumMcastScopeVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IEnumMcastScope * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IEnumMcastScope * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IEnumMcastScope * This);
        
        DECLSPEC_XFGVIRT(IEnumMcastScope, Next)
        HRESULT ( STDMETHODCALLTYPE *Next )( 
            __RPC__in IEnumMcastScope * This,
            /* [in] */ ULONG celt,
            /* [out] */ __RPC__deref_out_opt IMcastScope **ppScopes,
            /* [full][out][in] */ __RPC__inout_opt ULONG *pceltFetched);
        
        DECLSPEC_XFGVIRT(IEnumMcastScope, Reset)
        HRESULT ( STDMETHODCALLTYPE *Reset )( 
            __RPC__in IEnumMcastScope * This);
        
        DECLSPEC_XFGVIRT(IEnumMcastScope, Skip)
        HRESULT ( STDMETHODCALLTYPE *Skip )( 
            __RPC__in IEnumMcastScope * This,
            /* [in] */ ULONG celt);
        
        DECLSPEC_XFGVIRT(IEnumMcastScope, Clone)
        HRESULT ( STDMETHODCALLTYPE *Clone )( 
            __RPC__in IEnumMcastScope * This,
            /* [retval][out] */ __RPC__deref_out_opt IEnumMcastScope **ppEnum);
        
        END_INTERFACE
    } IEnumMcastScopeVtbl;

    interface IEnumMcastScope
    {
        CONST_VTBL struct IEnumMcastScopeVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IEnumMcastScope_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IEnumMcastScope_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IEnumMcastScope_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IEnumMcastScope_Next(This,celt,ppScopes,pceltFetched)	\
    ( (This)->lpVtbl -> Next(This,celt,ppScopes,pceltFetched) ) 

#define IEnumMcastScope_Reset(This)	\
    ( (This)->lpVtbl -> Reset(This) ) 

#define IEnumMcastScope_Skip(This,celt)	\
    ( (This)->lpVtbl -> Skip(This,celt) ) 

#define IEnumMcastScope_Clone(This,ppEnum)	\
    ( (This)->lpVtbl -> Clone(This,ppEnum) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IEnumMcastScope_INTERFACE_DEFINED__ */


#ifndef __IMcastAddressAllocation_INTERFACE_DEFINED__
#define __IMcastAddressAllocation_INTERFACE_DEFINED__

/* interface IMcastAddressAllocation */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IMcastAddressAllocation;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("DF0DAEF1-A289-11D1-8697-006008B0E5D2")
    IMcastAddressAllocation : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Scopes( 
            /* [retval][out] */ __RPC__out VARIANT *pVariant) = 0;
        
        virtual /* [helpstring][hidden][id] */ HRESULT STDMETHODCALLTYPE EnumerateScopes( 
            /* [out] */ __RPC__deref_out_opt IEnumMcastScope **ppEnumMcastScope) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE RequestAddress( 
            /* [in] */ __RPC__in_opt IMcastScope *pScope,
            /* [in] */ DATE LeaseStartTime,
            /* [in] */ DATE LeaseStopTime,
            /* [in] */ long NumAddresses,
            /* [retval][out] */ __RPC__deref_out_opt IMcastLeaseInfo **ppLeaseResponse) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE RenewAddress( 
            /* [in] */ long lReserved,
            /* [in] */ __RPC__in_opt IMcastLeaseInfo *pRenewRequest,
            /* [retval][out] */ __RPC__deref_out_opt IMcastLeaseInfo **ppRenewResponse) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE ReleaseAddress( 
            /* [in] */ __RPC__in_opt IMcastLeaseInfo *pReleaseRequest) = 0;
        
        virtual /* [helpstring][hidden][id] */ HRESULT STDMETHODCALLTYPE CreateLeaseInfo( 
            /* [in] */ DATE LeaseStartTime,
            /* [in] */ DATE LeaseStopTime,
            /* [in] */ DWORD dwNumAddresses,
            /* [in] */ __RPC__deref_in_opt LPWSTR *ppAddresses,
            /* [in] */ __RPC__in LPWSTR pRequestID,
            /* [in] */ __RPC__in LPWSTR pServerAddress,
            /* [retval][out] */ __RPC__deref_out_opt IMcastLeaseInfo **ppReleaseRequest) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE CreateLeaseInfoFromVariant( 
            /* [in] */ DATE LeaseStartTime,
            /* [in] */ DATE LeaseStopTime,
            /* [in] */ VARIANT vAddresses,
            /* [in] */ __RPC__in BSTR pRequestID,
            /* [in] */ __RPC__in BSTR pServerAddress,
            /* [retval][out] */ __RPC__deref_out_opt IMcastLeaseInfo **ppReleaseRequest) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMcastAddressAllocationVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMcastAddressAllocation * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMcastAddressAllocation * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMcastAddressAllocation * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IMcastAddressAllocation * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IMcastAddressAllocation * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IMcastAddressAllocation * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IMcastAddressAllocation * This,
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
        
        DECLSPEC_XFGVIRT(IMcastAddressAllocation, get_Scopes)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Scopes )( 
            __RPC__in IMcastAddressAllocation * This,
            /* [retval][out] */ __RPC__out VARIANT *pVariant);
        
        DECLSPEC_XFGVIRT(IMcastAddressAllocation, EnumerateScopes)
        /* [helpstring][hidden][id] */ HRESULT ( STDMETHODCALLTYPE *EnumerateScopes )( 
            __RPC__in IMcastAddressAllocation * This,
            /* [out] */ __RPC__deref_out_opt IEnumMcastScope **ppEnumMcastScope);
        
        DECLSPEC_XFGVIRT(IMcastAddressAllocation, RequestAddress)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *RequestAddress )( 
            __RPC__in IMcastAddressAllocation * This,
            /* [in] */ __RPC__in_opt IMcastScope *pScope,
            /* [in] */ DATE LeaseStartTime,
            /* [in] */ DATE LeaseStopTime,
            /* [in] */ long NumAddresses,
            /* [retval][out] */ __RPC__deref_out_opt IMcastLeaseInfo **ppLeaseResponse);
        
        DECLSPEC_XFGVIRT(IMcastAddressAllocation, RenewAddress)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *RenewAddress )( 
            __RPC__in IMcastAddressAllocation * This,
            /* [in] */ long lReserved,
            /* [in] */ __RPC__in_opt IMcastLeaseInfo *pRenewRequest,
            /* [retval][out] */ __RPC__deref_out_opt IMcastLeaseInfo **ppRenewResponse);
        
        DECLSPEC_XFGVIRT(IMcastAddressAllocation, ReleaseAddress)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *ReleaseAddress )( 
            __RPC__in IMcastAddressAllocation * This,
            /* [in] */ __RPC__in_opt IMcastLeaseInfo *pReleaseRequest);
        
        DECLSPEC_XFGVIRT(IMcastAddressAllocation, CreateLeaseInfo)
        /* [helpstring][hidden][id] */ HRESULT ( STDMETHODCALLTYPE *CreateLeaseInfo )( 
            __RPC__in IMcastAddressAllocation * This,
            /* [in] */ DATE LeaseStartTime,
            /* [in] */ DATE LeaseStopTime,
            /* [in] */ DWORD dwNumAddresses,
            /* [in] */ __RPC__deref_in_opt LPWSTR *ppAddresses,
            /* [in] */ __RPC__in LPWSTR pRequestID,
            /* [in] */ __RPC__in LPWSTR pServerAddress,
            /* [retval][out] */ __RPC__deref_out_opt IMcastLeaseInfo **ppReleaseRequest);
        
        DECLSPEC_XFGVIRT(IMcastAddressAllocation, CreateLeaseInfoFromVariant)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *CreateLeaseInfoFromVariant )( 
            __RPC__in IMcastAddressAllocation * This,
            /* [in] */ DATE LeaseStartTime,
            /* [in] */ DATE LeaseStopTime,
            /* [in] */ VARIANT vAddresses,
            /* [in] */ __RPC__in BSTR pRequestID,
            /* [in] */ __RPC__in BSTR pServerAddress,
            /* [retval][out] */ __RPC__deref_out_opt IMcastLeaseInfo **ppReleaseRequest);
        
        END_INTERFACE
    } IMcastAddressAllocationVtbl;

    interface IMcastAddressAllocation
    {
        CONST_VTBL struct IMcastAddressAllocationVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMcastAddressAllocation_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMcastAddressAllocation_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMcastAddressAllocation_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMcastAddressAllocation_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IMcastAddressAllocation_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IMcastAddressAllocation_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IMcastAddressAllocation_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IMcastAddressAllocation_get_Scopes(This,pVariant)	\
    ( (This)->lpVtbl -> get_Scopes(This,pVariant) ) 

#define IMcastAddressAllocation_EnumerateScopes(This,ppEnumMcastScope)	\
    ( (This)->lpVtbl -> EnumerateScopes(This,ppEnumMcastScope) ) 

#define IMcastAddressAllocation_RequestAddress(This,pScope,LeaseStartTime,LeaseStopTime,NumAddresses,ppLeaseResponse)	\
    ( (This)->lpVtbl -> RequestAddress(This,pScope,LeaseStartTime,LeaseStopTime,NumAddresses,ppLeaseResponse) ) 

#define IMcastAddressAllocation_RenewAddress(This,lReserved,pRenewRequest,ppRenewResponse)	\
    ( (This)->lpVtbl -> RenewAddress(This,lReserved,pRenewRequest,ppRenewResponse) ) 

#define IMcastAddressAllocation_ReleaseAddress(This,pReleaseRequest)	\
    ( (This)->lpVtbl -> ReleaseAddress(This,pReleaseRequest) ) 

#define IMcastAddressAllocation_CreateLeaseInfo(This,LeaseStartTime,LeaseStopTime,dwNumAddresses,ppAddresses,pRequestID,pServerAddress,ppReleaseRequest)	\
    ( (This)->lpVtbl -> CreateLeaseInfo(This,LeaseStartTime,LeaseStopTime,dwNumAddresses,ppAddresses,pRequestID,pServerAddress,ppReleaseRequest) ) 

#define IMcastAddressAllocation_CreateLeaseInfoFromVariant(This,LeaseStartTime,LeaseStopTime,vAddresses,pRequestID,pServerAddress,ppReleaseRequest)	\
    ( (This)->lpVtbl -> CreateLeaseInfoFromVariant(This,LeaseStartTime,LeaseStopTime,vAddresses,pRequestID,pServerAddress,ppReleaseRequest) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMcastAddressAllocation_INTERFACE_DEFINED__ */



#ifndef __McastLib_LIBRARY_DEFINED__
#define __McastLib_LIBRARY_DEFINED__

/* library McastLib */
/* [helpstring][version][uuid] */ 







EXTERN_C const IID LIBID_McastLib;

EXTERN_C const CLSID CLSID_McastAddressAllocation;

#ifdef __cplusplus

class DECLSPEC_UUID("DF0DAEF2-A289-11D1-8697-006008B0E5D2")
McastAddressAllocation;
#endif
#endif /* __McastLib_LIBRARY_DEFINED__ */

/* interface __MIDL_itf_mdhcp_0000_0005 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_mdhcp_0000_0005_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mdhcp_0000_0005_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

unsigned long             __RPC_USER  BSTR_UserSize(     __RPC__in unsigned long *, unsigned long            , __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserMarshal(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out BSTR * ); 
void                      __RPC_USER  BSTR_UserFree(     __RPC__in unsigned long *, __RPC__in BSTR * ); 

unsigned long             __RPC_USER  VARIANT_UserSize(     __RPC__in unsigned long *, unsigned long            , __RPC__in VARIANT * ); 
unsigned char * __RPC_USER  VARIANT_UserMarshal(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in VARIANT * ); 
unsigned char * __RPC_USER  VARIANT_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out VARIANT * ); 
void                      __RPC_USER  VARIANT_UserFree(     __RPC__in unsigned long *, __RPC__in VARIANT * ); 

unsigned long             __RPC_USER  BSTR_UserSize64(     __RPC__in unsigned long *, unsigned long            , __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserMarshal64(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out BSTR * ); 
void                      __RPC_USER  BSTR_UserFree64(     __RPC__in unsigned long *, __RPC__in BSTR * ); 

unsigned long             __RPC_USER  VARIANT_UserSize64(     __RPC__in unsigned long *, unsigned long            , __RPC__in VARIANT * ); 
unsigned char * __RPC_USER  VARIANT_UserMarshal64(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in VARIANT * ); 
unsigned char * __RPC_USER  VARIANT_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out VARIANT * ); 
void                      __RPC_USER  VARIANT_UserFree64(     __RPC__in unsigned long *, __RPC__in VARIANT * ); 

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


