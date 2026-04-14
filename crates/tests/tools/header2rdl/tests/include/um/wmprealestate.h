

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

#ifndef __wmprealestatepri_h__
#define __wmprealestatepri_h__

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

#ifndef __IWMPNodeRealEstate_FWD_DEFINED__
#define __IWMPNodeRealEstate_FWD_DEFINED__
typedef interface IWMPNodeRealEstate IWMPNodeRealEstate;

#endif 	/* __IWMPNodeRealEstate_FWD_DEFINED__ */


#ifndef __IWMPNodeRealEstateHost_FWD_DEFINED__
#define __IWMPNodeRealEstateHost_FWD_DEFINED__
typedef interface IWMPNodeRealEstateHost IWMPNodeRealEstateHost;

#endif 	/* __IWMPNodeRealEstateHost_FWD_DEFINED__ */


#ifndef __IWMPNodeWindowed_FWD_DEFINED__
#define __IWMPNodeWindowed_FWD_DEFINED__
typedef interface IWMPNodeWindowed IWMPNodeWindowed;

#endif 	/* __IWMPNodeWindowed_FWD_DEFINED__ */


#ifndef __IWMPNodeWindowedHost_FWD_DEFINED__
#define __IWMPNodeWindowedHost_FWD_DEFINED__
typedef interface IWMPNodeWindowedHost IWMPNodeWindowedHost;

#endif 	/* __IWMPNodeWindowedHost_FWD_DEFINED__ */


#ifndef __IWMPWindowMessageSink_FWD_DEFINED__
#define __IWMPWindowMessageSink_FWD_DEFINED__
typedef interface IWMPWindowMessageSink IWMPWindowMessageSink;

#endif 	/* __IWMPWindowMessageSink_FWD_DEFINED__ */


#ifndef __IWMPNodeWindowless_FWD_DEFINED__
#define __IWMPNodeWindowless_FWD_DEFINED__
typedef interface IWMPNodeWindowless IWMPNodeWindowless;

#endif 	/* __IWMPNodeWindowless_FWD_DEFINED__ */


#ifndef __IWMPNodeWindowlessHost_FWD_DEFINED__
#define __IWMPNodeWindowlessHost_FWD_DEFINED__
typedef interface IWMPNodeWindowlessHost IWMPNodeWindowlessHost;

#endif 	/* __IWMPNodeWindowlessHost_FWD_DEFINED__ */


#ifndef __IWMPVideoRenderConfig_FWD_DEFINED__
#define __IWMPVideoRenderConfig_FWD_DEFINED__
typedef interface IWMPVideoRenderConfig IWMPVideoRenderConfig;

#endif 	/* __IWMPVideoRenderConfig_FWD_DEFINED__ */


#ifndef __IWMPAudioRenderConfig_FWD_DEFINED__
#define __IWMPAudioRenderConfig_FWD_DEFINED__
typedef interface IWMPAudioRenderConfig IWMPAudioRenderConfig;

#endif 	/* __IWMPAudioRenderConfig_FWD_DEFINED__ */


#ifndef __IWMPRenderConfig_FWD_DEFINED__
#define __IWMPRenderConfig_FWD_DEFINED__
typedef interface IWMPRenderConfig IWMPRenderConfig;

#endif 	/* __IWMPRenderConfig_FWD_DEFINED__ */


/* header files for imported files */
#include "unknwn.h"
#include "ocidl.h"
#include "mfidl.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_wmprealestatepri_0000_0000 */
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


extern RPC_IF_HANDLE __MIDL_itf_wmprealestatepri_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_wmprealestatepri_0000_0000_v0_0_s_ifspec;

#ifndef __IWMPNodeRealEstate_INTERFACE_DEFINED__
#define __IWMPNodeRealEstate_INTERFACE_DEFINED__

