

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

#ifndef __wtsprotocol_h__
#define __wtsprotocol_h__

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

#ifndef __IWTSProtocolManager_FWD_DEFINED__
#define __IWTSProtocolManager_FWD_DEFINED__
typedef interface IWTSProtocolManager IWTSProtocolManager;

#endif 	/* __IWTSProtocolManager_FWD_DEFINED__ */


#ifndef __IWTSProtocolListener_FWD_DEFINED__
#define __IWTSProtocolListener_FWD_DEFINED__
typedef interface IWTSProtocolListener IWTSProtocolListener;

#endif 	/* __IWTSProtocolListener_FWD_DEFINED__ */


#ifndef __IWTSProtocolListenerCallback_FWD_DEFINED__
#define __IWTSProtocolListenerCallback_FWD_DEFINED__
typedef interface IWTSProtocolListenerCallback IWTSProtocolListenerCallback;

#endif 	/* __IWTSProtocolListenerCallback_FWD_DEFINED__ */


#ifndef __IWTSProtocolConnection_FWD_DEFINED__
#define __IWTSProtocolConnection_FWD_DEFINED__
typedef interface IWTSProtocolConnection IWTSProtocolConnection;

#endif 	/* __IWTSProtocolConnection_FWD_DEFINED__ */


#ifndef __IWTSProtocolConnectionCallback_FWD_DEFINED__
#define __IWTSProtocolConnectionCallback_FWD_DEFINED__
typedef interface IWTSProtocolConnectionCallback IWTSProtocolConnectionCallback;

#endif 	/* __IWTSProtocolConnectionCallback_FWD_DEFINED__ */


#ifndef __IWTSProtocolShadowConnection_FWD_DEFINED__
#define __IWTSProtocolShadowConnection_FWD_DEFINED__
typedef interface IWTSProtocolShadowConnection IWTSProtocolShadowConnection;

#endif 	/* __IWTSProtocolShadowConnection_FWD_DEFINED__ */


#ifndef __IWTSProtocolShadowCallback_FWD_DEFINED__
#define __IWTSProtocolShadowCallback_FWD_DEFINED__
typedef interface IWTSProtocolShadowCallback IWTSProtocolShadowCallback;

#endif 	/* __IWTSProtocolShadowCallback_FWD_DEFINED__ */


#ifndef __IWTSProtocolLicenseConnection_FWD_DEFINED__
#define __IWTSProtocolLicenseConnection_FWD_DEFINED__
typedef interface IWTSProtocolLicenseConnection IWTSProtocolLicenseConnection;

#endif 	/* __IWTSProtocolLicenseConnection_FWD_DEFINED__ */


#ifndef __IWTSProtocolLogonErrorRedirector_FWD_DEFINED__
#define __IWTSProtocolLogonErrorRedirector_FWD_DEFINED__
typedef interface IWTSProtocolLogonErrorRedirector IWTSProtocolLogonErrorRedirector;

#endif 	/* __IWTSProtocolLogonErrorRedirector_FWD_DEFINED__ */


#ifndef __IWRdsProtocolSettings_FWD_DEFINED__
#define __IWRdsProtocolSettings_FWD_DEFINED__
typedef interface IWRdsProtocolSettings IWRdsProtocolSettings;

#endif 	/* __IWRdsProtocolSettings_FWD_DEFINED__ */


#ifndef __IWRdsProtocolManager_FWD_DEFINED__
#define __IWRdsProtocolManager_FWD_DEFINED__
typedef interface IWRdsProtocolManager IWRdsProtocolManager;

#endif 	/* __IWRdsProtocolManager_FWD_DEFINED__ */


#ifndef __IWRdsProtocolListener_FWD_DEFINED__
#define __IWRdsProtocolListener_FWD_DEFINED__
typedef interface IWRdsProtocolListener IWRdsProtocolListener;

#endif 	/* __IWRdsProtocolListener_FWD_DEFINED__ */


#ifndef __IWRdsProtocolListenerCallback_FWD_DEFINED__
#define __IWRdsProtocolListenerCallback_FWD_DEFINED__
typedef interface IWRdsProtocolListenerCallback IWRdsProtocolListenerCallback;

#endif 	/* __IWRdsProtocolListenerCallback_FWD_DEFINED__ */


#ifndef __IWRdsProtocolConnection_FWD_DEFINED__
#define __IWRdsProtocolConnection_FWD_DEFINED__
typedef interface IWRdsProtocolConnection IWRdsProtocolConnection;

#endif 	/* __IWRdsProtocolConnection_FWD_DEFINED__ */


#ifndef __IWRdsProtocolConnectionCallback_FWD_DEFINED__
#define __IWRdsProtocolConnectionCallback_FWD_DEFINED__
typedef interface IWRdsProtocolConnectionCallback IWRdsProtocolConnectionCallback;

#endif 	/* __IWRdsProtocolConnectionCallback_FWD_DEFINED__ */


#ifndef __IWRdsProtocolShadowConnection_FWD_DEFINED__
#define __IWRdsProtocolShadowConnection_FWD_DEFINED__
typedef interface IWRdsProtocolShadowConnection IWRdsProtocolShadowConnection;

#endif 	/* __IWRdsProtocolShadowConnection_FWD_DEFINED__ */


#ifndef __IWRdsProtocolShadowCallback_FWD_DEFINED__
#define __IWRdsProtocolShadowCallback_FWD_DEFINED__
typedef interface IWRdsProtocolShadowCallback IWRdsProtocolShadowCallback;

#endif 	/* __IWRdsProtocolShadowCallback_FWD_DEFINED__ */


#ifndef __IWRdsProtocolLicenseConnection_FWD_DEFINED__
#define __IWRdsProtocolLicenseConnection_FWD_DEFINED__
typedef interface IWRdsProtocolLicenseConnection IWRdsProtocolLicenseConnection;

#endif 	/* __IWRdsProtocolLicenseConnection_FWD_DEFINED__ */


#ifndef __IWRdsProtocolLogonErrorRedirector_FWD_DEFINED__
#define __IWRdsProtocolLogonErrorRedirector_FWD_DEFINED__
typedef interface IWRdsProtocolLogonErrorRedirector IWRdsProtocolLogonErrorRedirector;

#endif 	/* __IWRdsProtocolLogonErrorRedirector_FWD_DEFINED__ */


#ifndef __IWRdsWddmIddProps_FWD_DEFINED__
#define __IWRdsWddmIddProps_FWD_DEFINED__
typedef interface IWRdsWddmIddProps IWRdsWddmIddProps;

#endif 	/* __IWRdsWddmIddProps_FWD_DEFINED__ */


#ifndef __IWRdsWddmIddProps1_FWD_DEFINED__
#define __IWRdsWddmIddProps1_FWD_DEFINED__
typedef interface IWRdsWddmIddProps1 IWRdsWddmIddProps1;

#endif 	/* __IWRdsWddmIddProps1_FWD_DEFINED__ */


#ifndef __IWRdsProtocolConnectionSettings_FWD_DEFINED__
#define __IWRdsProtocolConnectionSettings_FWD_DEFINED__
typedef interface IWRdsProtocolConnectionSettings IWRdsProtocolConnectionSettings;

#endif 	/* __IWRdsProtocolConnectionSettings_FWD_DEFINED__ */


#ifndef __IWRdsEnhancedFastReconnectArbitrator_FWD_DEFINED__
#define __IWRdsEnhancedFastReconnectArbitrator_FWD_DEFINED__
typedef interface IWRdsEnhancedFastReconnectArbitrator IWRdsEnhancedFastReconnectArbitrator;

#endif 	/* __IWRdsEnhancedFastReconnectArbitrator_FWD_DEFINED__ */


#ifndef __IWRdsProtocolConnection2_FWD_DEFINED__
#define __IWRdsProtocolConnection2_FWD_DEFINED__
typedef interface IWRdsProtocolConnection2 IWRdsProtocolConnection2;

#endif 	/* __IWRdsProtocolConnection2_FWD_DEFINED__ */


/* header files for imported files */
#include "oaidl.h"
#include "ocidl.h"
#include "wtsdefs.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_wtsprotocol_0000_0000 */
/* [local] */ 

#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)










extern RPC_IF_HANDLE __MIDL_itf_wtsprotocol_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_wtsprotocol_0000_0000_v0_0_s_ifspec;

#ifndef __IWTSProtocolManager_INTERFACE_DEFINED__
#define __IWTSProtocolManager_INTERFACE_DEFINED__

