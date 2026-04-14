

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


#ifndef __rendezvoussession_h__
#define __rendezvoussession_h__

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

#ifndef __IRendezvousSession_FWD_DEFINED__
#define __IRendezvousSession_FWD_DEFINED__
typedef interface IRendezvousSession IRendezvousSession;

#endif 	/* __IRendezvousSession_FWD_DEFINED__ */


#ifndef __DRendezvousSessionEvents_FWD_DEFINED__
#define __DRendezvousSessionEvents_FWD_DEFINED__
typedef interface DRendezvousSessionEvents DRendezvousSessionEvents;

#endif 	/* __DRendezvousSessionEvents_FWD_DEFINED__ */


#ifndef __IRendezvousApplication_FWD_DEFINED__
#define __IRendezvousApplication_FWD_DEFINED__
typedef interface IRendezvousApplication IRendezvousApplication;

#endif 	/* __IRendezvousApplication_FWD_DEFINED__ */


#ifndef __RendezvousApplication_FWD_DEFINED__
#define __RendezvousApplication_FWD_DEFINED__

#ifdef __cplusplus
typedef class RendezvousApplication RendezvousApplication;
#else
typedef struct RendezvousApplication RendezvousApplication;
#endif /* __cplusplus */

#endif 	/* __RendezvousApplication_FWD_DEFINED__ */


/* header files for imported files */
#include "oaidl.h"
#include "ocidl.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_rendezvoussession_0000_0000 */
/* [local] */ 

#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
#define	DISPID_EVENT_ON_STATE_CHANGED	( 5 )

#define	DISPID_EVENT_ON_TERMINATION	( 6 )

#define	DISPID_EVENT_ON_CONTEXT_DATA	( 7 )

#define	DISPID_EVENT_ON_SEND_ERROR	( 8 )



extern RPC_IF_HANDLE __MIDL_itf_rendezvoussession_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_rendezvoussession_0000_0000_v0_0_s_ifspec;


#ifndef __RendezvousSessionLib_LIBRARY_DEFINED__
#define __RendezvousSessionLib_LIBRARY_DEFINED__

/* library RendezvousSessionLib */
/* [helpstring][version][uuid] */ 

typedef /* [public][public][public] */ 
enum __MIDL___MIDL_itf_rendezvoussession_0000_0000_0001
    {
        RSS_UNKNOWN	= 0,
        RSS_READY	= ( RSS_UNKNOWN + 1 ) ,
        RSS_INVITATION	= ( RSS_READY + 1 ) ,
        RSS_ACCEPTED	= ( RSS_INVITATION + 1 ) ,
        RSS_CONNECTED	= ( RSS_ACCEPTED + 1 ) ,
        RSS_CANCELLED	= ( RSS_CONNECTED + 1 ) ,
        RSS_DECLINED	= ( RSS_CANCELLED + 1 ) ,
        RSS_TERMINATED	= ( RSS_DECLINED + 1 ) 
    } 	RENDEZVOUS_SESSION_STATE;

typedef /* [public] */ 
enum __MIDL___MIDL_itf_rendezvoussession_0000_0000_0002
    {
        RSF_NONE	= 0,
        RSF_INVITER	= 0x1,
        RSF_INVITEE	= 0x2,
        RSF_ORIGINAL_INVITER	= 0x4,
        RSF_REMOTE_LEGACYSESSION	= 0x8,
        RSF_REMOTE_WIN7SESSION	= 0x10
    } 	RENDEZVOUS_SESSION_FLAGS;


EXTERN_C const IID LIBID_RendezvousSessionLib;

#ifndef __IRendezvousSession_INTERFACE_DEFINED__
#define __IRendezvousSession_INTERFACE_DEFINED__

