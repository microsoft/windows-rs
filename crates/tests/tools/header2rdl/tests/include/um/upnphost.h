

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

#ifndef __upnphost_h__
#define __upnphost_h__

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

#ifndef __IUPnPEventSink_FWD_DEFINED__
#define __IUPnPEventSink_FWD_DEFINED__
typedef interface IUPnPEventSink IUPnPEventSink;

#endif 	/* __IUPnPEventSink_FWD_DEFINED__ */


#ifndef __IUPnPEventSource_FWD_DEFINED__
#define __IUPnPEventSource_FWD_DEFINED__
typedef interface IUPnPEventSource IUPnPEventSource;

#endif 	/* __IUPnPEventSource_FWD_DEFINED__ */


#ifndef __IUPnPRegistrar_FWD_DEFINED__
#define __IUPnPRegistrar_FWD_DEFINED__
typedef interface IUPnPRegistrar IUPnPRegistrar;

#endif 	/* __IUPnPRegistrar_FWD_DEFINED__ */


#ifndef __IUPnPReregistrar_FWD_DEFINED__
#define __IUPnPReregistrar_FWD_DEFINED__
typedef interface IUPnPReregistrar IUPnPReregistrar;

#endif 	/* __IUPnPReregistrar_FWD_DEFINED__ */


#ifndef __IUPnPDeviceControl_FWD_DEFINED__
#define __IUPnPDeviceControl_FWD_DEFINED__
typedef interface IUPnPDeviceControl IUPnPDeviceControl;

#endif 	/* __IUPnPDeviceControl_FWD_DEFINED__ */


#ifndef __IUPnPDeviceControlHttpHeaders_FWD_DEFINED__
#define __IUPnPDeviceControlHttpHeaders_FWD_DEFINED__
typedef interface IUPnPDeviceControlHttpHeaders IUPnPDeviceControlHttpHeaders;

#endif 	/* __IUPnPDeviceControlHttpHeaders_FWD_DEFINED__ */


#ifndef __IUPnPDeviceProvider_FWD_DEFINED__
#define __IUPnPDeviceProvider_FWD_DEFINED__
typedef interface IUPnPDeviceProvider IUPnPDeviceProvider;

#endif 	/* __IUPnPDeviceProvider_FWD_DEFINED__ */


#ifndef __IUPnPRemoteEndpointInfo_FWD_DEFINED__
#define __IUPnPRemoteEndpointInfo_FWD_DEFINED__
typedef interface IUPnPRemoteEndpointInfo IUPnPRemoteEndpointInfo;

#endif 	/* __IUPnPRemoteEndpointInfo_FWD_DEFINED__ */


#ifndef __IUPnPEventSink_FWD_DEFINED__
#define __IUPnPEventSink_FWD_DEFINED__
typedef interface IUPnPEventSink IUPnPEventSink;

#endif 	/* __IUPnPEventSink_FWD_DEFINED__ */


#ifndef __IUPnPEventSource_FWD_DEFINED__
#define __IUPnPEventSource_FWD_DEFINED__
typedef interface IUPnPEventSource IUPnPEventSource;

#endif 	/* __IUPnPEventSource_FWD_DEFINED__ */


#ifndef __IUPnPRegistrar_FWD_DEFINED__
#define __IUPnPRegistrar_FWD_DEFINED__
typedef interface IUPnPRegistrar IUPnPRegistrar;

#endif 	/* __IUPnPRegistrar_FWD_DEFINED__ */


#ifndef __IUPnPReregistrar_FWD_DEFINED__
#define __IUPnPReregistrar_FWD_DEFINED__
typedef interface IUPnPReregistrar IUPnPReregistrar;

#endif 	/* __IUPnPReregistrar_FWD_DEFINED__ */


#ifndef __IUPnPDeviceControl_FWD_DEFINED__
#define __IUPnPDeviceControl_FWD_DEFINED__
typedef interface IUPnPDeviceControl IUPnPDeviceControl;

#endif 	/* __IUPnPDeviceControl_FWD_DEFINED__ */


#ifndef __IUPnPDeviceControlHttpHeaders_FWD_DEFINED__
#define __IUPnPDeviceControlHttpHeaders_FWD_DEFINED__
typedef interface IUPnPDeviceControlHttpHeaders IUPnPDeviceControlHttpHeaders;

#endif 	/* __IUPnPDeviceControlHttpHeaders_FWD_DEFINED__ */