/* interface IWTSProtocolManager */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IWTSProtocolManager;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("F9EAF6CC-ED79-4f01-821D-1F881B9F66CC")
    IWTSProtocolManager : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE CreateListener( 
            /* [string][in] */ __RPC__in_string WCHAR *wszListenerName,
            /* [out] */ __RPC__deref_out_opt IWTSProtocolListener **pProtocolListener) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE NotifyServiceStateChange( 
            /* [in] */ __RPC__in WTS_SERVICE_STATE *pTSServiceStateChange) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE NotifySessionOfServiceStart( 
            /* [in] */ __RPC__in WTS_SESSION_ID *SessionId) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE NotifySessionOfServiceStop( 
            /* [in] */ __RPC__in WTS_SESSION_ID *SessionId) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE NotifySessionStateChange( 
            /* [in] */ __RPC__in WTS_SESSION_ID *SessionId,
            /* [in] */ ULONG EventId) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWTSProtocolManagerVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IWTSProtocolManager * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IWTSProtocolManager * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IWTSProtocolManager * This);
        
        DECLSPEC_XFGVIRT(IWTSProtocolManager, CreateListener)
        HRESULT ( STDMETHODCALLTYPE *CreateListener )( 
            __RPC__in IWTSProtocolManager * This,
            /* [string][in] */ __RPC__in_string WCHAR *wszListenerName,
            /* [out] */ __RPC__deref_out_opt IWTSProtocolListener **pProtocolListener);
        
        DECLSPEC_XFGVIRT(IWTSProtocolManager, NotifyServiceStateChange)
        HRESULT ( STDMETHODCALLTYPE *NotifyServiceStateChange )( 
            __RPC__in IWTSProtocolManager * This,
            /* [in] */ __RPC__in WTS_SERVICE_STATE *pTSServiceStateChange);
        
        DECLSPEC_XFGVIRT(IWTSProtocolManager, NotifySessionOfServiceStart)
        HRESULT ( STDMETHODCALLTYPE *NotifySessionOfServiceStart )( 
            __RPC__in IWTSProtocolManager * This,
            /* [in] */ __RPC__in WTS_SESSION_ID *SessionId);
        
        DECLSPEC_XFGVIRT(IWTSProtocolManager, NotifySessionOfServiceStop)
        HRESULT ( STDMETHODCALLTYPE *NotifySessionOfServiceStop )( 
            __RPC__in IWTSProtocolManager * This,
            /* [in] */ __RPC__in WTS_SESSION_ID *SessionId);
        
        DECLSPEC_XFGVIRT(IWTSProtocolManager, NotifySessionStateChange)
        HRESULT ( STDMETHODCALLTYPE *NotifySessionStateChange )( 
            __RPC__in IWTSProtocolManager * This,
            /* [in] */ __RPC__in WTS_SESSION_ID *SessionId,
            /* [in] */ ULONG EventId);
        
        END_INTERFACE
    } IWTSProtocolManagerVtbl;

    interface IWTSProtocolManager
    {
        CONST_VTBL struct IWTSProtocolManagerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWTSProtocolManager_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWTSProtocolManager_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWTSProtocolManager_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWTSProtocolManager_CreateListener(This,wszListenerName,pProtocolListener)	\
    ( (This)->lpVtbl -> CreateListener(This,wszListenerName,pProtocolListener) ) 

#define IWTSProtocolManager_NotifyServiceStateChange(This,pTSServiceStateChange)	\
    ( (This)->lpVtbl -> NotifyServiceStateChange(This,pTSServiceStateChange) ) 

#define IWTSProtocolManager_NotifySessionOfServiceStart(This,SessionId)	\
    ( (This)->lpVtbl -> NotifySessionOfServiceStart(This,SessionId) ) 

#define IWTSProtocolManager_NotifySessionOfServiceStop(This,SessionId)	\
    ( (This)->lpVtbl -> NotifySessionOfServiceStop(This,SessionId) ) 

#define IWTSProtocolManager_NotifySessionStateChange(This,SessionId,EventId)	\
    ( (This)->lpVtbl -> NotifySessionStateChange(This,SessionId,EventId) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWTSProtocolManager_INTERFACE_DEFINED__ */


#ifndef __IWTSProtocolListener_INTERFACE_DEFINED__
#define __IWTSProtocolListener_INTERFACE_DEFINED__

/* interface IWTSProtocolListener */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IWTSProtocolListener;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("23083765-45f0-4394-8f69-32b2bc0ef4ca")
    IWTSProtocolListener : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE StartListen( 
            /* [in] */ __RPC__in_opt IWTSProtocolListenerCallback *pCallback) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE StopListen( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWTSProtocolListenerVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IWTSProtocolListener * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IWTSProtocolListener * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IWTSProtocolListener * This);
        
        DECLSPEC_XFGVIRT(IWTSProtocolListener, StartListen)
        HRESULT ( STDMETHODCALLTYPE *StartListen )( 
            __RPC__in IWTSProtocolListener * This,
            /* [in] */ __RPC__in_opt IWTSProtocolListenerCallback *pCallback);
        
        DECLSPEC_XFGVIRT(IWTSProtocolListener, StopListen)
        HRESULT ( STDMETHODCALLTYPE *StopListen )( 
            __RPC__in IWTSProtocolListener * This);
        
        END_INTERFACE
    } IWTSProtocolListenerVtbl;

    interface IWTSProtocolListener
    {
        CONST_VTBL struct IWTSProtocolListenerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWTSProtocolListener_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWTSProtocolListener_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWTSProtocolListener_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWTSProtocolListener_StartListen(This,pCallback)	\
    ( (This)->lpVtbl -> StartListen(This,pCallback) ) 

#define IWTSProtocolListener_StopListen(This)	\
    ( (This)->lpVtbl -> StopListen(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWTSProtocolListener_INTERFACE_DEFINED__ */


#ifndef __IWTSProtocolListenerCallback_INTERFACE_DEFINED__
#define __IWTSProtocolListenerCallback_INTERFACE_DEFINED__

/* interface IWTSProtocolListenerCallback */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IWTSProtocolListenerCallback;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("23083765-1a2d-4de2-97de-4a35f260f0b3")
    IWTSProtocolListenerCallback : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE OnConnected( 
            /* [in] */ __RPC__in_opt IWTSProtocolConnection *pConnection,
            /* [out] */ __RPC__deref_out_opt IWTSProtocolConnectionCallback **pCallback) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWTSProtocolListenerCallbackVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IWTSProtocolListenerCallback * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IWTSProtocolListenerCallback * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IWTSProtocolListenerCallback * This);
        
        DECLSPEC_XFGVIRT(IWTSProtocolListenerCallback, OnConnected)
        HRESULT ( STDMETHODCALLTYPE *OnConnected )( 
            __RPC__in IWTSProtocolListenerCallback * This,
            /* [in] */ __RPC__in_opt IWTSProtocolConnection *pConnection,
            /* [out] */ __RPC__deref_out_opt IWTSProtocolConnectionCallback **pCallback);
        
        END_INTERFACE
    } IWTSProtocolListenerCallbackVtbl;

    interface IWTSProtocolListenerCallback
    {
        CONST_VTBL struct IWTSProtocolListenerCallbackVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWTSProtocolListenerCallback_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWTSProtocolListenerCallback_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWTSProtocolListenerCallback_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWTSProtocolListenerCallback_OnConnected(This,pConnection,pCallback)	\
    ( (This)->lpVtbl -> OnConnected(This,pConnection,pCallback) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWTSProtocolListenerCallback_INTERFACE_DEFINED__ */


#ifndef __IWTSProtocolConnection_INTERFACE_DEFINED__
#define __IWTSProtocolConnection_INTERFACE_DEFINED__

/* interface IWTSProtocolConnection */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IWTSProtocolConnection;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("23083765-9095-4648-98bf-ef81c914032d")
    IWTSProtocolConnection : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetLogonErrorRedirector( 
            /* [out] */ __RPC__deref_out_opt IWTSProtocolLogonErrorRedirector **ppLogonErrorRedir) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SendPolicyData( 
            /* [in] */ __RPC__in WTS_POLICY_DATA *pPolicyData) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE AcceptConnection( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetClientData( 
            /* [out] */ __RPC__out WTS_CLIENT_DATA *pClientData) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetUserCredentials( 
            /* [out] */ __RPC__out WTS_USER_CREDENTIAL *pUserCreds) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetLicenseConnection( 
            /* [out] */ __RPC__deref_out_opt IWTSProtocolLicenseConnection **ppLicenseConnection) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE AuthenticateClientToSession( 
            /* [out] */ __RPC__out WTS_SESSION_ID *SessionId) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE NotifySessionId( 
            /* [in] */ __RPC__in WTS_SESSION_ID *SessionId) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetProtocolHandles( 
            /* [out] */ __RPC__out HANDLE_PTR *pKeyboardHandle,
            /* [out] */ __RPC__out HANDLE_PTR *pMouseHandle,
            /* [out] */ __RPC__out HANDLE_PTR *pBeepHandle,
            /* [out] */ __RPC__out HANDLE_PTR *pVideoHandle) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ConnectNotify( 
            /* [in] */ ULONG SessionId) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE IsUserAllowedToLogon( 
            /* [in] */ ULONG SessionId,
            /* [in] */ HANDLE_PTR UserToken,
            /* [string][in] */ __RPC__in_string WCHAR *pDomainName,
            /* [string][in] */ __RPC__in_string WCHAR *pUserName) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SessionArbitrationEnumeration( 
            /* [in] */ HANDLE_PTR hUserToken,
            /* [in] */ BOOL bSingleSessionPerUserEnabled,
            /* [size_is][out] */ __RPC__out_ecount_full(*pdwSessionIdentifierCount) ULONG *pSessionIdArray,
            /* [out][in] */ __RPC__inout ULONG *pdwSessionIdentifierCount) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE LogonNotify( 
            /* [in] */ HANDLE_PTR hClientToken,
            /* [string][in] */ __RPC__in_string WCHAR *wszUserName,
            /* [string][in] */ __RPC__in_string WCHAR *wszDomainName,
            /* [in] */ __RPC__in WTS_SESSION_ID *SessionId) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetUserData( 
            /* [in] */ __RPC__in WTS_POLICY_DATA *pPolicyData,
            /* [out][in] */ __RPC__inout WTS_USER_DATA *pClientData) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE DisconnectNotify( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Close( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetProtocolStatus( 
            /* [out] */ __RPC__out WTS_PROTOCOL_STATUS *pProtocolStatus) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetLastInputTime( 
            /* [out] */ __RPC__out ULONG64 *pLastInputTime) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetErrorInfo( 
            /* [in] */ ULONG ulError) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SendBeep( 
            /* [in] */ ULONG Frequency,
            /* [in] */ ULONG Duration) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreateVirtualChannel( 
            /* [string][in] */ __RPC__in_string CHAR *szEndpointName,
            /* [in] */ BOOL bStatic,
            /* [in] */ ULONG RequestedPriority,
            /* [out] */ __RPC__out ULONG_PTR *phChannel) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE QueryProperty( 
            /* [in] */ GUID QueryType,
            /* [in] */ ULONG ulNumEntriesIn,
            /* [in] */ ULONG ulNumEntriesOut,
            /* [size_is][optional][in] */ __RPC__in_ecount_full(ulNumEntriesIn) PWTS_PROPERTY_VALUE pPropertyEntriesIn,
            /* [size_is][optional][out] */ __RPC__out_ecount_full(ulNumEntriesOut) PWTS_PROPERTY_VALUE pPropertyEntriesOut) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetShadowConnection( 
            /* [out] */ __RPC__deref_out_opt IWTSProtocolShadowConnection **ppShadowConnection) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWTSProtocolConnectionVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IWTSProtocolConnection * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IWTSProtocolConnection * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IWTSProtocolConnection * This);
        
        DECLSPEC_XFGVIRT(IWTSProtocolConnection, GetLogonErrorRedirector)
        HRESULT ( STDMETHODCALLTYPE *GetLogonErrorRedirector )( 
            __RPC__in IWTSProtocolConnection * This,
            /* [out] */ __RPC__deref_out_opt IWTSProtocolLogonErrorRedirector **ppLogonErrorRedir);
        
        DECLSPEC_XFGVIRT(IWTSProtocolConnection, SendPolicyData)
        HRESULT ( STDMETHODCALLTYPE *SendPolicyData )( 
            __RPC__in IWTSProtocolConnection * This,
            /* [in] */ __RPC__in WTS_POLICY_DATA *pPolicyData);
        
        DECLSPEC_XFGVIRT(IWTSProtocolConnection, AcceptConnection)
        HRESULT ( STDMETHODCALLTYPE *AcceptConnection )( 
            __RPC__in IWTSProtocolConnection * This);
        
        DECLSPEC_XFGVIRT(IWTSProtocolConnection, GetClientData)
        HRESULT ( STDMETHODCALLTYPE *GetClientData )( 
            __RPC__in IWTSProtocolConnection * This,
            /* [out] */ __RPC__out WTS_CLIENT_DATA *pClientData);
        
        DECLSPEC_XFGVIRT(IWTSProtocolConnection, GetUserCredentials)
        HRESULT ( STDMETHODCALLTYPE *GetUserCredentials )( 
            __RPC__in IWTSProtocolConnection * This,
            /* [out] */ __RPC__out WTS_USER_CREDENTIAL *pUserCreds);
        
        DECLSPEC_XFGVIRT(IWTSProtocolConnection, GetLicenseConnection)
        HRESULT ( STDMETHODCALLTYPE *GetLicenseConnection )( 
            __RPC__in IWTSProtocolConnection * This,
            /* [out] */ __RPC__deref_out_opt IWTSProtocolLicenseConnection **ppLicenseConnection);
        
        DECLSPEC_XFGVIRT(IWTSProtocolConnection, AuthenticateClientToSession)
        HRESULT ( STDMETHODCALLTYPE *AuthenticateClientToSession )( 
            __RPC__in IWTSProtocolConnection * This,
            /* [out] */ __RPC__out WTS_SESSION_ID *SessionId);
        
        DECLSPEC_XFGVIRT(IWTSProtocolConnection, NotifySessionId)
        HRESULT ( STDMETHODCALLTYPE *NotifySessionId )( 
            __RPC__in IWTSProtocolConnection * This,
            /* [in] */ __RPC__in WTS_SESSION_ID *SessionId);
        
        DECLSPEC_XFGVIRT(IWTSProtocolConnection, GetProtocolHandles)
        HRESULT ( STDMETHODCALLTYPE *GetProtocolHandles )( 
            __RPC__in IWTSProtocolConnection * This,
            /* [out] */ __RPC__out HANDLE_PTR *pKeyboardHandle,
            /* [out] */ __RPC__out HANDLE_PTR *pMouseHandle,
            /* [out] */ __RPC__out HANDLE_PTR *pBeepHandle,
            /* [out] */ __RPC__out HANDLE_PTR *pVideoHandle);
        
        DECLSPEC_XFGVIRT(IWTSProtocolConnection, ConnectNotify)
        HRESULT ( STDMETHODCALLTYPE *ConnectNotify )( 
            __RPC__in IWTSProtocolConnection * This,
            /* [in] */ ULONG SessionId);
        
        DECLSPEC_XFGVIRT(IWTSProtocolConnection, IsUserAllowedToLogon)
        HRESULT ( STDMETHODCALLTYPE *IsUserAllowedToLogon )( 
            __RPC__in IWTSProtocolConnection * This,
            /* [in] */ ULONG SessionId,
            /* [in] */ HANDLE_PTR UserToken,
            /* [string][in] */ __RPC__in_string WCHAR *pDomainName,
            /* [string][in] */ __RPC__in_string WCHAR *pUserName);
        
        DECLSPEC_XFGVIRT(IWTSProtocolConnection, SessionArbitrationEnumeration)
        HRESULT ( STDMETHODCALLTYPE *SessionArbitrationEnumeration )( 
            __RPC__in IWTSProtocolConnection * This,
            /* [in] */ HANDLE_PTR hUserToken,
            /* [in] */ BOOL bSingleSessionPerUserEnabled,
            /* [size_is][out] */ __RPC__out_ecount_full(*pdwSessionIdentifierCount) ULONG *pSessionIdArray,
            /* [out][in] */ __RPC__inout ULONG *pdwSessionIdentifierCount);
        
        DECLSPEC_XFGVIRT(IWTSProtocolConnection, LogonNotify)
        HRESULT ( STDMETHODCALLTYPE *LogonNotify )( 
            __RPC__in IWTSProtocolConnection * This,
            /* [in] */ HANDLE_PTR hClientToken,
            /* [string][in] */ __RPC__in_string WCHAR *wszUserName,
            /* [string][in] */ __RPC__in_string WCHAR *wszDomainName,
            /* [in] */ __RPC__in WTS_SESSION_ID *SessionId);
        
        DECLSPEC_XFGVIRT(IWTSProtocolConnection, GetUserData)
        HRESULT ( STDMETHODCALLTYPE *GetUserData )( 
            __RPC__in IWTSProtocolConnection * This,
            /* [in] */ __RPC__in WTS_POLICY_DATA *pPolicyData,
            /* [out][in] */ __RPC__inout WTS_USER_DATA *pClientData);
        
        DECLSPEC_XFGVIRT(IWTSProtocolConnection, DisconnectNotify)
        HRESULT ( STDMETHODCALLTYPE *DisconnectNotify )( 
            __RPC__in IWTSProtocolConnection * This);
        
        DECLSPEC_XFGVIRT(IWTSProtocolConnection, Close)
        HRESULT ( STDMETHODCALLTYPE *Close )( 
            __RPC__in IWTSProtocolConnection * This);
        
        DECLSPEC_XFGVIRT(IWTSProtocolConnection, GetProtocolStatus)
        HRESULT ( STDMETHODCALLTYPE *GetProtocolStatus )( 
            __RPC__in IWTSProtocolConnection * This,
            /* [out] */ __RPC__out WTS_PROTOCOL_STATUS *pProtocolStatus);
        
        DECLSPEC_XFGVIRT(IWTSProtocolConnection, GetLastInputTime)
        HRESULT ( STDMETHODCALLTYPE *GetLastInputTime )( 
            __RPC__in IWTSProtocolConnection * This,
            /* [out] */ __RPC__out ULONG64 *pLastInputTime);
        
        DECLSPEC_XFGVIRT(IWTSProtocolConnection, SetErrorInfo)
        HRESULT ( STDMETHODCALLTYPE *SetErrorInfo )( 
            __RPC__in IWTSProtocolConnection * This,
            /* [in] */ ULONG ulError);
        
        DECLSPEC_XFGVIRT(IWTSProtocolConnection, SendBeep)
        HRESULT ( STDMETHODCALLTYPE *SendBeep )( 
            __RPC__in IWTSProtocolConnection * This,
            /* [in] */ ULONG Frequency,
            /* [in] */ ULONG Duration);
        
        DECLSPEC_XFGVIRT(IWTSProtocolConnection, CreateVirtualChannel)
        HRESULT ( STDMETHODCALLTYPE *CreateVirtualChannel )( 
            __RPC__in IWTSProtocolConnection * This,
            /* [string][in] */ __RPC__in_string CHAR *szEndpointName,
            /* [in] */ BOOL bStatic,
            /* [in] */ ULONG RequestedPriority,
            /* [out] */ __RPC__out ULONG_PTR *phChannel);
        
        DECLSPEC_XFGVIRT(IWTSProtocolConnection, QueryProperty)
        HRESULT ( STDMETHODCALLTYPE *QueryProperty )( 
            __RPC__in IWTSProtocolConnection * This,
            /* [in] */ GUID QueryType,
            /* [in] */ ULONG ulNumEntriesIn,
            /* [in] */ ULONG ulNumEntriesOut,
            /* [size_is][optional][in] */ __RPC__in_ecount_full(ulNumEntriesIn) PWTS_PROPERTY_VALUE pPropertyEntriesIn,
            /* [size_is][optional][out] */ __RPC__out_ecount_full(ulNumEntriesOut) PWTS_PROPERTY_VALUE pPropertyEntriesOut);
        
        DECLSPEC_XFGVIRT(IWTSProtocolConnection, GetShadowConnection)
        HRESULT ( STDMETHODCALLTYPE *GetShadowConnection )( 
            __RPC__in IWTSProtocolConnection * This,
            /* [out] */ __RPC__deref_out_opt IWTSProtocolShadowConnection **ppShadowConnection);
        
        END_INTERFACE
    } IWTSProtocolConnectionVtbl;

    interface IWTSProtocolConnection
    {
        CONST_VTBL struct IWTSProtocolConnectionVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWTSProtocolConnection_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWTSProtocolConnection_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWTSProtocolConnection_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWTSProtocolConnection_GetLogonErrorRedirector(This,ppLogonErrorRedir)	\
    ( (This)->lpVtbl -> GetLogonErrorRedirector(This,ppLogonErrorRedir) ) 

#define IWTSProtocolConnection_SendPolicyData(This,pPolicyData)	\
    ( (This)->lpVtbl -> SendPolicyData(This,pPolicyData) ) 

#define IWTSProtocolConnection_AcceptConnection(This)	\
    ( (This)->lpVtbl -> AcceptConnection(This) ) 

#define IWTSProtocolConnection_GetClientData(This,pClientData)	\
    ( (This)->lpVtbl -> GetClientData(This,pClientData) ) 

#define IWTSProtocolConnection_GetUserCredentials(This,pUserCreds)	\
    ( (This)->lpVtbl -> GetUserCredentials(This,pUserCreds) ) 

#define IWTSProtocolConnection_GetLicenseConnection(This,ppLicenseConnection)	\
    ( (This)->lpVtbl -> GetLicenseConnection(This,ppLicenseConnection) ) 

#define IWTSProtocolConnection_AuthenticateClientToSession(This,SessionId)	\
    ( (This)->lpVtbl -> AuthenticateClientToSession(This,SessionId) ) 

#define IWTSProtocolConnection_NotifySessionId(This,SessionId)	\
    ( (This)->lpVtbl -> NotifySessionId(This,SessionId) ) 

#define IWTSProtocolConnection_GetProtocolHandles(This,pKeyboardHandle,pMouseHandle,pBeepHandle,pVideoHandle)	\
    ( (This)->lpVtbl -> GetProtocolHandles(This,pKeyboardHandle,pMouseHandle,pBeepHandle,pVideoHandle) ) 

#define IWTSProtocolConnection_ConnectNotify(This,SessionId)	\
    ( (This)->lpVtbl -> ConnectNotify(This,SessionId) ) 

#define IWTSProtocolConnection_IsUserAllowedToLogon(This,SessionId,UserToken,pDomainName,pUserName)	\
    ( (This)->lpVtbl -> IsUserAllowedToLogon(This,SessionId,UserToken,pDomainName,pUserName) ) 

#define IWTSProtocolConnection_SessionArbitrationEnumeration(This,hUserToken,bSingleSessionPerUserEnabled,pSessionIdArray,pdwSessionIdentifierCount)	\
    ( (This)->lpVtbl -> SessionArbitrationEnumeration(This,hUserToken,bSingleSessionPerUserEnabled,pSessionIdArray,pdwSessionIdentifierCount) ) 

#define IWTSProtocolConnection_LogonNotify(This,hClientToken,wszUserName,wszDomainName,SessionId)	\
    ( (This)->lpVtbl -> LogonNotify(This,hClientToken,wszUserName,wszDomainName,SessionId) ) 

#define IWTSProtocolConnection_GetUserData(This,pPolicyData,pClientData)	\
    ( (This)->lpVtbl -> GetUserData(This,pPolicyData,pClientData) ) 

#define IWTSProtocolConnection_DisconnectNotify(This)	\
    ( (This)->lpVtbl -> DisconnectNotify(This) ) 

#define IWTSProtocolConnection_Close(This)	\
    ( (This)->lpVtbl -> Close(This) ) 

#define IWTSProtocolConnection_GetProtocolStatus(This,pProtocolStatus)	\
    ( (This)->lpVtbl -> GetProtocolStatus(This,pProtocolStatus) ) 

#define IWTSProtocolConnection_GetLastInputTime(This,pLastInputTime)	\
    ( (This)->lpVtbl -> GetLastInputTime(This,pLastInputTime) ) 

#define IWTSProtocolConnection_SetErrorInfo(This,ulError)	\
    ( (This)->lpVtbl -> SetErrorInfo(This,ulError) ) 

#define IWTSProtocolConnection_SendBeep(This,Frequency,Duration)	\
    ( (This)->lpVtbl -> SendBeep(This,Frequency,Duration) ) 

#define IWTSProtocolConnection_CreateVirtualChannel(This,szEndpointName,bStatic,RequestedPriority,phChannel)	\
    ( (This)->lpVtbl -> CreateVirtualChannel(This,szEndpointName,bStatic,RequestedPriority,phChannel) ) 

#define IWTSProtocolConnection_QueryProperty(This,QueryType,ulNumEntriesIn,ulNumEntriesOut,pPropertyEntriesIn,pPropertyEntriesOut)	\
    ( (This)->lpVtbl -> QueryProperty(This,QueryType,ulNumEntriesIn,ulNumEntriesOut,pPropertyEntriesIn,pPropertyEntriesOut) ) 

#define IWTSProtocolConnection_GetShadowConnection(This,ppShadowConnection)	\
    ( (This)->lpVtbl -> GetShadowConnection(This,ppShadowConnection) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWTSProtocolConnection_INTERFACE_DEFINED__ */


#ifndef __IWTSProtocolConnectionCallback_INTERFACE_DEFINED__
#define __IWTSProtocolConnectionCallback_INTERFACE_DEFINED__

/* interface IWTSProtocolConnectionCallback */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IWTSProtocolConnectionCallback;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("23083765-75eb-41fe-b4fb-e086242afa0f")
    IWTSProtocolConnectionCallback : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE OnReady( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE BrokenConnection( 
            /* [in] */ ULONG Reason,
            /* [in] */ ULONG Source) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE StopScreenUpdates( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RedrawWindow( 
            /* [optional][in] */ __RPC__in WTS_SMALL_RECT *rect) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE DisplayIOCtl( 
            /* [in] */ __RPC__in WTS_DISPLAY_IOCTL *DisplayIOCtl) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWTSProtocolConnectionCallbackVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IWTSProtocolConnectionCallback * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IWTSProtocolConnectionCallback * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IWTSProtocolConnectionCallback * This);
        
        DECLSPEC_XFGVIRT(IWTSProtocolConnectionCallback, OnReady)
        HRESULT ( STDMETHODCALLTYPE *OnReady )( 
            __RPC__in IWTSProtocolConnectionCallback * This);
        
        DECLSPEC_XFGVIRT(IWTSProtocolConnectionCallback, BrokenConnection)
        HRESULT ( STDMETHODCALLTYPE *BrokenConnection )( 
            __RPC__in IWTSProtocolConnectionCallback * This,
            /* [in] */ ULONG Reason,
            /* [in] */ ULONG Source);
        
        DECLSPEC_XFGVIRT(IWTSProtocolConnectionCallback, StopScreenUpdates)
        HRESULT ( STDMETHODCALLTYPE *StopScreenUpdates )( 
            __RPC__in IWTSProtocolConnectionCallback * This);
        
        DECLSPEC_XFGVIRT(IWTSProtocolConnectionCallback, RedrawWindow)
        HRESULT ( STDMETHODCALLTYPE *RedrawWindow )( 
            __RPC__in IWTSProtocolConnectionCallback * This,
            /* [optional][in] */ __RPC__in WTS_SMALL_RECT *rect);
        
        DECLSPEC_XFGVIRT(IWTSProtocolConnectionCallback, DisplayIOCtl)
        HRESULT ( STDMETHODCALLTYPE *DisplayIOCtl )( 
            __RPC__in IWTSProtocolConnectionCallback * This,
            /* [in] */ __RPC__in WTS_DISPLAY_IOCTL *DisplayIOCtl);
        
        END_INTERFACE
    } IWTSProtocolConnectionCallbackVtbl;

    interface IWTSProtocolConnectionCallback
    {
        CONST_VTBL struct IWTSProtocolConnectionCallbackVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWTSProtocolConnectionCallback_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWTSProtocolConnectionCallback_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWTSProtocolConnectionCallback_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWTSProtocolConnectionCallback_OnReady(This)	\
    ( (This)->lpVtbl -> OnReady(This) ) 

#define IWTSProtocolConnectionCallback_BrokenConnection(This,Reason,Source)	\
    ( (This)->lpVtbl -> BrokenConnection(This,Reason,Source) ) 

#define IWTSProtocolConnectionCallback_StopScreenUpdates(This)	\
    ( (This)->lpVtbl -> StopScreenUpdates(This) ) 

#define IWTSProtocolConnectionCallback_RedrawWindow(This,rect)	\
    ( (This)->lpVtbl -> RedrawWindow(This,rect) ) 

#define IWTSProtocolConnectionCallback_DisplayIOCtl(This,DisplayIOCtl)	\
    ( (This)->lpVtbl -> DisplayIOCtl(This,DisplayIOCtl) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWTSProtocolConnectionCallback_INTERFACE_DEFINED__ */


#ifndef __IWTSProtocolShadowConnection_INTERFACE_DEFINED__
#define __IWTSProtocolShadowConnection_INTERFACE_DEFINED__

/* interface IWTSProtocolShadowConnection */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IWTSProtocolShadowConnection;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("ee3b0c14-37fb-456b-bab3-6d6cd51e13bf")
    IWTSProtocolShadowConnection : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Start( 
            /* [string][in] */ __RPC__in_string WCHAR *pTargetServerName,
            /* [in] */ DWORD TargetSessionId,
            /* [in] */ BYTE HotKeyVk,
            /* [in] */ USHORT HotkeyModifiers,
            /* [in] */ __RPC__in_opt IWTSProtocolShadowCallback *pShadowCallback) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Stop( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE DoTarget( 
            /* [size_is][in] */ __RPC__in_ecount_full(Param1Size) PBYTE pParam1,
            /* [in] */ DWORD Param1Size,
            /* [size_is][in] */ __RPC__in_ecount_full(Param2Size) PBYTE pParam2,
            /* [in] */ DWORD Param2Size,
            /* [size_is][in] */ __RPC__in_ecount_full(Param3Size) PBYTE pParam3,
            /* [in] */ DWORD Param3Size,
            /* [size_is][in] */ __RPC__in_ecount_full(Param4Size) PBYTE pParam4,
            /* [in] */ DWORD Param4Size,
            /* [string][in] */ __RPC__in_string WCHAR *pClientName) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWTSProtocolShadowConnectionVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IWTSProtocolShadowConnection * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IWTSProtocolShadowConnection * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IWTSProtocolShadowConnection * This);
        
        DECLSPEC_XFGVIRT(IWTSProtocolShadowConnection, Start)
        HRESULT ( STDMETHODCALLTYPE *Start )( 
            __RPC__in IWTSProtocolShadowConnection * This,
            /* [string][in] */ __RPC__in_string WCHAR *pTargetServerName,
            /* [in] */ DWORD TargetSessionId,
            /* [in] */ BYTE HotKeyVk,
            /* [in] */ USHORT HotkeyModifiers,
            /* [in] */ __RPC__in_opt IWTSProtocolShadowCallback *pShadowCallback);
        
        DECLSPEC_XFGVIRT(IWTSProtocolShadowConnection, Stop)
        HRESULT ( STDMETHODCALLTYPE *Stop )( 
            __RPC__in IWTSProtocolShadowConnection * This);
        
        DECLSPEC_XFGVIRT(IWTSProtocolShadowConnection, DoTarget)
        HRESULT ( STDMETHODCALLTYPE *DoTarget )( 
            __RPC__in IWTSProtocolShadowConnection * This,
            /* [size_is][in] */ __RPC__in_ecount_full(Param1Size) PBYTE pParam1,
            /* [in] */ DWORD Param1Size,
            /* [size_is][in] */ __RPC__in_ecount_full(Param2Size) PBYTE pParam2,
            /* [in] */ DWORD Param2Size,
            /* [size_is][in] */ __RPC__in_ecount_full(Param3Size) PBYTE pParam3,
            /* [in] */ DWORD Param3Size,
            /* [size_is][in] */ __RPC__in_ecount_full(Param4Size) PBYTE pParam4,
            /* [in] */ DWORD Param4Size,
            /* [string][in] */ __RPC__in_string WCHAR *pClientName);
        
        END_INTERFACE
    } IWTSProtocolShadowConnectionVtbl;

    interface IWTSProtocolShadowConnection
    {
        CONST_VTBL struct IWTSProtocolShadowConnectionVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWTSProtocolShadowConnection_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWTSProtocolShadowConnection_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWTSProtocolShadowConnection_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWTSProtocolShadowConnection_Start(This,pTargetServerName,TargetSessionId,HotKeyVk,HotkeyModifiers,pShadowCallback)	\
    ( (This)->lpVtbl -> Start(This,pTargetServerName,TargetSessionId,HotKeyVk,HotkeyModifiers,pShadowCallback) ) 

#define IWTSProtocolShadowConnection_Stop(This)	\
    ( (This)->lpVtbl -> Stop(This) ) 

#define IWTSProtocolShadowConnection_DoTarget(This,pParam1,Param1Size,pParam2,Param2Size,pParam3,Param3Size,pParam4,Param4Size,pClientName)	\
    ( (This)->lpVtbl -> DoTarget(This,pParam1,Param1Size,pParam2,Param2Size,pParam3,Param3Size,pParam4,Param4Size,pClientName) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWTSProtocolShadowConnection_INTERFACE_DEFINED__ */


#ifndef __IWTSProtocolShadowCallback_INTERFACE_DEFINED__
#define __IWTSProtocolShadowCallback_INTERFACE_DEFINED__

/* interface IWTSProtocolShadowCallback */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IWTSProtocolShadowCallback;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("503a2504-aae5-4ab1-93e0-6d1c4bc6f71a")
    IWTSProtocolShadowCallback : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE StopShadow( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE InvokeTargetShadow( 
            /* [string][full][in] */ __RPC__in_opt_string WCHAR *pTargetServerName,
            /* [in] */ ULONG TargetSessionId,
            /* [size_is][in] */ __RPC__in_ecount_full(Param1Size) PBYTE pParam1,
            /* [in] */ DWORD Param1Size,
            /* [size_is][in] */ __RPC__in_ecount_full(Param2Size) PBYTE pParam2,
            /* [in] */ DWORD Param2Size,
            /* [size_is][in] */ __RPC__in_ecount_full(Param3Size) PBYTE pParam3,
            /* [in] */ DWORD Param3Size,
            /* [size_is][in] */ __RPC__in_ecount_full(Param4Size) PBYTE pParam4,
            /* [in] */ DWORD Param4Size,
            /* [string][in] */ __RPC__in_string WCHAR *pClientName) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWTSProtocolShadowCallbackVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IWTSProtocolShadowCallback * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IWTSProtocolShadowCallback * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IWTSProtocolShadowCallback * This);
        
        DECLSPEC_XFGVIRT(IWTSProtocolShadowCallback, StopShadow)
        HRESULT ( STDMETHODCALLTYPE *StopShadow )( 
            __RPC__in IWTSProtocolShadowCallback * This);
        
        DECLSPEC_XFGVIRT(IWTSProtocolShadowCallback, InvokeTargetShadow)
        HRESULT ( STDMETHODCALLTYPE *InvokeTargetShadow )( 
            __RPC__in IWTSProtocolShadowCallback * This,
            /* [string][full][in] */ __RPC__in_opt_string WCHAR *pTargetServerName,
            /* [in] */ ULONG TargetSessionId,
            /* [size_is][in] */ __RPC__in_ecount_full(Param1Size) PBYTE pParam1,
            /* [in] */ DWORD Param1Size,
            /* [size_is][in] */ __RPC__in_ecount_full(Param2Size) PBYTE pParam2,
            /* [in] */ DWORD Param2Size,
            /* [size_is][in] */ __RPC__in_ecount_full(Param3Size) PBYTE pParam3,
            /* [in] */ DWORD Param3Size,
            /* [size_is][in] */ __RPC__in_ecount_full(Param4Size) PBYTE pParam4,
            /* [in] */ DWORD Param4Size,
            /* [string][in] */ __RPC__in_string WCHAR *pClientName);
        
        END_INTERFACE
    } IWTSProtocolShadowCallbackVtbl;

    interface IWTSProtocolShadowCallback
    {
        CONST_VTBL struct IWTSProtocolShadowCallbackVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWTSProtocolShadowCallback_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWTSProtocolShadowCallback_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWTSProtocolShadowCallback_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWTSProtocolShadowCallback_StopShadow(This)	\
    ( (This)->lpVtbl -> StopShadow(This) ) 

#define IWTSProtocolShadowCallback_InvokeTargetShadow(This,pTargetServerName,TargetSessionId,pParam1,Param1Size,pParam2,Param2Size,pParam3,Param3Size,pParam4,Param4Size,pClientName)	\
    ( (This)->lpVtbl -> InvokeTargetShadow(This,pTargetServerName,TargetSessionId,pParam1,Param1Size,pParam2,Param2Size,pParam3,Param3Size,pParam4,Param4Size,pClientName) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWTSProtocolShadowCallback_INTERFACE_DEFINED__ */


#ifndef __IWTSProtocolLicenseConnection_INTERFACE_DEFINED__
#define __IWTSProtocolLicenseConnection_INTERFACE_DEFINED__

/* interface IWTSProtocolLicenseConnection */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IWTSProtocolLicenseConnection;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("23083765-178c-4079-8e4a-fea6496a4d70")
    IWTSProtocolLicenseConnection : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE RequestLicensingCapabilities( 
            /* [out] */ __RPC__out PWTS_LICENSE_CAPABILITIES ppLicenseCapabilities,
            /* [out][in] */ __RPC__inout ULONG *pcbLicenseCapabilities) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SendClientLicense( 
            /* [size_is][in] */ __RPC__in_ecount_full(cbClientLicense) PBYTE pClientLicense,
            /* [in] */ ULONG cbClientLicense) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RequestClientLicense( 
            /* [size_is][in] */ __RPC__in_ecount_full(Reserve2) PBYTE Reserve1,
            /* [in] */ ULONG Reserve2,
            /* [size_is][out] */ __RPC__out_ecount_full(*pcbClientLicense) PBYTE ppClientLicense,
            /* [out][in] */ __RPC__inout ULONG *pcbClientLicense) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ProtocolComplete( 
            /* [in] */ ULONG ulComplete) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWTSProtocolLicenseConnectionVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IWTSProtocolLicenseConnection * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IWTSProtocolLicenseConnection * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IWTSProtocolLicenseConnection * This);
        
        DECLSPEC_XFGVIRT(IWTSProtocolLicenseConnection, RequestLicensingCapabilities)
        HRESULT ( STDMETHODCALLTYPE *RequestLicensingCapabilities )( 
            __RPC__in IWTSProtocolLicenseConnection * This,
            /* [out] */ __RPC__out PWTS_LICENSE_CAPABILITIES ppLicenseCapabilities,
            /* [out][in] */ __RPC__inout ULONG *pcbLicenseCapabilities);
        
        DECLSPEC_XFGVIRT(IWTSProtocolLicenseConnection, SendClientLicense)
        HRESULT ( STDMETHODCALLTYPE *SendClientLicense )( 
            __RPC__in IWTSProtocolLicenseConnection * This,
            /* [size_is][in] */ __RPC__in_ecount_full(cbClientLicense) PBYTE pClientLicense,
            /* [in] */ ULONG cbClientLicense);
        
        DECLSPEC_XFGVIRT(IWTSProtocolLicenseConnection, RequestClientLicense)
        HRESULT ( STDMETHODCALLTYPE *RequestClientLicense )( 
            __RPC__in IWTSProtocolLicenseConnection * This,
            /* [size_is][in] */ __RPC__in_ecount_full(Reserve2) PBYTE Reserve1,
            /* [in] */ ULONG Reserve2,
            /* [size_is][out] */ __RPC__out_ecount_full(*pcbClientLicense) PBYTE ppClientLicense,
            /* [out][in] */ __RPC__inout ULONG *pcbClientLicense);
        
        DECLSPEC_XFGVIRT(IWTSProtocolLicenseConnection, ProtocolComplete)
        HRESULT ( STDMETHODCALLTYPE *ProtocolComplete )( 
            __RPC__in IWTSProtocolLicenseConnection * This,
            /* [in] */ ULONG ulComplete);
        
        END_INTERFACE
    } IWTSProtocolLicenseConnectionVtbl;

    interface IWTSProtocolLicenseConnection
    {
        CONST_VTBL struct IWTSProtocolLicenseConnectionVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWTSProtocolLicenseConnection_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWTSProtocolLicenseConnection_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWTSProtocolLicenseConnection_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWTSProtocolLicenseConnection_RequestLicensingCapabilities(This,ppLicenseCapabilities,pcbLicenseCapabilities)	\
    ( (This)->lpVtbl -> RequestLicensingCapabilities(This,ppLicenseCapabilities,pcbLicenseCapabilities) ) 

#define IWTSProtocolLicenseConnection_SendClientLicense(This,pClientLicense,cbClientLicense)	\
    ( (This)->lpVtbl -> SendClientLicense(This,pClientLicense,cbClientLicense) ) 

#define IWTSProtocolLicenseConnection_RequestClientLicense(This,Reserve1,Reserve2,ppClientLicense,pcbClientLicense)	\
    ( (This)->lpVtbl -> RequestClientLicense(This,Reserve1,Reserve2,ppClientLicense,pcbClientLicense) ) 

#define IWTSProtocolLicenseConnection_ProtocolComplete(This,ulComplete)	\
    ( (This)->lpVtbl -> ProtocolComplete(This,ulComplete) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWTSProtocolLicenseConnection_INTERFACE_DEFINED__ */


#ifndef __IWTSProtocolLogonErrorRedirector_INTERFACE_DEFINED__
#define __IWTSProtocolLogonErrorRedirector_INTERFACE_DEFINED__

/* interface IWTSProtocolLogonErrorRedirector */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IWTSProtocolLogonErrorRedirector;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("FD9B61A7-2916-4627-8DEE-4328711AD6CB")
    IWTSProtocolLogonErrorRedirector : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE OnBeginPainting( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RedirectStatus( 
            /* [string][in] */ __RPC__in_string LPCWSTR pszMessage,
            /* [out] */ __RPC__out WTS_LOGON_ERROR_REDIRECTOR_RESPONSE *pResponse) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RedirectMessage( 
            /* [string][in] */ __RPC__in_string LPCWSTR pszCaption,
            /* [string][in] */ __RPC__in_string LPCWSTR pszMessage,
            /* [in] */ UINT uType,
            /* [out] */ __RPC__out WTS_LOGON_ERROR_REDIRECTOR_RESPONSE *pResponse) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RedirectLogonError( 
            /* [in] */ LONG ntsStatus,
            /* [in] */ LONG ntsSubstatus,
            /* [string][in] */ __RPC__in_string LPCWSTR pszCaption,
            /* [string][in] */ __RPC__in_string LPCWSTR pszMessage,
            /* [in] */ UINT uType,
            /* [out] */ __RPC__out WTS_LOGON_ERROR_REDIRECTOR_RESPONSE *pResponse) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWTSProtocolLogonErrorRedirectorVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IWTSProtocolLogonErrorRedirector * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IWTSProtocolLogonErrorRedirector * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IWTSProtocolLogonErrorRedirector * This);
        
        DECLSPEC_XFGVIRT(IWTSProtocolLogonErrorRedirector, OnBeginPainting)
        HRESULT ( STDMETHODCALLTYPE *OnBeginPainting )( 
            __RPC__in IWTSProtocolLogonErrorRedirector * This);
        
        DECLSPEC_XFGVIRT(IWTSProtocolLogonErrorRedirector, RedirectStatus)
        HRESULT ( STDMETHODCALLTYPE *RedirectStatus )( 
            __RPC__in IWTSProtocolLogonErrorRedirector * This,
            /* [string][in] */ __RPC__in_string LPCWSTR pszMessage,
            /* [out] */ __RPC__out WTS_LOGON_ERROR_REDIRECTOR_RESPONSE *pResponse);
        
        DECLSPEC_XFGVIRT(IWTSProtocolLogonErrorRedirector, RedirectMessage)
        HRESULT ( STDMETHODCALLTYPE *RedirectMessage )( 
            __RPC__in IWTSProtocolLogonErrorRedirector * This,
            /* [string][in] */ __RPC__in_string LPCWSTR pszCaption,
            /* [string][in] */ __RPC__in_string LPCWSTR pszMessage,
            /* [in] */ UINT uType,
            /* [out] */ __RPC__out WTS_LOGON_ERROR_REDIRECTOR_RESPONSE *pResponse);
        
        DECLSPEC_XFGVIRT(IWTSProtocolLogonErrorRedirector, RedirectLogonError)
        HRESULT ( STDMETHODCALLTYPE *RedirectLogonError )( 
            __RPC__in IWTSProtocolLogonErrorRedirector * This,
            /* [in] */ LONG ntsStatus,
            /* [in] */ LONG ntsSubstatus,
            /* [string][in] */ __RPC__in_string LPCWSTR pszCaption,
            /* [string][in] */ __RPC__in_string LPCWSTR pszMessage,
            /* [in] */ UINT uType,
            /* [out] */ __RPC__out WTS_LOGON_ERROR_REDIRECTOR_RESPONSE *pResponse);
        
        END_INTERFACE
    } IWTSProtocolLogonErrorRedirectorVtbl;

    interface IWTSProtocolLogonErrorRedirector
    {
        CONST_VTBL struct IWTSProtocolLogonErrorRedirectorVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWTSProtocolLogonErrorRedirector_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWTSProtocolLogonErrorRedirector_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWTSProtocolLogonErrorRedirector_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWTSProtocolLogonErrorRedirector_OnBeginPainting(This)	\
    ( (This)->lpVtbl -> OnBeginPainting(This) ) 

#define IWTSProtocolLogonErrorRedirector_RedirectStatus(This,pszMessage,pResponse)	\
    ( (This)->lpVtbl -> RedirectStatus(This,pszMessage,pResponse) ) 

#define IWTSProtocolLogonErrorRedirector_RedirectMessage(This,pszCaption,pszMessage,uType,pResponse)	\
    ( (This)->lpVtbl -> RedirectMessage(This,pszCaption,pszMessage,uType,pResponse) ) 

#define IWTSProtocolLogonErrorRedirector_RedirectLogonError(This,ntsStatus,ntsSubstatus,pszCaption,pszMessage,uType,pResponse)	\
    ( (This)->lpVtbl -> RedirectLogonError(This,ntsStatus,ntsSubstatus,pszCaption,pszMessage,uType,pResponse) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWTSProtocolLogonErrorRedirector_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_wtsprotocol_0000_0009 */
/* [local] */ 













extern RPC_IF_HANDLE __MIDL_itf_wtsprotocol_0000_0009_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_wtsprotocol_0000_0009_v0_0_s_ifspec;

#ifndef __IWRdsProtocolSettings_INTERFACE_DEFINED__
#define __IWRdsProtocolSettings_INTERFACE_DEFINED__

/* interface IWRdsProtocolSettings */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IWRdsProtocolSettings;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("654A5A6A-2550-47EB-B6F7-EBD637475265")
    IWRdsProtocolSettings : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetSettings( 
            /* [in] */ WRDS_SETTING_TYPE WRdsSettingType,
            /* [in] */ WRDS_SETTING_LEVEL WRdsSettingLevel,
            /* [ref][out] */ __RPC__out PWRDS_SETTINGS pWRdsSettings) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE MergeSettings( 
            /* [ref][in] */ __RPC__in PWRDS_SETTINGS pWRdsSettings,
            /* [in] */ WRDS_CONNECTION_SETTING_LEVEL WRdsConnectionSettingLevel,
            /* [ref][out][in] */ __RPC__inout PWRDS_CONNECTION_SETTINGS pWRdsConnectionSettings) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWRdsProtocolSettingsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IWRdsProtocolSettings * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IWRdsProtocolSettings * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IWRdsProtocolSettings * This);
        
        DECLSPEC_XFGVIRT(IWRdsProtocolSettings, GetSettings)
        HRESULT ( STDMETHODCALLTYPE *GetSettings )( 
            __RPC__in IWRdsProtocolSettings * This,
            /* [in] */ WRDS_SETTING_TYPE WRdsSettingType,
            /* [in] */ WRDS_SETTING_LEVEL WRdsSettingLevel,
            /* [ref][out] */ __RPC__out PWRDS_SETTINGS pWRdsSettings);
        
        DECLSPEC_XFGVIRT(IWRdsProtocolSettings, MergeSettings)
        HRESULT ( STDMETHODCALLTYPE *MergeSettings )( 
            __RPC__in IWRdsProtocolSettings * This,
            /* [ref][in] */ __RPC__in PWRDS_SETTINGS pWRdsSettings,
            /* [in] */ WRDS_CONNECTION_SETTING_LEVEL WRdsConnectionSettingLevel,
            /* [ref][out][in] */ __RPC__inout PWRDS_CONNECTION_SETTINGS pWRdsConnectionSettings);
        
        END_INTERFACE
    } IWRdsProtocolSettingsVtbl;

    interface IWRdsProtocolSettings
    {
        CONST_VTBL struct IWRdsProtocolSettingsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWRdsProtocolSettings_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWRdsProtocolSettings_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWRdsProtocolSettings_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWRdsProtocolSettings_GetSettings(This,WRdsSettingType,WRdsSettingLevel,pWRdsSettings)	\
    ( (This)->lpVtbl -> GetSettings(This,WRdsSettingType,WRdsSettingLevel,pWRdsSettings) ) 

#define IWRdsProtocolSettings_MergeSettings(This,pWRdsSettings,WRdsConnectionSettingLevel,pWRdsConnectionSettings)	\
    ( (This)->lpVtbl -> MergeSettings(This,pWRdsSettings,WRdsConnectionSettingLevel,pWRdsConnectionSettings) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWRdsProtocolSettings_INTERFACE_DEFINED__ */


#ifndef __IWRdsProtocolManager_INTERFACE_DEFINED__
#define __IWRdsProtocolManager_INTERFACE_DEFINED__

/* interface IWRdsProtocolManager */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IWRdsProtocolManager;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("DC796967-3ABB-40CD-A446-105276B58950")
    IWRdsProtocolManager : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Initialize( 
            /* [in] */ __RPC__in_opt IWRdsProtocolSettings *pIWRdsSettings,
            /* [in] */ __RPC__in PWRDS_SETTINGS pWRdsSettings) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreateListener( 
            /* [string][in] */ __RPC__in_string WCHAR *wszListenerName,
            /* [out] */ __RPC__deref_out_opt IWRdsProtocolListener **pProtocolListener) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE NotifyServiceStateChange( 
            /* [in] */ __RPC__in WRDS_SERVICE_STATE *pTSServiceStateChange) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE NotifySessionOfServiceStart( 
            /* [in] */ __RPC__in WRDS_SESSION_ID *SessionId) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE NotifySessionOfServiceStop( 
            /* [in] */ __RPC__in WRDS_SESSION_ID *SessionId) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE NotifySessionStateChange( 
            /* [in] */ __RPC__in WRDS_SESSION_ID *SessionId,
            /* [in] */ ULONG EventId) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE NotifySettingsChange( 
            /* [in] */ __RPC__in PWRDS_SETTINGS pWRdsSettings) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Uninitialize( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWRdsProtocolManagerVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IWRdsProtocolManager * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IWRdsProtocolManager * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IWRdsProtocolManager * This);
        
        DECLSPEC_XFGVIRT(IWRdsProtocolManager, Initialize)
        HRESULT ( STDMETHODCALLTYPE *Initialize )( 
            __RPC__in IWRdsProtocolManager * This,
            /* [in] */ __RPC__in_opt IWRdsProtocolSettings *pIWRdsSettings,
            /* [in] */ __RPC__in PWRDS_SETTINGS pWRdsSettings);
        
        DECLSPEC_XFGVIRT(IWRdsProtocolManager, CreateListener)
        HRESULT ( STDMETHODCALLTYPE *CreateListener )( 
            __RPC__in IWRdsProtocolManager * This,
            /* [string][in] */ __RPC__in_string WCHAR *wszListenerName,
            /* [out] */ __RPC__deref_out_opt IWRdsProtocolListener **pProtocolListener);
        
        DECLSPEC_XFGVIRT(IWRdsProtocolManager, NotifyServiceStateChange)
        HRESULT ( STDMETHODCALLTYPE *NotifyServiceStateChange )( 
            __RPC__in IWRdsProtocolManager * This,
            /* [in] */ __RPC__in WRDS_SERVICE_STATE *pTSServiceStateChange);
        
        DECLSPEC_XFGVIRT(IWRdsProtocolManager, NotifySessionOfServiceStart)
        HRESULT ( STDMETHODCALLTYPE *NotifySessionOfServiceStart )( 
            __RPC__in IWRdsProtocolManager * This,
            /* [in] */ __RPC__in WRDS_SESSION_ID *SessionId);
        
        DECLSPEC_XFGVIRT(IWRdsProtocolManager, NotifySessionOfServiceStop)
        HRESULT ( STDMETHODCALLTYPE *NotifySessionOfServiceStop )( 
            __RPC__in IWRdsProtocolManager * This,
            /* [in] */ __RPC__in WRDS_SESSION_ID *SessionId);
        
        DECLSPEC_XFGVIRT(IWRdsProtocolManager, NotifySessionStateChange)
        HRESULT ( STDMETHODCALLTYPE *NotifySessionStateChange )( 
            __RPC__in IWRdsProtocolManager * This,
            /* [in] */ __RPC__in WRDS_SESSION_ID *SessionId,
            /* [in] */ ULONG EventId);
        
        DECLSPEC_XFGVIRT(IWRdsProtocolManager, NotifySettingsChange)
        HRESULT ( STDMETHODCALLTYPE *NotifySettingsChange )( 
            __RPC__in IWRdsProtocolManager * This,
            /* [in] */ __RPC__in PWRDS_SETTINGS pWRdsSettings);
        
        DECLSPEC_XFGVIRT(IWRdsProtocolManager, Uninitialize)
        HRESULT ( STDMETHODCALLTYPE *Uninitialize )( 
            __RPC__in IWRdsProtocolManager * This);
        
        END_INTERFACE
    } IWRdsProtocolManagerVtbl;

    interface IWRdsProtocolManager
    {
        CONST_VTBL struct IWRdsProtocolManagerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWRdsProtocolManager_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWRdsProtocolManager_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWRdsProtocolManager_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWRdsProtocolManager_Initialize(This,pIWRdsSettings,pWRdsSettings)	\
    ( (This)->lpVtbl -> Initialize(This,pIWRdsSettings,pWRdsSettings) ) 

#define IWRdsProtocolManager_CreateListener(This,wszListenerName,pProtocolListener)	\
    ( (This)->lpVtbl -> CreateListener(This,wszListenerName,pProtocolListener) ) 

#define IWRdsProtocolManager_NotifyServiceStateChange(This,pTSServiceStateChange)	\
    ( (This)->lpVtbl -> NotifyServiceStateChange(This,pTSServiceStateChange) ) 

#define IWRdsProtocolManager_NotifySessionOfServiceStart(This,SessionId)	\
    ( (This)->lpVtbl -> NotifySessionOfServiceStart(This,SessionId) ) 

#define IWRdsProtocolManager_NotifySessionOfServiceStop(This,SessionId)	\
    ( (This)->lpVtbl -> NotifySessionOfServiceStop(This,SessionId) ) 

#define IWRdsProtocolManager_NotifySessionStateChange(This,SessionId,EventId)	\
    ( (This)->lpVtbl -> NotifySessionStateChange(This,SessionId,EventId) ) 

#define IWRdsProtocolManager_NotifySettingsChange(This,pWRdsSettings)	\
    ( (This)->lpVtbl -> NotifySettingsChange(This,pWRdsSettings) ) 

#define IWRdsProtocolManager_Uninitialize(This)	\
    ( (This)->lpVtbl -> Uninitialize(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWRdsProtocolManager_INTERFACE_DEFINED__ */


#ifndef __IWRdsProtocolListener_INTERFACE_DEFINED__
#define __IWRdsProtocolListener_INTERFACE_DEFINED__

/* interface IWRdsProtocolListener */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IWRdsProtocolListener;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("FCBC131B-C686-451D-A773-E279E230F540")
    IWRdsProtocolListener : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetSettings( 
            /* [in] */ WRDS_LISTENER_SETTING_LEVEL WRdsListenerSettingLevel,
            /* [ref][out] */ __RPC__out PWRDS_LISTENER_SETTINGS pWRdsListenerSettings) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE StartListen( 
            /* [in] */ __RPC__in_opt IWRdsProtocolListenerCallback *pCallback) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE StopListen( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWRdsProtocolListenerVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IWRdsProtocolListener * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IWRdsProtocolListener * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IWRdsProtocolListener * This);
        
        DECLSPEC_XFGVIRT(IWRdsProtocolListener, GetSettings)
        HRESULT ( STDMETHODCALLTYPE *GetSettings )( 
            __RPC__in IWRdsProtocolListener * This,
            /* [in] */ WRDS_LISTENER_SETTING_LEVEL WRdsListenerSettingLevel,
            /* [ref][out] */ __RPC__out PWRDS_LISTENER_SETTINGS pWRdsListenerSettings);
        
        DECLSPEC_XFGVIRT(IWRdsProtocolListener, StartListen)
        HRESULT ( STDMETHODCALLTYPE *StartListen )( 
            __RPC__in IWRdsProtocolListener * This,
            /* [in] */ __RPC__in_opt IWRdsProtocolListenerCallback *pCallback);
        
        DECLSPEC_XFGVIRT(IWRdsProtocolListener, StopListen)
        HRESULT ( STDMETHODCALLTYPE *StopListen )( 
            __RPC__in IWRdsProtocolListener * This);
        
        END_INTERFACE
    } IWRdsProtocolListenerVtbl;

    interface IWRdsProtocolListener
    {
        CONST_VTBL struct IWRdsProtocolListenerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWRdsProtocolListener_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWRdsProtocolListener_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWRdsProtocolListener_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWRdsProtocolListener_GetSettings(This,WRdsListenerSettingLevel,pWRdsListenerSettings)	\
    ( (This)->lpVtbl -> GetSettings(This,WRdsListenerSettingLevel,pWRdsListenerSettings) ) 

#define IWRdsProtocolListener_StartListen(This,pCallback)	\
    ( (This)->lpVtbl -> StartListen(This,pCallback) ) 

#define IWRdsProtocolListener_StopListen(This)	\
    ( (This)->lpVtbl -> StopListen(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWRdsProtocolListener_INTERFACE_DEFINED__ */


#ifndef __IWRdsProtocolListenerCallback_INTERFACE_DEFINED__
#define __IWRdsProtocolListenerCallback_INTERFACE_DEFINED__

/* interface IWRdsProtocolListenerCallback */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IWRdsProtocolListenerCallback;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("3AB27E5B-4449-4DC1-B74A-91621D4FE984")
    IWRdsProtocolListenerCallback : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE OnConnected( 
            /* [in] */ __RPC__in_opt IWRdsProtocolConnection *pConnection,
            /* [in] */ __RPC__in PWRDS_CONNECTION_SETTINGS pWRdsConnectionSettings,
            /* [out] */ __RPC__deref_out_opt IWRdsProtocolConnectionCallback **pCallback) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWRdsProtocolListenerCallbackVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IWRdsProtocolListenerCallback * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IWRdsProtocolListenerCallback * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IWRdsProtocolListenerCallback * This);
        
        DECLSPEC_XFGVIRT(IWRdsProtocolListenerCallback, OnConnected)
        HRESULT ( STDMETHODCALLTYPE *OnConnected )( 
            __RPC__in IWRdsProtocolListenerCallback * This,
            /* [in] */ __RPC__in_opt IWRdsProtocolConnection *pConnection,
            /* [in] */ __RPC__in PWRDS_CONNECTION_SETTINGS pWRdsConnectionSettings,
            /* [out] */ __RPC__deref_out_opt IWRdsProtocolConnectionCallback **pCallback);
        
        END_INTERFACE
    } IWRdsProtocolListenerCallbackVtbl;

    interface IWRdsProtocolListenerCallback
    {
        CONST_VTBL struct IWRdsProtocolListenerCallbackVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWRdsProtocolListenerCallback_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWRdsProtocolListenerCallback_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWRdsProtocolListenerCallback_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWRdsProtocolListenerCallback_OnConnected(This,pConnection,pWRdsConnectionSettings,pCallback)	\
    ( (This)->lpVtbl -> OnConnected(This,pConnection,pWRdsConnectionSettings,pCallback) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWRdsProtocolListenerCallback_INTERFACE_DEFINED__ */


#ifndef __IWRdsProtocolConnection_INTERFACE_DEFINED__
#define __IWRdsProtocolConnection_INTERFACE_DEFINED__

/* interface IWRdsProtocolConnection */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IWRdsProtocolConnection;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("324ED94F-FDAF-4FF6-81A8-42ABE755830B")
    IWRdsProtocolConnection : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetLogonErrorRedirector( 
            /* [out] */ __RPC__deref_out_opt IWRdsProtocolLogonErrorRedirector **ppLogonErrorRedir) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE AcceptConnection( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetClientData( 
            /* [out] */ __RPC__out WRDS_CLIENT_DATA *pClientData) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetClientMonitorData( 
            /* [out] */ __RPC__out UINT *pNumMonitors,
            /* [out] */ __RPC__out UINT *pPrimaryMonitor) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetUserCredentials( 
            /* [out] */ __RPC__out WRDS_USER_CREDENTIAL *pUserCreds) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetLicenseConnection( 
            /* [out] */ __RPC__deref_out_opt IWRdsProtocolLicenseConnection **ppLicenseConnection) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE AuthenticateClientToSession( 
            /* [out] */ __RPC__out WRDS_SESSION_ID *SessionId) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE NotifySessionId( 
            /* [in] */ __RPC__in WRDS_SESSION_ID *SessionId,
            /* [in] */ HANDLE_PTR SessionHandle) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetInputHandles( 
            /* [out] */ __RPC__out HANDLE_PTR *pKeyboardHandle,
            /* [out] */ __RPC__out HANDLE_PTR *pMouseHandle,
            /* [out] */ __RPC__out HANDLE_PTR *pBeepHandle) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetVideoHandle( 
            /* [out] */ __RPC__out HANDLE_PTR *pVideoHandle) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ConnectNotify( 
            /* [in] */ ULONG SessionId) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE IsUserAllowedToLogon( 
            /* [in] */ ULONG SessionId,
            /* [in] */ HANDLE_PTR UserToken,
            /* [string][in] */ __RPC__in_string WCHAR *pDomainName,
            /* [string][in] */ __RPC__in_string WCHAR *pUserName) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SessionArbitrationEnumeration( 
            /* [in] */ HANDLE_PTR hUserToken,
            /* [in] */ BOOL bSingleSessionPerUserEnabled,
            /* [size_is][out] */ __RPC__out_ecount_full(*pdwSessionIdentifierCount) ULONG *pSessionIdArray,
            /* [out][in] */ __RPC__inout ULONG *pdwSessionIdentifierCount) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE LogonNotify( 
            /* [in] */ HANDLE_PTR hClientToken,
            /* [string][in] */ __RPC__in_string WCHAR *wszUserName,
            /* [string][in] */ __RPC__in_string WCHAR *wszDomainName,
            /* [in] */ __RPC__in WRDS_SESSION_ID *SessionId,
            /* [out][in] */ __RPC__inout PWRDS_CONNECTION_SETTINGS pWRdsConnectionSettings) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE PreDisconnect( 
            /* [in] */ ULONG DisconnectReason) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE DisconnectNotify( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Close( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetProtocolStatus( 
            /* [out] */ __RPC__out WRDS_PROTOCOL_STATUS *pProtocolStatus) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetLastInputTime( 
            /* [out] */ __RPC__out ULONG64 *pLastInputTime) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetErrorInfo( 
            /* [in] */ ULONG ulError) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreateVirtualChannel( 
            /* [string][in] */ __RPC__in_string CHAR *szEndpointName,
            /* [in] */ BOOL bStatic,
            /* [in] */ ULONG RequestedPriority,
            /* [out] */ __RPC__out ULONG_PTR *phChannel) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE QueryProperty( 
            /* [in] */ GUID QueryType,
            /* [in] */ ULONG ulNumEntriesIn,
            /* [in] */ ULONG ulNumEntriesOut,
            /* [size_is][optional][in] */ __RPC__in_ecount_full(ulNumEntriesIn) PWRDS_PROPERTY_VALUE pPropertyEntriesIn,
            /* [size_is][optional][out] */ __RPC__out_ecount_full(ulNumEntriesOut) PWRDS_PROPERTY_VALUE pPropertyEntriesOut) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetShadowConnection( 
            /* [out] */ __RPC__deref_out_opt IWRdsProtocolShadowConnection **ppShadowConnection) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE NotifyCommandProcessCreated( 
            /* [in] */ ULONG SessionId) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWRdsProtocolConnectionVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IWRdsProtocolConnection * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IWRdsProtocolConnection * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IWRdsProtocolConnection * This);
        
        DECLSPEC_XFGVIRT(IWRdsProtocolConnection, GetLogonErrorRedirector)
        HRESULT ( STDMETHODCALLTYPE *GetLogonErrorRedirector )( 
            __RPC__in IWRdsProtocolConnection * This,
            /* [out] */ __RPC__deref_out_opt IWRdsProtocolLogonErrorRedirector **ppLogonErrorRedir);
        
        DECLSPEC_XFGVIRT(IWRdsProtocolConnection, AcceptConnection)
        HRESULT ( STDMETHODCALLTYPE *AcceptConnection )( 
            __RPC__in IWRdsProtocolConnection * This);
        
        DECLSPEC_XFGVIRT(IWRdsProtocolConnection, GetClientData)
        HRESULT ( STDMETHODCALLTYPE *GetClientData )( 
            __RPC__in IWRdsProtocolConnection * This,
            /* [out] */ __RPC__out WRDS_CLIENT_DATA *pClientData);
        
        DECLSPEC_XFGVIRT(IWRdsProtocolConnection, GetClientMonitorData)
        HRESULT ( STDMETHODCALLTYPE *GetClientMonitorData )( 
            __RPC__in IWRdsProtocolConnection * This,
            /* [out] */ __RPC__out UINT *pNumMonitors,
            /* [out] */ __RPC__out UINT *pPrimaryMonitor);
        
        DECLSPEC_XFGVIRT(IWRdsProtocolConnection, GetUserCredentials)
        HRESULT ( STDMETHODCALLTYPE *GetUserCredentials )( 
            __RPC__in IWRdsProtocolConnection * This,
            /* [out] */ __RPC__out WRDS_USER_CREDENTIAL *pUserCreds);
        
        DECLSPEC_XFGVIRT(IWRdsProtocolConnection, GetLicenseConnection)
        HRESULT ( STDMETHODCALLTYPE *GetLicenseConnection )( 
            __RPC__in IWRdsProtocolConnection * This,
            /* [out] */ __RPC__deref_out_opt IWRdsProtocolLicenseConnection **ppLicenseConnection);
        
        DECLSPEC_XFGVIRT(IWRdsProtocolConnection, AuthenticateClientToSession)
        HRESULT ( STDMETHODCALLTYPE *AuthenticateClientToSession )( 
            __RPC__in IWRdsProtocolConnection * This,
            /* [out] */ __RPC__out WRDS_SESSION_ID *SessionId);
        
        DECLSPEC_XFGVIRT(IWRdsProtocolConnection, NotifySessionId)
        HRESULT ( STDMETHODCALLTYPE *NotifySessionId )( 
            __RPC__in IWRdsProtocolConnection * This,
            /* [in] */ __RPC__in WRDS_SESSION_ID *SessionId,
            /* [in] */ HANDLE_PTR SessionHandle);
        
        DECLSPEC_XFGVIRT(IWRdsProtocolConnection, GetInputHandles)
        HRESULT ( STDMETHODCALLTYPE *GetInputHandles )( 
            __RPC__in IWRdsProtocolConnection * This,
            /* [out] */ __RPC__out HANDLE_PTR *pKeyboardHandle,
            /* [out] */ __RPC__out HANDLE_PTR *pMouseHandle,
            /* [out] */ __RPC__out HANDLE_PTR *pBeepHandle);
        
        DECLSPEC_XFGVIRT(IWRdsProtocolConnection, GetVideoHandle)
        HRESULT ( STDMETHODCALLTYPE *GetVideoHandle )( 
            __RPC__in IWRdsProtocolConnection * This,
            /* [out] */ __RPC__out HANDLE_PTR *pVideoHandle);
        
        DECLSPEC_XFGVIRT(IWRdsProtocolConnection, ConnectNotify)
        HRESULT ( STDMETHODCALLTYPE *ConnectNotify )( 
            __RPC__in IWRdsProtocolConnection * This,
            /* [in] */ ULONG SessionId);
        
        DECLSPEC_XFGVIRT(IWRdsProtocolConnection, IsUserAllowedToLogon)
        HRESULT ( STDMETHODCALLTYPE *IsUserAllowedToLogon )( 
            __RPC__in IWRdsProtocolConnection * This,
            /* [in] */ ULONG SessionId,
            /* [in] */ HANDLE_PTR UserToken,
            /* [string][in] */ __RPC__in_string WCHAR *pDomainName,
            /* [string][in] */ __RPC__in_string WCHAR *pUserName);
        
        DECLSPEC_XFGVIRT(IWRdsProtocolConnection, SessionArbitrationEnumeration)
        HRESULT ( STDMETHODCALLTYPE *SessionArbitrationEnumeration )( 
            __RPC__in IWRdsProtocolConnection * This,
            /* [in] */ HANDLE_PTR hUserToken,
            /* [in] */ BOOL bSingleSessionPerUserEnabled,
            /* [size_is][out] */ __RPC__out_ecount_full(*pdwSessionIdentifierCount) ULONG *pSessionIdArray,
            /* [out][in] */ __RPC__inout ULONG *pdwSessionIdentifierCount);
        
        DECLSPEC_XFGVIRT(IWRdsProtocolConnection, LogonNotify)
        HRESULT ( STDMETHODCALLTYPE *LogonNotify )( 
            __RPC__in IWRdsProtocolConnection * This,
            /* [in] */ HANDLE_PTR hClientToken,
            /* [string][in] */ __RPC__in_string WCHAR *wszUserName,
            /* [string][in] */ __RPC__in_string WCHAR *wszDomainName,
            /* [in] */ __RPC__in WRDS_SESSION_ID *SessionId,
            /* [out][in] */ __RPC__inout PWRDS_CONNECTION_SETTINGS pWRdsConnectionSettings);
        
        DECLSPEC_XFGVIRT(IWRdsProtocolConnection, PreDisconnect)
        HRESULT ( STDMETHODCALLTYPE *PreDisconnect )( 
            __RPC__in IWRdsProtocolConnection * This,
            /* [in] */ ULONG DisconnectReason);
        
        DECLSPEC_XFGVIRT(IWRdsProtocolConnection, DisconnectNotify)
        HRESULT ( STDMETHODCALLTYPE *DisconnectNotify )( 
            __RPC__in IWRdsProtocolConnection * This);
        
        DECLSPEC_XFGVIRT(IWRdsProtocolConnection, Close)
        HRESULT ( STDMETHODCALLTYPE *Close )( 
            __RPC__in IWRdsProtocolConnection * This);
        
        DECLSPEC_XFGVIRT(IWRdsProtocolConnection, GetProtocolStatus)
        HRESULT ( STDMETHODCALLTYPE *GetProtocolStatus )( 
            __RPC__in IWRdsProtocolConnection * This,
            /* [out] */ __RPC__out WRDS_PROTOCOL_STATUS *pProtocolStatus);
        
        DECLSPEC_XFGVIRT(IWRdsProtocolConnection, GetLastInputTime)
        HRESULT ( STDMETHODCALLTYPE *GetLastInputTime )( 
            __RPC__in IWRdsProtocolConnection * This,
            /* [out] */ __RPC__out ULONG64 *pLastInputTime);
        
        DECLSPEC_XFGVIRT(IWRdsProtocolConnection, SetErrorInfo)
        HRESULT ( STDMETHODCALLTYPE *SetErrorInfo )( 
            __RPC__in IWRdsProtocolConnection * This,
            /* [in] */ ULONG ulError);
        
        DECLSPEC_XFGVIRT(IWRdsProtocolConnection, CreateVirtualChannel)
        HRESULT ( STDMETHODCALLTYPE *CreateVirtualChannel )( 
            __RPC__in IWRdsProtocolConnection * This,
            /* [string][in] */ __RPC__in_string CHAR *szEndpointName,
            /* [in] */ BOOL bStatic,
            /* [in] */ ULONG RequestedPriority,
            /* [out] */ __RPC__out ULONG_PTR *phChannel);
        
        DECLSPEC_XFGVIRT(IWRdsProtocolConnection, QueryProperty)
        HRESULT ( STDMETHODCALLTYPE *QueryProperty )( 
            __RPC__in IWRdsProtocolConnection * This,
            /* [in] */ GUID QueryType,
            /* [in] */ ULONG ulNumEntriesIn,
            /* [in] */ ULONG ulNumEntriesOut,
            /* [size_is][optional][in] */ __RPC__in_ecount_full(ulNumEntriesIn) PWRDS_PROPERTY_VALUE pPropertyEntriesIn,
            /* [size_is][optional][out] */ __RPC__out_ecount_full(ulNumEntriesOut) PWRDS_PROPERTY_VALUE pPropertyEntriesOut);
        
        DECLSPEC_XFGVIRT(IWRdsProtocolConnection, GetShadowConnection)
        HRESULT ( STDMETHODCALLTYPE *GetShadowConnection )( 
            __RPC__in IWRdsProtocolConnection * This,
            /* [out] */ __RPC__deref_out_opt IWRdsProtocolShadowConnection **ppShadowConnection);
        
        DECLSPEC_XFGVIRT(IWRdsProtocolConnection, NotifyCommandProcessCreated)
        HRESULT ( STDMETHODCALLTYPE *NotifyCommandProcessCreated )( 
            __RPC__in IWRdsProtocolConnection * This,
            /* [in] */ ULONG SessionId);
        
        END_INTERFACE
    } IWRdsProtocolConnectionVtbl;

    interface IWRdsProtocolConnection
    {
        CONST_VTBL struct IWRdsProtocolConnectionVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWRdsProtocolConnection_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWRdsProtocolConnection_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWRdsProtocolConnection_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWRdsProtocolConnection_GetLogonErrorRedirector(This,ppLogonErrorRedir)	\
    ( (This)->lpVtbl -> GetLogonErrorRedirector(This,ppLogonErrorRedir) ) 

#define IWRdsProtocolConnection_AcceptConnection(This)	\
    ( (This)->lpVtbl -> AcceptConnection(This) ) 

#define IWRdsProtocolConnection_GetClientData(This,pClientData)	\
    ( (This)->lpVtbl -> GetClientData(This,pClientData) ) 

#define IWRdsProtocolConnection_GetClientMonitorData(This,pNumMonitors,pPrimaryMonitor)	\
    ( (This)->lpVtbl -> GetClientMonitorData(This,pNumMonitors,pPrimaryMonitor) ) 

#define IWRdsProtocolConnection_GetUserCredentials(This,pUserCreds)	\
    ( (This)->lpVtbl -> GetUserCredentials(This,pUserCreds) ) 

#define IWRdsProtocolConnection_GetLicenseConnection(This,ppLicenseConnection)	\
    ( (This)->lpVtbl -> GetLicenseConnection(This,ppLicenseConnection) ) 

#define IWRdsProtocolConnection_AuthenticateClientToSession(This,SessionId)	\
    ( (This)->lpVtbl -> AuthenticateClientToSession(This,SessionId) ) 

#define IWRdsProtocolConnection_NotifySessionId(This,SessionId,SessionHandle)	\
    ( (This)->lpVtbl -> NotifySessionId(This,SessionId,SessionHandle) ) 

#define IWRdsProtocolConnection_GetInputHandles(This,pKeyboardHandle,pMouseHandle,pBeepHandle)	\
    ( (This)->lpVtbl -> GetInputHandles(This,pKeyboardHandle,pMouseHandle,pBeepHandle) ) 

#define IWRdsProtocolConnection_GetVideoHandle(This,pVideoHandle)	\
    ( (This)->lpVtbl -> GetVideoHandle(This,pVideoHandle) ) 

#define IWRdsProtocolConnection_ConnectNotify(This,SessionId)	\
    ( (This)->lpVtbl -> ConnectNotify(This,SessionId) ) 

#define IWRdsProtocolConnection_IsUserAllowedToLogon(This,SessionId,UserToken,pDomainName,pUserName)	\
    ( (This)->lpVtbl -> IsUserAllowedToLogon(This,SessionId,UserToken,pDomainName,pUserName) ) 

#define IWRdsProtocolConnection_SessionArbitrationEnumeration(This,hUserToken,bSingleSessionPerUserEnabled,pSessionIdArray,pdwSessionIdentifierCount)	\
    ( (This)->lpVtbl -> SessionArbitrationEnumeration(This,hUserToken,bSingleSessionPerUserEnabled,pSessionIdArray,pdwSessionIdentifierCount) ) 

#define IWRdsProtocolConnection_LogonNotify(This,hClientToken,wszUserName,wszDomainName,SessionId,pWRdsConnectionSettings)	\
    ( (This)->lpVtbl -> LogonNotify(This,hClientToken,wszUserName,wszDomainName,SessionId,pWRdsConnectionSettings) ) 

#define IWRdsProtocolConnection_PreDisconnect(This,DisconnectReason)	\
    ( (This)->lpVtbl -> PreDisconnect(This,DisconnectReason) ) 

#define IWRdsProtocolConnection_DisconnectNotify(This)	\
    ( (This)->lpVtbl -> DisconnectNotify(This) ) 

#define IWRdsProtocolConnection_Close(This)	\
    ( (This)->lpVtbl -> Close(This) ) 

#define IWRdsProtocolConnection_GetProtocolStatus(This,pProtocolStatus)	\
    ( (This)->lpVtbl -> GetProtocolStatus(This,pProtocolStatus) ) 

#define IWRdsProtocolConnection_GetLastInputTime(This,pLastInputTime)	\
    ( (This)->lpVtbl -> GetLastInputTime(This,pLastInputTime) ) 

#define IWRdsProtocolConnection_SetErrorInfo(This,ulError)	\
    ( (This)->lpVtbl -> SetErrorInfo(This,ulError) ) 

#define IWRdsProtocolConnection_CreateVirtualChannel(This,szEndpointName,bStatic,RequestedPriority,phChannel)	\
    ( (This)->lpVtbl -> CreateVirtualChannel(This,szEndpointName,bStatic,RequestedPriority,phChannel) ) 

#define IWRdsProtocolConnection_QueryProperty(This,QueryType,ulNumEntriesIn,ulNumEntriesOut,pPropertyEntriesIn,pPropertyEntriesOut)	\
    ( (This)->lpVtbl -> QueryProperty(This,QueryType,ulNumEntriesIn,ulNumEntriesOut,pPropertyEntriesIn,pPropertyEntriesOut) ) 

#define IWRdsProtocolConnection_GetShadowConnection(This,ppShadowConnection)	\
    ( (This)->lpVtbl -> GetShadowConnection(This,ppShadowConnection) ) 

#define IWRdsProtocolConnection_NotifyCommandProcessCreated(This,SessionId)	\
    ( (This)->lpVtbl -> NotifyCommandProcessCreated(This,SessionId) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWRdsProtocolConnection_INTERFACE_DEFINED__ */


#ifndef __IWRdsProtocolConnectionCallback_INTERFACE_DEFINED__
#define __IWRdsProtocolConnectionCallback_INTERFACE_DEFINED__

/* interface IWRdsProtocolConnectionCallback */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IWRdsProtocolConnectionCallback;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("F1D70332-D070-4EF1-A088-78313536C2D6")
    IWRdsProtocolConnectionCallback : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE OnReady( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE BrokenConnection( 
            /* [in] */ ULONG Reason,
            /* [in] */ ULONG Source) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE StopScreenUpdates( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RedrawWindow( 
            /* [optional][in] */ __RPC__in WRDS_SMALL_RECT *rect) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetConnectionId( 
            /* [out] */ __RPC__out ULONG *pConnectionId) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWRdsProtocolConnectionCallbackVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IWRdsProtocolConnectionCallback * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IWRdsProtocolConnectionCallback * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IWRdsProtocolConnectionCallback * This);
        
        DECLSPEC_XFGVIRT(IWRdsProtocolConnectionCallback, OnReady)
        HRESULT ( STDMETHODCALLTYPE *OnReady )( 
            __RPC__in IWRdsProtocolConnectionCallback * This);
        
        DECLSPEC_XFGVIRT(IWRdsProtocolConnectionCallback, BrokenConnection)
        HRESULT ( STDMETHODCALLTYPE *BrokenConnection )( 
            __RPC__in IWRdsProtocolConnectionCallback * This,
            /* [in] */ ULONG Reason,
            /* [in] */ ULONG Source);
        
        DECLSPEC_XFGVIRT(IWRdsProtocolConnectionCallback, StopScreenUpdates)
        HRESULT ( STDMETHODCALLTYPE *StopScreenUpdates )( 
            __RPC__in IWRdsProtocolConnectionCallback * This);
        
        DECLSPEC_XFGVIRT(IWRdsProtocolConnectionCallback, RedrawWindow)
        HRESULT ( STDMETHODCALLTYPE *RedrawWindow )( 
            __RPC__in IWRdsProtocolConnectionCallback * This,
            /* [optional][in] */ __RPC__in WRDS_SMALL_RECT *rect);
        
        DECLSPEC_XFGVIRT(IWRdsProtocolConnectionCallback, GetConnectionId)
        HRESULT ( STDMETHODCALLTYPE *GetConnectionId )( 
            __RPC__in IWRdsProtocolConnectionCallback * This,
            /* [out] */ __RPC__out ULONG *pConnectionId);
        
        END_INTERFACE
    } IWRdsProtocolConnectionCallbackVtbl;

    interface IWRdsProtocolConnectionCallback
    {
        CONST_VTBL struct IWRdsProtocolConnectionCallbackVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWRdsProtocolConnectionCallback_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWRdsProtocolConnectionCallback_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWRdsProtocolConnectionCallback_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWRdsProtocolConnectionCallback_OnReady(This)	\
    ( (This)->lpVtbl -> OnReady(This) ) 

#define IWRdsProtocolConnectionCallback_BrokenConnection(This,Reason,Source)	\
    ( (This)->lpVtbl -> BrokenConnection(This,Reason,Source) ) 

#define IWRdsProtocolConnectionCallback_StopScreenUpdates(This)	\
    ( (This)->lpVtbl -> StopScreenUpdates(This) ) 

#define IWRdsProtocolConnectionCallback_RedrawWindow(This,rect)	\
    ( (This)->lpVtbl -> RedrawWindow(This,rect) ) 

#define IWRdsProtocolConnectionCallback_GetConnectionId(This,pConnectionId)	\
    ( (This)->lpVtbl -> GetConnectionId(This,pConnectionId) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWRdsProtocolConnectionCallback_INTERFACE_DEFINED__ */


#ifndef __IWRdsProtocolShadowConnection_INTERFACE_DEFINED__
#define __IWRdsProtocolShadowConnection_INTERFACE_DEFINED__

/* interface IWRdsProtocolShadowConnection */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IWRdsProtocolShadowConnection;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("9AE85CE6-CADE-4548-8FEB-99016597F60A")
    IWRdsProtocolShadowConnection : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Start( 
            /* [string][in] */ __RPC__in_string WCHAR *pTargetServerName,
            /* [in] */ DWORD TargetSessionId,
            /* [in] */ BYTE HotKeyVk,
            /* [in] */ USHORT HotkeyModifiers,
            /* [in] */ __RPC__in_opt IWRdsProtocolShadowCallback *pShadowCallback) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Stop( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE DoTarget( 
            /* [size_is][in] */ __RPC__in_ecount_full(Param1Size) PBYTE pParam1,
            /* [in] */ DWORD Param1Size,
            /* [size_is][in] */ __RPC__in_ecount_full(Param2Size) PBYTE pParam2,
            /* [in] */ DWORD Param2Size,
            /* [size_is][in] */ __RPC__in_ecount_full(Param3Size) PBYTE pParam3,
            /* [in] */ DWORD Param3Size,
            /* [size_is][in] */ __RPC__in_ecount_full(Param4Size) PBYTE pParam4,
            /* [in] */ DWORD Param4Size,
            /* [string][in] */ __RPC__in_string WCHAR *pClientName) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWRdsProtocolShadowConnectionVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IWRdsProtocolShadowConnection * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IWRdsProtocolShadowConnection * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IWRdsProtocolShadowConnection * This);
        
        DECLSPEC_XFGVIRT(IWRdsProtocolShadowConnection, Start)
        HRESULT ( STDMETHODCALLTYPE *Start )( 
            __RPC__in IWRdsProtocolShadowConnection * This,
            /* [string][in] */ __RPC__in_string WCHAR *pTargetServerName,
            /* [in] */ DWORD TargetSessionId,
            /* [in] */ BYTE HotKeyVk,
            /* [in] */ USHORT HotkeyModifiers,
            /* [in] */ __RPC__in_opt IWRdsProtocolShadowCallback *pShadowCallback);
        
        DECLSPEC_XFGVIRT(IWRdsProtocolShadowConnection, Stop)
        HRESULT ( STDMETHODCALLTYPE *Stop )( 
            __RPC__in IWRdsProtocolShadowConnection * This);
        
        DECLSPEC_XFGVIRT(IWRdsProtocolShadowConnection, DoTarget)
        HRESULT ( STDMETHODCALLTYPE *DoTarget )( 
            __RPC__in IWRdsProtocolShadowConnection * This,
            /* [size_is][in] */ __RPC__in_ecount_full(Param1Size) PBYTE pParam1,
            /* [in] */ DWORD Param1Size,
            /* [size_is][in] */ __RPC__in_ecount_full(Param2Size) PBYTE pParam2,
            /* [in] */ DWORD Param2Size,
            /* [size_is][in] */ __RPC__in_ecount_full(Param3Size) PBYTE pParam3,
            /* [in] */ DWORD Param3Size,
            /* [size_is][in] */ __RPC__in_ecount_full(Param4Size) PBYTE pParam4,
            /* [in] */ DWORD Param4Size,
            /* [string][in] */ __RPC__in_string WCHAR *pClientName);
        
        END_INTERFACE
    } IWRdsProtocolShadowConnectionVtbl;

    interface IWRdsProtocolShadowConnection
    {
        CONST_VTBL struct IWRdsProtocolShadowConnectionVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWRdsProtocolShadowConnection_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWRdsProtocolShadowConnection_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWRdsProtocolShadowConnection_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWRdsProtocolShadowConnection_Start(This,pTargetServerName,TargetSessionId,HotKeyVk,HotkeyModifiers,pShadowCallback)	\
    ( (This)->lpVtbl -> Start(This,pTargetServerName,TargetSessionId,HotKeyVk,HotkeyModifiers,pShadowCallback) ) 

#define IWRdsProtocolShadowConnection_Stop(This)	\
    ( (This)->lpVtbl -> Stop(This) ) 

#define IWRdsProtocolShadowConnection_DoTarget(This,pParam1,Param1Size,pParam2,Param2Size,pParam3,Param3Size,pParam4,Param4Size,pClientName)	\
    ( (This)->lpVtbl -> DoTarget(This,pParam1,Param1Size,pParam2,Param2Size,pParam3,Param3Size,pParam4,Param4Size,pClientName) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWRdsProtocolShadowConnection_INTERFACE_DEFINED__ */


#ifndef __IWRdsProtocolShadowCallback_INTERFACE_DEFINED__
#define __IWRdsProtocolShadowCallback_INTERFACE_DEFINED__

/* interface IWRdsProtocolShadowCallback */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IWRdsProtocolShadowCallback;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("E0667CE0-0372-40D6-ADB2-A0F3322674D6")
    IWRdsProtocolShadowCallback : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE StopShadow( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE InvokeTargetShadow( 
            /* [string][full][in] */ __RPC__in_opt_string WCHAR *pTargetServerName,
            /* [in] */ ULONG TargetSessionId,
            /* [size_is][in] */ __RPC__in_ecount_full(Param1Size) PBYTE pParam1,
            /* [in] */ DWORD Param1Size,
            /* [size_is][in] */ __RPC__in_ecount_full(Param2Size) PBYTE pParam2,
            /* [in] */ DWORD Param2Size,
            /* [size_is][in] */ __RPC__in_ecount_full(Param3Size) PBYTE pParam3,
            /* [in] */ DWORD Param3Size,
            /* [size_is][in] */ __RPC__in_ecount_full(Param4Size) PBYTE pParam4,
            /* [in] */ DWORD Param4Size,
            /* [string][in] */ __RPC__in_string WCHAR *pClientName) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWRdsProtocolShadowCallbackVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IWRdsProtocolShadowCallback * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IWRdsProtocolShadowCallback * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IWRdsProtocolShadowCallback * This);
        
        DECLSPEC_XFGVIRT(IWRdsProtocolShadowCallback, StopShadow)
        HRESULT ( STDMETHODCALLTYPE *StopShadow )( 
            __RPC__in IWRdsProtocolShadowCallback * This);
        
        DECLSPEC_XFGVIRT(IWRdsProtocolShadowCallback, InvokeTargetShadow)
        HRESULT ( STDMETHODCALLTYPE *InvokeTargetShadow )( 
            __RPC__in IWRdsProtocolShadowCallback * This,
            /* [string][full][in] */ __RPC__in_opt_string WCHAR *pTargetServerName,
            /* [in] */ ULONG TargetSessionId,
            /* [size_is][in] */ __RPC__in_ecount_full(Param1Size) PBYTE pParam1,
            /* [in] */ DWORD Param1Size,
            /* [size_is][in] */ __RPC__in_ecount_full(Param2Size) PBYTE pParam2,
            /* [in] */ DWORD Param2Size,
            /* [size_is][in] */ __RPC__in_ecount_full(Param3Size) PBYTE pParam3,
            /* [in] */ DWORD Param3Size,
            /* [size_is][in] */ __RPC__in_ecount_full(Param4Size) PBYTE pParam4,
            /* [in] */ DWORD Param4Size,
            /* [string][in] */ __RPC__in_string WCHAR *pClientName);
        
        END_INTERFACE
    } IWRdsProtocolShadowCallbackVtbl;

    interface IWRdsProtocolShadowCallback
    {
        CONST_VTBL struct IWRdsProtocolShadowCallbackVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWRdsProtocolShadowCallback_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWRdsProtocolShadowCallback_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWRdsProtocolShadowCallback_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWRdsProtocolShadowCallback_StopShadow(This)	\
    ( (This)->lpVtbl -> StopShadow(This) ) 

#define IWRdsProtocolShadowCallback_InvokeTargetShadow(This,pTargetServerName,TargetSessionId,pParam1,Param1Size,pParam2,Param2Size,pParam3,Param3Size,pParam4,Param4Size,pClientName)	\
    ( (This)->lpVtbl -> InvokeTargetShadow(This,pTargetServerName,TargetSessionId,pParam1,Param1Size,pParam2,Param2Size,pParam3,Param3Size,pParam4,Param4Size,pClientName) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWRdsProtocolShadowCallback_INTERFACE_DEFINED__ */


#ifndef __IWRdsProtocolLicenseConnection_INTERFACE_DEFINED__
#define __IWRdsProtocolLicenseConnection_INTERFACE_DEFINED__

/* interface IWRdsProtocolLicenseConnection */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IWRdsProtocolLicenseConnection;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("1D6A145F-D095-4424-957A-407FAE822D84")
    IWRdsProtocolLicenseConnection : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE RequestLicensingCapabilities( 
            /* [out] */ __RPC__out PWRDS_LICENSE_CAPABILITIES ppLicenseCapabilities,
            /* [out][in] */ __RPC__inout ULONG *pcbLicenseCapabilities) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SendClientLicense( 
            /* [size_is][in] */ __RPC__in_ecount_full(cbClientLicense) PBYTE pClientLicense,
            /* [in] */ ULONG cbClientLicense) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RequestClientLicense( 
            /* [size_is][in] */ __RPC__in_ecount_full(Reserve2) PBYTE Reserve1,
            /* [in] */ ULONG Reserve2,
            /* [size_is][out] */ __RPC__out_ecount_full(*pcbClientLicense) PBYTE ppClientLicense,
            /* [out][in] */ __RPC__inout ULONG *pcbClientLicense) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ProtocolComplete( 
            /* [in] */ ULONG ulComplete) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWRdsProtocolLicenseConnectionVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IWRdsProtocolLicenseConnection * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IWRdsProtocolLicenseConnection * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IWRdsProtocolLicenseConnection * This);
        
        DECLSPEC_XFGVIRT(IWRdsProtocolLicenseConnection, RequestLicensingCapabilities)
        HRESULT ( STDMETHODCALLTYPE *RequestLicensingCapabilities )( 
            __RPC__in IWRdsProtocolLicenseConnection * This,
            /* [out] */ __RPC__out PWRDS_LICENSE_CAPABILITIES ppLicenseCapabilities,
            /* [out][in] */ __RPC__inout ULONG *pcbLicenseCapabilities);
        
        DECLSPEC_XFGVIRT(IWRdsProtocolLicenseConnection, SendClientLicense)
        HRESULT ( STDMETHODCALLTYPE *SendClientLicense )( 
            __RPC__in IWRdsProtocolLicenseConnection * This,
            /* [size_is][in] */ __RPC__in_ecount_full(cbClientLicense) PBYTE pClientLicense,
            /* [in] */ ULONG cbClientLicense);
        
        DECLSPEC_XFGVIRT(IWRdsProtocolLicenseConnection, RequestClientLicense)
        HRESULT ( STDMETHODCALLTYPE *RequestClientLicense )( 
            __RPC__in IWRdsProtocolLicenseConnection * This,
            /* [size_is][in] */ __RPC__in_ecount_full(Reserve2) PBYTE Reserve1,
            /* [in] */ ULONG Reserve2,
            /* [size_is][out] */ __RPC__out_ecount_full(*pcbClientLicense) PBYTE ppClientLicense,
            /* [out][in] */ __RPC__inout ULONG *pcbClientLicense);
        
        DECLSPEC_XFGVIRT(IWRdsProtocolLicenseConnection, ProtocolComplete)
        HRESULT ( STDMETHODCALLTYPE *ProtocolComplete )( 
            __RPC__in IWRdsProtocolLicenseConnection * This,
            /* [in] */ ULONG ulComplete);
        
        END_INTERFACE
    } IWRdsProtocolLicenseConnectionVtbl;

    interface IWRdsProtocolLicenseConnection
    {
        CONST_VTBL struct IWRdsProtocolLicenseConnectionVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWRdsProtocolLicenseConnection_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWRdsProtocolLicenseConnection_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWRdsProtocolLicenseConnection_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWRdsProtocolLicenseConnection_RequestLicensingCapabilities(This,ppLicenseCapabilities,pcbLicenseCapabilities)	\
    ( (This)->lpVtbl -> RequestLicensingCapabilities(This,ppLicenseCapabilities,pcbLicenseCapabilities) ) 

#define IWRdsProtocolLicenseConnection_SendClientLicense(This,pClientLicense,cbClientLicense)	\
    ( (This)->lpVtbl -> SendClientLicense(This,pClientLicense,cbClientLicense) ) 

#define IWRdsProtocolLicenseConnection_RequestClientLicense(This,Reserve1,Reserve2,ppClientLicense,pcbClientLicense)	\
    ( (This)->lpVtbl -> RequestClientLicense(This,Reserve1,Reserve2,ppClientLicense,pcbClientLicense) ) 

#define IWRdsProtocolLicenseConnection_ProtocolComplete(This,ulComplete)	\
    ( (This)->lpVtbl -> ProtocolComplete(This,ulComplete) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWRdsProtocolLicenseConnection_INTERFACE_DEFINED__ */


#ifndef __IWRdsProtocolLogonErrorRedirector_INTERFACE_DEFINED__
#define __IWRdsProtocolLogonErrorRedirector_INTERFACE_DEFINED__

/* interface IWRdsProtocolLogonErrorRedirector */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IWRdsProtocolLogonErrorRedirector;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("519FE83B-142A-4120-A3D5-A405D315281A")
    IWRdsProtocolLogonErrorRedirector : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE OnBeginPainting( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RedirectStatus( 
            /* [string][in] */ __RPC__in_string LPCWSTR pszMessage,
            /* [out] */ __RPC__out WRDS_LOGON_ERROR_REDIRECTOR_RESPONSE *pResponse) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RedirectMessage( 
            /* [string][in] */ __RPC__in_string LPCWSTR pszCaption,
            /* [string][in] */ __RPC__in_string LPCWSTR pszMessage,
            /* [in] */ UINT uType,
            /* [out] */ __RPC__out WRDS_LOGON_ERROR_REDIRECTOR_RESPONSE *pResponse) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RedirectLogonError( 
            /* [in] */ LONG ntsStatus,
            /* [in] */ LONG ntsSubstatus,
            /* [string][in] */ __RPC__in_string LPCWSTR pszCaption,
            /* [string][in] */ __RPC__in_string LPCWSTR pszMessage,
            /* [in] */ UINT uType,
            /* [out] */ __RPC__out WRDS_LOGON_ERROR_REDIRECTOR_RESPONSE *pResponse) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWRdsProtocolLogonErrorRedirectorVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IWRdsProtocolLogonErrorRedirector * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IWRdsProtocolLogonErrorRedirector * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IWRdsProtocolLogonErrorRedirector * This);
        
        DECLSPEC_XFGVIRT(IWRdsProtocolLogonErrorRedirector, OnBeginPainting)
        HRESULT ( STDMETHODCALLTYPE *OnBeginPainting )( 
            __RPC__in IWRdsProtocolLogonErrorRedirector * This);
        
        DECLSPEC_XFGVIRT(IWRdsProtocolLogonErrorRedirector, RedirectStatus)
        HRESULT ( STDMETHODCALLTYPE *RedirectStatus )( 
            __RPC__in IWRdsProtocolLogonErrorRedirector * This,
            /* [string][in] */ __RPC__in_string LPCWSTR pszMessage,
            /* [out] */ __RPC__out WRDS_LOGON_ERROR_REDIRECTOR_RESPONSE *pResponse);
        
        DECLSPEC_XFGVIRT(IWRdsProtocolLogonErrorRedirector, RedirectMessage)
        HRESULT ( STDMETHODCALLTYPE *RedirectMessage )( 
            __RPC__in IWRdsProtocolLogonErrorRedirector * This,
            /* [string][in] */ __RPC__in_string LPCWSTR pszCaption,
            /* [string][in] */ __RPC__in_string LPCWSTR pszMessage,
            /* [in] */ UINT uType,
            /* [out] */ __RPC__out WRDS_LOGON_ERROR_REDIRECTOR_RESPONSE *pResponse);
        
        DECLSPEC_XFGVIRT(IWRdsProtocolLogonErrorRedirector, RedirectLogonError)
        HRESULT ( STDMETHODCALLTYPE *RedirectLogonError )( 
            __RPC__in IWRdsProtocolLogonErrorRedirector * This,
            /* [in] */ LONG ntsStatus,
            /* [in] */ LONG ntsSubstatus,
            /* [string][in] */ __RPC__in_string LPCWSTR pszCaption,
            /* [string][in] */ __RPC__in_string LPCWSTR pszMessage,
            /* [in] */ UINT uType,
            /* [out] */ __RPC__out WRDS_LOGON_ERROR_REDIRECTOR_RESPONSE *pResponse);
        
        END_INTERFACE
    } IWRdsProtocolLogonErrorRedirectorVtbl;

    interface IWRdsProtocolLogonErrorRedirector
    {
        CONST_VTBL struct IWRdsProtocolLogonErrorRedirectorVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWRdsProtocolLogonErrorRedirector_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWRdsProtocolLogonErrorRedirector_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWRdsProtocolLogonErrorRedirector_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWRdsProtocolLogonErrorRedirector_OnBeginPainting(This)	\
    ( (This)->lpVtbl -> OnBeginPainting(This) ) 

#define IWRdsProtocolLogonErrorRedirector_RedirectStatus(This,pszMessage,pResponse)	\
    ( (This)->lpVtbl -> RedirectStatus(This,pszMessage,pResponse) ) 

#define IWRdsProtocolLogonErrorRedirector_RedirectMessage(This,pszCaption,pszMessage,uType,pResponse)	\
    ( (This)->lpVtbl -> RedirectMessage(This,pszCaption,pszMessage,uType,pResponse) ) 

#define IWRdsProtocolLogonErrorRedirector_RedirectLogonError(This,ntsStatus,ntsSubstatus,pszCaption,pszMessage,uType,pResponse)	\
    ( (This)->lpVtbl -> RedirectLogonError(This,ntsStatus,ntsSubstatus,pszCaption,pszMessage,uType,pResponse) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWRdsProtocolLogonErrorRedirector_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_wtsprotocol_0000_0019 */
/* [local] */ 




extern RPC_IF_HANDLE __MIDL_itf_wtsprotocol_0000_0019_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_wtsprotocol_0000_0019_v0_0_s_ifspec;

#ifndef __IWRdsWddmIddProps_INTERFACE_DEFINED__
#define __IWRdsWddmIddProps_INTERFACE_DEFINED__

/* interface IWRdsWddmIddProps */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IWRdsWddmIddProps;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("1382DF4D-A289-43D1-A184-144726F9AF90")
    IWRdsWddmIddProps : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetHardwareId( 
            /* [size_is][string][in] */ __RPC__in_ecount_full_string(Count) WCHAR *pDisplayDriverHardwareId,
            /* [in] */ DWORD Count) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnDriverLoad( 
            /* [in] */ ULONG SessionId,
            /* [in] */ HANDLE_PTR DriverHandle) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnDriverUnload( 
            /* [in] */ ULONG SessionId) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EnableWddmIdd( 
            /* [in] */ BOOL Enabled) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWRdsWddmIddPropsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IWRdsWddmIddProps * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IWRdsWddmIddProps * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IWRdsWddmIddProps * This);
        
        DECLSPEC_XFGVIRT(IWRdsWddmIddProps, GetHardwareId)
        HRESULT ( STDMETHODCALLTYPE *GetHardwareId )( 
            __RPC__in IWRdsWddmIddProps * This,
            /* [size_is][string][in] */ __RPC__in_ecount_full_string(Count) WCHAR *pDisplayDriverHardwareId,
            /* [in] */ DWORD Count);
        
        DECLSPEC_XFGVIRT(IWRdsWddmIddProps, OnDriverLoad)
        HRESULT ( STDMETHODCALLTYPE *OnDriverLoad )( 
            __RPC__in IWRdsWddmIddProps * This,
            /* [in] */ ULONG SessionId,
            /* [in] */ HANDLE_PTR DriverHandle);
        
        DECLSPEC_XFGVIRT(IWRdsWddmIddProps, OnDriverUnload)
        HRESULT ( STDMETHODCALLTYPE *OnDriverUnload )( 
            __RPC__in IWRdsWddmIddProps * This,
            /* [in] */ ULONG SessionId);
        
        DECLSPEC_XFGVIRT(IWRdsWddmIddProps, EnableWddmIdd)
        HRESULT ( STDMETHODCALLTYPE *EnableWddmIdd )( 
            __RPC__in IWRdsWddmIddProps * This,
            /* [in] */ BOOL Enabled);
        
        END_INTERFACE
    } IWRdsWddmIddPropsVtbl;

    interface IWRdsWddmIddProps
    {
        CONST_VTBL struct IWRdsWddmIddPropsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWRdsWddmIddProps_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWRdsWddmIddProps_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWRdsWddmIddProps_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWRdsWddmIddProps_GetHardwareId(This,pDisplayDriverHardwareId,Count)	\
    ( (This)->lpVtbl -> GetHardwareId(This,pDisplayDriverHardwareId,Count) ) 

#define IWRdsWddmIddProps_OnDriverLoad(This,SessionId,DriverHandle)	\
    ( (This)->lpVtbl -> OnDriverLoad(This,SessionId,DriverHandle) ) 

#define IWRdsWddmIddProps_OnDriverUnload(This,SessionId)	\
    ( (This)->lpVtbl -> OnDriverUnload(This,SessionId) ) 

#define IWRdsWddmIddProps_EnableWddmIdd(This,Enabled)	\
    ( (This)->lpVtbl -> EnableWddmIdd(This,Enabled) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWRdsWddmIddProps_INTERFACE_DEFINED__ */


#ifndef __IWRdsWddmIddProps1_INTERFACE_DEFINED__
#define __IWRdsWddmIddProps1_INTERFACE_DEFINED__

/* interface IWRdsWddmIddProps1 */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IWRdsWddmIddProps1;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("60F71B1A-3682-4BC7-997E-4E4F02A08148")
    IWRdsWddmIddProps1 : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetHardwareId( 
            /* [size_is][string][in] */ __RPC__in_ecount_full_string(Count) WCHAR *pDisplayDriverHardwareId,
            /* [in] */ DWORD Count) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnDriverLoad( 
            /* [in] */ ULONG SessionId,
            /* [in] */ __RPC__in LPCWSTR DeviceInstance) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnDriverUnload( 
            /* [in] */ ULONG SessionId) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWRdsWddmIddProps1Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IWRdsWddmIddProps1 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IWRdsWddmIddProps1 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IWRdsWddmIddProps1 * This);
        
        DECLSPEC_XFGVIRT(IWRdsWddmIddProps1, GetHardwareId)
        HRESULT ( STDMETHODCALLTYPE *GetHardwareId )( 
            __RPC__in IWRdsWddmIddProps1 * This,
            /* [size_is][string][in] */ __RPC__in_ecount_full_string(Count) WCHAR *pDisplayDriverHardwareId,
            /* [in] */ DWORD Count);
        
        DECLSPEC_XFGVIRT(IWRdsWddmIddProps1, OnDriverLoad)
        HRESULT ( STDMETHODCALLTYPE *OnDriverLoad )( 
            __RPC__in IWRdsWddmIddProps1 * This,
            /* [in] */ ULONG SessionId,
            /* [in] */ __RPC__in LPCWSTR DeviceInstance);
        
        DECLSPEC_XFGVIRT(IWRdsWddmIddProps1, OnDriverUnload)
        HRESULT ( STDMETHODCALLTYPE *OnDriverUnload )( 
            __RPC__in IWRdsWddmIddProps1 * This,
            /* [in] */ ULONG SessionId);
        
        END_INTERFACE
    } IWRdsWddmIddProps1Vtbl;

    interface IWRdsWddmIddProps1
    {
        CONST_VTBL struct IWRdsWddmIddProps1Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWRdsWddmIddProps1_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWRdsWddmIddProps1_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWRdsWddmIddProps1_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWRdsWddmIddProps1_GetHardwareId(This,pDisplayDriverHardwareId,Count)	\
    ( (This)->lpVtbl -> GetHardwareId(This,pDisplayDriverHardwareId,Count) ) 

#define IWRdsWddmIddProps1_OnDriverLoad(This,SessionId,DeviceInstance)	\
    ( (This)->lpVtbl -> OnDriverLoad(This,SessionId,DeviceInstance) ) 

#define IWRdsWddmIddProps1_OnDriverUnload(This,SessionId)	\
    ( (This)->lpVtbl -> OnDriverUnload(This,SessionId) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWRdsWddmIddProps1_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_wtsprotocol_0000_0021 */
/* [local] */ 




extern RPC_IF_HANDLE __MIDL_itf_wtsprotocol_0000_0021_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_wtsprotocol_0000_0021_v0_0_s_ifspec;

#ifndef __IWRdsProtocolConnectionSettings_INTERFACE_DEFINED__
#define __IWRdsProtocolConnectionSettings_INTERFACE_DEFINED__

/* interface IWRdsProtocolConnectionSettings */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IWRdsProtocolConnectionSettings;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("83FCF5D3-F6F4-EA94-9CD2-32F280E1E510")
    IWRdsProtocolConnectionSettings : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SetConnectionSetting( 
            /* [in] */ GUID PropertyID,
            /* [in] */ __RPC__in PWTS_PROPERTY_VALUE pPropertyEntriesIn) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetConnectionSetting( 
            /* [in] */ GUID PropertyID,
            /* [out] */ __RPC__out PWTS_PROPERTY_VALUE pPropertyEntriesOut) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWRdsProtocolConnectionSettingsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IWRdsProtocolConnectionSettings * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IWRdsProtocolConnectionSettings * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IWRdsProtocolConnectionSettings * This);
        
        DECLSPEC_XFGVIRT(IWRdsProtocolConnectionSettings, SetConnectionSetting)
        HRESULT ( STDMETHODCALLTYPE *SetConnectionSetting )( 
            __RPC__in IWRdsProtocolConnectionSettings * This,
            /* [in] */ GUID PropertyID,
            /* [in] */ __RPC__in PWTS_PROPERTY_VALUE pPropertyEntriesIn);
        
        DECLSPEC_XFGVIRT(IWRdsProtocolConnectionSettings, GetConnectionSetting)
        HRESULT ( STDMETHODCALLTYPE *GetConnectionSetting )( 
            __RPC__in IWRdsProtocolConnectionSettings * This,
            /* [in] */ GUID PropertyID,
            /* [out] */ __RPC__out PWTS_PROPERTY_VALUE pPropertyEntriesOut);
        
        END_INTERFACE
    } IWRdsProtocolConnectionSettingsVtbl;

    interface IWRdsProtocolConnectionSettings
    {
        CONST_VTBL struct IWRdsProtocolConnectionSettingsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWRdsProtocolConnectionSettings_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWRdsProtocolConnectionSettings_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWRdsProtocolConnectionSettings_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWRdsProtocolConnectionSettings_SetConnectionSetting(This,PropertyID,pPropertyEntriesIn)	\
    ( (This)->lpVtbl -> SetConnectionSetting(This,PropertyID,pPropertyEntriesIn) ) 

#define IWRdsProtocolConnectionSettings_GetConnectionSetting(This,PropertyID,pPropertyEntriesOut)	\
    ( (This)->lpVtbl -> GetConnectionSetting(This,PropertyID,pPropertyEntriesOut) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWRdsProtocolConnectionSettings_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_wtsprotocol_0000_0022 */
/* [local] */ 




extern RPC_IF_HANDLE __MIDL_itf_wtsprotocol_0000_0022_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_wtsprotocol_0000_0022_v0_0_s_ifspec;

#ifndef __IWRdsEnhancedFastReconnectArbitrator_INTERFACE_DEFINED__
#define __IWRdsEnhancedFastReconnectArbitrator_INTERFACE_DEFINED__

/* interface IWRdsEnhancedFastReconnectArbitrator */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IWRdsEnhancedFastReconnectArbitrator;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("5718AE9B-47F2-499F-B634-D8175BD51131")
    IWRdsEnhancedFastReconnectArbitrator : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetSessionForEnhancedFastReconnect( 
            /* [in] */ __RPC__in LONG *pSessionIdArray,
            /* [in] */ DWORD dwSessionCount,
            /* [out] */ __RPC__out LONG *pResultSessionId) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWRdsEnhancedFastReconnectArbitratorVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IWRdsEnhancedFastReconnectArbitrator * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IWRdsEnhancedFastReconnectArbitrator * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IWRdsEnhancedFastReconnectArbitrator * This);
        
        DECLSPEC_XFGVIRT(IWRdsEnhancedFastReconnectArbitrator, GetSessionForEnhancedFastReconnect)
        HRESULT ( STDMETHODCALLTYPE *GetSessionForEnhancedFastReconnect )( 
            __RPC__in IWRdsEnhancedFastReconnectArbitrator * This,
            /* [in] */ __RPC__in LONG *pSessionIdArray,
            /* [in] */ DWORD dwSessionCount,
            /* [out] */ __RPC__out LONG *pResultSessionId);
        
        END_INTERFACE
    } IWRdsEnhancedFastReconnectArbitratorVtbl;

    interface IWRdsEnhancedFastReconnectArbitrator
    {
        CONST_VTBL struct IWRdsEnhancedFastReconnectArbitratorVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWRdsEnhancedFastReconnectArbitrator_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWRdsEnhancedFastReconnectArbitrator_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWRdsEnhancedFastReconnectArbitrator_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWRdsEnhancedFastReconnectArbitrator_GetSessionForEnhancedFastReconnect(This,pSessionIdArray,dwSessionCount,pResultSessionId)	\
    ( (This)->lpVtbl -> GetSessionForEnhancedFastReconnect(This,pSessionIdArray,dwSessionCount,pResultSessionId) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWRdsEnhancedFastReconnectArbitrator_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_wtsprotocol_0000_0023 */
/* [local] */ 

#if (NTDDI_VERSION >= NTDDI_WIN11_GE)



extern RPC_IF_HANDLE __MIDL_itf_wtsprotocol_0000_0023_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_wtsprotocol_0000_0023_v0_0_s_ifspec;

#ifndef __IWRdsProtocolConnection2_INTERFACE_DEFINED__
#define __IWRdsProtocolConnection2_INTERFACE_DEFINED__

/* interface IWRdsProtocolConnection2 */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IWRdsProtocolConnection2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("C2BD9B66-4A76-4701-B6A3-BFAFC1482169")
    IWRdsProtocolConnection2 : public IWRdsProtocolConnection
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetSerializedUserCredential( 
            /* [out] */ __RPC__deref_out_opt WRDS_SERIALIZED_USER_CREDENTIAL **userCredential) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWRdsProtocolConnection2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IWRdsProtocolConnection2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IWRdsProtocolConnection2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IWRdsProtocolConnection2 * This);
        
        DECLSPEC_XFGVIRT(IWRdsProtocolConnection, GetLogonErrorRedirector)
        HRESULT ( STDMETHODCALLTYPE *GetLogonErrorRedirector )( 
            __RPC__in IWRdsProtocolConnection2 * This,
            /* [out] */ __RPC__deref_out_opt IWRdsProtocolLogonErrorRedirector **ppLogonErrorRedir);
        
        DECLSPEC_XFGVIRT(IWRdsProtocolConnection, AcceptConnection)
        HRESULT ( STDMETHODCALLTYPE *AcceptConnection )( 
            __RPC__in IWRdsProtocolConnection2 * This);
        
        DECLSPEC_XFGVIRT(IWRdsProtocolConnection, GetClientData)
        HRESULT ( STDMETHODCALLTYPE *GetClientData )( 
            __RPC__in IWRdsProtocolConnection2 * This,
            /* [out] */ __RPC__out WRDS_CLIENT_DATA *pClientData);
        
        DECLSPEC_XFGVIRT(IWRdsProtocolConnection, GetClientMonitorData)
        HRESULT ( STDMETHODCALLTYPE *GetClientMonitorData )( 
            __RPC__in IWRdsProtocolConnection2 * This,
            /* [out] */ __RPC__out UINT *pNumMonitors,
            /* [out] */ __RPC__out UINT *pPrimaryMonitor);
        
        DECLSPEC_XFGVIRT(IWRdsProtocolConnection, GetUserCredentials)
        HRESULT ( STDMETHODCALLTYPE *GetUserCredentials )( 
            __RPC__in IWRdsProtocolConnection2 * This,
            /* [out] */ __RPC__out WRDS_USER_CREDENTIAL *pUserCreds);
        
        DECLSPEC_XFGVIRT(IWRdsProtocolConnection, GetLicenseConnection)
        HRESULT ( STDMETHODCALLTYPE *GetLicenseConnection )( 
            __RPC__in IWRdsProtocolConnection2 * This,
            /* [out] */ __RPC__deref_out_opt IWRdsProtocolLicenseConnection **ppLicenseConnection);
        
        DECLSPEC_XFGVIRT(IWRdsProtocolConnection, AuthenticateClientToSession)
        HRESULT ( STDMETHODCALLTYPE *AuthenticateClientToSession )( 
            __RPC__in IWRdsProtocolConnection2 * This,
            /* [out] */ __RPC__out WRDS_SESSION_ID *SessionId);
        
        DECLSPEC_XFGVIRT(IWRdsProtocolConnection, NotifySessionId)
        HRESULT ( STDMETHODCALLTYPE *NotifySessionId )( 
            __RPC__in IWRdsProtocolConnection2 * This,
            /* [in] */ __RPC__in WRDS_SESSION_ID *SessionId,
            /* [in] */ HANDLE_PTR SessionHandle);
        
        DECLSPEC_XFGVIRT(IWRdsProtocolConnection, GetInputHandles)
        HRESULT ( STDMETHODCALLTYPE *GetInputHandles )( 
            __RPC__in IWRdsProtocolConnection2 * This,
            /* [out] */ __RPC__out HANDLE_PTR *pKeyboardHandle,
            /* [out] */ __RPC__out HANDLE_PTR *pMouseHandle,
            /* [out] */ __RPC__out HANDLE_PTR *pBeepHandle);
        
        DECLSPEC_XFGVIRT(IWRdsProtocolConnection, GetVideoHandle)
        HRESULT ( STDMETHODCALLTYPE *GetVideoHandle )( 
            __RPC__in IWRdsProtocolConnection2 * This,
            /* [out] */ __RPC__out HANDLE_PTR *pVideoHandle);
        
        DECLSPEC_XFGVIRT(IWRdsProtocolConnection, ConnectNotify)
        HRESULT ( STDMETHODCALLTYPE *ConnectNotify )( 
            __RPC__in IWRdsProtocolConnection2 * This,
            /* [in] */ ULONG SessionId);
        
        DECLSPEC_XFGVIRT(IWRdsProtocolConnection, IsUserAllowedToLogon)
        HRESULT ( STDMETHODCALLTYPE *IsUserAllowedToLogon )( 
            __RPC__in IWRdsProtocolConnection2 * This,
            /* [in] */ ULONG SessionId,
            /* [in] */ HANDLE_PTR UserToken,
            /* [string][in] */ __RPC__in_string WCHAR *pDomainName,
            /* [string][in] */ __RPC__in_string WCHAR *pUserName);
        
        DECLSPEC_XFGVIRT(IWRdsProtocolConnection, SessionArbitrationEnumeration)
        HRESULT ( STDMETHODCALLTYPE *SessionArbitrationEnumeration )( 
            __RPC__in IWRdsProtocolConnection2 * This,
            /* [in] */ HANDLE_PTR hUserToken,
            /* [in] */ BOOL bSingleSessionPerUserEnabled,
            /* [size_is][out] */ __RPC__out_ecount_full(*pdwSessionIdentifierCount) ULONG *pSessionIdArray,
            /* [out][in] */ __RPC__inout ULONG *pdwSessionIdentifierCount);
        
        DECLSPEC_XFGVIRT(IWRdsProtocolConnection, LogonNotify)
        HRESULT ( STDMETHODCALLTYPE *LogonNotify )( 
            __RPC__in IWRdsProtocolConnection2 * This,
            /* [in] */ HANDLE_PTR hClientToken,
            /* [string][in] */ __RPC__in_string WCHAR *wszUserName,
            /* [string][in] */ __RPC__in_string WCHAR *wszDomainName,
            /* [in] */ __RPC__in WRDS_SESSION_ID *SessionId,
            /* [out][in] */ __RPC__inout PWRDS_CONNECTION_SETTINGS pWRdsConnectionSettings);
        
        DECLSPEC_XFGVIRT(IWRdsProtocolConnection, PreDisconnect)
        HRESULT ( STDMETHODCALLTYPE *PreDisconnect )( 
            __RPC__in IWRdsProtocolConnection2 * This,
            /* [in] */ ULONG DisconnectReason);
        
        DECLSPEC_XFGVIRT(IWRdsProtocolConnection, DisconnectNotify)
        HRESULT ( STDMETHODCALLTYPE *DisconnectNotify )( 
            __RPC__in IWRdsProtocolConnection2 * This);
        
        DECLSPEC_XFGVIRT(IWRdsProtocolConnection, Close)
        HRESULT ( STDMETHODCALLTYPE *Close )( 
            __RPC__in IWRdsProtocolConnection2 * This);
        
        DECLSPEC_XFGVIRT(IWRdsProtocolConnection, GetProtocolStatus)
        HRESULT ( STDMETHODCALLTYPE *GetProtocolStatus )( 
            __RPC__in IWRdsProtocolConnection2 * This,
            /* [out] */ __RPC__out WRDS_PROTOCOL_STATUS *pProtocolStatus);
        
        DECLSPEC_XFGVIRT(IWRdsProtocolConnection, GetLastInputTime)
        HRESULT ( STDMETHODCALLTYPE *GetLastInputTime )( 
            __RPC__in IWRdsProtocolConnection2 * This,
            /* [out] */ __RPC__out ULONG64 *pLastInputTime);
        
        DECLSPEC_XFGVIRT(IWRdsProtocolConnection, SetErrorInfo)
        HRESULT ( STDMETHODCALLTYPE *SetErrorInfo )( 
            __RPC__in IWRdsProtocolConnection2 * This,
            /* [in] */ ULONG ulError);
        
        DECLSPEC_XFGVIRT(IWRdsProtocolConnection, CreateVirtualChannel)
        HRESULT ( STDMETHODCALLTYPE *CreateVirtualChannel )( 
            __RPC__in IWRdsProtocolConnection2 * This,
            /* [string][in] */ __RPC__in_string CHAR *szEndpointName,
            /* [in] */ BOOL bStatic,
            /* [in] */ ULONG RequestedPriority,
            /* [out] */ __RPC__out ULONG_PTR *phChannel);
        
        DECLSPEC_XFGVIRT(IWRdsProtocolConnection, QueryProperty)
        HRESULT ( STDMETHODCALLTYPE *QueryProperty )( 
            __RPC__in IWRdsProtocolConnection2 * This,
            /* [in] */ GUID QueryType,
            /* [in] */ ULONG ulNumEntriesIn,
            /* [in] */ ULONG ulNumEntriesOut,
            /* [size_is][optional][in] */ __RPC__in_ecount_full(ulNumEntriesIn) PWRDS_PROPERTY_VALUE pPropertyEntriesIn,
            /* [size_is][optional][out] */ __RPC__out_ecount_full(ulNumEntriesOut) PWRDS_PROPERTY_VALUE pPropertyEntriesOut);
        
        DECLSPEC_XFGVIRT(IWRdsProtocolConnection, GetShadowConnection)
        HRESULT ( STDMETHODCALLTYPE *GetShadowConnection )( 
            __RPC__in IWRdsProtocolConnection2 * This,
            /* [out] */ __RPC__deref_out_opt IWRdsProtocolShadowConnection **ppShadowConnection);
        
        DECLSPEC_XFGVIRT(IWRdsProtocolConnection, NotifyCommandProcessCreated)
        HRESULT ( STDMETHODCALLTYPE *NotifyCommandProcessCreated )( 
            __RPC__in IWRdsProtocolConnection2 * This,
            /* [in] */ ULONG SessionId);
        
        DECLSPEC_XFGVIRT(IWRdsProtocolConnection2, GetSerializedUserCredential)
        HRESULT ( STDMETHODCALLTYPE *GetSerializedUserCredential )( 
            __RPC__in IWRdsProtocolConnection2 * This,
            /* [out] */ __RPC__deref_out_opt WRDS_SERIALIZED_USER_CREDENTIAL **userCredential);
        
        END_INTERFACE
    } IWRdsProtocolConnection2Vtbl;

    interface IWRdsProtocolConnection2
    {
        CONST_VTBL struct IWRdsProtocolConnection2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWRdsProtocolConnection2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWRdsProtocolConnection2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWRdsProtocolConnection2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWRdsProtocolConnection2_GetLogonErrorRedirector(This,ppLogonErrorRedir)	\
    ( (This)->lpVtbl -> GetLogonErrorRedirector(This,ppLogonErrorRedir) ) 

#define IWRdsProtocolConnection2_AcceptConnection(This)	\
    ( (This)->lpVtbl -> AcceptConnection(This) ) 

#define IWRdsProtocolConnection2_GetClientData(This,pClientData)	\
    ( (This)->lpVtbl -> GetClientData(This,pClientData) ) 

#define IWRdsProtocolConnection2_GetClientMonitorData(This,pNumMonitors,pPrimaryMonitor)	\
    ( (This)->lpVtbl -> GetClientMonitorData(This,pNumMonitors,pPrimaryMonitor) ) 

#define IWRdsProtocolConnection2_GetUserCredentials(This,pUserCreds)	\
    ( (This)->lpVtbl -> GetUserCredentials(This,pUserCreds) ) 

#define IWRdsProtocolConnection2_GetLicenseConnection(This,ppLicenseConnection)	\
    ( (This)->lpVtbl -> GetLicenseConnection(This,ppLicenseConnection) ) 

#define IWRdsProtocolConnection2_AuthenticateClientToSession(This,SessionId)	\
    ( (This)->lpVtbl -> AuthenticateClientToSession(This,SessionId) ) 

#define IWRdsProtocolConnection2_NotifySessionId(This,SessionId,SessionHandle)	\
    ( (This)->lpVtbl -> NotifySessionId(This,SessionId,SessionHandle) ) 

#define IWRdsProtocolConnection2_GetInputHandles(This,pKeyboardHandle,pMouseHandle,pBeepHandle)	\
    ( (This)->lpVtbl -> GetInputHandles(This,pKeyboardHandle,pMouseHandle,pBeepHandle) ) 

#define IWRdsProtocolConnection2_GetVideoHandle(This,pVideoHandle)	\
    ( (This)->lpVtbl -> GetVideoHandle(This,pVideoHandle) ) 

#define IWRdsProtocolConnection2_ConnectNotify(This,SessionId)	\
    ( (This)->lpVtbl -> ConnectNotify(This,SessionId) ) 

#define IWRdsProtocolConnection2_IsUserAllowedToLogon(This,SessionId,UserToken,pDomainName,pUserName)	\
    ( (This)->lpVtbl -> IsUserAllowedToLogon(This,SessionId,UserToken,pDomainName,pUserName) ) 

#define IWRdsProtocolConnection2_SessionArbitrationEnumeration(This,hUserToken,bSingleSessionPerUserEnabled,pSessionIdArray,pdwSessionIdentifierCount)	\
    ( (This)->lpVtbl -> SessionArbitrationEnumeration(This,hUserToken,bSingleSessionPerUserEnabled,pSessionIdArray,pdwSessionIdentifierCount) ) 

#define IWRdsProtocolConnection2_LogonNotify(This,hClientToken,wszUserName,wszDomainName,SessionId,pWRdsConnectionSettings)	\
    ( (This)->lpVtbl -> LogonNotify(This,hClientToken,wszUserName,wszDomainName,SessionId,pWRdsConnectionSettings) ) 

#define IWRdsProtocolConnection2_PreDisconnect(This,DisconnectReason)	\
    ( (This)->lpVtbl -> PreDisconnect(This,DisconnectReason) ) 

#define IWRdsProtocolConnection2_DisconnectNotify(This)	\
    ( (This)->lpVtbl -> DisconnectNotify(This) ) 

#define IWRdsProtocolConnection2_Close(This)	\
    ( (This)->lpVtbl -> Close(This) ) 

#define IWRdsProtocolConnection2_GetProtocolStatus(This,pProtocolStatus)	\
    ( (This)->lpVtbl -> GetProtocolStatus(This,pProtocolStatus) ) 

#define IWRdsProtocolConnection2_GetLastInputTime(This,pLastInputTime)	\
    ( (This)->lpVtbl -> GetLastInputTime(This,pLastInputTime) ) 

#define IWRdsProtocolConnection2_SetErrorInfo(This,ulError)	\
    ( (This)->lpVtbl -> SetErrorInfo(This,ulError) ) 

#define IWRdsProtocolConnection2_CreateVirtualChannel(This,szEndpointName,bStatic,RequestedPriority,phChannel)	\
    ( (This)->lpVtbl -> CreateVirtualChannel(This,szEndpointName,bStatic,RequestedPriority,phChannel) ) 

#define IWRdsProtocolConnection2_QueryProperty(This,QueryType,ulNumEntriesIn,ulNumEntriesOut,pPropertyEntriesIn,pPropertyEntriesOut)	\
    ( (This)->lpVtbl -> QueryProperty(This,QueryType,ulNumEntriesIn,ulNumEntriesOut,pPropertyEntriesIn,pPropertyEntriesOut) ) 

#define IWRdsProtocolConnection2_GetShadowConnection(This,ppShadowConnection)	\
    ( (This)->lpVtbl -> GetShadowConnection(This,ppShadowConnection) ) 

#define IWRdsProtocolConnection2_NotifyCommandProcessCreated(This,SessionId)	\
    ( (This)->lpVtbl -> NotifyCommandProcessCreated(This,SessionId) ) 


#define IWRdsProtocolConnection2_GetSerializedUserCredential(This,userCredential)	\
    ( (This)->lpVtbl -> GetSerializedUserCredential(This,userCredential) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWRdsProtocolConnection2_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_wtsprotocol_0000_0024 */
/* [local] */ 

#endif
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_wtsprotocol_0000_0024_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_wtsprotocol_0000_0024_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


