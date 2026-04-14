

/* this ALWAYS GENERATED file contains the definitions for the interfaces */


 /* File created by MIDL compiler version 8.01.0628 */
/* @@MIDL_FILE_HEADING(  ) */



/* verify that the <rpcndr.h> version is high enough to compile this file*/
#ifndef __REQUIRED_RPCNDR_H_VERSION__
#define __REQUIRED_RPCNDR_H_VERSION__ 500
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

#ifndef __eventsys_h__
#define __eventsys_h__

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

#ifndef __IEventSystem_FWD_DEFINED__
#define __IEventSystem_FWD_DEFINED__
typedef interface IEventSystem IEventSystem;

#endif 	/* __IEventSystem_FWD_DEFINED__ */


#ifndef __IEventPublisher_FWD_DEFINED__
#define __IEventPublisher_FWD_DEFINED__
typedef interface IEventPublisher IEventPublisher;

#endif 	/* __IEventPublisher_FWD_DEFINED__ */


#ifndef __IEventClass_FWD_DEFINED__
#define __IEventClass_FWD_DEFINED__
typedef interface IEventClass IEventClass;

#endif 	/* __IEventClass_FWD_DEFINED__ */


#ifndef __IEventClass2_FWD_DEFINED__
#define __IEventClass2_FWD_DEFINED__
typedef interface IEventClass2 IEventClass2;

#endif 	/* __IEventClass2_FWD_DEFINED__ */


#ifndef __IEventSubscription_FWD_DEFINED__
#define __IEventSubscription_FWD_DEFINED__
typedef interface IEventSubscription IEventSubscription;

#endif 	/* __IEventSubscription_FWD_DEFINED__ */


#ifndef __IFiringControl_FWD_DEFINED__
#define __IFiringControl_FWD_DEFINED__
typedef interface IFiringControl IFiringControl;

#endif 	/* __IFiringControl_FWD_DEFINED__ */


#ifndef __IPublisherFilter_FWD_DEFINED__
#define __IPublisherFilter_FWD_DEFINED__
typedef interface IPublisherFilter IPublisherFilter;

#endif 	/* __IPublisherFilter_FWD_DEFINED__ */


#ifndef __IMultiInterfacePublisherFilter_FWD_DEFINED__
#define __IMultiInterfacePublisherFilter_FWD_DEFINED__
typedef interface IMultiInterfacePublisherFilter IMultiInterfacePublisherFilter;

#endif 	/* __IMultiInterfacePublisherFilter_FWD_DEFINED__ */


#ifndef __IEventObjectChange_FWD_DEFINED__
#define __IEventObjectChange_FWD_DEFINED__
typedef interface IEventObjectChange IEventObjectChange;

#endif 	/* __IEventObjectChange_FWD_DEFINED__ */


#ifndef __IEventObjectChange2_FWD_DEFINED__
#define __IEventObjectChange2_FWD_DEFINED__
typedef interface IEventObjectChange2 IEventObjectChange2;

#endif 	/* __IEventObjectChange2_FWD_DEFINED__ */


#ifndef __IEnumEventObject_FWD_DEFINED__
#define __IEnumEventObject_FWD_DEFINED__
typedef interface IEnumEventObject IEnumEventObject;

#endif 	/* __IEnumEventObject_FWD_DEFINED__ */


#ifndef __IEventObjectCollection_FWD_DEFINED__
#define __IEventObjectCollection_FWD_DEFINED__
typedef interface IEventObjectCollection IEventObjectCollection;

#endif 	/* __IEventObjectCollection_FWD_DEFINED__ */


#ifndef __IEventProperty_FWD_DEFINED__
#define __IEventProperty_FWD_DEFINED__
typedef interface IEventProperty IEventProperty;

#endif 	/* __IEventProperty_FWD_DEFINED__ */


#ifndef __IEventControl_FWD_DEFINED__
#define __IEventControl_FWD_DEFINED__
typedef interface IEventControl IEventControl;

#endif 	/* __IEventControl_FWD_DEFINED__ */


#ifndef __IMultiInterfaceEventControl_FWD_DEFINED__
#define __IMultiInterfaceEventControl_FWD_DEFINED__
typedef interface IMultiInterfaceEventControl IMultiInterfaceEventControl;

#endif 	/* __IMultiInterfaceEventControl_FWD_DEFINED__ */


#ifndef __IDontSupportEventSubscription_FWD_DEFINED__
#define __IDontSupportEventSubscription_FWD_DEFINED__
typedef interface IDontSupportEventSubscription IDontSupportEventSubscription;

#endif 	/* __IDontSupportEventSubscription_FWD_DEFINED__ */


#ifndef __CEventSystem_FWD_DEFINED__
#define __CEventSystem_FWD_DEFINED__

#ifdef __cplusplus
typedef class CEventSystem CEventSystem;
#else
typedef struct CEventSystem CEventSystem;
#endif /* __cplusplus */

#endif 	/* __CEventSystem_FWD_DEFINED__ */


#ifndef __CEventPublisher_FWD_DEFINED__
#define __CEventPublisher_FWD_DEFINED__

#ifdef __cplusplus
typedef class CEventPublisher CEventPublisher;
#else
typedef struct CEventPublisher CEventPublisher;
#endif /* __cplusplus */

#endif 	/* __CEventPublisher_FWD_DEFINED__ */


#ifndef __CEventClass_FWD_DEFINED__
#define __CEventClass_FWD_DEFINED__

#ifdef __cplusplus
typedef class CEventClass CEventClass;
#else
typedef struct CEventClass CEventClass;
#endif /* __cplusplus */

#endif 	/* __CEventClass_FWD_DEFINED__ */


#ifndef __CEventSubscription_FWD_DEFINED__
#define __CEventSubscription_FWD_DEFINED__

#ifdef __cplusplus
typedef class CEventSubscription CEventSubscription;
#else
typedef struct CEventSubscription CEventSubscription;
#endif /* __cplusplus */

#endif 	/* __CEventSubscription_FWD_DEFINED__ */


#ifndef __EventObjectChange_FWD_DEFINED__
#define __EventObjectChange_FWD_DEFINED__

#ifdef __cplusplus
typedef class EventObjectChange EventObjectChange;
#else
typedef struct EventObjectChange EventObjectChange;
#endif /* __cplusplus */

#endif 	/* __EventObjectChange_FWD_DEFINED__ */


#ifndef __EventObjectChange2_FWD_DEFINED__
#define __EventObjectChange2_FWD_DEFINED__

#ifdef __cplusplus
typedef class EventObjectChange2 EventObjectChange2;
#else
typedef struct EventObjectChange2 EventObjectChange2;
#endif /* __cplusplus */

#endif 	/* __EventObjectChange2_FWD_DEFINED__ */


/* header files for imported files */
#include "oaidl.h"
#include "ocidl.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_eventsys_0000_0000 */
/* [local] */ 

#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
#define PROGID_EventSystem OLESTR("EventSystem.EventSystem")
#define PROGID_EventPublisher OLESTR("EventSystem.EventPublisher")
#define PROGID_EventClass OLESTR("EventSystem.EventClass")
#define PROGID_EventSubscription OLESTR("EventSystem.EventSubscription")
#define PROGID_EventPublisherCollection OLESTR("EventSystem.EventPublisherCollection")
#define PROGID_EventClassCollection OLESTR("EventSystem.EventClassCollection")
#define PROGID_EventSubscriptionCollection OLESTR("EventSystem.EventSubscriptionCollection")
#define PROGID_EventSubsystem OLESTR("EventSystem.EventSubsystem")
#define EVENTSYSTEM_PUBLISHER_ID OLESTR("{d0564c30-9df4-11d1-a281-00c04fca0aa7}")
#define EVENTSYSTEM_SUBSYSTEM_CLSID OLESTR("{503c1fd8-b605-11d2-a92d-006008c60e24}")






extern RPC_IF_HANDLE __MIDL_itf_eventsys_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_eventsys_0000_0000_v0_0_s_ifspec;

#ifndef __IEventSystem_INTERFACE_DEFINED__
#define __IEventSystem_INTERFACE_DEFINED__

/* interface IEventSystem */
/* [unique][helpstring][proxy][dual][uuid][object] */ 

// *****************************************************************
// This is a Deprecated interface - Use COMAdmin interfaces instead.
// *****************************************************************

