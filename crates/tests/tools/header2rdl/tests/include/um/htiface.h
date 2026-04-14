

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

#ifndef __htiface_h__
#define __htiface_h__

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

#ifndef __ITargetFrame_FWD_DEFINED__
#define __ITargetFrame_FWD_DEFINED__
typedef interface ITargetFrame ITargetFrame;

#endif 	/* __ITargetFrame_FWD_DEFINED__ */


#ifndef __ITargetEmbedding_FWD_DEFINED__
#define __ITargetEmbedding_FWD_DEFINED__
typedef interface ITargetEmbedding ITargetEmbedding;

#endif 	/* __ITargetEmbedding_FWD_DEFINED__ */


#ifndef __ITargetFramePriv_FWD_DEFINED__
#define __ITargetFramePriv_FWD_DEFINED__
typedef interface ITargetFramePriv ITargetFramePriv;

#endif 	/* __ITargetFramePriv_FWD_DEFINED__ */


#ifndef __ITargetFramePriv2_FWD_DEFINED__
#define __ITargetFramePriv2_FWD_DEFINED__
typedef interface ITargetFramePriv2 ITargetFramePriv2;

#endif 	/* __ITargetFramePriv2_FWD_DEFINED__ */


/* header files for imported files */
#include "objidl.h"
#include "oleidl.h"
#include "urlmon.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_htiface_0000_0000 */
/* [local] */ 

//=--------------------------------------------------------------------------=
// HTIface.h
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

//--------------------------------------------------------------------------
// OLE Hyperlinking ITargetFrame Interfaces.

#ifndef _LPTARGETFRAME2_DEFINED														
#include "htiframe.h"														
#endif // _LPTARGETFRAME2_DEFINED														


EXTERN_C const IID IID_ITargetFrame;
EXTERN_C const IID IID_ITargetEmbedding;
EXTERN_C const IID IID_ITargetFramePriv;
EXTERN_C const IID IID_ITargetFramePriv2;
#ifndef _LPTARGETFRAME_DEFINED
#define _LPTARGETFRAME_DEFINED


extern RPC_IF_HANDLE __MIDL_itf_htiface_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_htiface_0000_0000_v0_0_s_ifspec;

#ifndef __ITargetFrame_INTERFACE_DEFINED__
#define __ITargetFrame_INTERFACE_DEFINED__

/* interface ITargetFrame */
/* [unique][uuid][object] */ 

typedef /* [unique] */  __RPC_unique_pointer ITargetFrame *LPTARGETFRAME;

typedef /* [public] */ 
enum __MIDL_ITargetFrame_0001
    {
        NAVIGATEFRAME_FL_RECORD	= 0x1,
        NAVIGATEFRAME_FL_POST	= 0x2,
        NAVIGATEFRAME_FL_NO_DOC_CACHE	= 0x4,
        NAVIGATEFRAME_FL_NO_IMAGE_CACHE	= 0x8,
        NAVIGATEFRAME_FL_AUTH_FAIL_CACHE_OK	= 0x10,
        NAVIGATEFRAME_FL_SENDING_FROM_FORM	= 0x20,
        NAVIGATEFRAME_FL_REALLY_SENDING_FROM_FORM	= 0x40
    } 	NAVIGATEFRAME_FLAGS;

typedef struct tagNavigateData
    {
    ULONG ulTarget;
    ULONG ulURL;
    ULONG ulRefURL;
    ULONG ulPostData;
    DWORD dwFlags;
    } 	NAVIGATEDATA;


