

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

#ifndef __adhoc_h__
#define __adhoc_h__

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

#ifndef __IDot11AdHocManager_FWD_DEFINED__
#define __IDot11AdHocManager_FWD_DEFINED__
typedef interface IDot11AdHocManager IDot11AdHocManager;

#endif 	/* __IDot11AdHocManager_FWD_DEFINED__ */


#ifndef __IDot11AdHocManagerNotificationSink_FWD_DEFINED__
#define __IDot11AdHocManagerNotificationSink_FWD_DEFINED__
typedef interface IDot11AdHocManagerNotificationSink IDot11AdHocManagerNotificationSink;

#endif 	/* __IDot11AdHocManagerNotificationSink_FWD_DEFINED__ */


#ifndef __IEnumDot11AdHocNetworks_FWD_DEFINED__
#define __IEnumDot11AdHocNetworks_FWD_DEFINED__
typedef interface IEnumDot11AdHocNetworks IEnumDot11AdHocNetworks;

#endif 	/* __IEnumDot11AdHocNetworks_FWD_DEFINED__ */


#ifndef __IDot11AdHocNetwork_FWD_DEFINED__
#define __IDot11AdHocNetwork_FWD_DEFINED__
typedef interface IDot11AdHocNetwork IDot11AdHocNetwork;

#endif 	/* __IDot11AdHocNetwork_FWD_DEFINED__ */


#ifndef __IDot11AdHocNetworkNotificationSink_FWD_DEFINED__
#define __IDot11AdHocNetworkNotificationSink_FWD_DEFINED__
typedef interface IDot11AdHocNetworkNotificationSink IDot11AdHocNetworkNotificationSink;

#endif 	/* __IDot11AdHocNetworkNotificationSink_FWD_DEFINED__ */


#ifndef __IDot11AdHocInterface_FWD_DEFINED__
#define __IDot11AdHocInterface_FWD_DEFINED__
typedef interface IDot11AdHocInterface IDot11AdHocInterface;

#endif 	/* __IDot11AdHocInterface_FWD_DEFINED__ */


#ifndef __IEnumDot11AdHocInterfaces_FWD_DEFINED__
#define __IEnumDot11AdHocInterfaces_FWD_DEFINED__
typedef interface IEnumDot11AdHocInterfaces IEnumDot11AdHocInterfaces;

#endif 	/* __IEnumDot11AdHocInterfaces_FWD_DEFINED__ */


#ifndef __IEnumDot11AdHocSecuritySettings_FWD_DEFINED__
#define __IEnumDot11AdHocSecuritySettings_FWD_DEFINED__
typedef interface IEnumDot11AdHocSecuritySettings IEnumDot11AdHocSecuritySettings;

#endif 	/* __IEnumDot11AdHocSecuritySettings_FWD_DEFINED__ */


#ifndef __IDot11AdHocSecuritySettings_FWD_DEFINED__
#define __IDot11AdHocSecuritySettings_FWD_DEFINED__
typedef interface IDot11AdHocSecuritySettings IDot11AdHocSecuritySettings;

#endif 	/* __IDot11AdHocSecuritySettings_FWD_DEFINED__ */


#ifndef __IDot11AdHocInterfaceNotificationSink_FWD_DEFINED__
#define __IDot11AdHocInterfaceNotificationSink_FWD_DEFINED__
typedef interface IDot11AdHocInterfaceNotificationSink IDot11AdHocInterfaceNotificationSink;

#endif 	/* __IDot11AdHocInterfaceNotificationSink_FWD_DEFINED__ */


#ifndef __Dot11AdHocManager_FWD_DEFINED__
#define __Dot11AdHocManager_FWD_DEFINED__

#ifdef __cplusplus
typedef class Dot11AdHocManager Dot11AdHocManager;
#else
typedef struct Dot11AdHocManager Dot11AdHocManager;
#endif /* __cplusplus */

#endif 	/* __Dot11AdHocManager_FWD_DEFINED__ */


/* header files for imported files */
#include "oaidl.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_adhoc_0000_0000 */
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
EXTERN_C const CLSID CLSID_AdHocManager;










typedef /* [v1_enum] */ 
enum tagDOT11_ADHOC_CIPHER_ALGORITHM
    {
        DOT11_ADHOC_CIPHER_ALGO_INVALID	= -1,
        DOT11_ADHOC_CIPHER_ALGO_NONE	= 0,
        DOT11_ADHOC_CIPHER_ALGO_CCMP	= 0x4,
        DOT11_ADHOC_CIPHER_ALGO_WEP	= 0x101
    } 	DOT11_ADHOC_CIPHER_ALGORITHM;

typedef /* [v1_enum] */ 
enum tagDOT11_ADHOC_AUTH_ALGORITHM
    {
        DOT11_ADHOC_AUTH_ALGO_INVALID	= -1,
        DOT11_ADHOC_AUTH_ALGO_80211_OPEN	= 1,
        DOT11_ADHOC_AUTH_ALGO_RSNA_PSK	= 7
    } 	DOT11_ADHOC_AUTH_ALGORITHM;

typedef /* [v1_enum] */ 
enum tagDOT11_ADHOC_NETWORK_CONNECTION_STATUS
    {
        DOT11_ADHOC_NETWORK_CONNECTION_STATUS_INVALID	= 0,
        DOT11_ADHOC_NETWORK_CONNECTION_STATUS_DISCONNECTED	= 11,
        DOT11_ADHOC_NETWORK_CONNECTION_STATUS_CONNECTING	= 12,
        DOT11_ADHOC_NETWORK_CONNECTION_STATUS_CONNECTED	= 13,
        DOT11_ADHOC_NETWORK_CONNECTION_STATUS_FORMED	= 14
    } 	DOT11_ADHOC_NETWORK_CONNECTION_STATUS;

