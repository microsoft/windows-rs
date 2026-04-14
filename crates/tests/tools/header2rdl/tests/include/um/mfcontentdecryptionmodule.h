

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

#ifndef __mfcontentdecryptionmodule_h__
#define __mfcontentdecryptionmodule_h__

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

#ifndef __IMFContentDecryptionModuleSession_FWD_DEFINED__
#define __IMFContentDecryptionModuleSession_FWD_DEFINED__
typedef interface IMFContentDecryptionModuleSession IMFContentDecryptionModuleSession;

#endif 	/* __IMFContentDecryptionModuleSession_FWD_DEFINED__ */


#ifndef __IMFContentDecryptionModuleSessionCallbacks_FWD_DEFINED__
#define __IMFContentDecryptionModuleSessionCallbacks_FWD_DEFINED__
typedef interface IMFContentDecryptionModuleSessionCallbacks IMFContentDecryptionModuleSessionCallbacks;

#endif 	/* __IMFContentDecryptionModuleSessionCallbacks_FWD_DEFINED__ */


#ifndef __IMFContentDecryptionModule_FWD_DEFINED__
#define __IMFContentDecryptionModule_FWD_DEFINED__
typedef interface IMFContentDecryptionModule IMFContentDecryptionModule;

#endif 	/* __IMFContentDecryptionModule_FWD_DEFINED__ */


#ifndef __IMFContentDecryptionModuleAccess_FWD_DEFINED__
#define __IMFContentDecryptionModuleAccess_FWD_DEFINED__
typedef interface IMFContentDecryptionModuleAccess IMFContentDecryptionModuleAccess;

#endif 	/* __IMFContentDecryptionModuleAccess_FWD_DEFINED__ */


#ifndef __IMFContentDecryptionModuleFactory_FWD_DEFINED__
#define __IMFContentDecryptionModuleFactory_FWD_DEFINED__
typedef interface IMFContentDecryptionModuleFactory IMFContentDecryptionModuleFactory;

#endif 	/* __IMFContentDecryptionModuleFactory_FWD_DEFINED__ */


/* header files for imported files */
#include "mfmediaengine.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_mfcontentdecryptionmodule_0000_0000 */
/* [local] */ 

#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP)


extern RPC_IF_HANDLE __MIDL_itf_mfcontentdecryptionmodule_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mfcontentdecryptionmodule_0000_0000_v0_0_s_ifspec;

#ifndef __IMFContentDecryptionModuleSession_INTERFACE_DEFINED__
#define __IMFContentDecryptionModuleSession_INTERFACE_DEFINED__

