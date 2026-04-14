

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

#ifndef __thumbcache_h__
#define __thumbcache_h__

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

#ifndef __ISharedBitmap_FWD_DEFINED__
#define __ISharedBitmap_FWD_DEFINED__
typedef interface ISharedBitmap ISharedBitmap;

#endif 	/* __ISharedBitmap_FWD_DEFINED__ */


#ifndef __IThumbnailCache_FWD_DEFINED__
#define __IThumbnailCache_FWD_DEFINED__
typedef interface IThumbnailCache IThumbnailCache;

#endif 	/* __IThumbnailCache_FWD_DEFINED__ */


#ifndef __IThumbnailProvider_FWD_DEFINED__
#define __IThumbnailProvider_FWD_DEFINED__
typedef interface IThumbnailProvider IThumbnailProvider;

#endif 	/* __IThumbnailProvider_FWD_DEFINED__ */


#ifndef __IThumbnailSettings_FWD_DEFINED__
#define __IThumbnailSettings_FWD_DEFINED__
typedef interface IThumbnailSettings IThumbnailSettings;

#endif 	/* __IThumbnailSettings_FWD_DEFINED__ */


#ifndef __IThumbnailCachePrimer_FWD_DEFINED__
#define __IThumbnailCachePrimer_FWD_DEFINED__
typedef interface IThumbnailCachePrimer IThumbnailCachePrimer;

#endif 	/* __IThumbnailCachePrimer_FWD_DEFINED__ */


#ifndef __LocalThumbnailCache_FWD_DEFINED__
#define __LocalThumbnailCache_FWD_DEFINED__

#ifdef __cplusplus
typedef class LocalThumbnailCache LocalThumbnailCache;
#else
typedef struct LocalThumbnailCache LocalThumbnailCache;
#endif /* __cplusplus */

#endif 	/* __LocalThumbnailCache_FWD_DEFINED__ */


#ifndef __SharedBitmap_FWD_DEFINED__
#define __SharedBitmap_FWD_DEFINED__

#ifdef __cplusplus
typedef class SharedBitmap SharedBitmap;
#else
typedef struct SharedBitmap SharedBitmap;
#endif /* __cplusplus */

#endif 	/* __SharedBitmap_FWD_DEFINED__ */


/* header files for imported files */
#include "oaidl.h"
#include "shtypes.h"
#include "shobjidl_core.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_thumbcache_0000_0000 */
/* [local] */ 

#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
typedef /* [v1_enum] */ 
enum WTS_FLAGS
    {
        WTS_NONE	= 0,
        WTS_EXTRACT	= 0,
        WTS_INCACHEONLY	= 0x1,
        WTS_FASTEXTRACT	= 0x2,
        WTS_FORCEEXTRACTION	= 0x4,
        WTS_SLOWRECLAIM	= 0x8,
        WTS_EXTRACTDONOTCACHE	= 0x20,
        WTS_SCALETOREQUESTEDSIZE	= 0x40,
        WTS_SKIPFASTEXTRACT	= 0x80,
        WTS_EXTRACTINPROC	= 0x100,
        WTS_CROPTOSQUARE	= 0x200,
        WTS_INSTANCESURROGATE	= 0x400,
        WTS_REQUIRESURROGATE	= 0x800,
        WTS_APPSTYLE	= 0x2000,
        WTS_WIDETHUMBNAILS	= 0x4000,
        WTS_IDEALCACHESIZEONLY	= 0x8000,
        WTS_SCALEUP	= 0x10000
    } 	WTS_FLAGS;

DEFINE_ENUM_FLAG_OPERATORS(WTS_FLAGS)
typedef /* [v1_enum] */ 
enum WTS_CACHEFLAGS
    {
        WTS_DEFAULT	= 0,
        WTS_LOWQUALITY	= 0x1,
        WTS_CACHED	= 0x2
    } 	WTS_CACHEFLAGS;

