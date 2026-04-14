

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

#ifndef __webevnts_h__
#define __webevnts_h__

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

#ifndef __IWebBrowserEventsService_FWD_DEFINED__
#define __IWebBrowserEventsService_FWD_DEFINED__
typedef interface IWebBrowserEventsService IWebBrowserEventsService;

#endif 	/* __IWebBrowserEventsService_FWD_DEFINED__ */


#ifndef __IWebBrowserEventsUrlService_FWD_DEFINED__
#define __IWebBrowserEventsUrlService_FWD_DEFINED__
typedef interface IWebBrowserEventsUrlService IWebBrowserEventsUrlService;

#endif 	/* __IWebBrowserEventsUrlService_FWD_DEFINED__ */


/* header files for imported files */
#include "objidl.h"
#include "oleidl.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_webevnts_0000_0000 */
/* [local] */ 

//=--------------------------------------------------------------------------=
// webevnts.h
//=--------------------------------------------------------------------------=
// (C) Copyright Microsoft Corporation.  All Rights Reserved.
//
// THIS CODE AND INFORMATION IS PROVIDED "AS IS" WITHOUT WARRANTY OF
// ANY KIND, EITHER EXPRESSED OR IMPLIED, INCLUDING BUT NOT LIMITED TO
// THE IMPLIED WARRANTIES OF MERCHANTABILITY AND/OR FITNESS FOR A
// PARTICULAR PURPOSE.
//=--------------------------------------------------------------------------=

#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
#pragma comment(lib,"uuid.lib")

//---------------------------------------------------------------------------=
// IWebBrowserEventsService and IWebBrowserUrlForEvents Interfaces.



#ifndef _LPWEBEVNTS_DEFINED
#define _LPWEBEVNTS_DEFINED


extern RPC_IF_HANDLE __MIDL_itf_webevnts_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_webevnts_0000_0000_v0_0_s_ifspec;

#ifndef __IWebBrowserEventsService_INTERFACE_DEFINED__
#define __IWebBrowserEventsService_INTERFACE_DEFINED__

/* interface IWebBrowserEventsService */
/* [object][hidden][helpcontext][helpstring][uuid] */ 