EXTERN_C const IID IID_IEventSystem;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("4E14FB9F-2E22-11D1-9964-00C04FBBB345")
    IEventSystem : public IDispatch
    {
    public:
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Query( 
            /* [in] */ __RPC__in BSTR progID,
            /* [in] */ __RPC__in BSTR queryCriteria,
            /* [out] */ __RPC__out int *errorIndex,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **ppInterface) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Store( 
            /* [in] */ __RPC__in BSTR ProgID,
            /* [in] */ __RPC__in_opt IUnknown *pInterface) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Remove( 
            /* [in] */ __RPC__in BSTR progID,
            /* [in] */ __RPC__in BSTR queryCriteria,
            /* [out] */ __RPC__out int *errorIndex) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_EventObjectChangeEventClassID( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrEventClassID) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE QueryS( 
            /* [in] */ __RPC__in BSTR progID,
            /* [in] */ __RPC__in BSTR queryCriteria,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **ppInterface) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE RemoveS( 
            /* [in] */ __RPC__in BSTR progID,
            /* [in] */ __RPC__in BSTR queryCriteria) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IEventSystemVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IEventSystem * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IEventSystem * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IEventSystem * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IEventSystem * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IEventSystem * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IEventSystem * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IEventSystem * This,
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
        
        DECLSPEC_XFGVIRT(IEventSystem, Query)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Query )( 
            __RPC__in IEventSystem * This,
            /* [in] */ __RPC__in BSTR progID,
            /* [in] */ __RPC__in BSTR queryCriteria,
            /* [out] */ __RPC__out int *errorIndex,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **ppInterface);
        
        DECLSPEC_XFGVIRT(IEventSystem, Store)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Store )( 
            __RPC__in IEventSystem * This,
            /* [in] */ __RPC__in BSTR ProgID,
            /* [in] */ __RPC__in_opt IUnknown *pInterface);
        
        DECLSPEC_XFGVIRT(IEventSystem, Remove)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Remove )( 
            __RPC__in IEventSystem * This,
            /* [in] */ __RPC__in BSTR progID,
            /* [in] */ __RPC__in BSTR queryCriteria,
            /* [out] */ __RPC__out int *errorIndex);
        
        DECLSPEC_XFGVIRT(IEventSystem, get_EventObjectChangeEventClassID)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_EventObjectChangeEventClassID )( 
            __RPC__in IEventSystem * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrEventClassID);
        
        DECLSPEC_XFGVIRT(IEventSystem, QueryS)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *QueryS )( 
            __RPC__in IEventSystem * This,
            /* [in] */ __RPC__in BSTR progID,
            /* [in] */ __RPC__in BSTR queryCriteria,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **ppInterface);
        
        DECLSPEC_XFGVIRT(IEventSystem, RemoveS)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *RemoveS )( 
            __RPC__in IEventSystem * This,
            /* [in] */ __RPC__in BSTR progID,
            /* [in] */ __RPC__in BSTR queryCriteria);
        
        END_INTERFACE
    } IEventSystemVtbl;

    interface IEventSystem
    {
        CONST_VTBL struct IEventSystemVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IEventSystem_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IEventSystem_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IEventSystem_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IEventSystem_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IEventSystem_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IEventSystem_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IEventSystem_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IEventSystem_Query(This,progID,queryCriteria,errorIndex,ppInterface)	\
    ( (This)->lpVtbl -> Query(This,progID,queryCriteria,errorIndex,ppInterface) ) 

#define IEventSystem_Store(This,ProgID,pInterface)	\
    ( (This)->lpVtbl -> Store(This,ProgID,pInterface) ) 

#define IEventSystem_Remove(This,progID,queryCriteria,errorIndex)	\
    ( (This)->lpVtbl -> Remove(This,progID,queryCriteria,errorIndex) ) 

#define IEventSystem_get_EventObjectChangeEventClassID(This,pbstrEventClassID)	\
    ( (This)->lpVtbl -> get_EventObjectChangeEventClassID(This,pbstrEventClassID) ) 

#define IEventSystem_QueryS(This,progID,queryCriteria,ppInterface)	\
    ( (This)->lpVtbl -> QueryS(This,progID,queryCriteria,ppInterface) ) 

#define IEventSystem_RemoveS(This,progID,queryCriteria)	\
    ( (This)->lpVtbl -> RemoveS(This,progID,queryCriteria) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IEventSystem_INTERFACE_DEFINED__ */


#ifndef __IEventPublisher_INTERFACE_DEFINED__
#define __IEventPublisher_INTERFACE_DEFINED__

/* interface IEventPublisher */
/* [unique][helpstring][proxy][dual][uuid][object] */ 

// ********************************************
// This is a Deprecated interface - Do Not Use.
// ********************************************

EXTERN_C const IID IID_IEventPublisher;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("E341516B-2E32-11D1-9964-00C04FBBB345")
    IEventPublisher : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_PublisherID( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrPublisherID) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_PublisherID( 
            /* [in] */ __RPC__in BSTR bstrPublisherID) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_PublisherName( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrPublisherName) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_PublisherName( 
            /* [in] */ __RPC__in BSTR bstrPublisherName) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_PublisherType( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrPublisherType) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_PublisherType( 
            /* [in] */ __RPC__in BSTR bstrPublisherType) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_OwnerSID( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrOwnerSID) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_OwnerSID( 
            /* [in] */ __RPC__in BSTR bstrOwnerSID) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Description( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrDescription) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_Description( 
            /* [in] */ __RPC__in BSTR bstrDescription) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetDefaultProperty( 
            /* [in] */ __RPC__in BSTR bstrPropertyName,
            /* [retval][out] */ __RPC__out VARIANT *propertyValue) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE PutDefaultProperty( 
            /* [in] */ __RPC__in BSTR bstrPropertyName,
            /* [in] */ __RPC__in VARIANT *propertyValue) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE RemoveDefaultProperty( 
            /* [in] */ __RPC__in BSTR bstrPropertyName) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetDefaultPropertyCollection( 
            /* [retval][out] */ __RPC__deref_out_opt IEventObjectCollection **collection) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IEventPublisherVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IEventPublisher * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IEventPublisher * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IEventPublisher * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IEventPublisher * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IEventPublisher * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IEventPublisher * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IEventPublisher * This,
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
        
        DECLSPEC_XFGVIRT(IEventPublisher, get_PublisherID)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_PublisherID )( 
            __RPC__in IEventPublisher * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrPublisherID);
        
        DECLSPEC_XFGVIRT(IEventPublisher, put_PublisherID)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_PublisherID )( 
            __RPC__in IEventPublisher * This,
            /* [in] */ __RPC__in BSTR bstrPublisherID);
        
        DECLSPEC_XFGVIRT(IEventPublisher, get_PublisherName)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_PublisherName )( 
            __RPC__in IEventPublisher * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrPublisherName);
        
        DECLSPEC_XFGVIRT(IEventPublisher, put_PublisherName)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_PublisherName )( 
            __RPC__in IEventPublisher * This,
            /* [in] */ __RPC__in BSTR bstrPublisherName);
        
        DECLSPEC_XFGVIRT(IEventPublisher, get_PublisherType)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_PublisherType )( 
            __RPC__in IEventPublisher * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrPublisherType);
        
        DECLSPEC_XFGVIRT(IEventPublisher, put_PublisherType)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_PublisherType )( 
            __RPC__in IEventPublisher * This,
            /* [in] */ __RPC__in BSTR bstrPublisherType);
        
        DECLSPEC_XFGVIRT(IEventPublisher, get_OwnerSID)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_OwnerSID )( 
            __RPC__in IEventPublisher * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrOwnerSID);
        
        DECLSPEC_XFGVIRT(IEventPublisher, put_OwnerSID)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_OwnerSID )( 
            __RPC__in IEventPublisher * This,
            /* [in] */ __RPC__in BSTR bstrOwnerSID);
        
        DECLSPEC_XFGVIRT(IEventPublisher, get_Description)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Description )( 
            __RPC__in IEventPublisher * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrDescription);
        
        DECLSPEC_XFGVIRT(IEventPublisher, put_Description)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Description )( 
            __RPC__in IEventPublisher * This,
            /* [in] */ __RPC__in BSTR bstrDescription);
        
        DECLSPEC_XFGVIRT(IEventPublisher, GetDefaultProperty)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetDefaultProperty )( 
            __RPC__in IEventPublisher * This,
            /* [in] */ __RPC__in BSTR bstrPropertyName,
            /* [retval][out] */ __RPC__out VARIANT *propertyValue);
        
        DECLSPEC_XFGVIRT(IEventPublisher, PutDefaultProperty)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *PutDefaultProperty )( 
            __RPC__in IEventPublisher * This,
            /* [in] */ __RPC__in BSTR bstrPropertyName,
            /* [in] */ __RPC__in VARIANT *propertyValue);
        
        DECLSPEC_XFGVIRT(IEventPublisher, RemoveDefaultProperty)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *RemoveDefaultProperty )( 
            __RPC__in IEventPublisher * This,
            /* [in] */ __RPC__in BSTR bstrPropertyName);
        
        DECLSPEC_XFGVIRT(IEventPublisher, GetDefaultPropertyCollection)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetDefaultPropertyCollection )( 
            __RPC__in IEventPublisher * This,
            /* [retval][out] */ __RPC__deref_out_opt IEventObjectCollection **collection);
        
        END_INTERFACE
    } IEventPublisherVtbl;

    interface IEventPublisher
    {
        CONST_VTBL struct IEventPublisherVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IEventPublisher_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IEventPublisher_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IEventPublisher_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IEventPublisher_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IEventPublisher_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IEventPublisher_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IEventPublisher_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IEventPublisher_get_PublisherID(This,pbstrPublisherID)	\
    ( (This)->lpVtbl -> get_PublisherID(This,pbstrPublisherID) ) 

#define IEventPublisher_put_PublisherID(This,bstrPublisherID)	\
    ( (This)->lpVtbl -> put_PublisherID(This,bstrPublisherID) ) 

#define IEventPublisher_get_PublisherName(This,pbstrPublisherName)	\
    ( (This)->lpVtbl -> get_PublisherName(This,pbstrPublisherName) ) 

#define IEventPublisher_put_PublisherName(This,bstrPublisherName)	\
    ( (This)->lpVtbl -> put_PublisherName(This,bstrPublisherName) ) 

#define IEventPublisher_get_PublisherType(This,pbstrPublisherType)	\
    ( (This)->lpVtbl -> get_PublisherType(This,pbstrPublisherType) ) 

#define IEventPublisher_put_PublisherType(This,bstrPublisherType)	\
    ( (This)->lpVtbl -> put_PublisherType(This,bstrPublisherType) ) 

#define IEventPublisher_get_OwnerSID(This,pbstrOwnerSID)	\
    ( (This)->lpVtbl -> get_OwnerSID(This,pbstrOwnerSID) ) 

#define IEventPublisher_put_OwnerSID(This,bstrOwnerSID)	\
    ( (This)->lpVtbl -> put_OwnerSID(This,bstrOwnerSID) ) 

#define IEventPublisher_get_Description(This,pbstrDescription)	\
    ( (This)->lpVtbl -> get_Description(This,pbstrDescription) ) 

#define IEventPublisher_put_Description(This,bstrDescription)	\
    ( (This)->lpVtbl -> put_Description(This,bstrDescription) ) 

#define IEventPublisher_GetDefaultProperty(This,bstrPropertyName,propertyValue)	\
    ( (This)->lpVtbl -> GetDefaultProperty(This,bstrPropertyName,propertyValue) ) 

#define IEventPublisher_PutDefaultProperty(This,bstrPropertyName,propertyValue)	\
    ( (This)->lpVtbl -> PutDefaultProperty(This,bstrPropertyName,propertyValue) ) 

#define IEventPublisher_RemoveDefaultProperty(This,bstrPropertyName)	\
    ( (This)->lpVtbl -> RemoveDefaultProperty(This,bstrPropertyName) ) 

#define IEventPublisher_GetDefaultPropertyCollection(This,collection)	\
    ( (This)->lpVtbl -> GetDefaultPropertyCollection(This,collection) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IEventPublisher_INTERFACE_DEFINED__ */


#ifndef __IEventClass_INTERFACE_DEFINED__
#define __IEventClass_INTERFACE_DEFINED__

/* interface IEventClass */
/* [unique][helpstring][proxy][dual][uuid][object] */ 


EXTERN_C const IID IID_IEventClass;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("fb2b72a0-7a68-11d1-88f9-0080c7d771bf")
    IEventClass : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_EventClassID( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrEventClassID) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_EventClassID( 
            /* [in] */ __RPC__in BSTR bstrEventClassID) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_EventClassName( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrEventClassName) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_EventClassName( 
            /* [in] */ __RPC__in BSTR bstrEventClassName) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_OwnerSID( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrOwnerSID) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_OwnerSID( 
            /* [in] */ __RPC__in BSTR bstrOwnerSID) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_FiringInterfaceID( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrFiringInterfaceID) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_FiringInterfaceID( 
            /* [in] */ __RPC__in BSTR bstrFiringInterfaceID) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Description( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrDescription) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_Description( 
            /* [in] */ __RPC__in BSTR bstrDescription) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_CustomConfigCLSID( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrCustomConfigCLSID) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_CustomConfigCLSID( 
            /* [in] */ __RPC__in BSTR bstrCustomConfigCLSID) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_TypeLib( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrTypeLib) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_TypeLib( 
            /* [in] */ __RPC__in BSTR bstrTypeLib) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IEventClassVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IEventClass * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IEventClass * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IEventClass * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IEventClass * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IEventClass * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IEventClass * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IEventClass * This,
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
        
        DECLSPEC_XFGVIRT(IEventClass, get_EventClassID)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_EventClassID )( 
            __RPC__in IEventClass * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrEventClassID);
        
        DECLSPEC_XFGVIRT(IEventClass, put_EventClassID)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_EventClassID )( 
            __RPC__in IEventClass * This,
            /* [in] */ __RPC__in BSTR bstrEventClassID);
        
        DECLSPEC_XFGVIRT(IEventClass, get_EventClassName)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_EventClassName )( 
            __RPC__in IEventClass * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrEventClassName);
        
        DECLSPEC_XFGVIRT(IEventClass, put_EventClassName)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_EventClassName )( 
            __RPC__in IEventClass * This,
            /* [in] */ __RPC__in BSTR bstrEventClassName);
        
        DECLSPEC_XFGVIRT(IEventClass, get_OwnerSID)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_OwnerSID )( 
            __RPC__in IEventClass * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrOwnerSID);
        
        DECLSPEC_XFGVIRT(IEventClass, put_OwnerSID)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_OwnerSID )( 
            __RPC__in IEventClass * This,
            /* [in] */ __RPC__in BSTR bstrOwnerSID);
        
        DECLSPEC_XFGVIRT(IEventClass, get_FiringInterfaceID)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_FiringInterfaceID )( 
            __RPC__in IEventClass * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrFiringInterfaceID);
        
        DECLSPEC_XFGVIRT(IEventClass, put_FiringInterfaceID)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_FiringInterfaceID )( 
            __RPC__in IEventClass * This,
            /* [in] */ __RPC__in BSTR bstrFiringInterfaceID);
        
        DECLSPEC_XFGVIRT(IEventClass, get_Description)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Description )( 
            __RPC__in IEventClass * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrDescription);
        
        DECLSPEC_XFGVIRT(IEventClass, put_Description)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Description )( 
            __RPC__in IEventClass * This,
            /* [in] */ __RPC__in BSTR bstrDescription);
        
        DECLSPEC_XFGVIRT(IEventClass, get_CustomConfigCLSID)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_CustomConfigCLSID )( 
            __RPC__in IEventClass * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrCustomConfigCLSID);
        
        DECLSPEC_XFGVIRT(IEventClass, put_CustomConfigCLSID)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_CustomConfigCLSID )( 
            __RPC__in IEventClass * This,
            /* [in] */ __RPC__in BSTR bstrCustomConfigCLSID);
        
        DECLSPEC_XFGVIRT(IEventClass, get_TypeLib)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_TypeLib )( 
            __RPC__in IEventClass * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrTypeLib);
        
        DECLSPEC_XFGVIRT(IEventClass, put_TypeLib)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_TypeLib )( 
            __RPC__in IEventClass * This,
            /* [in] */ __RPC__in BSTR bstrTypeLib);
        
        END_INTERFACE
    } IEventClassVtbl;

    interface IEventClass
    {
        CONST_VTBL struct IEventClassVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IEventClass_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IEventClass_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IEventClass_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IEventClass_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IEventClass_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IEventClass_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IEventClass_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IEventClass_get_EventClassID(This,pbstrEventClassID)	\
    ( (This)->lpVtbl -> get_EventClassID(This,pbstrEventClassID) ) 

#define IEventClass_put_EventClassID(This,bstrEventClassID)	\
    ( (This)->lpVtbl -> put_EventClassID(This,bstrEventClassID) ) 

#define IEventClass_get_EventClassName(This,pbstrEventClassName)	\
    ( (This)->lpVtbl -> get_EventClassName(This,pbstrEventClassName) ) 

#define IEventClass_put_EventClassName(This,bstrEventClassName)	\
    ( (This)->lpVtbl -> put_EventClassName(This,bstrEventClassName) ) 

#define IEventClass_get_OwnerSID(This,pbstrOwnerSID)	\
    ( (This)->lpVtbl -> get_OwnerSID(This,pbstrOwnerSID) ) 

#define IEventClass_put_OwnerSID(This,bstrOwnerSID)	\
    ( (This)->lpVtbl -> put_OwnerSID(This,bstrOwnerSID) ) 

#define IEventClass_get_FiringInterfaceID(This,pbstrFiringInterfaceID)	\
    ( (This)->lpVtbl -> get_FiringInterfaceID(This,pbstrFiringInterfaceID) ) 

#define IEventClass_put_FiringInterfaceID(This,bstrFiringInterfaceID)	\
    ( (This)->lpVtbl -> put_FiringInterfaceID(This,bstrFiringInterfaceID) ) 

#define IEventClass_get_Description(This,pbstrDescription)	\
    ( (This)->lpVtbl -> get_Description(This,pbstrDescription) ) 

#define IEventClass_put_Description(This,bstrDescription)	\
    ( (This)->lpVtbl -> put_Description(This,bstrDescription) ) 

#define IEventClass_get_CustomConfigCLSID(This,pbstrCustomConfigCLSID)	\
    ( (This)->lpVtbl -> get_CustomConfigCLSID(This,pbstrCustomConfigCLSID) ) 

#define IEventClass_put_CustomConfigCLSID(This,bstrCustomConfigCLSID)	\
    ( (This)->lpVtbl -> put_CustomConfigCLSID(This,bstrCustomConfigCLSID) ) 

#define IEventClass_get_TypeLib(This,pbstrTypeLib)	\
    ( (This)->lpVtbl -> get_TypeLib(This,pbstrTypeLib) ) 

#define IEventClass_put_TypeLib(This,bstrTypeLib)	\
    ( (This)->lpVtbl -> put_TypeLib(This,bstrTypeLib) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IEventClass_INTERFACE_DEFINED__ */


#ifndef __IEventClass2_INTERFACE_DEFINED__
#define __IEventClass2_INTERFACE_DEFINED__

/* interface IEventClass2 */
/* [unique][helpstring][proxy][dual][uuid][object] */ 


EXTERN_C const IID IID_IEventClass2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("fb2b72a1-7a68-11d1-88f9-0080c7d771bf")
    IEventClass2 : public IEventClass
    {
    public:
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_PublisherID( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrPublisherID) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_PublisherID( 
            /* [in] */ __RPC__in BSTR bstrPublisherID) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_MultiInterfacePublisherFilterCLSID( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrPubFilCLSID) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_MultiInterfacePublisherFilterCLSID( 
            /* [in] */ __RPC__in BSTR bstrPubFilCLSID) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_AllowInprocActivation( 
            /* [retval][out] */ __RPC__out BOOL *pfAllowInprocActivation) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_AllowInprocActivation( 
            /* [in] */ BOOL fAllowInprocActivation) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_FireInParallel( 
            /* [retval][out] */ __RPC__out BOOL *pfFireInParallel) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_FireInParallel( 
            /* [in] */ BOOL fFireInParallel) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IEventClass2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IEventClass2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IEventClass2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IEventClass2 * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IEventClass2 * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IEventClass2 * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IEventClass2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IEventClass2 * This,
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
        
        DECLSPEC_XFGVIRT(IEventClass, get_EventClassID)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_EventClassID )( 
            __RPC__in IEventClass2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrEventClassID);
        
        DECLSPEC_XFGVIRT(IEventClass, put_EventClassID)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_EventClassID )( 
            __RPC__in IEventClass2 * This,
            /* [in] */ __RPC__in BSTR bstrEventClassID);
        
        DECLSPEC_XFGVIRT(IEventClass, get_EventClassName)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_EventClassName )( 
            __RPC__in IEventClass2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrEventClassName);
        
        DECLSPEC_XFGVIRT(IEventClass, put_EventClassName)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_EventClassName )( 
            __RPC__in IEventClass2 * This,
            /* [in] */ __RPC__in BSTR bstrEventClassName);
        
        DECLSPEC_XFGVIRT(IEventClass, get_OwnerSID)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_OwnerSID )( 
            __RPC__in IEventClass2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrOwnerSID);
        
        DECLSPEC_XFGVIRT(IEventClass, put_OwnerSID)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_OwnerSID )( 
            __RPC__in IEventClass2 * This,
            /* [in] */ __RPC__in BSTR bstrOwnerSID);
        
        DECLSPEC_XFGVIRT(IEventClass, get_FiringInterfaceID)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_FiringInterfaceID )( 
            __RPC__in IEventClass2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrFiringInterfaceID);
        
        DECLSPEC_XFGVIRT(IEventClass, put_FiringInterfaceID)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_FiringInterfaceID )( 
            __RPC__in IEventClass2 * This,
            /* [in] */ __RPC__in BSTR bstrFiringInterfaceID);
        
        DECLSPEC_XFGVIRT(IEventClass, get_Description)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Description )( 
            __RPC__in IEventClass2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrDescription);
        
        DECLSPEC_XFGVIRT(IEventClass, put_Description)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Description )( 
            __RPC__in IEventClass2 * This,
            /* [in] */ __RPC__in BSTR bstrDescription);
        
        DECLSPEC_XFGVIRT(IEventClass, get_CustomConfigCLSID)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_CustomConfigCLSID )( 
            __RPC__in IEventClass2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrCustomConfigCLSID);
        
        DECLSPEC_XFGVIRT(IEventClass, put_CustomConfigCLSID)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_CustomConfigCLSID )( 
            __RPC__in IEventClass2 * This,
            /* [in] */ __RPC__in BSTR bstrCustomConfigCLSID);
        
        DECLSPEC_XFGVIRT(IEventClass, get_TypeLib)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_TypeLib )( 
            __RPC__in IEventClass2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrTypeLib);
        
        DECLSPEC_XFGVIRT(IEventClass, put_TypeLib)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_TypeLib )( 
            __RPC__in IEventClass2 * This,
            /* [in] */ __RPC__in BSTR bstrTypeLib);
        
        DECLSPEC_XFGVIRT(IEventClass2, get_PublisherID)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_PublisherID )( 
            __RPC__in IEventClass2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrPublisherID);
        
        DECLSPEC_XFGVIRT(IEventClass2, put_PublisherID)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_PublisherID )( 
            __RPC__in IEventClass2 * This,
            /* [in] */ __RPC__in BSTR bstrPublisherID);
        
        DECLSPEC_XFGVIRT(IEventClass2, get_MultiInterfacePublisherFilterCLSID)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_MultiInterfacePublisherFilterCLSID )( 
            __RPC__in IEventClass2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrPubFilCLSID);
        
        DECLSPEC_XFGVIRT(IEventClass2, put_MultiInterfacePublisherFilterCLSID)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_MultiInterfacePublisherFilterCLSID )( 
            __RPC__in IEventClass2 * This,
            /* [in] */ __RPC__in BSTR bstrPubFilCLSID);
        
        DECLSPEC_XFGVIRT(IEventClass2, get_AllowInprocActivation)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_AllowInprocActivation )( 
            __RPC__in IEventClass2 * This,
            /* [retval][out] */ __RPC__out BOOL *pfAllowInprocActivation);
        
        DECLSPEC_XFGVIRT(IEventClass2, put_AllowInprocActivation)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_AllowInprocActivation )( 
            __RPC__in IEventClass2 * This,
            /* [in] */ BOOL fAllowInprocActivation);
        
        DECLSPEC_XFGVIRT(IEventClass2, get_FireInParallel)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_FireInParallel )( 
            __RPC__in IEventClass2 * This,
            /* [retval][out] */ __RPC__out BOOL *pfFireInParallel);
        
        DECLSPEC_XFGVIRT(IEventClass2, put_FireInParallel)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_FireInParallel )( 
            __RPC__in IEventClass2 * This,
            /* [in] */ BOOL fFireInParallel);
        
        END_INTERFACE
    } IEventClass2Vtbl;

    interface IEventClass2
    {
        CONST_VTBL struct IEventClass2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IEventClass2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IEventClass2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IEventClass2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IEventClass2_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IEventClass2_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IEventClass2_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IEventClass2_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IEventClass2_get_EventClassID(This,pbstrEventClassID)	\
    ( (This)->lpVtbl -> get_EventClassID(This,pbstrEventClassID) ) 

#define IEventClass2_put_EventClassID(This,bstrEventClassID)	\
    ( (This)->lpVtbl -> put_EventClassID(This,bstrEventClassID) ) 

#define IEventClass2_get_EventClassName(This,pbstrEventClassName)	\
    ( (This)->lpVtbl -> get_EventClassName(This,pbstrEventClassName) ) 

#define IEventClass2_put_EventClassName(This,bstrEventClassName)	\
    ( (This)->lpVtbl -> put_EventClassName(This,bstrEventClassName) ) 

#define IEventClass2_get_OwnerSID(This,pbstrOwnerSID)	\
    ( (This)->lpVtbl -> get_OwnerSID(This,pbstrOwnerSID) ) 

#define IEventClass2_put_OwnerSID(This,bstrOwnerSID)	\
    ( (This)->lpVtbl -> put_OwnerSID(This,bstrOwnerSID) ) 

#define IEventClass2_get_FiringInterfaceID(This,pbstrFiringInterfaceID)	\
    ( (This)->lpVtbl -> get_FiringInterfaceID(This,pbstrFiringInterfaceID) ) 

#define IEventClass2_put_FiringInterfaceID(This,bstrFiringInterfaceID)	\
    ( (This)->lpVtbl -> put_FiringInterfaceID(This,bstrFiringInterfaceID) ) 

#define IEventClass2_get_Description(This,pbstrDescription)	\
    ( (This)->lpVtbl -> get_Description(This,pbstrDescription) ) 

#define IEventClass2_put_Description(This,bstrDescription)	\
    ( (This)->lpVtbl -> put_Description(This,bstrDescription) ) 

#define IEventClass2_get_CustomConfigCLSID(This,pbstrCustomConfigCLSID)	\
    ( (This)->lpVtbl -> get_CustomConfigCLSID(This,pbstrCustomConfigCLSID) ) 

#define IEventClass2_put_CustomConfigCLSID(This,bstrCustomConfigCLSID)	\
    ( (This)->lpVtbl -> put_CustomConfigCLSID(This,bstrCustomConfigCLSID) ) 

#define IEventClass2_get_TypeLib(This,pbstrTypeLib)	\
    ( (This)->lpVtbl -> get_TypeLib(This,pbstrTypeLib) ) 

#define IEventClass2_put_TypeLib(This,bstrTypeLib)	\
    ( (This)->lpVtbl -> put_TypeLib(This,bstrTypeLib) ) 


#define IEventClass2_get_PublisherID(This,pbstrPublisherID)	\
    ( (This)->lpVtbl -> get_PublisherID(This,pbstrPublisherID) ) 

#define IEventClass2_put_PublisherID(This,bstrPublisherID)	\
    ( (This)->lpVtbl -> put_PublisherID(This,bstrPublisherID) ) 

#define IEventClass2_get_MultiInterfacePublisherFilterCLSID(This,pbstrPubFilCLSID)	\
    ( (This)->lpVtbl -> get_MultiInterfacePublisherFilterCLSID(This,pbstrPubFilCLSID) ) 

#define IEventClass2_put_MultiInterfacePublisherFilterCLSID(This,bstrPubFilCLSID)	\
    ( (This)->lpVtbl -> put_MultiInterfacePublisherFilterCLSID(This,bstrPubFilCLSID) ) 

#define IEventClass2_get_AllowInprocActivation(This,pfAllowInprocActivation)	\
    ( (This)->lpVtbl -> get_AllowInprocActivation(This,pfAllowInprocActivation) ) 

#define IEventClass2_put_AllowInprocActivation(This,fAllowInprocActivation)	\
    ( (This)->lpVtbl -> put_AllowInprocActivation(This,fAllowInprocActivation) ) 

#define IEventClass2_get_FireInParallel(This,pfFireInParallel)	\
    ( (This)->lpVtbl -> get_FireInParallel(This,pfFireInParallel) ) 

#define IEventClass2_put_FireInParallel(This,fFireInParallel)	\
    ( (This)->lpVtbl -> put_FireInParallel(This,fFireInParallel) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IEventClass2_INTERFACE_DEFINED__ */


#ifndef __IEventSubscription_INTERFACE_DEFINED__
#define __IEventSubscription_INTERFACE_DEFINED__

/* interface IEventSubscription */
/* [unique][helpstring][proxy][dual][uuid][object] */ 


EXTERN_C const IID IID_IEventSubscription;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("4A6B0E15-2E38-11D1-9965-00C04FBBB345")
    IEventSubscription : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_SubscriptionID( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrSubscriptionID) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_SubscriptionID( 
            /* [in] */ __RPC__in BSTR bstrSubscriptionID) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_SubscriptionName( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrSubscriptionName) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_SubscriptionName( 
            /* [in] */ __RPC__in BSTR bstrSubscriptionName) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_PublisherID( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrPublisherID) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_PublisherID( 
            /* [in] */ __RPC__in BSTR bstrPublisherID) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_EventClassID( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrEventClassID) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_EventClassID( 
            /* [in] */ __RPC__in BSTR bstrEventClassID) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_MethodName( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrMethodName) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_MethodName( 
            /* [in] */ __RPC__in BSTR bstrMethodName) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_SubscriberCLSID( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrSubscriberCLSID) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_SubscriberCLSID( 
            /* [in] */ __RPC__in BSTR bstrSubscriberCLSID) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_SubscriberInterface( 
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **ppSubscriberInterface) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_SubscriberInterface( 
            /* [in] */ __RPC__in_opt IUnknown *pSubscriberInterface) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_PerUser( 
            /* [retval][out] */ __RPC__out BOOL *pfPerUser) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_PerUser( 
            /* [in] */ BOOL fPerUser) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_OwnerSID( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrOwnerSID) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_OwnerSID( 
            /* [in] */ __RPC__in BSTR bstrOwnerSID) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Enabled( 
            /* [retval][out] */ __RPC__out BOOL *pfEnabled) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_Enabled( 
            /* [in] */ BOOL fEnabled) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Description( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrDescription) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_Description( 
            /* [in] */ __RPC__in BSTR bstrDescription) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_MachineName( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrMachineName) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_MachineName( 
            /* [in] */ __RPC__in BSTR bstrMachineName) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetPublisherProperty( 
            /* [in] */ __RPC__in BSTR bstrPropertyName,
            /* [retval][out] */ __RPC__out VARIANT *propertyValue) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE PutPublisherProperty( 
            /* [in] */ __RPC__in BSTR bstrPropertyName,
            /* [in] */ __RPC__in VARIANT *propertyValue) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE RemovePublisherProperty( 
            /* [in] */ __RPC__in BSTR bstrPropertyName) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetPublisherPropertyCollection( 
            /* [retval][out] */ __RPC__deref_out_opt IEventObjectCollection **collection) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetSubscriberProperty( 
            /* [in] */ __RPC__in BSTR bstrPropertyName,
            /* [retval][out] */ __RPC__out VARIANT *propertyValue) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE PutSubscriberProperty( 
            /* [in] */ __RPC__in BSTR bstrPropertyName,
            /* [in] */ __RPC__in VARIANT *propertyValue) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE RemoveSubscriberProperty( 
            /* [in] */ __RPC__in BSTR bstrPropertyName) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetSubscriberPropertyCollection( 
            /* [retval][out] */ __RPC__deref_out_opt IEventObjectCollection **collection) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_InterfaceID( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrInterfaceID) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_InterfaceID( 
            /* [in] */ __RPC__in BSTR bstrInterfaceID) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IEventSubscriptionVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IEventSubscription * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IEventSubscription * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IEventSubscription * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IEventSubscription * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IEventSubscription * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IEventSubscription * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IEventSubscription * This,
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
        
        DECLSPEC_XFGVIRT(IEventSubscription, get_SubscriptionID)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SubscriptionID )( 
            __RPC__in IEventSubscription * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrSubscriptionID);
        
        DECLSPEC_XFGVIRT(IEventSubscription, put_SubscriptionID)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_SubscriptionID )( 
            __RPC__in IEventSubscription * This,
            /* [in] */ __RPC__in BSTR bstrSubscriptionID);
        
        DECLSPEC_XFGVIRT(IEventSubscription, get_SubscriptionName)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SubscriptionName )( 
            __RPC__in IEventSubscription * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrSubscriptionName);
        
        DECLSPEC_XFGVIRT(IEventSubscription, put_SubscriptionName)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_SubscriptionName )( 
            __RPC__in IEventSubscription * This,
            /* [in] */ __RPC__in BSTR bstrSubscriptionName);
        
        DECLSPEC_XFGVIRT(IEventSubscription, get_PublisherID)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_PublisherID )( 
            __RPC__in IEventSubscription * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrPublisherID);
        
        DECLSPEC_XFGVIRT(IEventSubscription, put_PublisherID)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_PublisherID )( 
            __RPC__in IEventSubscription * This,
            /* [in] */ __RPC__in BSTR bstrPublisherID);
        
        DECLSPEC_XFGVIRT(IEventSubscription, get_EventClassID)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_EventClassID )( 
            __RPC__in IEventSubscription * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrEventClassID);
        
        DECLSPEC_XFGVIRT(IEventSubscription, put_EventClassID)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_EventClassID )( 
            __RPC__in IEventSubscription * This,
            /* [in] */ __RPC__in BSTR bstrEventClassID);
        
        DECLSPEC_XFGVIRT(IEventSubscription, get_MethodName)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_MethodName )( 
            __RPC__in IEventSubscription * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrMethodName);
        
        DECLSPEC_XFGVIRT(IEventSubscription, put_MethodName)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_MethodName )( 
            __RPC__in IEventSubscription * This,
            /* [in] */ __RPC__in BSTR bstrMethodName);
        
        DECLSPEC_XFGVIRT(IEventSubscription, get_SubscriberCLSID)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SubscriberCLSID )( 
            __RPC__in IEventSubscription * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrSubscriberCLSID);
        
        DECLSPEC_XFGVIRT(IEventSubscription, put_SubscriberCLSID)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_SubscriberCLSID )( 
            __RPC__in IEventSubscription * This,
            /* [in] */ __RPC__in BSTR bstrSubscriberCLSID);
        
        DECLSPEC_XFGVIRT(IEventSubscription, get_SubscriberInterface)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SubscriberInterface )( 
            __RPC__in IEventSubscription * This,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **ppSubscriberInterface);
        
        DECLSPEC_XFGVIRT(IEventSubscription, put_SubscriberInterface)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_SubscriberInterface )( 
            __RPC__in IEventSubscription * This,
            /* [in] */ __RPC__in_opt IUnknown *pSubscriberInterface);
        
        DECLSPEC_XFGVIRT(IEventSubscription, get_PerUser)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_PerUser )( 
            __RPC__in IEventSubscription * This,
            /* [retval][out] */ __RPC__out BOOL *pfPerUser);
        
        DECLSPEC_XFGVIRT(IEventSubscription, put_PerUser)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_PerUser )( 
            __RPC__in IEventSubscription * This,
            /* [in] */ BOOL fPerUser);
        
        DECLSPEC_XFGVIRT(IEventSubscription, get_OwnerSID)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_OwnerSID )( 
            __RPC__in IEventSubscription * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrOwnerSID);
        
        DECLSPEC_XFGVIRT(IEventSubscription, put_OwnerSID)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_OwnerSID )( 
            __RPC__in IEventSubscription * This,
            /* [in] */ __RPC__in BSTR bstrOwnerSID);
        
        DECLSPEC_XFGVIRT(IEventSubscription, get_Enabled)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Enabled )( 
            __RPC__in IEventSubscription * This,
            /* [retval][out] */ __RPC__out BOOL *pfEnabled);
        
        DECLSPEC_XFGVIRT(IEventSubscription, put_Enabled)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Enabled )( 
            __RPC__in IEventSubscription * This,
            /* [in] */ BOOL fEnabled);
        
        DECLSPEC_XFGVIRT(IEventSubscription, get_Description)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Description )( 
            __RPC__in IEventSubscription * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrDescription);
        
        DECLSPEC_XFGVIRT(IEventSubscription, put_Description)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Description )( 
            __RPC__in IEventSubscription * This,
            /* [in] */ __RPC__in BSTR bstrDescription);
        
        DECLSPEC_XFGVIRT(IEventSubscription, get_MachineName)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_MachineName )( 
            __RPC__in IEventSubscription * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrMachineName);
        
        DECLSPEC_XFGVIRT(IEventSubscription, put_MachineName)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_MachineName )( 
            __RPC__in IEventSubscription * This,
            /* [in] */ __RPC__in BSTR bstrMachineName);
        
        DECLSPEC_XFGVIRT(IEventSubscription, GetPublisherProperty)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetPublisherProperty )( 
            __RPC__in IEventSubscription * This,
            /* [in] */ __RPC__in BSTR bstrPropertyName,
            /* [retval][out] */ __RPC__out VARIANT *propertyValue);
        
        DECLSPEC_XFGVIRT(IEventSubscription, PutPublisherProperty)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *PutPublisherProperty )( 
            __RPC__in IEventSubscription * This,
            /* [in] */ __RPC__in BSTR bstrPropertyName,
            /* [in] */ __RPC__in VARIANT *propertyValue);
        
        DECLSPEC_XFGVIRT(IEventSubscription, RemovePublisherProperty)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *RemovePublisherProperty )( 
            __RPC__in IEventSubscription * This,
            /* [in] */ __RPC__in BSTR bstrPropertyName);
        
        DECLSPEC_XFGVIRT(IEventSubscription, GetPublisherPropertyCollection)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetPublisherPropertyCollection )( 
            __RPC__in IEventSubscription * This,
            /* [retval][out] */ __RPC__deref_out_opt IEventObjectCollection **collection);
        
        DECLSPEC_XFGVIRT(IEventSubscription, GetSubscriberProperty)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetSubscriberProperty )( 
            __RPC__in IEventSubscription * This,
            /* [in] */ __RPC__in BSTR bstrPropertyName,
            /* [retval][out] */ __RPC__out VARIANT *propertyValue);
        
        DECLSPEC_XFGVIRT(IEventSubscription, PutSubscriberProperty)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *PutSubscriberProperty )( 
            __RPC__in IEventSubscription * This,
            /* [in] */ __RPC__in BSTR bstrPropertyName,
            /* [in] */ __RPC__in VARIANT *propertyValue);
        
        DECLSPEC_XFGVIRT(IEventSubscription, RemoveSubscriberProperty)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *RemoveSubscriberProperty )( 
            __RPC__in IEventSubscription * This,
            /* [in] */ __RPC__in BSTR bstrPropertyName);
        
        DECLSPEC_XFGVIRT(IEventSubscription, GetSubscriberPropertyCollection)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetSubscriberPropertyCollection )( 
            __RPC__in IEventSubscription * This,
            /* [retval][out] */ __RPC__deref_out_opt IEventObjectCollection **collection);
        
        DECLSPEC_XFGVIRT(IEventSubscription, get_InterfaceID)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_InterfaceID )( 
            __RPC__in IEventSubscription * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrInterfaceID);
        
        DECLSPEC_XFGVIRT(IEventSubscription, put_InterfaceID)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_InterfaceID )( 
            __RPC__in IEventSubscription * This,
            /* [in] */ __RPC__in BSTR bstrInterfaceID);
        
        END_INTERFACE
    } IEventSubscriptionVtbl;

    interface IEventSubscription
    {
        CONST_VTBL struct IEventSubscriptionVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IEventSubscription_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IEventSubscription_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IEventSubscription_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IEventSubscription_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IEventSubscription_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IEventSubscription_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IEventSubscription_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IEventSubscription_get_SubscriptionID(This,pbstrSubscriptionID)	\
    ( (This)->lpVtbl -> get_SubscriptionID(This,pbstrSubscriptionID) ) 

#define IEventSubscription_put_SubscriptionID(This,bstrSubscriptionID)	\
    ( (This)->lpVtbl -> put_SubscriptionID(This,bstrSubscriptionID) ) 

#define IEventSubscription_get_SubscriptionName(This,pbstrSubscriptionName)	\
    ( (This)->lpVtbl -> get_SubscriptionName(This,pbstrSubscriptionName) ) 

#define IEventSubscription_put_SubscriptionName(This,bstrSubscriptionName)	\
    ( (This)->lpVtbl -> put_SubscriptionName(This,bstrSubscriptionName) ) 

#define IEventSubscription_get_PublisherID(This,pbstrPublisherID)	\
    ( (This)->lpVtbl -> get_PublisherID(This,pbstrPublisherID) ) 

#define IEventSubscription_put_PublisherID(This,bstrPublisherID)	\
    ( (This)->lpVtbl -> put_PublisherID(This,bstrPublisherID) ) 

#define IEventSubscription_get_EventClassID(This,pbstrEventClassID)	\
    ( (This)->lpVtbl -> get_EventClassID(This,pbstrEventClassID) ) 

#define IEventSubscription_put_EventClassID(This,bstrEventClassID)	\
    ( (This)->lpVtbl -> put_EventClassID(This,bstrEventClassID) ) 

#define IEventSubscription_get_MethodName(This,pbstrMethodName)	\
    ( (This)->lpVtbl -> get_MethodName(This,pbstrMethodName) ) 

#define IEventSubscription_put_MethodName(This,bstrMethodName)	\
    ( (This)->lpVtbl -> put_MethodName(This,bstrMethodName) ) 

#define IEventSubscription_get_SubscriberCLSID(This,pbstrSubscriberCLSID)	\
    ( (This)->lpVtbl -> get_SubscriberCLSID(This,pbstrSubscriberCLSID) ) 

#define IEventSubscription_put_SubscriberCLSID(This,bstrSubscriberCLSID)	\
    ( (This)->lpVtbl -> put_SubscriberCLSID(This,bstrSubscriberCLSID) ) 

#define IEventSubscription_get_SubscriberInterface(This,ppSubscriberInterface)	\
    ( (This)->lpVtbl -> get_SubscriberInterface(This,ppSubscriberInterface) ) 

#define IEventSubscription_put_SubscriberInterface(This,pSubscriberInterface)	\
    ( (This)->lpVtbl -> put_SubscriberInterface(This,pSubscriberInterface) ) 

#define IEventSubscription_get_PerUser(This,pfPerUser)	\
    ( (This)->lpVtbl -> get_PerUser(This,pfPerUser) ) 

#define IEventSubscription_put_PerUser(This,fPerUser)	\
    ( (This)->lpVtbl -> put_PerUser(This,fPerUser) ) 

#define IEventSubscription_get_OwnerSID(This,pbstrOwnerSID)	\
    ( (This)->lpVtbl -> get_OwnerSID(This,pbstrOwnerSID) ) 

#define IEventSubscription_put_OwnerSID(This,bstrOwnerSID)	\
    ( (This)->lpVtbl -> put_OwnerSID(This,bstrOwnerSID) ) 

#define IEventSubscription_get_Enabled(This,pfEnabled)	\
    ( (This)->lpVtbl -> get_Enabled(This,pfEnabled) ) 

#define IEventSubscription_put_Enabled(This,fEnabled)	\
    ( (This)->lpVtbl -> put_Enabled(This,fEnabled) ) 

#define IEventSubscription_get_Description(This,pbstrDescription)	\
    ( (This)->lpVtbl -> get_Description(This,pbstrDescription) ) 

#define IEventSubscription_put_Description(This,bstrDescription)	\
    ( (This)->lpVtbl -> put_Description(This,bstrDescription) ) 

#define IEventSubscription_get_MachineName(This,pbstrMachineName)	\
    ( (This)->lpVtbl -> get_MachineName(This,pbstrMachineName) ) 

#define IEventSubscription_put_MachineName(This,bstrMachineName)	\
    ( (This)->lpVtbl -> put_MachineName(This,bstrMachineName) ) 

#define IEventSubscription_GetPublisherProperty(This,bstrPropertyName,propertyValue)	\
    ( (This)->lpVtbl -> GetPublisherProperty(This,bstrPropertyName,propertyValue) ) 

#define IEventSubscription_PutPublisherProperty(This,bstrPropertyName,propertyValue)	\
    ( (This)->lpVtbl -> PutPublisherProperty(This,bstrPropertyName,propertyValue) ) 

#define IEventSubscription_RemovePublisherProperty(This,bstrPropertyName)	\
    ( (This)->lpVtbl -> RemovePublisherProperty(This,bstrPropertyName) ) 

#define IEventSubscription_GetPublisherPropertyCollection(This,collection)	\
    ( (This)->lpVtbl -> GetPublisherPropertyCollection(This,collection) ) 

#define IEventSubscription_GetSubscriberProperty(This,bstrPropertyName,propertyValue)	\
    ( (This)->lpVtbl -> GetSubscriberProperty(This,bstrPropertyName,propertyValue) ) 

#define IEventSubscription_PutSubscriberProperty(This,bstrPropertyName,propertyValue)	\
    ( (This)->lpVtbl -> PutSubscriberProperty(This,bstrPropertyName,propertyValue) ) 

#define IEventSubscription_RemoveSubscriberProperty(This,bstrPropertyName)	\
    ( (This)->lpVtbl -> RemoveSubscriberProperty(This,bstrPropertyName) ) 

#define IEventSubscription_GetSubscriberPropertyCollection(This,collection)	\
    ( (This)->lpVtbl -> GetSubscriberPropertyCollection(This,collection) ) 

#define IEventSubscription_get_InterfaceID(This,pbstrInterfaceID)	\
    ( (This)->lpVtbl -> get_InterfaceID(This,pbstrInterfaceID) ) 

#define IEventSubscription_put_InterfaceID(This,bstrInterfaceID)	\
    ( (This)->lpVtbl -> put_InterfaceID(This,bstrInterfaceID) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IEventSubscription_INTERFACE_DEFINED__ */


#ifndef __IFiringControl_INTERFACE_DEFINED__
#define __IFiringControl_INTERFACE_DEFINED__

/* interface IFiringControl */
/* [unique][helpstring][proxy][dual][uuid][object] */ 


EXTERN_C const IID IID_IFiringControl;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("e0498c93-4efe-11d1-9971-00c04fbbb345")
    IFiringControl : public IDispatch
    {
    public:
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE FireSubscription( 
            /* [in] */ __RPC__in_opt IEventSubscription *subscription) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IFiringControlVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IFiringControl * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IFiringControl * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IFiringControl * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IFiringControl * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IFiringControl * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IFiringControl * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IFiringControl * This,
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
        
        DECLSPEC_XFGVIRT(IFiringControl, FireSubscription)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *FireSubscription )( 
            __RPC__in IFiringControl * This,
            /* [in] */ __RPC__in_opt IEventSubscription *subscription);
        
        END_INTERFACE
    } IFiringControlVtbl;

    interface IFiringControl
    {
        CONST_VTBL struct IFiringControlVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IFiringControl_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IFiringControl_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IFiringControl_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IFiringControl_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IFiringControl_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IFiringControl_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IFiringControl_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IFiringControl_FireSubscription(This,subscription)	\
    ( (This)->lpVtbl -> FireSubscription(This,subscription) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IFiringControl_INTERFACE_DEFINED__ */


#ifndef __IPublisherFilter_INTERFACE_DEFINED__
#define __IPublisherFilter_INTERFACE_DEFINED__

/* interface IPublisherFilter */
/* [unique][helpstring][uuid][object] */ 

// ****************************************************************************
// This is a Deprecated interface - Use IMultiInterfacePublisherFilter instead.
// ****************************************************************************

EXTERN_C const IID IID_IPublisherFilter;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("465e5cc0-7b26-11d1-88fb-0080c7d771bf")
    IPublisherFilter : public IUnknown
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE Initialize( 
            /* [in] */ __RPC__in BSTR methodName,
            /* [unique][in] */ __RPC__in_opt IDispatch *dispUserDefined) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE PrepareToFire( 
            /* [in] */ __RPC__in BSTR methodName,
            /* [in] */ __RPC__in_opt IFiringControl *firingControl) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IPublisherFilterVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IPublisherFilter * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IPublisherFilter * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IPublisherFilter * This);
        
        DECLSPEC_XFGVIRT(IPublisherFilter, Initialize)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *Initialize )( 
            __RPC__in IPublisherFilter * This,
            /* [in] */ __RPC__in BSTR methodName,
            /* [unique][in] */ __RPC__in_opt IDispatch *dispUserDefined);
        
        DECLSPEC_XFGVIRT(IPublisherFilter, PrepareToFire)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *PrepareToFire )( 
            __RPC__in IPublisherFilter * This,
            /* [in] */ __RPC__in BSTR methodName,
            /* [in] */ __RPC__in_opt IFiringControl *firingControl);
        
        END_INTERFACE
    } IPublisherFilterVtbl;

    interface IPublisherFilter
    {
        CONST_VTBL struct IPublisherFilterVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IPublisherFilter_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IPublisherFilter_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IPublisherFilter_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IPublisherFilter_Initialize(This,methodName,dispUserDefined)	\
    ( (This)->lpVtbl -> Initialize(This,methodName,dispUserDefined) ) 

#define IPublisherFilter_PrepareToFire(This,methodName,firingControl)	\
    ( (This)->lpVtbl -> PrepareToFire(This,methodName,firingControl) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IPublisherFilter_INTERFACE_DEFINED__ */


#ifndef __IMultiInterfacePublisherFilter_INTERFACE_DEFINED__
#define __IMultiInterfacePublisherFilter_INTERFACE_DEFINED__

/* interface IMultiInterfacePublisherFilter */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IMultiInterfacePublisherFilter;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("465e5cc1-7b26-11d1-88fb-0080c7d771bf")
    IMultiInterfacePublisherFilter : public IUnknown
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE Initialize( 
            /* [in] */ __RPC__in_opt IMultiInterfaceEventControl *pEIC) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE PrepareToFire( 
            /* [in] */ __RPC__in REFIID iid,
            /* [in] */ __RPC__in BSTR methodName,
            /* [in] */ __RPC__in_opt IFiringControl *firingControl) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMultiInterfacePublisherFilterVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMultiInterfacePublisherFilter * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMultiInterfacePublisherFilter * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMultiInterfacePublisherFilter * This);
        
        DECLSPEC_XFGVIRT(IMultiInterfacePublisherFilter, Initialize)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *Initialize )( 
            __RPC__in IMultiInterfacePublisherFilter * This,
            /* [in] */ __RPC__in_opt IMultiInterfaceEventControl *pEIC);
        
        DECLSPEC_XFGVIRT(IMultiInterfacePublisherFilter, PrepareToFire)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *PrepareToFire )( 
            __RPC__in IMultiInterfacePublisherFilter * This,
            /* [in] */ __RPC__in REFIID iid,
            /* [in] */ __RPC__in BSTR methodName,
            /* [in] */ __RPC__in_opt IFiringControl *firingControl);
        
        END_INTERFACE
    } IMultiInterfacePublisherFilterVtbl;

    interface IMultiInterfacePublisherFilter
    {
        CONST_VTBL struct IMultiInterfacePublisherFilterVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMultiInterfacePublisherFilter_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMultiInterfacePublisherFilter_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMultiInterfacePublisherFilter_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMultiInterfacePublisherFilter_Initialize(This,pEIC)	\
    ( (This)->lpVtbl -> Initialize(This,pEIC) ) 

#define IMultiInterfacePublisherFilter_PrepareToFire(This,iid,methodName,firingControl)	\
    ( (This)->lpVtbl -> PrepareToFire(This,iid,methodName,firingControl) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMultiInterfacePublisherFilter_INTERFACE_DEFINED__ */


#ifndef __IEventObjectChange_INTERFACE_DEFINED__
#define __IEventObjectChange_INTERFACE_DEFINED__

/* interface IEventObjectChange */
/* [unique][helpstring][uuid][object] */ 

typedef /* [public][public][public][public][public][public] */ 
enum __MIDL_IEventObjectChange_0001
    {
        EOC_NewObject	= 0,
        EOC_ModifiedObject	= ( EOC_NewObject + 1 ) ,
        EOC_DeletedObject	= ( EOC_ModifiedObject + 1 ) 
    } 	EOC_ChangeType;


EXTERN_C const IID IID_IEventObjectChange;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("F4A07D70-2E25-11D1-9964-00C04FBBB345")
    IEventObjectChange : public IUnknown
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE ChangedSubscription( 
            /* [in] */ EOC_ChangeType changeType,
            /* [in] */ __RPC__in BSTR bstrSubscriptionID) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE ChangedEventClass( 
            /* [in] */ EOC_ChangeType changeType,
            /* [in] */ __RPC__in BSTR bstrEventClassID) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE ChangedPublisher( 
            /* [in] */ EOC_ChangeType changeType,
            /* [in] */ __RPC__in BSTR bstrPublisherID) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IEventObjectChangeVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IEventObjectChange * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IEventObjectChange * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IEventObjectChange * This);
        
        DECLSPEC_XFGVIRT(IEventObjectChange, ChangedSubscription)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *ChangedSubscription )( 
            __RPC__in IEventObjectChange * This,
            /* [in] */ EOC_ChangeType changeType,
            /* [in] */ __RPC__in BSTR bstrSubscriptionID);
        
        DECLSPEC_XFGVIRT(IEventObjectChange, ChangedEventClass)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *ChangedEventClass )( 
            __RPC__in IEventObjectChange * This,
            /* [in] */ EOC_ChangeType changeType,
            /* [in] */ __RPC__in BSTR bstrEventClassID);
        
        DECLSPEC_XFGVIRT(IEventObjectChange, ChangedPublisher)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *ChangedPublisher )( 
            __RPC__in IEventObjectChange * This,
            /* [in] */ EOC_ChangeType changeType,
            /* [in] */ __RPC__in BSTR bstrPublisherID);
        
        END_INTERFACE
    } IEventObjectChangeVtbl;

    interface IEventObjectChange
    {
        CONST_VTBL struct IEventObjectChangeVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IEventObjectChange_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IEventObjectChange_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IEventObjectChange_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IEventObjectChange_ChangedSubscription(This,changeType,bstrSubscriptionID)	\
    ( (This)->lpVtbl -> ChangedSubscription(This,changeType,bstrSubscriptionID) ) 

#define IEventObjectChange_ChangedEventClass(This,changeType,bstrEventClassID)	\
    ( (This)->lpVtbl -> ChangedEventClass(This,changeType,bstrEventClassID) ) 

#define IEventObjectChange_ChangedPublisher(This,changeType,bstrPublisherID)	\
    ( (This)->lpVtbl -> ChangedPublisher(This,changeType,bstrPublisherID) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IEventObjectChange_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_eventsys_0000_0009 */
/* [local] */ 

#ifndef _COMEVENTSYSCHANGEINFO_
#define _COMEVENTSYSCHANGEINFO_
typedef /* [public][public][public][hidden] */ struct __MIDL___MIDL_itf_eventsys_0000_0009_0001
    {
    DWORD cbSize;
    EOC_ChangeType changeType;
    BSTR objectId;
    BSTR partitionId;
    BSTR applicationId;
    GUID reserved[ 10 ];
    } 	COMEVENTSYSCHANGEINFO;

#endif // _COMEVENTSYSCHANGEINFO_


extern RPC_IF_HANDLE __MIDL_itf_eventsys_0000_0009_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_eventsys_0000_0009_v0_0_s_ifspec;

#ifndef __IEventObjectChange2_INTERFACE_DEFINED__
#define __IEventObjectChange2_INTERFACE_DEFINED__

/* interface IEventObjectChange2 */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IEventObjectChange2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("7701A9C3-BD68-438f-83E0-67BF4F53A422")
    IEventObjectChange2 : public IUnknown
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE ChangedSubscription( 
            /* [in] */ __RPC__in COMEVENTSYSCHANGEINFO *pInfo) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE ChangedEventClass( 
            /* [in] */ __RPC__in COMEVENTSYSCHANGEINFO *pInfo) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IEventObjectChange2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IEventObjectChange2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IEventObjectChange2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IEventObjectChange2 * This);
        
        DECLSPEC_XFGVIRT(IEventObjectChange2, ChangedSubscription)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *ChangedSubscription )( 
            __RPC__in IEventObjectChange2 * This,
            /* [in] */ __RPC__in COMEVENTSYSCHANGEINFO *pInfo);
        
        DECLSPEC_XFGVIRT(IEventObjectChange2, ChangedEventClass)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *ChangedEventClass )( 
            __RPC__in IEventObjectChange2 * This,
            /* [in] */ __RPC__in COMEVENTSYSCHANGEINFO *pInfo);
        
        END_INTERFACE
    } IEventObjectChange2Vtbl;

    interface IEventObjectChange2
    {
        CONST_VTBL struct IEventObjectChange2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IEventObjectChange2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IEventObjectChange2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IEventObjectChange2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IEventObjectChange2_ChangedSubscription(This,pInfo)	\
    ( (This)->lpVtbl -> ChangedSubscription(This,pInfo) ) 

#define IEventObjectChange2_ChangedEventClass(This,pInfo)	\
    ( (This)->lpVtbl -> ChangedEventClass(This,pInfo) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IEventObjectChange2_INTERFACE_DEFINED__ */


#ifndef __IEnumEventObject_INTERFACE_DEFINED__
#define __IEnumEventObject_INTERFACE_DEFINED__

/* interface IEnumEventObject */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IEnumEventObject;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("F4A07D63-2E25-11D1-9964-00C04FBBB345")
    IEnumEventObject : public IUnknown
    {
    public:
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Clone( 
            /* [out] */ __RPC__deref_out_opt IEnumEventObject **ppInterface) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Next( 
            /* [in] */ ULONG cReqElem,
            /* [length_is][size_is][out] */ __RPC__out_ecount_part(cReqElem, *cRetElem) IUnknown **ppInterface,
            /* [out] */ __RPC__out ULONG *cRetElem) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Reset( void) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Skip( 
            /* [in] */ ULONG cSkipElem) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IEnumEventObjectVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IEnumEventObject * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IEnumEventObject * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IEnumEventObject * This);
        
        DECLSPEC_XFGVIRT(IEnumEventObject, Clone)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Clone )( 
            __RPC__in IEnumEventObject * This,
            /* [out] */ __RPC__deref_out_opt IEnumEventObject **ppInterface);
        
        DECLSPEC_XFGVIRT(IEnumEventObject, Next)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Next )( 
            __RPC__in IEnumEventObject * This,
            /* [in] */ ULONG cReqElem,
            /* [length_is][size_is][out] */ __RPC__out_ecount_part(cReqElem, *cRetElem) IUnknown **ppInterface,
            /* [out] */ __RPC__out ULONG *cRetElem);
        
        DECLSPEC_XFGVIRT(IEnumEventObject, Reset)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Reset )( 
            __RPC__in IEnumEventObject * This);
        
        DECLSPEC_XFGVIRT(IEnumEventObject, Skip)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Skip )( 
            __RPC__in IEnumEventObject * This,
            /* [in] */ ULONG cSkipElem);
        
        END_INTERFACE
    } IEnumEventObjectVtbl;

    interface IEnumEventObject
    {
        CONST_VTBL struct IEnumEventObjectVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IEnumEventObject_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IEnumEventObject_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IEnumEventObject_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IEnumEventObject_Clone(This,ppInterface)	\
    ( (This)->lpVtbl -> Clone(This,ppInterface) ) 

#define IEnumEventObject_Next(This,cReqElem,ppInterface,cRetElem)	\
    ( (This)->lpVtbl -> Next(This,cReqElem,ppInterface,cRetElem) ) 

#define IEnumEventObject_Reset(This)	\
    ( (This)->lpVtbl -> Reset(This) ) 

#define IEnumEventObject_Skip(This,cSkipElem)	\
    ( (This)->lpVtbl -> Skip(This,cSkipElem) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IEnumEventObject_INTERFACE_DEFINED__ */


#ifndef __IEventObjectCollection_INTERFACE_DEFINED__
#define __IEventObjectCollection_INTERFACE_DEFINED__

/* interface IEventObjectCollection */
/* [unique][helpstring][proxy][dual][uuid][object] */ 


EXTERN_C const IID IID_IEventObjectCollection;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("f89ac270-d4eb-11d1-b682-00805fc79216")
    IEventObjectCollection : public IDispatch
    {
    public:
        virtual /* [helpstring][restricted][propget][id] */ HRESULT STDMETHODCALLTYPE get__NewEnum( 
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **ppUnkEnum) = 0;
        
        virtual /* [propget][id] */ HRESULT STDMETHODCALLTYPE get_Item( 
            /* [in] */ __RPC__in BSTR objectID,
            /* [retval][out] */ __RPC__out VARIANT *pItem) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_NewEnum( 
            /* [retval][out] */ __RPC__deref_out_opt IEnumEventObject **ppEnum) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Count( 
            /* [retval][out] */ __RPC__out long *pCount) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Add( 
            /* [in] */ __RPC__in VARIANT *item,
            /* [in] */ __RPC__in BSTR objectID) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Remove( 
            /* [in] */ __RPC__in BSTR objectID) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IEventObjectCollectionVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IEventObjectCollection * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IEventObjectCollection * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IEventObjectCollection * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IEventObjectCollection * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IEventObjectCollection * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IEventObjectCollection * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IEventObjectCollection * This,
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
        
        DECLSPEC_XFGVIRT(IEventObjectCollection, get__NewEnum)
        /* [helpstring][restricted][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get__NewEnum )( 
            __RPC__in IEventObjectCollection * This,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **ppUnkEnum);
        
        DECLSPEC_XFGVIRT(IEventObjectCollection, get_Item)
        /* [propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Item )( 
            __RPC__in IEventObjectCollection * This,
            /* [in] */ __RPC__in BSTR objectID,
            /* [retval][out] */ __RPC__out VARIANT *pItem);
        
        DECLSPEC_XFGVIRT(IEventObjectCollection, get_NewEnum)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_NewEnum )( 
            __RPC__in IEventObjectCollection * This,
            /* [retval][out] */ __RPC__deref_out_opt IEnumEventObject **ppEnum);
        
        DECLSPEC_XFGVIRT(IEventObjectCollection, get_Count)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Count )( 
            __RPC__in IEventObjectCollection * This,
            /* [retval][out] */ __RPC__out long *pCount);
        
        DECLSPEC_XFGVIRT(IEventObjectCollection, Add)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Add )( 
            __RPC__in IEventObjectCollection * This,
            /* [in] */ __RPC__in VARIANT *item,
            /* [in] */ __RPC__in BSTR objectID);
        
        DECLSPEC_XFGVIRT(IEventObjectCollection, Remove)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Remove )( 
            __RPC__in IEventObjectCollection * This,
            /* [in] */ __RPC__in BSTR objectID);
        
        END_INTERFACE
    } IEventObjectCollectionVtbl;

    interface IEventObjectCollection
    {
        CONST_VTBL struct IEventObjectCollectionVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IEventObjectCollection_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IEventObjectCollection_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IEventObjectCollection_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IEventObjectCollection_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IEventObjectCollection_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IEventObjectCollection_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IEventObjectCollection_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IEventObjectCollection_get__NewEnum(This,ppUnkEnum)	\
    ( (This)->lpVtbl -> get__NewEnum(This,ppUnkEnum) ) 

#define IEventObjectCollection_get_Item(This,objectID,pItem)	\
    ( (This)->lpVtbl -> get_Item(This,objectID,pItem) ) 

#define IEventObjectCollection_get_NewEnum(This,ppEnum)	\
    ( (This)->lpVtbl -> get_NewEnum(This,ppEnum) ) 

#define IEventObjectCollection_get_Count(This,pCount)	\
    ( (This)->lpVtbl -> get_Count(This,pCount) ) 

#define IEventObjectCollection_Add(This,item,objectID)	\
    ( (This)->lpVtbl -> Add(This,item,objectID) ) 

#define IEventObjectCollection_Remove(This,objectID)	\
    ( (This)->lpVtbl -> Remove(This,objectID) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IEventObjectCollection_INTERFACE_DEFINED__ */


#ifndef __IEventProperty_INTERFACE_DEFINED__
#define __IEventProperty_INTERFACE_DEFINED__

/* interface IEventProperty */
/* [unique][helpstring][proxy][dual][uuid][object] */ 


EXTERN_C const IID IID_IEventProperty;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("da538ee2-f4de-11d1-b6bb-00805fc79216")
    IEventProperty : public IDispatch
    {
    public:
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Name( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *propertyName) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_Name( 
            /* [in] */ __RPC__in BSTR propertyName) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Value( 
            /* [retval][out] */ __RPC__out VARIANT *propertyValue) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_Value( 
            /* [in] */ __RPC__in VARIANT *propertyValue) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IEventPropertyVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IEventProperty * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IEventProperty * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IEventProperty * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IEventProperty * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IEventProperty * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IEventProperty * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IEventProperty * This,
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
        
        DECLSPEC_XFGVIRT(IEventProperty, get_Name)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Name )( 
            __RPC__in IEventProperty * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *propertyName);
        
        DECLSPEC_XFGVIRT(IEventProperty, put_Name)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Name )( 
            __RPC__in IEventProperty * This,
            /* [in] */ __RPC__in BSTR propertyName);
        
        DECLSPEC_XFGVIRT(IEventProperty, get_Value)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Value )( 
            __RPC__in IEventProperty * This,
            /* [retval][out] */ __RPC__out VARIANT *propertyValue);
        
        DECLSPEC_XFGVIRT(IEventProperty, put_Value)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Value )( 
            __RPC__in IEventProperty * This,
            /* [in] */ __RPC__in VARIANT *propertyValue);
        
        END_INTERFACE
    } IEventPropertyVtbl;

    interface IEventProperty
    {
        CONST_VTBL struct IEventPropertyVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IEventProperty_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IEventProperty_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IEventProperty_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IEventProperty_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IEventProperty_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IEventProperty_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IEventProperty_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IEventProperty_get_Name(This,propertyName)	\
    ( (This)->lpVtbl -> get_Name(This,propertyName) ) 

#define IEventProperty_put_Name(This,propertyName)	\
    ( (This)->lpVtbl -> put_Name(This,propertyName) ) 

#define IEventProperty_get_Value(This,propertyValue)	\
    ( (This)->lpVtbl -> get_Value(This,propertyValue) ) 

#define IEventProperty_put_Value(This,propertyValue)	\
    ( (This)->lpVtbl -> put_Value(This,propertyValue) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IEventProperty_INTERFACE_DEFINED__ */


#ifndef __IEventControl_INTERFACE_DEFINED__
#define __IEventControl_INTERFACE_DEFINED__

/* interface IEventControl */
/* [unique][helpstring][proxy][dual][uuid][object] */ 

// *************************************************************************
// This is a Deprecated interface - Use IMultiInterfaceEventControl instead.
// *************************************************************************

EXTERN_C const IID IID_IEventControl;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("0343e2f4-86f6-11d1-b760-00c04fb926af")
    IEventControl : public IDispatch
    {
    public:
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE SetPublisherFilter( 
            /* [in] */ __RPC__in BSTR methodName,
            /* [unique][in] */ __RPC__in_opt IPublisherFilter *pPublisherFilter) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_AllowInprocActivation( 
            /* [retval][out] */ __RPC__out BOOL *pfAllowInprocActivation) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_AllowInprocActivation( 
            /* [in] */ BOOL fAllowInprocActivation) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetSubscriptions( 
            /* [unique][in] */ __RPC__in_opt BSTR methodName,
            /* [unique][in] */ __RPC__in_opt BSTR optionalCriteria,
            /* [unique][in] */ __RPC__in_opt int *optionalErrorIndex,
            /* [retval][out] */ __RPC__deref_out_opt IEventObjectCollection **ppCollection) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE SetDefaultQuery( 
            /* [in] */ __RPC__in BSTR methodName,
            /* [in] */ __RPC__in BSTR criteria,
            /* [retval][out] */ __RPC__out int *errorIndex) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IEventControlVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IEventControl * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IEventControl * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IEventControl * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IEventControl * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IEventControl * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IEventControl * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IEventControl * This,
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
        
        DECLSPEC_XFGVIRT(IEventControl, SetPublisherFilter)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *SetPublisherFilter )( 
            __RPC__in IEventControl * This,
            /* [in] */ __RPC__in BSTR methodName,
            /* [unique][in] */ __RPC__in_opt IPublisherFilter *pPublisherFilter);
        
        DECLSPEC_XFGVIRT(IEventControl, get_AllowInprocActivation)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_AllowInprocActivation )( 
            __RPC__in IEventControl * This,
            /* [retval][out] */ __RPC__out BOOL *pfAllowInprocActivation);
        
        DECLSPEC_XFGVIRT(IEventControl, put_AllowInprocActivation)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_AllowInprocActivation )( 
            __RPC__in IEventControl * This,
            /* [in] */ BOOL fAllowInprocActivation);
        
        DECLSPEC_XFGVIRT(IEventControl, GetSubscriptions)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetSubscriptions )( 
            __RPC__in IEventControl * This,
            /* [unique][in] */ __RPC__in_opt BSTR methodName,
            /* [unique][in] */ __RPC__in_opt BSTR optionalCriteria,
            /* [unique][in] */ __RPC__in_opt int *optionalErrorIndex,
            /* [retval][out] */ __RPC__deref_out_opt IEventObjectCollection **ppCollection);
        
        DECLSPEC_XFGVIRT(IEventControl, SetDefaultQuery)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *SetDefaultQuery )( 
            __RPC__in IEventControl * This,
            /* [in] */ __RPC__in BSTR methodName,
            /* [in] */ __RPC__in BSTR criteria,
            /* [retval][out] */ __RPC__out int *errorIndex);
        
        END_INTERFACE
    } IEventControlVtbl;

    interface IEventControl
    {
        CONST_VTBL struct IEventControlVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IEventControl_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IEventControl_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IEventControl_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IEventControl_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IEventControl_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IEventControl_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IEventControl_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IEventControl_SetPublisherFilter(This,methodName,pPublisherFilter)	\
    ( (This)->lpVtbl -> SetPublisherFilter(This,methodName,pPublisherFilter) ) 

#define IEventControl_get_AllowInprocActivation(This,pfAllowInprocActivation)	\
    ( (This)->lpVtbl -> get_AllowInprocActivation(This,pfAllowInprocActivation) ) 

#define IEventControl_put_AllowInprocActivation(This,fAllowInprocActivation)	\
    ( (This)->lpVtbl -> put_AllowInprocActivation(This,fAllowInprocActivation) ) 

#define IEventControl_GetSubscriptions(This,methodName,optionalCriteria,optionalErrorIndex,ppCollection)	\
    ( (This)->lpVtbl -> GetSubscriptions(This,methodName,optionalCriteria,optionalErrorIndex,ppCollection) ) 

#define IEventControl_SetDefaultQuery(This,methodName,criteria,errorIndex)	\
    ( (This)->lpVtbl -> SetDefaultQuery(This,methodName,criteria,errorIndex) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IEventControl_INTERFACE_DEFINED__ */


#ifndef __IMultiInterfaceEventControl_INTERFACE_DEFINED__
#define __IMultiInterfaceEventControl_INTERFACE_DEFINED__

/* interface IMultiInterfaceEventControl */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IMultiInterfaceEventControl;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("0343e2f5-86f6-11d1-b760-00c04fb926af")
    IMultiInterfaceEventControl : public IUnknown
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE SetMultiInterfacePublisherFilter( 
            /* [unique][in] */ __RPC__in_opt IMultiInterfacePublisherFilter *classFilter) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetSubscriptions( 
            /* [in] */ __RPC__in REFIID eventIID,
            /* [unique][in] */ __RPC__in_opt BSTR bstrMethodName,
            /* [unique][in] */ __RPC__in_opt BSTR optionalCriteria,
            /* [unique][in] */ __RPC__in_opt int *optionalErrorIndex,
            /* [retval][out] */ __RPC__deref_out_opt IEventObjectCollection **ppCollection) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE SetDefaultQuery( 
            /* [in] */ __RPC__in REFIID eventIID,
            /* [in] */ __RPC__in BSTR bstrMethodName,
            /* [in] */ __RPC__in BSTR bstrCriteria,
            /* [retval][out] */ __RPC__out int *errorIndex) = 0;
        
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_AllowInprocActivation( 
            /* [retval][out] */ __RPC__out BOOL *pfAllowInprocActivation) = 0;
        
        virtual /* [helpstring][propput] */ HRESULT STDMETHODCALLTYPE put_AllowInprocActivation( 
            /* [in] */ BOOL fAllowInprocActivation) = 0;
        
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_FireInParallel( 
            /* [retval][out] */ __RPC__out BOOL *pfFireInParallel) = 0;
        
        virtual /* [helpstring][propput] */ HRESULT STDMETHODCALLTYPE put_FireInParallel( 
            /* [in] */ BOOL fFireInParallel) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMultiInterfaceEventControlVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMultiInterfaceEventControl * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMultiInterfaceEventControl * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMultiInterfaceEventControl * This);
        
        DECLSPEC_XFGVIRT(IMultiInterfaceEventControl, SetMultiInterfacePublisherFilter)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *SetMultiInterfacePublisherFilter )( 
            __RPC__in IMultiInterfaceEventControl * This,
            /* [unique][in] */ __RPC__in_opt IMultiInterfacePublisherFilter *classFilter);
        
        DECLSPEC_XFGVIRT(IMultiInterfaceEventControl, GetSubscriptions)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetSubscriptions )( 
            __RPC__in IMultiInterfaceEventControl * This,
            /* [in] */ __RPC__in REFIID eventIID,
            /* [unique][in] */ __RPC__in_opt BSTR bstrMethodName,
            /* [unique][in] */ __RPC__in_opt BSTR optionalCriteria,
            /* [unique][in] */ __RPC__in_opt int *optionalErrorIndex,
            /* [retval][out] */ __RPC__deref_out_opt IEventObjectCollection **ppCollection);
        
        DECLSPEC_XFGVIRT(IMultiInterfaceEventControl, SetDefaultQuery)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *SetDefaultQuery )( 
            __RPC__in IMultiInterfaceEventControl * This,
            /* [in] */ __RPC__in REFIID eventIID,
            /* [in] */ __RPC__in BSTR bstrMethodName,
            /* [in] */ __RPC__in BSTR bstrCriteria,
            /* [retval][out] */ __RPC__out int *errorIndex);
        
        DECLSPEC_XFGVIRT(IMultiInterfaceEventControl, get_AllowInprocActivation)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_AllowInprocActivation )( 
            __RPC__in IMultiInterfaceEventControl * This,
            /* [retval][out] */ __RPC__out BOOL *pfAllowInprocActivation);
        
        DECLSPEC_XFGVIRT(IMultiInterfaceEventControl, put_AllowInprocActivation)
        /* [helpstring][propput] */ HRESULT ( STDMETHODCALLTYPE *put_AllowInprocActivation )( 
            __RPC__in IMultiInterfaceEventControl * This,
            /* [in] */ BOOL fAllowInprocActivation);
        
        DECLSPEC_XFGVIRT(IMultiInterfaceEventControl, get_FireInParallel)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_FireInParallel )( 
            __RPC__in IMultiInterfaceEventControl * This,
            /* [retval][out] */ __RPC__out BOOL *pfFireInParallel);
        
        DECLSPEC_XFGVIRT(IMultiInterfaceEventControl, put_FireInParallel)
        /* [helpstring][propput] */ HRESULT ( STDMETHODCALLTYPE *put_FireInParallel )( 
            __RPC__in IMultiInterfaceEventControl * This,
            /* [in] */ BOOL fFireInParallel);
        
        END_INTERFACE
    } IMultiInterfaceEventControlVtbl;

    interface IMultiInterfaceEventControl
    {
        CONST_VTBL struct IMultiInterfaceEventControlVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMultiInterfaceEventControl_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMultiInterfaceEventControl_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMultiInterfaceEventControl_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMultiInterfaceEventControl_SetMultiInterfacePublisherFilter(This,classFilter)	\
    ( (This)->lpVtbl -> SetMultiInterfacePublisherFilter(This,classFilter) ) 

#define IMultiInterfaceEventControl_GetSubscriptions(This,eventIID,bstrMethodName,optionalCriteria,optionalErrorIndex,ppCollection)	\
    ( (This)->lpVtbl -> GetSubscriptions(This,eventIID,bstrMethodName,optionalCriteria,optionalErrorIndex,ppCollection) ) 

#define IMultiInterfaceEventControl_SetDefaultQuery(This,eventIID,bstrMethodName,bstrCriteria,errorIndex)	\
    ( (This)->lpVtbl -> SetDefaultQuery(This,eventIID,bstrMethodName,bstrCriteria,errorIndex) ) 

#define IMultiInterfaceEventControl_get_AllowInprocActivation(This,pfAllowInprocActivation)	\
    ( (This)->lpVtbl -> get_AllowInprocActivation(This,pfAllowInprocActivation) ) 

#define IMultiInterfaceEventControl_put_AllowInprocActivation(This,fAllowInprocActivation)	\
    ( (This)->lpVtbl -> put_AllowInprocActivation(This,fAllowInprocActivation) ) 

#define IMultiInterfaceEventControl_get_FireInParallel(This,pfFireInParallel)	\
    ( (This)->lpVtbl -> get_FireInParallel(This,pfFireInParallel) ) 

#define IMultiInterfaceEventControl_put_FireInParallel(This,fFireInParallel)	\
    ( (This)->lpVtbl -> put_FireInParallel(This,fFireInParallel) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMultiInterfaceEventControl_INTERFACE_DEFINED__ */


#ifndef __IDontSupportEventSubscription_INTERFACE_DEFINED__
#define __IDontSupportEventSubscription_INTERFACE_DEFINED__

/* interface IDontSupportEventSubscription */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IDontSupportEventSubscription;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("784121F1-62A6-4b89-855F-D65F296DE83A")
    IDontSupportEventSubscription : public IUnknown
    {
    public:
    };
    
    
#else 	/* C style interface */

    typedef struct IDontSupportEventSubscriptionVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IDontSupportEventSubscription * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IDontSupportEventSubscription * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IDontSupportEventSubscription * This);
        
        END_INTERFACE
    } IDontSupportEventSubscriptionVtbl;

    interface IDontSupportEventSubscription
    {
        CONST_VTBL struct IDontSupportEventSubscriptionVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDontSupportEventSubscription_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDontSupportEventSubscription_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDontSupportEventSubscription_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDontSupportEventSubscription_INTERFACE_DEFINED__ */



#ifndef __DummyEventSystemLib_LIBRARY_DEFINED__
#define __DummyEventSystemLib_LIBRARY_DEFINED__

/* library DummyEventSystemLib */
/* [helpstring][version][uuid] */ 


EXTERN_C const IID LIBID_DummyEventSystemLib;

EXTERN_C const CLSID CLSID_CEventSystem;

#ifdef __cplusplus

class DECLSPEC_UUID("4E14FBA2-2E22-11D1-9964-00C04FBBB345")
CEventSystem;
#endif

EXTERN_C const CLSID CLSID_CEventPublisher;

#ifdef __cplusplus

class DECLSPEC_UUID("ab944620-79c6-11d1-88f9-0080c7d771bf")
CEventPublisher;
#endif

EXTERN_C const CLSID CLSID_CEventClass;

#ifdef __cplusplus

class DECLSPEC_UUID("cdbec9c0-7a68-11d1-88f9-0080c7d771bf")
CEventClass;
#endif

EXTERN_C const CLSID CLSID_CEventSubscription;

#ifdef __cplusplus

class DECLSPEC_UUID("7542e960-79c7-11d1-88f9-0080c7d771bf")
CEventSubscription;
#endif

EXTERN_C const CLSID CLSID_EventObjectChange;

#ifdef __cplusplus

class DECLSPEC_UUID("d0565000-9df4-11d1-a281-00c04fca0aa7")
EventObjectChange;
#endif

EXTERN_C const CLSID CLSID_EventObjectChange2;

#ifdef __cplusplus

class DECLSPEC_UUID("BB07BACD-CD56-4e63-A8FF-CBF0355FB9F4")
EventObjectChange2;
#endif
#endif /* __DummyEventSystemLib_LIBRARY_DEFINED__ */

/* interface __MIDL_itf_eventsys_0000_0017 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_eventsys_0000_0017_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_eventsys_0000_0017_v0_0_s_ifspec;

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


