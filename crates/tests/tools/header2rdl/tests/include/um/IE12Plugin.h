

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

#ifndef __ie12plugin_h__
#define __ie12plugin_h__

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

#ifndef __ISurfacePresenterFlipBuffer_FWD_DEFINED__
#define __ISurfacePresenterFlipBuffer_FWD_DEFINED__
typedef interface ISurfacePresenterFlipBuffer ISurfacePresenterFlipBuffer;

#endif 	/* __ISurfacePresenterFlipBuffer_FWD_DEFINED__ */


#ifndef __ISurfacePresenterFlip_FWD_DEFINED__
#define __ISurfacePresenterFlip_FWD_DEFINED__
typedef interface ISurfacePresenterFlip ISurfacePresenterFlip;

#endif 	/* __ISurfacePresenterFlip_FWD_DEFINED__ */


#ifndef __ISurfacePresenterFlip2_FWD_DEFINED__
#define __ISurfacePresenterFlip2_FWD_DEFINED__
typedef interface ISurfacePresenterFlip2 ISurfacePresenterFlip2;

#endif 	/* __ISurfacePresenterFlip2_FWD_DEFINED__ */


#ifndef __IViewObjectPresentFlipSite_FWD_DEFINED__
#define __IViewObjectPresentFlipSite_FWD_DEFINED__
typedef interface IViewObjectPresentFlipSite IViewObjectPresentFlipSite;

#endif 	/* __IViewObjectPresentFlipSite_FWD_DEFINED__ */


#ifndef __IViewObjectPresentFlipSite2_FWD_DEFINED__
#define __IViewObjectPresentFlipSite2_FWD_DEFINED__
typedef interface IViewObjectPresentFlipSite2 IViewObjectPresentFlipSite2;

#endif 	/* __IViewObjectPresentFlipSite2_FWD_DEFINED__ */


#ifndef __IViewObjectPresentFlip_FWD_DEFINED__
#define __IViewObjectPresentFlip_FWD_DEFINED__
typedef interface IViewObjectPresentFlip IViewObjectPresentFlip;

#endif 	/* __IViewObjectPresentFlip_FWD_DEFINED__ */


#ifndef __IViewObjectPresentFlip2_FWD_DEFINED__
#define __IViewObjectPresentFlip2_FWD_DEFINED__
typedef interface IViewObjectPresentFlip2 IViewObjectPresentFlip2;

#endif 	/* __IViewObjectPresentFlip2_FWD_DEFINED__ */


#ifndef __IActiveXUIHandlerSite2_FWD_DEFINED__
#define __IActiveXUIHandlerSite2_FWD_DEFINED__
typedef interface IActiveXUIHandlerSite2 IActiveXUIHandlerSite2;

#endif 	/* __IActiveXUIHandlerSite2_FWD_DEFINED__ */


#ifndef __ICaretPositionProvider_FWD_DEFINED__
#define __ICaretPositionProvider_FWD_DEFINED__
typedef interface ICaretPositionProvider ICaretPositionProvider;

#endif 	/* __ICaretPositionProvider_FWD_DEFINED__ */


#ifndef __ITridentTouchInput_FWD_DEFINED__
#define __ITridentTouchInput_FWD_DEFINED__
typedef interface ITridentTouchInput ITridentTouchInput;

#endif 	/* __ITridentTouchInput_FWD_DEFINED__ */


#ifndef __ITridentTouchInputSite_FWD_DEFINED__
#define __ITridentTouchInputSite_FWD_DEFINED__
typedef interface ITridentTouchInputSite ITridentTouchInputSite;

#endif 	/* __ITridentTouchInputSite_FWD_DEFINED__ */


#ifndef __IMediaActivityNotifySite_FWD_DEFINED__
#define __IMediaActivityNotifySite_FWD_DEFINED__
typedef interface IMediaActivityNotifySite IMediaActivityNotifySite;

#endif 	/* __IMediaActivityNotifySite_FWD_DEFINED__ */


#ifndef __IAudioSessionSite_FWD_DEFINED__
#define __IAudioSessionSite_FWD_DEFINED__
typedef interface IAudioSessionSite IAudioSessionSite;

#endif 	/* __IAudioSessionSite_FWD_DEFINED__ */


#ifndef __IPrintTaskRequestHandler_FWD_DEFINED__
#define __IPrintTaskRequestHandler_FWD_DEFINED__
typedef interface IPrintTaskRequestHandler IPrintTaskRequestHandler;

#endif 	/* __IPrintTaskRequestHandler_FWD_DEFINED__ */


#ifndef __IPrintTaskRequestFactory_FWD_DEFINED__
#define __IPrintTaskRequestFactory_FWD_DEFINED__
typedef interface IPrintTaskRequestFactory IPrintTaskRequestFactory;

#endif 	/* __IPrintTaskRequestFactory_FWD_DEFINED__ */


#ifndef __IScrollableContextMenu_FWD_DEFINED__
#define __IScrollableContextMenu_FWD_DEFINED__
typedef interface IScrollableContextMenu IScrollableContextMenu;

#endif 	/* __IScrollableContextMenu_FWD_DEFINED__ */


#ifndef __IScrollableContextMenu2_FWD_DEFINED__
#define __IScrollableContextMenu2_FWD_DEFINED__
typedef interface IScrollableContextMenu2 IScrollableContextMenu2;

#endif 	/* __IScrollableContextMenu2_FWD_DEFINED__ */


#ifndef __IActiveXUIHandlerSite_FWD_DEFINED__
#define __IActiveXUIHandlerSite_FWD_DEFINED__
typedef interface IActiveXUIHandlerSite IActiveXUIHandlerSite;

#endif 	/* __IActiveXUIHandlerSite_FWD_DEFINED__ */


#ifndef __IActiveXUIHandlerSite3_FWD_DEFINED__
#define __IActiveXUIHandlerSite3_FWD_DEFINED__
typedef interface IActiveXUIHandlerSite3 IActiveXUIHandlerSite3;

#endif 	/* __IActiveXUIHandlerSite3_FWD_DEFINED__ */


/* header files for imported files */
#include "ocidl.h"
#include "inspectable.h"
#include "mshtml.h"
#include "dxgi.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_ie12plugin_0000_0000 */
/* [local] */ 

#pragma once
//;begin_internal


extern RPC_IF_HANDLE __MIDL_itf_ie12plugin_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_ie12plugin_0000_0000_v0_0_s_ifspec;

#ifndef __ISurfacePresenterFlipBuffer_INTERFACE_DEFINED__
#define __ISurfacePresenterFlipBuffer_INTERFACE_DEFINED__

