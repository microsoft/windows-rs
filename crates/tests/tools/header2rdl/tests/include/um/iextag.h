

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

#ifndef __iextag_h__
#define __iextag_h__

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

#ifndef __IPeerFactory_FWD_DEFINED__
#define __IPeerFactory_FWD_DEFINED__
typedef interface IPeerFactory IPeerFactory;

#endif 	/* __IPeerFactory_FWD_DEFINED__ */


#ifndef __IHomePage_FWD_DEFINED__
#define __IHomePage_FWD_DEFINED__
typedef interface IHomePage IHomePage;

#endif 	/* __IHomePage_FWD_DEFINED__ */


#ifndef __IIntelliForms_FWD_DEFINED__
#define __IIntelliForms_FWD_DEFINED__
typedef interface IIntelliForms IIntelliForms;

#endif 	/* __IIntelliForms_FWD_DEFINED__ */


#ifndef __Iwfolders_FWD_DEFINED__
#define __Iwfolders_FWD_DEFINED__
typedef interface Iwfolders Iwfolders;

#endif 	/* __Iwfolders_FWD_DEFINED__ */


#ifndef __IAnchorClick_FWD_DEFINED__
#define __IAnchorClick_FWD_DEFINED__
typedef interface IAnchorClick IAnchorClick;

#endif 	/* __IAnchorClick_FWD_DEFINED__ */


#ifndef __HTMLPersistEvents_FWD_DEFINED__
#define __HTMLPersistEvents_FWD_DEFINED__
typedef interface HTMLPersistEvents HTMLPersistEvents;

#endif 	/* __HTMLPersistEvents_FWD_DEFINED__ */


#ifndef __IHTMLUserDataOM_FWD_DEFINED__
#define __IHTMLUserDataOM_FWD_DEFINED__
typedef interface IHTMLUserDataOM IHTMLUserDataOM;

#endif 	/* __IHTMLUserDataOM_FWD_DEFINED__ */


#ifndef __IHTMLPersistDataOM_FWD_DEFINED__
#define __IHTMLPersistDataOM_FWD_DEFINED__
typedef interface IHTMLPersistDataOM IHTMLPersistDataOM;

#endif 	/* __IHTMLPersistDataOM_FWD_DEFINED__ */


#ifndef __IHTMLPersistData_FWD_DEFINED__
#define __IHTMLPersistData_FWD_DEFINED__
typedef interface IHTMLPersistData IHTMLPersistData;

#endif 	/* __IHTMLPersistData_FWD_DEFINED__ */


#ifndef __IDownloadBehavior_FWD_DEFINED__
#define __IDownloadBehavior_FWD_DEFINED__
typedef interface IDownloadBehavior IDownloadBehavior;

#endif 	/* __IDownloadBehavior_FWD_DEFINED__ */


#ifndef __LayoutRectEvents_FWD_DEFINED__
#define __LayoutRectEvents_FWD_DEFINED__
typedef interface LayoutRectEvents LayoutRectEvents;

#endif 	/* __LayoutRectEvents_FWD_DEFINED__ */


#ifndef __ILayoutRect_FWD_DEFINED__
#define __ILayoutRect_FWD_DEFINED__
typedef interface ILayoutRect ILayoutRect;

#endif 	/* __ILayoutRect_FWD_DEFINED__ */


#ifndef __IDeviceRect_FWD_DEFINED__
#define __IDeviceRect_FWD_DEFINED__
typedef interface IDeviceRect IDeviceRect;

#endif 	/* __IDeviceRect_FWD_DEFINED__ */


#ifndef __IHeaderFooter_FWD_DEFINED__
#define __IHeaderFooter_FWD_DEFINED__
typedef interface IHeaderFooter IHeaderFooter;

#endif 	/* __IHeaderFooter_FWD_DEFINED__ */


#ifndef __IHeaderFooter2_FWD_DEFINED__
#define __IHeaderFooter2_FWD_DEFINED__
typedef interface IHeaderFooter2 IHeaderFooter2;

#endif 	/* __IHeaderFooter2_FWD_DEFINED__ */


#ifndef __PeerFactory_FWD_DEFINED__
#define __PeerFactory_FWD_DEFINED__

#ifdef __cplusplus
typedef class PeerFactory PeerFactory;
#else
typedef struct PeerFactory PeerFactory;
#endif /* __cplusplus */

#endif 	/* __PeerFactory_FWD_DEFINED__ */


#ifndef __IntelliForms_FWD_DEFINED__
#define __IntelliForms_FWD_DEFINED__

#ifdef __cplusplus
typedef class IntelliForms IntelliForms;
#else
typedef struct IntelliForms IntelliForms;
#endif /* __cplusplus */

#endif 	/* __IntelliForms_FWD_DEFINED__ */


#ifndef __HomePage_FWD_DEFINED__
#define __HomePage_FWD_DEFINED__

#ifdef __cplusplus
typedef class HomePage HomePage;
#else
typedef struct HomePage HomePage;
#endif /* __cplusplus */

#endif 	/* __HomePage_FWD_DEFINED__ */


#ifndef __CPersistUserData_FWD_DEFINED__
#define __CPersistUserData_FWD_DEFINED__

#ifdef __cplusplus
typedef class CPersistUserData CPersistUserData;
#else
typedef struct CPersistUserData CPersistUserData;
#endif /* __cplusplus */

#endif 	/* __CPersistUserData_FWD_DEFINED__ */


#ifndef __CPersistDataPeer_FWD_DEFINED__
#define __CPersistDataPeer_FWD_DEFINED__

#ifdef __cplusplus
typedef class CPersistDataPeer CPersistDataPeer;
#else
typedef struct CPersistDataPeer CPersistDataPeer;
#endif /* __cplusplus */

#endif 	/* __CPersistDataPeer_FWD_DEFINED__ */


#ifndef __CPersistShortcut_FWD_DEFINED__
#define __CPersistShortcut_FWD_DEFINED__

#ifdef __cplusplus
typedef class CPersistShortcut CPersistShortcut;
#else
typedef struct CPersistShortcut CPersistShortcut;
#endif /* __cplusplus */

#endif 	/* __CPersistShortcut_FWD_DEFINED__ */


#ifndef __CPersistHistory_FWD_DEFINED__
#define __CPersistHistory_FWD_DEFINED__

#ifdef __cplusplus
typedef class CPersistHistory CPersistHistory;
#else
typedef struct CPersistHistory CPersistHistory;
#endif /* __cplusplus */

#endif 	/* __CPersistHistory_FWD_DEFINED__ */


#ifndef __CPersistSnapshot_FWD_DEFINED__
#define __CPersistSnapshot_FWD_DEFINED__

#ifdef __cplusplus
typedef class CPersistSnapshot CPersistSnapshot;
#else
typedef struct CPersistSnapshot CPersistSnapshot;
#endif /* __cplusplus */

#endif 	/* __CPersistSnapshot_FWD_DEFINED__ */


#ifndef __CDownloadBehavior_FWD_DEFINED__
#define __CDownloadBehavior_FWD_DEFINED__

#ifdef __cplusplus
typedef class CDownloadBehavior CDownloadBehavior;
#else
typedef struct CDownloadBehavior CDownloadBehavior;
#endif /* __cplusplus */

#endif 	/* __CDownloadBehavior_FWD_DEFINED__ */


#ifndef __wfolders_FWD_DEFINED__
#define __wfolders_FWD_DEFINED__

#ifdef __cplusplus
typedef class wfolders wfolders;
#else
typedef struct wfolders wfolders;
#endif /* __cplusplus */

#endif 	/* __wfolders_FWD_DEFINED__ */


#ifndef __AnchorClick_FWD_DEFINED__
#define __AnchorClick_FWD_DEFINED__

#ifdef __cplusplus
typedef class AnchorClick AnchorClick;
#else
typedef struct AnchorClick AnchorClick;
#endif /* __cplusplus */

#endif 	/* __AnchorClick_FWD_DEFINED__ */


#ifndef __CLayoutRect_FWD_DEFINED__
#define __CLayoutRect_FWD_DEFINED__