EXTERN_C const IID IID_ITargetFrame;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("d5f78c80-5252-11cf-90fa-00AA0042106e")
    ITargetFrame : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SetFrameName( 
            /* [in] */ __RPC__in LPCWSTR pszFrameName) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetFrameName( 
            /* [out] */ __RPC__deref_out_opt LPWSTR *ppszFrameName) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetParentFrame( 
            /* [out] */ __RPC__deref_out_opt IUnknown **ppunkParent) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE FindFrame( 
            /* [in] */ __RPC__in LPCWSTR pszTargetName,
            /* [in] */ __RPC__in_opt IUnknown *ppunkContextFrame,
            /* [in] */ DWORD dwFlags,
            /* [out] */ __RPC__deref_out_opt IUnknown **ppunkTargetFrame) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetFrameSrc( 
            /* [in] */ __RPC__in LPCWSTR pszFrameSrc) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetFrameSrc( 
            /* [out] */ __RPC__deref_out_opt LPWSTR *ppszFrameSrc) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetFramesContainer( 
            /* [out] */ __RPC__deref_out_opt IOleContainer **ppContainer) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetFrameOptions( 
            /* [in] */ DWORD dwFlags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetFrameOptions( 
            /* [out] */ __RPC__out DWORD *pdwFlags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetFrameMargins( 
            /* [in] */ DWORD dwWidth,
            /* [in] */ DWORD dwHeight) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetFrameMargins( 
            /* [out] */ __RPC__out DWORD *pdwWidth,
            /* [out] */ __RPC__out DWORD *pdwHeight) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RemoteNavigate( 
            /* [in] */ ULONG cLength,
            /* [size_is][in] */ __RPC__in_ecount_full(cLength) ULONG *pulData) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnChildFrameActivate( 
            /* [in] */ __RPC__in_opt IUnknown *pUnkChildFrame) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnChildFrameDeactivate( 
            /* [in] */ __RPC__in_opt IUnknown *pUnkChildFrame) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITargetFrameVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITargetFrame * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITargetFrame * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITargetFrame * This);
        
        DECLSPEC_XFGVIRT(ITargetFrame, SetFrameName)
        HRESULT ( STDMETHODCALLTYPE *SetFrameName )( 
            __RPC__in ITargetFrame * This,
            /* [in] */ __RPC__in LPCWSTR pszFrameName);
        
        DECLSPEC_XFGVIRT(ITargetFrame, GetFrameName)
        HRESULT ( STDMETHODCALLTYPE *GetFrameName )( 
            __RPC__in ITargetFrame * This,
            /* [out] */ __RPC__deref_out_opt LPWSTR *ppszFrameName);
        
        DECLSPEC_XFGVIRT(ITargetFrame, GetParentFrame)
        HRESULT ( STDMETHODCALLTYPE *GetParentFrame )( 
            __RPC__in ITargetFrame * This,
            /* [out] */ __RPC__deref_out_opt IUnknown **ppunkParent);
        
        DECLSPEC_XFGVIRT(ITargetFrame, FindFrame)
        HRESULT ( STDMETHODCALLTYPE *FindFrame )( 
            __RPC__in ITargetFrame * This,
            /* [in] */ __RPC__in LPCWSTR pszTargetName,
            /* [in] */ __RPC__in_opt IUnknown *ppunkContextFrame,
            /* [in] */ DWORD dwFlags,
            /* [out] */ __RPC__deref_out_opt IUnknown **ppunkTargetFrame);
        
        DECLSPEC_XFGVIRT(ITargetFrame, SetFrameSrc)
        HRESULT ( STDMETHODCALLTYPE *SetFrameSrc )( 
            __RPC__in ITargetFrame * This,
            /* [in] */ __RPC__in LPCWSTR pszFrameSrc);
        
        DECLSPEC_XFGVIRT(ITargetFrame, GetFrameSrc)
        HRESULT ( STDMETHODCALLTYPE *GetFrameSrc )( 
            __RPC__in ITargetFrame * This,
            /* [out] */ __RPC__deref_out_opt LPWSTR *ppszFrameSrc);
        
        DECLSPEC_XFGVIRT(ITargetFrame, GetFramesContainer)
        HRESULT ( STDMETHODCALLTYPE *GetFramesContainer )( 
            __RPC__in ITargetFrame * This,
            /* [out] */ __RPC__deref_out_opt IOleContainer **ppContainer);
        
        DECLSPEC_XFGVIRT(ITargetFrame, SetFrameOptions)
        HRESULT ( STDMETHODCALLTYPE *SetFrameOptions )( 
            __RPC__in ITargetFrame * This,
            /* [in] */ DWORD dwFlags);
        
        DECLSPEC_XFGVIRT(ITargetFrame, GetFrameOptions)
        HRESULT ( STDMETHODCALLTYPE *GetFrameOptions )( 
            __RPC__in ITargetFrame * This,
            /* [out] */ __RPC__out DWORD *pdwFlags);
        
        DECLSPEC_XFGVIRT(ITargetFrame, SetFrameMargins)
        HRESULT ( STDMETHODCALLTYPE *SetFrameMargins )( 
            __RPC__in ITargetFrame * This,
            /* [in] */ DWORD dwWidth,
            /* [in] */ DWORD dwHeight);
        
        DECLSPEC_XFGVIRT(ITargetFrame, GetFrameMargins)
        HRESULT ( STDMETHODCALLTYPE *GetFrameMargins )( 
            __RPC__in ITargetFrame * This,
            /* [out] */ __RPC__out DWORD *pdwWidth,
            /* [out] */ __RPC__out DWORD *pdwHeight);
        
        DECLSPEC_XFGVIRT(ITargetFrame, RemoteNavigate)
        HRESULT ( STDMETHODCALLTYPE *RemoteNavigate )( 
            __RPC__in ITargetFrame * This,
            /* [in] */ ULONG cLength,
            /* [size_is][in] */ __RPC__in_ecount_full(cLength) ULONG *pulData);
        
        DECLSPEC_XFGVIRT(ITargetFrame, OnChildFrameActivate)
        HRESULT ( STDMETHODCALLTYPE *OnChildFrameActivate )( 
            __RPC__in ITargetFrame * This,
            /* [in] */ __RPC__in_opt IUnknown *pUnkChildFrame);
        
        DECLSPEC_XFGVIRT(ITargetFrame, OnChildFrameDeactivate)
        HRESULT ( STDMETHODCALLTYPE *OnChildFrameDeactivate )( 
            __RPC__in ITargetFrame * This,
            /* [in] */ __RPC__in_opt IUnknown *pUnkChildFrame);
        
        END_INTERFACE
    } ITargetFrameVtbl;

    interface ITargetFrame
    {
        CONST_VTBL struct ITargetFrameVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITargetFrame_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITargetFrame_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITargetFrame_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITargetFrame_SetFrameName(This,pszFrameName)	\
    ( (This)->lpVtbl -> SetFrameName(This,pszFrameName) ) 

#define ITargetFrame_GetFrameName(This,ppszFrameName)	\
    ( (This)->lpVtbl -> GetFrameName(This,ppszFrameName) ) 

#define ITargetFrame_GetParentFrame(This,ppunkParent)	\
    ( (This)->lpVtbl -> GetParentFrame(This,ppunkParent) ) 

#define ITargetFrame_FindFrame(This,pszTargetName,ppunkContextFrame,dwFlags,ppunkTargetFrame)	\
    ( (This)->lpVtbl -> FindFrame(This,pszTargetName,ppunkContextFrame,dwFlags,ppunkTargetFrame) ) 

#define ITargetFrame_SetFrameSrc(This,pszFrameSrc)	\
    ( (This)->lpVtbl -> SetFrameSrc(This,pszFrameSrc) ) 

#define ITargetFrame_GetFrameSrc(This,ppszFrameSrc)	\
    ( (This)->lpVtbl -> GetFrameSrc(This,ppszFrameSrc) ) 

#define ITargetFrame_GetFramesContainer(This,ppContainer)	\
    ( (This)->lpVtbl -> GetFramesContainer(This,ppContainer) ) 

#define ITargetFrame_SetFrameOptions(This,dwFlags)	\
    ( (This)->lpVtbl -> SetFrameOptions(This,dwFlags) ) 

#define ITargetFrame_GetFrameOptions(This,pdwFlags)	\
    ( (This)->lpVtbl -> GetFrameOptions(This,pdwFlags) ) 

#define ITargetFrame_SetFrameMargins(This,dwWidth,dwHeight)	\
    ( (This)->lpVtbl -> SetFrameMargins(This,dwWidth,dwHeight) ) 

#define ITargetFrame_GetFrameMargins(This,pdwWidth,pdwHeight)	\
    ( (This)->lpVtbl -> GetFrameMargins(This,pdwWidth,pdwHeight) ) 

#define ITargetFrame_RemoteNavigate(This,cLength,pulData)	\
    ( (This)->lpVtbl -> RemoteNavigate(This,cLength,pulData) ) 

#define ITargetFrame_OnChildFrameActivate(This,pUnkChildFrame)	\
    ( (This)->lpVtbl -> OnChildFrameActivate(This,pUnkChildFrame) ) 

#define ITargetFrame_OnChildFrameDeactivate(This,pUnkChildFrame)	\
    ( (This)->lpVtbl -> OnChildFrameDeactivate(This,pUnkChildFrame) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITargetFrame_INTERFACE_DEFINED__ */


#ifndef __ITargetEmbedding_INTERFACE_DEFINED__
#define __ITargetEmbedding_INTERFACE_DEFINED__

/* interface ITargetEmbedding */
/* [unique][uuid][object] */ 

typedef /* [unique] */  __RPC_unique_pointer ITargetEmbedding *LPTARGETEMBEDDING;


EXTERN_C const IID IID_ITargetEmbedding;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("548793C0-9E74-11cf-9655-00A0C9034923")
    ITargetEmbedding : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetTargetFrame( 
            /* [out] */ __RPC__deref_out_opt ITargetFrame **ppTargetFrame) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITargetEmbeddingVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITargetEmbedding * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITargetEmbedding * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITargetEmbedding * This);
        
        DECLSPEC_XFGVIRT(ITargetEmbedding, GetTargetFrame)
        HRESULT ( STDMETHODCALLTYPE *GetTargetFrame )( 
            __RPC__in ITargetEmbedding * This,
            /* [out] */ __RPC__deref_out_opt ITargetFrame **ppTargetFrame);
        
        END_INTERFACE
    } ITargetEmbeddingVtbl;

    interface ITargetEmbedding
    {
        CONST_VTBL struct ITargetEmbeddingVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITargetEmbedding_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITargetEmbedding_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITargetEmbedding_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITargetEmbedding_GetTargetFrame(This,ppTargetFrame)	\
    ( (This)->lpVtbl -> GetTargetFrame(This,ppTargetFrame) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITargetEmbedding_INTERFACE_DEFINED__ */


#ifndef __ITargetFramePriv_INTERFACE_DEFINED__
#define __ITargetFramePriv_INTERFACE_DEFINED__

/* interface ITargetFramePriv */
/* [unique][uuid][object] */ 

typedef /* [unique] */  __RPC_unique_pointer ITargetFramePriv *LPTARGETFRAMEPRIV;


EXTERN_C const IID IID_ITargetFramePriv;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("9216E421-2BF5-11d0-82B4-00A0C90C29C5")
    ITargetFramePriv : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE FindFrameDownwards( 
            /* [in] */ __RPC__in LPCWSTR pszTargetName,
            /* [in] */ DWORD dwFlags,
            /* [out] */ __RPC__deref_out_opt IUnknown **ppunkTargetFrame) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE FindFrameInContext( 
            /* [in] */ __RPC__in LPCWSTR pszTargetName,
            /* [in] */ __RPC__in_opt IUnknown *punkContextFrame,
            /* [in] */ DWORD dwFlags,
            /* [out] */ __RPC__deref_out_opt IUnknown **ppunkTargetFrame) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnChildFrameActivate( 
            /* [in] */ __RPC__in_opt IUnknown *pUnkChildFrame) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnChildFrameDeactivate( 
            /* [in] */ __RPC__in_opt IUnknown *pUnkChildFrame) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE NavigateHack( 
            /* [in] */ DWORD grfHLNF,
            /* [unique][in] */ __RPC__in_opt LPBC pbc,
            /* [unique][in] */ __RPC__in_opt IBindStatusCallback *pibsc,
            /* [unique][in] */ __RPC__in_opt LPCWSTR pszTargetName,
            /* [in] */ __RPC__in LPCWSTR pszUrl,
            /* [unique][in] */ __RPC__in_opt LPCWSTR pszLocation) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE FindBrowserByIndex( 
            /* [in] */ DWORD dwID,
            /* [out] */ __RPC__deref_out_opt IUnknown **ppunkBrowser) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITargetFramePrivVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITargetFramePriv * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITargetFramePriv * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITargetFramePriv * This);
        
        DECLSPEC_XFGVIRT(ITargetFramePriv, FindFrameDownwards)
        HRESULT ( STDMETHODCALLTYPE *FindFrameDownwards )( 
            __RPC__in ITargetFramePriv * This,
            /* [in] */ __RPC__in LPCWSTR pszTargetName,
            /* [in] */ DWORD dwFlags,
            /* [out] */ __RPC__deref_out_opt IUnknown **ppunkTargetFrame);
        
        DECLSPEC_XFGVIRT(ITargetFramePriv, FindFrameInContext)
        HRESULT ( STDMETHODCALLTYPE *FindFrameInContext )( 
            __RPC__in ITargetFramePriv * This,
            /* [in] */ __RPC__in LPCWSTR pszTargetName,
            /* [in] */ __RPC__in_opt IUnknown *punkContextFrame,
            /* [in] */ DWORD dwFlags,
            /* [out] */ __RPC__deref_out_opt IUnknown **ppunkTargetFrame);
        
        DECLSPEC_XFGVIRT(ITargetFramePriv, OnChildFrameActivate)
        HRESULT ( STDMETHODCALLTYPE *OnChildFrameActivate )( 
            __RPC__in ITargetFramePriv * This,
            /* [in] */ __RPC__in_opt IUnknown *pUnkChildFrame);
        
        DECLSPEC_XFGVIRT(ITargetFramePriv, OnChildFrameDeactivate)
        HRESULT ( STDMETHODCALLTYPE *OnChildFrameDeactivate )( 
            __RPC__in ITargetFramePriv * This,
            /* [in] */ __RPC__in_opt IUnknown *pUnkChildFrame);
        
        DECLSPEC_XFGVIRT(ITargetFramePriv, NavigateHack)
        HRESULT ( STDMETHODCALLTYPE *NavigateHack )( 
            __RPC__in ITargetFramePriv * This,
            /* [in] */ DWORD grfHLNF,
            /* [unique][in] */ __RPC__in_opt LPBC pbc,
            /* [unique][in] */ __RPC__in_opt IBindStatusCallback *pibsc,
            /* [unique][in] */ __RPC__in_opt LPCWSTR pszTargetName,
            /* [in] */ __RPC__in LPCWSTR pszUrl,
            /* [unique][in] */ __RPC__in_opt LPCWSTR pszLocation);
        
        DECLSPEC_XFGVIRT(ITargetFramePriv, FindBrowserByIndex)
        HRESULT ( STDMETHODCALLTYPE *FindBrowserByIndex )( 
            __RPC__in ITargetFramePriv * This,
            /* [in] */ DWORD dwID,
            /* [out] */ __RPC__deref_out_opt IUnknown **ppunkBrowser);
        
        END_INTERFACE
    } ITargetFramePrivVtbl;

    interface ITargetFramePriv
    {
        CONST_VTBL struct ITargetFramePrivVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITargetFramePriv_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITargetFramePriv_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITargetFramePriv_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITargetFramePriv_FindFrameDownwards(This,pszTargetName,dwFlags,ppunkTargetFrame)	\
    ( (This)->lpVtbl -> FindFrameDownwards(This,pszTargetName,dwFlags,ppunkTargetFrame) ) 

#define ITargetFramePriv_FindFrameInContext(This,pszTargetName,punkContextFrame,dwFlags,ppunkTargetFrame)	\
    ( (This)->lpVtbl -> FindFrameInContext(This,pszTargetName,punkContextFrame,dwFlags,ppunkTargetFrame) ) 

#define ITargetFramePriv_OnChildFrameActivate(This,pUnkChildFrame)	\
    ( (This)->lpVtbl -> OnChildFrameActivate(This,pUnkChildFrame) ) 

#define ITargetFramePriv_OnChildFrameDeactivate(This,pUnkChildFrame)	\
    ( (This)->lpVtbl -> OnChildFrameDeactivate(This,pUnkChildFrame) ) 

#define ITargetFramePriv_NavigateHack(This,grfHLNF,pbc,pibsc,pszTargetName,pszUrl,pszLocation)	\
    ( (This)->lpVtbl -> NavigateHack(This,grfHLNF,pbc,pibsc,pszTargetName,pszUrl,pszLocation) ) 

#define ITargetFramePriv_FindBrowserByIndex(This,dwID,ppunkBrowser)	\
    ( (This)->lpVtbl -> FindBrowserByIndex(This,dwID,ppunkBrowser) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITargetFramePriv_INTERFACE_DEFINED__ */


#ifndef __ITargetFramePriv2_INTERFACE_DEFINED__
#define __ITargetFramePriv2_INTERFACE_DEFINED__

/* interface ITargetFramePriv2 */
/* [unique][uuid][object] */ 

typedef /* [unique] */  __RPC_unique_pointer ITargetFramePriv2 *LPTARGETFRAMEPRIV2;


EXTERN_C const IID IID_ITargetFramePriv2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("B2C867E6-69D6-46F2-A611-DED9A4BD7FEF")
    ITargetFramePriv2 : public ITargetFramePriv
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE AggregatedNavigation2( 
            /* [in] */ DWORD grfHLNF,
            /* [unique][in] */ __RPC__in_opt LPBC pbc,
            /* [unique][in] */ __RPC__in_opt IBindStatusCallback *pibsc,
            /* [unique][in] */ __RPC__in_opt LPCWSTR pszTargetName,
            /* [in] */ __RPC__in_opt IUri *pUri,
            /* [unique][in] */ __RPC__in_opt LPCWSTR pszLocation) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITargetFramePriv2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITargetFramePriv2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITargetFramePriv2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITargetFramePriv2 * This);
        
        DECLSPEC_XFGVIRT(ITargetFramePriv, FindFrameDownwards)
        HRESULT ( STDMETHODCALLTYPE *FindFrameDownwards )( 
            __RPC__in ITargetFramePriv2 * This,
            /* [in] */ __RPC__in LPCWSTR pszTargetName,
            /* [in] */ DWORD dwFlags,
            /* [out] */ __RPC__deref_out_opt IUnknown **ppunkTargetFrame);
        
        DECLSPEC_XFGVIRT(ITargetFramePriv, FindFrameInContext)
        HRESULT ( STDMETHODCALLTYPE *FindFrameInContext )( 
            __RPC__in ITargetFramePriv2 * This,
            /* [in] */ __RPC__in LPCWSTR pszTargetName,
            /* [in] */ __RPC__in_opt IUnknown *punkContextFrame,
            /* [in] */ DWORD dwFlags,
            /* [out] */ __RPC__deref_out_opt IUnknown **ppunkTargetFrame);
        
        DECLSPEC_XFGVIRT(ITargetFramePriv, OnChildFrameActivate)
        HRESULT ( STDMETHODCALLTYPE *OnChildFrameActivate )( 
            __RPC__in ITargetFramePriv2 * This,
            /* [in] */ __RPC__in_opt IUnknown *pUnkChildFrame);
        
        DECLSPEC_XFGVIRT(ITargetFramePriv, OnChildFrameDeactivate)
        HRESULT ( STDMETHODCALLTYPE *OnChildFrameDeactivate )( 
            __RPC__in ITargetFramePriv2 * This,
            /* [in] */ __RPC__in_opt IUnknown *pUnkChildFrame);
        
        DECLSPEC_XFGVIRT(ITargetFramePriv, NavigateHack)
        HRESULT ( STDMETHODCALLTYPE *NavigateHack )( 
            __RPC__in ITargetFramePriv2 * This,
            /* [in] */ DWORD grfHLNF,
            /* [unique][in] */ __RPC__in_opt LPBC pbc,
            /* [unique][in] */ __RPC__in_opt IBindStatusCallback *pibsc,
            /* [unique][in] */ __RPC__in_opt LPCWSTR pszTargetName,
            /* [in] */ __RPC__in LPCWSTR pszUrl,
            /* [unique][in] */ __RPC__in_opt LPCWSTR pszLocation);
        
        DECLSPEC_XFGVIRT(ITargetFramePriv, FindBrowserByIndex)
        HRESULT ( STDMETHODCALLTYPE *FindBrowserByIndex )( 
            __RPC__in ITargetFramePriv2 * This,
            /* [in] */ DWORD dwID,
            /* [out] */ __RPC__deref_out_opt IUnknown **ppunkBrowser);
        
        DECLSPEC_XFGVIRT(ITargetFramePriv2, AggregatedNavigation2)
        HRESULT ( STDMETHODCALLTYPE *AggregatedNavigation2 )( 
            __RPC__in ITargetFramePriv2 * This,
            /* [in] */ DWORD grfHLNF,
            /* [unique][in] */ __RPC__in_opt LPBC pbc,
            /* [unique][in] */ __RPC__in_opt IBindStatusCallback *pibsc,
            /* [unique][in] */ __RPC__in_opt LPCWSTR pszTargetName,
            /* [in] */ __RPC__in_opt IUri *pUri,
            /* [unique][in] */ __RPC__in_opt LPCWSTR pszLocation);
        
        END_INTERFACE
    } ITargetFramePriv2Vtbl;

    interface ITargetFramePriv2
    {
        CONST_VTBL struct ITargetFramePriv2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITargetFramePriv2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITargetFramePriv2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITargetFramePriv2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITargetFramePriv2_FindFrameDownwards(This,pszTargetName,dwFlags,ppunkTargetFrame)	\
    ( (This)->lpVtbl -> FindFrameDownwards(This,pszTargetName,dwFlags,ppunkTargetFrame) ) 

#define ITargetFramePriv2_FindFrameInContext(This,pszTargetName,punkContextFrame,dwFlags,ppunkTargetFrame)	\
    ( (This)->lpVtbl -> FindFrameInContext(This,pszTargetName,punkContextFrame,dwFlags,ppunkTargetFrame) ) 

#define ITargetFramePriv2_OnChildFrameActivate(This,pUnkChildFrame)	\
    ( (This)->lpVtbl -> OnChildFrameActivate(This,pUnkChildFrame) ) 

#define ITargetFramePriv2_OnChildFrameDeactivate(This,pUnkChildFrame)	\
    ( (This)->lpVtbl -> OnChildFrameDeactivate(This,pUnkChildFrame) ) 

#define ITargetFramePriv2_NavigateHack(This,grfHLNF,pbc,pibsc,pszTargetName,pszUrl,pszLocation)	\
    ( (This)->lpVtbl -> NavigateHack(This,grfHLNF,pbc,pibsc,pszTargetName,pszUrl,pszLocation) ) 

#define ITargetFramePriv2_FindBrowserByIndex(This,dwID,ppunkBrowser)	\
    ( (This)->lpVtbl -> FindBrowserByIndex(This,dwID,ppunkBrowser) ) 


#define ITargetFramePriv2_AggregatedNavigation2(This,grfHLNF,pbc,pibsc,pszTargetName,pUri,pszLocation)	\
    ( (This)->lpVtbl -> AggregatedNavigation2(This,grfHLNF,pbc,pibsc,pszTargetName,pUri,pszLocation) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITargetFramePriv2_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_htiface_0000_0004 */
/* [local] */ 

#endif
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_htiface_0000_0004_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_htiface_0000_0004_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


