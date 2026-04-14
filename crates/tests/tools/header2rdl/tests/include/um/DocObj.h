

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

#ifndef __docobj_h__
#define __docobj_h__

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

#ifndef __IOleDocument_FWD_DEFINED__
#define __IOleDocument_FWD_DEFINED__
typedef interface IOleDocument IOleDocument;

#endif 	/* __IOleDocument_FWD_DEFINED__ */


#ifndef __IOleDocumentSite_FWD_DEFINED__
#define __IOleDocumentSite_FWD_DEFINED__
typedef interface IOleDocumentSite IOleDocumentSite;

#endif 	/* __IOleDocumentSite_FWD_DEFINED__ */


#ifndef __IOleDocumentView_FWD_DEFINED__
#define __IOleDocumentView_FWD_DEFINED__
typedef interface IOleDocumentView IOleDocumentView;

#endif 	/* __IOleDocumentView_FWD_DEFINED__ */


#ifndef __IEnumOleDocumentViews_FWD_DEFINED__
#define __IEnumOleDocumentViews_FWD_DEFINED__
typedef interface IEnumOleDocumentViews IEnumOleDocumentViews;

#endif 	/* __IEnumOleDocumentViews_FWD_DEFINED__ */


#ifndef __IContinueCallback_FWD_DEFINED__
#define __IContinueCallback_FWD_DEFINED__
typedef interface IContinueCallback IContinueCallback;

#endif 	/* __IContinueCallback_FWD_DEFINED__ */


#ifndef __IPrint_FWD_DEFINED__
#define __IPrint_FWD_DEFINED__
typedef interface IPrint IPrint;

#endif 	/* __IPrint_FWD_DEFINED__ */


#ifndef __IOleCommandTarget_FWD_DEFINED__
#define __IOleCommandTarget_FWD_DEFINED__
typedef interface IOleCommandTarget IOleCommandTarget;

#endif 	/* __IOleCommandTarget_FWD_DEFINED__ */


#ifndef __IZoomEvents_FWD_DEFINED__
#define __IZoomEvents_FWD_DEFINED__
typedef interface IZoomEvents IZoomEvents;

#endif 	/* __IZoomEvents_FWD_DEFINED__ */


#ifndef __IProtectFocus_FWD_DEFINED__
#define __IProtectFocus_FWD_DEFINED__
typedef interface IProtectFocus IProtectFocus;

#endif 	/* __IProtectFocus_FWD_DEFINED__ */


#ifndef __IProtectedModeMenuServices_FWD_DEFINED__
#define __IProtectedModeMenuServices_FWD_DEFINED__
typedef interface IProtectedModeMenuServices IProtectedModeMenuServices;

#endif 	/* __IProtectedModeMenuServices_FWD_DEFINED__ */


/* header files for imported files */
#include "ocidl.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_docobj_0000_0000 */
/* [local] */ 

//=--------------------------------------------------------------------------=
// DocObj.h
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
// OLE Document Object Interfaces.









////////////////////////////////////////////////////////////////////////////
//  Interface Definitions
#ifndef _LPOLEDOCUMENT_DEFINED
#define _LPOLEDOCUMENT_DEFINED


extern RPC_IF_HANDLE __MIDL_itf_docobj_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_docobj_0000_0000_v0_0_s_ifspec;

#ifndef __IOleDocument_INTERFACE_DEFINED__
#define __IOleDocument_INTERFACE_DEFINED__

/* interface IOleDocument */
/* [unique][uuid][object] */ 

typedef /* [unique] */  __RPC_unique_pointer IOleDocument *LPOLEDOCUMENT;

typedef /* [public] */ 
enum __MIDL_IOleDocument_0001
    {
        DOCMISC_CANCREATEMULTIPLEVIEWS	= 1,
        DOCMISC_SUPPORTCOMPLEXRECTANGLES	= 2,
        DOCMISC_CANTOPENEDIT	= 4,
        DOCMISC_NOFILESUPPORT	= 8
    } 	DOCMISC;