DEFINE_ENUM_FLAG_OPERATORS(WTS_CACHEFLAGS)
typedef /* [v1_enum] */ 
enum WTS_CONTEXTFLAGS
    {
        WTSCF_DEFAULT	= 0,
        WTSCF_APPSTYLE	= 0x1,
        WTSCF_SQUARE	= 0x2,
        WTSCF_WIDE	= 0x4,
        WTSCF_FAST	= 0x8
    } 	WTS_CONTEXTFLAGS;

DEFINE_ENUM_FLAG_OPERATORS(WTS_CONTEXTFLAGS)
typedef /* [v1_enum] */ 
enum WTS_ALPHATYPE
    {
        WTSAT_UNKNOWN	= 0,
        WTSAT_RGB	= 1,
        WTSAT_ARGB	= 2
    } 	WTS_ALPHATYPE;

typedef struct WTS_THUMBNAILID
    {
    BYTE rgbKey[ 16 ];
    } 	WTS_THUMBNAILID;

#define WTS_E_FAILEDEXTRACTION                     MAKE_HRESULT(SEVERITY_ERROR, FACILITY_ITF, 0xB200)
#define WTS_E_EXTRACTIONTIMEDOUT                   MAKE_HRESULT(SEVERITY_ERROR, FACILITY_ITF, 0xB201)
#define WTS_E_SURROGATEUNAVAILABLE                 MAKE_HRESULT(SEVERITY_ERROR, FACILITY_ITF, 0xB202)
#define WTS_E_FASTEXTRACTIONNOTSUPPORTED           MAKE_HRESULT(SEVERITY_ERROR, FACILITY_ITF, 0xB203)
#define WTS_E_DATAFILEUNAVAILABLE                  MAKE_HRESULT(SEVERITY_ERROR, FACILITY_ITF, 0xB204)
#define WTS_E_EXTRACTIONPENDING                    MAKE_HRESULT(SEVERITY_ERROR, FACILITY_ITF, 0xB205)
#define WTS_E_EXTRACTIONBLOCKED                    MAKE_HRESULT(SEVERITY_ERROR, FACILITY_ITF, 0xB206)
#define WTS_E_NOSTORAGEPROVIDERTHUMBNAILHANDLER    MAKE_HRESULT(SEVERITY_ERROR, FACILITY_ITF, 0xB207)


extern RPC_IF_HANDLE __MIDL_itf_thumbcache_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_thumbcache_0000_0000_v0_0_s_ifspec;

#ifndef __ISharedBitmap_INTERFACE_DEFINED__
#define __ISharedBitmap_INTERFACE_DEFINED__

/* interface ISharedBitmap */
/* [object][unique][uuid] */ 


