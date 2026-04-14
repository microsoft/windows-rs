

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

#ifndef __tsgauthenticationengine_h__
#define __tsgauthenticationengine_h__

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

#ifndef __ITSGAuthenticateUserSink_FWD_DEFINED__
#define __ITSGAuthenticateUserSink_FWD_DEFINED__
typedef interface ITSGAuthenticateUserSink ITSGAuthenticateUserSink;

#endif 	/* __ITSGAuthenticateUserSink_FWD_DEFINED__ */


#ifndef __ITSGAuthenticationEngine_FWD_DEFINED__
#define __ITSGAuthenticationEngine_FWD_DEFINED__
typedef interface ITSGAuthenticationEngine ITSGAuthenticationEngine;

#endif 	/* __ITSGAuthenticationEngine_FWD_DEFINED__ */


/* header files for imported files */
#include "oaidl.h"
#include "ocidl.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_tsgauthenticationengine_0000_0000 */
/* [local] */ 

#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


extern RPC_IF_HANDLE __MIDL_itf_tsgauthenticationengine_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_tsgauthenticationengine_0000_0000_v0_0_s_ifspec;

#ifndef __ITSGAuthenticateUserSink_INTERFACE_DEFINED__
#define __ITSGAuthenticateUserSink_INTERFACE_DEFINED__

/* interface ITSGAuthenticateUserSink */
/* [helpstring][uuid][object] */ 


