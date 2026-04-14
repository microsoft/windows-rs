

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

#ifndef __windowssideshowapi_h__
#define __windowssideshowapi_h__

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

#ifndef __ISideShowSession_FWD_DEFINED__
#define __ISideShowSession_FWD_DEFINED__
typedef interface ISideShowSession ISideShowSession;

#endif 	/* __ISideShowSession_FWD_DEFINED__ */


#ifndef __ISideShowNotificationManager_FWD_DEFINED__
#define __ISideShowNotificationManager_FWD_DEFINED__
typedef interface ISideShowNotificationManager ISideShowNotificationManager;

#endif 	/* __ISideShowNotificationManager_FWD_DEFINED__ */


#ifndef __ISideShowNotification_FWD_DEFINED__
#define __ISideShowNotification_FWD_DEFINED__
typedef interface ISideShowNotification ISideShowNotification;

#endif 	/* __ISideShowNotification_FWD_DEFINED__ */


#ifndef __ISideShowContentManager_FWD_DEFINED__
#define __ISideShowContentManager_FWD_DEFINED__
typedef interface ISideShowContentManager ISideShowContentManager;

#endif 	/* __ISideShowContentManager_FWD_DEFINED__ */


#ifndef __ISideShowContent_FWD_DEFINED__
#define __ISideShowContent_FWD_DEFINED__
typedef interface ISideShowContent ISideShowContent;

#endif 	/* __ISideShowContent_FWD_DEFINED__ */


#ifndef __ISideShowEvents_FWD_DEFINED__
#define __ISideShowEvents_FWD_DEFINED__
typedef interface ISideShowEvents ISideShowEvents;

#endif 	/* __ISideShowEvents_FWD_DEFINED__ */


#ifndef __ISideShowCapabilities_FWD_DEFINED__
#define __ISideShowCapabilities_FWD_DEFINED__
typedef interface ISideShowCapabilities ISideShowCapabilities;

#endif 	/* __ISideShowCapabilities_FWD_DEFINED__ */


#ifndef __ISideShowCapabilitiesCollection_FWD_DEFINED__
#define __ISideShowCapabilitiesCollection_FWD_DEFINED__
typedef interface ISideShowCapabilitiesCollection ISideShowCapabilitiesCollection;

#endif 	/* __ISideShowCapabilitiesCollection_FWD_DEFINED__ */


#ifndef __ISideShowBulkCapabilities_FWD_DEFINED__
#define __ISideShowBulkCapabilities_FWD_DEFINED__
typedef interface ISideShowBulkCapabilities ISideShowBulkCapabilities;

#endif 	/* __ISideShowBulkCapabilities_FWD_DEFINED__ */


#ifndef __ISideShowKeyCollection_FWD_DEFINED__
#define __ISideShowKeyCollection_FWD_DEFINED__
typedef interface ISideShowKeyCollection ISideShowKeyCollection;

#endif 	/* __ISideShowKeyCollection_FWD_DEFINED__ */


#ifndef __ISideShowPropVariantCollection_FWD_DEFINED__
#define __ISideShowPropVariantCollection_FWD_DEFINED__
typedef interface ISideShowPropVariantCollection ISideShowPropVariantCollection;

#endif 	/* __ISideShowPropVariantCollection_FWD_DEFINED__ */


#ifndef __SideShowSession_FWD_DEFINED__
#define __SideShowSession_FWD_DEFINED__

#ifdef __cplusplus
typedef class SideShowSession SideShowSession;
#else
typedef struct SideShowSession SideShowSession;
#endif /* __cplusplus */

#endif 	/* __SideShowSession_FWD_DEFINED__ */


#ifndef __SideShowNotification_FWD_DEFINED__
#define __SideShowNotification_FWD_DEFINED__

#ifdef __cplusplus
typedef class SideShowNotification SideShowNotification;
#else
typedef struct SideShowNotification SideShowNotification;
#endif /* __cplusplus */

#endif 	/* __SideShowNotification_FWD_DEFINED__ */


#ifndef __SideShowKeyCollection_FWD_DEFINED__
#define __SideShowKeyCollection_FWD_DEFINED__

#ifdef __cplusplus
typedef class SideShowKeyCollection SideShowKeyCollection;
#else
typedef struct SideShowKeyCollection SideShowKeyCollection;
#endif /* __cplusplus */

#endif 	/* __SideShowKeyCollection_FWD_DEFINED__ */


#ifndef __SideShowPropVariantCollection_FWD_DEFINED__
#define __SideShowPropVariantCollection_FWD_DEFINED__

#ifdef __cplusplus
typedef class SideShowPropVariantCollection SideShowPropVariantCollection;
#else
typedef struct SideShowPropVariantCollection SideShowPropVariantCollection;
#endif /* __cplusplus */

#endif 	/* __SideShowPropVariantCollection_FWD_DEFINED__ */


/* header files for imported files */
#include "oaidl.h"
#include "propsys.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_windowssideshowapi_0000_0000 */
/* [local] */ 

#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
#if (_WIN32_WINNT >= 0x0600) // Windows Vista and later
typedef GUID APPLICATION_ID;

typedef GUID ENDPOINT_ID;

typedef LPWSTR DEVICE_ID;

typedef REFGUID REFAPPLICATION_ID;

typedef REFGUID REFENDPOINT_ID;

typedef ENDPOINT_ID *PENDPOINT_ID;

typedef APPLICATION_ID *PAPPLICATION_ID;

typedef DEVICE_ID *PDEVICE_ID;

