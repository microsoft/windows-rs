

/* this ALWAYS GENERATED file contains the definitions for the interfaces */


 /* File created by MIDL compiler version 8.01.0628 */
/* @@MIDL_FILE_HEADING(  ) */



/* verify that the <rpcndr.h> version is high enough to compile this file*/
#ifndef __REQUIRED_RPCNDR_H_VERSION__
#define __REQUIRED_RPCNDR_H_VERSION__ 500
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

#ifndef __webapplication_h__
#define __webapplication_h__

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

#ifndef __IWebApplicationScriptEvents_FWD_DEFINED__
#define __IWebApplicationScriptEvents_FWD_DEFINED__
typedef interface IWebApplicationScriptEvents IWebApplicationScriptEvents;

#endif 	/* __IWebApplicationScriptEvents_FWD_DEFINED__ */


#ifndef __IWebApplicationNavigationEvents_FWD_DEFINED__
#define __IWebApplicationNavigationEvents_FWD_DEFINED__
typedef interface IWebApplicationNavigationEvents IWebApplicationNavigationEvents;

#endif 	/* __IWebApplicationNavigationEvents_FWD_DEFINED__ */


#ifndef __IWebApplicationUIEvents_FWD_DEFINED__
#define __IWebApplicationUIEvents_FWD_DEFINED__
typedef interface IWebApplicationUIEvents IWebApplicationUIEvents;

#endif 	/* __IWebApplicationUIEvents_FWD_DEFINED__ */


#ifndef __IWebApplicationUpdateEvents_FWD_DEFINED__
#define __IWebApplicationUpdateEvents_FWD_DEFINED__
typedef interface IWebApplicationUpdateEvents IWebApplicationUpdateEvents;

#endif 	/* __IWebApplicationUpdateEvents_FWD_DEFINED__ */


#ifndef __IWebApplicationHost_FWD_DEFINED__
#define __IWebApplicationHost_FWD_DEFINED__
typedef interface IWebApplicationHost IWebApplicationHost;

#endif 	/* __IWebApplicationHost_FWD_DEFINED__ */


#ifndef __IWebApplicationActivation_FWD_DEFINED__
#define __IWebApplicationActivation_FWD_DEFINED__
typedef interface IWebApplicationActivation IWebApplicationActivation;

#endif 	/* __IWebApplicationActivation_FWD_DEFINED__ */


#ifndef __IWebApplicationAuthoringMode_FWD_DEFINED__
#define __IWebApplicationAuthoringMode_FWD_DEFINED__
typedef interface IWebApplicationAuthoringMode IWebApplicationAuthoringMode;

#endif 	/* __IWebApplicationAuthoringMode_FWD_DEFINED__ */


/* header files for imported files */
#include "oaidl.h"
#include "ocidl.h"
#include "Mshtml.h"
#include "activscp.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_webapplication_0000_0000 */
/* [local] */ 

/*******************************************************
 *                                                     *
 *     Copyright (C) Microsoft. All rights reserved.   *
 *                                                     *
 *******************************************************/
#pragma once

#pragma comment(lib,"uuid.lib")

#include <urlmon.h>
#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


extern RPC_IF_HANDLE __MIDL_itf_webapplication_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_webapplication_0000_0000_v0_0_s_ifspec;

#ifndef __IWebApplicationScriptEvents_INTERFACE_DEFINED__
#define __IWebApplicationScriptEvents_INTERFACE_DEFINED__

/* interface IWebApplicationScriptEvents */
/* [uuid][local][object] */ 


