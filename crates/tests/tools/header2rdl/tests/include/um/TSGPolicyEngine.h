

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

#ifndef __tsgpolicyengine_h__
#define __tsgpolicyengine_h__

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

#ifndef __ITSGAuthorizeConnectionSink_FWD_DEFINED__
#define __ITSGAuthorizeConnectionSink_FWD_DEFINED__
typedef interface ITSGAuthorizeConnectionSink ITSGAuthorizeConnectionSink;

#endif 	/* __ITSGAuthorizeConnectionSink_FWD_DEFINED__ */


#ifndef __ITSGAuthorizeResourceSink_FWD_DEFINED__
#define __ITSGAuthorizeResourceSink_FWD_DEFINED__
typedef interface ITSGAuthorizeResourceSink ITSGAuthorizeResourceSink;

#endif 	/* __ITSGAuthorizeResourceSink_FWD_DEFINED__ */


#ifndef __ITSGPolicyEngine_FWD_DEFINED__
#define __ITSGPolicyEngine_FWD_DEFINED__
typedef interface ITSGPolicyEngine ITSGPolicyEngine;

#endif 	/* __ITSGPolicyEngine_FWD_DEFINED__ */


#ifndef __ITSGAccountingEngine_FWD_DEFINED__
#define __ITSGAccountingEngine_FWD_DEFINED__
typedef interface ITSGAccountingEngine ITSGAccountingEngine;

#endif 	/* __ITSGAccountingEngine_FWD_DEFINED__ */


/* header files for imported files */
#include "oaidl.h"
#include "ocidl.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_tsgpolicyengine_0000_0000 */
/* [local] */ 

#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
typedef /* [public][public][public][public] */ 
enum __MIDL___MIDL_itf_tsgpolicyengine_0000_0000_0001
    {
        AA_AUTH_MIN	= 0,
        AA_AUTH_BASIC	= ( AA_AUTH_MIN + 1 ) ,
        AA_AUTH_NTLM	= ( AA_AUTH_BASIC + 1 ) ,
        AA_AUTH_SC	= ( AA_AUTH_NTLM + 1 ) ,
        AA_AUTH_LOGGEDONCREDENTIALS	= ( AA_AUTH_SC + 1 ) ,
        AA_AUTH_NEGOTIATE	= ( AA_AUTH_LOGGEDONCREDENTIALS + 1 ) ,
        AA_AUTH_ANY	= ( AA_AUTH_NEGOTIATE + 1 ) ,
        AA_AUTH_COOKIE	= ( AA_AUTH_ANY + 1 ) ,
        AA_AUTH_DIGEST	= ( AA_AUTH_COOKIE + 1 ) ,
        AA_AUTH_ORGID	= ( AA_AUTH_DIGEST + 1 ) ,
        AA_AUTH_CONID	= ( AA_AUTH_ORGID + 1 ) ,
        AA_AUTH_SSPI_NTLM	= ( AA_AUTH_CONID + 1 ) ,
        AA_AUTH_MAX	= ( AA_AUTH_SSPI_NTLM + 1 ) 
    } 	AAAuthSchemes;

typedef /* [public][public] */ 
enum __MIDL___MIDL_itf_tsgpolicyengine_0000_0000_0002
    {
        AA_MAIN_SESSION_CREATION	= 0,
        AA_SUB_SESSION_CREATION	= ( AA_MAIN_SESSION_CREATION + 1 ) ,
        AA_SUB_SESSION_CLOSED	= ( AA_SUB_SESSION_CREATION + 1 ) ,
        AA_MAIN_SESSION_CLOSED	= ( AA_SUB_SESSION_CLOSED + 1 ) 
    } 	AAAccountingDataType;

typedef /* [public][public] */ struct __MIDL___MIDL_itf_tsgpolicyengine_0000_0000_0003
    {
    BSTR userName;
    BSTR clientName;
    AAAuthSchemes authType;
    BSTR resourceName;
    int portNumber;
    BSTR protocolName;
    int numberOfBytesReceived;
    int numberOfBytesTransfered;
    BSTR reasonForDisconnect;
    GUID mainSessionId;
    int subSessionId;
    } 	AAAccountingData;