EXTERN_C const IID IID_ISharedBitmap;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("091162a4-bc96-411f-aae8-c5122cd03363")
    ISharedBitmap : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetSharedBitmap( 
            /* [out] */ __RPC__deref_out_opt HBITMAP *phbm) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetSize( 
            /* [out] */ __RPC__out SIZE *pSize) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetFormat( 
            /* [out] */ __RPC__out WTS_ALPHATYPE *pat) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE InitializeBitmap( 
            /* [in] */ __RPC__in HBITMAP hbm,
            /* [in] */ WTS_ALPHATYPE wtsAT) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Detach( 
            /* [out] */ __RPC__deref_out_opt HBITMAP *phbm) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISharedBitmapVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ISharedBitmap * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ISharedBitmap * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ISharedBitmap * This);
        
        DECLSPEC_XFGVIRT(ISharedBitmap, GetSharedBitmap)
        HRESULT ( STDMETHODCALLTYPE *GetSharedBitmap )( 
            __RPC__in ISharedBitmap * This,
            /* [out] */ __RPC__deref_out_opt HBITMAP *phbm);
        
        DECLSPEC_XFGVIRT(ISharedBitmap, GetSize)
        HRESULT ( STDMETHODCALLTYPE *GetSize )( 
            __RPC__in ISharedBitmap * This,
            /* [out] */ __RPC__out SIZE *pSize);
        
        DECLSPEC_XFGVIRT(ISharedBitmap, GetFormat)
        HRESULT ( STDMETHODCALLTYPE *GetFormat )( 
            __RPC__in ISharedBitmap * This,
            /* [out] */ __RPC__out WTS_ALPHATYPE *pat);
        
        DECLSPEC_XFGVIRT(ISharedBitmap, InitializeBitmap)
        HRESULT ( STDMETHODCALLTYPE *InitializeBitmap )( 
            __RPC__in ISharedBitmap * This,
            /* [in] */ __RPC__in HBITMAP hbm,
            /* [in] */ WTS_ALPHATYPE wtsAT);
        
        DECLSPEC_XFGVIRT(ISharedBitmap, Detach)
        HRESULT ( STDMETHODCALLTYPE *Detach )( 
            __RPC__in ISharedBitmap * This,
            /* [out] */ __RPC__deref_out_opt HBITMAP *phbm);
        
        END_INTERFACE
    } ISharedBitmapVtbl;

    interface ISharedBitmap
    {
        CONST_VTBL struct ISharedBitmapVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISharedBitmap_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISharedBitmap_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISharedBitmap_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISharedBitmap_GetSharedBitmap(This,phbm)	\
    ( (This)->lpVtbl -> GetSharedBitmap(This,phbm) ) 

#define ISharedBitmap_GetSize(This,pSize)	\
    ( (This)->lpVtbl -> GetSize(This,pSize) ) 

#define ISharedBitmap_GetFormat(This,pat)	\
    ( (This)->lpVtbl -> GetFormat(This,pat) ) 

#define ISharedBitmap_InitializeBitmap(This,hbm,wtsAT)	\
    ( (This)->lpVtbl -> InitializeBitmap(This,hbm,wtsAT) ) 

#define ISharedBitmap_Detach(This,phbm)	\
    ( (This)->lpVtbl -> Detach(This,phbm) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISharedBitmap_INTERFACE_DEFINED__ */


#ifndef __IThumbnailCache_INTERFACE_DEFINED__
#define __IThumbnailCache_INTERFACE_DEFINED__

/* interface IThumbnailCache */
/* [uuid][object] */ 


EXTERN_C const IID IID_IThumbnailCache;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("F676C15D-596A-4ce2-8234-33996F445DB1")
    IThumbnailCache : public IUnknown
    {
    public:
        virtual /* [local] */ HRESULT STDMETHODCALLTYPE GetThumbnail( 
            /* [in] */ IShellItem *pShellItem,
            /* [in] */ UINT cxyRequestedThumbSize,
            /* [in] */ WTS_FLAGS flags,
            /* [annotation][unique][out] */ 
            _Out_opt_  ISharedBitmap **ppvThumb,
            /* [annotation][unique][out] */ 
            _Out_opt_  WTS_CACHEFLAGS *pOutFlags,
            /* [annotation][unique][out] */ 
            _Out_opt_  WTS_THUMBNAILID *pThumbnailID) = 0;
        
        virtual /* [local] */ HRESULT STDMETHODCALLTYPE GetThumbnailByID( 
            /* [in] */ WTS_THUMBNAILID thumbnailID,
            /* [in] */ UINT cxyRequestedThumbSize,
            /* [annotation][unique][out] */ 
            _Out_opt_  ISharedBitmap **ppvThumb,
            /* [annotation][unique][out] */ 
            _Out_opt_  WTS_CACHEFLAGS *pOutFlags) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IThumbnailCacheVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IThumbnailCache * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IThumbnailCache * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IThumbnailCache * This);
        
        DECLSPEC_XFGVIRT(IThumbnailCache, GetThumbnail)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *GetThumbnail )( 
            IThumbnailCache * This,
            /* [in] */ IShellItem *pShellItem,
            /* [in] */ UINT cxyRequestedThumbSize,
            /* [in] */ WTS_FLAGS flags,
            /* [annotation][unique][out] */ 
            _Out_opt_  ISharedBitmap **ppvThumb,
            /* [annotation][unique][out] */ 
            _Out_opt_  WTS_CACHEFLAGS *pOutFlags,
            /* [annotation][unique][out] */ 
            _Out_opt_  WTS_THUMBNAILID *pThumbnailID);
        
        DECLSPEC_XFGVIRT(IThumbnailCache, GetThumbnailByID)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *GetThumbnailByID )( 
            IThumbnailCache * This,
            /* [in] */ WTS_THUMBNAILID thumbnailID,
            /* [in] */ UINT cxyRequestedThumbSize,
            /* [annotation][unique][out] */ 
            _Out_opt_  ISharedBitmap **ppvThumb,
            /* [annotation][unique][out] */ 
            _Out_opt_  WTS_CACHEFLAGS *pOutFlags);
        
        END_INTERFACE
    } IThumbnailCacheVtbl;

    interface IThumbnailCache
    {
        CONST_VTBL struct IThumbnailCacheVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IThumbnailCache_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IThumbnailCache_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IThumbnailCache_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IThumbnailCache_GetThumbnail(This,pShellItem,cxyRequestedThumbSize,flags,ppvThumb,pOutFlags,pThumbnailID)	\
    ( (This)->lpVtbl -> GetThumbnail(This,pShellItem,cxyRequestedThumbSize,flags,ppvThumb,pOutFlags,pThumbnailID) ) 

#define IThumbnailCache_GetThumbnailByID(This,thumbnailID,cxyRequestedThumbSize,ppvThumb,pOutFlags)	\
    ( (This)->lpVtbl -> GetThumbnailByID(This,thumbnailID,cxyRequestedThumbSize,ppvThumb,pOutFlags) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */



/* [call_as] */ HRESULT STDMETHODCALLTYPE IThumbnailCache_RemoteGetThumbnail_Proxy( 
    __RPC__in IThumbnailCache * This,
    /* [in] */ __RPC__in_opt IShellItem *pShellItem,
    /* [in] */ UINT cxyRequestedThumbSize,
    /* [in] */ WTS_FLAGS flags,
    /* [out] */ __RPC__deref_out_opt ISharedBitmap **ppvThumb,
    /* [out] */ __RPC__out WTS_CACHEFLAGS *pOutFlags,
    /* [out] */ __RPC__out WTS_THUMBNAILID *pThumbnailID);


void __RPC_STUB IThumbnailCache_RemoteGetThumbnail_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


/* [call_as] */ HRESULT STDMETHODCALLTYPE IThumbnailCache_RemoteGetThumbnailByID_Proxy( 
    __RPC__in IThumbnailCache * This,
    /* [in] */ WTS_THUMBNAILID thumbnailID,
    /* [in] */ UINT cxyRequestedThumbSize,
    /* [out] */ __RPC__deref_out_opt ISharedBitmap **ppvThumb,
    /* [out] */ __RPC__out WTS_CACHEFLAGS *pOutFlags);


void __RPC_STUB IThumbnailCache_RemoteGetThumbnailByID_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);



#endif 	/* __IThumbnailCache_INTERFACE_DEFINED__ */


#ifndef __IThumbnailProvider_INTERFACE_DEFINED__
#define __IThumbnailProvider_INTERFACE_DEFINED__

/* interface IThumbnailProvider */
/* [object][uuid] */ 


EXTERN_C const IID IID_IThumbnailProvider;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("e357fccd-a995-4576-b01f-234630154e96")
    IThumbnailProvider : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetThumbnail( 
            /* [in] */ UINT cx,
            /* [out] */ __RPC__deref_out_opt HBITMAP *phbmp,
            /* [out] */ __RPC__out WTS_ALPHATYPE *pdwAlpha) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IThumbnailProviderVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IThumbnailProvider * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IThumbnailProvider * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IThumbnailProvider * This);
        
        DECLSPEC_XFGVIRT(IThumbnailProvider, GetThumbnail)
        HRESULT ( STDMETHODCALLTYPE *GetThumbnail )( 
            __RPC__in IThumbnailProvider * This,
            /* [in] */ UINT cx,
            /* [out] */ __RPC__deref_out_opt HBITMAP *phbmp,
            /* [out] */ __RPC__out WTS_ALPHATYPE *pdwAlpha);
        
        END_INTERFACE
    } IThumbnailProviderVtbl;

    interface IThumbnailProvider
    {
        CONST_VTBL struct IThumbnailProviderVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IThumbnailProvider_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IThumbnailProvider_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IThumbnailProvider_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IThumbnailProvider_GetThumbnail(This,cx,phbmp,pdwAlpha)	\
    ( (This)->lpVtbl -> GetThumbnail(This,cx,phbmp,pdwAlpha) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IThumbnailProvider_INTERFACE_DEFINED__ */


#ifndef __IThumbnailSettings_INTERFACE_DEFINED__
#define __IThumbnailSettings_INTERFACE_DEFINED__

/* interface IThumbnailSettings */
/* [object][uuid] */ 


EXTERN_C const IID IID_IThumbnailSettings;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("F4376F00-BEF5-4d45-80F3-1E023BBF1209")
    IThumbnailSettings : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SetContext( 
            /* [in] */ WTS_CONTEXTFLAGS dwContext) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IThumbnailSettingsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IThumbnailSettings * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IThumbnailSettings * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IThumbnailSettings * This);
        
        DECLSPEC_XFGVIRT(IThumbnailSettings, SetContext)
        HRESULT ( STDMETHODCALLTYPE *SetContext )( 
            __RPC__in IThumbnailSettings * This,
            /* [in] */ WTS_CONTEXTFLAGS dwContext);
        
        END_INTERFACE
    } IThumbnailSettingsVtbl;

    interface IThumbnailSettings
    {
        CONST_VTBL struct IThumbnailSettingsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IThumbnailSettings_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IThumbnailSettings_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IThumbnailSettings_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IThumbnailSettings_SetContext(This,dwContext)	\
    ( (This)->lpVtbl -> SetContext(This,dwContext) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IThumbnailSettings_INTERFACE_DEFINED__ */


#ifndef __IThumbnailCachePrimer_INTERFACE_DEFINED__
#define __IThumbnailCachePrimer_INTERFACE_DEFINED__

/* interface IThumbnailCachePrimer */
/* [unique][object][uuid] */ 


EXTERN_C const IID IID_IThumbnailCachePrimer;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("0f03f8fe-2b26-46f0-965a-212aa8d66b76")
    IThumbnailCachePrimer : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE PageInThumbnail( 
            /* [in] */ __RPC__in_opt IShellItem *psi,
            /* [in] */ WTS_FLAGS wtsFlags,
            /* [in] */ UINT cxyRequestedThumbSize) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IThumbnailCachePrimerVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IThumbnailCachePrimer * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IThumbnailCachePrimer * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IThumbnailCachePrimer * This);
        
        DECLSPEC_XFGVIRT(IThumbnailCachePrimer, PageInThumbnail)
        HRESULT ( STDMETHODCALLTYPE *PageInThumbnail )( 
            __RPC__in IThumbnailCachePrimer * This,
            /* [in] */ __RPC__in_opt IShellItem *psi,
            /* [in] */ WTS_FLAGS wtsFlags,
            /* [in] */ UINT cxyRequestedThumbSize);
        
        END_INTERFACE
    } IThumbnailCachePrimerVtbl;

    interface IThumbnailCachePrimer
    {
        CONST_VTBL struct IThumbnailCachePrimerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IThumbnailCachePrimer_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IThumbnailCachePrimer_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IThumbnailCachePrimer_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IThumbnailCachePrimer_PageInThumbnail(This,psi,wtsFlags,cxyRequestedThumbSize)	\
    ( (This)->lpVtbl -> PageInThumbnail(This,psi,wtsFlags,cxyRequestedThumbSize) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IThumbnailCachePrimer_INTERFACE_DEFINED__ */



#ifndef __ThumbCacheLib_LIBRARY_DEFINED__
#define __ThumbCacheLib_LIBRARY_DEFINED__

/* library ThumbCacheLib */
/* [uuid] */ 


EXTERN_C const IID LIBID_ThumbCacheLib;

EXTERN_C const CLSID CLSID_LocalThumbnailCache;

#ifdef __cplusplus

class DECLSPEC_UUID("50EF4544-AC9F-4A8E-B21B-8A26180DB13F")
LocalThumbnailCache;
#endif

EXTERN_C const CLSID CLSID_SharedBitmap;

#ifdef __cplusplus

class DECLSPEC_UUID("4db26476-6787-4046-b836-e8412a9e8a27")
SharedBitmap;
#endif
#endif /* __ThumbCacheLib_LIBRARY_DEFINED__ */

/* interface __MIDL_itf_thumbcache_0000_0006 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_thumbcache_0000_0006_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_thumbcache_0000_0006_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

unsigned long             __RPC_USER  HBITMAP_UserSize(     __RPC__in unsigned long *, unsigned long            , __RPC__in HBITMAP * ); 
unsigned char * __RPC_USER  HBITMAP_UserMarshal(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in HBITMAP * ); 
unsigned char * __RPC_USER  HBITMAP_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out HBITMAP * ); 
void                      __RPC_USER  HBITMAP_UserFree(     __RPC__in unsigned long *, __RPC__in HBITMAP * ); 

unsigned long             __RPC_USER  HBITMAP_UserSize64(     __RPC__in unsigned long *, unsigned long            , __RPC__in HBITMAP * ); 
unsigned char * __RPC_USER  HBITMAP_UserMarshal64(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in HBITMAP * ); 
unsigned char * __RPC_USER  HBITMAP_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out HBITMAP * ); 
void                      __RPC_USER  HBITMAP_UserFree64(     __RPC__in unsigned long *, __RPC__in HBITMAP * ); 

/* [local] */ HRESULT STDMETHODCALLTYPE IThumbnailCache_GetThumbnail_Proxy( 
    IThumbnailCache * This,
    /* [in] */ IShellItem *pShellItem,
    /* [in] */ UINT cxyRequestedThumbSize,
    /* [in] */ WTS_FLAGS flags,
    /* [annotation][unique][out] */ 
    _Out_opt_  ISharedBitmap **ppvThumb,
    /* [annotation][unique][out] */ 
    _Out_opt_  WTS_CACHEFLAGS *pOutFlags,
    /* [annotation][unique][out] */ 
    _Out_opt_  WTS_THUMBNAILID *pThumbnailID);


/* [call_as] */ HRESULT STDMETHODCALLTYPE IThumbnailCache_GetThumbnail_Stub( 
    __RPC__in IThumbnailCache * This,
    /* [in] */ __RPC__in_opt IShellItem *pShellItem,
    /* [in] */ UINT cxyRequestedThumbSize,
    /* [in] */ WTS_FLAGS flags,
    /* [out] */ __RPC__deref_out_opt ISharedBitmap **ppvThumb,
    /* [out] */ __RPC__out WTS_CACHEFLAGS *pOutFlags,
    /* [out] */ __RPC__out WTS_THUMBNAILID *pThumbnailID);

/* [local] */ HRESULT STDMETHODCALLTYPE IThumbnailCache_GetThumbnailByID_Proxy( 
    IThumbnailCache * This,
    /* [in] */ WTS_THUMBNAILID thumbnailID,
    /* [in] */ UINT cxyRequestedThumbSize,
    /* [annotation][unique][out] */ 
    _Out_opt_  ISharedBitmap **ppvThumb,
    /* [annotation][unique][out] */ 
    _Out_opt_  WTS_CACHEFLAGS *pOutFlags);


/* [call_as] */ HRESULT STDMETHODCALLTYPE IThumbnailCache_GetThumbnailByID_Stub( 
    __RPC__in IThumbnailCache * This,
    /* [in] */ WTS_THUMBNAILID thumbnailID,
    /* [in] */ UINT cxyRequestedThumbSize,
    /* [out] */ __RPC__deref_out_opt ISharedBitmap **ppvThumb,
    /* [out] */ __RPC__out WTS_CACHEFLAGS *pOutFlags);



/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


