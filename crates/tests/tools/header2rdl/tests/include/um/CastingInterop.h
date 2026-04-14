

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

#ifndef __castinginterop_h__
#define __castinginterop_h__

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

#ifndef __ICastingEventHandler_FWD_DEFINED__
#define __ICastingEventHandler_FWD_DEFINED__
typedef interface ICastingEventHandler ICastingEventHandler;

#endif 	/* __ICastingEventHandler_FWD_DEFINED__ */


#ifndef __ICastingController_FWD_DEFINED__
#define __ICastingController_FWD_DEFINED__
typedef interface ICastingController ICastingController;

#endif 	/* __ICastingController_FWD_DEFINED__ */


#ifndef __ICastingSourceInfo_FWD_DEFINED__
#define __ICastingSourceInfo_FWD_DEFINED__
typedef interface ICastingSourceInfo ICastingSourceInfo;

#endif 	/* __ICastingSourceInfo_FWD_DEFINED__ */


/* header files for imported files */
#include "objidl.h"
#include "propsys.h"
#include "EventToken.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_castinginterop_0000_0000 */
/* [local] */ 

#if (NTDDI_VERSION >= NTDDI_WINTHRESHOLD)
extern const __declspec(selectany) WCHAR CastingSourceInfo_Property_PreferredSourceUriScheme[] = L"PreferredSourceUriScheme";
extern const __declspec(selectany) WCHAR CastingSourceInfo_Property_CastingTypes[] = L"CastingTypes";
extern const __declspec(selectany) WCHAR CastingSourceInfo_Property_ProtectedMedia[] = L"ProtectedMedia";
typedef 
enum CASTING_CONNECTION_ERROR_STATUS
    {
        CASTING_CONNECTION_ERROR_STATUS_SUCCEEDED	= 0,
        CASTING_CONNECTION_ERROR_STATUS_DEVICE_DID_NOT_RESPOND	= 1,
        CASTING_CONNECTION_ERROR_STATUS_DEVICE_ERROR	= 2,
        CASTING_CONNECTION_ERROR_STATUS_DEVICE_LOCKED	= 3,
        CASTING_CONNECTION_ERROR_STATUS_PROTECTED_PLAYBACK_FAILED	= 4,
        CASTING_CONNECTION_ERROR_STATUS_INVALID_CASTING_SOURCE	= 5,
        CASTING_CONNECTION_ERROR_STATUS_UNKNOWN	= 6
    } 	CASTING_CONNECTION_ERROR_STATUS;

typedef 
enum CASTING_CONNECTION_STATE
    {
        CASTING_CONNECTION_STATE_DISCONNECTED	= 0,
        CASTING_CONNECTION_STATE_CONNECTED	= 1,
        CASTING_CONNECTION_STATE_RENDERING	= 2,
        CASTING_CONNECTION_STATE_DISCONNECTING	= 3,
        CASTING_CONNECTION_STATE_CONNECTING	= 4
    } 	CASTING_CONNECTION_STATE;



extern RPC_IF_HANDLE __MIDL_itf_castinginterop_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_castinginterop_0000_0000_v0_0_s_ifspec;

#ifndef __ICastingEventHandler_INTERFACE_DEFINED__
#define __ICastingEventHandler_INTERFACE_DEFINED__

/* interface ICastingEventHandler */
/* [object][uuid] */ 


