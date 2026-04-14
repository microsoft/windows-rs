

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

#ifndef __subscriptionservicespri_h__
#define __subscriptionservicespri_h__

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

#ifndef __IWMPSubscriptionService_FWD_DEFINED__
#define __IWMPSubscriptionService_FWD_DEFINED__
typedef interface IWMPSubscriptionService IWMPSubscriptionService;

#endif 	/* __IWMPSubscriptionService_FWD_DEFINED__ */


#ifndef __IWMPSubscriptionServiceCallback_FWD_DEFINED__
#define __IWMPSubscriptionServiceCallback_FWD_DEFINED__
typedef interface IWMPSubscriptionServiceCallback IWMPSubscriptionServiceCallback;

#endif 	/* __IWMPSubscriptionServiceCallback_FWD_DEFINED__ */


#ifndef __IWMPSubscriptionService2_FWD_DEFINED__
#define __IWMPSubscriptionService2_FWD_DEFINED__
typedef interface IWMPSubscriptionService2 IWMPSubscriptionService2;

#endif 	/* __IWMPSubscriptionService2_FWD_DEFINED__ */


#ifndef __IWMPDownloadItem_FWD_DEFINED__
#define __IWMPDownloadItem_FWD_DEFINED__
typedef interface IWMPDownloadItem IWMPDownloadItem;

#endif 	/* __IWMPDownloadItem_FWD_DEFINED__ */


#ifndef __IWMPDownloadItem2_FWD_DEFINED__
#define __IWMPDownloadItem2_FWD_DEFINED__
typedef interface IWMPDownloadItem2 IWMPDownloadItem2;

#endif 	/* __IWMPDownloadItem2_FWD_DEFINED__ */


#ifndef __IWMPDownloadCollection_FWD_DEFINED__
#define __IWMPDownloadCollection_FWD_DEFINED__
typedef interface IWMPDownloadCollection IWMPDownloadCollection;

#endif 	/* __IWMPDownloadCollection_FWD_DEFINED__ */


#ifndef __IWMPDownloadManager_FWD_DEFINED__
#define __IWMPDownloadManager_FWD_DEFINED__
typedef interface IWMPDownloadManager IWMPDownloadManager;

#endif 	/* __IWMPDownloadManager_FWD_DEFINED__ */


/* header files for imported files */
#include "oaidl.h"
#include "ocidl.h"
#include "wmp.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_subscriptionservicespri_0000_0000 */
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
#pragma warning(push)
#pragma warning(disable:4001) 
#pragma once
#pragma warning(push)
#pragma warning(disable:4001) 
#pragma once
#pragma warning(pop)
#pragma warning(pop)
#pragma region Desktop Family
#pragma endregion
#ifndef SUBSCRIPTION_CAP_ALLOWPLAY
#define SUBSCRIPTION_CAP_ALLOWPLAY              0x00000001
#endif
#ifndef SUBSCRIPTION_CAP_ALLOWCDBURN
#define SUBSCRIPTION_CAP_ALLOWCDBURN            0x00000002
#endif
#ifndef SUBSCRIPTION_CAP_ALLOWPDATRANSFER
#define SUBSCRIPTION_CAP_ALLOWPDATRANSFER       0x00000004
#endif
#ifndef SUBSCRIPTION_CAP_BACKGROUNDPROCESSING
#define SUBSCRIPTION_CAP_BACKGROUNDPROCESSING   0x00000008
#endif
#ifndef SUBSCRIPTION_CAP_DEVICEAVAILABLE
#define SUBSCRIPTION_CAP_DEVICEAVAILABLE        0x00000010
#endif
#ifndef SUBSCRIPTION_CAP_PREPAREFORSYNC
#define SUBSCRIPTION_CAP_PREPAREFORSYNC         0x00000020
#endif
#ifndef SUBSCRIPTION_CAP_IS_CONTENTPARTNER
#define SUBSCRIPTION_CAP_IS_CONTENTPARTNER      0x00000040
#endif
#ifndef SUBSCRIPTION_CAP_ALTLOGIN
#define SUBSCRIPTION_CAP_ALTLOGIN      0x00000080
#endif
#ifndef SUBSCRIPTION_V1_CAPS
#define SUBSCRIPTION_V1_CAPS                    0x0000000F
#endif
#ifndef SUBSCRIPTION_CAP_UILESSMODE_ALLOWPLAY
#define SUBSCRIPTION_CAP_UILESSMODE_ALLOWPLAY     0x00000100
#endif
typedef /* [public][helpstring] */ 
enum WMPSubscriptionServiceEvent
    {
        wmpsseCurrentBegin	= 1,
        wmpsseCurrentEnd	= 2,
        wmpsseFullBegin	= 3,
        wmpsseFullEnd	= 4
    } 	WMPSubscriptionServiceEvent;