#ifndef __IUPnPDeviceProvider_FWD_DEFINED__
#define __IUPnPDeviceProvider_FWD_DEFINED__
typedef interface IUPnPDeviceProvider IUPnPDeviceProvider;

#endif 	/* __IUPnPDeviceProvider_FWD_DEFINED__ */


#ifndef __IUPnPRemoteEndpointInfo_FWD_DEFINED__
#define __IUPnPRemoteEndpointInfo_FWD_DEFINED__
typedef interface IUPnPRemoteEndpointInfo IUPnPRemoteEndpointInfo;

#endif 	/* __IUPnPRemoteEndpointInfo_FWD_DEFINED__ */


#ifndef __UPnPRegistrar_FWD_DEFINED__
#define __UPnPRegistrar_FWD_DEFINED__

#ifdef __cplusplus
typedef class UPnPRegistrar UPnPRegistrar;
#else
typedef struct UPnPRegistrar UPnPRegistrar;
#endif /* __cplusplus */

#endif 	/* __UPnPRegistrar_FWD_DEFINED__ */


#ifndef __UPnPRemoteEndpointInfo_FWD_DEFINED__
#define __UPnPRemoteEndpointInfo_FWD_DEFINED__

#ifdef __cplusplus
typedef class UPnPRemoteEndpointInfo UPnPRemoteEndpointInfo;
#else
typedef struct UPnPRemoteEndpointInfo UPnPRemoteEndpointInfo;
#endif /* __cplusplus */

#endif 	/* __UPnPRemoteEndpointInfo_FWD_DEFINED__ */


/* header files for imported files */
#include "ocidl.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_upnphost_0000_0000 */
/* [local] */ 

//+-------------------------------------------------------------------------
//
//  Microsoft Windows
//  Copyright (c) Microsoft Corporation. All rights reserved.
//
//--------------------------------------------------------------------------
#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)








#define UPNP_E_REQUIRED_ELEMENT_ERROR        MAKE_HRESULT(SEVERITY_ERROR, FACILITY_ITF, 0xA020)
#define UPNP_E_DUPLICATE_NOT_ALLOWED         MAKE_HRESULT(SEVERITY_ERROR, FACILITY_ITF, 0xA021)
#define UPNP_E_DUPLICATE_SERVICE_ID          MAKE_HRESULT(SEVERITY_ERROR, FACILITY_ITF, 0xA022)
#define UPNP_E_INVALID_DESCRIPTION           MAKE_HRESULT(SEVERITY_ERROR, FACILITY_ITF, 0xA023)
#define UPNP_E_INVALID_SERVICE               MAKE_HRESULT(SEVERITY_ERROR, FACILITY_ITF, 0xA024)
#define UPNP_E_INVALID_ICON                  MAKE_HRESULT(SEVERITY_ERROR, FACILITY_ITF, 0xA025)
#define UPNP_E_INVALID_XML                   MAKE_HRESULT(SEVERITY_ERROR, FACILITY_ITF, 0xA026)
#define UPNP_E_INVALID_ROOT_NAMESPACE        MAKE_HRESULT(SEVERITY_ERROR, FACILITY_ITF, 0xA027)
#define UPNP_E_SUFFIX_TOO_LONG               MAKE_HRESULT(SEVERITY_ERROR, FACILITY_ITF, 0xA028)
#define UPNP_E_URLBASE_PRESENT               MAKE_HRESULT(SEVERITY_ERROR, FACILITY_ITF, 0xA029)
#define UPNP_E_VALUE_TOO_LONG                MAKE_HRESULT(SEVERITY_ERROR, FACILITY_ITF, 0xA030)
#define UPNP_E_DEVICE_RUNNING                MAKE_HRESULT(SEVERITY_ERROR, FACILITY_ITF, 0xA031)
#define UPNP_E_DEVICE_NOTREGISTERED          MAKE_HRESULT(SEVERITY_ERROR, FACILITY_ITF, 0xA032)
// Remote address value is a string
#define REMOTE_ADDRESS_VALUE_NAME L"RemoteAddress"
// Address family value is a DWORD
#define ADDRESS_FAMILY_VALUE_NAME L"AddressFamily"


extern RPC_IF_HANDLE __MIDL_itf_upnphost_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_upnphost_0000_0000_v0_0_s_ifspec;