typedef unsigned long CONTENT_ID;

typedef CONTENT_ID *PCONTENT_ID;

typedef unsigned long NOTIFICATION_ID;

typedef NOTIFICATION_ID *PNOTIFICATION_ID;












extern RPC_IF_HANDLE __MIDL_itf_windowssideshowapi_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_windowssideshowapi_0000_0000_v0_0_s_ifspec;

#ifndef __ISideShowSession_INTERFACE_DEFINED__
#define __ISideShowSession_INTERFACE_DEFINED__

/* interface ISideShowSession */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_ISideShowSession;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("e22331ee-9e7d-4922-9fc2-ab7aa41ce491")
    ISideShowSession : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE RegisterContent( 
            /* [in] */ __RPC__in REFAPPLICATION_ID in_applicationId,
            /* [in] */ __RPC__in REFENDPOINT_ID in_endpointId,
            /* [out] */ __RPC__deref_out_opt ISideShowContentManager **out_ppIContent) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RegisterNotifications( 
            /* [in] */ __RPC__in REFAPPLICATION_ID in_applicationId,
            /* [out] */ __RPC__deref_out_opt ISideShowNotificationManager **out_ppINotification) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISideShowSessionVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ISideShowSession * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ISideShowSession * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ISideShowSession * This);
        
        DECLSPEC_XFGVIRT(ISideShowSession, RegisterContent)
        HRESULT ( STDMETHODCALLTYPE *RegisterContent )( 
            __RPC__in ISideShowSession * This,
            /* [in] */ __RPC__in REFAPPLICATION_ID in_applicationId,
            /* [in] */ __RPC__in REFENDPOINT_ID in_endpointId,
            /* [out] */ __RPC__deref_out_opt ISideShowContentManager **out_ppIContent);
        
        DECLSPEC_XFGVIRT(ISideShowSession, RegisterNotifications)
        HRESULT ( STDMETHODCALLTYPE *RegisterNotifications )( 
            __RPC__in ISideShowSession * This,
            /* [in] */ __RPC__in REFAPPLICATION_ID in_applicationId,
            /* [out] */ __RPC__deref_out_opt ISideShowNotificationManager **out_ppINotification);
        
        END_INTERFACE
    } ISideShowSessionVtbl;

    interface ISideShowSession
    {
        CONST_VTBL struct ISideShowSessionVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISideShowSession_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISideShowSession_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISideShowSession_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISideShowSession_RegisterContent(This,in_applicationId,in_endpointId,out_ppIContent)	\
    ( (This)->lpVtbl -> RegisterContent(This,in_applicationId,in_endpointId,out_ppIContent) ) 

#define ISideShowSession_RegisterNotifications(This,in_applicationId,out_ppINotification)	\
    ( (This)->lpVtbl -> RegisterNotifications(This,in_applicationId,out_ppINotification) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISideShowSession_INTERFACE_DEFINED__ */


#ifndef __ISideShowNotificationManager_INTERFACE_DEFINED__
#define __ISideShowNotificationManager_INTERFACE_DEFINED__

/* interface ISideShowNotificationManager */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_ISideShowNotificationManager;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("63cea909-f2b9-4302-b5e1-c68e6d9ab833")
    ISideShowNotificationManager : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Show( 
            /* [in] */ __RPC__in_opt ISideShowNotification *in_pINotification) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Revoke( 
            /* [in] */ const NOTIFICATION_ID in_notificationId) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RevokeAll( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISideShowNotificationManagerVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ISideShowNotificationManager * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ISideShowNotificationManager * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ISideShowNotificationManager * This);
        
        DECLSPEC_XFGVIRT(ISideShowNotificationManager, Show)
        HRESULT ( STDMETHODCALLTYPE *Show )( 
            __RPC__in ISideShowNotificationManager * This,
            /* [in] */ __RPC__in_opt ISideShowNotification *in_pINotification);
        
        DECLSPEC_XFGVIRT(ISideShowNotificationManager, Revoke)
        HRESULT ( STDMETHODCALLTYPE *Revoke )( 
            __RPC__in ISideShowNotificationManager * This,
            /* [in] */ const NOTIFICATION_ID in_notificationId);
        
        DECLSPEC_XFGVIRT(ISideShowNotificationManager, RevokeAll)
        HRESULT ( STDMETHODCALLTYPE *RevokeAll )( 
            __RPC__in ISideShowNotificationManager * This);
        
        END_INTERFACE
    } ISideShowNotificationManagerVtbl;

    interface ISideShowNotificationManager
    {
        CONST_VTBL struct ISideShowNotificationManagerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISideShowNotificationManager_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISideShowNotificationManager_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISideShowNotificationManager_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISideShowNotificationManager_Show(This,in_pINotification)	\
    ( (This)->lpVtbl -> Show(This,in_pINotification) ) 

#define ISideShowNotificationManager_Revoke(This,in_notificationId)	\
    ( (This)->lpVtbl -> Revoke(This,in_notificationId) ) 

#define ISideShowNotificationManager_RevokeAll(This)	\
    ( (This)->lpVtbl -> RevokeAll(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISideShowNotificationManager_INTERFACE_DEFINED__ */


#ifndef __ISideShowNotification_INTERFACE_DEFINED__
#define __ISideShowNotification_INTERFACE_DEFINED__

/* interface ISideShowNotification */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_ISideShowNotification;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("03c93300-8ab2-41c5-9b79-46127a30e148")
    ISideShowNotification : public IUnknown
    {
    public:
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_NotificationId( 
            /* [retval][out] */ __RPC__out PNOTIFICATION_ID out_pNotificationId) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_NotificationId( 
            /* [in] */ NOTIFICATION_ID in_notificationId) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_Title( 
            /* [retval][string][out] */ __RPC__deref_out_opt_string LPWSTR *out_ppwszTitle) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_Title( 
            /* [string][in] */ __RPC__in_string LPWSTR in_pwszTitle) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_Message( 
            /* [retval][string][out] */ __RPC__deref_out_opt_string LPWSTR *out_ppwszMessage) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_Message( 
            /* [string][in] */ __RPC__in_string LPWSTR in_pwszMessage) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_Image( 
            /* [retval][out] */ __RPC__deref_out_opt HICON *out_phIcon) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_Image( 
            /* [in] */ __RPC__in HICON in_hIcon) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_ExpirationTime( 
            /* [retval][out] */ __RPC__out SYSTEMTIME *out_pTime) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_ExpirationTime( 
            /* [unique][in] */ __RPC__in_opt SYSTEMTIME *in_pTime) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISideShowNotificationVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ISideShowNotification * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ISideShowNotification * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ISideShowNotification * This);
        
        DECLSPEC_XFGVIRT(ISideShowNotification, get_NotificationId)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_NotificationId )( 
            __RPC__in ISideShowNotification * This,
            /* [retval][out] */ __RPC__out PNOTIFICATION_ID out_pNotificationId);
        
        DECLSPEC_XFGVIRT(ISideShowNotification, put_NotificationId)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_NotificationId )( 
            __RPC__in ISideShowNotification * This,
            /* [in] */ NOTIFICATION_ID in_notificationId);
        
        DECLSPEC_XFGVIRT(ISideShowNotification, get_Title)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Title )( 
            __RPC__in ISideShowNotification * This,
            /* [retval][string][out] */ __RPC__deref_out_opt_string LPWSTR *out_ppwszTitle);
        
        DECLSPEC_XFGVIRT(ISideShowNotification, put_Title)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_Title )( 
            __RPC__in ISideShowNotification * This,
            /* [string][in] */ __RPC__in_string LPWSTR in_pwszTitle);
        
        DECLSPEC_XFGVIRT(ISideShowNotification, get_Message)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Message )( 
            __RPC__in ISideShowNotification * This,
            /* [retval][string][out] */ __RPC__deref_out_opt_string LPWSTR *out_ppwszMessage);
        
        DECLSPEC_XFGVIRT(ISideShowNotification, put_Message)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_Message )( 
            __RPC__in ISideShowNotification * This,
            /* [string][in] */ __RPC__in_string LPWSTR in_pwszMessage);
        
        DECLSPEC_XFGVIRT(ISideShowNotification, get_Image)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Image )( 
            __RPC__in ISideShowNotification * This,
            /* [retval][out] */ __RPC__deref_out_opt HICON *out_phIcon);
        
        DECLSPEC_XFGVIRT(ISideShowNotification, put_Image)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_Image )( 
            __RPC__in ISideShowNotification * This,
            /* [in] */ __RPC__in HICON in_hIcon);
        
        DECLSPEC_XFGVIRT(ISideShowNotification, get_ExpirationTime)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_ExpirationTime )( 
            __RPC__in ISideShowNotification * This,
            /* [retval][out] */ __RPC__out SYSTEMTIME *out_pTime);
        
        DECLSPEC_XFGVIRT(ISideShowNotification, put_ExpirationTime)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_ExpirationTime )( 
            __RPC__in ISideShowNotification * This,
            /* [unique][in] */ __RPC__in_opt SYSTEMTIME *in_pTime);
        
        END_INTERFACE
    } ISideShowNotificationVtbl;

    interface ISideShowNotification
    {
        CONST_VTBL struct ISideShowNotificationVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISideShowNotification_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISideShowNotification_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISideShowNotification_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISideShowNotification_get_NotificationId(This,out_pNotificationId)	\
    ( (This)->lpVtbl -> get_NotificationId(This,out_pNotificationId) ) 

#define ISideShowNotification_put_NotificationId(This,in_notificationId)	\
    ( (This)->lpVtbl -> put_NotificationId(This,in_notificationId) ) 

#define ISideShowNotification_get_Title(This,out_ppwszTitle)	\
    ( (This)->lpVtbl -> get_Title(This,out_ppwszTitle) ) 

#define ISideShowNotification_put_Title(This,in_pwszTitle)	\
    ( (This)->lpVtbl -> put_Title(This,in_pwszTitle) ) 

#define ISideShowNotification_get_Message(This,out_ppwszMessage)	\
    ( (This)->lpVtbl -> get_Message(This,out_ppwszMessage) ) 

#define ISideShowNotification_put_Message(This,in_pwszMessage)	\
    ( (This)->lpVtbl -> put_Message(This,in_pwszMessage) ) 

#define ISideShowNotification_get_Image(This,out_phIcon)	\
    ( (This)->lpVtbl -> get_Image(This,out_phIcon) ) 

#define ISideShowNotification_put_Image(This,in_hIcon)	\
    ( (This)->lpVtbl -> put_Image(This,in_hIcon) ) 

#define ISideShowNotification_get_ExpirationTime(This,out_pTime)	\
    ( (This)->lpVtbl -> get_ExpirationTime(This,out_pTime) ) 

#define ISideShowNotification_put_ExpirationTime(This,in_pTime)	\
    ( (This)->lpVtbl -> put_ExpirationTime(This,in_pTime) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISideShowNotification_INTERFACE_DEFINED__ */


#ifndef __ISideShowContentManager_INTERFACE_DEFINED__
#define __ISideShowContentManager_INTERFACE_DEFINED__

/* interface ISideShowContentManager */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_ISideShowContentManager;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("a5d5b66b-eef9-41db-8d7e-e17c33ab10b0")
    ISideShowContentManager : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Add( 
            /* [in] */ __RPC__in_opt ISideShowContent *in_pIContent) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Remove( 
            /* [in] */ const CONTENT_ID in_contentId) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RemoveAll( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetEventSink( 
            /* [unique][in] */ __RPC__in_opt ISideShowEvents *in_pIEvents) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetDeviceCapabilities( 
            /* [out] */ __RPC__deref_out_opt ISideShowCapabilitiesCollection **out_ppCollection) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISideShowContentManagerVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ISideShowContentManager * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ISideShowContentManager * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ISideShowContentManager * This);
        
        DECLSPEC_XFGVIRT(ISideShowContentManager, Add)
        HRESULT ( STDMETHODCALLTYPE *Add )( 
            __RPC__in ISideShowContentManager * This,
            /* [in] */ __RPC__in_opt ISideShowContent *in_pIContent);
        
        DECLSPEC_XFGVIRT(ISideShowContentManager, Remove)
        HRESULT ( STDMETHODCALLTYPE *Remove )( 
            __RPC__in ISideShowContentManager * This,
            /* [in] */ const CONTENT_ID in_contentId);
        
        DECLSPEC_XFGVIRT(ISideShowContentManager, RemoveAll)
        HRESULT ( STDMETHODCALLTYPE *RemoveAll )( 
            __RPC__in ISideShowContentManager * This);
        
        DECLSPEC_XFGVIRT(ISideShowContentManager, SetEventSink)
        HRESULT ( STDMETHODCALLTYPE *SetEventSink )( 
            __RPC__in ISideShowContentManager * This,
            /* [unique][in] */ __RPC__in_opt ISideShowEvents *in_pIEvents);
        
        DECLSPEC_XFGVIRT(ISideShowContentManager, GetDeviceCapabilities)
        HRESULT ( STDMETHODCALLTYPE *GetDeviceCapabilities )( 
            __RPC__in ISideShowContentManager * This,
            /* [out] */ __RPC__deref_out_opt ISideShowCapabilitiesCollection **out_ppCollection);
        
        END_INTERFACE
    } ISideShowContentManagerVtbl;

    interface ISideShowContentManager
    {
        CONST_VTBL struct ISideShowContentManagerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISideShowContentManager_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISideShowContentManager_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISideShowContentManager_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISideShowContentManager_Add(This,in_pIContent)	\
    ( (This)->lpVtbl -> Add(This,in_pIContent) ) 

#define ISideShowContentManager_Remove(This,in_contentId)	\
    ( (This)->lpVtbl -> Remove(This,in_contentId) ) 

#define ISideShowContentManager_RemoveAll(This)	\
    ( (This)->lpVtbl -> RemoveAll(This) ) 

#define ISideShowContentManager_SetEventSink(This,in_pIEvents)	\
    ( (This)->lpVtbl -> SetEventSink(This,in_pIEvents) ) 

#define ISideShowContentManager_GetDeviceCapabilities(This,out_ppCollection)	\
    ( (This)->lpVtbl -> GetDeviceCapabilities(This,out_ppCollection) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISideShowContentManager_INTERFACE_DEFINED__ */


#ifndef __ISideShowContent_INTERFACE_DEFINED__
#define __ISideShowContent_INTERFACE_DEFINED__

/* interface ISideShowContent */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_ISideShowContent;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("c18552ed-74ff-4fec-be07-4cfed29d4887")
    ISideShowContent : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetContent( 
            /* [unique][in] */ __RPC__in_opt ISideShowCapabilities *in_pICapabilities,
            /* [out] */ __RPC__out DWORD *out_pdwSize,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*out_pdwSize) BYTE **out_ppbData) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_ContentId( 
            /* [out] */ __RPC__out PCONTENT_ID out_pcontentId) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_DifferentiateContent( 
            /* [out] */ __RPC__out BOOL *out_pfDifferentiateContent) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISideShowContentVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ISideShowContent * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ISideShowContent * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ISideShowContent * This);
        
        DECLSPEC_XFGVIRT(ISideShowContent, GetContent)
        HRESULT ( STDMETHODCALLTYPE *GetContent )( 
            __RPC__in ISideShowContent * This,
            /* [unique][in] */ __RPC__in_opt ISideShowCapabilities *in_pICapabilities,
            /* [out] */ __RPC__out DWORD *out_pdwSize,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*out_pdwSize) BYTE **out_ppbData);
        
        DECLSPEC_XFGVIRT(ISideShowContent, get_ContentId)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_ContentId )( 
            __RPC__in ISideShowContent * This,
            /* [out] */ __RPC__out PCONTENT_ID out_pcontentId);
        
        DECLSPEC_XFGVIRT(ISideShowContent, get_DifferentiateContent)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_DifferentiateContent )( 
            __RPC__in ISideShowContent * This,
            /* [out] */ __RPC__out BOOL *out_pfDifferentiateContent);
        
        END_INTERFACE
    } ISideShowContentVtbl;

    interface ISideShowContent
    {
        CONST_VTBL struct ISideShowContentVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISideShowContent_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISideShowContent_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISideShowContent_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISideShowContent_GetContent(This,in_pICapabilities,out_pdwSize,out_ppbData)	\
    ( (This)->lpVtbl -> GetContent(This,in_pICapabilities,out_pdwSize,out_ppbData) ) 

#define ISideShowContent_get_ContentId(This,out_pcontentId)	\
    ( (This)->lpVtbl -> get_ContentId(This,out_pcontentId) ) 

#define ISideShowContent_get_DifferentiateContent(This,out_pfDifferentiateContent)	\
    ( (This)->lpVtbl -> get_DifferentiateContent(This,out_pfDifferentiateContent) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISideShowContent_INTERFACE_DEFINED__ */


#ifndef __ISideShowEvents_INTERFACE_DEFINED__
#define __ISideShowEvents_INTERFACE_DEFINED__

/* interface ISideShowEvents */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_ISideShowEvents;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("61feca4c-deb4-4a7e-8d75-51f1132d615b")
    ISideShowEvents : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE ContentMissing( 
            /* [in] */ const CONTENT_ID in_contentId,
            /* [out] */ __RPC__deref_out_opt ISideShowContent **out_ppIContent) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ApplicationEvent( 
            /* [in] */ __RPC__in_opt ISideShowCapabilities *in_pICapabilities,
            /* [in] */ const DWORD in_dwEventId,
            /* [in] */ const DWORD in_dwEventSize,
            /* [size_is][unique][in] */ __RPC__in_ecount_full_opt(in_dwEventSize) const BYTE *in_pbEventData) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE DeviceAdded( 
            /* [in] */ __RPC__in_opt ISideShowCapabilities *in_pIDevice) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE DeviceRemoved( 
            /* [in] */ __RPC__in_opt ISideShowCapabilities *in_pIDevice) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISideShowEventsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ISideShowEvents * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ISideShowEvents * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ISideShowEvents * This);
        
        DECLSPEC_XFGVIRT(ISideShowEvents, ContentMissing)
        HRESULT ( STDMETHODCALLTYPE *ContentMissing )( 
            __RPC__in ISideShowEvents * This,
            /* [in] */ const CONTENT_ID in_contentId,
            /* [out] */ __RPC__deref_out_opt ISideShowContent **out_ppIContent);
        
        DECLSPEC_XFGVIRT(ISideShowEvents, ApplicationEvent)
        HRESULT ( STDMETHODCALLTYPE *ApplicationEvent )( 
            __RPC__in ISideShowEvents * This,
            /* [in] */ __RPC__in_opt ISideShowCapabilities *in_pICapabilities,
            /* [in] */ const DWORD in_dwEventId,
            /* [in] */ const DWORD in_dwEventSize,
            /* [size_is][unique][in] */ __RPC__in_ecount_full_opt(in_dwEventSize) const BYTE *in_pbEventData);
        
        DECLSPEC_XFGVIRT(ISideShowEvents, DeviceAdded)
        HRESULT ( STDMETHODCALLTYPE *DeviceAdded )( 
            __RPC__in ISideShowEvents * This,
            /* [in] */ __RPC__in_opt ISideShowCapabilities *in_pIDevice);
        
        DECLSPEC_XFGVIRT(ISideShowEvents, DeviceRemoved)
        HRESULT ( STDMETHODCALLTYPE *DeviceRemoved )( 
            __RPC__in ISideShowEvents * This,
            /* [in] */ __RPC__in_opt ISideShowCapabilities *in_pIDevice);
        
        END_INTERFACE
    } ISideShowEventsVtbl;

    interface ISideShowEvents
    {
        CONST_VTBL struct ISideShowEventsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISideShowEvents_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISideShowEvents_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISideShowEvents_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISideShowEvents_ContentMissing(This,in_contentId,out_ppIContent)	\
    ( (This)->lpVtbl -> ContentMissing(This,in_contentId,out_ppIContent) ) 

#define ISideShowEvents_ApplicationEvent(This,in_pICapabilities,in_dwEventId,in_dwEventSize,in_pbEventData)	\
    ( (This)->lpVtbl -> ApplicationEvent(This,in_pICapabilities,in_dwEventId,in_dwEventSize,in_pbEventData) ) 

#define ISideShowEvents_DeviceAdded(This,in_pIDevice)	\
    ( (This)->lpVtbl -> DeviceAdded(This,in_pIDevice) ) 

#define ISideShowEvents_DeviceRemoved(This,in_pIDevice)	\
    ( (This)->lpVtbl -> DeviceRemoved(This,in_pIDevice) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISideShowEvents_INTERFACE_DEFINED__ */


#ifndef __ISideShowCapabilities_INTERFACE_DEFINED__
#define __ISideShowCapabilities_INTERFACE_DEFINED__

/* interface ISideShowCapabilities */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_ISideShowCapabilities;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("535e1379-c09e-4a54-a511-597bab3a72b8")
    ISideShowCapabilities : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetCapability( 
            /* [in] */ __RPC__in REFPROPERTYKEY in_keyCapability,
            /* [out][in] */ __RPC__inout PROPVARIANT *inout_pValue) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISideShowCapabilitiesVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ISideShowCapabilities * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ISideShowCapabilities * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ISideShowCapabilities * This);
        
        DECLSPEC_XFGVIRT(ISideShowCapabilities, GetCapability)
        HRESULT ( STDMETHODCALLTYPE *GetCapability )( 
            __RPC__in ISideShowCapabilities * This,
            /* [in] */ __RPC__in REFPROPERTYKEY in_keyCapability,
            /* [out][in] */ __RPC__inout PROPVARIANT *inout_pValue);
        
        END_INTERFACE
    } ISideShowCapabilitiesVtbl;

    interface ISideShowCapabilities
    {
        CONST_VTBL struct ISideShowCapabilitiesVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISideShowCapabilities_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISideShowCapabilities_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISideShowCapabilities_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISideShowCapabilities_GetCapability(This,in_keyCapability,inout_pValue)	\
    ( (This)->lpVtbl -> GetCapability(This,in_keyCapability,inout_pValue) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISideShowCapabilities_INTERFACE_DEFINED__ */


#ifndef __ISideShowCapabilitiesCollection_INTERFACE_DEFINED__
#define __ISideShowCapabilitiesCollection_INTERFACE_DEFINED__

/* interface ISideShowCapabilitiesCollection */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_ISideShowCapabilitiesCollection;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("50305597-5e0d-4ff7-b3af-33d0d9bd52dd")
    ISideShowCapabilitiesCollection : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetCount( 
            /* [out] */ __RPC__out DWORD *out_pdwCount) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetAt( 
            /* [in] */ DWORD in_dwIndex,
            /* [out] */ __RPC__deref_out_opt ISideShowCapabilities **out_ppCapabilities) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISideShowCapabilitiesCollectionVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ISideShowCapabilitiesCollection * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ISideShowCapabilitiesCollection * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ISideShowCapabilitiesCollection * This);
        
        DECLSPEC_XFGVIRT(ISideShowCapabilitiesCollection, GetCount)
        HRESULT ( STDMETHODCALLTYPE *GetCount )( 
            __RPC__in ISideShowCapabilitiesCollection * This,
            /* [out] */ __RPC__out DWORD *out_pdwCount);
        
        DECLSPEC_XFGVIRT(ISideShowCapabilitiesCollection, GetAt)
        HRESULT ( STDMETHODCALLTYPE *GetAt )( 
            __RPC__in ISideShowCapabilitiesCollection * This,
            /* [in] */ DWORD in_dwIndex,
            /* [out] */ __RPC__deref_out_opt ISideShowCapabilities **out_ppCapabilities);
        
        END_INTERFACE
    } ISideShowCapabilitiesCollectionVtbl;

    interface ISideShowCapabilitiesCollection
    {
        CONST_VTBL struct ISideShowCapabilitiesCollectionVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISideShowCapabilitiesCollection_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISideShowCapabilitiesCollection_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISideShowCapabilitiesCollection_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISideShowCapabilitiesCollection_GetCount(This,out_pdwCount)	\
    ( (This)->lpVtbl -> GetCount(This,out_pdwCount) ) 

#define ISideShowCapabilitiesCollection_GetAt(This,in_dwIndex,out_ppCapabilities)	\
    ( (This)->lpVtbl -> GetAt(This,in_dwIndex,out_ppCapabilities) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISideShowCapabilitiesCollection_INTERFACE_DEFINED__ */


#ifndef __ISideShowBulkCapabilities_INTERFACE_DEFINED__
#define __ISideShowBulkCapabilities_INTERFACE_DEFINED__

/* interface ISideShowBulkCapabilities */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_ISideShowBulkCapabilities;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("3A2B7FBC-3AD5-48bd-BBF1-0E6CFBD10807")
    ISideShowBulkCapabilities : public ISideShowCapabilities
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetCapabilities( 
            /* [in] */ __RPC__in_opt ISideShowKeyCollection *in_keyCollection,
            /* [out][in] */ __RPC__deref_inout_opt ISideShowPropVariantCollection **inout_pValues) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISideShowBulkCapabilitiesVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ISideShowBulkCapabilities * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ISideShowBulkCapabilities * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ISideShowBulkCapabilities * This);
        
        DECLSPEC_XFGVIRT(ISideShowCapabilities, GetCapability)
        HRESULT ( STDMETHODCALLTYPE *GetCapability )( 
            __RPC__in ISideShowBulkCapabilities * This,
            /* [in] */ __RPC__in REFPROPERTYKEY in_keyCapability,
            /* [out][in] */ __RPC__inout PROPVARIANT *inout_pValue);
        
        DECLSPEC_XFGVIRT(ISideShowBulkCapabilities, GetCapabilities)
        HRESULT ( STDMETHODCALLTYPE *GetCapabilities )( 
            __RPC__in ISideShowBulkCapabilities * This,
            /* [in] */ __RPC__in_opt ISideShowKeyCollection *in_keyCollection,
            /* [out][in] */ __RPC__deref_inout_opt ISideShowPropVariantCollection **inout_pValues);
        
        END_INTERFACE
    } ISideShowBulkCapabilitiesVtbl;

    interface ISideShowBulkCapabilities
    {
        CONST_VTBL struct ISideShowBulkCapabilitiesVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISideShowBulkCapabilities_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISideShowBulkCapabilities_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISideShowBulkCapabilities_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISideShowBulkCapabilities_GetCapability(This,in_keyCapability,inout_pValue)	\
    ( (This)->lpVtbl -> GetCapability(This,in_keyCapability,inout_pValue) ) 


#define ISideShowBulkCapabilities_GetCapabilities(This,in_keyCollection,inout_pValues)	\
    ( (This)->lpVtbl -> GetCapabilities(This,in_keyCollection,inout_pValues) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISideShowBulkCapabilities_INTERFACE_DEFINED__ */


#ifndef __ISideShowKeyCollection_INTERFACE_DEFINED__
#define __ISideShowKeyCollection_INTERFACE_DEFINED__

/* interface ISideShowKeyCollection */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_ISideShowKeyCollection;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("045473BC-A37B-4957-B144-68105411ED8E")
    ISideShowKeyCollection : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Add( 
            /* [in] */ __RPC__in REFPROPERTYKEY Key) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Clear( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetAt( 
            /* [in] */ const DWORD dwIndex,
            /* [out][in] */ __RPC__inout PROPERTYKEY *pKey) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCount( 
            /* [in] */ __RPC__in DWORD *pcElems) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RemoveAt( 
            /* [in] */ const DWORD dwIndex) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISideShowKeyCollectionVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ISideShowKeyCollection * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ISideShowKeyCollection * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ISideShowKeyCollection * This);
        
        DECLSPEC_XFGVIRT(ISideShowKeyCollection, Add)
        HRESULT ( STDMETHODCALLTYPE *Add )( 
            __RPC__in ISideShowKeyCollection * This,
            /* [in] */ __RPC__in REFPROPERTYKEY Key);
        
        DECLSPEC_XFGVIRT(ISideShowKeyCollection, Clear)
        HRESULT ( STDMETHODCALLTYPE *Clear )( 
            __RPC__in ISideShowKeyCollection * This);
        
        DECLSPEC_XFGVIRT(ISideShowKeyCollection, GetAt)
        HRESULT ( STDMETHODCALLTYPE *GetAt )( 
            __RPC__in ISideShowKeyCollection * This,
            /* [in] */ const DWORD dwIndex,
            /* [out][in] */ __RPC__inout PROPERTYKEY *pKey);
        
        DECLSPEC_XFGVIRT(ISideShowKeyCollection, GetCount)
        HRESULT ( STDMETHODCALLTYPE *GetCount )( 
            __RPC__in ISideShowKeyCollection * This,
            /* [in] */ __RPC__in DWORD *pcElems);
        
        DECLSPEC_XFGVIRT(ISideShowKeyCollection, RemoveAt)
        HRESULT ( STDMETHODCALLTYPE *RemoveAt )( 
            __RPC__in ISideShowKeyCollection * This,
            /* [in] */ const DWORD dwIndex);
        
        END_INTERFACE
    } ISideShowKeyCollectionVtbl;

    interface ISideShowKeyCollection
    {
        CONST_VTBL struct ISideShowKeyCollectionVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISideShowKeyCollection_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISideShowKeyCollection_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISideShowKeyCollection_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISideShowKeyCollection_Add(This,Key)	\
    ( (This)->lpVtbl -> Add(This,Key) ) 

#define ISideShowKeyCollection_Clear(This)	\
    ( (This)->lpVtbl -> Clear(This) ) 

#define ISideShowKeyCollection_GetAt(This,dwIndex,pKey)	\
    ( (This)->lpVtbl -> GetAt(This,dwIndex,pKey) ) 

#define ISideShowKeyCollection_GetCount(This,pcElems)	\
    ( (This)->lpVtbl -> GetCount(This,pcElems) ) 

#define ISideShowKeyCollection_RemoveAt(This,dwIndex)	\
    ( (This)->lpVtbl -> RemoveAt(This,dwIndex) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISideShowKeyCollection_INTERFACE_DEFINED__ */


#ifndef __ISideShowPropVariantCollection_INTERFACE_DEFINED__
#define __ISideShowPropVariantCollection_INTERFACE_DEFINED__

/* interface ISideShowPropVariantCollection */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_ISideShowPropVariantCollection;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("2EA7A549-7BFF-4aae-BAB0-22D43111DE49")
    ISideShowPropVariantCollection : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Add( 
            /* [in] */ __RPC__in const PROPVARIANT *pValue) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Clear( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetAt( 
            /* [in] */ const DWORD dwIndex,
            /* [out][in] */ __RPC__inout PROPVARIANT *pValue) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCount( 
            /* [in] */ __RPC__in DWORD *pcElems) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RemoveAt( 
            /* [in] */ const DWORD dwIndex) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISideShowPropVariantCollectionVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ISideShowPropVariantCollection * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ISideShowPropVariantCollection * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ISideShowPropVariantCollection * This);
        
        DECLSPEC_XFGVIRT(ISideShowPropVariantCollection, Add)
        HRESULT ( STDMETHODCALLTYPE *Add )( 
            __RPC__in ISideShowPropVariantCollection * This,
            /* [in] */ __RPC__in const PROPVARIANT *pValue);
        
        DECLSPEC_XFGVIRT(ISideShowPropVariantCollection, Clear)
        HRESULT ( STDMETHODCALLTYPE *Clear )( 
            __RPC__in ISideShowPropVariantCollection * This);
        
        DECLSPEC_XFGVIRT(ISideShowPropVariantCollection, GetAt)
        HRESULT ( STDMETHODCALLTYPE *GetAt )( 
            __RPC__in ISideShowPropVariantCollection * This,
            /* [in] */ const DWORD dwIndex,
            /* [out][in] */ __RPC__inout PROPVARIANT *pValue);
        
        DECLSPEC_XFGVIRT(ISideShowPropVariantCollection, GetCount)
        HRESULT ( STDMETHODCALLTYPE *GetCount )( 
            __RPC__in ISideShowPropVariantCollection * This,
            /* [in] */ __RPC__in DWORD *pcElems);
        
        DECLSPEC_XFGVIRT(ISideShowPropVariantCollection, RemoveAt)
        HRESULT ( STDMETHODCALLTYPE *RemoveAt )( 
            __RPC__in ISideShowPropVariantCollection * This,
            /* [in] */ const DWORD dwIndex);
        
        END_INTERFACE
    } ISideShowPropVariantCollectionVtbl;

    interface ISideShowPropVariantCollection
    {
        CONST_VTBL struct ISideShowPropVariantCollectionVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISideShowPropVariantCollection_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISideShowPropVariantCollection_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISideShowPropVariantCollection_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISideShowPropVariantCollection_Add(This,pValue)	\
    ( (This)->lpVtbl -> Add(This,pValue) ) 

#define ISideShowPropVariantCollection_Clear(This)	\
    ( (This)->lpVtbl -> Clear(This) ) 

#define ISideShowPropVariantCollection_GetAt(This,dwIndex,pValue)	\
    ( (This)->lpVtbl -> GetAt(This,dwIndex,pValue) ) 

#define ISideShowPropVariantCollection_GetCount(This,pcElems)	\
    ( (This)->lpVtbl -> GetCount(This,pcElems) ) 

#define ISideShowPropVariantCollection_RemoveAt(This,dwIndex)	\
    ( (This)->lpVtbl -> RemoveAt(This,dwIndex) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISideShowPropVariantCollection_INTERFACE_DEFINED__ */



#ifndef __WindowsSideShowApiLibrary_LIBRARY_DEFINED__
#define __WindowsSideShowApiLibrary_LIBRARY_DEFINED__

/* library WindowsSideShowApiLibrary */
/* [helpstring][version][uuid] */ 


EXTERN_C const IID LIBID_WindowsSideShowApiLibrary;

EXTERN_C const CLSID CLSID_SideShowSession;

#ifdef __cplusplus

class DECLSPEC_UUID("e20543b9-f785-4ea2-981e-c4ffa76bbc7c")
SideShowSession;
#endif

EXTERN_C const CLSID CLSID_SideShowNotification;

#ifdef __cplusplus

class DECLSPEC_UUID("0ce3e86f-d5cd-4525-a766-1abab1a752f5")
SideShowNotification;
#endif

EXTERN_C const CLSID CLSID_SideShowKeyCollection;

#ifdef __cplusplus

class DECLSPEC_UUID("DFBBDBF8-18DE-49b8-83DC-EBC727C62D94")
SideShowKeyCollection;
#endif

EXTERN_C const CLSID CLSID_SideShowPropVariantCollection;

#ifdef __cplusplus

class DECLSPEC_UUID("E640F415-539E-4923-96CD-5F093BC250CD")
SideShowPropVariantCollection;
#endif
#endif /* __WindowsSideShowApiLibrary_LIBRARY_DEFINED__ */

/* interface __MIDL_itf_windowssideshowapi_0000_0012 */
/* [local] */ 

#endif // (_WIN32_WINNT >= 0x0600)
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_windowssideshowapi_0000_0012_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_windowssideshowapi_0000_0012_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

unsigned long             __RPC_USER  BSTR_UserSize(     __RPC__in unsigned long *, unsigned long            , __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserMarshal(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out BSTR * ); 
void                      __RPC_USER  BSTR_UserFree(     __RPC__in unsigned long *, __RPC__in BSTR * ); 

unsigned long             __RPC_USER  HICON_UserSize(     __RPC__in unsigned long *, unsigned long            , __RPC__in HICON * ); 
unsigned char * __RPC_USER  HICON_UserMarshal(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in HICON * ); 
unsigned char * __RPC_USER  HICON_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out HICON * ); 
void                      __RPC_USER  HICON_UserFree(     __RPC__in unsigned long *, __RPC__in HICON * ); 

unsigned long             __RPC_USER  LPSAFEARRAY_UserSize(     __RPC__in unsigned long *, unsigned long            , __RPC__in LPSAFEARRAY * ); 
unsigned char * __RPC_USER  LPSAFEARRAY_UserMarshal(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in LPSAFEARRAY * ); 
unsigned char * __RPC_USER  LPSAFEARRAY_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out LPSAFEARRAY * ); 
void                      __RPC_USER  LPSAFEARRAY_UserFree(     __RPC__in unsigned long *, __RPC__in LPSAFEARRAY * ); 

unsigned long             __RPC_USER  BSTR_UserSize64(     __RPC__in unsigned long *, unsigned long            , __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserMarshal64(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out BSTR * ); 
void                      __RPC_USER  BSTR_UserFree64(     __RPC__in unsigned long *, __RPC__in BSTR * ); 

unsigned long             __RPC_USER  HICON_UserSize64(     __RPC__in unsigned long *, unsigned long            , __RPC__in HICON * ); 
unsigned char * __RPC_USER  HICON_UserMarshal64(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in HICON * ); 
unsigned char * __RPC_USER  HICON_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out HICON * ); 
void                      __RPC_USER  HICON_UserFree64(     __RPC__in unsigned long *, __RPC__in HICON * ); 

unsigned long             __RPC_USER  LPSAFEARRAY_UserSize64(     __RPC__in unsigned long *, unsigned long            , __RPC__in LPSAFEARRAY * ); 
unsigned char * __RPC_USER  LPSAFEARRAY_UserMarshal64(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in LPSAFEARRAY * ); 
unsigned char * __RPC_USER  LPSAFEARRAY_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out LPSAFEARRAY * ); 
void                      __RPC_USER  LPSAFEARRAY_UserFree64(     __RPC__in unsigned long *, __RPC__in LPSAFEARRAY * ); 

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