#ifdef __cplusplus
typedef class CLayoutRect CLayoutRect;
#else
typedef struct CLayoutRect CLayoutRect;
#endif /* __cplusplus */

#endif 	/* __CLayoutRect_FWD_DEFINED__ */


#ifndef __CDeviceRect_FWD_DEFINED__
#define __CDeviceRect_FWD_DEFINED__

#ifdef __cplusplus
typedef class CDeviceRect CDeviceRect;
#else
typedef struct CDeviceRect CDeviceRect;
#endif /* __cplusplus */

#endif 	/* __CDeviceRect_FWD_DEFINED__ */


#ifndef __CHeaderFooter_FWD_DEFINED__
#define __CHeaderFooter_FWD_DEFINED__

#ifdef __cplusplus
typedef class CHeaderFooter CHeaderFooter;
#else
typedef struct CHeaderFooter CHeaderFooter;
#endif /* __cplusplus */

#endif 	/* __CHeaderFooter_FWD_DEFINED__ */


/* header files for imported files */
#include "oaidl.h"
#include "ocidl.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_iextag_0000_0000 */
/* [local] */ 

//=--------------------------------------------------------------------------=
// iextag.h
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


extern RPC_IF_HANDLE __MIDL_itf_iextag_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_iextag_0000_0000_v0_0_s_ifspec;

#ifndef __IPeerFactory_INTERFACE_DEFINED__
#define __IPeerFactory_INTERFACE_DEFINED__

/* interface IPeerFactory */
/* [object][unique][helpstring][uuid] */ 


EXTERN_C const IID IID_IPeerFactory;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("6663F9D3-B482-11d1-89C6-00C04FB6BFC4")
    IPeerFactory : public IUnknown
    {
    public:
    };
    
    