#ifndef __IUPnPEventSink_INTERFACE_DEFINED__
#define __IUPnPEventSink_INTERFACE_DEFINED__

/* interface IUPnPEventSink */
/* [object][unique][uuid] */ 


EXTERN_C const IID IID_IUPnPEventSink;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("204810b4-73b2-11d4-bf42-00b0d0118b56")
    IUPnPEventSink : public IUnknown
    {
    public:
        virtual /* [hidden][helpstring] */ HRESULT STDMETHODCALLTYPE OnStateChanged( 
            /* [in] */ DWORD cChanges,
            /* [size_is][in] */ __RPC__in_ecount_full(cChanges) DISPID rgdispidChanges[  ]) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE OnStateChangedSafe( 
            /* [in] */ VARIANT varsadispidChanges) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IUPnPEventSinkVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IUPnPEventSink * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IUPnPEventSink * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IUPnPEventSink * This);
        
        DECLSPEC_XFGVIRT(IUPnPEventSink, OnStateChanged)
        /* [hidden][helpstring] */ HRESULT ( STDMETHODCALLTYPE *OnStateChanged )( 
            __RPC__in IUPnPEventSink * This,
            /* [in] */ DWORD cChanges,
            /* [size_is][in] */ __RPC__in_ecount_full(cChanges) DISPID rgdispidChanges[  ]);
        
        DECLSPEC_XFGVIRT(IUPnPEventSink, OnStateChangedSafe)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *OnStateChangedSafe )( 
            __RPC__in IUPnPEventSink * This,
            /* [in] */ VARIANT varsadispidChanges);
        
        END_INTERFACE
    } IUPnPEventSinkVtbl;

    interface IUPnPEventSink
    {
        CONST_VTBL struct IUPnPEventSinkVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IUPnPEventSink_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IUPnPEventSink_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IUPnPEventSink_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IUPnPEventSink_OnStateChanged(This,cChanges,rgdispidChanges)	\
    ( (This)->lpVtbl -> OnStateChanged(This,cChanges,rgdispidChanges) ) 

#define IUPnPEventSink_OnStateChangedSafe(This,varsadispidChanges)	\
    ( (This)->lpVtbl -> OnStateChangedSafe(This,varsadispidChanges) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IUPnPEventSink_INTERFACE_DEFINED__ */


#ifndef __IUPnPEventSource_INTERFACE_DEFINED__
#define __IUPnPEventSource_INTERFACE_DEFINED__

/* interface IUPnPEventSource */
/* [object][unique][uuid] */ 


EXTERN_C const IID IID_IUPnPEventSource;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("204810b5-73b2-11d4-bf42-00b0d0118b56")
    IUPnPEventSource : public IUnknown
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE Advise( 
            /* [in] */ __RPC__in_opt IUPnPEventSink *pesSubscriber) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE Unadvise( 
            /* [in] */ __RPC__in_opt IUPnPEventSink *pesSubscriber) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IUPnPEventSourceVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IUPnPEventSource * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IUPnPEventSource * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IUPnPEventSource * This);
        
        DECLSPEC_XFGVIRT(IUPnPEventSource, Advise)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *Advise )( 
            __RPC__in IUPnPEventSource * This,
            /* [in] */ __RPC__in_opt IUPnPEventSink *pesSubscriber);
        
        DECLSPEC_XFGVIRT(IUPnPEventSource, Unadvise)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *Unadvise )( 
            __RPC__in IUPnPEventSource * This,
            /* [in] */ __RPC__in_opt IUPnPEventSink *pesSubscriber);
        
        END_INTERFACE
    } IUPnPEventSourceVtbl;

    interface IUPnPEventSource
    {
        CONST_VTBL struct IUPnPEventSourceVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IUPnPEventSource_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IUPnPEventSource_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IUPnPEventSource_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IUPnPEventSource_Advise(This,pesSubscriber)	\
    ( (This)->lpVtbl -> Advise(This,pesSubscriber) ) 

#define IUPnPEventSource_Unadvise(This,pesSubscriber)	\
    ( (This)->lpVtbl -> Unadvise(This,pesSubscriber) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IUPnPEventSource_INTERFACE_DEFINED__ */


#ifndef __IUPnPRegistrar_INTERFACE_DEFINED__
#define __IUPnPRegistrar_INTERFACE_DEFINED__

/* interface IUPnPRegistrar */
/* [object][unique][uuid] */ 


EXTERN_C const IID IID_IUPnPRegistrar;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("204810b6-73b2-11d4-bf42-00b0d0118b56")
    IUPnPRegistrar : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE RegisterDevice( 
            /* [in] */ __RPC__in BSTR bstrXMLDesc,
            /* [in] */ __RPC__in BSTR bstrProgIDDeviceControlClass,
            /* [in] */ __RPC__in BSTR bstrInitString,
            /* [in] */ __RPC__in BSTR bstrContainerId,
            /* [in] */ __RPC__in BSTR bstrResourcePath,
            /* [in] */ long nLifeTime,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrDeviceIdentifier) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RegisterRunningDevice( 
            /* [in] */ __RPC__in BSTR bstrXMLDesc,
            /* [in] */ __RPC__in_opt IUnknown *punkDeviceControl,
            /* [in] */ __RPC__in BSTR bstrInitString,
            /* [in] */ __RPC__in BSTR bstrResourcePath,
            /* [in] */ long nLifeTime,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrDeviceIdentifier) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RegisterDeviceProvider( 
            /* [in] */ __RPC__in BSTR bstrProviderName,
            /* [in] */ __RPC__in BSTR bstrProgIDProviderClass,
            /* [in] */ __RPC__in BSTR bstrInitString,
            /* [in] */ __RPC__in BSTR bstrContainerId) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetUniqueDeviceName( 
            /* [in] */ __RPC__in BSTR bstrDeviceIdentifier,
            /* [in] */ __RPC__in BSTR bstrTemplateUDN,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrUDN) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE UnregisterDevice( 
            /* [in] */ __RPC__in BSTR bstrDeviceIdentifier,
            /* [in] */ BOOL fPermanent) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE UnregisterDeviceProvider( 
            /* [in] */ __RPC__in BSTR bstrProviderName) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IUPnPRegistrarVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IUPnPRegistrar * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IUPnPRegistrar * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IUPnPRegistrar * This);
        
        DECLSPEC_XFGVIRT(IUPnPRegistrar, RegisterDevice)
        HRESULT ( STDMETHODCALLTYPE *RegisterDevice )( 
            __RPC__in IUPnPRegistrar * This,
            /* [in] */ __RPC__in BSTR bstrXMLDesc,
            /* [in] */ __RPC__in BSTR bstrProgIDDeviceControlClass,
            /* [in] */ __RPC__in BSTR bstrInitString,
            /* [in] */ __RPC__in BSTR bstrContainerId,
            /* [in] */ __RPC__in BSTR bstrResourcePath,
            /* [in] */ long nLifeTime,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrDeviceIdentifier);
        
        DECLSPEC_XFGVIRT(IUPnPRegistrar, RegisterRunningDevice)
        HRESULT ( STDMETHODCALLTYPE *RegisterRunningDevice )( 
            __RPC__in IUPnPRegistrar * This,
            /* [in] */ __RPC__in BSTR bstrXMLDesc,
            /* [in] */ __RPC__in_opt IUnknown *punkDeviceControl,
            /* [in] */ __RPC__in BSTR bstrInitString,
            /* [in] */ __RPC__in BSTR bstrResourcePath,
            /* [in] */ long nLifeTime,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrDeviceIdentifier);
        
        DECLSPEC_XFGVIRT(IUPnPRegistrar, RegisterDeviceProvider)
        HRESULT ( STDMETHODCALLTYPE *RegisterDeviceProvider )( 
            __RPC__in IUPnPRegistrar * This,
            /* [in] */ __RPC__in BSTR bstrProviderName,
            /* [in] */ __RPC__in BSTR bstrProgIDProviderClass,
            /* [in] */ __RPC__in BSTR bstrInitString,
            /* [in] */ __RPC__in BSTR bstrContainerId);
        
        DECLSPEC_XFGVIRT(IUPnPRegistrar, GetUniqueDeviceName)
        HRESULT ( STDMETHODCALLTYPE *GetUniqueDeviceName )( 
            __RPC__in IUPnPRegistrar * This,
            /* [in] */ __RPC__in BSTR bstrDeviceIdentifier,
            /* [in] */ __RPC__in BSTR bstrTemplateUDN,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrUDN);
        
        DECLSPEC_XFGVIRT(IUPnPRegistrar, UnregisterDevice)
        HRESULT ( STDMETHODCALLTYPE *UnregisterDevice )( 
            __RPC__in IUPnPRegistrar * This,
            /* [in] */ __RPC__in BSTR bstrDeviceIdentifier,
            /* [in] */ BOOL fPermanent);
        
        DECLSPEC_XFGVIRT(IUPnPRegistrar, UnregisterDeviceProvider)
        HRESULT ( STDMETHODCALLTYPE *UnregisterDeviceProvider )( 
            __RPC__in IUPnPRegistrar * This,
            /* [in] */ __RPC__in BSTR bstrProviderName);
        
        END_INTERFACE
    } IUPnPRegistrarVtbl;

    interface IUPnPRegistrar
    {
        CONST_VTBL struct IUPnPRegistrarVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IUPnPRegistrar_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IUPnPRegistrar_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IUPnPRegistrar_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IUPnPRegistrar_RegisterDevice(This,bstrXMLDesc,bstrProgIDDeviceControlClass,bstrInitString,bstrContainerId,bstrResourcePath,nLifeTime,pbstrDeviceIdentifier)	\
    ( (This)->lpVtbl -> RegisterDevice(This,bstrXMLDesc,bstrProgIDDeviceControlClass,bstrInitString,bstrContainerId,bstrResourcePath,nLifeTime,pbstrDeviceIdentifier) ) 

#define IUPnPRegistrar_RegisterRunningDevice(This,bstrXMLDesc,punkDeviceControl,bstrInitString,bstrResourcePath,nLifeTime,pbstrDeviceIdentifier)	\
    ( (This)->lpVtbl -> RegisterRunningDevice(This,bstrXMLDesc,punkDeviceControl,bstrInitString,bstrResourcePath,nLifeTime,pbstrDeviceIdentifier) ) 

#define IUPnPRegistrar_RegisterDeviceProvider(This,bstrProviderName,bstrProgIDProviderClass,bstrInitString,bstrContainerId)	\
    ( (This)->lpVtbl -> RegisterDeviceProvider(This,bstrProviderName,bstrProgIDProviderClass,bstrInitString,bstrContainerId) ) 

#define IUPnPRegistrar_GetUniqueDeviceName(This,bstrDeviceIdentifier,bstrTemplateUDN,pbstrUDN)	\
    ( (This)->lpVtbl -> GetUniqueDeviceName(This,bstrDeviceIdentifier,bstrTemplateUDN,pbstrUDN) ) 

#define IUPnPRegistrar_UnregisterDevice(This,bstrDeviceIdentifier,fPermanent)	\
    ( (This)->lpVtbl -> UnregisterDevice(This,bstrDeviceIdentifier,fPermanent) ) 

#define IUPnPRegistrar_UnregisterDeviceProvider(This,bstrProviderName)	\
    ( (This)->lpVtbl -> UnregisterDeviceProvider(This,bstrProviderName) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IUPnPRegistrar_INTERFACE_DEFINED__ */


#ifndef __IUPnPReregistrar_INTERFACE_DEFINED__
#define __IUPnPReregistrar_INTERFACE_DEFINED__

/* interface IUPnPReregistrar */
/* [object][unique][uuid] */ 


EXTERN_C const IID IID_IUPnPReregistrar;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("204810b7-73b2-11d4-bf42-00b0d0118b56")
    IUPnPReregistrar : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE ReregisterDevice( 
            /* [in] */ __RPC__in BSTR bstrDeviceIdentifier,
            /* [in] */ __RPC__in BSTR bstrXMLDesc,
            /* [in] */ __RPC__in BSTR bstrProgIDDeviceControlClass,
            /* [in] */ __RPC__in BSTR bstrInitString,
            /* [in] */ __RPC__in BSTR bstrContainerId,
            /* [in] */ __RPC__in BSTR bstrResourcePath,
            /* [in] */ long nLifeTime) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ReregisterRunningDevice( 
            /* [in] */ __RPC__in BSTR bstrDeviceIdentifier,
            /* [in] */ __RPC__in BSTR bstrXMLDesc,
            /* [in] */ __RPC__in_opt IUnknown *punkDeviceControl,
            /* [in] */ __RPC__in BSTR bstrInitString,
            /* [in] */ __RPC__in BSTR bstrResourcePath,
            /* [in] */ long nLifeTime) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IUPnPReregistrarVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IUPnPReregistrar * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IUPnPReregistrar * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IUPnPReregistrar * This);
        
        DECLSPEC_XFGVIRT(IUPnPReregistrar, ReregisterDevice)
        HRESULT ( STDMETHODCALLTYPE *ReregisterDevice )( 
            __RPC__in IUPnPReregistrar * This,
            /* [in] */ __RPC__in BSTR bstrDeviceIdentifier,
            /* [in] */ __RPC__in BSTR bstrXMLDesc,
            /* [in] */ __RPC__in BSTR bstrProgIDDeviceControlClass,
            /* [in] */ __RPC__in BSTR bstrInitString,
            /* [in] */ __RPC__in BSTR bstrContainerId,
            /* [in] */ __RPC__in BSTR bstrResourcePath,
            /* [in] */ long nLifeTime);
        
        DECLSPEC_XFGVIRT(IUPnPReregistrar, ReregisterRunningDevice)
        HRESULT ( STDMETHODCALLTYPE *ReregisterRunningDevice )( 
            __RPC__in IUPnPReregistrar * This,
            /* [in] */ __RPC__in BSTR bstrDeviceIdentifier,
            /* [in] */ __RPC__in BSTR bstrXMLDesc,
            /* [in] */ __RPC__in_opt IUnknown *punkDeviceControl,
            /* [in] */ __RPC__in BSTR bstrInitString,
            /* [in] */ __RPC__in BSTR bstrResourcePath,
            /* [in] */ long nLifeTime);
        
        END_INTERFACE
    } IUPnPReregistrarVtbl;

    interface IUPnPReregistrar
    {
        CONST_VTBL struct IUPnPReregistrarVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IUPnPReregistrar_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IUPnPReregistrar_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IUPnPReregistrar_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IUPnPReregistrar_ReregisterDevice(This,bstrDeviceIdentifier,bstrXMLDesc,bstrProgIDDeviceControlClass,bstrInitString,bstrContainerId,bstrResourcePath,nLifeTime)	\
    ( (This)->lpVtbl -> ReregisterDevice(This,bstrDeviceIdentifier,bstrXMLDesc,bstrProgIDDeviceControlClass,bstrInitString,bstrContainerId,bstrResourcePath,nLifeTime) ) 

#define IUPnPReregistrar_ReregisterRunningDevice(This,bstrDeviceIdentifier,bstrXMLDesc,punkDeviceControl,bstrInitString,bstrResourcePath,nLifeTime)	\
    ( (This)->lpVtbl -> ReregisterRunningDevice(This,bstrDeviceIdentifier,bstrXMLDesc,punkDeviceControl,bstrInitString,bstrResourcePath,nLifeTime) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IUPnPReregistrar_INTERFACE_DEFINED__ */


#ifndef __IUPnPDeviceControl_INTERFACE_DEFINED__
#define __IUPnPDeviceControl_INTERFACE_DEFINED__

/* interface IUPnPDeviceControl */
/* [object][unique][uuid] */ 


EXTERN_C const IID IID_IUPnPDeviceControl;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("204810ba-73b2-11d4-bf42-00b0d0118b56")
    IUPnPDeviceControl : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Initialize( 
            /* [in] */ __RPC__in BSTR bstrXMLDesc,
            /* [in] */ __RPC__in BSTR bstrDeviceIdentifier,
            /* [in] */ __RPC__in BSTR bstrInitString) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetServiceObject( 
            /* [in] */ __RPC__in BSTR bstrUDN,
            /* [in] */ __RPC__in BSTR bstrServiceId,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppdispService) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IUPnPDeviceControlVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IUPnPDeviceControl * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IUPnPDeviceControl * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IUPnPDeviceControl * This);
        
        DECLSPEC_XFGVIRT(IUPnPDeviceControl, Initialize)
        HRESULT ( STDMETHODCALLTYPE *Initialize )( 
            __RPC__in IUPnPDeviceControl * This,
            /* [in] */ __RPC__in BSTR bstrXMLDesc,
            /* [in] */ __RPC__in BSTR bstrDeviceIdentifier,
            /* [in] */ __RPC__in BSTR bstrInitString);
        
        DECLSPEC_XFGVIRT(IUPnPDeviceControl, GetServiceObject)
        HRESULT ( STDMETHODCALLTYPE *GetServiceObject )( 
            __RPC__in IUPnPDeviceControl * This,
            /* [in] */ __RPC__in BSTR bstrUDN,
            /* [in] */ __RPC__in BSTR bstrServiceId,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppdispService);
        
        END_INTERFACE
    } IUPnPDeviceControlVtbl;

    interface IUPnPDeviceControl
    {
        CONST_VTBL struct IUPnPDeviceControlVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IUPnPDeviceControl_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IUPnPDeviceControl_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IUPnPDeviceControl_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IUPnPDeviceControl_Initialize(This,bstrXMLDesc,bstrDeviceIdentifier,bstrInitString)	\
    ( (This)->lpVtbl -> Initialize(This,bstrXMLDesc,bstrDeviceIdentifier,bstrInitString) ) 

#define IUPnPDeviceControl_GetServiceObject(This,bstrUDN,bstrServiceId,ppdispService)	\
    ( (This)->lpVtbl -> GetServiceObject(This,bstrUDN,bstrServiceId,ppdispService) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IUPnPDeviceControl_INTERFACE_DEFINED__ */


#ifndef __IUPnPDeviceControlHttpHeaders_INTERFACE_DEFINED__
#define __IUPnPDeviceControlHttpHeaders_INTERFACE_DEFINED__

/* interface IUPnPDeviceControlHttpHeaders */
/* [object][unique][uuid] */ 


EXTERN_C const IID IID_IUPnPDeviceControlHttpHeaders;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("204810bb-73b2-11d4-bf42-00b0d0118b56")
    IUPnPDeviceControlHttpHeaders : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetAdditionalResponseHeaders( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *bstrHttpResponseHeaders) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IUPnPDeviceControlHttpHeadersVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IUPnPDeviceControlHttpHeaders * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IUPnPDeviceControlHttpHeaders * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IUPnPDeviceControlHttpHeaders * This);
        
        DECLSPEC_XFGVIRT(IUPnPDeviceControlHttpHeaders, GetAdditionalResponseHeaders)
        HRESULT ( STDMETHODCALLTYPE *GetAdditionalResponseHeaders )( 
            __RPC__in IUPnPDeviceControlHttpHeaders * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *bstrHttpResponseHeaders);
        
        END_INTERFACE
    } IUPnPDeviceControlHttpHeadersVtbl;

    interface IUPnPDeviceControlHttpHeaders
    {
        CONST_VTBL struct IUPnPDeviceControlHttpHeadersVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IUPnPDeviceControlHttpHeaders_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IUPnPDeviceControlHttpHeaders_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IUPnPDeviceControlHttpHeaders_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IUPnPDeviceControlHttpHeaders_GetAdditionalResponseHeaders(This,bstrHttpResponseHeaders)	\
    ( (This)->lpVtbl -> GetAdditionalResponseHeaders(This,bstrHttpResponseHeaders) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IUPnPDeviceControlHttpHeaders_INTERFACE_DEFINED__ */


#ifndef __IUPnPDeviceProvider_INTERFACE_DEFINED__
#define __IUPnPDeviceProvider_INTERFACE_DEFINED__

/* interface IUPnPDeviceProvider */
/* [object][unique][uuid] */ 


EXTERN_C const IID IID_IUPnPDeviceProvider;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("204810b8-73b2-11d4-bf42-00b0d0118b56")
    IUPnPDeviceProvider : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Start( 
            /* [in] */ __RPC__in BSTR bstrInitString) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Stop( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IUPnPDeviceProviderVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IUPnPDeviceProvider * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IUPnPDeviceProvider * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IUPnPDeviceProvider * This);
        
        DECLSPEC_XFGVIRT(IUPnPDeviceProvider, Start)
        HRESULT ( STDMETHODCALLTYPE *Start )( 
            __RPC__in IUPnPDeviceProvider * This,
            /* [in] */ __RPC__in BSTR bstrInitString);
        
        DECLSPEC_XFGVIRT(IUPnPDeviceProvider, Stop)
        HRESULT ( STDMETHODCALLTYPE *Stop )( 
            __RPC__in IUPnPDeviceProvider * This);
        
        END_INTERFACE
    } IUPnPDeviceProviderVtbl;

    interface IUPnPDeviceProvider
    {
        CONST_VTBL struct IUPnPDeviceProviderVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IUPnPDeviceProvider_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IUPnPDeviceProvider_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IUPnPDeviceProvider_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IUPnPDeviceProvider_Start(This,bstrInitString)	\
    ( (This)->lpVtbl -> Start(This,bstrInitString) ) 

#define IUPnPDeviceProvider_Stop(This)	\
    ( (This)->lpVtbl -> Stop(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IUPnPDeviceProvider_INTERFACE_DEFINED__ */


#ifndef __IUPnPRemoteEndpointInfo_INTERFACE_DEFINED__
#define __IUPnPRemoteEndpointInfo_INTERFACE_DEFINED__

/* interface IUPnPRemoteEndpointInfo */
/* [object][unique][uuid] */ 


EXTERN_C const IID IID_IUPnPRemoteEndpointInfo;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("c92eb863-0269-4aff-9c72-75321bba2952")
    IUPnPRemoteEndpointInfo : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetDwordValue( 
            /* [in] */ __RPC__in BSTR bstrValueName,
            /* [out] */ __RPC__out DWORD *pdwValue) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetStringValue( 
            /* [in] */ __RPC__in BSTR bstrValueName,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrValue) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetGuidValue( 
            /* [in] */ __RPC__in BSTR bstrValueName,
            /* [out] */ __RPC__out GUID *pguidValue) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IUPnPRemoteEndpointInfoVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IUPnPRemoteEndpointInfo * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IUPnPRemoteEndpointInfo * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IUPnPRemoteEndpointInfo * This);
        
        DECLSPEC_XFGVIRT(IUPnPRemoteEndpointInfo, GetDwordValue)
        HRESULT ( STDMETHODCALLTYPE *GetDwordValue )( 
            __RPC__in IUPnPRemoteEndpointInfo * This,
            /* [in] */ __RPC__in BSTR bstrValueName,
            /* [out] */ __RPC__out DWORD *pdwValue);
        
        DECLSPEC_XFGVIRT(IUPnPRemoteEndpointInfo, GetStringValue)
        HRESULT ( STDMETHODCALLTYPE *GetStringValue )( 
            __RPC__in IUPnPRemoteEndpointInfo * This,
            /* [in] */ __RPC__in BSTR bstrValueName,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrValue);
        
        DECLSPEC_XFGVIRT(IUPnPRemoteEndpointInfo, GetGuidValue)
        HRESULT ( STDMETHODCALLTYPE *GetGuidValue )( 
            __RPC__in IUPnPRemoteEndpointInfo * This,
            /* [in] */ __RPC__in BSTR bstrValueName,
            /* [out] */ __RPC__out GUID *pguidValue);
        
        END_INTERFACE
    } IUPnPRemoteEndpointInfoVtbl;

    interface IUPnPRemoteEndpointInfo
    {
        CONST_VTBL struct IUPnPRemoteEndpointInfoVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IUPnPRemoteEndpointInfo_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IUPnPRemoteEndpointInfo_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IUPnPRemoteEndpointInfo_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IUPnPRemoteEndpointInfo_GetDwordValue(This,bstrValueName,pdwValue)	\
    ( (This)->lpVtbl -> GetDwordValue(This,bstrValueName,pdwValue) ) 

#define IUPnPRemoteEndpointInfo_GetStringValue(This,bstrValueName,pbstrValue)	\
    ( (This)->lpVtbl -> GetStringValue(This,bstrValueName,pbstrValue) ) 

#define IUPnPRemoteEndpointInfo_GetGuidValue(This,bstrValueName,pguidValue)	\
    ( (This)->lpVtbl -> GetGuidValue(This,bstrValueName,pguidValue) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IUPnPRemoteEndpointInfo_INTERFACE_DEFINED__ */



#ifndef __UPnPHostLib_LIBRARY_DEFINED__
#define __UPnPHostLib_LIBRARY_DEFINED__

/* library UPnPHostLib */
/* [helpstring][version][uuid] */ 










EXTERN_C const IID LIBID_UPnPHostLib;

EXTERN_C const CLSID CLSID_UPnPRegistrar;

#ifdef __cplusplus

class DECLSPEC_UUID("204810b9-73b2-11d4-bf42-00b0d0118b56")
UPnPRegistrar;
#endif

EXTERN_C const CLSID CLSID_UPnPRemoteEndpointInfo;

#ifdef __cplusplus

class DECLSPEC_UUID("2e5e84e9-4049-4244-b728-2d24227157c7")
UPnPRemoteEndpointInfo;
#endif
#endif /* __UPnPHostLib_LIBRARY_DEFINED__ */

/* interface __MIDL_itf_upnphost_0000_0009 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_upnphost_0000_0009_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_upnphost_0000_0009_v0_0_s_ifspec;

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