extern RPC_IF_HANDLE __MIDL_itf_subscriptionservicespri_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_subscriptionservicespri_0000_0000_v0_0_s_ifspec;

#ifndef __IWMPSubscriptionService_INTERFACE_DEFINED__
#define __IWMPSubscriptionService_INTERFACE_DEFINED__

/* interface IWMPSubscriptionService */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IWMPSubscriptionService;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("376055F8-2A59-4a73-9501-DCA5273A7A10")
    IWMPSubscriptionService : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE allowPlay( 
            HWND hwnd,
            IWMPMedia *pMedia,
            BOOL *pfAllowPlay) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE allowCDBurn( 
            HWND hwnd,
            IWMPPlaylist *pPlaylist,
            BOOL *pfAllowBurn) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE allowPDATransfer( 
            HWND hwnd,
            IWMPPlaylist *pPlaylist,
            BOOL *pfAllowTransfer) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE startBackgroundProcessing( 
            HWND hwnd) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWMPSubscriptionServiceVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IWMPSubscriptionService * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IWMPSubscriptionService * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IWMPSubscriptionService * This);
        
        DECLSPEC_XFGVIRT(IWMPSubscriptionService, allowPlay)
        HRESULT ( STDMETHODCALLTYPE *allowPlay )( 
            IWMPSubscriptionService * This,
            HWND hwnd,
            IWMPMedia *pMedia,
            BOOL *pfAllowPlay);
        
        DECLSPEC_XFGVIRT(IWMPSubscriptionService, allowCDBurn)
        HRESULT ( STDMETHODCALLTYPE *allowCDBurn )( 
            IWMPSubscriptionService * This,
            HWND hwnd,
            IWMPPlaylist *pPlaylist,
            BOOL *pfAllowBurn);
        
        DECLSPEC_XFGVIRT(IWMPSubscriptionService, allowPDATransfer)
        HRESULT ( STDMETHODCALLTYPE *allowPDATransfer )( 
            IWMPSubscriptionService * This,
            HWND hwnd,
            IWMPPlaylist *pPlaylist,
            BOOL *pfAllowTransfer);
        
        DECLSPEC_XFGVIRT(IWMPSubscriptionService, startBackgroundProcessing)
        HRESULT ( STDMETHODCALLTYPE *startBackgroundProcessing )( 
            IWMPSubscriptionService * This,
            HWND hwnd);
        
        END_INTERFACE
    } IWMPSubscriptionServiceVtbl;

    interface IWMPSubscriptionService
    {
        CONST_VTBL struct IWMPSubscriptionServiceVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWMPSubscriptionService_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWMPSubscriptionService_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWMPSubscriptionService_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWMPSubscriptionService_allowPlay(This,hwnd,pMedia,pfAllowPlay)	\
    ( (This)->lpVtbl -> allowPlay(This,hwnd,pMedia,pfAllowPlay) ) 

#define IWMPSubscriptionService_allowCDBurn(This,hwnd,pPlaylist,pfAllowBurn)	\
    ( (This)->lpVtbl -> allowCDBurn(This,hwnd,pPlaylist,pfAllowBurn) ) 

#define IWMPSubscriptionService_allowPDATransfer(This,hwnd,pPlaylist,pfAllowTransfer)	\
    ( (This)->lpVtbl -> allowPDATransfer(This,hwnd,pPlaylist,pfAllowTransfer) ) 

#define IWMPSubscriptionService_startBackgroundProcessing(This,hwnd)	\
    ( (This)->lpVtbl -> startBackgroundProcessing(This,hwnd) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWMPSubscriptionService_INTERFACE_DEFINED__ */


#ifndef __IWMPSubscriptionServiceCallback_INTERFACE_DEFINED__
#define __IWMPSubscriptionServiceCallback_INTERFACE_DEFINED__

/* interface IWMPSubscriptionServiceCallback */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IWMPSubscriptionServiceCallback;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("DD01D127-2DC2-4c3a-876E-63312079F9B0")
    IWMPSubscriptionServiceCallback : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE onComplete( 
            HRESULT hrResult) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWMPSubscriptionServiceCallbackVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IWMPSubscriptionServiceCallback * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IWMPSubscriptionServiceCallback * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IWMPSubscriptionServiceCallback * This);
        
        DECLSPEC_XFGVIRT(IWMPSubscriptionServiceCallback, onComplete)
        HRESULT ( STDMETHODCALLTYPE *onComplete )( 
            IWMPSubscriptionServiceCallback * This,
            HRESULT hrResult);
        
        END_INTERFACE
    } IWMPSubscriptionServiceCallbackVtbl;

    interface IWMPSubscriptionServiceCallback
    {
        CONST_VTBL struct IWMPSubscriptionServiceCallbackVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWMPSubscriptionServiceCallback_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWMPSubscriptionServiceCallback_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWMPSubscriptionServiceCallback_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWMPSubscriptionServiceCallback_onComplete(This,hrResult)	\
    ( (This)->lpVtbl -> onComplete(This,hrResult) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWMPSubscriptionServiceCallback_INTERFACE_DEFINED__ */


#ifndef __IWMPSubscriptionService2_INTERFACE_DEFINED__
#define __IWMPSubscriptionService2_INTERFACE_DEFINED__

/* interface IWMPSubscriptionService2 */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IWMPSubscriptionService2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("A94C120E-D600-4ec6-B05E-EC9D56D84DE0")
    IWMPSubscriptionService2 : public IWMPSubscriptionService
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE stopBackgroundProcessing( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE serviceEvent( 
            WMPSubscriptionServiceEvent event) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE deviceAvailable( 
            BSTR bstrDeviceName,
            IWMPSubscriptionServiceCallback *pCB) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE prepareForSync( 
            BSTR bstrFilename,
            BSTR bstrDeviceName,
            IWMPSubscriptionServiceCallback *pCB) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWMPSubscriptionService2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IWMPSubscriptionService2 * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IWMPSubscriptionService2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IWMPSubscriptionService2 * This);
        
        DECLSPEC_XFGVIRT(IWMPSubscriptionService, allowPlay)
        HRESULT ( STDMETHODCALLTYPE *allowPlay )( 
            IWMPSubscriptionService2 * This,
            HWND hwnd,
            IWMPMedia *pMedia,
            BOOL *pfAllowPlay);
        
        DECLSPEC_XFGVIRT(IWMPSubscriptionService, allowCDBurn)
        HRESULT ( STDMETHODCALLTYPE *allowCDBurn )( 
            IWMPSubscriptionService2 * This,
            HWND hwnd,
            IWMPPlaylist *pPlaylist,
            BOOL *pfAllowBurn);
        
        DECLSPEC_XFGVIRT(IWMPSubscriptionService, allowPDATransfer)
        HRESULT ( STDMETHODCALLTYPE *allowPDATransfer )( 
            IWMPSubscriptionService2 * This,
            HWND hwnd,
            IWMPPlaylist *pPlaylist,
            BOOL *pfAllowTransfer);
        
        DECLSPEC_XFGVIRT(IWMPSubscriptionService, startBackgroundProcessing)
        HRESULT ( STDMETHODCALLTYPE *startBackgroundProcessing )( 
            IWMPSubscriptionService2 * This,
            HWND hwnd);
        
        DECLSPEC_XFGVIRT(IWMPSubscriptionService2, stopBackgroundProcessing)
        HRESULT ( STDMETHODCALLTYPE *stopBackgroundProcessing )( 
            IWMPSubscriptionService2 * This);
        
        DECLSPEC_XFGVIRT(IWMPSubscriptionService2, serviceEvent)
        HRESULT ( STDMETHODCALLTYPE *serviceEvent )( 
            IWMPSubscriptionService2 * This,
            WMPSubscriptionServiceEvent event);
        
        DECLSPEC_XFGVIRT(IWMPSubscriptionService2, deviceAvailable)
        HRESULT ( STDMETHODCALLTYPE *deviceAvailable )( 
            IWMPSubscriptionService2 * This,
            BSTR bstrDeviceName,
            IWMPSubscriptionServiceCallback *pCB);
        
        DECLSPEC_XFGVIRT(IWMPSubscriptionService2, prepareForSync)
        HRESULT ( STDMETHODCALLTYPE *prepareForSync )( 
            IWMPSubscriptionService2 * This,
            BSTR bstrFilename,
            BSTR bstrDeviceName,
            IWMPSubscriptionServiceCallback *pCB);
        
        END_INTERFACE
    } IWMPSubscriptionService2Vtbl;

    interface IWMPSubscriptionService2
    {
        CONST_VTBL struct IWMPSubscriptionService2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWMPSubscriptionService2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWMPSubscriptionService2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWMPSubscriptionService2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWMPSubscriptionService2_allowPlay(This,hwnd,pMedia,pfAllowPlay)	\
    ( (This)->lpVtbl -> allowPlay(This,hwnd,pMedia,pfAllowPlay) ) 

#define IWMPSubscriptionService2_allowCDBurn(This,hwnd,pPlaylist,pfAllowBurn)	\
    ( (This)->lpVtbl -> allowCDBurn(This,hwnd,pPlaylist,pfAllowBurn) ) 

#define IWMPSubscriptionService2_allowPDATransfer(This,hwnd,pPlaylist,pfAllowTransfer)	\
    ( (This)->lpVtbl -> allowPDATransfer(This,hwnd,pPlaylist,pfAllowTransfer) ) 

#define IWMPSubscriptionService2_startBackgroundProcessing(This,hwnd)	\
    ( (This)->lpVtbl -> startBackgroundProcessing(This,hwnd) ) 


#define IWMPSubscriptionService2_stopBackgroundProcessing(This)	\
    ( (This)->lpVtbl -> stopBackgroundProcessing(This) ) 

#define IWMPSubscriptionService2_serviceEvent(This,event)	\
    ( (This)->lpVtbl -> serviceEvent(This,event) ) 

#define IWMPSubscriptionService2_deviceAvailable(This,bstrDeviceName,pCB)	\
    ( (This)->lpVtbl -> deviceAvailable(This,bstrDeviceName,pCB) ) 

#define IWMPSubscriptionService2_prepareForSync(This,bstrFilename,bstrDeviceName,pCB)	\
    ( (This)->lpVtbl -> prepareForSync(This,bstrFilename,bstrDeviceName,pCB) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWMPSubscriptionService2_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_subscriptionservicespri_0000_0003 */
/* [local] */ 

#ifndef __WMPNotifySubscriptionPluginAddRemove
#define __WMPNotifySubscriptionPluginAddRemove
__inline BOOL WMPNotifySubscriptionPluginAddRemove()
{
    BOOL fRet = FALSE;
    UINT  msg = RegisterWindowMessageA( "WMPlayer_PluginAddRemove" );
    if( 0 != msg ) 
     {
        fRet = PostMessage( HWND_BROADCAST, msg, 1, 0 );
     }
     return fRet;
}
#endif
#define WMP_SUBSCR_DL_TYPE_BACKGROUND       L"background"
#define WMP_SUBSCR_DL_TYPE_REALTIME         L"real time"
typedef /* [public][helpstring] */ 
enum WMPSubscriptionDownloadState
    {
        wmpsdlsDownloading	= 0,
        wmpsdlsPaused	= ( wmpsdlsDownloading + 1 ) ,
        wmpsdlsProcessing	= ( wmpsdlsPaused + 1 ) ,
        wmpsdlsCompleted	= ( wmpsdlsProcessing + 1 ) ,
        wmpsdlsCancelled	= ( wmpsdlsCompleted + 1 ) 
    } 	WMPSubscriptionDownloadState;



extern RPC_IF_HANDLE __MIDL_itf_subscriptionservicespri_0000_0003_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_subscriptionservicespri_0000_0003_v0_0_s_ifspec;

#ifndef __IWMPDownloadItem_INTERFACE_DEFINED__
#define __IWMPDownloadItem_INTERFACE_DEFINED__

/* interface IWMPDownloadItem */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IWMPDownloadItem;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("C9470E8E-3F6B-46a9-A0A9-452815C34297")
    IWMPDownloadItem : public IDispatch
    {
    public:
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_sourceURL( 
            /* [retval][out] */ BSTR *pbstrURL) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_size( 
            /* [retval][out] */ long *plSize) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_type( 
            /* [retval][out] */ BSTR *pbstrType) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_progress( 
            /* [retval][out] */ long *plProgress) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_downloadState( 
            /* [retval][out] */ WMPSubscriptionDownloadState *pwmpsdls) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE pause( void) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE resume( void) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE cancel( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWMPDownloadItemVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IWMPDownloadItem * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IWMPDownloadItem * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IWMPDownloadItem * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            IWMPDownloadItem * This,
            /* [out] */ UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            IWMPDownloadItem * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            IWMPDownloadItem * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR *rgszNames,
            /* [range][in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IWMPDownloadItem * This,
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
        
        DECLSPEC_XFGVIRT(IWMPDownloadItem, get_sourceURL)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_sourceURL )( 
            IWMPDownloadItem * This,
            /* [retval][out] */ BSTR *pbstrURL);
        
        DECLSPEC_XFGVIRT(IWMPDownloadItem, get_size)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_size )( 
            IWMPDownloadItem * This,
            /* [retval][out] */ long *plSize);
        
        DECLSPEC_XFGVIRT(IWMPDownloadItem, get_type)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_type )( 
            IWMPDownloadItem * This,
            /* [retval][out] */ BSTR *pbstrType);
        
        DECLSPEC_XFGVIRT(IWMPDownloadItem, get_progress)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_progress )( 
            IWMPDownloadItem * This,
            /* [retval][out] */ long *plProgress);
        
        DECLSPEC_XFGVIRT(IWMPDownloadItem, get_downloadState)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_downloadState )( 
            IWMPDownloadItem * This,
            /* [retval][out] */ WMPSubscriptionDownloadState *pwmpsdls);
        
        DECLSPEC_XFGVIRT(IWMPDownloadItem, pause)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *pause )( 
            IWMPDownloadItem * This);
        
        DECLSPEC_XFGVIRT(IWMPDownloadItem, resume)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *resume )( 
            IWMPDownloadItem * This);
        
        DECLSPEC_XFGVIRT(IWMPDownloadItem, cancel)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *cancel )( 
            IWMPDownloadItem * This);
        
        END_INTERFACE
    } IWMPDownloadItemVtbl;

    interface IWMPDownloadItem
    {
        CONST_VTBL struct IWMPDownloadItemVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWMPDownloadItem_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWMPDownloadItem_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWMPDownloadItem_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWMPDownloadItem_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IWMPDownloadItem_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IWMPDownloadItem_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IWMPDownloadItem_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IWMPDownloadItem_get_sourceURL(This,pbstrURL)	\
    ( (This)->lpVtbl -> get_sourceURL(This,pbstrURL) ) 

#define IWMPDownloadItem_get_size(This,plSize)	\
    ( (This)->lpVtbl -> get_size(This,plSize) ) 

#define IWMPDownloadItem_get_type(This,pbstrType)	\
    ( (This)->lpVtbl -> get_type(This,pbstrType) ) 

#define IWMPDownloadItem_get_progress(This,plProgress)	\
    ( (This)->lpVtbl -> get_progress(This,plProgress) ) 

#define IWMPDownloadItem_get_downloadState(This,pwmpsdls)	\
    ( (This)->lpVtbl -> get_downloadState(This,pwmpsdls) ) 

#define IWMPDownloadItem_pause(This)	\
    ( (This)->lpVtbl -> pause(This) ) 

#define IWMPDownloadItem_resume(This)	\
    ( (This)->lpVtbl -> resume(This) ) 

#define IWMPDownloadItem_cancel(This)	\
    ( (This)->lpVtbl -> cancel(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWMPDownloadItem_INTERFACE_DEFINED__ */


#ifndef __IWMPDownloadItem2_INTERFACE_DEFINED__
#define __IWMPDownloadItem2_INTERFACE_DEFINED__

/* interface IWMPDownloadItem2 */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IWMPDownloadItem2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("9FBB3336-6DA3-479d-B8FF-67D46E20A987")
    IWMPDownloadItem2 : public IWMPDownloadItem
    {
    public:
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE getItemInfo( 
            /* [in] */ BSTR bstrItemName,
            /* [retval][out] */ BSTR *pbstrVal) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWMPDownloadItem2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IWMPDownloadItem2 * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IWMPDownloadItem2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IWMPDownloadItem2 * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            IWMPDownloadItem2 * This,
            /* [out] */ UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            IWMPDownloadItem2 * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            IWMPDownloadItem2 * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR *rgszNames,
            /* [range][in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IWMPDownloadItem2 * This,
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
        
        DECLSPEC_XFGVIRT(IWMPDownloadItem, get_sourceURL)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_sourceURL )( 
            IWMPDownloadItem2 * This,
            /* [retval][out] */ BSTR *pbstrURL);
        
        DECLSPEC_XFGVIRT(IWMPDownloadItem, get_size)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_size )( 
            IWMPDownloadItem2 * This,
            /* [retval][out] */ long *plSize);
        
        DECLSPEC_XFGVIRT(IWMPDownloadItem, get_type)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_type )( 
            IWMPDownloadItem2 * This,
            /* [retval][out] */ BSTR *pbstrType);
        
        DECLSPEC_XFGVIRT(IWMPDownloadItem, get_progress)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_progress )( 
            IWMPDownloadItem2 * This,
            /* [retval][out] */ long *plProgress);
        
        DECLSPEC_XFGVIRT(IWMPDownloadItem, get_downloadState)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_downloadState )( 
            IWMPDownloadItem2 * This,
            /* [retval][out] */ WMPSubscriptionDownloadState *pwmpsdls);
        
        DECLSPEC_XFGVIRT(IWMPDownloadItem, pause)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *pause )( 
            IWMPDownloadItem2 * This);
        
        DECLSPEC_XFGVIRT(IWMPDownloadItem, resume)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *resume )( 
            IWMPDownloadItem2 * This);
        
        DECLSPEC_XFGVIRT(IWMPDownloadItem, cancel)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *cancel )( 
            IWMPDownloadItem2 * This);
        
        DECLSPEC_XFGVIRT(IWMPDownloadItem2, getItemInfo)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *getItemInfo )( 
            IWMPDownloadItem2 * This,
            /* [in] */ BSTR bstrItemName,
            /* [retval][out] */ BSTR *pbstrVal);
        
        END_INTERFACE
    } IWMPDownloadItem2Vtbl;

    interface IWMPDownloadItem2
    {
        CONST_VTBL struct IWMPDownloadItem2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWMPDownloadItem2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWMPDownloadItem2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWMPDownloadItem2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWMPDownloadItem2_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IWMPDownloadItem2_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IWMPDownloadItem2_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IWMPDownloadItem2_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IWMPDownloadItem2_get_sourceURL(This,pbstrURL)	\
    ( (This)->lpVtbl -> get_sourceURL(This,pbstrURL) ) 

#define IWMPDownloadItem2_get_size(This,plSize)	\
    ( (This)->lpVtbl -> get_size(This,plSize) ) 

#define IWMPDownloadItem2_get_type(This,pbstrType)	\
    ( (This)->lpVtbl -> get_type(This,pbstrType) ) 

#define IWMPDownloadItem2_get_progress(This,plProgress)	\
    ( (This)->lpVtbl -> get_progress(This,plProgress) ) 

#define IWMPDownloadItem2_get_downloadState(This,pwmpsdls)	\
    ( (This)->lpVtbl -> get_downloadState(This,pwmpsdls) ) 

#define IWMPDownloadItem2_pause(This)	\
    ( (This)->lpVtbl -> pause(This) ) 

#define IWMPDownloadItem2_resume(This)	\
    ( (This)->lpVtbl -> resume(This) ) 

#define IWMPDownloadItem2_cancel(This)	\
    ( (This)->lpVtbl -> cancel(This) ) 


#define IWMPDownloadItem2_getItemInfo(This,bstrItemName,pbstrVal)	\
    ( (This)->lpVtbl -> getItemInfo(This,bstrItemName,pbstrVal) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWMPDownloadItem2_INTERFACE_DEFINED__ */


#ifndef __IWMPDownloadCollection_INTERFACE_DEFINED__
#define __IWMPDownloadCollection_INTERFACE_DEFINED__

/* interface IWMPDownloadCollection */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IWMPDownloadCollection;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("0A319C7F-85F9-436c-B88E-82FD88000E1C")
    IWMPDownloadCollection : public IDispatch
    {
    public:
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_id( 
            /* [retval][out] */ long *plId) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_count( 
            /* [retval][out] */ long *plCount) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE item( 
            /* [in] */ long lItem,
            /* [retval][out] */ IWMPDownloadItem2 **ppDownload) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE startDownload( 
            /* [in] */ BSTR bstrSourceURL,
            /* [in] */ BSTR bstrType,
            /* [retval][out] */ IWMPDownloadItem2 **ppDownload) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE removeItem( 
            /* [in] */ long lItem) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Clear( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWMPDownloadCollectionVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IWMPDownloadCollection * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IWMPDownloadCollection * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IWMPDownloadCollection * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            IWMPDownloadCollection * This,
            /* [out] */ UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            IWMPDownloadCollection * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            IWMPDownloadCollection * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR *rgszNames,
            /* [range][in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IWMPDownloadCollection * This,
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
        
        DECLSPEC_XFGVIRT(IWMPDownloadCollection, get_id)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_id )( 
            IWMPDownloadCollection * This,
            /* [retval][out] */ long *plId);
        
        DECLSPEC_XFGVIRT(IWMPDownloadCollection, get_count)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_count )( 
            IWMPDownloadCollection * This,
            /* [retval][out] */ long *plCount);
        
        DECLSPEC_XFGVIRT(IWMPDownloadCollection, item)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *item )( 
            IWMPDownloadCollection * This,
            /* [in] */ long lItem,
            /* [retval][out] */ IWMPDownloadItem2 **ppDownload);
        
        DECLSPEC_XFGVIRT(IWMPDownloadCollection, startDownload)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *startDownload )( 
            IWMPDownloadCollection * This,
            /* [in] */ BSTR bstrSourceURL,
            /* [in] */ BSTR bstrType,
            /* [retval][out] */ IWMPDownloadItem2 **ppDownload);
        
        DECLSPEC_XFGVIRT(IWMPDownloadCollection, removeItem)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *removeItem )( 
            IWMPDownloadCollection * This,
            /* [in] */ long lItem);
        
        DECLSPEC_XFGVIRT(IWMPDownloadCollection, Clear)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Clear )( 
            IWMPDownloadCollection * This);
        
        END_INTERFACE
    } IWMPDownloadCollectionVtbl;

    interface IWMPDownloadCollection
    {
        CONST_VTBL struct IWMPDownloadCollectionVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWMPDownloadCollection_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWMPDownloadCollection_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWMPDownloadCollection_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWMPDownloadCollection_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IWMPDownloadCollection_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IWMPDownloadCollection_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IWMPDownloadCollection_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IWMPDownloadCollection_get_id(This,plId)	\
    ( (This)->lpVtbl -> get_id(This,plId) ) 

#define IWMPDownloadCollection_get_count(This,plCount)	\
    ( (This)->lpVtbl -> get_count(This,plCount) ) 

#define IWMPDownloadCollection_item(This,lItem,ppDownload)	\
    ( (This)->lpVtbl -> item(This,lItem,ppDownload) ) 

#define IWMPDownloadCollection_startDownload(This,bstrSourceURL,bstrType,ppDownload)	\
    ( (This)->lpVtbl -> startDownload(This,bstrSourceURL,bstrType,ppDownload) ) 

#define IWMPDownloadCollection_removeItem(This,lItem)	\
    ( (This)->lpVtbl -> removeItem(This,lItem) ) 

#define IWMPDownloadCollection_Clear(This)	\
    ( (This)->lpVtbl -> Clear(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWMPDownloadCollection_INTERFACE_DEFINED__ */


#ifndef __IWMPDownloadManager_INTERFACE_DEFINED__
#define __IWMPDownloadManager_INTERFACE_DEFINED__

/* interface IWMPDownloadManager */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IWMPDownloadManager;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("E15E9AD1-8F20-4cc4-9EC7-1A328CA86A0D")
    IWMPDownloadManager : public IDispatch
    {
    public:
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE getDownloadCollection( 
            /* [in] */ long lCollectionId,
            /* [retval][out] */ IWMPDownloadCollection **ppCollection) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE createDownloadCollection( 
            /* [retval][out] */ IWMPDownloadCollection **ppCollection) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWMPDownloadManagerVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IWMPDownloadManager * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IWMPDownloadManager * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IWMPDownloadManager * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            IWMPDownloadManager * This,
            /* [out] */ UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            IWMPDownloadManager * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            IWMPDownloadManager * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR *rgszNames,
            /* [range][in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IWMPDownloadManager * This,
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
        
        DECLSPEC_XFGVIRT(IWMPDownloadManager, getDownloadCollection)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *getDownloadCollection )( 
            IWMPDownloadManager * This,
            /* [in] */ long lCollectionId,
            /* [retval][out] */ IWMPDownloadCollection **ppCollection);
        
        DECLSPEC_XFGVIRT(IWMPDownloadManager, createDownloadCollection)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *createDownloadCollection )( 
            IWMPDownloadManager * This,
            /* [retval][out] */ IWMPDownloadCollection **ppCollection);
        
        END_INTERFACE
    } IWMPDownloadManagerVtbl;

    interface IWMPDownloadManager
    {
        CONST_VTBL struct IWMPDownloadManagerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWMPDownloadManager_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWMPDownloadManager_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWMPDownloadManager_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWMPDownloadManager_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IWMPDownloadManager_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IWMPDownloadManager_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IWMPDownloadManager_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IWMPDownloadManager_getDownloadCollection(This,lCollectionId,ppCollection)	\
    ( (This)->lpVtbl -> getDownloadCollection(This,lCollectionId,ppCollection) ) 

#define IWMPDownloadManager_createDownloadCollection(This,ppCollection)	\
    ( (This)->lpVtbl -> createDownloadCollection(This,ppCollection) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWMPDownloadManager_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_subscriptionservicespri_0000_0007 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_subscriptionservicespri_0000_0007_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_subscriptionservicespri_0000_0007_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

unsigned long             __RPC_USER  BSTR_UserSize(     unsigned long *, unsigned long            , BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserMarshal(  unsigned long *, unsigned char *, BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserUnmarshal(unsigned long *, unsigned char *, BSTR * ); 
void                      __RPC_USER  BSTR_UserFree(     unsigned long *, BSTR * ); 

unsigned long             __RPC_USER  HWND_UserSize(     unsigned long *, unsigned long            , HWND * ); 
unsigned char * __RPC_USER  HWND_UserMarshal(  unsigned long *, unsigned char *, HWND * ); 
unsigned char * __RPC_USER  HWND_UserUnmarshal(unsigned long *, unsigned char *, HWND * ); 
void                      __RPC_USER  HWND_UserFree(     unsigned long *, HWND * ); 

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