EXTERN_C const IID IID_IWebApplicationScriptEvents;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("7C3F6998-1567-4BBA-B52B-48D32141D613")
    IWebApplicationScriptEvents : public IUnknown
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE BeforeScriptExecute( 
            /* [in] */ IHTMLWindow2 *htmlWindow) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE ScriptError( 
            /* [annotation][in] */ 
            _In_opt_  IHTMLWindow2 *htmlWindow,
            /* [in] */ IActiveScriptError *scriptError,
            /* [string][in] */ LPCWSTR url,
            /* [in] */ BOOL errorHandled) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWebApplicationScriptEventsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IWebApplicationScriptEvents * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IWebApplicationScriptEvents * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IWebApplicationScriptEvents * This);
        
        DECLSPEC_XFGVIRT(IWebApplicationScriptEvents, BeforeScriptExecute)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *BeforeScriptExecute )( 
            IWebApplicationScriptEvents * This,
            /* [in] */ IHTMLWindow2 *htmlWindow);
        
        DECLSPEC_XFGVIRT(IWebApplicationScriptEvents, ScriptError)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *ScriptError )( 
            IWebApplicationScriptEvents * This,
            /* [annotation][in] */ 
            _In_opt_  IHTMLWindow2 *htmlWindow,
            /* [in] */ IActiveScriptError *scriptError,
            /* [string][in] */ LPCWSTR url,
            /* [in] */ BOOL errorHandled);
        
        END_INTERFACE
    } IWebApplicationScriptEventsVtbl;

    interface IWebApplicationScriptEvents
    {
        CONST_VTBL struct IWebApplicationScriptEventsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWebApplicationScriptEvents_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWebApplicationScriptEvents_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWebApplicationScriptEvents_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWebApplicationScriptEvents_BeforeScriptExecute(This,htmlWindow)	\
    ( (This)->lpVtbl -> BeforeScriptExecute(This,htmlWindow) ) 

#define IWebApplicationScriptEvents_ScriptError(This,htmlWindow,scriptError,url,errorHandled)	\
    ( (This)->lpVtbl -> ScriptError(This,htmlWindow,scriptError,url,errorHandled) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWebApplicationScriptEvents_INTERFACE_DEFINED__ */


#ifndef __IWebApplicationNavigationEvents_INTERFACE_DEFINED__
#define __IWebApplicationNavigationEvents_INTERFACE_DEFINED__

/* interface IWebApplicationNavigationEvents */
/* [uuid][local][object] */ 


EXTERN_C const IID IID_IWebApplicationNavigationEvents;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("C22615D2-D318-4DA2-8422-1FCAF77B10E4")
    IWebApplicationNavigationEvents : public IUnknown
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE BeforeNavigate( 
            /* [in] */ IHTMLWindow2 *htmlWindow,
            /* [string][in] */ LPCWSTR url,
            /* [in] */ DWORD navigationFlags,
            /* [annotation][string][in] */ 
            _In_opt_  LPCWSTR targetFrameName) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE NavigateComplete( 
            /* [in] */ IHTMLWindow2 *htmlWindow,
            /* [string][in] */ LPCWSTR url) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE NavigateError( 
            /* [in] */ IHTMLWindow2 *htmlWindow,
            /* [string][in] */ LPCWSTR url,
            /* [annotation][string][in] */ 
            _In_opt_  LPCWSTR targetFrameName,
            /* [in] */ DWORD statusCode) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE DocumentComplete( 
            /* [in] */ IHTMLWindow2 *htmlWindow,
            /* [string][in] */ LPCWSTR url) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE DownloadBegin( void) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE DownloadComplete( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWebApplicationNavigationEventsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IWebApplicationNavigationEvents * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IWebApplicationNavigationEvents * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IWebApplicationNavigationEvents * This);
        
        DECLSPEC_XFGVIRT(IWebApplicationNavigationEvents, BeforeNavigate)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *BeforeNavigate )( 
            IWebApplicationNavigationEvents * This,
            /* [in] */ IHTMLWindow2 *htmlWindow,
            /* [string][in] */ LPCWSTR url,
            /* [in] */ DWORD navigationFlags,
            /* [annotation][string][in] */ 
            _In_opt_  LPCWSTR targetFrameName);
        
        DECLSPEC_XFGVIRT(IWebApplicationNavigationEvents, NavigateComplete)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *NavigateComplete )( 
            IWebApplicationNavigationEvents * This,
            /* [in] */ IHTMLWindow2 *htmlWindow,
            /* [string][in] */ LPCWSTR url);
        
        DECLSPEC_XFGVIRT(IWebApplicationNavigationEvents, NavigateError)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *NavigateError )( 
            IWebApplicationNavigationEvents * This,
            /* [in] */ IHTMLWindow2 *htmlWindow,
            /* [string][in] */ LPCWSTR url,
            /* [annotation][string][in] */ 
            _In_opt_  LPCWSTR targetFrameName,
            /* [in] */ DWORD statusCode);
        
        DECLSPEC_XFGVIRT(IWebApplicationNavigationEvents, DocumentComplete)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *DocumentComplete )( 
            IWebApplicationNavigationEvents * This,
            /* [in] */ IHTMLWindow2 *htmlWindow,
            /* [string][in] */ LPCWSTR url);
        
        DECLSPEC_XFGVIRT(IWebApplicationNavigationEvents, DownloadBegin)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *DownloadBegin )( 
            IWebApplicationNavigationEvents * This);
        
        DECLSPEC_XFGVIRT(IWebApplicationNavigationEvents, DownloadComplete)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *DownloadComplete )( 
            IWebApplicationNavigationEvents * This);
        
        END_INTERFACE
    } IWebApplicationNavigationEventsVtbl;

    interface IWebApplicationNavigationEvents
    {
        CONST_VTBL struct IWebApplicationNavigationEventsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWebApplicationNavigationEvents_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWebApplicationNavigationEvents_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWebApplicationNavigationEvents_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWebApplicationNavigationEvents_BeforeNavigate(This,htmlWindow,url,navigationFlags,targetFrameName)	\
    ( (This)->lpVtbl -> BeforeNavigate(This,htmlWindow,url,navigationFlags,targetFrameName) ) 

#define IWebApplicationNavigationEvents_NavigateComplete(This,htmlWindow,url)	\
    ( (This)->lpVtbl -> NavigateComplete(This,htmlWindow,url) ) 

#define IWebApplicationNavigationEvents_NavigateError(This,htmlWindow,url,targetFrameName,statusCode)	\
    ( (This)->lpVtbl -> NavigateError(This,htmlWindow,url,targetFrameName,statusCode) ) 

#define IWebApplicationNavigationEvents_DocumentComplete(This,htmlWindow,url)	\
    ( (This)->lpVtbl -> DocumentComplete(This,htmlWindow,url) ) 

#define IWebApplicationNavigationEvents_DownloadBegin(This)	\
    ( (This)->lpVtbl -> DownloadBegin(This) ) 

#define IWebApplicationNavigationEvents_DownloadComplete(This)	\
    ( (This)->lpVtbl -> DownloadComplete(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWebApplicationNavigationEvents_INTERFACE_DEFINED__ */


#ifndef __IWebApplicationUIEvents_INTERFACE_DEFINED__
#define __IWebApplicationUIEvents_INTERFACE_DEFINED__

/* interface IWebApplicationUIEvents */
/* [uuid][local][object] */ 


EXTERN_C const IID IID_IWebApplicationUIEvents;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("5B2B3F99-328C-41D5-A6f7-7483ED8E71DD")
    IWebApplicationUIEvents : public IUnknown
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE SecurityProblem( 
            /* [in] */ DWORD securityProblem,
            /* [out] */ HRESULT *result) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWebApplicationUIEventsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IWebApplicationUIEvents * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IWebApplicationUIEvents * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IWebApplicationUIEvents * This);
        
        DECLSPEC_XFGVIRT(IWebApplicationUIEvents, SecurityProblem)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *SecurityProblem )( 
            IWebApplicationUIEvents * This,
            /* [in] */ DWORD securityProblem,
            /* [out] */ HRESULT *result);
        
        END_INTERFACE
    } IWebApplicationUIEventsVtbl;

    interface IWebApplicationUIEvents
    {
        CONST_VTBL struct IWebApplicationUIEventsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWebApplicationUIEvents_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWebApplicationUIEvents_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWebApplicationUIEvents_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWebApplicationUIEvents_SecurityProblem(This,securityProblem,result)	\
    ( (This)->lpVtbl -> SecurityProblem(This,securityProblem,result) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWebApplicationUIEvents_INTERFACE_DEFINED__ */


#ifndef __IWebApplicationUpdateEvents_INTERFACE_DEFINED__
#define __IWebApplicationUpdateEvents_INTERFACE_DEFINED__

/* interface IWebApplicationUpdateEvents */
/* [uuid][local][object] */ 


EXTERN_C const IID IID_IWebApplicationUpdateEvents;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("3E59E6B7-C652-4DAF-AD5E-16FEB350CDE3")
    IWebApplicationUpdateEvents : public IUnknown
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE OnPaint( void) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE OnCssChanged( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWebApplicationUpdateEventsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IWebApplicationUpdateEvents * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IWebApplicationUpdateEvents * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IWebApplicationUpdateEvents * This);
        
        DECLSPEC_XFGVIRT(IWebApplicationUpdateEvents, OnPaint)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *OnPaint )( 
            IWebApplicationUpdateEvents * This);
        
        DECLSPEC_XFGVIRT(IWebApplicationUpdateEvents, OnCssChanged)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *OnCssChanged )( 
            IWebApplicationUpdateEvents * This);
        
        END_INTERFACE
    } IWebApplicationUpdateEventsVtbl;

    interface IWebApplicationUpdateEvents
    {
        CONST_VTBL struct IWebApplicationUpdateEventsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWebApplicationUpdateEvents_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWebApplicationUpdateEvents_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWebApplicationUpdateEvents_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWebApplicationUpdateEvents_OnPaint(This)	\
    ( (This)->lpVtbl -> OnPaint(This) ) 

#define IWebApplicationUpdateEvents_OnCssChanged(This)	\
    ( (This)->lpVtbl -> OnCssChanged(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWebApplicationUpdateEvents_INTERFACE_DEFINED__ */


#ifndef __IWebApplicationHost_INTERFACE_DEFINED__
#define __IWebApplicationHost_INTERFACE_DEFINED__

/* interface IWebApplicationHost */
/* [uuid][local][object] */ 


EXTERN_C const IID IID_IWebApplicationHost;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("CECBD2C3-A3A5-4749-9681-20E9161C6794")
    IWebApplicationHost : public IUnknown
    {
    public:
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_HWND( 
            /* [retval][out] */ HWND *hwnd) = 0;
        
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_Document( 
            /* [retval][out] */ IHTMLDocument2 **htmlDocument) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE Refresh( void) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE Advise( 
            /* [in] */ REFIID interfaceId,
            /* [in] */ IUnknown *callback,
            /* [out] */ DWORD *cookie) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE Unadvise( 
            /* [in] */ DWORD cookie) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWebApplicationHostVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IWebApplicationHost * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IWebApplicationHost * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IWebApplicationHost * This);
        
        DECLSPEC_XFGVIRT(IWebApplicationHost, get_HWND)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_HWND )( 
            IWebApplicationHost * This,
            /* [retval][out] */ HWND *hwnd);
        
        DECLSPEC_XFGVIRT(IWebApplicationHost, get_Document)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Document )( 
            IWebApplicationHost * This,
            /* [retval][out] */ IHTMLDocument2 **htmlDocument);
        
        DECLSPEC_XFGVIRT(IWebApplicationHost, Refresh)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *Refresh )( 
            IWebApplicationHost * This);
        
        DECLSPEC_XFGVIRT(IWebApplicationHost, Advise)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *Advise )( 
            IWebApplicationHost * This,
            /* [in] */ REFIID interfaceId,
            /* [in] */ IUnknown *callback,
            /* [out] */ DWORD *cookie);
        
        DECLSPEC_XFGVIRT(IWebApplicationHost, Unadvise)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *Unadvise )( 
            IWebApplicationHost * This,
            /* [in] */ DWORD cookie);
        
        END_INTERFACE
    } IWebApplicationHostVtbl;

    interface IWebApplicationHost
    {
        CONST_VTBL struct IWebApplicationHostVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWebApplicationHost_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWebApplicationHost_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWebApplicationHost_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWebApplicationHost_get_HWND(This,hwnd)	\
    ( (This)->lpVtbl -> get_HWND(This,hwnd) ) 

#define IWebApplicationHost_get_Document(This,htmlDocument)	\
    ( (This)->lpVtbl -> get_Document(This,htmlDocument) ) 

#define IWebApplicationHost_Refresh(This)	\
    ( (This)->lpVtbl -> Refresh(This) ) 

#define IWebApplicationHost_Advise(This,interfaceId,callback,cookie)	\
    ( (This)->lpVtbl -> Advise(This,interfaceId,callback,cookie) ) 

#define IWebApplicationHost_Unadvise(This,cookie)	\
    ( (This)->lpVtbl -> Unadvise(This,cookie) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWebApplicationHost_INTERFACE_DEFINED__ */


#ifndef __IWebApplicationActivation_INTERFACE_DEFINED__
#define __IWebApplicationActivation_INTERFACE_DEFINED__

/* interface IWebApplicationActivation */
/* [uuid][local][object] */ 


EXTERN_C const IID IID_IWebApplicationActivation;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("BCDCD0DE-330E-481B-B843-4898A6A8EBAC")
    IWebApplicationActivation : public IUnknown
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE CancelPendingActivation( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWebApplicationActivationVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IWebApplicationActivation * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IWebApplicationActivation * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IWebApplicationActivation * This);
        
        DECLSPEC_XFGVIRT(IWebApplicationActivation, CancelPendingActivation)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *CancelPendingActivation )( 
            IWebApplicationActivation * This);
        
        END_INTERFACE
    } IWebApplicationActivationVtbl;

    interface IWebApplicationActivation
    {
        CONST_VTBL struct IWebApplicationActivationVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWebApplicationActivation_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWebApplicationActivation_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWebApplicationActivation_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWebApplicationActivation_CancelPendingActivation(This)	\
    ( (This)->lpVtbl -> CancelPendingActivation(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWebApplicationActivation_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_webapplication_0000_0006 */
/* [local] */ 

// Implemented by the activator. The query service implementation is required to respond to
// requests for SID_SWebApplicationAuthor.


extern RPC_IF_HANDLE __MIDL_itf_webapplication_0000_0006_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_webapplication_0000_0006_v0_0_s_ifspec;

#ifndef __IWebApplicationAuthoringMode_INTERFACE_DEFINED__
#define __IWebApplicationAuthoringMode_INTERFACE_DEFINED__

/* interface IWebApplicationAuthoringMode */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IWebApplicationAuthoringMode;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("720AEA93-1964-4DB0-B005-29EB9E2B18A9")
    IWebApplicationAuthoringMode : public IServiceProvider
    {
    public:
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_AuthoringClientBinary( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *designModeDllPath) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWebApplicationAuthoringModeVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IWebApplicationAuthoringMode * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IWebApplicationAuthoringMode * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IWebApplicationAuthoringMode * This);
        
        DECLSPEC_XFGVIRT(IServiceProvider, QueryService)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *QueryService )( 
            IWebApplicationAuthoringMode * This,
            /* [annotation][in] */ 
            _In_  REFGUID guidService,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][out] */ 
            _Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IWebApplicationAuthoringMode, get_AuthoringClientBinary)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_AuthoringClientBinary )( 
            __RPC__in IWebApplicationAuthoringMode * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *designModeDllPath);
        
        END_INTERFACE
    } IWebApplicationAuthoringModeVtbl;

    interface IWebApplicationAuthoringMode
    {
        CONST_VTBL struct IWebApplicationAuthoringModeVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWebApplicationAuthoringMode_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWebApplicationAuthoringMode_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWebApplicationAuthoringMode_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWebApplicationAuthoringMode_QueryService(This,guidService,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryService(This,guidService,riid,ppvObject) ) 


#define IWebApplicationAuthoringMode_get_AuthoringClientBinary(This,designModeDllPath)	\
    ( (This)->lpVtbl -> get_AuthoringClientBinary(This,designModeDllPath) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWebApplicationAuthoringMode_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_webapplication_0000_0007 */
/* [local] */ 


#define SID_SWebApplicationAuthor IID_IWebApplicationAuthoringMode

typedef HRESULT (*RegisterAuthoringClientFunctionType)(_In_ IWebApplicationAuthoringMode* authoringModeObject, _In_ IWebApplicationHost* host);

typedef HRESULT (* UnregisterAuthoringClientFunctionType)(_In_ IWebApplicationHost* host);
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_webapplication_0000_0007_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_webapplication_0000_0007_v0_0_s_ifspec;

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