typedef /* [v1_enum] */ 
enum tagDOT11_ADHOC_CONNECT_FAIL_REASON
    {
        DOT11_ADHOC_CONNECT_FAIL_DOMAIN_MISMATCH	= 0,
        DOT11_ADHOC_CONNECT_FAIL_PASSPHRASE_MISMATCH	= 1,
        DOT11_ADHOC_CONNECT_FAIL_OTHER	= 2
    } 	DOT11_ADHOC_CONNECT_FAIL_REASON;



extern RPC_IF_HANDLE __MIDL_itf_adhoc_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_adhoc_0000_0000_v0_0_s_ifspec;

#ifndef __IDot11AdHocManager_INTERFACE_DEFINED__
#define __IDot11AdHocManager_INTERFACE_DEFINED__

/* interface IDot11AdHocManager */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_IDot11AdHocManager;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("8F10CC26-CF0D-42a0-ACBE-E2DE7007384D")
    IDot11AdHocManager : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE CreateNetwork( 
            /* [string][in] */ LPCWSTR Name,
            /* [string][in] */ LPCWSTR Password,
            /* [in] */ LONG GeographicalId,
            /* [in] */ IDot11AdHocInterface *pInterface,
            /* [in] */ IDot11AdHocSecuritySettings *pSecurity,
            /* [in] */ GUID *pContextGuid,
            /* [out] */ IDot11AdHocNetwork **pIAdHoc) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CommitCreatedNetwork( 
            /* [in] */ IDot11AdHocNetwork *pIAdHoc,
            /* [in] */ BOOLEAN fSaveProfile,
            /* [in] */ BOOLEAN fMakeSavedProfileUserSpecific) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetIEnumDot11AdHocNetworks( 
            /* [in] */ GUID *pContextGuid,
            /* [out] */ IEnumDot11AdHocNetworks **ppEnum) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetIEnumDot11AdHocInterfaces( 
            /* [out] */ IEnumDot11AdHocInterfaces **ppEnum) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetNetwork( 
            /* [in] */ GUID *NetworkSignature,
            /* [out] */ IDot11AdHocNetwork **pNetwork) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDot11AdHocManagerVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IDot11AdHocManager * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IDot11AdHocManager * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IDot11AdHocManager * This);
        
        DECLSPEC_XFGVIRT(IDot11AdHocManager, CreateNetwork)
        HRESULT ( STDMETHODCALLTYPE *CreateNetwork )( 
            IDot11AdHocManager * This,
            /* [string][in] */ LPCWSTR Name,
            /* [string][in] */ LPCWSTR Password,
            /* [in] */ LONG GeographicalId,
            /* [in] */ IDot11AdHocInterface *pInterface,
            /* [in] */ IDot11AdHocSecuritySettings *pSecurity,
            /* [in] */ GUID *pContextGuid,
            /* [out] */ IDot11AdHocNetwork **pIAdHoc);
        
        DECLSPEC_XFGVIRT(IDot11AdHocManager, CommitCreatedNetwork)
        HRESULT ( STDMETHODCALLTYPE *CommitCreatedNetwork )( 
            IDot11AdHocManager * This,
            /* [in] */ IDot11AdHocNetwork *pIAdHoc,
            /* [in] */ BOOLEAN fSaveProfile,
            /* [in] */ BOOLEAN fMakeSavedProfileUserSpecific);
        
        DECLSPEC_XFGVIRT(IDot11AdHocManager, GetIEnumDot11AdHocNetworks)
        HRESULT ( STDMETHODCALLTYPE *GetIEnumDot11AdHocNetworks )( 
            IDot11AdHocManager * This,
            /* [in] */ GUID *pContextGuid,
            /* [out] */ IEnumDot11AdHocNetworks **ppEnum);
        
        DECLSPEC_XFGVIRT(IDot11AdHocManager, GetIEnumDot11AdHocInterfaces)
        HRESULT ( STDMETHODCALLTYPE *GetIEnumDot11AdHocInterfaces )( 
            IDot11AdHocManager * This,
            /* [out] */ IEnumDot11AdHocInterfaces **ppEnum);
        
        DECLSPEC_XFGVIRT(IDot11AdHocManager, GetNetwork)
        HRESULT ( STDMETHODCALLTYPE *GetNetwork )( 
            IDot11AdHocManager * This,
            /* [in] */ GUID *NetworkSignature,
            /* [out] */ IDot11AdHocNetwork **pNetwork);
        
        END_INTERFACE
    } IDot11AdHocManagerVtbl;

    interface IDot11AdHocManager
    {
        CONST_VTBL struct IDot11AdHocManagerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDot11AdHocManager_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDot11AdHocManager_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDot11AdHocManager_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDot11AdHocManager_CreateNetwork(This,Name,Password,GeographicalId,pInterface,pSecurity,pContextGuid,pIAdHoc)	\
    ( (This)->lpVtbl -> CreateNetwork(This,Name,Password,GeographicalId,pInterface,pSecurity,pContextGuid,pIAdHoc) ) 

#define IDot11AdHocManager_CommitCreatedNetwork(This,pIAdHoc,fSaveProfile,fMakeSavedProfileUserSpecific)	\
    ( (This)->lpVtbl -> CommitCreatedNetwork(This,pIAdHoc,fSaveProfile,fMakeSavedProfileUserSpecific) ) 

#define IDot11AdHocManager_GetIEnumDot11AdHocNetworks(This,pContextGuid,ppEnum)	\
    ( (This)->lpVtbl -> GetIEnumDot11AdHocNetworks(This,pContextGuid,ppEnum) ) 

#define IDot11AdHocManager_GetIEnumDot11AdHocInterfaces(This,ppEnum)	\
    ( (This)->lpVtbl -> GetIEnumDot11AdHocInterfaces(This,ppEnum) ) 

#define IDot11AdHocManager_GetNetwork(This,NetworkSignature,pNetwork)	\
    ( (This)->lpVtbl -> GetNetwork(This,NetworkSignature,pNetwork) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDot11AdHocManager_INTERFACE_DEFINED__ */


#ifndef __IDot11AdHocManagerNotificationSink_INTERFACE_DEFINED__
#define __IDot11AdHocManagerNotificationSink_INTERFACE_DEFINED__

/* interface IDot11AdHocManagerNotificationSink */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_IDot11AdHocManagerNotificationSink;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("8F10CC27-CF0D-42a0-ACBE-E2DE7007384D")
    IDot11AdHocManagerNotificationSink : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE OnNetworkAdd( 
            /* [in] */ IDot11AdHocNetwork *pIAdHocNetwork) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnNetworkRemove( 
            /* [in] */ GUID *Signature) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnInterfaceAdd( 
            /* [in] */ IDot11AdHocInterface *pIAdHocInterface) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnInterfaceRemove( 
            /* [in] */ GUID *Signature) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDot11AdHocManagerNotificationSinkVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IDot11AdHocManagerNotificationSink * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IDot11AdHocManagerNotificationSink * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IDot11AdHocManagerNotificationSink * This);
        
        DECLSPEC_XFGVIRT(IDot11AdHocManagerNotificationSink, OnNetworkAdd)
        HRESULT ( STDMETHODCALLTYPE *OnNetworkAdd )( 
            IDot11AdHocManagerNotificationSink * This,
            /* [in] */ IDot11AdHocNetwork *pIAdHocNetwork);
        
        DECLSPEC_XFGVIRT(IDot11AdHocManagerNotificationSink, OnNetworkRemove)
        HRESULT ( STDMETHODCALLTYPE *OnNetworkRemove )( 
            IDot11AdHocManagerNotificationSink * This,
            /* [in] */ GUID *Signature);
        
        DECLSPEC_XFGVIRT(IDot11AdHocManagerNotificationSink, OnInterfaceAdd)
        HRESULT ( STDMETHODCALLTYPE *OnInterfaceAdd )( 
            IDot11AdHocManagerNotificationSink * This,
            /* [in] */ IDot11AdHocInterface *pIAdHocInterface);
        
        DECLSPEC_XFGVIRT(IDot11AdHocManagerNotificationSink, OnInterfaceRemove)
        HRESULT ( STDMETHODCALLTYPE *OnInterfaceRemove )( 
            IDot11AdHocManagerNotificationSink * This,
            /* [in] */ GUID *Signature);
        
        END_INTERFACE
    } IDot11AdHocManagerNotificationSinkVtbl;

    interface IDot11AdHocManagerNotificationSink
    {
        CONST_VTBL struct IDot11AdHocManagerNotificationSinkVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDot11AdHocManagerNotificationSink_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDot11AdHocManagerNotificationSink_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDot11AdHocManagerNotificationSink_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDot11AdHocManagerNotificationSink_OnNetworkAdd(This,pIAdHocNetwork)	\
    ( (This)->lpVtbl -> OnNetworkAdd(This,pIAdHocNetwork) ) 

#define IDot11AdHocManagerNotificationSink_OnNetworkRemove(This,Signature)	\
    ( (This)->lpVtbl -> OnNetworkRemove(This,Signature) ) 

#define IDot11AdHocManagerNotificationSink_OnInterfaceAdd(This,pIAdHocInterface)	\
    ( (This)->lpVtbl -> OnInterfaceAdd(This,pIAdHocInterface) ) 

#define IDot11AdHocManagerNotificationSink_OnInterfaceRemove(This,Signature)	\
    ( (This)->lpVtbl -> OnInterfaceRemove(This,Signature) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDot11AdHocManagerNotificationSink_INTERFACE_DEFINED__ */


#ifndef __IEnumDot11AdHocNetworks_INTERFACE_DEFINED__
#define __IEnumDot11AdHocNetworks_INTERFACE_DEFINED__

/* interface IEnumDot11AdHocNetworks */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_IEnumDot11AdHocNetworks;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("8F10CC28-CF0D-42a0-ACBE-E2DE7007384D")
    IEnumDot11AdHocNetworks : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Next( 
            /* [in] */ ULONG cElt,
            /* [length_is][size_is][out] */ IDot11AdHocNetwork **rgElt,
            /* [out] */ ULONG *pcEltFetched) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Skip( 
            /* [in] */ ULONG cElt) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Reset( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Clone( 
            /* [out] */ IEnumDot11AdHocNetworks **ppEnum) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IEnumDot11AdHocNetworksVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IEnumDot11AdHocNetworks * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IEnumDot11AdHocNetworks * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IEnumDot11AdHocNetworks * This);
        
        DECLSPEC_XFGVIRT(IEnumDot11AdHocNetworks, Next)
        HRESULT ( STDMETHODCALLTYPE *Next )( 
            IEnumDot11AdHocNetworks * This,
            /* [in] */ ULONG cElt,
            /* [length_is][size_is][out] */ IDot11AdHocNetwork **rgElt,
            /* [out] */ ULONG *pcEltFetched);
        
        DECLSPEC_XFGVIRT(IEnumDot11AdHocNetworks, Skip)
        HRESULT ( STDMETHODCALLTYPE *Skip )( 
            IEnumDot11AdHocNetworks * This,
            /* [in] */ ULONG cElt);
        
        DECLSPEC_XFGVIRT(IEnumDot11AdHocNetworks, Reset)
        HRESULT ( STDMETHODCALLTYPE *Reset )( 
            IEnumDot11AdHocNetworks * This);
        
        DECLSPEC_XFGVIRT(IEnumDot11AdHocNetworks, Clone)
        HRESULT ( STDMETHODCALLTYPE *Clone )( 
            IEnumDot11AdHocNetworks * This,
            /* [out] */ IEnumDot11AdHocNetworks **ppEnum);
        
        END_INTERFACE
    } IEnumDot11AdHocNetworksVtbl;

    interface IEnumDot11AdHocNetworks
    {
        CONST_VTBL struct IEnumDot11AdHocNetworksVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IEnumDot11AdHocNetworks_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IEnumDot11AdHocNetworks_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IEnumDot11AdHocNetworks_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IEnumDot11AdHocNetworks_Next(This,cElt,rgElt,pcEltFetched)	\
    ( (This)->lpVtbl -> Next(This,cElt,rgElt,pcEltFetched) ) 

#define IEnumDot11AdHocNetworks_Skip(This,cElt)	\
    ( (This)->lpVtbl -> Skip(This,cElt) ) 

#define IEnumDot11AdHocNetworks_Reset(This)	\
    ( (This)->lpVtbl -> Reset(This) ) 

#define IEnumDot11AdHocNetworks_Clone(This,ppEnum)	\
    ( (This)->lpVtbl -> Clone(This,ppEnum) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IEnumDot11AdHocNetworks_INTERFACE_DEFINED__ */


#ifndef __IDot11AdHocNetwork_INTERFACE_DEFINED__
#define __IDot11AdHocNetwork_INTERFACE_DEFINED__

/* interface IDot11AdHocNetwork */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_IDot11AdHocNetwork;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("8F10CC29-CF0D-42a0-ACBE-E2DE7007384D")
    IDot11AdHocNetwork : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetStatus( 
            /* [out][in] */ DOT11_ADHOC_NETWORK_CONNECTION_STATUS *eStatus) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetSSID( 
            /* [string][out] */ LPWSTR *ppszwSSID) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE HasProfile( 
            /* [out][in] */ BOOLEAN *pf11d) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetProfileName( 
            /* [string][out] */ LPWSTR *ppszwProfileName) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE DeleteProfile( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetSignalQuality( 
            /* [out] */ ULONG *puStrengthValue,
            /* [out] */ ULONG *puStrengthMax) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetSecuritySetting( 
            /* [out] */ IDot11AdHocSecuritySettings **pAdHocSecuritySetting) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetContextGuid( 
            /* [out][in] */ GUID *pContextGuid) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetSignature( 
            /* [out][in] */ GUID *pSignature) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetInterface( 
            /* [out] */ IDot11AdHocInterface **pAdHocInterface) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Connect( 
            /* [string][in] */ LPCWSTR Passphrase,
            /* [in] */ LONG GeographicalId,
            /* [in] */ BOOLEAN fSaveProfile,
            /* [in] */ BOOLEAN fMakeSavedProfileUserSpecific) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Disconnect( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDot11AdHocNetworkVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IDot11AdHocNetwork * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IDot11AdHocNetwork * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IDot11AdHocNetwork * This);
        
        DECLSPEC_XFGVIRT(IDot11AdHocNetwork, GetStatus)
        HRESULT ( STDMETHODCALLTYPE *GetStatus )( 
            IDot11AdHocNetwork * This,
            /* [out][in] */ DOT11_ADHOC_NETWORK_CONNECTION_STATUS *eStatus);
        
        DECLSPEC_XFGVIRT(IDot11AdHocNetwork, GetSSID)
        HRESULT ( STDMETHODCALLTYPE *GetSSID )( 
            IDot11AdHocNetwork * This,
            /* [string][out] */ LPWSTR *ppszwSSID);
        
        DECLSPEC_XFGVIRT(IDot11AdHocNetwork, HasProfile)
        HRESULT ( STDMETHODCALLTYPE *HasProfile )( 
            IDot11AdHocNetwork * This,
            /* [out][in] */ BOOLEAN *pf11d);
        
        DECLSPEC_XFGVIRT(IDot11AdHocNetwork, GetProfileName)
        HRESULT ( STDMETHODCALLTYPE *GetProfileName )( 
            IDot11AdHocNetwork * This,
            /* [string][out] */ LPWSTR *ppszwProfileName);
        
        DECLSPEC_XFGVIRT(IDot11AdHocNetwork, DeleteProfile)
        HRESULT ( STDMETHODCALLTYPE *DeleteProfile )( 
            IDot11AdHocNetwork * This);
        
        DECLSPEC_XFGVIRT(IDot11AdHocNetwork, GetSignalQuality)
        HRESULT ( STDMETHODCALLTYPE *GetSignalQuality )( 
            IDot11AdHocNetwork * This,
            /* [out] */ ULONG *puStrengthValue,
            /* [out] */ ULONG *puStrengthMax);
        
        DECLSPEC_XFGVIRT(IDot11AdHocNetwork, GetSecuritySetting)
        HRESULT ( STDMETHODCALLTYPE *GetSecuritySetting )( 
            IDot11AdHocNetwork * This,
            /* [out] */ IDot11AdHocSecuritySettings **pAdHocSecuritySetting);
        
        DECLSPEC_XFGVIRT(IDot11AdHocNetwork, GetContextGuid)
        HRESULT ( STDMETHODCALLTYPE *GetContextGuid )( 
            IDot11AdHocNetwork * This,
            /* [out][in] */ GUID *pContextGuid);
        
        DECLSPEC_XFGVIRT(IDot11AdHocNetwork, GetSignature)
        HRESULT ( STDMETHODCALLTYPE *GetSignature )( 
            IDot11AdHocNetwork * This,
            /* [out][in] */ GUID *pSignature);
        
        DECLSPEC_XFGVIRT(IDot11AdHocNetwork, GetInterface)
        HRESULT ( STDMETHODCALLTYPE *GetInterface )( 
            IDot11AdHocNetwork * This,
            /* [out] */ IDot11AdHocInterface **pAdHocInterface);
        
        DECLSPEC_XFGVIRT(IDot11AdHocNetwork, Connect)
        HRESULT ( STDMETHODCALLTYPE *Connect )( 
            IDot11AdHocNetwork * This,
            /* [string][in] */ LPCWSTR Passphrase,
            /* [in] */ LONG GeographicalId,
            /* [in] */ BOOLEAN fSaveProfile,
            /* [in] */ BOOLEAN fMakeSavedProfileUserSpecific);
        
        DECLSPEC_XFGVIRT(IDot11AdHocNetwork, Disconnect)
        HRESULT ( STDMETHODCALLTYPE *Disconnect )( 
            IDot11AdHocNetwork * This);
        
        END_INTERFACE
    } IDot11AdHocNetworkVtbl;

    interface IDot11AdHocNetwork
    {
        CONST_VTBL struct IDot11AdHocNetworkVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDot11AdHocNetwork_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDot11AdHocNetwork_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDot11AdHocNetwork_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDot11AdHocNetwork_GetStatus(This,eStatus)	\
    ( (This)->lpVtbl -> GetStatus(This,eStatus) ) 

#define IDot11AdHocNetwork_GetSSID(This,ppszwSSID)	\
    ( (This)->lpVtbl -> GetSSID(This,ppszwSSID) ) 

#define IDot11AdHocNetwork_HasProfile(This,pf11d)	\
    ( (This)->lpVtbl -> HasProfile(This,pf11d) ) 

#define IDot11AdHocNetwork_GetProfileName(This,ppszwProfileName)	\
    ( (This)->lpVtbl -> GetProfileName(This,ppszwProfileName) ) 

#define IDot11AdHocNetwork_DeleteProfile(This)	\
    ( (This)->lpVtbl -> DeleteProfile(This) ) 

#define IDot11AdHocNetwork_GetSignalQuality(This,puStrengthValue,puStrengthMax)	\
    ( (This)->lpVtbl -> GetSignalQuality(This,puStrengthValue,puStrengthMax) ) 

#define IDot11AdHocNetwork_GetSecuritySetting(This,pAdHocSecuritySetting)	\
    ( (This)->lpVtbl -> GetSecuritySetting(This,pAdHocSecuritySetting) ) 

#define IDot11AdHocNetwork_GetContextGuid(This,pContextGuid)	\
    ( (This)->lpVtbl -> GetContextGuid(This,pContextGuid) ) 

#define IDot11AdHocNetwork_GetSignature(This,pSignature)	\
    ( (This)->lpVtbl -> GetSignature(This,pSignature) ) 

#define IDot11AdHocNetwork_GetInterface(This,pAdHocInterface)	\
    ( (This)->lpVtbl -> GetInterface(This,pAdHocInterface) ) 

#define IDot11AdHocNetwork_Connect(This,Passphrase,GeographicalId,fSaveProfile,fMakeSavedProfileUserSpecific)	\
    ( (This)->lpVtbl -> Connect(This,Passphrase,GeographicalId,fSaveProfile,fMakeSavedProfileUserSpecific) ) 

#define IDot11AdHocNetwork_Disconnect(This)	\
    ( (This)->lpVtbl -> Disconnect(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDot11AdHocNetwork_INTERFACE_DEFINED__ */


#ifndef __IDot11AdHocNetworkNotificationSink_INTERFACE_DEFINED__
#define __IDot11AdHocNetworkNotificationSink_INTERFACE_DEFINED__

/* interface IDot11AdHocNetworkNotificationSink */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_IDot11AdHocNetworkNotificationSink;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("8F10CC2A-CF0D-42a0-ACBE-E2DE7007384D")
    IDot11AdHocNetworkNotificationSink : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE OnStatusChange( 
            DOT11_ADHOC_NETWORK_CONNECTION_STATUS eStatus) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnConnectFail( 
            DOT11_ADHOC_CONNECT_FAIL_REASON eFailReason) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDot11AdHocNetworkNotificationSinkVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IDot11AdHocNetworkNotificationSink * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IDot11AdHocNetworkNotificationSink * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IDot11AdHocNetworkNotificationSink * This);
        
        DECLSPEC_XFGVIRT(IDot11AdHocNetworkNotificationSink, OnStatusChange)
        HRESULT ( STDMETHODCALLTYPE *OnStatusChange )( 
            IDot11AdHocNetworkNotificationSink * This,
            DOT11_ADHOC_NETWORK_CONNECTION_STATUS eStatus);
        
        DECLSPEC_XFGVIRT(IDot11AdHocNetworkNotificationSink, OnConnectFail)
        HRESULT ( STDMETHODCALLTYPE *OnConnectFail )( 
            IDot11AdHocNetworkNotificationSink * This,
            DOT11_ADHOC_CONNECT_FAIL_REASON eFailReason);
        
        END_INTERFACE
    } IDot11AdHocNetworkNotificationSinkVtbl;

    interface IDot11AdHocNetworkNotificationSink
    {
        CONST_VTBL struct IDot11AdHocNetworkNotificationSinkVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDot11AdHocNetworkNotificationSink_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDot11AdHocNetworkNotificationSink_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDot11AdHocNetworkNotificationSink_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDot11AdHocNetworkNotificationSink_OnStatusChange(This,eStatus)	\
    ( (This)->lpVtbl -> OnStatusChange(This,eStatus) ) 

#define IDot11AdHocNetworkNotificationSink_OnConnectFail(This,eFailReason)	\
    ( (This)->lpVtbl -> OnConnectFail(This,eFailReason) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDot11AdHocNetworkNotificationSink_INTERFACE_DEFINED__ */


#ifndef __IDot11AdHocInterface_INTERFACE_DEFINED__
#define __IDot11AdHocInterface_INTERFACE_DEFINED__

/* interface IDot11AdHocInterface */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_IDot11AdHocInterface;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("8F10CC2B-CF0D-42a0-ACBE-E2DE7007384D")
    IDot11AdHocInterface : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetDeviceSignature( 
            /* [out][in] */ GUID *pSignature) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetFriendlyName( 
            /* [string][out] */ LPWSTR *ppszName) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE IsDot11d( 
            /* [out][in] */ BOOLEAN *pf11d) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE IsAdHocCapable( 
            /* [out][in] */ BOOLEAN *pfAdHocCapable) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE IsRadioOn( 
            /* [out][in] */ BOOLEAN *pfIsRadioOn) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetActiveNetwork( 
            /* [out] */ IDot11AdHocNetwork **ppNetwork) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetIEnumSecuritySettings( 
            /* [out] */ IEnumDot11AdHocSecuritySettings **ppEnum) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetIEnumDot11AdHocNetworks( 
            /* [in] */ GUID *pFilterGuid,
            /* [out] */ IEnumDot11AdHocNetworks **ppEnum) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetStatus( 
            /* [out][in] */ DOT11_ADHOC_NETWORK_CONNECTION_STATUS *pState) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDot11AdHocInterfaceVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IDot11AdHocInterface * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IDot11AdHocInterface * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IDot11AdHocInterface * This);
        
        DECLSPEC_XFGVIRT(IDot11AdHocInterface, GetDeviceSignature)
        HRESULT ( STDMETHODCALLTYPE *GetDeviceSignature )( 
            IDot11AdHocInterface * This,
            /* [out][in] */ GUID *pSignature);
        
        DECLSPEC_XFGVIRT(IDot11AdHocInterface, GetFriendlyName)
        HRESULT ( STDMETHODCALLTYPE *GetFriendlyName )( 
            IDot11AdHocInterface * This,
            /* [string][out] */ LPWSTR *ppszName);
        
        DECLSPEC_XFGVIRT(IDot11AdHocInterface, IsDot11d)
        HRESULT ( STDMETHODCALLTYPE *IsDot11d )( 
            IDot11AdHocInterface * This,
            /* [out][in] */ BOOLEAN *pf11d);
        
        DECLSPEC_XFGVIRT(IDot11AdHocInterface, IsAdHocCapable)
        HRESULT ( STDMETHODCALLTYPE *IsAdHocCapable )( 
            IDot11AdHocInterface * This,
            /* [out][in] */ BOOLEAN *pfAdHocCapable);
        
        DECLSPEC_XFGVIRT(IDot11AdHocInterface, IsRadioOn)
        HRESULT ( STDMETHODCALLTYPE *IsRadioOn )( 
            IDot11AdHocInterface * This,
            /* [out][in] */ BOOLEAN *pfIsRadioOn);
        
        DECLSPEC_XFGVIRT(IDot11AdHocInterface, GetActiveNetwork)
        HRESULT ( STDMETHODCALLTYPE *GetActiveNetwork )( 
            IDot11AdHocInterface * This,
            /* [out] */ IDot11AdHocNetwork **ppNetwork);
        
        DECLSPEC_XFGVIRT(IDot11AdHocInterface, GetIEnumSecuritySettings)
        HRESULT ( STDMETHODCALLTYPE *GetIEnumSecuritySettings )( 
            IDot11AdHocInterface * This,
            /* [out] */ IEnumDot11AdHocSecuritySettings **ppEnum);
        
        DECLSPEC_XFGVIRT(IDot11AdHocInterface, GetIEnumDot11AdHocNetworks)
        HRESULT ( STDMETHODCALLTYPE *GetIEnumDot11AdHocNetworks )( 
            IDot11AdHocInterface * This,
            /* [in] */ GUID *pFilterGuid,
            /* [out] */ IEnumDot11AdHocNetworks **ppEnum);
        
        DECLSPEC_XFGVIRT(IDot11AdHocInterface, GetStatus)
        HRESULT ( STDMETHODCALLTYPE *GetStatus )( 
            IDot11AdHocInterface * This,
            /* [out][in] */ DOT11_ADHOC_NETWORK_CONNECTION_STATUS *pState);
        
        END_INTERFACE
    } IDot11AdHocInterfaceVtbl;

    interface IDot11AdHocInterface
    {
        CONST_VTBL struct IDot11AdHocInterfaceVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDot11AdHocInterface_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDot11AdHocInterface_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDot11AdHocInterface_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDot11AdHocInterface_GetDeviceSignature(This,pSignature)	\
    ( (This)->lpVtbl -> GetDeviceSignature(This,pSignature) ) 

#define IDot11AdHocInterface_GetFriendlyName(This,ppszName)	\
    ( (This)->lpVtbl -> GetFriendlyName(This,ppszName) ) 

#define IDot11AdHocInterface_IsDot11d(This,pf11d)	\
    ( (This)->lpVtbl -> IsDot11d(This,pf11d) ) 

#define IDot11AdHocInterface_IsAdHocCapable(This,pfAdHocCapable)	\
    ( (This)->lpVtbl -> IsAdHocCapable(This,pfAdHocCapable) ) 

#define IDot11AdHocInterface_IsRadioOn(This,pfIsRadioOn)	\
    ( (This)->lpVtbl -> IsRadioOn(This,pfIsRadioOn) ) 

#define IDot11AdHocInterface_GetActiveNetwork(This,ppNetwork)	\
    ( (This)->lpVtbl -> GetActiveNetwork(This,ppNetwork) ) 

#define IDot11AdHocInterface_GetIEnumSecuritySettings(This,ppEnum)	\
    ( (This)->lpVtbl -> GetIEnumSecuritySettings(This,ppEnum) ) 

#define IDot11AdHocInterface_GetIEnumDot11AdHocNetworks(This,pFilterGuid,ppEnum)	\
    ( (This)->lpVtbl -> GetIEnumDot11AdHocNetworks(This,pFilterGuid,ppEnum) ) 

#define IDot11AdHocInterface_GetStatus(This,pState)	\
    ( (This)->lpVtbl -> GetStatus(This,pState) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDot11AdHocInterface_INTERFACE_DEFINED__ */


#ifndef __IEnumDot11AdHocInterfaces_INTERFACE_DEFINED__
#define __IEnumDot11AdHocInterfaces_INTERFACE_DEFINED__

/* interface IEnumDot11AdHocInterfaces */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_IEnumDot11AdHocInterfaces;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("8F10CC2C-CF0D-42a0-ACBE-E2DE7007384D")
    IEnumDot11AdHocInterfaces : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Next( 
            /* [in] */ ULONG cElt,
            /* [length_is][size_is][out] */ IDot11AdHocInterface **rgElt,
            /* [out] */ ULONG *pcEltFetched) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Skip( 
            /* [in] */ ULONG cElt) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Reset( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Clone( 
            /* [out] */ IEnumDot11AdHocInterfaces **ppEnum) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IEnumDot11AdHocInterfacesVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IEnumDot11AdHocInterfaces * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IEnumDot11AdHocInterfaces * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IEnumDot11AdHocInterfaces * This);
        
        DECLSPEC_XFGVIRT(IEnumDot11AdHocInterfaces, Next)
        HRESULT ( STDMETHODCALLTYPE *Next )( 
            IEnumDot11AdHocInterfaces * This,
            /* [in] */ ULONG cElt,
            /* [length_is][size_is][out] */ IDot11AdHocInterface **rgElt,
            /* [out] */ ULONG *pcEltFetched);
        
        DECLSPEC_XFGVIRT(IEnumDot11AdHocInterfaces, Skip)
        HRESULT ( STDMETHODCALLTYPE *Skip )( 
            IEnumDot11AdHocInterfaces * This,
            /* [in] */ ULONG cElt);
        
        DECLSPEC_XFGVIRT(IEnumDot11AdHocInterfaces, Reset)
        HRESULT ( STDMETHODCALLTYPE *Reset )( 
            IEnumDot11AdHocInterfaces * This);
        
        DECLSPEC_XFGVIRT(IEnumDot11AdHocInterfaces, Clone)
        HRESULT ( STDMETHODCALLTYPE *Clone )( 
            IEnumDot11AdHocInterfaces * This,
            /* [out] */ IEnumDot11AdHocInterfaces **ppEnum);
        
        END_INTERFACE
    } IEnumDot11AdHocInterfacesVtbl;

    interface IEnumDot11AdHocInterfaces
    {
        CONST_VTBL struct IEnumDot11AdHocInterfacesVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IEnumDot11AdHocInterfaces_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IEnumDot11AdHocInterfaces_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IEnumDot11AdHocInterfaces_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IEnumDot11AdHocInterfaces_Next(This,cElt,rgElt,pcEltFetched)	\
    ( (This)->lpVtbl -> Next(This,cElt,rgElt,pcEltFetched) ) 

#define IEnumDot11AdHocInterfaces_Skip(This,cElt)	\
    ( (This)->lpVtbl -> Skip(This,cElt) ) 

#define IEnumDot11AdHocInterfaces_Reset(This)	\
    ( (This)->lpVtbl -> Reset(This) ) 

#define IEnumDot11AdHocInterfaces_Clone(This,ppEnum)	\
    ( (This)->lpVtbl -> Clone(This,ppEnum) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IEnumDot11AdHocInterfaces_INTERFACE_DEFINED__ */


#ifndef __IEnumDot11AdHocSecuritySettings_INTERFACE_DEFINED__
#define __IEnumDot11AdHocSecuritySettings_INTERFACE_DEFINED__

/* interface IEnumDot11AdHocSecuritySettings */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_IEnumDot11AdHocSecuritySettings;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("8F10CC2D-CF0D-42a0-ACBE-E2DE7007384D")
    IEnumDot11AdHocSecuritySettings : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Next( 
            /* [in] */ ULONG cElt,
            /* [length_is][size_is][out] */ IDot11AdHocSecuritySettings **rgElt,
            /* [out] */ ULONG *pcEltFetched) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Skip( 
            /* [in] */ ULONG cElt) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Reset( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Clone( 
            /* [out] */ IEnumDot11AdHocSecuritySettings **ppEnum) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IEnumDot11AdHocSecuritySettingsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IEnumDot11AdHocSecuritySettings * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IEnumDot11AdHocSecuritySettings * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IEnumDot11AdHocSecuritySettings * This);
        
        DECLSPEC_XFGVIRT(IEnumDot11AdHocSecuritySettings, Next)
        HRESULT ( STDMETHODCALLTYPE *Next )( 
            IEnumDot11AdHocSecuritySettings * This,
            /* [in] */ ULONG cElt,
            /* [length_is][size_is][out] */ IDot11AdHocSecuritySettings **rgElt,
            /* [out] */ ULONG *pcEltFetched);
        
        DECLSPEC_XFGVIRT(IEnumDot11AdHocSecuritySettings, Skip)
        HRESULT ( STDMETHODCALLTYPE *Skip )( 
            IEnumDot11AdHocSecuritySettings * This,
            /* [in] */ ULONG cElt);
        
        DECLSPEC_XFGVIRT(IEnumDot11AdHocSecuritySettings, Reset)
        HRESULT ( STDMETHODCALLTYPE *Reset )( 
            IEnumDot11AdHocSecuritySettings * This);
        
        DECLSPEC_XFGVIRT(IEnumDot11AdHocSecuritySettings, Clone)
        HRESULT ( STDMETHODCALLTYPE *Clone )( 
            IEnumDot11AdHocSecuritySettings * This,
            /* [out] */ IEnumDot11AdHocSecuritySettings **ppEnum);
        
        END_INTERFACE
    } IEnumDot11AdHocSecuritySettingsVtbl;

    interface IEnumDot11AdHocSecuritySettings
    {
        CONST_VTBL struct IEnumDot11AdHocSecuritySettingsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IEnumDot11AdHocSecuritySettings_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IEnumDot11AdHocSecuritySettings_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IEnumDot11AdHocSecuritySettings_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IEnumDot11AdHocSecuritySettings_Next(This,cElt,rgElt,pcEltFetched)	\
    ( (This)->lpVtbl -> Next(This,cElt,rgElt,pcEltFetched) ) 

#define IEnumDot11AdHocSecuritySettings_Skip(This,cElt)	\
    ( (This)->lpVtbl -> Skip(This,cElt) ) 

#define IEnumDot11AdHocSecuritySettings_Reset(This)	\
    ( (This)->lpVtbl -> Reset(This) ) 

#define IEnumDot11AdHocSecuritySettings_Clone(This,ppEnum)	\
    ( (This)->lpVtbl -> Clone(This,ppEnum) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IEnumDot11AdHocSecuritySettings_INTERFACE_DEFINED__ */


#ifndef __IDot11AdHocSecuritySettings_INTERFACE_DEFINED__
#define __IDot11AdHocSecuritySettings_INTERFACE_DEFINED__

/* interface IDot11AdHocSecuritySettings */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_IDot11AdHocSecuritySettings;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("8F10CC2E-CF0D-42a0-ACBE-E2DE7007384D")
    IDot11AdHocSecuritySettings : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetDot11AuthAlgorithm( 
            /* [out][in] */ DOT11_ADHOC_AUTH_ALGORITHM *pAuth) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetDot11CipherAlgorithm( 
            /* [out][in] */ DOT11_ADHOC_CIPHER_ALGORITHM *pCipher) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDot11AdHocSecuritySettingsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IDot11AdHocSecuritySettings * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IDot11AdHocSecuritySettings * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IDot11AdHocSecuritySettings * This);
        
        DECLSPEC_XFGVIRT(IDot11AdHocSecuritySettings, GetDot11AuthAlgorithm)
        HRESULT ( STDMETHODCALLTYPE *GetDot11AuthAlgorithm )( 
            IDot11AdHocSecuritySettings * This,
            /* [out][in] */ DOT11_ADHOC_AUTH_ALGORITHM *pAuth);
        
        DECLSPEC_XFGVIRT(IDot11AdHocSecuritySettings, GetDot11CipherAlgorithm)
        HRESULT ( STDMETHODCALLTYPE *GetDot11CipherAlgorithm )( 
            IDot11AdHocSecuritySettings * This,
            /* [out][in] */ DOT11_ADHOC_CIPHER_ALGORITHM *pCipher);
        
        END_INTERFACE
    } IDot11AdHocSecuritySettingsVtbl;

    interface IDot11AdHocSecuritySettings
    {
        CONST_VTBL struct IDot11AdHocSecuritySettingsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDot11AdHocSecuritySettings_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDot11AdHocSecuritySettings_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDot11AdHocSecuritySettings_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDot11AdHocSecuritySettings_GetDot11AuthAlgorithm(This,pAuth)	\
    ( (This)->lpVtbl -> GetDot11AuthAlgorithm(This,pAuth) ) 

#define IDot11AdHocSecuritySettings_GetDot11CipherAlgorithm(This,pCipher)	\
    ( (This)->lpVtbl -> GetDot11CipherAlgorithm(This,pCipher) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDot11AdHocSecuritySettings_INTERFACE_DEFINED__ */


#ifndef __IDot11AdHocInterfaceNotificationSink_INTERFACE_DEFINED__
#define __IDot11AdHocInterfaceNotificationSink_INTERFACE_DEFINED__

/* interface IDot11AdHocInterfaceNotificationSink */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_IDot11AdHocInterfaceNotificationSink;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("8F10CC2F-CF0D-42a0-ACBE-E2DE7007384D")
    IDot11AdHocInterfaceNotificationSink : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE OnConnectionStatusChange( 
            DOT11_ADHOC_NETWORK_CONNECTION_STATUS eStatus) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDot11AdHocInterfaceNotificationSinkVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IDot11AdHocInterfaceNotificationSink * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IDot11AdHocInterfaceNotificationSink * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IDot11AdHocInterfaceNotificationSink * This);
        
        DECLSPEC_XFGVIRT(IDot11AdHocInterfaceNotificationSink, OnConnectionStatusChange)
        HRESULT ( STDMETHODCALLTYPE *OnConnectionStatusChange )( 
            IDot11AdHocInterfaceNotificationSink * This,
            DOT11_ADHOC_NETWORK_CONNECTION_STATUS eStatus);
        
        END_INTERFACE
    } IDot11AdHocInterfaceNotificationSinkVtbl;

    interface IDot11AdHocInterfaceNotificationSink
    {
        CONST_VTBL struct IDot11AdHocInterfaceNotificationSinkVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDot11AdHocInterfaceNotificationSink_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDot11AdHocInterfaceNotificationSink_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDot11AdHocInterfaceNotificationSink_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDot11AdHocInterfaceNotificationSink_OnConnectionStatusChange(This,eStatus)	\
    ( (This)->lpVtbl -> OnConnectionStatusChange(This,eStatus) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDot11AdHocInterfaceNotificationSink_INTERFACE_DEFINED__ */



#ifndef __ADHOCLib_LIBRARY_DEFINED__
#define __ADHOCLib_LIBRARY_DEFINED__

/* library ADHOCLib */
/* [helpstring][version][uuid] */ 


EXTERN_C const IID LIBID_ADHOCLib;

EXTERN_C const CLSID CLSID_Dot11AdHocManager;

#ifdef __cplusplus

class DECLSPEC_UUID("DD06A84F-83BD-4d01-8AB9-2389FEA0869E")
Dot11AdHocManager;
#endif
#endif /* __ADHOCLib_LIBRARY_DEFINED__ */

/* interface __MIDL_itf_adhoc_0000_0011 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_adhoc_0000_0011_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_adhoc_0000_0011_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