/* interface IRendezvousSession */
/* [unique][oleautomation][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IRendezvousSession;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("9BA4B1DD-8B0C-48B7-9E7C-2F25857C8DF5")
    IRendezvousSession : public IUnknown
    {
    public:
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_State( 
            /* [retval][out] */ __RPC__out RENDEZVOUS_SESSION_STATE *pSessionState) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_RemoteUser( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *bstrUserName) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_Flags( 
            /* [retval][out] */ __RPC__out LONG *pFlags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SendContextData( 
            /* [in] */ __RPC__in BSTR bstrData) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Terminate( 
            /* [in] */ HRESULT hr,
            /* [in] */ __RPC__in BSTR bstrAppData) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IRendezvousSessionVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IRendezvousSession * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IRendezvousSession * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IRendezvousSession * This);
        
        DECLSPEC_XFGVIRT(IRendezvousSession, get_State)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_State )( 
            __RPC__in IRendezvousSession * This,
            /* [retval][out] */ __RPC__out RENDEZVOUS_SESSION_STATE *pSessionState);
        
        DECLSPEC_XFGVIRT(IRendezvousSession, get_RemoteUser)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_RemoteUser )( 
            __RPC__in IRendezvousSession * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *bstrUserName);
        
        DECLSPEC_XFGVIRT(IRendezvousSession, get_Flags)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Flags )( 
            __RPC__in IRendezvousSession * This,
            /* [retval][out] */ __RPC__out LONG *pFlags);
        
        DECLSPEC_XFGVIRT(IRendezvousSession, SendContextData)
        HRESULT ( STDMETHODCALLTYPE *SendContextData )( 
            __RPC__in IRendezvousSession * This,
            /* [in] */ __RPC__in BSTR bstrData);
        
        DECLSPEC_XFGVIRT(IRendezvousSession, Terminate)
        HRESULT ( STDMETHODCALLTYPE *Terminate )( 
            __RPC__in IRendezvousSession * This,
            /* [in] */ HRESULT hr,
            /* [in] */ __RPC__in BSTR bstrAppData);
        
        END_INTERFACE
    } IRendezvousSessionVtbl;

    interface IRendezvousSession
    {
        CONST_VTBL struct IRendezvousSessionVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IRendezvousSession_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IRendezvousSession_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IRendezvousSession_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IRendezvousSession_get_State(This,pSessionState)	\
    ( (This)->lpVtbl -> get_State(This,pSessionState) ) 

#define IRendezvousSession_get_RemoteUser(This,bstrUserName)	\
    ( (This)->lpVtbl -> get_RemoteUser(This,bstrUserName) ) 

#define IRendezvousSession_get_Flags(This,pFlags)	\
    ( (This)->lpVtbl -> get_Flags(This,pFlags) ) 

#define IRendezvousSession_SendContextData(This,bstrData)	\
    ( (This)->lpVtbl -> SendContextData(This,bstrData) ) 

#define IRendezvousSession_Terminate(This,hr,bstrAppData)	\
    ( (This)->lpVtbl -> Terminate(This,hr,bstrAppData) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IRendezvousSession_INTERFACE_DEFINED__ */


#ifndef __DRendezvousSessionEvents_DISPINTERFACE_DEFINED__
#define __DRendezvousSessionEvents_DISPINTERFACE_DEFINED__

/* dispinterface DRendezvousSessionEvents */
/* [uuid] */ 


EXTERN_C const IID DIID_DRendezvousSessionEvents;

#if defined(__cplusplus) && !defined(CINTERFACE)

    MIDL_INTERFACE("3FA19CF8-64C4-4F53-AE60-635B3806ECA6")
    DRendezvousSessionEvents : public IDispatch
    {
    };
    
#else 	/* C style interface */

    typedef struct DRendezvousSessionEventsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in DRendezvousSessionEvents * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in DRendezvousSessionEvents * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in DRendezvousSessionEvents * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in DRendezvousSessionEvents * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in DRendezvousSessionEvents * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in DRendezvousSessionEvents * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            DRendezvousSessionEvents * This,
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
        
        END_INTERFACE
    } DRendezvousSessionEventsVtbl;

    interface DRendezvousSessionEvents
    {
        CONST_VTBL struct DRendezvousSessionEventsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define DRendezvousSessionEvents_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define DRendezvousSessionEvents_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define DRendezvousSessionEvents_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define DRendezvousSessionEvents_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define DRendezvousSessionEvents_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define DRendezvousSessionEvents_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define DRendezvousSessionEvents_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */


#endif 	/* __DRendezvousSessionEvents_DISPINTERFACE_DEFINED__ */


#ifndef __IRendezvousApplication_INTERFACE_DEFINED__
#define __IRendezvousApplication_INTERFACE_DEFINED__

/* interface IRendezvousApplication */
/* [unique][oleautomation][uuid][object] */ 


EXTERN_C const IID IID_IRendezvousApplication;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("4F4D070B-A275-49FB-B10D-8EC26387B50D")
    IRendezvousApplication : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SetRendezvousSession( 
            /* [in] */ __RPC__in_opt IUnknown *pRendezvousSession) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IRendezvousApplicationVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IRendezvousApplication * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IRendezvousApplication * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IRendezvousApplication * This);
        
        DECLSPEC_XFGVIRT(IRendezvousApplication, SetRendezvousSession)
        HRESULT ( STDMETHODCALLTYPE *SetRendezvousSession )( 
            __RPC__in IRendezvousApplication * This,
            /* [in] */ __RPC__in_opt IUnknown *pRendezvousSession);
        
        END_INTERFACE
    } IRendezvousApplicationVtbl;

    interface IRendezvousApplication
    {
        CONST_VTBL struct IRendezvousApplicationVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IRendezvousApplication_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IRendezvousApplication_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IRendezvousApplication_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IRendezvousApplication_SetRendezvousSession(This,pRendezvousSession)	\
    ( (This)->lpVtbl -> SetRendezvousSession(This,pRendezvousSession) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IRendezvousApplication_INTERFACE_DEFINED__ */


EXTERN_C const CLSID CLSID_RendezvousApplication;

#ifdef __cplusplus

class DECLSPEC_UUID("0B7E019A-B5DE-47fa-8966-9082F82FB192")
RendezvousApplication;
#endif
#endif /* __RendezvousSessionLib_LIBRARY_DEFINED__ */

/* interface __MIDL_itf_rendezvoussession_0000_0001 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_rendezvoussession_0000_0001_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_rendezvoussession_0000_0001_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