/* interface IMFContentDecryptionModuleSession */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IMFContentDecryptionModuleSession;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("4e233efd-1dd2-49e8-b577-d63eee4c0d33")
    IMFContentDecryptionModuleSession : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetSessionId( 
            /* [out] */ __RPC__deref_out_opt LPWSTR *sessionId) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetExpiration( 
            /* [out] */ __RPC__out double *expiration) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetKeyStatuses( 
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*numKeyStatuses) MFMediaKeyStatus **keyStatuses,
            /* [out] */ __RPC__out UINT *numKeyStatuses) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Load( 
            /* [in] */ __RPC__in LPCWSTR sessionId,
            /* [out] */ __RPC__out BOOL *loaded) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GenerateRequest( 
            /* [in] */ __RPC__in LPCWSTR initDataType,
            /* [size_is][in] */ __RPC__in_ecount_full(initDataSize) const BYTE *initData,
            /* [in] */ DWORD initDataSize) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Update( 
            /* [size_is][in] */ __RPC__in_ecount_full(responseSize) const BYTE *response,
            /* [in] */ DWORD responseSize) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Close( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Remove( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFContentDecryptionModuleSessionVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMFContentDecryptionModuleSession * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMFContentDecryptionModuleSession * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMFContentDecryptionModuleSession * This);
        
        DECLSPEC_XFGVIRT(IMFContentDecryptionModuleSession, GetSessionId)
        HRESULT ( STDMETHODCALLTYPE *GetSessionId )( 
            __RPC__in IMFContentDecryptionModuleSession * This,
            /* [out] */ __RPC__deref_out_opt LPWSTR *sessionId);
        
        DECLSPEC_XFGVIRT(IMFContentDecryptionModuleSession, GetExpiration)
        HRESULT ( STDMETHODCALLTYPE *GetExpiration )( 
            __RPC__in IMFContentDecryptionModuleSession * This,
            /* [out] */ __RPC__out double *expiration);
        
        DECLSPEC_XFGVIRT(IMFContentDecryptionModuleSession, GetKeyStatuses)
        HRESULT ( STDMETHODCALLTYPE *GetKeyStatuses )( 
            __RPC__in IMFContentDecryptionModuleSession * This,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*numKeyStatuses) MFMediaKeyStatus **keyStatuses,
            /* [out] */ __RPC__out UINT *numKeyStatuses);
        
        DECLSPEC_XFGVIRT(IMFContentDecryptionModuleSession, Load)
        HRESULT ( STDMETHODCALLTYPE *Load )( 
            __RPC__in IMFContentDecryptionModuleSession * This,
            /* [in] */ __RPC__in LPCWSTR sessionId,
            /* [out] */ __RPC__out BOOL *loaded);
        
        DECLSPEC_XFGVIRT(IMFContentDecryptionModuleSession, GenerateRequest)
        HRESULT ( STDMETHODCALLTYPE *GenerateRequest )( 
            __RPC__in IMFContentDecryptionModuleSession * This,
            /* [in] */ __RPC__in LPCWSTR initDataType,
            /* [size_is][in] */ __RPC__in_ecount_full(initDataSize) const BYTE *initData,
            /* [in] */ DWORD initDataSize);
        
        DECLSPEC_XFGVIRT(IMFContentDecryptionModuleSession, Update)
        HRESULT ( STDMETHODCALLTYPE *Update )( 
            __RPC__in IMFContentDecryptionModuleSession * This,
            /* [size_is][in] */ __RPC__in_ecount_full(responseSize) const BYTE *response,
            /* [in] */ DWORD responseSize);
        
        DECLSPEC_XFGVIRT(IMFContentDecryptionModuleSession, Close)
        HRESULT ( STDMETHODCALLTYPE *Close )( 
            __RPC__in IMFContentDecryptionModuleSession * This);
        
        DECLSPEC_XFGVIRT(IMFContentDecryptionModuleSession, Remove)
        HRESULT ( STDMETHODCALLTYPE *Remove )( 
            __RPC__in IMFContentDecryptionModuleSession * This);
        
        END_INTERFACE
    } IMFContentDecryptionModuleSessionVtbl;

    interface IMFContentDecryptionModuleSession
    {
        CONST_VTBL struct IMFContentDecryptionModuleSessionVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFContentDecryptionModuleSession_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFContentDecryptionModuleSession_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFContentDecryptionModuleSession_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFContentDecryptionModuleSession_GetSessionId(This,sessionId)	\
    ( (This)->lpVtbl -> GetSessionId(This,sessionId) ) 

#define IMFContentDecryptionModuleSession_GetExpiration(This,expiration)	\
    ( (This)->lpVtbl -> GetExpiration(This,expiration) ) 

#define IMFContentDecryptionModuleSession_GetKeyStatuses(This,keyStatuses,numKeyStatuses)	\
    ( (This)->lpVtbl -> GetKeyStatuses(This,keyStatuses,numKeyStatuses) ) 

#define IMFContentDecryptionModuleSession_Load(This,sessionId,loaded)	\
    ( (This)->lpVtbl -> Load(This,sessionId,loaded) ) 

#define IMFContentDecryptionModuleSession_GenerateRequest(This,initDataType,initData,initDataSize)	\
    ( (This)->lpVtbl -> GenerateRequest(This,initDataType,initData,initDataSize) ) 

#define IMFContentDecryptionModuleSession_Update(This,response,responseSize)	\
    ( (This)->lpVtbl -> Update(This,response,responseSize) ) 

#define IMFContentDecryptionModuleSession_Close(This)	\
    ( (This)->lpVtbl -> Close(This) ) 

#define IMFContentDecryptionModuleSession_Remove(This)	\
    ( (This)->lpVtbl -> Remove(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFContentDecryptionModuleSession_INTERFACE_DEFINED__ */


#ifndef __IMFContentDecryptionModuleSessionCallbacks_INTERFACE_DEFINED__
#define __IMFContentDecryptionModuleSessionCallbacks_INTERFACE_DEFINED__

/* interface IMFContentDecryptionModuleSessionCallbacks */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IMFContentDecryptionModuleSessionCallbacks;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("3f96ee40-ad81-4096-8470-59a4b770f89a")
    IMFContentDecryptionModuleSessionCallbacks : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE KeyMessage( 
            /* [in] */ MF_MEDIAKEYSESSION_MESSAGETYPE messageType,
            /* [size_is][in] */ __RPC__in_ecount_full(messageSize) const BYTE *message,
            /* [in] */ DWORD messageSize,
            /* [optional][in] */ __RPC__in LPCWSTR destinationURL) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE KeyStatusChanged( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFContentDecryptionModuleSessionCallbacksVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMFContentDecryptionModuleSessionCallbacks * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMFContentDecryptionModuleSessionCallbacks * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMFContentDecryptionModuleSessionCallbacks * This);
        
        DECLSPEC_XFGVIRT(IMFContentDecryptionModuleSessionCallbacks, KeyMessage)
        HRESULT ( STDMETHODCALLTYPE *KeyMessage )( 
            __RPC__in IMFContentDecryptionModuleSessionCallbacks * This,
            /* [in] */ MF_MEDIAKEYSESSION_MESSAGETYPE messageType,
            /* [size_is][in] */ __RPC__in_ecount_full(messageSize) const BYTE *message,
            /* [in] */ DWORD messageSize,
            /* [optional][in] */ __RPC__in LPCWSTR destinationURL);
        
        DECLSPEC_XFGVIRT(IMFContentDecryptionModuleSessionCallbacks, KeyStatusChanged)
        HRESULT ( STDMETHODCALLTYPE *KeyStatusChanged )( 
            __RPC__in IMFContentDecryptionModuleSessionCallbacks * This);
        
        END_INTERFACE
    } IMFContentDecryptionModuleSessionCallbacksVtbl;

    interface IMFContentDecryptionModuleSessionCallbacks
    {
        CONST_VTBL struct IMFContentDecryptionModuleSessionCallbacksVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFContentDecryptionModuleSessionCallbacks_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFContentDecryptionModuleSessionCallbacks_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFContentDecryptionModuleSessionCallbacks_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFContentDecryptionModuleSessionCallbacks_KeyMessage(This,messageType,message,messageSize,destinationURL)	\
    ( (This)->lpVtbl -> KeyMessage(This,messageType,message,messageSize,destinationURL) ) 

#define IMFContentDecryptionModuleSessionCallbacks_KeyStatusChanged(This)	\
    ( (This)->lpVtbl -> KeyStatusChanged(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFContentDecryptionModuleSessionCallbacks_INTERFACE_DEFINED__ */


#ifndef __IMFContentDecryptionModule_INTERFACE_DEFINED__
#define __IMFContentDecryptionModule_INTERFACE_DEFINED__

/* interface IMFContentDecryptionModule */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IMFContentDecryptionModule;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("87be986c-10be-4943-bf48-4b54ce1983a2")
    IMFContentDecryptionModule : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SetContentEnabler( 
            /* [in] */ __RPC__in_opt IMFContentEnabler *contentEnabler,
            /* [in] */ __RPC__in_opt IMFAsyncResult *result) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetSuspendNotify( 
            /* [out] */ __RPC__deref_out_opt IMFCdmSuspendNotify **notify) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetPMPHostApp( 
            /* [in] */ __RPC__in_opt IMFPMPHostApp *pmpHostApp) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreateSession( 
            /* [in] */ MF_MEDIAKEYSESSION_TYPE sessionType,
            /* [in] */ __RPC__in_opt IMFContentDecryptionModuleSessionCallbacks *callbacks,
            /* [out] */ __RPC__deref_out_opt IMFContentDecryptionModuleSession **session) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetServerCertificate( 
            /* [size_is][in] */ __RPC__in_ecount_full(certificateSize) const BYTE *certificate,
            /* [in] */ DWORD certificateSize) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreateTrustedInput( 
            /* [size_is][in] */ __RPC__in_ecount_full(contentInitDataSize) const BYTE *contentInitData,
            /* [in] */ DWORD contentInitDataSize,
            /* [out] */ __RPC__deref_out_opt IMFTrustedInput **trustedInput) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetProtectionSystemIds( 
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*count) GUID **systemIds,
            /* [out] */ __RPC__out DWORD *count) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFContentDecryptionModuleVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMFContentDecryptionModule * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMFContentDecryptionModule * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMFContentDecryptionModule * This);
        
        DECLSPEC_XFGVIRT(IMFContentDecryptionModule, SetContentEnabler)
        HRESULT ( STDMETHODCALLTYPE *SetContentEnabler )( 
            __RPC__in IMFContentDecryptionModule * This,
            /* [in] */ __RPC__in_opt IMFContentEnabler *contentEnabler,
            /* [in] */ __RPC__in_opt IMFAsyncResult *result);
        
        DECLSPEC_XFGVIRT(IMFContentDecryptionModule, GetSuspendNotify)
        HRESULT ( STDMETHODCALLTYPE *GetSuspendNotify )( 
            __RPC__in IMFContentDecryptionModule * This,
            /* [out] */ __RPC__deref_out_opt IMFCdmSuspendNotify **notify);
        
        DECLSPEC_XFGVIRT(IMFContentDecryptionModule, SetPMPHostApp)
        HRESULT ( STDMETHODCALLTYPE *SetPMPHostApp )( 
            __RPC__in IMFContentDecryptionModule * This,
            /* [in] */ __RPC__in_opt IMFPMPHostApp *pmpHostApp);
        
        DECLSPEC_XFGVIRT(IMFContentDecryptionModule, CreateSession)
        HRESULT ( STDMETHODCALLTYPE *CreateSession )( 
            __RPC__in IMFContentDecryptionModule * This,
            /* [in] */ MF_MEDIAKEYSESSION_TYPE sessionType,
            /* [in] */ __RPC__in_opt IMFContentDecryptionModuleSessionCallbacks *callbacks,
            /* [out] */ __RPC__deref_out_opt IMFContentDecryptionModuleSession **session);
        
        DECLSPEC_XFGVIRT(IMFContentDecryptionModule, SetServerCertificate)
        HRESULT ( STDMETHODCALLTYPE *SetServerCertificate )( 
            __RPC__in IMFContentDecryptionModule * This,
            /* [size_is][in] */ __RPC__in_ecount_full(certificateSize) const BYTE *certificate,
            /* [in] */ DWORD certificateSize);
        
        DECLSPEC_XFGVIRT(IMFContentDecryptionModule, CreateTrustedInput)
        HRESULT ( STDMETHODCALLTYPE *CreateTrustedInput )( 
            __RPC__in IMFContentDecryptionModule * This,
            /* [size_is][in] */ __RPC__in_ecount_full(contentInitDataSize) const BYTE *contentInitData,
            /* [in] */ DWORD contentInitDataSize,
            /* [out] */ __RPC__deref_out_opt IMFTrustedInput **trustedInput);
        
        DECLSPEC_XFGVIRT(IMFContentDecryptionModule, GetProtectionSystemIds)
        HRESULT ( STDMETHODCALLTYPE *GetProtectionSystemIds )( 
            __RPC__in IMFContentDecryptionModule * This,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*count) GUID **systemIds,
            /* [out] */ __RPC__out DWORD *count);
        
        END_INTERFACE
    } IMFContentDecryptionModuleVtbl;

    interface IMFContentDecryptionModule
    {
        CONST_VTBL struct IMFContentDecryptionModuleVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFContentDecryptionModule_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFContentDecryptionModule_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFContentDecryptionModule_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFContentDecryptionModule_SetContentEnabler(This,contentEnabler,result)	\
    ( (This)->lpVtbl -> SetContentEnabler(This,contentEnabler,result) ) 

#define IMFContentDecryptionModule_GetSuspendNotify(This,notify)	\
    ( (This)->lpVtbl -> GetSuspendNotify(This,notify) ) 

#define IMFContentDecryptionModule_SetPMPHostApp(This,pmpHostApp)	\
    ( (This)->lpVtbl -> SetPMPHostApp(This,pmpHostApp) ) 

#define IMFContentDecryptionModule_CreateSession(This,sessionType,callbacks,session)	\
    ( (This)->lpVtbl -> CreateSession(This,sessionType,callbacks,session) ) 

#define IMFContentDecryptionModule_SetServerCertificate(This,certificate,certificateSize)	\
    ( (This)->lpVtbl -> SetServerCertificate(This,certificate,certificateSize) ) 

#define IMFContentDecryptionModule_CreateTrustedInput(This,contentInitData,contentInitDataSize,trustedInput)	\
    ( (This)->lpVtbl -> CreateTrustedInput(This,contentInitData,contentInitDataSize,trustedInput) ) 

#define IMFContentDecryptionModule_GetProtectionSystemIds(This,systemIds,count)	\
    ( (This)->lpVtbl -> GetProtectionSystemIds(This,systemIds,count) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFContentDecryptionModule_INTERFACE_DEFINED__ */


#ifndef __IMFContentDecryptionModuleAccess_INTERFACE_DEFINED__
#define __IMFContentDecryptionModuleAccess_INTERFACE_DEFINED__

/* interface IMFContentDecryptionModuleAccess */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IMFContentDecryptionModuleAccess;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("a853d1f4-e2a0-4303-9edc-f1a68ee43136")
    IMFContentDecryptionModuleAccess : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE CreateContentDecryptionModule( 
            /* [in] */ __RPC__in_opt IPropertyStore *contentDecryptionModuleProperties,
            /* [out] */ __RPC__deref_out_opt IMFContentDecryptionModule **contentDecryptionModule) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetConfiguration( 
            /* [out] */ __RPC__deref_out_opt IPropertyStore **configuration) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetKeySystem( 
            /* [out] */ __RPC__deref_out_opt LPWSTR *keySystem) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFContentDecryptionModuleAccessVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMFContentDecryptionModuleAccess * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMFContentDecryptionModuleAccess * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMFContentDecryptionModuleAccess * This);
        
        DECLSPEC_XFGVIRT(IMFContentDecryptionModuleAccess, CreateContentDecryptionModule)
        HRESULT ( STDMETHODCALLTYPE *CreateContentDecryptionModule )( 
            __RPC__in IMFContentDecryptionModuleAccess * This,
            /* [in] */ __RPC__in_opt IPropertyStore *contentDecryptionModuleProperties,
            /* [out] */ __RPC__deref_out_opt IMFContentDecryptionModule **contentDecryptionModule);
        
        DECLSPEC_XFGVIRT(IMFContentDecryptionModuleAccess, GetConfiguration)
        HRESULT ( STDMETHODCALLTYPE *GetConfiguration )( 
            __RPC__in IMFContentDecryptionModuleAccess * This,
            /* [out] */ __RPC__deref_out_opt IPropertyStore **configuration);
        
        DECLSPEC_XFGVIRT(IMFContentDecryptionModuleAccess, GetKeySystem)
        HRESULT ( STDMETHODCALLTYPE *GetKeySystem )( 
            __RPC__in IMFContentDecryptionModuleAccess * This,
            /* [out] */ __RPC__deref_out_opt LPWSTR *keySystem);
        
        END_INTERFACE
    } IMFContentDecryptionModuleAccessVtbl;

    interface IMFContentDecryptionModuleAccess
    {
        CONST_VTBL struct IMFContentDecryptionModuleAccessVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFContentDecryptionModuleAccess_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFContentDecryptionModuleAccess_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFContentDecryptionModuleAccess_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFContentDecryptionModuleAccess_CreateContentDecryptionModule(This,contentDecryptionModuleProperties,contentDecryptionModule)	\
    ( (This)->lpVtbl -> CreateContentDecryptionModule(This,contentDecryptionModuleProperties,contentDecryptionModule) ) 

#define IMFContentDecryptionModuleAccess_GetConfiguration(This,configuration)	\
    ( (This)->lpVtbl -> GetConfiguration(This,configuration) ) 

#define IMFContentDecryptionModuleAccess_GetKeySystem(This,keySystem)	\
    ( (This)->lpVtbl -> GetKeySystem(This,keySystem) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFContentDecryptionModuleAccess_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mfcontentdecryptionmodule_0000_0004 */
/* [local] */ 

EXTERN_C const DECLSPEC_SELECTANY PROPERTYKEY MF_CONTENTDECRYPTIONMODULE_INPRIVATESTOREPATH = { { 0x730cb3ac, 0x51dc, 0x49da, { 0xa5, 0x78, 0xb9, 0x53, 0x86, 0xb6, 0x2a, 0xfe } }, 0x01 }; 
EXTERN_C const DECLSPEC_SELECTANY PROPERTYKEY MF_CONTENTDECRYPTIONMODULE_STOREPATH =          { { 0x77d993b9, 0xba61, 0x4bb7, { 0x92, 0xc6, 0x18, 0xc8, 0x6a, 0x18, 0x9c, 0x06 } }, 0x02 }; 
EXTERN_C const DECLSPEC_SELECTANY PROPERTYKEY MF_CONTENTDECRYPTIONMODULE_PMPSTORECONTEXT =    { { 0x6d2a2835, 0xc3a9, 0x4681, { 0x97, 0xf2, 0x0a, 0xf5, 0x6b, 0xe9, 0x34, 0x46 } }, 0x03 }; 


extern RPC_IF_HANDLE __MIDL_itf_mfcontentdecryptionmodule_0000_0004_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mfcontentdecryptionmodule_0000_0004_v0_0_s_ifspec;

#ifndef __IMFContentDecryptionModuleFactory_INTERFACE_DEFINED__
#define __IMFContentDecryptionModuleFactory_INTERFACE_DEFINED__

/* interface IMFContentDecryptionModuleFactory */
/* [local][uuid][object] */ 


EXTERN_C const IID IID_IMFContentDecryptionModuleFactory;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("7d5abf16-4cbb-4e08-b977-9ba59049943e")
    IMFContentDecryptionModuleFactory : public IUnknown
    {
    public:
        virtual BOOL STDMETHODCALLTYPE IsTypeSupported( 
            /* [in] */ LPCWSTR keySystem,
            /* [optional][in] */ LPCWSTR contentType) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreateContentDecryptionModuleAccess( 
            /* [in] */ LPCWSTR keySystem,
            /* [size_is][size_is][in] */ IPropertyStore **configurations,
            /* [in] */ DWORD numConfigurations,
            /* [out] */ IMFContentDecryptionModuleAccess **contentDecryptionModuleAccess) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFContentDecryptionModuleFactoryVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMFContentDecryptionModuleFactory * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMFContentDecryptionModuleFactory * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMFContentDecryptionModuleFactory * This);
        
        DECLSPEC_XFGVIRT(IMFContentDecryptionModuleFactory, IsTypeSupported)
        BOOL ( STDMETHODCALLTYPE *IsTypeSupported )( 
            IMFContentDecryptionModuleFactory * This,
            /* [in] */ LPCWSTR keySystem,
            /* [optional][in] */ LPCWSTR contentType);
        
        DECLSPEC_XFGVIRT(IMFContentDecryptionModuleFactory, CreateContentDecryptionModuleAccess)
        HRESULT ( STDMETHODCALLTYPE *CreateContentDecryptionModuleAccess )( 
            IMFContentDecryptionModuleFactory * This,
            /* [in] */ LPCWSTR keySystem,
            /* [size_is][size_is][in] */ IPropertyStore **configurations,
            /* [in] */ DWORD numConfigurations,
            /* [out] */ IMFContentDecryptionModuleAccess **contentDecryptionModuleAccess);
        
        END_INTERFACE
    } IMFContentDecryptionModuleFactoryVtbl;

    interface IMFContentDecryptionModuleFactory
    {
        CONST_VTBL struct IMFContentDecryptionModuleFactoryVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFContentDecryptionModuleFactory_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFContentDecryptionModuleFactory_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFContentDecryptionModuleFactory_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFContentDecryptionModuleFactory_IsTypeSupported(This,keySystem,contentType)	\
    ( (This)->lpVtbl -> IsTypeSupported(This,keySystem,contentType) ) 

#define IMFContentDecryptionModuleFactory_CreateContentDecryptionModuleAccess(This,keySystem,configurations,numConfigurations,contentDecryptionModuleAccess)	\
    ( (This)->lpVtbl -> CreateContentDecryptionModuleAccess(This,keySystem,configurations,numConfigurations,contentDecryptionModuleAccess) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFContentDecryptionModuleFactory_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mfcontentdecryptionmodule_0000_0005 */
/* [local] */ 

EXTERN_GUID( MF_CONTENTDECRYPTIONMODULE_SERVICE, 0x15320c45, 0xff80, 0x484a, 0x9d, 0xcb, 0xd, 0xf8, 0x94, 0xe6, 0x9a, 0x1);
EXTERN_GUID(MF_ENCRYPTEDMEDIAEXTENSIONS_ACTIVATE, 
    0x2df7b51e, 0x797b, 0x4d06, 0xbe, 0x71, 0xd1, 0x4a, 0x52, 0xcf, 0x84, 0x21);
EXTERN_GUID(MF_ENCRYPTEDMEDIAEXTENSIONS_ACTIVATABLE_CLASS_ID,
    0x77631a31, 0xe5e7, 0x4785, 0xbf, 0x17, 0x20, 0xf5, 0x7b, 0x22, 0x48, 0x02);
EXTERN_GUID(MF_ENCRYPTEDMEDIAEXTENSIONS_INITIALIZATION_DATA,
    0x3e73735c, 0xe6c0, 0x481d, 0x82, 0x60, 0xee, 0x5d, 0xb1, 0x34, 0x3b, 0x5f);
STDAPI MFCreateEncryptedMediaExtensionsStoreActivate(
    _In_ IMFPMPHostApp *pmpHost,
    _In_ IStream *objectStream,
    _In_ LPCWSTR classId,
    _Outptr_ IMFActivate** activate
    );
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_mfcontentdecryptionmodule_0000_0005_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mfcontentdecryptionmodule_0000_0005_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