#else 	/* C style interface */

    typedef struct IPeerFactoryVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IPeerFactory * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IPeerFactory * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IPeerFactory * This);
        
        END_INTERFACE
    } IPeerFactoryVtbl;

    interface IPeerFactory
    {
        CONST_VTBL struct IPeerFactoryVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IPeerFactory_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IPeerFactory_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IPeerFactory_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IPeerFactory_INTERFACE_DEFINED__ */


#ifndef __IHomePage_INTERFACE_DEFINED__
#define __IHomePage_INTERFACE_DEFINED__

/* interface IHomePage */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IHomePage;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("766BF2AF-D650-11d1-9811-00C04FC31D2E")
    IHomePage : public IDispatch
    {
    public:
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE navigateHomePage( void) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE setHomePage( 
            /* [in] */ __RPC__in BSTR bstrURL) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE isHomePage( 
            /* [in] */ __RPC__in BSTR bstrURL,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *p) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IHomePageVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IHomePage * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IHomePage * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IHomePage * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IHomePage * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IHomePage * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IHomePage * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IHomePage * This,
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
        
        DECLSPEC_XFGVIRT(IHomePage, navigateHomePage)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *navigateHomePage )( 
            __RPC__in IHomePage * This);
        
        DECLSPEC_XFGVIRT(IHomePage, setHomePage)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *setHomePage )( 
            __RPC__in IHomePage * This,
            /* [in] */ __RPC__in BSTR bstrURL);
        
        DECLSPEC_XFGVIRT(IHomePage, isHomePage)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *isHomePage )( 
            __RPC__in IHomePage * This,
            /* [in] */ __RPC__in BSTR bstrURL,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *p);
        
        END_INTERFACE
    } IHomePageVtbl;

    interface IHomePage
    {
        CONST_VTBL struct IHomePageVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IHomePage_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IHomePage_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IHomePage_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IHomePage_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IHomePage_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IHomePage_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IHomePage_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IHomePage_navigateHomePage(This)	\
    ( (This)->lpVtbl -> navigateHomePage(This) ) 

#define IHomePage_setHomePage(This,bstrURL)	\
    ( (This)->lpVtbl -> setHomePage(This,bstrURL) ) 

#define IHomePage_isHomePage(This,bstrURL,p)	\
    ( (This)->lpVtbl -> isHomePage(This,bstrURL,p) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IHomePage_INTERFACE_DEFINED__ */


#ifndef __IIntelliForms_INTERFACE_DEFINED__
#define __IIntelliForms_INTERFACE_DEFINED__

/* interface IIntelliForms */
/* [unique][dual][uuid][object] */ 


EXTERN_C const IID IID_IIntelliForms;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("9B9F68E6-1AAA-11d2-BCA5-00C04FD929DB")
    IIntelliForms : public IDispatch
    {
    public:
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_enabled( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pVal) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_enabled( 
            /* [in] */ VARIANT_BOOL bVal) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IIntelliFormsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IIntelliForms * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IIntelliForms * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IIntelliForms * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IIntelliForms * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IIntelliForms * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IIntelliForms * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IIntelliForms * This,
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
        
        DECLSPEC_XFGVIRT(IIntelliForms, get_enabled)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_enabled )( 
            __RPC__in IIntelliForms * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pVal);
        
        DECLSPEC_XFGVIRT(IIntelliForms, put_enabled)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_enabled )( 
            __RPC__in IIntelliForms * This,
            /* [in] */ VARIANT_BOOL bVal);
        
        END_INTERFACE
    } IIntelliFormsVtbl;

    interface IIntelliForms
    {
        CONST_VTBL struct IIntelliFormsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IIntelliForms_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IIntelliForms_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IIntelliForms_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IIntelliForms_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IIntelliForms_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IIntelliForms_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IIntelliForms_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IIntelliForms_get_enabled(This,pVal)	\
    ( (This)->lpVtbl -> get_enabled(This,pVal) ) 

#define IIntelliForms_put_enabled(This,bVal)	\
    ( (This)->lpVtbl -> put_enabled(This,bVal) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IIntelliForms_INTERFACE_DEFINED__ */


#ifndef __Iwfolders_INTERFACE_DEFINED__
#define __Iwfolders_INTERFACE_DEFINED__

/* interface Iwfolders */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_Iwfolders;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("BAE31F98-1B81-11D2-A97A-00C04F8ECB02")
    Iwfolders : public IDispatch
    {
    public:
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE navigate( 
            /* [in] */ __RPC__in BSTR bstrUrl,
            /* [out][retval] */ __RPC__deref_out_opt BSTR *pbstrRetVal) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE navigateFrame( 
            /* [in] */ __RPC__in BSTR bstrUrl,
            /* [in] */ __RPC__in BSTR bstrTargetFrame,
            /* [out][retval] */ __RPC__deref_out_opt BSTR *pbstrRetVal) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE navigateNoSite( 
            /* [in] */ __RPC__in BSTR bstrUrl,
            /* [in] */ __RPC__in BSTR bstrTargetFrame,
            /* [in] */ DWORD dwhwnd,
            /* [in] */ __RPC__in_opt IUnknown *pwb) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IwfoldersVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in Iwfolders * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in Iwfolders * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in Iwfolders * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in Iwfolders * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in Iwfolders * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in Iwfolders * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            Iwfolders * This,
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
        
        DECLSPEC_XFGVIRT(Iwfolders, navigate)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *navigate )( 
            __RPC__in Iwfolders * This,
            /* [in] */ __RPC__in BSTR bstrUrl,
            /* [out][retval] */ __RPC__deref_out_opt BSTR *pbstrRetVal);
        
        DECLSPEC_XFGVIRT(Iwfolders, navigateFrame)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *navigateFrame )( 
            __RPC__in Iwfolders * This,
            /* [in] */ __RPC__in BSTR bstrUrl,
            /* [in] */ __RPC__in BSTR bstrTargetFrame,
            /* [out][retval] */ __RPC__deref_out_opt BSTR *pbstrRetVal);
        
        DECLSPEC_XFGVIRT(Iwfolders, navigateNoSite)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *navigateNoSite )( 
            __RPC__in Iwfolders * This,
            /* [in] */ __RPC__in BSTR bstrUrl,
            /* [in] */ __RPC__in BSTR bstrTargetFrame,
            /* [in] */ DWORD dwhwnd,
            /* [in] */ __RPC__in_opt IUnknown *pwb);
        
        END_INTERFACE
    } IwfoldersVtbl;

    interface Iwfolders
    {
        CONST_VTBL struct IwfoldersVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define Iwfolders_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define Iwfolders_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define Iwfolders_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define Iwfolders_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define Iwfolders_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define Iwfolders_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define Iwfolders_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define Iwfolders_navigate(This,bstrUrl,pbstrRetVal)	\
    ( (This)->lpVtbl -> navigate(This,bstrUrl,pbstrRetVal) ) 

#define Iwfolders_navigateFrame(This,bstrUrl,bstrTargetFrame,pbstrRetVal)	\
    ( (This)->lpVtbl -> navigateFrame(This,bstrUrl,bstrTargetFrame,pbstrRetVal) ) 

#define Iwfolders_navigateNoSite(This,bstrUrl,bstrTargetFrame,dwhwnd,pwb)	\
    ( (This)->lpVtbl -> navigateNoSite(This,bstrUrl,bstrTargetFrame,dwhwnd,pwb) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __Iwfolders_INTERFACE_DEFINED__ */


#ifndef __IAnchorClick_INTERFACE_DEFINED__
#define __IAnchorClick_INTERFACE_DEFINED__

/* interface IAnchorClick */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IAnchorClick;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("13D5413B-33B9-11D2-95A7-00C04F8ECB02")
    IAnchorClick : public IDispatch
    {
    public:
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE ProcOnClick( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAnchorClickVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IAnchorClick * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IAnchorClick * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IAnchorClick * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IAnchorClick * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IAnchorClick * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IAnchorClick * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IAnchorClick * This,
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
        
        DECLSPEC_XFGVIRT(IAnchorClick, ProcOnClick)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *ProcOnClick )( 
            __RPC__in IAnchorClick * This);
        
        END_INTERFACE
    } IAnchorClickVtbl;

    interface IAnchorClick
    {
        CONST_VTBL struct IAnchorClickVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAnchorClick_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAnchorClick_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAnchorClick_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAnchorClick_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IAnchorClick_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IAnchorClick_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IAnchorClick_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IAnchorClick_ProcOnClick(This)	\
    ( (This)->lpVtbl -> ProcOnClick(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAnchorClick_INTERFACE_DEFINED__ */


#ifndef __IHTMLUserDataOM_INTERFACE_DEFINED__
#define __IHTMLUserDataOM_INTERFACE_DEFINED__

/* interface IHTMLUserDataOM */
/* [object][uuid][dual][oleautomation] */ 


EXTERN_C const IID IID_IHTMLUserDataOM;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("3050f48f-98b5-11cf-bb82-00aa00bdce0b")
    IHTMLUserDataOM : public IDispatch
    {
    public:
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_XMLDocument( 
            /* [out][retval] */ __RPC__deref_out_opt IDispatch **p) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE save( 
            /* [in] */ __RPC__in BSTR strName) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE load( 
            /* [in] */ __RPC__in BSTR strName) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE getAttribute( 
            /* [in] */ __RPC__in BSTR name,
            /* [out][retval] */ __RPC__out VARIANT *pValue) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE setAttribute( 
            /* [in] */ __RPC__in BSTR name,
            /* [in] */ VARIANT value) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE removeAttribute( 
            /* [in] */ __RPC__in BSTR name) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_expires( 
            /* [in] */ __RPC__in BSTR bstr) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_expires( 
            /* [out][retval] */ __RPC__deref_out_opt BSTR *pbstr) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IHTMLUserDataOMVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IHTMLUserDataOM * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IHTMLUserDataOM * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IHTMLUserDataOM * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IHTMLUserDataOM * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IHTMLUserDataOM * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IHTMLUserDataOM * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IHTMLUserDataOM * This,
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
        
        DECLSPEC_XFGVIRT(IHTMLUserDataOM, get_XMLDocument)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_XMLDocument )( 
            __RPC__in IHTMLUserDataOM * This,
            /* [out][retval] */ __RPC__deref_out_opt IDispatch **p);
        
        DECLSPEC_XFGVIRT(IHTMLUserDataOM, save)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *save )( 
            __RPC__in IHTMLUserDataOM * This,
            /* [in] */ __RPC__in BSTR strName);
        
        DECLSPEC_XFGVIRT(IHTMLUserDataOM, load)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *load )( 
            __RPC__in IHTMLUserDataOM * This,
            /* [in] */ __RPC__in BSTR strName);
        
        DECLSPEC_XFGVIRT(IHTMLUserDataOM, getAttribute)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *getAttribute )( 
            __RPC__in IHTMLUserDataOM * This,
            /* [in] */ __RPC__in BSTR name,
            /* [out][retval] */ __RPC__out VARIANT *pValue);
        
        DECLSPEC_XFGVIRT(IHTMLUserDataOM, setAttribute)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *setAttribute )( 
            __RPC__in IHTMLUserDataOM * This,
            /* [in] */ __RPC__in BSTR name,
            /* [in] */ VARIANT value);
        
        DECLSPEC_XFGVIRT(IHTMLUserDataOM, removeAttribute)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *removeAttribute )( 
            __RPC__in IHTMLUserDataOM * This,
            /* [in] */ __RPC__in BSTR name);
        
        DECLSPEC_XFGVIRT(IHTMLUserDataOM, put_expires)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_expires )( 
            __RPC__in IHTMLUserDataOM * This,
            /* [in] */ __RPC__in BSTR bstr);
        
        DECLSPEC_XFGVIRT(IHTMLUserDataOM, get_expires)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_expires )( 
            __RPC__in IHTMLUserDataOM * This,
            /* [out][retval] */ __RPC__deref_out_opt BSTR *pbstr);
        
        END_INTERFACE
    } IHTMLUserDataOMVtbl;

    interface IHTMLUserDataOM
    {
        CONST_VTBL struct IHTMLUserDataOMVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IHTMLUserDataOM_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IHTMLUserDataOM_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IHTMLUserDataOM_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IHTMLUserDataOM_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IHTMLUserDataOM_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IHTMLUserDataOM_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IHTMLUserDataOM_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IHTMLUserDataOM_get_XMLDocument(This,p)	\
    ( (This)->lpVtbl -> get_XMLDocument(This,p) ) 

#define IHTMLUserDataOM_save(This,strName)	\
    ( (This)->lpVtbl -> save(This,strName) ) 

#define IHTMLUserDataOM_load(This,strName)	\
    ( (This)->lpVtbl -> load(This,strName) ) 

#define IHTMLUserDataOM_getAttribute(This,name,pValue)	\
    ( (This)->lpVtbl -> getAttribute(This,name,pValue) ) 

#define IHTMLUserDataOM_setAttribute(This,name,value)	\
    ( (This)->lpVtbl -> setAttribute(This,name,value) ) 

#define IHTMLUserDataOM_removeAttribute(This,name)	\
    ( (This)->lpVtbl -> removeAttribute(This,name) ) 

#define IHTMLUserDataOM_put_expires(This,bstr)	\
    ( (This)->lpVtbl -> put_expires(This,bstr) ) 

#define IHTMLUserDataOM_get_expires(This,pbstr)	\
    ( (This)->lpVtbl -> get_expires(This,pbstr) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IHTMLUserDataOM_INTERFACE_DEFINED__ */


#ifndef __IHTMLPersistDataOM_INTERFACE_DEFINED__
#define __IHTMLPersistDataOM_INTERFACE_DEFINED__

/* interface IHTMLPersistDataOM */
/* [object][uuid][dual][oleautomation] */ 


EXTERN_C const IID IID_IHTMLPersistDataOM;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("3050f4c0-98b5-11cf-bb82-00aa00bdce0b")
    IHTMLPersistDataOM : public IDispatch
    {
    public:
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_XMLDocument( 
            /* [out][retval] */ __RPC__deref_out_opt IDispatch **p) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE getAttribute( 
            /* [in] */ __RPC__in BSTR name,
            /* [out][retval] */ __RPC__out VARIANT *pValue) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE setAttribute( 
            /* [in] */ __RPC__in BSTR name,
            /* [in] */ VARIANT value) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE removeAttribute( 
            /* [in] */ __RPC__in BSTR name) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IHTMLPersistDataOMVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IHTMLPersistDataOM * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IHTMLPersistDataOM * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IHTMLPersistDataOM * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IHTMLPersistDataOM * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IHTMLPersistDataOM * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IHTMLPersistDataOM * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IHTMLPersistDataOM * This,
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
        
        DECLSPEC_XFGVIRT(IHTMLPersistDataOM, get_XMLDocument)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_XMLDocument )( 
            __RPC__in IHTMLPersistDataOM * This,
            /* [out][retval] */ __RPC__deref_out_opt IDispatch **p);
        
        DECLSPEC_XFGVIRT(IHTMLPersistDataOM, getAttribute)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *getAttribute )( 
            __RPC__in IHTMLPersistDataOM * This,
            /* [in] */ __RPC__in BSTR name,
            /* [out][retval] */ __RPC__out VARIANT *pValue);
        
        DECLSPEC_XFGVIRT(IHTMLPersistDataOM, setAttribute)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *setAttribute )( 
            __RPC__in IHTMLPersistDataOM * This,
            /* [in] */ __RPC__in BSTR name,
            /* [in] */ VARIANT value);
        
        DECLSPEC_XFGVIRT(IHTMLPersistDataOM, removeAttribute)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *removeAttribute )( 
            __RPC__in IHTMLPersistDataOM * This,
            /* [in] */ __RPC__in BSTR name);
        
        END_INTERFACE
    } IHTMLPersistDataOMVtbl;

    interface IHTMLPersistDataOM
    {
        CONST_VTBL struct IHTMLPersistDataOMVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IHTMLPersistDataOM_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IHTMLPersistDataOM_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IHTMLPersistDataOM_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IHTMLPersistDataOM_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IHTMLPersistDataOM_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IHTMLPersistDataOM_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IHTMLPersistDataOM_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IHTMLPersistDataOM_get_XMLDocument(This,p)	\
    ( (This)->lpVtbl -> get_XMLDocument(This,p) ) 

#define IHTMLPersistDataOM_getAttribute(This,name,pValue)	\
    ( (This)->lpVtbl -> getAttribute(This,name,pValue) ) 

#define IHTMLPersistDataOM_setAttribute(This,name,value)	\
    ( (This)->lpVtbl -> setAttribute(This,name,value) ) 

#define IHTMLPersistDataOM_removeAttribute(This,name)	\
    ( (This)->lpVtbl -> removeAttribute(This,name) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IHTMLPersistDataOM_INTERFACE_DEFINED__ */


#ifndef __IHTMLPersistData_INTERFACE_DEFINED__
#define __IHTMLPersistData_INTERFACE_DEFINED__

/* interface IHTMLPersistData */
/* [object][uuid][dual][oleautomation] */ 


EXTERN_C const IID IID_IHTMLPersistData;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("3050f4c5-98b5-11cf-bb82-00aa00bdce0b")
    IHTMLPersistData : public IUnknown
    {
    public:
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE save( 
            /* [in] */ __RPC__in_opt IUnknown *pUnk,
            /* [in] */ long lType,
            /* [out][retval] */ __RPC__out VARIANT_BOOL *fContinueBroacast) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE load( 
            /* [in] */ __RPC__in_opt IUnknown *pUnk,
            /* [in] */ long lType,
            /* [out][retval] */ __RPC__out VARIANT_BOOL *fDoDefault) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE queryType( 
            /* [in] */ long lType,
            /* [out][retval] */ __RPC__out VARIANT_BOOL *pfSupportsType) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IHTMLPersistDataVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IHTMLPersistData * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IHTMLPersistData * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IHTMLPersistData * This);
        
        DECLSPEC_XFGVIRT(IHTMLPersistData, save)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *save )( 
            __RPC__in IHTMLPersistData * This,
            /* [in] */ __RPC__in_opt IUnknown *pUnk,
            /* [in] */ long lType,
            /* [out][retval] */ __RPC__out VARIANT_BOOL *fContinueBroacast);
        
        DECLSPEC_XFGVIRT(IHTMLPersistData, load)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *load )( 
            __RPC__in IHTMLPersistData * This,
            /* [in] */ __RPC__in_opt IUnknown *pUnk,
            /* [in] */ long lType,
            /* [out][retval] */ __RPC__out VARIANT_BOOL *fDoDefault);
        
        DECLSPEC_XFGVIRT(IHTMLPersistData, queryType)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *queryType )( 
            __RPC__in IHTMLPersistData * This,
            /* [in] */ long lType,
            /* [out][retval] */ __RPC__out VARIANT_BOOL *pfSupportsType);
        
        END_INTERFACE
    } IHTMLPersistDataVtbl;

    interface IHTMLPersistData
    {
        CONST_VTBL struct IHTMLPersistDataVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IHTMLPersistData_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IHTMLPersistData_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IHTMLPersistData_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IHTMLPersistData_save(This,pUnk,lType,fContinueBroacast)	\
    ( (This)->lpVtbl -> save(This,pUnk,lType,fContinueBroacast) ) 

#define IHTMLPersistData_load(This,pUnk,lType,fDoDefault)	\
    ( (This)->lpVtbl -> load(This,pUnk,lType,fDoDefault) ) 

#define IHTMLPersistData_queryType(This,lType,pfSupportsType)	\
    ( (This)->lpVtbl -> queryType(This,lType,pfSupportsType) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IHTMLPersistData_INTERFACE_DEFINED__ */


#ifndef __IDownloadBehavior_INTERFACE_DEFINED__
#define __IDownloadBehavior_INTERFACE_DEFINED__

/* interface IDownloadBehavior */
/* [object][uuid][dual][oleautomation] */ 


EXTERN_C const IID IID_IDownloadBehavior;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("3050f5bd-98b5-11cf-bb82-00aa00bdce0b")
    IDownloadBehavior : public IDispatch
    {
    public:
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE startDownload( 
            /* [in] */ __RPC__in BSTR bstrUrl,
            /* [in] */ __RPC__in_opt IDispatch *pdispCallback) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDownloadBehaviorVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IDownloadBehavior * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IDownloadBehavior * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IDownloadBehavior * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IDownloadBehavior * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IDownloadBehavior * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IDownloadBehavior * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IDownloadBehavior * This,
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
        
        DECLSPEC_XFGVIRT(IDownloadBehavior, startDownload)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *startDownload )( 
            __RPC__in IDownloadBehavior * This,
            /* [in] */ __RPC__in BSTR bstrUrl,
            /* [in] */ __RPC__in_opt IDispatch *pdispCallback);
        
        END_INTERFACE
    } IDownloadBehaviorVtbl;

    interface IDownloadBehavior
    {
        CONST_VTBL struct IDownloadBehaviorVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDownloadBehavior_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDownloadBehavior_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDownloadBehavior_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDownloadBehavior_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IDownloadBehavior_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IDownloadBehavior_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IDownloadBehavior_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IDownloadBehavior_startDownload(This,bstrUrl,pdispCallback)	\
    ( (This)->lpVtbl -> startDownload(This,bstrUrl,pdispCallback) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDownloadBehavior_INTERFACE_DEFINED__ */


#ifndef __ILayoutRect_INTERFACE_DEFINED__
#define __ILayoutRect_INTERFACE_DEFINED__

/* interface ILayoutRect */
/* [object][uuid][dual][oleautomation] */ 


EXTERN_C const IID IID_ILayoutRect;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("3050f665-98b5-11cf-bb82-00aa00bdce0b")
    ILayoutRect : public IDispatch
    {
    public:
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_nextRect( 
            /* [in] */ __RPC__in BSTR bstrElementId) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_nextRect( 
            /* [out][retval] */ __RPC__deref_out_opt BSTR *pbstrElementId) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_contentSrc( 
            /* [in] */ VARIANT varContentSrc) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_contentSrc( 
            /* [out][retval] */ __RPC__out VARIANT *pvarContentSrc) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_honorPageBreaks( 
            /* [in] */ VARIANT_BOOL v) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_honorPageBreaks( 
            /* [out][retval] */ __RPC__out VARIANT_BOOL *p) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_honorPageRules( 
            /* [in] */ VARIANT_BOOL v) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_honorPageRules( 
            /* [out][retval] */ __RPC__out VARIANT_BOOL *p) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_nextRectElement( 
            /* [in] */ __RPC__in_opt IDispatch *pElem) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_nextRectElement( 
            /* [out][retval] */ __RPC__deref_out_opt IDispatch **ppElem) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_contentDocument( 
            /* [out][retval] */ __RPC__deref_out_opt IDispatch **pDoc) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ILayoutRectVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ILayoutRect * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ILayoutRect * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ILayoutRect * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ILayoutRect * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ILayoutRect * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ILayoutRect * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ILayoutRect * This,
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
        
        DECLSPEC_XFGVIRT(ILayoutRect, put_nextRect)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_nextRect )( 
            __RPC__in ILayoutRect * This,
            /* [in] */ __RPC__in BSTR bstrElementId);
        
        DECLSPEC_XFGVIRT(ILayoutRect, get_nextRect)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_nextRect )( 
            __RPC__in ILayoutRect * This,
            /* [out][retval] */ __RPC__deref_out_opt BSTR *pbstrElementId);
        
        DECLSPEC_XFGVIRT(ILayoutRect, put_contentSrc)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_contentSrc )( 
            __RPC__in ILayoutRect * This,
            /* [in] */ VARIANT varContentSrc);
        
        DECLSPEC_XFGVIRT(ILayoutRect, get_contentSrc)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_contentSrc )( 
            __RPC__in ILayoutRect * This,
            /* [out][retval] */ __RPC__out VARIANT *pvarContentSrc);
        
        DECLSPEC_XFGVIRT(ILayoutRect, put_honorPageBreaks)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_honorPageBreaks )( 
            __RPC__in ILayoutRect * This,
            /* [in] */ VARIANT_BOOL v);
        
        DECLSPEC_XFGVIRT(ILayoutRect, get_honorPageBreaks)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_honorPageBreaks )( 
            __RPC__in ILayoutRect * This,
            /* [out][retval] */ __RPC__out VARIANT_BOOL *p);
        
        DECLSPEC_XFGVIRT(ILayoutRect, put_honorPageRules)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_honorPageRules )( 
            __RPC__in ILayoutRect * This,
            /* [in] */ VARIANT_BOOL v);
        
        DECLSPEC_XFGVIRT(ILayoutRect, get_honorPageRules)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_honorPageRules )( 
            __RPC__in ILayoutRect * This,
            /* [out][retval] */ __RPC__out VARIANT_BOOL *p);
        
        DECLSPEC_XFGVIRT(ILayoutRect, put_nextRectElement)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_nextRectElement )( 
            __RPC__in ILayoutRect * This,
            /* [in] */ __RPC__in_opt IDispatch *pElem);
        
        DECLSPEC_XFGVIRT(ILayoutRect, get_nextRectElement)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_nextRectElement )( 
            __RPC__in ILayoutRect * This,
            /* [out][retval] */ __RPC__deref_out_opt IDispatch **ppElem);
        
        DECLSPEC_XFGVIRT(ILayoutRect, get_contentDocument)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_contentDocument )( 
            __RPC__in ILayoutRect * This,
            /* [out][retval] */ __RPC__deref_out_opt IDispatch **pDoc);
        
        END_INTERFACE
    } ILayoutRectVtbl;

    interface ILayoutRect
    {
        CONST_VTBL struct ILayoutRectVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ILayoutRect_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ILayoutRect_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ILayoutRect_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ILayoutRect_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ILayoutRect_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ILayoutRect_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ILayoutRect_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ILayoutRect_put_nextRect(This,bstrElementId)	\
    ( (This)->lpVtbl -> put_nextRect(This,bstrElementId) ) 

#define ILayoutRect_get_nextRect(This,pbstrElementId)	\
    ( (This)->lpVtbl -> get_nextRect(This,pbstrElementId) ) 

#define ILayoutRect_put_contentSrc(This,varContentSrc)	\
    ( (This)->lpVtbl -> put_contentSrc(This,varContentSrc) ) 

#define ILayoutRect_get_contentSrc(This,pvarContentSrc)	\
    ( (This)->lpVtbl -> get_contentSrc(This,pvarContentSrc) ) 

#define ILayoutRect_put_honorPageBreaks(This,v)	\
    ( (This)->lpVtbl -> put_honorPageBreaks(This,v) ) 

#define ILayoutRect_get_honorPageBreaks(This,p)	\
    ( (This)->lpVtbl -> get_honorPageBreaks(This,p) ) 

#define ILayoutRect_put_honorPageRules(This,v)	\
    ( (This)->lpVtbl -> put_honorPageRules(This,v) ) 

#define ILayoutRect_get_honorPageRules(This,p)	\
    ( (This)->lpVtbl -> get_honorPageRules(This,p) ) 

#define ILayoutRect_put_nextRectElement(This,pElem)	\
    ( (This)->lpVtbl -> put_nextRectElement(This,pElem) ) 

#define ILayoutRect_get_nextRectElement(This,ppElem)	\
    ( (This)->lpVtbl -> get_nextRectElement(This,ppElem) ) 

#define ILayoutRect_get_contentDocument(This,pDoc)	\
    ( (This)->lpVtbl -> get_contentDocument(This,pDoc) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ILayoutRect_INTERFACE_DEFINED__ */


#ifndef __IDeviceRect_INTERFACE_DEFINED__
#define __IDeviceRect_INTERFACE_DEFINED__

/* interface IDeviceRect */
/* [object][uuid][dual][oleautomation] */ 


EXTERN_C const IID IID_IDeviceRect;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("3050f6d5-98b5-11cf-bb82-00aa00bdce0b")
    IDeviceRect : public IDispatch
    {
    public:
    };
    
    
#else 	/* C style interface */

    typedef struct IDeviceRectVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IDeviceRect * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IDeviceRect * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IDeviceRect * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IDeviceRect * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IDeviceRect * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IDeviceRect * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IDeviceRect * This,
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
    } IDeviceRectVtbl;

    interface IDeviceRect
    {
        CONST_VTBL struct IDeviceRectVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDeviceRect_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDeviceRect_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDeviceRect_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDeviceRect_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IDeviceRect_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IDeviceRect_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IDeviceRect_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDeviceRect_INTERFACE_DEFINED__ */


#ifndef __IHeaderFooter_INTERFACE_DEFINED__
#define __IHeaderFooter_INTERFACE_DEFINED__

/* interface IHeaderFooter */
/* [object][uuid][dual][oleautomation] */ 


EXTERN_C const IID IID_IHeaderFooter;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("3050f6ce-98b5-11cf-bb82-00aa00bdce0b")
    IHeaderFooter : public IDispatch
    {
    public:
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_htmlHead( 
            /* [out][retval] */ __RPC__deref_out_opt BSTR *p) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_htmlFoot( 
            /* [out][retval] */ __RPC__deref_out_opt BSTR *p) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_textHead( 
            /* [in] */ __RPC__in BSTR v) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_textHead( 
            /* [out][retval] */ __RPC__deref_out_opt BSTR *p) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_textFoot( 
            /* [in] */ __RPC__in BSTR v) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_textFoot( 
            /* [out][retval] */ __RPC__deref_out_opt BSTR *p) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_page( 
            /* [in] */ DWORD v) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_page( 
            /* [out][retval] */ __RPC__out DWORD *p) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_pageTotal( 
            /* [in] */ DWORD v) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_pageTotal( 
            /* [out][retval] */ __RPC__out DWORD *p) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_URL( 
            /* [in] */ __RPC__in BSTR v) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_URL( 
            /* [out][retval] */ __RPC__deref_out_opt BSTR *p) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_title( 
            /* [in] */ __RPC__in BSTR v) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_title( 
            /* [out][retval] */ __RPC__deref_out_opt BSTR *p) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_dateShort( 
            /* [in] */ __RPC__in BSTR v) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_dateShort( 
            /* [out][retval] */ __RPC__deref_out_opt BSTR *p) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_dateLong( 
            /* [in] */ __RPC__in BSTR v) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_dateLong( 
            /* [out][retval] */ __RPC__deref_out_opt BSTR *p) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_timeShort( 
            /* [in] */ __RPC__in BSTR v) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_timeShort( 
            /* [out][retval] */ __RPC__deref_out_opt BSTR *p) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_timeLong( 
            /* [in] */ __RPC__in BSTR v) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_timeLong( 
            /* [out][retval] */ __RPC__deref_out_opt BSTR *p) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IHeaderFooterVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IHeaderFooter * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IHeaderFooter * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IHeaderFooter * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IHeaderFooter * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IHeaderFooter * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IHeaderFooter * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IHeaderFooter * This,
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
        
        DECLSPEC_XFGVIRT(IHeaderFooter, get_htmlHead)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_htmlHead )( 
            __RPC__in IHeaderFooter * This,
            /* [out][retval] */ __RPC__deref_out_opt BSTR *p);
        
        DECLSPEC_XFGVIRT(IHeaderFooter, get_htmlFoot)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_htmlFoot )( 
            __RPC__in IHeaderFooter * This,
            /* [out][retval] */ __RPC__deref_out_opt BSTR *p);
        
        DECLSPEC_XFGVIRT(IHeaderFooter, put_textHead)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_textHead )( 
            __RPC__in IHeaderFooter * This,
            /* [in] */ __RPC__in BSTR v);
        
        DECLSPEC_XFGVIRT(IHeaderFooter, get_textHead)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_textHead )( 
            __RPC__in IHeaderFooter * This,
            /* [out][retval] */ __RPC__deref_out_opt BSTR *p);
        
        DECLSPEC_XFGVIRT(IHeaderFooter, put_textFoot)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_textFoot )( 
            __RPC__in IHeaderFooter * This,
            /* [in] */ __RPC__in BSTR v);
        
        DECLSPEC_XFGVIRT(IHeaderFooter, get_textFoot)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_textFoot )( 
            __RPC__in IHeaderFooter * This,
            /* [out][retval] */ __RPC__deref_out_opt BSTR *p);
        
        DECLSPEC_XFGVIRT(IHeaderFooter, put_page)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_page )( 
            __RPC__in IHeaderFooter * This,
            /* [in] */ DWORD v);
        
        DECLSPEC_XFGVIRT(IHeaderFooter, get_page)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_page )( 
            __RPC__in IHeaderFooter * This,
            /* [out][retval] */ __RPC__out DWORD *p);
        
        DECLSPEC_XFGVIRT(IHeaderFooter, put_pageTotal)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_pageTotal )( 
            __RPC__in IHeaderFooter * This,
            /* [in] */ DWORD v);
        
        DECLSPEC_XFGVIRT(IHeaderFooter, get_pageTotal)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_pageTotal )( 
            __RPC__in IHeaderFooter * This,
            /* [out][retval] */ __RPC__out DWORD *p);
        
        DECLSPEC_XFGVIRT(IHeaderFooter, put_URL)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_URL )( 
            __RPC__in IHeaderFooter * This,
            /* [in] */ __RPC__in BSTR v);
        
        DECLSPEC_XFGVIRT(IHeaderFooter, get_URL)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_URL )( 
            __RPC__in IHeaderFooter * This,
            /* [out][retval] */ __RPC__deref_out_opt BSTR *p);
        
        DECLSPEC_XFGVIRT(IHeaderFooter, put_title)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_title )( 
            __RPC__in IHeaderFooter * This,
            /* [in] */ __RPC__in BSTR v);
        
        DECLSPEC_XFGVIRT(IHeaderFooter, get_title)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_title )( 
            __RPC__in IHeaderFooter * This,
            /* [out][retval] */ __RPC__deref_out_opt BSTR *p);
        
        DECLSPEC_XFGVIRT(IHeaderFooter, put_dateShort)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_dateShort )( 
            __RPC__in IHeaderFooter * This,
            /* [in] */ __RPC__in BSTR v);
        
        DECLSPEC_XFGVIRT(IHeaderFooter, get_dateShort)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_dateShort )( 
            __RPC__in IHeaderFooter * This,
            /* [out][retval] */ __RPC__deref_out_opt BSTR *p);
        
        DECLSPEC_XFGVIRT(IHeaderFooter, put_dateLong)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_dateLong )( 
            __RPC__in IHeaderFooter * This,
            /* [in] */ __RPC__in BSTR v);
        
        DECLSPEC_XFGVIRT(IHeaderFooter, get_dateLong)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_dateLong )( 
            __RPC__in IHeaderFooter * This,
            /* [out][retval] */ __RPC__deref_out_opt BSTR *p);
        
        DECLSPEC_XFGVIRT(IHeaderFooter, put_timeShort)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_timeShort )( 
            __RPC__in IHeaderFooter * This,
            /* [in] */ __RPC__in BSTR v);
        
        DECLSPEC_XFGVIRT(IHeaderFooter, get_timeShort)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_timeShort )( 
            __RPC__in IHeaderFooter * This,
            /* [out][retval] */ __RPC__deref_out_opt BSTR *p);
        
        DECLSPEC_XFGVIRT(IHeaderFooter, put_timeLong)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_timeLong )( 
            __RPC__in IHeaderFooter * This,
            /* [in] */ __RPC__in BSTR v);
        
        DECLSPEC_XFGVIRT(IHeaderFooter, get_timeLong)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_timeLong )( 
            __RPC__in IHeaderFooter * This,
            /* [out][retval] */ __RPC__deref_out_opt BSTR *p);
        
        END_INTERFACE
    } IHeaderFooterVtbl;

    interface IHeaderFooter
    {
        CONST_VTBL struct IHeaderFooterVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IHeaderFooter_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IHeaderFooter_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IHeaderFooter_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IHeaderFooter_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IHeaderFooter_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IHeaderFooter_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IHeaderFooter_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IHeaderFooter_get_htmlHead(This,p)	\
    ( (This)->lpVtbl -> get_htmlHead(This,p) ) 

#define IHeaderFooter_get_htmlFoot(This,p)	\
    ( (This)->lpVtbl -> get_htmlFoot(This,p) ) 

#define IHeaderFooter_put_textHead(This,v)	\
    ( (This)->lpVtbl -> put_textHead(This,v) ) 

#define IHeaderFooter_get_textHead(This,p)	\
    ( (This)->lpVtbl -> get_textHead(This,p) ) 

#define IHeaderFooter_put_textFoot(This,v)	\
    ( (This)->lpVtbl -> put_textFoot(This,v) ) 

#define IHeaderFooter_get_textFoot(This,p)	\
    ( (This)->lpVtbl -> get_textFoot(This,p) ) 

#define IHeaderFooter_put_page(This,v)	\
    ( (This)->lpVtbl -> put_page(This,v) ) 

#define IHeaderFooter_get_page(This,p)	\
    ( (This)->lpVtbl -> get_page(This,p) ) 

#define IHeaderFooter_put_pageTotal(This,v)	\
    ( (This)->lpVtbl -> put_pageTotal(This,v) ) 

#define IHeaderFooter_get_pageTotal(This,p)	\
    ( (This)->lpVtbl -> get_pageTotal(This,p) ) 

#define IHeaderFooter_put_URL(This,v)	\
    ( (This)->lpVtbl -> put_URL(This,v) ) 

#define IHeaderFooter_get_URL(This,p)	\
    ( (This)->lpVtbl -> get_URL(This,p) ) 

#define IHeaderFooter_put_title(This,v)	\
    ( (This)->lpVtbl -> put_title(This,v) ) 

#define IHeaderFooter_get_title(This,p)	\
    ( (This)->lpVtbl -> get_title(This,p) ) 

#define IHeaderFooter_put_dateShort(This,v)	\
    ( (This)->lpVtbl -> put_dateShort(This,v) ) 

#define IHeaderFooter_get_dateShort(This,p)	\
    ( (This)->lpVtbl -> get_dateShort(This,p) ) 

#define IHeaderFooter_put_dateLong(This,v)	\
    ( (This)->lpVtbl -> put_dateLong(This,v) ) 

#define IHeaderFooter_get_dateLong(This,p)	\
    ( (This)->lpVtbl -> get_dateLong(This,p) ) 

#define IHeaderFooter_put_timeShort(This,v)	\
    ( (This)->lpVtbl -> put_timeShort(This,v) ) 

#define IHeaderFooter_get_timeShort(This,p)	\
    ( (This)->lpVtbl -> get_timeShort(This,p) ) 

#define IHeaderFooter_put_timeLong(This,v)	\
    ( (This)->lpVtbl -> put_timeLong(This,v) ) 

#define IHeaderFooter_get_timeLong(This,p)	\
    ( (This)->lpVtbl -> get_timeLong(This,p) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IHeaderFooter_INTERFACE_DEFINED__ */


#ifndef __IHeaderFooter2_INTERFACE_DEFINED__
#define __IHeaderFooter2_INTERFACE_DEFINED__

/* interface IHeaderFooter2 */
/* [object][uuid][dual][oleautomation] */ 


EXTERN_C const IID IID_IHeaderFooter2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("305104a5-98b5-11cf-bb82-00aa00bdce0b")
    IHeaderFooter2 : public IHeaderFooter
    {
    public:
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_font( 
            /* [in] */ __RPC__in BSTR v) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_font( 
            /* [out][retval] */ __RPC__deref_out_opt BSTR *p) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IHeaderFooter2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IHeaderFooter2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IHeaderFooter2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IHeaderFooter2 * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IHeaderFooter2 * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IHeaderFooter2 * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IHeaderFooter2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IHeaderFooter2 * This,
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
        
        DECLSPEC_XFGVIRT(IHeaderFooter, get_htmlHead)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_htmlHead )( 
            __RPC__in IHeaderFooter2 * This,
            /* [out][retval] */ __RPC__deref_out_opt BSTR *p);
        
        DECLSPEC_XFGVIRT(IHeaderFooter, get_htmlFoot)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_htmlFoot )( 
            __RPC__in IHeaderFooter2 * This,
            /* [out][retval] */ __RPC__deref_out_opt BSTR *p);
        
        DECLSPEC_XFGVIRT(IHeaderFooter, put_textHead)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_textHead )( 
            __RPC__in IHeaderFooter2 * This,
            /* [in] */ __RPC__in BSTR v);
        
        DECLSPEC_XFGVIRT(IHeaderFooter, get_textHead)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_textHead )( 
            __RPC__in IHeaderFooter2 * This,
            /* [out][retval] */ __RPC__deref_out_opt BSTR *p);
        
        DECLSPEC_XFGVIRT(IHeaderFooter, put_textFoot)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_textFoot )( 
            __RPC__in IHeaderFooter2 * This,
            /* [in] */ __RPC__in BSTR v);
        
        DECLSPEC_XFGVIRT(IHeaderFooter, get_textFoot)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_textFoot )( 
            __RPC__in IHeaderFooter2 * This,
            /* [out][retval] */ __RPC__deref_out_opt BSTR *p);
        
        DECLSPEC_XFGVIRT(IHeaderFooter, put_page)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_page )( 
            __RPC__in IHeaderFooter2 * This,
            /* [in] */ DWORD v);
        
        DECLSPEC_XFGVIRT(IHeaderFooter, get_page)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_page )( 
            __RPC__in IHeaderFooter2 * This,
            /* [out][retval] */ __RPC__out DWORD *p);
        
        DECLSPEC_XFGVIRT(IHeaderFooter, put_pageTotal)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_pageTotal )( 
            __RPC__in IHeaderFooter2 * This,
            /* [in] */ DWORD v);
        
        DECLSPEC_XFGVIRT(IHeaderFooter, get_pageTotal)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_pageTotal )( 
            __RPC__in IHeaderFooter2 * This,
            /* [out][retval] */ __RPC__out DWORD *p);
        
        DECLSPEC_XFGVIRT(IHeaderFooter, put_URL)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_URL )( 
            __RPC__in IHeaderFooter2 * This,
            /* [in] */ __RPC__in BSTR v);
        
        DECLSPEC_XFGVIRT(IHeaderFooter, get_URL)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_URL )( 
            __RPC__in IHeaderFooter2 * This,
            /* [out][retval] */ __RPC__deref_out_opt BSTR *p);
        
        DECLSPEC_XFGVIRT(IHeaderFooter, put_title)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_title )( 
            __RPC__in IHeaderFooter2 * This,
            /* [in] */ __RPC__in BSTR v);
        
        DECLSPEC_XFGVIRT(IHeaderFooter, get_title)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_title )( 
            __RPC__in IHeaderFooter2 * This,
            /* [out][retval] */ __RPC__deref_out_opt BSTR *p);
        
        DECLSPEC_XFGVIRT(IHeaderFooter, put_dateShort)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_dateShort )( 
            __RPC__in IHeaderFooter2 * This,
            /* [in] */ __RPC__in BSTR v);
        
        DECLSPEC_XFGVIRT(IHeaderFooter, get_dateShort)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_dateShort )( 
            __RPC__in IHeaderFooter2 * This,
            /* [out][retval] */ __RPC__deref_out_opt BSTR *p);
        
        DECLSPEC_XFGVIRT(IHeaderFooter, put_dateLong)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_dateLong )( 
            __RPC__in IHeaderFooter2 * This,
            /* [in] */ __RPC__in BSTR v);
        
        DECLSPEC_XFGVIRT(IHeaderFooter, get_dateLong)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_dateLong )( 
            __RPC__in IHeaderFooter2 * This,
            /* [out][retval] */ __RPC__deref_out_opt BSTR *p);
        
        DECLSPEC_XFGVIRT(IHeaderFooter, put_timeShort)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_timeShort )( 
            __RPC__in IHeaderFooter2 * This,
            /* [in] */ __RPC__in BSTR v);
        
        DECLSPEC_XFGVIRT(IHeaderFooter, get_timeShort)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_timeShort )( 
            __RPC__in IHeaderFooter2 * This,
            /* [out][retval] */ __RPC__deref_out_opt BSTR *p);
        
        DECLSPEC_XFGVIRT(IHeaderFooter, put_timeLong)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_timeLong )( 
            __RPC__in IHeaderFooter2 * This,
            /* [in] */ __RPC__in BSTR v);
        
        DECLSPEC_XFGVIRT(IHeaderFooter, get_timeLong)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_timeLong )( 
            __RPC__in IHeaderFooter2 * This,
            /* [out][retval] */ __RPC__deref_out_opt BSTR *p);
        
        DECLSPEC_XFGVIRT(IHeaderFooter2, put_font)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_font )( 
            __RPC__in IHeaderFooter2 * This,
            /* [in] */ __RPC__in BSTR v);
        
        DECLSPEC_XFGVIRT(IHeaderFooter2, get_font)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_font )( 
            __RPC__in IHeaderFooter2 * This,
            /* [out][retval] */ __RPC__deref_out_opt BSTR *p);
        
        END_INTERFACE
    } IHeaderFooter2Vtbl;

    interface IHeaderFooter2
    {
        CONST_VTBL struct IHeaderFooter2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IHeaderFooter2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IHeaderFooter2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IHeaderFooter2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IHeaderFooter2_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IHeaderFooter2_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IHeaderFooter2_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IHeaderFooter2_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IHeaderFooter2_get_htmlHead(This,p)	\
    ( (This)->lpVtbl -> get_htmlHead(This,p) ) 

#define IHeaderFooter2_get_htmlFoot(This,p)	\
    ( (This)->lpVtbl -> get_htmlFoot(This,p) ) 

#define IHeaderFooter2_put_textHead(This,v)	\
    ( (This)->lpVtbl -> put_textHead(This,v) ) 

#define IHeaderFooter2_get_textHead(This,p)	\
    ( (This)->lpVtbl -> get_textHead(This,p) ) 

#define IHeaderFooter2_put_textFoot(This,v)	\
    ( (This)->lpVtbl -> put_textFoot(This,v) ) 

#define IHeaderFooter2_get_textFoot(This,p)	\
    ( (This)->lpVtbl -> get_textFoot(This,p) ) 

#define IHeaderFooter2_put_page(This,v)	\
    ( (This)->lpVtbl -> put_page(This,v) ) 

#define IHeaderFooter2_get_page(This,p)	\
    ( (This)->lpVtbl -> get_page(This,p) ) 

#define IHeaderFooter2_put_pageTotal(This,v)	\
    ( (This)->lpVtbl -> put_pageTotal(This,v) ) 

#define IHeaderFooter2_get_pageTotal(This,p)	\
    ( (This)->lpVtbl -> get_pageTotal(This,p) ) 

#define IHeaderFooter2_put_URL(This,v)	\
    ( (This)->lpVtbl -> put_URL(This,v) ) 

#define IHeaderFooter2_get_URL(This,p)	\
    ( (This)->lpVtbl -> get_URL(This,p) ) 

#define IHeaderFooter2_put_title(This,v)	\
    ( (This)->lpVtbl -> put_title(This,v) ) 

#define IHeaderFooter2_get_title(This,p)	\
    ( (This)->lpVtbl -> get_title(This,p) ) 

#define IHeaderFooter2_put_dateShort(This,v)	\
    ( (This)->lpVtbl -> put_dateShort(This,v) ) 

#define IHeaderFooter2_get_dateShort(This,p)	\
    ( (This)->lpVtbl -> get_dateShort(This,p) ) 

#define IHeaderFooter2_put_dateLong(This,v)	\
    ( (This)->lpVtbl -> put_dateLong(This,v) ) 

#define IHeaderFooter2_get_dateLong(This,p)	\
    ( (This)->lpVtbl -> get_dateLong(This,p) ) 

#define IHeaderFooter2_put_timeShort(This,v)	\
    ( (This)->lpVtbl -> put_timeShort(This,v) ) 

#define IHeaderFooter2_get_timeShort(This,p)	\
    ( (This)->lpVtbl -> get_timeShort(This,p) ) 

#define IHeaderFooter2_put_timeLong(This,v)	\
    ( (This)->lpVtbl -> put_timeLong(This,v) ) 

#define IHeaderFooter2_get_timeLong(This,p)	\
    ( (This)->lpVtbl -> get_timeLong(This,p) ) 


#define IHeaderFooter2_put_font(This,v)	\
    ( (This)->lpVtbl -> put_font(This,v) ) 

#define IHeaderFooter2_get_font(This,p)	\
    ( (This)->lpVtbl -> get_font(This,p) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IHeaderFooter2_INTERFACE_DEFINED__ */



#ifndef __IEXTagLib_LIBRARY_DEFINED__
#define __IEXTagLib_LIBRARY_DEFINED__

/* library IEXTagLib */
/* [helpstring][version][uuid] */ 


EXTERN_C const IID LIBID_IEXTagLib;

EXTERN_C const CLSID CLSID_PeerFactory;

#ifdef __cplusplus

class DECLSPEC_UUID("3050F4CF-98B5-11CF-BB82-00AA00BDCE0B")
PeerFactory;
#endif

EXTERN_C const CLSID CLSID_IntelliForms;

#ifdef __cplusplus

class DECLSPEC_UUID("613AB92E-16BF-11d2-BCA5-00C04FD929DB")
IntelliForms;
#endif

EXTERN_C const CLSID CLSID_HomePage;

#ifdef __cplusplus

class DECLSPEC_UUID("766BF2AE-D650-11d1-9811-00C04FC31D2E")
HomePage;
#endif

EXTERN_C const CLSID CLSID_CPersistUserData;

#ifdef __cplusplus

class DECLSPEC_UUID("3050f48e-98b5-11cf-bb82-00aa00bdce0b")
CPersistUserData;
#endif

EXTERN_C const CLSID CLSID_CPersistDataPeer;

#ifdef __cplusplus

class DECLSPEC_UUID("3050f487-98b5-11cf-bb82-00aa00bdce0b")
CPersistDataPeer;
#endif

EXTERN_C const CLSID CLSID_CPersistShortcut;

#ifdef __cplusplus

class DECLSPEC_UUID("3050f4c6-98b5-11cf-bb82-00aa00bdce0b")
CPersistShortcut;
#endif

EXTERN_C const CLSID CLSID_CPersistHistory;

#ifdef __cplusplus

class DECLSPEC_UUID("3050f4c8-98b5-11cf-bb82-00aa00bdce0b")
CPersistHistory;
#endif

EXTERN_C const CLSID CLSID_CPersistSnapshot;

#ifdef __cplusplus

class DECLSPEC_UUID("3050f4c9-98b5-11cf-bb82-00aa00bdce0b")
CPersistSnapshot;
#endif

EXTERN_C const CLSID CLSID_CDownloadBehavior;

#ifdef __cplusplus

class DECLSPEC_UUID("3050f5be-98b5-11cf-bb82-00aa00bdce0b")
CDownloadBehavior;
#endif

EXTERN_C const CLSID CLSID_wfolders;

#ifdef __cplusplus

class DECLSPEC_UUID("BAE31F9A-1B81-11D2-A97A-00C04F8ECB02")
wfolders;
#endif

EXTERN_C const CLSID CLSID_AnchorClick;

#ifdef __cplusplus

class DECLSPEC_UUID("13D5413C-33B9-11D2-95A7-00C04F8ECB02")
AnchorClick;
#endif

EXTERN_C const CLSID CLSID_CLayoutRect;

#ifdef __cplusplus

class DECLSPEC_UUID("3050f664-98b5-11cf-bb82-00aa00bdce0b")
CLayoutRect;
#endif

EXTERN_C const CLSID CLSID_CDeviceRect;

#ifdef __cplusplus

class DECLSPEC_UUID("3050f6d4-98b5-11cf-bb82-00aa00bdce0b")
CDeviceRect;
#endif

EXTERN_C const CLSID CLSID_CHeaderFooter;

#ifdef __cplusplus

class DECLSPEC_UUID("3050f6cd-98b5-11cf-bb82-00aa00bdce0b")
CHeaderFooter;
#endif
#endif /* __IEXTagLib_LIBRARY_DEFINED__ */

/* interface __MIDL_itf_iextag_0000_0016 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_iextag_0000_0016_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_iextag_0000_0016_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

unsigned long             __RPC_USER  BSTR_UserSize(     __RPC__in unsigned long *, unsigned long            , __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserMarshal(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out BSTR * ); 
void                      __RPC_USER  BSTR_UserFree(     __RPC__in unsigned long *, __RPC__in BSTR * ); 

unsigned long             __RPC_USER  VARIANT_UserSize(     __RPC__in unsigned long *, unsigned long            , __RPC__in VARIANT * ); 
unsigned char * __RPC_USER  VARIANT_UserMarshal(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in VARIANT * ); 
unsigned char * __RPC_USER  VARIANT_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out VARIANT * ); 
void                      __RPC_USER  VARIANT_UserFree(     __RPC__in unsigned long *, __RPC__in VARIANT * ); 

unsigned long             __RPC_USER  BSTR_UserSize64(     __RPC__in unsigned long *, unsigned long            , __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserMarshal64(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out BSTR * ); 
void                      __RPC_USER  BSTR_UserFree64(     __RPC__in unsigned long *, __RPC__in BSTR * ); 

unsigned long             __RPC_USER  VARIANT_UserSize64(     __RPC__in unsigned long *, unsigned long            , __RPC__in VARIANT * ); 
unsigned char * __RPC_USER  VARIANT_UserMarshal64(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in VARIANT * ); 
unsigned char * __RPC_USER  VARIANT_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out VARIANT * ); 
void                      __RPC_USER  VARIANT_UserFree64(     __RPC__in unsigned long *, __RPC__in VARIANT * ); 

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