EXTERN_C const IID IID_ICastingEventHandler;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("C79A6CB7-BEBD-47a6-A2AD-4D45AD79C7BC")
    ICastingEventHandler : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE OnStateChanged( 
            /* [in] */ CASTING_CONNECTION_STATE newState) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnError( 
            /* [in] */ CASTING_CONNECTION_ERROR_STATUS errorStatus,
            /* [in] */ __RPC__in LPCWSTR errorMessage) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ICastingEventHandlerVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ICastingEventHandler * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ICastingEventHandler * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ICastingEventHandler * This);
        
        DECLSPEC_XFGVIRT(ICastingEventHandler, OnStateChanged)
        HRESULT ( STDMETHODCALLTYPE *OnStateChanged )( 
            __RPC__in ICastingEventHandler * This,
            /* [in] */ CASTING_CONNECTION_STATE newState);
        
        DECLSPEC_XFGVIRT(ICastingEventHandler, OnError)
        HRESULT ( STDMETHODCALLTYPE *OnError )( 
            __RPC__in ICastingEventHandler * This,
            /* [in] */ CASTING_CONNECTION_ERROR_STATUS errorStatus,
            /* [in] */ __RPC__in LPCWSTR errorMessage);
        
        END_INTERFACE
    } ICastingEventHandlerVtbl;

    interface ICastingEventHandler
    {
        CONST_VTBL struct ICastingEventHandlerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ICastingEventHandler_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ICastingEventHandler_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ICastingEventHandler_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ICastingEventHandler_OnStateChanged(This,newState)	\
    ( (This)->lpVtbl -> OnStateChanged(This,newState) ) 

#define ICastingEventHandler_OnError(This,errorStatus,errorMessage)	\
    ( (This)->lpVtbl -> OnError(This,errorStatus,errorMessage) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ICastingEventHandler_INTERFACE_DEFINED__ */


#ifndef __ICastingController_INTERFACE_DEFINED__
#define __ICastingController_INTERFACE_DEFINED__

/* interface ICastingController */
/* [object][uuid] */ 


EXTERN_C const IID IID_ICastingController;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("F0A56423-A664-4fbd-8B43-409A45E8D9A1")
    ICastingController : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Initialize( 
            /* [in] */ __RPC__in_opt IUnknown *castingEngine,
            /* [in] */ __RPC__in_opt IUnknown *castingSource) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Connect( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Disconnect( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Advise( 
            /* [in] */ __RPC__in_opt ICastingEventHandler *eventHandler,
            /* [out] */ __RPC__out DWORD *cookie) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE UnAdvise( 
            /* [in] */ DWORD cookie) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ICastingControllerVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ICastingController * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ICastingController * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ICastingController * This);
        
        DECLSPEC_XFGVIRT(ICastingController, Initialize)
        HRESULT ( STDMETHODCALLTYPE *Initialize )( 
            __RPC__in ICastingController * This,
            /* [in] */ __RPC__in_opt IUnknown *castingEngine,
            /* [in] */ __RPC__in_opt IUnknown *castingSource);
        
        DECLSPEC_XFGVIRT(ICastingController, Connect)
        HRESULT ( STDMETHODCALLTYPE *Connect )( 
            __RPC__in ICastingController * This);
        
        DECLSPEC_XFGVIRT(ICastingController, Disconnect)
        HRESULT ( STDMETHODCALLTYPE *Disconnect )( 
            __RPC__in ICastingController * This);
        
        DECLSPEC_XFGVIRT(ICastingController, Advise)
        HRESULT ( STDMETHODCALLTYPE *Advise )( 
            __RPC__in ICastingController * This,
            /* [in] */ __RPC__in_opt ICastingEventHandler *eventHandler,
            /* [out] */ __RPC__out DWORD *cookie);
        
        DECLSPEC_XFGVIRT(ICastingController, UnAdvise)
        HRESULT ( STDMETHODCALLTYPE *UnAdvise )( 
            __RPC__in ICastingController * This,
            /* [in] */ DWORD cookie);
        
        END_INTERFACE
    } ICastingControllerVtbl;

    interface ICastingController
    {
        CONST_VTBL struct ICastingControllerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ICastingController_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ICastingController_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ICastingController_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ICastingController_Initialize(This,castingEngine,castingSource)	\
    ( (This)->lpVtbl -> Initialize(This,castingEngine,castingSource) ) 

#define ICastingController_Connect(This)	\
    ( (This)->lpVtbl -> Connect(This) ) 

#define ICastingController_Disconnect(This)	\
    ( (This)->lpVtbl -> Disconnect(This) ) 

#define ICastingController_Advise(This,eventHandler,cookie)	\
    ( (This)->lpVtbl -> Advise(This,eventHandler,cookie) ) 

#define ICastingController_UnAdvise(This,cookie)	\
    ( (This)->lpVtbl -> UnAdvise(This,cookie) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ICastingController_INTERFACE_DEFINED__ */


#ifndef __ICastingSourceInfo_INTERFACE_DEFINED__
#define __ICastingSourceInfo_INTERFACE_DEFINED__

/* interface ICastingSourceInfo */
/* [object][uuid] */ 


EXTERN_C const IID IID_ICastingSourceInfo;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("45101AB7-7C3A-4bce-9500-12C09024B298")
    ICastingSourceInfo : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetController( 
            /* [out] */ __RPC__deref_out_opt ICastingController **controller) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetProperties( 
            /* [out] */ __RPC__deref_out_opt INamedPropertyStore **props) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ICastingSourceInfoVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ICastingSourceInfo * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ICastingSourceInfo * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ICastingSourceInfo * This);
        
        DECLSPEC_XFGVIRT(ICastingSourceInfo, GetController)
        HRESULT ( STDMETHODCALLTYPE *GetController )( 
            __RPC__in ICastingSourceInfo * This,
            /* [out] */ __RPC__deref_out_opt ICastingController **controller);
        
        DECLSPEC_XFGVIRT(ICastingSourceInfo, GetProperties)
        HRESULT ( STDMETHODCALLTYPE *GetProperties )( 
            __RPC__in ICastingSourceInfo * This,
            /* [out] */ __RPC__deref_out_opt INamedPropertyStore **props);
        
        END_INTERFACE
    } ICastingSourceInfoVtbl;

    interface ICastingSourceInfo
    {
        CONST_VTBL struct ICastingSourceInfoVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ICastingSourceInfo_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ICastingSourceInfo_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ICastingSourceInfo_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ICastingSourceInfo_GetController(This,controller)	\
    ( (This)->lpVtbl -> GetController(This,controller) ) 

#define ICastingSourceInfo_GetProperties(This,props)	\
    ( (This)->lpVtbl -> GetProperties(This,props) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ICastingSourceInfo_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_castinginterop_0000_0003 */
/* [local] */ 

#endif //(NTDDI_VERSION >= NTDDI_WINTHRESHOLD)


extern RPC_IF_HANDLE __MIDL_itf_castinginterop_0000_0003_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_castinginterop_0000_0003_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