EXTERN_C const IID IID_IOleDocument;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("b722bcc5-4e68-101b-a2bc-00aa00404770")
    IOleDocument : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE CreateView( 
            /* [unique][in] */ __RPC__in_opt IOleInPlaceSite *pIPSite,
            /* [unique][in] */ __RPC__in_opt IStream *pstm,
            /* [in] */ DWORD dwReserved,
            /* [out] */ __RPC__deref_out_opt IOleDocumentView **ppView) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetDocMiscStatus( 
            /* [out] */ __RPC__out DWORD *pdwStatus) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EnumViews( 
            /* [out] */ __RPC__deref_out_opt IEnumOleDocumentViews **ppEnum,
            /* [out] */ __RPC__deref_out_opt IOleDocumentView **ppView) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IOleDocumentVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IOleDocument * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IOleDocument * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IOleDocument * This);
        
        DECLSPEC_XFGVIRT(IOleDocument, CreateView)
        HRESULT ( STDMETHODCALLTYPE *CreateView )( 
            __RPC__in IOleDocument * This,
            /* [unique][in] */ __RPC__in_opt IOleInPlaceSite *pIPSite,
            /* [unique][in] */ __RPC__in_opt IStream *pstm,
            /* [in] */ DWORD dwReserved,
            /* [out] */ __RPC__deref_out_opt IOleDocumentView **ppView);
        
        DECLSPEC_XFGVIRT(IOleDocument, GetDocMiscStatus)
        HRESULT ( STDMETHODCALLTYPE *GetDocMiscStatus )( 
            __RPC__in IOleDocument * This,
            /* [out] */ __RPC__out DWORD *pdwStatus);
        
        DECLSPEC_XFGVIRT(IOleDocument, EnumViews)
        HRESULT ( STDMETHODCALLTYPE *EnumViews )( 
            __RPC__in IOleDocument * This,
            /* [out] */ __RPC__deref_out_opt IEnumOleDocumentViews **ppEnum,
            /* [out] */ __RPC__deref_out_opt IOleDocumentView **ppView);
        
        END_INTERFACE
    } IOleDocumentVtbl;

    interface IOleDocument
    {
        CONST_VTBL struct IOleDocumentVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IOleDocument_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IOleDocument_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IOleDocument_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IOleDocument_CreateView(This,pIPSite,pstm,dwReserved,ppView)	\
    ( (This)->lpVtbl -> CreateView(This,pIPSite,pstm,dwReserved,ppView) ) 

#define IOleDocument_GetDocMiscStatus(This,pdwStatus)	\
    ( (This)->lpVtbl -> GetDocMiscStatus(This,pdwStatus) ) 

#define IOleDocument_EnumViews(This,ppEnum,ppView)	\
    ( (This)->lpVtbl -> EnumViews(This,ppEnum,ppView) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IOleDocument_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_docobj_0000_0001 */
/* [local] */ 

#endif
#ifndef _LPOLEDOCUMENTSITE_DEFINED
#define _LPOLEDOCUMENTSITE_DEFINED


extern RPC_IF_HANDLE __MIDL_itf_docobj_0000_0001_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_docobj_0000_0001_v0_0_s_ifspec;

#ifndef __IOleDocumentSite_INTERFACE_DEFINED__
#define __IOleDocumentSite_INTERFACE_DEFINED__

/* interface IOleDocumentSite */
/* [unique][uuid][object] */ 

typedef /* [unique] */  __RPC_unique_pointer IOleDocumentSite *LPOLEDOCUMENTSITE;


EXTERN_C const IID IID_IOleDocumentSite;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("b722bcc7-4e68-101b-a2bc-00aa00404770")
    IOleDocumentSite : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE ActivateMe( 
            /* [in] */ __RPC__in_opt IOleDocumentView *pViewToActivate) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IOleDocumentSiteVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IOleDocumentSite * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IOleDocumentSite * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IOleDocumentSite * This);
        
        DECLSPEC_XFGVIRT(IOleDocumentSite, ActivateMe)
        HRESULT ( STDMETHODCALLTYPE *ActivateMe )( 
            __RPC__in IOleDocumentSite * This,
            /* [in] */ __RPC__in_opt IOleDocumentView *pViewToActivate);
        
        END_INTERFACE
    } IOleDocumentSiteVtbl;

    interface IOleDocumentSite
    {
        CONST_VTBL struct IOleDocumentSiteVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IOleDocumentSite_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IOleDocumentSite_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IOleDocumentSite_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IOleDocumentSite_ActivateMe(This,pViewToActivate)	\
    ( (This)->lpVtbl -> ActivateMe(This,pViewToActivate) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IOleDocumentSite_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_docobj_0000_0002 */
/* [local] */ 

#endif
#ifndef _LPOLEDOCUMENTVIEW_DEFINED
#define _LPOLEDOCUMENTVIEW_DEFINED


extern RPC_IF_HANDLE __MIDL_itf_docobj_0000_0002_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_docobj_0000_0002_v0_0_s_ifspec;

#ifndef __IOleDocumentView_INTERFACE_DEFINED__
#define __IOleDocumentView_INTERFACE_DEFINED__

/* interface IOleDocumentView */
/* [unique][uuid][object] */ 

typedef /* [unique] */  __RPC_unique_pointer IOleDocumentView *LPOLEDOCUMENTVIEW;


EXTERN_C const IID IID_IOleDocumentView;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("b722bcc6-4e68-101b-a2bc-00aa00404770")
    IOleDocumentView : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SetInPlaceSite( 
            /* [unique][in] */ __RPC__in_opt IOleInPlaceSite *pIPSite) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetInPlaceSite( 
            /* [out] */ __RPC__deref_out_opt IOleInPlaceSite **ppIPSite) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetDocument( 
            /* [out] */ __RPC__deref_out_opt IUnknown **ppunk) = 0;
        
        virtual /* [input_sync] */ HRESULT STDMETHODCALLTYPE SetRect( 
            /* [in] */ __RPC__in LPRECT prcView) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRect( 
            /* [out] */ __RPC__out LPRECT prcView) = 0;
        
        virtual /* [input_sync] */ HRESULT STDMETHODCALLTYPE SetRectComplex( 
            /* [unique][in] */ __RPC__in_opt LPRECT prcView,
            /* [unique][in] */ __RPC__in_opt LPRECT prcHScroll,
            /* [unique][in] */ __RPC__in_opt LPRECT prcVScroll,
            /* [unique][in] */ __RPC__in_opt LPRECT prcSizeBox) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Show( 
            /* [in] */ BOOL fShow) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE UIActivate( 
            /* [in] */ BOOL fUIActivate) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Open( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CloseView( 
            /* [in] */ DWORD dwReserved) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SaveViewState( 
            /* [in] */ __RPC__in_opt LPSTREAM pstm) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ApplyViewState( 
            /* [in] */ __RPC__in_opt LPSTREAM pstm) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Clone( 
            /* [in] */ __RPC__in_opt IOleInPlaceSite *pIPSiteNew,
            /* [out] */ __RPC__deref_out_opt IOleDocumentView **ppViewNew) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IOleDocumentViewVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IOleDocumentView * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IOleDocumentView * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IOleDocumentView * This);
        
        DECLSPEC_XFGVIRT(IOleDocumentView, SetInPlaceSite)
        HRESULT ( STDMETHODCALLTYPE *SetInPlaceSite )( 
            __RPC__in IOleDocumentView * This,
            /* [unique][in] */ __RPC__in_opt IOleInPlaceSite *pIPSite);
        
        DECLSPEC_XFGVIRT(IOleDocumentView, GetInPlaceSite)
        HRESULT ( STDMETHODCALLTYPE *GetInPlaceSite )( 
            __RPC__in IOleDocumentView * This,
            /* [out] */ __RPC__deref_out_opt IOleInPlaceSite **ppIPSite);
        
        DECLSPEC_XFGVIRT(IOleDocumentView, GetDocument)
        HRESULT ( STDMETHODCALLTYPE *GetDocument )( 
            __RPC__in IOleDocumentView * This,
            /* [out] */ __RPC__deref_out_opt IUnknown **ppunk);
        
        DECLSPEC_XFGVIRT(IOleDocumentView, SetRect)
        /* [input_sync] */ HRESULT ( STDMETHODCALLTYPE *SetRect )( 
            __RPC__in IOleDocumentView * This,
            /* [in] */ __RPC__in LPRECT prcView);
        
        DECLSPEC_XFGVIRT(IOleDocumentView, GetRect)
        HRESULT ( STDMETHODCALLTYPE *GetRect )( 
            __RPC__in IOleDocumentView * This,
            /* [out] */ __RPC__out LPRECT prcView);
        
        DECLSPEC_XFGVIRT(IOleDocumentView, SetRectComplex)
        /* [input_sync] */ HRESULT ( STDMETHODCALLTYPE *SetRectComplex )( 
            __RPC__in IOleDocumentView * This,
            /* [unique][in] */ __RPC__in_opt LPRECT prcView,
            /* [unique][in] */ __RPC__in_opt LPRECT prcHScroll,
            /* [unique][in] */ __RPC__in_opt LPRECT prcVScroll,
            /* [unique][in] */ __RPC__in_opt LPRECT prcSizeBox);
        
        DECLSPEC_XFGVIRT(IOleDocumentView, Show)
        HRESULT ( STDMETHODCALLTYPE *Show )( 
            __RPC__in IOleDocumentView * This,
            /* [in] */ BOOL fShow);
        
        DECLSPEC_XFGVIRT(IOleDocumentView, UIActivate)
        HRESULT ( STDMETHODCALLTYPE *UIActivate )( 
            __RPC__in IOleDocumentView * This,
            /* [in] */ BOOL fUIActivate);
        
        DECLSPEC_XFGVIRT(IOleDocumentView, Open)
        HRESULT ( STDMETHODCALLTYPE *Open )( 
            __RPC__in IOleDocumentView * This);
        
        DECLSPEC_XFGVIRT(IOleDocumentView, CloseView)
        HRESULT ( STDMETHODCALLTYPE *CloseView )( 
            __RPC__in IOleDocumentView * This,
            /* [in] */ DWORD dwReserved);
        
        DECLSPEC_XFGVIRT(IOleDocumentView, SaveViewState)
        HRESULT ( STDMETHODCALLTYPE *SaveViewState )( 
            __RPC__in IOleDocumentView * This,
            /* [in] */ __RPC__in_opt LPSTREAM pstm);
        
        DECLSPEC_XFGVIRT(IOleDocumentView, ApplyViewState)
        HRESULT ( STDMETHODCALLTYPE *ApplyViewState )( 
            __RPC__in IOleDocumentView * This,
            /* [in] */ __RPC__in_opt LPSTREAM pstm);
        
        DECLSPEC_XFGVIRT(IOleDocumentView, Clone)
        HRESULT ( STDMETHODCALLTYPE *Clone )( 
            __RPC__in IOleDocumentView * This,
            /* [in] */ __RPC__in_opt IOleInPlaceSite *pIPSiteNew,
            /* [out] */ __RPC__deref_out_opt IOleDocumentView **ppViewNew);
        
        END_INTERFACE
    } IOleDocumentViewVtbl;

    interface IOleDocumentView
    {
        CONST_VTBL struct IOleDocumentViewVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IOleDocumentView_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IOleDocumentView_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IOleDocumentView_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IOleDocumentView_SetInPlaceSite(This,pIPSite)	\
    ( (This)->lpVtbl -> SetInPlaceSite(This,pIPSite) ) 

#define IOleDocumentView_GetInPlaceSite(This,ppIPSite)	\
    ( (This)->lpVtbl -> GetInPlaceSite(This,ppIPSite) ) 

#define IOleDocumentView_GetDocument(This,ppunk)	\
    ( (This)->lpVtbl -> GetDocument(This,ppunk) ) 

#define IOleDocumentView_SetRect(This,prcView)	\
    ( (This)->lpVtbl -> SetRect(This,prcView) ) 

#define IOleDocumentView_GetRect(This,prcView)	\
    ( (This)->lpVtbl -> GetRect(This,prcView) ) 

#define IOleDocumentView_SetRectComplex(This,prcView,prcHScroll,prcVScroll,prcSizeBox)	\
    ( (This)->lpVtbl -> SetRectComplex(This,prcView,prcHScroll,prcVScroll,prcSizeBox) ) 

#define IOleDocumentView_Show(This,fShow)	\
    ( (This)->lpVtbl -> Show(This,fShow) ) 

#define IOleDocumentView_UIActivate(This,fUIActivate)	\
    ( (This)->lpVtbl -> UIActivate(This,fUIActivate) ) 

#define IOleDocumentView_Open(This)	\
    ( (This)->lpVtbl -> Open(This) ) 

#define IOleDocumentView_CloseView(This,dwReserved)	\
    ( (This)->lpVtbl -> CloseView(This,dwReserved) ) 

#define IOleDocumentView_SaveViewState(This,pstm)	\
    ( (This)->lpVtbl -> SaveViewState(This,pstm) ) 

#define IOleDocumentView_ApplyViewState(This,pstm)	\
    ( (This)->lpVtbl -> ApplyViewState(This,pstm) ) 

#define IOleDocumentView_Clone(This,pIPSiteNew,ppViewNew)	\
    ( (This)->lpVtbl -> Clone(This,pIPSiteNew,ppViewNew) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IOleDocumentView_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_docobj_0000_0003 */
/* [local] */ 

#endif
#ifndef _LPENUMOLEDOCUMENTVIEWS_DEFINED
#define _LPENUMOLEDOCUMENTVIEWS_DEFINED


extern RPC_IF_HANDLE __MIDL_itf_docobj_0000_0003_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_docobj_0000_0003_v0_0_s_ifspec;

#ifndef __IEnumOleDocumentViews_INTERFACE_DEFINED__
#define __IEnumOleDocumentViews_INTERFACE_DEFINED__

/* interface IEnumOleDocumentViews */
/* [unique][uuid][object] */ 

typedef /* [unique] */  __RPC_unique_pointer IEnumOleDocumentViews *LPENUMOLEDOCUMENTVIEWS;


EXTERN_C const IID IID_IEnumOleDocumentViews;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("b722bcc8-4e68-101b-a2bc-00aa00404770")
    IEnumOleDocumentViews : public IUnknown
    {
    public:
        virtual /* [local] */ HRESULT __stdcall Next( 
            /* [in] */ ULONG cViews,
            /* [out] */ IOleDocumentView **rgpView,
            /* [out] */ ULONG *pcFetched) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Skip( 
            /* [in] */ ULONG cViews) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Reset( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Clone( 
            /* [out] */ __RPC__deref_out_opt IEnumOleDocumentViews **ppEnum) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IEnumOleDocumentViewsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IEnumOleDocumentViews * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IEnumOleDocumentViews * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IEnumOleDocumentViews * This);
        
        DECLSPEC_XFGVIRT(IEnumOleDocumentViews, Next)
        /* [local] */ HRESULT ( __stdcall *Next )( 
            IEnumOleDocumentViews * This,
            /* [in] */ ULONG cViews,
            /* [out] */ IOleDocumentView **rgpView,
            /* [out] */ ULONG *pcFetched);
        
        DECLSPEC_XFGVIRT(IEnumOleDocumentViews, Skip)
        HRESULT ( STDMETHODCALLTYPE *Skip )( 
            __RPC__in IEnumOleDocumentViews * This,
            /* [in] */ ULONG cViews);
        
        DECLSPEC_XFGVIRT(IEnumOleDocumentViews, Reset)
        HRESULT ( STDMETHODCALLTYPE *Reset )( 
            __RPC__in IEnumOleDocumentViews * This);
        
        DECLSPEC_XFGVIRT(IEnumOleDocumentViews, Clone)
        HRESULT ( STDMETHODCALLTYPE *Clone )( 
            __RPC__in IEnumOleDocumentViews * This,
            /* [out] */ __RPC__deref_out_opt IEnumOleDocumentViews **ppEnum);
        
        END_INTERFACE
    } IEnumOleDocumentViewsVtbl;

    interface IEnumOleDocumentViews
    {
        CONST_VTBL struct IEnumOleDocumentViewsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IEnumOleDocumentViews_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IEnumOleDocumentViews_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IEnumOleDocumentViews_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IEnumOleDocumentViews_Next(This,cViews,rgpView,pcFetched)	\
    ( (This)->lpVtbl -> Next(This,cViews,rgpView,pcFetched) ) 

#define IEnumOleDocumentViews_Skip(This,cViews)	\
    ( (This)->lpVtbl -> Skip(This,cViews) ) 

#define IEnumOleDocumentViews_Reset(This)	\
    ( (This)->lpVtbl -> Reset(This) ) 

#define IEnumOleDocumentViews_Clone(This,ppEnum)	\
    ( (This)->lpVtbl -> Clone(This,ppEnum) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */



/* [call_as] */ HRESULT __stdcall IEnumOleDocumentViews_RemoteNext_Proxy( 
    __RPC__in IEnumOleDocumentViews * This,
    /* [in] */ ULONG cViews,
    /* [length_is][size_is][out] */ __RPC__out_ecount_part(cViews, *pcFetched) IOleDocumentView **rgpView,
    /* [out] */ __RPC__out ULONG *pcFetched);


void __RPC_STUB IEnumOleDocumentViews_RemoteNext_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);



#endif 	/* __IEnumOleDocumentViews_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_docobj_0000_0004 */
/* [local] */ 

#endif
#ifndef _LPCONTINUECALLBACK_DEFINED
#define _LPCONTINUECALLBACK_DEFINED


extern RPC_IF_HANDLE __MIDL_itf_docobj_0000_0004_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_docobj_0000_0004_v0_0_s_ifspec;

#ifndef __IContinueCallback_INTERFACE_DEFINED__
#define __IContinueCallback_INTERFACE_DEFINED__

/* interface IContinueCallback */
/* [unique][uuid][object] */ 

typedef /* [unique] */  __RPC_unique_pointer IContinueCallback *LPCONTINUECALLBACK;


EXTERN_C const IID IID_IContinueCallback;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("b722bcca-4e68-101b-a2bc-00aa00404770")
    IContinueCallback : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE FContinue( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE FContinuePrinting( 
            /* [in] */ LONG nCntPrinted,
            /* [in] */ LONG nCurPage,
            /* [unique][in] */ __RPC__in_opt wchar_t *pwszPrintStatus) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IContinueCallbackVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IContinueCallback * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IContinueCallback * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IContinueCallback * This);
        
        DECLSPEC_XFGVIRT(IContinueCallback, FContinue)
        HRESULT ( STDMETHODCALLTYPE *FContinue )( 
            __RPC__in IContinueCallback * This);
        
        DECLSPEC_XFGVIRT(IContinueCallback, FContinuePrinting)
        HRESULT ( STDMETHODCALLTYPE *FContinuePrinting )( 
            __RPC__in IContinueCallback * This,
            /* [in] */ LONG nCntPrinted,
            /* [in] */ LONG nCurPage,
            /* [unique][in] */ __RPC__in_opt wchar_t *pwszPrintStatus);
        
        END_INTERFACE
    } IContinueCallbackVtbl;

    interface IContinueCallback
    {
        CONST_VTBL struct IContinueCallbackVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IContinueCallback_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IContinueCallback_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IContinueCallback_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IContinueCallback_FContinue(This)	\
    ( (This)->lpVtbl -> FContinue(This) ) 

#define IContinueCallback_FContinuePrinting(This,nCntPrinted,nCurPage,pwszPrintStatus)	\
    ( (This)->lpVtbl -> FContinuePrinting(This,nCntPrinted,nCurPage,pwszPrintStatus) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IContinueCallback_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_docobj_0000_0005 */
/* [local] */ 

#endif
#ifndef _LPPRINT_DEFINED
#define _LPPRINT_DEFINED


extern RPC_IF_HANDLE __MIDL_itf_docobj_0000_0005_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_docobj_0000_0005_v0_0_s_ifspec;

#ifndef __IPrint_INTERFACE_DEFINED__
#define __IPrint_INTERFACE_DEFINED__

/* interface IPrint */
/* [unique][uuid][object] */ 

typedef /* [unique] */  __RPC_unique_pointer IPrint *LPPRINT;

typedef /* [public] */ 
enum __MIDL_IPrint_0001
    {
        PRINTFLAG_MAYBOTHERUSER	= 1,
        PRINTFLAG_PROMPTUSER	= 2,
        PRINTFLAG_USERMAYCHANGEPRINTER	= 4,
        PRINTFLAG_RECOMPOSETODEVICE	= 8,
        PRINTFLAG_DONTACTUALLYPRINT	= 16,
        PRINTFLAG_FORCEPROPERTIES	= 32,
        PRINTFLAG_PRINTTOFILE	= 64
    } 	PRINTFLAG;

typedef struct tagPAGERANGE
    {
    LONG nFromPage;
    LONG nToPage;
    } 	PAGERANGE;

typedef struct tagPAGESET
    {
    ULONG cbStruct;
    BOOL fOddPages;
    BOOL fEvenPages;
    ULONG cPageRange;
    /* [size_is] */ PAGERANGE rgPages[ 1 ];
    } 	PAGESET;

#define PAGESET_TOLASTPAGE   ((WORD)(-1L))

EXTERN_C const IID IID_IPrint;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("b722bcc9-4e68-101b-a2bc-00aa00404770")
    IPrint : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SetInitialPageNum( 
            /* [in] */ LONG nFirstPage) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetPageInfo( 
            /* [out] */ __RPC__out LONG *pnFirstPage,
            /* [out] */ __RPC__out LONG *pcPages) = 0;
        
        virtual /* [local] */ HRESULT __stdcall Print( 
            /* [in] */ DWORD grfFlags,
            /* [out][in] */ DVTARGETDEVICE **pptd,
            /* [out][in] */ PAGESET **ppPageSet,
            /* [unique][out][in] */ STGMEDIUM *pstgmOptions,
            /* [in] */ IContinueCallback *pcallback,
            /* [in] */ LONG nFirstPage,
            /* [out] */ LONG *pcPagesPrinted,
            /* [out] */ LONG *pnLastPage) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IPrintVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IPrint * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IPrint * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IPrint * This);
        
        DECLSPEC_XFGVIRT(IPrint, SetInitialPageNum)
        HRESULT ( STDMETHODCALLTYPE *SetInitialPageNum )( 
            __RPC__in IPrint * This,
            /* [in] */ LONG nFirstPage);
        
        DECLSPEC_XFGVIRT(IPrint, GetPageInfo)
        HRESULT ( STDMETHODCALLTYPE *GetPageInfo )( 
            __RPC__in IPrint * This,
            /* [out] */ __RPC__out LONG *pnFirstPage,
            /* [out] */ __RPC__out LONG *pcPages);
        
        DECLSPEC_XFGVIRT(IPrint, Print)
        /* [local] */ HRESULT ( __stdcall *Print )( 
            IPrint * This,
            /* [in] */ DWORD grfFlags,
            /* [out][in] */ DVTARGETDEVICE **pptd,
            /* [out][in] */ PAGESET **ppPageSet,
            /* [unique][out][in] */ STGMEDIUM *pstgmOptions,
            /* [in] */ IContinueCallback *pcallback,
            /* [in] */ LONG nFirstPage,
            /* [out] */ LONG *pcPagesPrinted,
            /* [out] */ LONG *pnLastPage);
        
        END_INTERFACE
    } IPrintVtbl;

    interface IPrint
    {
        CONST_VTBL struct IPrintVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IPrint_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IPrint_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IPrint_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IPrint_SetInitialPageNum(This,nFirstPage)	\
    ( (This)->lpVtbl -> SetInitialPageNum(This,nFirstPage) ) 

#define IPrint_GetPageInfo(This,pnFirstPage,pcPages)	\
    ( (This)->lpVtbl -> GetPageInfo(This,pnFirstPage,pcPages) ) 

#define IPrint_Print(This,grfFlags,pptd,ppPageSet,pstgmOptions,pcallback,nFirstPage,pcPagesPrinted,pnLastPage)	\
    ( (This)->lpVtbl -> Print(This,grfFlags,pptd,ppPageSet,pstgmOptions,pcallback,nFirstPage,pcPagesPrinted,pnLastPage) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */



/* [call_as] */ HRESULT __stdcall IPrint_RemotePrint_Proxy( 
    __RPC__in IPrint * This,
    /* [in] */ DWORD grfFlags,
    /* [out][in] */ __RPC__deref_inout_opt DVTARGETDEVICE **pptd,
    /* [out][in] */ __RPC__deref_inout_opt PAGESET **pppageset,
    /* [unique][out][in] */ __RPC__inout_opt RemSTGMEDIUM *pstgmOptions,
    /* [in] */ __RPC__in_opt IContinueCallback *pcallback,
    /* [in] */ LONG nFirstPage,
    /* [out] */ __RPC__out LONG *pcPagesPrinted,
    /* [out] */ __RPC__out LONG *pnLastPage);


void __RPC_STUB IPrint_RemotePrint_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);



#endif 	/* __IPrint_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_docobj_0000_0006 */
/* [local] */ 

#endif
#ifndef _LPOLECOMMANDTARGET_DEFINED
#define _LPOLECOMMANDTARGET_DEFINED


extern RPC_IF_HANDLE __MIDL_itf_docobj_0000_0006_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_docobj_0000_0006_v0_0_s_ifspec;

#ifndef __IOleCommandTarget_INTERFACE_DEFINED__
#define __IOleCommandTarget_INTERFACE_DEFINED__

/* interface IOleCommandTarget */
/* [unique][uuid][object] */ 

typedef /* [unique] */  __RPC_unique_pointer IOleCommandTarget *LPOLECOMMANDTARGET;

typedef 
enum OLECMDF
    {
        OLECMDF_SUPPORTED	= 0x1,
        OLECMDF_ENABLED	= 0x2,
        OLECMDF_LATCHED	= 0x4,
        OLECMDF_NINCHED	= 0x8,
        OLECMDF_INVISIBLE	= 0x10,
        OLECMDF_DEFHIDEONCTXTMENU	= 0x20
    } 	OLECMDF;

typedef struct _tagOLECMD
    {
    ULONG cmdID;
    DWORD cmdf;
    } 	OLECMD;

typedef struct _tagOLECMDTEXT
    {
    DWORD cmdtextf;
    ULONG cwActual;
    ULONG cwBuf;
    /* [size_is] */ wchar_t rgwz[ 1 ];
    } 	OLECMDTEXT;

typedef 
enum OLECMDTEXTF
    {
        OLECMDTEXTF_NONE	= 0,
        OLECMDTEXTF_NAME	= 1,
        OLECMDTEXTF_STATUS	= 2
    } 	OLECMDTEXTF;

typedef 
enum OLECMDEXECOPT
    {
        OLECMDEXECOPT_DODEFAULT	= 0,
        OLECMDEXECOPT_PROMPTUSER	= 1,
        OLECMDEXECOPT_DONTPROMPTUSER	= 2,
        OLECMDEXECOPT_SHOWHELP	= 3
    } 	OLECMDEXECOPT;

/* OLECMDID_STOPDOWNLOAD and OLECMDID_ALLOWUILESSSAVEAS are supported for QueryStatus Only */
typedef 
enum OLECMDID
    {
        OLECMDID_OPEN	= 1,
        OLECMDID_NEW	= 2,
        OLECMDID_SAVE	= 3,
        OLECMDID_SAVEAS	= 4,
        OLECMDID_SAVECOPYAS	= 5,
        OLECMDID_PRINT	= 6,
        OLECMDID_PRINTPREVIEW	= 7,
        OLECMDID_PAGESETUP	= 8,
        OLECMDID_SPELL	= 9,
        OLECMDID_PROPERTIES	= 10,
        OLECMDID_CUT	= 11,
        OLECMDID_COPY	= 12,
        OLECMDID_PASTE	= 13,
        OLECMDID_PASTESPECIAL	= 14,
        OLECMDID_UNDO	= 15,
        OLECMDID_REDO	= 16,
        OLECMDID_SELECTALL	= 17,
        OLECMDID_CLEARSELECTION	= 18,
        OLECMDID_ZOOM	= 19,
        OLECMDID_GETZOOMRANGE	= 20,
        OLECMDID_UPDATECOMMANDS	= 21,
        OLECMDID_REFRESH	= 22,
        OLECMDID_STOP	= 23,
        OLECMDID_HIDETOOLBARS	= 24,
        OLECMDID_SETPROGRESSMAX	= 25,
        OLECMDID_SETPROGRESSPOS	= 26,
        OLECMDID_SETPROGRESSTEXT	= 27,
        OLECMDID_SETTITLE	= 28,
        OLECMDID_SETDOWNLOADSTATE	= 29,
        OLECMDID_STOPDOWNLOAD	= 30,
        OLECMDID_ONTOOLBARACTIVATED	= 31,
        OLECMDID_FIND	= 32,
        OLECMDID_DELETE	= 33,
        OLECMDID_HTTPEQUIV	= 34,
        OLECMDID_HTTPEQUIV_DONE	= 35,
        OLECMDID_ENABLE_INTERACTION	= 36,
        OLECMDID_ONUNLOAD	= 37,
        OLECMDID_PROPERTYBAG2	= 38,
        OLECMDID_PREREFRESH	= 39,
        OLECMDID_SHOWSCRIPTERROR	= 40,
        OLECMDID_SHOWMESSAGE	= 41,
        OLECMDID_SHOWFIND	= 42,
        OLECMDID_SHOWPAGESETUP	= 43,
        OLECMDID_SHOWPRINT	= 44,
        OLECMDID_CLOSE	= 45,
        OLECMDID_ALLOWUILESSSAVEAS	= 46,
        OLECMDID_DONTDOWNLOADCSS	= 47,
        OLECMDID_UPDATEPAGESTATUS	= 48,
        OLECMDID_PRINT2	= 49,
        OLECMDID_PRINTPREVIEW2	= 50,
        OLECMDID_SETPRINTTEMPLATE	= 51,
        OLECMDID_GETPRINTTEMPLATE	= 52,
        OLECMDID_PAGEACTIONBLOCKED	= 55,
        OLECMDID_PAGEACTIONUIQUERY	= 56,
        OLECMDID_FOCUSVIEWCONTROLS	= 57,
        OLECMDID_FOCUSVIEWCONTROLSQUERY	= 58,
        OLECMDID_SHOWPAGEACTIONMENU	= 59,
        OLECMDID_ADDTRAVELENTRY	= 60,
        OLECMDID_UPDATETRAVELENTRY	= 61,
        OLECMDID_UPDATEBACKFORWARDSTATE	= 62,
        OLECMDID_OPTICAL_ZOOM	= 63,
        OLECMDID_OPTICAL_GETZOOMRANGE	= 64,
        OLECMDID_WINDOWSTATECHANGED	= 65,
        OLECMDID_ACTIVEXINSTALLSCOPE	= 66,
        OLECMDID_UPDATETRAVELENTRY_DATARECOVERY	= 67,
        OLECMDID_SHOWTASKDLG	= 68,
        OLECMDID_POPSTATEEVENT	= 69,
        OLECMDID_VIEWPORT_MODE	= 70,
        OLECMDID_LAYOUT_VIEWPORT_WIDTH	= 71,
        OLECMDID_VISUAL_VIEWPORT_EXCLUDE_BOTTOM	= 72,
        OLECMDID_USER_OPTICAL_ZOOM	= 73,
        OLECMDID_PAGEAVAILABLE	= 74,
        OLECMDID_GETUSERSCALABLE	= 75,
        OLECMDID_UPDATE_CARET	= 76,
        OLECMDID_ENABLE_VISIBILITY	= 77,
        OLECMDID_MEDIA_PLAYBACK	= 78,
        OLECMDID_SETFAVICON	= 79,
        OLECMDID_SET_HOST_FULLSCREENMODE	= 80,
        OLECMDID_EXITFULLSCREEN	= 81,
        OLECMDID_SCROLLCOMPLETE	= 82,
        OLECMDID_ONBEFOREUNLOAD	= 83,
        OLECMDID_SHOWMESSAGE_BLOCKABLE	= 84,
        OLECMDID_SHOWTASKDLG_BLOCKABLE	= 85
    } 	OLECMDID;

typedef 
enum MEDIAPLAYBACK_STATE
    {
        MEDIAPLAYBACK_RESUME	= 0,
        MEDIAPLAYBACK_PAUSE	= 1,
        MEDIAPLAYBACK_PAUSE_AND_SUSPEND	= 2,
        MEDIAPLAYBACK_RESUME_FROM_SUSPEND	= 3
    } 	MEDIAPLAYBACK_STATE;

#define OLECMDERR_E_FIRST            (OLE_E_LAST+1)
#define OLECMDERR_E_NOTSUPPORTED     (OLECMDERR_E_FIRST)
#define OLECMDERR_E_DISABLED         (OLECMDERR_E_FIRST+1)
#define OLECMDERR_E_NOHELP           (OLECMDERR_E_FIRST+2)
#define OLECMDERR_E_CANCELED         (OLECMDERR_E_FIRST+3)
#define OLECMDERR_E_UNKNOWNGROUP     (OLECMDERR_E_FIRST+4)
#define MSOCMDERR_E_FIRST            OLECMDERR_E_FIRST
#define MSOCMDERR_E_NOTSUPPORTED     OLECMDERR_E_NOTSUPPORTED
#define MSOCMDERR_E_DISABLED         OLECMDERR_E_DISABLED
#define MSOCMDERR_E_NOHELP           OLECMDERR_E_NOHELP
#define MSOCMDERR_E_CANCELED         OLECMDERR_E_CANCELED
#define MSOCMDERR_E_UNKNOWNGROUP     OLECMDERR_E_UNKNOWNGROUP
#define OLECMD_TASKDLGID_ONBEFOREUNLOAD            1
#if(NTDDI_VERSION >= NTDDI_WINXPSP2)
#define OLECMDARGINDEX_SHOWPAGEACTIONMENU_HWND     0
#define OLECMDARGINDEX_SHOWPAGEACTIONMENU_X        1
#define OLECMDARGINDEX_SHOWPAGEACTIONMENU_Y        2
#define OLECMDARGINDEX_ACTIVEXINSTALL_PUBLISHER    0
#define OLECMDARGINDEX_ACTIVEXINSTALL_DISPLAYNAME  1
#define OLECMDARGINDEX_ACTIVEXINSTALL_CLSID        2
#define OLECMDARGINDEX_ACTIVEXINSTALL_INSTALLSCOPE 3
#define OLECMDARGINDEX_ACTIVEXINSTALL_SOURCEURL    4
#define INSTALL_SCOPE_INVALID    0
#define INSTALL_SCOPE_MACHINE    1
#define INSTALL_SCOPE_USER       2
typedef 
enum IGNOREMIME
    {
        IGNOREMIME_PROMPT	= 0x1,
        IGNOREMIME_TEXT	= 0x2
    } 	IGNOREMIME;

typedef 
enum WPCSETTING
    {
        WPCSETTING_LOGGING_ENABLED	= 0x1,
        WPCSETTING_FILEDOWNLOAD_BLOCKED	= 0x2
    } 	WPCSETTING;

#endif

EXTERN_C const IID IID_IOleCommandTarget;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("b722bccb-4e68-101b-a2bc-00aa00404770")
    IOleCommandTarget : public IUnknown
    {
    public:
        virtual /* [input_sync] */ HRESULT STDMETHODCALLTYPE QueryStatus( 
            /* [unique][in] */ __RPC__in_opt const GUID *pguidCmdGroup,
            /* [in] */ ULONG cCmds,
            /* [out][in][size_is] */ __RPC__inout_ecount_full(cCmds) OLECMD prgCmds[  ],
            /* [unique][out][in] */ __RPC__inout_opt OLECMDTEXT *pCmdText) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Exec( 
            /* [unique][in] */ __RPC__in_opt const GUID *pguidCmdGroup,
            /* [in] */ DWORD nCmdID,
            /* [in] */ DWORD nCmdexecopt,
            /* [unique][in] */ __RPC__in_opt VARIANT *pvaIn,
            /* [unique][out][in] */ __RPC__inout_opt VARIANT *pvaOut) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IOleCommandTargetVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IOleCommandTarget * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IOleCommandTarget * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IOleCommandTarget * This);
        
        DECLSPEC_XFGVIRT(IOleCommandTarget, QueryStatus)
        /* [input_sync] */ HRESULT ( STDMETHODCALLTYPE *QueryStatus )( 
            __RPC__in IOleCommandTarget * This,
            /* [unique][in] */ __RPC__in_opt const GUID *pguidCmdGroup,
            /* [in] */ ULONG cCmds,
            /* [out][in][size_is] */ __RPC__inout_ecount_full(cCmds) OLECMD prgCmds[  ],
            /* [unique][out][in] */ __RPC__inout_opt OLECMDTEXT *pCmdText);
        
        DECLSPEC_XFGVIRT(IOleCommandTarget, Exec)
        HRESULT ( STDMETHODCALLTYPE *Exec )( 
            __RPC__in IOleCommandTarget * This,
            /* [unique][in] */ __RPC__in_opt const GUID *pguidCmdGroup,
            /* [in] */ DWORD nCmdID,
            /* [in] */ DWORD nCmdexecopt,
            /* [unique][in] */ __RPC__in_opt VARIANT *pvaIn,
            /* [unique][out][in] */ __RPC__inout_opt VARIANT *pvaOut);
        
        END_INTERFACE
    } IOleCommandTargetVtbl;

    interface IOleCommandTarget
    {
        CONST_VTBL struct IOleCommandTargetVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IOleCommandTarget_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IOleCommandTarget_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IOleCommandTarget_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IOleCommandTarget_QueryStatus(This,pguidCmdGroup,cCmds,prgCmds,pCmdText)	\
    ( (This)->lpVtbl -> QueryStatus(This,pguidCmdGroup,cCmds,prgCmds,pCmdText) ) 

#define IOleCommandTarget_Exec(This,pguidCmdGroup,nCmdID,nCmdexecopt,pvaIn,pvaOut)	\
    ( (This)->lpVtbl -> Exec(This,pguidCmdGroup,nCmdID,nCmdexecopt,pvaIn,pvaOut) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IOleCommandTarget_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_docobj_0000_0007 */
/* [local] */ 

#endif
typedef enum
{
      OLECMDIDF_REFRESH_NORMAL          = 0,
      OLECMDIDF_REFRESH_IFEXPIRED       = 1,
      OLECMDIDF_REFRESH_CONTINUE        = 2,
      OLECMDIDF_REFRESH_COMPLETELY      = 3,
      OLECMDIDF_REFRESH_NO_CACHE        = 4,
      OLECMDIDF_REFRESH_RELOAD          = 5,
      OLECMDIDF_REFRESH_LEVELMASK       = 0x00FF,
      OLECMDIDF_REFRESH_CLEARUSERINPUT  = 0x1000,
#if(NTDDI_VERSION >= NTDDI_WINXPSP2)
      OLECMDIDF_REFRESH_PROMPTIFOFFLINE = 0x2000,
      OLECMDIDF_REFRESH_THROUGHSCRIPT   = 0x4000,
      OLECMDIDF_REFRESH_SKIPBEFOREUNLOADEVENT     = 0x8000,
      OLECMDIDF_REFRESH_PAGEACTION_ACTIVEXINSTALL = 0x00010000,
      OLECMDIDF_REFRESH_PAGEACTION_FILEDOWNLOAD   = 0x00020000,
      OLECMDIDF_REFRESH_PAGEACTION_LOCALMACHINE   = 0x00040000,
      OLECMDIDF_REFRESH_PAGEACTION_POPUPWINDOW    = 0x00080000,
      OLECMDIDF_REFRESH_PAGEACTION_PROTLOCKDOWNLOCALMACHINE  = 0x00100000,
      OLECMDIDF_REFRESH_PAGEACTION_PROTLOCKDOWNTRUSTED       = 0x00200000,
      OLECMDIDF_REFRESH_PAGEACTION_PROTLOCKDOWNINTRANET      = 0x00400000,
      OLECMDIDF_REFRESH_PAGEACTION_PROTLOCKDOWNINTERNET      = 0x00800000,
      OLECMDIDF_REFRESH_PAGEACTION_PROTLOCKDOWNRESTRICTED    = 0x01000000,
#elif(NTDDI_VERSION >= NTDDI_WINXP)
      OLECMDIDF_REFRESH_PROMPTIFOFFLINE = 0x2000,
      OLECMDIDF_REFRESH_THROUGHSCRIPT   = 0x4000,
#else
      OLECMDIDF_REFRESH_PROMPTIFOFFLINE = 0x2000,
#endif
      OLECMDIDF_REFRESH_PAGEACTION_MIXEDCONTENT              = 0x02000000,
      OLECMDIDF_REFRESH_PAGEACTION_INVALID_CERT              = 0x04000000,
      OLECMDIDF_REFRESH_PAGEACTION_ALLOW_VERSION             = 0x08000000,
} OLECMDID_REFRESHFLAG;
#if(NTDDI_VERSION >= NTDDI_WINXPSP2)
typedef enum
{
      OLECMDIDF_PAGEACTION_FILEDOWNLOAD                       = 0x00000001,
      OLECMDIDF_PAGEACTION_ACTIVEXINSTALL                     = 0x00000002,
      OLECMDIDF_PAGEACTION_ACTIVEXTRUSTFAIL                   = 0x00000004,
      OLECMDIDF_PAGEACTION_ACTIVEXUSERDISABLE                 = 0x00000008,
      OLECMDIDF_PAGEACTION_ACTIVEXDISALLOW                    = 0x00000010,
      OLECMDIDF_PAGEACTION_ACTIVEXUNSAFE                      = 0x00000020,
      OLECMDIDF_PAGEACTION_POPUPWINDOW                        = 0x00000040,
      OLECMDIDF_PAGEACTION_LOCALMACHINE                       = 0x00000080,
      OLECMDIDF_PAGEACTION_MIMETEXTPLAIN                      = 0x00000100,
      OLECMDIDF_PAGEACTION_SCRIPTNAVIGATE                     = 0x00000200,
      OLECMDIDF_PAGEACTION_SCRIPTNAVIGATE_ACTIVEXINSTALL      = 0x00000200,
      OLECMDIDF_PAGEACTION_PROTLOCKDOWNLOCALMACHINE           = 0x00000400,
      OLECMDIDF_PAGEACTION_PROTLOCKDOWNTRUSTED                = 0x00000800,
      OLECMDIDF_PAGEACTION_PROTLOCKDOWNINTRANET               = 0x00001000,
      OLECMDIDF_PAGEACTION_PROTLOCKDOWNINTERNET               = 0x00002000,
      OLECMDIDF_PAGEACTION_PROTLOCKDOWNRESTRICTED             = 0x00004000,
      OLECMDIDF_PAGEACTION_PROTLOCKDOWNDENY                   = 0x00008000,
      OLECMDIDF_PAGEACTION_POPUPALLOWED                       = 0x00010000,
      OLECMDIDF_PAGEACTION_SCRIPTPROMPT                       = 0x00020000,
      OLECMDIDF_PAGEACTION_ACTIVEXUSERAPPROVAL                = 0x00040000,
      OLECMDIDF_PAGEACTION_MIXEDCONTENT                       = 0x00080000,
      OLECMDIDF_PAGEACTION_INVALID_CERT                       = 0x00100000,
      OLECMDIDF_PAGEACTION_INTRANETZONEREQUEST                = 0x00200000,
      OLECMDIDF_PAGEACTION_XSSFILTERED                        = 0x00400000,
      OLECMDIDF_PAGEACTION_SPOOFABLEIDNHOST                   = 0x00800000,
      OLECMDIDF_PAGEACTION_ACTIVEX_EPM_INCOMPATIBLE           = 0x01000000,
      OLECMDIDF_PAGEACTION_SCRIPTNAVIGATE_ACTIVEXUSERAPPROVAL = 0x02000000,
      OLECMDIDF_PAGEACTION_WPCBLOCKED                         = 0x04000000,
      OLECMDIDF_PAGEACTION_WPCBLOCKED_ACTIVEX                 = 0x08000000,
      OLECMDIDF_PAGEACTION_EXTENSION_COMPAT_BLOCKED           = 0x10000000,
      OLECMDIDF_PAGEACTION_NORESETACTIVEX                     = 0x20000000,
      OLECMDIDF_PAGEACTION_GENERIC_STATE                      = 0x40000000,
      OLECMDIDF_PAGEACTION_RESET                              = (int) 0x80000000,
} OLECMDID_PAGEACTIONFLAG;
typedef enum
{
      OLECMDIDF_BROWSERSTATE_EXTENSIONSOFF                      = 0x00000001,
      OLECMDIDF_BROWSERSTATE_IESECURITY                         = 0x00000002,
      OLECMDIDF_BROWSERSTATE_PROTECTEDMODE_OFF                  = 0x00000004,
      OLECMDIDF_BROWSERSTATE_RESET                              = 0x00000008,
      OLECMDIDF_BROWSERSTATE_REQUIRESACTIVEX                    = 0x00000010,
      OLECMDIDF_BROWSERSTATE_DESKTOPHTMLDIALOG                  = 0x00000020,
      OLECMDIDF_BROWSERSTATE_BLOCKEDVERSION                     = 0x00000040,
} OLECMDID_BROWSERSTATEFLAG;
typedef enum
{
      OLECMDIDF_OPTICAL_ZOOM_NOPERSIST                          = 0x00000001,
      OLECMDIDF_OPTICAL_ZOOM_NOLAYOUT                           = 0x00000010,
      OLECMDIDF_OPTICAL_ZOOM_NOTRANSIENT                        = 0x00000020,
      OLECMDIDF_OPTICAL_ZOOM_RELOADFORNEWTAB                    = 0x00000040,
} OLECMDID_OPTICAL_ZOOMFLAG;
typedef enum
{
    PAGEACTION_UI_DEFAULT     = 0,
    PAGEACTION_UI_MODAL       = 1,
    PAGEACTION_UI_MODELESS    = 2,
    PAGEACTION_UI_SILENT      = 3,
} PAGEACTION_UI;
#endif
typedef enum
{
    OLECMDIDF_WINDOWSTATE_USERVISIBLE        = 0x00000001,
    OLECMDIDF_WINDOWSTATE_ENABLED            = 0x00000002,
    OLECMDIDF_WINDOWSTATE_USERVISIBLE_VALID  = 0x00010000,
    OLECMDIDF_WINDOWSTATE_ENABLED_VALID      = 0x00020000,
} OLECMDID_WINDOWSTATE_FLAG;
typedef enum
{
    OLECMDIDF_VIEWPORTMODE_FIXED_LAYOUT_WIDTH          = 0x00000001,
    OLECMDIDF_VIEWPORTMODE_EXCLUDE_VISUAL_BOTTOM       = 0x00000002,
    OLECMDIDF_VIEWPORTMODE_FIXED_LAYOUT_WIDTH_VALID    = 0x00010000,
    OLECMDIDF_VIEWPORTMODE_EXCLUDE_VISUAL_BOTTOM_VALID = 0x00020000,
} OLECMDID_VIEWPORT_MODE_FLAG;

////////////////////////////////////////////////////////////////////////////
//  Aliases to original office-compatible names
#define IMsoDocument             IOleDocument
#define IMsoDocumentSite         IOleDocumentSite
#define IMsoView                 IOleDocumentView
#define IEnumMsoView             IEnumOleDocumentViews
#define IMsoCommandTarget        IOleCommandTarget
#define LPMSODOCUMENT            LPOLEDOCUMENT
#define LPMSODOCUMENTSITE        LPOLEDOCUMENTSITE
#define LPMSOVIEW                LPOLEDOCUMENTVIEW
#define LPENUMMSOVIEW            LPENUMOLEDOCUMENTVIEWS
#define LPMSOCOMMANDTARGET       LPOLECOMMANDTARGET
#define MSOCMD                   OLECMD
#define MSOCMDTEXT               OLECMDTEXT
#define IID_IMsoDocument         IID_IOleDocument
#define IID_IMsoDocumentSite     IID_IOleDocumentSite
#define IID_IMsoView             IID_IOleDocumentView
#define IID_IEnumMsoView         IID_IEnumOleDocumentViews
#define IID_IMsoCommandTarget    IID_IOleCommandTarget
#define MSOCMDF_SUPPORTED OLECMDF_SUPPORTED
#define MSOCMDF_ENABLED OLECMDF_ENABLED
#define MSOCMDF_LATCHED OLECMDF_LATCHED
#define MSOCMDF_NINCHED OLECMDF_NINCHED
#define MSOCMDTEXTF_NONE OLECMDTEXTF_NONE
#define MSOCMDTEXTF_NAME OLECMDTEXTF_NAME
#define MSOCMDTEXTF_STATUS OLECMDTEXTF_STATUS
#define MSOCMDEXECOPT_DODEFAULT OLECMDEXECOPT_DODEFAULT
#define MSOCMDEXECOPT_PROMPTUSER OLECMDEXECOPT_PROMPTUSER
#define MSOCMDEXECOPT_DONTPROMPTUSER OLECMDEXECOPT_DONTPROMPTUSER
#define MSOCMDEXECOPT_SHOWHELP OLECMDEXECOPT_SHOWHELP
#define MSOCMDID_OPEN OLECMDID_OPEN
#define MSOCMDID_NEW OLECMDID_NEW
#define MSOCMDID_SAVE OLECMDID_SAVE
#define MSOCMDID_SAVEAS OLECMDID_SAVEAS
#define MSOCMDID_SAVECOPYAS OLECMDID_SAVECOPYAS
#define MSOCMDID_PRINT OLECMDID_PRINT
#define MSOCMDID_PRINTPREVIEW OLECMDID_PRINTPREVIEW
#define MSOCMDID_PAGESETUP OLECMDID_PAGESETUP
#define MSOCMDID_SPELL OLECMDID_SPELL
#define MSOCMDID_PROPERTIES OLECMDID_PROPERTIES
#define MSOCMDID_CUT OLECMDID_CUT
#define MSOCMDID_COPY OLECMDID_COPY
#define MSOCMDID_PASTE OLECMDID_PASTE
#define MSOCMDID_PASTESPECIAL OLECMDID_PASTESPECIAL
#define MSOCMDID_UNDO OLECMDID_UNDO
#define MSOCMDID_REDO OLECMDID_REDO
#define MSOCMDID_SELECTALL OLECMDID_SELECTALL
#define MSOCMDID_CLEARSELECTION OLECMDID_CLEARSELECTION
#define MSOCMDID_ZOOM OLECMDID_ZOOM
#define MSOCMDID_GETZOOMRANGE OLECMDID_GETZOOMRANGE
EXTERN_C const GUID SID_SContainerDispatch;


extern RPC_IF_HANDLE __MIDL_itf_docobj_0000_0007_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_docobj_0000_0007_v0_0_s_ifspec;

#ifndef __IZoomEvents_INTERFACE_DEFINED__
#define __IZoomEvents_INTERFACE_DEFINED__

/* interface IZoomEvents */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IZoomEvents;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("41B68150-904C-4e17-A0BA-A438182E359D")
    IZoomEvents : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE OnZoomPercentChanged( 
            /* [in] */ ULONG ulZoomPercent) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IZoomEventsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IZoomEvents * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IZoomEvents * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IZoomEvents * This);
        
        DECLSPEC_XFGVIRT(IZoomEvents, OnZoomPercentChanged)
        HRESULT ( STDMETHODCALLTYPE *OnZoomPercentChanged )( 
            __RPC__in IZoomEvents * This,
            /* [in] */ ULONG ulZoomPercent);
        
        END_INTERFACE
    } IZoomEventsVtbl;

    interface IZoomEvents
    {
        CONST_VTBL struct IZoomEventsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IZoomEvents_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IZoomEvents_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IZoomEvents_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IZoomEvents_OnZoomPercentChanged(This,ulZoomPercent)	\
    ( (This)->lpVtbl -> OnZoomPercentChanged(This,ulZoomPercent) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IZoomEvents_INTERFACE_DEFINED__ */


#ifndef __IProtectFocus_INTERFACE_DEFINED__
#define __IProtectFocus_INTERFACE_DEFINED__

/* interface IProtectFocus */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IProtectFocus;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("d81f90a3-8156-44f7-ad28-5abb87003274")
    IProtectFocus : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE AllowFocusChange( 
            /* [out] */ __RPC__out BOOL *pfAllow) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IProtectFocusVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IProtectFocus * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IProtectFocus * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IProtectFocus * This);
        
        DECLSPEC_XFGVIRT(IProtectFocus, AllowFocusChange)
        HRESULT ( STDMETHODCALLTYPE *AllowFocusChange )( 
            __RPC__in IProtectFocus * This,
            /* [out] */ __RPC__out BOOL *pfAllow);
        
        END_INTERFACE
    } IProtectFocusVtbl;

    interface IProtectFocus
    {
        CONST_VTBL struct IProtectFocusVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IProtectFocus_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IProtectFocus_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IProtectFocus_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IProtectFocus_AllowFocusChange(This,pfAllow)	\
    ( (This)->lpVtbl -> AllowFocusChange(This,pfAllow) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IProtectFocus_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_docobj_0000_0009 */
/* [local] */ 

#define SID_SProtectFocus  IID_IProtectFocus
#ifndef _LPPROTECTEDMODEMENUSERVICES_DEFINED
#define _LPPROTECTEDMODEMENUSERVICES_DEFINED


extern RPC_IF_HANDLE __MIDL_itf_docobj_0000_0009_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_docobj_0000_0009_v0_0_s_ifspec;

#ifndef __IProtectedModeMenuServices_INTERFACE_DEFINED__
#define __IProtectedModeMenuServices_INTERFACE_DEFINED__

/* interface IProtectedModeMenuServices */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IProtectedModeMenuServices;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("73c105ee-9dff-4a07-b83c-7eff290c266e")
    IProtectedModeMenuServices : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE CreateMenu( 
            /* [out] */ __RPC__deref_out_opt HMENU *phMenu) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE LoadMenu( 
            /* [string][in] */ __RPC__in_string LPCWSTR pszModuleName,
            /* [string][in] */ __RPC__in_string LPCWSTR pszMenuName,
            /* [out] */ __RPC__deref_out_opt HMENU *phMenu) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE LoadMenuID( 
            /* [string][in] */ __RPC__in_string LPCWSTR pszModuleName,
            /* [in] */ WORD wResourceID,
            /* [out] */ __RPC__deref_out_opt HMENU *phMenu) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IProtectedModeMenuServicesVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IProtectedModeMenuServices * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IProtectedModeMenuServices * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IProtectedModeMenuServices * This);
        
        DECLSPEC_XFGVIRT(IProtectedModeMenuServices, CreateMenu)
        HRESULT ( STDMETHODCALLTYPE *CreateMenu )( 
            __RPC__in IProtectedModeMenuServices * This,
            /* [out] */ __RPC__deref_out_opt HMENU *phMenu);
        
        DECLSPEC_XFGVIRT(IProtectedModeMenuServices, LoadMenu)
        HRESULT ( STDMETHODCALLTYPE *LoadMenu )( 
            __RPC__in IProtectedModeMenuServices * This,
            /* [string][in] */ __RPC__in_string LPCWSTR pszModuleName,
            /* [string][in] */ __RPC__in_string LPCWSTR pszMenuName,
            /* [out] */ __RPC__deref_out_opt HMENU *phMenu);
        
        DECLSPEC_XFGVIRT(IProtectedModeMenuServices, LoadMenuID)
        HRESULT ( STDMETHODCALLTYPE *LoadMenuID )( 
            __RPC__in IProtectedModeMenuServices * This,
            /* [string][in] */ __RPC__in_string LPCWSTR pszModuleName,
            /* [in] */ WORD wResourceID,
            /* [out] */ __RPC__deref_out_opt HMENU *phMenu);
        
        END_INTERFACE
    } IProtectedModeMenuServicesVtbl;

    interface IProtectedModeMenuServices
    {
        CONST_VTBL struct IProtectedModeMenuServicesVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IProtectedModeMenuServices_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IProtectedModeMenuServices_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IProtectedModeMenuServices_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IProtectedModeMenuServices_CreateMenu(This,phMenu)	\
    ( (This)->lpVtbl -> CreateMenu(This,phMenu) ) 

#define IProtectedModeMenuServices_LoadMenu(This,pszModuleName,pszMenuName,phMenu)	\
    ( (This)->lpVtbl -> LoadMenu(This,pszModuleName,pszMenuName,phMenu) ) 

#define IProtectedModeMenuServices_LoadMenuID(This,pszModuleName,wResourceID,phMenu)	\
    ( (This)->lpVtbl -> LoadMenuID(This,pszModuleName,wResourceID,phMenu) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IProtectedModeMenuServices_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_docobj_0000_0010 */
/* [local] */ 

#endif
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP) && !WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
typedef struct tagPAGESET {} PAGESET;
#endif // WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP) && !WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


extern RPC_IF_HANDLE __MIDL_itf_docobj_0000_0010_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_docobj_0000_0010_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

unsigned long             __RPC_USER  HMENU_UserSize(     __RPC__in unsigned long *, unsigned long            , __RPC__in HMENU * ); 
unsigned char * __RPC_USER  HMENU_UserMarshal(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in HMENU * ); 
unsigned char * __RPC_USER  HMENU_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out HMENU * ); 
void                      __RPC_USER  HMENU_UserFree(     __RPC__in unsigned long *, __RPC__in HMENU * ); 

unsigned long             __RPC_USER  VARIANT_UserSize(     __RPC__in unsigned long *, unsigned long            , __RPC__in VARIANT * ); 
unsigned char * __RPC_USER  VARIANT_UserMarshal(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in VARIANT * ); 
unsigned char * __RPC_USER  VARIANT_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out VARIANT * ); 
void                      __RPC_USER  VARIANT_UserFree(     __RPC__in unsigned long *, __RPC__in VARIANT * ); 

unsigned long             __RPC_USER  HMENU_UserSize64(     __RPC__in unsigned long *, unsigned long            , __RPC__in HMENU * ); 
unsigned char * __RPC_USER  HMENU_UserMarshal64(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in HMENU * ); 
unsigned char * __RPC_USER  HMENU_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out HMENU * ); 
void                      __RPC_USER  HMENU_UserFree64(     __RPC__in unsigned long *, __RPC__in HMENU * ); 

unsigned long             __RPC_USER  VARIANT_UserSize64(     __RPC__in unsigned long *, unsigned long            , __RPC__in VARIANT * ); 
unsigned char * __RPC_USER  VARIANT_UserMarshal64(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in VARIANT * ); 
unsigned char * __RPC_USER  VARIANT_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out VARIANT * ); 
void                      __RPC_USER  VARIANT_UserFree64(     __RPC__in unsigned long *, __RPC__in VARIANT * ); 

