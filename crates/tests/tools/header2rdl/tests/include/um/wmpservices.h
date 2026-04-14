

/* this ALWAYS GENERATED file contains the definitions for the interfaces */


 /* File created by MIDL compiler version 8.01.0628 */
/* @@MIDL_FILE_HEADING(  ) */

#pragma warning( disable: 4049 )  /* more than 64k source lines */


/* verify that the <rpcndr.h> version is high enough to compile this file*/
#ifndef __REQUIRED_RPCNDR_H_VERSION__
#define __REQUIRED_RPCNDR_H_VERSION__ 475
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

#ifndef __wmpservices_h__
#define __wmpservices_h__

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

#ifndef __IWMPServices_FWD_DEFINED__
#define __IWMPServices_FWD_DEFINED__
typedef interface IWMPServices IWMPServices;

#endif 	/* __IWMPServices_FWD_DEFINED__ */


#ifndef __IWMPMediaPluginRegistrar_FWD_DEFINED__
#define __IWMPMediaPluginRegistrar_FWD_DEFINED__
typedef interface IWMPMediaPluginRegistrar IWMPMediaPluginRegistrar;

#endif 	/* __IWMPMediaPluginRegistrar_FWD_DEFINED__ */


#ifndef __IWMPPlugin_FWD_DEFINED__
#define __IWMPPlugin_FWD_DEFINED__
typedef interface IWMPPlugin IWMPPlugin;

#endif 	/* __IWMPPlugin_FWD_DEFINED__ */


#ifndef __IWMPPluginEnable_FWD_DEFINED__
#define __IWMPPluginEnable_FWD_DEFINED__
typedef interface IWMPPluginEnable IWMPPluginEnable;

#endif 	/* __IWMPPluginEnable_FWD_DEFINED__ */


#ifndef __IWMPGraphCreation_FWD_DEFINED__
#define __IWMPGraphCreation_FWD_DEFINED__
typedef interface IWMPGraphCreation IWMPGraphCreation;

#endif 	/* __IWMPGraphCreation_FWD_DEFINED__ */


#ifndef __IWMPConvert_FWD_DEFINED__
#define __IWMPConvert_FWD_DEFINED__
typedef interface IWMPConvert IWMPConvert;

#endif 	/* __IWMPConvert_FWD_DEFINED__ */


#ifndef __IWMPTranscodePolicy_FWD_DEFINED__
#define __IWMPTranscodePolicy_FWD_DEFINED__
typedef interface IWMPTranscodePolicy IWMPTranscodePolicy;

#endif 	/* __IWMPTranscodePolicy_FWD_DEFINED__ */


#ifndef __IWMPUserEventSink_FWD_DEFINED__
#define __IWMPUserEventSink_FWD_DEFINED__
typedef interface IWMPUserEventSink IWMPUserEventSink;

#endif 	/* __IWMPUserEventSink_FWD_DEFINED__ */


/* header files for imported files */
#include "oaidl.h"
#include "ocidl.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_wmpservices_0000_0000 */
/* [local] */ 

//=========================================================================
//
// Microsoft Windows Media Technologies
// Copyright (C) Microsoft Corporation. All rights reserved.
//
//=========================================================================
#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
typedef /* [public][helpstring] */ 
enum WMPServices_StreamState
    {
        WMPServices_StreamState_Stop	= 0,
        WMPServices_StreamState_Pause	= ( WMPServices_StreamState_Stop + 1 ) ,
        WMPServices_StreamState_Play	= ( WMPServices_StreamState_Pause + 1 ) 
    } 	WMPServices_StreamState;



extern RPC_IF_HANDLE __MIDL_itf_wmpservices_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_wmpservices_0000_0000_v0_0_s_ifspec;

#ifndef __IWMPServices_INTERFACE_DEFINED__
#define __IWMPServices_INTERFACE_DEFINED__