/* interface IWMPNodeRealEstate */
/* [oleautomation][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IWMPNodeRealEstate;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("42751198-5A50-4460-BCB4-709F8BDC8E59")
    IWMPNodeRealEstate : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetDesiredSize( 
            /* [out] */ LPSIZE pSize) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetRects( 
            /* [in] */ const RECT *pSrc,
            /* [in] */ const RECT *pDest,
            /* [in] */ const RECT *pClip) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRects( 
            /* [out] */ RECT *pSrc,
            /* [out] */ RECT *pDest,
            /* [out] */ RECT *pClip) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetWindowless( 
            /* [in] */ BOOL fWindowless) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetWindowless( 
            /* [out] */ BOOL *pfWindowless) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetFullScreen( 
            /* [in] */ BOOL fFullScreen) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetFullScreen( 
            /* [out] */ BOOL *pfFullScreen) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWMPNodeRealEstateVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IWMPNodeRealEstate * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IWMPNodeRealEstate * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IWMPNodeRealEstate * This);
        
        DECLSPEC_XFGVIRT(IWMPNodeRealEstate, GetDesiredSize)
        HRESULT ( STDMETHODCALLTYPE *GetDesiredSize )( 
            IWMPNodeRealEstate * This,
            /* [out] */ LPSIZE pSize);
        
        DECLSPEC_XFGVIRT(IWMPNodeRealEstate, SetRects)
        HRESULT ( STDMETHODCALLTYPE *SetRects )( 
            IWMPNodeRealEstate * This,
            /* [in] */ const RECT *pSrc,
            /* [in] */ const RECT *pDest,
            /* [in] */ const RECT *pClip);
        
        DECLSPEC_XFGVIRT(IWMPNodeRealEstate, GetRects)
        HRESULT ( STDMETHODCALLTYPE *GetRects )( 
            IWMPNodeRealEstate * This,
            /* [out] */ RECT *pSrc,
            /* [out] */ RECT *pDest,
            /* [out] */ RECT *pClip);
        
        DECLSPEC_XFGVIRT(IWMPNodeRealEstate, SetWindowless)
        HRESULT ( STDMETHODCALLTYPE *SetWindowless )( 
            IWMPNodeRealEstate * This,
            /* [in] */ BOOL fWindowless);
        
        DECLSPEC_XFGVIRT(IWMPNodeRealEstate, GetWindowless)
        HRESULT ( STDMETHODCALLTYPE *GetWindowless )( 
            IWMPNodeRealEstate * This,
            /* [out] */ BOOL *pfWindowless);
        
        DECLSPEC_XFGVIRT(IWMPNodeRealEstate, SetFullScreen)
        HRESULT ( STDMETHODCALLTYPE *SetFullScreen )( 
            IWMPNodeRealEstate * This,
            /* [in] */ BOOL fFullScreen);
        
        DECLSPEC_XFGVIRT(IWMPNodeRealEstate, GetFullScreen)
        HRESULT ( STDMETHODCALLTYPE *GetFullScreen )( 
            IWMPNodeRealEstate * This,
            /* [out] */ BOOL *pfFullScreen);
        
        END_INTERFACE
    } IWMPNodeRealEstateVtbl;

    interface IWMPNodeRealEstate
    {
        CONST_VTBL struct IWMPNodeRealEstateVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWMPNodeRealEstate_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWMPNodeRealEstate_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWMPNodeRealEstate_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWMPNodeRealEstate_GetDesiredSize(This,pSize)	\
    ( (This)->lpVtbl -> GetDesiredSize(This,pSize) ) 

#define IWMPNodeRealEstate_SetRects(This,pSrc,pDest,pClip)	\
    ( (This)->lpVtbl -> SetRects(This,pSrc,pDest,pClip) ) 

#define IWMPNodeRealEstate_GetRects(This,pSrc,pDest,pClip)	\
    ( (This)->lpVtbl -> GetRects(This,pSrc,pDest,pClip) ) 

#define IWMPNodeRealEstate_SetWindowless(This,fWindowless)	\
    ( (This)->lpVtbl -> SetWindowless(This,fWindowless) ) 

#define IWMPNodeRealEstate_GetWindowless(This,pfWindowless)	\
    ( (This)->lpVtbl -> GetWindowless(This,pfWindowless) ) 

#define IWMPNodeRealEstate_SetFullScreen(This,fFullScreen)	\
    ( (This)->lpVtbl -> SetFullScreen(This,fFullScreen) ) 

#define IWMPNodeRealEstate_GetFullScreen(This,pfFullScreen)	\
    ( (This)->lpVtbl -> GetFullScreen(This,pfFullScreen) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWMPNodeRealEstate_INTERFACE_DEFINED__ */


#ifndef __IWMPNodeRealEstateHost_INTERFACE_DEFINED__
#define __IWMPNodeRealEstateHost_INTERFACE_DEFINED__

/* interface IWMPNodeRealEstateHost */
/* [oleautomation][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IWMPNodeRealEstateHost;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("1491087D-2C6B-44c8-B019-B3C929D2ADA9")
    IWMPNodeRealEstateHost : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE OnDesiredSizeChange( 
            /* [in] */ LPSIZE pSize) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnFullScreenTransition( 
            /* [in] */ BOOL fFullScreen) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWMPNodeRealEstateHostVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IWMPNodeRealEstateHost * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IWMPNodeRealEstateHost * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IWMPNodeRealEstateHost * This);
        
        DECLSPEC_XFGVIRT(IWMPNodeRealEstateHost, OnDesiredSizeChange)
        HRESULT ( STDMETHODCALLTYPE *OnDesiredSizeChange )( 
            IWMPNodeRealEstateHost * This,
            /* [in] */ LPSIZE pSize);
        
        DECLSPEC_XFGVIRT(IWMPNodeRealEstateHost, OnFullScreenTransition)
        HRESULT ( STDMETHODCALLTYPE *OnFullScreenTransition )( 
            IWMPNodeRealEstateHost * This,
            /* [in] */ BOOL fFullScreen);
        
        END_INTERFACE
    } IWMPNodeRealEstateHostVtbl;

    interface IWMPNodeRealEstateHost
    {
        CONST_VTBL struct IWMPNodeRealEstateHostVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWMPNodeRealEstateHost_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWMPNodeRealEstateHost_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWMPNodeRealEstateHost_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWMPNodeRealEstateHost_OnDesiredSizeChange(This,pSize)	\
    ( (This)->lpVtbl -> OnDesiredSizeChange(This,pSize) ) 

#define IWMPNodeRealEstateHost_OnFullScreenTransition(This,fFullScreen)	\
    ( (This)->lpVtbl -> OnFullScreenTransition(This,fFullScreen) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWMPNodeRealEstateHost_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_wmprealestatepri_0000_0002 */
/* [local] */ 

typedef LONG_PTR OLE_HWND;



extern RPC_IF_HANDLE __MIDL_itf_wmprealestatepri_0000_0002_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_wmprealestatepri_0000_0002_v0_0_s_ifspec;

#ifndef __IWMPNodeWindowed_INTERFACE_DEFINED__
#define __IWMPNodeWindowed_INTERFACE_DEFINED__

/* interface IWMPNodeWindowed */
/* [oleautomation][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IWMPNodeWindowed;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("96740BFA-C56A-45d1-A3A4-762914D4ADE9")
    IWMPNodeWindowed : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SetOwnerWindow( 
            /* [in] */ OLE_HWND hwnd) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetOwnerWindow( 
            /* [out] */ OLE_HWND *phwnd) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWMPNodeWindowedVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IWMPNodeWindowed * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IWMPNodeWindowed * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IWMPNodeWindowed * This);
        
        DECLSPEC_XFGVIRT(IWMPNodeWindowed, SetOwnerWindow)
        HRESULT ( STDMETHODCALLTYPE *SetOwnerWindow )( 
            IWMPNodeWindowed * This,
            /* [in] */ OLE_HWND hwnd);
        
        DECLSPEC_XFGVIRT(IWMPNodeWindowed, GetOwnerWindow)
        HRESULT ( STDMETHODCALLTYPE *GetOwnerWindow )( 
            IWMPNodeWindowed * This,
            /* [out] */ OLE_HWND *phwnd);
        
        END_INTERFACE
    } IWMPNodeWindowedVtbl;

    interface IWMPNodeWindowed
    {
        CONST_VTBL struct IWMPNodeWindowedVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWMPNodeWindowed_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWMPNodeWindowed_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWMPNodeWindowed_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWMPNodeWindowed_SetOwnerWindow(This,hwnd)	\
    ( (This)->lpVtbl -> SetOwnerWindow(This,hwnd) ) 

#define IWMPNodeWindowed_GetOwnerWindow(This,phwnd)	\
    ( (This)->lpVtbl -> GetOwnerWindow(This,phwnd) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWMPNodeWindowed_INTERFACE_DEFINED__ */


#ifndef __IWMPNodeWindowedHost_INTERFACE_DEFINED__
#define __IWMPNodeWindowedHost_INTERFACE_DEFINED__

/* interface IWMPNodeWindowedHost */
/* [oleautomation][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IWMPNodeWindowedHost;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("A300415A-54AA-4081-ADBF-3B13610D8958")
    IWMPNodeWindowedHost : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE OnWindowMessageFromRenderer( 
            /* [in] */ UINT uMsg,
            /* [in] */ WPARAM wparam,
            /* [in] */ LPARAM lparam,
            /* [out] */ LRESULT *plRet,
            /* [out] */ BOOL *pfHandled) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWMPNodeWindowedHostVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IWMPNodeWindowedHost * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IWMPNodeWindowedHost * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IWMPNodeWindowedHost * This);
        
        DECLSPEC_XFGVIRT(IWMPNodeWindowedHost, OnWindowMessageFromRenderer)
        HRESULT ( STDMETHODCALLTYPE *OnWindowMessageFromRenderer )( 
            IWMPNodeWindowedHost * This,
            /* [in] */ UINT uMsg,
            /* [in] */ WPARAM wparam,
            /* [in] */ LPARAM lparam,
            /* [out] */ LRESULT *plRet,
            /* [out] */ BOOL *pfHandled);
        
        END_INTERFACE
    } IWMPNodeWindowedHostVtbl;

    interface IWMPNodeWindowedHost
    {
        CONST_VTBL struct IWMPNodeWindowedHostVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWMPNodeWindowedHost_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWMPNodeWindowedHost_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWMPNodeWindowedHost_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWMPNodeWindowedHost_OnWindowMessageFromRenderer(This,uMsg,wparam,lparam,plRet,pfHandled)	\
    ( (This)->lpVtbl -> OnWindowMessageFromRenderer(This,uMsg,wparam,lparam,plRet,pfHandled) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWMPNodeWindowedHost_INTERFACE_DEFINED__ */


#ifndef __IWMPWindowMessageSink_INTERFACE_DEFINED__
#define __IWMPWindowMessageSink_INTERFACE_DEFINED__

/* interface IWMPWindowMessageSink */
/* [oleautomation][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IWMPWindowMessageSink;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("3A0DAA30-908D-4789-BA87-AED879B5C49B")
    IWMPWindowMessageSink : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE OnWindowMessage( 
            UINT uMsg,
            WPARAM wparam,
            LPARAM lparam,
            LRESULT *plRet,
            BOOL *pfHandled) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWMPWindowMessageSinkVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IWMPWindowMessageSink * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IWMPWindowMessageSink * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IWMPWindowMessageSink * This);
        
        DECLSPEC_XFGVIRT(IWMPWindowMessageSink, OnWindowMessage)
        HRESULT ( STDMETHODCALLTYPE *OnWindowMessage )( 
            IWMPWindowMessageSink * This,
            UINT uMsg,
            WPARAM wparam,
            LPARAM lparam,
            LRESULT *plRet,
            BOOL *pfHandled);
        
        END_INTERFACE
    } IWMPWindowMessageSinkVtbl;

    interface IWMPWindowMessageSink
    {
        CONST_VTBL struct IWMPWindowMessageSinkVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWMPWindowMessageSink_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWMPWindowMessageSink_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWMPWindowMessageSink_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWMPWindowMessageSink_OnWindowMessage(This,uMsg,wparam,lparam,plRet,pfHandled)	\
    ( (This)->lpVtbl -> OnWindowMessage(This,uMsg,wparam,lparam,plRet,pfHandled) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWMPWindowMessageSink_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_wmprealestatepri_0000_0005 */
/* [local] */ 

typedef LONG_PTR OLE_HDC;



extern RPC_IF_HANDLE __MIDL_itf_wmprealestatepri_0000_0005_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_wmprealestatepri_0000_0005_v0_0_s_ifspec;

#ifndef __IWMPNodeWindowless_INTERFACE_DEFINED__
#define __IWMPNodeWindowless_INTERFACE_DEFINED__

/* interface IWMPNodeWindowless */
/* [oleautomation][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IWMPNodeWindowless;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("9B9199AD-780C-4eda-B816-261EBA5D1575")
    IWMPNodeWindowless : public IWMPWindowMessageSink
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE OnDraw( 
            /* [in] */ OLE_HDC hdc,
            /* [in] */ const RECT *prcDraw) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWMPNodeWindowlessVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IWMPNodeWindowless * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IWMPNodeWindowless * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IWMPNodeWindowless * This);
        
        DECLSPEC_XFGVIRT(IWMPWindowMessageSink, OnWindowMessage)
        HRESULT ( STDMETHODCALLTYPE *OnWindowMessage )( 
            IWMPNodeWindowless * This,
            UINT uMsg,
            WPARAM wparam,
            LPARAM lparam,
            LRESULT *plRet,
            BOOL *pfHandled);
        
        DECLSPEC_XFGVIRT(IWMPNodeWindowless, OnDraw)
        HRESULT ( STDMETHODCALLTYPE *OnDraw )( 
            IWMPNodeWindowless * This,
            /* [in] */ OLE_HDC hdc,
            /* [in] */ const RECT *prcDraw);
        
        END_INTERFACE
    } IWMPNodeWindowlessVtbl;

    interface IWMPNodeWindowless
    {
        CONST_VTBL struct IWMPNodeWindowlessVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWMPNodeWindowless_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWMPNodeWindowless_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWMPNodeWindowless_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWMPNodeWindowless_OnWindowMessage(This,uMsg,wparam,lparam,plRet,pfHandled)	\
    ( (This)->lpVtbl -> OnWindowMessage(This,uMsg,wparam,lparam,plRet,pfHandled) ) 


#define IWMPNodeWindowless_OnDraw(This,hdc,prcDraw)	\
    ( (This)->lpVtbl -> OnDraw(This,hdc,prcDraw) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWMPNodeWindowless_INTERFACE_DEFINED__ */


#ifndef __IWMPNodeWindowlessHost_INTERFACE_DEFINED__
#define __IWMPNodeWindowlessHost_INTERFACE_DEFINED__

/* interface IWMPNodeWindowlessHost */
/* [oleautomation][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IWMPNodeWindowlessHost;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("BE7017C6-CE34-4901-8106-770381AA6E3E")
    IWMPNodeWindowlessHost : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE InvalidateRect( 
            /* [in] */ const RECT *prc,
            /* [in] */ BOOL fErase) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWMPNodeWindowlessHostVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IWMPNodeWindowlessHost * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IWMPNodeWindowlessHost * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IWMPNodeWindowlessHost * This);
        
        DECLSPEC_XFGVIRT(IWMPNodeWindowlessHost, InvalidateRect)
        HRESULT ( STDMETHODCALLTYPE *InvalidateRect )( 
            IWMPNodeWindowlessHost * This,
            /* [in] */ const RECT *prc,
            /* [in] */ BOOL fErase);
        
        END_INTERFACE
    } IWMPNodeWindowlessHostVtbl;

    interface IWMPNodeWindowlessHost
    {
        CONST_VTBL struct IWMPNodeWindowlessHostVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWMPNodeWindowlessHost_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWMPNodeWindowlessHost_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWMPNodeWindowlessHost_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWMPNodeWindowlessHost_InvalidateRect(This,prc,fErase)	\
    ( (This)->lpVtbl -> InvalidateRect(This,prc,fErase) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWMPNodeWindowlessHost_INTERFACE_DEFINED__ */


#ifndef __IWMPVideoRenderConfig_INTERFACE_DEFINED__
#define __IWMPVideoRenderConfig_INTERFACE_DEFINED__

/* interface IWMPVideoRenderConfig */
/* [oleautomation][unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IWMPVideoRenderConfig;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("6D6CF803-1EC0-4c8d-B3CA-F18E27282074")
    IWMPVideoRenderConfig : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE put_presenterActivate( 
            /* [in] */ IMFActivate *pActivate) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWMPVideoRenderConfigVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IWMPVideoRenderConfig * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IWMPVideoRenderConfig * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IWMPVideoRenderConfig * This);
        
        DECLSPEC_XFGVIRT(IWMPVideoRenderConfig, put_presenterActivate)
        HRESULT ( STDMETHODCALLTYPE *put_presenterActivate )( 
            IWMPVideoRenderConfig * This,
            /* [in] */ IMFActivate *pActivate);
        
        END_INTERFACE
    } IWMPVideoRenderConfigVtbl;

    interface IWMPVideoRenderConfig
    {
        CONST_VTBL struct IWMPVideoRenderConfigVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWMPVideoRenderConfig_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWMPVideoRenderConfig_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWMPVideoRenderConfig_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWMPVideoRenderConfig_put_presenterActivate(This,pActivate)	\
    ( (This)->lpVtbl -> put_presenterActivate(This,pActivate) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWMPVideoRenderConfig_INTERFACE_DEFINED__ */


#ifndef __IWMPAudioRenderConfig_INTERFACE_DEFINED__
#define __IWMPAudioRenderConfig_INTERFACE_DEFINED__

/* interface IWMPAudioRenderConfig */
/* [oleautomation][unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IWMPAudioRenderConfig;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("e79c6349-5997-4ce4-917c-22a3391ec564")
    IWMPAudioRenderConfig : public IUnknown
    {
    public:
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_audioOutputDevice( 
            /* [retval][out] */ BSTR *pbstrOutputDevice) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_audioOutputDevice( 
            /* [unique][in] */ BSTR bstrOutputDevice) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWMPAudioRenderConfigVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IWMPAudioRenderConfig * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IWMPAudioRenderConfig * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IWMPAudioRenderConfig * This);
        
        DECLSPEC_XFGVIRT(IWMPAudioRenderConfig, get_audioOutputDevice)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_audioOutputDevice )( 
            IWMPAudioRenderConfig * This,
            /* [retval][out] */ BSTR *pbstrOutputDevice);
        
        DECLSPEC_XFGVIRT(IWMPAudioRenderConfig, put_audioOutputDevice)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_audioOutputDevice )( 
            IWMPAudioRenderConfig * This,
            /* [unique][in] */ BSTR bstrOutputDevice);
        
        END_INTERFACE
    } IWMPAudioRenderConfigVtbl;

    interface IWMPAudioRenderConfig
    {
        CONST_VTBL struct IWMPAudioRenderConfigVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWMPAudioRenderConfig_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWMPAudioRenderConfig_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWMPAudioRenderConfig_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWMPAudioRenderConfig_get_audioOutputDevice(This,pbstrOutputDevice)	\
    ( (This)->lpVtbl -> get_audioOutputDevice(This,pbstrOutputDevice) ) 

#define IWMPAudioRenderConfig_put_audioOutputDevice(This,bstrOutputDevice)	\
    ( (This)->lpVtbl -> put_audioOutputDevice(This,bstrOutputDevice) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWMPAudioRenderConfig_INTERFACE_DEFINED__ */


#ifndef __IWMPRenderConfig_INTERFACE_DEFINED__
#define __IWMPRenderConfig_INTERFACE_DEFINED__

/* interface IWMPRenderConfig */
/* [oleautomation][unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IWMPRenderConfig;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("959506C1-0314-4EC5-9E61-8528DB5E5478")
    IWMPRenderConfig : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE put_inProcOnly( 
            /* [in] */ BOOL fInProc) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_inProcOnly( 
            /* [out] */ BOOL *pfInProc) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWMPRenderConfigVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IWMPRenderConfig * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IWMPRenderConfig * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IWMPRenderConfig * This);
        
        DECLSPEC_XFGVIRT(IWMPRenderConfig, put_inProcOnly)
        HRESULT ( STDMETHODCALLTYPE *put_inProcOnly )( 
            IWMPRenderConfig * This,
            /* [in] */ BOOL fInProc);
        
        DECLSPEC_XFGVIRT(IWMPRenderConfig, get_inProcOnly)
        HRESULT ( STDMETHODCALLTYPE *get_inProcOnly )( 
            IWMPRenderConfig * This,
            /* [out] */ BOOL *pfInProc);
        
        END_INTERFACE
    } IWMPRenderConfigVtbl;

    interface IWMPRenderConfig
    {
        CONST_VTBL struct IWMPRenderConfigVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWMPRenderConfig_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWMPRenderConfig_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWMPRenderConfig_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWMPRenderConfig_put_inProcOnly(This,fInProc)	\
    ( (This)->lpVtbl -> put_inProcOnly(This,fInProc) ) 

#define IWMPRenderConfig_get_inProcOnly(This,pfInProc)	\
    ( (This)->lpVtbl -> get_inProcOnly(This,pfInProc) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWMPRenderConfig_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_wmprealestatepri_0000_0010 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_wmprealestatepri_0000_0010_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_wmprealestatepri_0000_0010_v0_0_s_ifspec;

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