typedef /* [public][public] */ 
enum __MIDL___MIDL_itf_tsgpolicyengine_0000_0000_0004
    {
        SESSION_TIMEOUT_ACTION_DISCONNECT	= 0,
        SESSION_TIMEOUT_ACTION_SILENT_REAUTH	= ( SESSION_TIMEOUT_ACTION_DISCONNECT + 1 ) 
    } 	SESSION_TIMEOUT_ACTION_TYPE;

typedef /* [public] */ 
enum __MIDL___MIDL_itf_tsgpolicyengine_0000_0000_0005
    {
        EnableAllRedirections	= 0,
        DisableAllRedirections	= ( EnableAllRedirections + 1 ) ,
        DriveRedirectionDisabled	= ( DisableAllRedirections + 1 ) ,
        PrinterRedirectionDisabled	= ( DriveRedirectionDisabled + 1 ) ,
        PortRedirectionDisabled	= ( PrinterRedirectionDisabled + 1 ) ,
        ClipboardRedirectionDisabled	= ( PortRedirectionDisabled + 1 ) ,
        PnpRedirectionDisabled	= ( ClipboardRedirectionDisabled + 1 ) ,
        AllowOnlySDRServers	= ( PnpRedirectionDisabled + 1 ) 
    } 	PolicyAttributeType;

#define MAX_POLICY_ATTRIBUTES 20
typedef DWORD PolicyAttributes[ 20 ];

typedef /* [public][public] */ 
enum __MIDL___MIDL_itf_tsgpolicyengine_0000_0000_0006
    {
        AA_UNTRUSTED	= 0,
        AA_TRUSTEDUSER_UNTRUSTEDCLIENT	= ( AA_UNTRUSTED + 1 ) ,
        AA_TRUSTEDUSER_TRUSTEDCLIENT	= ( AA_TRUSTEDUSER_UNTRUSTEDCLIENT + 1 ) 
    } 	AATrustClassID;



extern RPC_IF_HANDLE __MIDL_itf_tsgpolicyengine_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_tsgpolicyengine_0000_0000_v0_0_s_ifspec;

#ifndef __ITSGAuthorizeConnectionSink_INTERFACE_DEFINED__
#define __ITSGAuthorizeConnectionSink_INTERFACE_DEFINED__

/* interface ITSGAuthorizeConnectionSink */
/* [helpstring][uuid][object] */ 