EXTERN_C const IID IID_ITSGAuthenticateUserSink;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("2c3e2e73-a782-47f9-8dfb-77ee1ed27a03")
    ITSGAuthenticateUserSink : public IUnknown
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE OnUserAuthenticated( 
            /* [in] */ __RPC__in BSTR userName,
            /* [in] */ __RPC__in BSTR userDomain,
            /* [in] */ ULONG_PTR context,
            /* [optional][in] */ HANDLE_PTR userToken) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE OnUserAuthenticationFailed( 
            /* [in] */ ULONG_PTR context,
            /* [in] */ HRESULT genericErrorCode,
            /* [in] */ HRESULT specificErrorCode) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE ReauthenticateUser( 
            /* [in] */ ULONG_PTR context) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE DisconnectUser( 
            /* [in] */ ULONG_PTR context) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITSGAuthenticateUserSinkVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITSGAuthenticateUserSink * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITSGAuthenticateUserSink * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITSGAuthenticateUserSink * This);
        
        DECLSPEC_XFGVIRT(ITSGAuthenticateUserSink, OnUserAuthenticated)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *OnUserAuthenticated )( 
            __RPC__in ITSGAuthenticateUserSink * This,
            /* [in] */ __RPC__in BSTR userName,
            /* [in] */ __RPC__in BSTR userDomain,
            /* [in] */ ULONG_PTR context,
            /* [optional][in] */ HANDLE_PTR userToken);
        
        DECLSPEC_XFGVIRT(ITSGAuthenticateUserSink, OnUserAuthenticationFailed)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *OnUserAuthenticationFailed )( 
            __RPC__in ITSGAuthenticateUserSink * This,
            /* [in] */ ULONG_PTR context,
            /* [in] */ HRESULT genericErrorCode,
            /* [in] */ HRESULT specificErrorCode);
        
        DECLSPEC_XFGVIRT(ITSGAuthenticateUserSink, ReauthenticateUser)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *ReauthenticateUser )( 
            __RPC__in ITSGAuthenticateUserSink * This,
            /* [in] */ ULONG_PTR context);
        
        DECLSPEC_XFGVIRT(ITSGAuthenticateUserSink, DisconnectUser)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *DisconnectUser )( 
            __RPC__in ITSGAuthenticateUserSink * This,
            /* [in] */ ULONG_PTR context);
        
        END_INTERFACE
    } ITSGAuthenticateUserSinkVtbl;

    interface ITSGAuthenticateUserSink
    {
        CONST_VTBL struct ITSGAuthenticateUserSinkVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITSGAuthenticateUserSink_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITSGAuthenticateUserSink_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITSGAuthenticateUserSink_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITSGAuthenticateUserSink_OnUserAuthenticated(This,userName,userDomain,context,userToken)	\
    ( (This)->lpVtbl -> OnUserAuthenticated(This,userName,userDomain,context,userToken) ) 

#define ITSGAuthenticateUserSink_OnUserAuthenticationFailed(This,context,genericErrorCode,specificErrorCode)	\
    ( (This)->lpVtbl -> OnUserAuthenticationFailed(This,context,genericErrorCode,specificErrorCode) ) 

#define ITSGAuthenticateUserSink_ReauthenticateUser(This,context)	\
    ( (This)->lpVtbl -> ReauthenticateUser(This,context) ) 

#define ITSGAuthenticateUserSink_DisconnectUser(This,context)	\
    ( (This)->lpVtbl -> DisconnectUser(This,context) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITSGAuthenticateUserSink_INTERFACE_DEFINED__ */


#ifndef __ITSGAuthenticationEngine_INTERFACE_DEFINED__
#define __ITSGAuthenticationEngine_INTERFACE_DEFINED__

/* interface ITSGAuthenticationEngine */
/* [helpstring][uuid][object] */ 


EXTERN_C const IID IID_ITSGAuthenticationEngine;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("9ee3e5bf-04ab-4691-998c-d7f622321a56")
    ITSGAuthenticationEngine : public IUnknown
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE AuthenticateUser( 
            /* [in] */ GUID mainSessionId,
            /* [in] */ __RPC__in BYTE *cookieData,
            /* [in] */ ULONG numCookieBytes,
            /* [in] */ ULONG_PTR context,
            /* [in] */ __RPC__in_opt ITSGAuthenticateUserSink *pSink) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE CancelAuthentication( 
            /* [in] */ GUID mainSessionId,
            /* [in] */ ULONG_PTR context) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITSGAuthenticationEngineVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITSGAuthenticationEngine * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITSGAuthenticationEngine * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITSGAuthenticationEngine * This);
        
        DECLSPEC_XFGVIRT(ITSGAuthenticationEngine, AuthenticateUser)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *AuthenticateUser )( 
            __RPC__in ITSGAuthenticationEngine * This,
            /* [in] */ GUID mainSessionId,
            /* [in] */ __RPC__in BYTE *cookieData,
            /* [in] */ ULONG numCookieBytes,
            /* [in] */ ULONG_PTR context,
            /* [in] */ __RPC__in_opt ITSGAuthenticateUserSink *pSink);
        
        DECLSPEC_XFGVIRT(ITSGAuthenticationEngine, CancelAuthentication)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *CancelAuthentication )( 
            __RPC__in ITSGAuthenticationEngine * This,
            /* [in] */ GUID mainSessionId,
            /* [in] */ ULONG_PTR context);
        
        END_INTERFACE
    } ITSGAuthenticationEngineVtbl;

    interface ITSGAuthenticationEngine
    {
        CONST_VTBL struct ITSGAuthenticationEngineVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITSGAuthenticationEngine_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITSGAuthenticationEngine_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITSGAuthenticationEngine_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITSGAuthenticationEngine_AuthenticateUser(This,mainSessionId,cookieData,numCookieBytes,context,pSink)	\
    ( (This)->lpVtbl -> AuthenticateUser(This,mainSessionId,cookieData,numCookieBytes,context,pSink) ) 

#define ITSGAuthenticationEngine_CancelAuthentication(This,mainSessionId,context)	\
    ( (This)->lpVtbl -> CancelAuthentication(This,mainSessionId,context) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITSGAuthenticationEngine_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_tsgauthenticationengine_0000_0002 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_tsgauthenticationengine_0000_0002_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_tsgauthenticationengine_0000_0002_v0_0_s_ifspec;

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