/* [local] */ HRESULT __stdcall IEnumOleDocumentViews_Next_Proxy( 
    IEnumOleDocumentViews * This,
    /* [in] */ ULONG cViews,
    /* [out] */ IOleDocumentView **rgpView,
    /* [out] */ ULONG *pcFetched);


/* [call_as] */ HRESULT __stdcall IEnumOleDocumentViews_Next_Stub( 
    __RPC__in IEnumOleDocumentViews * This,
    /* [in] */ ULONG cViews,
    /* [length_is][size_is][out] */ __RPC__out_ecount_part(cViews, *pcFetched) IOleDocumentView **rgpView,
    /* [out] */ __RPC__out ULONG *pcFetched);

/* [local] */ HRESULT __stdcall IPrint_Print_Proxy( 
    IPrint * This,
    /* [in] */ DWORD grfFlags,
    /* [out][in] */ DVTARGETDEVICE **pptd,
    /* [out][in] */ PAGESET **ppPageSet,
    /* [unique][out][in] */ STGMEDIUM *pstgmOptions,
    /* [in] */ IContinueCallback *pcallback,
    /* [in] */ LONG nFirstPage,
    /* [out] */ LONG *pcPagesPrinted,
    /* [out] */ LONG *pnLastPage);


/* [call_as] */ HRESULT __stdcall IPrint_Print_Stub( 
    __RPC__in IPrint * This,
    /* [in] */ DWORD grfFlags,
    /* [out][in] */ __RPC__deref_inout_opt DVTARGETDEVICE **pptd,
    /* [out][in] */ __RPC__deref_inout_opt PAGESET **pppageset,
    /* [unique][out][in] */ __RPC__inout_opt RemSTGMEDIUM *pstgmOptions,
    /* [in] */ __RPC__in_opt IContinueCallback *pcallback,
    /* [in] */ LONG nFirstPage,
    /* [out] */ __RPC__out LONG *pcPagesPrinted,
    /* [out] */ __RPC__out LONG *pnLastPage);



/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