/* interface IWMPServices */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IWMPServices;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("AFB6B76B-1E20-4198-83B3-191DB6E0B149")
    IWMPServices : public IUnknown
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetStreamTime( 
            /* [retval][out] */ LONGLONG *prt) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetStreamState( 
            /* [retval][out] */ WMPServices_StreamState *pState) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWMPServicesVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IWMPServices * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IWMPServices * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IWMPServices * This);
        
        DECLSPEC_XFGVIRT(IWMPServices, GetStreamTime)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetStreamTime )( 
            IWMPServices * This,
            /* [retval][out] */ LONGLONG *prt);
        
        DECLSPEC_XFGVIRT(IWMPServices, GetStreamState)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetStreamState )( 
            IWMPServices * This,
            /* [retval][out] */ WMPServices_StreamState *pState);
        
        END_INTERFACE
    } IWMPServicesVtbl;

    interface IWMPServices
    {
        CONST_VTBL struct IWMPServicesVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWMPServices_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWMPServices_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWMPServices_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWMPServices_GetStreamTime(This,prt)	\
    ( (This)->lpVtbl -> GetStreamTime(This,prt) ) 

#define IWMPServices_GetStreamState(This,pState)	\
    ( (This)->lpVtbl -> GetStreamState(This,pState) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWMPServices_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_wmpservices_0000_0001 */
/* [local] */ 

EXTERN_GUID( CLSID_WMPMediaPluginRegistrar, 0x5569e7f5,0x424b,0x4b93, 0x89, 0xca, 0x79, 0xd1, 0x79, 0x24, 0x68, 0x9a );
EXTERN_GUID( WMP_PLUGINTYPE_DSP, 0x6434baea, 0x4954, 0x498d, 0xab, 0xd5, 0x2b, 0x7, 0x12, 0x3e, 0x1f, 0x4 );
EXTERN_GUID( WMP_PLUGINTYPE_DSP_OUTOFPROC, 0xef29b174, 0xc347, 0x44cc, 0x9a, 0x4f, 0x23, 0x99, 0x11, 0x8f, 0xf3, 0x8c );
EXTERN_GUID( WMP_PLUGINTYPE_RENDERING, 0xa8554541, 0x115d, 0x406a, 0xa4, 0xc7, 0x51, 0x11, 0x1c, 0x33, 0x1, 0x83 );


extern RPC_IF_HANDLE __MIDL_itf_wmpservices_0000_0001_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_wmpservices_0000_0001_v0_0_s_ifspec;

#ifndef __IWMPMediaPluginRegistrar_INTERFACE_DEFINED__
#define __IWMPMediaPluginRegistrar_INTERFACE_DEFINED__

/* interface IWMPMediaPluginRegistrar */
/* [unique][helpstring][uuid][object][local] */ 


EXTERN_C const IID IID_IWMPMediaPluginRegistrar;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("68E27045-05BD-40b2-9720-23088C78E390")
    IWMPMediaPluginRegistrar : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE WMPRegisterPlayerPlugin( 
            /* [in] */ LPWSTR pwszFriendlyName,
            /* [in] */ LPWSTR pwszDescription,
            /* [in] */ LPWSTR pwszUninstallString,
            DWORD dwPriority,
            GUID guidPluginType,
            CLSID clsid,
            UINT cMediaTypes,
            LPVOID pMediaTypes) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE WMPUnRegisterPlayerPlugin( 
            GUID guidPluginType,
            CLSID clsid) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWMPMediaPluginRegistrarVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IWMPMediaPluginRegistrar * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IWMPMediaPluginRegistrar * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IWMPMediaPluginRegistrar * This);
        
        DECLSPEC_XFGVIRT(IWMPMediaPluginRegistrar, WMPRegisterPlayerPlugin)
        HRESULT ( STDMETHODCALLTYPE *WMPRegisterPlayerPlugin )( 
            IWMPMediaPluginRegistrar * This,
            /* [in] */ LPWSTR pwszFriendlyName,
            /* [in] */ LPWSTR pwszDescription,
            /* [in] */ LPWSTR pwszUninstallString,
            DWORD dwPriority,
            GUID guidPluginType,
            CLSID clsid,
            UINT cMediaTypes,
            LPVOID pMediaTypes);
        
        DECLSPEC_XFGVIRT(IWMPMediaPluginRegistrar, WMPUnRegisterPlayerPlugin)
        HRESULT ( STDMETHODCALLTYPE *WMPUnRegisterPlayerPlugin )( 
            IWMPMediaPluginRegistrar * This,
            GUID guidPluginType,
            CLSID clsid);
        
        END_INTERFACE
    } IWMPMediaPluginRegistrarVtbl;

    interface IWMPMediaPluginRegistrar
    {
        CONST_VTBL struct IWMPMediaPluginRegistrarVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWMPMediaPluginRegistrar_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWMPMediaPluginRegistrar_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWMPMediaPluginRegistrar_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWMPMediaPluginRegistrar_WMPRegisterPlayerPlugin(This,pwszFriendlyName,pwszDescription,pwszUninstallString,dwPriority,guidPluginType,clsid,cMediaTypes,pMediaTypes)	\
    ( (This)->lpVtbl -> WMPRegisterPlayerPlugin(This,pwszFriendlyName,pwszDescription,pwszUninstallString,dwPriority,guidPluginType,clsid,cMediaTypes,pMediaTypes) ) 

#define IWMPMediaPluginRegistrar_WMPUnRegisterPlayerPlugin(This,guidPluginType,clsid)	\
    ( (This)->lpVtbl -> WMPUnRegisterPlayerPlugin(This,guidPluginType,clsid) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWMPMediaPluginRegistrar_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_wmpservices_0000_0002 */
/* [local] */ 

typedef /* [public][helpstring] */ 
enum WMPPlugin_Caps
    {
        WMPPlugin_Caps_CannotConvertFormats	= 1
    } 	WMPPlugin_Caps;



extern RPC_IF_HANDLE __MIDL_itf_wmpservices_0000_0002_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_wmpservices_0000_0002_v0_0_s_ifspec;

#ifndef __IWMPPlugin_INTERFACE_DEFINED__
#define __IWMPPlugin_INTERFACE_DEFINED__

/* interface IWMPPlugin */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IWMPPlugin;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("f1392a70-024c-42bb-a998-73dfdfe7d5a7")
    IWMPPlugin : public IUnknown
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE Init( 
            DWORD_PTR dwPlaybackContext) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE Shutdown( void) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetID( 
            /* [retval][out] */ GUID *pGUID) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetCaps( 
            /* [retval][out] */ DWORD *pdwFlags) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE AdviseWMPServices( 
            /* [in] */ IWMPServices *pWMPServices) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE UnAdviseWMPServices( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWMPPluginVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IWMPPlugin * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IWMPPlugin * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IWMPPlugin * This);
        
        DECLSPEC_XFGVIRT(IWMPPlugin, Init)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *Init )( 
            IWMPPlugin * This,
            DWORD_PTR dwPlaybackContext);
        
        DECLSPEC_XFGVIRT(IWMPPlugin, Shutdown)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *Shutdown )( 
            IWMPPlugin * This);
        
        DECLSPEC_XFGVIRT(IWMPPlugin, GetID)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetID )( 
            IWMPPlugin * This,
            /* [retval][out] */ GUID *pGUID);
        
        DECLSPEC_XFGVIRT(IWMPPlugin, GetCaps)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetCaps )( 
            IWMPPlugin * This,
            /* [retval][out] */ DWORD *pdwFlags);
        
        DECLSPEC_XFGVIRT(IWMPPlugin, AdviseWMPServices)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *AdviseWMPServices )( 
            IWMPPlugin * This,
            /* [in] */ IWMPServices *pWMPServices);
        
        DECLSPEC_XFGVIRT(IWMPPlugin, UnAdviseWMPServices)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *UnAdviseWMPServices )( 
            IWMPPlugin * This);
        
        END_INTERFACE
    } IWMPPluginVtbl;

    interface IWMPPlugin
    {
        CONST_VTBL struct IWMPPluginVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWMPPlugin_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWMPPlugin_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWMPPlugin_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWMPPlugin_Init(This,dwPlaybackContext)	\
    ( (This)->lpVtbl -> Init(This,dwPlaybackContext) ) 

#define IWMPPlugin_Shutdown(This)	\
    ( (This)->lpVtbl -> Shutdown(This) ) 

#define IWMPPlugin_GetID(This,pGUID)	\
    ( (This)->lpVtbl -> GetID(This,pGUID) ) 

#define IWMPPlugin_GetCaps(This,pdwFlags)	\
    ( (This)->lpVtbl -> GetCaps(This,pdwFlags) ) 

#define IWMPPlugin_AdviseWMPServices(This,pWMPServices)	\
    ( (This)->lpVtbl -> AdviseWMPServices(This,pWMPServices) ) 

#define IWMPPlugin_UnAdviseWMPServices(This)	\
    ( (This)->lpVtbl -> UnAdviseWMPServices(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWMPPlugin_INTERFACE_DEFINED__ */


#ifndef __IWMPPluginEnable_INTERFACE_DEFINED__
#define __IWMPPluginEnable_INTERFACE_DEFINED__

/* interface IWMPPluginEnable */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IWMPPluginEnable;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("5FCA444C-7AD1-479d-A4EF-40566A5309D6")
    IWMPPluginEnable : public IUnknown
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE SetEnable( 
            /* [in] */ BOOL fEnable) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetEnable( 
            /* [out] */ BOOL *pfEnable) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWMPPluginEnableVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IWMPPluginEnable * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IWMPPluginEnable * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IWMPPluginEnable * This);
        
        DECLSPEC_XFGVIRT(IWMPPluginEnable, SetEnable)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *SetEnable )( 
            IWMPPluginEnable * This,
            /* [in] */ BOOL fEnable);
        
        DECLSPEC_XFGVIRT(IWMPPluginEnable, GetEnable)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetEnable )( 
            IWMPPluginEnable * This,
            /* [out] */ BOOL *pfEnable);
        
        END_INTERFACE
    } IWMPPluginEnableVtbl;

    interface IWMPPluginEnable
    {
        CONST_VTBL struct IWMPPluginEnableVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWMPPluginEnable_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWMPPluginEnable_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWMPPluginEnable_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWMPPluginEnable_SetEnable(This,fEnable)	\
    ( (This)->lpVtbl -> SetEnable(This,fEnable) ) 

#define IWMPPluginEnable_GetEnable(This,pfEnable)	\
    ( (This)->lpVtbl -> GetEnable(This,pfEnable) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWMPPluginEnable_INTERFACE_DEFINED__ */


#ifndef __IWMPGraphCreation_INTERFACE_DEFINED__
#define __IWMPGraphCreation_INTERFACE_DEFINED__

/* interface IWMPGraphCreation */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IWMPGraphCreation;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("bfb377e5-c594-4369-a970-de896d5ece74")
    IWMPGraphCreation : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GraphCreationPreRender( 
            /* [in] */ IUnknown *pFilterGraph,
            /* [in] */ IUnknown *pReserved) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GraphCreationPostRender( 
            /* [in] */ IUnknown *pFilterGraph) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetGraphCreationFlags( 
            /* [out] */ DWORD *pdwFlags) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWMPGraphCreationVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IWMPGraphCreation * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IWMPGraphCreation * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IWMPGraphCreation * This);
        
        DECLSPEC_XFGVIRT(IWMPGraphCreation, GraphCreationPreRender)
        HRESULT ( STDMETHODCALLTYPE *GraphCreationPreRender )( 
            IWMPGraphCreation * This,
            /* [in] */ IUnknown *pFilterGraph,
            /* [in] */ IUnknown *pReserved);
        
        DECLSPEC_XFGVIRT(IWMPGraphCreation, GraphCreationPostRender)
        HRESULT ( STDMETHODCALLTYPE *GraphCreationPostRender )( 
            IWMPGraphCreation * This,
            /* [in] */ IUnknown *pFilterGraph);
        
        DECLSPEC_XFGVIRT(IWMPGraphCreation, GetGraphCreationFlags)
        HRESULT ( STDMETHODCALLTYPE *GetGraphCreationFlags )( 
            IWMPGraphCreation * This,
            /* [out] */ DWORD *pdwFlags);
        
        END_INTERFACE
    } IWMPGraphCreationVtbl;

    interface IWMPGraphCreation
    {
        CONST_VTBL struct IWMPGraphCreationVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWMPGraphCreation_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWMPGraphCreation_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWMPGraphCreation_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWMPGraphCreation_GraphCreationPreRender(This,pFilterGraph,pReserved)	\
    ( (This)->lpVtbl -> GraphCreationPreRender(This,pFilterGraph,pReserved) ) 

#define IWMPGraphCreation_GraphCreationPostRender(This,pFilterGraph)	\
    ( (This)->lpVtbl -> GraphCreationPostRender(This,pFilterGraph) ) 

#define IWMPGraphCreation_GetGraphCreationFlags(This,pdwFlags)	\
    ( (This)->lpVtbl -> GetGraphCreationFlags(This,pdwFlags) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWMPGraphCreation_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_wmpservices_0000_0005 */
/* [local] */ 

#define WMPGC_FLAGS_ALLOW_PREROLL    0x00000001
#define WMPGC_FLAGS_SUPPRESS_DIALOGS 0x00000002
#define WMPGC_FLAGS_IGNORE_AV_SYNC   0x00000004
#define WMPGC_FLAGS_DISABLE_PLUGINS  0x00000008
#define WMPGC_FLAGS_USE_CUSTOM_GRAPH 0x00000010


extern RPC_IF_HANDLE __MIDL_itf_wmpservices_0000_0005_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_wmpservices_0000_0005_v0_0_s_ifspec;

#ifndef __IWMPConvert_INTERFACE_DEFINED__
#define __IWMPConvert_INTERFACE_DEFINED__

/* interface IWMPConvert */
/* [oleautomation][unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IWMPConvert;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("D683162F-57D4-4108-8373-4A9676D1C2E9")
    IWMPConvert : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE ConvertFile( 
            /* [in] */ BSTR bstrInputFile,
            /* [in] */ BSTR bstrDestinationFolder,
            /* [out] */ BSTR *pbstrOutputFile) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetErrorURL( 
            /* [out] */ BSTR *pbstrURL) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWMPConvertVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IWMPConvert * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IWMPConvert * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IWMPConvert * This);
        
        DECLSPEC_XFGVIRT(IWMPConvert, ConvertFile)
        HRESULT ( STDMETHODCALLTYPE *ConvertFile )( 
            IWMPConvert * This,
            /* [in] */ BSTR bstrInputFile,
            /* [in] */ BSTR bstrDestinationFolder,
            /* [out] */ BSTR *pbstrOutputFile);
        
        DECLSPEC_XFGVIRT(IWMPConvert, GetErrorURL)
        HRESULT ( STDMETHODCALLTYPE *GetErrorURL )( 
            IWMPConvert * This,
            /* [out] */ BSTR *pbstrURL);
        
        END_INTERFACE
    } IWMPConvertVtbl;

    interface IWMPConvert
    {
        CONST_VTBL struct IWMPConvertVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWMPConvert_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWMPConvert_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWMPConvert_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWMPConvert_ConvertFile(This,bstrInputFile,bstrDestinationFolder,pbstrOutputFile)	\
    ( (This)->lpVtbl -> ConvertFile(This,bstrInputFile,bstrDestinationFolder,pbstrOutputFile) ) 

#define IWMPConvert_GetErrorURL(This,pbstrURL)	\
    ( (This)->lpVtbl -> GetErrorURL(This,pbstrURL) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWMPConvert_INTERFACE_DEFINED__ */


#ifndef __IWMPTranscodePolicy_INTERFACE_DEFINED__
#define __IWMPTranscodePolicy_INTERFACE_DEFINED__

/* interface IWMPTranscodePolicy */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IWMPTranscodePolicy;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("B64CBAC3-401C-4327-A3E8-B9FEB3A8C25C")
    IWMPTranscodePolicy : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE allowTranscode( 
            /* [out] */ VARIANT_BOOL *pvbAllow) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWMPTranscodePolicyVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IWMPTranscodePolicy * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IWMPTranscodePolicy * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IWMPTranscodePolicy * This);
        
        DECLSPEC_XFGVIRT(IWMPTranscodePolicy, allowTranscode)
        HRESULT ( STDMETHODCALLTYPE *allowTranscode )( 
            IWMPTranscodePolicy * This,
            /* [out] */ VARIANT_BOOL *pvbAllow);
        
        END_INTERFACE
    } IWMPTranscodePolicyVtbl;

    interface IWMPTranscodePolicy
    {
        CONST_VTBL struct IWMPTranscodePolicyVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWMPTranscodePolicy_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWMPTranscodePolicy_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWMPTranscodePolicy_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWMPTranscodePolicy_allowTranscode(This,pvbAllow)	\
    ( (This)->lpVtbl -> allowTranscode(This,pvbAllow) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWMPTranscodePolicy_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_wmpservices_0000_0007 */
/* [local] */ 

#define WMPUE_EC_USER   EC_USER + 0x100


extern RPC_IF_HANDLE __MIDL_itf_wmpservices_0000_0007_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_wmpservices_0000_0007_v0_0_s_ifspec;

#ifndef __IWMPUserEventSink_INTERFACE_DEFINED__
#define __IWMPUserEventSink_INTERFACE_DEFINED__

/* interface IWMPUserEventSink */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IWMPUserEventSink;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("CFCCFA72-C343-48C3-A2DE-B7A4402E39F2")
    IWMPUserEventSink : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE NotifyUserEvent( 
            /* [in] */ long EventCode) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWMPUserEventSinkVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IWMPUserEventSink * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IWMPUserEventSink * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IWMPUserEventSink * This);
        
        DECLSPEC_XFGVIRT(IWMPUserEventSink, NotifyUserEvent)
        HRESULT ( STDMETHODCALLTYPE *NotifyUserEvent )( 
            IWMPUserEventSink * This,
            /* [in] */ long EventCode);
        
        END_INTERFACE
    } IWMPUserEventSinkVtbl;

    interface IWMPUserEventSink
    {
        CONST_VTBL struct IWMPUserEventSinkVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWMPUserEventSink_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWMPUserEventSink_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWMPUserEventSink_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWMPUserEventSink_NotifyUserEvent(This,EventCode)	\
    ( (This)->lpVtbl -> NotifyUserEvent(This,EventCode) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWMPUserEventSink_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_wmpservices_0000_0008 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_wmpservices_0000_0008_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_wmpservices_0000_0008_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

unsigned long             __RPC_USER  BSTR_UserSize(     unsigned long *, unsigned long            , BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserMarshal(  unsigned long *, unsigned char *, BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserUnmarshal(unsigned long *, unsigned char *, BSTR * ); 
void                      __RPC_USER  BSTR_UserFree(     unsigned long *, BSTR * ); 

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


