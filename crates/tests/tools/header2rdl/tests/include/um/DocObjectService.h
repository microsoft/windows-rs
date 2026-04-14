

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

#ifndef __docobjectservice_h__
#define __docobjectservice_h__

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

#ifndef __IDocObjectService_FWD_DEFINED__
#define __IDocObjectService_FWD_DEFINED__
typedef interface IDocObjectService IDocObjectService;

#endif 	/* __IDocObjectService_FWD_DEFINED__ */


/* header files for imported files */
#include "objidl.h"
#include "mshtml.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_docobjectservice_0000_0000 */
/* [local] */ 

//=--------------------------------------------------------------------------=
// DocObjectService.h
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


extern RPC_IF_HANDLE __MIDL_itf_docobjectservice_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_docobjectservice_0000_0000_v0_0_s_ifspec;

#ifndef __IDocObjectService_INTERFACE_DEFINED__
#define __IDocObjectService_INTERFACE_DEFINED__

/* interface IDocObjectService */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IDocObjectService;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("3050f801-98b5-11cf-bb82-00aa00bdce0b")
    IDocObjectService : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE FireBeforeNavigate2( 
            /* [in] */ __RPC__in_opt IDispatch *pDispatch,
            /* [in] */ __RPC__in LPCWSTR lpszUrl,
            /* [in] */ DWORD dwFlags,
            /* [in] */ __RPC__in LPCWSTR lpszFrameName,
            /* [in] */ __RPC__in BYTE *pPostData,
            /* [in] */ DWORD cbPostData,
            /* [in] */ __RPC__in LPCWSTR lpszHeaders,
            /* [in] */ BOOL fPlayNavSound,
            /* [out] */ __RPC__out BOOL *pfCancel) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE FireNavigateComplete2( 
            /* [in] */ __RPC__in_opt IHTMLWindow2 *pHTMLWindow2,
            /* [in] */ DWORD dwFlags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE FireDownloadBegin( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE FireDownloadComplete( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE FireDocumentComplete( 
            /* [in] */ __RPC__in_opt IHTMLWindow2 *pHTMLWindow,
            /* [in] */ DWORD dwFlags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE UpdateDesktopComponent( 
            /* [in] */ __RPC__in_opt IHTMLWindow2 *pHTMLWindow) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetPendingUrl( 
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrPendingUrl) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ActiveElementChanged( 
            __RPC__in_opt IHTMLElement *pHTMLElement) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetUrlSearchComponent( 
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrSearch) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE IsErrorUrl( 
            /* [in] */ __RPC__in LPCWSTR lpszUrl,
            /* [out] */ __RPC__out BOOL *pfIsError) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDocObjectServiceVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IDocObjectService * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IDocObjectService * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IDocObjectService * This);
        
        DECLSPEC_XFGVIRT(IDocObjectService, FireBeforeNavigate2)
        HRESULT ( STDMETHODCALLTYPE *FireBeforeNavigate2 )( 
            __RPC__in IDocObjectService * This,
            /* [in] */ __RPC__in_opt IDispatch *pDispatch,
            /* [in] */ __RPC__in LPCWSTR lpszUrl,
            /* [in] */ DWORD dwFlags,
            /* [in] */ __RPC__in LPCWSTR lpszFrameName,
            /* [in] */ __RPC__in BYTE *pPostData,
            /* [in] */ DWORD cbPostData,
            /* [in] */ __RPC__in LPCWSTR lpszHeaders,
            /* [in] */ BOOL fPlayNavSound,
            /* [out] */ __RPC__out BOOL *pfCancel);
        
        DECLSPEC_XFGVIRT(IDocObjectService, FireNavigateComplete2)
        HRESULT ( STDMETHODCALLTYPE *FireNavigateComplete2 )( 
            __RPC__in IDocObjectService * This,
            /* [in] */ __RPC__in_opt IHTMLWindow2 *pHTMLWindow2,
            /* [in] */ DWORD dwFlags);
        
        DECLSPEC_XFGVIRT(IDocObjectService, FireDownloadBegin)
        HRESULT ( STDMETHODCALLTYPE *FireDownloadBegin )( 
            __RPC__in IDocObjectService * This);
        
        DECLSPEC_XFGVIRT(IDocObjectService, FireDownloadComplete)
        HRESULT ( STDMETHODCALLTYPE *FireDownloadComplete )( 
            __RPC__in IDocObjectService * This);
        
        DECLSPEC_XFGVIRT(IDocObjectService, FireDocumentComplete)
        HRESULT ( STDMETHODCALLTYPE *FireDocumentComplete )( 
            __RPC__in IDocObjectService * This,
            /* [in] */ __RPC__in_opt IHTMLWindow2 *pHTMLWindow,
            /* [in] */ DWORD dwFlags);
        
        DECLSPEC_XFGVIRT(IDocObjectService, UpdateDesktopComponent)
        HRESULT ( STDMETHODCALLTYPE *UpdateDesktopComponent )( 
            __RPC__in IDocObjectService * This,
            /* [in] */ __RPC__in_opt IHTMLWindow2 *pHTMLWindow);
        
        DECLSPEC_XFGVIRT(IDocObjectService, GetPendingUrl)
        HRESULT ( STDMETHODCALLTYPE *GetPendingUrl )( 
            __RPC__in IDocObjectService * This,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrPendingUrl);
        
        DECLSPEC_XFGVIRT(IDocObjectService, ActiveElementChanged)
        HRESULT ( STDMETHODCALLTYPE *ActiveElementChanged )( 
            __RPC__in IDocObjectService * This,
            __RPC__in_opt IHTMLElement *pHTMLElement);
        
        DECLSPEC_XFGVIRT(IDocObjectService, GetUrlSearchComponent)
        HRESULT ( STDMETHODCALLTYPE *GetUrlSearchComponent )( 
            __RPC__in IDocObjectService * This,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrSearch);
        
        DECLSPEC_XFGVIRT(IDocObjectService, IsErrorUrl)
        HRESULT ( STDMETHODCALLTYPE *IsErrorUrl )( 
            __RPC__in IDocObjectService * This,
            /* [in] */ __RPC__in LPCWSTR lpszUrl,
            /* [out] */ __RPC__out BOOL *pfIsError);
        
        END_INTERFACE
    } IDocObjectServiceVtbl;

    interface IDocObjectService
    {
        CONST_VTBL struct IDocObjectServiceVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDocObjectService_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDocObjectService_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDocObjectService_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDocObjectService_FireBeforeNavigate2(This,pDispatch,lpszUrl,dwFlags,lpszFrameName,pPostData,cbPostData,lpszHeaders,fPlayNavSound,pfCancel)	\
    ( (This)->lpVtbl -> FireBeforeNavigate2(This,pDispatch,lpszUrl,dwFlags,lpszFrameName,pPostData,cbPostData,lpszHeaders,fPlayNavSound,pfCancel) ) 

#define IDocObjectService_FireNavigateComplete2(This,pHTMLWindow2,dwFlags)	\
    ( (This)->lpVtbl -> FireNavigateComplete2(This,pHTMLWindow2,dwFlags) ) 

#define IDocObjectService_FireDownloadBegin(This)	\
    ( (This)->lpVtbl -> FireDownloadBegin(This) ) 

#define IDocObjectService_FireDownloadComplete(This)	\
    ( (This)->lpVtbl -> FireDownloadComplete(This) ) 

#define IDocObjectService_FireDocumentComplete(This,pHTMLWindow,dwFlags)	\
    ( (This)->lpVtbl -> FireDocumentComplete(This,pHTMLWindow,dwFlags) ) 

#define IDocObjectService_UpdateDesktopComponent(This,pHTMLWindow)	\
    ( (This)->lpVtbl -> UpdateDesktopComponent(This,pHTMLWindow) ) 

#define IDocObjectService_GetPendingUrl(This,pbstrPendingUrl)	\
    ( (This)->lpVtbl -> GetPendingUrl(This,pbstrPendingUrl) ) 

#define IDocObjectService_ActiveElementChanged(This,pHTMLElement)	\
    ( (This)->lpVtbl -> ActiveElementChanged(This,pHTMLElement) ) 

#define IDocObjectService_GetUrlSearchComponent(This,pbstrSearch)	\
    ( (This)->lpVtbl -> GetUrlSearchComponent(This,pbstrSearch) ) 

#define IDocObjectService_IsErrorUrl(This,lpszUrl,pfIsError)	\
    ( (This)->lpVtbl -> IsErrorUrl(This,lpszUrl,pfIsError) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDocObjectService_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_docobjectservice_0000_0001 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_docobjectservice_0000_0001_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_docobjectservice_0000_0001_v0_0_s_ifspec;

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