EXTERN_C const IID IID_IWebBrowserEventsService;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("54A8F188-9EBD-4795-AD16-9B4945119636")
    IWebBrowserEventsService : public IUnknown
    {
    public:
        virtual /* [helpcontext][helpstring] */ HRESULT STDMETHODCALLTYPE FireBeforeNavigate2Event( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pfCancel) = 0;
        
        virtual /* [helpcontext][helpstring] */ HRESULT STDMETHODCALLTYPE FireNavigateComplete2Event( void) = 0;
        
        virtual /* [helpcontext][helpstring] */ HRESULT STDMETHODCALLTYPE FireDownloadBeginEvent( void) = 0;
        
        virtual /* [helpcontext][helpstring] */ HRESULT STDMETHODCALLTYPE FireDownloadCompleteEvent( void) = 0;
        
        virtual /* [helpcontext][helpstring] */ HRESULT STDMETHODCALLTYPE FireDocumentCompleteEvent( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWebBrowserEventsServiceVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IWebBrowserEventsService * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IWebBrowserEventsService * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IWebBrowserEventsService * This);
        
        DECLSPEC_XFGVIRT(IWebBrowserEventsService, FireBeforeNavigate2Event)
        /* [helpcontext][helpstring] */ HRESULT ( STDMETHODCALLTYPE *FireBeforeNavigate2Event )( 
            __RPC__in IWebBrowserEventsService * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pfCancel);
        
        DECLSPEC_XFGVIRT(IWebBrowserEventsService, FireNavigateComplete2Event)
        /* [helpcontext][helpstring] */ HRESULT ( STDMETHODCALLTYPE *FireNavigateComplete2Event )( 
            __RPC__in IWebBrowserEventsService * This);
        
        DECLSPEC_XFGVIRT(IWebBrowserEventsService, FireDownloadBeginEvent)
        /* [helpcontext][helpstring] */ HRESULT ( STDMETHODCALLTYPE *FireDownloadBeginEvent )( 
            __RPC__in IWebBrowserEventsService * This);
        
        DECLSPEC_XFGVIRT(IWebBrowserEventsService, FireDownloadCompleteEvent)
        /* [helpcontext][helpstring] */ HRESULT ( STDMETHODCALLTYPE *FireDownloadCompleteEvent )( 
            __RPC__in IWebBrowserEventsService * This);
        
        DECLSPEC_XFGVIRT(IWebBrowserEventsService, FireDocumentCompleteEvent)
        /* [helpcontext][helpstring] */ HRESULT ( STDMETHODCALLTYPE *FireDocumentCompleteEvent )( 
            __RPC__in IWebBrowserEventsService * This);
        
        END_INTERFACE
    } IWebBrowserEventsServiceVtbl;

    interface IWebBrowserEventsService
    {
        CONST_VTBL struct IWebBrowserEventsServiceVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWebBrowserEventsService_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWebBrowserEventsService_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWebBrowserEventsService_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWebBrowserEventsService_FireBeforeNavigate2Event(This,pfCancel)	\
    ( (This)->lpVtbl -> FireBeforeNavigate2Event(This,pfCancel) ) 

#define IWebBrowserEventsService_FireNavigateComplete2Event(This)	\
    ( (This)->lpVtbl -> FireNavigateComplete2Event(This) ) 

#define IWebBrowserEventsService_FireDownloadBeginEvent(This)	\
    ( (This)->lpVtbl -> FireDownloadBeginEvent(This) ) 

#define IWebBrowserEventsService_FireDownloadCompleteEvent(This)	\
    ( (This)->lpVtbl -> FireDownloadCompleteEvent(This) ) 

#define IWebBrowserEventsService_FireDocumentCompleteEvent(This)	\
    ( (This)->lpVtbl -> FireDocumentCompleteEvent(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWebBrowserEventsService_INTERFACE_DEFINED__ */


#ifndef __IWebBrowserEventsUrlService_INTERFACE_DEFINED__
#define __IWebBrowserEventsUrlService_INTERFACE_DEFINED__

/* interface IWebBrowserEventsUrlService */
/* [object][hidden][helpcontext][helpstring][uuid] */ 


EXTERN_C const IID IID_IWebBrowserEventsUrlService;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("87CC5D04-EAFA-4833-9820-8F986530CC00")
    IWebBrowserEventsUrlService : public IUnknown
    {
    public:
        virtual /* [helpcontext][helpstring] */ HRESULT STDMETHODCALLTYPE GetUrlForEvents( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pUrl) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWebBrowserEventsUrlServiceVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IWebBrowserEventsUrlService * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IWebBrowserEventsUrlService * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IWebBrowserEventsUrlService * This);
        
        DECLSPEC_XFGVIRT(IWebBrowserEventsUrlService, GetUrlForEvents)
        /* [helpcontext][helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetUrlForEvents )( 
            __RPC__in IWebBrowserEventsUrlService * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pUrl);
        
        END_INTERFACE
    } IWebBrowserEventsUrlServiceVtbl;

    interface IWebBrowserEventsUrlService
    {
        CONST_VTBL struct IWebBrowserEventsUrlServiceVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWebBrowserEventsUrlService_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWebBrowserEventsUrlService_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWebBrowserEventsUrlService_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWebBrowserEventsUrlService_GetUrlForEvents(This,pUrl)	\
    ( (This)->lpVtbl -> GetUrlForEvents(This,pUrl) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWebBrowserEventsUrlService_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_webevnts_0000_0002 */
/* [local] */ 

#endif
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_webevnts_0000_0002_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_webevnts_0000_0002_v0_0_s_ifspec;

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