EXTERN_C const IID IID_ITSGAuthorizeConnectionSink;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("c27ece33-7781-4318-98ef-1cf2da7b7005")
    ITSGAuthorizeConnectionSink : public IUnknown
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE OnConnectionAuthorized( 
            /* [in] */ HRESULT hrIn,
            /* [in] */ GUID mainSessionId,
            /* [in] */ ULONG cbSoHResponse,
            /* [size_is][in] */ __RPC__in_ecount_full(cbSoHResponse) BYTE *pbSoHResponse,
            /* [in] */ ULONG idleTimeout,
            /* [in] */ ULONG sessionTimeout,
            /* [in] */ SESSION_TIMEOUT_ACTION_TYPE sessionTimeoutAction,
            /* [in] */ AATrustClassID trustClass,
            /* [in] */ __RPC__in_ecount_full(20) PolicyAttributes policyAttributes) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITSGAuthorizeConnectionSinkVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITSGAuthorizeConnectionSink * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITSGAuthorizeConnectionSink * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITSGAuthorizeConnectionSink * This);
        
        DECLSPEC_XFGVIRT(ITSGAuthorizeConnectionSink, OnConnectionAuthorized)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *OnConnectionAuthorized )( 
            __RPC__in ITSGAuthorizeConnectionSink * This,
            /* [in] */ HRESULT hrIn,
            /* [in] */ GUID mainSessionId,
            /* [in] */ ULONG cbSoHResponse,
            /* [size_is][in] */ __RPC__in_ecount_full(cbSoHResponse) BYTE *pbSoHResponse,
            /* [in] */ ULONG idleTimeout,
            /* [in] */ ULONG sessionTimeout,
            /* [in] */ SESSION_TIMEOUT_ACTION_TYPE sessionTimeoutAction,
            /* [in] */ AATrustClassID trustClass,
            /* [in] */ __RPC__in_ecount_full(20) PolicyAttributes policyAttributes);
        
        END_INTERFACE
    } ITSGAuthorizeConnectionSinkVtbl;

    interface ITSGAuthorizeConnectionSink
    {
        CONST_VTBL struct ITSGAuthorizeConnectionSinkVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITSGAuthorizeConnectionSink_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITSGAuthorizeConnectionSink_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITSGAuthorizeConnectionSink_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITSGAuthorizeConnectionSink_OnConnectionAuthorized(This,hrIn,mainSessionId,cbSoHResponse,pbSoHResponse,idleTimeout,sessionTimeout,sessionTimeoutAction,trustClass,policyAttributes)	\
    ( (This)->lpVtbl -> OnConnectionAuthorized(This,hrIn,mainSessionId,cbSoHResponse,pbSoHResponse,idleTimeout,sessionTimeout,sessionTimeoutAction,trustClass,policyAttributes) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITSGAuthorizeConnectionSink_INTERFACE_DEFINED__ */


#ifndef __ITSGAuthorizeResourceSink_INTERFACE_DEFINED__
#define __ITSGAuthorizeResourceSink_INTERFACE_DEFINED__

/* interface ITSGAuthorizeResourceSink */
/* [helpstring][uuid][object] */ 


EXTERN_C const IID IID_ITSGAuthorizeResourceSink;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("feddfcd4-fa12-4435-ae55-7ad1a9779af7")
    ITSGAuthorizeResourceSink : public IUnknown
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE OnChannelAuthorized( 
            /* [in] */ HRESULT hrIn,
            /* [in] */ GUID mainSessionId,
            /* [in] */ int subSessionId,
            /* [size_is][in] */ __RPC__in_ecount_full(numAllowedResourceNames) BSTR *allowedResourceNames,
            /* [in] */ ULONG numAllowedResourceNames,
            /* [size_is][in] */ __RPC__in_ecount_full(numFailedResourceNames) BSTR *failedResourceNames,
            /* [in] */ ULONG numFailedResourceNames) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITSGAuthorizeResourceSinkVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITSGAuthorizeResourceSink * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITSGAuthorizeResourceSink * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITSGAuthorizeResourceSink * This);
        
        DECLSPEC_XFGVIRT(ITSGAuthorizeResourceSink, OnChannelAuthorized)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *OnChannelAuthorized )( 
            __RPC__in ITSGAuthorizeResourceSink * This,
            /* [in] */ HRESULT hrIn,
            /* [in] */ GUID mainSessionId,
            /* [in] */ int subSessionId,
            /* [size_is][in] */ __RPC__in_ecount_full(numAllowedResourceNames) BSTR *allowedResourceNames,
            /* [in] */ ULONG numAllowedResourceNames,
            /* [size_is][in] */ __RPC__in_ecount_full(numFailedResourceNames) BSTR *failedResourceNames,
            /* [in] */ ULONG numFailedResourceNames);
        
        END_INTERFACE
    } ITSGAuthorizeResourceSinkVtbl;

    interface ITSGAuthorizeResourceSink
    {
        CONST_VTBL struct ITSGAuthorizeResourceSinkVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITSGAuthorizeResourceSink_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITSGAuthorizeResourceSink_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITSGAuthorizeResourceSink_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITSGAuthorizeResourceSink_OnChannelAuthorized(This,hrIn,mainSessionId,subSessionId,allowedResourceNames,numAllowedResourceNames,failedResourceNames,numFailedResourceNames)	\
    ( (This)->lpVtbl -> OnChannelAuthorized(This,hrIn,mainSessionId,subSessionId,allowedResourceNames,numAllowedResourceNames,failedResourceNames,numFailedResourceNames) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITSGAuthorizeResourceSink_INTERFACE_DEFINED__ */


#ifndef __ITSGPolicyEngine_INTERFACE_DEFINED__
#define __ITSGPolicyEngine_INTERFACE_DEFINED__

/* interface ITSGPolicyEngine */
/* [helpstring][uuid][object] */ 


EXTERN_C const IID IID_ITSGPolicyEngine;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("8bc24f08-6223-42f4-a5b4-8e37cd135bbd")
    ITSGPolicyEngine : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE AuthorizeConnection( 
            /* [in] */ GUID mainSessionId,
            /* [in] */ __RPC__in BSTR username,
            /* [in] */ AAAuthSchemes authType,
            /* [in] */ __RPC__in BSTR clientMachineIP,
            /* [in] */ __RPC__in BSTR clientMachineName,
            /* [size_is][in] */ __RPC__in_ecount_full(numSOHBytes) BYTE *sohData,
            /* [in] */ ULONG numSOHBytes,
            /* [size_is][in] */ __RPC__in_ecount_full(numCookieBytes) BYTE *cookieData,
            /* [in] */ ULONG numCookieBytes,
            /* [in] */ HANDLE_PTR userToken,
            /* [in] */ __RPC__in_opt ITSGAuthorizeConnectionSink *pSink) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE AuthorizeResource( 
            /* [in] */ GUID mainSessionId,
            /* [in] */ int subSessionId,
            /* [in] */ __RPC__in BSTR username,
            /* [size_is][in] */ __RPC__in_ecount_full(numResources) BSTR *resourceNames,
            /* [in] */ ULONG numResources,
            /* [size_is][in] */ __RPC__in_ecount_full(numAlternateResourceName) BSTR *alternateResourceNames,
            /* [in] */ ULONG numAlternateResourceName,
            /* [in] */ ULONG portNumber,
            /* [in] */ __RPC__in BSTR operation,
            /* [size_is][in] */ __RPC__in_ecount_full(numBytesInCookie) BYTE *cookie,
            /* [in] */ ULONG numBytesInCookie,
            /* [in] */ __RPC__in_opt ITSGAuthorizeResourceSink *pSink) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Refresh( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE IsQuarantineEnabled( 
            /* [out] */ __RPC__out BOOL *quarantineEnabled) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITSGPolicyEngineVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITSGPolicyEngine * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITSGPolicyEngine * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITSGPolicyEngine * This);
        
        DECLSPEC_XFGVIRT(ITSGPolicyEngine, AuthorizeConnection)
        HRESULT ( STDMETHODCALLTYPE *AuthorizeConnection )( 
            __RPC__in ITSGPolicyEngine * This,
            /* [in] */ GUID mainSessionId,
            /* [in] */ __RPC__in BSTR username,
            /* [in] */ AAAuthSchemes authType,
            /* [in] */ __RPC__in BSTR clientMachineIP,
            /* [in] */ __RPC__in BSTR clientMachineName,
            /* [size_is][in] */ __RPC__in_ecount_full(numSOHBytes) BYTE *sohData,
            /* [in] */ ULONG numSOHBytes,
            /* [size_is][in] */ __RPC__in_ecount_full(numCookieBytes) BYTE *cookieData,
            /* [in] */ ULONG numCookieBytes,
            /* [in] */ HANDLE_PTR userToken,
            /* [in] */ __RPC__in_opt ITSGAuthorizeConnectionSink *pSink);
        
        DECLSPEC_XFGVIRT(ITSGPolicyEngine, AuthorizeResource)
        HRESULT ( STDMETHODCALLTYPE *AuthorizeResource )( 
            __RPC__in ITSGPolicyEngine * This,
            /* [in] */ GUID mainSessionId,
            /* [in] */ int subSessionId,
            /* [in] */ __RPC__in BSTR username,
            /* [size_is][in] */ __RPC__in_ecount_full(numResources) BSTR *resourceNames,
            /* [in] */ ULONG numResources,
            /* [size_is][in] */ __RPC__in_ecount_full(numAlternateResourceName) BSTR *alternateResourceNames,
            /* [in] */ ULONG numAlternateResourceName,
            /* [in] */ ULONG portNumber,
            /* [in] */ __RPC__in BSTR operation,
            /* [size_is][in] */ __RPC__in_ecount_full(numBytesInCookie) BYTE *cookie,
            /* [in] */ ULONG numBytesInCookie,
            /* [in] */ __RPC__in_opt ITSGAuthorizeResourceSink *pSink);
        
        DECLSPEC_XFGVIRT(ITSGPolicyEngine, Refresh)
        HRESULT ( STDMETHODCALLTYPE *Refresh )( 
            __RPC__in ITSGPolicyEngine * This);
        
        DECLSPEC_XFGVIRT(ITSGPolicyEngine, IsQuarantineEnabled)
        HRESULT ( STDMETHODCALLTYPE *IsQuarantineEnabled )( 
            __RPC__in ITSGPolicyEngine * This,
            /* [out] */ __RPC__out BOOL *quarantineEnabled);
        
        END_INTERFACE
    } ITSGPolicyEngineVtbl;

    interface ITSGPolicyEngine
    {
        CONST_VTBL struct ITSGPolicyEngineVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITSGPolicyEngine_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITSGPolicyEngine_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITSGPolicyEngine_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITSGPolicyEngine_AuthorizeConnection(This,mainSessionId,username,authType,clientMachineIP,clientMachineName,sohData,numSOHBytes,cookieData,numCookieBytes,userToken,pSink)	\
    ( (This)->lpVtbl -> AuthorizeConnection(This,mainSessionId,username,authType,clientMachineIP,clientMachineName,sohData,numSOHBytes,cookieData,numCookieBytes,userToken,pSink) ) 

#define ITSGPolicyEngine_AuthorizeResource(This,mainSessionId,subSessionId,username,resourceNames,numResources,alternateResourceNames,numAlternateResourceName,portNumber,operation,cookie,numBytesInCookie,pSink)	\
    ( (This)->lpVtbl -> AuthorizeResource(This,mainSessionId,subSessionId,username,resourceNames,numResources,alternateResourceNames,numAlternateResourceName,portNumber,operation,cookie,numBytesInCookie,pSink) ) 

#define ITSGPolicyEngine_Refresh(This)	\
    ( (This)->lpVtbl -> Refresh(This) ) 

#define ITSGPolicyEngine_IsQuarantineEnabled(This,quarantineEnabled)	\
    ( (This)->lpVtbl -> IsQuarantineEnabled(This,quarantineEnabled) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITSGPolicyEngine_INTERFACE_DEFINED__ */


#ifndef __ITSGAccountingEngine_INTERFACE_DEFINED__
#define __ITSGAccountingEngine_INTERFACE_DEFINED__

/* interface ITSGAccountingEngine */
/* [helpstring][uuid][object] */ 


EXTERN_C const IID IID_ITSGAccountingEngine;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("4ce2a0c9-e874-4f1a-86f4-06bbb9115338")
    ITSGAccountingEngine : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE DoAccounting( 
            /* [in] */ AAAccountingDataType accountingDataType,
            /* [in] */ AAAccountingData accountingData) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITSGAccountingEngineVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITSGAccountingEngine * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITSGAccountingEngine * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITSGAccountingEngine * This);
        
        DECLSPEC_XFGVIRT(ITSGAccountingEngine, DoAccounting)
        HRESULT ( STDMETHODCALLTYPE *DoAccounting )( 
            __RPC__in ITSGAccountingEngine * This,
            /* [in] */ AAAccountingDataType accountingDataType,
            /* [in] */ AAAccountingData accountingData);
        
        END_INTERFACE
    } ITSGAccountingEngineVtbl;

    interface ITSGAccountingEngine
    {
        CONST_VTBL struct ITSGAccountingEngineVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITSGAccountingEngine_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITSGAccountingEngine_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITSGAccountingEngine_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITSGAccountingEngine_DoAccounting(This,accountingDataType,accountingData)	\
    ( (This)->lpVtbl -> DoAccounting(This,accountingDataType,accountingData) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITSGAccountingEngine_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_tsgpolicyengine_0000_0004 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_tsgpolicyengine_0000_0004_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_tsgpolicyengine_0000_0004_v0_0_s_ifspec;

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