/* interface ISurfacePresenterFlipBuffer */
/* [uuid][unique][object] */ 


EXTERN_C const IID IID_ISurfacePresenterFlipBuffer;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("e43f4a08-8bbc-4665-ac92-c55ce61fd7e7")
    ISurfacePresenterFlipBuffer : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE BeginDraw( 
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][out][retval] */ __RPC__deref_out_opt void **ppBuffer) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EndDraw( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISurfacePresenterFlipBufferVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ISurfacePresenterFlipBuffer * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ISurfacePresenterFlipBuffer * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ISurfacePresenterFlipBuffer * This);
        
        DECLSPEC_XFGVIRT(ISurfacePresenterFlipBuffer, BeginDraw)
        HRESULT ( STDMETHODCALLTYPE *BeginDraw )( 
            __RPC__in ISurfacePresenterFlipBuffer * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][out][retval] */ __RPC__deref_out_opt void **ppBuffer);
        
        DECLSPEC_XFGVIRT(ISurfacePresenterFlipBuffer, EndDraw)
        HRESULT ( STDMETHODCALLTYPE *EndDraw )( 
            __RPC__in ISurfacePresenterFlipBuffer * This);
        
        END_INTERFACE
    } ISurfacePresenterFlipBufferVtbl;

    interface ISurfacePresenterFlipBuffer
    {
        CONST_VTBL struct ISurfacePresenterFlipBufferVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISurfacePresenterFlipBuffer_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISurfacePresenterFlipBuffer_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISurfacePresenterFlipBuffer_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISurfacePresenterFlipBuffer_BeginDraw(This,riid,ppBuffer)	\
    ( (This)->lpVtbl -> BeginDraw(This,riid,ppBuffer) ) 

#define ISurfacePresenterFlipBuffer_EndDraw(This)	\
    ( (This)->lpVtbl -> EndDraw(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISurfacePresenterFlipBuffer_INTERFACE_DEFINED__ */


#ifndef __ISurfacePresenterFlip_INTERFACE_DEFINED__
#define __ISurfacePresenterFlip_INTERFACE_DEFINED__

/* interface ISurfacePresenterFlip */
/* [uuid][unique][object] */ 


EXTERN_C const IID IID_ISurfacePresenterFlip;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("30510848-98b5-11cf-bb82-00aa00bdce0b")
    ISurfacePresenterFlip : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Present( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetBuffer( 
            /* [in] */ UINT backBufferIndex,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][out][retval] */ __RPC__deref_out_opt void **ppBuffer) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISurfacePresenterFlipVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ISurfacePresenterFlip * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ISurfacePresenterFlip * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ISurfacePresenterFlip * This);
        
        DECLSPEC_XFGVIRT(ISurfacePresenterFlip, Present)
        HRESULT ( STDMETHODCALLTYPE *Present )( 
            __RPC__in ISurfacePresenterFlip * This);
        
        DECLSPEC_XFGVIRT(ISurfacePresenterFlip, GetBuffer)
        HRESULT ( STDMETHODCALLTYPE *GetBuffer )( 
            __RPC__in ISurfacePresenterFlip * This,
            /* [in] */ UINT backBufferIndex,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][out][retval] */ __RPC__deref_out_opt void **ppBuffer);
        
        END_INTERFACE
    } ISurfacePresenterFlipVtbl;

    interface ISurfacePresenterFlip
    {
        CONST_VTBL struct ISurfacePresenterFlipVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISurfacePresenterFlip_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISurfacePresenterFlip_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISurfacePresenterFlip_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISurfacePresenterFlip_Present(This)	\
    ( (This)->lpVtbl -> Present(This) ) 

#define ISurfacePresenterFlip_GetBuffer(This,backBufferIndex,riid,ppBuffer)	\
    ( (This)->lpVtbl -> GetBuffer(This,backBufferIndex,riid,ppBuffer) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISurfacePresenterFlip_INTERFACE_DEFINED__ */


#ifndef __ISurfacePresenterFlip2_INTERFACE_DEFINED__
#define __ISurfacePresenterFlip2_INTERFACE_DEFINED__

/* interface ISurfacePresenterFlip2 */
/* [uuid][unique][object] */ 


EXTERN_C const IID IID_ISurfacePresenterFlip2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("30510865-98b5-11cf-bb82-00aa00bdce0b")
    ISurfacePresenterFlip2 : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SetRotation( 
            /* [in] */ DXGI_MODE_ROTATION dxgiRotation) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISurfacePresenterFlip2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ISurfacePresenterFlip2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ISurfacePresenterFlip2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ISurfacePresenterFlip2 * This);
        
        DECLSPEC_XFGVIRT(ISurfacePresenterFlip2, SetRotation)
        HRESULT ( STDMETHODCALLTYPE *SetRotation )( 
            __RPC__in ISurfacePresenterFlip2 * This,
            /* [in] */ DXGI_MODE_ROTATION dxgiRotation);
        
        END_INTERFACE
    } ISurfacePresenterFlip2Vtbl;

    interface ISurfacePresenterFlip2
    {
        CONST_VTBL struct ISurfacePresenterFlip2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISurfacePresenterFlip2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISurfacePresenterFlip2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISurfacePresenterFlip2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISurfacePresenterFlip2_SetRotation(This,dxgiRotation)	\
    ( (This)->lpVtbl -> SetRotation(This,dxgiRotation) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISurfacePresenterFlip2_INTERFACE_DEFINED__ */


#ifndef __IViewObjectPresentFlipSite_INTERFACE_DEFINED__
#define __IViewObjectPresentFlipSite_INTERFACE_DEFINED__

/* interface IViewObjectPresentFlipSite */
/* [uuid][unique][object] */ 


EXTERN_C const IID IID_IViewObjectPresentFlipSite;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("30510846-98b5-11cf-bb82-00aa00bdce0b")
    IViewObjectPresentFlipSite : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE CreateSurfacePresenterFlip( 
            /* [in] */ __RPC__in_opt IUnknown *pDevice,
            /* [in] */ UINT width,
            /* [in] */ UINT height,
            /* [in] */ UINT backBufferCount,
            /* [in] */ DXGI_FORMAT format,
            /* [in] */ VIEW_OBJECT_ALPHA_MODE mode,
            /* [out][retval] */ __RPC__deref_out_opt ISurfacePresenterFlip **ppSPFlip) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetDeviceLuid( 
            /* [out][retval] */ __RPC__out LUID *pLuid) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EnterFullScreen( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ExitFullScreen( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE IsFullScreen( 
            /* [out][retval] */ __RPC__out BOOL *pfFullScreen) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetBoundingRect( 
            /* [out] */ __RPC__out RECT *pRect) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetMetrics( 
            /* [out] */ __RPC__out POINT *pPos,
            /* [out] */ __RPC__out SIZE *pSize,
            /* [out] */ __RPC__out float *pScaleX,
            /* [out] */ __RPC__out float *pScaleY) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetFullScreenSize( 
            /* [out] */ __RPC__out SIZE *pSize) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IViewObjectPresentFlipSiteVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IViewObjectPresentFlipSite * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IViewObjectPresentFlipSite * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IViewObjectPresentFlipSite * This);
        
        DECLSPEC_XFGVIRT(IViewObjectPresentFlipSite, CreateSurfacePresenterFlip)
        HRESULT ( STDMETHODCALLTYPE *CreateSurfacePresenterFlip )( 
            __RPC__in IViewObjectPresentFlipSite * This,
            /* [in] */ __RPC__in_opt IUnknown *pDevice,
            /* [in] */ UINT width,
            /* [in] */ UINT height,
            /* [in] */ UINT backBufferCount,
            /* [in] */ DXGI_FORMAT format,
            /* [in] */ VIEW_OBJECT_ALPHA_MODE mode,
            /* [out][retval] */ __RPC__deref_out_opt ISurfacePresenterFlip **ppSPFlip);
        
        DECLSPEC_XFGVIRT(IViewObjectPresentFlipSite, GetDeviceLuid)
        HRESULT ( STDMETHODCALLTYPE *GetDeviceLuid )( 
            __RPC__in IViewObjectPresentFlipSite * This,
            /* [out][retval] */ __RPC__out LUID *pLuid);
        
        DECLSPEC_XFGVIRT(IViewObjectPresentFlipSite, EnterFullScreen)
        HRESULT ( STDMETHODCALLTYPE *EnterFullScreen )( 
            __RPC__in IViewObjectPresentFlipSite * This);
        
        DECLSPEC_XFGVIRT(IViewObjectPresentFlipSite, ExitFullScreen)
        HRESULT ( STDMETHODCALLTYPE *ExitFullScreen )( 
            __RPC__in IViewObjectPresentFlipSite * This);
        
        DECLSPEC_XFGVIRT(IViewObjectPresentFlipSite, IsFullScreen)
        HRESULT ( STDMETHODCALLTYPE *IsFullScreen )( 
            __RPC__in IViewObjectPresentFlipSite * This,
            /* [out][retval] */ __RPC__out BOOL *pfFullScreen);
        
        DECLSPEC_XFGVIRT(IViewObjectPresentFlipSite, GetBoundingRect)
        HRESULT ( STDMETHODCALLTYPE *GetBoundingRect )( 
            __RPC__in IViewObjectPresentFlipSite * This,
            /* [out] */ __RPC__out RECT *pRect);
        
        DECLSPEC_XFGVIRT(IViewObjectPresentFlipSite, GetMetrics)
        HRESULT ( STDMETHODCALLTYPE *GetMetrics )( 
            __RPC__in IViewObjectPresentFlipSite * This,
            /* [out] */ __RPC__out POINT *pPos,
            /* [out] */ __RPC__out SIZE *pSize,
            /* [out] */ __RPC__out float *pScaleX,
            /* [out] */ __RPC__out float *pScaleY);
        
        DECLSPEC_XFGVIRT(IViewObjectPresentFlipSite, GetFullScreenSize)
        HRESULT ( STDMETHODCALLTYPE *GetFullScreenSize )( 
            __RPC__in IViewObjectPresentFlipSite * This,
            /* [out] */ __RPC__out SIZE *pSize);
        
        END_INTERFACE
    } IViewObjectPresentFlipSiteVtbl;

    interface IViewObjectPresentFlipSite
    {
        CONST_VTBL struct IViewObjectPresentFlipSiteVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IViewObjectPresentFlipSite_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IViewObjectPresentFlipSite_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IViewObjectPresentFlipSite_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IViewObjectPresentFlipSite_CreateSurfacePresenterFlip(This,pDevice,width,height,backBufferCount,format,mode,ppSPFlip)	\
    ( (This)->lpVtbl -> CreateSurfacePresenterFlip(This,pDevice,width,height,backBufferCount,format,mode,ppSPFlip) ) 

#define IViewObjectPresentFlipSite_GetDeviceLuid(This,pLuid)	\
    ( (This)->lpVtbl -> GetDeviceLuid(This,pLuid) ) 

#define IViewObjectPresentFlipSite_EnterFullScreen(This)	\
    ( (This)->lpVtbl -> EnterFullScreen(This) ) 

#define IViewObjectPresentFlipSite_ExitFullScreen(This)	\
    ( (This)->lpVtbl -> ExitFullScreen(This) ) 

#define IViewObjectPresentFlipSite_IsFullScreen(This,pfFullScreen)	\
    ( (This)->lpVtbl -> IsFullScreen(This,pfFullScreen) ) 

#define IViewObjectPresentFlipSite_GetBoundingRect(This,pRect)	\
    ( (This)->lpVtbl -> GetBoundingRect(This,pRect) ) 

#define IViewObjectPresentFlipSite_GetMetrics(This,pPos,pSize,pScaleX,pScaleY)	\
    ( (This)->lpVtbl -> GetMetrics(This,pPos,pSize,pScaleX,pScaleY) ) 

#define IViewObjectPresentFlipSite_GetFullScreenSize(This,pSize)	\
    ( (This)->lpVtbl -> GetFullScreenSize(This,pSize) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IViewObjectPresentFlipSite_INTERFACE_DEFINED__ */


#ifndef __IViewObjectPresentFlipSite2_INTERFACE_DEFINED__
#define __IViewObjectPresentFlipSite2_INTERFACE_DEFINED__

/* interface IViewObjectPresentFlipSite2 */
/* [uuid][unique][object] */ 


EXTERN_C const IID IID_IViewObjectPresentFlipSite2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("aad0cbf1-e7fd-4f12-8902-c78132a8e01d")
    IViewObjectPresentFlipSite2 : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetRotationForCurrentOutput( 
            /* [out] */ __RPC__out DXGI_MODE_ROTATION *pDxgiRotation) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IViewObjectPresentFlipSite2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IViewObjectPresentFlipSite2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IViewObjectPresentFlipSite2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IViewObjectPresentFlipSite2 * This);
        
        DECLSPEC_XFGVIRT(IViewObjectPresentFlipSite2, GetRotationForCurrentOutput)
        HRESULT ( STDMETHODCALLTYPE *GetRotationForCurrentOutput )( 
            __RPC__in IViewObjectPresentFlipSite2 * This,
            /* [out] */ __RPC__out DXGI_MODE_ROTATION *pDxgiRotation);
        
        END_INTERFACE
    } IViewObjectPresentFlipSite2Vtbl;

    interface IViewObjectPresentFlipSite2
    {
        CONST_VTBL struct IViewObjectPresentFlipSite2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IViewObjectPresentFlipSite2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IViewObjectPresentFlipSite2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IViewObjectPresentFlipSite2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IViewObjectPresentFlipSite2_GetRotationForCurrentOutput(This,pDxgiRotation)	\
    ( (This)->lpVtbl -> GetRotationForCurrentOutput(This,pDxgiRotation) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IViewObjectPresentFlipSite2_INTERFACE_DEFINED__ */


#ifndef __IViewObjectPresentFlip_INTERFACE_DEFINED__
#define __IViewObjectPresentFlip_INTERFACE_DEFINED__

/* interface IViewObjectPresentFlip */
/* [uuid][unique][object] */ 


EXTERN_C const IID IID_IViewObjectPresentFlip;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("30510847-98b5-11cf-bb82-00aa00bdce0b")
    IViewObjectPresentFlip : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE NotifyRender( 
            BOOL fRecreatePresenter) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RenderObjectToBitmap( 
            /* [in] */ __RPC__in_opt IUnknown *pBitmap) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RenderObjectToSharedBuffer( 
            /* [in] */ __RPC__in_opt ISurfacePresenterFlipBuffer *pBuffer) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IViewObjectPresentFlipVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IViewObjectPresentFlip * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IViewObjectPresentFlip * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IViewObjectPresentFlip * This);
        
        DECLSPEC_XFGVIRT(IViewObjectPresentFlip, NotifyRender)
        HRESULT ( STDMETHODCALLTYPE *NotifyRender )( 
            __RPC__in IViewObjectPresentFlip * This,
            BOOL fRecreatePresenter);
        
        DECLSPEC_XFGVIRT(IViewObjectPresentFlip, RenderObjectToBitmap)
        HRESULT ( STDMETHODCALLTYPE *RenderObjectToBitmap )( 
            __RPC__in IViewObjectPresentFlip * This,
            /* [in] */ __RPC__in_opt IUnknown *pBitmap);
        
        DECLSPEC_XFGVIRT(IViewObjectPresentFlip, RenderObjectToSharedBuffer)
        HRESULT ( STDMETHODCALLTYPE *RenderObjectToSharedBuffer )( 
            __RPC__in IViewObjectPresentFlip * This,
            /* [in] */ __RPC__in_opt ISurfacePresenterFlipBuffer *pBuffer);
        
        END_INTERFACE
    } IViewObjectPresentFlipVtbl;

    interface IViewObjectPresentFlip
    {
        CONST_VTBL struct IViewObjectPresentFlipVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IViewObjectPresentFlip_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IViewObjectPresentFlip_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IViewObjectPresentFlip_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IViewObjectPresentFlip_NotifyRender(This,fRecreatePresenter)	\
    ( (This)->lpVtbl -> NotifyRender(This,fRecreatePresenter) ) 

#define IViewObjectPresentFlip_RenderObjectToBitmap(This,pBitmap)	\
    ( (This)->lpVtbl -> RenderObjectToBitmap(This,pBitmap) ) 

#define IViewObjectPresentFlip_RenderObjectToSharedBuffer(This,pBuffer)	\
    ( (This)->lpVtbl -> RenderObjectToSharedBuffer(This,pBuffer) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IViewObjectPresentFlip_INTERFACE_DEFINED__ */


#ifndef __IViewObjectPresentFlip2_INTERFACE_DEFINED__
#define __IViewObjectPresentFlip2_INTERFACE_DEFINED__

/* interface IViewObjectPresentFlip2 */
/* [uuid][unique][object] */ 


EXTERN_C const IID IID_IViewObjectPresentFlip2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("30510856-98b5-11cf-bb82-00aa00bdce0b")
    IViewObjectPresentFlip2 : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE NotifyLeavingView( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IViewObjectPresentFlip2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IViewObjectPresentFlip2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IViewObjectPresentFlip2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IViewObjectPresentFlip2 * This);
        
        DECLSPEC_XFGVIRT(IViewObjectPresentFlip2, NotifyLeavingView)
        HRESULT ( STDMETHODCALLTYPE *NotifyLeavingView )( 
            __RPC__in IViewObjectPresentFlip2 * This);
        
        END_INTERFACE
    } IViewObjectPresentFlip2Vtbl;

    interface IViewObjectPresentFlip2
    {
        CONST_VTBL struct IViewObjectPresentFlip2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IViewObjectPresentFlip2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IViewObjectPresentFlip2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IViewObjectPresentFlip2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IViewObjectPresentFlip2_NotifyLeavingView(This)	\
    ( (This)->lpVtbl -> NotifyLeavingView(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IViewObjectPresentFlip2_INTERFACE_DEFINED__ */


#ifndef __IActiveXUIHandlerSite2_INTERFACE_DEFINED__
#define __IActiveXUIHandlerSite2_INTERFACE_DEFINED__

/* interface IActiveXUIHandlerSite2 */
/* [uuid][unique][object] */ 


EXTERN_C const IID IID_IActiveXUIHandlerSite2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("7E3707B2-D087-4542-AC1F-A0D2FCD080FD")
    IActiveXUIHandlerSite2 : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE AddSuspensionExemption( 
            /* [out][retval] */ __RPC__out ULONGLONG *pullCookie) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RemoveSuspensionExemption( 
            /* [in] */ ULONGLONG ullCookie) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IActiveXUIHandlerSite2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IActiveXUIHandlerSite2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IActiveXUIHandlerSite2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IActiveXUIHandlerSite2 * This);
        
        DECLSPEC_XFGVIRT(IActiveXUIHandlerSite2, AddSuspensionExemption)
        HRESULT ( STDMETHODCALLTYPE *AddSuspensionExemption )( 
            __RPC__in IActiveXUIHandlerSite2 * This,
            /* [out][retval] */ __RPC__out ULONGLONG *pullCookie);
        
        DECLSPEC_XFGVIRT(IActiveXUIHandlerSite2, RemoveSuspensionExemption)
        HRESULT ( STDMETHODCALLTYPE *RemoveSuspensionExemption )( 
            __RPC__in IActiveXUIHandlerSite2 * This,
            /* [in] */ ULONGLONG ullCookie);
        
        END_INTERFACE
    } IActiveXUIHandlerSite2Vtbl;

    interface IActiveXUIHandlerSite2
    {
        CONST_VTBL struct IActiveXUIHandlerSite2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IActiveXUIHandlerSite2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IActiveXUIHandlerSite2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IActiveXUIHandlerSite2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IActiveXUIHandlerSite2_AddSuspensionExemption(This,pullCookie)	\
    ( (This)->lpVtbl -> AddSuspensionExemption(This,pullCookie) ) 

#define IActiveXUIHandlerSite2_RemoveSuspensionExemption(This,ullCookie)	\
    ( (This)->lpVtbl -> RemoveSuspensionExemption(This,ullCookie) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IActiveXUIHandlerSite2_INTERFACE_DEFINED__ */


#ifndef __ICaretPositionProvider_INTERFACE_DEFINED__
#define __ICaretPositionProvider_INTERFACE_DEFINED__

/* interface ICaretPositionProvider */
/* [uuid][unique][object] */ 


EXTERN_C const IID IID_ICaretPositionProvider;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("58DA43A2-108E-4D5B-9F75-E5F74F93FFF5")
    ICaretPositionProvider : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetCaretPosition( 
            /* [out] */ __RPC__out POINT *pptCaret,
            /* [out] */ __RPC__out float *pflHeight) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ICaretPositionProviderVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ICaretPositionProvider * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ICaretPositionProvider * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ICaretPositionProvider * This);
        
        DECLSPEC_XFGVIRT(ICaretPositionProvider, GetCaretPosition)
        HRESULT ( STDMETHODCALLTYPE *GetCaretPosition )( 
            __RPC__in ICaretPositionProvider * This,
            /* [out] */ __RPC__out POINT *pptCaret,
            /* [out] */ __RPC__out float *pflHeight);
        
        END_INTERFACE
    } ICaretPositionProviderVtbl;

    interface ICaretPositionProvider
    {
        CONST_VTBL struct ICaretPositionProviderVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ICaretPositionProvider_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ICaretPositionProvider_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ICaretPositionProvider_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ICaretPositionProvider_GetCaretPosition(This,pptCaret,pflHeight)	\
    ( (This)->lpVtbl -> GetCaretPosition(This,pptCaret,pflHeight) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ICaretPositionProvider_INTERFACE_DEFINED__ */


#ifndef __ITridentTouchInput_INTERFACE_DEFINED__
#define __ITridentTouchInput_INTERFACE_DEFINED__

/* interface ITridentTouchInput */
/* [uuid][unique][object] */ 


EXTERN_C const IID IID_ITridentTouchInput;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("30510850-98b5-11cf-bb82-00aa00bdce0b")
    ITridentTouchInput : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE OnPointerMessage( 
            /* [in] */ UINT msg,
            /* [in] */ WPARAM wParam,
            /* [in] */ LPARAM lParam,
            /* [out] */ __RPC__out BOOL *pfAllowManipulations) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITridentTouchInputVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITridentTouchInput * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITridentTouchInput * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITridentTouchInput * This);
        
        DECLSPEC_XFGVIRT(ITridentTouchInput, OnPointerMessage)
        HRESULT ( STDMETHODCALLTYPE *OnPointerMessage )( 
            __RPC__in ITridentTouchInput * This,
            /* [in] */ UINT msg,
            /* [in] */ WPARAM wParam,
            /* [in] */ LPARAM lParam,
            /* [out] */ __RPC__out BOOL *pfAllowManipulations);
        
        END_INTERFACE
    } ITridentTouchInputVtbl;

    interface ITridentTouchInput
    {
        CONST_VTBL struct ITridentTouchInputVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITridentTouchInput_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITridentTouchInput_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITridentTouchInput_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITridentTouchInput_OnPointerMessage(This,msg,wParam,lParam,pfAllowManipulations)	\
    ( (This)->lpVtbl -> OnPointerMessage(This,msg,wParam,lParam,pfAllowManipulations) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITridentTouchInput_INTERFACE_DEFINED__ */


#ifndef __ITridentTouchInputSite_INTERFACE_DEFINED__
#define __ITridentTouchInputSite_INTERFACE_DEFINED__

/* interface ITridentTouchInputSite */
/* [uuid][unique][object] */ 


EXTERN_C const IID IID_ITridentTouchInputSite;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("30510849-98b5-11cf-bb82-00aa00bdce0b")
    ITridentTouchInputSite : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SetManipulationMode( 
            /* [in] */ styleMsTouchAction msTouchAction) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ZoomToPoint( 
            /* [in] */ LONG x,
            /* [in] */ LONG y) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITridentTouchInputSiteVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITridentTouchInputSite * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITridentTouchInputSite * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITridentTouchInputSite * This);
        
        DECLSPEC_XFGVIRT(ITridentTouchInputSite, SetManipulationMode)
        HRESULT ( STDMETHODCALLTYPE *SetManipulationMode )( 
            __RPC__in ITridentTouchInputSite * This,
            /* [in] */ styleMsTouchAction msTouchAction);
        
        DECLSPEC_XFGVIRT(ITridentTouchInputSite, ZoomToPoint)
        HRESULT ( STDMETHODCALLTYPE *ZoomToPoint )( 
            __RPC__in ITridentTouchInputSite * This,
            /* [in] */ LONG x,
            /* [in] */ LONG y);
        
        END_INTERFACE
    } ITridentTouchInputSiteVtbl;

    interface ITridentTouchInputSite
    {
        CONST_VTBL struct ITridentTouchInputSiteVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITridentTouchInputSite_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITridentTouchInputSite_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITridentTouchInputSite_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITridentTouchInputSite_SetManipulationMode(This,msTouchAction)	\
    ( (This)->lpVtbl -> SetManipulationMode(This,msTouchAction) ) 

#define ITridentTouchInputSite_ZoomToPoint(This,x,y)	\
    ( (This)->lpVtbl -> ZoomToPoint(This,x,y) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITridentTouchInputSite_INTERFACE_DEFINED__ */


#ifndef __IMediaActivityNotifySite_INTERFACE_DEFINED__
#define __IMediaActivityNotifySite_INTERFACE_DEFINED__

/* interface IMediaActivityNotifySite */
/* [uuid][unique][object] */ 

typedef 
enum MediaActivityNotifyType
    {
        MediaPlayback	= 0,
        MediaRecording	= ( MediaPlayback + 1 ) ,
        MediaCasting	= ( MediaRecording + 1 ) 
    } 	MEDIA_ACTIVITY_NOTIFY_TYPE;


EXTERN_C const IID IID_IMediaActivityNotifySite;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("8165cfef-179d-46c2-bc71-3fa726dc1f8d")
    IMediaActivityNotifySite : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE OnMediaActivityStarted( 
            /* [in] */ MEDIA_ACTIVITY_NOTIFY_TYPE mediaActivityType) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnMediaActivityStopped( 
            /* [in] */ MEDIA_ACTIVITY_NOTIFY_TYPE mediaActivityType) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMediaActivityNotifySiteVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMediaActivityNotifySite * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMediaActivityNotifySite * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMediaActivityNotifySite * This);
        
        DECLSPEC_XFGVIRT(IMediaActivityNotifySite, OnMediaActivityStarted)
        HRESULT ( STDMETHODCALLTYPE *OnMediaActivityStarted )( 
            __RPC__in IMediaActivityNotifySite * This,
            /* [in] */ MEDIA_ACTIVITY_NOTIFY_TYPE mediaActivityType);
        
        DECLSPEC_XFGVIRT(IMediaActivityNotifySite, OnMediaActivityStopped)
        HRESULT ( STDMETHODCALLTYPE *OnMediaActivityStopped )( 
            __RPC__in IMediaActivityNotifySite * This,
            /* [in] */ MEDIA_ACTIVITY_NOTIFY_TYPE mediaActivityType);
        
        END_INTERFACE
    } IMediaActivityNotifySiteVtbl;

    interface IMediaActivityNotifySite
    {
        CONST_VTBL struct IMediaActivityNotifySiteVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMediaActivityNotifySite_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMediaActivityNotifySite_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMediaActivityNotifySite_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMediaActivityNotifySite_OnMediaActivityStarted(This,mediaActivityType)	\
    ( (This)->lpVtbl -> OnMediaActivityStarted(This,mediaActivityType) ) 

#define IMediaActivityNotifySite_OnMediaActivityStopped(This,mediaActivityType)	\
    ( (This)->lpVtbl -> OnMediaActivityStopped(This,mediaActivityType) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMediaActivityNotifySite_INTERFACE_DEFINED__ */


#ifndef __IAudioSessionSite_INTERFACE_DEFINED__
#define __IAudioSessionSite_INTERFACE_DEFINED__

/* interface IAudioSessionSite */
/* [uuid][unique][object] */ 


EXTERN_C const IID IID_IAudioSessionSite;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("d7d8b684-d02d-4517-b6b7-19e3dfe29c45")
    IAudioSessionSite : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetAudioSessionGuid( 
            /* [out][retval] */ __RPC__out GUID *audioSessionGuid) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnAudioStreamCreated( 
            /* [string][in] */ __RPC__in_string LPCWSTR endpointID) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnAudioStreamDestroyed( 
            /* [string][in] */ __RPC__in_string LPCWSTR endpointID) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAudioSessionSiteVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IAudioSessionSite * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IAudioSessionSite * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IAudioSessionSite * This);
        
        DECLSPEC_XFGVIRT(IAudioSessionSite, GetAudioSessionGuid)
        HRESULT ( STDMETHODCALLTYPE *GetAudioSessionGuid )( 
            __RPC__in IAudioSessionSite * This,
            /* [out][retval] */ __RPC__out GUID *audioSessionGuid);
        
        DECLSPEC_XFGVIRT(IAudioSessionSite, OnAudioStreamCreated)
        HRESULT ( STDMETHODCALLTYPE *OnAudioStreamCreated )( 
            __RPC__in IAudioSessionSite * This,
            /* [string][in] */ __RPC__in_string LPCWSTR endpointID);
        
        DECLSPEC_XFGVIRT(IAudioSessionSite, OnAudioStreamDestroyed)
        HRESULT ( STDMETHODCALLTYPE *OnAudioStreamDestroyed )( 
            __RPC__in IAudioSessionSite * This,
            /* [string][in] */ __RPC__in_string LPCWSTR endpointID);
        
        END_INTERFACE
    } IAudioSessionSiteVtbl;

    interface IAudioSessionSite
    {
        CONST_VTBL struct IAudioSessionSiteVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAudioSessionSite_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAudioSessionSite_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAudioSessionSite_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAudioSessionSite_GetAudioSessionGuid(This,audioSessionGuid)	\
    ( (This)->lpVtbl -> GetAudioSessionGuid(This,audioSessionGuid) ) 

#define IAudioSessionSite_OnAudioStreamCreated(This,endpointID)	\
    ( (This)->lpVtbl -> OnAudioStreamCreated(This,endpointID) ) 

#define IAudioSessionSite_OnAudioStreamDestroyed(This,endpointID)	\
    ( (This)->lpVtbl -> OnAudioStreamDestroyed(This,endpointID) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAudioSessionSite_INTERFACE_DEFINED__ */


#ifndef __IPrintTaskRequestHandler_INTERFACE_DEFINED__
#define __IPrintTaskRequestHandler_INTERFACE_DEFINED__

/* interface IPrintTaskRequestHandler */
/* [local][uuid][unique][object] */ 


EXTERN_C const IID IID_IPrintTaskRequestHandler;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("191CD340-CF36-44FF-BD53-D1B701799D9B")
    IPrintTaskRequestHandler : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE HandlePrintTaskRequest( 
            /* [in] */ IInspectable *pPrintTaskRequest) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IPrintTaskRequestHandlerVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IPrintTaskRequestHandler * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IPrintTaskRequestHandler * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IPrintTaskRequestHandler * This);
        
        DECLSPEC_XFGVIRT(IPrintTaskRequestHandler, HandlePrintTaskRequest)
        HRESULT ( STDMETHODCALLTYPE *HandlePrintTaskRequest )( 
            IPrintTaskRequestHandler * This,
            /* [in] */ IInspectable *pPrintTaskRequest);
        
        END_INTERFACE
    } IPrintTaskRequestHandlerVtbl;

    interface IPrintTaskRequestHandler
    {
        CONST_VTBL struct IPrintTaskRequestHandlerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IPrintTaskRequestHandler_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IPrintTaskRequestHandler_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IPrintTaskRequestHandler_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IPrintTaskRequestHandler_HandlePrintTaskRequest(This,pPrintTaskRequest)	\
    ( (This)->lpVtbl -> HandlePrintTaskRequest(This,pPrintTaskRequest) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IPrintTaskRequestHandler_INTERFACE_DEFINED__ */


#ifndef __IPrintTaskRequestFactory_INTERFACE_DEFINED__
#define __IPrintTaskRequestFactory_INTERFACE_DEFINED__

/* interface IPrintTaskRequestFactory */
/* [uuid][unique][object] */ 


EXTERN_C const IID IID_IPrintTaskRequestFactory;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("BB516745-8C34-4F8B-9605-684DCB144BE5")
    IPrintTaskRequestFactory : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE CreatePrintTaskRequest( 
            /* [in] */ __RPC__in_opt IPrintTaskRequestHandler *pPrintTaskRequestHandler) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IPrintTaskRequestFactoryVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IPrintTaskRequestFactory * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IPrintTaskRequestFactory * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IPrintTaskRequestFactory * This);
        
        DECLSPEC_XFGVIRT(IPrintTaskRequestFactory, CreatePrintTaskRequest)
        HRESULT ( STDMETHODCALLTYPE *CreatePrintTaskRequest )( 
            __RPC__in IPrintTaskRequestFactory * This,
            /* [in] */ __RPC__in_opt IPrintTaskRequestHandler *pPrintTaskRequestHandler);
        
        END_INTERFACE
    } IPrintTaskRequestFactoryVtbl;

    interface IPrintTaskRequestFactory
    {
        CONST_VTBL struct IPrintTaskRequestFactoryVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IPrintTaskRequestFactory_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IPrintTaskRequestFactory_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IPrintTaskRequestFactory_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IPrintTaskRequestFactory_CreatePrintTaskRequest(This,pPrintTaskRequestHandler)	\
    ( (This)->lpVtbl -> CreatePrintTaskRequest(This,pPrintTaskRequestHandler) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IPrintTaskRequestFactory_INTERFACE_DEFINED__ */


#ifndef __IScrollableContextMenu_INTERFACE_DEFINED__
#define __IScrollableContextMenu_INTERFACE_DEFINED__

/* interface IScrollableContextMenu */
/* [uuid][unique][object] */ 


EXTERN_C const IID IID_IScrollableContextMenu;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("30510854-98b5-11cf-bb82-00aa00bdce0b")
    IScrollableContextMenu : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE AddItem( 
            /* [string][in] */ __RPC__in_string LPCWSTR itemText,
            /* [in] */ DWORD cmdID) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ShowModal( 
            /* [in] */ int x,
            /* [in] */ int y,
            /* [out] */ __RPC__out DWORD *cmdID) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IScrollableContextMenuVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IScrollableContextMenu * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IScrollableContextMenu * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IScrollableContextMenu * This);
        
        DECLSPEC_XFGVIRT(IScrollableContextMenu, AddItem)
        HRESULT ( STDMETHODCALLTYPE *AddItem )( 
            __RPC__in IScrollableContextMenu * This,
            /* [string][in] */ __RPC__in_string LPCWSTR itemText,
            /* [in] */ DWORD cmdID);
        
        DECLSPEC_XFGVIRT(IScrollableContextMenu, ShowModal)
        HRESULT ( STDMETHODCALLTYPE *ShowModal )( 
            __RPC__in IScrollableContextMenu * This,
            /* [in] */ int x,
            /* [in] */ int y,
            /* [out] */ __RPC__out DWORD *cmdID);
        
        END_INTERFACE
    } IScrollableContextMenuVtbl;

    interface IScrollableContextMenu
    {
        CONST_VTBL struct IScrollableContextMenuVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IScrollableContextMenu_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IScrollableContextMenu_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IScrollableContextMenu_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IScrollableContextMenu_AddItem(This,itemText,cmdID)	\
    ( (This)->lpVtbl -> AddItem(This,itemText,cmdID) ) 

#define IScrollableContextMenu_ShowModal(This,x,y,cmdID)	\
    ( (This)->lpVtbl -> ShowModal(This,x,y,cmdID) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IScrollableContextMenu_INTERFACE_DEFINED__ */


#ifndef __IScrollableContextMenu2_INTERFACE_DEFINED__
#define __IScrollableContextMenu2_INTERFACE_DEFINED__

/* interface IScrollableContextMenu2 */
/* [uuid][unique][object] */ 

typedef 
enum tagSCROLLABLECONTEXTMENU_PLACEMENT
    {
        SCMP_TOP	= 0,
        SCMP_BOTTOM	= 1,
        SCMP_LEFT	= 2,
        SCMP_RIGHT	= 3,
        SCMP_FULL	= 4
    } 	SCROLLABLECONTEXTMENU_PLACEMENT;


EXTERN_C const IID IID_IScrollableContextMenu2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("F77E9056-8674-4936-924C-0E4A06FA634A")
    IScrollableContextMenu2 : public IScrollableContextMenu
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE AddSeparator( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetPlacement( 
            /* [in] */ SCROLLABLECONTEXTMENU_PLACEMENT scmp) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IScrollableContextMenu2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IScrollableContextMenu2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IScrollableContextMenu2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IScrollableContextMenu2 * This);
        
        DECLSPEC_XFGVIRT(IScrollableContextMenu, AddItem)
        HRESULT ( STDMETHODCALLTYPE *AddItem )( 
            __RPC__in IScrollableContextMenu2 * This,
            /* [string][in] */ __RPC__in_string LPCWSTR itemText,
            /* [in] */ DWORD cmdID);
        
        DECLSPEC_XFGVIRT(IScrollableContextMenu, ShowModal)
        HRESULT ( STDMETHODCALLTYPE *ShowModal )( 
            __RPC__in IScrollableContextMenu2 * This,
            /* [in] */ int x,
            /* [in] */ int y,
            /* [out] */ __RPC__out DWORD *cmdID);
        
        DECLSPEC_XFGVIRT(IScrollableContextMenu2, AddSeparator)
        HRESULT ( STDMETHODCALLTYPE *AddSeparator )( 
            __RPC__in IScrollableContextMenu2 * This);
        
        DECLSPEC_XFGVIRT(IScrollableContextMenu2, SetPlacement)
        HRESULT ( STDMETHODCALLTYPE *SetPlacement )( 
            __RPC__in IScrollableContextMenu2 * This,
            /* [in] */ SCROLLABLECONTEXTMENU_PLACEMENT scmp);
        
        END_INTERFACE
    } IScrollableContextMenu2Vtbl;

    interface IScrollableContextMenu2
    {
        CONST_VTBL struct IScrollableContextMenu2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IScrollableContextMenu2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IScrollableContextMenu2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IScrollableContextMenu2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IScrollableContextMenu2_AddItem(This,itemText,cmdID)	\
    ( (This)->lpVtbl -> AddItem(This,itemText,cmdID) ) 

#define IScrollableContextMenu2_ShowModal(This,x,y,cmdID)	\
    ( (This)->lpVtbl -> ShowModal(This,x,y,cmdID) ) 


#define IScrollableContextMenu2_AddSeparator(This)	\
    ( (This)->lpVtbl -> AddSeparator(This) ) 

#define IScrollableContextMenu2_SetPlacement(This,scmp)	\
    ( (This)->lpVtbl -> SetPlacement(This,scmp) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IScrollableContextMenu2_INTERFACE_DEFINED__ */


#ifndef __IActiveXUIHandlerSite_INTERFACE_DEFINED__
#define __IActiveXUIHandlerSite_INTERFACE_DEFINED__

/* interface IActiveXUIHandlerSite */
/* [uuid][unique][object] */ 


EXTERN_C const IID IID_IActiveXUIHandlerSite;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("30510853-98b5-11cf-bb82-00aa00bdce0b")
    IActiveXUIHandlerSite : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE CreateScrollableContextMenu( 
            /* [out][retval] */ __RPC__deref_out_opt IScrollableContextMenu **scrollableContextMenu) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE PickFileAndGetResult( 
            /* [in] */ __RPC__in_opt IUnknown *filePicker,
            /* [in] */ BOOL allowMultipleSelections,
            /* [out][retval] */ __RPC__deref_out_opt IUnknown **result) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IActiveXUIHandlerSiteVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IActiveXUIHandlerSite * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IActiveXUIHandlerSite * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IActiveXUIHandlerSite * This);
        
        DECLSPEC_XFGVIRT(IActiveXUIHandlerSite, CreateScrollableContextMenu)
        HRESULT ( STDMETHODCALLTYPE *CreateScrollableContextMenu )( 
            __RPC__in IActiveXUIHandlerSite * This,
            /* [out][retval] */ __RPC__deref_out_opt IScrollableContextMenu **scrollableContextMenu);
        
        DECLSPEC_XFGVIRT(IActiveXUIHandlerSite, PickFileAndGetResult)
        HRESULT ( STDMETHODCALLTYPE *PickFileAndGetResult )( 
            __RPC__in IActiveXUIHandlerSite * This,
            /* [in] */ __RPC__in_opt IUnknown *filePicker,
            /* [in] */ BOOL allowMultipleSelections,
            /* [out][retval] */ __RPC__deref_out_opt IUnknown **result);
        
        END_INTERFACE
    } IActiveXUIHandlerSiteVtbl;

    interface IActiveXUIHandlerSite
    {
        CONST_VTBL struct IActiveXUIHandlerSiteVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IActiveXUIHandlerSite_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IActiveXUIHandlerSite_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IActiveXUIHandlerSite_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IActiveXUIHandlerSite_CreateScrollableContextMenu(This,scrollableContextMenu)	\
    ( (This)->lpVtbl -> CreateScrollableContextMenu(This,scrollableContextMenu) ) 

#define IActiveXUIHandlerSite_PickFileAndGetResult(This,filePicker,allowMultipleSelections,result)	\
    ( (This)->lpVtbl -> PickFileAndGetResult(This,filePicker,allowMultipleSelections,result) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IActiveXUIHandlerSite_INTERFACE_DEFINED__ */


#ifndef __IActiveXUIHandlerSite3_INTERFACE_DEFINED__
#define __IActiveXUIHandlerSite3_INTERFACE_DEFINED__

/* interface IActiveXUIHandlerSite3 */
/* [uuid][unique][object] */ 


EXTERN_C const IID IID_IActiveXUIHandlerSite3;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("7904009a-1238-47f4-901c-871375c34608")
    IActiveXUIHandlerSite3 : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE MessageBoxW( 
            /* [unique][in] */ __RPC__in_opt HWND hwnd,
            /* [unique][in] */ __RPC__in_opt LPCWSTR text,
            /* [unique][in] */ __RPC__in_opt LPCWSTR caption,
            /* [in] */ UINT type,
            /* [out] */ __RPC__out INT *result) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IActiveXUIHandlerSite3Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IActiveXUIHandlerSite3 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IActiveXUIHandlerSite3 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IActiveXUIHandlerSite3 * This);
        
        DECLSPEC_XFGVIRT(IActiveXUIHandlerSite3, MessageBoxW)
        HRESULT ( STDMETHODCALLTYPE *MessageBoxW )( 
            __RPC__in IActiveXUIHandlerSite3 * This,
            /* [unique][in] */ __RPC__in_opt HWND hwnd,
            /* [unique][in] */ __RPC__in_opt LPCWSTR text,
            /* [unique][in] */ __RPC__in_opt LPCWSTR caption,
            /* [in] */ UINT type,
            /* [out] */ __RPC__out INT *result);
        
        END_INTERFACE
    } IActiveXUIHandlerSite3Vtbl;

    interface IActiveXUIHandlerSite3
    {
        CONST_VTBL struct IActiveXUIHandlerSite3Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IActiveXUIHandlerSite3_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IActiveXUIHandlerSite3_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IActiveXUIHandlerSite3_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IActiveXUIHandlerSite3_MessageBoxW(This,hwnd,text,caption,type,result)	\
    ( (This)->lpVtbl -> MessageBoxW(This,hwnd,text,caption,type,result) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IActiveXUIHandlerSite3_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_ie12plugin_0000_0019 */
/* [local] */ 

//;end_internal


extern RPC_IF_HANDLE __MIDL_itf_ie12plugin_0000_0019_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_ie12plugin_0000_0019_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

unsigned long             __RPC_USER  HWND_UserSize(     __RPC__in unsigned long *, unsigned long            , __RPC__in HWND * ); 
unsigned char * __RPC_USER  HWND_UserMarshal(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in HWND * ); 
unsigned char * __RPC_USER  HWND_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out HWND * ); 
void                      __RPC_USER  HWND_UserFree(     __RPC__in unsigned long *, __RPC__in HWND * ); 

unsigned long             __RPC_USER  HWND_UserSize64(     __RPC__in unsigned long *, unsigned long            , __RPC__in HWND * ); 
unsigned char * __RPC_USER  HWND_UserMarshal64(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in HWND * ); 
unsigned char * __RPC_USER  HWND_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out HWND * ); 
void                      __RPC_USER  HWND_UserFree64(     __RPC__in unsigned long *, __RPC__in HWND * ); 

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


